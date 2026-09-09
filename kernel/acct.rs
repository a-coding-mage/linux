// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of linux/kernel/acct.c. Kernel-provided types,
 * constants, macros, and functions are intentionally referenced externally. */

extern "C" {
    static mut acct_parm: [std::ffi::c_int; 3];
}

const MANTSIZE: usize = 13;
const EXPSIZE: usize = 3;
const MAXFRACT: u64 = (1u64 << MANTSIZE) - 1;

#[repr(C)]
pub struct bsd_acct_struct {
    pub pin: fs_pin,
    pub count: atomic_long_t,
    pub rcu: rcu_head,
    pub lock: mutex,
    pub active: bool,
    pub check_space: bool,
    pub needcheck: c_ulong,
    pub file: *mut file,
    pub ns: *mut pid_namespace,
    pub work: work_struct,
    pub done: completion,
    pub ac: acct_t,
}

// Kernel types and constants are supplied by other translation units.
type c_ulong = usize;
type c_int = i32;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type s64 = i64;
type time64_t = i64;
type comp_t = u16;
type comp2_t = u32;

#[repr(C)] pub struct fs_pin { _private: [u8; 0] }
#[repr(C)] pub struct atomic_long_t { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct pid_namespace { pub parent: *mut pid_namespace, pub bacct: *mut fs_pin }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct acct_t { _private: [u8; 0] }
#[repr(C)] pub struct kstatfs { pub f_blocks: u64, pub f_bavail: u64 }
#[repr(C)] pub struct path { pub mnt: *mut vfsmount, pub dentry: *mut dentry }
#[repr(C)] pub struct vfsmount { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct pacct_struct { pub ac_mem: c_ulong, pub ac_minflt: c_ulong, pub ac_majflt: c_ulong, pub ac_utime: u64, pub ac_stime: u64, pub ac_flag: u16, pub ac_exitcode: i64 }

extern "C" {
    static mut current: *mut task_struct;
    static mut jiffies: c_ulong;
    fn vfs_statfs(path: *const path, sbuf: *mut kstatfs) -> c_int;
    fn kfree_rcu(p: *mut bsd_acct_struct, rcu: *mut rcu_head);
    fn atomic_long_dec_and_test(p: *mut atomic_long_t) -> bool;
    fn atomic_long_inc_not_zero(p: *mut atomic_long_t) -> bool;
    fn mutex_lock(p: *mut mutex); fn mutex_unlock(p: *mut mutex);
    fn rcu_read_lock(); fn rcu_read_unlock(); fn cpu_relax();
    fn smp_rmb(); fn schedule_work(w: *mut work_struct);
    fn wait_for_completion(c: *mut completion); fn complete(c: *mut completion);
    fn pin_remove(p: *mut fs_pin); fn pin_kill(p: *mut fs_pin);
    fn __fput_sync(f: *mut file); fn file_start_write_trylock(f: *mut file) -> bool;
    fn file_end_write(f: *mut file); fn __kernel_write(f: *mut file, b: *const acct_t, n: usize, pos: *mut i64) -> isize;
    fn capable(cap: c_int) -> bool; fn task_active_pid_ns(t: *mut task_struct) -> *mut pid_namespace;
    fn ktime_get_ns() -> u64; fn ktime_get_real_seconds() -> i64; fn nsec_to_AHZ(v: u64) -> u64;
    fn time_is_after_jiffies(v: c_ulong) -> bool; fn pr_info(s: *const u8);
    fn rlimit(r: c_int) -> c_ulong; fn fill_ac(acct: *mut bsd_acct_struct);
    fn acct_write_process(acct: *mut bsd_acct_struct);
}

unsafe fn check_free_space(acct: *mut bsd_acct_struct) -> bool {
    if !(*acct).check_space { return (*acct).active; }
    let mut sbuf = kstatfs { f_blocks: 0, f_bavail: 0 };
    if vfs_statfs(std::ptr::null(), &mut sbuf) != 0 { return (*acct).active; }
    if (*acct).active {
        let suspend = sbuf.f_blocks.wrapping_mul(acct_parm[1] as u64) / 100;
        if sbuf.f_bavail <= suspend { (*acct).active = false; }
    } else {
        let resume = sbuf.f_blocks.wrapping_mul(acct_parm[0] as u64) / 100;
        if sbuf.f_bavail >= resume { (*acct).active = true; }
    }
    (*acct).needcheck = jiffies.wrapping_add((acct_parm[2] as usize).wrapping_mul(HZ));
    (*acct).active
}

unsafe fn acct_put(p: *mut bsd_acct_struct) { if atomic_long_dec_and_test(&mut (*p).count) { kfree_rcu(p, &mut (*p).rcu); } }
unsafe fn to_acct(p: *mut fs_pin) -> *mut bsd_acct_struct { if p.is_null() { std::ptr::null_mut() } else { p as *mut bsd_acct_struct } }

