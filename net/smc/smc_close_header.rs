/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Shared Memory Communications over RDMA (SMC-R) and RoCE
 *
 * Socket Closing
 *
 * Copyright IBM Corp. 2016
 *
 * Author(s):  Ursula Braun <ubraun@linux.vnet.ibm.com>
 */

// Dependency intent from <linux/workqueue.h> and "smc.h" is supplied by the
// surrounding translation unit.

pub const SMC_MAX_STREAM_WAIT_TIMEOUT: usize = 2 * HZ;
pub const SMC_CLOSE_SOCK_PUT_DELAY: usize = HZ;

// Opaque types declared by smc.h.
#[repr(C)]
pub struct smc_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct smc_connection {
    _private: [u8; 0],
}

extern "C" {
    pub fn smc_close_wake_tx_prepared(smc: *mut smc_sock);
    pub fn smc_close_active(smc: *mut smc_sock) -> i32;
    pub fn smc_close_shutdown_write(smc: *mut smc_sock) -> i32;
    pub fn smc_close_init(smc: *mut smc_sock);
    pub fn smc_clcsock_release(smc: *mut smc_sock);
    pub fn smc_close_abort(conn: *mut smc_connection) -> i32;
    pub fn smc_close_active_abort(smc: *mut smc_sock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
