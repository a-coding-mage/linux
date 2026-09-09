// SPDX-License-Identifier: MIT
// Copyright 2025 Advanced Micro Devices, Inc.

// Register definitions and helper macros are supplied by the surrounding project.

const DCN_BASE_INST0_SEG2: u32 = 0x0000_34C0;
const MMHUB_BASE_INST0_SEG1: u32 = 0x0001_A000;
const DAGB0_WRCLI_OSD_PENDING: u32 = 0x1A083;

#[inline]
unsafe fn dmub_dcn60_get_fb_base_offset(
    dmub: *mut dmub_srv,
    fb_base: *mut u64,
    fb_offset: *mut u64,
) {
    let mut tmp_fb_base: u32 = 0;
    let mut tmp_fb_offset: u32 = 0;
    if (*dmub).soc_fb_info.fb_base != 0 || (*dmub).soc_fb_info.fb_offset != 0 {
        *fb_base = (*dmub).soc_fb_info.fb_base;
        *fb_offset = (*dmub).soc_fb_info.fb_offset;
        return;
    }
    REG_GET!(dmub, DCN_VM_FB_LOCATION_BASE, FB_BASE, &mut tmp_fb_base);
    REG_GET!(dmub, DCN_VM_FB_OFFSET, FB_OFFSET, &mut tmp_fb_offset);
    if !(*dmub).no_ext_reg_access && tmp_fb_base == 0 && tmp_fb_offset == 0 {
        REG_GET!(dmub, MMMC_VM_FB_LOCATION_BASE, FB_BASE, &mut tmp_fb_base);
        REG_GET!(dmub, MMMC_VM_FB_OFFSET, FB_OFFSET, &mut tmp_fb_offset);
    }
    *fb_base = (tmp_fb_base as u64) << 24;
    *fb_offset = (tmp_fb_offset as u64) << 24;
}

#[inline]
unsafe fn dmub_dcn60_translate_addr(
    addr_in: *const dmub_addr,
    fb_base: u64,
    fb_offset: u64,
    addr_out: *mut dmub_addr,
) {
    (*addr_out).quad_part = (*addr_in).quad_part.wrapping_sub(fb_base).wrapping_add(fb_offset);
}

