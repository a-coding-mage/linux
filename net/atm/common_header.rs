/* SPDX-License-Identifier: GPL-2.0 */
/* net/atm/common.h - ATM sockets (common part for PVC and SVC) */

/* Written 1995-2000 by Werner Almesberger, EPFL LRC/ICA */

/* C includes: linux/net.h; linux/poll.h (for poll_table). */

extern "C" {
    pub fn vcc_create(
        net: *mut net,
        sock: *mut socket,
        protocol: ::core::ffi::c_int,
        family: ::core::ffi::c_int,
        kern: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn vcc_release(sock: *mut socket) -> ::core::ffi::c_int;
    pub fn vcc_connect(
        sock: *mut socket,
        itf: ::core::ffi::c_int,
        vpi: ::core::ffi::c_short,
        vci: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn vcc_recvmsg(
        sock: *mut socket,
        msg: *mut msghdr,
        size: usize,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn vcc_sendmsg(
        sock: *mut socket,
        m: *mut msghdr,
        total_len: usize,
    ) -> ::core::ffi::c_int;
    pub fn vcc_poll(
        file: *mut file,
        sock: *mut socket,
        wait: *mut poll_table,
    ) -> __poll_t;
    pub fn vcc_ioctl(
        sock: *mut socket,
        cmd: ::core::ffi::c_uint,
        arg: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    pub fn vcc_compat_ioctl(
        sock: *mut socket,
        cmd: ::core::ffi::c_uint,
        arg: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    pub fn vcc_setsockopt(
        sock: *mut socket,
        level: ::core::ffi::c_int,
        optname: ::core::ffi::c_int,
        optval: sockptr_t,
        optlen: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn vcc_getsockopt(
        sock: *mut socket,
        level: ::core::ffi::c_int,
        optname: ::core::ffi::c_int,
        opt: *mut sockopt_t,
    ) -> ::core::ffi::c_int;
    pub fn vcc_process_recv_queue(vcc: *mut atm_vcc);

    pub fn atmpvc_init() -> ::core::ffi::c_int;
    pub fn atmpvc_exit();
    pub fn atm_sysfs_init() -> ::core::ffi::c_int;
    pub fn atm_sysfs_exit();

    #[cfg(CONFIG_PROC_FS)]
    pub fn atm_proc_init() -> ::core::ffi::c_int;
    #[cfg(CONFIG_PROC_FS)]
    pub fn atm_proc_exit();

    pub fn atm_dev_release_vccs(dev: *mut atm_dev);
}

/* When CONFIG_PROC_FS is disabled, the C header provides these inline stubs. */
#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub fn atm_proc_init() -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_PROC_FS))]
#[inline]
pub fn atm_proc_exit() {
    /* nothing */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
