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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Dependencies are supplied by the surrounding translated driver.

pub unsafe fn dmub_srv_dcn32_regs_init(dmub: *mut dmub_srv, ctx: *mut dc_context) {
    let regs = (*dmub).regs_dcn32;
    macro_rules! dmub_sr { ($reg:ident) => { (*regs).offset.$reg = BASE((*ctx).dcn_reg_offsets[$reg##_BASE_IDX]) + $reg; }; }
    DMUB_DCN32_REGS!();
    DMCUB_INTERNAL_REGS!();
    macro_rules! dmub_sf { ($reg:ident, $field:ident) => { (*regs).mask.$reg##__$field = FD_MASK($reg, $field); }; }
    DMUB_DCN32_FIELDS!();
    macro_rules! dmub_sf_shift { ($reg:ident, $field:ident) => { (*regs).shift.$reg##__$field = FD_SHIFT($reg, $field); }; }
    DMUB_DCN32_FIELDS!();
}

unsafe fn dmub_dcn32_get_fb_base_offset(dmub: *mut dmub_srv, fb_base: *mut u64, fb_offset: *mut u64) {
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
unsafe fn dmub_dcn32_translate_addr(addr_in: *const dmub_addr, fb_base: u64, fb_offset: u64, addr_out: *mut dmub_addr) {
    (*addr_out).quad_part = (*addr_in).quad_part.wrapping_sub(fb_base).wrapping_add(fb_offset);
}

pub unsafe fn dmub_dcn32_reset(dmub: *mut dmub_srv) {
    let mut cmd: dmub_gpint_data_register = core::mem::zeroed();
    let timeout_us: u32 = 1 * 1000 * 1000;
    let poll_delay_us: u32 = 1;
    let mut i: u32 = 0;
    let (mut enabled, mut in_reset, mut scratch, mut pwait_mode) = (0, 0, 0, 0);
    REG_GET!(dmub, DMCUB_CNTL, DMCUB_ENABLE, &mut enabled);
    REG_GET!(dmub, DMCUB_CNTL2, DMCUB_SOFT_RESET, &mut in_reset);
    if enabled != 0 && in_reset == 0 {
        cmd.bits.status = 1; cmd.bits.command_code = DMUB_GPINT__STOP_FW; cmd.bits.param = 0;
        ((*dmub).hw_funcs.set_gpint)(dmub, cmd);
        while i < timeout_us { scratch = REG_READ!(dmub, DMCUB_SCRATCH7); if scratch == DMUB_GPINT__STOP_FW_RESPONSE { break; } udelay(poll_delay_us); i += 1; }
        while i < timeout_us { REG_GET!(dmub, DMCUB_CNTL, DMCUB_PWAIT_MODE_STATUS, &mut pwait_mode); if pwait_mode & (1 << 0) != 0 { break; } udelay(poll_delay_us); i += 1; }
    }
    if enabled != 0 { REG_UPDATE!(dmub, DMCUB_CNTL2, DMCUB_SOFT_RESET, 1); udelay(1); REG_UPDATE!(dmub, DMCUB_CNTL, DMCUB_ENABLE, 0); }
    if i >= timeout_us { BREAK_TO_DEBUGGER!(); }
    REG_UPDATE!(dmub, DMCUB_REGION3_CW2_TOP_ADDRESS, DMCUB_REGION3_CW2_ENABLE, 0); REG_UPDATE!(dmub, DMCUB_REGION3_CW3_TOP_ADDRESS, DMCUB_REGION3_CW3_ENABLE, 0); REG_UPDATE!(dmub, DMCUB_REGION3_CW4_TOP_ADDRESS, DMCUB_REGION3_CW4_ENABLE, 0); REG_UPDATE!(dmub, DMCUB_REGION3_CW5_TOP_ADDRESS, DMCUB_REGION3_CW5_ENABLE, 0); REG_UPDATE!(dmub, DMCUB_REGION3_CW6_TOP_ADDRESS, DMCUB_REGION3_CW6_ENABLE, 0); REG_UPDATE!(dmub, DMCUB_REGION3_CW7_TOP_ADDRESS, DMCUB_REGION3_CW7_ENABLE, 0);
    REG_WRITE!(dmub, DMCUB_INBOX1_RPTR, 0); REG_WRITE!(dmub, DMCUB_INBOX1_WPTR, 0); REG_WRITE!(dmub, DMCUB_OUTBOX1_RPTR, 0); REG_WRITE!(dmub, DMCUB_OUTBOX1_WPTR, 0); REG_WRITE!(dmub, DMCUB_OUTBOX0_RPTR, 0); REG_WRITE!(dmub, DMCUB_OUTBOX0_WPTR, 0); REG_WRITE!(dmub, DMCUB_SCRATCH0, 0);
    cmd.all = 0; ((*dmub).hw_funcs.set_gpint)(dmub, cmd);
}

pub unsafe fn dmub_dcn32_reset_release(dmub: *mut dmub_srv) { REG_UPDATE!(dmub, MMHUBBUB_SOFT_RESET, DMUIF_SOFT_RESET, 0); REG_WRITE!(dmub, DMCUB_SCRATCH15, (*dmub).psp_version & 0x001100FF); REG_UPDATE_2!(dmub, DMCUB_CNTL, DMCUB_ENABLE, 1, DMCUB_TRACEPORT_EN, 1); REG_UPDATE!(dmub, DMCUB_CNTL2, DMCUB_SOFT_RESET, 0); }

pub unsafe fn dmub_dcn32_backdoor_load(dmub: *mut dmub_srv, cw0: *const dmub_window, cw1: *const dmub_window) { let (mut fb_base, mut fb_offset) = (0, 0); let mut offset: dmub_addr = core::mem::zeroed(); dmub_dcn32_get_fb_base_offset(dmub, &mut fb_base, &mut fb_offset); REG_UPDATE!(dmub, DMCUB_SEC_CNTL, DMCUB_SEC_RESET, 1); REG_UPDATE!(dmub, DMCUB_CNTL, DMCUB_ENABLE, 0); dmub_dcn32_translate_addr(&(*cw0).offset, fb_base, fb_offset, &mut offset); REG_WRITE!(dmub, DMCUB_REGION3_CW0_OFFSET, offset.u.low_part); REG_WRITE!(dmub, DMCUB_REGION3_CW0_OFFSET_HIGH, offset.u.high_part); REG_WRITE!(dmub, DMCUB_REGION3_CW0_BASE_ADDRESS, (*cw0).region.base); REG_SET_2!(dmub, DMCUB_REGION3_CW0_TOP_ADDRESS, 0, DMCUB_REGION3_CW0_TOP_ADDRESS, (*cw0).region.top, DMCUB_REGION3_CW0_ENABLE, 1); dmub_dcn32_translate_addr(&(*cw1).offset, fb_base, fb_offset, &mut offset); REG_WRITE!(dmub, DMCUB_REGION3_CW1_OFFSET, offset.u.low_part); REG_WRITE!(dmub, DMCUB_REGION3_CW1_OFFSET_HIGH, offset.u.high_part); REG_WRITE!(dmub, DMCUB_REGION3_CW1_BASE_ADDRESS, (*cw1).region.base); REG_SET_2!(dmub, DMCUB_REGION3_CW1_TOP_ADDRESS, 0, DMCUB_REGION3_CW1_TOP_ADDRESS, (*cw1).region.top, DMCUB_REGION3_CW1_ENABLE, 1); REG_UPDATE_2!(dmub, DMCUB_SEC_CNTL, DMCUB_SEC_RESET, 0, DMCUB_MEM_UNIT_ID, 0x20); }

pub unsafe fn dmub_dcn32_backdoor_load_zfb_mode(dmub: *mut dmub_srv, cw0: *const dmub_window, cw1: *const dmub_window) { let mut offset = (*cw0).offset; REG_UPDATE!(dmub, DMCUB_SEC_CNTL, DMCUB_SEC_RESET, 1); REG_UPDATE!(dmub, DMCUB_CNTL, DMCUB_ENABLE, 0); REG_WRITE!(dmub, DMCUB_REGION3_CW0_OFFSET, offset.u.low_part); REG_WRITE!(dmub, DMCUB_REGION3_CW0_OFFSET_HIGH, offset.u.high_part); REG_WRITE!(dmub, DMCUB_REGION3_CW0_BASE_ADDRESS, (*cw0).region.base); REG_SET_2!(dmub, DMCUB_REGION3_CW0_TOP_ADDRESS, 0, DMCUB_REGION3_CW0_TOP_ADDRESS, (*cw0).region.top, DMCUB_REGION3_CW0_ENABLE, 1); offset = (*cw1).offset; REG_WRITE!(dmub, DMCUB_REGION3_CW1_OFFSET, offset.u.low_part); REG_WRITE!(dmub, DMCUB_REGION3_CW1_OFFSET_HIGH, offset.u.high_part); REG_WRITE!(dmub, DMCUB_REGION3_CW1_BASE_ADDRESS, (*cw1).region.base); REG_SET_2!(dmub, DMCUB_REGION3_CW1_TOP_ADDRESS, 0, DMCUB_REGION3_CW1_TOP_ADDRESS, (*cw1).region.top, DMCUB_REGION3_CW1_ENABLE, 1); REG_UPDATE_2!(dmub, DMCUB_SEC_CNTL, DMCUB_SEC_RESET, 0, DMCUB_MEM_UNIT_ID, 0x20); }

pub unsafe fn dmub_dcn32_setup_windows(dmub: *mut dmub_srv, cw2: *const dmub_window, cw3: *const dmub_window, cw4: *const dmub_window, cw5: *const dmub_window, cw6: *const dmub_window, region6: *const dmub_window) {
    let _ = (dmub, cw2, region6); let mut o = (*cw3).offset;
    REG_WRITE!(dmub,DMCUB_REGION3_CW3_OFFSET,o.u.low_part); REG_WRITE!(dmub,DMCUB_REGION3_CW3_OFFSET_HIGH,o.u.high_part); REG_WRITE!(dmub,DMCUB_REGION3_CW3_BASE_ADDRESS,(*cw3).region.base); REG_SET_2!(dmub,DMCUB_REGION3_CW3_TOP_ADDRESS,0,DMCUB_REGION3_CW3_TOP_ADDRESS,(*cw3).region.top,DMCUB_REGION3_CW3_ENABLE,1);
    o=(*cw4).offset; REG_WRITE!(dmub,DMCUB_REGION3_CW4_OFFSET,o.u.low_part); REG_WRITE!(dmub,DMCUB_REGION3_CW4_OFFSET_HIGH,o.u.high_part); REG_WRITE!(dmub,DMCUB_REGION3_CW4_BASE_ADDRESS,(*cw4).region.base); REG_SET_2!(dmub,DMCUB_REGION3_CW4_TOP_ADDRESS,0,DMCUB_REGION3_CW4_TOP_ADDRESS,(*cw4).region.top,DMCUB_REGION3_CW4_ENABLE,1);
    o=(*cw5).offset; REG_WRITE!(dmub,DMCUB_REGION3_CW5_OFFSET,o.u.low_part); REG_WRITE!(dmub,DMCUB_REGION3_CW5_OFFSET_HIGH,o.u.high_part); REG_WRITE!(dmub,DMCUB_REGION3_CW5_BASE_ADDRESS,(*cw5).region.base); REG_SET_2!(dmub,DMCUB_REGION3_CW5_TOP_ADDRESS,0,DMCUB_REGION3_CW5_TOP_ADDRESS,(*cw5).region.top,DMCUB_REGION3_CW5_ENABLE,1); REG_WRITE!(dmub,DMCUB_REGION5_OFFSET,o.u.low_part); REG_WRITE!(dmub,DMCUB_REGION5_OFFSET_HIGH,o.u.high_part); REG_SET_2!(dmub,DMCUB_REGION5_TOP_ADDRESS,0,DMCUB_REGION5_TOP_ADDRESS,(*cw5).region.top-(*cw5).region.base-1,DMCUB_REGION5_ENABLE,1);
    o=(*cw6).offset; REG_WRITE!(dmub,DMCUB_REGION3_CW6_OFFSET,o.u.low_part); REG_WRITE!(dmub,DMCUB_REGION3_CW6_OFFSET_HIGH,o.u.high_part); REG_WRITE!(dmub,DMCUB_REGION3_CW6_BASE_ADDRESS,(*cw6).region.base); REG_SET_2!(dmub,DMCUB_REGION3_CW6_TOP_ADDRESS,0,DMCUB_REGION3_CW6_TOP_ADDRESS,(*cw6).region.top,DMCUB_REGION3_CW6_ENABLE,1);
}
pub unsafe fn dmub_dcn32_setup_mailbox(dmub:*mut dmub_srv, r:*const dmub_region){REG_WRITE!(dmub,DMCUB_INBOX1_BASE_ADDRESS,(*r).base);REG_WRITE!(dmub,DMCUB_INBOX1_SIZE,(*r).top-(*r).base);}
pub unsafe fn dmub_dcn32_get_inbox1_wptr(dmub:*mut dmub_srv)->u32{REG_READ!(dmub,DMCUB_INBOX1_WPTR)}
pub unsafe fn dmub_dcn32_get_inbox1_rptr(dmub:*mut dmub_srv)->u32{REG_READ!(dmub,DMCUB_INBOX1_RPTR)}
pub unsafe fn dmub_dcn32_set_inbox1_wptr(dmub:*mut dmub_srv,v:u32){REG_WRITE!(dmub,DMCUB_INBOX1_WPTR,v);}
pub unsafe fn dmub_dcn32_setup_out_mailbox(dmub:*mut dmub_srv,r:*const dmub_region){REG_WRITE!(dmub,DMCUB_OUTBOX1_BASE_ADDRESS,(*r).base);REG_WRITE!(dmub,DMCUB_OUTBOX1_SIZE,(*r).top-(*r).base);}
pub unsafe fn dmub_dcn32_get_outbox1_wptr(dmub:*mut dmub_srv)->u32{REG_READ!(dmub,DMCUB_OUTBOX1_WPTR)}
pub unsafe fn dmub_dcn32_set_outbox1_rptr(dmub:*mut dmub_srv,v:u32){REG_WRITE!(dmub,DMCUB_OUTBOX1_RPTR,v);}
pub unsafe fn dmub_dcn32_is_hw_init(dmub:*mut dmub_srv)->bool{let mut s:dmub_fw_boot_status=core::mem::zeroed();let mut e=0;s.all=REG_READ!(dmub,DMCUB_SCRATCH0);REG_GET!(dmub,DMCUB_CNTL,DMCUB_ENABLE,&mut e);e!=0&&s.bits.dal_fw}
pub unsafe fn dmub_dcn32_is_supported(dmub:*mut dmub_srv)->bool{let mut v=0;REG_GET!(dmub,CC_DC_PIPE_DIS,DC_DMCUB_ENABLE,&mut v);v!=0}
pub unsafe fn dmub_dcn32_set_gpint(dmub:*mut dmub_srv,r:dmub_gpint_data_register){REG_WRITE!(dmub,DMCUB_GPINT_DATAIN1,r.all);}
pub unsafe fn dmub_dcn32_is_gpint_acked(dmub:*mut dmub_srv,mut r:dmub_gpint_data_register)->bool{let mut t:dmub_gpint_data_register=core::mem::zeroed();r.bits.status=0;t.all=REG_READ!(dmub,DMCUB_GPINT_DATAIN1);t.all==r.all}
pub unsafe fn dmub_dcn32_get_gpint_response(dmub:*mut dmub_srv)->u32{REG_READ!(dmub,DMCUB_SCRATCH7)}
pub unsafe fn dmub_dcn32_get_gpint_dataout(dmub:*mut dmub_srv)->u32{let d=REG_READ!(dmub,DMCUB_GPINT_DATAOUT);REG_UPDATE!(dmub,DMCUB_INTERRUPT_ENABLE,DMCUB_GPINT_IH_INT_EN,0);REG_WRITE!(dmub,DMCUB_GPINT_DATAOUT,0);REG_UPDATE!(dmub,DMCUB_INTERRUPT_ACK,DMCUB_GPINT_IH_INT_ACK,1);REG_UPDATE!(dmub,DMCUB_INTERRUPT_ACK,DMCUB_GPINT_IH_INT_ACK,0);REG_UPDATE!(dmub,DMCUB_INTERRUPT_ENABLE,DMCUB_GPINT_IH_INT_EN,1);d}
pub unsafe fn dmub_dcn32_get_fw_boot_status(dmub:*mut dmub_srv)->dmub_fw_boot_status{let mut s:dmub_fw_boot_status=core::mem::zeroed();s.all=REG_READ!(dmub,DMCUB_SCRATCH0);s}
pub unsafe fn dmub_dcn32_enable_dmub_boot_options(dmub:*mut dmub_srv,p:*const dmub_srv_hw_params){let mut o:dmub_fw_boot_options=core::mem::zeroed();o.bits.z10_disable=(*p).disable_z10;REG_WRITE!(dmub,DMCUB_SCRATCH14,o.all);}
pub unsafe fn dmub_dcn32_skip_dmub_panel_power_sequence(dmub:*mut dmub_srv,skip:bool){let mut o:dmub_fw_boot_options=core::mem::zeroed();o.all=REG_READ!(dmub,DMCUB_SCRATCH14);o.bits.skip_phy_init_panel_sequence=skip;REG_WRITE!(dmub,DMCUB_SCRATCH14,o.all);}
pub unsafe fn dmub_dcn32_setup_outbox0(dmub:*mut dmub_srv,r:*const dmub_region){REG_WRITE!(dmub,DMCUB_OUTBOX0_BASE_ADDRESS,(*r).base);REG_WRITE!(dmub,DMCUB_OUTBOX0_SIZE,(*r).top-(*r).base);}
pub unsafe fn dmub_dcn32_get_outbox0_wptr(dmub:*mut dmub_srv)->u32{REG_READ!(dmub,DMCUB_OUTBOX0_WPTR)}
pub unsafe fn dmub_dcn32_set_outbox0_rptr(dmub:*mut dmub_srv,v:u32){REG_WRITE!(dmub,DMCUB_OUTBOX0_RPTR,v);}
pub unsafe fn dmub_dcn32_get_current_time(dmub:*mut dmub_srv)->u32{REG_READ!(dmub,DMCUB_TIMER_CURRENT)}

pub unsafe fn dmub_dcn32_get_diagnostic_data(dmub: *mut dmub_srv) {
    if dmub.is_null() { return; }
    let timeout = (*dmub).debug.timeout_info;
    core::ptr::write_bytes(&mut (*dmub).debug as *mut _, 0, 1);
    (*dmub).debug.timeout_info = timeout;
    (*dmub).debug.dmcub_version = (*dmub).fw_version;
    (*dmub).debug.scratch[0]=REG_READ!(dmub,DMCUB_SCRATCH0); (*dmub).debug.scratch[1]=REG_READ!(dmub,DMCUB_SCRATCH1); (*dmub).debug.scratch[2]=REG_READ!(dmub,DMCUB_SCRATCH2); (*dmub).debug.scratch[3]=REG_READ!(dmub,DMCUB_SCRATCH3); (*dmub).debug.scratch[4]=REG_READ!(dmub,DMCUB_SCRATCH4); (*dmub).debug.scratch[5]=REG_READ!(dmub,DMCUB_SCRATCH5); (*dmub).debug.scratch[6]=REG_READ!(dmub,DMCUB_SCRATCH6); (*dmub).debug.scratch[7]=REG_READ!(dmub,DMCUB_SCRATCH7); (*dmub).debug.scratch[8]=REG_READ!(dmub,DMCUB_SCRATCH8); (*dmub).debug.scratch[9]=REG_READ!(dmub,DMCUB_SCRATCH9); (*dmub).debug.scratch[10]=REG_READ!(dmub,DMCUB_SCRATCH10); (*dmub).debug.scratch[11]=REG_READ!(dmub,DMCUB_SCRATCH11); (*dmub).debug.scratch[12]=REG_READ!(dmub,DMCUB_SCRATCH12); (*dmub).debug.scratch[13]=REG_READ!(dmub,DMCUB_SCRATCH13); (*dmub).debug.scratch[14]=REG_READ!(dmub,DMCUB_SCRATCH14); (*dmub).debug.scratch[15]=REG_READ!(dmub,DMCUB_SCRATCH15); (*dmub).debug.scratch[16]=REG_READ!(dmub,DMCUB_SCRATCH16);
    (*dmub).debug.undefined_address_fault_addr=REG_READ!(dmub,DMCUB_UNDEFINED_ADDRESS_FAULT_ADDR); (*dmub).debug.inst_fetch_fault_addr=REG_READ!(dmub,DMCUB_INST_FETCH_FAULT_ADDR); (*dmub).debug.data_write_fault_addr=REG_READ!(dmub,DMCUB_DATA_WRITE_FAULT_ADDR);
    (*dmub).debug.inbox1_rptr=REG_READ!(dmub,DMCUB_INBOX1_RPTR);(*dmub).debug.inbox1_wptr=REG_READ!(dmub,DMCUB_INBOX1_WPTR);(*dmub).debug.inbox1_size=REG_READ!(dmub,DMCUB_INBOX1_SIZE);(*dmub).debug.inbox0_rptr=REG_READ!(dmub,DMCUB_INBOX0_RPTR);(*dmub).debug.inbox0_wptr=REG_READ!(dmub,DMCUB_INBOX0_WPTR);(*dmub).debug.inbox0_size=REG_READ!(dmub,DMCUB_INBOX0_SIZE);(*dmub).debug.outbox1_rptr=REG_READ!(dmub,DMCUB_OUTBOX1_RPTR);(*dmub).debug.outbox1_wptr=REG_READ!(dmub,DMCUB_OUTBOX1_WPTR);(*dmub).debug.outbox1_size=REG_READ!(dmub,DMCUB_OUTBOX1_SIZE);
    let (mut a,mut b,mut c,mut d,mut e)=(0,0,0,0,0);REG_GET!(dmub,DMCUB_CNTL,DMCUB_ENABLE,&mut a);REG_GET!(dmub,DMCUB_CNTL,DMCUB_PWAIT_MODE_STATUS,&mut b);REG_GET!(dmub,DMCUB_CNTL2,DMCUB_SOFT_RESET,&mut c);REG_GET!(dmub,DMCUB_CNTL,DMCUB_TRACEPORT_EN,&mut d);REG_GET!(dmub,DMCUB_REGION3_CW6_TOP_ADDRESS,DMCUB_REGION3_CW6_ENABLE,&mut e);(*dmub).debug.is_dmcub_enabled=a as u8;(*dmub).debug.is_pwait=b as u8;(*dmub).debug.is_dmcub_soft_reset=c as u8;(*dmub).debug.is_traceport_en=d as u8;(*dmub).debug.is_cw6_enabled=e as u8;(*dmub).debug.gpint_datain0=REG_READ!(dmub,DMCUB_GPINT_DATAIN0);
}
pub unsafe fn dmub_dcn32_configure_dmub_in_system_memory(dmub:*mut dmub_srv){REG_WRITE!(dmub,DMCUB_REGION3_TMR_AXI_SPACE,0x4);}
pub unsafe fn dmub_dcn32_send_inbox0_cmd(dmub:*mut dmub_srv,data:dmub_inbox0_data_register){REG_WRITE!(dmub,DMCUB_INBOX0_WPTR,data.inbox0_cmd_common.all);}
pub unsafe fn dmub_dcn32_clear_inbox0_ack_register(dmub:*mut dmub_srv){REG_WRITE!(dmub,DMCUB_SCRATCH17,0);}
pub unsafe fn dmub_dcn32_read_inbox0_ack_register(dmub:*mut dmub_srv)->u32{REG_READ!(dmub,DMCUB_SCRATCH17)}
pub unsafe fn dmub_dcn32_save_surf_addr(dmub:*mut dmub_srv,addr:*const dc_plane_address,subvp_index:u8){let mut index=0;if subvp_index==0{index=REG_READ!(dmub,DMCUB_SCRATCH15);if index!=0{REG_WRITE!(dmub,DMCUB_SCRATCH9,(*addr).grph.addr.low_part);REG_WRITE!(dmub,DMCUB_SCRATCH11,(*addr).grph.meta_addr.low_part);}else{REG_WRITE!(dmub,DMCUB_SCRATCH12,(*addr).grph.addr.low_part);REG_WRITE!(dmub,DMCUB_SCRATCH13,(*addr).grph.meta_addr.low_part);}REG_WRITE!(dmub,DMCUB_SCRATCH15,(!index)!=0);}else if subvp_index==1{index=REG_READ!(dmub,DMCUB_SCRATCH23);if index!=0{REG_WRITE!(dmub,DMCUB_SCRATCH18,(*addr).grph.addr.low_part);REG_WRITE!(dmub,DMCUB_SCRATCH19,(*addr).grph.meta_addr.low_part);}else{REG_WRITE!(dmub,DMCUB_SCRATCH20,(*addr).grph.addr.low_part);REG_WRITE!(dmub,DMCUB_SCRATCH22,(*addr).grph.meta_addr.low_part);}REG_WRITE!(dmub,DMCUB_SCRATCH23,(!index)!=0);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
