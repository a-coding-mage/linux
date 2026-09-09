// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018-2019 Synopsys, Inc. and/or its affiliates.
 * Synopsys DesignWare eDMA v0 core
 *
 * Author: Gustavo Pimentel <gustavo.pimentel@synopsys.com>
 */

// Linux debugfs, bitfield, and eDMA declarations are supplied by the surrounding crate.

const WRITE_STR: &str = "write";
const READ_STR: &str = "read";
const CHANNEL_STR: &str = "channel";
const REGISTERS_STR: &str = "registers";

#[repr(C)]
struct DwEdmaDebugfsEntry {
    dw: *mut dw_edma,
    name: *const core::ffi::c_char,
    reg: *mut core::ffi::c_void,
    dir: dw_edma_dir,
    ch: u16,
}

unsafe fn regs_addr(dw: *mut dw_edma, name: &str) -> *mut core::ffi::c_void {
    dw_edma_v0_regs_addr(dw, name.as_ptr() as *const core::ffi::c_char)
}

unsafe fn regs_ch_addr(dw: *mut dw_edma, name: &str, dir: dw_edma_dir, ch: u16) -> *mut core::ffi::c_void {
    dw_edma_v0_ch_regs_addr(dw, name.as_ptr() as *const core::ffi::c_char, dir, ch)
}

unsafe fn debugfs_u32_get(data: *mut core::ffi::c_void, val: *mut u64) -> i32 {
    let entry = &*(data as *const DwEdmaDebugfsEntry);
    let dw = &*entry.dw;
    let reg = entry.reg;

    if dw.chip.mf == EDMA_MF_EDMA_LEGACY && reg as usize >= regs_addr(entry.dw, "type.legacy.ch") as usize {
        let mut flags: libc::c_ulong = 0;
        let mut viewport_sel: u32 = if entry.dir == EDMA_DIR_READ { 1u32 << 31 } else { 0 };
        viewport_sel |= ((entry.ch as u32) << EDMA_V0_VIEWPORT_SHIFT) & EDMA_V0_VIEWPORT_MASK;

        raw_spin_lock_irqsave(&dw.lock, &mut flags);
        writel(viewport_sel, regs_addr(entry.dw, "type.legacy.viewport_sel"));
        *val = readl(reg) as u64;
        raw_spin_unlock_irqrestore(&dw.lock, flags);
    } else {
        *val = readl(reg) as u64;
    }
    0
}

unsafe fn dw_edma_debugfs_create_x32(
    dw: *mut dw_edma,
    ini: *const DwEdmaDebugfsEntry,
    nr_entries: i32,
    dent: *mut dentry,
) {
    let entries = devm_kcalloc((*dw).chip.dev, nr_entries as usize, core::mem::size_of::<DwEdmaDebugfsEntry>(), GFP_KERNEL);
    if entries.is_null() { return; }
    for i in 0..nr_entries {
        *entries.add(i as usize) = *ini.add(i as usize);
        debugfs_create_file_unsafe((*entries.add(i as usize)).name, 0o444, dent, entries.add(i as usize) as *mut _, &fops_x32);
    }
}

unsafe fn entry(dw: *mut dw_edma, name: &'static [u8], reg: *mut core::ffi::c_void, dir: dw_edma_dir, ch: u16) -> DwEdmaDebugfsEntry {
    DwEdmaDebugfsEntry { dw, name: name.as_ptr() as *const _, reg, dir, ch }
}

unsafe fn dw_edma_debugfs_regs_ch(dw: *mut dw_edma, dir: dw_edma_dir, ch: u16, dent: *mut dentry) {
    let regs = [
        entry(dw, b"ch_control1\0", regs_ch_addr(dw, "ch_control1", dir, ch), dir, ch),
        entry(dw, b"ch_control2\0", regs_ch_addr(dw, "ch_control2", dir, ch), dir, ch),
        entry(dw, b"transfer_size\0", regs_ch_addr(dw, "transfer_size", dir, ch), dir, ch),
        entry(dw, b"sar.lsb\0", regs_ch_addr(dw, "sar.lsb", dir, ch), dir, ch),
        entry(dw, b"sar.msb\0", regs_ch_addr(dw, "sar.msb", dir, ch), dir, ch),
        entry(dw, b"dar.lsb\0", regs_ch_addr(dw, "dar.lsb", dir, ch), dir, ch),
        entry(dw, b"dar.msb\0", regs_ch_addr(dw, "dar.msb", dir, ch), dir, ch),
        entry(dw, b"llp.lsb\0", regs_ch_addr(dw, "llp.lsb", dir, ch), dir, ch),
        entry(dw, b"llp.msb\0", regs_ch_addr(dw, "llp.msb", dir, ch), dir, ch),
    ];
    dw_edma_debugfs_create_x32(dw, regs.as_ptr(), regs.len() as i32, dent);
}

