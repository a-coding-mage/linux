/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

pub const L1_CACHE_SHIFT: u32 = 6;
pub const L1_CACHE_BYTES: u32 = 1 << L1_CACHE_SHIFT;

pub const CLIDR_LOUU_SHIFT: u32 = 27;
pub const CLIDR_LOC_SHIFT: u32 = 24;
pub const CLIDR_LOUIS_SHIFT: u32 = 21;

#[inline]
pub const fn CLIDR_LOUU(clidr: u64) -> u64 {
    (clidr >> CLIDR_LOUU_SHIFT) & 0x7
}

#[inline]
pub const fn CLIDR_LOC(clidr: u64) -> u64 {
    (clidr >> CLIDR_LOC_SHIFT) & 0x7
}

#[inline]
pub const fn CLIDR_LOUIS(clidr: u64) -> u64 {
    (clidr >> CLIDR_LOUIS_SHIFT) & 0x7
}

/* Ctypen, bits[3(n - 1) + 2 : 3(n - 1)], for n = 1 to 7 */
#[inline]
pub const fn CLIDR_CTYPE_SHIFT(level: u32) -> u32 {
    3 * (level - 1)
}

#[inline]
pub const fn CLIDR_CTYPE_MASK(level: u32) -> u32 {
    7 << CLIDR_CTYPE_SHIFT(level)
}

#[inline]
pub const fn CLIDR_CTYPE(clidr: u32, level: u32) -> u32 {
    (clidr & CLIDR_CTYPE_MASK(level)) >> CLIDR_CTYPE_SHIFT(level)
}

/* Ttypen, bits [2(n - 1) + 34 : 2(n - 1) + 33], for n = 1 to 7 */
#[inline]
pub const fn CLIDR_TTYPE_SHIFT(level: u32) -> u32 {
    2 * (level - 1) + CLIDR_EL1_Ttypen_SHIFT
}

/*
 * Memory returned by kmalloc() may be used for DMA, so we must make
 * sure that all such allocations are cache aligned. Otherwise,
 * unrelated code may cause parts of the buffer to be read into the
 * cache before the transfer is done, causing old data to be seen by
 * the CPU.
 */
pub const ARCH_DMA_MINALIGN: u32 = 128;
pub const ARCH_KMALLOC_MINALIGN: u32 = 8;

/* The following declarations are excluded for assembler and VDSO builds. */

#[cfg(feature = "CONFIG_KASAN_SW_TAGS")]
pub const ARCH_SLAB_MINALIGN: u64 = 1u64 << KASAN_SHADOW_SCALE_SHIFT;

#[cfg(feature = "CONFIG_KASAN_HW_TAGS")]
#[inline]
pub fn arch_slab_minalign() -> usize {
    unsafe {
        if kasan_hw_tags_enabled() {
            MTE_GRANULE_SIZE as usize
        } else {
            core::mem::align_of::<u64>()
        }
    }
}

#[inline]
pub const fn CTR_L1IP(ctr: u32) -> u32 {
    SYS_FIELD_GET_CTR_EL0_L1Ip(ctr)
}

pub const ICACHEF_ALIASING: usize = 0;
extern "C" {
    pub static mut __icache_flags: c_ulong;
}

/*
 * Whilst the D-side always behaves as PIPT on AArch64, aliasing is
 * permitted in the I-cache.
 */
#[inline]
pub unsafe fn icache_is_aliasing() -> i32 {
    test_bit(ICACHEF_ALIASING, core::ptr::addr_of!(__icache_flags))
}

#[inline]
pub unsafe fn cache_type_cwg() -> u32 {
    SYS_FIELD_GET_CTR_EL0_CWG(read_cpuid_cachetype())
}

#[inline]
pub unsafe fn cache_line_size_of_cpu() -> u32 {
    let cwg = cache_type_cwg();
    if cwg != 0 {
        4 << cwg
    } else {
        ARCH_DMA_MINALIGN
    }
}

extern "C" {
    pub fn cache_line_size() -> i32;
}

#[inline]
pub unsafe fn dma_get_cache_alignment() -> i32 {
    cache_line_size()
}

#[inline]
pub unsafe fn arch_sync_dma_flush() {
    dsb(SY);
}

/* Compress a u64 MPIDR value into 32 bits. */
#[inline]
pub unsafe fn arch_compact_of_hwid(id: u64) -> u64 {
    let aff3 = MPIDR_AFFINITY_LEVEL(id, 3);

    /*
     * These bits are expected to be RES0. If not, return a value with
     * the upper 32 bits set to force the caller to give up on 32 bit
     * cache ids.
     */
    if FIELD_GET(GENMASK_ULL(63, 40), id) != 0 {
        return id;
    }

    (aff3 << 24) | FIELD_GET(GENMASK_ULL(23, 0), id)
}

/* arch_compact_of_hwid is also exposed under its original macro name. */

/*
 * Read the effective value of CTR_EL0.
 *
 * According to ARM ARM for ARMv8-A (ARM DDI 0487C.a),
 * section D10.2.33 "CTR_EL0, Cache Type Register" :
 *
 * CTR_EL0.IDC reports the data cache clean requirements for
 * instruction to data coherence.
 *
 *  0 - dcache clean to PoU is required unless :
 *     (CLIDR_EL1.LoC == 0) || (CLIDR_EL1.LoUIS == 0 && CLIDR_EL1.LoUU == 0)
 *  1 - dcache clean to PoU is not required for i-to-d coherence.
 *
 * This routine provides the CTR_EL0 with the IDC field updated to the
 * effective state.
 */
#[inline]
pub unsafe fn read_cpuid_effective_cachetype() -> u32 {
    let mut ctr = read_cpuid_cachetype();

    if (ctr & BIT(CTR_EL0_IDC_SHIFT)) == 0 {
        let clidr = read_sysreg(clidr_el1);

        if CLIDR_LOC(clidr) == 0
            || (CLIDR_LOUIS(clidr) == 0 && CLIDR_LOUU(clidr) == 0)
        {
            ctr |= BIT(CTR_EL0_IDC_SHIFT);
        }
    }

    ctr
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
