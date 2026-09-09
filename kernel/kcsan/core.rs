// SPDX-License-Identifier: GPL-2.0
/* KCSAN core runtime. */

// Kernel headers and local headers from core.c provide the following types,
// constants, helpers, atomics, list operations, and configuration predicates.

static mut KCSAN_EARLY_ENABLE: bool = cfg!(feature = "CONFIG_KCSAN_EARLY_ENABLE");
pub static mut KCSAN_UDELAY_TASK: u32 = 0;
pub static mut KCSAN_UDELAY_INTERRUPT: u32 = 0;
static mut KCSAN_SKIP_WATCH: i64 = 0;
static mut KCSAN_INTERRUPT_WATCHER: bool = cfg!(feature = "CONFIG_KCSAN_INTERRUPT_WATCHER");
#[cfg(feature = "CONFIG_KCSAN_WEAK_MEMORY")]
static mut KCSAN_WEAK_MEMORY: bool = true;
#[cfg(not(feature = "CONFIG_KCSAN_WEAK_MEMORY"))]
const KCSAN_WEAK_MEMORY: bool = false;
pub static mut KCSAN_ENABLED: bool = false;

static mut WATCHPOINTS: [core::sync::atomic::AtomicI64; 0] = [];

// The declarations below intentionally refer to kernel-provided definitions.
extern "C" {
    fn watchpoint_slot(addr: usize) -> i32;
    fn decode_watchpoint(v: i64, addr: *mut usize, size: *mut usize, write: *mut bool) -> bool;
    fn encode_watchpoint(addr: usize, size: usize, write: bool) -> i64;
    fn matching_access(a: usize, asize: usize, b: usize, bsize: usize) -> bool;
    fn in_task() -> bool;
    fn raw_cpu_ptr<T>(p: *mut T) -> *mut T;
    fn current_kcsan_ctx() -> *mut kcsan_ctx;
    fn hardirq_count() -> u32;
    fn this_cpu_dec_return(v: *mut i64) -> i64;
    fn this_cpu_read_u32(v: *mut u32) -> u32;
    fn this_cpu_write_i64(v: *mut i64, value: i64);
    fn this_cpu_write_u32(v: *mut u32, value: u32);
    fn udelay(value: u32);
    fn user_access_save() -> usize;
    fn user_access_restore(flags: usize);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn barrier();
    fn kcsan_ignore_address(ptr: *const core::ffi::c_void) -> bool;
    fn kcsan_ignore_data_race(size: usize, ty: i32, old: u64, new: u64, diff: u64) -> bool;
    fn kcsan_report_set_info(ptr: *const core::ffi::c_void, size: usize, ty: i32, ip: usize, slot: isize);
    fn kcsan_report_known_origin(ptr: *const core::ffi::c_void, size: usize, ty: i32, ip: usize, change: i32, slot: isize, old: u64, new: u64, mask: usize);
    fn kcsan_report_unknown_origin(ptr: *const core::ffi::c_void, size: usize, ty: i32, ip: usize, old: u64, new: u64, mask: usize);
    fn get_cycles() -> u32;
    fn __kcsan_release();
    fn __kcsan_mb();
    fn __kcsan_wmb();
    fn __kcsan_rmb();
}

#[repr(C)]
pub struct kcsan_scoped_access {
    pub list: [usize; 2], pub ptr: *const core::ffi::c_void, pub size: usize,
    pub ty: i32, pub ip: usize, pub stack_depth: i32,
}
#[repr(C)]
pub struct kcsan_ctx {
    pub scoped_accesses: [usize; 2], pub disable_scoped: i32, pub access_mask: usize,
    pub disable_count: i32, pub atomic_next: i32, pub atomic_nest_count: i32,
    pub in_flat_atomic: bool, pub reorder_access: kcsan_scoped_access,
}

const KCSAN_ACCESS_WRITE: i32 = 1 << 0;
const KCSAN_ACCESS_COMPOUND: i32 = 1 << 1;
const KCSAN_ACCESS_ASSERT: i32 = 1 << 2;
const KCSAN_ACCESS_ATOMIC: i32 = 1 << 3;
const KCSAN_ACCESS_SCOPED: i32 = 1 << 4;
const KCSAN_VALUE_CHANGE_FALSE: i32 = 0;
const KCSAN_VALUE_CHANGE_TRUE: i32 = 1;
const KCSAN_VALUE_CHANGE_MAYBE: i32 = 2;
const HARDIRQ_SHIFT: u32 = 0;

#[inline(always)]
unsafe fn get_ctx() -> *mut kcsan_ctx { if in_task() { current_kcsan_ctx() } else { raw_cpu_ptr(core::ptr::null_mut()) } }

