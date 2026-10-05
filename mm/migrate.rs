// SPDX-License-Identifier: GPL-2.0
// Executable owner of mm/migrate.c, reconciled against
// e1d84f501551943a11f4c5271e9f5c85d7e15168 (original C remains immutable).
mod b {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/migrate_native_generated.rs"
    ));
}
use b::*;
#[cfg(CONFIG_NUMA_MIGRATION)]
use core::mem::size_of;
use core::mem::zeroed;
use core::ptr::{addr_of_mut, null, null_mut};
use kernel::ffi::{c_int, c_long, c_uint, c_ulong, c_void};
include!("migrate_native_aliases.rs");
type NewFolio = rust_migrate_new_folio_t;
type FreeFolio = rust_migrate_free_folio_t;

const E_AGAIN: c_int = -(b::EAGAIN as c_int);
const E_BUSY: c_int = -(b::EBUSY as c_int);
const E_INVAL: c_int = -(b::EINVAL as c_int);
const E_NOMEM: c_int = -(b::ENOMEM as c_int);
#[cfg(CONFIG_NUMA_MIGRATION)]
const E_FAULT: c_int = -(b::EFAULT as c_int);
#[cfg(CONFIG_NUMA_MIGRATION)]
const E_NOENT: c_int = -(b::ENOENT as c_int);
#[cfg(any(CONFIG_NUMA_MIGRATION, CONFIG_NUMA_BALANCING))]
const E_ACCES: c_int = -(b::EACCES as c_int);
#[cfg(CONFIG_NUMA_MIGRATION)]
const E_NODEV: c_int = -(b::ENODEV as c_int);
#[cfg(CONFIG_NUMA_MIGRATION)]
const E_PERM: c_int = -(b::EPERM as c_int);
#[cfg(CONFIG_NUMA_MIGRATION)]
const E_SRCH: c_int = -(b::ESRCH as c_int);
#[cfg(CONFIG_NUMA_MIGRATION)]
const E_INTR: c_int = -(b::EINTR as c_int);
const E_OPNOTSUPP: c_int = -(b::EOPNOTSUPP as c_int);

// Original source-local state: registration remains intentionally unserialized.
static mut OFFLINE_MOVABLE_OPS: *const movable_operations = null();
static mut ZSMALLOC_MOVABLE_OPS: *const movable_operations = null();

// VM diagnostics do not evaluate their expressions with DEBUG_VM disabled.
macro_rules! vm_diag {
    ($leaf:ident($($arg:expr),* $(,)?)) => {{
        #[cfg(CONFIG_DEBUG_VM)]
        { $leaf($($arg),*); }
    }};
}

#[no_mangle]
pub unsafe extern "C" fn set_movable_ops(ops: *const movable_operations, kind: pagetype) -> c_int {
    let slot = match kind {
        PGTY_offline => addr_of_mut!(OFFLINE_MOVABLE_OPS),
        PGTY_zsmalloc => addr_of_mut!(ZSMALLOC_MOVABLE_OPS),
        _ => return E_INVAL,
    };
    if !(*slot).is_null() && !ops.is_null() {
        return E_BUSY;
    }
    *slot = ops;
    0
}

unsafe fn page_movable_ops(page: *mut page) -> *const movable_operations {
    vm_diag!(warn_movable_type(!page_has_movable_ops(page), page));
    if page_offline(page) {
        return OFFLINE_MOVABLE_OPS;
    }
    if page_zsmalloc(page) {
        return ZSMALLOC_MOVABLE_OPS;
    }
    null()
}

#[no_mangle]
pub unsafe extern "C" fn isolate_movable_ops_page(page: *mut page, mode: isolate_mode_t) -> bool {
    let folio = folio_get_nontail_page(page);
    if folio.is_null() {
        return false;
    }
    if !page_has_movable_ops(page) || !folio_trylock(folio) {
        folio_put(folio);
        return false;
    }
    let mut isolated = false;
    vm_diag!(warn_isolate_type(!page_has_movable_ops(page), page));
    if !page_movable_ops_isolated(page) {
        let mops = page_movable_ops(page);
        if !warn_missing_movable_ops(mops.is_null()) && ((*mops).isolate_page.unwrap())(page, mode)
        {
            vm_diag!(warn_isolate_driver(page_movable_ops_isolated(page), page));
            set_page_movable_ops_isolated(page);
            isolated = true;
        }
    }
    folio_unlock(folio);
    if !isolated {
        folio_put(folio);
    }
    isolated
}

unsafe fn putback_movable_ops_page(page: *mut page) {
    let folio = page_folio(page);
    vm_diag!(warn_putback_type(!page_has_movable_ops(page), page));
    vm_diag!(warn_putback_isolated(
        !page_movable_ops_isolated(page),
        page
    ));
    folio_lock(folio);
    ((*page_movable_ops(page)).putback_page.unwrap())(page);
    clear_page_movable_ops_isolated(page);
    folio_unlock(folio);
    folio_put(folio);
}

unsafe fn migrate_movable_ops_page(dst: *mut page, src: *mut page, mode: migrate_mode) -> c_int {
    vm_diag!(warn_move_type(!page_has_movable_ops(src), src));
    vm_diag!(warn_move_isolated(!page_movable_ops_isolated(src), src));
    let rc = ((*page_movable_ops(src)).migrate_page.unwrap())(dst, src, mode);
    if rc == 0 {
        clear_page_movable_ops_isolated(src);
    }
    rc
}

#[no_mangle]
pub unsafe extern "C" fn putback_movable_pages(head: *mut list_head) {
    let mut link = (*head).next;
    while link != head {
        let next = (*link).next;
        let folio = folio_from_lru(link);
        if folio_test_hugetlb(folio) {
            folio_putback_hugetlb(folio);
        } else {
            list_del(link);
            if page_has_movable_ops(folio_page(folio, 0)) {
                putback_movable_ops_page(folio_page(folio, 0));
            } else {
                node_stat_mod_folio(
                    folio,
                    isolated_stat(folio),
                    (folio_nr_pages(folio) as c_long).wrapping_neg(),
                );
                folio_putback_lru(folio);
            }
        }
        link = next;
    }
}

unsafe fn isolated_stat(folio: *mut folio) -> node_stat_item {
    (NR_ISOLATED_ANON as c_uint).wrapping_add(folio_is_file_lru(folio) as c_uint) as node_stat_item
}

#[no_mangle]
pub unsafe extern "C" fn isolate_folio_to_list(folio: *mut folio, head: *mut list_head) -> bool {
    if folio_test_hugetlb(folio) {
        return folio_isolate_hugetlb(folio, head);
    }
    if page_has_movable_ops(folio_page(folio, 0)) {
        if !isolate_movable_ops_page(folio_page(folio, 0), RUST_MIGRATE_ISOLATE_UNEVICTABLE as _) {
            return false;
        }
    } else {
        if !folio_isolate_lru(folio) {
            return false;
        }
        node_stat_add_folio(folio, isolated_stat(folio));
    }
    list_add(folio_lru(folio), head);
    true
}

include!("migrate_ptes.rs");
include!("migrate_mapping.rs");
include!("migrate_move.rs");
include!("migrate_batch.rs");
#[cfg(CONFIG_NUMA_MIGRATION)]
include!("migrate_syscall.rs");
#[cfg(CONFIG_NUMA_BALANCING)]
include!("migrate_numa.rs");

// Historical provenance marker retained; see reconciled hash in review/PROVENANCE.md.
// Historical initial transcription: d482bb509b7d065808de40ce78b5bca39f40b783
// SOURCE-COMMIT: e1d84f501551943a11f4c5271e9f5c85d7e15168
