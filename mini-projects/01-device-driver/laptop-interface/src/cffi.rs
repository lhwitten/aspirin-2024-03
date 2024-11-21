use libc::{c_char, c_int, c_uint, c_void, size_t};

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum SpReturn {
    SP_OK = 0,
    SP_ERR_ARG = -1,
    SP_ERR_FAIL = -2,
    SP_ERR_MEM = -3,
    SP_ERR_SUPP = -4,
    SP_ERR_OS = -5,
    SP_ERR_ACCESS = -6,
    SP_ERR_CANCELLED = -7,
    SP_ERR_TIMEOUT = -8,
}

pub const SP_MODE_READ_WRITE: u32 = 3;

#[repr(C)]
pub struct SpPort {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum SpParity {
    SP_PARITY_INVALID = -1,
    SP_PARITY_NONE = 0,
    SP_PARITY_ODD = 1,
    SP_PARITY_EVEN = 2,
    SP_PARITY_MARK = 3,
    SP_PARITY_SPACE = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum SpFlowcontrol {
    SP_FLOWCONTROL_NONE = 0,
    SP_FLOWCONTROL_XONXOFF = 1,
    SP_FLOWCONTROL_RTSCTS = 2,
    SP_FLOWCONTROL_DTRDSR = 3,
}

extern "C" {
    pub fn sp_get_port_by_name(name: *const c_char, port_ptr: *mut *mut SpPort) -> SpReturn;
    pub fn sp_free_port(port: *mut SpPort);
    pub fn sp_open(port: *mut SpPort, flags: u32) -> SpReturn;
    pub fn sp_close(port: *mut SpPort) -> SpReturn;
    pub fn sp_blocking_write(
        port: *mut SpPort,
        buf: *const c_void,
        count: size_t,
        timeout_ms: c_uint,
    ) -> c_int;
    pub fn sp_blocking_read(
        port: *mut SpPort,
        buf: *mut c_void,
        count: size_t,
        timeout_ms: c_uint,
    ) -> c_int;
    pub fn sp_list_ports(port_list_ptr: *mut *mut *mut SpPort) -> SpReturn;
    pub fn sp_free_port_list(port_list: *mut *mut SpPort);
    pub fn sp_get_port_name(port: *const SpPort) -> *const c_char;
    pub fn sp_set_baudrate(port: *mut SpPort, baudrate: c_uint) -> SpReturn;
    pub fn sp_set_bits(port: *mut SpPort, bits: c_int) -> SpReturn;
    pub fn sp_set_parity(port: *mut SpPort, parity: SpParity) -> SpReturn;
    pub fn sp_set_stopbits(port: *mut SpPort, stopbits: c_int) -> SpReturn;
    pub fn sp_set_flowcontrol(port: *mut SpPort, flowcontrol: SpFlowcontrol) -> SpReturn;
}
