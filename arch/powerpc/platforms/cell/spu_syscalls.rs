// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SPU file system -- system call stubs
 *
 * (C) Copyright IBM Deutschland Entwicklung GmbH 2005
 * (C) Copyright 2006-2007, IBM Corporation
 *
 * Author: Arnd Bergmann <arndb@de.ibm.com>
 */

// Kernel declarations supplied by the corresponding Linux headers.

/* protected by rcu */
static mut spufs_calls: *mut spufs_calls = core::ptr::null_mut();

// CONFIG_SPU_FS_MODULE selects the module-reference implementation below.
#[cfg(CONFIG_SPU_FS_MODULE)]
#[inline]
unsafe fn spufs_calls_get() -> *mut spufs_calls {
    let mut calls: *mut spufs_calls = core::ptr::null_mut();

    rcu_read_lock();
    calls = rcu_dereference(spufs_calls);
    if !calls.is_null() && !try_module_get((*calls).owner) {
        calls = core::ptr::null_mut();
    }
    rcu_read_unlock();

    calls
}

#[cfg(CONFIG_SPU_FS_MODULE)]
#[inline]
unsafe fn spufs_calls_put(calls: *mut spufs_calls) {
    if calls.is_null() {
        return;
    }

    BUG_ON(calls != spufs_calls);

    // we don't need to rcu this, as we hold a reference to the module
    module_put((*spufs_calls).owner);
}

#[cfg(not(CONFIG_SPU_FS_MODULE))]
#[inline]
unsafe fn spufs_calls_get() -> *mut spufs_calls {
    spufs_calls
}

#[cfg(not(CONFIG_SPU_FS_MODULE))]
#[inline]
unsafe fn spufs_calls_put(_calls: *mut spufs_calls) {}

// DEFINE_CLASS(spufs_calls, struct spufs_calls *, spufs_calls_put(_T),
//               spufs_calls_get(), void)

#[no_mangle]
pub unsafe extern "C" fn spu_create(
    name: *const core::ffi::c_char,
    flags: core::ffi::c_uint,
    mode: umode_t,
    neighbor_fd: core::ffi::c_int,
) -> core::ffi::c_long {
    let calls = spufs_calls_get();
    if calls.is_null() {
        return -ENOSYS as core::ffi::c_long;
    }

    if flags & SPU_CREATE_AFFINITY_SPU != 0 {
        let neighbor = fdget(neighbor_fd);
        if fd_empty(neighbor) {
            spufs_calls_put(calls);
            return -EBADF as core::ffi::c_long;
        }
        let result = ((*calls).create_thread)(name, flags, mode, fd_file(neighbor));
        fdput(neighbor);
        spufs_calls_put(calls);
        result
    } else {
        let result = ((*calls).create_thread)(name, flags, mode, core::ptr::null_mut());
        spufs_calls_put(calls);
        result
    }
}

#[no_mangle]
pub unsafe extern "C" fn spu_run(
    fd: core::ffi::c_int,
    unpc: *mut u32,
    ustatus: *mut u32,
) -> core::ffi::c_long {
    let calls = spufs_calls_get();
    if calls.is_null() {
        return -ENOSYS as core::ffi::c_long;
    }

    let arg = fdget(fd);
    if fd_empty(arg) {
        spufs_calls_put(calls);
        return -EBADF as core::ffi::c_long;
    }

    let result = ((*calls).spu_run)(fd_file(arg), unpc, ustatus);
    fdput(arg);
    spufs_calls_put(calls);
    result
}

#[cfg(CONFIG_COREDUMP)]
pub unsafe extern "C" fn elf_coredump_extra_notes_size() -> core::ffi::c_int {
    let calls = spufs_calls_get();
    if calls.is_null() {
        return 0;
    }

    let result = ((*calls).coredump_extra_notes_size)();
    spufs_calls_put(calls);
    result
}

#[cfg(CONFIG_COREDUMP)]
pub unsafe extern "C" fn elf_coredump_extra_notes_write(
    cprm: *mut coredump_params,
) -> core::ffi::c_int {
    let calls = spufs_calls_get();
    if calls.is_null() {
        return 0;
    }

    let result = ((*calls).coredump_extra_notes_write)(cprm);
    spufs_calls_put(calls);
    result
}

#[no_mangle]
pub unsafe extern "C" fn notify_spus_active() {
    let calls = spufs_calls_get();
    if calls.is_null() {
        return;
    }

    ((*calls).notify_spus_active)();
    spufs_calls_put(calls);
}

#[no_mangle]
pub unsafe extern "C" fn register_spu_syscalls(calls: *mut spufs_calls) -> core::ffi::c_int {
    if !spufs_calls.is_null() {
        return -EBUSY;
    }

    rcu_assign_pointer(spufs_calls, calls);
    0
}

#[no_mangle]
pub unsafe extern "C" fn unregister_spu_syscalls(calls: *mut spufs_calls) {
    BUG_ON((*spufs_calls).owner != (*calls).owner);
    RCU_INIT_POINTER(spufs_calls, core::ptr::null_mut());
    synchronize_rcu();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
