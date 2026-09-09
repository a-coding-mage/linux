/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependency supplied by dmub_dcn31.h is intentionally not reproduced here.

#[repr(C)]
pub struct dmub_srv { _private: [u8; 0] }

macro_rules! dmub_dcn35_regs {
    ($m:ident) => {
        $m!(DMCUB_CNTL) $m!(DMCUB_CNTL2) $m!(DMCUB_SEC_CNTL)
        $m!(DMCUB_INBOX0_SIZE) $m!(DMCUB_INBOX0_RPTR) $m!(DMCUB_INBOX0_WPTR)
        $m!(DMCUB_INBOX1_BASE_ADDRESS) $m!(DMCUB_INBOX1_SIZE) $m!(DMCUB_INBOX1_RPTR) $m!(DMCUB_INBOX1_WPTR)
        $m!(DMCUB_OUTBOX0_BASE_ADDRESS) $m!(DMCUB_OUTBOX0_SIZE) $m!(DMCUB_OUTBOX0_RPTR) $m!(DMCUB_OUTBOX0_WPTR)
        $m!(DMCUB_OUTBOX1_BASE_ADDRESS) $m!(DMCUB_OUTBOX1_SIZE) $m!(DMCUB_OUTBOX1_RPTR) $m!(DMCUB_OUTBOX1_WPTR)
        $m!(DMCUB_REGION3_CW0_OFFSET) $m!(DMCUB_REGION3_CW1_OFFSET) $m!(DMCUB_REGION3_CW2_OFFSET) $m!(DMCUB_REGION3_CW3_OFFSET)
        $m!(DMCUB_REGION3_CW4_OFFSET) $m!(DMCUB_REGION3_CW5_OFFSET) $m!(DMCUB_REGION3_CW6_OFFSET) $m!(DMCUB_REGION3_CW7_OFFSET)
        $m!(DMCUB_REGION3_CW0_OFFSET_HIGH) $m!(DMCUB_REGION3_CW1_OFFSET_HIGH) $m!(DMCUB_REGION3_CW2_OFFSET_HIGH) $m!(DMCUB_REGION3_CW3_OFFSET_HIGH)
        $m!(DMCUB_REGION3_CW4_OFFSET_HIGH) $m!(DMCUB_REGION3_CW5_OFFSET_HIGH) $m!(DMCUB_REGION3_CW6_OFFSET_HIGH) $m!(DMCUB_REGION3_CW7_OFFSET_HIGH)
        $m!(DMCUB_REGION3_CW0_BASE_ADDRESS) $m!(DMCUB_REGION3_CW1_BASE_ADDRESS) $m!(DMCUB_REGION3_CW2_BASE_ADDRESS) $m!(DMCUB_REGION3_CW3_BASE_ADDRESS)
        $m!(DMCUB_REGION3_CW4_BASE_ADDRESS) $m!(DMCUB_REGION3_CW5_BASE_ADDRESS) $m!(DMCUB_REGION3_CW6_BASE_ADDRESS) $m!(DMCUB_REGION3_CW7_BASE_ADDRESS)
        $m!(DMCUB_REGION3_CW0_TOP_ADDRESS) $m!(DMCUB_REGION3_CW1_TOP_ADDRESS) $m!(DMCUB_REGION3_CW2_TOP_ADDRESS) $m!(DMCUB_REGION3_CW3_TOP_ADDRESS)
        $m!(DMCUB_REGION3_CW4_TOP_ADDRESS) $m!(DMCUB_REGION3_CW5_TOP_ADDRESS) $m!(DMCUB_REGION3_CW6_TOP_ADDRESS) $m!(DMCUB_REGION3_CW7_TOP_ADDRESS)
        $m!(DMCUB_REGION4_OFFSET) $m!(DMCUB_REGION4_OFFSET_HIGH) $m!(DMCUB_REGION4_TOP_ADDRESS)
        $m!(DMCUB_REGION5_OFFSET) $m!(DMCUB_REGION5_OFFSET_HIGH) $m!(DMCUB_REGION5_TOP_ADDRESS)
        $m!(DMCUB_REGION6_OFFSET) $m!(DMCUB_REGION6_OFFSET_HIGH) $m!(DMCUB_REGION6_TOP_ADDRESS)
        $m!(DMCUB_SCRATCH0) $m!(DMCUB_SCRATCH1) $m!(DMCUB_SCRATCH2) $m!(DMCUB_SCRATCH3) $m!(DMCUB_SCRATCH4) $m!(DMCUB_SCRATCH5) $m!(DMCUB_SCRATCH6) $m!(DMCUB_SCRATCH7)
        $m!(DMCUB_SCRATCH8) $m!(DMCUB_SCRATCH9) $m!(DMCUB_SCRATCH10) $m!(DMCUB_SCRATCH11) $m!(DMCUB_SCRATCH12) $m!(DMCUB_SCRATCH13) $m!(DMCUB_SCRATCH14) $m!(DMCUB_SCRATCH15)
        $m!(DMCUB_SCRATCH16) $m!(DMCUB_SCRATCH17) $m!(DMCUB_SCRATCH18) $m!(DMCUB_SCRATCH19) $m!(DMCUB_SCRATCH20) $m!(DMCUB_SCRATCH21)
        $m!(DMCUB_GPINT_DATAIN0) $m!(DMCUB_GPINT_DATAIN1) $m!(DMCUB_GPINT_DATAOUT) $m!(CC_DC_PIPE_DIS) $m!(MMHUBBUB_SOFT_RESET)
        $m!(DCN_VM_FB_LOCATION_BASE) $m!(DCN_VM_FB_OFFSET) $m!(DMCUB_TIMER_CURRENT) $m!(DMCUB_INST_FETCH_FAULT_ADDR)
        $m!(DMCUB_UNDEFINED_ADDRESS_FAULT_ADDR) $m!(DMCUB_DATA_WRITE_FAULT_ADDR) $m!(DMCUB_REGION3_TMR_AXI_SPACE)
        $m!(DMCUB_INTERRUPT_ENABLE) $m!(DMCUB_INTERRUPT_ACK) $m!(DMU_CLK_CNTL)
    };
}

