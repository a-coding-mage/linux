/*
 *	mcfmmu.h -- definitions for the ColdFire v4e MMU
 *
 *	(C) Copyright 2011,  Greg Ungerer <gerg@uclinux.org>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

/* MMU support registers are mapped using the processor MMUBASE register. */
pub const MMUBASE: u32 = 0xfe00_0000;

/* MMU support registers. */
pub const MMUCR: u32 = MMUBASE + 0x00; /* Control register */
pub const MMUOR: u32 = MMUBASE + 0x04; /* Operation register */
pub const MMUSR: u32 = MMUBASE + 0x08; /* Status register */
pub const MMUAR: u32 = MMUBASE + 0x10; /* TLB Address register */
pub const MMUTR: u32 = MMUBASE + 0x14; /* TLB Tag register */
pub const MMUDR: u32 = MMUBASE + 0x18; /* TLB Data register */

/* MMU Control register bit flags */
pub const MMUCR_EN: u32 = 0x0000_0001; /* Virtual mode enable */
pub const MMUCR_ASM: u32 = 0x0000_0002; /* Address space mode */

/* MMU Operation register. */
pub const MMUOR_UAA: u32 = 0x0000_0001; /* Update allocation address */
pub const MMUOR_ACC: u32 = 0x0000_0002; /* TLB access */
pub const MMUOR_RD: u32 = 0x0000_0004; /* TLB access read */
pub const MMUOR_WR: u32 = 0x0000_0000; /* TLB access write */
pub const MMUOR_ADR: u32 = 0x0000_0008; /* TLB address select */
pub const MMUOR_ITLB: u32 = 0x0000_0010; /* ITLB operation */
pub const MMUOR_CAS: u32 = 0x0000_0020; /* Clear non-locked ASID TLBs */
pub const MMUOR_CNL: u32 = 0x0000_0040; /* Clear non-locked TLBs */
pub const MMUOR_CA: u32 = 0x0000_0080; /* Clear all TLBs */
pub const MMUOR_STLB: u32 = 0x0000_0100; /* Search TLBs */
pub const MMUOR_AAN: u32 = 16; /* TLB allocation address */
pub const MMUOR_AAMASK: u32 = 0xffff_0000; /* AA mask */

/* MMU Status register. */
pub const MMUSR_HIT: u32 = 0x0000_0002; /* Search TLB hit */
pub const MMUSR_WF: u32 = 0x0000_0008; /* Write access fault */
pub const MMUSR_RF: u32 = 0x0000_0010; /* Read access fault */
pub const MMUSR_SPF: u32 = 0x0000_0020; /* Supervisor protect fault */

/* MMU Read/Write Tag register. */
pub const MMUTR_V: u32 = 0x0000_0001; /* Valid */
pub const MMUTR_SG: u32 = 0x0000_0002; /* Shared global */
pub const MMUTR_IDN: u32 = 2; /* Address Space ID */
pub const MMUTR_IDMASK: u32 = 0x0000_03fc; /* ASID mask */
pub const MMUTR_VAN: u32 = 10; /* Virtual Address */
pub const MMUTR_VAMASK: u32 = 0xffff_fc00; /* VA mask */

/* MMU Read/Write Data register. */
pub const MMUDR_LK: u32 = 0x0000_0002; /* Lock entry */
pub const MMUDR_X: u32 = 0x0000_0004; /* Execute access enable */
pub const MMUDR_W: u32 = 0x0000_0008; /* Write access enable */
pub const MMUDR_R: u32 = 0x0000_0010; /* Read access enable */
pub const MMUDR_SP: u32 = 0x0000_0020; /* Supervisor access enable */
pub const MMUDR_CM_CWT: u32 = 0x0000_0000; /* Cachable write thru */
pub const MMUDR_CM_CCB: u32 = 0x0000_0040; /* Cachable copy back */
pub const MMUDR_CM_NCP: u32 = 0x0000_0080; /* Non-cachable precise */
pub const MMUDR_CM_NCI: u32 = 0x0000_00c0; /* Non-cachable imprecise */
pub const MMUDR_SZ_1MB: u32 = 0x0000_0000; /* 1MB page size */
pub const MMUDR_SZ_4KB: u32 = 0x0000_0100; /* 4kB page size */
pub const MMUDR_SZ_8KB: u32 = 0x0000_0200; /* 8kB page size */
pub const MMUDR_SZ_1KB: u32 = 0x0000_0300; /* 1kB page size */
pub const MMUDR_PAN: u32 = 10; /* Physical address */
pub const MMUDR_PAMASK: u32 = 0xffff_fc00; /* PA mask */

/* C-only declarations; excluded by the original __ASSEMBLER__ condition. */
pub unsafe fn mmu_read(a: u32) -> u32 {
    core::ptr::read_volatile(a as *const u32)
}

pub unsafe fn mmu_write(a: u32, v: u32) {
    core::ptr::write_volatile(a as *mut u32, v);
    core::arch::asm!("nop");
}

extern "C" {
    pub fn cf_bootmem_alloc();
    pub fn cf_mmu_context_init();
    pub fn cf_tlb_miss(regs: *mut pt_regs, write: i32, dtlb: i32, extension_word: i32) -> i32;
}

/* Supplied by the surrounding kernel translation. */
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
