/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of asm-generic/percpu.h.  Included C headers and
// configuration-selected definitions are supplied by other translation units.

#[cfg(feature = "smp")]
extern "C" {
    pub static mut __per_cpu_offset: [usize; NR_CPUS];
}

#[cfg(feature = "smp")]
#[inline(always)]
pub unsafe fn per_cpu_offset(x: usize) -> usize { __per_cpu_offset[x] }

#[cfg(feature = "smp")]
#[inline(always)]
pub unsafe fn __my_cpu_offset() -> usize { per_cpu_offset(raw_smp_processor_id() as usize) }

#[cfg(feature = "smp")]
#[inline(always)]
pub unsafe fn my_cpu_offset() -> usize {
    #[cfg(feature = "debug_preempt")]
    { per_cpu_offset(smp_processor_id() as usize) }
    #[cfg(not(feature = "debug_preempt"))]
    { __my_cpu_offset() }
}

#[cfg(feature = "smp")]
#[inline(always)]
pub unsafe fn arch_raw_cpu_ptr<T>(ptr: *mut T) -> *mut T {
    // SHIFT_PERCPU_PTR(ptr, __my_cpu_offset), whose architecture-specific
    // implementation is provided by the surrounding kernel translation.
    (ptr as *mut u8).wrapping_offset(__my_cpu_offset() as isize) as *mut T
}

#[cfg(all(feature = "smp", feature = "have_setup_per_cpu_area"))]
extern "C" { pub fn setup_per_cpu_areas(); }

#[cfg(feature = "smp")]
pub const PER_CPU_BASE_SECTION: &str = ".data..percpu";
#[cfg(not(feature = "smp"))]
pub const PER_CPU_BASE_SECTION: &str = ".data";

#[inline(always)]
pub unsafe fn raw_cpu_generic_read<T: Copy>(pcp: *mut T) -> T { *arch_raw_cpu_ptr(pcp) }

#[inline(always)]
pub unsafe fn raw_cpu_generic_to_op<T>(pcp: *mut T, val: T, op: impl FnOnce(&mut T, T)) {
    op(&mut *arch_raw_cpu_ptr(pcp), val)
}

#[inline(always)]
pub unsafe fn raw_cpu_generic_add_return<T>(pcp: *mut T, val: T) -> T
where T: Copy + std::ops::AddAssign {
    let p = arch_raw_cpu_ptr(pcp); (*p) += val; *p
}

#[inline(always)]
pub unsafe fn raw_cpu_generic_xchg<T: Copy>(pcp: *mut T, nval: T) -> T {
    let p = arch_raw_cpu_ptr(pcp); let ret = *p; *p = nval; ret
}

#[inline(always)]
pub unsafe fn raw_cpu_generic_try_cmpxchg<T: Copy + PartialEq>(pcp: *mut T, ovalp: *mut T, nval: T) -> bool {
    let p = arch_raw_cpu_ptr(pcp); let val = *p; let old = *ovalp;
    if val == old { *p = nval; true } else { *ovalp = val; false }
}

#[inline(always)]
pub unsafe fn raw_cpu_generic_cmpxchg<T: Copy + PartialEq>(pcp: *mut T, oval: T, nval: T) -> T {
    let mut old = oval;
    raw_cpu_generic_try_cmpxchg(pcp, &mut old, nval); old
}

// The following macros retain the source header's call shape.  The C
// preprocessor's TYPEOF_UNQUAL, READ_ONCE, and IRQ/preemption primitives are
// intentionally left as dependencies of the enclosing kernel translation.
#[macro_export]
macro_rules! raw_cpu_read { ($pcp:expr) => { unsafe { $crate::raw_cpu_generic_read($pcp) } }; }
#[macro_export]
macro_rules! raw_cpu_write { ($pcp:expr, $v:expr) => { unsafe { *$crate::arch_raw_cpu_ptr($pcp) = $v } }; }
#[macro_export]
macro_rules! raw_cpu_add { ($pcp:expr, $v:expr) => { unsafe { *$crate::arch_raw_cpu_ptr($pcp) += $v } }; }
#[macro_export]
macro_rules! raw_cpu_and { ($pcp:expr, $v:expr) => { unsafe { *$crate::arch_raw_cpu_ptr($pcp) &= $v } }; }
#[macro_export]
macro_rules! raw_cpu_or { ($pcp:expr, $v:expr) => { unsafe { *$crate::arch_raw_cpu_ptr($pcp) |= $v } }; }

