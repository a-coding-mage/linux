/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

/* Translated from lib/bpf/bpf_core_read.h. */
/* C dependency intent: #include "bpf_helpers.h" */

/*
 * enum bpf_field_info_kind is passed as a second argument into
 * __builtin_preserve_field_info() built-in to get a specific aspect of
 * a field, captured as a first argument. __builtin_preserve_field_info(field,
 * info_kind) returns __u32 integer and produces BTF field relocation, which
 * is understood and processed by libbpf during BPF object loading. See
 * selftests/bpf for examples.
 */
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum bpf_field_info_kind {
    BPF_FIELD_BYTE_OFFSET = 0, /* field byte offset */
    BPF_FIELD_BYTE_SIZE = 1,
    BPF_FIELD_EXISTS = 2, /* field existence in target kernel */
    BPF_FIELD_SIGNED = 3,
    BPF_FIELD_LSHIFT_U64 = 4,
    BPF_FIELD_RSHIFT_U64 = 5,
}

/* second argument to __builtin_btf_type_id() built-in */
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum bpf_type_id_kind {
    BPF_TYPE_ID_LOCAL = 0,  /* BTF type ID in local program */
    BPF_TYPE_ID_TARGET = 1, /* BTF type ID in target kernel */
}

/* second argument to __builtin_preserve_type_info() built-in */
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum bpf_type_info_kind {
    BPF_TYPE_EXISTS = 0, /* type existence in target kernel */
    BPF_TYPE_SIZE = 1,   /* type size in target kernel */
    BPF_TYPE_MATCHES = 2, /* type match in target kernel */
}

/* second argument to __builtin_preserve_enum_value() built-in */
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum bpf_enum_value_kind {
    BPF_ENUMVAL_EXISTS = 0, /* enum value existence in kernel */
    BPF_ENUMVAL_VALUE = 1,  /* enum value value relocation */
}

macro_rules! __CORE_RELO {
    ($src:expr, $field:tt, BYTE_OFFSET) => {
        __builtin_preserve_field_info!((*$src).$field, bpf_field_info_kind::BPF_FIELD_BYTE_OFFSET)
    };
    ($src:expr, $field:tt, BYTE_SIZE) => {
        __builtin_preserve_field_info!((*$src).$field, bpf_field_info_kind::BPF_FIELD_BYTE_SIZE)
    };
    ($src:expr, $field:tt, EXISTS) => {
        __builtin_preserve_field_info!((*$src).$field, bpf_field_info_kind::BPF_FIELD_EXISTS)
    };
    ($src:expr, $field:tt, SIGNED) => {
        __builtin_preserve_field_info!((*$src).$field, bpf_field_info_kind::BPF_FIELD_SIGNED)
    };
    ($src:expr, $field:tt, LSHIFT_U64) => {
        __builtin_preserve_field_info!((*$src).$field, bpf_field_info_kind::BPF_FIELD_LSHIFT_U64)
    };
    ($src:expr, $field:tt, RSHIFT_U64) => {
        __builtin_preserve_field_info!((*$src).$field, bpf_field_info_kind::BPF_FIELD_RSHIFT_U64)
    };
}

/*
 * C preprocessor condition:
 * #if __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__
 */
#[cfg(target_endian = "little")]
macro_rules! __CORE_BITFIELD_PROBE_READ {
    ($dst:expr, $src:expr, $fld:tt) => {
        bpf_probe_read_kernel(
            $dst as *mut core::ffi::c_void,
            __CORE_RELO!($src, $fld, BYTE_SIZE) as usize,
            ($src as *const u8).wrapping_add(__CORE_RELO!($src, $fld, BYTE_OFFSET) as usize)
                as *const core::ffi::c_void,
        )
    };
}

/*
 * semantics of LSHIFT_64 assumes loading values into low-ordered bytes, so
 * for big-endian we need to adjust destination pointer accordingly, based on
 * field byte size
 */
