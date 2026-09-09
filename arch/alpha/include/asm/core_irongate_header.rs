/* SPDX-License-Identifier: GPL-2.0 */

/*
 * IRONGATE is the internal name for the AMD-751 K7 core logic chipset
 * which provides memory controller and PCI access for NAUTILUS-based
 * EV6 (21264) systems.
 *
 * This file is based on:
 *
 * IronGate management library, (c) 1999 Alpha Processor, Inc.
 * Copyright (C) 1999 Alpha Processor, Inc.,
 *     (David Daniel, Stig Telfer, Soohoon Lee)
 */

/*
 * The 21264 supports, and internally recognizes, a 44-bit physical
 * address space that is divided equally between memory address space
 * and I/O address space. Memory address space resides in the lower
 * half of the physical address space (PA[43]=0) and I/O address space
 * resides in the upper half of the physical address space (PA[43]=1).
 */

/*
 * Irongate CSR map. Some of the CSRs are 8 or 16 bits, but all access
 * through the routines given is 32-bit.
 *
 * The first 0x40 bytes are standard as per the PCI spec.
 */

pub type Igcsr32 = core::cell::UnsafeCell<u32>;

#[repr(C)]
pub struct Irongate0 {
    pub dev_vendor: Igcsr32,       /* 0x00 - device ID, vendor ID */
    pub stat_cmd: Igcsr32,         /* 0x04 - status, command */
    pub class: Igcsr32,            /* 0x08 - class code, rev ID */
    pub latency: Igcsr32,          /* 0x0C - header type, PCI latency */
    pub bar0: Igcsr32,             /* 0x10 - BAR0 - AGP */
    pub bar1: Igcsr32,             /* 0x14 - BAR1 - GART */
    pub bar2: Igcsr32,             /* 0x18 - Power Management reg block */
    pub rsrvd0: [Igcsr32; 6],      /* 0x1C-0x33 reserved */
    pub capptr: Igcsr32,            /* 0x34 - Capabilities pointer */
    pub rsrvd1: [Igcsr32; 2],      /* 0x38-0x3F reserved */
    pub bacsr10: Igcsr32,          /* 0x40 - base address chip selects */
    pub bacsr32: Igcsr32,          /* 0x44 - base address chip selects */
    pub bacsr54_eccms761: Igcsr32, /* 0x48 - 751: base addr. chip selects
                                      761: ECC, mode/status */
    pub rsrvd2: [Igcsr32; 1],      /* 0x4C-0x4F reserved */
    pub drammap: Igcsr32,           /* 0x50 - address mapping control */
    pub dramtm: Igcsr32,            /* 0x54 - timing, driver strength */
    pub dramms: Igcsr32,            /* 0x58 - DRAM mode/status */
    pub rsrvd3: [Igcsr32; 1],       /* 0x5C-0x5F reserved */
    pub biu0: Igcsr32,              /* 0x60 - bus interface unit */
    pub biusip: Igcsr32,            /* 0x64 - Serial initialisation pkt */
    pub rsrvd4: [Igcsr32; 2],       /* 0x68-0x6F reserved */
    pub mro: Igcsr32,               /* 0x70 - memory request optimiser */
    pub rsrvd5: [Igcsr32; 3],       /* 0x74-0x7F reserved */
    pub whami: Igcsr32,             /* 0x80 - who am I */
    pub pciarb: Igcsr32,            /* 0x84 - PCI arbitration control */
    pub pcicfg: Igcsr32,            /* 0x88 - PCI config status */
    pub rsrvd6: [Igcsr32; 4],       /* 0x8C-0x9B reserved */
    pub pci_mem: Igcsr32,           /* 0x9C - PCI top of memory, 761 only */
    pub agpcap: Igcsr32,            /* 0xA0 - AGP Capability Identifier */
    pub agpstat: Igcsr32,           /* 0xA4 - AGP status register */
    pub agpcmd: Igcsr32,            /* 0xA8 - AGP control register */
    pub agpva: Igcsr32,             /* 0xAC - AGP Virtual Address Space */
    pub agpmode: Igcsr32,           /* 0xB0 - AGP/GART mode control */
}

