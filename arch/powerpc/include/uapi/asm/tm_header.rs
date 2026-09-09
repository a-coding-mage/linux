/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Reason codes describing kernel causes for transaction aborts.  By
 * convention, bit0 is copied to TEXASR[56] (IBM bit 7) which is set if
 * the failure is persistent.  PAPR saves 0xff-0xe0 for the hypervisor.
 */
pub const TM_CAUSE_PERSISTENT: u32 = 0x01;
pub const TM_CAUSE_KVM_RESCHED: u32 = 0xe0; /* From PAPR */
pub const TM_CAUSE_KVM_FAC_UNAV: u32 = 0xe2; /* From PAPR */
pub const TM_CAUSE_RESCHED: u32 = 0xde;
pub const TM_CAUSE_TLBI: u32 = 0xdc;
pub const TM_CAUSE_FAC_UNAV: u32 = 0xda;
pub const TM_CAUSE_SYSCALL: u32 = 0xd8;
pub const TM_CAUSE_MISC: u32 = 0xd6; /* future use */
pub const TM_CAUSE_SIGNAL: u32 = 0xd4;
pub const TM_CAUSE_ALIGNMENT: u32 = 0xd2;
pub const TM_CAUSE_EMULATE: u32 = 0xd0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
