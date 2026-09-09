// SPDX-License-Identifier: GPL-2.0
/*
 * ip22-mc.c: Routines for manipulating SGI Memory Controller.
 *
 * Copyright (C) 1996 David S. Miller (davem@davemloft.net)
 * Copyright (C) 1999 Andrew R. Baker (andrewb@uab.edu) - Indigo2 changes
 * Copyright (C) 2003 Ladislav Michl  (ladis@linux-mips.org)
 * Copyright (C) 2004 Peter Fuerst    (pf@net.alphadv.de) - IP28
 */

// External kernel, architecture, and SGI declarations are supplied by the
// corresponding Rust translation units.

#[repr(C)]
pub struct sgimc_regs {
    pub systemid: u32,
    pub cpuctrl0: u32,
    pub cstat: u32,
    pub gstat: u32,
    pub cpuctrl1: u32,
    pub divider: u32,
    pub giopar: u32,
    pub mconfig0: u32,
    pub mconfig1: u32,
    pub cmacc: u32,
}

pub static mut sgimc: *mut sgimc_regs = core::ptr::null_mut();

#[inline]
unsafe fn get_bank_addr(memconfig: u32) -> usize {
    ((memconfig & SGIMC_MCONFIG_BASEADDR) as usize)
        << (if ((*sgimc).systemid & SGIMC_SYSID_MASKREV) >= 5 { 24 } else { 22 })
}

#[inline]
unsafe fn get_bank_size(memconfig: u32) -> usize {
    (((memconfig & SGIMC_MCONFIG_RMASK) + 0x0100) as usize)
        << (if ((*sgimc).systemid & SGIMC_SYSID_MASKREV) >= 5 { 16 } else { 14 })
}

#[inline]
unsafe fn get_bank_config(bank: i32) -> u32 {
    let res = if bank > 1 { (*sgimc).mconfig1 } else { (*sgimc).mconfig0 };
    if bank % 2 != 0 { res & 0xffff } else { res >> 16 }
}

#[cfg(any(feature = "CONFIG_SGI_IP28", feature = "CONFIG_32BIT"))]
unsafe fn probe_memory() {
    /* prom detects all usable memory */
}

#[cfg(not(any(feature = "CONFIG_SGI_IP28", feature = "CONFIG_32BIT")))]
unsafe fn probe_memory() {
    /* Detect installed memory, which PROM misses */
    printk(KERN_INFO, "MC: Probing memory configuration:\n");
    for i in 0..4 {
        let tmp = get_bank_config(i);
        if tmp & SGIMC_MCONFIG_BVALID == 0 {
            continue;
        }
        let size = get_bank_size(tmp);
        let addr = get_bank_addr(tmp);
        printk(KERN_INFO, " bank%d: %3ldM @ %08lx\n", i, size / 1024 / 1024, addr);
        if addr >= SGIMC_SEG1_BADDR {
            memblock_add(addr, size);
        }
    }
}

pub unsafe fn sgimc_init() {
    let mut tmp: u32;

    /* ioremap can't fail */
    sgimc = ioremap(SGIMC_BASE, core::mem::size_of::<sgimc_regs>()) as *mut sgimc_regs;

    printk(
        KERN_INFO,
        "MC: SGI memory controller Revision %d\n",
        ((*sgimc).systemid & SGIMC_SYSID_MASKREV) as i32,
    );

    /* Place the MC into a known state. */
    tmp = (*sgimc).cpuctrl0;
    tmp &= !SGIMC_CCTRL0_WDOG;
    (*sgimc).cpuctrl0 = tmp;

    (*sgimc).cstat = 0;
    (*sgimc).gstat = 0;

    /* don't touch parity settings for IP28 */
    tmp = (*sgimc).cpuctrl0;
    #[cfg(not(feature = "CONFIG_SGI_IP28"))]
    {
        tmp |= SGIMC_CCTRL0_EPERRGIO | SGIMC_CCTRL0_EPERRMEM;
    }
    tmp |= SGIMC_CCTRL0_R4KNOCHKPARR;
    (*sgimc).cpuctrl0 = tmp;

    tmp = (*sgimc).cpuctrl1;
    tmp &= !0xf;
    tmp |= 0xd;
    (*sgimc).cpuctrl1 = tmp;

    (*sgimc).divider = 0x101;

    tmp = (*sgimc).giopar & SGIMC_GIOPAR_GFX64;
    tmp |= SGIMC_GIOPAR_HPC64;
    tmp |= SGIMC_GIOPAR_ONEBUS;

    if ip22_is_fullhouse() {
        if SGIOC_SYSID_BOARDREV((*sgioc).sysid) < 2 {
            tmp |= SGIMC_GIOPAR_HPC264;
            tmp |= SGIMC_GIOPAR_PLINEEXP0;
            tmp |= SGIMC_GIOPAR_MASTEREXP1;
            tmp |= SGIMC_GIOPAR_RTIMEEXP0;
        } else {
            tmp |= SGIMC_GIOPAR_HPC264;
            tmp |= SGIMC_GIOPAR_PLINEEXP0;
            tmp |= SGIMC_GIOPAR_PLINEEXP1;
            tmp |= SGIMC_GIOPAR_MASTEREISA;
        }
    } else {
        tmp |= SGIMC_GIOPAR_EISA64;
        tmp |= SGIMC_GIOPAR_MASTEREISA;
    }
    (*sgimc).giopar = tmp;

    probe_memory();
}

#[cfg(feature = "CONFIG_SGI_IP28")]
pub unsafe fn prom_cleanup() {
    let mut mconfig1: u32;
    let mut flags: usize = 0;
    let mut lock: spinlock_t = core::mem::zeroed();

    spin_lock_irqsave(&mut lock, &mut flags);
    mconfig1 = (*sgimc).mconfig1;
    (*sgimc).mconfig1 = (mconfig1 & 0xffff0000) | 0x2060;
    iob();
    *(PHYS_TO_XKSEG_UNCACHED(0x60000000) as *mut usize) = 0;
    iob();
    (*sgimc).cmacc = ((*sgimc).cmacc & !0xf) | 4;
    iob();
    (*sgimc).mconfig1 = mconfig1;
    iob();
    spin_unlock_irqrestore(&mut lock, flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
