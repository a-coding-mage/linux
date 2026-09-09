/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/closure.h. The included Linux types and helpers are
// supplied by other translation units.

use core::ffi::c_void;

pub struct closure;
pub struct closure_syncer;
pub type closure_fn = unsafe extern "C" fn(*mut work_struct);

extern "C" {
    pub static mut bcache_debug: *mut dentry;
}

#[repr(C)]
pub struct closure_waitlist {
    pub list: llist_head,
}

#[repr(u32)]
pub enum closure_state {
    CLOSURE_BITS_START = 1u32 << 26,
    CLOSURE_DESTRUCTOR = 1u32 << 26,
    CLOSURE_WAITING = 1u32 << 28,
    CLOSURE_RUNNING = 1u32 << 30,
}

pub const CLOSURE_GUARD_MASK: u32 =
    ((CLOSURE_DESTRUCTOR | CLOSURE_WAITING | CLOSURE_RUNNING) << 1);
pub const CLOSURE_REMAINING_MASK: u32 = CLOSURE_BITS_START - 1;
pub const CLOSURE_REMAINING_INITIALIZER: u32 = 1 | CLOSURE_RUNNING;

#[cfg(feature = "CONFIG_DEBUG_CLOSURES")]
pub const CLOSURE_MAGIC_DEAD: u32 = 0xc054dead;
#[cfg(feature = "CONFIG_DEBUG_CLOSURES")]
pub const CLOSURE_MAGIC_ALIVE: u32 = 0xc054a11e;
#[cfg(feature = "CONFIG_DEBUG_CLOSURES")]
pub const CLOSURE_MAGIC_STACK: u32 = 0xc05451cc;

#[repr(C)]
pub union closure_data {
    pub fields: closure_fields,
    pub work: work_struct,
}

#[repr(C)]
pub struct closure_fields {
    pub wq: *mut workqueue_struct,
    pub s: *mut closure_syncer,
    pub list: llist_node,
    pub fn_: Option<closure_fn>,
}

#[repr(C)]
pub struct closure {
    pub data: closure_data,
    pub parent: *mut closure,
    pub remaining: atomic_t,
    pub closure_get_happened: bool,
    #[cfg(feature = "CONFIG_DEBUG_CLOSURES")]
    pub magic: u32,
    #[cfg(feature = "CONFIG_DEBUG_CLOSURES")]
    pub all: list_head,
    #[cfg(feature = "CONFIG_DEBUG_CLOSURES")]
    pub ip: usize,
    #[cfg(feature = "CONFIG_DEBUG_CLOSURES")]
    pub waiting_on: usize,
}

extern "C" {
    pub fn closure_sub(cl: *mut closure, v: i32);
    pub fn closure_put(cl: *mut closure);
    pub fn __closure_wake_up(list: *mut closure_waitlist);
    pub fn closure_wait(list: *mut closure_waitlist, cl: *mut closure) -> bool;
    pub fn __closure_sync(cl: *mut closure);
    pub fn __closure_sync_timeout(cl: *mut closure, timeout: c_ulong) -> i32;
    pub fn closure_return_sync(cl: *mut closure);
}

#[inline]
pub unsafe fn closure_nr_remaining(cl: *mut closure) -> u32 {
    (atomic_read(&(*cl).remaining) as u32) & CLOSURE_REMAINING_MASK
}

#[inline]
pub unsafe fn closure_sync(cl: *mut closure) {
    #[cfg(feature = "CONFIG_DEBUG_CLOSURES")]
    { BUG_ON(closure_nr_remaining(cl) != 1 && !(*cl).closure_get_happened); }
    if (*cl).closure_get_happened { __closure_sync(cl); }
}

#[inline]
pub unsafe fn closure_sync_timeout(cl: *mut closure, timeout: c_ulong) -> i32 {
    #[cfg(feature = "CONFIG_DEBUG_CLOSURES")]
    { BUG_ON(closure_nr_remaining(cl) != 1 && !(*cl).closure_get_happened); }
    if (*cl).closure_get_happened { __closure_sync_timeout(cl, timeout) } else { 0 }
}

#[inline] pub unsafe fn closure_debug_create(_cl: *mut closure) {}
#[inline] pub unsafe fn closure_debug_destroy(_cl: *mut closure) {}

#[inline]
pub unsafe fn closure_set_ip(cl: *mut closure) {
    #[cfg(feature = "CONFIG_DEBUG_CLOSURES")]
    { (*cl).ip = _THIS_IP_ as usize; }
}
#[inline]
pub unsafe fn closure_set_ret_ip(cl: *mut closure) {
    #[cfg(feature = "CONFIG_DEBUG_CLOSURES")]
    { (*cl).ip = _RET_IP_ as usize; }
}
#[inline]
pub unsafe fn closure_set_waiting(cl: *mut closure, f: c_ulong) {
    #[cfg(feature = "CONFIG_DEBUG_CLOSURES")]
    { (*cl).waiting_on = f as usize; }
}
#[inline] pub unsafe fn closure_set_stopped(cl: *mut closure) {
    atomic_sub(CLOSURE_RUNNING as i32, &mut (*cl).remaining);
}

#[inline]
pub unsafe fn set_closure_fn(cl: *mut closure, f: Option<closure_fn>, wq: *mut workqueue_struct) {
    closure_set_ip(cl);
    (*cl).data.fields.fn_ = f;
    (*cl).data.fields.wq = wq;
}

#[inline]
pub unsafe fn closure_get(cl: *mut closure) {
    (*cl).closure_get_happened = true;
    atomic_inc(&mut (*cl).remaining);
}

