// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2017 Pengutronix, Jan Luebbe <kernel@pengutronix.de>
 */

// Kernel dependencies supplied by the surrounding translation unit.

const SDRAM_NUM_CS: usize = 4;
const SDRAM_CONFIG_REG: usize = 0x0;
const SDRAM_CONFIG_ECC_MASK: u32 = 1 << 18;
const SDRAM_CONFIG_REGISTERED_MASK: u32 = 1 << 17;
const SDRAM_CONFIG_BUS_WIDTH_MASK: u32 = 1 << 15;
const SDRAM_ADDR_CTRL_REG: usize = 0x10;
const SDRAM_ERR_DATA_H_REG: usize = 0x40;
const SDRAM_ERR_DATA_L_REG: usize = 0x44;
const SDRAM_ERR_RECV_ECC_REG: usize = 0x48;
const SDRAM_ERR_RECV_ECC_VALUE_MASK: u32 = 0xff;
const SDRAM_ERR_CALC_ECC_REG: usize = 0x4c;
const SDRAM_ERR_CALC_ECC_ROW_OFFSET: u32 = 8;
const SDRAM_ERR_CALC_ECC_ROW_MASK: u32 = 0xffff << SDRAM_ERR_CALC_ECC_ROW_OFFSET;
const SDRAM_ERR_CALC_ECC_VALUE_MASK: u32 = 0xff;
const SDRAM_ERR_ADDR_REG: usize = 0x50;
const SDRAM_ERR_ADDR_BANK_OFFSET: u32 = 23;
const SDRAM_ERR_ADDR_BANK_MASK: u32 = 0x7 << SDRAM_ERR_ADDR_BANK_OFFSET;
const SDRAM_ERR_ADDR_COL_OFFSET: u32 = 8;
const SDRAM_ERR_ADDR_COL_MASK: u32 = 0x7fff << SDRAM_ERR_ADDR_COL_OFFSET;
const SDRAM_ERR_ADDR_CS_OFFSET: u32 = 1;
const SDRAM_ERR_ADDR_CS_MASK: u32 = 0x3 << SDRAM_ERR_ADDR_CS_OFFSET;
const SDRAM_ERR_ADDR_TYPE_MASK: u32 = 1;
const SDRAM_ERR_CTRL_REG: usize = 0x54;
const SDRAM_ERR_CTRL_THR_OFFSET: u32 = 16;
const SDRAM_ERR_CTRL_THR_MASK: u32 = 0xff << SDRAM_ERR_CTRL_THR_OFFSET;
const SDRAM_ERR_CTRL_PROP_MASK: u32 = 1 << 9;
const SDRAM_ERR_SBE_COUNT_REG: usize = 0x58;
const SDRAM_ERR_DBE_COUNT_REG: usize = 0x5c;
const SDRAM_ERR_CAUSE_ERR_REG: usize = 0xd0;
const SDRAM_ERR_CAUSE_MSG_REG: usize = 0xd8;
const SDRAM_ERR_CAUSE_DBE_MASK: u32 = 1 << 1;
const SDRAM_ERR_CAUSE_SBE_MASK: u32 = 1;
const SDRAM_RANK_CTRL_REG: usize = 0x1e0;

const fn sdram_addr_ctrl_size_high_offset(cs: u32) -> u32 { 20 + cs }
const fn sdram_addr_ctrl_size_high_mask(cs: u32) -> u32 { 1 << sdram_addr_ctrl_size_high_offset(cs) }
const fn sdram_addr_ctrl_addr_sel_mask(cs: u32) -> u32 { 1 << (16 + cs) }
const fn sdram_addr_ctrl_size_low_offset(cs: u32) -> u32 { cs * 4 + 2 }
const fn sdram_addr_ctrl_size_low_mask(cs: u32) -> u32 { 0x3 << sdram_addr_ctrl_size_low_offset(cs) }
const fn sdram_addr_ctrl_struct_offset(cs: u32) -> u32 { cs * 4 }
const fn sdram_addr_ctrl_struct_mask(cs: u32) -> u32 { 0x3 << sdram_addr_ctrl_struct_offset(cs) }
const fn sdram_rank_ctrl_exist_mask(cs: u32) -> u32 { 1 << cs }

