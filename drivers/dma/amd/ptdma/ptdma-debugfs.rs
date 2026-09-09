// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Passthrough DMA device driver
 * -- Based on the CCP driver
 *
 * Copyright (C) 2016,2021 Advanced Micro Devices, Inc.
 *
 * Author: Sanjay R Mehta <sanju.mehta@amd.com>
 * Author: Gary R Hook <gary.hook@amd.com>
 */

// Linux debugfs, seq_file, PTDMA, and AE4DMA dependencies are supplied externally.

const RI_VERSION_NUM: u32 = 0x0000003f;
const RI_NUM_VQM: u32 = 0x00078000;
const RI_NVQM_SHIFT: u32 = 15;

extern "C" {
    fn debugfs_initialized() -> bool;
    fn debugfs_create_file(name: *const i8, mode: u32, parent: *mut Dentry,
                           data: *mut core::ffi::c_void, fops: *const FileOperations) -> *mut Dentry;
    fn debugfs_create_dir(name: *const i8, parent: *mut Dentry) -> *mut Dentry;
    fn ioread32(addr: *const u8) -> u32;
    fn readl(addr: *const u8) -> u32;
    fn dev_name(dev: *mut Device) -> *const i8;
    fn seq_printf(s: *mut SeqFile, fmt: *const i8, ...);
    fn seq_puts(s: *mut SeqFile, text: *const i8);
}

#[repr(C)] pub struct Dentry { _private: [u8; 0] }
#[repr(C)] pub struct Device { _private: [u8; 0] }
#[repr(C)] pub struct SeqFile { pub private: *mut core::ffi::c_void }
#[repr(C)] pub struct FileOperations { _private: [u8; 0] }

#[repr(C)] pub struct DmaDevice { pub dbg_dev_root: *mut Dentry }
#[repr(C)] pub struct PtDevice {
    pub dev: *mut Device,
    pub ver: u32,
    pub io_regs: *mut u8,
    pub cmd_count: i32,
    pub total_interrupts: i64,
    pub dma_dev: DmaDevice,
    pub cmd_q: PtCmdQueue,
}
#[repr(C)] pub struct PtCmdQueue {
    pub total_pt_ops: i64,
    pub pt: *mut PtDevice,
    pub reg_control: *mut u8,
}
#[repr(C)] pub struct Ae4CmdQueue { pub id: i32, pub cmd_q: PtCmdQueue }
#[repr(C)] pub struct Ae4Device {
    pub pt: PtDevice,
    pub cmd_q_count: i32,
    pub ae4cmd_q: *mut Ae4CmdQueue,
}

extern "C" {
    static pt_debugfs_info_fops: FileOperations;
    static pt_debugfs_queue_fops: FileOperations;
    static pt_debugfs_stats_fops: FileOperations;
}

// Values and structures supplied by the corresponding driver headers.
extern "C" {
    static AE4_DMA_VERSION: u32;
    static CMD_PT_VERSION: usize;
    static CMD_Q_LEN: i32;
    static INT_EMPTY_QUEUE: u32;
    static INT_QUEUE_STOPPED: u32;
    static INT_ERROR: u32;
    static INT_COMPLETION: u32;
}

unsafe fn pt_debugfs_info_show(s: *mut SeqFile, _p: *mut core::ffi::c_void) -> i32 {
    let pt = (*s).private as *mut PtDevice;
    let mut ae4: *mut Ae4Device;
    let regval: u32;

    seq_printf(s, b"Device name: %s\n\0".as_ptr() as *const i8, dev_name((*pt).dev));

    if (*pt).ver == AE4_DMA_VERSION {
        ae4 = pt as *mut Ae4Device;
        seq_printf(s, b"   # Queues: %d\n\0".as_ptr() as *const i8, (*ae4).cmd_q_count);
        seq_printf(s, b"     # Cmds per queue: %d\n\0".as_ptr() as *const i8, CMD_Q_LEN);
    } else {
        seq_printf(s, b"   # Queues: %d\n\0".as_ptr() as *const i8, 1);
        seq_printf(s, b"     # Cmds: %d\n\0".as_ptr() as *const i8, (*pt).cmd_count);
    }

    regval = ioread32((*pt).io_regs.add(CMD_PT_VERSION));
    seq_printf(s, b"    Version: %d\n\0".as_ptr() as *const i8, regval & RI_VERSION_NUM);
    seq_puts(s, b"    Engines:\0".as_ptr() as *const i8);
    seq_puts(s, b"\n\0".as_ptr() as *const i8);
    seq_printf(s, b"     Queues: %d\n\0".as_ptr() as *const i8,
               (regval & RI_NUM_VQM) >> RI_NVQM_SHIFT);
    0
}

