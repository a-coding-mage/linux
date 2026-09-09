/* hw_random/core.c: HWRNG core API (Rust translation) */
/* Linux kernel dependencies are supplied by the surrounding build. */

const RNG_MODULE_NAME: &str = "hw_random";
const RNG_BUFFER_SIZE: usize = if 32 > SMP_CACHE_BYTES { 32 } else { SMP_CACHE_BYTES };

static mut current_rng: *mut hwrng = core::ptr::null_mut();
/* the current rng has been explicitly chosen by user via sysfs */
static mut cur_rng_set_by_user: i32 = 0;
static mut hwrng_fill: *mut task_struct = core::ptr::null_mut();
/* list of registered rngs */
static mut rng_list: list_head = LIST_HEAD_INIT;
/* Protects rng_list, hwrng_fill and updating on current_rng */
static mut rng_mutex: mutex = DEFINE_MUTEX_INIT;
/* Protects rng read functions, data_avail, rng_buffer and rng_fillbuf */
static mut reading_mutex: mutex = DEFINE_MUTEX_INIT;
static mut data_avail: i32 = 0;
static mut rng_buffer: *mut u8 = core::ptr::null_mut();
static mut rng_fillbuf: *mut u8 = core::ptr::null_mut();
/* Set to true during suspend-resume by PM notifier, protected by rng_mutex */
static mut hwrng_fillfn_stopped: bool = false;
static mut current_quality: u16 = 0;
static mut default_quality: u16 = 1024;

extern "C" {
    static SMP_CACHE_BYTES: usize;
    static mut rng_dev_groups: *const *const attribute_group;
}

unsafe fn rng_buffer_size() -> usize { RNG_BUFFER_SIZE }

unsafe fn cleanup_rng_work(work: *mut work_struct) {
    let rng = container_of(work, hwrng, cleanup_work);
    mutex_lock(&mut rng_mutex);
    if kref_read(&(*rng).ref_) != 0 { mutex_unlock(&mut rng_mutex); return; }
    if let Some(cleanup) = (*rng).cleanup { cleanup(rng); }
    complete(&mut (*rng).cleanup_done);
    mutex_unlock(&mut rng_mutex);
}

unsafe extern "C" fn cleanup_rng(kref: *mut kref) {
    let rng = container_of(kref, hwrng, ref_);
    schedule_work(&mut (*rng).cleanup_work);
}

unsafe fn set_current_rng(rng: *mut hwrng) -> i32 {
    BUG_ON(!mutex_is_locked(&rng_mutex));
    let err = hwrng_init(rng); if err != 0 { return err; }
    let old_rng = rcu_dereference_protected(current_rng, lockdep_is_held(&rng_mutex));
    rcu_assign_pointer(&mut current_rng, rng);
    if !old_rng.is_null() { synchronize_rcu(); kref_put(&mut (*old_rng).ref_, cleanup_rng); }
    hwrng_start_hwrng_fillfn(); 0
}

unsafe fn drop_current_rng() {
    let rng = rcu_dereference_protected(current_rng, lockdep_is_held(&rng_mutex));
    if rng.is_null() { return; }
    RCU_INIT_POINTER(&mut current_rng, core::ptr::null_mut());
    synchronize_rcu(); hwrng_stop_hwrng_fillfn();
    kref_put(&mut (*rng).ref_, cleanup_rng);
}

unsafe fn get_current_rng_nolock() -> *mut hwrng {
    let rng = rcu_dereference_protected(current_rng, lockdep_is_held(&rng_mutex));
    if !rng.is_null() { kref_get(&mut (*rng).ref_); } rng
}
unsafe fn get_current_rng() -> *mut hwrng {
    rcu_read_lock(); let rng = rcu_dereference(current_rng);
    if !rng.is_null() { kref_get(&mut (*rng).ref_); } rcu_read_unlock(); rng
}
unsafe fn put_rng(rng: *mut hwrng) { if !rng.is_null() { kref_put(&mut (*rng).ref_, cleanup_rng); } }

unsafe fn hwrng_init(rng: *mut hwrng) -> i32 {
    if kref_get_unless_zero(&mut (*rng).ref_) { current_quality = (*rng).quality; return 0; }
    if let Some(init) = (*rng).init { let ret = init(rng); if ret != 0 { return ret; } }
    kref_init(&mut (*rng).ref_); reinit_completion(&mut (*rng).cleanup_done);
    current_quality = (*rng).quality; 0
}