#[repr(C)]
struct axp_mc_drvdata {
    base: *mut core::ffi::c_void,
    width: u32,
    cs_addr_sel: [bool; SDRAM_NUM_CS],
    msg: [u8; 128],
}

unsafe fn axp_mc_calc_address(drvdata: *mut axp_mc_drvdata, cs: u8, bank: u16, row: u16, col: u16) -> u32 {
    let d = &*drvdata;
    if d.width == 8 {
        if d.cs_addr_sel[cs as usize] {
            ((row as u32 & 0xfff8) << 16) | ((bank as u32 & 7) << 16) |
                ((row as u32 & 7) << 13) | ((col as u32 & 0x3ff) << 3)
        } else {
            ((row as u32 & (0xffff << 16)) | ((bank as u32 & 7) << 13) |
                ((col as u32 & 0x3ff) << 3))
        }
    } else if d.width == 4 {
        if d.cs_addr_sel[cs as usize] {
            ((row as u32 & 0xfff0) << 15) | ((bank as u32 & 7) << 16) |
                ((row as u32 & 0xf) << 12) | ((col as u32 & 0x3ff) << 2)
        } else {
            ((row as u32 & (0xffff << 15)) | ((bank as u32 & 7) << 12) |
                ((col as u32 & 0x3ff) << 2))
        }
    } else if d.cs_addr_sel[cs as usize] {
        ((row as u32 & 0xffe0) << 14) | ((bank as u32 & 7) << 16) |
            ((row as u32 & 0x1f) << 11) | ((col as u32 & 0x3ff) << 1)
    } else {
        ((row as u32 & (0xffff << 14)) | ((bank as u32 & 7) << 11) |
            ((col as u32 & 0x3ff) << 1))
    }
}

unsafe fn axp_mc_check(mci: *mut mem_ctl_info) {
    let drvdata = (*mci).pvt_info as *mut axp_mc_drvdata;
    let base = (*drvdata).base as *mut u8;
    let read = |off: usize| core::ptr::read_volatile(base.add(off) as *const u32);
    let write = |off: usize, val: u32| core::ptr::write_volatile(base.add(off) as *mut u32, val);
    let _data_h = read(SDRAM_ERR_DATA_H_REG);
    let _data_l = read(SDRAM_ERR_DATA_L_REG);
    let recv_ecc = read(SDRAM_ERR_RECV_ECC_REG);
    let calc_ecc = read(SDRAM_ERR_CALC_ECC_REG);
    let addr = read(SDRAM_ERR_ADDR_REG);
    let mut cnt_sbe = read(SDRAM_ERR_SBE_COUNT_REG);
    let mut cnt_dbe = read(SDRAM_ERR_DBE_COUNT_REG);
    let _cause_err = read(SDRAM_ERR_CAUSE_ERR_REG);
    let _cause_msg = read(SDRAM_ERR_CAUSE_MSG_REG);
    write(SDRAM_ERR_CAUSE_ERR_REG, !(SDRAM_ERR_CAUSE_DBE_MASK | SDRAM_ERR_CAUSE_SBE_MASK));
    write(SDRAM_ERR_CAUSE_MSG_REG, !(SDRAM_ERR_CAUSE_DBE_MASK | SDRAM_ERR_CAUSE_SBE_MASK));
    if cnt_sbe != 0 { write(SDRAM_ERR_SBE_COUNT_REG, 0); }
    if cnt_dbe != 0 { write(SDRAM_ERR_DBE_COUNT_REG, 0); }
    if cnt_sbe == 0 && cnt_dbe == 0 { return; }
    if addr & SDRAM_ERR_ADDR_TYPE_MASK == 0 {
        if cnt_sbe != 0 { cnt_sbe -= 1; } else { dev_warn((*mci).pdev, "inconsistent SBE count detected\n"); }
    } else if cnt_dbe != 0 { cnt_dbe -= 1; } else { dev_warn((*mci).pdev, "inconsistent DBE count detected\n"); }
    if cnt_sbe != 0 { edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, cnt_sbe, 0, 0, 0, -1, -1, -1, (*mci).ctl_name, "details unavailable (multiple errors)"); }
    if cnt_dbe != 0 { edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, cnt_dbe, 0, 0, 0, -1, -1, -1, (*mci).ctl_name, "details unavailable (multiple errors)"); }
    let cs_val = ((addr & SDRAM_ERR_ADDR_CS_MASK) >> SDRAM_ERR_ADDR_CS_OFFSET) as u8;
    let bank_val = ((addr & SDRAM_ERR_ADDR_BANK_MASK) >> SDRAM_ERR_ADDR_BANK_OFFSET) as u16;
    let row_val = ((calc_ecc & SDRAM_ERR_CALC_ECC_ROW_MASK) >> SDRAM_ERR_CALC_ECC_ROW_OFFSET) as u16;
    let col_val = ((addr & SDRAM_ERR_ADDR_COL_MASK) >> SDRAM_ERR_ADDR_COL_OFFSET) as u16;
    let syndrome_val = (recv_ecc ^ calc_ecc) as u8;
    let addr_val = axp_mc_calc_address(drvdata, cs_val, bank_val, row_val, col_val);
    let msg = &mut (*drvdata).msg;
    let text = format!("row=0x{:04x} bank=0x{:x} col=0x{:04x} cs={}", row_val, bank_val, col_val, cs_val);
    let n = core::cmp::min(text.len(), msg.len().saturating_sub(1));
    msg[..n].copy_from_slice(&text.as_bytes()[..n]); msg[n] = 0;
    if addr & SDRAM_ERR_ADDR_TYPE_MASK == 0 {
        edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1, addr_val >> PAGE_SHIFT, addr_val & !PAGE_MASK, syndrome_val as u32, cs_val as i32, -1, -1, (*mci).ctl_name, msg.as_ptr());
    } else {
        edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1, addr_val >> PAGE_SHIFT, addr_val & !PAGE_MASK, syndrome_val as u32, cs_val as i32, -1, -1, (*mci).ctl_name, msg.as_ptr());
    }
}

