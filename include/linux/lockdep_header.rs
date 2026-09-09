/* SPDX-License-Identifier: GPL-2.0 */
/* Runtime locking correctness validator (source-level Rust translation). */

/* C header dependencies are supplied by the surrounding kernel translation. */

#[cfg(CONFIG_LOCKDEP)]
#[repr(C)]
pub struct lock_list {
    pub entry: list_head,
    pub class: *mut lock_class,
    pub links_to: *mut lock_class,
    pub trace: *const lock_trace,
    pub distance: u16,
    pub dep: u8,
    pub only_xr: u8,
    pub parent: *mut lock_list,
}

#[cfg(CONFIG_LOCKDEP)]
#[repr(C)]
pub struct lock_chain {
    pub irq_context: u32,
    pub depth: u32,
    pub base: u32,
    pub entry: hlist_node,
    pub chain_key: u64,
}

#[cfg(CONFIG_LOCKDEP)]
pub const LOCKDEP_RECURSION_BITS: u32 = 16;
#[cfg(CONFIG_LOCKDEP)]
pub const LOCKDEP_OFF: u32 = 1u32 << LOCKDEP_RECURSION_BITS;
#[cfg(CONFIG_LOCKDEP)]
pub const LOCKDEP_RECURSION_MASK: u32 = LOCKDEP_OFF - 1;

#[cfg(CONFIG_LOCKDEP)]
extern "C" {
    pub fn lockdep_init();
    pub fn lockdep_reset();
    pub fn lockdep_reset_lock(lock: *mut lockdep_map);
    pub fn lockdep_free_key_range(start: *mut core::ffi::c_void, size: usize);
    pub fn lockdep_sys_exit();
    pub fn lockdep_set_selftest_task(task: *mut task_struct);
    pub fn lockdep_init_task(task: *mut task_struct);
    pub fn lockdep_register_key(key: *mut lock_class_key);
    pub fn lockdep_unregister_key(key: *mut lock_class_key);
    pub fn lockdep_init_map_type(lock: *mut lockdep_map, name: *const i8,
        key: *mut lock_class_key, subclass: i32, inner: u8, outer: u8, lock_type: u8);
    pub fn lock_acquire(lock: *mut lockdep_map, subclass: u32, trylock: i32,
        read: i32, check: i32, nest_lock: *mut lockdep_map, ip: usize);
    pub fn lock_release(lock: *mut lockdep_map, ip: usize);
    pub fn lock_sync(lock: *mut lockdep_map, subclass: u32, read: i32,
        check: i32, nest_lock: *mut lockdep_map, ip: usize);
    pub fn lock_is_held_type(lock: *const lockdep_map, read: i32) -> i32;
    pub fn lock_set_class(lock: *mut lockdep_map, name: *const i8,
        key: *mut lock_class_key, subclass: u32, ip: usize);
    pub fn lock_downgrade(lock: *mut lockdep_map, ip: usize);
    pub fn lock_pin_lock(lock: *mut lockdep_map) -> pin_cookie;
    pub fn lock_repin_lock(lock: *mut lockdep_map, cookie: pin_cookie);
    pub fn lock_unpin_lock(lock: *mut lockdep_map, cookie: pin_cookie);
    pub fn lock_sequence(lock: *mut lockdep_map) -> u32;
}

#[cfg(CONFIG_LOCKDEP)]
#[inline]
pub unsafe fn lockdep_init_map_waits(lock: *mut lockdep_map, name: *const i8,
    key: *mut lock_class_key, subclass: i32, inner: u8, outer: u8) {
    lockdep_init_map_type(lock, name, key, subclass, inner, outer, LD_LOCK_NORMAL);
}
#[cfg(CONFIG_LOCKDEP)]
#[inline]
pub unsafe fn lockdep_init_map_wait(lock: *mut lockdep_map, name: *const i8,
    key: *mut lock_class_key, subclass: i32, inner: u8) {
    lockdep_init_map_waits(lock, name, key, subclass, inner, LD_WAIT_INV);
}
#[cfg(CONFIG_LOCKDEP)]
#[inline]
pub unsafe fn lockdep_init_map(lock: *mut lockdep_map, name: *const i8,
    key: *mut lock_class_key, subclass: i32) {
    lockdep_init_map_wait(lock, name, key, subclass, LD_WAIT_INV);
}
#[cfg(CONFIG_LOCKDEP)]
#[inline]
pub unsafe fn lockdep_match_key(lock: *mut lockdep_map, key: *mut lock_class_key) -> i32 {
    ((*lock).key == key) as i32
}
#[cfg(CONFIG_LOCKDEP)]
#[inline]
pub unsafe fn lock_is_held(lock: *const lockdep_map) -> i32 { lock_is_held_type(lock, -1) }

pub const LOCK_STATE_UNKNOWN: i32 = -1;
pub const LOCK_STATE_NOT_HELD: i32 = 0;
pub const LOCK_STATE_HELD: i32 = 1;
pub const SINGLE_DEPTH_NESTING: i32 = 1;

#[cfg(not(CONFIG_LOCKDEP))]
pub unsafe fn lockdep_init_task(_: *mut task_struct) {}
#[cfg(not(CONFIG_LOCKDEP))]
pub unsafe fn lockdep_off() {}
#[cfg(not(CONFIG_LOCKDEP))]
pub unsafe fn lockdep_on() {}
#[cfg(not(CONFIG_LOCKDEP))]
pub unsafe fn lockdep_set_selftest_task(_: *mut task_struct) {}

#[cfg(CONFIG_PROVE_LOCKING)]
extern "C" { pub fn lockdep_set_lock_cmp_fn(map: *mut lockdep_map, cmp: lock_cmp_fn, print: lock_print_fn); }

