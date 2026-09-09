// SPDX-License-Identifier: GPL-2.0-only
/*
 * Routines common to most mpc86xx-based boards.
 */

// Translated dependencies: linux/init.h, linux/of_platform.h, asm/reg.h,
// asm/synch.h, and mpc86xx.h provide the declarations referenced below.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct of_device_id {
    pub name: *const c_char,
    pub type_: *const c_char,
    pub compatible: *const c_char,
    pub data: *const c_void,
}

unsafe extern "C" {
    fn of_platform_bus_probe(
        root: *mut c_void,
        matches: *const of_device_id,
        parent: *mut c_void,
    ) -> c_int;

    fn mtspr(spr: u32, value: u32);
    fn mfspr(spr: u32) -> u32;
    fn isync();
}

unsafe extern "C" {
    static SPRN_TBWL: u32;
    static SPRN_TBWU: u32;
    static SPRN_HID0: u32;
    static HID0_TBEN: u32;
}

static MPC86XX_COMMON_IDS: [of_device_id; 7] = [
    of_device_id {
        name: core::ptr::null(),
        type_: c"soc".as_ptr(),
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"soc".as_ptr(),
        data: core::ptr::null(),
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"simple-bus".as_ptr(),
        data: core::ptr::null(),
    },
    of_device_id {
        name: c"localbus".as_ptr(),
        type_: core::ptr::null(),
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"gianfar".as_ptr(),
        data: core::ptr::null(),
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: c"fsl,mpc8641-pcie".as_ptr(),
        data: core::ptr::null(),
    },
    of_device_id {
        name: core::ptr::null(),
        type_: core::ptr::null(),
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

pub unsafe fn mpc86xx_common_publish_devices() -> c_int {
    of_platform_bus_probe(
        core::ptr::null_mut(),
        MPC86XX_COMMON_IDS.as_ptr(),
        core::ptr::null_mut(),
    )
}

pub unsafe fn mpc86xx_time_init() -> i64 {
    let mut temp: u32;

    /* Set the time base to zero */
    mtspr(SPRN_TBWL, 0);
    mtspr(SPRN_TBWU, 0);

    temp = mfspr(SPRN_HID0);
    temp |= HID0_TBEN;
    mtspr(SPRN_HID0, temp);
    isync();

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
