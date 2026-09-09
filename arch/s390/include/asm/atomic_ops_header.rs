/* SPDX-License-Identifier: GPL-2.0 */
/* Low level function for atomic operations. Copyright IBM Corp. 1999, 2016 */

// C dependencies: linux/limits.h, asm/march.h, and asm/asm.h.
// `MARCH_HAS_Z196_FEATURES` and `__HAVE_ASM_FLAG_OUTPUTS__` are build-time C
// conditions; the corresponding Rust cfg features preserve their intent.

#[inline(always)]
pub unsafe fn __atomic_read(ptr: *const i32) -> i32 {
    let mut val: i32;
    core::arch::asm!("l {val},[{ptr}]", val = out(reg) val, ptr = in(reg) ptr, options(nostack));
    val
}

#[inline(always)]
pub unsafe fn __atomic_set(ptr: *mut i32, val: i32) {
    if val >= i16::MIN as i32 && val <= i16::MAX as i32 {
        core::arch::asm!("mvhi [{ptr}],{val}", ptr = in(reg) ptr, val = const val);
    } else {
        core::arch::asm!("st {val},[{ptr}]", val = in(reg) val, ptr = in(reg) ptr);
    }
}

#[inline(always)]
pub unsafe fn __atomic64_read(ptr: *const i64) -> i64 {
    let mut val: i64;
    core::arch::asm!("lg {val},[{ptr}]", val = out(reg) val, ptr = in(reg) ptr, options(nostack));
    val
}

#[inline(always)]
pub unsafe fn __atomic64_set(ptr: *mut i64, val: i64) {
    if val >= i16::MIN as i64 && val <= i16::MAX as i64 {
        core::arch::asm!("mvghi [{ptr}],{val}", ptr = in(reg) ptr, val = const val);
    } else {
        core::arch::asm!("stg {val},[{ptr}]", val = in(reg) val, ptr = in(reg) ptr);
    }
}

#[cfg(feature = "MARCH_HAS_Z196_FEATURES")]
macro_rules! atomic_op {
    ($name:ident, $ty:ty, $op:literal, $barrier:literal) => {
        #[inline(always)]
        pub unsafe fn $name(val: $ty, ptr: *mut $ty) -> $ty {
            let mut old: $ty;
            core::arch::asm!(concat!($op, " {old},{val},[{ptr}]", $barrier),
                old = lateout(reg) old, val = in(reg) val, ptr = in(reg) ptr,
                options(nostack));
            old
        }
    };
}

#[cfg(feature = "MARCH_HAS_Z196_FEATURES")]
macro_rules! atomic_ops {
    ($name:ident, $ty:ty, $op:literal) => {
        atomic_op!($name, $ty, $op, "");
        atomic_op!(concat_idents!($name, _barrier), $ty, $op, "\\nbcr 14,0");
    };
}

// The names below are expanded by the C preprocessor in the original header.
// Rust identifiers are emitted explicitly to retain the public interface.
#[cfg(feature = "MARCH_HAS_Z196_FEATURES")]
macro_rules! z196_atomic_functions {
    ($ty:ty, $read:ident, $and:ident, $or:ident, $xor:ident, $a:literal, $n:literal, $o:literal, $x:literal) => {
        atomic_op!($read, $ty, $a, ""); atomic_op!($and, $ty, $n, "");
        atomic_op!($or, $ty, $o, ""); atomic_op!($xor, $ty, $x, "");
    };
}

// Z196 forms (barrier variants are kept as separate declarations).
#[cfg(feature = "MARCH_HAS_Z196_FEATURES")]
z196_atomic_functions!(i32, __atomic_add, __atomic_and, __atomic_or, __atomic_xor, "laa", "lan", "lao", "lax");
#[cfg(feature = "MARCH_HAS_Z196_FEATURES")]
z196_atomic_functions!(i64, __atomic64_add, __atomic64_and, __atomic64_or, __atomic64_xor, "laag", "lang", "laog", "laxg");

#[cfg(feature = "MARCH_HAS_Z196_FEATURES")]
atomic_op!(__atomic_add_barrier, i32, "laa", "\\nbcr 14,0");
#[cfg(feature = "MARCH_HAS_Z196_FEATURES")]
atomic_op!(__atomic_and_barrier, i32, "lan", "\\nbcr 14,0");
#[cfg(feature = "MARCH_HAS_Z196_FEATURES")]
atomic_op!(__atomic_or_barrier, i32, "lao", "\\nbcr 14,0");
#[cfg(feature = "MARCH_HAS_Z196_FEATURES")]
atomic_op!(__atomic_xor_barrier, i32, "lax", "\\nbcr 14,0");
#[cfg(feature = "MARCH_HAS_Z196_FEATURES")]
atomic_op!(__atomic64_add_barrier, i64, "laag", "\\nbcr 14,0");
#[cfg(feature = "MARCH_HAS_Z196_FEATURES")]
atomic_op!(__atomic64_and_barrier, i64, "lang", "\\nbcr 14,0");
#[cfg(feature = "MARCH_HAS_Z196_FEATURES")]
atomic_op!(__atomic64_or_barrier, i64, "laog", "\\nbcr 14,0");
#[cfg(feature = "MARCH_HAS_Z196_FEATURES")]
atomic_op!(__atomic64_xor_barrier, i64, "laxg", "\\nbcr 14,0");

