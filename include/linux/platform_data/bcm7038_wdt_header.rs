// Translation of the declarations from bcm7038_wdt.h.

#[repr(C)]
pub struct bcm7038_wdt_platform_data {
    pub clk_name: *const core::ffi::c_char,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
