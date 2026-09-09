/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Bit definitions for the MCF54xx ACR and CACR registers.
 */

/* Define the Cache register flags. */
pub const CACR_DEC: u32 = 0x80000000; /* Enable data cache */
pub const CACR_DWP: u32 = 0x40000000; /* Data write protection */
pub const CACR_DESB: u32 = 0x20000000; /* Enable data store buffer */
pub const CACR_DDPI: u32 = 0x10000000; /* Disable invalidation by CPUSHL */
pub const CACR_DHCLK: u32 = 0x08000000; /* Half data cache lock mode */
pub const CACR_DDCM_WT: u32 = 0x00000000; /* Write through cache */
pub const CACR_DDCM_CP: u32 = 0x02000000; /* Copyback cache */
pub const CACR_DDCM_P: u32 = 0x04000000; /* No cache, precise */
pub const CACR_DDCM_IMP: u32 = 0x06000000; /* No cache, imprecise */
pub const CACR_DCINVA: u32 = 0x01000000; /* Invalidate data cache */
pub const CACR_BEC: u32 = 0x00080000; /* Enable branch cache */
pub const CACR_BCINVA: u32 = 0x00040000; /* Invalidate branch cache */
pub const CACR_IEC: u32 = 0x00008000; /* Enable instruction cache */
pub const CACR_DNFB: u32 = 0x00002000; /* Inhibited fill buffer */
pub const CACR_IDPI: u32 = 0x00001000; /* Disable CPUSHL */
pub const CACR_IHLCK: u32 = 0x00000800; /* Instruction cache half lock */
pub const CACR_IDCM: u32 = 0x00000400; /* Instruction cache inhibit */
pub const CACR_ICINVA: u32 = 0x00000100; /* Invalidate instr cache */
pub const CACR_EUSP: u32 = 0x00000020; /* Enable separate user a7 */

pub const ACR_BASE_POS: u32 = 24;
pub const ACR_MASK_POS: u32 = 16;
pub const ACR_ENABLE: u32 = 0x00008000;
pub const ACR_USER: u32 = 0x00000000;
pub const ACR_SUPER: u32 = 0x00002000;
pub const ACR_ANY: u32 = 0x00004000;
pub const ACR_CM_WT: u32 = 0x00000000;
pub const ACR_CM_CP: u32 = 0x00000020;
pub const ACR_CM_OFF_PRE: u32 = 0x00000040;
pub const ACR_CM_OFF_IMP: u32 = 0x00000060;
pub const ACR_CM: u32 = 0x00000060;
pub const ACR_SP: u32 = 0x00000008;
pub const ACR_WPROTECT: u32 = 0x00000004;

#[inline]
pub const fn ACR_BA(x: u32) -> u32 { x & 0xff000000 }
#[inline]
pub const fn ACR_ADMSK(x: u32) -> u32 { ((x.wrapping_sub(1) & 0xff000000) >> 8) }

#[cfg(CONFIG_M5407)]
pub const ICACHE_SIZE: u32 = 0x4000;
#[cfg(CONFIG_M5407)]
pub const DCACHE_SIZE: u32 = 0x2000;
#[cfg(CONFIG_M54xx)]
pub const ICACHE_SIZE: u32 = 0x8000;
#[cfg(CONFIG_M54xx)]
pub const DCACHE_SIZE: u32 = 0x8000;
#[cfg(CONFIG_M5441x)]
pub const ICACHE_SIZE: u32 = 0x2000;
#[cfg(CONFIG_M5441x)]
pub const DCACHE_SIZE: u32 = 0x2000;

pub const CACHE_LINE_SIZE: u32 = 0x0010;
pub const CACHE_WAYS: u32 = 4;
pub const ICACHE_SET_MASK: u32 = (ICACHE_SIZE / 64 - 1) << CACHE_WAYS;
pub const DCACHE_SET_MASK: u32 = (DCACHE_SIZE / 64 - 1) << CACHE_WAYS;
pub const ICACHE_MAX_ADDR: u32 = ICACHE_SET_MASK;
pub const DCACHE_MAX_ADDR: u32 = DCACHE_SET_MASK;

