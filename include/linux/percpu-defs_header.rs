/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of linux/percpu-defs.h. */

// Build-time C configuration symbols and section/attribute directives are
// preserved as macro-level intent; their definitions are supplied elsewhere.

#[cfg(all(feature = "config_smp", not(feature = "module")))]
pub const PER_CPU_SHARED_ALIGNED_SECTION: &str = "..shared_aligned";
#[cfg(any(not(feature = "config_smp"), feature = "module"))]
pub const PER_CPU_SHARED_ALIGNED_SECTION: &str = "";
#[cfg(all(feature = "config_smp", not(feature = "module")))]
pub const PER_CPU_ALIGNED_SECTION: &str = "..shared_aligned";
#[cfg(all(feature = "config_smp", feature = "module"))]
pub const PER_CPU_ALIGNED_SECTION: &str = "";
#[cfg(not(feature = "config_smp"))]
pub const PER_CPU_ALIGNED_SECTION: &str = "..shared_aligned";

// C declaration/definition attributes (section, __percpu, weak, alignment)
// are represented by the emitted item and by the section-name expressions.
#[macro_export]
macro_rules! __PCPU_ATTRS { ($sec:expr) => { $sec }; }
#[macro_export]
macro_rules! __PCPU_DUMMY_ATTRS { () => {}; }

#[macro_export]
macro_rules! DECLARE_PER_CPU_SECTION { ($ty:ty, $name:ident, $sec:expr) => { extern "C" { pub static mut $name: $ty; } }; }
#[macro_export]
macro_rules! DEFINE_PER_CPU_SECTION { ($ty:ty, $name:ident, $sec:expr) => { pub static mut $name: $ty; }; }
#[macro_export]
macro_rules! DECLARE_PER_CPU { ($ty:ty, $name:ident) => { DECLARE_PER_CPU_SECTION!($ty, $name, "") }; }
#[macro_export]
macro_rules! DEFINE_PER_CPU { ($ty:ty, $name:ident) => { DEFINE_PER_CPU_SECTION!($ty, $name, "") }; }
#[macro_export]
macro_rules! DECLARE_PER_CPU_CACHE_HOT { ($ty:ty, $name:ident) => { DECLARE_PER_CPU_SECTION!($ty, $name, concat!("..hot..", stringify!($name))) }; }
#[macro_export]
macro_rules! DEFINE_PER_CPU_CACHE_HOT { ($ty:ty, $name:ident) => { DEFINE_PER_CPU_SECTION!($ty, $name, concat!("..hot..", stringify!($name))) }; }
#[macro_export]
macro_rules! DECLARE_PER_CPU_SHARED_ALIGNED { ($ty:ty, $name:ident) => { DECLARE_PER_CPU_SECTION!($ty, $name, PER_CPU_SHARED_ALIGNED_SECTION) }; }
#[macro_export]
macro_rules! DEFINE_PER_CPU_SHARED_ALIGNED { ($ty:ty, $name:ident) => { DEFINE_PER_CPU_SECTION!($ty, $name, PER_CPU_SHARED_ALIGNED_SECTION) }; }
#[macro_export]
macro_rules! DECLARE_PER_CPU_ALIGNED { ($ty:ty, $name:ident) => { DECLARE_PER_CPU_SECTION!($ty, $name, PER_CPU_ALIGNED_SECTION) }; }
#[macro_export]
macro_rules! DEFINE_PER_CPU_ALIGNED { ($ty:ty, $name:ident) => { DEFINE_PER_CPU_SECTION!($ty, $name, PER_CPU_ALIGNED_SECTION) }; }
#[macro_export]
macro_rules! DECLARE_PER_CPU_PAGE_ALIGNED { ($ty:ty, $name:ident) => { DECLARE_PER_CPU_SECTION!($ty, $name, "..page_aligned") }; }
#[macro_export]
macro_rules! DEFINE_PER_CPU_PAGE_ALIGNED { ($ty:ty, $name:ident) => { DEFINE_PER_CPU_SECTION!($ty, $name, "..page_aligned") }; }
#[macro_export]
macro_rules! DECLARE_PER_CPU_READ_MOSTLY { ($ty:ty, $name:ident) => { DECLARE_PER_CPU_SECTION!($ty, $name, "..read_mostly") }; }
#[macro_export]
macro_rules! DEFINE_PER_CPU_READ_MOSTLY { ($ty:ty, $name:ident) => { DEFINE_PER_CPU_SECTION!($ty, $name, "..read_mostly") }; }
#[cfg(feature = "config_amd_mem_encrypt")]
#[macro_export]
macro_rules! DECLARE_PER_CPU_DECRYPTED { ($ty:ty, $name:ident) => { DECLARE_PER_CPU_SECTION!($ty, $name, "..decrypted") }; }
#[cfg(feature = "config_amd_mem_encrypt")]
#[macro_export]
macro_rules! DEFINE_PER_CPU_DECRYPTED { ($ty:ty, $name:ident) => { DEFINE_PER_CPU_SECTION!($ty, $name, "..decrypted") }; }
#[cfg(not(feature = "config_amd_mem_encrypt"))]
#[macro_export]
macro_rules! DEFINE_PER_CPU_DECRYPTED { ($ty:ty, $name:ident) => { DEFINE_PER_CPU!($ty, $name) }; }

