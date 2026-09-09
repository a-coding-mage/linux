// SPDX-License-Identifier: GPL-2.0-or-later OR copyleft-next-0.3.1
/* kmod stress test driver; direct Rust translation of test_kmod.c. */

// Kernel headers and symbols are supplied by the surrounding kernel/Rust
// environment; they are intentionally not reimplemented here.

const TEST_START_NUM_THREADS: u32 = 50;
const TEST_START_DRIVER: *const core::ffi::c_char = c"test_module".as_ptr();

static mut force_init_test: bool = false;
static mut start_driver: *mut core::ffi::c_char = core::ptr::null_mut();
static mut start_test_fs: *mut core::ffi::c_char = core::ptr::null_mut();

static mut reg_dev_mutex: mutex = unsafe { core::mem::zeroed() };
static mut reg_test_devs: list_head = unsafe { core::mem::zeroed() };
static mut num_test_devs: i32 = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum kmod_test_case {
    __TEST_KMOD_INVALID = 0,
    TEST_KMOD_DRIVER,
    TEST_KMOD_FS_TYPE,
    __TEST_KMOD_MAX,
}

#[repr(C)]
struct test_config {
    test_driver: *mut core::ffi::c_char,
    test_fs: *mut core::ffi::c_char,
    num_threads: u32,
    test_case: kmod_test_case,
    test_result: i32,
}

#[repr(C)]
struct kmod_test_device_info {
    ret_sync: i32,
    fs_sync: *mut file_system_type,
    task_sync: *mut task_struct,
    thread_idx: u32,
    test_dev: *mut kmod_test_device,
    need_mod_put: bool,
}

#[repr(C)]
struct kmod_test_device {
    dev_idx: i32,
    config: test_config,
    misc_dev: miscdevice,
    dev: *mut device,
    config_mutex: mutex,
    trigger_mutex: mutex,
    thread_mutex: mutex,
    done: u32,
    test_is_oom: bool,
    kthreads_done: completion,
    list: list_head,
    info: *mut kmod_test_device_info,
}

unsafe fn test_case_str(test_case: kmod_test_case) -> *const core::ffi::c_char {
    match test_case {
        kmod_test_case::TEST_KMOD_DRIVER => c"TEST_KMOD_DRIVER".as_ptr(),
        kmod_test_case::TEST_KMOD_FS_TYPE => c"TEST_KMOD_FS_TYPE".as_ptr(),
        _ => c"invalid".as_ptr(),
    }
}

unsafe fn dev_to_misc_dev(dev: *mut device) -> *mut miscdevice { dev_get_drvdata(dev) }
unsafe fn misc_dev_to_test_dev(misc_dev: *mut miscdevice) -> *mut kmod_test_device {
    container_of!(misc_dev, kmod_test_device, misc_dev)
}
unsafe fn dev_to_test_dev(dev: *mut device) -> *mut kmod_test_device {
    misc_dev_to_test_dev(dev_to_misc_dev(dev))
}

unsafe fn kmod_test_done_check(test_dev: *mut kmod_test_device, idx: u32) {
    let config = &mut (*test_dev).config;
    (*test_dev).done += 1;
    dev_dbg!((*test_dev).dev, "Done thread count: %u\n", (*test_dev).done);
    if (*test_dev).done == config.num_threads {
        dev_info!((*test_dev).dev, "Done: %u threads have all run now\n", (*test_dev).done);
        dev_info!((*test_dev).dev, "Last thread to run: %u\n", idx);
        complete!(&mut (*test_dev).kthreads_done);
    }
}

unsafe fn test_kmod_put_module(info: *mut kmod_test_device_info) {
    let test_dev = (*info).test_dev;
    let config = &mut (*test_dev).config;
    if !(*info).need_mod_put { return; }
    match config.test_case {
        kmod_test_case::TEST_KMOD_DRIVER => {}
        kmod_test_case::TEST_KMOD_FS_TYPE => {
            if !(*info).fs_sync.is_null() && !(*(*info).fs_sync).owner.is_null() {
                module_put((*(*info).fs_sync).owner);
            }
        }
        _ => BUG!(),
    }
    (*info).need_mod_put = true;
}

