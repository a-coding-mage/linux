/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Shared Memory Communications over RDMA (SMC-R) and RoCE
 *
 * Manage send buffer
 *
 * Copyright IBM Corp. 2016
 *
 * Author(s):  Ursula Braun <ubraun@linux.vnet.ibm.com>
 */

// Dependencies supplied by the corresponding kernel and SMC translation units:
// linux/socket.h, linux/types.h, smc.h, and smc_cdc.h.

#[inline]
pub unsafe fn smc_tx_prepared_sends(conn: *mut smc_connection) -> i32 {
    let sndbuf_desc = core::ptr::read_volatile(&(*conn).sndbuf_desc);
    let mut sent: smc_host_cursor = core::mem::MaybeUninit::uninit().assume_init();
    let mut prep: smc_host_cursor = core::mem::MaybeUninit::uninit().assume_init();

    if sndbuf_desc.is_null() {
        return 0;
    }

    smc_curs_copy(&mut sent, &(*conn).tx_curs_sent, conn);
    smc_curs_copy(&mut prep, &(*conn).tx_curs_prep, conn);
    smc_curs_diff((*sndbuf_desc).len, &sent, &prep)
}

extern "C" {
    pub fn smc_tx_pending(conn: *mut smc_connection);
    pub fn smc_tx_work(work: *mut work_struct);
    pub fn smc_tx_init(smc: *mut smc_sock);
    pub fn smc_tx_sendmsg(smc: *mut smc_sock, msg: *mut msghdr, len: usize) -> isize;
    pub fn smc_tx_sndbuf_nonempty(conn: *mut smc_connection) -> i32;
    pub fn smc_tx_sndbuf_nonfull(smc: *mut smc_sock);
    pub fn smc_tx_consumer_update(conn: *mut smc_connection, force: bool);
    pub fn smcd_tx_ism_write(
        conn: *mut smc_connection,
        data: *mut core::ffi::c_void,
        len: usize,
        offset: u32,
        signal: i32,
    ) -> i32;

    fn smc_curs_copy(
        dst: *mut smc_host_cursor,
        src: *const smc_host_cursor,
        conn: *mut smc_connection,
    );
    fn smc_curs_diff(
        len: u32,
        sent: *const smc_host_cursor,
        prep: *const smc_host_cursor,
    ) -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