#[cfg(not(feature = "MARCH_HAS_Z196_FEATURES"))]
macro_rules! legacy_atomic_op {
    ($name:ident, $ty:ty, $op:literal) => {
        #[inline(always)]
        pub unsafe fn $name(val: $ty, ptr: *mut $ty) -> $ty {
            let mut old: $ty;
            let mut new: $ty;
            core::arch::asm!(concat!("0: lr {new},{old}\\n", $op, " {new},{val}\\ncs {old},{new},[{ptr}]\\njl 0b"),
                old = inout(reg) *ptr => old, new = lateout(reg) new,
                val = in(reg) val, ptr = in(reg) ptr, options(nostack));
            old
        }
    };
}

// The legacy definitions use compare-and-swap retry loops, as in the source.
#[cfg(not(feature = "MARCH_HAS_Z196_FEATURES"))]
legacy_atomic_op!(__atomic_add, i32, "ar");
#[cfg(not(feature = "MARCH_HAS_Z196_FEATURES"))]
legacy_atomic_op!(__atomic_and, i32, "nr");
#[cfg(not(feature = "MARCH_HAS_Z196_FEATURES"))]
legacy_atomic_op!(__atomic_or, i32, "or");
#[cfg(not(feature = "MARCH_HAS_Z196_FEATURES"))]
legacy_atomic_op!(__atomic_xor, i32, "xr");
#[cfg(not(feature = "MARCH_HAS_Z196_FEATURES"))]
legacy_atomic_op!(__atomic64_add, i64, "agr");
#[cfg(not(feature = "MARCH_HAS_Z196_FEATURES"))]
legacy_atomic_op!(__atomic64_and, i64, "ngr");
#[cfg(not(feature = "MARCH_HAS_Z196_FEATURES"))]
legacy_atomic_op!(__atomic64_or, i64, "ogr");
#[cfg(not(feature = "MARCH_HAS_Z196_FEATURES"))]
legacy_atomic_op!(__atomic64_xor, i64, "xgr");

#[cfg(not(feature = "MARCH_HAS_Z196_FEATURES"))]
legacy_atomic_op!(__atomic_add_barrier, i32, "ar");
#[cfg(not(feature = "MARCH_HAS_Z196_FEATURES"))]
legacy_atomic_op!(__atomic64_add_barrier, i64, "agr");

#[inline(always)]
pub unsafe fn __atomic_add_const(val: i32, ptr: *mut i32) { let _ = __atomic_add(val, ptr); }
#[inline(always)]
pub unsafe fn __atomic_add_const_barrier(val: i32, ptr: *mut i32) { let _ = __atomic_add(val, ptr); }
#[inline(always)]
pub unsafe fn __atomic64_add_const(val: i64, ptr: *mut i64) { let _ = __atomic64_add(val, ptr); }
#[inline(always)]
pub unsafe fn __atomic64_add_const_barrier(val: i64, ptr: *mut i64) { let _ = __atomic64_add(val, ptr); }

#[inline(always)]
pub unsafe fn __atomic_add_and_test(val: i32, ptr: *mut i32) -> bool { __atomic_add(val, ptr) == -val }
#[inline(always)]
pub unsafe fn __atomic_add_and_test_barrier(val: i32, ptr: *mut i32) -> bool { __atomic_add_barrier(val, ptr) == -val }
#[inline(always)]
pub unsafe fn __atomic_add_const_and_test(val: i32, ptr: *mut i32) -> bool { __atomic_add(val, ptr) == -val }
#[inline(always)]
pub unsafe fn __atomic_add_const_and_test_barrier(val: i32, ptr: *mut i32) -> bool { __atomic_add_barrier(val, ptr) == -val }
#[inline(always)]
pub unsafe fn __atomic64_add_and_test(val: i64, ptr: *mut i64) -> bool { __atomic64_add(val, ptr) == -val }
#[inline(always)]
pub unsafe fn __atomic64_add_and_test_barrier(val: i64, ptr: *mut i64) -> bool { __atomic64_add_barrier(val, ptr) == -val }
#[inline(always)]
pub unsafe fn __atomic64_add_const_and_test(val: i64, ptr: *mut i64) -> bool { __atomic64_add(val, ptr) == -val }
#[inline(always)]
pub unsafe fn __atomic64_add_const_and_test_barrier(val: i64, ptr: *mut i64) -> bool { __atomic64_add_barrier(val, ptr) == -val }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
