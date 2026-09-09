// Translated from linux/gpio/aspeed.h.
// The original header includes <linux/types.h> for u8 and u16.

use core::ffi::c_void;

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aspeed_gpio_copro_ops {
    pub request_access: Option<unsafe extern "C" fn(data: *mut c_void) -> i32>,
    pub release_access: Option<unsafe extern "C" fn(data: *mut c_void) -> i32>,
}

unsafe extern "C" {
    pub fn aspeed_gpio_copro_grab_gpio(
        desc: *mut gpio_desc,
        vreg_offset: *mut u16,
        dreg_offset: *mut u16,
        bit: *mut u8,
    ) -> i32;

    pub fn aspeed_gpio_copro_release_gpio(desc: *mut gpio_desc) -> i32;

    pub fn aspeed_gpio_copro_set_ops(
        ops: *const aspeed_gpio_copro_ops,
        data: *mut c_void,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