unsafe fn axp_mc_read_config(mci: *mut mem_ctl_info) {
    let d = (*mci).pvt_info as *mut axp_mc_drvdata;
    let base = (*d).base as *mut u8;
    let config = core::ptr::read_volatile(base.add(SDRAM_CONFIG_REG) as *const u32);
    (*d).width = if config & SDRAM_CONFIG_BUS_WIDTH_MASK != 0 { 8 } else { 4 };
    let addr_ctrl = core::ptr::read_volatile(base.add(SDRAM_ADDR_CTRL_REG) as *const u32);
    let rank_ctrl = core::ptr::read_volatile(base.add(SDRAM_RANK_CTRL_REG) as *const u32);
    for i in 0..SDRAM_NUM_CS {
        if rank_ctrl & sdram_rank_ctrl_exist_mask(i as u32) == 0 { continue; }
        (*d).cs_addr_sel[i] = addr_ctrl & sdram_addr_ctrl_addr_sel_mask(i as u32) != 0;
        let cs_struct = (addr_ctrl & sdram_addr_ctrl_struct_mask(i as u32)) >> sdram_addr_ctrl_struct_offset(i as u32);
        let cs_size = ((addr_ctrl & sdram_addr_ctrl_size_high_mask(i as u32)) >> (sdram_addr_ctrl_size_high_offset(i as u32) - 2)) |
            ((addr_ctrl & sdram_addr_ctrl_size_low_mask(i as u32)) >> sdram_addr_ctrl_size_low_offset(i as u32));
        let dimm = (*mci).dimms[i];
        (*dimm).nr_pages = match cs_size { 0 => 524288, 1 => 65536, 2 => 131072, 3 => 262144, 4 => 1048576, 5 => 2097152, _ => (*dimm).nr_pages };
        (*dimm).grain = 8;
        (*dimm).dtype = if cs_struct != 0 { DEV_X16 } else { DEV_X8 };
        (*dimm).mtype = if config & SDRAM_CONFIG_REGISTERED_MASK != 0 { MEM_RDDR3 } else { MEM_DDR3 };
        (*dimm).edac_mode = EDAC_SECDED;
    }
}

#[repr(C)] struct aurora_l2_drvdata { base: *mut core::ffi::c_void, msg: [u8; 128], inject_addr: u32, inject_mask: u32, inject_ctl: u8, debugfs: *mut dentry }

