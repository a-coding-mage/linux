/* SPDX-License-Identifier: GPL-2.0 */

pub const BCMA_MIPS_IPSFLAG: u32 = 0x0F08;
/* which sbflags get routed to mips interrupt 1 */
pub const BCMA_MIPS_IPSFLAG_IRQ1: u32 = 0x0000003F;
pub const BCMA_MIPS_IPSFLAG_IRQ1_SHIFT: u32 = 0;
/* which sbflags get routed to mips interrupt 2 */
pub const BCMA_MIPS_IPSFLAG_IRQ2: u32 = 0x00003F00;
pub const BCMA_MIPS_IPSFLAG_IRQ2_SHIFT: u32 = 8;
/* which sbflags get routed to mips interrupt 3 */
pub const BCMA_MIPS_IPSFLAG_IRQ3: u32 = 0x003F0000;
pub const BCMA_MIPS_IPSFLAG_IRQ3_SHIFT: u32 = 16;
/* which sbflags get routed to mips interrupt 4 */
pub const BCMA_MIPS_IPSFLAG_IRQ4: u32 = 0x3F000000;
pub const BCMA_MIPS_IPSFLAG_IRQ4_SHIFT: u32 = 24;

/* MIPS 74K core registers */
pub const BCMA_MIPS_MIPS74K_CORECTL: u32 = 0x0000;
pub const BCMA_MIPS_MIPS74K_EXCEPTBASE: u32 = 0x0004;
pub const BCMA_MIPS_MIPS74K_BIST: u32 = 0x000C;
pub const BCMA_MIPS_MIPS74K_INTMASK_INT0: u32 = 0x0014;

#[inline]
pub const fn BCMA_MIPS_MIPS74K_INTMASK(int_: u32) -> u32 {
    int_.wrapping_mul(4).wrapping_add(BCMA_MIPS_MIPS74K_INTMASK_INT0)
}

pub const BCMA_MIPS_MIPS74K_NMIMASK: u32 = 0x002C;
pub const BCMA_MIPS_MIPS74K_GPIOSEL: u32 = 0x0040;
pub const BCMA_MIPS_MIPS74K_GPIOOUT: u32 = 0x0044;
pub const BCMA_MIPS_MIPS74K_GPIOEN: u32 = 0x0048;
pub const BCMA_MIPS_MIPS74K_CLKCTLST: u32 = 0x01E0;

pub const BCMA_MIPS_OOBSELINA74: u32 = 0x004;
pub const BCMA_MIPS_OOBSELOUTA30: u32 = 0x100;

#[repr(C)]
pub struct bcma_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bcma_drv_mips {
    pub core: *mut bcma_device,
    /* C bit-fields: setup_done is bit 0 and early_setup_done is bit 1. */
    pub setup_flags: u8,
}

extern "C" {
    pub fn bcma_cpu_clock(mcore: *mut bcma_drv_mips) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
