// SPDX-License-Identifier: GPL-2.0
/* Test module to generate lockups */

// Linux kernel includes and module parameter macros are supplied by the kernel bindings.

use core::ffi::c_void;

#[repr(C)]
pub struct file;
#[repr(C)]
pub struct inode;
#[repr(C)]
pub struct page { pub lru: list_head }
#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)]
pub struct work_struct;
#[repr(C)]
pub struct task_struct { pub pid: i32, pub mm: *mut c_void, pub in_iowait: u8 }

type U64 = u64;
type S64 = i64;
type GfpT = u32;

static mut time_secs: u32 = 0;
static mut time_nsecs: u32 = 0;
static mut cooldown_secs: u32 = 0;
static mut cooldown_nsecs: u32 = 0;
static mut iterations: u32 = 1;
static mut all_cpus: bool = false;
static mut wait_state: i32 = 0;
static mut state: *mut u8 = b"R\0" as *const u8 as *mut u8;
static mut use_hrtimer: bool = false;
static mut iowait: bool = false;
static mut lock_read: bool = false;
static mut lock_single: bool = false;
static mut reacquire_locks: bool = false;
static mut touch_softlockup: bool = false;
static mut touch_hardlockup: bool = false;
static mut call_cond_resched: bool = false;
static mut measure_lock_wait: bool = false;
static mut lock_wait_threshold: usize = usize::MAX;
static mut test_disable_irq: bool = false;
static mut disable_softirq: bool = false;
static mut disable_preempt: bool = false;
static mut lock_rcu: bool = false;
static mut lock_mmap_sem: bool = false;
static mut lock_rwsem_ptr: usize = 0;
static mut lock_mutex_ptr: usize = 0;
static mut lock_spinlock_ptr: usize = 0;
static mut lock_rwlock_ptr: usize = 0;
static mut alloc_pages_nr: u32 = 0;
static mut alloc_pages_order: u32 = 0;
static mut alloc_pages_gfp: GfpT = 0;
static mut alloc_pages_atomic: bool = false;
static mut reallocate_pages: bool = false;
pub static mut test_file: *mut file = core::ptr::null_mut();
static mut test_inode: *mut inode = core::ptr::null_mut();
static mut test_file_path: [u8; 256] = [0; 256];
static mut test_lock_inode: bool = false;
static mut test_lock_mapping: bool = false;
static mut test_lock_sb_umount: bool = false;
static mut alloc_pages_failed: i32 = 0;
static mut max_lock_wait: i64 = 0;
static mut main_task: *mut task_struct = core::ptr::null_mut();
static mut master_cpu: i32 = 0;