unsafe extern "C" fn rng_dev_open(_inode: *mut inode, filp: *mut file) -> i32 {
    if (*filp).f_mode & FMODE_READ == 0 || (*filp).f_mode & FMODE_WRITE != 0 { return -EINVAL; } 0
}

unsafe fn rng_get_data(rng: *mut hwrng, buffer: *mut u8, size: usize, wait: bool) -> i32 {
    BUG_ON(!mutex_is_locked(&reading_mutex));
    if let Some(read) = (*rng).read {
        let mut err = read(rng, buffer, size, wait);
        if err > 0 && err as usize > size { WARN_ON_ONCE(true); err = size as i32; } return err;
    }
    let present = if let Some(data_present) = (*rng).data_present { data_present(rng, wait) } else { 1 };
    if present != 0 { return (*rng).data_read.unwrap()(rng, buffer as *mut u32); } 0
}

unsafe extern "C" fn rng_dev_read(filp: *mut file, mut buf: *mut u8, mut size: usize, _offp: *mut loff_t) -> isize {
    let mut buffer = [0u8; RNG_BUFFER_SIZE]; let mut ret: isize = 0; let mut err: i32 = 0;
    while size != 0 {
        let rng = get_current_rng(); if rng.is_null() { err = -ENODEV; break; }
        if mutex_lock_interruptible(&mut reading_mutex) != 0 { put_rng(rng); err = -ERESTARTSYS; break; }
        if data_avail == 0 {
            let n = rng_get_data(rng, rng_buffer, rng_buffer_size(), (*filp).f_flags & O_NONBLOCK == 0);
            if n < 0 { mutex_unlock(&mut reading_mutex); put_rng(rng); err = n; break; }
            if n == 0 && (*filp).f_flags & O_NONBLOCK != 0 { mutex_unlock(&mut reading_mutex); put_rng(rng); err = -EAGAIN; break; }
            data_avail = n;
        }
        let len = core::cmp::min(data_avail as usize, size); data_avail -= len as i32;
        core::ptr::copy_nonoverlapping(rng_buffer.add(data_avail as usize), buffer.as_mut_ptr(), len);
        mutex_unlock(&mut reading_mutex); put_rng(rng);
        if len != 0 { if copy_to_user(buf, buffer.as_ptr(), len) != 0 { err = -EFAULT; break; } size -= len; buf = buf.add(len); ret += len as isize; }
        if need_resched() { schedule_timeout_interruptible(1); }
        if signal_pending(current()) { err = -ERESTARTSYS; break; }
    }
    memzero_explicit(buffer.as_mut_ptr(), buffer.len()); if ret != 0 { ret } else { err as isize }
}

unsafe fn enable_best_rng() -> i32 {
    BUG_ON(!mutex_is_locked(&rng_mutex));
    if list_empty(&rng_list) { drop_current_rng(); cur_rng_set_by_user = 0; return 0; }
    let mut new_rng: *mut hwrng = core::ptr::null_mut();
    list_for_each_entry(|rng: *mut hwrng| { if new_rng.is_null() || (*rng).quality > (*new_rng).quality { new_rng = rng; } }, &rng_list, list);
    let cur_rng = rcu_dereference_protected(current_rng, lockdep_is_held(&rng_mutex));
    let ret = if new_rng == cur_rng { 0 } else { set_current_rng(new_rng) }; if ret == 0 { cur_rng_set_by_user = 0; } ret
}

unsafe fn hwrng_start_hwrng_fillfn() {
    lockdep_assert_held(&rng_mutex);
    if hwrng_fillfn_stopped { return; }
    if hwrng_fill.is_null() { let t = kthread_run(hwrng_fillfn, core::ptr::null_mut(), b"hwrng\0".as_ptr()); if IS_ERR(t) { pr_err(b"hwrng_fill thread creation failed\n\0".as_ptr()); hwrng_fill = core::ptr::null_mut(); } else { hwrng_fill = t; } }
}
unsafe fn hwrng_stop_hwrng_fillfn() { lockdep_assert_held(&rng_mutex); if !hwrng_fill.is_null() { kthread_stop(hwrng_fill); hwrng_fill = core::ptr::null_mut(); } }