pub unsafe fn dmub_dcn60_reset(dmub: *mut dmub_srv) {
    let mut cmd: dmub_gpint_data_register = core::mem::zeroed();
    let timeout_us: u32 = 1 * 1000 * 1000;
    let poll_delay_us: u32 = 1;
    let mut i: u32 = 0;
    let (mut enabled, mut in_reset, mut scratch, mut pwait_mode, mut outstanding_req) = (0, 0, 0, 0, 0);
    REG_GET!(dmub, DMCUB_CNTL, DMCUB_ENABLE, &mut enabled);
    REG_GET!(dmub, DMCUB_CNTL2, DMCUB_SOFT_RESET, &mut in_reset);
    if enabled != 0 && in_reset == 0 {
        cmd.bits.status = 1;
        cmd.bits.command_code = DMUB_GPINT__STOP_FW;
        cmd.bits.param = 0;
        (*dmub).hw_funcs.set_gpint(dmub, cmd);
        while i < timeout_us {
            scratch = REG_READ!(dmub, DMCUB_SCRATCH7);
            if scratch == DMUB_GPINT__STOP_FW_RESPONSE { break; }
            udelay(poll_delay_us);
            i += 1;
        }
        while i < timeout_us {
            REG_GET!(dmub, DMCUB_CNTL, DMCUB_PWAIT_MODE_STATUS, &mut pwait_mode);
            if pwait_mode & (1 << 0) != 0 { break; }
            udelay(poll_delay_us);
            i += 1;
        }
        if !(*dmub).no_ext_reg_access {
            while i < timeout_us {
                outstanding_req = (*dmub).funcs.reg_read((*dmub).user_ctx, DAGB0_WRCLI_OSD_PENDING);
                if outstanding_req & 0x10 == 0 { break; }
                udelay(poll_delay_us);
                i += 1;
            }
        }
    }
    if enabled != 0 {
        REG_UPDATE!(dmub, DMCUB_CNTL2, DMCUB_SOFT_RESET, 1);
        udelay(1);
        REG_UPDATE!(dmub, DMCUB_CNTL, DMCUB_ENABLE, 0);
    }
    if i >= timeout_us { BREAK_TO_DEBUGGER!(); }
    REG_UPDATE!(dmub, DMCUB_REGION3_CW2_TOP_ADDRESS, DMCUB_REGION3_CW2_ENABLE, 0);
    REG_UPDATE!(dmub, DMCUB_REGION3_CW3_TOP_ADDRESS, DMCUB_REGION3_CW3_ENABLE, 0);
    REG_UPDATE!(dmub, DMCUB_REGION3_CW4_TOP_ADDRESS, DMCUB_REGION3_CW4_ENABLE, 0);
    REG_UPDATE!(dmub, DMCUB_REGION3_CW5_TOP_ADDRESS, DMCUB_REGION3_CW5_ENABLE, 0);
    REG_UPDATE!(dmub, DMCUB_REGION3_CW6_TOP_ADDRESS, DMCUB_REGION3_CW6_ENABLE, 0);
    REG_UPDATE!(dmub, DMCUB_REGION3_CW7_TOP_ADDRESS, DMCUB_REGION3_CW7_ENABLE, 0);
    REG_WRITE!(dmub, DMCUB_INBOX1_RPTR, 0); REG_WRITE!(dmub, DMCUB_INBOX1_WPTR, 0);
    REG_WRITE!(dmub, DMCUB_OUTBOX1_RPTR, 0); REG_WRITE!(dmub, DMCUB_OUTBOX1_WPTR, 0);
    REG_WRITE!(dmub, DMCUB_OUTBOX0_RPTR, 0); REG_WRITE!(dmub, DMCUB_OUTBOX0_WPTR, 0);
    REG_WRITE!(dmub, DMCUB_SCRATCH0, 0);
    cmd.all = 0;
    (*dmub).hw_funcs.set_gpint(dmub, cmd);
}

pub unsafe fn dmub_dcn60_reset_release(dmub: *mut dmub_srv) {
    REG_UPDATE!(dmub, MMHUBBUB_SOFT_RESET, DMUIF_SOFT_RESET, 0);
    REG_WRITE!(dmub, DMCUB_SCRATCH15, (*dmub).psp_version & 0x0011_00FF);
    REG_UPDATE_2!(dmub, DMCUB_CNTL, DMCUB_ENABLE, 1, DMCUB_TRACEPORT_EN, 1);
    REG_UPDATE!(dmub, DMCUB_CNTL2, DMCUB_SOFT_RESET, 0);
}

