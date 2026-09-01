// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

/*
 * C header dependencies removed from executable Rust:
 * <vmlinux.h>, <bpf/bpf_helpers.h>, and <bpf_may_goto.h>.
 * Symbols supplied by those headers, such as barrier!() and
 * cond_break_label!(), are referenced as external dependencies.
 */

extern "C" {
    #[link_name = "CONFIG_X86_64"]
    pub static mut CONFIG_X86_64: bool;
}

/*
 * __unqual_typeof(x) - Declare an unqualified scalar type, leaving
 *                      non-scalar types unchanged,
 *
 * Prefer C11 _Generic for better compile-times and simpler code. Note: 'char'
 * is not type-compatible with 'signed char', and we define a separate case.
 *
 * This is copied verbatim from kernel's include/linux/compiler_types.h, but
 * with default expression (for pointers) changed from (x) to (typeof(x)0).
 *
 * This is because LLVM has a bug where for lvalue (x), it does not get rid of
 * an extra address_space qualifier, but does in case of rvalue (typeof(x)0).
 * Hence, for pointers, we need to create an rvalue expression to get the
 * desired type. See https://github.com/llvm/llvm-project/issues/53400.
 *
 * Rust has no direct source-level equivalent for C typeof/_Generic type
 * unqualification macros. The operation is preserved at each translated use by
 * binding through the pointee/value type inferred by Rust.
 */

/* No-op for BPF */
#[inline(always)]
pub fn cpu_relax() {}

#[inline(always)]
pub unsafe fn READ_ONCE<T: Copy>(x: *const T) -> T {
    core::ptr::read_volatile(x)
}

#[inline(always)]
pub unsafe fn WRITE_ONCE<T>(x: *mut T, val: T) -> T
where
    T: Copy,
{
    core::ptr::write_volatile(x, val);
    val
}

macro_rules! cmpxchg {
    ($p:expr, $old:expr, $new:expr) => {{
        /*
         * C uses __sync_val_compare_and_swap((p), old, new). This preserves the
         * source-level atomic compare-exchange intent; the intrinsic is an
         * external/compiler dependency for the eventual BPF Rust environment.
         */
        core::intrinsics::atomic_cxchg_seqcst_seqcst($p, $old, $new).0
    }};
}

macro_rules! try_cmpxchg {
    ($p:expr, $pold:expr, $new:expr) => {{
        let __o = *$pold;
        let __r = cmpxchg!($p, __o, $new);
        if __r != __o {
            *$pold = __r;
        }
        __r == __o
    }};
}

macro_rules! try_cmpxchg_relaxed {
    ($p:expr, $pold:expr, $new:expr) => {
        try_cmpxchg!($p, $pold, $new)
    };
}

macro_rules! try_cmpxchg_acquire {
    ($p:expr, $pold:expr, $new:expr) => {
        try_cmpxchg!($p, $pold, $new)
    };
}

macro_rules! smp_mb {
    () => {{
        let mut __val: core::ffi::c_ulong = 0;
        let _ = core::intrinsics::atomic_xadd_seqcst(&mut __val, 0);
    }};
}

macro_rules! smp_rmb {
    () => {{
        if !CONFIG_X86_64 {
            smp_mb!();
        } else {
            barrier!();
        }
    }};
}

macro_rules! smp_wmb {
    () => {{
        if !CONFIG_X86_64 {
            smp_mb!();
        } else {
            barrier!();
        }
    }};
}

/* Control dependency provides LOAD->STORE, provide LOAD->LOAD */
macro_rules! smp_acquire__after_ctrl_dep {
    () => {{
        smp_rmb!();
    }};
}

/*
 * If __BPF_FEATURE_LOAD_ACQ_STORE_REL is defined, the C header uses
 * __atomic_load_n(..., __ATOMIC_ACQUIRE) and
 * __atomic_store_n(..., __ATOMIC_RELEASE), lowered by Clang to
 * BPF_LOAD_ACQ/BPF_STORE_REL. Otherwise it uses the barrier fallback below.
 * This Rust translation keeps the fallback as the file-local portable form.
 */
macro_rules! smp_load_acquire {
    ($p:expr) => {{
        let __v = READ_ONCE($p);
        if !CONFIG_X86_64 {
            smp_mb!();
        }
        barrier!();
        __v
    }};
}

macro_rules! smp_store_release {
    ($p:expr, $val:expr) => {{
        if !CONFIG_X86_64 {
            smp_mb!();
        }
        barrier!();
        WRITE_ONCE($p, $val);
    }};
}

macro_rules! smp_cond_load_relaxed_label {
    ($p:expr, $cond_expr:expr, $label:ident) => {{
        let __ptr = $p;
        let mut VAL;
        loop {
            VAL = READ_ONCE(__ptr);
            if $cond_expr {
                break;
            }
            cond_break_label!($label);
            cpu_relax();
        }
        VAL
    }};
}

macro_rules! smp_cond_load_acquire_label {
    ($p:expr, $cond_expr:expr, $label:ident) => {{
        let __val = smp_cond_load_relaxed_label!($p, $cond_expr, $label);
        smp_acquire__after_ctrl_dep!();
        __val
    }};
}

macro_rules! atomic_read {
    ($p:expr) => {
        READ_ONCE(core::ptr::addr_of!((*$p).counter))
    };
}

macro_rules! atomic_cond_read_relaxed_label {
    ($p:expr, $cond_expr:expr, $label:ident) => {
        smp_cond_load_relaxed_label!(core::ptr::addr_of!((*$p).counter), $cond_expr, $label)
    };
}

macro_rules! atomic_cond_read_acquire_label {
    ($p:expr, $cond_expr:expr, $label:ident) => {
        smp_cond_load_acquire_label!(core::ptr::addr_of!((*$p).counter), $cond_expr, $label)
    };
}

macro_rules! atomic_try_cmpxchg_relaxed {
    ($p:expr, $pold:expr, $new:expr) => {
        try_cmpxchg_relaxed!(core::ptr::addr_of_mut!((*$p).counter), $pold, $new)
    };
}

macro_rules! atomic_try_cmpxchg_acquire {
    ($p:expr, $pold:expr, $new:expr) => {
        try_cmpxchg_acquire!(core::ptr::addr_of_mut!((*$p).counter), $pold, $new)
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