#[cfg(CONFIG_EDAC_DEBUG)]
unsafe fn aurora_l2_inject(d: *mut aurora_l2_drvdata) {
    (*d).inject_addr &= AURORA_ERR_INJECT_CTL_ADDR_MASK;
    (*d).inject_ctl &= AURORA_ERR_INJECT_CTL_EN_MASK as u8;
    let b = (*d).base as *mut u8;
    core::ptr::write_volatile(b.add(AURORA_ERR_INJECT_CTL_REG) as *mut u32, 0);
    core::ptr::write_volatile(b.add(AURORA_ERR_INJECT_MASK_REG) as *mut u32, (*d).inject_mask);
    core::ptr::write_volatile(b.add(AURORA_ERR_INJECT_CTL_REG) as *mut u32, (*d).inject_addr | (*d).inject_ctl as u32);
}

unsafe fn aurora_l2_check(dci: *mut edac_device_ctl_info) {
    let d = (*dci).pvt_info as *mut aurora_l2_drvdata;
    let b = (*d).base as *mut u8;
    let rd = |o: usize| core::ptr::read_volatile(b.add(o) as *const u32);
    let cnt = rd(AURORA_ERR_CNT_REG); let attr = rd(AURORA_ERR_ATTR_CAP_REG); let addr = rd(AURORA_ERR_ADDR_CAP_REG); let way = rd(AURORA_ERR_WAY_CAP_REG);
    let mut ce = (cnt & AURORA_ERR_CNT_CE_MASK) >> AURORA_ERR_CNT_CE_OFFSET;
    let mut ue = (cnt & AURORA_ERR_CNT_UE_MASK) >> AURORA_ERR_CNT_UE_OFFSET;
    if ce != 0 || ue != 0 { core::ptr::write_volatile(b.add(AURORA_ERR_CNT_REG) as *mut u32, AURORA_ERR_CNT_CLR); }
    if attr & AURORA_ERR_ATTR_CAP_VALID != 0 {
        let src = (attr & AURORA_ERR_ATTR_SRC_MSK) >> AURORA_ERR_ATTR_SRC_OFF;
        let txn = (attr & AURORA_ERR_ATTR_TXN_MSK) >> AURORA_ERR_ATTR_TXN_OFF;
        let err = (attr & AURORA_ERR_ATTR_ERR_MSK) >> AURORA_ERR_ATTR_ERR_OFF;
        let text = format!("src={} txn={} err={} addr=0x{:x} index=0x{:x} way=0x{:x}", if src <= 3 { format!("CPU{}", src) } else { "IO".into() }, txn, err, addr & AURORA_ERR_ADDR_CAP_ADDR_MASK, (way & AURORA_ERR_WAY_IDX_MSK) >> AURORA_ERR_WAY_IDX_OFF, (way & AURORA_ERR_WAY_CAP_WAY_MASK) >> AURORA_ERR_WAY_CAP_WAY_OFFSET);
        let n = core::cmp::min(text.len(), 127); (*d).msg[..n].copy_from_slice(&text.as_bytes()[..n]); (*d).msg[n] = 0;
        core::ptr::write_volatile(b.add(AURORA_ERR_ATTR_CAP_REG) as *mut u32, AURORA_ERR_ATTR_CAP_VALID);
        if err != 0 { if ue != 0 { ue -= 1; } edac_device_handle_ue(dci, 0, 0, (*d).msg.as_ptr()); } else { if ce != 0 { ce -= 1; } edac_device_handle_ce(dci, 0, 0, (*d).msg.as_ptr()); }
    }
    while ue != 0 { ue -= 1; edac_device_handle_ue(dci, 0, 0, "details unavailable (multiple errors)"); }
    while ce != 0 { ce -= 1; edac_device_handle_ue(dci, 0, 0, "details unavailable (multiple errors)"); }
}

unsafe fn aurora_l2_poll(dci: *mut edac_device_ctl_info) { aurora_l2_check(dci); #[cfg(CONFIG_EDAC_DEBUG)] { aurora_l2_inject((*dci).pvt_info as *mut aurora_l2_drvdata); } }

// Platform-driver registration, probe/remove glue, module metadata, and the
// CONFIG_EDAC_DEBUG debugfs setup are provided by the translated kernel ABI.
unsafe fn armada_xp_edac_init() -> i32 { if ghes_get_devices() != 0 { return -EBUSY; } edac_op_state = EDAC_OPSTATE_POLL; platform_register_drivers(); 0 }
unsafe fn armada_xp_edac_exit() { platform_unregister_drivers(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
