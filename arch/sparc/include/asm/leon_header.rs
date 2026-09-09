/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2004 Konrad Eisele (eiselekd@web.de,konrad@gaisler.com) Gaisler Research
 * Copyright (C) 2004 Stefan Holst (mail@s-holst.de) Uni-Stuttgart
 * Copyright (C) 2009 Daniel Hellstrom (daniel@gaisler.com) Aeroflex Gaisler AB
 * Copyright (C) 2009 Konrad Eisele (konrad@gaisler.com) Aeroflex Gaisler AB
 */

pub const LEON_CNR_CTRL: u32 = 0x000;
pub const LEON_CNR_CTXP: u32 = 0x100;
pub const LEON_CNR_CTX: u32 = 0x200;
pub const LEON_CNR_F: u32 = 0x300;
pub const LEON_CNR_FADDR: u32 = 0x400;
pub const LEON_CNR_CTX_NCTX: u32 = 256;
pub const LEON_CNR_CTRL_TLBDIS: u32 = 0x80000000;
pub const LEON_MMUTLB_ENT_MAX: u32 = 64;
pub const LEON_DIAGF_LVL: u32 = 0x3;
pub const LEON_DIAGF_WR: u32 = 0x8;
pub const LEON_DIAGF_WR_SHIFT: u32 = 3;
pub const LEON_DIAGF_HIT: u32 = 0x10;
pub const LEON_DIAGF_HIT_SHIFT: u32 = 4;
pub const LEON_DIAGF_CTX: u32 = 0x1fe0;
pub const LEON_DIAGF_CTX_SHIFT: u32 = 5;
pub const LEON_DIAGF_VALID: u32 = 0x2000;
pub const LEON_DIAGF_VALID_SHIFT: u32 = 13;
pub const LEON_IRQMASK_R: u32 = 0x0000fffe;
pub const LEON_IRQPRIO_R: u32 = 0xfffe0000;
pub const LEON_MCFG2_SRAMDIS: u32 = 0x00002000;
pub const LEON_MCFG2_SDRAMEN: u32 = 0x00004000;
pub const LEON_MCFG2_SRAMBANKSZ: u32 = 0x00001e00;
pub const LEON_MCFG2_SRAMBANKSZ_SHIFT: u32 = 9;
pub const LEON_MCFG2_SDRAMBANKSZ: u32 = 0x03800000;
pub const LEON_MCFG2_SDRAMBANKSZ_SHIFT: u32 = 23;
pub const LEON_TCNT0_MASK: u32 = 0x7fffff;
pub const ASI_LEON3_SYSCTRL: u32 = 0x02;
pub const ASI_LEON3_SYSCTRL_ICFG: u32 = 0x08;
pub const ASI_LEON3_SYSCTRL_DCFG: u32 = 0x0c;
pub const ASI_LEON3_SYSCTRL_CFG_SNOOPING: u32 = 1 << 27;

#[macro_export]
macro_rules! LEON_HARD_INT { ($x:expr) => { 1u32 << ($x) }; }
#[macro_export]
macro_rules! ASI_LEON3_SYSCTRL_CFG_SSIZE { ($c:expr) => { 1u32 << (($c >> 20) & 0xf) }; }

extern "C" {
    pub fn leon_switch_mm();
    pub fn leon_init_IRQ();
    pub fn leon_swprobe(vaddr: usize, paddr: *mut usize) -> usize;
    pub fn leon_flush_icache_all();
    pub fn leon_flush_dcache_all();
    pub fn leon_flush_cache_all();
    pub fn leon_flush_tlb_all();
    pub static mut leon_flush_during_switch: i32;
    pub fn leon_flush_needed() -> i32;
}

#[inline(always)]
pub unsafe fn leon_store_reg(paddr: usize, value: usize) {
    core::arch::asm!("sta {value}, [{paddr}] 0", value = in(reg) value, paddr = in(reg) paddr, options(nostack));
}

#[inline(always)]
pub unsafe fn leon_load_reg(paddr: usize) -> usize {
    let retval: usize;
    core::arch::asm!("lda [{paddr}] 0, {retval}", paddr = in(reg) paddr, retval = out(reg) retval, options(nostack));
    retval
}

#[macro_export]
macro_rules! LEON3_BYPASS_LOAD_PA { ($x:expr) => { unsafe { $crate::leon_load_reg($x as usize) } }; }
#[macro_export]
macro_rules! LEON3_BYPASS_STORE_PA { ($x:expr, $v:expr) => { unsafe { $crate::leon_store_reg($x as usize, $v as usize) } }; }
#[macro_export]
macro_rules! LEON_BYPASS_LOAD_PA { ($x:expr) => { unsafe { $crate::leon_load_reg($x as usize) } }; }
#[macro_export]
macro_rules! LEON_BYPASS_STORE_PA { ($x:expr, $v:expr) => { unsafe { $crate::leon_store_reg($x as usize, $v as usize) } }; }