/* Version 4 cores have separate instruction and data caches. */
/* Enabling CACR_DESB requires a "nop" to flush the store buffer. */
/* Use '+' instead of '|' for assembler's sake. */
#[cfg(CONFIG_M5407)]
pub const CACHE_MODE: u32 = CACR_DEC + CACR_DESB + CACR_DDCM_P + CACR_BEC + CACR_IEC;
#[cfg(not(CONFIG_M5407))]
pub const CACHE_MODE: u32 = CACR_DEC + CACR_DESB + CACR_DDCM_P + CACR_BEC + CACR_IEC + CACR_EUSP;
pub const CACHE_INIT: u32 = CACR_DCINVA + CACR_BCINVA + CACR_ICINVA;

/* Build-time CONFIG_MMU and CONFIG_CACHE_COPYBACK conditions are preserved with cfg. */
#[cfg(CONFIG_MMU)]
pub const ACR0_MODE: u32 = ACR_BA(IOMEMBASE) + ACR_ADMSK(IOMEMSIZE) + ACR_ENABLE + ACR_SUPER + ACR_CM_OFF_PRE + ACR_SP;
#[cfg(all(CONFIG_MMU, CONFIG_CACHE_COPYBACK))]
pub const ACR1_MODE: u32 = ACR_BA(CONFIG_RAMBASE) + ACR_ADMSK(CONFIG_RAMSIZE) + ACR_ENABLE + ACR_SUPER + ACR_SP + ACR_CM_CP;
#[cfg(all(CONFIG_MMU, not(CONFIG_CACHE_COPYBACK)))]
pub const ACR1_MODE: u32 = ACR_BA(CONFIG_RAMBASE) + ACR_ADMSK(CONFIG_RAMSIZE) + ACR_ENABLE + ACR_SUPER + ACR_SP + ACR_CM_WT;
#[cfg(CONFIG_MMU)]
pub const ACR2_MODE: u32 = 0;
#[cfg(CONFIG_MMU)]
pub const ACR3_MODE: u32 = ACR_BA(CONFIG_RAMBASE) + ACR_ADMSK(CONFIG_RAMSIZE) + ACR_ENABLE + ACR_SUPER + ACR_SP;

#[cfg(not(CONFIG_MMU))]
#[cfg(CONFIG_CACHE_COPYBACK)]
pub const DATA_CACHE_MODE: u32 = ACR_ENABLE + ACR_ANY + ACR_CM_CP;
#[cfg(all(not(CONFIG_MMU), not(CONFIG_CACHE_COPYBACK)))]
pub const DATA_CACHE_MODE: u32 = ACR_ENABLE + ACR_ANY + ACR_CM_WT;
#[cfg(not(CONFIG_MMU))]
pub const INSN_CACHE_MODE: u32 = ACR_ENABLE + ACR_ANY;
#[cfg(not(CONFIG_MMU))]
pub const CACHE_INVALIDATE: u32 = CACHE_MODE + CACR_DCINVA + CACR_BCINVA + CACR_ICINVA;
#[cfg(not(CONFIG_MMU))]
pub const CACHE_INVALIDATEI: u32 = CACHE_MODE + CACR_BCINVA + CACR_ICINVA;
#[cfg(not(CONFIG_MMU))]
pub const CACHE_INVALIDATED: u32 = CACHE_MODE + CACR_DCINVA;
#[cfg(not(CONFIG_MMU))]
pub const ACR0_MODE: u32 = 0x000f0000 + DATA_CACHE_MODE;
#[cfg(not(CONFIG_MMU))]
pub const ACR1_MODE: u32 = 0;
#[cfg(not(CONFIG_MMU))]
pub const ACR2_MODE: u32 = 0x000f0000 + INSN_CACHE_MODE;
#[cfg(not(CONFIG_MMU))]
pub const ACR3_MODE: u32 = 0;

/* Copyback cache mode must push dirty cache lines first. */
#[cfg(all(not(CONFIG_MMU), CONFIG_CACHE_COPYBACK))]
pub const CACHE_PUSH: () = ();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
