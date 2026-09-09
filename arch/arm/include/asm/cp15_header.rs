/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: asm/barrier.h (provides isb()).

/* CR1 bits (CP#15 CR1) */
pub const CR_M: usize = 1 << 0; /* MMU enable */
pub const CR_A: usize = 1 << 1; /* Alignment abort enable */
pub const CR_C: usize = 1 << 2; /* Dcache enable */
pub const CR_W: usize = 1 << 3; /* Write buffer enable */
pub const CR_P: usize = 1 << 4; /* 32-bit exception handler */
pub const CR_D: usize = 1 << 5; /* 32-bit data address range */
pub const CR_L: usize = 1 << 6; /* Implementation defined */
pub const CR_B: usize = 1 << 7; /* Big endian */
pub const CR_S: usize = 1 << 8; /* System MMU protection */
pub const CR_R: usize = 1 << 9; /* ROM MMU protection */
pub const CR_F: usize = 1 << 10; /* Implementation defined */
pub const CR_Z: usize = 1 << 11; /* Implementation defined */
pub const CR_I: usize = 1 << 12; /* Icache enable */
pub const CR_V: usize = 1 << 13; /* Vectors relocated to 0xffff0000 */
pub const CR_RR: usize = 1 << 14; /* Round Robin cache replacement */
pub const CR_L4: usize = 1 << 15; /* LDR pc can set T bit */
pub const CR_DT: usize = 1 << 16;
// CONFIG_MMU selects CR_HA; otherwise CR_BR is provided.
#[cfg(feature = "CONFIG_MMU")]
pub const CR_HA: usize = 1 << 17; /* Hardware management of Access Flag */
#[cfg(not(feature = "CONFIG_MMU"))]
pub const CR_BR: usize = 1 << 17; /* MPU Background region enable (PMSA) */
pub const CR_IT: usize = 1 << 18;
pub const CR_ST: usize = 1 << 19;
pub const CR_FI: usize = 1 << 21; /* Fast interrupt (lower latency mode) */
pub const CR_U: usize = 1 << 22; /* Unaligned access operation */
pub const CR_XP: usize = 1 << 23; /* Extended page tables */
pub const CR_VE: usize = 1 << 24; /* Vectored interrupts */
pub const CR_EE: usize = 1 << 25; /* Exception (Big) Endian */
pub const CR_TRE: usize = 1 << 28; /* TEX remap enable */
pub const CR_AFE: usize = 1 << 29; /* Access flag enable */
pub const CR_TE: usize = 1 << 30; /* Thumb exception enable */

// The original header is excluded for assembly sources.
// For ARM architecture >= 4, vectors_high() evaluates get_cr() & CR_V;
// older architectures return 0.
extern "C" {
    fn isb();
}

#[cfg(feature = "LINUX_ARM_ARCH_GE_4")]
#[inline]
pub unsafe fn vectors_high() -> usize { get_cr() & CR_V }

#[cfg(not(feature = "LINUX_ARM_ARCH_GE_4"))]
#[inline]
pub const fn vectors_high() -> usize { 0 }

#[cfg(feature = "CONFIG_CPU_CP15")]
extern "C" {
    pub static mut cr_alignment: usize; /* defined in entry-armv.S */
}

#[cfg(feature = "CONFIG_CPU_CP15")]
#[inline]
pub unsafe fn get_cr() -> usize {
    let val: usize;
    core::arch::asm!("mrc p15, 0, {0}, c1, c0, 0 // get CR", out(reg) val, options(nomem, nostack));
    val
}

#[cfg(feature = "CONFIG_CPU_CP15")]
#[inline]
pub unsafe fn set_cr(val: usize) {
    core::arch::asm!("mcr p15, 0, {0}, c1, c0, 0 // set CR", in(reg) val, options(nomem, nostack));
    isb();
}

#[cfg(feature = "CONFIG_CPU_CP15")]
#[inline]
pub unsafe fn get_auxcr() -> u32 {
    let val: u32;
    core::arch::asm!("mrc p15, 0, {0}, c1, c0, 1 // get AUXCR", out(reg) val, options(nomem, nostack));
    val
}

#[cfg(feature = "CONFIG_CPU_CP15")]
#[inline]
pub unsafe fn set_auxcr(val: u32) {
    core::arch::asm!("mcr p15, 0, {0}, c1, c0, 1 // set AUXCR", in(reg) val, options(nomem, nostack));
    isb();
}

pub const fn CPACC_FULL(n: usize) -> u32 { 3 << (n * 2) }
pub const fn CPACC_SVC(n: usize) -> u32 { 1 << (n * 2) }
pub const fn CPACC_DISABLE(_n: usize) -> u32 { 0 }

#[cfg(feature = "CONFIG_CPU_CP15")]
#[inline]
pub unsafe fn get_copro_access() -> u32 {
    let val: u32;
    core::arch::asm!("mrc p15, 0, {0}, c1, c0, 2 // get copro access", out(reg) val, options(nomem, nostack));
    val
}

#[cfg(feature = "CONFIG_CPU_CP15")]
#[inline]
pub unsafe fn set_copro_access(val: u32) {
    core::arch::asm!("mcr p15, 0, {0}, c1, c0, 2 // set copro access", in(reg) val, options(nomem, nostack));
    isb();
}

#[cfg(not(feature = "CONFIG_CPU_CP15"))]
pub const cr_alignment: usize = 0;

#[cfg(not(feature = "CONFIG_CPU_CP15"))]
#[inline]
pub const fn get_cr() -> usize { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
