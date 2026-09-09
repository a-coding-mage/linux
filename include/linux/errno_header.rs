/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: declarations from <uapi/linux/errno.h> are supplied externally.

/*
 * These should never be seen by user programs.  To return one of ERESTART*
 * codes, signal_pending() MUST be set.  Note that ptrace can observe these
 * at syscall exit tracing, but they will never be left for the debugged user
 * process to see.
 */
pub const ERESTARTSYS: i32 = 512;
pub const ERESTARTNOINTR: i32 = 513;
pub const ERESTARTNOHAND: i32 = 514; /* restart if no handler.. */
pub const ENOIOCTLCMD: i32 = 515; /* No ioctl command */
pub const ERESTART_RESTARTBLOCK: i32 = 516; /* restart by calling sys_restart_syscall */
pub const EPROBE_DEFER: i32 = 517; /* Driver requests probe retry */
pub const EOPENSTALE: i32 = 518; /* open found a stale dentry */
pub const ENOPARAM: i32 = 519; /* Parameter not supported */

/* Defined for the NFSv3 protocol */
pub const EBADHANDLE: i32 = 521; /* Illegal NFS file handle */
pub const ENOTSYNC: i32 = 522; /* Update synchronization mismatch */
pub const EBADCOOKIE: i32 = 523; /* Cookie is stale */
pub const ENOTSUPP: i32 = 524; /* Operation is not supported */
pub const ETOOSMALL: i32 = 525; /* Buffer or request is too small */
pub const ESERVERFAULT: i32 = 526; /* An untranslatable error occurred */
pub const EBADTYPE: i32 = 527; /* Type not supported by server */
pub const EJUKEBOX: i32 = 528; /* Request initiated, but will not complete before timeout */
pub const EIOCBQUEUED: i32 = 529; /* iocb queued, will get completion event */
pub const ERECALLCONFLICT: i32 = 530; /* conflict with recalled state */
pub const ENOGRACE: i32 = 531; /* NFS file lock reclaim refused */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