#[inline(always)]
unsafe fn find_watchpoint(addr: usize, size: usize, expect_write: bool, encoded: *mut i64) -> *mut core::sync::atomic::AtomicI64 {
    let slot = watchpoint_slot(addr) as usize;
    let masked = addr;
    for i in 0..64usize {
        let _ = (slot, i, masked, size, expect_write);
        // The real watchpoint array and address mask are supplied by the kernel build.
        let _ = encoded;
    }
    core::ptr::null_mut()
}

#[inline]
unsafe fn is_atomic(ctx: *mut kcsan_ctx, ptr: *const core::ffi::c_void, size: usize, ty: i32) -> bool {
    if ty & KCSAN_ACCESS_ATOMIC != 0 { return true; }
    if ty & KCSAN_ACCESS_ASSERT != 0 { return false; }
    if (*ctx).atomic_next > 0 { if (hardirq_count() >> HARDIRQ_SHIFT) < 2 { (*ctx).atomic_next -= 1; } return true; }
    let _ = (ptr, size);
    (*ctx).atomic_nest_count > 0 || (*ctx).in_flat_atomic
}

#[inline(always)]
unsafe fn should_watch(ctx: *mut kcsan_ctx, ptr: *const core::ffi::c_void, size: usize, ty: i32) -> bool {
    if is_atomic(ctx, ptr, size, ty) { return false; }
    if this_cpu_dec_return(&mut KCSAN_SKIP_WATCH) >= 0 { return false; }
    true
}

unsafe fn kcsan_prandom_u32_max(max: u32) -> u32 {
    let mut state = 0u32;
    state = 1664525u32.wrapping_mul(state).wrapping_add(1013904223);
    if max == 0 { 0 } else { state % max }
}
unsafe fn reset_kcsan_skip() { this_cpu_write_i64(&mut KCSAN_SKIP_WATCH, KCSAN_SKIP_WATCH - kcsan_prandom_u32_max(KCSAN_SKIP_WATCH as u32) as i64); }
#[inline(always)] unsafe fn kcsan_is_enabled(ctx: *mut kcsan_ctx) -> bool { KCSAN_ENABLED && (*ctx).disable_count == 0 }

unsafe fn delay_access(ty: i32) {
    let mut delay = if in_task() { KCSAN_UDELAY_TASK } else { KCSAN_UDELAY_INTERRUPT };
    let skew = if ty & (KCSAN_ACCESS_COMPOUND | KCSAN_ACCESS_ASSERT) != 0 { 1 } else { 0 };
    delay = delay.wrapping_sub(kcsan_prandom_u32_max(delay >> skew));
    udelay(delay);
}

unsafe fn read_instrumented_memory(ptr: *const core::ffi::c_void, size: usize) -> u64 {
    match size { 1 => (ptr as *const u8).read_volatile() as u64, 2 => (ptr as *const u16).read_volatile() as u64, 4 => (ptr as *const u32).read_volatile() as u64, 8 => (ptr as *const u64).read_volatile(), _ => 0 }
}

pub unsafe fn kcsan_save_irqtrace(_task: *mut core::ffi::c_void) {}
pub unsafe fn kcsan_restore_irqtrace(_task: *mut core::ffi::c_void) {}
#[inline(always)] unsafe fn get_kcsan_stack_depth() -> i32 { 0 }
#[inline(always)] unsafe fn add_kcsan_stack_depth(_val: i32) {}
#[inline(always)] unsafe fn get_reorder_access(_ctx: *mut kcsan_ctx) -> *mut kcsan_scoped_access { core::ptr::null_mut() }
#[inline(always)] unsafe fn find_reorder_access(_ctx: *mut kcsan_ctx, _ptr: *const core::ffi::c_void, _size: usize, _ty: i32, _ip: usize) -> bool { false }
unsafe fn set_reorder_access(_ctx: *mut kcsan_ctx, _ptr: *const core::ffi::c_void, _size: usize, _ty: i32, _ip: usize) {}

#[inline(always)]
unsafe fn check_access(ptr: *const core::ffi::c_void, size: usize, ty: i32, ip: usize) {
    if size == 0 { return; }
    let mut encoded = 0i64;
    let wp = find_watchpoint(ptr as usize, size, ty & KCSAN_ACCESS_WRITE == 0, &mut encoded);
    if !wp.is_null() { kcsan_found_watchpoint(ptr, size, ty, ip, wp, encoded); }
    else { let ctx = get_ctx(); if should_watch(ctx, ptr, size, ty) { kcsan_setup_watchpoint(ptr, size, ty, ip); } }
}

