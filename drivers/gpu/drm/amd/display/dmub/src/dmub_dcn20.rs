/* Rust translation of dmub_dcn20.c. */

pub const dmub_srv_dcn20_regs: dmub_srv_common_regs = dmub_srv_common_regs {
    regs: [DMUB_COMMON_REGS!(), DMCUB_INTERNAL_REGS!()],
    masks: [DMUB_COMMON_FIELDS_MASKS!()],
    shifts: [DMUB_COMMON_FIELDS_SHIFTS!()],
};

unsafe fn dmub_dcn20_get_fb_base_offset(dmub: *mut dmub_srv, fb_base: *mut u64, fb_offset: *mut u64) {
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

unsafe fn dmub_dcn20_translate_addr(addr_in: *const dmub_addr, fb_base: u64, fb_offset: u64, addr_out: *mut dmub_addr) {
    (*addr_out).quad_part = (*addr_in).quad_part.wrapping_sub(fb_base).wrapping_add(fb_offset);
}

pub unsafe fn dmub_dcn20_use_cached_inbox(dmub: *mut dmub_srv) -> bool {
    !((*dmub).fw_version >= DMUB_FW_VERSION!(1, 0, 0) && (*dmub).fw_version <= DMUB_FW_VERSION!(1, 10, 0))
}

pub unsafe fn dmub_dcn20_reset(dmub: *mut dmub_srv) {
    let mut cmd: dmub_gpint_data_register = core::mem::zeroed();
    let timeout: u32 = 30;
    let mut in_reset = 0u32; let mut scratch = 0u32; let mut i: u32;
    REG_GET!(dmub, DMCUB_CNTL, DMCUB_SOFT_RESET, &mut in_reset);
    if in_reset == 0 {
        cmd.bits.status = 1; cmd.bits.command_code = DMUB_GPINT__STOP_FW; cmd.bits.param = 0;
        ((*dmub).hw_funcs.set_gpint)(dmub, cmd);
        i = 0; while i < timeout { if ((*dmub).hw_funcs.is_gpint_acked)(dmub, cmd) { break; } i += 1; }
        i = 0; while i < timeout { scratch = ((*dmub).hw_funcs.get_gpint_response)(dmub); if scratch == DMUB_GPINT__STOP_FW_RESPONSE { break; } i += 1; }
        cmd.all = 0; ((*dmub).hw_funcs.set_gpint)(dmub, cmd);
    }
    REG_UPDATE!(dmub, DMCUB_CNTL, DMCUB_SOFT_RESET, 1);
    REG_UPDATE!(dmub, DMCUB_CNTL, DMCUB_ENABLE, 0);
    REG_UPDATE!(dmub, MMHUBBUB_SOFT_RESET, DMUIF_SOFT_RESET, 1);
    REG_WRITE!(dmub, DMCUB_INBOX1_RPTR, 0); REG_WRITE!(dmub, DMCUB_INBOX1_WPTR, 0);
    REG_WRITE!(dmub, DMCUB_OUTBOX1_RPTR, 0); REG_WRITE!(dmub, DMCUB_OUTBOX1_WPTR, 0); REG_WRITE!(dmub, DMCUB_SCRATCH0, 0);
}

pub unsafe fn dmub_dcn20_reset_release(dmub: *mut dmub_srv) {
    REG_UPDATE!(dmub, MMHUBBUB_SOFT_RESET, DMUIF_SOFT_RESET, 0);
    REG_WRITE!(dmub, DMCUB_SCRATCH15, (*dmub).psp_version & 0x001100FF);
    REG_UPDATE_2!(dmub, DMCUB_CNTL, DMCUB_ENABLE, 1, DMCUB_TRACEPORT_EN, 1);
    REG_UPDATE!(dmub, DMCUB_CNTL, DMCUB_SOFT_RESET, 0);
}

pub unsafe fn dmub_dcn20_backdoor_load(dmub: *mut dmub_srv, cw0: *const dmub_window, cw1: *const dmub_window) {
    let mut offset: dmub_addr = core::mem::zeroed(); let (mut fb_base, mut fb_offset) = (0u64, 0u64);
    dmub_dcn20_get_fb_base_offset(dmub, &mut fb_base, &mut fb_offset);
    REG_UPDATE!(dmub, DMCUB_SEC_CNTL, DMCUB_SEC_RESET, 1); REG_UPDATE_2!(dmub, DMCUB_MEM_CNTL, DMCUB_MEM_READ_SPACE, 0x3, DMCUB_MEM_WRITE_SPACE, 0x3);
    dmub_dcn20_translate_addr(&(*cw0).offset, fb_base, fb_offset, &mut offset);
    REG_WRITE!(dmub, DMCUB_REGION3_CW0_OFFSET, offset.u.low_part); REG_WRITE!(dmub, DMCUB_REGION3_CW0_OFFSET_HIGH, offset.u.high_part); REG_WRITE!(dmub, DMCUB_REGION3_CW0_BASE_ADDRESS, (*cw0).region.base); REG_SET_2!(dmub, DMCUB_REGION3_CW0_TOP_ADDRESS, 0, DMCUB_REGION3_CW0_TOP_ADDRESS, (*cw0).region.top, DMCUB_REGION3_CW0_ENABLE, 1);
    dmub_dcn20_translate_addr(&(*cw1).offset, fb_base, fb_offset, &mut offset);
    REG_WRITE!(dmub, DMCUB_REGION3_CW1_OFFSET, offset.u.low_part); REG_WRITE!(dmub, DMCUB_REGION3_CW1_OFFSET_HIGH, offset.u.high_part); REG_WRITE!(dmub, DMCUB_REGION3_CW1_BASE_ADDRESS, (*cw1).region.base); REG_SET_2!(dmub, DMCUB_REGION3_CW1_TOP_ADDRESS, 0, DMCUB_REGION3_CW1_TOP_ADDRESS, (*cw1).region.top, DMCUB_REGION3_CW1_ENABLE, 1);
    REG_UPDATE_2!(dmub, DMCUB_SEC_CNTL, DMCUB_SEC_RESET, 0, DMCUB_MEM_UNIT_ID, 0x20);
}

pub unsafe fn dmub_dcn20_setup_windows(dmub: *mut dmub_srv, cw2: *const dmub_window, cw3: *const dmub_window, cw4: *const dmub_window, cw5: *const dmub_window, cw6: *const dmub_window, _region6: *const dmub_window) {
    let mut offset: dmub_addr = core::mem::zeroed(); let (mut fb_base, mut fb_offset) = (0u64, 0u64); dmub_dcn20_get_fb_base_offset(dmub, &mut fb_base, &mut fb_offset);
    if (*cw2).region.base != (*cw2).region.top { dmub_dcn20_translate_addr(&(*cw2).offset, fb_base, fb_offset, &mut offset); REG_WRITE!(dmub,DMCUB_REGION3_CW2_OFFSET,offset.u.low_part); REG_WRITE!(dmub,DMCUB_REGION3_CW2_OFFSET_HIGH,offset.u.high_part); REG_WRITE!(dmub,DMCUB_REGION3_CW2_BASE_ADDRESS,(*cw2).region.base); REG_SET_2!(dmub,DMCUB_REGION3_CW2_TOP_ADDRESS,0,DMCUB_REGION3_CW2_TOP_ADDRESS,(*cw2).region.top,DMCUB_REGION3_CW2_ENABLE,1); } else { REG_WRITE!(dmub,DMCUB_REGION3_CW2_OFFSET,0); REG_WRITE!(dmub,DMCUB_REGION3_CW2_OFFSET_HIGH,0); REG_WRITE!(dmub,DMCUB_REGION3_CW2_BASE_ADDRESS,0); REG_WRITE!(dmub,DMCUB_REGION3_CW2_TOP_ADDRESS,0); }
    dmub_dcn20_translate_addr(&(*cw3).offset,fb_base,fb_offset,&mut offset); REG_WRITE!(dmub,DMCUB_REGION3_CW3_OFFSET,offset.u.low_part); REG_WRITE!(dmub,DMCUB_REGION3_CW3_OFFSET_HIGH,offset.u.high_part); REG_WRITE!(dmub,DMCUB_REGION3_CW3_BASE_ADDRESS,(*cw3).region.base); REG_SET_2!(dmub,DMCUB_REGION3_CW3_TOP_ADDRESS,0,DMCUB_REGION3_CW3_TOP_ADDRESS,(*cw3).region.top,DMCUB_REGION3_CW3_ENABLE,1);
    dmub_dcn20_translate_addr(&(*cw4).offset,fb_base,fb_offset,&mut offset); if dmub_dcn20_use_cached_inbox(dmub) { REG_WRITE!(dmub,DMCUB_REGION3_CW4_OFFSET,offset.u.low_part); REG_WRITE!(dmub,DMCUB_REGION3_CW4_OFFSET_HIGH,offset.u.high_part); REG_WRITE!(dmub,DMCUB_REGION3_CW4_BASE_ADDRESS,(*cw4).region.base); REG_SET_2!(dmub,DMCUB_REGION3_CW4_TOP_ADDRESS,0,DMCUB_REGION3_CW4_TOP_ADDRESS,(*cw4).region.top,DMCUB_REGION3_CW4_ENABLE,1); } else { REG_WRITE!(dmub,DMCUB_REGION4_OFFSET,offset.u.low_part); REG_WRITE!(dmub,DMCUB_REGION4_OFFSET_HIGH,offset.u.high_part); REG_WRITE!(dmub,DMCUB_REGION4_TOP_ADDRESS,(*cw4).region.top.wrapping_sub((*cw4).region.base).wrapping_sub(1)); }
    dmub_dcn20_translate_addr(&(*cw5).offset,fb_base,fb_offset,&mut offset); REG_WRITE!(dmub,DMCUB_REGION3_CW5_OFFSET,offset.u.low_part); REG_WRITE!(dmub,DMCUB_REGION3_CW5_OFFSET_HIGH,offset.u.high_part); REG_WRITE!(dmub,DMCUB_REGION3_CW5_BASE_ADDRESS,(*cw5).region.base); REG_SET_2!(dmub,DMCUB_REGION3_CW5_TOP_ADDRESS,0,DMCUB_REGION3_CW5_TOP_ADDRESS,(*cw5).region.top,DMCUB_REGION3_CW5_ENABLE,1); REG_WRITE!(dmub,DMCUB_REGION5_OFFSET,offset.u.low_part); REG_WRITE!(dmub,DMCUB_REGION5_OFFSET_HIGH,offset.u.high_part); REG_WRITE!(dmub,DMCUB_REGION5_TOP_ADDRESS,(*cw5).region.top.wrapping_sub((*cw5).region.base).wrapping_sub(1));
    dmub_dcn20_translate_addr(&(*cw6).offset,fb_base,fb_offset,&mut offset); REG_WRITE!(dmub,DMCUB_REGION3_CW6_OFFSET,offset.u.low_part); REG_WRITE!(dmub,DMCUB_REGION3_CW6_OFFSET_HIGH,offset.u.high_part); REG_WRITE!(dmub,DMCUB_REGION3_CW6_BASE_ADDRESS,(*cw6).region.base); REG_SET_2!(dmub,DMCUB_REGION3_CW6_TOP_ADDRESS,0,DMCUB_REGION3_CW6_TOP_ADDRESS,(*cw6).region.top,DMCUB_REGION3_CW6_ENABLE,1);
}

pub unsafe fn dmub_dcn20_setup_mailbox(dmub:*mut dmub_srv,inbox1:*const dmub_region){if dmub_dcn20_use_cached_inbox(dmub){REG_WRITE!(dmub,DMCUB_INBOX1_BASE_ADDRESS,(*inbox1).base)}else{REG_WRITE!(dmub,DMCUB_INBOX1_BASE_ADDRESS,0x80000000)} REG_WRITE!(dmub,DMCUB_INBOX1_SIZE,(*inbox1).top-(*inbox1).base)}
pub unsafe fn dmub_dcn20_get_inbox1_wptr(dmub:*mut dmub_srv)->u32{REG_READ!(dmub,DMCUB_INBOX1_WPTR)}
pub unsafe fn dmub_dcn20_get_inbox1_rptr(dmub:*mut dmub_srv)->u32{REG_READ!(dmub,DMCUB_INBOX1_RPTR)}
pub unsafe fn dmub_dcn20_set_inbox1_wptr(dmub:*mut dmub_srv,wptr_offset:u32){REG_WRITE!(dmub,DMCUB_INBOX1_WPTR,wptr_offset)}
pub unsafe fn dmub_dcn20_setup_out_mailbox(dmub:*mut dmub_srv,outbox1:*const dmub_region){if dmub_dcn20_use_cached_inbox(dmub){REG_WRITE!(dmub,DMCUB_OUTBOX1_BASE_ADDRESS,(*outbox1).base)}else{REG_WRITE!(dmub,DMCUB_OUTBOX1_BASE_ADDRESS,0x80002000)} REG_WRITE!(dmub,DMCUB_OUTBOX1_SIZE,(*outbox1).top-(*outbox1).base)}
pub unsafe fn dmub_dcn20_get_outbox1_wptr(dmub:*mut dmub_srv)->u32{REG_READ!(dmub,DMCUB_OUTBOX1_WPTR)}
pub unsafe fn dmub_dcn20_set_outbox1_rptr(dmub:*mut dmub_srv,rptr_offset:u32){REG_WRITE!(dmub,DMCUB_OUTBOX1_RPTR,rptr_offset)}
pub unsafe fn dmub_dcn20_setup_outbox0(dmub:*mut dmub_srv,outbox0:*const dmub_region){REG_WRITE!(dmub,DMCUB_OUTBOX0_BASE_ADDRESS,(*outbox0).base);REG_WRITE!(dmub,DMCUB_OUTBOX0_SIZE,(*outbox0).top-(*outbox0).base)}
pub unsafe fn dmub_dcn20_get_outbox0_wptr(dmub:*mut dmub_srv)->u32{REG_READ!(dmub,DMCUB_OUTBOX0_WPTR)}
pub unsafe fn dmub_dcn20_set_outbox0_rptr(dmub:*mut dmub_srv,rptr_offset:u32){REG_WRITE!(dmub,DMCUB_OUTBOX0_RPTR,rptr_offset)}
pub unsafe fn dmub_dcn20_is_hw_init(dmub:*mut dmub_srv)->bool{let mut v=0;REG_GET!(dmub,DMCUB_CNTL,DMCUB_ENABLE,&mut v);v!=0}
pub unsafe fn dmub_dcn20_is_supported(dmub:*mut dmub_srv)->bool{let mut v=0;REG_GET!(dmub,CC_DC_PIPE_DIS,DC_DMCUB_ENABLE,&mut v);v!=0}
pub unsafe fn dmub_dcn20_set_gpint(dmub:*mut dmub_srv,reg:dmub_gpint_data_register){REG_WRITE!(dmub,DMCUB_GPINT_DATAIN1,reg.all)}
pub unsafe fn dmub_dcn20_is_gpint_acked(dmub:*mut dmub_srv,mut reg:dmub_gpint_data_register)->bool{let test=REG_READ!(dmub,DMCUB_GPINT_DATAIN1);reg.bits.status=0;test==reg.all}
pub unsafe fn dmub_dcn20_get_gpint_response(dmub:*mut dmub_srv)->u32{REG_READ!(dmub,DMCUB_SCRATCH7)}
pub unsafe fn dmub_dcn20_get_fw_boot_status(dmub:*mut dmub_srv)->dmub_fw_boot_status{let mut s:dmub_fw_boot_status=core::mem::zeroed();s.all=REG_READ!(dmub,DMCUB_SCRATCH0);s}
pub unsafe fn dmub_dcn20_enable_dmub_boot_options(dmub:*mut dmub_srv,_params:*const dmub_srv_hw_params){let o:dmub_fw_boot_options=core::mem::zeroed();REG_WRITE!(dmub,DMCUB_SCRATCH14,o.all)}
pub unsafe fn dmub_dcn20_skip_dmub_panel_power_sequence(dmub:*mut dmub_srv,skip:bool){let mut o:dmub_fw_boot_options=core::mem::zeroed();o.all=REG_READ!(dmub,DMCUB_SCRATCH14);o.bits.skip_phy_init_panel_sequence=skip;REG_WRITE!(dmub,DMCUB_SCRATCH14,o.all)}
pub unsafe fn dmub_dcn20_get_current_time(dmub:*mut dmub_srv)->u32{REG_READ!(dmub,DMCUB_TIMER_CURRENT)}

pub unsafe fn dmub_dcn20_get_diagnostic_data(dmub:*mut dmub_srv){
    if dmub.is_null(){return} let timeout=(*dmub).debug.timeout_info; core::ptr::write_bytes(&mut (*dmub).debug as *mut _,0,1); (*dmub).debug.timeout_info=timeout; (*dmub).debug.dmcub_version=(*dmub).fw_version;
    (*dmub).debug.scratch[0]=REG_READ!(dmub,DMCUB_SCRATCH0);(*dmub).debug.scratch[1]=REG_READ!(dmub,DMCUB_SCRATCH1);(*dmub).debug.scratch[2]=REG_READ!(dmub,DMCUB_SCRATCH2);(*dmub).debug.scratch[3]=REG_READ!(dmub,DMCUB_SCRATCH3);(*dmub).debug.scratch[4]=REG_READ!(dmub,DMCUB_SCRATCH4);(*dmub).debug.scratch[5]=REG_READ!(dmub,DMCUB_SCRATCH5);(*dmub).debug.scratch[6]=REG_READ!(dmub,DMCUB_SCRATCH6);(*dmub).debug.scratch[7]=REG_READ!(dmub,DMCUB_SCRATCH7);(*dmub).debug.scratch[8]=REG_READ!(dmub,DMCUB_SCRATCH8);(*dmub).debug.scratch[9]=REG_READ!(dmub,DMCUB_SCRATCH9);(*dmub).debug.scratch[10]=REG_READ!(dmub,DMCUB_SCRATCH10);(*dmub).debug.scratch[11]=REG_READ!(dmub,DMCUB_SCRATCH11);(*dmub).debug.scratch[12]=REG_READ!(dmub,DMCUB_SCRATCH12);(*dmub).debug.scratch[13]=REG_READ!(dmub,DMCUB_SCRATCH13);(*dmub).debug.scratch[14]=REG_READ!(dmub,DMCUB_SCRATCH14);(*dmub).debug.scratch[15]=REG_READ!(dmub,DMCUB_SCRATCH15);
    (*dmub).debug.undefined_address_fault_addr=REG_READ!(dmub,DMCUB_UNDEFINED_ADDRESS_FAULT_ADDR);(*dmub).debug.inst_fetch_fault_addr=REG_READ!(dmub,DMCUB_INST_FETCH_FAULT_ADDR);(*dmub).debug.data_write_fault_addr=REG_READ!(dmub,DMCUB_DATA_WRITE_FAULT_ADDR);(*dmub).debug.inbox1_rptr=REG_READ!(dmub,DMCUB_INBOX1_RPTR);(*dmub).debug.inbox1_wptr=REG_READ!(dmub,DMCUB_INBOX1_WPTR);(*dmub).debug.inbox1_size=REG_READ!(dmub,DMCUB_INBOX1_SIZE);(*dmub).debug.inbox0_rptr=REG_READ!(dmub,DMCUB_INBOX0_RPTR);(*dmub).debug.inbox0_wptr=REG_READ!(dmub,DMCUB_INBOX0_WPTR);(*dmub).debug.inbox0_size=REG_READ!(dmub,DMCUB_INBOX0_SIZE);
    let mut a=0; REG_GET!(dmub,DMCUB_CNTL,DMCUB_ENABLE,&mut a);(*dmub).debug.is_dmcub_enabled=a as u8;REG_GET!(dmub,DMCUB_CNTL,DMCUB_SOFT_RESET,&mut a);(*dmub).debug.is_dmcub_soft_reset=a as u8;REG_GET!(dmub,DMCUB_SEC_CNTL,DMCUB_SEC_RESET_STATUS,&mut a);(*dmub).debug.is_dmcub_secure_reset=a as u8;REG_GET!(dmub,DMCUB_CNTL,DMCUB_TRACEPORT_EN,&mut a);(*dmub).debug.is_traceport_en=a as u8;REG_GET!(dmub,DMCUB_REGION3_CW0_TOP_ADDRESS,DMCUB_REGION3_CW0_ENABLE,&mut a);(*dmub).debug.is_cw0_enabled=a as u8;REG_GET!(dmub,DMCUB_REGION3_CW6_TOP_ADDRESS,DMCUB_REGION3_CW6_ENABLE,&mut a);(*dmub).debug.is_cw6_enabled=a as u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
