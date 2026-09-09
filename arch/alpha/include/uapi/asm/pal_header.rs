/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Common PAL-code
 */
pub const PAL_halt: i32 = 0;
pub const PAL_cflush: i32 = 1;
pub const PAL_draina: i32 = 2;
pub const PAL_bpt: i32 = 128;
pub const PAL_bugchk: i32 = 129;
pub const PAL_chmk: i32 = 131;
pub const PAL_callsys: i32 = 131;
pub const PAL_imb: i32 = 134;
pub const PAL_rduniq: i32 = 158;
pub const PAL_wruniq: i32 = 159;
pub const PAL_gentrap: i32 = 170;
pub const PAL_nphalt: i32 = 190;

/*
 * VMS specific PAL-code
 */
pub const PAL_swppal: i32 = 10;
pub const PAL_mfpr_vptb: i32 = 41;

/*
 * OSF specific PAL-code
 */
pub const PAL_cserve: i32 = 9;
pub const PAL_wripir: i32 = 13;
pub const PAL_rdmces: i32 = 16;
pub const PAL_wrmces: i32 = 17;
pub const PAL_wrfen: i32 = 43;
pub const PAL_wrvptptr: i32 = 45;
pub const PAL_jtopal: i32 = 46;
pub const PAL_swpctx: i32 = 48;
pub const PAL_wrval: i32 = 49;
pub const PAL_rdval: i32 = 50;
pub const PAL_tbi: i32 = 51;
pub const PAL_wrent: i32 = 52;
pub const PAL_swpipl: i32 = 53;
pub const PAL_rdps: i32 = 54;
pub const PAL_wrkgp: i32 = 55;
pub const PAL_wrusp: i32 = 56;
pub const PAL_wrperfmon: i32 = 57;
pub const PAL_rdusp: i32 = 58;
pub const PAL_whami: i32 = 60;
pub const PAL_retsys: i32 = 61;
pub const PAL_wtint: i32 = 62;
pub const PAL_rti: i32 = 63;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
