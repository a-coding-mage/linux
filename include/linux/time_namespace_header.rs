/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers are intentionally
// referenced but not defined here.

pub struct user_namespace;
extern "C" {
    pub static mut init_user_ns: user_namespace;
}

pub struct seq_file;
pub struct vm_area_struct;

#[repr(C)]
pub struct timens_offsets {
    pub monotonic: timespec64,
    pub boottime: timespec64,
}

#[repr(C)]
pub struct time_namespace {
    pub user_ns: *mut user_namespace,
    pub ucounts: *mut ucounts,
    pub ns: ns_common,
    pub offsets: timens_offsets,
    #[cfg(CONFIG_TIME_NS_VDSO)]
    pub vvar_page: *mut page,
    // If set prevents changing offsets after any task joined namespace.
    pub frozen_offsets: bool,
}

extern "C" {
    pub static mut init_time_ns: time_namespace;
}

#[cfg(CONFIG_TIME_NS)]
#[inline]
pub unsafe fn to_time_ns(ns: *mut ns_common) -> *mut time_namespace {
    // Equivalent to container_of(ns, struct time_namespace, ns).
    (ns as *mut u8).sub(core::mem::offset_of!(time_namespace, ns)) as *mut time_namespace
}

#[cfg(CONFIG_TIME_NS)]
pub unsafe fn time_ns_init();

#[cfg(CONFIG_TIME_NS)]
#[inline]
pub unsafe fn get_time_ns(ns: *mut time_namespace) -> *mut time_namespace {
    ns_ref_inc(ns);
    ns
}

#[cfg(CONFIG_TIME_NS)]
extern "C" {
    pub fn copy_time_ns(
        flags: u64,
        user_ns: *mut user_namespace,
        old_ns: *mut time_namespace,
    ) -> *mut time_namespace;
    pub fn free_time_ns(ns: *mut time_namespace);
    pub fn timens_on_fork(nsproxy: *mut nsproxy, tsk: *mut task_struct);
}

#[cfg(CONFIG_TIME_NS)]
#[inline]
pub unsafe fn put_time_ns(ns: *mut time_namespace) {
    if ns_ref_put(ns) {
        free_time_ns(ns);
    }
}

#[cfg(CONFIG_TIME_NS)]
extern "C" {
    pub fn proc_timens_show_offsets(p: *mut task_struct, m: *mut seq_file);
}

#[repr(C)]
pub struct proc_timens_offset {
    pub clockid: i32,
    pub val: timespec64,
}

#[cfg(CONFIG_TIME_NS)]
extern "C" {
    pub fn proc_timens_set_offset(
        file: *mut file,
        p: *mut task_struct,
        offsets: *mut proc_timens_offset,
        n: i32,
    ) -> i32;
}

#[cfg(CONFIG_TIME_NS)]
#[inline]
pub unsafe fn timens_add_monotonic(ts: *mut timespec64) {
    let ns_offsets = &mut (*(*current).nsproxy).time_ns.offsets;
    *ts = timespec64_add(*ts, ns_offsets.monotonic);
}

#[cfg(CONFIG_TIME_NS)]
#[inline]
pub unsafe fn timens_add_boottime(ts: *mut timespec64) {
    let ns_offsets = &mut (*(*current).nsproxy).time_ns.offsets;
    *ts = timespec64_add(*ts, ns_offsets.boottime);
}

#[cfg(CONFIG_TIME_NS)]
#[inline]
pub unsafe fn timens_add_boottime_ns(nsec: u64) -> u64 {
    let ns_offsets = &mut (*(*current).nsproxy).time_ns.offsets;
    nsec.wrapping_add(timespec64_to_ns(&ns_offsets.boottime))
}

#[cfg(CONFIG_TIME_NS)]
#[inline]
pub unsafe fn timens_sub_boottime(ts: *mut timespec64) {
    let ns_offsets = &mut (*(*current).nsproxy).time_ns.offsets;
    *ts = timespec64_sub(*ts, ns_offsets.boottime);
}

#[cfg(CONFIG_TIME_NS)]
extern "C" {
    pub fn do_timens_ktime_to_host(
        clockid: clockid_t,
        tim: ktime_t,
        offsets: *mut timens_offsets,
    ) -> ktime_t;
}

#[cfg(CONFIG_TIME_NS)]
#[inline]
pub unsafe fn timens_ktime_to_host(clockid: clockid_t, tim: ktime_t) -> ktime_t {
    let ns = (*current).nsproxy.time_ns;
    if likely(ns == &raw mut init_time_ns) {
        return tim;
    }
    do_timens_ktime_to_host(clockid, tim, &mut (*ns).offsets)
}

#[cfg(not(CONFIG_TIME_NS))]
#[inline]
pub unsafe fn time_ns_init() {}

#[cfg(not(CONFIG_TIME_NS))]
#[inline]
pub unsafe fn get_time_ns(_ns: *mut time_namespace) -> *mut time_namespace {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_TIME_NS))]
#[inline]
pub unsafe fn put_time_ns(_ns: *mut time_namespace) {}

#[cfg(not(CONFIG_TIME_NS))]
#[inline]
pub unsafe fn copy_time_ns(
    flags: u64,
    _user_ns: *mut user_namespace,
    old_ns: *mut time_namespace,
) -> *mut time_namespace {
    if flags & CLONE_NEWTIME != 0 {
        return ERR_PTR(-EINVAL);
    }
    old_ns
}

#[cfg(not(CONFIG_TIME_NS))]
#[inline]
pub unsafe fn timens_on_fork(_nsproxy: *mut nsproxy, _tsk: *mut task_struct) {}

#[cfg(not(CONFIG_TIME_NS))]
#[inline]
pub unsafe fn timens_add_monotonic(_ts: *mut timespec64) {}

#[cfg(not(CONFIG_TIME_NS))]
#[inline]
pub unsafe fn timens_add_boottime(_ts: *mut timespec64) {}

#[cfg(not(CONFIG_TIME_NS))]
#[inline]
pub unsafe fn timens_add_boottime_ns(nsec: u64) -> u64 {
    nsec
}

#[cfg(not(CONFIG_TIME_NS))]
#[inline]
pub unsafe fn timens_sub_boottime(_ts: *mut timespec64) {}

#[cfg(not(CONFIG_TIME_NS))]
#[inline]
pub unsafe fn timens_ktime_to_host(_clockid: clockid_t, tim: ktime_t) -> ktime_t {
    tim
}

#[cfg(CONFIG_TIME_NS_VDSO)]
extern "C" {
    pub fn timens_commit(tsk: *mut task_struct, ns: *mut time_namespace);
    pub fn find_timens_vvar_page(vma: *mut vm_area_struct) -> *mut page;
}

#[cfg(not(CONFIG_TIME_NS_VDSO))]
#[inline]
pub unsafe fn timens_commit(_tsk: *mut task_struct, _ns: *mut time_namespace) {}

#[cfg(not(CONFIG_TIME_NS_VDSO))]
#[inline]
pub unsafe fn find_timens_vvar_page(_vma: *mut vm_area_struct) -> *mut page {
    core::ptr::null_mut()
}

// DEFINE_FREE(time_ns, struct time_namespace *, if (_T) put_time_ns(_T))

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
