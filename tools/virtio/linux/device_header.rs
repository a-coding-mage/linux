#[repr(C)]
pub struct device {
    pub parent: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const core::ffi::c_char,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
