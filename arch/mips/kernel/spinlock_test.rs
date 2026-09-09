// SPDX-License-Identifier: GPL-2.0
// Translated from the Linux kernel implementation. The included kernel types,
// functions, and debugfs macros are supplied by the surrounding kernel crate.

use core::ffi::c_void;

extern "C" {
    fn ktime_get() -> ktime_t;
    fn ktime_us_delta(finish: ktime_t, start: ktime_t) -> u64;
    fn raw_spin_lock(lock: *mut raw_spinlock_t);
    fn raw_spin_unlock(lock: *mut raw_spinlock_t);
    fn atomic_dec(value: *mut atomic_t);
    fn atomic_read(value: *const atomic_t) -> i32;
    fn atomic_set(value: *mut atomic_t, number: i32);
    fn kthread_run(
        threadfn: unsafe extern "C" fn(*mut c_void) -> i32,
        data: *mut c_void,
        name: *const u8,
    ) -> *mut task_struct;
    fn debugfs_create_file_unsafe(
        name: *const u8,
        mode: u32,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const c_void,
    ) -> *mut dentry;
}

#[repr(C)]
pub struct raw_spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

type ktime_t = i64;

extern "C" {
    static mut mips_debugfs_dir: *mut dentry;
}

const S_IRUGO: u32 = 0o444;

#[repr(C)]
struct spin_multi_state {
    lock: raw_spinlock_t,
    start_wait: atomic_t,
    enter_wait: atomic_t,
    exit_wait: atomic_t,
    loops: i32,
}

#[repr(C)]
struct spin_multi_per_thread {
    state: *mut spin_multi_state,
    start: ktime_t,
}

unsafe extern "C" fn ss_get(_data: *mut c_void, val: *mut u64) -> i32 {
    let mut loops: i32 = 1_000_000;
    let mut cont: i32 = 1;
    let mut ss_spin = raw_spinlock_t { _private: [] };

    let start = ktime_get();

    while cont != 0 {
        raw_spin_lock(&mut ss_spin);
        loops -= 1;
        if loops == 0 {
            cont = 0;
        }
        raw_spin_unlock(&mut ss_spin);
    }

    let finish = ktime_get();
    *val = ktime_us_delta(finish, start);
    0
}

// DEFINE_DEBUGFS_ATTRIBUTE(fops_ss, ss_get, NULL, "%llu\\n");
static fops_ss: () = ();

unsafe extern "C" fn multi_other(data: *mut c_void) -> i32 {
    let pt = &mut *(data as *mut spin_multi_per_thread);
    let s = &mut *pt.state;
    let mut loops = s.loops;
    let mut cont: i32 = 1;

    atomic_dec(&mut s.enter_wait);

    while atomic_read(&s.enter_wait) != 0 {
        // spin
    }

    pt.start = ktime_get();
    atomic_dec(&mut s.start_wait);

    while atomic_read(&s.start_wait) != 0 {
        // spin
    }

    while cont != 0 {
        raw_spin_lock(&mut s.lock);
        loops -= 1;
        if loops == 0 {
            cont = 0;
        }
        raw_spin_unlock(&mut s.lock);
    }

    atomic_dec(&mut s.exit_wait);
    while atomic_read(&s.exit_wait) != 0 {
        // spin
    }
    0
}

unsafe extern "C" fn multi_get(_data: *mut c_void, val: *mut u64) -> i32 {
    let mut ms = spin_multi_state {
        lock: raw_spinlock_t { _private: [] },
        start_wait: atomic_t { _private: [] },
        enter_wait: atomic_t { _private: [] },
        exit_wait: atomic_t { _private: [] },
        loops: 1_000_000,
    };
    let mut t1 = spin_multi_per_thread { state: &mut ms, start: 0 };
    let mut t2 = spin_multi_per_thread { state: &mut ms, start: 0 };

    atomic_set(&mut ms.start_wait, 2);
    atomic_set(&mut ms.enter_wait, 2);
    atomic_set(&mut ms.exit_wait, 2);

    kthread_run(multi_other, &mut t2 as *mut _ as *mut c_void, b"multi_get\0".as_ptr());
    multi_other(&mut t1 as *mut _ as *mut c_void);

    let finish = ktime_get();
    *val = ktime_us_delta(finish, t1.start);
    0
}

// DEFINE_DEBUGFS_ATTRIBUTE(fops_multi, multi_get, NULL, "%llu\\n");
static fops_multi: () = ();

unsafe extern "C" fn spinlock_test() -> i32 {
    debugfs_create_file_unsafe(
        b"spin_single\0".as_ptr(),
        S_IRUGO,
        mips_debugfs_dir,
        core::ptr::null_mut(),
        &fops_ss as *const _ as *const c_void,
    );
    debugfs_create_file_unsafe(
        b"spin_multi\0".as_ptr(),
        S_IRUGO,
        mips_debugfs_dir,
        core::ptr::null_mut(),
        &fops_multi as *const _ as *const c_void,
    );
    0
}

// device_initcall(spinlock_test);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
