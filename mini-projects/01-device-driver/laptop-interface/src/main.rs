// most to least significant - right, left, bottom, top, NW, SW, SE, NE
mod ffi;

use ffi::*;
use libc::{c_void, size_t};
use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::ptr;
use std::thread;
use std::time::Duration;

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

fn main() {
    unsafe {
        // List available serial ports
        let mut port_list: *mut *mut SpPort = ptr::null_mut();
        let res = sp_list_ports(&mut port_list);
        if res != SpReturn::SP_OK {
            eprintln!("Failed to list ports");
            return;
        }

        let mut i = 0;
        let mut selected_port: *mut SpPort = ptr::null_mut();
        println!("Available serial ports:");
        while !(*port_list.offset(i)).is_null() {
            let port = *port_list.offset(i);
            let name = sp_get_port_name(port);
            let name_str = CStr::from_ptr(name).to_string_lossy();
            println!("Port {}: {}", i, name_str);

            // Check if the port name contains "usb"
            if name_str.to_lowercase().contains("usb") {
                selected_port = port;
            }

            i += 1;
        }

        if selected_port.is_null() {
            eprintln!("No serial port containing 'usb' found");
            sp_free_port_list(port_list);
            return;
        }

        // Get the port name
        let port_name = sp_get_port_name(selected_port);
        let port_name_cstr = CStr::from_ptr(port_name);
        let port_name_cstring = CString::new(port_name_cstr.to_bytes()).unwrap();
        let port_name_str = port_name_cstr.to_str().expect("Should have port name");

        sp_free_port_list(port_list); // Free the port list as we don't need it anymore

        // Get the port by name
        let mut port_handle: *mut SpPort = ptr::null_mut();
        let res = sp_get_port_by_name(port_name_cstring.as_ptr(), &mut port_handle);
        if res != SpReturn::SP_OK {
            eprintln!("Failed to get port");
            return;
        }

        // Open the port
        let res = sp_open(port_handle, SP_MODE_READ_WRITE);
        if res != SpReturn::SP_OK {
            eprintln!("Failed to open port");
            sp_free_port(port_handle);
            return;
        }

        // Configure port settings
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

        // Write "clear all leds" to the port
        let clear_all_leds = b"clear all leds";
        match blocking_write(port_handle, clear_all_leds) {
            Ok(_) => println!(
                "Wrote {} to the port {}",
                std::str::from_utf8(clear_all_leds).unwrap(),
                port_name_str
            ),
            Err(e) => eprintln!(
                "Failed to write {} to {}: {}",
                std::str::from_utf8(clear_all_leds).unwrap(),
                port_name_str,
                e
            ),
        }

        // Wait for the device to process the command
        thread::sleep(Duration::from_millis(100));

        let start_controller = b"start controller";
        match blocking_write(port_handle, start_controller) {
            Ok(_) => println!(
                "Wrote {} to the port {}",
                std::str::from_utf8(start_controller).unwrap(),
                port_name_str
            ),
            Err(e) => eprintln!(
                "Failed to write {} to {}: {}",
                std::str::from_utf8(start_controller).unwrap(),
                port_name_str,
                e
            ),
        }

        loop {
            // Optionally, read any responses from the device
            let mut buffer = [0u8; 3];
            buffer.fill(0);
            let bytes_read = sp_blocking_read(
                port_handle,
                buffer.as_mut_ptr() as *mut c_void,
                buffer.len(),
                1000,
            );

            let adc_value = u16::from_be_bytes([buffer[1], buffer[2]]);
            println!("Button: {}, ADC: {}", buffer[0], adc_value);
        }

        // Close and free the port
        sp_close(port_handle);
        sp_free_port(port_handle);
    }
}