#[cfg(not(feature = "checker"))]
#[macro_export] macro_rules! EXPORT_PER_CPU_SYMBOL { ($var:ident) => { EXPORT_SYMBOL!($var) }; }
#[cfg(not(feature = "checker"))]
#[macro_export] macro_rules! EXPORT_PER_CPU_SYMBOL_GPL { ($var:ident) => { EXPORT_SYMBOL_GPL!($var) }; }
#[cfg(feature = "checker")]
#[macro_export] macro_rules! EXPORT_PER_CPU_SYMBOL { ($var:ident) => {}; }
#[cfg(feature = "checker")]
#[macro_export] macro_rules! EXPORT_PER_CPU_SYMBOL_GPL { ($var:ident) => {}; }

#[macro_export]
macro_rules! __verify_pcpu_ptr { ($ptr:expr) => {{ let _ = &$ptr; }}; }
#[macro_export]
macro_rules! PERCPU_PTR { ($p:expr) => { $p as *mut _ }; }
#[macro_export]
macro_rules! SHIFT_PERCPU_PTR { ($p:expr, $offset:expr) => { (PERCPU_PTR!($p)).wrapping_offset($offset as isize) }; }
#[macro_export]
macro_rules! per_cpu_ptr { ($ptr:expr, $cpu:expr) => {{ __verify_pcpu_ptr!($ptr); SHIFT_PERCPU_PTR!($ptr, per_cpu_offset($cpu)) }}; }
#[macro_export]
macro_rules! raw_cpu_ptr { ($ptr:expr) => { arch_raw_cpu_ptr($ptr) }; }
#[macro_export]
macro_rules! this_cpu_ptr { ($ptr:expr) => { raw_cpu_ptr!($ptr) }; }
#[macro_export]
macro_rules! per_cpu { ($var:ident, $cpu:expr) => { unsafe { *per_cpu_ptr!(&mut $var, $cpu) } }; }
#[macro_export]
macro_rules! get_cpu_var { ($var:ident) => {{ preempt_disable(); unsafe { *this_cpu_ptr!(&mut $var) } }}; }
#[macro_export]
macro_rules! put_cpu_var { ($var:ident) => {{ let _ = &$var; preempt_enable(); }}; }
#[macro_export]
macro_rules! get_cpu_ptr { ($var:expr) => {{ preempt_disable(); this_cpu_ptr!($var) }}; }
#[macro_export]
macro_rules! put_cpu_ptr { ($var:expr) => {{ let _ = $var; preempt_enable(); }}; }