#[cfg(target_endian = "big")]
macro_rules! __CORE_BITFIELD_PROBE_READ {
    ($dst:expr, $src:expr, $fld:tt) => {
        bpf_probe_read_kernel(
            ($dst as *mut u8).wrapping_add(8 - __CORE_RELO!($src, $fld, BYTE_SIZE) as usize)
                as *mut core::ffi::c_void,
            __CORE_RELO!($src, $fld, BYTE_SIZE) as usize,
            ($src as *const u8).wrapping_add(__CORE_RELO!($src, $fld, BYTE_OFFSET) as usize)
                as *const core::ffi::c_void,
        )
    };
}

/*
 * Extract bitfield, identified by s->field, and return its value as u64.
 * All this is done in relocatable manner, so bitfield changes such as
 * signedness, bit size, offset changes, this will be handled automatically.
 * This version of macro is using bpf_probe_read_kernel() to read underlying
 * integer storage. Macro functions as an expression and its return type is
 * bpf_probe_read_kernel()'s return value: 0, on success, <0 on error.
 */
macro_rules! BPF_CORE_READ_BITFIELD_PROBED {
    ($s:expr, $field:tt) => {{
        let mut val: u64 = 0;

        __CORE_BITFIELD_PROBE_READ!(&mut val as *mut u64, $s, $field);
        val = val.wrapping_shl(__CORE_RELO!($s, $field, LSHIFT_U64) as u32);
        if __CORE_RELO!($s, $field, SIGNED) != 0 {
            val = ((val as i64) >> (__CORE_RELO!($s, $field, RSHIFT_U64) as u32)) as u64;
        } else {
            val >>= __CORE_RELO!($s, $field, RSHIFT_U64) as u32;
        }
        val
    }};
}

/*
 * Extract bitfield, identified by s->field, and return its value as u64.
 * This version of macro is using direct memory reads and should be used from
 * BPF program types that support such functionality (e.g., typed raw
 * tracepoints).
 */
macro_rules! BPF_CORE_READ_BITFIELD {
    ($s:expr, $field:tt) => {{
        let p = ($s as *const u8).wrapping_add(__CORE_RELO!($s, $field, BYTE_OFFSET) as usize)
            as *const core::ffi::c_void;
        let mut val: u64;

        /*
         * This is a so-called barrier_var() operation that makes specified
         * variable "a black box" for optimizing compiler.
         * It forces compiler to perform BYTE_OFFSET relocation on p and use
         * its calculated value in the switch below, instead of applying
         * the same relocation 4 times for each individual memory load.
         */
        core::arch::asm!("", inout(reg) p => _);

        match __CORE_RELO!($s, $field, BYTE_SIZE) {
            1 => val = *(p as *const u8) as u64,
            2 => val = *(p as *const u16) as u64,
            4 => val = *(p as *const u32) as u64,
            8 => val = *(p as *const u64),
            _ => val = 0,
        }
        val = val.wrapping_shl(__CORE_RELO!($s, $field, LSHIFT_U64) as u32);
        if __CORE_RELO!($s, $field, SIGNED) != 0 {
            val = ((val as i64) >> (__CORE_RELO!($s, $field, RSHIFT_U64) as u32)) as u64;
        } else {
            val >>= __CORE_RELO!($s, $field, RSHIFT_U64) as u32;
        }
        val
    }};
}

/*
 * Write to a bitfield, identified by s->field.
 * This is the inverse of BPF_CORE_WRITE_BITFIELD().
 */
