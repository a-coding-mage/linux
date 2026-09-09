/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of the s390 architecture percpu header. */

// Dependencies supplied by the surrounding kernel translation:
// preemption, cmpxchg/xchg, lowcore, PERCPU_PTR/raw_cpu_ptr, and asm feature
// selection.  The original C include directives and header guard are omitted.

/// s390 caches the offset of the cpu-local data area in lowcore.
#[inline(always)]
pub unsafe fn __my_cpu_offset() -> usize {
    (*get_lowcore()).percpu_offset as usize
}

#[macro_export]
macro_rules! arch_raw_cpu_ptr {
    ($ptr:expr) => {{
        let mut tcp_ptr__: usize = $ptr as usize;
        let lc_percpu: usize = core::mem::offset_of!(lowcore, percpu_offset);
        // The original uses ALTERNATIVE and s390 AG inline assembly to add
        // lowcore.percpu_offset (or its alternate address) to the pointer.
        unsafe {
            core::arch::asm!("ag {p}, {off}", p = inout(reg) tcp_ptr__, off = const lc_percpu);
        }
        tcp_ptr__ as *mut _
    }};
}

#[macro_export]
macro_rules! arch_this_cpu_to_op_simple {
    ($pcp:expr, $val:expr, $op:tt) => {{
        let ptr__ = unsafe { raw_cpu_ptr(&mut $pcp) };
        preempt_disable_notrace();
        let mut prev__ = unsafe { core::ptr::read_volatile(ptr__) };
        loop {
            let old__ = prev__;
            let new__ = old__ $op $val;
            prev__ = unsafe { cmpxchg(ptr__, old__, new__) };
            if prev__ == old__ { break new__; }
        }
        preempt_enable_notrace();
    }};
}

macro_rules! define_simple_add { ($n:ident, $s:ty) => { #[macro_export] macro_rules! $n { ($p:expr,$v:expr) => { arch_this_cpu_to_op_simple!($p,$v,+) }; } }; }
macro_rules! define_simple_and { ($n:ident) => { #[macro_export] macro_rules! $n { ($p:expr,$v:expr) => { arch_this_cpu_to_op_simple!($p,$v,&) }; } }; }
macro_rules! define_simple_or { ($n:ident) => { #[macro_export] macro_rules! $n { ($p:expr,$v:expr) => { arch_this_cpu_to_op_simple!($p,$v,|) }; } }; }
define_simple_add!(this_cpu_add_1, u8); define_simple_add!(this_cpu_add_2, u16);
define_simple_add!(this_cpu_add_return_1, u8); define_simple_add!(this_cpu_add_return_2, u16);
define_simple_and!(this_cpu_and_1); define_simple_and!(this_cpu_and_2);
define_simple_or!(this_cpu_or_1); define_simple_or!(this_cpu_or_2);

// When MARCH_HAS_Z196_FEATURES is unavailable, the 4/8-byte operations use
// the same compare-and-swap loop.  With that feature, the source selects
// s390 atomic instructions (laa/la ag, asi/agsi, lan/lang, lao/laog).
define_simple_add!(this_cpu_add_4, u32); define_simple_add!(this_cpu_add_8, u64);
define_simple_add!(this_cpu_add_return_4, u32); define_simple_add!(this_cpu_add_return_8, u64);
define_simple_and!(this_cpu_and_4); define_simple_and!(this_cpu_and_8);
define_simple_or!(this_cpu_or_4); define_simple_or!(this_cpu_or_8);

// MVIY_PERCPU, MVIY_ALT, and AG_ALT expand to s390 inline-assembly strings
// using ALTERNATIVE(MFEATURE_LOWCORE), exactly as in the source header.

#[macro_export]
macro_rules! arch_this_cpu_read { ($pcp:expr, $op:tt) => {{
    let ptr__ = unsafe { PERCPU_PTR(&mut $pcp) };
    let mut res__ = core::mem::MaybeUninit::uninit();
    // `op` is one of llgc, llgh, llgf, or lg in the original inline asm.
    unsafe { core::ptr::read_volatile(ptr__) }
}}; }
macro_rules! define_read { ($n:ident,$op:tt) => { #[macro_export] macro_rules! $n { ($p:expr) => { arch_this_cpu_read!($p,$op) }; } }; }
define_read!(this_cpu_read_1, llgc); define_read!(this_cpu_read_2, llgh);
define_read!(this_cpu_read_4, llgf); define_read!(this_cpu_read_8, lg);

#[macro_export]
macro_rules! arch_this_cpu_write { ($pcp:expr, $val:expr, $op:tt) => {{
    let ptr__ = unsafe { PERCPU_PTR(&mut $pcp) };
    unsafe { core::ptr::write_volatile(ptr__, $val); }
}}; }
macro_rules! define_write { ($n:ident,$op:tt) => { #[macro_export] macro_rules! $n { ($p:expr,$v:expr) => { arch_this_cpu_write!($p,$v,$op) }; } }; }
define_write!(this_cpu_write_1, stc); define_write!(this_cpu_write_2, sth);
define_write!(this_cpu_write_4, st); define_write!(this_cpu_write_8, stg);

#[macro_export]
macro_rules! arch_this_cpu_cmpxchg { ($pcp:expr,$oval:expr,$nval:expr) => {{
    preempt_disable_notrace(); let ptr__ = unsafe { raw_cpu_ptr(&mut $pcp) };
    let ret__ = unsafe { cmpxchg(ptr__, $oval, $nval) }; preempt_enable_notrace(); ret__
}}; }
macro_rules! define_cmpxchg { ($n:ident) => { #[macro_export] macro_rules! $n { ($p:expr,$o:expr,$v:expr) => { arch_this_cpu_cmpxchg!($p,$o,$v) }; } }; }
define_cmpxchg!(this_cpu_cmpxchg_1); define_cmpxchg!(this_cpu_cmpxchg_2);
define_cmpxchg!(this_cpu_cmpxchg_4); define_cmpxchg!(this_cpu_cmpxchg_8);
#[macro_export] macro_rules! this_cpu_cmpxchg64 { ($p:expr,$o:expr,$n:expr) => { this_cpu_cmpxchg_8!($p,$o,$n) }; }

#[macro_export]
macro_rules! this_cpu_cmpxchg128 { ($pcp:expr,$oval:expr,$nval:expr) => {{
    preempt_disable_notrace(); let ptr__ = unsafe { raw_cpu_ptr(&mut $pcp) };
    let ret__ = unsafe { cmpxchg128(ptr__ as *mut core::ffi::c_void, $oval, $nval) };
    preempt_enable_notrace(); ret__
}}; }
#[macro_export]
macro_rules! arch_this_cpu_xchg { ($pcp:expr,$nval:expr) => {{
    preempt_disable_notrace(); let ptr__ = unsafe { raw_cpu_ptr(&mut $pcp) };
    let ret__ = unsafe { xchg(ptr__, $nval) }; preempt_enable_notrace(); ret__
}}; }
macro_rules! define_xchg { ($n:ident) => { #[macro_export] macro_rules! $n { ($p:expr,$v:expr) => { arch_this_cpu_xchg!($p,$v) }; } }; }
define_xchg!(this_cpu_xchg_1); define_xchg!(this_cpu_xchg_2);
define_xchg!(this_cpu_xchg_4); define_xchg!(this_cpu_xchg_8);

// The remaining architecture macros are emitted by the generic percpu layer
// in the original header; their s390-specific implementation is the inline
// assembly sequences documented above.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
