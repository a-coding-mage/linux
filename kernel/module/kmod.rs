// SPDX-License-Identifier: GPL-2.0
/*
 * kmod - the kernel module loader
 *
 * Copyright (C) 2023 Luis Chamberlain <mcgrof@kernel.org>
 */

// Dependencies supplied by the surrounding kernel build are intentionally not
// redefined here.

const MAX_KMOD_CONCURRENT: usize = 50;
static mut KMOD_CONCURRENT_MAX: Semaphore = unsafe { Semaphore::new(MAX_KMOD_CONCURRENT) };

const MAX_KMOD_ALL_BUSY_TIMEOUT: usize = 5;

/*
	modprobe_path is set via /proc/sys.
*/
#[no_mangle]
pub static mut modprobe_path: [core::ffi::c_char; KMOD_PATH_LEN] = CONFIG_MODPROBE_PATH;

unsafe fn free_modprobe_argv(info: *mut subprocess_info) {
    kfree((*info).argv.add(3) as *mut core::ffi::c_void); /* check call_modprobe() */
    kfree((*info).argv as *mut core::ffi::c_void);
}

unsafe fn call_modprobe(orig_module_name: *mut core::ffi::c_char, wait: core::ffi::c_int) -> core::ffi::c_int {
    static mut ENVP: [*mut core::ffi::c_char; 4] = [
        b"HOME=/\0".as_ptr() as *mut core::ffi::c_char,
        b"TERM=linux\0".as_ptr() as *mut core::ffi::c_char,
        b"PATH=/sbin:/usr/sbin:/bin:/usr/bin\0".as_ptr() as *mut core::ffi::c_char,
        core::ptr::null_mut(),
    ];
    let mut info: *mut subprocess_info;
    let module_name: *mut core::ffi::c_char;
    let mut ret: core::ffi::c_int;

    let argv = kmalloc((5 * core::mem::size_of::<*mut core::ffi::c_char>()) as usize, GFP_KERNEL)
        as *mut *mut core::ffi::c_char;
    if argv.is_null() {
        kmod_dup_request_announce(orig_module_name, -ENOMEM);
        return -ENOMEM;
    }

    module_name = kstrdup(orig_module_name, GFP_KERNEL);
    if module_name.is_null() {
        kfree(argv as *mut core::ffi::c_void);
        kmod_dup_request_announce(orig_module_name, -ENOMEM);
        return -ENOMEM;
    }

    *argv.add(0) = modprobe_path.as_mut_ptr();
    *argv.add(1) = b"-q\0".as_ptr() as *mut core::ffi::c_char;
    *argv.add(2) = b"--\0".as_ptr() as *mut core::ffi::c_char;
    *argv.add(3) = module_name; /* check free_modprobe_argv() */
    *argv.add(4) = core::ptr::null_mut();

    info = call_usermodehelper_setup(modprobe_path.as_mut_ptr(), argv, ENVP.as_mut_ptr(), GFP_KERNEL,
                                     None, Some(free_modprobe_argv), None);
    if info.is_null() {
        kfree(module_name as *mut core::ffi::c_void);
        kfree(argv as *mut core::ffi::c_void);
        kmod_dup_request_announce(orig_module_name, -ENOMEM);
        return -ENOMEM;
    }

    ret = call_usermodehelper_exec(info, wait | UMH_KILLABLE);
    kmod_dup_request_announce(orig_module_name, ret);
    ret
}

/**
 * __request_module - try to load a kernel module
 * @wait: wait (or not) for the operation to complete
 * @fmt: printf style format string for the name of the module
 * @...: arguments as specified in the format string
 *
 * Load a module using the user mode module loader. The function returns
 * zero on success or a negative errno code or positive exit code from
 * "modprobe" on failure. Note that a successful module load does not mean
 * the module did not then unload and exit on an error of its own. Callers
 * must check that the service they requested is now available not blindly
 * invoke it.
 *
 * If module auto-loading support is disabled then this function
 * simply returns -ENOENT.
 */
#[no_mangle]
pub unsafe extern "C" fn __request_module(wait: bool, fmt: *const core::ffi::c_char, mut args: ...) -> core::ffi::c_int {
    let mut module_name: [core::ffi::c_char; MODULE_NAME_LEN] = [0; MODULE_NAME_LEN];
    let mut ret: core::ffi::c_int;
    let mut dup_ret: core::ffi::c_int = 0;

    WARN_ON_ONCE(wait && current_is_async());

    if modprobe_path[0] == 0 {
        return -ENOENT;
    }

    ret = vsnprintf(module_name.as_mut_ptr(), MODULE_NAME_LEN, fmt, args);
    if ret >= MODULE_NAME_LEN as core::ffi::c_int {
        return -ENAMETOOLONG;
    }

    ret = security_kernel_module_request(module_name.as_mut_ptr());
    if ret != 0 {
        return ret;
    }

    ret = down_timeout(&mut KMOD_CONCURRENT_MAX, (MAX_KMOD_ALL_BUSY_TIMEOUT * HZ) as _);
    if ret != 0 {
        pr_warn_ratelimited!("request_module: modprobe %s cannot be processed, kmod busy with %d threads for more than %d seconds now", module_name.as_ptr(), MAX_KMOD_CONCURRENT, MAX_KMOD_ALL_BUSY_TIMEOUT);
        return ret;
    }

    trace_module_request(module_name.as_mut_ptr(), wait, _RET_IP_);

    if kmod_dup_request_exists_wait(module_name.as_mut_ptr(), wait, &mut dup_ret) {
        ret = dup_ret;
    } else {
        ret = call_modprobe(module_name.as_mut_ptr(), if wait { UMH_WAIT_PROC } else { UMH_WAIT_EXEC });
    }

    up(&mut KMOD_CONCURRENT_MAX);
    ret
}

// EXPORT_SYMBOL(__request_module);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
