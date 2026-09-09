// SPDX-License-Identifier: GPL-2.0-only
/*
 * Freescale SOC support functions
 *
 * Author: Scott Wood <scottwood@freescale.com>
 *
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

use core::ffi::c_void;

// Declarations supplied by the corresponding platform headers.
extern "C" {
    fn find_node_by_devtype(parent: *mut c_void, devtype: *const u8) -> *mut c_void;
    fn getprop(node: *mut c_void, name: *const u8, buf: *mut c_void, max_len: i32) -> i32;
    fn dt_xlate_addr(
        node: *mut c_void,
        prop: *const u32,
        size: i32,
        result: *mut usize,
    ) -> i32;
    fn printf(format: *const u8, ...);
}

// MAX_PROP_LEN is supplied by the platform headers.
static mut prop_buf: [u32; MAX_PROP_LEN / 4] = [0; MAX_PROP_LEN / 4];

pub unsafe fn fsl_get_immr() -> *mut u32 {
    let mut soc: *mut c_void;
    let mut ret: usize = 0;

    soc = find_node_by_devtype(core::ptr::null_mut(), b"soc\0".as_ptr());
    if !soc.is_null() {
        let mut size: i32;
        let mut naddr: u32;

        size = getprop(
            soc,
            b"#address-cells\0".as_ptr(),
            prop_buf.as_mut_ptr() as *mut c_void,
            MAX_PROP_LEN as i32,
        );
        if size == 4 {
            naddr = prop_buf[0];
        } else {
            naddr = 2;
        }

        if naddr != 1 && naddr != 2 {
            goto_err();
            return fsl_get_immr_result(ret);
        }

        size = getprop(
            soc,
            b"ranges\0".as_ptr(),
            prop_buf.as_mut_ptr() as *mut c_void,
            MAX_PROP_LEN as i32,
        );

        if size < 12 {
            goto_err();
            return fsl_get_immr_result(ret);
        }
        if prop_buf[0] != 0 {
            goto_err();
            return fsl_get_immr_result(ret);
        }
        if naddr == 2 && prop_buf[1] != 0 {
            goto_err();
            return fsl_get_immr_result(ret);
        }

        if dt_xlate_addr(soc, prop_buf.as_ptr().add(naddr as usize), 8, &mut ret) == 0 {
            ret = 0;
        }
    }

    fsl_get_immr_result(ret)
}

#[inline(always)]
unsafe fn goto_err() {}

unsafe fn fsl_get_immr_result(ret: usize) -> *mut u32 {
    if ret == 0 {
        printf(b"fsl_get_immr: Failed to find immr base\r\n\0".as_ptr());
    }

    ret as *mut u32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
