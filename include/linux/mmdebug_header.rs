/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/mmdebug.h.  The declarations below are supplied by
// other kernel components; configuration branches are preserved with cfg.

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}
#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct vma_iterator {
    _private: [u8; 0],
}
#[repr(C)]
pub struct vma_merge_struct {
    _private: [u8; 0],
}

extern "C" {
    pub fn dump_page(page: *const page, reason: *const core::ffi::c_char);
    pub fn dump_vma(vma: *const vm_area_struct);
    pub fn dump_mm(mm: *const mm_struct);
    pub fn dump_vmg(vmg: *const vma_merge_struct, reason: *const core::ffi::c_char);
    pub fn vma_iter_dump_tree(vmi: *const vma_iterator);
}

// CONFIG_DEBUG_VM is a build-time condition; select the corresponding macro
// definitions in the consuming configuration.
#[cfg(CONFIG_DEBUG_VM)]
#[macro_export]
macro_rules! VM_BUG_ON { ($cond:expr) => { BUG_ON!($cond) }; }

#[cfg(CONFIG_DEBUG_VM)]
#[macro_export]
macro_rules! VM_BUG_ON_PAGE { ($cond:expr, $page:expr) => {{ if unlikely($cond) { unsafe { dump_page($page, concat!(stringify!($cond), "\0").as_ptr() as *const _); } BUG!(); } }}; }
#[cfg(CONFIG_DEBUG_VM)]
#[macro_export]
macro_rules! VM_BUG_ON_FOLIO { ($cond:expr, $folio:expr) => {{ if unlikely($cond) { unsafe { dump_page(&(*$folio).page, concat!(stringify!($cond), "\0").as_ptr() as *const _); } BUG!(); } }}; }
#[cfg(CONFIG_DEBUG_VM)]
#[macro_export]
macro_rules! VM_BUG_ON_VMA { ($cond:expr, $vma:expr) => {{ if unlikely($cond) { unsafe { dump_vma($vma); } BUG!(); } }}; }
#[cfg(CONFIG_DEBUG_VM)]
#[macro_export]
macro_rules! VM_BUG_ON_MM { ($cond:expr, $mm:expr) => {{ if unlikely($cond) { unsafe { dump_mm($mm); } BUG!(); } }}; }

#[cfg(CONFIG_DEBUG_VM)]
#[macro_export]
macro_rules! VM_WARN_ON_PAGE { ($cond:expr, $page:expr) => {{ let __ret_warn = ($cond) as i32; if unlikely(__ret_warn != 0) { unsafe { dump_page($page, concat!(stringify!($cond), "\0").as_ptr() as *const _); } WARN_ON!(true); } unlikely(__ret_warn != 0) }}; }
#[cfg(CONFIG_DEBUG_VM)]
#[macro_export]
macro_rules! VM_WARN_ON_FOLIO { ($cond:expr, $folio:expr) => {{ let __ret_warn = ($cond) as i32; if unlikely(__ret_warn != 0) { unsafe { dump_page(&(*$folio).page, concat!(stringify!($cond), "\0").as_ptr() as *const _); } WARN_ON!(true); } unlikely(__ret_warn != 0) }}; }
#[cfg(CONFIG_DEBUG_VM)]
#[macro_export]
macro_rules! VM_WARN_ON_VMG { ($cond:expr, $vmg:expr) => {{ let __ret_warn = ($cond) as i32; if unlikely(__ret_warn != 0) { unsafe { dump_vmg($vmg, concat!(stringify!($cond), "\0").as_ptr() as *const _); } WARN_ON!(true); } unlikely(__ret_warn != 0) }}; }

