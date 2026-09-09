// SPDX-License-Identifier: GPL-2.0
// Dependencies corresponding to <linux/mm.h> and <linux/page-isolation.h>
// are supplied by the surrounding kernel translation.

pub static mut _debug_guardpage_minorder: ::core::ffi::c_uint = 0;

pub static mut _debug_pagealloc_enabled_early: bool =
    cfg!(feature = "CONFIG_DEBUG_PAGEALLOC_ENABLE_DEFAULT");

// DEFINE_STATIC_KEY_FALSE(_debug_pagealloc_enabled);
// DEFINE_STATIC_KEY_FALSE(_debug_guardpage_enabled);

extern "C" {
    fn kstrtobool(buf: *mut ::core::ffi::c_char, val: *mut bool) -> i32;
    fn kstrtouint(
        buf: *mut ::core::ffi::c_char,
        base: ::core::ffi::c_uint,
        res: *mut ::core::ffi::c_uint,
    ) -> i32;
    fn pr_err(fmt: *const ::core::ffi::c_char, ...);
    fn pr_info(fmt: *const ::core::ffi::c_char, ...);
    fn debug_guardpage_minorder() -> ::core::ffi::c_uint;
    fn __SetPageGuard(page: *mut Page);
    fn __ClearPageGuard(page: *mut Page);
    fn INIT_LIST_HEAD(list: *mut ListHead);
    fn set_page_private(page: *mut Page, value: usize);
}

#[repr(C)]
pub struct Zone {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Page {
    pub buddy_list: ListHead,
}

#[repr(C)]
pub struct ListHead {
    _private: [u8; 0],
}

unsafe fn early_debug_pagealloc(buf: *mut ::core::ffi::c_char) -> i32 {
    kstrtobool(buf, &raw mut _debug_pagealloc_enabled_early)
}

unsafe fn debug_guardpage_minorder_setup(buf: *mut ::core::ffi::c_char) -> i32 {
    let mut res: ::core::ffi::c_uint = 0;

    // MAX_PAGE_ORDER and the kernel's nullable-string formatting are supplied
    // by the surrounding kernel translation.
    if buf.is_null()
        || kstrtouint(buf, 10, &mut res) < 0
        || res > MAX_PAGE_ORDER / 2
    {
        pr_err(
            b"Bad debug_guardpage_minorder value: %s\n\0".as_ptr() as *const _,
            if buf.is_null() {
                b"(missing)\0".as_ptr() as *const _
            } else {
                buf
            },
        );
        return 0;
    }
    _debug_guardpage_minorder = res;
    pr_info(
        b"Setting debug_guardpage_minorder to %u\n\0".as_ptr() as *const _,
        res,
    );
    0
}

pub unsafe fn __set_page_guard(
    _zone: *mut Zone,
    page: *mut Page,
    order: ::core::ffi::c_uint,
) -> bool {
    if order >= debug_guardpage_minorder() {
        return false;
    }

    __SetPageGuard(page);
    INIT_LIST_HEAD(&mut (*page).buddy_list);
    set_page_private(page, order as usize);

    true
}

pub unsafe fn __clear_page_guard(
    _zone: *mut Zone,
    page: *mut Page,
    _order: ::core::ffi::c_uint,
) {
    __ClearPageGuard(page);
    set_page_private(page, 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