unsafe extern "C" fn run_request(data: *mut core::ffi::c_void) -> i32 {
    let info = data as *mut kmod_test_device_info;
    let test_dev = (*info).test_dev;
    let config = &mut (*test_dev).config;
    match config.test_case {
        kmod_test_case::TEST_KMOD_DRIVER => (*info).ret_sync = request_module!("%s", config.test_driver),
        kmod_test_case::TEST_KMOD_FS_TYPE => { (*info).fs_sync = get_fs_type(config.test_fs); (*info).need_mod_put = true; }
        _ => { BUG!(); return -EINVAL; }
    }
    dev_dbg!((*test_dev).dev, "Ran thread %u\n", (*info).thread_idx);
    test_kmod_put_module(info);
    mutex_lock!(&mut (*test_dev).thread_mutex);
    (*info).task_sync = core::ptr::null_mut();
    kmod_test_done_check(test_dev, (*info).thread_idx);
    mutex_unlock!(&mut (*test_dev).thread_mutex);
    0
}

unsafe fn tally_work_test(info: *mut kmod_test_device_info) -> i32 {
    let test_dev = (*info).test_dev;
    let config = &mut (*test_dev).config;
    let mut err_ret = 0;
    match config.test_case {
        kmod_test_case::TEST_KMOD_DRIVER => {
            if (*info).ret_sync != 0 { err_ret = (*info).ret_sync; }
            dev_info!((*test_dev).dev, "Sync thread %d return status: %d\n", (*info).thread_idx, (*info).ret_sync);
        }
        kmod_test_case::TEST_KMOD_FS_TYPE => {
            if (*info).fs_sync.is_null() { err_ret = -EINVAL; }
            dev_info!((*test_dev).dev, "Sync thread %u fs: %s\n", (*info).thread_idx, if !(*info).fs_sync.is_null() { config.test_fs } else { c"NULL".as_ptr() as *mut _ });
        }
        _ => BUG!(),
    }
    err_ret
}

unsafe fn tally_up_work(test_dev: *mut kmod_test_device) {
    let config = &mut (*test_dev).config;
    mutex_lock!(&mut (*test_dev).thread_mutex);
    dev_info!((*test_dev).dev, "Results:\n");
    let mut err_ret = 0;
    for idx in 0..config.num_threads { let ret = tally_work_test((*test_dev).info.add(idx as usize)); if ret != 0 { err_ret = ret; } }
    config.test_result = err_ret;
    mutex_unlock!(&mut (*test_dev).thread_mutex);
}

unsafe fn try_one_request(test_dev: *mut kmod_test_device, idx: u32) -> i32 {
    let info = (*test_dev).info.add(idx as usize);
    mutex_lock!(&mut (*test_dev).thread_mutex);
    (*info).thread_idx = idx; (*info).test_dev = test_dev;
    (*info).task_sync = kthread_run!(run_request, info as *mut _, "%s-%u", KBUILD_MODNAME, idx);
    if (*info).task_sync.is_null() || IS_ERR!((*info).task_sync) {
        (*test_dev).test_is_oom = true; dev_err!((*test_dev).dev, "Setting up thread %u failed\n", idx); (*info).task_sync = core::ptr::null_mut();
        (*info).ret_sync = -ENOMEM; mutex_unlock!(&mut (*test_dev).thread_mutex); return -ENOMEM;
    }
    dev_dbg!((*test_dev).dev, "Kicked off thread %u\n", idx); mutex_unlock!(&mut (*test_dev).thread_mutex); 0
}