#[inline]
pub unsafe fn closure_get_not_zero(cl: *mut closure) -> bool {
    let mut old = atomic_read(&(*cl).remaining) as u32;
    loop {
        if old & CLOSURE_REMAINING_MASK == 0 { return false; }
        if atomic_try_cmpxchg_acquire(&mut (*cl).remaining, &mut old, old.wrapping_add(1)) { return true; }
    }
}

#[inline]
pub unsafe fn closure_init(cl: *mut closure, parent: *mut closure) {
    (*cl).data.fields.fn_ = None;
    (*cl).parent = parent;
    if !parent.is_null() { closure_get(parent); }
    atomic_set(&mut (*cl).remaining, CLOSURE_REMAINING_INITIALIZER as i32);
    (*cl).closure_get_happened = false;
    closure_debug_create(cl);
    closure_set_ip(cl);
}

#[inline] pub unsafe fn closure_wake_up(list: *mut closure_waitlist) { smp_mb(); __closure_wake_up(list); }

#[inline]
pub unsafe fn closure_call(cl: *mut closure, f: closure_fn, wq: *mut workqueue_struct, parent: *mut closure) {
    closure_init(cl, parent);
    set_closure_fn(cl, Some(f), wq);
}

#[macro_export]
macro_rules! continue_at { ($cl:expr, $fn:expr, $wq:expr) => {{
    unsafe { set_closure_fn($cl, $fn, $wq); closure_sub($cl, CLOSURE_RUNNING as i32 + 1); }
}}; }
#[macro_export] macro_rules! closure_return { ($cl:expr) => { continue_at!($cl, None, core::ptr::null_mut()) }; }
#[macro_export]
macro_rules! continue_at_nobarrier { ($cl:expr, $fn:expr, $wq:expr) => {{
    unsafe { set_closure_fn($cl, $fn, $wq); closure_queue($cl); }
}}; }
#[macro_export]
macro_rules! closure_type { ($name:ident, $ty:ty, $member:ident) => {
    let cl = container_of!(ws, closure, work); let $name = container_of!(cl, $ty, $member);
} }

#[inline] pub unsafe fn closure_init_stack(cl: *mut closure) {
    core::ptr::write_bytes(cl as *mut u8, 0, core::mem::size_of::<closure>());
    atomic_set(&mut (*cl).remaining, CLOSURE_REMAINING_INITIALIZER as i32);
    #[cfg(feature = "CONFIG_DEBUG_CLOSURES")] { (*cl).magic = CLOSURE_MAGIC_STACK; }
}
#[inline] pub unsafe fn closure_init_stack_release(cl: *mut closure) {
    core::ptr::write_bytes(cl as *mut u8, 0, core::mem::size_of::<closure>());
    atomic_set_release(&mut (*cl).remaining, CLOSURE_REMAINING_INITIALIZER as i32);
    #[cfg(feature = "CONFIG_DEBUG_CLOSURES")] { (*cl).magic = CLOSURE_MAGIC_STACK; }
}

#[inline] pub unsafe fn closure_queue(cl: *mut closure) {
    let wq = (*cl).data.fields.wq;
    if !wq.is_null() {
        INIT_WORK(&mut (*cl).data.work, (*cl).data.fields.fn_);
        BUG_ON(!queue_work(wq, &mut (*cl).data.work));
    } else if let Some(f) = (*cl).data.fields.fn_ { f(&mut (*cl).data.work); }
}

#[macro_export]
macro_rules! CLOSURE_CALLBACK { ($name:ident) => { unsafe extern "C" fn $name(ws: *mut work_struct) }; }

#[macro_export]
macro_rules! closure_return_with_destructor { ($cl:expr, $destructor:expr) => {{
    unsafe { set_closure_fn($cl, Some($destructor), core::ptr::null_mut());
        closure_sub($cl, CLOSURE_RUNNING as i32 - CLOSURE_DESTRUCTOR as i32 + 1); }
}}; }

#[macro_export]
macro_rules! __closure_wait_event { ($waitlist:expr, $cond:expr) => {{
    let mut cl = core::mem::MaybeUninit::<closure>::uninit(); unsafe {
        closure_init_stack(cl.as_mut_ptr());
        loop { closure_wait($waitlist, cl.as_mut_ptr()); if $cond { break; } closure_sync(cl.as_mut_ptr()); }
        closure_wake_up($waitlist); closure_sync(cl.as_mut_ptr());
    }
}}; }
#[macro_export]
macro_rules! closure_wait_event { ($waitlist:expr, $cond:expr) => {{
    if !$cond { __closure_wait_event!($waitlist, $cond); }
}}; }
#[macro_export]
macro_rules! closure_wait_event_timeout { ($waitlist:expr, $cond:expr, $timeout:expr) => {{
    let until = unsafe { jiffies() }.wrapping_add($timeout);
    if $cond { max_t!(i64, 1, until.wrapping_sub(unsafe { jiffies() }) as i64) }
    else { __closure_wait_event_timeout!($waitlist, $cond, until) }
}}; }
#[macro_export]
macro_rules! __closure_wait_event_timeout { ($waitlist:expr, $cond:expr, $until:expr) => {{
    let mut cl = core::mem::MaybeUninit::<closure>::uninit(); let mut t: i64;
    unsafe { closure_init_stack(cl.as_mut_ptr()); loop {
        closure_wait($waitlist, cl.as_mut_ptr());
        if $cond { t = max_t!(i64, 1, ($until).wrapping_sub(jiffies()) as i64); break; }
        t = max_t!(i64, 0, ($until).wrapping_sub(jiffies()) as i64); if t == 0 { break; }
        closure_sync_timeout(cl.as_mut_ptr(), t as c_ulong);
    } closure_wake_up($waitlist); closure_sync(cl.as_mut_ptr()); t }
}}; }

// C build-time configuration branches and Linux helper macros remain external.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
