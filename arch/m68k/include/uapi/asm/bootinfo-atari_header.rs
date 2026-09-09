/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
** asm/bootinfo-atari.h -- Atari-specific boot information definitions
*/

/*
 * Atari-specific tags
 */
pub const BI_ATARI_MCH_COOKIE: u16 = 0x8000; /* _MCH cookie from TOS (__be32) */
pub const BI_ATARI_MCH_TYPE: u16 = 0x8001; /* special machine type (__be32) */

/*
 * mch_cookie values (upper word of BI_ATARI_MCH_COOKIE)
 */
pub const ATARI_MCH_ST: u32 = 0;
pub const ATARI_MCH_STE: u32 = 1;
pub const ATARI_MCH_TT: u32 = 2;
pub const ATARI_MCH_FALCON: u32 = 3;

/*
 * Atari machine types (BI_ATARI_MCH_TYPE)
 */
pub const ATARI_MACH_NORMAL: u32 = 0; /* no special machine type */
pub const ATARI_MACH_MEDUSA: u32 = 1; /* Medusa 040 */
pub const ATARI_MACH_HADES: u32 = 2; /* Hades 040 or 060 */
pub const ATARI_MACH_AB40: u32 = 3; /* Afterburner040 on Falcon */

/*
 * Latest Atari bootinfo version
 *
 * MK_BI_VERSION is supplied by the bootinfo definitions dependency.
 */
pub const ATARI_BOOTI_VERSION: u32 = MK_BI_VERSION!(2, 1);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
