// SPDX-License-Identifier: GPL-2.0-only

// Dependency declarations and architecture-specific constants are supplied by
// the surrounding kernel translation unit.

unsafe fn alloc_gcs(addr: libc::c_ulong, size: libc::c_ulong) -> libc::c_ulong {
    vm_mmap_shadow_stack(addr, size, 0)
}

unsafe fn gcs_size(mut size: libc::c_ulong) -> libc::c_ulong {
    if size != 0 {
        return PAGE_ALIGN(size);
    }

    /* Allocate RLIMIT_STACK/2 with limits of PAGE_SIZE..2G */
    size = PAGE_ALIGN(core::cmp::min(
        rlimit(RLIMIT_STACK) / 2,
        SZ_2G,
    ));
    core::cmp::max(PAGE_SIZE, size)
}

pub unsafe fn gcs_alloc_thread_stack(
    tsk: *mut task_struct,
    args: *const kernel_clone_args,
) -> libc::c_ulong {
    let mut addr: libc::c_ulong;
    let mut size: libc::c_ulong;

    if !system_supports_gcs() {
        return 0;
    }

    if !task_gcs_el0_enabled(tsk) {
        return 0;
    }

    if ((*args).flags & (CLONE_VFORK | CLONE_VM)) != CLONE_VM {
        (*tsk).thread.gcspr_el0 = read_sysreg_s(SYS_GCSPR_EL0);
        return 0;
    }

    size = (*args).stack_size / 2;
    size = gcs_size(size);
    addr = alloc_gcs(0, size);
    if IS_ERR_VALUE(addr) {
        return addr;
    }

    (*tsk).thread.gcs_base = addr;
    (*tsk).thread.gcs_size = size;
    (*tsk).thread.gcspr_el0 = addr + size - core::mem::size_of::<u64>() as libc::c_ulong;

    addr
}

pub unsafe fn map_shadow_stack(
    mut addr: libc::c_ulong,
    size: libc::c_ulong,
    flags: libc::c_uint,
) -> libc::c_long {
    let alloc_size: libc::c_ulong;
    let mut cap_ptr: *mut libc::c_ulong;
    let mut cap_val: libc::c_ulong;
    let mut ret: libc::c_int = 0;
    let cap_offset: libc::c_int;

    if !system_supports_gcs() {
        return -EOPNOTSUPP;
    }
    if flags & !(SHADOW_STACK_SET_TOKEN | SHADOW_STACK_SET_MARKER) != 0 {
        return -EINVAL;
    }
    if !PAGE_ALIGNED(addr) {
        return -EINVAL;
    }
    if size == 8 || !IS_ALIGNED(size, 8) {
        return -EINVAL;
    }

    /*
     * An overflow would result in attempting to write the restore token
     * to the wrong location. Not catastrophic, but just return the right
     * error code and block it.
     */
    alloc_size = PAGE_ALIGN(size);
    if alloc_size < size {
        return -EOVERFLOW;
    }

    addr = alloc_gcs(addr, alloc_size);
    if IS_ERR_VALUE(addr) {
        return addr as libc::c_long;
    }

    /*
     * Put a cap token at the end of the allocated region so it
     * can be switched to.
     */
    if flags & SHADOW_STACK_SET_TOKEN != 0 {
        /* Leave an extra empty frame as a top of stack marker? */
        if flags & SHADOW_STACK_SET_MARKER != 0 {
            cap_offset = 2;
        } else {
            cap_offset = 1;
        }

        cap_ptr = (addr + size
            - (cap_offset as libc::c_ulong * core::mem::size_of::<libc::c_ulong>() as libc::c_ulong))
            as *mut libc::c_ulong;
        cap_val = GCS_CAP(cap_ptr);

        put_user_gcs(cap_val, cap_ptr, &mut ret);
        if ret != 0 {
            vm_munmap(addr, size);
            return -EFAULT;
        }

        /* Ensure the new cap is ordered before standard memory accesses. */
        gcsb_dsync();
    }

    addr as libc::c_long
}

/* Apply the GCS mode configured for the specified task to the hardware. */
pub unsafe fn gcs_set_el0_mode(task: *mut task_struct) {
    let mut gcscre0_el1: u64 = GCSCRE0_EL1_nTR;

    if (*task).thread.gcs_el0_mode & PR_SHADOW_STACK_ENABLE != 0 {
        gcscre0_el1 |= GCSCRE0_EL1_RVCHKEN | GCSCRE0_EL1_PCRSEL;
    }
    if (*task).thread.gcs_el0_mode & PR_SHADOW_STACK_WRITE != 0 {
        gcscre0_el1 |= GCSCRE0_EL1_STREn;
    }
    if (*task).thread.gcs_el0_mode & PR_SHADOW_STACK_PUSH != 0 {
        gcscre0_el1 |= GCSCRE0_EL1_PUSHMEn;
    }
    write_sysreg_s(gcscre0_el1, SYS_GCSCRE0_EL1);
}

pub unsafe fn gcs_free(task: *mut task_struct) {
    if !system_supports_gcs() || (*task).mm.is_null() || (*task).mm != current.mm {
        return;
    }
    if (*task).thread.gcs_base != 0 {
        vm_munmap((*task).thread.gcs_base, (*task).thread.gcs_size);
    }
    (*task).thread.gcspr_el0 = 0;
    (*task).thread.gcs_base = 0;
    (*task).thread.gcs_size = 0;
}

pub unsafe fn arch_set_shadow_stack_status(task: *mut task_struct, arg: libc::c_ulong) -> libc::c_int {
    let mut gcs: libc::c_ulong;
    let mut size: libc::c_ulong;
    let ret: libc::c_int;

    if !system_supports_gcs() || is_compat_thread(task_thread_info(task)) {
        return -EINVAL;
    }
    if arg & !PR_SHADOW_STACK_SUPPORTED_STATUS_MASK != 0 {
        return -EINVAL;
    }
    ret = gcs_check_locked(task, arg);
    if ret != 0 {
        return ret;
    }
    if arg & PR_SHADOW_STACK_ENABLE != 0 && !task_gcs_el0_enabled(task) {
        if (*task).thread.gcs_base != 0 || (*task).thread.gcspr_el0 != 0 {
            return -EINVAL;
        }
        if task != current {
            return -EBUSY;
        }
        size = gcs_size(0);
        gcs = alloc_gcs(0, size);
        if IS_ERR_VALUE(gcs) {
            return gcs as libc::c_int;
        }
        (*task).thread.gcspr_el0 = gcs + size - core::mem::size_of::<u64>() as libc::c_ulong;
        (*task).thread.gcs_base = gcs;
        (*task).thread.gcs_size = size;
        if task == current {
            write_sysreg_s((*task).thread.gcspr_el0, SYS_GCSPR_EL0);
        }
    }
    (*task).thread.gcs_el0_mode = arg;
    if task == current {
        gcs_set_el0_mode(task);
    }
    0
}

pub unsafe fn arch_get_shadow_stack_status(
    task: *mut task_struct,
    arg: *mut libc::c_ulong,
) -> libc::c_int {
    if !system_supports_gcs() || is_compat_thread(task_thread_info(task)) {
        return -EINVAL;
    }
    put_user((*task).thread.gcs_el0_mode, arg)
}

pub unsafe fn arch_lock_shadow_stack_status(task: *mut task_struct, arg: libc::c_ulong) -> libc::c_int {
    if !system_supports_gcs() || is_compat_thread(task_thread_info(task)) {
        return -EINVAL;
    }
    /* We support locking unknown bits so applications can prevent future changes. */
    (*task).thread.gcs_el0_locked |= arg;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