pub unsafe fn dmub_dcn60_backdoor_load(dmub: *mut dmub_srv, cw0: *const dmub_window, cw1: *const dmub_window) {
    let (mut fb_base, mut fb_offset) = (0u64, 0u64); let mut offset: dmub_addr = core::mem::zeroed();
    dmub_dcn60_get_fb_base_offset(dmub, &mut fb_base, &mut fb_offset);
    REG_UPDATE!(dmub, DMCUB_SEC_CNTL, DMCUB_SEC_RESET, 1); REG_UPDATE!(dmub, DMCUB_CNTL, DMCUB_ENABLE, 0);
    dmub_dcn60_translate_addr(&(*cw0).offset, fb_base, fb_offset, &mut offset);
    REG_WRITE!(dmub, DMCUB_REGION3_CW0_OFFSET, offset.u.low_part); REG_WRITE!(dmub, DMCUB_REGION3_CW0_OFFSET_HIGH, offset.u.high_part);
    REG_WRITE!(dmub, DMCUB_REGION3_CW0_BASE_ADDRESS, (*cw0).region.base);
    REG_SET_2!(dmub, DMCUB_REGION3_CW0_TOP_ADDRESS, 0, DMCUB_REGION3_CW0_TOP_ADDRESS, (*cw0).region.top, DMCUB_REGION3_CW0_ENABLE, 1);
    dmub_dcn60_translate_addr(&(*cw1).offset, fb_base, fb_offset, &mut offset);
    REG_WRITE!(dmub, DMCUB_REGION3_CW1_OFFSET, offset.u.low_part); REG_WRITE!(dmub, DMCUB_REGION3_CW1_OFFSET_HIGH, offset.u.high_part);
    REG_WRITE!(dmub, DMCUB_REGION3_CW1_BASE_ADDRESS, (*cw1).region.base);
    REG_SET_2!(dmub, DMCUB_REGION3_CW1_TOP_ADDRESS, 0, DMCUB_REGION3_CW1_TOP_ADDRESS, (*cw1).region.top, DMCUB_REGION3_CW1_ENABLE, 1);
    REG_UPDATE_3!(dmub, DMCUB_SEC_CNTL, DMCUB_SEC_RESET, 0, DMCUB_MEM_SEC_LVL, 0x2, DMCUB_MEM_UNIT_ID, 0x20);
}

pub unsafe fn dmub_dcn60_backdoor_load_zfb_mode(dmub: *mut dmub_srv, cw0: *const dmub_window, cw1: *const dmub_window) {
    let mut offset: dmub_addr = core::mem::zeroed();
    REG_UPDATE!(dmub, DMCUB_SEC_CNTL, DMCUB_SEC_RESET, 1); REG_UPDATE!(dmub, DMCUB_CNTL, DMCUB_ENABLE, 0);
    offset = (*cw0).offset; REG_WRITE!(dmub, DMCUB_REGION3_CW0_OFFSET, offset.u.low_part); REG_WRITE!(dmub, DMCUB_REGION3_CW0_OFFSET_HIGH, offset.u.high_part); REG_WRITE!(dmub, DMCUB_REGION3_CW0_BASE_ADDRESS, (*cw0).region.base); REG_SET_2!(dmub, DMCUB_REGION3_CW0_TOP_ADDRESS, 0, DMCUB_REGION3_CW0_TOP_ADDRESS, (*cw0).region.top, DMCUB_REGION3_CW0_ENABLE, 1);
    offset = (*cw1).offset; REG_WRITE!(dmub, DMCUB_REGION3_CW1_OFFSET, offset.u.low_part); REG_WRITE!(dmub, DMCUB_REGION3_CW1_OFFSET_HIGH, offset.u.high_part); REG_WRITE!(dmub, DMCUB_REGION3_CW1_BASE_ADDRESS, (*cw1).region.base); REG_SET_2!(dmub, DMCUB_REGION3_CW1_TOP_ADDRESS, 0, DMCUB_REGION3_CW1_TOP_ADDRESS, (*cw1).region.top, DMCUB_REGION3_CW1_ENABLE, 1);
    REG_UPDATE_3!(dmub, DMCUB_SEC_CNTL, DMCUB_SEC_RESET, 0, DMCUB_MEM_SEC_LVL, 0x2, DMCUB_MEM_UNIT_ID, 0x20);
}

