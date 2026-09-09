/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Alpha floating-point control register defines:
 */
pub const FPCR_DNOD: u64 = 1u64 << 47; /* denorm INV trap disable */
pub const FPCR_DNZ: u64 = 1u64 << 48; /* denorms to zero */
pub const FPCR_INVD: u64 = 1u64 << 49; /* invalid op disable (opt.) */
pub const FPCR_DZED: u64 = 1u64 << 50; /* division by zero disable (opt.) */
pub const FPCR_OVFD: u64 = 1u64 << 51; /* overflow disable (optional) */
pub const FPCR_INV: u64 = 1u64 << 52; /* invalid operation */
pub const FPCR_DZE: u64 = 1u64 << 53; /* division by zero */
pub const FPCR_OVF: u64 = 1u64 << 54; /* overflow */
pub const FPCR_UNF: u64 = 1u64 << 55; /* underflow */
pub const FPCR_INE: u64 = 1u64 << 56; /* inexact */
pub const FPCR_IOV: u64 = 1u64 << 57; /* integer overflow */
pub const FPCR_UNDZ: u64 = 1u64 << 60; /* underflow to zero (opt.) */
pub const FPCR_UNFD: u64 = 1u64 << 61; /* underflow disable (opt.) */
pub const FPCR_INED: u64 = 1u64 << 62; /* inexact disable (opt.) */
pub const FPCR_SUM: u64 = 1u64 << 63; /* summary bit */

pub const FPCR_DYN_SHIFT: u32 = 58; /* first dynamic rounding mode bit */
pub const FPCR_DYN_CHOPPED: u64 = 0x0u64 << FPCR_DYN_SHIFT; /* towards 0 */
pub const FPCR_DYN_MINUS: u64 = 0x1u64 << FPCR_DYN_SHIFT; /* towards -INF */
pub const FPCR_DYN_NORMAL: u64 = 0x2u64 << FPCR_DYN_SHIFT; /* towards nearest */
pub const FPCR_DYN_PLUS: u64 = 0x3u64 << FPCR_DYN_SHIFT; /* towards +INF */
pub const FPCR_DYN_MASK: u64 = 0x3u64 << FPCR_DYN_SHIFT;

pub const FPCR_MASK: u64 = 0xffff800000000000u64;

/*
 * IEEE trap enables are implemented in software.  These per-thread
 * bits are stored in the "ieee_state" field of "struct thread_info".
 * Thus, the bits are defined so as not to conflict with the
 * floating-point enable bit (which is architected).  On top of that,
 * we want to make these bits compatible with OSF/1 so
 * ieee_set_fp_control() etc. can be implemented easily and
 * compatibly.  The corresponding definitions are in
 * /usr/include/machine/fpu.h under OSF/1.
 */
pub const IEEE_TRAP_ENABLE_INV: u64 = 1u64 << 1; /* invalid op */
pub const IEEE_TRAP_ENABLE_DZE: u64 = 1u64 << 2; /* division by zero */
pub const IEEE_TRAP_ENABLE_OVF: u64 = 1u64 << 3; /* overflow */
pub const IEEE_TRAP_ENABLE_UNF: u64 = 1u64 << 4; /* underflow */
pub const IEEE_TRAP_ENABLE_INE: u64 = 1u64 << 5; /* inexact */
pub const IEEE_TRAP_ENABLE_DNO: u64 = 1u64 << 6; /* denorm */
pub const IEEE_TRAP_ENABLE_MASK: u64 = IEEE_TRAP_ENABLE_INV
    | IEEE_TRAP_ENABLE_DZE
    | IEEE_TRAP_ENABLE_OVF
    | IEEE_TRAP_ENABLE_UNF
    | IEEE_TRAP_ENABLE_INE
    | IEEE_TRAP_ENABLE_DNO;

