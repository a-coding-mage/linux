/* SPDX-License-Identifier: GPL-2.0 */
/*
 * arch/arm/include/asm/vfp.h
 *
 * VFP register definitions.
 * First, the standard VFP set.
 */

/* FPSID bits */
pub const FPSID_IMPLEMENTER_BIT: u32 = 24;
pub const FPSID_IMPLEMENTER_MASK: u32 = 0xff << FPSID_IMPLEMENTER_BIT;
pub const FPSID_SOFTWARE: u32 = 1 << 23;
pub const FPSID_FORMAT_BIT: u32 = 21;
pub const FPSID_FORMAT_MASK: u32 = 0x3 << FPSID_FORMAT_BIT;
pub const FPSID_NODOUBLE: u32 = 1 << 20;
pub const FPSID_ARCH_BIT: u32 = 16;
pub const FPSID_ARCH_MASK: u32 = 0xF << FPSID_ARCH_BIT;
pub const FPSID_CPUID_ARCH_MASK: u32 = 0x7F << FPSID_ARCH_BIT;
pub const FPSID_PART_BIT: u32 = 8;
pub const FPSID_PART_MASK: u32 = 0xFF << FPSID_PART_BIT;
pub const FPSID_VARIANT_BIT: u32 = 4;
pub const FPSID_VARIANT_MASK: u32 = 0xF << FPSID_VARIANT_BIT;
pub const FPSID_REV_BIT: u32 = 0;
pub const FPSID_REV_MASK: u32 = 0xF << FPSID_REV_BIT;

/* FPEXC bits */
pub const FPEXC_EX: u32 = 1 << 31;
pub const FPEXC_EN: u32 = 1 << 30;
pub const FPEXC_DEX: u32 = 1 << 29;
pub const FPEXC_FP2V: u32 = 1 << 28;
pub const FPEXC_VV: u32 = 1 << 27;
pub const FPEXC_TFV: u32 = 1 << 26;
pub const FPEXC_LENGTH_BIT: u32 = 8;
pub const FPEXC_LENGTH_MASK: u32 = 7 << FPEXC_LENGTH_BIT;
pub const FPEXC_IDF: u32 = 1 << 7;
pub const FPEXC_IXF: u32 = 1 << 4;
pub const FPEXC_UFF: u32 = 1 << 3;
pub const FPEXC_OFF: u32 = 1 << 2;
pub const FPEXC_DZF: u32 = 1 << 1;
pub const FPEXC_IOF: u32 = 1 << 0;
pub const FPEXC_TRAP_MASK: u32 = FPEXC_IDF | FPEXC_IXF | FPEXC_UFF | FPEXC_OFF | FPEXC_DZF | FPEXC_IOF;

/* FPSCR bits */
pub const FPSCR_DEFAULT_NAN: u32 = 1 << 25;
pub const FPSCR_FLUSHTOZERO: u32 = 1 << 24;
pub const FPSCR_ROUND_NEAREST: u32 = 0 << 22;
pub const FPSCR_ROUND_PLUSINF: u32 = 1 << 22;
pub const FPSCR_ROUND_MINUSINF: u32 = 2 << 22;
pub const FPSCR_ROUND_TOZERO: u32 = 3 << 22;
pub const FPSCR_RMODE_BIT: u32 = 22;
pub const FPSCR_RMODE_MASK: u32 = 3 << FPSCR_RMODE_BIT;
pub const FPSCR_STRIDE_BIT: u32 = 20;
pub const FPSCR_STRIDE_MASK: u32 = 3 << FPSCR_STRIDE_BIT;
pub const FPSCR_LENGTH_BIT: u32 = 16;
pub const FPSCR_LENGTH_MASK: u32 = 7 << FPSCR_LENGTH_BIT;
pub const FPSCR_IOE: u32 = 1 << 8;
pub const FPSCR_DZE: u32 = 1 << 9;
pub const FPSCR_OFE: u32 = 1 << 10;
pub const FPSCR_UFE: u32 = 1 << 11;
pub const FPSCR_IXE: u32 = 1 << 12;
pub const FPSCR_IDE: u32 = 1 << 15;
pub const FPSCR_IOC: u32 = 1 << 0;
pub const FPSCR_DZC: u32 = 1 << 1;
pub const FPSCR_OFC: u32 = 1 << 2;
pub const FPSCR_UFC: u32 = 1 << 3;
pub const FPSCR_IXC: u32 = 1 << 4;
pub const FPSCR_IDC: u32 = 1 << 7;

/* MVFR0 bits */
pub const MVFR0_A_SIMD_BIT: u32 = 0;
pub const MVFR0_A_SIMD_MASK: u32 = 0xf << MVFR0_A_SIMD_BIT;
pub const MVFR0_SP_BIT: u32 = 4;
pub const MVFR0_SP_MASK: u32 = 0xf << MVFR0_SP_BIT;
pub const MVFR0_DP_BIT: u32 = 8;
pub const MVFR0_DP_MASK: u32 = 0xf << MVFR0_DP_BIT;

/* MVFR1 bits */
pub const MVFR1_ASIMDHP_BIT: u32 = 20;
pub const MVFR1_ASIMDHP_MASK: u32 = 0xf << MVFR1_ASIMDHP_BIT;
pub const MVFR1_FPHP_BIT: u32 = 24;
pub const MVFR1_FPHP_MASK: u32 = 0xf << MVFR1_FPHP_BIT;

/* Bit patterns for decoding the packaged operation descriptors */
pub const VFPOPDESC_LENGTH_BIT: u32 = 9;
pub const VFPOPDESC_LENGTH_MASK: u32 = 0x07 << VFPOPDESC_LENGTH_BIT;
pub const VFPOPDESC_UNUSED_BIT: u32 = 24;
pub const VFPOPDESC_UNUSED_MASK: u32 = 0xFF << VFPOPDESC_UNUSED_BIT;
pub const VFPOPDESC_OPDESC_MASK: u32 = !(VFPOPDESC_LENGTH_MASK | VFPOPDESC_UNUSED_MASK);

unsafe extern "C" {
    pub fn vfp_disable();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
