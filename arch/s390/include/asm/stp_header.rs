/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Copyright IBM Corp. 2006
 *  Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com)
 */

/* Dependency supplied by the surrounding kernel translation. */
extern "C" {
    pub static mut s390_epoch_delta_notifier: atomic_notifier_head;
}

/* notifier for syncs */

/* STP interruption parameter. C bit-fields are represented by their storage word. */
#[repr(C, packed)]
pub struct stp_irq_parm {
    pub bits: u32, /* tsc: 1, lac: 1, tcpc: 1; remaining bits reserved */
}

pub const STP_OP_SYNC: u32 = 1;
pub const STP_OP_CTRL: u32 = 3;

#[repr(C, packed)]
pub struct stp_sstpi {
    pub reserved0: u32,
    pub status: u32, /* tu: 1, lu: 1, reserved: 6, stratum: 8, vbits: 16 */
    pub leap_status: u32, /* leaps: 16, tmd: 4, ctn: 4, reserved: 3, c: 1, tst: 4 */
    pub timezone: u32, /* tzo: 16, dsto: 16 */
    pub ctrl: u32, /* ctrl: 16, reserved: 16 */
    pub tto: u32,
    pub reserved1: u32,
    pub ctnid: [u32; 3],
    pub reserved2: u32,
    pub todoff: u64,
    pub rsvd: [u32; 50],
}

#[repr(C, packed)]
pub struct stp_tzib {
    pub tzan: u32, /* tzan: 16, reserved: 16 */
    pub tzo_dsto: u32, /* tzo: 16, dsto: 16 */
    pub stn: u32,
    pub dstn: u32,
    pub dst_on_alg: u64,
    pub dst_off_alg: u64,
}

#[repr(C, packed)]
pub struct stp_tcpib {
    pub codes: u32, /* atcode: 4, ntcode: 4, d: 1, reserved: 23 */
    pub tto: i32,
    pub atzib: stp_tzib,
    pub ntzib: stp_tzib,
    pub dst_offsets: u32, /* adst_offset: 16, ndst_offset: 16 */
    pub rsvd1: u32,
    pub ntzib_update: u64,
    pub ndsto_update: u64,
}

#[repr(C, packed)]
pub struct stp_lsoib {
    pub flags: u32, /* p: 1, reserved: 31 */
    pub lso: u32, /* also: 16, nlso: 16 */
    pub nlsout: u64,
}

#[repr(C, packed)]
pub struct stp_stzi {
    pub rsvd0: [u32; 3],
    pub data_ts: u64,
    pub rsvd1: [u32; 22],
    pub tcpib: stp_tcpib,
    pub lsoib: stp_lsoib,
}

/* Functions needed by the machine check handler */
extern "C" {
    pub fn stp_sync_check() -> i32;
    pub fn stp_island_check() -> i32;
    pub fn stp_queue_work();
    pub fn stp_enabled() -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