/* Return a formatted buffer containing the current statistics of queue for PTDMA */
unsafe fn pt_debugfs_stats_show(s: *mut SeqFile, _p: *mut core::ffi::c_void) -> i32 {
    let pt = (*s).private as *mut PtDevice;
    seq_printf(s, b"Total Interrupts Handled: %ld\n\0".as_ptr() as *const i8,
               (*pt).total_interrupts);
    0
}

unsafe fn pt_debugfs_queue_show(s: *mut SeqFile, _p: *mut core::ffi::c_void) -> i32 {
    let cmd_q = (*s).private as *mut PtCmdQueue;
    if cmd_q.is_null() { return 0; }
    seq_printf(s, b"               Pass-Thru: %ld\n\0".as_ptr() as *const i8,
               (*cmd_q).total_pt_ops);
    let pt = (*cmd_q).pt;
    let regval: u32;
    if (*pt).ver == AE4_DMA_VERSION {
        regval = readl((*cmd_q).reg_control.add(0x4));
        seq_printf(s, b"     Enabled Interrupts:: status 0x%x\n\0".as_ptr() as *const i8, regval);
    } else {
        regval = ioread32((*cmd_q).reg_control.add(0x000c));
        seq_puts(s, b"      Enabled Interrupts:\0".as_ptr() as *const i8);
        if regval & INT_EMPTY_QUEUE != 0 { seq_puts(s, b" EMPTY\0".as_ptr() as *const i8); }
        if regval & INT_QUEUE_STOPPED != 0 { seq_puts(s, b" STOPPED\0".as_ptr() as *const i8); }
        if regval & INT_ERROR != 0 { seq_puts(s, b" ERROR\0".as_ptr() as *const i8); }
        if regval & INT_COMPLETION != 0 { seq_puts(s, b" COMPLETION\0".as_ptr() as *const i8); }
        seq_puts(s, b"\n\0".as_ptr() as *const i8);
    }
    0
}

pub unsafe fn ptdma_debugfs_setup(pt: *mut PtDevice) {
    if !debugfs_initialized() { return; }
    debugfs_create_file(b"info\0".as_ptr() as *const i8, 0o400, (*pt).dma_dev.dbg_dev_root,
                        pt as *mut _, &pt_debugfs_info_fops);
    debugfs_create_file(b"stats\0".as_ptr() as *const i8, 0o400, (*pt).dma_dev.dbg_dev_root,
                        pt as *mut _, &pt_debugfs_stats_fops);

    if (*pt).ver == AE4_DMA_VERSION {
        let ae4 = pt as *mut Ae4Device;
        for i in 0..(*ae4).cmd_q_count {
            let ae4cmd_q = (*ae4).ae4cmd_q.add(i as usize);
            let cmd_q = &mut (*ae4cmd_q).cmd_q as *mut PtCmdQueue;
            let mut name = [0i8; 30];
            // snprintf(name, 29, "q%d", ae4cmd_q->id)
            *name.as_mut_ptr() = b'q' as i8;
            *name.as_mut_ptr().add(1) = b'0' as i8 + (*ae4cmd_q).id as i8;
            let dir = debugfs_create_dir(name.as_ptr(), (*pt).dma_dev.dbg_dev_root);
            debugfs_create_file(b"stats\0".as_ptr() as *const i8, 0o400, dir,
                                cmd_q as *mut _, &pt_debugfs_queue_fops);
        }
    } else {
        let dir = debugfs_create_dir(b"q\0".as_ptr() as *const i8, (*pt).dma_dev.dbg_dev_root);
        debugfs_create_file(b"stats\0".as_ptr() as *const i8, 0o400, dir,
                            &mut (*pt).cmd_q, &pt_debugfs_queue_fops);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
