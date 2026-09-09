/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Hardware-accelerated CRC-32 variants for Linux on z Systems
 *
 * Use the z/Architecture Vector Extension Facility to accelerate the
 * computing of CRC-32 checksums.
 *
 * This CRC-32 implementation algorithm processes the most-significant
 * bit first (BE).
 *
 * Copyright IBM Corp. 2015
 * Author(s): Hendrik Brueckner <brueckner@linux.vnet.ibm.com>
 */

/* External low-level vector/FPU operations supplied by the surrounding code. */
unsafe extern "C" {
    fn fpu_vlm(first: i32, last: i32, addr: *const core::ffi::c_void);
    fn fpu_vzero(register: i32);
    fn fpu_vlvgf(register: i32, value: u32, index: i32);
    fn fpu_vx(dst: i32, src1: i32, src2: i32);
    fn fpu_vl(register: i32, addr: *const u8);
    fn fpu_vgfmag(dst: i32, constants: i32, src1: i32, src2: i32);
    fn fpu_vgfmg(dst: i32, constants: i32, src: i32);
    fn fpu_vupllf(dst: i32, src: i32);
    fn fpu_vlgvf(register: i32, index: i32) -> u32;
}

/* Vector register range containing CRC-32 constants */
const CONST_R1R2: i32 = 9;
const CONST_R3R4: i32 = 10;
const CONST_R5: i32 = 11;
const CONST_R6: i32 = 12;
const CONST_RU_POLY: i32 = 13;
const CONST_CRC_POLY: i32 = 14;

static mut CONSTANTS_CRC_32_BE: [u64; 12] = [
    0x08833794c, 0x0e6228b11, /* R1, R2 */
    0x0c5b9cd4c, 0x0e8a45605, /* R3, R4 */
    0x0f200aa66, 1u64 << 32, /* R5, x32 */
    0x0490d678d, 1,           /* R6, 1 */
    0x104d101df, 0,           /* u */
    0x104C11DB7, 0,           /* P(x) */
];

/**
 * crc32_be_vgfm_16 - Compute CRC-32 (BE variant) with vector registers
 * @crc: Initial CRC value, typically ~0.
 * @buf: Input buffer pointer, performance might be improved if the
 *       buffer is on a doubleword boundary.
 * @size: Size of the buffer, must be 64 bytes or greater.
 */
pub unsafe fn crc32_be_vgfm_16(mut crc: u32, mut buf: *const u8, mut size: usize) -> u32 {
    /* Load CRC-32 constants */
    fpu_vlm(CONST_R1R2, CONST_CRC_POLY, core::ptr::addr_of!(CONSTANTS_CRC_32_BE) as *const _ as *const core::ffi::c_void);
    fpu_vzero(0);

    /* Load the initial CRC value into the leftmost word of V0. */
    fpu_vlvgf(0, crc, 0);

    /* Load a 64-byte data chunk and XOR with CRC */
    fpu_vlm(1, 4, buf as *const core::ffi::c_void);
    fpu_vx(1, 0, 1);
    buf = buf.add(64);
    size -= 64;

    while size >= 64 {
        /* Load the next 64-byte data chunk into V5 to V8 */
        fpu_vlm(5, 8, buf as *const core::ffi::c_void);
        fpu_vgfmag(1, CONST_R1R2, 1, 5);
        fpu_vgfmag(2, CONST_R1R2, 2, 6);
        fpu_vgfmag(3, CONST_R1R2, 3, 7);
        fpu_vgfmag(4, CONST_R1R2, 4, 8);
        buf = buf.add(64);
        size -= 64;
    }

    /* Fold V1 to V4 into a single 128-bit value in V1 */
    fpu_vgfmag(1, CONST_R3R4, 1, 2);
    fpu_vgfmag(1, CONST_R3R4, 1, 3);
    fpu_vgfmag(1, CONST_R3R4, 1, 4);

    while size >= 16 {
        fpu_vl(2, buf);
        fpu_vgfmag(1, CONST_R3R4, 1, 2);
        buf = buf.add(16);
        size -= 16;
    }

    fpu_vgfmg(1, CONST_R5, 1);
    fpu_vgfmg(1, CONST_R6, 1);
    fpu_vupllf(2, 1);
    fpu_vgfmg(2, CONST_RU_POLY, 2);
    fpu_vupllf(2, 2);
    fpu_vgfmag(2, CONST_CRC_POLY, 2, 1);
    fpu_vlgvf(2, 3)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
