/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding Linux translation:
// linux/types.h, asm/page.h, and asm/shmparam.h.

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[cfg(CONFIG_SYSVIPC)]
#[repr(C)]
pub struct sysv_shm {
    pub shm_clist: list_head,
}

#[cfg(CONFIG_SYSVIPC)]
unsafe extern "C" {
    pub fn do_shmat(
        shmid: c_int,
        shmaddr: *mut c_char,
        shmflg: c_int,
        addr: *mut c_ulong,
        shmlba: c_ulong,
    ) -> c_long;

    pub fn exit_shm(task: *mut task_struct);
}

#[cfg(CONFIG_SYSVIPC)]
#[macro_export]
macro_rules! shm_init_task {
    ($task:expr) => {
        INIT_LIST_HEAD(unsafe { &mut (*$task).sysvshm.shm_clist })
    };
}

#[cfg(not(CONFIG_SYSVIPC))]
#[repr(C)]
pub struct sysv_shm {
    /* empty */
}

#[cfg(not(CONFIG_SYSVIPC))]
#[inline]
pub unsafe fn do_shmat(
    _shmid: c_int,
    _shmaddr: *mut c_char,
    _shmflg: c_int,
    _addr: *mut c_ulong,
    _shmlba: c_ulong,
) -> c_long {
    -(ENOSYS as c_long)
}

#[cfg(not(CONFIG_SYSVIPC))]
#[inline]
pub unsafe fn exit_shm(_task: *mut task_struct) {}

#[cfg(not(CONFIG_SYSVIPC))]
#[inline]
pub unsafe fn shm_init_task(_task: *mut task_struct) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
