// SPDX-License-Identifier: GPL-2.0-only
// Semantic source authority: mm/shmem.c @ e1d84f501551943a11f4c5271e9f5c85d7e15168
// Authority SHA256: 43da71bf59c4b3f2971ba3a9e9aaa704504ea1b8f872ac70c8f892fb0c31ef87
// Historical initial transcription: d482bb509b7d065808de40ce78b5bca39f40b783
// SOURCE-COMMIT: e1d84f501551943a11f4c5271e9f5c85d7e15168
// This file is a source-phase proposal; the review inventory is authoritative
// about coverage. Generated bindings must come from the selected kernel config.
use b::*;
use core::mem::{size_of, zeroed, MaybeUninit};
use core::ptr::{addr_of, addr_of_mut, null, null_mut};
use kernel::bindings::shmem_native as b;
use kernel::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use kernel::str::CStrExt;
type Pgoff = rust_shmem_pgoff_t;
include!("shmem_native_aliases.rs");
#[inline]
fn round_up(v: c_ulong, n: c_ulong) -> c_ulong {
    v.wrapping_add(n - 1) & !(n - 1)
}
#[inline]
fn round_down(v: c_ulong, n: c_ulong) -> c_ulong {
    v & !(n - 1)
}
#[inline]
fn vm_acct(size: loff_t) -> loff_t {
    let mask = RUST_SHMEM_PAGE_SIZE as loff_t - 1;
    (size.wrapping_add(mask) & !mask) >> RUST_SHMEM_PAGE_SHIFT
}
#[inline]
unsafe fn SHMEM_SB(sb: *mut super_block) -> *mut shmem_sb_info {
    (*sb).s_fs_info.cast()
}
#[inline]
unsafe fn err_ptr<T>(err: c_int) -> *mut T {
    err as isize as *mut T
}
#[inline]
unsafe fn ptr_err<T>(p: *mut T) -> c_int {
    p as isize as c_int
}
macro_rules! vm_bug {
    ($e:expr) => {
        #[cfg(CONFIG_DEBUG_VM)]
        rust_shmem_vm_bug($e);
    };
}
macro_rules! vm_bug_folio {
    ($e:expr, $f:expr) => {
        #[cfg(CONFIG_DEBUG_VM)]
        rust_shmem_vm_bug_folio($e, $f);
    };
}
#[cfg(CONFIG_SHMEM)]
include!("shmem_account.rs");
#[cfg(CONFIG_SHMEM)]
include!("shmem_cache.rs");
#[cfg(CONFIG_SHMEM)]
include!("shmem_truncate.rs");
#[cfg(CONFIG_SHMEM)]
include!("shmem_swap.rs");
#[cfg(CONFIG_SHMEM)]
include!("shmem_huge.rs");
#[cfg(CONFIG_SHMEM)]
include!("shmem_folio.rs");
#[cfg(CONFIG_SHMEM)]
include!("shmem_vm.rs");
#[cfg(CONFIG_SHMEM)]
include!("shmem_inode.rs");
include!("shmem_common.rs");
#[cfg(all(CONFIG_SHMEM, CONFIG_TMPFS))]
include!("shmem_io.rs");
#[cfg(all(CONFIG_SHMEM, CONFIG_TMPFS))]
include!("shmem_directory.rs");
#[cfg(all(CONFIG_SHMEM, CONFIG_TMPFS, CONFIG_TMPFS_XATTR))]
include!("shmem_xattr.rs");
#[cfg(all(CONFIG_SHMEM, CONFIG_TMPFS))]
include!("shmem_mount.rs");
#[cfg(CONFIG_SHMEM)]
include!("shmem_super.rs");
#[cfg(all(CONFIG_SHMEM, CONFIG_TRANSPARENT_HUGEPAGE))]
include!("shmem_huge_config.rs");
#[cfg(not(CONFIG_SHMEM))]
include!("shmem_tiny.rs");

#[inline]
fn round_up_u64(v: u64, n: u64) -> u64 {
    v.wrapping_add(n - 1) & !(n - 1)
}
#[inline]
fn round_down_u64(v: u64, n: u64) -> u64 {
    v & !(n - 1)
}