extern "C" {
    fn local_clock() -> U64;
    fn raw_smp_processor_id() -> i32;
    fn signal_pending(task: *mut task_struct) -> bool;
    fn pr_notice(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
    fn pr_notice_ratelimited(fmt: *const u8, ...);
    fn mutex_lock(p: *mut c_void); fn mutex_unlock(p: *mut c_void);
    fn down_read(p: *mut c_void); fn down_write(p: *mut c_void);
    fn up_read(p: *mut c_void); fn up_write(p: *mut c_void);
    fn spin_lock(p: *mut c_void); fn spin_unlock(p: *mut c_void);
    fn read_lock(p: *mut c_void); fn read_unlock(p: *mut c_void);
    fn write_lock(p: *mut c_void); fn write_unlock(p: *mut c_void);
    fn local_irq_disable(); fn local_irq_enable();
    fn local_bh_disable(); fn local_bh_enable();
    fn preempt_disable(); fn preempt_enable();
    fn rcu_read_lock(); fn rcu_read_unlock();
    fn mmap_read_lock(mm: *mut c_void); fn mmap_read_unlock(mm: *mut c_void);
    fn mmap_write_lock(mm: *mut c_void); fn mmap_write_unlock(mm: *mut c_void);
    fn mdelay(ms: u32); fn ndelay(ns: u32); fn schedule_hrtimeout(t: *mut c_void, mode: i32); fn schedule_timeout(t: i64);
    fn touch_softlockup_watchdog(); fn touch_nmi_watchdog(); fn cond_resched();
    fn alloc_pages(gfp: GfpT, order: u32) -> *mut page; fn __free_pages(p: *mut page, order: u32);
    fn list_add(n: *mut list_head, h: *mut list_head); fn init_list_head(h: *mut list_head);
    fn atomic_inc(v: *mut i32); fn atomic64_read(v: *mut i64) -> i64; fn atomic64_cmpxchg(v: *mut i64, old: i64, new: i64) -> i64;
    fn access_ok(p: *const c_void, size: usize) -> bool; fn get_kernel_nofault(dst: *mut c_void, src: *const c_void) -> i32;
    fn filp_open(path: *const u8, flags: i32, mode: i32) -> *mut file; fn file_inode(f: *mut file) -> *mut inode;
    fn fput(f: *mut file); fn ptr_err(p: *mut file) -> i64;
}

unsafe fn test_lock(master: bool, verbose: bool) {
    let wait_start = if measure_lock_wait { local_clock() } else { 0 };
    if lock_mutex_ptr != 0 && master { if verbose { pr_notice(b"lock mutex %ps\n\0".as_ptr()); } mutex_lock(lock_mutex_ptr as *mut c_void); }
    if lock_rwsem_ptr != 0 && master { if verbose { pr_notice(b"lock rw_semaphore %ps\n\0".as_ptr()); } if lock_read { down_read(lock_rwsem_ptr as *mut c_void); } else { down_write(lock_rwsem_ptr as *mut c_void); } }
    if lock_mmap_sem && master { if lock_read { mmap_read_lock((*main_task).mm); } else { mmap_write_lock((*main_task).mm); } }
    if test_disable_irq { local_irq_disable(); } if disable_softirq { local_bh_disable(); } if disable_preempt { preempt_disable(); } if lock_rcu { rcu_read_lock(); }
    if lock_spinlock_ptr != 0 && master { spin_lock(lock_spinlock_ptr as *mut c_void); }
    if lock_rwlock_ptr != 0 && master { if lock_read { read_lock(lock_rwlock_ptr as *mut c_void); } else { write_lock(lock_rwlock_ptr as *mut c_void); } }
    if measure_lock_wait { let cur_wait = local_clock() as i64 - wait_start as i64; let mut max_wait = atomic64_read(&mut max_lock_wait); loop { if cur_wait < max_wait { break; } max_wait = atomic64_cmpxchg(&mut max_lock_wait, max_wait, cur_wait); if max_wait == cur_wait { break; } } if cur_wait > lock_wait_threshold as i64 { pr_notice_ratelimited(b"lock wait %lld ns\n\0".as_ptr(), cur_wait); } }
}

unsafe fn test_unlock(master: bool, verbose: bool) {
    if lock_rwlock_ptr != 0 && master { if lock_read { read_unlock(lock_rwlock_ptr as *mut c_void); } else { write_unlock(lock_rwlock_ptr as *mut c_void); } }
    if lock_spinlock_ptr != 0 && master { spin_unlock(lock_spinlock_ptr as *mut c_void); }
    if lock_rcu { rcu_read_unlock(); } if disable_preempt { preempt_enable(); } if disable_softirq { local_bh_enable(); } if test_disable_irq { local_irq_enable(); }
    if lock_mmap_sem && master { if lock_read { mmap_read_unlock((*main_task).mm); } else { mmap_write_unlock((*main_task).mm); } }
    if lock_rwsem_ptr != 0 && master { if lock_read { up_read(lock_rwsem_ptr as *mut c_void); } else { up_write(lock_rwsem_ptr as *mut c_void); } }
    if lock_mutex_ptr != 0 && master { mutex_unlock(lock_mutex_ptr as *mut c_void); }
    let _ = verbose;
}

unsafe fn test_alloc_pages(pages: *mut list_head) { for _ in 0..alloc_pages_nr { let page = alloc_pages(alloc_pages_gfp, alloc_pages_order); if page.is_null() { atomic_inc(&mut alloc_pages_failed); break; } list_add(&mut (*page).lru, pages); } }
unsafe fn test_free_pages(pages: *mut list_head) { init_list_head(pages); }
unsafe fn test_wait(secs: u32, nsecs: u32) { if wait_state == 0 { if secs != 0 { mdelay(secs.wrapping_mul(1000)); } if nsecs != 0 { ndelay(nsecs); } return; } if use_hrtimer { schedule_hrtimeout(core::ptr::null_mut(), 0); } else { schedule_timeout((secs as i64).wrapping_mul(100) + nsecs as i64); } }

unsafe fn test_lockup(master: bool) {
    let lockup_start = local_clock(); let mut iter = 0; let mut pages = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
    pr_notice(b"Start on CPU%d\n\0".as_ptr(), raw_smp_processor_id()); test_lock(master, true); test_alloc_pages(&mut pages);
    while { iter += 1; iter <= iterations && !signal_pending(main_task) } { if iowait { (*main_task).in_iowait = 1; } test_wait(time_secs, time_nsecs); if iowait { (*main_task).in_iowait = 0; } if reallocate_pages { test_free_pages(&mut pages); } if reacquire_locks { test_unlock(master, false); } if touch_softlockup { touch_softlockup_watchdog(); } if touch_hardlockup { touch_nmi_watchdog(); } if call_cond_resched { cond_resched(); } test_wait(cooldown_secs, cooldown_nsecs); if reacquire_locks { test_lock(master, false); } if reallocate_pages { test_alloc_pages(&mut pages); } }
    pr_notice(b"Finish on CPU%d in %lld ns\n\0".as_ptr(), raw_smp_processor_id(), local_clock().wrapping_sub(lockup_start)); test_free_pages(&mut pages); test_unlock(master, true);
}

unsafe fn test_kernel_ptr(addr: usize, size: i32) -> bool { if addr == 0 { return false; } let ptr = addr as *const c_void; let mut buf = 0u8; if access_ok(ptr, 1) || access_ok((addr.wrapping_add(size as usize - 1)) as *const c_void, 1) { pr_err(b"user space ptr invalid in kernel: %#lx\n\0".as_ptr(), addr); return true; } if get_kernel_nofault(&mut buf as *mut u8 as *mut c_void, ptr) != 0 || get_kernel_nofault(&mut buf as *mut u8 as *mut c_void, (addr.wrapping_add(size as usize - 1)) as *const c_void) != 0 { pr_err(b"invalid kernel ptr: %#lx\n\0".as_ptr(), addr); return true; } false }

// CONFIG_DEBUG_SPINLOCK/CONFIG_PREEMPT_RT conditional magic checks are preserved by the source-level dependency intent.
#[allow(dead_code)]
unsafe fn test_lockup_init() -> i32 {
    main_task = core::ptr::null_mut();
    let c = *state as char; wait_state = match c { 'S' => 1, 'D' => 2, 'K' => 3, 'R' => 0, _ => return -22 };
    if alloc_pages_atomic { alloc_pages_gfp = 0; }
    if test_kernel_ptr(lock_spinlock_ptr, 1) || test_kernel_ptr(lock_rwlock_ptr, 1) || test_kernel_ptr(lock_mutex_ptr, 1) || test_kernel_ptr(lock_rwsem_ptr, 1) { return -22; }
    if all_cpus { preempt_disable(); master_cpu = raw_smp_processor_id(); preempt_enable(); } else { test_lockup(true); }
    if test_file != core::ptr::null_mut() { fput(test_file); } if signal_pending(main_task) { -4 } else { -11 }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
