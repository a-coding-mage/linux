// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2020 Western Digital Corporation or its affiliates.
 */

use core::ffi::c_void;

/* Declarations supplied by the surrounding kernel sources. */
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
    pub data: Option<unsafe extern "C" fn(*const c_void)>,
}

unsafe extern "C" {
    static dtb_early_va: *const c_void;
    static __soc_early_init_table_start: of_device_id;
    static __soc_early_init_table_end: of_device_id;

    fn fdt_node_check_compatible(
        fdt: *const c_void,
        nodeoffset: i32,
        compatible: *const core::ffi::c_char,
    ) -> i32;
}

/*
 * This is called extremely early, before parse_dtb(), to allow initializing
 * SoC hardware before memory or any device driver initialization.
 */
pub unsafe fn soc_early_init() {
    let mut early_fn: Option<unsafe extern "C" fn(*const c_void)>;
    let mut s = &__soc_early_init_table_start as *const of_device_id;
    let end = &__soc_early_init_table_end as *const of_device_id;
    let fdt = dtb_early_va;

    while (s as *const c_void) < (end as *const c_void) {
        if fdt_node_check_compatible(fdt, 0, (*s).compatible) == 0 {
            early_fn = (*s).data;
            if let Some(early_fn) = early_fn {
                early_fn(fdt);
            }
            return;
        }
        s = s.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