// The once variants retain function-local static state, as in the C macros.
#[cfg(CONFIG_DEBUG_VM)]
#[macro_export]
macro_rules! VM_WARN_ON_ONCE_PAGE { ($cond:expr, $page:expr) => {{ static mut __WARNED: bool = false; let __ret_warn_once = ($cond) as i32; if unlikely(__ret_warn_once != 0 && unsafe { !__WARNED }) { unsafe { dump_page($page, concat!(stringify!($cond), "\0").as_ptr() as *const _); __WARNED = true; } WARN_ON!(true); } unlikely(__ret_warn_once != 0) }}; }
#[cfg(CONFIG_DEBUG_VM)]
#[macro_export]
macro_rules! VM_WARN_ON_ONCE_FOLIO { ($cond:expr, $folio:expr) => {{ static mut __WARNED: bool = false; let __ret_warn_once = ($cond) as i32; if unlikely(__ret_warn_once != 0 && unsafe { !__WARNED }) { unsafe { dump_page(&(*$folio).page, concat!(stringify!($cond), "\0").as_ptr() as *const _); __WARNED = true; } WARN_ON!(true); } unlikely(__ret_warn_once != 0) }}; }
#[cfg(CONFIG_DEBUG_VM)]
#[macro_export]
macro_rules! VM_WARN_ON_ONCE_MM { ($cond:expr, $mm:expr) => {{ static mut __WARNED: bool = false; let __ret_warn_once = ($cond) as i32; if unlikely(__ret_warn_once != 0 && unsafe { !__WARNED }) { unsafe { dump_mm($mm); __WARNED = true; } WARN_ON!(true); } unlikely(__ret_warn_once != 0) }}; }
#[cfg(CONFIG_DEBUG_VM)]
#[macro_export]
macro_rules! VM_WARN_ON_ONCE_VMA { ($cond:expr, $vma:expr) => {{ static mut __WARNED: bool = false; let __ret_warn_once = ($cond) as i32; if unlikely(__ret_warn_once != 0 && unsafe { !__WARNED }) { unsafe { dump_vma($vma); __WARNED = true; } WARN_ON!(true); } unlikely(__ret_warn_once != 0) }}; }

#[cfg(CONFIG_DEBUG_VM)]
#[macro_export]
macro_rules! VM_WARN_ON { ($cond:expr) => {{ WARN_ON!($cond); }}; }
#[cfg(CONFIG_DEBUG_VM)]
#[macro_export]
macro_rules! VM_WARN_ON_ONCE { ($cond:expr) => {{ WARN_ON_ONCE!($cond); }}; }
#[cfg(CONFIG_DEBUG_VM)]
#[macro_export]
macro_rules! VM_WARN_ONCE { ($cond:expr $(, $format:tt)*) => {{ WARN_ONCE!($cond $(, $format)*); }}; }
#[cfg(CONFIG_DEBUG_VM)]
#[macro_export]
macro_rules! VM_WARN { ($cond:expr $(, $format:tt)*) => {{ WARN!($cond $(, $format)*); }}; }

