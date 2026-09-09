// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel headers are referenced externally.

/*
 * We don't expose the real in-memory order of objects for security reasons.
 * But still the comparison results should be suitable for sorting. So we
 * obfuscate kernel pointers values and compare the production instead.
 *
 * The obfuscation is done in two steps. First we xor the kernel pointer with
 * a random value, which puts pointer into a new position in a reordered space.
 * Secondly we multiply the xor production with a large odd random number to
 * permute its bits even more (the odd multiplier guarantees that the product
 * is unique ever after the high bits are truncated, since any odd number is
 * relative prime to 2^n).
 *
 * Note also that the obfuscation itself is invisible to userspace and if needed
 * it can be changed to an alternate scheme.
 */
static mut cookies: [[libc::c_ulong; 2]; KCMP_TYPES] = [[0; 2]; KCMP_TYPES];

unsafe fn kptr_obfuscate(v: libc::c_long, type_: libc::c_int) -> libc::c_long {
    (v ^ cookies[type_ as usize][0] as libc::c_long)
        .wrapping_mul(cookies[type_ as usize][1] as libc::c_long)
}

/*
 * 0 - equal, i.e. v1 = v2
 * 1 - less than, i.e. v1 < v2
 * 2 - greater than, i.e. v1 > v2
 * 3 - not equal but ordering unavailable (reserved for future)
 */
unsafe fn kcmp_ptr(
    v1: *mut libc::c_void,
    v2: *mut libc::c_void,
    type_: enum_kcmp_type,
) -> libc::c_int {
    let t1 = kptr_obfuscate(v1 as libc::c_long, type_ as libc::c_int);
    let t2 = kptr_obfuscate(v2 as libc::c_long, type_ as libc::c_int);
    ((t1 < t2) as libc::c_int) | (((t1 > t2) as libc::c_int) << 1)
}

/* The caller must have pinned the task */
unsafe fn get_file_raw_ptr(task: *mut task_struct, idx: libc::c_uint) -> *mut file {
    let file = fget_task(task, idx);
    if !file.is_null() {
        fput(file);
    }
    file
}

unsafe fn kcmp_unlock(l1: *mut rw_semaphore, l2: *mut rw_semaphore) {
    if l2 != l1 {
        up_read(l2);
    }
    up_read(l1);
}

unsafe fn kcmp_lock(l1_: *mut rw_semaphore, l2_: *mut rw_semaphore) -> libc::c_int {
    let mut l1 = l1_;
    let mut l2 = l2_;
    if l2 > l1 {
        core::mem::swap(&mut l1, &mut l2);
    }
    let mut err = down_read_killable(l1);
    if err == 0 && l1 != l2 {
        err = down_read_killable_nested(l2, SINGLE_DEPTH_NESTING);
        if err != 0 {
            up_read(l1);
        }
    }
    err
}

#[cfg(CONFIG_EPOLL)]
unsafe fn kcmp_epoll_target(
    task1: *mut task_struct,
    task2: *mut task_struct,
    idx1: libc::c_ulong,
    uslot: *mut kcmp_epoll_slot,
) -> libc::c_int {
    let mut slot = core::mem::MaybeUninit::<kcmp_epoll_slot>::uninit();
    if copy_from_user(slot.as_mut_ptr(), uslot, core::mem::size_of::<kcmp_epoll_slot>()) != 0 {
        return -EFAULT;
    }
    let slot = slot.assume_init();
    let filp = get_file_raw_ptr(task1, idx1 as libc::c_uint);
    if filp.is_null() {
        return -EBADF;
    }
    let filp_epoll = fget_task(task2, slot.efd);
    if filp_epoll.is_null() {
        return -EBADF;
    }
    let filp_tgt = get_epoll_tfile_raw_ptr(filp_epoll, slot.tfd, slot.toff);
    fput(filp_epoll);
    if IS_ERR(filp_tgt) {
        return PTR_ERR(filp_tgt);
    }
    kcmp_ptr(filp as *mut libc::c_void, filp_tgt as *mut libc::c_void, KCMP_FILE)
}

