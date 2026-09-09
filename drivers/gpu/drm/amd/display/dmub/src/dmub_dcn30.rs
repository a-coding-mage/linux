/*
 * Copyright 2020 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding translation unit:
// dmub_srv.h, dmub_reg.h, dmub_dcn20.h, dmub_dcn30.h,
// sienna_cichlid_ip_offset.h, dcn_3_0_0_offset.h, dcn_3_0_0_sh_mask.h

const BASE_INNER: &str = "DCN_BASE__INST0_SEG";

/* Registers. */

pub static dmub_srv_dcn30_regs: dmub_srv_common_regs = dmub_srv_common_regs {
    regs: [
        DMUB_COMMON_REGS!(),
        DMCUB_INTERNAL_REGS!(),
    ],
    masks: [DMUB_COMMON_FIELDS!()],
    shifts: [DMUB_COMMON_FIELDS!()],
};

/* Shared functions. */

unsafe fn dmub_dcn30_get_fb_base_offset(
    dmub: *mut dmub_srv,
    fb_base: *mut u64,
    fb_offset: *mut u64,
) {
    let mut tmp: u32 = 0;

    if (*dmub).soc_fb_info.fb_base != 0 || (*dmub).soc_fb_info.fb_offset != 0 {
        *fb_base = (*dmub).soc_fb_info.fb_base;
        *fb_offset = (*dmub).soc_fb_info.fb_offset;
        return;
    }

    REG_GET!(DCN_VM_FB_LOCATION_BASE, FB_BASE, &mut tmp);
    *fb_base = (tmp as u64) << 24;

    REG_GET!(DCN_VM_FB_OFFSET, FB_OFFSET, &mut tmp);
    *fb_offset = (tmp as u64) << 24;
}

#[inline]
unsafe fn dmub_dcn30_translate_addr(
    addr_in: *const dmub_addr,
    fb_base: u64,
    fb_offset: u64,
    addr_out: *mut dmub_addr,
) {
    (*addr_out).quad_part = (*addr_in).quad_part - fb_base + fb_offset;
}

pub unsafe fn dmub_dcn30_backdoor_load(
    dmub: *mut dmub_srv,
    cw0: *const dmub_window,
    cw1: *const dmub_window,
) {
    let mut offset: dmub_addr;
    let (mut fb_base, mut fb_offset): (u64, u64) = (0, 0);

    dmub_dcn30_get_fb_base_offset(dmub, &mut fb_base, &mut fb_offset);

    REG_UPDATE!(DMCUB_SEC_CNTL, DMCUB_SEC_RESET, 1);

    /* MEM_CTNL read/write space doesn't exist. */

    dmub_dcn30_translate_addr(&(*cw0).offset, fb_base, fb_offset, &mut offset);

    REG_WRITE!(DMCUB_REGION3_CW0_OFFSET, offset.u.low_part);
    REG_WRITE!(DMCUB_REGION3_CW0_OFFSET_HIGH, offset.u.high_part);
    REG_WRITE!(DMCUB_REGION3_CW0_BASE_ADDRESS, (*cw0).region.base);
    REG_SET_2!(DMCUB_REGION3_CW0_TOP_ADDRESS, 0,
        DMCUB_REGION3_CW0_TOP_ADDRESS, (*cw0).region.top,
        DMCUB_REGION3_CW0_ENABLE, 1);

    dmub_dcn30_translate_addr(&(*cw1).offset, fb_base, fb_offset, &mut offset);

    REG_WRITE!(DMCUB_REGION3_CW1_OFFSET, offset.u.low_part);
    REG_WRITE!(DMCUB_REGION3_CW1_OFFSET_HIGH, offset.u.high_part);
    REG_WRITE!(DMCUB_REGION3_CW1_BASE_ADDRESS, (*cw1).region.base);
    REG_SET_2!(DMCUB_REGION3_CW1_TOP_ADDRESS, 0,
        DMCUB_REGION3_CW1_TOP_ADDRESS, (*cw1).region.top,
        DMCUB_REGION3_CW1_ENABLE, 1);

    REG_UPDATE_2!(DMCUB_SEC_CNTL, DMCUB_SEC_RESET, 0, DMCUB_MEM_UNIT_ID, 0x20);
}

