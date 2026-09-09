// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel headers are referenced externally.

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct static_key_true {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
struct once_work {
    work: work_struct,
    key: *mut static_key_true,
    module: *mut module,
}

extern "C" {
    fn static_key_enabled(key: *mut static_key_true) -> bool;
    fn static_branch_disable(key: *mut static_key_true);
    fn module_put(module: *mut module);
    fn kfree(ptr: *mut core::ffi::c_void);
    fn kmalloc_obj(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn init_work(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn schedule_work(work: *mut work_struct);
    fn module_get(module: *mut module);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
}

static mut once_lock: spinlock_t = spinlock_t { _private: [] };

unsafe extern "C" fn once_deferred(w: *mut work_struct) {
    let work = (w as *mut u8).sub(core::mem::offset_of!(once_work, work)) as *mut once_work;
    debug_assert!(static_key_enabled((*work).key));
    static_branch_disable((*work).key);
    module_put((*work).module);
    kfree(work as *mut core::ffi::c_void);
}

unsafe fn once_disable_jump(key: *mut static_key_true, module: *mut module) {
    let w = kmalloc_obj(core::mem::size_of::<once_work>(), 0) as *mut once_work;
    if w.is_null() {
        return;
    }

    init_work(&mut (*w).work, once_deferred);
    (*w).key = key;
    (*w).module = module;
    module_get(module);
    schedule_work(&mut (*w).work);
}

pub unsafe fn __do_once_start(done: *mut bool, flags: *mut usize) -> bool {
    spin_lock_irqsave(&mut once_lock, flags);
    if *done {
        spin_unlock_irqrestore(&mut once_lock, *flags);
        /* Keep sparse happy by restoring an even lock count on
         * this lock. In case we return here, we don't call into
         * __do_once_done but return early in the DO_ONCE() macro.
         */
        return false;
    }

    true
}

pub unsafe fn __do_once_done(
    done: *mut bool,
    once_key: *mut static_key_true,
    flags: *mut usize,
    module: *mut module,
) {
    *done = true;
    spin_unlock_irqrestore(&mut once_lock, *flags);
    once_disable_jump(once_key, module);
}

static mut once_mutex: mutex = mutex { _private: [] };

pub unsafe fn __do_once_sleepable_start(done: *mut bool) -> bool {
    mutex_lock(&mut once_mutex);
    if *done {
        mutex_unlock(&mut once_mutex);
        /* Keep sparse happy by restoring an even lock count on
         * this mutex. In case we return here, we don't call into
         * __do_once_done but return early in the DO_ONCE_SLEEPABLE() macro.
         */
        return false;
    }

    true
}

pub unsafe fn __do_once_sleepable_done(
    done: *mut bool,
    once_key: *mut static_key_true,
    _module: *mut module,
) {
    *done = true;
    mutex_unlock(&mut once_mutex);
    static_branch_disable(once_key);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
