/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

/* CONFIG_ARC_CACHE_LINE_SHIFT is a build-time configuration value. */
pub const L1_CACHE_SHIFT: u32 = 6;
pub const L1_CACHE_BYTES: u32 = 1u32 << L1_CACHE_SHIFT;
pub const CACHE_LINE_MASK: u32 = !(L1_CACHE_BYTES - 1);

/*
 * ARC700 doesn't cache any access in top 1G (0xc000_0000 to 0xFFFF_FFFF)
 * Ideal for wiring memory mapped peripherals as we don't need to do
 * explicit uncached accesses (LD.di/ST.di) hence more portable drivers
 */
pub const ARC_UNCACHED_ADDR_SPACE: u32 = 0xc000_0000;

/* Uncached access macros. */
#[inline]
pub unsafe fn arc_read_uncached_32(ptr: *const core::ffi::c_void) -> u32 {
    let ret: u32;
    core::arch::asm!(
        "ld.di {ret}, [{ptr}]",
        ret = lateout(reg) ret,
        ptr = in(reg) ptr,
        options(nostack)
    );
    ret
}

#[inline]
pub unsafe fn arc_write_uncached_32(ptr: *mut core::ffi::c_void, data: u32) {
    core::arch::asm!(
        "st.di {data}, [{ptr}]",
        data = in(reg) data,
        ptr = in(reg) ptr,
        options(nostack)
    );
}

/* Largest line length for either L1 or L2 is 128 bytes. */
pub const SMP_CACHE_BYTES: usize = 128;
#[inline]
pub const fn cache_line_size() -> usize { SMP_CACHE_BYTES }
pub const ARCH_DMA_MINALIGN: usize = SMP_CACHE_BYTES;

/*
 * Make sure slab-allocated buffers are 64-bit aligned when atomic64_t uses
 * ARCv2 64-bit atomics (LLOCKD/SCONDD). This guarantess runtime 64-bit
 * alignment for any atomic64_t embedded in buffer.
 * Default ARCH_SLAB_MINALIGN is __alignof__(long long) which has a relaxed
 * value of 4 (and not 8) in ARC ABI.
 */
/* CONFIG_ARC_HAS_LL64 && CONFIG_ARC_HAS_LLSC: */
pub const ARCH_SLAB_MINALIGN: usize = 8;

unsafe extern "C" {
    pub static mut ioc_enable: core::ffi::c_int;
    pub static mut perip_base: core::ffi::c_ulong;
    pub static mut perip_end: core::ffi::c_ulong;
}

/* Instruction cache related Auxiliary registers. */
pub const ARC_REG_IC_BCR: u32 = 0x77;
pub const ARC_REG_IC_IVIC: u32 = 0x10;
pub const ARC_REG_IC_CTRL: u32 = 0x11;
pub const ARC_REG_IC_IVIR: u32 = 0x16;
pub const ARC_REG_IC_ENDR: u32 = 0x17;
pub const ARC_REG_IC_IVIL: u32 = 0x19;
pub const ARC_REG_IC_PTAG: u32 = 0x1e;
pub const ARC_REG_IC_PTAG_HI: u32 = 0x1f;

/* Bit val in IC_CTRL. */
pub const IC_CTRL_DIS: u32 = 0x1;

/* Data cache related Auxiliary registers. */
pub const ARC_REG_DC_BCR: u32 = 0x72;
pub const ARC_REG_DC_IVDC: u32 = 0x47;
pub const ARC_REG_DC_CTRL: u32 = 0x48;
pub const ARC_REG_DC_IVDL: u32 = 0x4a;
pub const ARC_REG_DC_FLSH: u32 = 0x4b;
pub const ARC_REG_DC_FLDL: u32 = 0x4c;
pub const ARC_REG_DC_STARTR: u32 = 0x4d;
pub const ARC_REG_DC_ENDR: u32 = 0x4e;
pub const ARC_REG_DC_PTAG: u32 = 0x5c;
pub const ARC_REG_DC_PTAG_HI: u32 = 0x5f;

/* Bit val in DC_CTRL. */
pub const DC_CTRL_DIS: u32 = 0x001;
pub const DC_CTRL_INV_MODE_FLUSH: u32 = 0x040;
pub const DC_CTRL_FLUSH_STATUS: u32 = 0x100;
pub const DC_CTRL_RGN_OP_INV: u32 = 0x200;
pub const DC_CTRL_RGN_OP_MSK: u32 = 0x200;

/* System-level cache (L2 cache) related Auxiliary registers. */
pub const ARC_REG_SLC_CFG: u32 = 0x901;
pub const ARC_REG_SLC_CTRL: u32 = 0x903;
pub const ARC_REG_SLC_FLUSH: u32 = 0x904;
pub const ARC_REG_SLC_INVALIDATE: u32 = 0x905;
pub const ARC_AUX_SLC_IVDL: u32 = 0x910;
pub const ARC_AUX_SLC_FLDL: u32 = 0x912;
pub const ARC_REG_SLC_RGN_START: u32 = 0x914;
pub const ARC_REG_SLC_RGN_START1: u32 = 0x915;
pub const ARC_REG_SLC_RGN_END: u32 = 0x916;
pub const ARC_REG_SLC_RGN_END1: u32 = 0x917;

/* Bit val in SLC_CONTROL. */
pub const SLC_CTRL_DIS: u32 = 0x001;
pub const SLC_CTRL_IM: u32 = 0x040;
pub const SLC_CTRL_BUSY: u32 = 0x100;
pub const SLC_CTRL_RGN_OP_INV: u32 = 0x200;

/* IO coherency related Auxiliary registers. */
pub const ARC_REG_IO_COH_ENABLE: u32 = 0x500;
pub const ARC_IO_COH_ENABLE_BIT: u32 = 1u32 << 0;
pub const ARC_REG_IO_COH_PARTIAL: u32 = 0x501;
pub const ARC_IO_COH_PARTIAL_BIT: u32 = 1u32 << 0;
pub const ARC_REG_IO_COH_AP0_BASE: u32 = 0x508;
pub const ARC_REG_IO_COH_AP0_SIZE: u32 = 0x509;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