// Width-specific aliases (the C header supplies 1, 2, 4, and 8 byte forms).
#[macro_export] macro_rules! raw_cpu_read_1 { ($p:expr) => { raw_cpu_read!($p) }; }
#[macro_export] macro_rules! raw_cpu_read_2 { ($p:expr) => { raw_cpu_read!($p) }; }
#[macro_export] macro_rules! raw_cpu_read_4 { ($p:expr) => { raw_cpu_read!($p) }; }
#[macro_export] macro_rules! raw_cpu_read_8 { ($p:expr) => { raw_cpu_read!($p) }; }
#[macro_export] macro_rules! raw_cpu_write_1 { ($p:expr,$v:expr) => { raw_cpu_write!($p,$v) }; }
#[macro_export] macro_rules! raw_cpu_write_2 { ($p:expr,$v:expr) => { raw_cpu_write!($p,$v) }; }
#[macro_export] macro_rules! raw_cpu_write_4 { ($p:expr,$v:expr) => { raw_cpu_write!($p,$v) }; }
#[macro_export] macro_rules! raw_cpu_write_8 { ($p:expr,$v:expr) => { raw_cpu_write!($p,$v) }; }

// Generic this_cpu operations disable interrupts around their raw operation.
#[macro_export]
macro_rules! this_cpu_generic_to_op { ($p:expr,$v:expr,$op:tt) => {{
    let _flags: usize = 0; let _ = _flags;
    match stringify!($op) { "=" => unsafe { *$crate::arch_raw_cpu_ptr($p) = $v },
        "+=" => unsafe { *$crate::arch_raw_cpu_ptr($p) += $v },
        "&=" => unsafe { *$crate::arch_raw_cpu_ptr($p) &= $v },
        "|=" => unsafe { *$crate::arch_raw_cpu_ptr($p) |= $v }, _ => {} }
}}; }
#[macro_export] macro_rules! this_cpu_read_1 { ($p:expr) => { raw_cpu_read!($p) }; }
#[macro_export] macro_rules! this_cpu_read_2 { ($p:expr) => { raw_cpu_read!($p) }; }
#[macro_export] macro_rules! this_cpu_read_4 { ($p:expr) => { raw_cpu_read!($p) }; }
#[macro_export] macro_rules! this_cpu_read_8 { ($p:expr) => { raw_cpu_read!($p) }; }
#[macro_export] macro_rules! this_cpu_write_1 { ($p:expr,$v:expr) => { this_cpu_generic_to_op!($p,$v,=) }; }
#[macro_export] macro_rules! this_cpu_write_2 { ($p:expr,$v:expr) => { this_cpu_generic_to_op!($p,$v,=) }; }
#[macro_export] macro_rules! this_cpu_write_4 { ($p:expr,$v:expr) => { this_cpu_generic_to_op!($p,$v,=) }; }
#[macro_export] macro_rules! this_cpu_write_8 { ($p:expr,$v:expr) => { this_cpu_generic_to_op!($p,$v,=) }; }

#[macro_export] macro_rules! raw_cpu_cmpxchg_1 { ($p:expr,$o:expr,$n:expr) => { unsafe { $crate::raw_cpu_generic_cmpxchg($p,$o,$n) } }; }
#[macro_export] macro_rules! raw_cpu_cmpxchg_2 { ($p:expr,$o:expr,$n:expr) => { raw_cpu_cmpxchg_1!($p,$o,$n) }; }
#[macro_export] macro_rules! raw_cpu_cmpxchg_4 { ($p:expr,$o:expr,$n:expr) => { raw_cpu_cmpxchg_1!($p,$o,$n) }; }
#[macro_export] macro_rules! raw_cpu_cmpxchg_8 { ($p:expr,$o:expr,$n:expr) => { raw_cpu_cmpxchg_1!($p,$o,$n) }; }
#[macro_export] macro_rules! this_cpu_cmpxchg_1 { ($p:expr,$o:expr,$n:expr) => { raw_cpu_cmpxchg_1!($p,$o,$n) }; }
#[macro_export] macro_rules! this_cpu_cmpxchg_2 { ($p:expr,$o:expr,$n:expr) => { this_cpu_cmpxchg_1!($p,$o,$n) }; }
#[macro_export] macro_rules! this_cpu_cmpxchg_4 { ($p:expr,$o:expr,$n:expr) => { this_cpu_cmpxchg_1!($p,$o,$n) }; }
#[macro_export] macro_rules! this_cpu_cmpxchg_8 { ($p:expr,$o:expr,$n:expr) => { this_cpu_cmpxchg_1!($p,$o,$n) }; }

