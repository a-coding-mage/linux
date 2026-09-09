/* Rust translation of dmub_dcn31.c. */

// External register definitions, types, helpers, and macros are supplied by
// the surrounding driver translation unit.

pub const DMUB_SRV_DCN31_REGS: dmub_srv_dcn31_regs = dmub_srv_dcn31_regs {
    regs: [DMUB_DCN31_REGS!(), DMCUB_INTERNAL_REGS!()],
    masks: [DMUB_DCN31_FIELDS_MASKS!()],
    shifts: [DMUB_DCN31_FIELDS_SHIFTS!()],
};

unsafe fn dmub_dcn31_get_fb_base_offset(dmub: *mut dmub_srv, fb_base: *mut u64, fb_offset: *mut u64) {
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
unsafe fn dmub_dcn31_translate_addr(addr_in: *const dmub_addr, fb_base: u64, fb_offset: u64, addr_out: *mut dmub_addr) {
    (*addr_out).quad_part = (*addr_in).quad_part.wrapping_sub(fb_base).wrapping_add(fb_offset);
}

pub unsafe fn dmub_dcn31_reset(dmub: *mut dmub_srv) {
    let mut cmd: dmub_gpint_data_register = core::mem::zeroed();
    let timeout: u32 = 100000;
    let (mut in_reset, mut is_enabled, mut scratch, mut i, mut pwait_mode): (u32,u32,u32,u32,u32) = (0,0,0,0,0);
    REG_GET!(DMCUB_CNTL2, DMCUB_SOFT_RESET, &mut in_reset);
    if in_reset == 0 {
        cmd.bits.status = 1; cmd.bits.command_code = DMUB_GPINT__STOP_FW; cmd.bits.param = 0;
        ((*dmub).hw_funcs.set_gpint)(dmub, cmd);
        for i in 0..timeout { if ((*dmub).hw_funcs.is_gpint_acked)(dmub, cmd) { break; } udelay(1); }
        for i in 0..timeout { scratch = REG_READ!(DMCUB_SCRATCH7); if scratch == DMUB_GPINT__STOP_FW_RESPONSE { break; } udelay(1); }
        for i in 0..timeout { REG_GET!(DMCUB_CNTL, DMCUB_PWAIT_MODE_STATUS, &mut pwait_mode); if pwait_mode & (1 << 0) != 0 { break; } udelay(1); }
    }
    REG_GET!(DMCUB_CNTL, DMCUB_ENABLE, &mut is_enabled);
    if is_enabled != 0 {
        REG_UPDATE!(DMCUB_CNTL2, DMCUB_SOFT_RESET, 1);
        REG_UPDATE!(MMHUBBUB_SOFT_RESET, DMUIF_SOFT_RESET, 1);
        REG_UPDATE!(DMCUB_CNTL, DMCUB_ENABLE, 0);
    }
    REG_WRITE!(DMCUB_INBOX1_RPTR, 0); REG_WRITE!(DMCUB_INBOX1_WPTR, 0);
    REG_WRITE!(DMCUB_OUTBOX1_RPTR, 0); REG_WRITE!(DMCUB_OUTBOX1_WPTR, 0);
    REG_WRITE!(DMCUB_OUTBOX0_RPTR, 0); REG_WRITE!(DMCUB_OUTBOX0_WPTR, 0);
    REG_WRITE!(DMCUB_SCRATCH0, 0);
    cmd.all = 0; ((*dmub).hw_funcs.set_gpint)(dmub, cmd);
}

pub unsafe fn dmub_dcn31_reset_release(dmub: *mut dmub_srv) {
    REG_UPDATE!(MMHUBBUB_SOFT_RESET, DMUIF_SOFT_RESET, 0);
    REG_WRITE!(DMCUB_SCRATCH15, (*dmub).psp_version & 0x001100FF);
    REG_UPDATE_2!(DMCUB_CNTL, DMCUB_ENABLE, 1, DMCUB_TRACEPORT_EN, 1);
    REG_UPDATE!(DMCUB_CNTL2, DMCUB_SOFT_RESET, 0);
}

pub unsafe fn dmub_dcn31_backdoor_load(dmub: *mut dmub_srv, cw0: *const dmub_window, cw1: *const dmub_window) {
    let (mut offset, mut fb_base, mut fb_offset): (dmub_addr,u64,u64) = (core::mem::zeroed(),0,0);
    dmub_dcn31_get_fb_base_offset(dmub, &mut fb_base, &mut fb_offset);
    REG_UPDATE!(DMCUB_SEC_CNTL, DMCUB_SEC_RESET, 1);
    dmub_dcn31_translate_addr(&(*cw0).offset, fb_base, fb_offset, &mut offset);
    REG_WRITE!(DMCUB_REGION3_CW0_OFFSET, offset.u.low_part); REG_WRITE!(DMCUB_REGION3_CW0_OFFSET_HIGH, offset.u.high_part);
    REG_WRITE!(DMCUB_REGION3_CW0_BASE_ADDRESS, (*cw0).region.base);
    REG_SET_2!(DMCUB_REGION3_CW0_TOP_ADDRESS, 0, DMCUB_REGION3_CW0_TOP_ADDRESS, (*cw0).region.top, DMCUB_REGION3_CW0_ENABLE, 1);
    dmub_dcn31_translate_addr(&(*cw1).offset, fb_base, fb_offset, &mut offset);
    REG_WRITE!(DMCUB_REGION3_CW1_OFFSET, offset.u.low_part); REG_WRITE!(DMCUB_REGION3_CW1_OFFSET_HIGH, offset.u.high_part);
    REG_WRITE!(DMCUB_REGION3_CW1_BASE_ADDRESS, (*cw1).region.base);
    REG_SET_2!(DMCUB_REGION3_CW1_TOP_ADDRESS, 0, DMCUB_REGION3_CW1_TOP_ADDRESS, (*cw1).region.top, DMCUB_REGION3_CW1_ENABLE, 1);
    REG_UPDATE_2!(DMCUB_SEC_CNTL, DMCUB_SEC_RESET, 0, DMCUB_MEM_UNIT_ID, 0x20);
}

pub unsafe fn dmub_dcn31_setup_windows(dmub: *mut dmub_srv, cw2: *const dmub_window, cw3: *const dmub_window, cw4: *const dmub_window, cw5: *const dmub_window, cw6: *const dmub_window, region6: *const dmub_window) {
    let _ = (dmub, cw2, region6);
    let mut offset = *cw3.offset(0).offset_addr();
    let windows = [(3, cw3), (4, cw4), (5, cw5), (6, cw6)];
    for &(n, cw) in &windows {
        offset = (*cw).offset;
        REG_WRITE!(REGION3_CW_OFFSET!(n), offset.u.low_part); REG_WRITE!(REGION3_CW_OFFSET_HIGH!(n), offset.u.high_part);
        REG_WRITE!(REGION3_CW_BASE_ADDRESS!(n), (*cw).region.base);
        REG_SET_2!(REGION3_CW_TOP_ADDRESS!(n), 0, REGION3_CW_TOP_ADDRESS!(n), (*cw).region.top, REGION3_CW_ENABLE!(n), 1);
        if n == 5 { REG_WRITE!(DMCUB_REGION5_OFFSET, offset.u.low_part); REG_WRITE!(DMCUB_REGION5_OFFSET_HIGH, offset.u.high_part); REG_SET_2!(DMCUB_REGION5_TOP_ADDRESS, 0, DMCUB_REGION5_TOP_ADDRESS, (*cw).region.top - (*cw).region.base - 1, DMCUB_REGION5_ENABLE, 1); }
    }
}

pub unsafe fn dmub_dcn31_setup_mailbox(dmub: *mut dmub_srv, inbox1: *const dmub_region) { REG_WRITE!(DMCUB_INBOX1_BASE_ADDRESS, (*inbox1).base); REG_WRITE!(DMCUB_INBOX1_SIZE, (*inbox1).top - (*inbox1).base); }
pub unsafe fn dmub_dcn31_get_inbox1_wptr(dmub: *mut dmub_srv) -> u32 { REG_READ!(DMCUB_INBOX1_WPTR) }
pub unsafe fn dmub_dcn31_get_inbox1_rptr(dmub: *mut dmub_srv) -> u32 { REG_READ!(DMCUB_INBOX1_RPTR) }
pub unsafe fn dmub_dcn31_set_inbox1_wptr(dmub: *mut dmub_srv, wptr_offset: u32) { REG_WRITE!(DMCUB_INBOX1_WPTR, wptr_offset); }
pub unsafe fn dmub_dcn31_setup_out_mailbox(dmub: *mut dmub_srv, outbox1: *const dmub_region) { REG_WRITE!(DMCUB_OUTBOX1_BASE_ADDRESS, (*outbox1).base); REG_WRITE!(DMCUB_OUTBOX1_SIZE, (*outbox1).top - (*outbox1).base); }
pub unsafe fn dmub_dcn31_get_outbox1_wptr(dmub: *mut dmub_srv) -> u32 { REG_READ!(DMCUB_OUTBOX1_WPTR) }
pub unsafe fn dmub_dcn31_set_outbox1_rptr(dmub: *mut dmub_srv, rptr_offset: u32) { REG_WRITE!(DMCUB_OUTBOX1_RPTR, rptr_offset); }
pub unsafe fn dmub_dcn31_is_hw_init(dmub: *mut dmub_srv) -> bool { let mut status: dmub_fw_boot_status = core::mem::zeroed(); let mut enable=0; status.all=REG_READ!(DMCUB_SCRATCH0); REG_GET!(DMCUB_CNTL,DMCUB_ENABLE,&mut enable); enable != 0 && status.bits.dal_fw }
pub unsafe fn dmub_dcn31_is_supported(dmub: *mut dmub_srv) -> bool { let mut supported=0; REG_GET!(CC_DC_PIPE_DIS,DC_DMCUB_ENABLE,&mut supported); supported != 0 }
pub unsafe fn dmub_dcn31_is_psrsu_supported(dmub: *mut dmub_srv) -> bool { (*dmub).fw_version >= DMUB_FW_VERSION!(4,0,59) }
pub unsafe fn dmub_dcn31_set_gpint(dmub: *mut dmub_srv, reg: dmub_gpint_data_register) { REG_WRITE!(DMCUB_GPINT_DATAIN1, reg.all); }
pub unsafe fn dmub_dcn31_is_gpint_acked(dmub: *mut dmub_srv, mut reg: dmub_gpint_data_register) -> bool { let mut test: dmub_gpint_data_register=core::mem::zeroed(); reg.bits.status=0; test.all=REG_READ!(DMCUB_GPINT_DATAIN1); test.all==reg.all }
pub unsafe fn dmub_dcn31_get_gpint_response(dmub: *mut dmub_srv) -> u32 { REG_READ!(DMCUB_SCRATCH7) }
pub unsafe fn dmub_dcn31_get_gpint_dataout(dmub: *mut dmub_srv) -> u32 { let dataout=REG_READ!(DMCUB_GPINT_DATAOUT); REG_UPDATE!(DMCUB_INTERRUPT_ENABLE,DMCUB_GPINT_IH_INT_EN,0); REG_WRITE!(DMCUB_GPINT_DATAOUT,0); REG_UPDATE!(DMCUB_INTERRUPT_ACK,DMCUB_GPINT_IH_INT_ACK,1); REG_UPDATE!(DMCUB_INTERRUPT_ACK,DMCUB_GPINT_IH_INT_ACK,0); REG_UPDATE!(DMCUB_INTERRUPT_ENABLE,DMCUB_GPINT_IH_INT_EN,1); dataout }
pub unsafe fn dmub_dcn31_get_fw_boot_status(dmub: *mut dmub_srv) -> dmub_fw_boot_status { let mut status=core::mem::zeroed(); status.all=REG_READ!(DMCUB_SCRATCH0); status }
pub unsafe fn dmub_dcn31_get_fw_boot_option(dmub: *mut dmub_srv) -> dmub_fw_boot_options { let mut option=core::mem::zeroed(); option.all=REG_READ!(DMCUB_SCRATCH14); option }

pub unsafe fn dmub_dcn31_enable_dmub_boot_options(dmub: *mut dmub_srv, params: *const dmub_srv_hw_params) {
    let mut boot_options: dmub_fw_boot_options=core::mem::zeroed();
    boot_options.bits.z10_disable=(*params).disable_z10; boot_options.bits.dpia_supported=(*params).dpia_supported; boot_options.bits.enable_dpia=if (*params).disable_dpia {0} else {1}; boot_options.bits.usb4_cm_version=(*params).usb4_cm_version; boot_options.bits.dpia_hpd_int_enable_supported=(*params).dpia_hpd_int_enable_supported; boot_options.bits.power_optimization=(*params).power_optimization; boot_options.bits.lower_hbr3_phy_ssc=(*params).lower_hbr3_phy_ssc; boot_options.bits.override_hbr3_pll_vco=(*params).override_hbr3_pll_vco; boot_options.bits.sel_mux_phy_c_d_phy_f_g=if (*dmub).asic==DMUB_ASIC_DCN31B {1} else {0}; boot_options.bits.disable_dpia_bw_allocation=(*params).disable_dpia_bw_allocation; REG_WRITE!(DMCUB_SCRATCH14,boot_options.all);
}
pub unsafe fn dmub_dcn31_skip_dmub_panel_power_sequence(dmub: *mut dmub_srv, skip: bool) { let mut boot_options:dmub_fw_boot_options=core::mem::zeroed(); boot_options.all=REG_READ!(DMCUB_SCRATCH14); boot_options.bits.skip_phy_init_panel_sequence=skip; REG_WRITE!(DMCUB_SCRATCH14,boot_options.all); }
pub unsafe fn dmub_dcn31_setup_outbox0(dmub: *mut dmub_srv, outbox0: *const dmub_region) { REG_WRITE!(DMCUB_OUTBOX0_BASE_ADDRESS,(*outbox0).base); REG_WRITE!(DMCUB_OUTBOX0_SIZE,(*outbox0).top-(*outbox0).base); }
pub unsafe fn dmub_dcn31_get_outbox0_wptr(dmub: *mut dmub_srv) -> u32 { REG_READ!(DMCUB_OUTBOX0_WPTR) }
pub unsafe fn dmub_dcn31_set_outbox0_rptr(dmub: *mut dmub_srv, rptr_offset:u32) { REG_WRITE!(DMCUB_OUTBOX0_RPTR,rptr_offset); }
pub unsafe fn dmub_dcn31_get_current_time(dmub:*mut dmub_srv)->u32 { REG_READ!(DMCUB_TIMER_CURRENT) }

pub unsafe fn dmub_dcn31_get_diagnostic_data(dmub:*mut dmub_srv) {
    if dmub.is_null() { return; }
    let timeout=(*dmub).debug.timeout_info; (*dmub).debug=core::mem::zeroed(); (*dmub).debug.timeout_info=timeout; (*dmub).debug.dmcub_version=(*dmub).fw_version;
    for i in 0..16 { (*dmub).debug.scratch[i]=REG_READ!(SCRATCH!(i)); }
    (*dmub).debug.undefined_address_fault_addr=REG_READ!(DMCUB_UNDEFINED_ADDRESS_FAULT_ADDR); (*dmub).debug.inst_fetch_fault_addr=REG_READ!(DMCUB_INST_FETCH_FAULT_ADDR); (*dmub).debug.data_write_fault_addr=REG_READ!(DMCUB_DATA_WRITE_FAULT_ADDR);
    (*dmub).debug.inbox1_rptr=REG_READ!(DMCUB_INBOX1_RPTR); (*dmub).debug.inbox1_wptr=REG_READ!(DMCUB_INBOX1_WPTR); (*dmub).debug.inbox1_size=REG_READ!(DMCUB_INBOX1_SIZE); (*dmub).debug.inbox0_rptr=REG_READ!(DMCUB_INBOX0_RPTR); (*dmub).debug.inbox0_wptr=REG_READ!(DMCUB_INBOX0_WPTR); (*dmub).debug.inbox0_size=REG_READ!(DMCUB_INBOX0_SIZE); (*dmub).debug.outbox1_rptr=REG_READ!(DMCUB_OUTBOX1_RPTR); (*dmub).debug.outbox1_wptr=REG_READ!(DMCUB_OUTBOX1_WPTR); (*dmub).debug.outbox1_size=REG_READ!(DMCUB_OUTBOX1_SIZE);
    let mut v=0; REG_GET!(DMCUB_CNTL,DMCUB_ENABLE,&mut v); (*dmub).debug.is_dmcub_enabled=v as u8; REG_GET!(DMCUB_CNTL,DMCUB_PWAIT_MODE_STATUS,&mut v); (*dmub).debug.is_pwait=v as u8; REG_GET!(DMCUB_CNTL2,DMCUB_SOFT_RESET,&mut v); (*dmub).debug.is_dmcub_soft_reset=v as u8; REG_GET!(DMCUB_SEC_CNTL,DMCUB_SEC_RESET_STATUS,&mut v); (*dmub).debug.is_dmcub_secure_reset=v as u8; REG_GET!(DMCUB_CNTL,DMCUB_TRACEPORT_EN,&mut v); (*dmub).debug.is_traceport_en=v as u8; REG_GET!(DMCUB_REGION3_CW0_TOP_ADDRESS,DMCUB_REGION3_CW0_ENABLE,&mut v); (*dmub).debug.is_cw0_enabled=v as u8; REG_GET!(DMCUB_REGION3_CW6_TOP_ADDRESS,DMCUB_REGION3_CW6_ENABLE,&mut v); (*dmub).debug.is_cw6_enabled=v as u8;
}
pub unsafe fn dmub_dcn31_should_detect(dmub:*mut dmub_srv)->bool { REG_READ!(DMCUB_SCRATCH0) & DMUB_FW_BOOT_STATUS_BIT_DETECTION_REQUIRED != 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
