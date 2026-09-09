/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the SPARC64 PSTATE header. The original _AC(..., UL)
// constants are represented as u64 values here.

/* The V9 PSTATE Register (with SpitFire extensions).
 *
 * -----------------------------------------------------------------------
 * | Resv | IG | MG | CLE | TLE |  MM  | RED | PEF | AM | PRIV | IE | AG |
 * -----------------------------------------------------------------------
 *  63  12  11   10    9     8    7   6   5     4     3     2     1    0
 */
/* IG on V9 conflicts with MCDE on M7. PSTATE_MCDE will only be used on
 * processors that support ADI which do not use IG, hence there is no
 * functional conflict
 */
pub const PSTATE_IG: u64 = 0x0000000000000800; /* Interrupt Globals. */
pub const PSTATE_MCDE: u64 = 0x0000000000000800; /* MCD Enable */
pub const PSTATE_MG: u64 = 0x0000000000000400; /* MMU Globals. */
pub const PSTATE_CLE: u64 = 0x0000000000000200; /* Current Little Endian. */
pub const PSTATE_TLE: u64 = 0x0000000000000100; /* Trap Little Endian. */
pub const PSTATE_MM: u64 = 0x00000000000000c0; /* Memory Model. */
pub const PSTATE_TSO: u64 = 0x0000000000000000; /* MM: TotalStoreOrder */
pub const PSTATE_PSO: u64 = 0x0000000000000040; /* MM: PartialStoreOrder */
pub const PSTATE_RMO: u64 = 0x0000000000000080; /* MM: RelaxedMemoryOrder */
pub const PSTATE_RED: u64 = 0x0000000000000020; /* Reset Error Debug. */
pub const PSTATE_PEF: u64 = 0x0000000000000010; /* Floating Point Enable. */
pub const PSTATE_AM: u64 = 0x0000000000000008; /* Address Mask. */
pub const PSTATE_PRIV: u64 = 0x0000000000000004; /* Privilege. */
pub const PSTATE_IE: u64 = 0x0000000000000002; /* Interrupt Enable. */
pub const PSTATE_AG: u64 = 0x0000000000000001; /* Alternate Globals. */

/* The V9 TSTATE Register (with SpitFire and Linux extensions). */
pub const TSTATE_GL: u64 = 0x0000070000000000;
pub const TSTATE_CCR: u64 = 0x000000ff00000000;
pub const TSTATE_XCC: u64 = 0x000000f000000000;
pub const TSTATE_XNEG: u64 = 0x0000008000000000;
pub const TSTATE_XZERO: u64 = 0x0000004000000000;
pub const TSTATE_XOVFL: u64 = 0x0000002000000000;
pub const TSTATE_XCARRY: u64 = 0x0000001000000000;
pub const TSTATE_ICC: u64 = 0x0000000f00000000;
pub const TSTATE_INEG: u64 = 0x0000000800000000;
pub const TSTATE_IZERO: u64 = 0x0000000400000000;
pub const TSTATE_IOVFL: u64 = 0x0000000200000000;
pub const TSTATE_ICARRY: u64 = 0x0000000100000000;
pub const TSTATE_ASI: u64 = 0x00000000ff000000;
pub const TSTATE_PIL: u64 = 0x0000000000f00000;
pub const TSTATE_PSTATE: u64 = 0x00000000000fff00;
pub const TSTATE_IG: u64 = 0x0000000000080000;
pub const TSTATE_MCDE: u64 = 0x0000000000080000;
pub const TSTATE_MG: u64 = 0x0000000000040000;
pub const TSTATE_CLE: u64 = 0x0000000000020000;
pub const TSTATE_TLE: u64 = 0x0000000000010000;
pub const TSTATE_MM: u64 = 0x000000000000c000;
pub const TSTATE_TSO: u64 = 0x0000000000000000;
pub const TSTATE_PSO: u64 = 0x0000000000004000;
pub const TSTATE_RMO: u64 = 0x0000000000008000;
pub const TSTATE_RED: u64 = 0x0000000000002000;
pub const TSTATE_PEF: u64 = 0x0000000000001000;
pub const TSTATE_AM: u64 = 0x0000000000000800;
pub const TSTATE_PRIV: u64 = 0x0000000000000400;
pub const TSTATE_IE: u64 = 0x0000000000000200;
pub const TSTATE_AG: u64 = 0x0000000000000100;
pub const TSTATE_SYSCALL: u64 = 0x0000000000000020;
pub const TSTATE_CWP: u64 = 0x000000000000001f;

/* Floating-Point Registers State Register. */
pub const FPRS_FEF: u64 = 0x0000000000000004;
pub const FPRS_DU: u64 = 0x0000000000000002;
pub const FPRS_DL: u64 = 0x0000000000000001;

/* Version Register. */
pub const VERS_MANUF: u64 = 0xffff000000000000;
pub const VERS_IMPL: u64 = 0x0000ffff00000000;
pub const VERS_MASK: u64 = 0x00000000ff000000;
pub const VERS_MAXTL: u64 = 0x000000000000ff00;
pub const VERS_MAXWIN: u64 = 0x000000000000001f;

/* Compatibility Feature Register (%asr26), SPARC-T4 and later */
pub const CFR_AES: u64 = 0x0000000000000001;
pub const CFR_DES: u64 = 0x0000000000000002;
pub const CFR_KASUMI: u64 = 0x0000000000000004;
pub const CFR_CAMELLIA: u64 = 0x0000000000000008;
pub const CFR_MD5: u64 = 0x0000000000000010;
pub const CFR_SHA1: u64 = 0x0000000000000020;
pub const CFR_SHA256: u64 = 0x0000000000000040;
pub const CFR_SHA512: u64 = 0x0000000000000080;
pub const CFR_MPMUL: u64 = 0x0000000000000100;
pub const CFR_MONTMUL: u64 = 0x0000000000000200;
pub const CFR_MONTSQR: u64 = 0x0000000000000400;
pub const CFR_CRC32C: u64 = 0x0000000000000800;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
