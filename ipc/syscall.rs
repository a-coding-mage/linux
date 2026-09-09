// SPDX-License-Identifier: GPL-2.0
/*
 * sys_ipc() is the old de-multiplexer for the SysV IPC calls.
 *
 * This is really horribly ugly, and new architectures should just wire up
 * the individual syscalls instead.
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[cfg(__ARCH_WANT_SYS_IPC)]
pub unsafe fn ksys_ipc(
    mut call: u32,
    first: i32,
    second: usize,
    third: usize,
    ptr: *mut core::ffi::c_void,
    fifth: isize,
) -> i32 {
    let version: u32 = call >> 16; // hack for backward compatibility
    call &= 0xffff;

    match call {
        SEMOP => ksys_semtimedop(first, ptr as *mut sembuf, second, core::ptr::null()),
        SEMTIMEDOP => {
            // IS_ENABLED(CONFIG_64BIT)
            if cfg!(target_pointer_width = "64") {
                ksys_semtimedop(first, ptr as *mut sembuf, second, fifth as *const kernel_timespec)
            // IS_ENABLED(CONFIG_COMPAT_32BIT_TIME)
            } else if cfg!(feature = "CONFIG_COMPAT_32BIT_TIME") {
                compat_ksys_semtimedop(first, ptr, second, fifth as *const old_timespec32)
            } else {
                -ENOSYS
            }
        }
        SEMGET => ksys_semget(first, second, third),
        SEMCTL => {
            let mut arg: usize = 0;
            if ptr.is_null() {
                return -EINVAL;
            }
            if get_user(&mut arg, ptr as *mut usize) != 0 {
                return -EFAULT;
            }
            ksys_old_semctl(first, second, third, arg)
        }
        MSGSND => ksys_msgsnd(first, ptr as *mut msgbuf, second, third),
        MSGRCV => match version {
            0 => {
                let mut tmp: ipc_kludge = core::mem::zeroed();
                if ptr.is_null() {
                    return -EINVAL;
                }
                if copy_from_user(&mut tmp, ptr, core::mem::size_of::<ipc_kludge>()) != 0 {
                    return -EFAULT;
                }
                ksys_msgrcv(first, tmp.msgp, second, tmp.msgtyp, third)
            }
            _ => ksys_msgrcv(first, ptr as *mut msgbuf, second, fifth, third),
        },
        MSGGET => ksys_msgget(first as key_t, second),
        MSGCTL => ksys_old_msgctl(first, second, ptr as *mut msqid_ds),
        SHMAT => match version {
            1 => {
                /*
                 * This was the entry point for kernel-originating calls
                 * from iBCS2 in 2.2 days.
                 */
                -EINVAL
            }
            _ => {
                let mut raddr: usize = 0;
                let ret = do_shmat(first, ptr as *mut u8, second, &mut raddr, SHMLBA);
                if ret != 0 {
                    return ret;
                }
                put_user(raddr, third as *mut usize)
            }
        },
        SHMDT => ksys_shmdt(ptr as *mut u8),
        SHMGET => ksys_shmget(first, second, third),
        SHMCTL => ksys_old_shmctl(first, second, ptr as *mut shmid_ds),
        _ => -ENOSYS,
    }
}

#[cfg(__ARCH_WANT_SYS_IPC)]
pub unsafe fn ipc(call: u32, first: i32, second: usize, third: usize,
                  ptr: *mut core::ffi::c_void, fifth: isize) -> i32 {
    ksys_ipc(call, first, second, third, ptr, fifth)
}

#[cfg(CONFIG_COMPAT)]
pub type compat_ipc_kludge = CompatIpcKludge;

#[cfg(CONFIG_COMPAT)]
#[repr(C)]
pub struct CompatIpcKludge {
    pub msgp: compat_uptr_t,
    pub msgtyp: compat_long_t,
}

#[cfg(all(CONFIG_COMPAT, CONFIG_ARCH_WANT_OLD_COMPAT_IPC))]
pub unsafe fn compat_ksys_ipc(
    mut call: u32, first: i32, second: i32, third: u32,
    ptr: compat_uptr_t, fifth: u32,
) -> i32 {
    let version = call >> 16; // hack for backward compatibility
    call &= 0xffff;
    let mut pad: u32;

    match call {
        SEMOP => ksys_semtimedop(first, compat_ptr(ptr), second as usize, core::ptr::null()),
        SEMTIMEDOP => {
            // !IS_ENABLED(CONFIG_COMPAT_32BIT_TIME)
            if !cfg!(feature = "CONFIG_COMPAT_32BIT_TIME") {
                return -ENOSYS;
            }
            compat_ksys_semtimedop(first, compat_ptr(ptr), second as usize, compat_ptr(fifth))
        }
        SEMGET => ksys_semget(first, second as usize, third as usize),
        SEMCTL => {
            if ptr == 0 {
                return -EINVAL;
            }
            if get_user(&mut pad, compat_ptr(ptr)) != 0 {
                return -EFAULT;
            }
            compat_ksys_old_semctl(first, second as usize, third as usize, pad)
        }
        MSGSND => compat_ksys_msgsnd(first, ptr, second as usize, third as usize),
        MSGRCV => {
            let uptr = compat_ptr(ptr);
            if first < 0 || second < 0 {
                return -EINVAL;
            }
            if version == 0 {
                let mut ipck: CompatIpcKludge = core::mem::zeroed();
                if uptr.is_null() {
                    return -EINVAL;
                }
                if copy_from_user(&mut ipck, uptr, core::mem::size_of::<CompatIpcKludge>()) != 0 {
                    return -EFAULT;
                }
                compat_ksys_msgrcv(first, ipck.msgp, second as usize, ipck.msgtyp, third as usize)
            } else {
                compat_ksys_msgrcv(first, ptr, second as usize, fifth as isize, third as usize)
            }
        }
        MSGGET => ksys_msgget(first, second as usize),
        MSGCTL => compat_ksys_old_msgctl(first, second as usize, compat_ptr(ptr)),
        SHMAT => {
            let mut raddr: usize = 0;
            if version == 1 {
                return -EINVAL;
            }
            let err = do_shmat(first, compat_ptr(ptr), second as usize, &mut raddr, COMPAT_SHMLBA);
            if err < 0 {
                return err;
            }
            put_user(raddr, compat_ptr(third))
        }
        SHMDT => ksys_shmdt(compat_ptr(ptr)),
        SHMGET => ksys_shmget(first, second as usize, third as usize),
        SHMCTL => compat_ksys_old_shmctl(first, second as usize, compat_ptr(ptr)),
        _ => -ENOSYS,
    }
}

#[cfg(all(CONFIG_COMPAT, CONFIG_ARCH_WANT_OLD_COMPAT_IPC))]
pub unsafe fn compat_ipc(call: u32, first: i32, second: i32, third: u32,
                          ptr: compat_uptr_t, fifth: u32) -> i32 {
    compat_ksys_ipc(call, first, second, third, ptr, fifth)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
