// SPDX-License-Identifier: GPL-2.0-only
// Rust implementation of mm/userfaultfd.c at 0db90fa02d8bc839349c44c13904a548f7dd062a.
// Native headers determine ABI layouts, architecture operations and constants.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    unused_imports,
    unused_variables,
    unused_mut,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    unreachable_pub
)]
#[allow(improper_ctypes)]
mod b {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/userfaultfd_native_generated.rs"
    ));
}
use b::*;
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, null, null_mut};
use kernel::ffi::{c_char, c_int, c_long, c_ulong, c_void};
include!("userfaultfd_native_aliases.rs");
const PAGE_SIZE: c_ulong = RUST_UFFD_PAGE_SIZE as c_ulong;
const PAGE_MASK: c_ulong = !(PAGE_SIZE - 1);
const PAGE_SHIFT: u32 = RUST_UFFD_PAGE_SHIFT as u32;
const UFFD_FEATURE_INITIALIZED: u32 = 1 << 31;
#[inline]
fn error(e: u32) -> c_int {
    -(e as c_int)
}
#[inline]
fn err_ptr<T>(e: u32) -> *mut T {
    (-(e as isize)) as *mut T
}
#[inline]
fn is_err<T>(p: *const T) -> bool {
    p as usize >= usize::MAX - MAX_ERRNO as usize + 1
}
#[inline]
fn ptr_err<T>(p: *const T) -> c_int {
    p as isize as c_int
}
#[inline]
fn mode_is(f: uffd_flags_t, m: mfill_atomic_mode) -> bool {
    f & RUST_UFFD_MFILL_ATOMIC_MODE_MASK as u32 == m as u32
}
#[inline]
fn set_mode(f: uffd_flags_t, m: mfill_atomic_mode) -> uffd_flags_t {
    (f & !(RUST_UFFD_MFILL_ATOMIC_MODE_MASK as u32)) | m as u32
}
#[inline]
unsafe fn vstart(v: *mut vm_area_struct) -> c_ulong {
    *vma_start_ptr(v)
}
#[inline]
unsafe fn vend(v: *mut vm_area_struct) -> c_ulong {
    *vma_end_ptr(v)
}
#[inline]
unsafe fn vflags(v: *mut vm_area_struct) -> vm_flags_t {
    *vma_flags_ptr(v)
}
#[inline]
unsafe fn vctx(v: *mut vm_area_struct) -> *mut userfaultfd_ctx {
    (*v).vm_userfaultfd_ctx.ctx
}
#[inline]
unsafe fn iterator(mm: *mut mm_struct, a: c_ulong) -> vma_iterator {
    let mut i = zeroed();
    vma_iter_init(&mut i, mm, a);
    i
}
macro_rules! container {
    ($p:expr,$t:ty,$f:ident) => {
        ($p as *mut u8).sub(core::mem::offset_of!($t, $f)) as *mut $t
    };
}
macro_rules! vm_warn {
    ($site:ident,$cond:expr $(,)?) => {{
        #[cfg(CONFIG_DEBUG_VM)]
        {
            $site($cond);
        }
    }};
}
include!("userfaultfd_fill.rs");
include!("userfaultfd_move.rs");
include!("userfaultfd_context.rs");
include!("userfaultfd_ioctl.rs");