macro_rules! BPF_CORE_WRITE_BITFIELD {
    ($s:expr, $field:tt, $new_val:expr) => {{
        let mut p = ($s as *mut u8).wrapping_add(__CORE_RELO!($s, $field, BYTE_OFFSET) as usize);
        let byte_size: u32 = __CORE_RELO!($s, $field, BYTE_SIZE) as u32;
        let lshift: u32 = __CORE_RELO!($s, $field, LSHIFT_U64) as u32;
        let rshift: u32 = __CORE_RELO!($s, $field, RSHIFT_U64) as u32;
        let mut mask: u64;
        let mut val: u64 = 0;
        let nval: u64 = $new_val as u64;
        let rpad: u32 = rshift.wrapping_sub(lshift);

        core::arch::asm!("", inout(reg) p);

        match byte_size {
            1 => val = *(p as *mut u8) as u64,
            2 => val = *(p as *mut u16) as u64,
            4 => val = *(p as *mut u32) as u64,
            8 => val = *(p as *mut u64),
            _ => {}
        }

        mask = (!0u64).wrapping_shl(rshift) >> lshift;
        val = (val & !mask) | (nval.wrapping_shl(rpad) & mask);

        match byte_size {
            1 => *(p as *mut u8) = val as u8,
            2 => *(p as *mut u16) = val as u16,
            4 => *(p as *mut u32) = val as u32,
            8 => *(p as *mut u64) = val,
            _ => {}
        }
    }};
}

/*
 * Differentiator between compilers builtin implementations. This is a
 * requirement due to the compiler parsing differences where GCC optimizes
 * early in parsing those constructs of type pointers to the builtin specific
 * type, resulting in not being possible to collect the required type
 * information in the builtin expansion.
 *
 * Rust has no file-local equivalent for typeof(), __COUNTER__, or the BPF
 * compiler builtins used below. These macros preserve the public macro names
 * and delegate to same-named builtin-like macros supplied by the embedding
 * BPF Rust environment.
 */
macro_rules! ___bpf_typeof {
    ($type_:ty) => {
        core::ptr::null::<$type_>()
    };
    ($type_:expr) => {
        __bpf_typeof_expr!($type_)
    };
}

macro_rules! ___bpf_field_ref1 {
    ($field:expr) => {
        $field
    };
}

macro_rules! ___bpf_field_ref2 {
    ($type_:ty, $field:tt) => {
        (*___bpf_typeof!($type_)).$field
    };
}

macro_rules! ___bpf_field_ref {
    ($field:expr) => {
        ___bpf_field_ref1!($field)
    };
    ($type_:ty, $field:tt) => {
        ___bpf_field_ref2!($type_, $field)
    };
}

/*
 * Convenience macro to check that field actually exists in target kernel's.
 * Returns:
 *    1, if matching field is present in target kernel;
 *    0, if no matching field found.
 *
 * Supports two forms:
 *   - field reference through variable access:
 *     bpf_core_field_exists(p->my_field);
 *   - field reference through type and field names:
 *     bpf_core_field_exists(struct my_type, my_field).
 */
macro_rules! bpf_core_field_exists {
    ($($field:tt)+) => {
        __builtin_preserve_field_info!(
            ___bpf_field_ref!($($field)+),
            bpf_field_info_kind::BPF_FIELD_EXISTS
        )
    };
}

/*
 * Convenience macro to get the byte size of a field. Works for integers,
 * struct/unions, pointers, arrays, and enums.
 */
macro_rules! bpf_core_field_size {
    ($($field:tt)+) => {
        __builtin_preserve_field_info!(
            ___bpf_field_ref!($($field)+),
            bpf_field_info_kind::BPF_FIELD_BYTE_SIZE
        )
    };
}

/* Convenience macro to get field's byte offset. */
macro_rules! bpf_core_field_offset {
    ($($field:tt)+) => {
        __builtin_preserve_field_info!(
            ___bpf_field_ref!($($field)+),
            bpf_field_info_kind::BPF_FIELD_BYTE_OFFSET
        )
    };
}

/*
 * Convenience macro to get BTF type ID of a specified type, using a local BTF
 * information. Return 32-bit unsigned integer with type ID from program's own
 * BTF. Always succeeds.
 */
macro_rules! bpf_core_type_id_local {
    ($type_:ty) => {
        __builtin_btf_type_id!(*___bpf_typeof!($type_), bpf_type_id_kind::BPF_TYPE_ID_LOCAL)
    };
}

/*
 * Convenience macro to get BTF type ID of a target kernel's type that matches
 * specified local type.
 */
macro_rules! bpf_core_type_id_kernel {
    ($type_:ty) => {
        __builtin_btf_type_id!(*___bpf_typeof!($type_), bpf_type_id_kind::BPF_TYPE_ID_TARGET)
    };
}

