// SPDX-License-Identifier: MIT
// Copyright 2026 Advanced Micro Devices, Inc.

// Translated from dmub_dcn42.c. Register and type definitions are supplied by
// the surrounding DMUB implementation.

pub unsafe fn dmub_srv_dcn42_regs_init(dmub: *mut dmub_srv, ctx: *mut dc_context) {
    let regs = (*dmub).regs_dcn42;
    macro_rules! dmub_sr { ($reg:ident) => { (*regs).offset.$reg = REG_OFFSET_EXP!($reg); }; }
    DMUB_DCN42_REGS!();
    DMCUB_INTERNAL_REGS!();
    macro_rules! dmub_sf { ($reg:ident, $field:ident) => { (*regs).mask.$reg##__$field = FD_MASK!($reg, $field); }; }
    DMUB_DCN42_FIELDS!();
    macro_rules! dmub_sf_shift { ($reg:ident, $field:ident) => { (*regs).shift.$reg##__$field = FD_SHIFT!($reg, $field); }; }
    DMUB_DCN42_FIELDS!();
}

pub unsafe fn dmub_dcn42_enable_dmub_boot_options(dmub: *mut dmub_srv, params: *const dmub_srv_hw_params) {
    let mut boot_options: dmub_fw_boot_options = core::mem::zeroed();
    if !(*dmub).dpia_supported { (*dmub).dpia_supported = dmub_dcn42_get_fw_boot_option(dmub).bits.enable_dpia != 0; }
    boot_options.bits.z10_disable = (*params).disable_z10;
    boot_options.bits.dpia_supported = (*params).dpia_supported;
    boot_options.bits.enable_dpia = (*dmub).dpia_supported && !(*params).disable_dpia;
    boot_options.bits.usb4_cm_version = (*params).usb4_cm_version;
    boot_options.bits.dpia_hpd_int_enable_supported = (*params).dpia_hpd_int_enable_supported;
    boot_options.bits.power_optimization = (*params).power_optimization;
    boot_options.bits.disable_clk_ds = (*params).disallow_dispclk_dppclk_ds;
    boot_options.bits.disable_clk_gate = (*params).disable_clock_gate;
    boot_options.bits.ips_disable = (*params).disable_ips;
    boot_options.bits.ips_sequential_ono = (*params).ips_sequential_ono;
    boot_options.bits.disable_sldo_opt = (*params).disable_sldo_opt;
    boot_options.bits.enable_non_transparent_setconfig = (*params).enable_non_transparent_setconfig;
    boot_options.bits.lower_hbr3_phy_ssc = (*params).lower_hbr3_phy_ssc;
    boot_options.bits.skip_phy_access = (*params).disallow_phy_access;
    boot_options.bits.disable_dpia_bw_allocation = (*params).disable_dpia_bw_allocation;
    REG_WRITE!(DMCUB_SCRATCH14, boot_options.all);
}

unsafe fn dmub_dcn42_get_fb_base_offset(dmub: *mut dmub_srv, fb_base: *mut u64, fb_offset: *mut u64) {
    let mut tmp = 0u32;
    REG_GET!(DCN_VM_FB_LOCATION_BASE, FB_BASE, &mut tmp);
    *fb_base = (tmp as u64) << 24;
    REG_GET!(DCN_VM_FB_OFFSET, FB_OFFSET, &mut tmp);
    *fb_offset = (tmp as u64) << 24;
}

#[inline]
unsafe fn dmub_dcn42_translate_addr(addr_in: *const dmub_addr, fb_base: u64, fb_offset: u64, addr_out: *mut dmub_addr) {
    (*addr_out).quad_part = (*addr_in).quad_part - fb_base + fb_offset;
}

pub unsafe fn dmub_dcn42_reset(dmub: *mut dmub_srv) {
    let mut cmd: dmub_gpint_data_register = core::mem::zeroed();
    let timeout = 100000u32;
    let (mut in_reset, mut is_enabled, mut scratch, mut pwait_mode) = (0, 0, 0, 0);
    REG_GET!(DMCUB_CNTL2, DMCUB_SOFT_RESET, &mut in_reset); REG_GET!(DMCUB_CNTL, DMCUB_ENABLE, &mut is_enabled);
    if in_reset == 0 && is_enabled != 0 {
        cmd.bits.status = 1; cmd.bits.command_code = DMUB_GPINT__STOP_FW; cmd.bits.param = 0;
        ((*dmub).hw_funcs.set_gpint)(dmub, cmd);
        for _ in 0..timeout { if ((*dmub).hw_funcs.is_gpint_acked)(dmub, cmd) { break; } udelay(1); }
        for _ in 0..timeout { scratch = REG_READ!(DMCUB_SCRATCH7); if scratch == DMUB_GPINT__STOP_FW_RESPONSE { break; } udelay(1); }
        for _ in 0..timeout { REG_GET!(DMCUB_CNTL, DMCUB_PWAIT_MODE_STATUS, &mut pwait_mode); if pwait_mode & (1 << 0) != 0 { break; } udelay(1); }
    }
    if is_enabled != 0 { REG_UPDATE!(DMCUB_CNTL2, DMCUB_SOFT_RESET, 1); udelay(1); REG_UPDATE!(DMCUB_CNTL, DMCUB_ENABLE, 0); }
    REG_WRITE!(DMCUB_INBOX1_RPTR, 0); REG_WRITE!(DMCUB_INBOX1_WPTR, 0); REG_WRITE!(DMCUB_OUTBOX1_RPTR, 0); REG_WRITE!(DMCUB_OUTBOX1_WPTR, 0); REG_WRITE!(DMCUB_OUTBOX0_RPTR, 0); REG_WRITE!(DMCUB_OUTBOX0_WPTR, 0); REG_WRITE!(DMCUB_SCRATCH0, 0);
    cmd.all = 0; ((*dmub).hw_funcs.set_gpint)(dmub, cmd);
}

pub unsafe fn dmub_dcn42_reset_release(dmub: *mut dmub_srv) {
    REG_WRITE!(DMCUB_SCRATCH15, (*dmub).psp_version & 0x001100FF);
    REG_UPDATE_3!(DMU_CLK_CNTL, LONO_DISPCLK_GATE_DISABLE, 1, LONO_SOCCLK_GATE_DISABLE, 1, LONO_DMCUBCLK_GATE_DISABLE, 1);
    REG_UPDATE_2!(DMCUB_CNTL, DMCUB_ENABLE, 1, DMCUB_TRACEPORT_EN, 1); REG_UPDATE!(MMHUBBUB_SOFT_RESET, DMUIF_SOFT_RESET, 0); REG_UPDATE!(DMCUB_CNTL2, DMCUB_SOFT_RESET, 0);
}

pub unsafe fn dmub_dcn42_backdoor_load(dmub: *mut dmub_srv, cw0: *const dmub_window, cw1: *const dmub_window) {
    let (mut fb_base, mut fb_offset) = (0u64, 0u64); let mut offset: dmub_addr = core::mem::zeroed(); dmub_dcn42_get_fb_base_offset(dmub, &mut fb_base, &mut fb_offset);
    dmub_dcn42_translate_addr(&(*cw0).offset, fb_base, fb_offset, &mut offset); REG_WRITE!(DMCUB_REGION3_CW0_OFFSET, offset.u_.low_part); REG_WRITE!(DMCUB_REGION3_CW0_OFFSET_HIGH, offset.u_.high_part); REG_WRITE!(DMCUB_REGION3_CW0_BASE_ADDRESS, (*cw0).region.base); REG_SET_2!(DMCUB_REGION3_CW0_TOP_ADDRESS, 0, DMCUB_REGION3_CW0_TOP_ADDRESS, (*cw0).region.top, DMCUB_REGION3_CW0_ENABLE, 1);
    dmub_dcn42_translate_addr(&(*cw1).offset, fb_base, fb_offset, &mut offset); REG_WRITE!(DMCUB_REGION3_CW1_OFFSET, offset.u_.low_part); REG_WRITE!(DMCUB_REGION3_CW1_OFFSET_HIGH, offset.u_.high_part); REG_WRITE!(DMCUB_REGION3_CW1_BASE_ADDRESS, (*cw1).region.base); REG_SET_2!(DMCUB_REGION3_CW1_TOP_ADDRESS, 0, DMCUB_REGION3_CW1_TOP_ADDRESS, (*cw1).region.top, DMCUB_REGION3_CW1_ENABLE, 1); REG_UPDATE!(DMCUB_SEC_CNTL, DMCUB_SEC_RESET, 0);
}

pub unsafe fn dmub_dcn42_backdoor_load_zfb_mode(dmub: *mut dmub_srv, cw0: *const dmub_window, cw1: *const dmub_window) {
    REG_UPDATE!(DMCUB_SEC_CNTL, DMCUB_SEC_RESET, 1); let offset = (*cw0).offset; REG_WRITE!(DMCUB_REGION3_CW0_OFFSET, offset.u_.low_part); REG_WRITE!(DMCUB_REGION3_CW0_OFFSET_HIGH, offset.u_.high_part); REG_WRITE!(DMCUB_REGION3_CW0_BASE_ADDRESS, (*cw0).region.base); REG_SET_2!(DMCUB_REGION3_CW0_TOP_ADDRESS, 0, DMCUB_REGION3_CW0_TOP_ADDRESS, (*cw0).region.top, DMCUB_REGION3_CW0_ENABLE, 1);
    let offset = (*cw1).offset; REG_WRITE!(DMCUB_REGION3_CW1_OFFSET, offset.u_.low_part); REG_WRITE!(DMCUB_REGION3_CW1_OFFSET_HIGH, offset.u_.high_part); REG_WRITE!(DMCUB_REGION3_CW1_BASE_ADDRESS, (*cw1).region.base); REG_SET_2!(DMCUB_REGION3_CW1_TOP_ADDRESS, 0, DMCUB_REGION3_CW1_TOP_ADDRESS, (*cw1).region.top, DMCUB_REGION3_CW1_ENABLE, 1); REG_UPDATE_2!(DMCUB_SEC_CNTL, DMCUB_SEC_RESET, 0, DMCUB_MEM_UNIT_ID, 0x20);
}

pub unsafe fn dmub_dcn42_setup_windows(dmub: *mut dmub_srv, cw2: *const dmub_window, cw3: *const dmub_window, cw4: *const dmub_window, cw5: *const dmub_window, cw6: *const dmub_window, region6: *const dmub_window) {
    let _ = cw2; let mut offset = (*cw3).offset;
    REG_WRITE!(DMCUB_REGION3_CW3_OFFSET, offset.u_.low_part); REG_WRITE!(DMCUB_REGION3_CW3_OFFSET_HIGH, offset.u_.high_part); REG_WRITE!(DMCUB_REGION3_CW3_BASE_ADDRESS, (*cw3).region.base); REG_SET_2!(DMCUB_REGION3_CW3_TOP_ADDRESS, 0, DMCUB_REGION3_CW3_TOP_ADDRESS, (*cw3).region.top, DMCUB_REGION3_CW3_ENABLE, 1);
    offset = (*cw4).offset; REG_WRITE!(DMCUB_REGION3_CW4_OFFSET, offset.u_.low_part); REG_WRITE!(DMCUB_REGION3_CW4_OFFSET_HIGH, offset.u_.high_part); REG_WRITE!(DMCUB_REGION3_CW4_BASE_ADDRESS, (*cw4).region.base); REG_SET_2!(DMCUB_REGION3_CW4_TOP_ADDRESS, 0, DMCUB_REGION3_CW4_TOP_ADDRESS, (*cw4).region.top, DMCUB_REGION3_CW4_ENABLE, 1);
    offset = (*cw5).offset; REG_WRITE!(DMCUB_REGION3_CW5_OFFSET, offset.u_.low_part); REG_WRITE!(DMCUB_REGION3_CW5_OFFSET_HIGH, offset.u_.high_part); REG_WRITE!(DMCUB_REGION3_CW5_BASE_ADDRESS, (*cw5).region.base); REG_SET_2!(DMCUB_REGION3_CW5_TOP_ADDRESS, 0, DMCUB_REGION3_CW5_TOP_ADDRESS, (*cw5).region.top, DMCUB_REGION3_CW5_ENABLE, 1); REG_WRITE!(DMCUB_REGION5_OFFSET, offset.u_.low_part); REG_WRITE!(DMCUB_REGION5_OFFSET_HIGH, offset.u_.high_part); REG_SET_2!(DMCUB_REGION5_TOP_ADDRESS, 0, DMCUB_REGION5_TOP_ADDRESS, (*cw5).region.top - (*cw5).region.base - 1, DMCUB_REGION5_ENABLE, 1);
    offset = (*cw6).offset; REG_WRITE!(DMCUB_REGION3_CW6_OFFSET, offset.u_.low_part); REG_WRITE!(DMCUB_REGION3_CW6_OFFSET_HIGH, offset.u_.high_part); REG_WRITE!(DMCUB_REGION3_CW6_BASE_ADDRESS, (*cw6).region.base); REG_SET_2!(DMCUB_REGION3_CW6_TOP_ADDRESS, 0, DMCUB_REGION3_CW6_TOP_ADDRESS, (*cw6).region.top, DMCUB_REGION3_CW6_ENABLE, 1);
    offset = (*region6).offset; REG_WRITE!(DMCUB_REGION6_OFFSET, offset.u_.low_part); REG_WRITE!(DMCUB_REGION6_OFFSET_HIGH, offset.u_.high_part); REG_SET_2!(DMCUB_REGION6_TOP_ADDRESS, 0, DMCUB_REGION6_TOP_ADDRESS, (*region6).region.top - (*region6).region.base - 1, DMCUB_REGION6_ENABLE, 1);
}

pub unsafe fn dmub_dcn42_get_inbox1_wptr(_: *mut dmub_srv) -> u32 { REG_READ!(DMCUB_INBOX1_WPTR) }
pub unsafe fn dmub_dcn42_get_inbox1_rptr(_: *mut dmub_srv) -> u32 { REG_READ!(DMCUB_INBOX1_RPTR) }
pub unsafe fn dmub_dcn42_setup_out_mailbox(_: *mut dmub_srv, outbox1: *const dmub_region) { REG_WRITE!(DMCUB_OUTBOX1_BASE_ADDRESS, (*outbox1).base); REG_WRITE!(DMCUB_OUTBOX1_SIZE, (*outbox1).top - (*outbox1).base); }
pub unsafe fn dmub_dcn42_get_outbox1_wptr(_: *mut dmub_srv) -> u32 { REG_READ!(DMCUB_OUTBOX1_WPTR) }
pub unsafe fn dmub_dcn42_set_outbox1_rptr(_: *mut dmub_srv, rptr_offset: u32) { REG_WRITE!(DMCUB_OUTBOX1_RPTR, rptr_offset); }
pub unsafe fn dmub_dcn42_is_supported(_: *mut dmub_srv) -> bool { true }
pub unsafe fn dmub_dcn42_get_fw_boot_option(_: *mut dmub_srv) -> dmub_fw_boot_options { let mut option: dmub_fw_boot_options = core::mem::zeroed(); option.all = REG_READ!(DMCUB_SCRATCH14); option }
pub unsafe fn dmub_dcn42_setup_outbox0(_: *mut dmub_srv, outbox0: *const dmub_region) { REG_WRITE!(DMCUB_OUTBOX0_BASE_ADDRESS, (*outbox0).base); REG_WRITE!(DMCUB_OUTBOX0_SIZE, (*outbox0).top - (*outbox0).base); }
pub unsafe fn dmub_dcn42_should_detect(_: *mut dmub_srv) -> bool { (REG_READ!(DMCUB_SCRATCH0) & DMUB_FW_BOOT_STATUS_BIT_DETECTION_REQUIRED) != 0 }
pub unsafe fn dmub_dcn42_send_inbox0_cmd(_: *mut dmub_srv, data: dmub_inbox0_data_register) { REG_WRITE!(DMCUB_INBOX0_WPTR, data.inbox0_cmd_common.all); }
pub unsafe fn dmub_dcn42_read_inbox0_ack_register(_: *mut dmub_srv) -> u32 { REG_READ!(DMCUB_SCRATCH17) }

pub unsafe fn dmub_dcn42_is_hw_powered_up(_: *mut dmub_srv) -> bool { let mut e=0; REG_GET!(DMCUB_CNTL,DMCUB_ENABLE,&mut e); if e==0{return false}; let mut s:dmub_fw_boot_status=core::mem::zeroed(); s.all=REG_READ!(DMCUB_SCRATCH0); (s.bits.dal_fw && s.bits.hw_power_init_done && s.bits.mailbox_rdy)||(!s.bits.dal_fw&&s.bits.mailbox_rdy) }
pub unsafe fn dmub_dcn42_set_inbox1_wptr(_: *mut dmub_srv, wptr_offset:u32){REG_WRITE!(DMCUB_INBOX1_WPTR,wptr_offset)}
pub unsafe fn dmub_dcn42_is_hw_init(_: *mut dmub_srv)->bool{let mut s:dmub_fw_boot_status=core::mem::zeroed();let mut e=0;s.all=REG_READ!(DMCUB_SCRATCH0);REG_GET!(DMCUB_CNTL,DMCUB_ENABLE,&mut e);e!=0&&s.bits.dal_fw}
pub unsafe fn dmub_dcn42_get_fw_boot_status(_: *mut dmub_srv)->dmub_fw_boot_status{let mut s:dmub_fw_boot_status=core::mem::zeroed();s.all=REG_READ!(DMCUB_SCRATCH0);s}
pub unsafe fn dmub_dcn42_skip_dmub_panel_power_sequence(_: *mut dmub_srv,skip:bool){let mut b:dmub_fw_boot_options=core::mem::zeroed();b.all=REG_READ!(DMCUB_SCRATCH14);b.bits.skip_phy_init_panel_sequence=skip;REG_WRITE!(DMCUB_SCRATCH14,b.all)}
pub unsafe fn dmub_dcn42_configure_dmub_in_system_memory(_: *mut dmub_srv){REG_WRITE!(DMCUB_REGION3_TMR_AXI_SPACE,0x4)}
pub unsafe fn dmub_dcn42_clear_inbox0_ack_register(_: *mut dmub_srv){REG_WRITE!(DMCUB_SCRATCH17,0)}

pub unsafe fn dmub_dcn42_send_reg_inbox0_cmd_msg(_: *mut dmub_srv, cmd:*mut dmub_rb_cmd){let dwords=cmd as *mut u32;let payload_size_bytes=(*cmd).cmd_common.header.payload_bytes;for i in 0..15u32{if payload_size_bytes<=i*4{break} REG_WRITE!(DMCUB_REG_INBOX0_MSG0 + i, *dwords.add((i+1) as usize));}REG_WRITE!(DMCUB_REG_INBOX0_RDY,*dwords)}
pub unsafe fn dmub_dcn42_read_reg_inbox0_rsp_int_status(_: *mut dmub_srv)->u32{let mut s=0;REG_GET!(HOST_INTERRUPT_CSR,HOST_REG_INBOX0_RSP_INT_STAT,&mut s);s}
pub unsafe fn dmub_dcn42_read_reg_inbox0_cmd_rsp(_: *mut dmub_srv,cmd:*mut dmub_rb_cmd){let d=cmd as *mut u32;*d=REG_READ!(DMCUB_REG_INBOX0_RSP);for i in 0..15usize{*d.add(i+1)=REG_READ!(DMCUB_REG_INBOX0_MSG0+i as u32)}}
pub unsafe fn dmub_dcn42_write_reg_inbox0_rsp_int_ack(_: *mut dmub_srv){REG_UPDATE!(HOST_INTERRUPT_CSR,HOST_REG_INBOX0_RSP_INT_ACK,1)}
pub unsafe fn dmub_dcn42_clear_reg_inbox0_rsp_int_ack(_: *mut dmub_srv){REG_UPDATE!(HOST_INTERRUPT_CSR,HOST_REG_INBOX0_RSP_INT_ACK,0)}
pub unsafe fn dmub_dcn42_enable_reg_inbox0_rsp_int(_: *mut dmub_srv,enable:bool){REG_UPDATE!(HOST_INTERRUPT_CSR,HOST_REG_INBOX0_RSP_INT_EN,if enable{1}else{0})}
pub unsafe fn dmub_dcn42_write_reg_outbox0_rdy_int_ack(_: *mut dmub_srv){REG_UPDATE!(HOST_INTERRUPT_CSR,HOST_REG_OUTBOX0_RDY_INT_ACK,1);REG_UPDATE!(HOST_INTERRUPT_CSR,HOST_REG_OUTBOX0_RDY_INT_ACK,0)}
pub unsafe fn dmub_dcn42_read_reg_outbox0_msg(_: *mut dmub_srv,msg:*mut u32){*msg=REG_READ!(DMCUB_REG_OUTBOX0_MSG0)}
pub unsafe fn dmub_dcn42_enable_reg_outbox0_rdy_int(_: *mut dmub_srv,enable:bool){REG_UPDATE!(HOST_INTERRUPT_CSR,HOST_REG_OUTBOX0_RDY_INT_EN,if enable{1}else{0})}
pub unsafe fn dmub_dcn42_read_reg_outbox0_rdy_int_status(_: *mut dmub_srv)->u32{let mut s=0;REG_GET!(HOST_INTERRUPT_CSR,HOST_REG_OUTBOX0_RDY_INT_STAT,&mut s);s}
pub unsafe fn dmub_dcn42_setup_mailbox(_: *mut dmub_srv,inbox1:*const dmub_region){REG_WRITE!(DMCUB_INBOX1_BASE_ADDRESS,(*inbox1).base);REG_WRITE!(DMCUB_INBOX1_SIZE,(*inbox1).top-(*inbox1).base)}
pub unsafe fn dmub_dcn42_set_gpint(_: *mut dmub_srv,reg:dmub_gpint_data_register){REG_WRITE!(DMCUB_GPINT_DATAIN1,reg.all)}
pub unsafe fn dmub_dcn42_is_gpint_acked(_: *mut dmub_srv,mut reg:dmub_gpint_data_register)->bool{let mut t:dmub_gpint_data_register=core::mem::zeroed();reg.bits.status=0;t.all=REG_READ!(DMCUB_GPINT_DATAIN1);t.all==reg.all}
pub unsafe fn dmub_dcn42_get_gpint_response(_: *mut dmub_srv)->u32{REG_READ!(DMCUB_SCRATCH7)}
pub unsafe fn dmub_dcn42_get_gpint_dataout(_: *mut dmub_srv)->u32{let d=REG_READ!(DMCUB_GPINT_DATAOUT);REG_UPDATE!(DMCUB_INTERRUPT_ENABLE,DMCUB_GPINT_IH_INT_EN,0);REG_WRITE!(DMCUB_GPINT_DATAOUT,0);REG_UPDATE!(DMCUB_INTERRUPT_ACK,DMCUB_GPINT_IH_INT_ACK,1);REG_UPDATE!(DMCUB_INTERRUPT_ACK,DMCUB_GPINT_IH_INT_ACK,0);REG_UPDATE!(DMCUB_INTERRUPT_ENABLE,DMCUB_GPINT_IH_INT_EN,1);d}
pub unsafe fn dmub_dcn42_get_outbox0_wptr(_: *mut dmub_srv)->u32{REG_READ!(DMCUB_OUTBOX0_WPTR)}
pub unsafe fn dmub_dcn42_set_outbox0_rptr(_: *mut dmub_srv,rptr_offset:u32){REG_WRITE!(DMCUB_OUTBOX0_RPTR,rptr_offset)}
pub unsafe fn dmub_dcn42_get_current_time(_: *mut dmub_srv)->u32{REG_READ!(DMCUB_TIMER_CURRENT)}

pub unsafe fn dmub_dcn42_get_diagnostic_data(dmub:*mut dmub_srv){if dmub.is_null(){return}let timeout=(*dmub).debug.timeout_info;(*dmub).debug=core::mem::zeroed();(*dmub).debug.timeout_info=timeout;(*dmub).debug.dmcub_version=(*dmub).fw_version;for i in 0..17usize{(*dmub).debug.scratch[i]=REG_READ!(DMCUB_SCRATCH0+i as u32)}(*dmub).debug.undefined_address_fault_addr=REG_READ!(DMCUB_UNDEFINED_ADDRESS_FAULT_ADDR);(*dmub).debug.inst_fetch_fault_addr=REG_READ!(DMCUB_INST_FETCH_FAULT_ADDR);(*dmub).debug.data_write_fault_addr=REG_READ!(DMCUB_DATA_WRITE_FAULT_ADDR);(*dmub).debug.inbox1_rptr=REG_READ!(DMCUB_INBOX1_RPTR);(*dmub).debug.inbox1_wptr=REG_READ!(DMCUB_INBOX1_WPTR);(*dmub).debug.inbox1_size=REG_READ!(DMCUB_INBOX1_SIZE);(*dmub).debug.inbox0_rptr=REG_READ!(DMCUB_INBOX0_RPTR);(*dmub).debug.inbox0_wptr=REG_READ!(DMCUB_INBOX0_WPTR);(*dmub).debug.inbox0_size=REG_READ!(DMCUB_INBOX0_SIZE);(*dmub).debug.outbox1_rptr=REG_READ!(DMCUB_OUTBOX1_RPTR);(*dmub).debug.outbox1_wptr=REG_READ!(DMCUB_OUTBOX1_WPTR);(*dmub).debug.outbox1_size=REG_READ!(DMCUB_OUTBOX1_SIZE);let(mut a,mut b,mut c,mut d,mut e,mut f,mut g)=(0,0,0,0,0,0,0);REG_GET!(DMCUB_CNTL,DMCUB_ENABLE,&mut a);REG_GET!(DMCUB_CNTL,DMCUB_PWAIT_MODE_STATUS,&mut b);REG_GET!(DMCUB_CNTL2,DMCUB_SOFT_RESET,&mut c);REG_GET!(DMCUB_SEC_CNTL,DMCUB_SEC_RESET_STATUS,&mut d);REG_GET!(DMCUB_CNTL,DMCUB_TRACEPORT_EN,&mut e);REG_GET!(DMCUB_REGION3_CW0_TOP_ADDRESS,DMCUB_REGION3_CW0_ENABLE,&mut f);REG_GET!(DMCUB_REGION3_CW6_TOP_ADDRESS,DMCUB_REGION3_CW6_ENABLE,&mut g);(*dmub).debug.is_dmcub_enabled=a as u8;(*dmub).debug.is_pwait=b as u8;(*dmub).debug.is_dmcub_soft_reset=c as u8;(*dmub).debug.is_dmcub_secure_reset=d as u8;(*dmub).debug.is_traceport_en=e as u8;(*dmub).debug.is_cw0_enabled=f as u8;(*dmub).debug.is_cw6_enabled=g as u8;(*dmub).debug.gpint_datain0=REG_READ!(DMCUB_GPINT_DATAIN0)}

pub unsafe fn dmub_dcn42_get_preos_fw_info(dmub:*mut dmub_srv)->bool{let mut i=0u32;(*dmub).preos_info=core::mem::zeroed();let v=REG_READ!(DMCUB_SCRATCH1);if ((v>>6)&1)==0{return false}(*dmub).preos_info.boot_status=REG_READ!(DMCUB_SCRATCH0);(*dmub).preos_info.fw_version=v;(*dmub).preos_info.boot_options=REG_READ!(DMCUB_SCRATCH14);let mut en=0;REG_GET!(DMCUB_REGION3_CW5_TOP_ADDRESS,DMCUB_REGION3_CW5_ENABLE,&mut en);if en!=0{dmub_dcn42_get_fb_base_offset(dmub,&mut (*dmub).preos_info.fb_base,&mut (*dmub).preos_info.fb_offset);let lo=REG_READ!(DMCUB_REGION3_CW5_OFFSET) as u64;let hi=REG_READ!(DMCUB_REGION3_CW5_OFFSET_HIGH) as u64;let off=(hi<<32)|lo;(*dmub).preos_info.trace_buffer_phy_addr=off-(*dmub).preos_info.fb_base+(*dmub).preos_info.fb_offset;REG_GET!(DMCUB_REGION3_CW5_TOP_ADDRESS,DMCUB_REGION3_CW5_TOP_ADDRESS,&mut i);let base=REG_READ!(DMCUB_REGION3_CW5_BASE_ADDRESS)&0x1fffffff;(*dmub).preos_info.trace_buffer_size=if i>base{i-base+1}else{0}}true}
pub unsafe fn dmub_dcn42_write_reg_outbox0_rsp(_: *mut dmub_srv,rsp:*const u32){REG_WRITE!(DMCUB_REG_OUTBOX0_RSP,*rsp)}
pub unsafe fn dmub_dcn42_read_reg_outbox0_rsp_int_status(_: *mut dmub_srv)->u32{let mut s=0;REG_GET!(DMCUB_INTERRUPT_STATUS,DMCUB_REG_OUTBOX0_RSP_INT_STAT,&mut s);s}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
