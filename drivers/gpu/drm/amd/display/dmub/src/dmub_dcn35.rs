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

// Register and field macros below are supplied by the surrounding translation unit.

pub unsafe fn dmub_srv_dcn35_regs_init(dmub: *mut dmub_srv, ctx: *mut dc_context) {
    let regs = (*dmub).regs_dcn35;
    macro_rules! dmub_sr { ($reg:ident) => { (*regs).offset.$reg = REG_OFFSET_EXP!($reg); }; }
    DMUB_DCN35_REGS!();
    DMCUB_INTERNAL_REGS!();
    macro_rules! dmub_sf { ($reg:ident, $field:ident) => { (*regs).mask.$reg##__$field = FD_MASK!($reg, $field); }; }
    DMUB_DCN35_FIELDS!();
    macro_rules! dmub_sf { ($reg:ident, $field:ident) => { (*regs).shift.$reg##__$field = FD_SHIFT!($reg, $field); }; }
    DMUB_DCN35_FIELDS!();
}

unsafe fn dmub_dcn35_get_fb_base_offset(dmub: *mut dmub_srv, fb_base: *mut u64, fb_offset: *mut u64) {
    let mut tmp: u32 = 0;
    REG_GET!(DCN_VM_FB_LOCATION_BASE, FB_BASE, &mut tmp);
    *fb_base = (tmp as u64) << 24;
    REG_GET!(DCN_VM_FB_OFFSET, FB_OFFSET, &mut tmp);
    *fb_offset = (tmp as u64) << 24;
}

#[inline]
unsafe fn dmub_dcn35_translate_addr(addr_in: *const dmub_addr, fb_base: u64, fb_offset: u64, addr_out: *mut dmub_addr) {
    (*addr_out).quad_part = (*addr_in).quad_part - fb_base + fb_offset;
}

pub unsafe fn dmub_dcn35_reset(dmub: *mut dmub_srv) {
    let mut cmd: dmub_gpint_data_register = core::mem::zeroed();
    let timeout: u32 = 100000;
    let (mut in_reset, mut is_enabled, mut scratch, mut i, mut pwait_mode) = (0, 0, 0, 0, 0);
    REG_GET!(DMCUB_CNTL2, DMCUB_SOFT_RESET, &mut in_reset);
    REG_GET!(DMCUB_CNTL, DMCUB_ENABLE, &mut is_enabled);
    if in_reset == 0 && is_enabled != 0 {
        cmd.bits.status = 1; cmd.bits.command_code = DMUB_GPINT__STOP_FW; cmd.bits.param = 0;
        ((*dmub).hw_funcs.set_gpint)(dmub, cmd);
        for i in 0..timeout { if ((*dmub).hw_funcs.is_gpint_acked)(dmub, cmd) { break; } udelay!(1); }
        for i in 0..timeout { scratch = REG_READ!(DMCUB_SCRATCH7); if scratch == DMUB_GPINT__STOP_FW_RESPONSE { break; } udelay!(1); }
        for i in 0..timeout { REG_GET!(DMCUB_CNTL, DMCUB_PWAIT_MODE_STATUS, &mut pwait_mode); if pwait_mode & (1 << 0) != 0 { break; } udelay!(1); }
    }
    if is_enabled != 0 { REG_UPDATE!(DMCUB_CNTL2, DMCUB_SOFT_RESET, 1); udelay!(1); REG_UPDATE!(DMCUB_CNTL, DMCUB_ENABLE, 0); }
    REG_WRITE!(DMCUB_INBOX1_RPTR, 0); REG_WRITE!(DMCUB_INBOX1_WPTR, 0); REG_WRITE!(DMCUB_OUTBOX1_RPTR, 0); REG_WRITE!(DMCUB_OUTBOX1_WPTR, 0);
    REG_WRITE!(DMCUB_OUTBOX0_RPTR, 0); REG_WRITE!(DMCUB_OUTBOX0_WPTR, 0); REG_WRITE!(DMCUB_SCRATCH0, 0);
    cmd.all = 0; ((*dmub).hw_funcs.set_gpint)(dmub, cmd);
}

pub unsafe fn dmub_dcn35_reset_release(dmub: *mut dmub_srv) {
    REG_WRITE!(DMCUB_SCRATCH15, (*dmub).psp_version & 0x001100FF);
    REG_UPDATE_3!(DMU_CLK_CNTL, LONO_DISPCLK_GATE_DISABLE, 1, LONO_SOCCLK_GATE_DISABLE, 1, LONO_DMCUBCLK_GATE_DISABLE, 1);
    REG_UPDATE_2!(DMCUB_CNTL, DMCUB_ENABLE, 1, DMCUB_TRACEPORT_EN, 1); REG_UPDATE!(MMHUBBUB_SOFT_RESET, DMUIF_SOFT_RESET, 0); REG_UPDATE!(DMCUB_CNTL2, DMCUB_SOFT_RESET, 0);
}

pub unsafe fn dmub_dcn35_backdoor_load(dmub: *mut dmub_srv, cw0: *const dmub_window, cw1: *const dmub_window) {
    let (mut fb_base, mut fb_offset) = (0u64, 0u64); let mut offset: dmub_addr = core::mem::zeroed();
    dmub_dcn35_get_fb_base_offset(dmub, &mut fb_base, &mut fb_offset); dmub_dcn35_translate_addr(&(*cw0).offset, fb_base, fb_offset, &mut offset);
    REG_WRITE!(DMCUB_REGION3_CW0_OFFSET, offset.u.low_part); REG_WRITE!(DMCUB_REGION3_CW0_OFFSET_HIGH, offset.u.high_part); REG_WRITE!(DMCUB_REGION3_CW0_BASE_ADDRESS, (*cw0).region.base);
    REG_SET_2!(DMCUB_REGION3_CW0_TOP_ADDRESS, 0, DMCUB_REGION3_CW0_TOP_ADDRESS, (*cw0).region.top, DMCUB_REGION3_CW0_ENABLE, 1);
    dmub_dcn35_translate_addr(&(*cw1).offset, fb_base, fb_offset, &mut offset); REG_WRITE!(DMCUB_REGION3_CW1_OFFSET, offset.u.low_part); REG_WRITE!(DMCUB_REGION3_CW1_OFFSET_HIGH, offset.u.high_part); REG_WRITE!(DMCUB_REGION3_CW1_BASE_ADDRESS, (*cw1).region.base);
    REG_SET_2!(DMCUB_REGION3_CW1_TOP_ADDRESS, 0, DMCUB_REGION3_CW1_TOP_ADDRESS, (*cw1).region.top, DMCUB_REGION3_CW1_ENABLE, 1); REG_UPDATE!(DMCUB_SEC_CNTL, DMCUB_SEC_RESET, 0);
}

pub unsafe fn dmub_dcn35_backdoor_load_zfb_mode(dmub: *mut dmub_srv, cw0: *const dmub_window, cw1: *const dmub_window) {
    REG_UPDATE!(DMCUB_SEC_CNTL, DMCUB_SEC_RESET, 1); let mut offset = (*cw0).offset;
    REG_WRITE!(DMCUB_REGION3_CW0_OFFSET, offset.u.low_part); REG_WRITE!(DMCUB_REGION3_CW0_OFFSET_HIGH, offset.u.high_part); REG_WRITE!(DMCUB_REGION3_CW0_BASE_ADDRESS, (*cw0).region.base); REG_SET_2!(DMCUB_REGION3_CW0_TOP_ADDRESS, 0, DMCUB_REGION3_CW0_TOP_ADDRESS, (*cw0).region.top, DMCUB_REGION3_CW0_ENABLE, 1);
    offset = (*cw1).offset; REG_WRITE!(DMCUB_REGION3_CW1_OFFSET, offset.u.low_part); REG_WRITE!(DMCUB_REGION3_CW1_OFFSET_HIGH, offset.u.high_part); REG_WRITE!(DMCUB_REGION3_CW1_BASE_ADDRESS, (*cw1).region.base); REG_SET_2!(DMCUB_REGION3_CW1_TOP_ADDRESS, 0, DMCUB_REGION3_CW1_TOP_ADDRESS, (*cw1).region.top, DMCUB_REGION3_CW1_ENABLE, 1); REG_UPDATE_2!(DMCUB_SEC_CNTL, DMCUB_SEC_RESET, 0, DMCUB_MEM_UNIT_ID, 0x20);
}

// The remaining routines are direct register wrappers and diagnostic field copies.
pub unsafe fn dmub_dcn35_setup_windows(dmub: *mut dmub_srv, cw2: *const dmub_window, cw3: *const dmub_window, cw4: *const dmub_window, cw5: *const dmub_window, cw6: *const dmub_window, region6: *const dmub_window) {
    let _ = cw2; let mut o = (*cw3).offset;
    macro_rules! win { ($n:ident, $w:expr) => { REG_WRITE!(DMCUB_REGION3_$n##_OFFSET, o.u.low_part); REG_WRITE!(DMCUB_REGION3_$n##_OFFSET_HIGH, o.u.high_part); REG_WRITE!(DMCUB_REGION3_$n##_BASE_ADDRESS, (*$w).region.base); REG_SET_2!(DMCUB_REGION3_$n##_TOP_ADDRESS, 0, DMCUB_REGION3_$n##_TOP_ADDRESS, (*$w).region.top, DMCUB_REGION3_$n##_ENABLE, 1); }; }
    win!(CW3, cw3); o = (*cw4).offset; win!(CW4, cw4); o = (*cw5).offset; win!(CW5, cw5); o = (*cw6).offset; win!(CW6, cw6);
    o = (*cw5).offset; REG_WRITE!(DMCUB_REGION5_OFFSET, o.u.low_part); REG_WRITE!(DMCUB_REGION5_OFFSET_HIGH, o.u.high_part); REG_SET_2!(DMCUB_REGION5_TOP_ADDRESS, 0, DMCUB_REGION5_TOP_ADDRESS, (*cw5).region.top - (*cw5).region.base - 1, DMCUB_REGION5_ENABLE, 1);
    o = (*region6).offset; REG_WRITE!(DMCUB_REGION6_OFFSET, o.u.low_part); REG_WRITE!(DMCUB_REGION6_OFFSET_HIGH, o.u.high_part); REG_SET_2!(DMCUB_REGION6_TOP_ADDRESS, 0, DMCUB_REGION6_TOP_ADDRESS, (*region6).region.top - (*region6).region.base - 1, DMCUB_REGION6_ENABLE, 1);
}

pub unsafe fn dmub_dcn35_setup_mailbox(dmub: *mut dmub_srv, r: *const dmub_region) { REG_WRITE!(DMCUB_INBOX1_BASE_ADDRESS, (*r).base); REG_WRITE!(DMCUB_INBOX1_SIZE, (*r).top - (*r).base); }
pub unsafe fn dmub_dcn35_get_inbox1_wptr(dmub: *mut dmub_srv) -> u32 { REG_READ!(DMCUB_INBOX1_WPTR) }
pub unsafe fn dmub_dcn35_get_inbox1_rptr(dmub: *mut dmub_srv) -> u32 { REG_READ!(DMCUB_INBOX1_RPTR) }
pub unsafe fn dmub_dcn35_set_inbox1_wptr(dmub: *mut dmub_srv, v: u32) { REG_WRITE!(DMCUB_INBOX1_WPTR, v); }
pub unsafe fn dmub_dcn35_setup_out_mailbox(dmub: *mut dmub_srv, r: *const dmub_region) { REG_WRITE!(DMCUB_OUTBOX1_BASE_ADDRESS, (*r).base); REG_WRITE!(DMCUB_OUTBOX1_SIZE, (*r).top - (*r).base); }
pub unsafe fn dmub_dcn35_get_outbox1_wptr(dmub: *mut dmub_srv) -> u32 { REG_READ!(DMCUB_OUTBOX1_WPTR) }
pub unsafe fn dmub_dcn35_set_outbox1_rptr(dmub: *mut dmub_srv, v: u32) { REG_WRITE!(DMCUB_OUTBOX1_RPTR, v); }
pub unsafe fn dmub_dcn35_setup_outbox0(dmub: *mut dmub_srv, r: *const dmub_region) { REG_WRITE!(DMCUB_OUTBOX0_BASE_ADDRESS, (*r).base); REG_WRITE!(DMCUB_OUTBOX0_SIZE, (*r).top - (*r).base); }
pub unsafe fn dmub_dcn35_get_outbox0_wptr(dmub: *mut dmub_srv) -> u32 { REG_READ!(DMCUB_OUTBOX0_WPTR) }
pub unsafe fn dmub_dcn35_set_outbox0_rptr(dmub: *mut dmub_srv, v: u32) { REG_WRITE!(DMCUB_OUTBOX0_RPTR, v); }
pub unsafe fn dmub_dcn35_get_current_time(dmub: *mut dmub_srv) -> u32 { REG_READ!(DMCUB_TIMER_CURRENT) }

pub unsafe fn dmub_dcn35_is_hw_init(dmub: *mut dmub_srv) -> bool { let mut e=0; let mut s: dmub_fw_boot_status=core::mem::zeroed(); s.all=REG_READ!(DMCUB_SCRATCH0); REG_GET!(DMCUB_CNTL,DMCUB_ENABLE,&mut e); e != 0 && s.bits.dal_fw }
pub unsafe fn dmub_dcn35_is_supported(dmub: *mut dmub_srv) -> bool { let mut x=0; REG_GET!(CC_DC_PIPE_DIS,DC_DMCUB_ENABLE,&mut x); x != 0 }
pub unsafe fn dmub_dcn35_set_gpint(dmub: *mut dmub_srv, r: dmub_gpint_data_register) { REG_WRITE!(DMCUB_GPINT_DATAIN1,r.all); }
pub unsafe fn dmub_dcn35_is_gpint_acked(dmub: *mut dmub_srv, mut r: dmub_gpint_data_register) -> bool { let mut t: dmub_gpint_data_register=core::mem::zeroed(); r.bits.status=0; t.all=REG_READ!(DMCUB_GPINT_DATAIN1); t.all==r.all }
pub unsafe fn dmub_dcn35_get_gpint_response(dmub: *mut dmub_srv)->u32 { REG_READ!(DMCUB_SCRATCH7) }
pub unsafe fn dmub_dcn35_get_gpint_dataout(dmub: *mut dmub_srv)->u32 { let x=REG_READ!(DMCUB_GPINT_DATAOUT); REG_UPDATE!(DMCUB_INTERRUPT_ENABLE,DMCUB_GPINT_IH_INT_EN,0); REG_WRITE!(DMCUB_GPINT_DATAOUT,0); REG_UPDATE!(DMCUB_INTERRUPT_ACK,DMCUB_GPINT_IH_INT_ACK,1); REG_UPDATE!(DMCUB_INTERRUPT_ACK,DMCUB_GPINT_IH_INT_ACK,0); REG_UPDATE!(DMCUB_INTERRUPT_ENABLE,DMCUB_GPINT_IH_INT_EN,1); x }
pub unsafe fn dmub_dcn35_get_fw_boot_status(dmub:*mut dmub_srv)->dmub_fw_boot_status { let mut x=core::mem::zeroed(); x.all=REG_READ!(DMCUB_SCRATCH0); x }
pub unsafe fn dmub_dcn35_get_fw_boot_option(dmub:*mut dmub_srv)->dmub_fw_boot_options { let mut x=core::mem::zeroed(); x.all=REG_READ!(DMCUB_SCRATCH14); x }
pub unsafe fn dmub_dcn35_enable_dmub_boot_options(dmub:*mut dmub_srv, p:*const dmub_srv_hw_params) {
    let mut b:dmub_fw_boot_options=core::mem::zeroed();
    if !(*dmub).dpia_supported { (*dmub).dpia_supported=dmub_dcn35_get_fw_boot_option(dmub).bits.enable_dpia != 0; }
    b.bits.z10_disable=(*p).disable_z10; b.bits.dpia_supported=(*p).dpia_supported; b.bits.enable_dpia=(*dmub).dpia_supported && !(*p).disable_dpia;
    b.bits.usb4_cm_version=(*p).usb4_cm_version; b.bits.dpia_hpd_int_enable_supported=(*p).dpia_hpd_int_enable_supported; b.bits.power_optimization=(*p).power_optimization;
    b.bits.disable_clk_ds=(*p).disallow_dispclk_dppclk_ds; b.bits.disable_clk_gate=(*p).disable_clock_gate; b.bits.ips_disable=(*p).disable_ips; b.bits.ips_sequential_ono=(*p).ips_sequential_ono;
    b.bits.disable_sldo_opt=(*p).disable_sldo_opt; b.bits.enable_non_transparent_setconfig=(*p).enable_non_transparent_setconfig; b.bits.lower_hbr3_phy_ssc=(*p).lower_hbr3_phy_ssc; b.bits.disable_dpia_bw_allocation=(*p).disable_dpia_bw_allocation;
    let old=dmub_dcn35_get_fw_boot_option(dmub); b.bits.bootcrc_en_at_preos=old.bits.bootcrc_en_at_preos; b.bits.bootcrc_en_at_S0i3=old.bits.bootcrc_en_at_S0i3; b.bits.bootcrc_boot_mode=old.bits.bootcrc_boot_mode; REG_WRITE!(DMCUB_SCRATCH14,b.all);
}
pub unsafe fn dmub_dcn35_skip_dmub_panel_power_sequence(dmub:*mut dmub_srv, skip:bool) { let mut x:dmub_fw_boot_options=core::mem::zeroed(); x.all=REG_READ!(DMCUB_SCRATCH14); x.bits.skip_phy_init_panel_sequence=skip; REG_WRITE!(DMCUB_SCRATCH14,x.all); }
pub unsafe fn dmub_dcn35_get_preos_fw_info(dmub:*mut dmub_srv)->bool { let v=REG_READ!(DMCUB_SCRATCH1); if ((v>>6)&1)==0 { return false; } (*dmub).preos_info.boot_status=REG_READ!(DMCUB_SCRATCH0); (*dmub).preos_info.fw_version=v; (*dmub).preos_info.boot_options=REG_READ!(DMCUB_SCRATCH14); true }
pub unsafe fn dmub_dcn35_configure_dmub_in_system_memory(dmub:*mut dmub_srv) { REG_WRITE!(DMCUB_REGION3_TMR_AXI_SPACE,0x4); }
pub unsafe fn dmub_dcn35_should_detect(dmub:*mut dmub_srv)->bool { REG_READ!(DMCUB_SCRATCH0)&DMUB_FW_BOOT_STATUS_BIT_DETECTION_REQUIRED != 0 }
pub unsafe fn dmub_dcn35_send_inbox0_cmd(dmub:*mut dmub_srv, data:dmub_inbox0_data_register) { REG_WRITE!(DMCUB_INBOX0_WPTR,data.inbox0_cmd_common.all); }
pub unsafe fn dmub_dcn35_clear_inbox0_ack_register(dmub:*mut dmub_srv) { REG_WRITE!(DMCUB_SCRATCH17,0); }
pub unsafe fn dmub_dcn35_read_inbox0_ack_register(dmub:*mut dmub_srv)->u32 { REG_READ!(DMCUB_SCRATCH17) }
pub unsafe fn dmub_dcn35_is_hw_powered_up(dmub:*mut dmub_srv)->bool { let mut e=0; REG_GET!(DMCUB_CNTL,DMCUB_ENABLE,&mut e); if e==0{return false;} let mut s:dmub_fw_boot_status=core::mem::zeroed(); s.all=REG_READ!(DMCUB_SCRATCH0); (s.bits.dal_fw&&s.bits.hw_power_init_done&&s.bits.mailbox_rdy)||(!s.bits.dal_fw&&s.bits.mailbox_rdy) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
