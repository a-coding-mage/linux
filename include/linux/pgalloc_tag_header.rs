/* SPDX-License-Identifier: GPL-2.0 */
/* page allocation tagging */

// C dependency: <linux/alloc_tag.h>
// The following items are enabled when CONFIG_MEM_ALLOC_PROFILING is enabled.

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
extern "C" {
    pub static mut page_alloc_tagging_ops: page_ext_operations;
    pub static mut alloc_tag_ref_mask: c_ulong;
    pub static mut alloc_tag_ref_offs: c_int;
    pub static mut kernel_tags: alloc_tag_kernel_section;
    pub static mut mem_profiling_compressed: static_key_false;
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
pub type pgalloc_tag_idx = u16;

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[repr(C)]
pub union pgtag_ref_handle {
    pub ref_: *mut codetag_ref,
    pub page: *mut page,
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
pub const CODETAG_ID_NULL: pgalloc_tag_idx = 0;
#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
pub const CODETAG_ID_EMPTY: pgalloc_tag_idx = 1;
#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
pub const CODETAG_ID_FIRST: pgalloc_tag_idx = 2;

#[cfg(all(CONFIG_MEM_ALLOC_PROFILING, CONFIG_MODULES))]
extern "C" {
    pub static mut module_tags: alloc_tag_module_section;
}

#[cfg(all(CONFIG_MEM_ALLOC_PROFILING, CONFIG_MODULES))]
#[inline]
pub unsafe fn module_idx_to_tag(idx: pgalloc_tag_idx) -> *mut alloc_tag {
    (*core::ptr::addr_of_mut!(module_tags)).first_tag.add(
        idx.wrapping_sub((*core::ptr::addr_of_mut!(kernel_tags)).count as pgalloc_tag_idx) as usize,
    )
}

#[cfg(all(CONFIG_MEM_ALLOC_PROFILING, CONFIG_MODULES))]
#[inline]
pub unsafe fn module_tag_to_idx(tag: *mut alloc_tag) -> pgalloc_tag_idx {
    (CODETAG_ID_FIRST as usize + (*core::ptr::addr_of_mut!(kernel_tags)).count as usize
        + tag.offset_from((*core::ptr::addr_of_mut!(module_tags)).first_tag) as usize) as pgalloc_tag_idx
}

#[cfg(all(CONFIG_MEM_ALLOC_PROFILING, not(CONFIG_MODULES)))]
#[inline]
pub unsafe fn module_idx_to_tag(_idx: pgalloc_tag_idx) -> *mut alloc_tag {
    pr_warn(c"invalid page tag reference %lu\n".as_ptr(), _idx as c_ulong);
    core::ptr::null_mut()
}

#[cfg(all(CONFIG_MEM_ALLOC_PROFILING, not(CONFIG_MODULES)))]
#[inline]
pub unsafe fn module_tag_to_idx(_tag: *mut alloc_tag) -> pgalloc_tag_idx {
    pr_warn(c"invalid page tag 0x%lx\n".as_ptr(), _tag as c_ulong);
    CODETAG_ID_NULL
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline]
pub unsafe fn idx_to_ref(idx: pgalloc_tag_idx, ref_: *mut codetag_ref) {
    match idx {
        CODETAG_ID_NULL => (*ref_).ct = core::ptr::null_mut(),
        CODETAG_ID_EMPTY => set_codetag_empty(ref_),
        _ => {
            let i = idx.wrapping_sub(CODETAG_ID_FIRST) as usize;
            (*ref_).ct = if i < (*core::ptr::addr_of_mut!(kernel_tags)).count as usize {
                &mut (*core::ptr::addr_of_mut!(kernel_tags)).first_tag.add(i).ct
            } else {
                &mut (*module_idx_to_tag(i as pgalloc_tag_idx)).ct
            };
        }
    }
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline]
pub unsafe fn ref_to_idx(ref_: *mut codetag_ref) -> pgalloc_tag_idx {
    if (*ref_).ct.is_null() { return CODETAG_ID_NULL; }
    if is_codetag_empty(ref_) { return CODETAG_ID_EMPTY; }
    let tag = ct_to_alloc_tag((*ref_).ct);
    let first = (*core::ptr::addr_of_mut!(kernel_tags)).first_tag;
    let count = (*core::ptr::addr_of_mut!(kernel_tags)).count as usize;
    if tag >= first && tag < first.add(count) {
        return (CODETAG_ID_FIRST as usize + tag.offset_from(first) as usize) as pgalloc_tag_idx;
    }
    module_tag_to_idx(tag)
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
extern "C" {
    pub fn __clear_page_tag_ref(page: *mut page);
    pub fn pgalloc_tag_split(folio: *mut folio, old_order: c_int, new_order: c_int);
    pub fn pgalloc_tag_swap(new: *mut folio, old: *mut folio);
    pub fn alloc_tag_sec_init();
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline]
pub unsafe fn clear_page_tag_ref(page: *mut page) {
    if mem_alloc_profiling_enabled() { __clear_page_tag_ref(page); }
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline]
pub unsafe fn pgalloc_tag_get(page: *mut page) -> *mut alloc_tag {
    if mem_alloc_profiling_enabled() { __pgalloc_tag_get(page) } else { core::ptr::null_mut() }
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline]
unsafe fn __pgalloc_tag_get(_page: *mut page) -> *mut alloc_tag {
    let mut tag: *mut alloc_tag = core::ptr::null_mut();
    let mut handle = pgtag_ref_handle { page: core::ptr::null_mut() };
    let mut ref_ = codetag_ref { ct: core::ptr::null_mut() };
    if get_page_tag_ref(_page, &mut ref_, &mut handle) {
        alloc_tag_sub_check(&mut ref_);
        if !ref_.ct.is_null() && !is_codetag_empty(&mut ref_) {
            tag = ct_to_alloc_tag(ref_.ct);
        }
        put_page_tag_ref(handle);
    }
    tag
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline]
pub unsafe fn get_page_tag_ref(page_: *mut page, ref_: *mut codetag_ref, handle: *mut pgtag_ref_handle) -> bool {
    if page_.is_null() { return false; }
    if static_key_enabled(&mem_profiling_compressed) {
        let idx = ((*page_).flags.f >> alloc_tag_ref_offs) & alloc_tag_ref_mask;
        idx_to_ref(idx as pgalloc_tag_idx, ref_);
        (*handle).page = page_;
    } else {
        let page_ext_ = page_ext_get(page_);
        if page_ext_.is_null() { return false; }
        let tmp = page_ext_data(page_ext_, &page_alloc_tagging_ops) as *mut codetag_ref;
        (*ref_).ct = (*tmp).ct;
        (*handle).ref_ = tmp;
    }
    true
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline]
pub unsafe fn put_page_tag_ref(handle: pgtag_ref_handle) {
    if WARN_ON(handle.ref_.is_null()) { return; }
    if !static_key_enabled(&mem_profiling_compressed) {
        page_ext_put((handle.ref_ as *mut u8).sub(page_alloc_tagging_ops.offset as usize) as *mut core::ffi::c_void);
    }
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline]
pub unsafe fn update_page_tag_ref(handle: pgtag_ref_handle, ref_: *mut codetag_ref) {
    if static_key_enabled(&mem_profiling_compressed) {
        let page_ = handle.page;
        if WARN_ON(page_.is_null() || ref_.is_null()) { return; }
        let idx = ((ref_to_idx(ref_) as c_ulong) & alloc_tag_ref_mask) << alloc_tag_ref_offs;
        loop {
            let old_flags = READ_ONCE((*page_).flags.f);
            let mut flags = old_flags;
            flags &= !(alloc_tag_ref_mask << alloc_tag_ref_offs);
            flags |= idx;
            if !unlikely(!try_cmpxchg(&mut (*page_).flags.f, &old_flags, flags)) { break; }
        }
    } else {
        if WARN_ON(handle.ref_.is_null() || ref_.is_null()) { return; }
        (*handle.ref_).ct = (*ref_).ct;
    }
}

#[cfg(not(CONFIG_MEM_ALLOC_PROFILING))]
#[inline] pub unsafe fn get_page_tag_ref(_p: *mut page, _r: *mut codetag_ref, _h: *mut pgtag_ref_handle) -> bool { false }

#[cfg(not(CONFIG_MEM_ALLOC_PROFILING))]
#[inline] pub unsafe fn clear_page_tag_ref(_page: *mut page) {}
#[cfg(not(CONFIG_MEM_ALLOC_PROFILING))]
#[inline] pub unsafe fn alloc_tag_sec_init() {}
#[cfg(not(CONFIG_MEM_ALLOC_PROFILING))]
#[inline] pub unsafe fn pgalloc_tag_split(_folio: *mut folio, _old_order: c_int, _new_order: c_int) {}
#[cfg(not(CONFIG_MEM_ALLOC_PROFILING))]
#[inline] pub unsafe fn pgalloc_tag_swap(_new: *mut folio, _old: *mut folio) {}
#[cfg(not(CONFIG_MEM_ALLOC_PROFILING))]
#[inline] pub unsafe fn pgalloc_tag_get(_page: *mut page) -> *mut alloc_tag { core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
