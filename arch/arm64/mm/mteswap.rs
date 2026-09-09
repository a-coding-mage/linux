// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation unit.

static mut mte_pages: XArray = XArray::new();

unsafe extern "C" {
    fn kmalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn page_mte_tagged(page: *mut page) -> bool;
    fn page_address(page: *mut page) -> *mut core::ffi::c_void;
    fn mte_save_page_tags(address: *mut core::ffi::c_void, storage: *mut core::ffi::c_void);
    fn page_swap_entry(page: *mut page) -> swp_entry_t;
    fn xa_store(
        array: *mut XArray,
        index: u64,
        entry: *mut core::ffi::c_void,
        gfp: u32,
    ) -> *mut core::ffi::c_void;
    fn xa_is_err(entry: *mut core::ffi::c_void) -> bool;
    fn xa_err(entry: *mut core::ffi::c_void) -> i32;
    fn xa_load(array: *mut XArray, index: u64) -> *mut core::ffi::c_void;
    fn try_page_mte_tagging(page: *mut page) -> bool;
    fn mte_restore_page_tags(address: *mut core::ffi::c_void, storage: *mut core::ffi::c_void);
    fn set_page_mte_tagged(page: *mut page);
    fn swp_entry(typ: i32, offset: pgoff_t) -> swp_entry_t;
    fn xa_erase(array: *mut XArray, index: u64) -> *mut core::ffi::c_void;
    fn swp_type(entry: swp_entry_t) -> i32;
    fn swp_offset(entry: swp_entry_t) -> pgoff_t;
    fn system_supports_mte() -> bool;
    fn folio_nr_pages(folio: *mut folio) -> i64;
    fn folio_page(folio: *mut folio, index: i64) -> *mut page;
    fn __xa_erase(array: *mut XArray, index: u64) -> *mut core::ffi::c_void;
    fn xa_lock(array: *mut XArray);
    fn xa_unlock(array: *mut XArray);
}

const MTE_PAGE_TAG_STORAGE: usize = 0; // supplied by <asm/mte.h>
const GFP_KERNEL: u32 = 0; // supplied by <linux/gfp.h>

#[repr(C)]
pub struct XArray {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct page {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct folio {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct swp_entry_t {
    pub val: u64,
}

pub type pgoff_t = u64;

pub unsafe fn mte_allocate_tag_storage() -> *mut core::ffi::c_void {
    /* tags granule is 16 bytes, 2 tags stored per byte */
    kmalloc(MTE_PAGE_TAG_STORAGE, GFP_KERNEL)
}

pub unsafe fn mte_free_tag_storage(storage: *mut i8) {
    kfree(storage as *mut core::ffi::c_void);
}

pub unsafe fn mte_save_tags(page: *mut page) -> i32 {
    let tag_storage: *mut core::ffi::c_void;
    let ret: *mut core::ffi::c_void;

    if !page_mte_tagged(page) {
        return 0;
    }

    tag_storage = mte_allocate_tag_storage();
    if tag_storage.is_null() {
        return -12; // -ENOMEM
    }

    mte_save_page_tags(page_address(page), tag_storage);

    /* lookup the swap entry.val from the page */
    ret = xa_store(&raw mut mte_pages, page_swap_entry(page).val, tag_storage, GFP_KERNEL);
    if xa_is_err(ret) {
        mte_free_tag_storage(tag_storage as *mut i8);
        return xa_err(ret);
    } else if !ret.is_null() {
        /* Entry is being replaced, free the old entry */
        mte_free_tag_storage(ret as *mut i8);
    }

    0
}

pub unsafe fn mte_restore_tags(mut entry: swp_entry_t, page: *mut page) {
    let tags = xa_load(&raw mut mte_pages, entry.val);

    if tags.is_null() {
        return;
    }

    if try_page_mte_tagging(page) {
        mte_restore_page_tags(page_address(page), tags);
        set_page_mte_tagged(page);
    }
}

pub unsafe fn mte_invalidate_tags(typ: i32, offset: pgoff_t) {
    let entry = swp_entry(typ, offset);
    let tags = xa_erase(&raw mut mte_pages, entry.val);

    mte_free_tag_storage(tags as *mut i8);
}

unsafe fn __mte_invalidate_tags(page: *mut page) {
    let entry = page_swap_entry(page);

    mte_invalidate_tags(swp_type(entry), swp_offset(entry));
}

pub unsafe fn mte_invalidate_tags_area(typ: i32) {
    let entry = swp_entry(typ, 0);
    let last_entry = swp_entry(typ + 1, 0);

    // XA_STATE(xa_state, &mte_pages, entry.val)
    let mut xa_index = entry.val;

    xa_lock(&raw mut mte_pages);
    while xa_index < last_entry.val {
        let tags = xa_load(&raw mut mte_pages, xa_index);
        if !tags.is_null() {
            __xa_erase(&raw mut mte_pages, xa_index);
            mte_free_tag_storage(tags as *mut i8);
        }
        xa_index += 1;
    }
    xa_unlock(&raw mut mte_pages);
}

pub unsafe fn arch_prepare_to_swap(folio: *mut folio) -> i32 {
    let mut i: i64;
    let nr: i64;
    let err: i32;

    if !system_supports_mte() {
        return 0;
    }

    nr = folio_nr_pages(folio);
    i = 0;
    while i < nr {
        err = mte_save_tags(folio_page(folio, i));
        if err != 0 {
            while i > 0 {
                i -= 1;
                __mte_invalidate_tags(folio_page(folio, i));
            }
            return err;
        }
        i += 1;
    }
    0
}

pub unsafe fn arch_swap_restore(mut entry: swp_entry_t, folio: *mut folio) {
    let nr: i64;

    if !system_supports_mte() {
        return;
    }

    nr = folio_nr_pages(folio);
    let mut i = 0;
    while i < nr {
        mte_restore_tags(entry, folio_page(folio, i));
        entry.val += 1;
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
