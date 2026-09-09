/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2024 Intel Corporation */

/* Dependency supplied by <net/libeth/rx.h>. */

/* Rx buffer management */

/* The largest size for a single descriptor as per HW */
pub const LIBIE_MAX_RX_BUF_LEN: u32 = 9728u32;

/* "True" HW-writeable space: minimum from SW and HW values */
#[inline]
pub const fn libie_rx_buf_len(hr: u32) -> u32 {
    let page_len = libeth_rx_page_len(hr);
    if page_len < LIBIE_MAX_RX_BUF_LEN {
        page_len
    } else {
        LIBIE_MAX_RX_BUF_LEN
    }
}

/* The maximum frame size as per HW (S/G) */
pub const __LIBIE_MAX_RX_FRM_LEN: u32 = 16382u32;

/* ATST, HW can chain up to 5 Rx descriptors */
#[inline]
pub const fn libie_max_rx_frm_len(hr: u32) -> u32 {
    let chained_len = libie_rx_buf_len(hr).wrapping_mul(5);
    if __LIBIE_MAX_RX_FRM_LEN < chained_len {
        __LIBIE_MAX_RX_FRM_LEN
    } else {
        chained_len
    }
}

/* Maximum frame size minus LL overhead */
pub const LIBIE_MAX_MTU: u32 =
    libie_max_rx_frm_len(LIBETH_MAX_HEADROOM).wrapping_sub(LIBETH_RX_LL_LEN);

/* O(1) converting i40e/ice/iavf's 8/10-bit hardware packet type to a parsed
 * bitfield struct.
 */

pub const LIBIE_RX_PT_NUM: usize = 154;

unsafe extern "C" {
    pub static libie_rx_pt_lut: [libeth_rx_pt; LIBIE_RX_PT_NUM];
}

/**
 * libie_rx_pt_parse - convert HW packet type to software bitfield structure
 * @pt: 10-bit hardware packet type value from the descriptor
 *
 * ```libie_rx_pt_lut``` must be accessed only using this wrapper.
 *
 * Return: parsed bitfield struct corresponding to the provided ptype.
 */
#[inline]
pub fn libie_rx_pt_parse(mut pt: u32) -> libeth_rx_pt {
    if pt >= LIBIE_RX_PT_NUM as u32 {
        pt = 0;
    }

    unsafe { libie_rx_pt_lut[pt as usize] }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
