// SPDX-License-Identifier: MIT
// Copyright 2024 Advanced Micro Devices, Inc.

// C dependencies supplied by the surrounding translation unit are intentionally
// left as external Rust symbols.

pub const DCN_BASE_INST0_SEG2: u32 = 0x000034C0;

pub static dmub_srv_dcn401_regs: dmub_srv_dcn401_regs = dmub_srv_dcn401_regs {
    regs: [DMUB_DCN401_REGS!(), DMCUB_INTERNAL_REGS!()],
    masks: [DMUB_DCN401_FIELDS_MASKS!()],
    shifts: [DMUB_DCN401_FIELDS_SHIFTS!()],
};

unsafe fn dmub_dcn401_get_fb_base_offset(dmub: *mut dmub_srv, fb_base: *mut u64, fb_offset: *mut u64) {
    let mut tmp: u32 = 0;
    if (*dmub).soc_fb_info.fb_base != 0 || (*dmub).soc_fb_info.fb_offset != 0 {
        *fb_base = (*dmub).soc_fb_info.fb_base;
        *fb_offset = (*dmub).soc_fb_info.fb_offset;
        return;
    }
    REG_GET!(dmub, DCN_VM_FB_LOCATION_BASE, FB_BASE, &mut tmp);
    *fb_base = (tmp as u64) << 24;
    REG_GET!(dmub, DCN_VM_FB_OFFSET, FB_OFFSET, &mut tmp);
    *fb_offset = (tmp as u64) << 24;
}

#[inline]
unsafe fn dmub_dcn401_translate_addr(addr_in: *const dmub_addr, fb_base: u64, fb_offset: u64, addr_out: *mut dmub_addr) {
    (*addr_out).quad_part = (*addr_in).quad_part.wrapping_sub(fb_base).wrapping_add(fb_offset);
}

pub unsafe fn dmub_dcn401_reset(dmub: *mut dmub_srv) {
    let mut cmd: dmub_gpint_data_register = core::mem::zeroed();
    let timeout_us: u32 = 1 * 1000 * 1000;
    let poll_delay_us: u32 = 1;
    let mut i: u32 = 0;
    let (mut enabled, mut in_reset, mut scratch, mut pwait_mode) = (0, 0, 0, 0);
    REG_GET!(dmub, DMCUB_CNTL, DMCUB_ENABLE, &mut enabled);
    REG_GET!(dmub, DMCUB_CNTL2, DMCUB_SOFT_RESET, &mut in_reset);
    if enabled != 0 && in_reset == 0 {
        cmd.bits.status = 1; cmd.bits.command_code = DMUB_GPINT__STOP_FW; cmd.bits.param = 0;
        (*dmub).hw_funcs.set_gpint(dmub, cmd);
        while i < timeout_us { scratch = REG_READ!(dmub, DMCUB_SCRATCH7); if scratch == DMUB_GPINT__STOP_FW_RESPONSE { break; } udelay(poll_delay_us); i += 1; }
        while i < timeout_us { REG_GET!(dmub, DMCUB_CNTL, DMCUB_PWAIT_MODE_STATUS, &mut pwait_mode); if pwait_mode & (1 << 0) != 0 { break; } udelay(poll_delay_us); i += 1; }
    }
    if enabled != 0 { REG_UPDATE!(dmub, DMCUB_CNTL2, DMCUB_SOFT_RESET, 1); udelay(1); REG_UPDATE!(dmub, DMCUB_CNTL, DMCUB_ENABLE, 0); }
    if i >= timeout_us { BREAK_TO_DEBUGGER!(); }
    REG_UPDATE!(dmub, DMCUB_REGION3_CW2_TOP_ADDRESS, DMCUB_REGION3_CW2_ENABLE, 0);
    REG_UPDATE!(dmub, DMCUB_REGION3_CW3_TOP_ADDRESS, DMCUB_REGION3_CW3_ENABLE, 0);
    REG_UPDATE!(dmub, DMCUB_REGION3_CW4_TOP_ADDRESS, DMCUB_REGION3_CW4_ENABLE, 0);
    REG_UPDATE!(dmub, DMCUB_REGION3_CW5_TOP_ADDRESS, DMCUB_REGION3_CW5_ENABLE, 0);
    REG_UPDATE!(dmub, DMCUB_REGION3_CW6_TOP_ADDRESS, DMCUB_REGION3_CW6_ENABLE, 0);
    REG_UPDATE!(dmub, DMCUB_REGION3_CW7_TOP_ADDRESS, DMCUB_REGION3_CW7_ENABLE, 0);
    for reg in [DMCUB_INBOX1_RPTR, DMCUB_INBOX1_WPTR, DMCUB_OUTBOX1_RPTR, DMCUB_OUTBOX1_WPTR, DMCUB_OUTBOX0_RPTR, DMCUB_OUTBOX0_WPTR, DMCUB_SCRATCH0] { REG_WRITE!(dmub, reg, 0); }
    cmd.all = 0; (*dmub).hw_funcs.set_gpint(dmub, cmd);
}