/* Convenience macro to check that provided named type exists in a target kernel. */
macro_rules! bpf_core_type_exists {
    ($type_:ty) => {
        __builtin_preserve_type_info!(*___bpf_typeof!($type_), bpf_type_info_kind::BPF_TYPE_EXISTS)
    };
}

/* Convenience macro to check that provided named type matches that in a target kernel. */
macro_rules! bpf_core_type_matches {
    ($type_:ty) => {
        __builtin_preserve_type_info!(*___bpf_typeof!($type_), bpf_type_info_kind::BPF_TYPE_MATCHES)
    };
}

/* Convenience macro to get the byte size of a provided named type in a target kernel. */
macro_rules! bpf_core_type_size {
    ($type_:ty) => {
        __builtin_preserve_type_info!(*___bpf_typeof!($type_), bpf_type_info_kind::BPF_TYPE_SIZE)
    };
}

/*
 * Convenience macro to check that provided enumerator value is defined in
 * a target kernel.
 */
macro_rules! bpf_core_enum_value_exists {
    ($enum_type:ty, $enum_value:expr) => {
        __builtin_preserve_enum_value!(
            ___bpf_typeof!($enum_type),
            $enum_value,
            bpf_enum_value_kind::BPF_ENUMVAL_EXISTS
        )
    };
}

/*
 * Convenience macro to get the integer value of an enumerator value in
 * a target kernel.
 */
macro_rules! bpf_core_enum_value {
    ($enum_type:ty, $enum_value:expr) => {
        __builtin_preserve_enum_value!(
            ___bpf_typeof!($enum_type),
            $enum_value,
            bpf_enum_value_kind::BPF_ENUMVAL_VALUE
        )
    };
}

/*
 * bpf_core_read() abstracts away bpf_probe_read_kernel() call and captures
 * offset relocation for source address using __builtin_preserve_access_index()
 * built-in, provided by Clang.
 */
macro_rules! bpf_core_read {
    ($dst:expr, $sz:expr, $src:expr) => {
        bpf_probe_read_kernel(
            $dst,
            $sz,
            __builtin_preserve_access_index!($src) as *const core::ffi::c_void,
        )
    };
}

/* NOTE: see comments for BPF_CORE_READ_USER() about the proper types use. */
macro_rules! bpf_core_read_user {
    ($dst:expr, $sz:expr, $src:expr) => {
        bpf_probe_read_user(
            $dst,
            $sz,
            __builtin_preserve_access_index!($src) as *const core::ffi::c_void,
        )
    };
}

/*
 * bpf_core_read_str() is a thin wrapper around bpf_probe_read_str()
 * additionally emitting BPF CO-RE field relocation for specified source
 * argument.
 */
macro_rules! bpf_core_read_str {
    ($dst:expr, $sz:expr, $src:expr) => {
        bpf_probe_read_kernel_str(
            $dst,
            $sz,
            __builtin_preserve_access_index!($src) as *const core::ffi::c_void,
        )
    };
}

/* NOTE: see comments for BPF_CORE_READ_USER() about the proper types use. */
macro_rules! bpf_core_read_user_str {
    ($dst:expr, $sz:expr, $src:expr) => {
        bpf_probe_read_user_str(
            $dst,
            $sz,
            __builtin_preserve_access_index!($src) as *const core::ffi::c_void,
        )
    };
}

extern "C" {
    pub fn bpf_rdonly_cast(obj: *const core::ffi::c_void, btf_id: u32) -> *mut core::ffi::c_void;
}

/*
 * Cast provided pointer *ptr* into a pointer to a specified *type* in such
 * a way that BPF verifier will become aware of associated kernel-side BTF
 * type. This allows to access members of kernel types directly without the
 * need to use BPF_CORE_READ() macros.
 */
macro_rules! bpf_core_cast {
    ($ptr:expr, $type_:ty) => {
        bpf_rdonly_cast(
            $ptr as *const core::ffi::c_void,
            bpf_core_type_id_kernel!($type_),
        ) as *mut $type_
    };
}