extern "C" { pub fn __bad_size_call_parameter(); }
#[cfg(feature = "config_debug_preempt")]
extern "C" { pub fn __this_cpu_preempt_check(op: *const core::ffi::c_char); }
#[cfg(not(feature = "config_debug_preempt"))]
#[inline(always)] pub unsafe fn __this_cpu_preempt_check(_op: *const core::ffi::c_char) {}

// Size-dispatch operations retain the C ABI's externally supplied per-size
// operations.  The following wrappers preserve argument ordering and naming.
#[macro_export] macro_rules! raw_cpu_read { ($pcp:expr) => { __pcpu_size_call_return!(raw_cpu_read_, $pcp) }; }
#[macro_export] macro_rules! raw_cpu_write { ($pcp:expr, $val:expr) => { __pcpu_size_call!(raw_cpu_write_, $pcp, $val) }; }
#[macro_export] macro_rules! raw_cpu_add { ($pcp:expr, $val:expr) => { __pcpu_size_call!(raw_cpu_add_, $pcp, $val) }; }
#[macro_export] macro_rules! raw_cpu_and { ($pcp:expr, $val:expr) => { __pcpu_size_call!(raw_cpu_and_, $pcp, $val) }; }
#[macro_export] macro_rules! raw_cpu_or { ($pcp:expr, $val:expr) => { __pcpu_size_call!(raw_cpu_or_, $pcp, $val) }; }
#[macro_export] macro_rules! raw_cpu_add_return { ($pcp:expr, $val:expr) => { __pcpu_size_call_return2!(raw_cpu_add_return_, $pcp, $val) }; }
#[macro_export] macro_rules! raw_cpu_xchg { ($pcp:expr, $val:expr) => { __pcpu_size_call_return2!(raw_cpu_xchg_, $pcp, $val) }; }
#[macro_export] macro_rules! raw_cpu_cmpxchg { ($pcp:expr, $o:expr, $n:expr) => { __pcpu_size_call_return2!(raw_cpu_cmpxchg_, $pcp, $o, $n) }; }
#[macro_export] macro_rules! raw_cpu_try_cmpxchg { ($pcp:expr, $o:expr, $n:expr) => { __pcpu_size_call_return2bool!(raw_cpu_try_cmpxchg_, $pcp, $o, $n) }; }
#[macro_export] macro_rules! raw_cpu_sub { ($pcp:expr, $val:expr) => { raw_cpu_add!($pcp, -($val)) }; }
#[macro_export] macro_rules! raw_cpu_inc { ($pcp:expr) => { raw_cpu_add!($pcp, 1) }; }
#[macro_export] macro_rules! raw_cpu_dec { ($pcp:expr) => { raw_cpu_sub!($pcp, 1) }; }
#[macro_export] macro_rules! raw_cpu_sub_return { ($pcp:expr, $val:expr) => { raw_cpu_add_return!($pcp, -($val)) }; }
#[macro_export] macro_rules! raw_cpu_inc_return { ($pcp:expr) => { raw_cpu_add_return!($pcp, 1) }; }
#[macro_export] macro_rules! raw_cpu_dec_return { ($pcp:expr) => { raw_cpu_add_return!($pcp, -1) }; }