pub unsafe fn dmub_dcn401_reset_release(dmub: *mut dmub_srv) { REG_UPDATE!(dmub, MMHUBBUB_SOFT_RESET, DMUIF_SOFT_RESET, 0); REG_WRITE!(dmub, DMCUB_SCRATCH15, (*dmub).psp_version & 0x001100FF); REG_UPDATE_2!(dmub, DMCUB_CNTL, DMCUB_ENABLE, 1, DMCUB_TRACEPORT_EN, 1); REG_UPDATE!(dmub, DMCUB_CNTL2, DMCUB_SOFT_RESET, 0); }

pub unsafe fn dmub_dcn401_backdoor_load(dmub: *mut dmub_srv, cw0: *const dmub_window, cw1: *const dmub_window) { let mut off: dmub_addr = core::mem::zeroed(); let (mut fb, mut fbo)=(0,0); dmub_dcn401_get_fb_base_offset(dmub,&mut fb,&mut fbo); REG_UPDATE!(dmub,DMCUB_SEC_CNTL,DMCUB_SEC_RESET,1); REG_UPDATE!(dmub,DMCUB_CNTL,DMCUB_ENABLE,0); for (w,p) in [(cw0,0),(cw1,1)] { dmub_dcn401_translate_addr(&(*w).offset,fb,fbo,&mut off); REG_WRITE!(dmub, if p==0 {DMCUB_REGION3_CW0_OFFSET}else{DMCUB_REGION3_CW1_OFFSET},off.u.low_part); REG_WRITE!(dmub, if p==0 {DMCUB_REGION3_CW0_OFFSET_HIGH}else{DMCUB_REGION3_CW1_OFFSET_HIGH},off.u.high_part); } REG_UPDATE_2!(dmub,DMCUB_SEC_CNTL,DMCUB_SEC_RESET,0,DMCUB_MEM_UNIT_ID,0x20); }

pub unsafe fn dmub_dcn401_backdoor_load_zfb_mode(dmub:*mut dmub_srv,cw0:*const dmub_window,cw1:*const dmub_window){REG_UPDATE!(dmub,DMCUB_SEC_CNTL,DMCUB_SEC_RESET,1);REG_UPDATE!(dmub,DMCUB_CNTL,DMCUB_ENABLE,0);let _=(cw0,cw1);REG_UPDATE_2!(dmub,DMCUB_SEC_CNTL,DMCUB_SEC_RESET,0,DMCUB_MEM_UNIT_ID,0x20);}

pub unsafe fn dmub_dcn401_setup_windows(dmub:*mut dmub_srv,cw2:*const dmub_window,cw3:*const dmub_window,cw4:*const dmub_window,cw5:*const dmub_window,cw6:*const dmub_window,region6:*const dmub_window){let _=cw2; for (w,offreg,highreg,basereg,topreg,enreg) in [(cw3,DMCUB_REGION3_CW3_OFFSET,DMCUB_REGION3_CW3_OFFSET_HIGH,DMCUB_REGION3_CW3_BASE_ADDRESS,DMCUB_REGION3_CW3_TOP_ADDRESS,DMCUB_REGION3_CW3_ENABLE),(cw4,DMCUB_REGION3_CW4_OFFSET,DMCUB_REGION3_CW4_OFFSET_HIGH,DMCUB_REGION3_CW4_BASE_ADDRESS,DMCUB_REGION3_CW4_TOP_ADDRESS,DMCUB_REGION3_CW4_ENABLE),(cw5,DMCUB_REGION3_CW5_OFFSET,DMCUB_REGION3_CW5_OFFSET_HIGH,DMCUB_REGION3_CW5_BASE_ADDRESS,DMCUB_REGION3_CW5_TOP_ADDRESS,DMCUB_REGION3_CW5_ENABLE),(cw6,DMCUB_REGION3_CW6_OFFSET,DMCUB_REGION3_CW6_OFFSET_HIGH,DMCUB_REGION3_CW6_BASE_ADDRESS,DMCUB_REGION3_CW6_TOP_ADDRESS,DMCUB_REGION3_CW6_ENABLE)]{let o=(*w).offset;REG_WRITE!(dmub,offreg,o.u.low_part);REG_WRITE!(dmub,highreg,o.u.high_part);REG_WRITE!(dmub,basereg,(*w).region.base);REG_SET_2!(dmub,topreg,0,topreg,(*w).region.top,enreg,1);}let o=(*region6).offset;REG_WRITE!(dmub,DMCUB_REGION6_OFFSET,o.u.low_part);REG_WRITE!(dmub,DMCUB_REGION6_OFFSET_HIGH,o.u.high_part);REG_SET_2!(dmub,DMCUB_REGION6_TOP_ADDRESS,0,DMCUB_REGION6_TOP_ADDRESS,(*region6).region.top-(*region6).region.base-1,DMCUB_REGION6_ENABLE,1);}

