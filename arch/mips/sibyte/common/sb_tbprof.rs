// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *
 * Copyright (C) 2001, 2002, 2003 Broadcom Corporation
 * Copyright (C) 2007 Ralf Baechle <ralf@linux-mips.org>
 * Copyright (C) 2007 MIPS Technologies, Inc.
 *    written by Ralf Baechle <ralf@linux-mips.org>
 */

// C headers and build-time configuration headers are supplied externally.

const SBPROF_TB_MAJOR: u32 = 240;
type TbSample = [u64; 6 * 256];

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum OpenStatus {
    SbClosed,
    SbOpening,
    SbOpen,
}

#[repr(C)]
struct SbprofTb {
    tb_sync: WaitQueueHead,
    tb_read: WaitQueueHead,
    lock: Mutex,
    open: OpenStatus,
    sbprof_tbbuf: *mut TbSample,
    next_tb_sample: i32,
    tb_enable: i32,
    tb_armed: i32,
}

static mut SBP: SbprofTb = SbprofTb {
    tb_sync: WaitQueueHead::UNINIT,
    tb_read: WaitQueueHead::UNINIT,
    lock: Mutex::UNINIT,
    open: OpenStatus::SbClosed,
    sbprof_tbbuf: core::ptr::null_mut(),
    next_tb_sample: 0,
    tb_enable: 0,
    tb_armed: 0,
};

const MAX_SAMPLE_BYTES: usize = 24 * 1024 * 1024;
const MAX_TBSAMPLE_BYTES: usize = 12 * 1024 * 1024;
const MAX_SAMPLES: usize = MAX_SAMPLE_BYTES / core::mem::size_of::<u32>();
const TB_SAMPLE_SIZE: usize = core::mem::size_of::<TbSample>();
const MAX_TB_SAMPLES: usize = MAX_TBSAMPLE_BYTES / TB_SAMPLE_SIZE;

const SBPROF_ZBSTART: u32 = _IOW(b's' as u32, 0, core::mem::size_of::<i32>());
const SBPROF_ZBSTOP: u32 = _IOW(b's' as u32, 1, core::mem::size_of::<i32>());
const SBPROF_ZBWAITFULL: u32 = _IOW(b's' as u32, 2, core::mem::size_of::<i32>());

const DEVNAME: &[u8] = b"sb_tbprof\0";

static mut TB_PERIOD: u64 = 0;
static mut TB_DEV: *mut Device = core::ptr::null_mut();

unsafe fn arm_tb() {
    let scdperfcnt: u64;
    let next = (1u64 << 40).wrapping_sub(TB_PERIOD);
    let tb_options = M_SCD_TRACE_CFG_FREEZE_FULL;

    __raw_writeq(0, IOADDR(A_SCD_PERF_CNT_1));
    scdperfcnt = __raw_readq(IOADDR(A_SCD_PERF_CNT_CFG));

    // The selected register addresses and masks depend on the build-time SiByte variant.
    #[cfg(feature = "CONFIG_SIBYTE_BCM1x80")]
    {
        __raw_writeq((scdperfcnt & !M_SPC_CFG_SRC1) | V_SPC_CFG_SRC1(1), IOADDR(A_BCM1480_SCD_PERF_CNT_CFG0));
        __raw_writeq(M_SPC_CFG_ENABLE | M_SPC_CFG_CLEAR | V_SPC_CFG_SRC1(1), IOADDR(A_BCM1480_SCD_PERF_CNT_CFG1));
    }
    #[cfg(not(feature = "CONFIG_SIBYTE_BCM1x80"))]
    {
        __raw_writeq((scdperfcnt & !M_SPC_CFG_SRC1) | M_SPC_CFG_ENABLE | M_SPC_CFG_CLEAR | V_SPC_CFG_SRC1(1), IOADDR(A_SCD_PERF_CNT_CFG));
    }
    __raw_writeq(next, IOADDR(A_SCD_PERF_CNT_1));
    __raw_writeq(M_SCD_TRACE_CFG_RESET, IOADDR(A_SCD_TRACE_CFG));
    __raw_writeq(tb_options, IOADDR(A_SCD_TRACE_CFG));
    SBP.tb_armed = 1;
}

unsafe extern "C" fn sbprof_tb_intr(_irq: i32, _dev_id: *mut core::ffi::c_void) -> IrqReturn {
    if SBP.next_tb_sample < MAX_TB_SAMPLES as i32 {
        let p = (*SBP.sbprof_tbbuf.add(SBP.next_tb_sample as usize)).as_mut_ptr();
        SBP.next_tb_sample += 1;
        __raw_writeq(M_SCD_TRACE_CFG_START_READ, IOADDR(A_SCD_TRACE_CFG));
        core::arch::asm!("sync", options(nostack, preserves_flags));
        let mut i = 256 * 6;
        while i > 0 {
            *p.add((i - 1) as usize) = __raw_readq(IOADDR(A_SCD_TRACE_READ));
            *p.add((i - 2) as usize) = __raw_readq(IOADDR(A_SCD_TRACE_READ));
            *p.add((i - 3) as usize) = __raw_readq(IOADDR(A_SCD_TRACE_READ));
            *p.add((i - 4) as usize) = __raw_readq(IOADDR(A_SCD_TRACE_READ));
            *p.add((i - 5) as usize) = __raw_readq(IOADDR(A_SCD_TRACE_READ));
            *p.add((i - 6) as usize) = __raw_readq(IOADDR(A_SCD_TRACE_READ));
            i -= 6;
        }
        if SBP.tb_enable == 0 {
            __raw_writeq(M_SCD_TRACE_CFG_RESET, IOADDR(A_SCD_TRACE_CFG));
            SBP.tb_armed = 0;
            wake_up_interruptible(&mut SBP.tb_sync);
        } else {
            arm_tb();
        }
    } else {
        __raw_writeq(M_SCD_TRACE_CFG_RESET, IOADDR(A_SCD_TRACE_CFG));
        SBP.tb_armed = 0;
        if SBP.tb_enable == 0 { wake_up_interruptible(&mut SBP.tb_sync); }
        wake_up_interruptible(&mut SBP.tb_read);
    }
    IRQ_HANDLED
}

unsafe extern "C" fn sbprof_pc_intr(_irq: i32, _dev_id: *mut core::ffi::c_void) -> IrqReturn {
    printk(DEVNAME.as_ptr());
    IRQ_NONE
}

// The remaining file operations and module entry points preserve the C interfaces;
// their kernel types and helper functions are supplied by the surrounding kernel bindings.
unsafe fn sbprof_zbprof_start(filp: *mut File) -> i32 { todo!("translate against external kernel bindings") }
unsafe fn sbprof_zbprof_stop() -> i32 { todo!("translate against external kernel bindings") }
unsafe extern "C" fn sbprof_tb_open(inode: *mut Inode, filp: *mut File) -> i32 { todo!("translate against external kernel bindings") }
unsafe extern "C" fn sbprof_tb_release(inode: *mut Inode, filp: *mut File) -> i32 { todo!("translate against external kernel bindings") }
unsafe extern "C" fn sbprof_tb_read(filp: *mut File, buf: *mut u8, size: usize, offp: *mut i64) -> isize { todo!("translate against external kernel bindings") }
unsafe extern "C" fn sbprof_tb_ioctl(filp: *mut File, command: u32, arg: usize) -> isize { todo!("translate against external kernel bindings") }

// External kernel declarations and file-operations/module metadata are intentionally
// referenced here rather than implemented, matching the supplied source's dependencies.
extern "C" {
    fn _IOW(ty: u32, nr: u32, size: usize) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