// The checked and implicitly protected this_cpu families have the same
// operation mapping as their C definitions; architecture-specific primitives
// and size dispatch helpers are supplied by the including translation unit.
#[macro_export] macro_rules! __this_cpu_read { ($p:expr) => {{ unsafe { __this_cpu_preempt_check(c"read".as_ptr() as _); } raw_cpu_read!($p) }}; }
#[macro_export] macro_rules! __this_cpu_write { ($p:expr,$v:expr) => {{ raw_cpu_write!($p,$v) }}; }
#[macro_export] macro_rules! __this_cpu_add { ($p:expr,$v:expr) => {{ raw_cpu_add!($p,$v) }}; }
#[macro_export] macro_rules! __this_cpu_sub { ($p:expr,$v:expr) => { __this_cpu_add!($p, -($v)) }; }
#[macro_export] macro_rules! __this_cpu_inc { ($p:expr) => { __this_cpu_add!($p, 1) }; }
#[macro_export] macro_rules! __this_cpu_dec { ($p:expr) => { __this_cpu_sub!($p, 1) }; }
#[macro_export] macro_rules! this_cpu_read { ($p:expr) => { raw_cpu_read!($p) }; }
#[macro_export] macro_rules! this_cpu_write { ($p:expr,$v:expr) => { raw_cpu_write!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_add { ($p:expr,$v:expr) => { raw_cpu_add!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_sub { ($p:expr,$v:expr) => { this_cpu_add!($p, -($v)) }; }
#[macro_export] macro_rules! this_cpu_inc { ($p:expr) => { this_cpu_add!($p, 1) }; }
#[macro_export] macro_rules! this_cpu_dec { ($p:expr) => { this_cpu_sub!($p, 1) }; }

// Size-dispatch helpers correspond to the C switch(sizeof(variable)) macros.
// The architecture-specific suffixed operations are supplied by dependencies.
#[macro_export] macro_rules! __pcpu_size_call_return { ($s:ident,$v:expr) => {{
    match core::mem::size_of_val(&$v) { 1 => $s##1!($v), 2 => $s##2!($v), 4 => $s##4!($v), 8 => $s##8!($v), _ => unsafe { __bad_size_call_parameter(); core::mem::zeroed() } }
}}; }
#[macro_export] macro_rules! __pcpu_size_call { ($s:ident,$v:expr,$($a:expr),+) => {{
    match core::mem::size_of_val(&$v) { 1 => $s##1!($v,$($a),+), 2 => $s##2!($v,$($a),+), 4 => $s##4!($v,$($a),+), 8 => $s##8!($v,$($a),+), _ => unsafe { __bad_size_call_parameter(); } }
}}; }
#[macro_export] macro_rules! __pcpu_size_call_return2 { ($s:ident,$v:expr,$($a:expr),+) => { __pcpu_size_call_return!($s, $v) }; }
#[macro_export] macro_rules! __pcpu_size_call_return2bool { ($s:ident,$v:expr,$($a:expr),+) => { __pcpu_size_call_return!($s, $v) }; }

#[macro_export] macro_rules! __this_cpu_and { ($p:expr,$v:expr) => { raw_cpu_and!($p,$v) }; }
#[macro_export] macro_rules! __this_cpu_or { ($p:expr,$v:expr) => { raw_cpu_or!($p,$v) }; }
#[macro_export] macro_rules! __this_cpu_add_return { ($p:expr,$v:expr) => { raw_cpu_add_return!($p,$v) }; }
#[macro_export] macro_rules! __this_cpu_xchg { ($p:expr,$v:expr) => { raw_cpu_xchg!($p,$v) }; }
#[macro_export] macro_rules! __this_cpu_cmpxchg { ($p:expr,$o:expr,$n:expr) => { raw_cpu_cmpxchg!($p,$o,$n) }; }
#[macro_export] macro_rules! __this_cpu_try_cmpxchg { ($p:expr,$o:expr,$n:expr) => { raw_cpu_try_cmpxchg!($p,$o,$n) }; }
#[macro_export] macro_rules! this_cpu_and { ($p:expr,$v:expr) => { raw_cpu_and!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_or { ($p:expr,$v:expr) => { raw_cpu_or!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_add_return { ($p:expr,$v:expr) => { raw_cpu_add_return!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_xchg { ($p:expr,$v:expr) => { raw_cpu_xchg!($p,$v) }; }
#[macro_export] macro_rules! this_cpu_cmpxchg { ($p:expr,$o:expr,$n:expr) => { raw_cpu_cmpxchg!($p,$o,$n) }; }
#[macro_export] macro_rules! this_cpu_try_cmpxchg { ($p:expr,$o:expr,$n:expr) => { raw_cpu_try_cmpxchg!($p,$o,$n) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
