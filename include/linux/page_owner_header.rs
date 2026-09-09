/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

#[cfg(feature = "CONFIG_PAGE_OWNER")]
extern "C" {
    pub static mut page_owner_inited: static_key_false;
    pub static mut page_owner_ops: page_ext_operations;

    pub fn __reset_page_owner(page: *mut page, order: u16);
    pub fn __set_page_owner(page: *mut page, order: u16, gfp_mask: gfp_t);
    pub fn __split_page_owner(page: *mut page, old_order: core::ffi::c_int, new_order: core::ffi::c_int);
    pub fn __folio_copy_owner(newfolio: *mut folio, old: *mut folio);
    pub fn __folio_set_owner_migrate_reason(folio: *mut folio, reason: migrate_reason);
    pub fn __dump_page_owner(page: *const page);
    pub fn pagetypeinfo_showmixedcount_print(
        m: *mut seq_file,
        pgdat: *mut pg_data_t,
        zone: *mut zone,
    );

    fn static_branch_unlikely(key: *const static_key_false) -> bool;
}

#[cfg(feature = "CONFIG_PAGE_OWNER")]
#[inline]
pub unsafe fn reset_page_owner(page: *mut page, order: u16) {
    if static_branch_unlikely(&raw const page_owner_inited) {
        __reset_page_owner(page, order);
    }
}

#[cfg(feature = "CONFIG_PAGE_OWNER")]
#[inline]
pub unsafe fn set_page_owner(page: *mut page, order: u16, gfp_mask: gfp_t) {
    if static_branch_unlikely(&raw const page_owner_inited) {
        __set_page_owner(page, order, gfp_mask);
    }
}

#[cfg(feature = "CONFIG_PAGE_OWNER")]
#[inline]
pub unsafe fn split_page_owner(page: *mut page, old_order: core::ffi::c_int, new_order: core::ffi::c_int) {
    if static_branch_unlikely(&raw const page_owner_inited) {
        __split_page_owner(page, old_order, new_order);
    }
}

#[cfg(feature = "CONFIG_PAGE_OWNER")]
#[inline]
pub unsafe fn folio_copy_owner(newfolio: *mut folio, old: *mut folio) {
    if static_branch_unlikely(&raw const page_owner_inited) {
        __folio_copy_owner(newfolio, old);
    }
}

#[cfg(feature = "CONFIG_PAGE_OWNER")]
#[inline]
pub unsafe fn folio_set_owner_migrate_reason(folio: *mut folio, reason: migrate_reason) {
    if static_branch_unlikely(&raw const page_owner_inited) {
        __folio_set_owner_migrate_reason(folio, reason);
    }
}

#[cfg(feature = "CONFIG_PAGE_OWNER")]
#[inline]
pub unsafe fn dump_page_owner(page: *const page) {
    if static_branch_unlikely(&raw const page_owner_inited) {
        __dump_page_owner(page);
    }
}

#[cfg(not(feature = "CONFIG_PAGE_OWNER"))]
#[inline]
pub unsafe fn reset_page_owner(_page: *mut page, _order: u16) {}

#[cfg(not(feature = "CONFIG_PAGE_OWNER"))]
#[inline]
pub unsafe fn set_page_owner(_page: *mut page, _order: u16, _gfp_mask: gfp_t) {}

#[cfg(not(feature = "CONFIG_PAGE_OWNER"))]
#[inline]
pub unsafe fn split_page_owner(
    _page: *mut page,
    _old_order: core::ffi::c_int,
    _new_order: core::ffi::c_int,
) {
}

#[cfg(not(feature = "CONFIG_PAGE_OWNER"))]
#[inline]
pub unsafe fn folio_copy_owner(_newfolio: *mut folio, _folio: *mut folio) {}

#[cfg(not(feature = "CONFIG_PAGE_OWNER"))]
#[inline]
pub unsafe fn folio_set_owner_migrate_reason(_folio: *mut folio, _reason: migrate_reason) {}

#[cfg(not(feature = "CONFIG_PAGE_OWNER"))]
#[inline]
pub unsafe fn dump_page_owner(_page: *const page) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
