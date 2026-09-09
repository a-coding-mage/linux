/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Hardware-accelerated CRC-32 variants for Linux on z Systems
 *
 * Use the z/Architecture Vector Extension Facility to accelerate the
 * computing of bitreflected CRC-32 checksums for IEEE 802.3 Ethernet
 * and Castagnoli.
 *
 * This CRC-32 implementation algorithm is bitreflected and processes
 * the least-significant bit first (Little-Endian).
 *
 * Copyright IBM Corp. 2015
 * Author(s): Hendrik Brueckner <brueckner@linux.vnet.ibm.com>
 */

// Dependencies: linux/types.h, asm/fpu.h, and crc32-vx.h.

/* Vector register range containing CRC-32 constants */
const CONST_PERM_LE2BE: i32 = 9;
const CONST_R2R1: i32 = 10;
const CONST_R4R3: i32 = 11;
const CONST_R5: i32 = 12;
const CONST_RU_POLY: i32 = 13;
const CONST_CRC_POLY: i32 = 14;

/*
 * The CRC-32 constant block contains reduction constants to fold and
 * process particular chunks of the input data stream in parallel.
 *
 * For the CRC-32 variants, the constants are precomputed according to
 * these definitions:
 *
 *     R1 = [(x4*128+32 mod P'(x) << 32)]' << 1
 *     R2 = [(x4*128-32 mod P'(x) << 32)]' << 1
 *     R3 = [(x128+32 mod P'(x) << 32)]'   << 1
 *     R4 = [(x128-32 mod P'(x) << 32)]'   << 1
 *     R5 = [(x64 mod P'(x) << 32)]'       << 1
 *     R6 = [(x32 mod P'(x) << 32)]'       << 1
 *
 *     The bitreflected Barret reduction constant, u', is defined as
 *     the bit reversal of floor(x**64 / P(x)).
 *
 *     where P(x) is the polynomial in the normal domain and the P'(x) is the
 *     polynomial in the reversed (bitreflected) domain.
 *
 * CRC-32 (IEEE 802.3 Ethernet, ...) polynomials:
 *
 *     P(x)  = 0x04C11DB7
 *     P'(x) = 0xEDB88320
 *
 * CRC-32C (Castagnoli) polynomials:
 *
 *     P(x)  = 0x1EDC6F41
 *     P'(x) = 0x82F63B78
 */

static mut constants_CRC_32_LE: [u64; 12] = [
    0x0f0e0d0c0b0a0908, 0x0706050403020100, // BE->LE mask
    0x1c6e41596, 0x154442bd4, // R2, R1
    0x0ccaa009e, 0x1751997d0, // R4, R3
    0x0, 0x163cd6124, // R5
    0x0, 0x1f7011641, // u'
    0x0, 0x1db710641, // P'(x) << 1
];

static mut constants_CRC_32C_LE: [u64; 12] = [
    0x0f0e0d0c0b0a0908, 0x0706050403020100, // BE->LE mask
    0x09e4addf8, 0x740eef02, // R2, R1
    0x14cd00bd6, 0xf20c0dfe, // R4, R3
    0x0, 0x0dd45aab8, // R5
    0x0, 0x0dea713f1, // u'
    0x0, 0x105ec76f0, // P'(x) << 1
];

/**
 * crc32_le_vgfm_generic - Compute CRC-32 (LE variant) with vector registers
 * @crc: Initial CRC value, typically ~0.
 * @buf: Input buffer pointer, performance might be improved if the
 *       buffer is on a doubleword boundary.
 * @size: Size of the buffer, must be 64 bytes or greater.
 * @constants: CRC-32 constant pool base pointer.
 */
unsafe fn crc32_le_vgfm_generic(
    mut crc: u32,
    mut buf: *const u8,
    mut size: usize,
    constants: *mut u64,
) -> u32 {
    fpu_vlm(CONST_PERM_LE2BE, CONST_CRC_POLY, constants);
    fpu_vzero(0);
    fpu_vlvgf(0, crc, 3);

    fpu_vlm(1, 4, buf);
    fpu_vperm(1, 1, 1, CONST_PERM_LE2BE);
    fpu_vperm(2, 2, 2, CONST_PERM_LE2BE);
    fpu_vperm(3, 3, 3, CONST_PERM_LE2BE);
    fpu_vperm(4, 4, 4, CONST_PERM_LE2BE);
    fpu_vx(1, 0, 1);
    buf = buf.add(64);
    size -= 64;

    while size >= 64 {
        fpu_vlm(5, 8, buf);
        fpu_vperm(5, 5, 5, CONST_PERM_LE2BE);
        fpu_vperm(6, 6, 6, CONST_PERM_LE2BE);
        fpu_vperm(7, 7, 7, CONST_PERM_LE2BE);
        fpu_vperm(8, 8, 8, CONST_PERM_LE2BE);
        fpu_vgfmag(1, CONST_R2R1, 1, 5);
        fpu_vgfmag(2, CONST_R2R1, 2, 6);
        fpu_vgfmag(3, CONST_R2R1, 3, 7);
        fpu_vgfmag(4, CONST_R2R1, 4, 8);
        buf = buf.add(64);
        size -= 64;
    }

    fpu_vgfmag(1, CONST_R4R3, 1, 2);
    fpu_vgfmag(1, CONST_R4R3, 1, 3);
    fpu_vgfmag(1, CONST_R4R3, 1, 4);

    while size >= 16 {
        fpu_vl(2, buf);
        fpu_vperm(2, 2, 2, CONST_PERM_LE2BE);
        fpu_vgfmag(1, CONST_R4R3, 1, 2);
        buf = buf.add(16);
        size -= 16;
    }

    fpu_vleib(9, 0x40, 7);
    fpu_vsrlb(0, CONST_R4R3, 9);
    fpu_vleig(0, 1, 0);
    fpu_vgfmg(1, 0, 1);

    fpu_vleib(9, 0x20, 7);
    fpu_vsrlb(2, 1, 9);
    fpu_vupllf(1, 1);
    fpu_vgfmag(1, CONST_R5, 1, 2);

    fpu_vupllf(2, 1);
    fpu_vgfmg(2, CONST_RU_POLY, 2);
    fpu_vupllf(2, 2);
    fpu_vgfmag(2, CONST_CRC_POLY, 2, 1);

    fpu_vlgvf(2, 2)
}

unsafe fn crc32_le_vgfm_16(crc: u32, buf: *const u8, size: usize) -> u32 {
    crc32_le_vgfm_generic(crc, buf, size, constants_CRC_32_LE.as_mut_ptr())
}

unsafe fn crc32c_le_vgfm_16(crc: u32, buf: *const u8, size: usize) -> u32 {
    crc32_le_vgfm_generic(crc, buf, size, constants_CRC_32C_LE.as_mut_ptr())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