pub unsafe fn dmub_dcn30_setup_windows(
    dmub: *mut dmub_srv,
    cw2: *const dmub_window,
    cw3: *const dmub_window,
    cw4: *const dmub_window,
    cw5: *const dmub_window,
    cw6: *const dmub_window,
    region6: *const dmub_window,
) {
    let _ = region6;
    let mut offset: dmub_addr;

    /* sienna_cichlid has hardwired virtual addressing for CW2-CW7 */

    offset = (*cw2).offset;
    if (*cw2).region.base != (*cw2).region.top {
        REG_WRITE!(DMCUB_REGION3_CW2_OFFSET, offset.u.low_part);
        REG_WRITE!(DMCUB_REGION3_CW2_OFFSET_HIGH, offset.u.high_part);
        REG_WRITE!(DMCUB_REGION3_CW2_BASE_ADDRESS, (*cw2).region.base);
        REG_SET_2!(DMCUB_REGION3_CW2_TOP_ADDRESS, 0,
            DMCUB_REGION3_CW2_TOP_ADDRESS, (*cw2).region.top,
            DMCUB_REGION3_CW2_ENABLE, 1);
    } else {
        REG_WRITE!(DMCUB_REGION3_CW2_OFFSET, 0);
        REG_WRITE!(DMCUB_REGION3_CW2_OFFSET_HIGH, 0);
        REG_WRITE!(DMCUB_REGION3_CW2_BASE_ADDRESS, 0);
        REG_WRITE!(DMCUB_REGION3_CW2_TOP_ADDRESS, 0);
    }

    offset = (*cw3).offset;
    REG_WRITE!(DMCUB_REGION3_CW3_OFFSET, offset.u.low_part);
    REG_WRITE!(DMCUB_REGION3_CW3_OFFSET_HIGH, offset.u.high_part);
    REG_WRITE!(DMCUB_REGION3_CW3_BASE_ADDRESS, (*cw3).region.base);
    REG_SET_2!(DMCUB_REGION3_CW3_TOP_ADDRESS, 0,
        DMCUB_REGION3_CW3_TOP_ADDRESS, (*cw3).region.top,
        DMCUB_REGION3_CW3_ENABLE, 1);

    offset = (*cw4).offset;
    /* New firmware can support CW4. */
    if dmub_dcn20_use_cached_inbox(dmub) {
        REG_WRITE!(DMCUB_REGION3_CW4_OFFSET, offset.u.low_part);
        REG_WRITE!(DMCUB_REGION3_CW4_OFFSET_HIGH, offset.u.high_part);
        REG_WRITE!(DMCUB_REGION3_CW4_BASE_ADDRESS, (*cw4).region.base);
        REG_SET_2!(DMCUB_REGION3_CW4_TOP_ADDRESS, 0,
            DMCUB_REGION3_CW4_TOP_ADDRESS, (*cw4).region.top,
            DMCUB_REGION3_CW4_ENABLE, 1);
    } else {
        REG_WRITE!(DMCUB_REGION4_OFFSET, offset.u.low_part);
        REG_WRITE!(DMCUB_REGION4_OFFSET_HIGH, offset.u.high_part);
        REG_SET_2!(DMCUB_REGION4_TOP_ADDRESS, 0, DMCUB_REGION4_TOP_ADDRESS,
            (*cw4).region.top - (*cw4).region.base - 1, DMCUB_REGION4_ENABLE, 1);
    }

    offset = (*cw5).offset;
    REG_WRITE!(DMCUB_REGION3_CW5_OFFSET, offset.u.low_part);
    REG_WRITE!(DMCUB_REGION3_CW5_OFFSET_HIGH, offset.u.high_part);
    REG_WRITE!(DMCUB_REGION3_CW5_BASE_ADDRESS, (*cw5).region.base);
    REG_SET_2!(DMCUB_REGION3_CW5_TOP_ADDRESS, 0,
        DMCUB_REGION3_CW5_TOP_ADDRESS, (*cw5).region.top,
        DMCUB_REGION3_CW5_ENABLE, 1);
    REG_WRITE!(DMCUB_REGION5_OFFSET, offset.u.low_part);
    REG_WRITE!(DMCUB_REGION5_OFFSET_HIGH, offset.u.high_part);
    REG_SET_2!(DMCUB_REGION5_TOP_ADDRESS, 0, DMCUB_REGION5_TOP_ADDRESS,
        (*cw5).region.top - (*cw5).region.base - 1, DMCUB_REGION5_ENABLE, 1);

    offset = (*cw6).offset;
    REG_WRITE!(DMCUB_REGION3_CW6_OFFSET, offset.u.low_part);
    REG_WRITE!(DMCUB_REGION3_CW6_OFFSET_HIGH, offset.u.high_part);
    REG_WRITE!(DMCUB_REGION3_CW6_BASE_ADDRESS, (*cw6).region.base);
    REG_SET_2!(DMCUB_REGION3_CW6_TOP_ADDRESS, 0,
        DMCUB_REGION3_CW6_TOP_ADDRESS, (*cw6).region.top,
        DMCUB_REGION3_CW6_ENABLE, 1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