#[cfg(not(CONFIG_EPOLL))]
unsafe fn kcmp_epoll_target(
    _task1: *mut task_struct,
    _task2: *mut task_struct,
    _idx1: libc::c_ulong,
    _uslot: *mut kcmp_epoll_slot,
) -> libc::c_int {
    -EOPNOTSUPP
}

unsafe fn kcmp(
    pid1: pid_t,
    pid2: pid_t,
    type_: libc::c_int,
    idx1: libc::c_ulong,
    idx2: libc::c_ulong,
) -> libc::c_long {
    let mut task1: *mut task_struct;
    let mut task2: *mut task_struct;
    let ret: libc::c_int;

    rcu_read_lock();
    task1 = find_task_by_vpid(pid1);
    task2 = find_task_by_vpid(pid2);
    if task1.is_null() || task2.is_null() {
        rcu_read_unlock();
        return -ESRCH as libc::c_long;
    }
    get_task_struct(task1);
    get_task_struct(task2);
    rcu_read_unlock();

    ret = kcmp_lock(&mut (*(*task1).signal).exec_update_lock,
                    &mut (*(*task2).signal).exec_update_lock);
    if ret != 0 {
        put_task_struct(task1);
        put_task_struct(task2);
        return ret as libc::c_long;
    }
    let mut result = ret;
    if !ptrace_may_access(task1, PTRACE_MODE_READ_REALCREDS)
        || !ptrace_may_access(task2, PTRACE_MODE_READ_REALCREDS)
    {
        result = -EPERM;
    } else {
        result = match type_ {
            KCMP_FILE => {
                let filp1 = get_file_raw_ptr(task1, idx1 as libc::c_uint);
                let filp2 = get_file_raw_ptr(task2, idx2 as libc::c_uint);
                if !filp1.is_null() && !filp2.is_null() {
                    kcmp_ptr(filp1 as *mut libc::c_void, filp2 as *mut libc::c_void, KCMP_FILE)
                } else { -EBADF }
            }
            KCMP_VM => kcmp_ptr((*task1).mm as *mut libc::c_void, (*task2).mm as *mut libc::c_void, KCMP_VM),
            KCMP_FILES => kcmp_ptr((*task1).files as *mut libc::c_void, (*task2).files as *mut libc::c_void, KCMP_FILES),
            KCMP_FS => kcmp_ptr((*task1).real_fs as *mut libc::c_void, (*task2).real_fs as *mut libc::c_void, KCMP_FS),
            KCMP_SIGHAND => kcmp_ptr((*task1).sighand as *mut libc::c_void, (*task2).sighand as *mut libc::c_void, KCMP_SIGHAND),
            KCMP_IO => kcmp_ptr((*task1).io_context as *mut libc::c_void, (*task2).io_context as *mut libc::c_void, KCMP_IO),
            KCMP_SYSVSEM => {
                #[cfg(CONFIG_SYSVIPC)]
                { kcmp_ptr((*task1).sysvsem.undo_list as *mut libc::c_void, (*task2).sysvsem.undo_list as *mut libc::c_void, KCMP_SYSVSEM) }
                #[cfg(not(CONFIG_SYSVIPC))]
                { -EOPNOTSUPP }
            }
            KCMP_EPOLL_TFD => kcmp_epoll_target(task1, task2, idx1, idx2 as *mut kcmp_epoll_slot),
            _ => -EINVAL,
        };
    }
    kcmp_unlock(&mut (*(*task1).signal).exec_update_lock,
                &mut (*(*task2).signal).exec_update_lock);
    put_task_struct(task1);
    put_task_struct(task2);
    result as libc::c_long
}

unsafe fn kcmp_cookies_init() -> libc::c_int {
    get_random_bytes(cookies.as_mut_ptr() as *mut libc::c_void, core::mem::size_of_val(&cookies));
    let mut i = 0;
    while i < KCMP_TYPES {
        cookies[i][1] |= (!(0 as libc::c_ulong) >> 1) | 1;
        i += 1;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
