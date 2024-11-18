use crate::cffi::*;
use libc::{c_void, size_t};
use std::ffi::{CStr, CString};
use std::io;
use std::ptr;

// Represents the state of the buttons and ADC value
#[derive(Debug, Clone, Copy)]
pub struct ControllerState {
    pub buttons: u8,
    pub adc_value: u16,
}

unsafe fn blocking_write(port_handle: *mut SpPort, data: &[u8]) -> io::Result<usize> {
    let mut written = 0;
    while written < data.len() {
        let bytes_written = sp_blocking_write(
            port_handle,
            data.as_ptr() as *const c_void,
            data.len() as size_t,
            1000,
        );
        if bytes_written < 0 {
            return Err(io::Error::last_os_error());
        }
        written += bytes_written as usize;
    }
    Ok(written)
}

pub unsafe fn init_serial() -> Result<*mut SpPort, String> {
    let mut port_list: *mut *mut SpPort = ptr::null_mut();
    let res = sp_list_ports(&mut port_list);
    if res != SpReturn::SP_OK {
        return Err("Failed to list ports".to_string());
    }

    let mut selected_port: *mut SpPort = ptr::null_mut();
    let mut i = 0;
    while !(*port_list.offset(i)).is_null() {
        let port = *port_list.offset(i);
        let name = sp_get_port_name(port);
        let name_str = CStr::from_ptr(name).to_string_lossy();
        println!("Port {}: {}", i, name_str);

        // Check if the port name contains "usb"
        if name_str.to_lowercase().contains("usb") {
            println!("Selected port: {}", name_str); // Add this debug log
            selected_port = port;
        }

        i += 1;
    }

    if selected_port.is_null() {
        return Err("No serial port containing 'usb' found".to_string());
    }

    // Get the port name
    let port_name = sp_get_port_name(selected_port);
    let port_name_cstr = CStr::from_ptr(port_name);
    let port_name_cstring = CString::new(port_name_cstr.to_bytes()).unwrap();
    let port_name_str = port_name_cstr.to_str().expect("Should have port name");

    sp_free_port_list(port_list);

    let mut port_handle: *mut SpPort = ptr::null_mut();
    let res = sp_get_port_by_name(port_name_cstring.as_ptr(), &mut port_handle);
    if res != SpReturn::SP_OK {
        return Err("Failed to get port by name".to_string());
    }

    let res = sp_open(port_handle, SP_MODE_READ_WRITE);
    if res != SpReturn::SP_OK {
        return Err("Failed to open port".to_string());
    }

    sp_set_baudrate(port_handle, 115200);
    sp_set_bits(port_handle, 8);
    sp_set_parity(port_handle, SpParity::SP_PARITY_NONE);
    sp_set_stopbits(port_handle, 1);
    sp_set_flowcontrol(port_handle, SpFlowcontrol::SP_FLOWCONTROL_NONE);

    // Write "init controller" to the port
    let init_controller = b"init controller";
    match blocking_write(port_handle, init_controller) {
        Ok(_) => println!(
            "Wrote {} to the port {}",
            std::str::from_utf8(init_controller).unwrap(),
            port_name_str
        ),
        Err(e) => eprintln!(
            "Failed to write {} to {}: {}",
            std::str::from_utf8(init_controller).unwrap(),
            port_name_str,
            e
        ),
    }

    // Write "init controller" to the port
    let start_controller = b"start controller";
    match blocking_write(port_handle, start_controller) {
        Ok(_) => println!(
            "Wrote {} to the port {}",
            std::str::from_utf8(start_controller).unwrap(),
            port_name_str
        ),
        Err(e) => eprintln!(
            "Failed to write {} to {}: {}",
            std::str::from_utf8(init_controller).unwrap(),
            port_name_str,
            e
        ),
    }

    Ok(port_handle)
}

pub unsafe fn read_controller_state(port_handle: *mut SpPort) -> Option<ControllerState> {
    let mut buffer = [0u8; 3];
    let bytes_read = sp_blocking_read(
        port_handle,
        buffer.as_mut_ptr() as *mut c_void,
        buffer.len(),
        1000,
    );

    if bytes_read == 3 {
        let adc_value = u16::from_be_bytes([buffer[1], buffer[2]]);
        println!("Buttons: {}, ADC value: {}", buffer[0], adc_value);
        Some(ControllerState {
            buttons: buffer[0],
            adc_value,
        })
    } else {
        None
    }
}