#[inline(always)]
pub unsafe fn sparc_leon3_get_dcachecfg() -> u32 {
    let retval: u32;
    core::arch::asm!("lda [{addr}] 2, {retval}", addr = in(reg) ASI_LEON3_SYSCTRL_DCFG, retval = out(reg) retval, options(nostack));
    retval
}

#[inline(always)] pub unsafe fn sparc_leon3_enable_snooping() { core::arch::asm!("lda [%g0] 2, %l1\n\tset 0x800000, %l2\n\tor %l2, %l1, %l2\n\tsta %l2, [%g0] 2", options(nostack)); }
#[inline(always)] pub unsafe fn sparc_leon3_snooping_enabled() -> i32 { let cctrl: u32; core::arch::asm!("lda [%g0] 2, {cctrl}", cctrl = out(reg) cctrl, options(nostack)); (((cctrl >> 23) & 1) != 0 && ((cctrl >> 17) & 1) != 0) as i32 }
#[inline(always)] pub unsafe fn sparc_leon3_disable_cache() { core::arch::asm!("lda [%g0] 2, %l1\n\tset 0x00000f, %l2\n\tandn %l2, %l1, %l2\n\tsta %l2, [%g0] 2", options(nostack)); }
#[inline(always)] pub unsafe fn sparc_leon3_asr17() -> u32 { let asr17: u32; core::arch::asm!("rd %asr17, {asr17}", asr17 = out(reg) asr17, options(nostack)); asr17 }
#[inline(always)] pub unsafe fn sparc_leon3_cpuid() -> u32 { sparc_leon3_asr17() >> 28 }

// CONFIG_SMP conditionals are preserved from the source build configuration.
#[cfg(feature = "CONFIG_SMP")]
pub const LEON3_IRQ_IPI_DEFAULT: u32 = 13;
#[cfg(feature = "CONFIG_SMP")]
pub const LEON3_IRQ_CROSS_CALL: u32 = 15;

// PAGE_SIZE_LEON_8K/PAGE_SIZE_LEON_16K and the source's page-layout conditional.
pub const LEON_PAGE_SIZE_LEON: u32 = 0;
pub const LEON_PGD_SH: u32 = 24;
pub const LEON_PGD_M: u32 = 0xff;
pub const LEON_PMD_SH: u32 = 18;
pub const LEON_PMD_SH_V: u32 = LEON_PGD_SH - 2;
pub const LEON_PMD_M: u32 = 0x3f;
pub const LEON_PTE_SH: u32 = 12;
pub const LEON_PTE_M: u32 = 0x3f;
pub const LEON3_XCCR_SETS_MASK: usize = 0x07000000;
pub const LEON3_XCCR_SSIZE_MASK: usize = 0x00f00000;
pub const LEON2_CCR_DSETS_MASK: usize = 0x03000000;
pub const LEON2_CFG_SSIZE_MASK: usize = 0x00007000;

pub enum vm_area_struct {}
pub enum device_node {}
pub enum task_struct {}
pub type irq_flow_handler_t = unsafe extern "C" fn();
pub type irqreturn_t = i32;

#[repr(C)]
pub struct leon3_cacheregs { pub ccr: usize, pub iccr: usize, pub dccr: usize }

extern "C" {
    pub fn leon_flush_pcache_all(vma: *mut vm_area_struct, page: usize);
    pub fn leon_build_device_irq(real_irq: u32, flow_handler: irq_flow_handler_t, name: *const core::ffi::c_char, do_ack: i32) -> u32;
    pub fn leon_update_virq_handling(virq: u32, flow_handler: irq_flow_handler_t, name: *const core::ffi::c_char, do_ack: i32);
    pub fn leon_init_timers();
    pub fn leon_node_init(dp: *mut device_node, nextp: *mut *mut *mut device_node);
    pub fn init_leon();
    pub fn poke_leonsparc();
    pub fn leon3_getCacheRegs(regs: *mut leon3_cacheregs);
    pub static mut leon3_ticker_irq: i32;
    pub static mut smpleon_ipi: [u32; 0];
    pub static mut linux_trap_ipi15_leon: [u32; 0];
    pub static mut leon_ipi_irq: i32;
}

#[macro_export] macro_rules! PFN { ($x:expr) => { ($x) >> PAGE_SHIFT }; }
#[macro_export] macro_rules! _pfn_valid { ($pfn:expr) => { ($pfn < last_valid_pfn) && ($pfn >= PFN!(phys_base)) }; }
pub const _SRMMU_PTE_PMASK_LEON: u32 = 0xffffffff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