pub unsafe fn dmub_dcn401_setup_mailbox(dmub:*mut dmub_srv,r:*const dmub_region){REG_WRITE!(dmub,DMCUB_INBOX1_BASE_ADDRESS,(*r).base);REG_WRITE!(dmub,DMCUB_INBOX1_SIZE,(*r).top-(*r).base);}
pub unsafe fn dmub_dcn401_get_inbox1_wptr(dmub:*mut dmub_srv)->u32{REG_READ!(dmub,DMCUB_INBOX1_WPTR)}
pub unsafe fn dmub_dcn401_get_inbox1_rptr(dmub:*mut dmub_srv)->u32{REG_READ!(dmub,DMCUB_INBOX1_RPTR)}
pub unsafe fn dmub_dcn401_set_inbox1_wptr(dmub:*mut dmub_srv,v:u32){REG_WRITE!(dmub,DMCUB_INBOX1_WPTR,v)}
pub unsafe fn dmub_dcn401_setup_out_mailbox(dmub:*mut dmub_srv,r:*const dmub_region){REG_WRITE!(dmub,DMCUB_OUTBOX1_BASE_ADDRESS,(*r).base);REG_WRITE!(dmub,DMCUB_OUTBOX1_SIZE,(*r).top-(*r).base);}
pub unsafe fn dmub_dcn401_get_outbox1_wptr(dmub:*mut dmub_srv)->u32{REG_READ!(dmub,DMCUB_OUTBOX1_WPTR)}
pub unsafe fn dmub_dcn401_set_outbox1_rptr(dmub:*mut dmub_srv,v:u32){REG_WRITE!(dmub,DMCUB_OUTBOX1_RPTR,v)}
pub unsafe fn dmub_dcn401_is_hw_init(dmub:*mut dmub_srv)->bool{let mut s:dmub_fw_boot_status=core::mem::zeroed();s.all=REG_READ!(dmub,DMCUB_SCRATCH0);let mut e=0;REG_GET!(dmub,DMCUB_CNTL,DMCUB_ENABLE,&mut e);e!=0&&s.bits.dal_fw}
pub unsafe fn dmub_dcn401_is_supported(dmub:*mut dmub_srv)->bool{let mut v=0;REG_GET!(dmub,CC_DC_PIPE_DIS,DC_DMCUB_ENABLE,&mut v);v!=0}
pub unsafe fn dmub_dcn401_set_gpint(dmub:*mut dmub_srv,r:dmub_gpint_data_register){REG_WRITE!(dmub,DMCUB_GPINT_DATAIN1,r.all)}
pub unsafe fn dmub_dcn401_get_gpint_response(dmub:*mut dmub_srv)->u32{REG_READ!(dmub,DMCUB_SCRATCH7)}
pub unsafe fn dmub_dcn401_get_current_time(dmub:*mut dmub_srv)->u32{REG_READ!(dmub,DMCUB_TIMER_CURRENT)}

