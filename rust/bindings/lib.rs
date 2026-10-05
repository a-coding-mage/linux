// SPDX-License-Identifier: GPL-2.0

//! Bindings.
//!
//! Imports the generated bindings by `bindgen`.
//!
//! This crate may not be directly used. If you need a kernel C API that is
//! not ported or wrapped in the `kernel` crate, then do so first instead of
//! using this crate.

#![no_std]
#![allow(
    clippy::all,
    missing_docs,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    improper_ctypes,
    unreachable_pub,
    unsafe_op_in_unsafe_fn
)]
#![feature(cfi_encoding)]

#[allow(dead_code)]
#[allow(clippy::cast_lossless)]
#[allow(clippy::ptr_as_ptr)]
#[allow(clippy::ref_as_ptr)]
#[allow(clippy::undocumented_unsafe_blocks)]
#[cfg_attr(CONFIG_RUSTC_HAS_UNNECESSARY_TRANSMUTES, allow(unnecessary_transmutes))]
#[cfg_attr(
    CONFIG_RUSTC_HAS_SUSPICIOUS_RUNTIME_SYMBOL_DEFINITIONS,
    allow(suspicious_runtime_symbol_definitions)
)]
mod bindings_raw {
    use pin_init::{MaybeZeroable, Zeroable};

    // Manual definition for blocklisted types.
    type __kernel_size_t = usize;
    type __kernel_ssize_t = isize;
    type __kernel_ptrdiff_t = isize;

    // with_primes unconditionally invokes this callback: NULL is outside its
    // C contract. A bare function pointer also preserves the C KCFI type of
    // with_primes itself; Option<fn> has a different nominal KCFI encoding.
    #[cfg(CONFIG_PRIME_NUMBERS_KUNIT_TEST)]
    pub type primes_fn = unsafe extern "C" fn(*mut ffi::c_void, *const primes);

    // `bindgen` doesn't automatically do this, see
    // <https://github.com/rust-lang/rust-bindgen/issues/3196>
    //
    // SAFETY: `__BindgenBitfieldUnit<Storage>` is a newtype around `Storage`.
    unsafe impl<Storage> Zeroable for __BindgenBitfieldUnit<Storage> where Storage: Zeroable {}

    // Use glob import here to expose all helpers.
    // Symbols defined within the module will take precedence to the glob import.
    pub use super::bindings_helper::*;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/bindings_generated.rs"
    ));
}

// When both a directly exposed symbol and a helper exists for the same function,
// the directly exposed symbol is preferred and the helper becomes dead code, so
// ignore the warning here.
#[allow(dead_code)]
mod bindings_helper {
    // Import the generated bindings for types.
    use super::bindings_raw::*;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/bindings_helpers_generated.rs"
    ));
}

mod bsearch;
mod rbtree;
mod sort;

pub use bindings_raw::*;
pub use bsearch::{bsearch, BsearchCmp};
pub use rbtree::*;
pub use sort::*;

pub const compat_ptr_ioctl: Option<
    unsafe extern "C" fn(*mut file, ffi::c_uint, ffi::c_ulong) -> ffi::c_long,
> = {
    #[cfg(CONFIG_COMPAT)]
    {
        Some(bindings_raw::compat_ptr_ioctl)
    }
    #[cfg(not(CONFIG_COMPAT))]
    {
        None
    }
};

// This native module inherits the crate's existing generated-code policy.
// The handwritten vmstat owner retains the original kernel warning policy.
#[cfg(CONFIG_RUST_VMSTAT)]
pub mod vmstat_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/vmstat_native_generated.rs"));
}

// Generated native declarations inherit this crate's existing policy only.
// Handwritten owners retain the kernel's ordinary diagnostics.
#[cfg(CONFIG_RUST_VMSCAN)]
pub mod vmscan_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/vmscan_native_generated.rs"));
}

#[cfg(CONFIG_RUST_SHMEM)]
pub mod shmem_native {
    type __kernel_size_t = usize;
    type __kernel_ssize_t = isize;
    type __kernel_ptrdiff_t = isize;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/shmem_native_generated.rs"));
}

#[cfg(CONFIG_RUST_FILEMAP)]
pub mod filemap_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/filemap_native_generated.rs"));
}

// Keep generated scheduler declarations under the existing bindings policy.
// Handwritten scheduler owners retain the ordinary kernel diagnostics.
#[cfg(CONFIG_RUST_SCHED_CORE)]
pub mod sched_core_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/sched_core_generated.rs"));
}

#[cfg(CONFIG_RUST_SCHED_FAIR)]
pub mod sched_fair_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/sched_fair_generated.rs"));
}

#[cfg(CONFIG_RUST_SCHED_RT)]
pub mod sched_rt_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/sched_rt_generated.rs"));
}

#[cfg(CONFIG_RUST_SCHED_SUPPORT)]
pub mod sched_support_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/sched_support_generated.rs"));
}

#[cfg(CONFIG_RUST_SCHED_DEADLINE)]
pub mod sched_deadline {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/sched_deadline_generated.rs"));
}

#[cfg(CONFIG_RUST_SCHED_PSI)]
pub mod sched_psi_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/sched_psi_generated.rs"));
}

#[cfg(CONFIG_RUST_SCHED_CPUTIME)]
pub mod cputime_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/cputime_generated.rs"));
}

#[cfg(CONFIG_RUST_SCHED_WAITING)]
pub mod sched_waiting_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/sched_waiting_generated.rs"));
}

#[cfg(CONFIG_RUST_SCHED_SYSCALLS)]
pub mod sched_syscalls_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/sched_syscalls_generated.rs"));
}

#[cfg(CONFIG_RUST_SCHED_LOADAVG)]
pub mod sched_loadavg_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/sched_loadavg_generated.rs"));
}

#[cfg(CONFIG_RUST_SCHED_AUTOGROUP)]
pub mod sched_autogroup_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/sched_autogroup_generated.rs"));
}

#[cfg(CONFIG_RUST_SCHED_PELT)]
pub mod sched_pelt_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/sched_pelt_generated.rs"));
}

#[cfg(CONFIG_RUST_SCHED_MEMBARRIER)]
pub mod sched_membarrier_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/sched_membarrier_generated.rs"));
}

#[cfg(CONFIG_RUST_SCHED_DEBUG)]
pub mod sched_debug_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/sched_debug_generated.rs"));
}

#[cfg(CONFIG_RUST_SCHED_CPUACCT)]
pub mod sched_cpuacct_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/sched_cpuacct_generated.rs"));
}

#[cfg(CONFIG_RUST_SCHED_ISOLATION)]
pub mod sched_isolation_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/sched_isolation_generated.rs"));
}

#[cfg(CONFIG_RUST_SCHED_STATS)]
pub mod sched_stats_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/sched_stats_generated.rs"));
}

#[cfg(CONFIG_RUST_SCHED_CPUFREQ)]
pub mod sched_cpufreq_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/sched_cpufreq_generated.rs"));
}

#[cfg(CONFIG_RUST_SCHED_CORE_COOKIE)]
pub mod sched_core_cookie_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/sched_core_cookie_generated.rs"));
}

#[cfg(CONFIG_RUST_SCHED_STOP_TASK)]
pub mod sched_stop_task_native {
    include!(concat!(env!("OBJTREE"), "/rust/bindings/sched_stop_task_generated.rs"));
}