pub unsafe fn dmub_dcn60_setup_windows(dmub: *mut dmub_srv, _cw2: *const dmub_window, cw3: *const dmub_window, cw4: *const dmub_window, cw5: *const dmub_window, cw6: *const dmub_window, region6: *const dmub_window) {
    let mut offset: dmub_addr;
    macro_rules! window { ($n:literal, $w:expr) => { offset = (*$w).offset; REG_WRITE!(dmub, concat_idents!(DMCUB_REGION3_CW, $n, _OFFSET), offset.u.low_part); REG_WRITE!(dmub, concat_idents!(DMCUB_REGION3_CW, $n, _OFFSET_HIGH), offset.u.high_part); REG_WRITE!(dmub, concat_idents!(DMCUB_REGION3_CW, $n, _BASE_ADDRESS), (*$w).region.base); REG_SET_2!(dmub, concat_idents!(DMCUB_REGION3_CW, $n, _TOP_ADDRESS), 0, concat_idents!(DMCUB_REGION3_CW, $n, _TOP_ADDRESS), (*$w).region.top, concat_idents!(DMCUB_REGION3_CW, $n, _ENABLE), 1); }; }
    window!(3, cw3); window!(4, cw4); window!(5, cw5); window!(6, cw6);
    REG_WRITE!(dmub, DMCUB_REGION5_OFFSET, offset.u.low_part); REG_WRITE!(dmub, DMCUB_REGION5_OFFSET_HIGH, offset.u.high_part); REG_SET_2!(dmub, DMCUB_REGION5_TOP_ADDRESS, 0, DMCUB_REGION5_TOP_ADDRESS, (*cw5).region.top - (*cw5).region.base - 1, DMCUB_REGION5_ENABLE, 1);
    offset = (*region6).offset; REG_WRITE!(dmub, DMCUB_REGION6_OFFSET, offset.u.low_part); REG_WRITE!(dmub, DMCUB_REGION6_OFFSET_HIGH, offset.u.high_part); REG_SET_2!(dmub, DMCUB_REGION6_TOP_ADDRESS, 0, DMCUB_REGION6_TOP_ADDRESS, (*region6).region.top - (*region6).region.base - 1, DMCUB_REGION6_ENABLE, 1);
}

pub unsafe fn dmub_dcn60_setup_mailbox(dmub: *mut dmub_srv, inbox1: *const dmub_region) { REG_WRITE!(dmub, DMCUB_INBOX1_BASE_ADDRESS, (*inbox1).base); REG_WRITE!(dmub, DMCUB_INBOX1_SIZE, (*inbox1).top - (*inbox1).base); }
pub unsafe fn dmub_dcn60_get_inbox1_wptr(dmub: *mut dmub_srv) -> u32 { REG_READ!(dmub, DMCUB_INBOX1_WPTR) }
pub unsafe fn dmub_dcn60_get_inbox1_rptr(dmub: *mut dmub_srv) -> u32 { REG_READ!(dmub, DMCUB_INBOX1_RPTR) }
pub unsafe fn dmub_dcn60_set_inbox1_wptr(dmub: *mut dmub_srv, wptr_offset: u32) { REG_WRITE!(dmub, DMCUB_INBOX1_WPTR, wptr_offset); }
pub unsafe fn dmub_dcn60_setup_out_mailbox(dmub: *mut dmub_srv, outbox1: *const dmub_region) { REG_WRITE!(dmub, DMCUB_OUTBOX1_BASE_ADDRESS, (*outbox1).base); REG_WRITE!(dmub, DMCUB_OUTBOX1_SIZE, (*outbox1).top - (*outbox1).base); }
pub unsafe fn dmub_dcn60_get_outbox1_wptr(dmub: *mut dmub_srv) -> u32 { REG_READ!(dmub, DMCUB_OUTBOX1_WPTR) }
pub unsafe fn dmub_dcn60_set_outbox1_rptr(dmub: *mut dmub_srv, rptr_offset: u32) { REG_WRITE!(dmub, DMCUB_OUTBOX1_RPTR, rptr_offset); }