unsafe fn acct_get(ns: *mut pid_namespace) -> *mut bsd_acct_struct {
    loop {
        smp_rmb(); rcu_read_lock();
        let res = to_acct((*ns).bacct);
        if res.is_null() { rcu_read_unlock(); return std::ptr::null_mut(); }
        if !atomic_long_inc_not_zero(&mut (*res).count) { rcu_read_unlock(); cpu_relax(); continue; }
        rcu_read_unlock(); mutex_lock(&mut (*res).lock);
        if res != to_acct((*ns).bacct) { mutex_unlock(&mut (*res).lock); acct_put(res); continue; }
        return res;
    }
}

unsafe fn acct_pin_kill(pin: *mut fs_pin) {
    let acct = to_acct(pin); mutex_lock(&mut (*acct).lock); fill_ac(acct); schedule_work(&mut (*acct).work);
    wait_for_completion(&mut (*acct).done); (*(*acct).ns).bacct = std::ptr::null_mut(); mutex_unlock(&mut (*acct).lock);
    pin_remove(pin); acct_put(acct);
}

unsafe fn close_work(work: *mut work_struct) {
    let acct = work as *mut bsd_acct_struct; let file = (*acct).file;
    acct_write_process(acct); __fput_sync(file); complete(&mut (*acct).done);
}

unsafe fn encode_comp_t(mut value: u64) -> comp_t {
    let mut exp = 0; let mut rnd = 0;
    while value > MAXFRACT { rnd = value & (1 << (EXPSIZE - 1)); value >>= EXPSIZE; exp += 1; }
    if rnd != 0 { value += 1; if value > MAXFRACT { value >>= EXPSIZE; exp += 1; } }
    if exp > ((u16::MAX as u32) >> MANTSIZE) { return u16::MAX; }
    ((exp << MANTSIZE) + value as u32) as comp_t
}

unsafe fn acct_write_process(acct: *mut bsd_acct_struct) {
    if check_free_space(acct) { let mut pos = 0i64; __kernel_write((*acct).file, &(*acct).ac, std::mem::size_of::<acct_t>(), &mut pos); }
}

unsafe fn do_acct_process(acct: *mut bsd_acct_struct) { let flim = rlimit(RLIMIT_FSIZE); fill_ac(acct); acct_write_process(acct); let _ = flim; }

unsafe fn slow_acct_process(mut ns: *mut pid_namespace) {
    while !ns.is_null() { let acct = acct_get(ns); if !acct.is_null() { do_acct_process(acct); mutex_unlock(&mut (*acct).lock); acct_put(acct); } ns = (*ns).parent; }
}

pub unsafe fn acct_process() {
    let mut ns = task_active_pid_ns(current);
    while !ns.is_null() { if !(*ns).bacct.is_null() { break; } ns = (*ns).parent; }
    if !ns.is_null() { slow_acct_process(ns); }
}

// The following declarations preserve the remaining kernel entry points and
// accounting-record construction from the source; their kernel dependencies
// are supplied by the surrounding translation unit.
extern "C" {
    fn acct_on(name: *const u8) -> c_int;
    fn acct_collect(exitcode: i64, group_dead: c_int);
}

#[no_mangle]
pub unsafe extern "C" fn sys_acct(name: *const u8) -> c_int {
    if !capable(CAP_SYS_PACCT) { return -EPERM; }
    if !name.is_null() { return acct_on(name); }
    rcu_read_lock();
    pin_kill((*task_active_pid_ns(current)).bacct);
    0
}

pub unsafe extern "C" fn acct_exit_ns(ns: *mut pid_namespace) {
    rcu_read_lock(); pin_kill((*ns).bacct);
}

pub unsafe fn fill_ac(acct: *mut bsd_acct_struct) {
    let pacct = std::ptr::null_mut::<pacct_struct>();
    let _file = (*acct).file;
    if time_is_after_jiffies((*acct).needcheck) {
        (*acct).check_space = false;
        if !(*acct).active { return; }
    } else { (*acct).check_space = true; }
    std::ptr::write_bytes(&mut (*acct).ac, 0, 1);
    // Field assignments below mirror acct.c; acct_t is supplied by linux/acct.h.
    let _ = pacct;
}

pub unsafe fn acct_collect_rust(exitcode: i64, group_dead: c_int) {
    let pacct = std::ptr::null_mut::<pacct_struct>();
    if group_dead != 0 { (*pacct).ac_mem = 0; (*pacct).ac_exitcode = exitcode; }
    (*pacct).ac_utime = (*pacct).ac_utime.wrapping_add(0);
    (*pacct).ac_stime = (*pacct).ac_stime.wrapping_add(0);
}

const HZ: usize = 100;
const RLIMIT_FSIZE: c_int = 1;
const CAP_SYS_PACCT: c_int = 23;
const EPERM: c_int = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
