/* SPDX-License-Identifier: GPL-2.0 */
// Dependency declarations supplied by the platform and other kernel headers.

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum maple_code {
    MAPLE_RESPONSE_FILEERR = -5,
    MAPLE_RESPONSE_AGAIN,
    MAPLE_RESPONSE_BADCMD,
    MAPLE_RESPONSE_BADFUNC,
    MAPLE_RESPONSE_NONE,
    MAPLE_COMMAND_DEVINFO = 1,
    MAPLE_COMMAND_ALLINFO,
    MAPLE_COMMAND_RESET,
    MAPLE_COMMAND_KILL,
    MAPLE_RESPONSE_DEVINFO,
    MAPLE_RESPONSE_ALLINFO,
    MAPLE_RESPONSE_OK,
    MAPLE_RESPONSE_DATATRF,
    MAPLE_COMMAND_GETCOND,
    MAPLE_COMMAND_GETMINFO,
    MAPLE_COMMAND_BREAD,
    MAPLE_COMMAND_BWRITE,
    MAPLE_COMMAND_BSYNC,
    MAPLE_COMMAND_SETCOND,
    MAPLE_COMMAND_MICCONTROL,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum maple_file_errors {
    MAPLE_FILEERR_INVALID_PARTITION = 0x01000000,
    MAPLE_FILEERR_PHASE_ERROR = 0x02000000,
    MAPLE_FILEERR_INVALID_BLOCK = 0x04000000,
    MAPLE_FILEERR_WRITE_ERROR = 0x08000000,
    MAPLE_FILEERR_INVALID_WRITE_LENGTH = 0x10000000,
    MAPLE_FILEERR_BAD_CRC = 0x20000000,
}

pub struct device;
pub struct list_head;
pub struct maple_device;
pub struct maple_driver;
pub struct atomic_t;
pub struct wait_queue_head_t;
pub struct device_driver;

#[repr(C)]
pub struct maple_buffer {
    pub bufx: [kernel::ffi::c_char; 0x400],
    pub buf: *mut kernel::ffi::c_void,
}

#[repr(C)]
pub struct mapleq {
    pub list: list_head,
    pub dev: *mut maple_device,
    pub recvbuf: *mut maple_buffer,
    pub sendbuf: *mut kernel::ffi::c_void,
    pub recvbuf_p2: *mut kernel::ffi::c_void,
    pub length: u8,
    pub command: maple_code,
}

#[repr(C)]
pub struct maple_devinfo {
    pub function: kernel::ffi::c_ulong,
    pub function_data: [kernel::ffi::c_ulong; 3],
    pub area_code: u8,
    pub connector_direction: u8,
    pub product_name: [kernel::ffi::c_char; 31],
    pub product_licence: [kernel::ffi::c_char; 61],
    pub standby_power: u16,
    pub max_power: u16,
}

#[repr(C)]
pub struct maple_device {
    pub driver: *mut maple_driver,
    pub mq: *mut mapleq,
    pub callback: Option<unsafe extern "C" fn(*mut mapleq)>,
    pub fileerr_handler: Option<unsafe extern "C" fn(*mut maple_device, *mut kernel::ffi::c_void)>,
    pub can_unload: Option<unsafe extern "C" fn(*mut maple_device) -> i32>,
    pub when: kernel::ffi::c_ulong,
    pub interval: kernel::ffi::c_ulong,
    pub function: kernel::ffi::c_ulong,
    pub devinfo: maple_devinfo,
    pub port: u8,
    pub unit: u8,
    pub product_name: [kernel::ffi::c_char; 32],
    pub product_licence: [kernel::ffi::c_char; 64],
    pub busy: atomic_t,
    pub maple_wait: wait_queue_head_t,
    pub dev: device,
}

#[repr(C)]
pub struct maple_driver {
    pub function: kernel::ffi::c_ulong,
    pub drv: device_driver,
}

extern "C" {
    pub fn maple_getcond_callback(
        dev: *mut maple_device,
        callback: Option<unsafe extern "C" fn(*mut mapleq)>,
        interval: kernel::ffi::c_ulong,
        function: kernel::ffi::c_ulong,
    );
    pub fn maple_driver_register(driver: *mut maple_driver) -> i32;
    pub fn maple_driver_unregister(driver: *mut maple_driver);
    pub fn maple_add_packet(
        mdev: *mut maple_device,
        function: u32,
        command: u32,
        length: u32,
        data: *mut kernel::ffi::c_void,
    ) -> i32;
    pub fn maple_clear_dev(mdev: *mut maple_device);
    pub fn container_of<T>(ptr: *const kernel::ffi::c_void) -> *mut T;
    pub fn container_of_const<T>(ptr: *const kernel::ffi::c_void) -> *const T;
    pub fn dev_get_drvdata(dev: *mut device) -> *mut kernel::ffi::c_void;
    pub fn dev_set_drvdata(dev: *mut device, data: *mut kernel::ffi::c_void);
}

#[inline]
pub unsafe fn to_maple_dev(n: *mut device) -> *mut maple_device {
    container_of::<maple_device>(n.cast())
}

#[inline]
pub unsafe fn to_maple_driver(n: *const device_driver) -> *const maple_driver {
    container_of_const::<maple_driver>(n.cast())
}

#[inline]
pub unsafe fn maple_get_drvdata(d: *mut maple_device) -> *mut kernel::ffi::c_void {
    dev_get_drvdata(&mut (*d).dev)
}

#[inline]
pub unsafe fn maple_set_drvdata(d: *mut maple_device, p: *mut kernel::ffi::c_void) {
    dev_set_drvdata(&mut (*d).dev, p)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