pub unsafe fn dmub_dcn60_is_hw_init(dmub: *mut dmub_srv) -> bool { let mut status: dmub_fw_boot_status = core::mem::zeroed(); let mut init = 0; status.all = REG_READ!(dmub, DMCUB_SCRATCH0); REG_GET!(dmub, DMCUB_CNTL, DMCUB_ENABLE, &mut init); init != 0 && status.bits.dal_fw }
pub unsafe fn dmub_dcn60_is_supported(_dmub: *mut dmub_srv) -> bool { true }
pub unsafe fn dmub_dcn60_set_gpint(dmub: *mut dmub_srv, reg: dmub_gpint_data_register) { REG_WRITE!(dmub, DMCUB_GPINT_DATAIN1, reg.all); }
pub unsafe fn dmub_dcn60_is_gpint_acked(dmub: *mut dmub_srv, mut reg: dmub_gpint_data_register) -> bool { reg.bits.status = 0; let mut test: dmub_gpint_data_register = core::mem::zeroed(); test.all = REG_READ!(dmub, DMCUB_GPINT_DATAIN1); test.all == reg.all }
pub unsafe fn dmub_dcn60_get_gpint_response(dmub: *mut dmub_srv) -> u32 { REG_READ!(dmub, DMCUB_SCRATCH7) }
pub unsafe fn dmub_dcn60_get_gpint_dataout(dmub: *mut dmub_srv) -> u32 { let dataout = REG_READ!(dmub, DMCUB_GPINT_DATAOUT); REG_UPDATE!(dmub, DMCUB_INTERRUPT_ENABLE, DMCUB_GPINT_IH_INT_EN, 0); REG_WRITE!(dmub, DMCUB_GPINT_DATAOUT, 0); REG_UPDATE!(dmub, DMCUB_INTERRUPT_ACK, DMCUB_GPINT_IH_INT_ACK, 1); REG_UPDATE!(dmub, DMCUB_INTERRUPT_ACK, DMCUB_GPINT_IH_INT_ACK, 0); REG_UPDATE!(dmub, DMCUB_INTERRUPT_ENABLE, DMCUB_GPINT_IH_INT_EN, 1); dataout }
pub unsafe fn dmub_dcn60_get_fw_boot_status(dmub: *mut dmub_srv) -> dmub_fw_boot_status { let mut status: dmub_fw_boot_status = core::mem::zeroed(); status.all = REG_READ!(dmub, DMCUB_SCRATCH0); status }
pub unsafe fn dmub_dcn60_enable_dmub_boot_options(dmub: *mut dmub_srv, params: *const dmub_srv_hw_params) { let mut options: dmub_fw_boot_options = core::mem::zeroed(); options.bits.z10_disable = (*params).disable_z10; options.bits.skip_phy_access = (*params).disallow_phy_access; REG_WRITE!(dmub, DMCUB_SCRATCH14, options.all); }
pub unsafe fn dmub_dcn60_skip_dmub_panel_power_sequence(dmub: *mut dmub_srv, skip: bool) { let mut options: dmub_fw_boot_options = core::mem::zeroed(); options.all = REG_READ!(dmub, DMCUB_SCRATCH14); options.bits.skip_phy_init_panel_sequence = skip; REG_WRITE!(dmub, DMCUB_SCRATCH14, options.all); }
pub unsafe fn dmub_dcn60_setup_outbox0(dmub: *mut dmub_srv, outbox0: *const dmub_region) { REG_WRITE!(dmub, DMCUB_OUTBOX0_BASE_ADDRESS, (*outbox0).base); REG_WRITE!(dmub, DMCUB_OUTBOX0_SIZE, (*outbox0).top - (*outbox0).base); }
pub unsafe fn dmub_dcn60_get_outbox0_wptr(dmub: *mut dmub_srv) -> u32 { REG_READ!(dmub, DMCUB_OUTBOX0_WPTR) }
pub unsafe fn dmub_dcn60_set_outbox0_rptr(dmub: *mut dmub_srv, rptr_offset: u32) { REG_WRITE!(dmub, DMCUB_OUTBOX0_RPTR, rptr_offset); }
pub unsafe fn dmub_dcn60_get_current_time(dmub: *mut dmub_srv) -> u32 { REG_READ!(dmub, DMCUB_TIMER_CURRENT) }