macro_rules! ___concat {
    ($a:ident, $b:ident) => {
        compile_error!("token concatenation has no direct stable macro_rules expression equivalent")
    };
}

macro_rules! ___apply {
    ($fn_:ident, $n:tt) => {
        ___concat!($fn_, $n)
    };
}

/* Variadic selector/counting helpers translated as macro_rules! forms. */
macro_rules! ___nth {
    ($_1:tt, $_2:tt, $_3:tt, $_4:tt, $_5:tt, $_6:tt, $_7:tt, $_8:tt, $_9:tt, $_10:tt, $__11:tt, $N:tt, $($rest:tt)*) => {
        $N
    };
}

macro_rules! ___narg {
    () => { 0 };
    ($a:tt) => { 1 };
    ($a:tt, $b:tt) => { 2 };
    ($a:tt, $b:tt, $c:tt) => { 3 };
    ($a:tt, $b:tt, $c:tt, $d:tt) => { 4 };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt) => { 5 };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt) => { 6 };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt) => { 7 };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt) => { 8 };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt, $i:tt) => { 9 };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt, $i:tt, $j:tt) => { 10 };
}

macro_rules! ___empty {
    () => { 0 };
    ($($args:tt)+) => { N };
}

macro_rules! ___last {
    ($x:tt) => { $x };
    ($a:tt, $x:tt) => { $x };
    ($a:tt, $b:tt, $x:tt) => { $x };
    ($a:tt, $b:tt, $c:tt, $x:tt) => { $x };
    ($a:tt, $b:tt, $c:tt, $d:tt, $x:tt) => { $x };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $x:tt) => { $x };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $x:tt) => { $x };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $x:tt) => { $x };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt, $x:tt) => { $x };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt, $i:tt, $x:tt) => { $x };
}

macro_rules! ___nolast {
    ($a:tt, $_:tt) => { $a };
    ($a:tt, $b:tt, $_:tt) => { $a, $b };
    ($a:tt, $b:tt, $c:tt, $_:tt) => { $a, $b, $c };
    ($a:tt, $b:tt, $c:tt, $d:tt, $_:tt) => { $a, $b, $c, $d };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $_:tt) => { $a, $b, $c, $d, $e };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $_:tt) => { $a, $b, $c, $d, $e, $f };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $_:tt) => { $a, $b, $c, $d, $e, $f, $g };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt, $_:tt) => { $a, $b, $c, $d, $e, $f, $g, $h };
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt, $i:tt, $_:tt) => { $a, $b, $c, $d, $e, $f, $g, $h, $i };
}

macro_rules! ___arrow {
    ($a:expr) => { $a };
    ($a:expr, $b:tt) => { (*$a).$b };
    ($a:expr, $b:tt, $c:tt) => { (*(*$a).$b).$c };
    ($a:expr, $b:tt, $c:tt, $d:tt) => { (*(*(*$a).$b).$c).$d };
    ($a:expr, $b:tt, $c:tt, $d:tt, $e:tt) => { (*(*(*(*$a).$b).$c).$d).$e };
    ($a:expr, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt) => { (*(*(*(*(*$a).$b).$c).$d).$e).$f };
    ($a:expr, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt) => { (*(*(*(*(*(*$a).$b).$c).$d).$e).$f).$g };
    ($a:expr, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt) => { (*(*(*(*(*(*(*$a).$b).$c).$d).$e).$f).$g).$h };
    ($a:expr, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt, $i:tt) => { (*(*(*(*(*(*(*(*$a).$b).$c).$d).$e).$f).$g).$h).$i };
    ($a:expr, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt, $i:tt, $j:tt) => { (*(*(*(*(*(*(*(*(*$a).$b).$c).$d).$e).$f).$g).$h).$i).$j };
}

