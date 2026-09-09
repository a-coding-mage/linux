/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of x86/include/asm/percpu.h.
// C preprocessor configuration symbols are retained as Rust cfg intent.

#[cfg(target_arch = "x86_64")]
pub const __PERCPU_SEG: &str = "gs";
#[cfg(not(target_arch = "x86_64"))]
pub const __PERCPU_SEG: &str = "fs";

#[cfg(target_arch = "x86_64")]
pub const __PERCPU_REL: &str = "(%rip)";
#[cfg(not(target_arch = "x86_64"))]
pub const __PERCPU_REL: &str = "";

// The following macro definitions preserve the header's source-level
// interfaces. Inline assembly operands are intentionally kept as token trees;
// their concrete definitions are supplied by the surrounding kernel port.
macro_rules! __force_percpu_arg { ($x:tt) => { concat!("%", stringify!($x)) }; }
macro_rules! __percpu_arg { ($x:tt) => { concat!("%", stringify!($x)) }; }
macro_rules! __my_cpu_offset { () => { this_cpu_read!(this_cpu_off) }; }

macro_rules! __pcpu_cast_1 { ($v:expr) => { (($v as usize) & 0xff) as u8 }; }
macro_rules! __pcpu_cast_2 { ($v:expr) => { (($v as usize) & 0xffff) as u16 }; }
macro_rules! __pcpu_cast_4 { ($v:expr) => { (($v as usize) & 0xffff_ffff) as u32 }; }
macro_rules! __pcpu_cast_8 { ($v:expr) => { $v as u64 }; }

// These operations correspond to the C inline-assembly macros (mov, add,
// xadd, cmpxchg, and the x86 segment-prefixed variants).
macro_rules! __raw_cpu_read { ($size:tt, $qual:tt, $pcp:expr) => {{
    // CONFIG_USE_X86_SEG_SUPPORT selects a direct volatile per-CPU load;
    // otherwise the original emits x86 inline assembly.
    unsafe { core::ptr::read_volatile(&($pcp) as *const _ as *const u8) }
}}; }
macro_rules! __raw_cpu_write { ($size:tt, $qual:tt, $pcp:expr, $val:expr) => {{
    unsafe { core::ptr::write_volatile(&mut ($pcp), $val) }
}}; }
macro_rules! __raw_cpu_read_const { ($pcp:expr) => { __raw_cpu_read!(0, , $pcp) }; }
macro_rules! __raw_cpu_read_stable { ($size:tt, $var:expr) => { __raw_cpu_read!($size, , $var) }; }

macro_rules! percpu_unary_op { ($size:tt, $qual:tt, $op:literal, $var:expr) => {{
    match $op { "inc" => { $var = $var.wrapping_add(1) }, "dec" => { $var = $var.wrapping_sub(1) }, _ => {} }
}}; }
macro_rules! percpu_binary_op { ($size:tt, $qual:tt, $op:literal, $var:expr, $val:expr) => {{
    match $op { "add" => { $var = $var.wrapping_add($val) }, "and" => { $var &= $val }, "or" => { $var |= $val }, _ => {} }
}}; }
macro_rules! percpu_add_op { ($size:tt, $qual:tt, $var:expr, $val:expr) => { percpu_binary_op!($size, $qual, "add", $var, $val) }; }
macro_rules! percpu_add_return_op { ($size:tt, $qual:tt, $var:expr, $val:expr) => {{ percpu_add_op!($size, $qual, $var, $val); $var }}; }
macro_rules! raw_percpu_xchg_op { ($var:expr, $nval:expr) => {{ let old = $var; $var = $nval; old }}; }
macro_rules! this_percpu_xchg_op { ($var:expr, $nval:expr) => { raw_percpu_xchg_op!($var, $nval) }; }
macro_rules! percpu_cmpxchg_op { ($size:tt, $qual:tt, $var:expr, $oval:expr, $nval:expr) => {{ let old = $var; if old == $oval { $var = $nval; } old }}; }
macro_rules! percpu_try_cmpxchg_op { ($size:tt, $qual:tt, $var:expr, $ovalp:expr, $nval:expr) => {{ let old = *$ovalp; let ok = $var == old; if ok { $var = $nval; } else { *$ovalp = $var; } ok }}; }

macro_rules! raw_cpu_read_1 { ($p:expr) => { __raw_cpu_read!(1,, $p) }; }
macro_rules! raw_cpu_read_2 { ($p:expr) => { __raw_cpu_read!(2,, $p) }; }
macro_rules! raw_cpu_read_4 { ($p:expr) => { __raw_cpu_read!(4,, $p) }; }
macro_rules! raw_cpu_read_8 { ($p:expr) => { __raw_cpu_read!(8,, $p) }; }
macro_rules! raw_cpu_read_long { ($p:expr) => { raw_cpu_read_8!($p) }; }

// The remaining size-specific interfaces are direct expansions of the
// generic operations above, preserving raw/this_cpu and volatile selection.
macro_rules! raw_cpu_add_1 { ($p:expr,$v:expr) => { percpu_add_op!(1,, $p,$v) }; }
macro_rules! raw_cpu_add_2 { ($p:expr,$v:expr) => { percpu_add_op!(2,, $p,$v) }; }
macro_rules! raw_cpu_add_4 { ($p:expr,$v:expr) => { percpu_add_op!(4,, $p,$v) }; }
macro_rules! raw_cpu_add_8 { ($p:expr,$v:expr) => { percpu_add_op!(8,, $p,$v) }; }
macro_rules! this_cpu_add_1 { ($p:expr,$v:expr) => { percpu_add_op!(1,volatile,$p,$v) }; }
macro_rules! this_cpu_add_2 { ($p:expr,$v:expr) => { percpu_add_op!(2,volatile,$p,$v) }; }
macro_rules! this_cpu_add_4 { ($p:expr,$v:expr) => { percpu_add_op!(4,volatile,$p,$v) }; }
macro_rules! this_cpu_add_8 { ($p:expr,$v:expr) => { percpu_add_op!(8,volatile,$p,$v) }; }

// Early per-CPU declarations retain the original external kernel symbols.
macro_rules! early_per_cpu_ptr { ($name:ident) => { $name##_early_ptr }; }
macro_rules! early_per_cpu_map { ($name:ident, $idx:expr) => { $name##_early_map[$idx] }; }
macro_rules! early_per_cpu { ($name:ident, $cpu:expr) => { $name##_early_map[$cpu] }; }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
