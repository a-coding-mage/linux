/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  include/linux/signalfd.h
 *
 *  Copyright (C) 2007  Davide Libenzi <davidel@xmailserver.org>
 *
 */

// Dependency intent: __u8, __u16, __u32, __s32, and __u64 are supplied by
// linux/types.h; O_CLOEXEC and O_NONBLOCK are supplied by linux/fcntl.h.

/* Flags for signalfd4.  */
pub const SFD_CLOEXEC: _ = O_CLOEXEC;
pub const SFD_NONBLOCK: _ = O_NONBLOCK;

#[repr(C)]
pub struct signalfd_siginfo {
	pub ssi_signo: __u32,
	pub ssi_errno: __s32,
	pub ssi_code: __s32,
	pub ssi_pid: __u32,
	pub ssi_uid: __u32,
	pub ssi_fd: __s32,
	pub ssi_tid: __u32,
	pub ssi_band: __u32,
	pub ssi_overrun: __u32,
	pub ssi_trapno: __u32,
	pub ssi_status: __s32,
	pub ssi_int: __s32,
	pub ssi_ptr: __u64,
	pub ssi_utime: __u64,
	pub ssi_stime: __u64,
	pub ssi_addr: __u64,
	pub ssi_addr_lsb: __u16,
	pub __pad2: __u16,
	pub ssi_syscall: __s32,
	pub ssi_call_addr: __u64,
	pub ssi_arch: __u32,

	/*
	 * Pad strcture to 128 bytes. Remember to update the
	 * pad size when you add new members. We use a fixed
	 * size structure to avoid compatibility problems with
	 * future versions, and we leave extra space for additional
	 * members. We use fixed size members because this strcture
	 * comes out of a read(2) and we really don't want to have
	 * a compat on read(2).
	 */
	pub __pad: [__u8; 28],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
