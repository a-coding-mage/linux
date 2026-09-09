/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by linux/ipc.h.

/* semop flags */
pub const SEM_UNDO: i32 = 0x1000; /* undo the operation on exit */

/* semctl Command Definitions. */
pub const GETPID: i32 = 11; /* get sempid */
pub const GETVAL: i32 = 12; /* get semval */
pub const GETALL: i32 = 13; /* get all semval's */
pub const GETNCNT: i32 = 14; /* get semncnt */
pub const GETZCNT: i32 = 15; /* get semzcnt */
pub const SETVAL: i32 = 16; /* set semval */
pub const SETALL: i32 = 17; /* set all semval's */

/* ipcs ctl cmds */
pub const SEM_STAT: i32 = 18;
pub const SEM_INFO: i32 = 19;
pub const SEM_STAT_ANY: i32 = 20;

/* Obsolete, used only for backwards compatibility and libc5 compiles */
#[repr(C)]
pub struct semid_ds {
    pub sem_perm: ipc_perm,                         /* permissions .. see ipc.h */
    pub sem_otime: __kernel_old_time_t,             /* last semop time */
    pub sem_ctime: __kernel_old_time_t,             /* create/last semctl() time */
    pub sem_base: *mut sem,                         /* ptr to first semaphore in array */
    pub sem_pending: *mut sem_queue,                /* pending operations to be processed */
    pub sem_pending_last: *mut *mut sem_queue,      /* last pending operation */
    pub undo: *mut sem_undo,                        /* undo requests on this array */
    pub sem_nsems: u16,                             /* no. of semaphores in array */
}

// Include the definition of semid64_ds from asm/sembuf.h.

/* semop system calls takes an array of these. */
#[repr(C)]
pub struct sembuf {
    pub sem_num: u16, /* semaphore index in array */
    pub sem_op: i16,  /* semaphore operation */
    pub sem_flg: i16, /* operation flags */
}

/* arg for semctl system calls. */
#[repr(C)]
pub union semun {
    pub val: i32,                         /* value for SETVAL */
    pub buf: *mut semid_ds,               /* buffer for IPC_STAT & IPC_SET */
    pub array: *mut u16,                  /* array for GETALL & SETALL */
    pub __buf: *mut seminfo,              /* buffer for IPC_INFO */
    pub __pad: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct seminfo {
    pub semmap: i32,
    pub semmni: i32,
    pub semmns: i32,
    pub semmnu: i32,
    pub semmsl: i32,
    pub semopm: i32,
    pub semume: i32,
    pub semusz: i32,
    pub semvmx: i32,
    pub semaem: i32,
}

/*
 * SEMMNI, SEMMSL and SEMMNS are default values which can be
 * modified by sysctl.
 * The values has been chosen to be larger than necessary for any
 * known configuration.
 *
 * SEMOPM should not be increased beyond 1000, otherwise there is the
 * risk that semop()/semtimedop() fails due to kernel memory fragmentation when
 * allocating the sop array.
 */

pub const SEMMNI: i32 = 32000; /* <= IPCMNI  max # of semaphore identifiers */
pub const SEMMSL: i32 = 32000; /* <= INT_MAX max num of semaphores per id */
pub const SEMMNS: i32 = SEMMNI * SEMMSL; /* <= INT_MAX max # of semaphores in system */
pub const SEMOPM: i32 = 500; /* <= 1 000 max num of ops per semop call */
pub const SEMVMX: i32 = 32767; /* <= 32767 semaphore maximum value */
pub const SEMAEM: i32 = SEMVMX; /* adjust on exit max value */

/* unused */
pub const SEMUME: i32 = SEMOPM; /* max num of undo entries per process */
pub const SEMMNU: i32 = SEMMNS; /* num of undo structures system wide */
pub const SEMMAP: i32 = SEMMNS; /* # of entries in semaphore map */
pub const SEMUSZ: i32 = 20; /* sizeof struct sem_undo */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