pub unsafe fn dmub_dcn401_is_gpint_acked(dmub:*mut dmub_srv,mut r:dmub_gpint_data_register)->bool{let mut t:dmub_gpint_data_register=core::mem::zeroed();r.bits.status=0;t.all=REG_READ!(dmub,DMCUB_GPINT_DATAIN1);t.all==r.all}
pub unsafe fn dmub_dcn401_get_gpint_dataout(dmub:*mut dmub_srv)->u32{let v=REG_READ!(dmub,DMCUB_GPINT_DATAOUT);REG_UPDATE!(dmub,DMCUB_INTERRUPT_ENABLE,DMCUB_GPINT_IH_INT_EN,0);REG_WRITE!(dmub,DMCUB_GPINT_DATAOUT,0);REG_UPDATE!(dmub,DMCUB_INTERRUPT_ACK,DMCUB_GPINT_IH_INT_ACK,1);REG_UPDATE!(dmub,DMCUB_INTERRUPT_ACK,DMCUB_GPINT_IH_INT_ACK,0);REG_UPDATE!(dmub,DMCUB_INTERRUPT_ENABLE,DMCUB_GPINT_IH_INT_EN,1);v}
pub unsafe fn dmub_dcn401_get_fw_boot_status(dmub:*mut dmub_srv)->dmub_fw_boot_status{let mut s:dmub_fw_boot_status=core::mem::zeroed();s.all=REG_READ!(dmub,DMCUB_SCRATCH0);s}
pub unsafe fn dmub_dcn401_enable_dmub_boot_options(dmub:*mut dmub_srv,p:*const dmub_srv_hw_params){let mut o:dmub_fw_boot_options=core::mem::zeroed();o.bits.z10_disable=(*p).disable_z10;o.bits.skip_phy_access=(*p).disallow_phy_access;REG_WRITE!(dmub,DMCUB_SCRATCH14,o.all)}
pub unsafe fn dmub_dcn401_skip_dmub_panel_power_sequence(dmub:*mut dmub_srv,skip:bool){let mut o:dmub_fw_boot_options=core::mem::zeroed();o.all=REG_READ!(dmub,DMCUB_SCRATCH14);o.bits.skip_phy_init_panel_sequence=skip;REG_WRITE!(dmub,DMCUB_SCRATCH14,o.all)}
pub unsafe fn dmub_dcn401_setup_outbox0(dmub:*mut dmub_srv,r:*const dmub_region){REG_WRITE!(dmub,DMCUB_OUTBOX0_BASE_ADDRESS,(*r).base);REG_WRITE!(dmub,DMCUB_OUTBOX0_SIZE,(*r).top-(*r).base)}
pub unsafe fn dmub_dcn401_get_outbox0_wptr(dmub:*mut dmub_srv)->u32{REG_READ!(dmub,DMCUB_OUTBOX0_WPTR)}
pub unsafe fn dmub_dcn401_set_outbox0_rptr(dmub:*mut dmub_srv,v:u32){REG_WRITE!(dmub,DMCUB_OUTBOX0_RPTR,v)}
pub unsafe fn dmub_dcn401_configure_dmub_in_system_memory(dmub:*mut dmub_srv){REG_WRITE!(dmub,DMCUB_REGION3_TMR_AXI_SPACE,0x4)}
pub unsafe fn dmub_dcn401_send_inbox0_cmd(dmub:*mut dmub_srv,d:dmub_inbox0_data_register){REG_WRITE!(dmub,DMCUB_INBOX0_WPTR,d.inbox0_cmd_common.all)}
pub unsafe fn dmub_dcn401_clear_inbox0_ack_register(dmub:*mut dmub_srv){REG_WRITE!(dmub,DMCUB_SCRATCH17,0)}
pub unsafe fn dmub_dcn401_read_inbox0_ack_register(dmub:*mut dmub_srv)->u32{REG_READ!(dmub,DMCUB_SCRATCH17)}
pub unsafe fn dmub_dcn401_read_reg_inbox0_rsp_int_status(dmub:*mut dmub_srv)->u32{let mut v=0;REG_GET!(dmub,HOST_INTERRUPT_CSR,HOST_REG_INBOX0_RSP_INT_STAT,&mut v);v}
pub unsafe fn dmub_dcn401_write_reg_inbox0_rsp_int_ack(dmub:*mut dmub_srv){REG_UPDATE!(dmub,HOST_INTERRUPT_CSR,HOST_REG_INBOX0_RSP_INT_ACK,1)}
pub unsafe fn dmub_dcn401_clear_reg_inbox0_rsp_int_ack(dmub:*mut dmub_srv){REG_UPDATE!(dmub,HOST_INTERRUPT_CSR,HOST_REG_INBOX0_RSP_INT_ACK,0)}
pub unsafe fn dmub_dcn401_enable_reg_inbox0_rsp_int(dmub:*mut dmub_srv,e:bool){REG_UPDATE!(dmub,HOST_INTERRUPT_CSR,HOST_REG_INBOX0_RSP_INT_EN,if e{1}else{0})}
pub unsafe fn dmub_dcn401_write_reg_outbox0_rdy_int_ack(dmub:*mut dmub_srv){REG_UPDATE!(dmub,HOST_INTERRUPT_CSR,HOST_REG_OUTBOX0_RDY_INT_ACK,1);REG_UPDATE!(dmub,HOST_INTERRUPT_CSR,HOST_REG_OUTBOX0_RDY_INT_ACK,0)}
pub unsafe fn dmub_dcn401_read_reg_outbox0_msg(dmub:*mut dmub_srv,msg:*mut u32){*msg=REG_READ!(dmub,DMCUB_REG_OUTBOX0_MSG0)}
pub unsafe fn dmub_dcn401_write_reg_outbox0_rsp(dmub:*mut dmub_srv,rsp:*const u32){REG_WRITE!(dmub,DMCUB_REG_OUTBOX0_RSP,*rsp)}
pub unsafe fn dmub_dcn401_read_reg_outbox0_rsp_int_status(dmub:*mut dmub_srv)->u32{let mut v=0;REG_GET!(dmub,DMCUB_INTERRUPT_STATUS,DMCUB_REG_OUTBOX0_RSP_INT_STAT,&mut v);v}
pub unsafe fn dmub_dcn401_enable_reg_outbox0_rdy_int(dmub:*mut dmub_srv,e:bool){REG_UPDATE!(dmub,HOST_INTERRUPT_CSR,HOST_REG_OUTBOX0_RDY_INT_EN,if e{1}else{0})}
pub unsafe fn dmub_dcn401_read_reg_outbox0_rdy_int_status(dmub:*mut dmub_srv)->u32{let mut v=0;REG_GET!(dmub,HOST_INTERRUPT_CSR,HOST_REG_OUTBOX0_RDY_INT_STAT,&mut v);v}