macro_rules! dmub_dcn35_fields {
    ($m:ident) => {
        $m!(DMCUB_CNTL, DMCUB_ENABLE) $m!(DMCUB_CNTL, DMCUB_TRACEPORT_EN) $m!(DMCUB_CNTL2, DMCUB_SOFT_RESET)
        $m!(DMCUB_SEC_CNTL, DMCUB_SEC_RESET) $m!(DMCUB_SEC_CNTL, DMCUB_MEM_UNIT_ID) $m!(DMCUB_SEC_CNTL, DMCUB_SEC_RESET_STATUS)
        $m!(DMCUB_REGION3_CW0_TOP_ADDRESS, DMCUB_REGION3_CW0_TOP_ADDRESS) $m!(DMCUB_REGION3_CW0_TOP_ADDRESS, DMCUB_REGION3_CW0_ENABLE)
        $m!(DMCUB_REGION3_CW1_TOP_ADDRESS, DMCUB_REGION3_CW1_TOP_ADDRESS) $m!(DMCUB_REGION3_CW1_TOP_ADDRESS, DMCUB_REGION3_CW1_ENABLE)
        $m!(DMCUB_REGION3_CW2_TOP_ADDRESS, DMCUB_REGION3_CW2_TOP_ADDRESS) $m!(DMCUB_REGION3_CW2_TOP_ADDRESS, DMCUB_REGION3_CW2_ENABLE)
        $m!(DMCUB_REGION3_CW3_TOP_ADDRESS, DMCUB_REGION3_CW3_TOP_ADDRESS) $m!(DMCUB_REGION3_CW3_TOP_ADDRESS, DMCUB_REGION3_CW3_ENABLE)
        $m!(DMCUB_REGION3_CW4_TOP_ADDRESS, DMCUB_REGION3_CW4_TOP_ADDRESS) $m!(DMCUB_REGION3_CW4_TOP_ADDRESS, DMCUB_REGION3_CW4_ENABLE)
        $m!(DMCUB_REGION3_CW5_TOP_ADDRESS, DMCUB_REGION3_CW5_TOP_ADDRESS) $m!(DMCUB_REGION3_CW5_TOP_ADDRESS, DMCUB_REGION3_CW5_ENABLE)
        $m!(DMCUB_REGION3_CW6_TOP_ADDRESS, DMCUB_REGION3_CW6_TOP_ADDRESS) $m!(DMCUB_REGION3_CW6_TOP_ADDRESS, DMCUB_REGION3_CW6_ENABLE)
        $m!(DMCUB_REGION3_CW7_TOP_ADDRESS, DMCUB_REGION3_CW7_TOP_ADDRESS) $m!(DMCUB_REGION3_CW7_TOP_ADDRESS, DMCUB_REGION3_CW7_ENABLE)
        $m!(DMCUB_REGION4_TOP_ADDRESS, DMCUB_REGION4_TOP_ADDRESS) $m!(DMCUB_REGION4_TOP_ADDRESS, DMCUB_REGION4_ENABLE)
        $m!(DMCUB_REGION5_TOP_ADDRESS, DMCUB_REGION5_TOP_ADDRESS) $m!(DMCUB_REGION5_TOP_ADDRESS, DMCUB_REGION5_ENABLE)
        $m!(DMCUB_REGION6_TOP_ADDRESS, DMCUB_REGION6_TOP_ADDRESS) $m!(DMCUB_REGION6_TOP_ADDRESS, DMCUB_REGION6_ENABLE)
        $m!(CC_DC_PIPE_DIS, DC_DMCUB_ENABLE) $m!(MMHUBBUB_SOFT_RESET, DMUIF_SOFT_RESET) $m!(DCN_VM_FB_LOCATION_BASE, FB_BASE)
        $m!(DCN_VM_FB_OFFSET, FB_OFFSET) $m!(DMCUB_INBOX0_WPTR, DMCUB_INBOX0_WPTR) $m!(DMCUB_REGION3_TMR_AXI_SPACE, DMCUB_REGION3_TMR_AXI_SPACE)
        $m!(DMCUB_INTERRUPT_ENABLE, DMCUB_GPINT_IH_INT_EN) $m!(DMCUB_INTERRUPT_ACK, DMCUB_GPINT_IH_INT_ACK) $m!(DMCUB_CNTL, DMCUB_PWAIT_MODE_STATUS)
        $m!(DMU_CLK_CNTL, LONO_DISPCLK_GATE_DISABLE) $m!(DMU_CLK_CNTL, LONO_SOCCLK_GATE_DISABLE) $m!(DMU_CLK_CNTL, LONO_DMCUBCLK_GATE_DISABLE)
    };
}