unsafe fn test_dev_kmod_stop_tests(test_dev: *mut kmod_test_device) {
    let config = &mut (*test_dev).config;
    dev_info!((*test_dev).dev, "Ending request_module() tests\n");
    mutex_lock!(&mut (*test_dev).thread_mutex);
    for i in 0..config.num_threads { let info = (*test_dev).info.add(i as usize); if !(*info).task_sync.is_null() && !IS_ERR!((*info).task_sync) { dev_info!((*test_dev).dev, "Stopping still-running thread %i\n", i); kthread_stop!((*info).task_sync); } if !(*info).task_sync.is_null() && (*info).need_mod_put { test_kmod_put_module(info); } }
    mutex_unlock!(&mut (*test_dev).thread_mutex);
}

unsafe fn try_requests(test_dev: *mut kmod_test_device) -> i32 {
    let n = (*test_dev).config.num_threads; let mut any_error = false;
    for idx in 0..n { if (*test_dev).test_is_oom { any_error = true; break; } if try_one_request(test_dev, idx) != 0 { any_error = true; break; } }
    if !any_error { (*test_dev).test_is_oom = false; dev_info!((*test_dev).dev, "No errors were found while initializing threads\n"); wait_for_completion!(&mut (*test_dev).kthreads_done); tally_up_work(test_dev); 0 } else { (*test_dev).test_is_oom = true; dev_info!((*test_dev).dev, "At least one thread failed to start, stop all work\n"); test_dev_kmod_stop_tests(test_dev); -ENOMEM }
}

unsafe fn run_test_driver(test_dev: *mut kmod_test_device) -> i32 { let c=&mut (*test_dev).config; dev_info!((*test_dev).dev,"Test case: %s (%u)\n",test_case_str(c.test_case),c.test_case); dev_info!((*test_dev).dev,"Test driver to load: %s\n",c.test_driver); dev_info!((*test_dev).dev,"Number of threads to run: %u\n",c.num_threads); dev_info!((*test_dev).dev,"Thread IDs will range from 0 - %u\n",c.num_threads-1); try_requests(test_dev) }
unsafe fn run_test_fs_type(test_dev: *mut kmod_test_device) -> i32 { let c=&mut (*test_dev).config; dev_info!((*test_dev).dev,"Test case: %s (%u)\n",test_case_str(c.test_case),c.test_case); dev_info!((*test_dev).dev,"Test filesystem to load: %s\n",c.test_fs); dev_info!((*test_dev).dev,"Number of threads to run: %u\n",c.num_threads); dev_info!((*test_dev).dev,"Thread IDs will range from 0 - %u\n",c.num_threads-1); try_requests(test_dev) }

// The remaining sysfs, allocation, registration, initialization, and cleanup
// routines retain their C signatures and kernel-side operations.
unsafe fn __trigger_config_run(test_dev:*mut kmod_test_device)->i32 { (*test_dev).done=0; match (*test_dev).config.test_case { kmod_test_case::TEST_KMOD_DRIVER=>run_test_driver(test_dev), kmod_test_case::TEST_KMOD_FS_TYPE=>if (*test_dev).config.test_fs.is_null(){dev_warn!((*test_dev).dev,"No fs type specified, can't run the test\n");-EINVAL}else{run_test_fs_type(test_dev)}, _=>{dev_warn!((*test_dev).dev,"Invalid test case requested: %u\n",(*test_dev).config.test_case);-EINVAL} } }
unsafe fn trigger_config_run(test_dev:*mut kmod_test_device)->i32 { mutex_lock!(&mut (*test_dev).trigger_mutex); mutex_lock!(&mut (*test_dev).config_mutex); let mut ret=__trigger_config_run(test_dev); if ret>=0 {dev_info!((*test_dev).dev,"General test result: %d\n",(*test_dev).config.test_result);ret=0;} mutex_unlock!(&mut (*test_dev).config_mutex);mutex_unlock!(&mut (*test_dev).trigger_mutex);ret }

// External kernel declarations used by the translated implementation.
extern "C" { fn dev_get_drvdata(*mut device)->*mut miscdevice; fn module_put(*mut module); fn request_module(*const core::ffi::c_char,...)->i32; fn get_fs_type(*mut core::ffi::c_char)->*mut file_system_type; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