pub unsafe fn dmub_dcn60_get_diagnostic_data(dmub: *mut dmub_srv) {
    if dmub.is_null() { return; }
    let timeout = (*dmub).debug.timeout_info;
    core::ptr::write_bytes(&mut (*dmub).debug, 0, 1);
    (*dmub).debug.timeout_info = timeout;
    (*dmub).debug.dmcub_version = (*dmub).fw_version;
    (*dmub).debug.scratch[0] = REG_READ!(dmub, DMCUB_SCRATCH0); (*dmub).debug.scratch[1] = REG_READ!(dmub, DMCUB_SCRATCH1); (*dmub).debug.scratch[2] = REG_READ!(dmub, DMCUB_SCRATCH2); (*dmub).debug.scratch[3] = REG_READ!(dmub, DMCUB_SCRATCH3);
    (*dmub).debug.scratch[4] = REG_READ!(dmub, DMCUB_SCRATCH4); (*dmub).debug.scratch[5] = REG_READ!(dmub, DMCUB_SCRATCH5); (*dmub).debug.scratch[6] = REG_READ!(dmub, DMCUB_SCRATCH6); (*dmub).debug.scratch[7] = REG_READ!(dmub, DMCUB_SCRATCH7);
    (*dmub).debug.scratch[8] = REG_READ!(dmub, DMCUB_SCRATCH8); (*dmub).debug.scratch[9] = REG_READ!(dmub, DMCUB_SCRATCH9); (*dmub).debug.scratch[10] = REG_READ!(dmub, DMCUB_SCRATCH10); (*dmub).debug.scratch[11] = REG_READ!(dmub, DMCUB_SCRATCH11);
    (*dmub).debug.scratch[12] = REG_READ!(dmub, DMCUB_SCRATCH12); (*dmub).debug.scratch[13] = REG_READ!(dmub, DMCUB_SCRATCH13); (*dmub).debug.scratch[14] = REG_READ!(dmub, DMCUB_SCRATCH14); (*dmub).debug.scratch[15] = REG_READ!(dmub, DMCUB_SCRATCH15); (*dmub).debug.scratch[16] = REG_READ!(dmub, DMCUB_SCRATCH16);
    (*dmub).debug.undefined_address_fault_addr = REG_READ!(dmub, DMCUB_UNDEFINED_ADDRESS_FAULT_ADDR); (*dmub).debug.inst_fetch_fault_addr = REG_READ!(dmub, DMCUB_INST_FETCH_FAULT_ADDR); (*dmub).debug.data_write_fault_addr = REG_READ!(dmub, DMCUB_DATA_WRITE_FAULT_ADDR);
    (*dmub).debug.inbox1_rptr = REG_READ!(dmub, DMCUB_INBOX1_RPTR); (*dmub).debug.inbox1_wptr = REG_READ!(dmub, DMCUB_INBOX1_WPTR); (*dmub).debug.inbox1_size = REG_READ!(dmub, DMCUB_INBOX1_SIZE);
    (*dmub).debug.inbox0_rptr = REG_READ!(dmub, DMCUB_INBOX0_RPTR); (*dmub).debug.inbox0_wptr = REG_READ!(dmub, DMCUB_INBOX0_WPTR); (*dmub).debug.inbox0_size = REG_READ!(dmub, DMCUB_INBOX0_SIZE);
    (*dmub).debug.outbox1_rptr = REG_READ!(dmub, DMCUB_OUTBOX1_RPTR); (*dmub).debug.outbox1_wptr = REG_READ!(dmub, DMCUB_OUTBOX1_WPTR); (*dmub).debug.outbox1_size = REG_READ!(dmub, DMCUB_OUTBOX1_SIZE);
    let (mut a, mut b, mut c, mut d, mut e, mut f, mut g) = (0,0,0,0,0,0,0);
    REG_GET!(dmub, DMCUB_CNTL, DMCUB_ENABLE, &mut a); (*dmub).debug.is_dmcub_enabled = a as u8;
    REG_GET!(dmub, DMCUB_CNTL, DMCUB_PWAIT_MODE_STATUS, &mut b); (*dmub).debug.is_pwait = b as u8;
    REG_GET!(dmub, DMCUB_CNTL2, DMCUB_SOFT_RESET, &mut c); (*dmub).debug.is_dmcub_soft_reset = c as u8;
    REG_GET!(dmub, DMCUB_SEC_CNTL, DMCUB_SEC_RESET_STATUS, &mut d); (*dmub).debug.is_dmcub_secure_reset = d as u8;
    REG_GET!(dmub, DMCUB_CNTL, DMCUB_TRACEPORT_EN, &mut e); (*dmub).debug.is_traceport_en = e as u8;
    REG_GET!(dmub, DMCUB_REGION3_CW0_TOP_ADDRESS, DMCUB_REGION3_CW0_ENABLE, &mut f); (*dmub).debug.is_cw0_enabled = f as u8;
    REG_GET!(dmub, DMCUB_REGION3_CW6_TOP_ADDRESS, DMCUB_REGION3_CW6_ENABLE, &mut g); (*dmub).debug.is_cw6_enabled = g as u8;
    (*dmub).debug.gpint_datain0 = REG_READ!(dmub, DMCUB_GPINT_DATAIN0);
}

