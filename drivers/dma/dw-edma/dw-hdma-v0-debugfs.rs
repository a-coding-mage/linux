// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2023 Cai Huoqing
 * Synopsys DesignWare HDMA v0 debugfs
 *
 * Author: Cai Huoqing <cai.huoqing@linux.dev>
 */

// Linux debugfs, bitfield, dw-hdma-v0, dw-hdma-v0-regs, and dw-edma-core
// dependencies are supplied by the surrounding translation unit.

const WRITE_STR: &[u8] = b"write\0";
const READ_STR: &[u8] = b"read\0";
const CHANNEL_STR: &[u8] = b"channel\0";
const REGISTERS_STR: &[u8] = b"registers\0";

#[repr(C)]
struct dw_hdma_debugfs_entry {
    name: *const core::ffi::c_char,
    reg: *mut core::ffi::c_void,
}

unsafe fn dw_hdma_debugfs_u32_get(data: *mut core::ffi::c_void, val: *mut u64) -> i32 {
    let entry = data as *mut dw_hdma_debugfs_entry;
    let reg = (*entry).reg;
    *val = readl(reg) as u64;
    0
}

// DEFINE_DEBUGFS_ATTRIBUTE(fops_x32, dw_hdma_debugfs_u32_get, NULL, "0x%08llx\n");
extern "C" {
    static fops_x32: core::ffi::c_void;
}

unsafe fn dw_hdma_debugfs_create_x32(
    dw: *mut dw_edma,
    ini: *const dw_hdma_debugfs_entry,
    nr_entries: i32,
    dent: *mut dentry,
) {
    let entries = devm_kcalloc(
        (*(*dw).chip).dev,
        nr_entries as usize,
        core::mem::size_of::<dw_hdma_debugfs_entry>(),
        GFP_KERNEL,
    ) as *mut dw_hdma_debugfs_entry;
    if entries.is_null() {
        return;
    }

    for i in 0..nr_entries {
        *entries.add(i as usize) = *ini.add(i as usize);
        debugfs_create_file_unsafe(
            (*entries.add(i as usize)).name,
            0o444,
            dent,
            entries.add(i as usize) as *mut core::ffi::c_void,
            &fops_x32,
        );
    }
}

unsafe fn dw_hdma_debugfs_regs_ch(
    dw: *mut dw_edma,
    dir: dw_edma_dir,
    ch: u16,
    dent: *mut dentry,
) {
    // CTX_REGISTER(dw, name, dir, ch) expands to { #name, REGS_CH_ADDR(dw, name, dir, ch) }.
    let debugfs_regs: [dw_hdma_debugfs_entry; 27] = [
        ctx_register!(dw, ch_en, dir, ch), ctx_register!(dw, doorbell, dir, ch),
        ctx_register!(dw, prefetch, dir, ch), ctx_register!(dw, handshake, dir, ch),
        ctx_register!(dw, llp.lsb, dir, ch), ctx_register!(dw, llp.msb, dir, ch),
        ctx_register!(dw, cycle_sync, dir, ch), ctx_register!(dw, transfer_size, dir, ch),
        ctx_register!(dw, sar.lsb, dir, ch), ctx_register!(dw, sar.msb, dir, ch),
        ctx_register!(dw, dar.lsb, dir, ch), ctx_register!(dw, dar.msb, dir, ch),
        ctx_register!(dw, watermark_en, dir, ch), ctx_register!(dw, control1, dir, ch),
        ctx_register!(dw, func_num, dir, ch), ctx_register!(dw, qos, dir, ch),
        ctx_register!(dw, ch_stat, dir, ch), ctx_register!(dw, int_stat, dir, ch),
        ctx_register!(dw, int_setup, dir, ch), ctx_register!(dw, int_clear, dir, ch),
        ctx_register!(dw, msi_stop.lsb, dir, ch), ctx_register!(dw, msi_stop.msb, dir, ch),
        ctx_register!(dw, msi_watermark.lsb, dir, ch), ctx_register!(dw, msi_watermark.msb, dir, ch),
        ctx_register!(dw, msi_abort.lsb, dir, ch), ctx_register!(dw, msi_abort.msb, dir, ch),
        ctx_register!(dw, msi_msgdata, dir, ch),
    ];
    dw_hdma_debugfs_create_x32(dw, debugfs_regs.as_ptr(), debugfs_regs.len() as i32, dent);
}

unsafe fn dw_hdma_debugfs_regs_wr(dw: *mut dw_edma, dent: *mut dentry) {
    let regs_dent = debugfs_create_dir(WRITE_STR.as_ptr() as *const _, dent);
    let mut name = [0i8; 32];
    for i in 0..(*dw).wr_ch_cnt {
        snprintf(name.as_mut_ptr(), name.len(), b"%s:%d\0".as_ptr() as *const _, CHANNEL_STR.as_ptr(), i as i32);
        let ch_dent = debugfs_create_dir(name.as_ptr(), regs_dent);
        dw_hdma_debugfs_regs_ch(dw, EDMA_DIR_WRITE, i, ch_dent);
    }
}

unsafe fn dw_hdma_debugfs_regs_rd(dw: *mut dw_edma, dent: *mut dentry) {
    let regs_dent = debugfs_create_dir(READ_STR.as_ptr() as *const _, dent);
    let mut name = [0i8; 32];
    for i in 0..(*dw).rd_ch_cnt {
        snprintf(name.as_mut_ptr(), name.len(), b"%s:%d\0".as_ptr() as *const _, CHANNEL_STR.as_ptr(), i as i32);
        let ch_dent = debugfs_create_dir(name.as_ptr(), regs_dent);
        dw_hdma_debugfs_regs_ch(dw, EDMA_DIR_READ, i, ch_dent);
    }
}

unsafe fn dw_hdma_debugfs_regs(dw: *mut dw_edma) {
    let regs_dent = debugfs_create_dir(REGISTERS_STR.as_ptr() as *const _, (*dw).dma.dbg_dev_root);
    dw_hdma_debugfs_regs_wr(dw, regs_dent);
    dw_hdma_debugfs_regs_rd(dw, regs_dent);
}

pub unsafe fn dw_hdma_v0_debugfs_on(dw: *mut dw_edma) {
    if !debugfs_initialized() {
        return;
    }
    debugfs_create_u32(b"mf\0".as_ptr() as *const _, 0o444, (*dw).dma.dbg_dev_root, &mut (*(*dw).chip).mf);
    debugfs_create_u16(b"wr_ch_cnt\0".as_ptr() as *const _, 0o444, (*dw).dma.dbg_dev_root, &mut (*dw).wr_ch_cnt);
    debugfs_create_u16(b"rd_ch_cnt\0".as_ptr() as *const _, 0o444, (*dw).dma.dbg_dev_root, &mut (*dw).rd_ch_cnt);
    dw_hdma_debugfs_regs(dw);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