/*
 * C condition:
 * #if defined(__clang__) && (__clang_major__ >= 19)
 * #define ___type(...) __typeof_unqual__(___arrow(__VA_ARGS__))
 * #elif defined(__GNUC__) && (__GNUC__ >= 14)
 * #define ___type(...) __typeof_unqual__(___arrow(__VA_ARGS__))
 * #else
 * #define ___type(...) typeof(___arrow(__VA_ARGS__))
 * #endif
 */
macro_rules! ___type {
    ($($args:tt)+) => {
        __typeof_unqual_or_typeof!(___arrow!($($args)+))
    };
}

macro_rules! ___read {
    ($read_fn:ident, $dst:expr, $src_type:ty, $src:expr, $accessor:tt) => {
        $read_fn!(
            $dst as *mut core::ffi::c_void,
            core::mem::size_of_val(&*$dst),
            &(*(($src) as $src_type)).$accessor,
        )
    };
}

/* "recursively" read a sequence of inner pointers using local __t var */
macro_rules! ___rd_first {
    ($fn_:ident, $src:expr, $a:tt) => {
        ___read!($fn_, &mut __t, ___type!($src), $src, $a);
    };
}

macro_rules! ___rd_last {
    ($fn_:ident, $($args:tt)+) => {
        ___read!($fn_, &mut __t, ___type!(___nolast!($($args)+)), __t, ___last!($($args)+));
    };
}

macro_rules! ___read_ptrs {
    ($fn_:ident, $src:expr, $($accessors:tt)+) => {{
        let mut __t: *const core::ffi::c_void;
        ___read_ptrs_impl!($fn_, $src, $($accessors)+);
    }};
}

macro_rules! ___read_ptrs_impl {
    ($fn_:ident, $src:expr, $a:tt) => {
        ___rd_first!($fn_, $src, $a)
    };
    ($fn_:ident, $src:expr, $a:tt, $($rest:tt)+) => {
        ___read_ptrs_impl!($fn_, $src, $a);
        ___rd_last!($fn_, $a, $($rest)+)
    };
}

macro_rules! ___core_read0 {
    ($fn_:ident, $fn_ptr:ident, $dst:expr, $src:expr, $a:tt) => {
        ___read!($fn_, $dst, ___type!($src), $src, $a);
    };
}

macro_rules! ___core_readN {
    ($fn_:ident, $fn_ptr:ident, $dst:expr, $src:expr, $($args:tt)+) => {
        ___read_ptrs!($fn_ptr, $src, ___nolast!($($args)+));
        ___read!(
            $fn_,
            $dst,
            ___type!($src, ___nolast!($($args)+)),
            __t,
            ___last!($($args)+)
        );
    };
}

macro_rules! ___core_read {
    ($fn_:ident, $fn_ptr:ident, $dst:expr, $src:expr, $a:tt) => {
        ___core_read0!($fn_, $fn_ptr, $dst, $src, $a)
    };
    ($fn_:ident, $fn_ptr:ident, $dst:expr, $src:expr, $a:tt, $($rest:tt)+) => {
        ___core_readN!($fn_, $fn_ptr, $dst, $src, $a, $($rest)+)
    };
}

/*
 * BPF_CORE_READ_INTO() is a more performance-conscious variant of
 * BPF_CORE_READ(), in which final field is read into user-provided storage.
 */
macro_rules! BPF_CORE_READ_INTO {
    ($dst:expr, $src:expr, $a:tt $(, $rest:tt)*) => {{
        ___core_read!(bpf_core_read, bpf_core_read, $dst, $src, $a $(, $rest)*)
    }};
}

/* Variant of BPF_CORE_READ_INTO() for reading from user-space memory. */
macro_rules! BPF_CORE_READ_USER_INTO {
    ($dst:expr, $src:expr, $a:tt $(, $rest:tt)*) => {{
        ___core_read!(bpf_core_read_user, bpf_core_read_user, $dst, $src, $a $(, $rest)*)
    }};
}

/* Non-CO-RE variant of BPF_CORE_READ_INTO() */
macro_rules! BPF_PROBE_READ_INTO {
    ($dst:expr, $src:expr, $a:tt $(, $rest:tt)*) => {{
        ___core_read!(bpf_probe_read_kernel, bpf_probe_read_kernel, $dst, $src, $a $(, $rest)*)
    }};
}

