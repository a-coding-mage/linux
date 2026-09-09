/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/page_idle.h.
// Dependencies from linux/bitops.h, linux/page-flags.h, and linux/page_ext.h
// are supplied by other translation units.

#[cfg(all(feature = "CONFIG_PAGE_IDLE_FLAG", not(target_pointer_width = "64")))]
// If there is not enough space to store Idle and Young bits in page flags, use
// page ext flags instead.
pub unsafe fn folio_test_young(folio: *const folio) -> bool {
    let page_ext: *mut page_ext = page_ext_get(&(*folio).page);
    let page_young: bool;

    if unlikely(!page_ext.is_null()) == false {
        return false;
    }

    page_young = test_bit(PAGE_EXT_YOUNG, &mut (*page_ext).flags);
    page_ext_put(page_ext);

    page_young
}

#[cfg(all(feature = "CONFIG_PAGE_IDLE_FLAG", not(target_pointer_width = "64")))]
pub unsafe fn folio_set_young(folio: *mut folio) {
    let page_ext: *mut page_ext = page_ext_get(&(*folio).page);

    if unlikely(!page_ext.is_null()) == false {
        return;
    }

    set_bit(PAGE_EXT_YOUNG, &mut (*page_ext).flags);
    page_ext_put(page_ext);
}

#[cfg(all(feature = "CONFIG_PAGE_IDLE_FLAG", not(target_pointer_width = "64")))]
pub unsafe fn folio_test_clear_young(folio: *mut folio) -> bool {
    let page_ext: *mut page_ext = page_ext_get(&(*folio).page);
    let page_young: bool;

    if unlikely(!page_ext.is_null()) == false {
        return false;
    }

    page_young = test_and_clear_bit(PAGE_EXT_YOUNG, &mut (*page_ext).flags);
    page_ext_put(page_ext);

    page_young
}

#[cfg(all(feature = "CONFIG_PAGE_IDLE_FLAG", not(target_pointer_width = "64")))]
pub unsafe fn folio_test_idle(folio: *const folio) -> bool {
    let page_ext: *mut page_ext = page_ext_get(&(*folio).page);
    let page_idle: bool;

    if unlikely(!page_ext.is_null()) == false {
        return false;
    }

    page_idle = test_bit(PAGE_EXT_IDLE, &mut (*page_ext).flags);
    page_ext_put(page_ext);

    page_idle
}

#[cfg(all(feature = "CONFIG_PAGE_IDLE_FLAG", not(target_pointer_width = "64")))]
pub unsafe fn folio_set_idle(folio: *mut folio) {
    let page_ext: *mut page_ext = page_ext_get(&(*folio).page);

    if unlikely(!page_ext.is_null()) == false {
        return;
    }

    set_bit(PAGE_EXT_IDLE, &mut (*page_ext).flags);
    page_ext_put(page_ext);
}

#[cfg(all(feature = "CONFIG_PAGE_IDLE_FLAG", not(target_pointer_width = "64")))]
pub unsafe fn folio_clear_idle(folio: *mut folio) {
    let page_ext: *mut page_ext = page_ext_get(&(*folio).page);

    if unlikely(!page_ext.is_null()) == false {
        return;
    }

    clear_bit(PAGE_EXT_IDLE, &mut (*page_ext).flags);
    page_ext_put(page_ext);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
