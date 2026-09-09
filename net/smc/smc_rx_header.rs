/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Shared Memory Communications over RDMA (SMC-R) and RoCE
 *
 * Manage RMBE
 *
 * Copyright IBM Corp. 2016
 *
 * Author(s):  Ursula Braun <ubraun@linux.vnet.ibm.com>
 */

// C dependencies:
// #include <linux/socket.h>
// #include <linux/types.h>
// #include "smc.h"

extern "C" {
    pub fn smc_rx_init(smc: *mut smc_sock);

    pub fn smc_rx_recvmsg(
        smc: *mut smc_sock,
        msg: *mut msghdr,
        pipe: *mut pipe_inode_info,
        len: usize,
        flags: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub fn smc_rx_wait(
        smc: *mut smc_sock,
        timeo: *mut core::ffi::c_long,
        peeked: usize,
        fcrit: Option<unsafe extern "C" fn(*mut smc_connection, usize) -> core::ffi::c_int>,
    ) -> core::ffi::c_int;
}

/// Returns the number of received bytes available beyond `peeked`.
#[inline]
pub unsafe fn smc_rx_data_available(conn: *mut smc_connection, peeked: usize) -> core::ffi::c_int {
    atomic_read(&(*conn).bytes_to_rcv) - peeked as core::ffi::c_int
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