unsafe extern "C" fn hwrng_fillfn(_unused: *mut core::ffi::c_void) -> i32 {
    let mut entropy_credit: usize = 0;
    while !kthread_should_stop() {
        let rng = get_current_rng(); if rng.is_null() { while !kthread_should_stop() { set_current_state(TASK_INTERRUPTIBLE); if !kthread_should_stop() { schedule(); } } set_current_state(TASK_RUNNING); break; }
        mutex_lock(&mut reading_mutex); let rc = rng_get_data(rng, rng_fillbuf, rng_buffer_size(), true);
        if current_quality != (*rng).quality { (*rng).quality = current_quality; } let quality = (*rng).quality; mutex_unlock(&mut reading_mutex);
        if rc <= 0 { hwrng_msleep(rng, 10000); } put_rng(rng); if rc <= 0 { continue; }
        let entropy = rc as usize * quality as usize * 8 + entropy_credit; if entropy >> 10 == 0 { entropy_credit = entropy; }
        add_hwgenerator_randomness(rng_fillbuf as *mut core::ffi::c_void, rc as usize, entropy >> 10, true);
    } 0
}

unsafe extern "C" fn hwrng_pm_notifier(_nb: *mut notifier_block, action: usize, _data: *mut core::ffi::c_void) -> i32 {
    match action { PM_SUSPEND_PREPARE | PM_HIBERNATION_PREPARE | PM_RESTORE_PREPARE => { mutex_lock(&mut rng_mutex); hwrng_stop_hwrng_fillfn(); hwrng_fillfn_stopped = true; mutex_unlock(&mut rng_mutex); }, PM_POST_SUSPEND | PM_POST_HIBERNATION | PM_POST_RESTORE => { mutex_lock(&mut rng_mutex); hwrng_fillfn_stopped = false; if !rcu_access_pointer(current_rng).is_null() { hwrng_start_hwrng_fillfn(); } mutex_unlock(&mut rng_mutex); }, _ => {} } NOTIFY_DONE
}

#[no_mangle] pub unsafe extern "C" fn hwrng_register(rng: *mut hwrng) -> i32 { if rng.is_null() || (*rng).name.is_null() || ((*rng).data_read.is_none() && (*rng).read.is_none()) { return -EINVAL; } mutex_lock(&mut rng_mutex); list_add_tail(&mut (*rng).list, &mut rng_list); (*rng).quality = core::cmp::min(default_quality, core::cmp::min(1024, if (*rng).quality == 0 { 1024 } else { (*rng).quality })); let ret = if cur_rng_set_by_user == 0 && (current_rng.is_null() || (*rng).quality > (*current_rng).quality) { set_current_rng(rng) } else { 0 }; mutex_unlock(&mut rng_mutex); ret }
#[no_mangle] pub unsafe extern "C" fn hwrng_unregister(rng: *mut hwrng) { mutex_lock(&mut rng_mutex); list_del(&mut (*rng).list); complete_all(&mut (*rng).dying); if current_rng == rng { if enable_best_rng() != 0 { drop_current_rng(); cur_rng_set_by_user = 0; } } mutex_unlock(&mut rng_mutex); wait_for_completion(&mut (*rng).cleanup_done); }
#[no_mangle] pub unsafe extern "C" fn devm_hwrng_register(dev: *mut device, rng: *mut hwrng) -> i32 { let p = devres_alloc(devm_hwrng_release, core::mem::size_of::<*mut hwrng>(), GFP_KERNEL); if p.is_null() { return -ENOMEM; } let e = hwrng_register(rng); if e != 0 { devres_free(p); return e; } *(p as *mut *mut hwrng) = rng; devres_add(dev, p); 0 }
#[no_mangle] pub unsafe extern "C" fn devm_hwrng_unregister(dev: *mut device, rng: *mut hwrng) { devres_release(dev, devm_hwrng_release, devm_hwrng_match, rng as *mut core::ffi::c_void); }
unsafe extern "C" fn devm_hwrng_release(_dev: *mut device, res: *mut core::ffi::c_void) { hwrng_unregister(*(res as *mut *mut hwrng)); }
unsafe extern "C" fn devm_hwrng_match(_dev: *mut device, res: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> i32 { if res.is_null() || *(res as *mut *mut hwrng) != data as *mut hwrng { 0 } else { 1 } }
#[no_mangle] pub unsafe extern "C" fn hwrng_msleep(rng: *mut hwrng, msecs: u32) -> isize { wait_for_completion_interruptible_timeout(&mut (*rng).dying, msecs_to_jiffies(msecs) + 1) }
#[no_mangle] pub unsafe extern "C" fn hwrng_yield(rng: *mut hwrng) -> isize { wait_for_completion_interruptible_timeout(&mut (*rng).dying, 1) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
