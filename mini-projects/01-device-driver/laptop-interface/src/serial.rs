use crate::cffi::*;
use libc::{c_void, size_t};
use std::ffi::{CStr, CString};
use std::io;
use std::ptr;
use std::thread;

// Represents the state of the buttons and ADC value
#[derive(Debug, Clone, Copy)]
pub struct ControllerState {
    pub buttons: u8,
    pub adc_value: u16,
}

/// Find and return a list of available USB ports
pub unsafe fn get_ports() -> Result<Vec<String>, String> {
    let mut port_list: *mut *mut SpPort = ptr::null_mut();
    let res = sp_list_ports(&mut port_list);
    if res != SpReturn::SP_OK {
        return Err("Failed to list ports".to_string());
    }

    let mut ports = Vec::new();
    let mut i = 0;
    while !(*port_list.offset(i)).is_null() {
        let port = *port_list.offset(i);
        let name = sp_get_port_name(port);
        let name_str = CStr::from_ptr(name).to_string_lossy();

        // Check if the port name contains "usb"
        if name_str.to_lowercase().contains("usb") {
            ports.push(name_str.to_string());
        }

        i += 1;
    }

    sp_free_port_list(port_list); // Free the port list
    if ports.is_empty() {
        return Err("No USB ports found".to_string());
    }

    Ok(ports)
}

/// Initialize the serial connection to the specified port
pub unsafe fn init_serial(port_name: &str) -> Result<*mut SpPort, String> {
    let port_name_cstring = CString::new(port_name).unwrap();
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

    // Write "stop controller" to the port to make sure it's in the right state
    let stop_controller = b"stop controller\n";
    match blocking_write(port_handle, stop_controller) {
        Ok(_) => println!(
            "Wrote {} to the port {}",
            std::str::from_utf8(stop_controller).unwrap(),
            port_name
        ),
        Err(e) => eprintln!(
            "Failed to write {} to {}: {}",
            std::str::from_utf8(stop_controller).unwrap(),
            port_name,
            e
        ),
    }

    thread::sleep(std::time::Duration::from_millis(100));

    // Write "reset" to the port
    let reset_controller = b"reset\n";
    match blocking_write(port_handle, reset_controller) {
        Ok(_) => println!(
            "Wrote {} to the port {}",
            std::str::from_utf8(reset_controller).unwrap(),
            port_name
        ),
        Err(e) => eprintln!(
            "Failed to write {} to {}: {}",
            std::str::from_utf8(reset_controller).unwrap(),
            port_name,
            e
        ),
    }

    thread::sleep(std::time::Duration::from_millis(100));

    // Write "init controller" to the port
    let init_controller = b"init controller\n";
    match blocking_write(port_handle, init_controller) {
        Ok(_) => println!(
            "Wrote {} to the port {}",
            std::str::from_utf8(init_controller).unwrap(),
            port_name
        ),
        Err(e) => eprintln!(
            "Failed to write {} to {}: {}",
            std::str::from_utf8(init_controller).unwrap(),
            port_name,
            e
        ),
    }

    Ok(port_handle)
}

/// Blocking write to the serial port
pub unsafe fn blocking_write(port_handle: *mut SpPort, data: &[u8]) -> io::Result<usize> {
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

/// Read the state of the controller
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
        Some(ControllerState {
            buttons: buffer[0],
            adc_value,
        })
    } else {
        None
    }
}
