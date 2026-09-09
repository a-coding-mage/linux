// SPDX-License-Identifier: GPL-2.0
/*
 * kernel/power/wakelock.c
 *
 * User space wakeup sources support.
 *
 * Copyright (C) 2012 Rafael J. Wysocki <rjw@sisk.pl>
 *
 * This code is based on the analogous interface allowing user space to
 * manipulate wakelocks on Android.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Declarations supplied by the kernel and by power.h.
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct rb_node { pub rb_left: *mut rb_node, pub rb_right: *mut rb_node, pub rb_parent_color: usize }
#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct spinlock { _private: [u8; 0] }
#[repr(C)] pub struct wakeup_source {
    pub active: bool,
    pub lock: spinlock,
    pub last_time: ktime_t,
}
#[repr(C)] pub struct ktime_t { pub tv64: i64 }

extern "C" {
    static mut wakelocks_lock: mutex;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn rb_first(root: *const rb_root) -> *mut rb_node;
    fn rb_next(node: *mut rb_node) -> *mut rb_node;
    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node);
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);
    fn sysfs_emit_at(buf: *mut c_char, at: c_int, fmt: *const c_char, ...) -> c_int;
    fn capable(cap: c_int) -> bool;
    fn isspace(c: c_int) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn kstrtou64(s: *const c_char, base: c_uint, result: *mut u64) -> c_int;
    fn skip_spaces(s: *const c_char) -> *const c_char;
    fn kstrndup(s: *const c_char, len: usize, flags: c_uint) -> *mut c_char;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn wakeup_source_register(dev: *mut c_void, name: *const c_char) -> *mut wakeup_source;
    fn wakeup_source_unregister(ws: *mut wakeup_source);
    fn ktime_get() -> ktime_t;
    fn ktime_sub(a: ktime_t, b: ktime_t) -> ktime_t;
    fn ktime_to_ns(t: ktime_t) -> i64;
    fn spin_lock_irq(lock: *mut spinlock);
    fn spin_unlock_irq(lock: *mut spinlock);
    fn schedule_work(work: *mut work_struct);
    fn __pm_wakeup_event(ws: *mut wakeup_source, msec: u64);
    fn __pm_stay_awake(ws: *mut wakeup_source);
    fn __pm_relax(ws: *mut wakeup_source);
}

const EINVAL: c_int = 22;
const ENOSPC: c_int = 28;
const ENOMEM: c_int = 12;
const EPERM: c_int = 1;
const CAP_BLOCK_SUSPEND: c_int = 36;
const GFP_KERNEL: c_uint = 0;
const NSEC_PER_SEC: u64 = 1_000_000_000;
const NSEC_PER_MSEC: u64 = 1_000_000;

#[repr(C)]
struct wakelock {
    name: *mut c_char,
    node: rb_node,
    ws: *mut wakeup_source,
    #[cfg(CONFIG_PM_WAKELOCKS_GC)]
    lru: list_head,
}

static mut wakelocks_tree: rb_root = rb_root { rb_node: core::ptr::null_mut() };

pub unsafe fn pm_show_wakelocks(buf: *mut c_char, show_active: bool) -> isize {
    let mut node = rb_first(&wakelocks_tree);
    let mut len: c_int = 0;
    mutex_lock(&raw mut wakelocks_lock);
    while !node.is_null() {
        let wl = (node as *mut u8).sub(core::mem::offset_of!(wakelock, node)) as *mut wakelock;
        if (*(*wl).ws).active == show_active {
            len += sysfs_emit_at(buf, len, b"%s \0".as_ptr() as *const c_char, (*wl).name);
        }
        node = rb_next(node);
    }
    if len > 0 { len -= 1; }
    len += sysfs_emit_at(buf, len, b"\n\0".as_ptr() as *const c_char);
    mutex_unlock(&raw mut wakelocks_lock);
    len as isize
}

#[cfg(CONFIG_PM_WAKELOCKS_LIMIT)]
static mut number_of_wakelocks: c_uint = 0;
#[cfg(CONFIG_PM_WAKELOCKS_LIMIT)]
#[inline] unsafe fn wakelocks_limit_exceeded() -> bool { number_of_wakelocks >= CONFIG_PM_WAKELOCKS_LIMIT }
#[cfg(CONFIG_PM_WAKELOCKS_LIMIT)]
#[inline] unsafe fn increment_wakelocks_number() { number_of_wakelocks += 1; }
#[cfg(CONFIG_PM_WAKELOCKS_LIMIT)]
#[inline] unsafe fn decrement_wakelocks_number() { number_of_wakelocks -= 1; }
#[cfg(not(CONFIG_PM_WAKELOCKS_LIMIT))]
#[inline] unsafe fn wakelocks_limit_exceeded() -> bool { false }
#[cfg(not(CONFIG_PM_WAKELOCKS_LIMIT))]
#[inline] unsafe fn increment_wakelocks_number() {}
#[cfg(not(CONFIG_PM_WAKELOCKS_LIMIT))]
#[inline] unsafe fn decrement_wakelocks_number() {}

// CONFIG_PM_WAKELOCKS_GC is a build-time condition preserved with Rust cfgs.
#[cfg(not(CONFIG_PM_WAKELOCKS_GC))]
#[inline] unsafe fn wakelocks_lru_most_recent(_wl: *mut wakelock) {}
#[cfg(not(CONFIG_PM_WAKELOCKS_GC))]
#[inline] unsafe fn wakelocks_gc() {}
#[cfg(not(CONFIG_PM_WAKELOCKS_GC))]
#[inline] unsafe fn wakelocks_lru_add(_wl: *mut wakelock) {}

unsafe fn wakelock_lookup_add(name: *const c_char, len: usize, add_if_not_found: bool) -> *mut wakelock {
    let mut node: *mut *mut rb_node = &raw mut wakelocks_tree.rb_node;
    let mut parent = *node;
    while !(*node).is_null() {
        parent = *node;
        let wl = ((*node as *mut u8).sub(core::mem::offset_of!(wakelock, node))) as *mut wakelock;
        let mut diff = strncmp(name, (*wl).name, len);
        if diff == 0 {
            if *(*wl).name.add(len) != 0 { diff = -1; } else { return wl; }
        }
        node = if diff < 0 { &mut (*(*node)).rb_left } else { &mut (*(*node)).rb_right };
    }
    if !add_if_not_found { return EINVAL as isize as *mut wakelock; }
    if wakelocks_limit_exceeded() { return ENOSPC as isize as *mut wakelock; }
    let wl = kzalloc(core::mem::size_of::<wakelock>(), GFP_KERNEL) as *mut wakelock;
    if wl.is_null() { return ENOMEM as isize as *mut wakelock; }
    (*wl).name = kstrndup(name, len, GFP_KERNEL);
    if (*wl).name.is_null() { kfree(wl as *mut c_void); return ENOMEM as isize as *mut wakelock; }
    (*wl).ws = wakeup_source_register(core::ptr::null_mut(), (*wl).name);
    if (*wl).ws.is_null() { kfree((*wl).name as *mut c_void); kfree(wl as *mut c_void); return ENOMEM as isize as *mut wakelock; }
    (*wl).ws.as_mut().unwrap().last_time = ktime_get();
    rb_link_node(&mut (*wl).node, parent, node);
    rb_insert_color(&mut (*wl).node, &raw mut wakelocks_tree);
    wakelocks_lru_add(wl);
    increment_wakelocks_number();
    wl
}

pub unsafe fn pm_wake_lock(buf: *const c_char) -> c_int {
    if !capable(CAP_BLOCK_SUSPEND) { return -EPERM; }
    let mut strp = buf;
    while *strp != 0 && isspace(*strp as c_int) == 0 { strp = strp.add(1); }
    let len = strp.offset_from(buf) as usize;
    if len == 0 { return -EINVAL; }
    let mut timeout_ns = 0u64;
    if *strp != 0 && *strp != b'\n' as c_char { if kstrtou64(skip_spaces(strp), 10, &mut timeout_ns) != 0 { return -EINVAL; } }
    mutex_lock(&raw mut wakelocks_lock);
    let wl = wakelock_lookup_add(buf, len, true);
    if (wl as isize) < 0 { mutex_unlock(&raw mut wakelocks_lock); return wl as isize as c_int; }
    if timeout_ns != 0 { __pm_wakeup_event((*wl).ws, (timeout_ns + NSEC_PER_MSEC - 1) / NSEC_PER_MSEC); } else { __pm_stay_awake((*wl).ws); }
    wakelocks_lru_most_recent(wl);
    mutex_unlock(&raw mut wakelocks_lock);
    0
}

pub unsafe fn pm_wake_unlock(buf: *const c_char) -> c_int {
    if !capable(CAP_BLOCK_SUSPEND) { return -EPERM; }
    let mut len = strlen(buf);
    if len == 0 { return -EINVAL; }
    if *buf.add(len - 1) == b'\n' as c_char { len -= 1; }
    if len == 0 { return -EINVAL; }
    mutex_lock(&raw mut wakelocks_lock);
    let wl = wakelock_lookup_add(buf, len, false);
    if (wl as isize) < 0 { mutex_unlock(&raw mut wakelocks_lock); return wl as isize as c_int; }
    __pm_relax((*wl).ws);
    wakelocks_lru_most_recent(wl);
    wakelocks_gc();
    mutex_unlock(&raw mut wakelocks_lock);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