unsafe fn dw_edma_debugfs_regs_wr(dw: *mut dw_edma, dent: *mut dentry) {
    let names = ["engine_en", "doorbell", "ch_arb_weight.lsb", "ch_arb_weight.msb", "int_status", "int_mask", "int_clear", "err_status", "done_imwr.lsb", "done_imwr.msb", "abort_imwr.lsb", "abort_imwr.msb", "ch01_imwr_data", "ch23_imwr_data", "ch45_imwr_data", "ch67_imwr_data", "linked_list_err_en"];
    let regs: alloc::vec::Vec<_> = names.iter().map(|n| entry(dw, n.as_bytes(), regs_addr(dw, &format!("wr_{n}")), EDMA_DIR_WRITE, 0)).collect();
    let regs_dent = debugfs_create_dir(WRITE_STR.as_ptr() as *const _, dent);
    dw_edma_debugfs_create_x32(dw, regs.as_ptr(), regs.len() as i32, regs_dent);
    for i in 0..(*dw).wr_ch_cnt { let mut name = alloc::format!("{CHANNEL_STR}:{i}"); let ch_dent = debugfs_create_dir(name.as_mut_ptr() as *const _, regs_dent); dw_edma_debugfs_regs_ch(dw, EDMA_DIR_WRITE, i, ch_dent); }
}

unsafe fn dw_edma_debugfs_regs_rd(dw: *mut dw_edma, dent: *mut dentry) {
    let names = ["engine_en", "doorbell", "ch_arb_weight.lsb", "ch_arb_weight.msb", "int_status", "int_mask", "int_clear", "err_status.lsb", "err_status.msb", "linked_list_err_en", "done_imwr.lsb", "done_imwr.msb", "abort_imwr.lsb", "abort_imwr.msb", "ch01_imwr_data", "ch23_imwr_data", "ch45_imwr_data", "ch67_imwr_data"];
    let regs: alloc::vec::Vec<_> = names.iter().map(|n| entry(dw, n.as_bytes(), regs_addr(dw, &format!("rd_{n}")), EDMA_DIR_READ, 0)).collect();
    let regs_dent = debugfs_create_dir(READ_STR.as_ptr() as *const _, dent);
    dw_edma_debugfs_create_x32(dw, regs.as_ptr(), regs.len() as i32, regs_dent);
    for i in 0..(*dw).rd_ch_cnt { let mut name = alloc::format!("{CHANNEL_STR}:{i}"); let ch_dent = debugfs_create_dir(name.as_mut_ptr() as *const _, regs_dent); dw_edma_debugfs_regs_ch(dw, EDMA_DIR_READ, i, ch_dent); }
}

unsafe fn dw_edma_debugfs_regs(dw: *mut dw_edma) {
    let regs_dent = debugfs_create_dir(REGISTERS_STR.as_ptr() as *const _, (*dw).dma.dbg_dev_root);
    let regs = [entry(dw, b"ctrl_data_arb_prior\0", regs_addr(dw, "ctrl_data_arb_prior"), EDMA_DIR_WRITE, 0), entry(dw, b"ctrl\0", regs_addr(dw, "ctrl"), EDMA_DIR_WRITE, 0)];
    dw_edma_debugfs_create_x32(dw, regs.as_ptr(), 2, regs_dent);
    dw_edma_debugfs_regs_wr(dw, regs_dent);
    dw_edma_debugfs_regs_rd(dw, regs_dent);
}

pub unsafe fn dw_edma_v0_debugfs_on(dw: *mut dw_edma) {
    if !debugfs_initialized() { return; }
    debugfs_create_u32(b"mf\0".as_ptr() as *const _, 0o444, (*dw).dma.dbg_dev_root, &mut (*dw).chip.mf);
    debugfs_create_u16(b"wr_ch_cnt\0".as_ptr() as *const _, 0o444, (*dw).dma.dbg_dev_root, &mut (*dw).wr_ch_cnt);
    debugfs_create_u16(b"rd_ch_cnt\0".as_ptr() as *const _, 0o444, (*dw).dma.dbg_dev_root, &mut (*dw).rd_ch_cnt);
    dw_edma_debugfs_regs(dw);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
