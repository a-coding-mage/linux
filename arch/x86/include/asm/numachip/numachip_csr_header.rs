/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Numascale NumaConnect-Specific Header file
 *
 * Copyright (C) 2011 Numascale AS. All rights reserved.
 *
 * Send feedback to <support@numascale.com>
 *
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/smp.h, linux/io.h

pub const CSR_NODE_SHIFT: usize = 16;

#[inline]
pub const fn csr_node_bits(p: usize) -> usize {
    p << CSR_NODE_SHIFT
}

pub const CSR_NODE_MASK: usize = 0x0fff; // 4K nodes

// 32K CSR space, b15 indicates geo/non-geo
pub const CSR_OFFSET_MASK: usize = 0x7fff;
pub const CSR_G0_NODE_IDS: usize = 0x008 + (0 << 12);
pub const CSR_G3_EXT_IRQ_GEN: usize = 0x030 + (3 << 12);

/*
 * Local CSR space starts in global CSR space with "nodeid" = 0xfff0, however
 * when using the direct mapping on x86_64, both start and size needs to be
 * aligned with PMD_SIZE which is 2M
 */
pub const NUMACHIP_LCSR_BASE: u64 = 0x3ffffe000000;
pub const NUMACHIP_LCSR_LIM: u64 = 0x3fffffffffff;
pub const NUMACHIP_LCSR_SIZE: u64 = NUMACHIP_LCSR_LIM - NUMACHIP_LCSR_BASE + 1;
pub const NUMACHIP_LAPIC_BITS: usize = 8;

extern "C" {
    fn __va(addr: usize) -> *mut core::ffi::c_void;
    fn swab32(value: u32) -> u32;
    fn readl(addr: *const core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn readq(addr: *const core::ffi::c_void) -> u64;
    fn writeq(value: u64, addr: *mut core::ffi::c_void);
    fn smp_processor_id() -> u32;
}

#[inline]
pub unsafe fn lcsr_address(offset: usize) -> *mut core::ffi::c_void {
    __va((NUMACHIP_LCSR_BASE as usize)
        | (1usize << 15)
        | csr_node_bits(0xfff0)
        | (offset & CSR_OFFSET_MASK))
}

#[inline]
pub unsafe fn read_lcsr(offset: usize) -> u32 {
    swab32(readl(lcsr_address(offset)))
}

#[inline]
pub unsafe fn write_lcsr(offset: usize, val: u32) {
    writel(swab32(val), lcsr_address(offset));
}

/*
 * On NumaChip2, local CSR space is 16MB and starts at fixed offset below 4G
 */

pub const NUMACHIP2_LCSR_BASE: usize = 0xf0000000;
pub const NUMACHIP2_LCSR_SIZE: usize = 0x1000000;
pub const NUMACHIP2_APIC_ICR: usize = 0x100000;
pub const NUMACHIP2_TIMER_DEADLINE: usize = 0x200000;
pub const NUMACHIP2_TIMER_INT: usize = 0x200008;
pub const NUMACHIP2_TIMER_NOW: usize = 0x200018;
pub const NUMACHIP2_TIMER_RESET: usize = 0x200020;

#[inline]
pub unsafe fn numachip2_lcsr_address(offset: usize) -> *mut core::ffi::c_void {
    __va(NUMACHIP2_LCSR_BASE | (offset & (NUMACHIP2_LCSR_SIZE - 1)))
}

#[inline]
pub unsafe fn numachip2_read32_lcsr(offset: usize) -> u32 {
    readl(numachip2_lcsr_address(offset))
}

#[inline]
pub unsafe fn numachip2_read64_lcsr(offset: usize) -> u64 {
    readq(numachip2_lcsr_address(offset))
}

#[inline]
pub unsafe fn numachip2_write32_lcsr(offset: usize, val: u32) {
    writel(val, numachip2_lcsr_address(offset));
}

#[inline]
pub unsafe fn numachip2_write64_lcsr(offset: usize, val: u64) {
    writeq(val, numachip2_lcsr_address(offset));
}

#[inline]
pub unsafe fn numachip2_timer() -> u32 {
    (smp_processor_id() % 48) << 6
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