#[cfg(not(CONFIG_DEBUG_VM))]
#[macro_export]
macro_rules! VM_BUG_ON { ($cond:expr) => { BUILD_BUG_ON_INVALID!($cond) }; }
#[cfg(not(CONFIG_DEBUG_VM))]
#[macro_export]
macro_rules! VM_BUG_ON_PAGE { ($cond:expr, $page:expr) => { BUILD_BUG_ON_INVALID!($cond) }; }
#[cfg(not(CONFIG_DEBUG_VM))]
#[macro_export]
macro_rules! VM_BUG_ON_FOLIO { ($cond:expr, $folio:expr) => { BUILD_BUG_ON_INVALID!($cond) }; }
#[cfg(not(CONFIG_DEBUG_VM))]
#[macro_export]
macro_rules! VM_BUG_ON_VMA { ($cond:expr, $vma:expr) => { BUILD_BUG_ON_INVALID!($cond) }; }
#[cfg(not(CONFIG_DEBUG_VM))]
#[macro_export]
macro_rules! VM_BUG_ON_MM { ($cond:expr, $mm:expr) => { BUILD_BUG_ON_INVALID!($cond) }; }
#[cfg(not(CONFIG_DEBUG_VM))]
#[macro_export]
macro_rules! VM_WARN_ON { ($cond:expr) => { BUILD_BUG_ON_INVALID!($cond) }; }
#[cfg(not(CONFIG_DEBUG_VM))]
#[macro_export]
macro_rules! VM_WARN_ON_ONCE { ($cond:expr) => { BUILD_BUG_ON_INVALID!($cond) }; }
#[cfg(not(CONFIG_DEBUG_VM))]
#[macro_export]
macro_rules! VM_WARN_ON_PAGE { ($cond:expr, $page:expr) => { BUILD_BUG_ON_INVALID!($cond) }; }
#[cfg(not(CONFIG_DEBUG_VM))]
#[macro_export]
macro_rules! VM_WARN_ON_ONCE_PAGE { ($cond:expr, $page:expr) => { BUILD_BUG_ON_INVALID!($cond) }; }
#[cfg(not(CONFIG_DEBUG_VM))]
#[macro_export]
macro_rules! VM_WARN_ON_FOLIO { ($cond:expr, $folio:expr) => { BUILD_BUG_ON_INVALID!($cond) }; }
#[cfg(not(CONFIG_DEBUG_VM))]
#[macro_export]
macro_rules! VM_WARN_ON_ONCE_FOLIO { ($cond:expr, $folio:expr) => { BUILD_BUG_ON_INVALID!($cond) }; }
#[cfg(not(CONFIG_DEBUG_VM))]
#[macro_export]
macro_rules! VM_WARN_ON_ONCE_MM { ($cond:expr, $mm:expr) => { BUILD_BUG_ON_INVALID!($cond) }; }
#[cfg(not(CONFIG_DEBUG_VM))]
#[macro_export]
macro_rules! VM_WARN_ON_ONCE_VMA { ($cond:expr, $vma:expr) => { BUILD_BUG_ON_INVALID!($cond) }; }
#[cfg(not(CONFIG_DEBUG_VM))]
#[macro_export]
macro_rules! VM_WARN_ON_VMG { ($cond:expr, $vmg:expr) => { BUILD_BUG_ON_INVALID!($cond) }; }
#[cfg(not(CONFIG_DEBUG_VM))]
#[macro_export]
macro_rules! VM_WARN_ONCE { ($cond:expr $(, $format:tt)*) => { BUILD_BUG_ON_INVALID!($cond) }; }
#[cfg(not(CONFIG_DEBUG_VM))]
#[macro_export]
macro_rules! VM_WARN { ($cond:expr $(, $format:tt)*) => { BUILD_BUG_ON_INVALID!($cond) }; }

#[cfg(CONFIG_DEBUG_VM_IRQSOFF)]
#[macro_export]
macro_rules! VM_WARN_ON_IRQS_ENABLED { () => { WARN_ON_ONCE!(!irqs_disabled()) }; }
#[cfg(not(CONFIG_DEBUG_VM_IRQSOFF))]
#[macro_export]
macro_rules! VM_WARN_ON_IRQS_ENABLED { () => {{}}; }

#[cfg(CONFIG_DEBUG_VIRTUAL)]
#[macro_export]
macro_rules! VIRTUAL_BUG_ON { ($cond:expr) => { BUG_ON!($cond) }; }
#[cfg(not(CONFIG_DEBUG_VIRTUAL))]
#[macro_export]
macro_rules! VIRTUAL_BUG_ON { ($cond:expr) => {{}}; }

#[cfg(CONFIG_DEBUG_VM_PGFLAGS)]
#[macro_export]
macro_rules! VM_BUG_ON_PGFLAGS { ($cond:expr, $page:expr) => { VM_BUG_ON_PAGE!($cond, $page) }; }
#[cfg(not(CONFIG_DEBUG_VM_PGFLAGS))]
#[macro_export]
macro_rules! VM_BUG_ON_PGFLAGS { ($cond:expr, $page:expr) => { BUILD_BUG_ON_INVALID!($cond) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
