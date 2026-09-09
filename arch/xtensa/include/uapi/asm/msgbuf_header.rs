/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * include/asm-xtensa/msgbuf.h
 *
 * The msqid64_ds structure for the Xtensa architecture.
 * Note extra padding because this structure is passed back and forth
 * between kernel and user space.
 *
 * Pad space is left for:
 * - 2 miscellaneous 32-bit values
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file "COPYING" in the main directory of
 * this archive for more details.
 */

// Dependency supplied by the corresponding IPC header.
use crate::ipcbuf_header::ipc64_perm;

#[repr(C)]
pub struct msqid64_ds {
    pub msg_perm: ipc64_perm,

    #[cfg(target_endian = "big")]
    pub msg_stime_high: ::core::ffi::c_ulong,
    #[cfg(target_endian = "big")]
    pub msg_stime: ::core::ffi::c_ulong, // last msgsnd time
    #[cfg(target_endian = "big")]
    pub msg_rtime_high: ::core::ffi::c_ulong,
    #[cfg(target_endian = "big")]
    pub msg_rtime: ::core::ffi::c_ulong, // last msgrcv time
    #[cfg(target_endian = "big")]
    pub msg_ctime_high: ::core::ffi::c_ulong,
    #[cfg(target_endian = "big")]
    pub msg_ctime: ::core::ffi::c_ulong, // last change time

    #[cfg(target_endian = "little")]
    pub msg_stime: ::core::ffi::c_ulong, // last msgsnd time
    #[cfg(target_endian = "little")]
    pub msg_stime_high: ::core::ffi::c_ulong,
    #[cfg(target_endian = "little")]
    pub msg_rtime: ::core::ffi::c_ulong, // last msgrcv time
    #[cfg(target_endian = "little")]
    pub msg_rtime_high: ::core::ffi::c_ulong,
    #[cfg(target_endian = "little")]
    pub msg_ctime: ::core::ffi::c_ulong, // last change time
    #[cfg(target_endian = "little")]
    pub msg_ctime_high: ::core::ffi::c_ulong,

    pub msg_cbytes: ::core::ffi::c_ulong, // current number of bytes on queue
    pub msg_qnum: ::core::ffi::c_ulong,   // number of messages in queue
    pub msg_qbytes: ::core::ffi::c_ulong, // max number of bytes on queue
    pub msg_lspid: __kernel_pid_t,        // pid of last msgsnd
    pub msg_lrpid: __kernel_pid_t,        // last receive pid
    pub __unused4: ::core::ffi::c_ulong,
    pub __unused5: ::core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