/* Non-CO-RE variant of BPF_CORE_READ_USER_INTO(). */
macro_rules! BPF_PROBE_READ_USER_INTO {
    ($dst:expr, $src:expr, $a:tt $(, $rest:tt)*) => {{
        ___core_read!(bpf_probe_read_user, bpf_probe_read_user, $dst, $src, $a $(, $rest)*)
    }};
}

/*
 * BPF_CORE_READ_STR_INTO() does same "pointer chasing" as
 * BPF_CORE_READ() for intermediate pointers, but then executes (and returns
 * corresponding error code) bpf_core_read_str() for final string read.
 */
macro_rules! BPF_CORE_READ_STR_INTO {
    ($dst:expr, $src:expr, $a:tt $(, $rest:tt)*) => {{
        ___core_read!(bpf_core_read_str, bpf_core_read, $dst, $src, $a $(, $rest)*)
    }};
}

/* Variant of BPF_CORE_READ_STR_INTO() for reading from user-space memory. */
macro_rules! BPF_CORE_READ_USER_STR_INTO {
    ($dst:expr, $src:expr, $a:tt $(, $rest:tt)*) => {{
        ___core_read!(bpf_core_read_user_str, bpf_core_read_user, $dst, $src, $a $(, $rest)*)
    }};
}

/* Non-CO-RE variant of BPF_CORE_READ_STR_INTO() */
macro_rules! BPF_PROBE_READ_STR_INTO {
    ($dst:expr, $src:expr, $a:tt $(, $rest:tt)*) => {{
        ___core_read!(bpf_probe_read_kernel_str, bpf_probe_read_kernel, $dst, $src, $a $(, $rest)*)
    }};
}

/* Non-CO-RE variant of BPF_CORE_READ_USER_STR_INTO(). */
macro_rules! BPF_PROBE_READ_USER_STR_INTO {
    ($dst:expr, $src:expr, $a:tt $(, $rest:tt)*) => {{
        ___core_read!(bpf_probe_read_user_str, bpf_probe_read_user, $dst, $src, $a $(, $rest)*)
    }};
}

/*
 * BPF_CORE_READ() is used to simplify BPF CO-RE relocatable read, especially
 * when there are few pointer chasing steps.
 *
 * N.B. Only up to 9 "field accessors" are supported, which should be more
 * than enough for any practical purpose.
 */
macro_rules! BPF_CORE_READ {
    ($src:expr, $a:tt $(, $rest:tt)*) => {{
        let mut __r: ___type!(($src), $a $(, $rest)*);
        BPF_CORE_READ_INTO!(&mut __r, $src, $a $(, $rest)*);
        __r
    }};
}

/* Variant of BPF_CORE_READ() for reading from user-space memory. */
macro_rules! BPF_CORE_READ_USER {
    ($src:expr, $a:tt $(, $rest:tt)*) => {{
        let mut __r: ___type!(($src), $a $(, $rest)*);
        BPF_CORE_READ_USER_INTO!(&mut __r, $src, $a $(, $rest)*);
        __r
    }};
}

/* Non-CO-RE variant of BPF_CORE_READ() */
macro_rules! BPF_PROBE_READ {
    ($src:expr, $a:tt $(, $rest:tt)*) => {{
        let mut __r: ___type!(($src), $a $(, $rest)*);
        BPF_PROBE_READ_INTO!(&mut __r, $src, $a $(, $rest)*);
        __r
    }};
}

/*
 * Non-CO-RE variant of BPF_CORE_READ_USER().
 *
 * As no CO-RE relocations are emitted, source types can be arbitrary and are
 * not restricted to kernel types only.
 */
macro_rules! BPF_PROBE_READ_USER {
    ($src:expr, $a:tt $(, $rest:tt)*) => {{
        let mut __r: ___type!(($src), $a $(, $rest)*);
        BPF_PROBE_READ_USER_INTO!(&mut __r, $src, $a $(, $rest)*);
        __r
    }};
}