macro_rules! offset_field { ($x:ident) => { pub $x: u32, }; }
macro_rules! shift_field { ($r:ident, $f:ident) => { pub $r##__$f: u8, }; }
macro_rules! mask_field { ($r:ident, $f:ident) => { pub $r##__$f: u32, }; }

#[repr(C)] pub struct dmub_srv_dcn35_reg_offset { dmub_dcn35_regs!(offset_field) }
#[repr(C)] pub struct dmub_srv_dcn35_reg_shift { dmub_dcn35_fields!(shift_field) }
#[repr(C)] pub struct dmub_srv_dcn35_reg_mask { dmub_dcn35_fields!(mask_field) }
#[repr(C)] pub struct dmub_srv_dcn35_regs { pub offset: dmub_srv_dcn35_reg_offset, pub mask: dmub_srv_dcn35_reg_mask, pub shift: dmub_srv_dcn35_reg_shift }

extern "C" {
    pub fn dmub_dcn35_init(dmub: *mut dmub_srv);
    pub fn dmub_dcn35_reset(dmub: *mut dmub_srv);
    pub fn dmub_dcn35_reset_release(dmub: *mut dmub_srv);
    pub fn dmub_dcn35_backdoor_load(dmub: *mut dmub_srv, cw0: *const dmub_window, cw1: *const dmub_window);
    pub fn dmub_dcn35_backdoor_load_zfb_mode(dmub: *mut dmub_srv, cw0: *const dmub_window, cw1: *const dmub_window);
    pub fn dmub_dcn35_setup_windows(dmub: *mut dmub_srv, cw2: *const dmub_window, cw3: *const dmub_window, cw4: *const dmub_window, cw5: *const dmub_window, cw6: *const dmub_window, region6: *const dmub_window);
    pub fn dmub_dcn35_setup_mailbox(dmub: *mut dmub_srv, inbox1: *const dmub_region);
    pub fn dmub_dcn35_get_inbox1_wptr(dmub: *mut dmub_srv) -> u32;
    pub fn dmub_dcn35_get_inbox1_rptr(dmub: *mut dmub_srv) -> u32;
    pub fn dmub_dcn35_set_inbox1_wptr(dmub: *mut dmub_srv, wptr_offset: u32);
    pub fn dmub_dcn35_setup_out_mailbox(dmub: *mut dmub_srv, outbox1: *const dmub_region);
    pub fn dmub_dcn35_get_outbox1_wptr(dmub: *mut dmub_srv) -> u32;
    pub fn dmub_dcn35_set_outbox1_rptr(dmub: *mut dmub_srv, rptr_offset: u32);
    pub fn dmub_dcn35_is_hw_init(dmub: *mut dmub_srv) -> bool;
    pub fn dmub_dcn35_is_supported(dmub: *mut dmub_srv) -> bool;
    pub fn dmub_dcn35_set_gpint(dmub: *mut dmub_srv, reg: dmub_gpint_data_register);
    pub fn dmub_dcn35_is_gpint_acked(dmub: *mut dmub_srv, reg: dmub_gpint_data_register) -> bool;
    pub fn dmub_dcn35_get_gpint_response(dmub: *mut dmub_srv) -> u32;
    pub fn dmub_dcn35_get_gpint_dataout(dmub: *mut dmub_srv) -> u32;
    pub fn dmub_dcn35_enable_dmub_boot_options(dmub: *mut dmub_srv, params: *const dmub_srv_hw_params);
    pub fn dmub_dcn35_skip_dmub_panel_power_sequence(dmub: *mut dmub_srv, skip: bool);
    pub fn dmub_dcn35_get_fw_boot_status(dmub: *mut dmub_srv) -> dmub_fw_boot_status;
    pub fn dmub_dcn35_get_fw_boot_option(dmub: *mut dmub_srv) -> dmub_fw_boot_options;
    pub fn dmub_dcn35_setup_outbox0(dmub: *mut dmub_srv, outbox0: *const dmub_region);
    pub fn dmub_dcn35_get_outbox0_wptr(dmub: *mut dmub_srv) -> u32;
    pub fn dmub_dcn35_set_outbox0_rptr(dmub: *mut dmub_srv, rptr_offset: u32);
    pub fn dmub_dcn35_get_current_time(dmub: *mut dmub_srv) -> u32;
    pub fn dmub_dcn35_get_diagnostic_data(dmub: *mut dmub_srv);
    pub fn dmub_dcn35_configure_dmub_in_system_memory(dmub: *mut dmub_srv);
    pub fn dmub_dcn35_send_inbox0_cmd(dmub: *mut dmub_srv, data: dmub_inbox0_data_register);
    pub fn dmub_dcn35_clear_inbox0_ack_register(dmub: *mut dmub_srv);
    pub fn dmub_dcn35_read_inbox0_ack_register(dmub: *mut dmub_srv) -> u32;
    pub fn dmub_dcn35_should_detect(dmub: *mut dmub_srv) -> bool;
    pub fn dmub_dcn35_is_hw_powered_up(dmub: *mut dmub_srv) -> bool;
    pub fn dmub_srv_dcn35_regs_init(dmub: *mut dmub_srv, ctx: *mut dc_context);
    pub fn dmub_dcn35_get_preos_fw_info(dmub: *mut dmub_srv) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
