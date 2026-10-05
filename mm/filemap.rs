// SPDX-License-Identifier: GPL-2.0-only
// Rust semantic owner of mm/filemap.c at e1d84f501551943a11f4c5271e9f5c85d7e15168.
// All native types/constants are obtained from the exact configured headers.
// Historical initial transcription: d482bb509b7d065808de40ce78b5bca39f40b783
// SOURCE-COMMIT: e1d84f501551943a11f4c5271e9f5c85d7e15168
use b::*;
use core::mem::{size_of, zeroed, MaybeUninit};
use core::ptr::{addr_of, addr_of_mut, null_mut};
use kernel::bindings::filemap_native as b;
use kernel::ffi::{c_int, c_long, c_uint, c_ulong, c_void};
type Pgoff = rust_filemap_pgoff_t;
include!("filemap_native_aliases.rs");
macro_rules! vm_bug_folio {
    ($condition:expr, $folio:expr) => {{
        #[cfg(CONFIG_DEBUG_VM)]
        rust_filemap_vm_bug_folio(($condition) != 0, $folio);
    }};
}
include!("filemap_cache.rs");
include!("filemap_writeback.rs");
include!("filemap_wait.rs");
include!("filemap_lookup.rs");
type Filler = Option<unsafe extern "C" fn(*mut file, *mut folio) -> c_int>;
include!("filemap_read.rs");
#[cfg(CONFIG_MMU)]
mod mmap {
    use super::*;
    include!("filemap_fault.rs");
}
#[cfg(CONFIG_MMU)]
pub use mmap::*;
#[cfg(not(CONFIG_MMU))]
include!("filemap_nommu.rs");
include!("filemap_write.rs");
#[cfg(CONFIG_CACHESTAT_SYSCALL)]
include!("filemap_cachestat.rs");
// Exact C usual arithmetic conversions for loff_t mixed with PAGE_SIZE (UL).
const PAGE_SHIFT: c_uint = RUST_FILEMAP_PAGE_SHIFT as c_uint;
const PAGE_SIZE: c_ulong = RUST_FILEMAP_PAGE_SIZE as c_ulong;
#[cfg(CONFIG_MMU)]
fn div_round_up_file_bytes(bytes: loff_t) -> Pgoff {
    #[cfg(target_pointer_width = "32")]
    {
        bytes
            .wrapping_add(PAGE_SIZE as loff_t)
            .wrapping_sub(1)
            .wrapping_div(PAGE_SIZE as loff_t) as Pgoff
    }
    #[cfg(target_pointer_width = "64")]
    {
        (bytes as c_ulong).wrapping_add(PAGE_SIZE).wrapping_sub(1) / PAGE_SIZE
    }
}
fn file_bytes_div_pages(bytes: loff_t) -> Pgoff {
    #[cfg(target_pointer_width = "32")]
    {
        bytes.wrapping_div(PAGE_SIZE as loff_t) as Pgoff
    }
    #[cfg(target_pointer_width = "64")]
    {
        bytes as c_ulong / PAGE_SIZE
    }
}
// Native integer constants are explicitly converted at C's use-site type.
const RUST_FILEMAP_WQ_FLAG_CUSTOM: c_uint = b::RUST_FILEMAP_WQ_FLAG_CUSTOM as c_uint;
const RUST_FILEMAP_WQ_FLAG_DONE: c_uint = b::RUST_FILEMAP_WQ_FLAG_DONE as c_uint;
const RUST_FILEMAP_WQ_FLAG_EXCLUSIVE: c_uint = b::RUST_FILEMAP_WQ_FLAG_EXCLUSIVE as c_uint;
const RUST_FILEMAP_WQ_FLAG_WOKEN: c_uint = b::RUST_FILEMAP_WQ_FLAG_WOKEN as c_uint;
const RUST_FILEMAP_FAULT_FLAG_KILLABLE: fault_flag =
    b::RUST_FILEMAP_FAULT_FLAG_KILLABLE as fault_flag;
const RUST_FILEMAP_FAULT_FLAG_RETRY_NOWAIT: fault_flag =
    b::RUST_FILEMAP_FAULT_FLAG_RETRY_NOWAIT as fault_flag;
#[cfg(CONFIG_MMU)]
const RUST_FILEMAP_FAULT_FLAG_ORIG_PTE_VALID: fault_flag =
    b::RUST_FILEMAP_FAULT_FLAG_ORIG_PTE_VALID as fault_flag;
#[cfg(CONFIG_MMU)]
const RUST_FILEMAP_FAULT_FLAG_TRIED: fault_flag = b::RUST_FILEMAP_FAULT_FLAG_TRIED as fault_flag;
const RUST_FILEMAP_VM_FAULT_SIGBUS: vm_fault_t = b::RUST_FILEMAP_VM_FAULT_SIGBUS as vm_fault_t;
const RUST_FILEMAP_VM_FAULT_RETRY: vm_fault_t = b::RUST_FILEMAP_VM_FAULT_RETRY as vm_fault_t;
#[cfg(CONFIG_MMU)]
const RUST_FILEMAP_VM_FAULT_LOCKED: vm_fault_t = b::RUST_FILEMAP_VM_FAULT_LOCKED as vm_fault_t;
#[cfg(CONFIG_MMU)]
const RUST_FILEMAP_VM_FAULT_MAJOR: vm_fault_t = b::RUST_FILEMAP_VM_FAULT_MAJOR as vm_fault_t;
#[cfg(CONFIG_MMU)]
const RUST_FILEMAP_VM_FAULT_NOPAGE: vm_fault_t = b::RUST_FILEMAP_VM_FAULT_NOPAGE as vm_fault_t;
#[cfg(CONFIG_MMU)]
const RUST_FILEMAP_VM_FAULT_OOM: vm_fault_t = b::RUST_FILEMAP_VM_FAULT_OOM as vm_fault_t;