#[macro_export] macro_rules! raw_cpu_add_return_1 { ($p:expr,$v:expr) => { unsafe { $crate::raw_cpu_generic_add_return($p,$v) } }; }
#[macro_export] macro_rules! raw_cpu_add_return_2 { ($p:expr,$v:expr) => { raw_cpu_add_return_1!($p,$v) }; }
#[macro_export] macro_rules! raw_cpu_add_return_4 { ($p:expr,$v:expr) => { raw_cpu_add_return_1!($p,$v) }; }
#[macro_export] macro_rules! raw_cpu_add_return_8 { ($p:expr,$v:expr) => { raw_cpu_add_return_1!($p,$v) }; }
#[macro_export] macro_rules! raw_cpu_xchg_1 { ($p:expr,$v:expr) => { unsafe { $crate::raw_cpu_generic_xchg($p,$v) } }; }
#[macro_export] macro_rules! raw_cpu_xchg_2 { ($p:expr,$v:expr) => { raw_cpu_xchg_1!($p,$v) }; }
#[macro_export] macro_rules! raw_cpu_xchg_4 { ($p:expr,$v:expr) => { raw_cpu_xchg_1!($p,$v) }; }
#[macro_export] macro_rules! raw_cpu_xchg_8 { ($p:expr,$v:expr) => { raw_cpu_xchg_1!($p,$v) }; }
#[macro_export] macro_rules! raw_cpu_try_cmpxchg_1 { ($p:expr,$o:expr,$n:expr) => { unsafe { $crate::raw_cpu_generic_try_cmpxchg($p,$o,$n) } }; }
#[macro_export] macro_rules! raw_cpu_try_cmpxchg_2 { ($p:expr,$o:expr,$n:expr) => { raw_cpu_try_cmpxchg_1!($p,$o,$n) }; }
#[macro_export] macro_rules! raw_cpu_try_cmpxchg_4 { ($p:expr,$o:expr,$n:expr) => { raw_cpu_try_cmpxchg_1!($p,$o,$n) }; }
#[macro_export] macro_rules! raw_cpu_try_cmpxchg_8 { ($p:expr,$o:expr,$n:expr) => { raw_cpu_try_cmpxchg_1!($p,$o,$n) }; }
#[macro_export] macro_rules! this_cpu_add_1 { ($p:expr,$v:expr) => { this_cpu_generic_to_op!($p,$v,+=) }; }
#[macro_export] macro_rules! this_cpu_add_2 { ($p:expr,$v:expr) => { this_cpu_add_1!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_add_4 { ($p:expr,$v:expr) => { this_cpu_add_1!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_add_8 { ($p:expr,$v:expr) => { this_cpu_add_1!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_and_1 { ($p:expr,$v:expr) => { this_cpu_generic_to_op!($p,$v,&=) }; }
#[macro_export] macro_rules! this_cpu_and_2 { ($p:expr,$v:expr) => { this_cpu_and_1!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_and_4 { ($p:expr,$v:expr) => { this_cpu_and_1!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_and_8 { ($p:expr,$v:expr) => { this_cpu_and_1!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_or_1 { ($p:expr,$v:expr) => { this_cpu_generic_to_op!($p,$v,|=) }; }
#[macro_export] macro_rules! this_cpu_or_2 { ($p:expr,$v:expr) => { this_cpu_or_1!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_or_4 { ($p:expr,$v:expr) => { this_cpu_or_1!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_or_8 { ($p:expr,$v:expr) => { this_cpu_or_1!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_add_return_1 { ($p:expr,$v:expr) => { raw_cpu_add_return_1!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_add_return_2 { ($p:expr,$v:expr) => { this_cpu_add_return_1!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_add_return_4 { ($p:expr,$v:expr) => { this_cpu_add_return_1!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_add_return_8 { ($p:expr,$v:expr) => { this_cpu_add_return_1!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_xchg_1 { ($p:expr,$v:expr) => { raw_cpu_xchg_1!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_xchg_2 { ($p:expr,$v:expr) => { this_cpu_xchg_1!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_xchg_4 { ($p:expr,$v:expr) => { this_cpu_xchg_1!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_xchg_8 { ($p:expr,$v:expr) => { this_cpu_xchg_1!($p,$v) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