/* Denorm and Underflow flushing */
pub const IEEE_MAP_DMZ: u64 = 1u64 << 12; /* Map denorm inputs to zero */
pub const IEEE_MAP_UMZ: u64 = 1u64 << 13; /* Map underflowed outputs to zero */

pub const IEEE_MAP_MASK: u64 = IEEE_MAP_DMZ | IEEE_MAP_UMZ;

/* status bits coming from fpcr: */
pub const IEEE_STATUS_INV: u64 = 1u64 << 17;
pub const IEEE_STATUS_DZE: u64 = 1u64 << 18;
pub const IEEE_STATUS_OVF: u64 = 1u64 << 19;
pub const IEEE_STATUS_UNF: u64 = 1u64 << 20;
pub const IEEE_STATUS_INE: u64 = 1u64 << 21;
pub const IEEE_STATUS_DNO: u64 = 1u64 << 22;

pub const IEEE_STATUS_MASK: u64 = IEEE_STATUS_INV
    | IEEE_STATUS_DZE
    | IEEE_STATUS_OVF
    | IEEE_STATUS_UNF
    | IEEE_STATUS_INE
    | IEEE_STATUS_DNO;

pub const IEEE_SW_MASK: u64 = IEEE_TRAP_ENABLE_MASK | IEEE_STATUS_MASK | IEEE_MAP_MASK;

pub const IEEE_CURRENT_RM_SHIFT: u32 = 32;
pub const IEEE_CURRENT_RM_MASK: u64 = 3u64 << IEEE_CURRENT_RM_SHIFT;

pub const IEEE_STATUS_TO_EXCSUM_SHIFT: u32 = 16;

pub const IEEE_INHERIT: u64 = 1u64 << 63; /* inherit on thread create? */

/*
 * Convert the software IEEE trap enable and status bits into the
 * hardware fpcr format.
 *
 * Digital Unix engineers receive my thanks for not defining the
 * software bits identical to the hardware bits.  The chip designers
 * receive my thanks for making all the not-implemented fpcr bits
 * RAZ forcing us to use system calls to read/write this value.
 */
#[inline]
pub fn ieee_swcr_to_fpcr(sw: u64) -> u64 {
    let mut fp: u64;
    fp = (sw & IEEE_STATUS_MASK) << 35;
    fp |= (sw & IEEE_MAP_DMZ) << 36;
    fp |= if (sw & IEEE_STATUS_MASK) != 0 { FPCR_SUM } else { 0 };
    fp |= (!sw & (IEEE_TRAP_ENABLE_INV | IEEE_TRAP_ENABLE_DZE | IEEE_TRAP_ENABLE_OVF)) << 48;
    fp |= (!sw & (IEEE_TRAP_ENABLE_UNF | IEEE_TRAP_ENABLE_INE)) << 57;
    fp |= if (sw & IEEE_MAP_UMZ) != 0 { FPCR_UNDZ | FPCR_UNFD } else { 0 };
    /*
     * Disable denormal operand traps only when denormal inputs are to be
     * flushed to zero.  Otherwise they must keep trapping, so that /S
     * instructions reach the kernel emulation handler.
     */
    fp |= if (sw & IEEE_MAP_DMZ) != 0 { FPCR_DNOD } else { 0 };
    fp
}

#[inline]
pub fn ieee_fpcr_to_swcr(fp: u64) -> u64 {
    let mut sw: u64;
    sw = (fp >> 35) & IEEE_STATUS_MASK;
    sw |= (fp >> 36) & IEEE_MAP_DMZ;
    sw |= (!fp >> 48) & (IEEE_TRAP_ENABLE_INV | IEEE_TRAP_ENABLE_DZE | IEEE_TRAP_ENABLE_OVF);
    sw |= (!fp >> 57) & (IEEE_TRAP_ENABLE_UNF | IEEE_TRAP_ENABLE_INE);
    sw |= (fp >> 47) & IEEE_MAP_UMZ;
    sw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