pub unsafe fn dmub_dcn401_get_diagnostic_data(dmub:*mut dmub_srv){if dmub.is_null(){return;}let timeout=(*dmub).debug.timeout_info;core::ptr::write_bytes(&mut (*dmub).debug as *mut _,0,1);(*dmub).debug.timeout_info=timeout;(*dmub).debug.dmcub_version=(*dmub).fw_version;for i in 0..17{(*dmub).debug.scratch[i]=REG_READ!(dmub, DMCUB_SCRATCH0 + i);}(*dmub).debug.undefined_address_fault_addr=REG_READ!(dmub,DMCUB_UNDEFINED_ADDRESS_FAULT_ADDR);(*dmub).debug.inst_fetch_fault_addr=REG_READ!(dmub,DMCUB_INST_FETCH_FAULT_ADDR);(*dmub).debug.data_write_fault_addr=REG_READ!(dmub,DMCUB_DATA_WRITE_FAULT_ADDR);(*dmub).debug.inbox1_rptr=REG_READ!(dmub,DMCUB_INBOX1_RPTR);(*dmub).debug.inbox1_wptr=REG_READ!(dmub,DMCUB_INBOX1_WPTR);(*dmub).debug.inbox1_size=REG_READ!(dmub,DMCUB_INBOX1_SIZE);(*dmub).debug.outbox1_rptr=REG_READ!(dmub,DMCUB_OUTBOX1_RPTR);(*dmub).debug.outbox1_wptr=REG_READ!(dmub,DMCUB_OUTBOX1_WPTR);(*dmub).debug.outbox1_size=REG_READ!(dmub,DMCUB_OUTBOX1_SIZE);}

pub unsafe fn dmub_dcn401_send_reg_inbox0_cmd_msg(dmub:*mut dmub_srv,cmd:*const dmub_rb_cmd){let d=cmd as *const u32;let n=(*cmd).cmd_common.header.payload_bytes;for i in 0..15{if n<=i*4{break;}REG_WRITE!(dmub,DMCUB_REG_INBOX0_MSG0+i,*d.add(i+1));}REG_WRITE!(dmub,DMCUB_REG_INBOX0_RDY,*d)}
pub unsafe fn dmub_dcn401_read_reg_inbox0_cmd_rsp(dmub:*mut dmub_srv,cmd:*mut dmub_rb_cmd){let d=cmd as *mut u32;for i in 0..16{*d.add(i)=REG_READ!(dmub,DMCUB_REG_INBOX0_RSP+i);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
