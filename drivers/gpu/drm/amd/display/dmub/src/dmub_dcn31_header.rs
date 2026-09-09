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
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
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

// C dependency: dmub_dcn20.h

#[repr(C)]
pub struct dmub_srv;

/* DCN31 register definitions. */
pub const DMUB_DCN31_REGS: &[&str] = &[
    "DMCUB_CNTL", "DMCUB_CNTL2", "DMCUB_SEC_CNTL", "DMCUB_INBOX0_SIZE",
    "DMCUB_INBOX0_RPTR", "DMCUB_INBOX0_WPTR", "DMCUB_INBOX1_BASE_ADDRESS",
    "DMCUB_INBOX1_SIZE", "DMCUB_INBOX1_RPTR", "DMCUB_INBOX1_WPTR",
    "DMCUB_OUTBOX0_BASE_ADDRESS", "DMCUB_OUTBOX0_SIZE", "DMCUB_OUTBOX0_RPTR",
    "DMCUB_OUTBOX0_WPTR", "DMCUB_OUTBOX1_BASE_ADDRESS", "DMCUB_OUTBOX1_SIZE",
    "DMCUB_OUTBOX1_RPTR", "DMCUB_OUTBOX1_WPTR", "DMCUB_REGION3_CW0_OFFSET",
    "DMCUB_REGION3_CW1_OFFSET", "DMCUB_REGION3_CW2_OFFSET", "DMCUB_REGION3_CW3_OFFSET",
    "DMCUB_REGION3_CW4_OFFSET", "DMCUB_REGION3_CW5_OFFSET", "DMCUB_REGION3_CW6_OFFSET",
    "DMCUB_REGION3_CW7_OFFSET", "DMCUB_REGION3_CW0_OFFSET_HIGH", "DMCUB_REGION3_CW1_OFFSET_HIGH",
    "DMCUB_REGION3_CW2_OFFSET_HIGH", "DMCUB_REGION3_CW3_OFFSET_HIGH", "DMCUB_REGION3_CW4_OFFSET_HIGH",
    "DMCUB_REGION3_CW5_OFFSET_HIGH", "DMCUB_REGION3_CW6_OFFSET_HIGH", "DMCUB_REGION3_CW7_OFFSET_HIGH",
    "DMCUB_REGION3_CW0_BASE_ADDRESS", "DMCUB_REGION3_CW1_BASE_ADDRESS", "DMCUB_REGION3_CW2_BASE_ADDRESS",
    "DMCUB_REGION3_CW3_BASE_ADDRESS", "DMCUB_REGION3_CW4_BASE_ADDRESS", "DMCUB_REGION3_CW5_BASE_ADDRESS",
    "DMCUB_REGION3_CW6_BASE_ADDRESS", "DMCUB_REGION3_CW7_BASE_ADDRESS", "DMCUB_REGION3_CW0_TOP_ADDRESS",
    "DMCUB_REGION3_CW1_TOP_ADDRESS", "DMCUB_REGION3_CW2_TOP_ADDRESS", "DMCUB_REGION3_CW3_TOP_ADDRESS",
    "DMCUB_REGION3_CW4_TOP_ADDRESS", "DMCUB_REGION3_CW5_TOP_ADDRESS", "DMCUB_REGION3_CW6_TOP_ADDRESS",
    "DMCUB_REGION3_CW7_TOP_ADDRESS", "DMCUB_REGION4_OFFSET", "DMCUB_REGION4_OFFSET_HIGH",
    "DMCUB_REGION4_TOP_ADDRESS", "DMCUB_REGION5_OFFSET", "DMCUB_REGION5_OFFSET_HIGH",
    "DMCUB_REGION5_TOP_ADDRESS", "DMCUB_SCRATCH0", "DMCUB_SCRATCH1", "DMCUB_SCRATCH2",
    "DMCUB_SCRATCH3", "DMCUB_SCRATCH4", "DMCUB_SCRATCH5", "DMCUB_SCRATCH6", "DMCUB_SCRATCH7",
    "DMCUB_SCRATCH8", "DMCUB_SCRATCH9", "DMCUB_SCRATCH10", "DMCUB_SCRATCH11", "DMCUB_SCRATCH12",
    "DMCUB_SCRATCH13", "DMCUB_SCRATCH14", "DMCUB_SCRATCH15", "DMCUB_GPINT_DATAIN1",
    "DMCUB_GPINT_DATAOUT", "CC_DC_PIPE_DIS", "MMHUBBUB_SOFT_RESET", "DCN_VM_FB_LOCATION_BASE",
    "DCN_VM_FB_OFFSET", "DMCUB_TIMER_CURRENT", "DMCUB_INST_FETCH_FAULT_ADDR",
    "DMCUB_UNDEFINED_ADDRESS_FAULT_ADDR", "DMCUB_DATA_WRITE_FAULT_ADDR", "DMCUB_INTERRUPT_ENABLE",
    "DMCUB_INTERRUPT_ACK",
];