#[repr(C)]
pub enum xhlock_context_t { XHLOCK_HARD, XHLOCK_SOFT, XHLOCK_CTX_NR }

#[cfg(CONFIG_LOCK_STAT)]
extern "C" {
    pub fn lock_contended(lock: *mut lockdep_map, ip: usize);
    pub fn lock_acquired(lock: *mut lockdep_map, ip: usize);
}
#[cfg(CONFIG_PROVE_LOCKING)]
extern "C" { pub fn print_irqtrace_events(curr: *mut task_struct); }
#[cfg(not(CONFIG_PROVE_LOCKING))]
pub unsafe fn print_irqtrace_events(_: *mut task_struct) {}

#[cfg(CONFIG_DEBUG_LOCKING_API_SELFTESTS)]
extern "C" { pub static mut force_read_lock_recursive: u32; }
#[cfg(not(CONFIG_DEBUG_LOCKING_API_SELFTESTS))]
pub const force_read_lock_recursive: u32 = 0;
#[cfg(CONFIG_LOCKDEP)]
extern "C" { pub fn read_lock_is_recursive() -> bool; }

#[cfg(not(CONFIG_LOCKDEP))]
#[inline] pub unsafe fn lock_is_held(_: *const core::ffi::c_void) -> i32 { 0 }

/* The following macros retain the original kernel call shapes and evaluation order. */
#[macro_export] macro_rules! lockdep_sequence { ($l:expr) => { unsafe { lock_sequence(&mut (*$l).dep_map) } }; }
#[macro_export] macro_rules! lock_acquire_exclusive { ($l:expr,$s:expr,$t:expr,$n:expr,$i:expr) => { unsafe { lock_acquire($l,$s,$t,0,1,$n,$i) } }; }
#[macro_export] macro_rules! lock_acquire_shared { ($l:expr,$s:expr,$t:expr,$n:expr,$i:expr) => { unsafe { lock_acquire($l,$s,$t,1,1,$n,$i) } }; }
#[macro_export] macro_rules! lock_acquire_shared_recursive { ($l:expr,$s:expr,$t:expr,$n:expr,$i:expr) => { unsafe { lock_acquire($l,$s,$t,2,1,$n,$i) } }; }
#[macro_export] macro_rules! spin_acquire { ($l:expr,$s:expr,$t:expr,$i:expr) => { lock_acquire_exclusive!($l,$s,$t,core::ptr::null_mut(),$i) }; }
#[macro_export] macro_rules! spin_release { ($l:expr,$i:expr) => { unsafe { lock_release($l,$i) } }; }
#[macro_export] macro_rules! rwlock_acquire { ($l:expr,$s:expr,$t:expr,$i:expr) => { lock_acquire_exclusive!($l,$s,$t,core::ptr::null_mut(),$i) }; }
#[macro_export] macro_rules! rwlock_release { ($l:expr,$i:expr) => { unsafe { lock_release($l,$i) } }; }
#[macro_export] macro_rules! mutex_acquire { ($l:expr,$s:expr,$t:expr,$i:expr) => { lock_acquire_exclusive!($l,$s,$t,core::ptr::null_mut(),$i) }; }
#[macro_export] macro_rules! mutex_release { ($l:expr,$i:expr) => { unsafe { lock_release($l,$i) } }; }
#[macro_export] macro_rules! lock_map_release { ($l:expr,$i:expr) => { unsafe { lock_release($l,$i) } }; }

/* C preprocessor-only assertions and configuration-dependent NOPs remain macros. */
#[macro_export] macro_rules! lockdep_assert { ($c:expr) => { if cfg!(CONFIG_LOCKDEP) { debug_assert!($c); } }; }
#[macro_export] macro_rules! lockdep_assert_once { ($c:expr) => { lockdep_assert!($c) }; }
#[macro_export] macro_rules! lockdep_is_held_type { ($l:expr,$r:expr) => { unsafe { lock_is_held_type(&(*$l).dep_map,$r) } }; }
#[macro_export] macro_rules! lockdep_is_held { ($l:expr) => { lockdep_is_held_type!($l,-1) }; }
#[macro_export] macro_rules! lockdep_assert_held { ($l:expr) => { lockdep_assert!(lockdep_is_held!($l) != LOCK_STATE_NOT_HELD) }; }
#[macro_export] macro_rules! lockdep_assert_not_held { ($l:expr) => { lockdep_assert!(lockdep_is_held!($l) != LOCK_STATE_HELD) }; }
#[macro_export] macro_rules! lockdep_set_class { ($l:expr,$k:expr) => { unsafe { lockdep_init_map_type(&mut (*$l).dep_map,stringify!($k).as_ptr() as *const i8,$k,0,(*$l).dep_map.wait_type_inner,(*$l).dep_map.wait_type_outer,(*$l).dep_map.lock_type) } }; }
#[macro_export] macro_rules! lockdep_set_subclass { ($l:expr,$s:expr) => { unsafe { lock_set_class(&mut (*$l).dep_map,(*$l).dep_map.name,(*$l).dep_map.key,$s,0) } }; }
#[macro_export] macro_rules! lockdep_pin_lock { ($l:expr) => { unsafe { lock_pin_lock(&mut (*$l).dep_map) } }; }
#[macro_export] macro_rules! lockdep_repin_lock { ($l:expr,$c:expr) => { unsafe { lock_repin_lock(&mut (*$l).dep_map,$c) } }; }
#[macro_export] macro_rules! lockdep_unpin_lock { ($l:expr,$c:expr) => { unsafe { lock_unpin_lock(&mut (*$l).dep_map,$c) } }; }

/* External kernel types intentionally remain unresolved dependencies, as in the header. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