pub unsafe fn dmub_dcn60_configure_dmub_in_system_memory(dmub: *mut dmub_srv) { REG_WRITE!(dmub, DMCUB_REGION3_TMR_AXI_SPACE, 0x4); }
pub unsafe fn dmub_dcn60_send_inbox0_cmd(dmub: *mut dmub_srv, data: dmub_inbox0_data_register) { REG_WRITE!(dmub, DMCUB_INBOX0_WPTR, data.inbox0_cmd_common.all); }
pub unsafe fn dmub_dcn60_clear_inbox0_ack_register(dmub: *mut dmub_srv) { REG_WRITE!(dmub, DMCUB_SCRATCH17, 0); }
pub unsafe fn dmub_dcn60_read_inbox0_ack_register(dmub: *mut dmub_srv) -> u32 { REG_READ!(dmub, DMCUB_SCRATCH17) }

pub unsafe fn dmub_dcn60_send_reg_inbox0_cmd_msg(dmub: *mut dmub_srv, cmd: *mut dmub_rb_cmd) {
    let dwords = cmd as *mut u32; let payload_size_bytes = (*cmd).cmd_common.header.payload_bytes;
    for msg_index in 0..15u32 { if payload_size_bytes <= msg_index * 4 { break; } REG_WRITE!(dmub, match msg_index { 0=>DMCUB_REG_INBOX0_MSG0,1=>DMCUB_REG_INBOX0_MSG1,2=>DMCUB_REG_INBOX0_MSG2,3=>DMCUB_REG_INBOX0_MSG3,4=>DMCUB_REG_INBOX0_MSG4,5=>DMCUB_REG_INBOX0_MSG5,6=>DMCUB_REG_INBOX0_MSG6,7=>DMCUB_REG_INBOX0_MSG7,8=>DMCUB_REG_INBOX0_MSG8,9=>DMCUB_REG_INBOX0_MSG9,10=>DMCUB_REG_INBOX0_MSG10,11=>DMCUB_REG_INBOX0_MSG11,12=>DMCUB_REG_INBOX0_MSG12,13=>DMCUB_REG_INBOX0_MSG13,_=>DMCUB_REG_INBOX0_MSG14 }, *dwords.add((msg_index+1) as usize)); }
    REG_WRITE!(dmub, DMCUB_REG_INBOX0_RDY, *dwords);
}
pub unsafe fn dmub_dcn60_read_reg_inbox0_rsp_int_status(dmub: *mut dmub_srv) -> u32 { let mut status=0; REG_GET!(dmub, HOST_INTERRUPT_CSR, HOST_REG_INBOX0_RSP_INT_STAT, &mut status); status }
pub unsafe fn dmub_dcn60_read_reg_inbox0_cmd_rsp(dmub: *mut dmub_srv, cmd: *mut dmub_rb_cmd) { let dwords=cmd as *mut u32; *dwords=REG_READ!(dmub,DMCUB_REG_INBOX0_RSP); for i in 0..15 { *dwords.add(i+1)=REG_READ!(dmub, match i {0=>DMCUB_REG_INBOX0_MSG0,1=>DMCUB_REG_INBOX0_MSG1,2=>DMCUB_REG_INBOX0_MSG2,3=>DMCUB_REG_INBOX0_MSG3,4=>DMCUB_REG_INBOX0_MSG4,5=>DMCUB_REG_INBOX0_MSG5,6=>DMCUB_REG_INBOX0_MSG6,7=>DMCUB_REG_INBOX0_MSG7,8=>DMCUB_REG_INBOX0_MSG8,9=>DMCUB_REG_INBOX0_MSG9,10=>DMCUB_REG_INBOX0_MSG10,11=>DMCUB_REG_INBOX0_MSG11,12=>DMCUB_REG_INBOX0_MSG12,13=>DMCUB_REG_INBOX0_MSG13,_=>DMCUB_REG_INBOX0_MSG14}); } }
pub unsafe fn dmub_dcn60_write_reg_inbox0_rsp_int_ack(dmub:*mut dmub_srv){REG_UPDATE!(dmub,HOST_INTERRUPT_CSR,HOST_REG_INBOX0_RSP_INT_ACK,1)}
pub unsafe fn dmub_dcn60_clear_reg_inbox0_rsp_int_ack(dmub:*mut dmub_srv){REG_UPDATE!(dmub,HOST_INTERRUPT_CSR,HOST_REG_INBOX0_RSP_INT_ACK,0)}
pub unsafe fn dmub_dcn60_enable_reg_inbox0_rsp_int(dmub:*mut dmub_srv,enable:bool){REG_UPDATE!(dmub,HOST_INTERRUPT_CSR,HOST_REG_INBOX0_RSP_INT_EN,if enable{1}else{0})}
pub unsafe fn dmub_dcn60_write_reg_outbox0_rdy_int_ack(dmub:*mut dmub_srv){REG_UPDATE!(dmub,HOST_INTERRUPT_CSR,HOST_REG_OUTBOX0_RDY_INT_ACK,1);REG_UPDATE!(dmub,HOST_INTERRUPT_CSR,HOST_REG_OUTBOX0_RDY_INT_ACK,0)}
pub unsafe fn dmub_dcn60_read_reg_outbox0_msg(dmub:*mut dmub_srv,msg:*mut u32){*msg=REG_READ!(dmub,DMCUB_REG_OUTBOX0_MSG0)}
pub unsafe fn dmub_dcn60_write_reg_outbox0_rsp(dmub:*mut dmub_srv,rsp:*const u32){REG_WRITE!(dmub,DMCUB_REG_OUTBOX0_RSP,*rsp)}
pub unsafe fn dmub_dcn60_read_reg_outbox0_rsp_int_status(dmub:*mut dmub_srv)->u32{let mut s=0;REG_GET!(dmub,DMCUB_INTERRUPT_STATUS,DMCUB_REG_OUTBOX0_RSP_INT_STAT,&mut s);s}
pub unsafe fn dmub_dcn60_enable_reg_outbox0_rdy_int(dmub:*mut dmub_srv,enable:bool){REG_UPDATE!(dmub,HOST_INTERRUPT_CSR,HOST_REG_OUTBOX0_RDY_INT_EN,if enable{1}else{0})}
pub unsafe fn dmub_dcn60_read_reg_outbox0_rdy_int_status(dmub:*mut dmub_srv)->u32{let mut s=0;REG_GET!(dmub,HOST_INTERRUPT_CSR,HOST_REG_OUTBOX0_RDY_INT_STAT,&mut s);s}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
