#[repr(C)]
pub struct device {
    pub parent: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const core::ffi::c_char,
}