// The C DMUB_DCN31_FIELDS() macro is retained as source-level metadata.
// Its generated members are represented explicitly below.

#[repr(C)]
pub struct dmub_srv_dcn31_reg_offset {
    pub _opaque: [u32; 0],
}

#[repr(C)]
pub struct dmub_srv_dcn31_reg_shift {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct dmub_srv_dcn31_reg_mask {
    pub _opaque: [u32; 0],
}

#[repr(C)]
pub struct dmub_srv_dcn31_regs {
    pub offset: dmub_srv_dcn31_reg_offset,
    pub mask: dmub_srv_dcn31_reg_mask,
    pub shift: dmub_srv_dcn31_reg_shift,
}

extern "C" {
    pub static dmub_srv_dcn31_regs: dmub_srv_dcn31_regs;

    pub fn dmub_dcn31_init(dmub: *mut dmub_srv);
    pub fn dmub_dcn31_reset(dmub: *mut dmub_srv);
    pub fn dmub_dcn31_reset_release(dmub: *mut dmub_srv);
    pub fn dmub_dcn31_backdoor_load(dmub: *mut dmub_srv, cw0: *const dmub_window, cw1: *const dmub_window);
    pub fn dmub_dcn31_setup_windows(dmub: *mut dmub_srv, cw2: *const dmub_window, cw3: *const dmub_window,
        cw4: *const dmub_window, cw5: *const dmub_window, cw6: *const dmub_window, region6: *const dmub_window);
    pub fn dmub_dcn31_setup_mailbox(dmub: *mut dmub_srv, inbox1: *const dmub_region);
    pub fn dmub_dcn31_get_inbox1_wptr(dmub: *mut dmub_srv) -> u32;
    pub fn dmub_dcn31_get_inbox1_rptr(dmub: *mut dmub_srv) -> u32;
    pub fn dmub_dcn31_set_inbox1_wptr(dmub: *mut dmub_srv, wptr_offset: u32);
    pub fn dmub_dcn31_setup_out_mailbox(dmub: *mut dmub_srv, outbox1: *const dmub_region);
    pub fn dmub_dcn31_get_outbox1_wptr(dmub: *mut dmub_srv) -> u32;
    pub fn dmub_dcn31_set_outbox1_rptr(dmub: *mut dmub_srv, rptr_offset: u32);
    pub fn dmub_dcn31_is_hw_init(dmub: *mut dmub_srv) -> bool;
    pub fn dmub_dcn31_is_supported(dmub: *mut dmub_srv) -> bool;
    pub fn dmub_dcn31_is_psrsu_supported(dmub: *mut dmub_srv) -> bool;
    pub fn dmub_dcn31_set_gpint(dmub: *mut dmub_srv, reg: dmub_gpint_data_register);
    pub fn dmub_dcn31_is_gpint_acked(dmub: *mut dmub_srv, reg: dmub_gpint_data_register) -> bool;
    pub fn dmub_dcn31_get_gpint_response(dmub: *mut dmub_srv) -> u32;
    pub fn dmub_dcn31_get_gpint_dataout(dmub: *mut dmub_srv) -> u32;
    pub fn dmub_dcn31_enable_dmub_boot_options(dmub: *mut dmub_srv, params: *const dmub_srv_hw_params);
    pub fn dmub_dcn31_skip_dmub_panel_power_sequence(dmub: *mut dmub_srv, skip: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