#[repr(C)]
pub struct Irongate1 {
    pub dev_vendor: Igcsr32,
    pub stat_cmd: Igcsr32,
    pub class: Igcsr32,
    pub htype: Igcsr32,
    pub rsrvd0: [Igcsr32; 2],
    pub busnos: Igcsr32,
    pub io_baselim_regs: Igcsr32,
    pub mem_baselim: Igcsr32,
    pub pfmem_baselim: Igcsr32,
    pub rsrvd1: [Igcsr32; 2],
    pub io_baselim: Igcsr32,
    pub rsrvd2: [Igcsr32; 2],
    pub interrupt: Igcsr32,
}

unsafe extern "C" {
    pub static mut IronECC: *mut Igcsr32;
}

/* Memory spaces. Irongate is consistent with a subset of the Tsunami map. */
#[cfg(USE_48_BIT_KSEG)]
pub const IRONGATE_BIAS: u64 = 0x80000000000;
#[cfg(not(USE_48_BIT_KSEG))]
pub const IRONGATE_BIAS: u64 = 0x10000000000;

/* IDENT_ADDR is supplied by the surrounding Alpha architecture headers. */
pub const IRONGATE_MEM: u64 = IDENT_ADDR | IRONGATE_BIAS | 0x000000000;
pub const IRONGATE_IACK_SC: u64 = IDENT_ADDR | IRONGATE_BIAS | 0x1F8000000;
pub const IRONGATE_IO: u64 = IDENT_ADDR | IRONGATE_BIAS | 0x1FC000000;
pub const IRONGATE_CONF: u64 = IDENT_ADDR | IRONGATE_BIAS | 0x1FE000000;

#[inline]
pub const fn igcsr(dev: u64, fun: u64, reg: u64) -> u64 {
    IRONGATE_CONF | (dev << 11) | (fun << 8) | reg
}

pub const IRONGATE0: *mut Irongate0 = igcsr(0, 0, 0) as *mut Irongate0;
pub const IRONGATE1: *mut Irongate1 = igcsr(1, 0, 0) as *mut Irongate1;

pub const SCB_Q_SYSERR: u32 = 0x620;
pub const SCB_Q_PROCERR: u32 = 0x630;
pub const SCB_Q_SYSMCHK: u32 = 0x660;
pub const SCB_Q_PROCMCHK: u32 = 0x670;

#[repr(C)]
pub struct el_IRONGATE_sysdata_mcheck {
    pub FrameSize: u32,
    pub FrameFlags: u32,
    pub CpuOffset: u32,
    pub SystemOffset: u32,
    pub MCHK_Code: u32,
    pub MCHK_Frame_Rev: u32,
    pub I_STAT: u64,
    pub DC_STAT: u64,
    pub C_ADDR: u64,
    pub DC1_SYNDROME: u64,
    pub DC0_SYNDROME: u64,
    pub C_STAT: u64,
    pub C_STS: u64,
    pub RESERVED0: u64,
    pub EXC_ADDR: u64,
    pub IER_CM: u64,
    pub ISUM: u64,
    pub MM_STAT: u64,
    pub PAL_BASE: u64,
    pub I_CTL: u64,
    pub PCTX: u64,
}

/* The following declarations are present only when the C header is built
 * with __KERNEL__. The asm/io_trivial.h dependency is intentionally external. */
#[cfg(__KERNEL__)]
#[inline]
pub unsafe fn irongate_ioportmap(addr: usize) -> *mut core::ffi::c_void {
    (addr as u64 + IRONGATE_IO) as *mut core::ffi::c_void
}

#[cfg(__KERNEL__)]
unsafe extern "C" {
    pub fn irongate_ioremap(addr: usize, size: usize) -> *mut core::ffi::c_void;
    pub fn irongate_iounmap(addr: *mut core::ffi::c_void);
}

#[cfg(__KERNEL__)]
#[inline]
pub unsafe fn irongate_is_ioaddr(addr: usize) -> i32 {
    (addr as u64 >= IRONGATE_MEM) as i32
}

#[cfg(__KERNEL__)]
#[inline]
pub unsafe fn irongate_is_mmio(xaddr: *const core::ffi::c_void) -> i32 {
    let addr = xaddr as u64;
    (addr < IRONGATE_IO || addr >= IRONGATE_CONF) as i32
}

/* C macros from asm/io_trivial.h select the irongate I/O implementation. */
// __IO_PREFIX = irongate
// irongate_trivial_rw_bw = 1
// irongate_trivial_rw_lq = 1
// irongate_trivial_io_bw = 1
// irongate_trivial_io_lq = 1
// irongate_trivial_iounmap = 0

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
