/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: definitions from <linux/types.h> are supplied externally.

pub const IPC_PRIVATE: __kernel_key_t = 0 as __kernel_key_t;

/* Obsolete, used only for backwards compatibility and libc5 compiles */
#[repr(C)]
pub struct ipc_perm {
    pub key: __kernel_key_t,
    pub uid: __kernel_uid_t,
    pub gid: __kernel_gid_t,
    pub cuid: __kernel_uid_t,
    pub cgid: __kernel_gid_t,
    pub mode: __kernel_mode_t,
    pub seq: u16,
}

// Dependency: the definition of ipc64_perm from <asm/ipcbuf.h> is supplied externally.

/* resource get request flags */
pub const IPC_CREAT: u32 = 0o001000;
pub const IPC_EXCL: u32 = 0o002000;
pub const IPC_NOWAIT: u32 = 0o004000;

/* these fields are used by the DIPC package so the kernel as standard
   should avoid using them if possible */
pub const IPC_DIPC: u32 = 0o010000;
pub const IPC_OWN: u32 = 0o020000;

/*
 * Control commands used with semctl, msgctl and shmctl
 * see also specific commands in sem.h, msg.h and shm.h
 */
pub const IPC_RMID: u32 = 0;
pub const IPC_SET: u32 = 1;
pub const IPC_STAT: u32 = 2;
pub const IPC_INFO: u32 = 3;

/*
 * Version flags for semctl, msgctl, and shmctl commands
 * These are passed as bitflags or-ed with the actual command
 */
pub const IPC_OLD: u32 = 0;
pub const IPC_64: u32 = 0x0100;

/*
 * These are used to wrap system calls.
 *
 * See architecture code for ugly details..
 */
#[repr(C)]
pub struct ipc_kludge {
    pub msgp: *mut msgbuf,
    pub msgtyp: ::core::ffi::c_long,
}

pub const SEMOP: u32 = 1;
pub const SEMGET: u32 = 2;
pub const SEMCTL: u32 = 3;
pub const SEMTIMEDOP: u32 = 4;
pub const MSGSND: u32 = 11;
pub const MSGRCV: u32 = 12;
pub const MSGGET: u32 = 13;
pub const MSGCTL: u32 = 14;
pub const SHMAT: u32 = 21;
pub const SHMDT: u32 = 22;
pub const SHMGET: u32 = 23;
pub const SHMCTL: u32 = 24;

/* Used by the DIPC package, try and avoid reusing it */
pub const DIPC: u32 = 25;

#[inline]
pub const fn IPCCALL(version: u32, op: u32) -> u32 {
    (version << 16) | op
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
