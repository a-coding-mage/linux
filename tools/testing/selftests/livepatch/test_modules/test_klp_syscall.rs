// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2017-2023 SUSE
 * Authors: Libor Pechacek <lpechacek@suse.cz>
 *          Nicolai Stange <nstange@suse.de>
 *          Marcos Paulo de Souza <mpdesouza@suse.com>
 */

/*
 * C dependencies:
 * linux/module.h
 * linux/kernel.h
 * linux/sched.h
 * linux/slab.h
 * linux/livepatch.h
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type ssize_t = isize;

const ENOMEM: c_int = 12;

extern "C" {
    static mut kernel_kobj: *mut kobject;
    static mut current: *mut task_struct;

    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn task_tgid_vnr(task: *mut task_struct) -> c_long;
    fn kobject_create_and_add(name: *const c_char, parent: *mut kobject) -> *mut kobject;
    fn kobject_put(kobj: *mut kobject);
    fn sysfs_create_file(kobj: *mut kobject, attr: *const attribute) -> c_int;
    fn klp_enable_patch(patch: *mut klp_patch) -> c_int;
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kobj_attribute {
    pub attr: attribute,
}

#[repr(C)]
pub struct task_struct {
    pub pid: c_int,
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct klp_func {
    pub old_name: *const c_char,
    pub new_func: Option<unsafe extern "C" fn() -> c_long>,
}

#[repr(C)]
pub struct klp_object {
    pub name: *const c_char,
    pub funcs: *mut klp_func,
}

#[repr(C)]
pub struct klp_patch {
    pub mod_: *mut module,
    pub objs: *mut klp_object,
}

/*
 * Before CONFIG_ARCH_HAS_SYSCALL_WRAPPER was introduced there were no
 * prefixes for system calls.
 * powerpc set this config based on configs, so it can be enabled or not.
 *
 * In C:
 * #if defined(CONFIG_ARCH_HAS_SYSCALL_WRAPPER)
 *   #if defined(__x86_64__)
 *     #define FN_PREFIX __x64_
 *   #elif defined(__s390x__)
 *     #define FN_PREFIX __s390x_
 *   #elif defined(__aarch64__)
 *     #define FN_PREFIX __arm64_
 *   #elif defined(__powerpc__)
 *     #define FN_PREFIX
 *   #else
 *     #error "Missing syscall wrapper for the given architecture."
 *   #endif
 * #else
 *   Do not set a prefix for architectures that do not enable wrappers.
 *   #define FN_PREFIX
 * #endif
 */
#[cfg(all(CONFIG_ARCH_HAS_SYSCALL_WRAPPER, target_arch = "x86_64"))]
const SYS_GETPID_OLD_NAME: &[u8] = b"__x64_sys_getpid\0";
#[cfg(all(CONFIG_ARCH_HAS_SYSCALL_WRAPPER, target_arch = "s390x"))]
const SYS_GETPID_OLD_NAME: &[u8] = b"__s390x_sys_getpid\0";
#[cfg(all(CONFIG_ARCH_HAS_SYSCALL_WRAPPER, target_arch = "aarch64"))]
const SYS_GETPID_OLD_NAME: &[u8] = b"__arm64_sys_getpid\0";
#[cfg(all(CONFIG_ARCH_HAS_SYSCALL_WRAPPER, target_arch = "powerpc"))]
const SYS_GETPID_OLD_NAME: &[u8] = b"sys_getpid\0";
#[cfg(not(CONFIG_ARCH_HAS_SYSCALL_WRAPPER))]
const SYS_GETPID_OLD_NAME: &[u8] = b"sys_getpid\0";

/* Protects klp_pids */
static mut kpid_mutex: mutex = mutex { _private: [] };

static mut npids: c_uint = 0;
static mut npids_pending: c_uint = 0;
static mut klp_pids: [c_int; NR_CPUS] = [0; NR_CPUS];
/* module_param_array(klp_pids, int, &npids_pending, 0); */
/* MODULE_PARM_DESC(klp_pids, "Array of pids to be transitioned to livepatched state."); */

unsafe extern "C" fn npids_show(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> ssize_t {
    sprintf(buf, b"%u\n\0".as_ptr() as *const c_char, npids_pending) as ssize_t
}

/* static struct kobj_attribute klp_attr = __ATTR_RO(npids); */
static mut klp_attr: kobj_attribute = kobj_attribute {
    attr: attribute { _private: [] },
};
static mut klp_kobj: *mut kobject = core::ptr::null_mut();

unsafe extern "C" fn lp_sys_getpid() -> c_long {
    let mut i: c_int;

    mutex_lock(&mut kpid_mutex);
    if npids_pending > 0 {
        i = 0;
        while i < npids as c_int {
            if (*current).pid == klp_pids[i as usize] {
                klp_pids[i as usize] = 0;
                npids_pending = npids_pending.wrapping_sub(1);
                break;
            }
            i += 1;
        }
    }
    mutex_unlock(&mut kpid_mutex);

    task_tgid_vnr(current)
}

static mut vmlinux_funcs: [klp_func; 2] = [
    klp_func {
        old_name: SYS_GETPID_OLD_NAME.as_ptr() as *const c_char,
        new_func: Some(lp_sys_getpid),
    },
    klp_func {
        old_name: core::ptr::null(),
        new_func: None,
    },
];

static mut objs: [klp_object; 2] = [
    klp_object {
        /* name being NULL means vmlinux */
        name: core::ptr::null(),
        funcs: unsafe { vmlinux_funcs.as_mut_ptr() },
    },
    klp_object {
        name: core::ptr::null(),
        funcs: core::ptr::null_mut(),
    },
];

extern "C" {
    static mut THIS_MODULE: *mut module;
}

static mut patch: klp_patch = klp_patch {
    mod_: unsafe { THIS_MODULE },
    objs: unsafe { objs.as_mut_ptr() },
};

unsafe extern "C" fn livepatch_init() -> c_int {
    let ret: c_int;

    klp_kobj = kobject_create_and_add(
        b"test_klp_syscall\0".as_ptr() as *const c_char,
        kernel_kobj,
    );
    if klp_kobj.is_null() {
        return -ENOMEM;
    }

    ret = sysfs_create_file(klp_kobj, &klp_attr.attr);
    if ret != 0 {
        kobject_put(klp_kobj);
        return ret;
    }

    /*
     * Save the number pids to transition to livepatched state before the
     * number of pending pids is decremented.
     */
    npids = npids_pending;

    ret = klp_enable_patch(&mut patch);
    if ret != 0 {
        kobject_put(klp_kobj);
    }

    ret
}

unsafe extern "C" fn livepatch_exit() {
    kobject_put(klp_kobj);
}

/* module_init(livepatch_init); */
/* module_exit(livepatch_exit); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_INFO(livepatch, "Y"); */
/* MODULE_AUTHOR("Libor Pechacek <lpechacek@suse.cz>"); */
/* MODULE_AUTHOR("Nicolai Stange <nstange@suse.de>"); */
/* MODULE_AUTHOR("Marcos Paulo de Souza <mpdesouza@suse.com>"); */
/* MODULE_DESCRIPTION("Livepatch test: syscall transition"); */