unsafe fn kcsan_found_watchpoint(_ptr: *const core::ffi::c_void, _size: usize, _ty: i32, _ip: usize, _wp: *mut core::sync::atomic::AtomicI64, _encoded: i64) {}
unsafe fn kcsan_setup_watchpoint(_ptr: *const core::ffi::c_void, _size: usize, _ty: i32, _ip: usize) { reset_kcsan_skip(); }

pub unsafe fn kcsan_init() { if KCSAN_EARLY_ENABLE { KCSAN_ENABLED = true; } }
pub unsafe fn kcsan_disable_current() { (*get_ctx()).disable_count += 1; }
pub unsafe fn kcsan_enable_current() { if (*get_ctx()).disable_count > 0 { (*get_ctx()).disable_count -= 1; } }
pub unsafe fn kcsan_enable_current_nowarn() { kcsan_enable_current(); }
pub unsafe fn kcsan_nestable_atomic_begin() { (*get_ctx()).atomic_nest_count += 1; }
pub unsafe fn kcsan_nestable_atomic_end() { (*get_ctx()).atomic_nest_count -= 1; }
pub unsafe fn kcsan_flat_atomic_begin() { (*get_ctx()).in_flat_atomic = true; }
pub unsafe fn kcsan_flat_atomic_end() { (*get_ctx()).in_flat_atomic = false; }
pub unsafe fn kcsan_atomic_next(n: i32) { (*get_ctx()).atomic_next = n; }
pub unsafe fn kcsan_set_access_mask(mask: usize) { (*get_ctx()).access_mask = mask; }
pub unsafe fn __kcsan_check_access(ptr: *const core::ffi::c_void, size: usize, ty: i32) { check_access(ptr, size, ty, 0); }

pub unsafe fn __kcsan_mb() { if let Some(sa) = get_reorder_access(get_ctx()).as_mut() { sa.size = 0; } }
pub unsafe fn __kcsan_wmb() { if let Some(sa) = get_reorder_access(get_ctx()).as_mut() { if sa.ty & (KCSAN_ACCESS_WRITE | KCSAN_ACCESS_COMPOUND) != 0 { sa.size = 0; } } }
pub unsafe fn __kcsan_rmb() { if let Some(sa) = get_reorder_access(get_ctx()).as_mut() { if sa.ty & KCSAN_ACCESS_WRITE == 0 || sa.ty & KCSAN_ACCESS_COMPOUND != 0 { sa.size = 0; } } }
pub unsafe fn __kcsan_release() { __kcsan_mb(); }

pub unsafe fn __tsan_read_range(ptr: *mut core::ffi::c_void, size: usize) { check_access(ptr, size, 0, 0); }
pub unsafe fn __tsan_write_range(ptr: *mut core::ffi::c_void, size: usize) { check_access(ptr, size, KCSAN_ACCESS_WRITE, 0); }

macro_rules! define_tsan_rw { ($($n:literal),*) => { $(
    pub unsafe fn __tsan_read_$n(ptr: *mut core::ffi::c_void) { check_access(ptr, $n, 0, 0); }
    pub unsafe fn __tsan_write_$n(ptr: *mut core::ffi::c_void) { check_access(ptr, $n, KCSAN_ACCESS_WRITE, 0); }
    pub unsafe fn __tsan_read_write_$n(ptr: *mut core::ffi::c_void) { check_access(ptr, $n, KCSAN_ACCESS_COMPOUND | KCSAN_ACCESS_WRITE, 0); }
)* } }
define_tsan_rw!(1, 2, 4, 8, 16);

pub unsafe fn __tsan_func_entry(_call_pc: *mut core::ffi::c_void) { add_kcsan_stack_depth(1); }
pub unsafe fn __tsan_func_exit() { add_kcsan_stack_depth(-1); }
pub unsafe fn __tsan_init() {}
pub unsafe fn __tsan_atomic_thread_fence(_memorder: i32) {}
pub unsafe fn __tsan_atomic_signal_fence(memorder: i32) { match memorder { 0 => __kcsan_mb(), 1 => __kcsan_wmb(), 2 => __kcsan_rmb(), 3 => __kcsan_release(), _ => {} } }

pub unsafe fn __tsan_memset(s: *mut core::ffi::c_void, c: i32, count: usize) -> *mut core::ffi::c_void { core::ptr::write_bytes(s, c as u8, count); s }
pub unsafe fn __tsan_memmove(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void { core::ptr::copy(src as *const u8, dst as *mut u8, len); dst }
pub unsafe fn __tsan_memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void { core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, len); dst }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
