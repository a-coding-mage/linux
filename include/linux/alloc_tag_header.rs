/* SPDX-License-Identifier: GPL-2.0 */
/* allocation tagging */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct AllocTagCounters {
    pub bytes: u64,
    pub calls: u64,
}

#[repr(C, align(8))]
pub struct AllocTag {
    pub ct: crate::codetag::CodeTag,
    pub counters: *mut AllocTagCounters,
}

#[repr(C)]
pub struct AllocTagKernelSection {
    pub first_tag: *mut AllocTag,
    pub count: core::ffi::c_ulong,
}

#[repr(C)]
pub union AllocTagModuleSectionStart {
    pub start_addr: core::ffi::c_ulong,
    pub first_tag: *mut AllocTag,
}

#[repr(C)]
pub struct AllocTagModuleSection {
    pub start: AllocTagModuleSectionStart,
    pub end_addr: core::ffi::c_ulong,
    /* used size */
    pub size: core::ffi::c_ulong,
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING_DEBUG)]
pub const CODETAG_EMPTY: *mut core::ffi::c_void = 1usize as *mut core::ffi::c_void;

#[cfg(CONFIG_MEM_ALLOC_PROFILING_DEBUG)]
#[inline]
pub unsafe fn is_codetag_empty(ref_: *mut crate::codetag::CodeTagRef) -> bool {
    (*ref_).ct == CODETAG_EMPTY
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING_DEBUG)]
#[inline]
pub unsafe fn set_codetag_empty(ref_: *mut crate::codetag::CodeTagRef) {
    if !ref_.is_null() {
        (*ref_).ct = CODETAG_EMPTY;
    }
}

#[cfg(not(CONFIG_MEM_ALLOC_PROFILING_DEBUG))]
#[inline]
pub unsafe fn is_codetag_empty(_ref_: *mut crate::codetag::CodeTagRef) -> bool { false }

#[cfg(not(CONFIG_MEM_ALLOC_PROFILING_DEBUG))]
#[inline]
pub unsafe fn set_codetag_empty(ref_: *mut crate::codetag::CodeTagRef) {
    if !ref_.is_null() {
        (*ref_).ct = core::ptr::null_mut();
    }
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
pub const ALLOC_TAG_SECTION_NAME: &str = "alloc_tags";

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[repr(C)]
pub struct CodeTagBytes {
    pub ct: *mut crate::codetag::CodeTag,
    pub bytes: i64,
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
extern "C" {
    pub fn alloc_tag_top_users(
        tags: *mut CodeTagBytes,
        count: usize,
        can_sleep: bool,
    ) -> usize;
    pub static mut mem_alloc_profiling_key: crate::static_key::StaticKey;
    pub fn mem_alloc_profiling_permanently_disabled() -> bool;
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline]
pub unsafe fn ct_to_alloc_tag(ct: *mut crate::codetag::CodeTag) -> *mut AllocTag {
    (ct as *mut u8).sub(core::mem::offset_of!(AllocTag, ct)) as *mut AllocTag
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline]
pub unsafe fn mem_alloc_profiling_enabled() -> bool {
    crate::static_key::static_branch_maybe(CONFIG_MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT, &mem_alloc_profiling_key)
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline]
pub unsafe fn alloc_tag_read(tag: *mut AllocTag) -> AllocTagCounters {
    let mut v = AllocTagCounters { bytes: 0, calls: 0 };
    for cpu in crate::percpu::for_each_possible_cpu() {
        let counter = crate::percpu::per_cpu_ptr((*tag).counters, cpu);
        v.bytes = v.bytes.wrapping_add((*counter).bytes);
        v.calls = v.calls.wrapping_add((*counter).calls);
    }
    v
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline]
pub unsafe fn __alloc_tag_ref_set(ref_: *mut crate::codetag::CodeTagRef, tag: *mut AllocTag) -> bool {
    alloc_tag_add_check(ref_, tag);
    if ref_.is_null() || tag.is_null() { return false; }
    (*ref_).ct = &mut (*tag).ct;
    true
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING_DEBUG)]
#[inline]
pub unsafe fn alloc_tag_add_check(ref_: *mut crate::codetag::CodeTagRef, tag: *mut AllocTag) {
    crate::bug::warn_once(ref_ != core::ptr::null_mut() && !(*ref_).ct.is_null() && !is_codetag_empty(ref_), "alloc_tag was not cleared (got tag)");
    crate::bug::warn_once(tag.is_null(), "current->alloc_tag not set");
}

#[cfg(not(CONFIG_MEM_ALLOC_PROFILING_DEBUG))]
#[inline]
pub unsafe fn alloc_tag_add_check(_ref_: *mut crate::codetag::CodeTagRef, _tag: *mut AllocTag) {}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline]
pub unsafe fn alloc_tag_ref_set(ref_: *mut crate::codetag::CodeTagRef, tag: *mut AllocTag) -> bool {
    if !__alloc_tag_ref_set(ref_, tag) { return false; }
    crate::percpu::this_cpu_inc(&mut (*(*tag).counters).calls);
    true
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline]
pub unsafe fn alloc_tag_add(ref_: *mut crate::codetag::CodeTagRef, tag: *mut AllocTag, bytes: usize) {
    if alloc_tag_ref_set(ref_, tag) { crate::percpu::this_cpu_add(&mut (*(*tag).counters).bytes, bytes); }
}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline]
pub unsafe fn alloc_tag_sub(ref_: *mut crate::codetag::CodeTagRef, bytes: usize) {
    if ref_.is_null() || (*ref_).ct.is_null() { return; }
    if is_codetag_empty(ref_) { (*ref_).ct = core::ptr::null_mut(); return; }
    let tag = ct_to_alloc_tag((*ref_).ct);
    crate::percpu::this_cpu_sub(&mut (*(*tag).counters).bytes, bytes);
    crate::percpu::this_cpu_dec(&mut (*(*tag).counters).calls);
    (*ref_).ct = core::ptr::null_mut();
}

#[cfg(not(CONFIG_MEM_ALLOC_PROFILING))]
#[inline] pub fn mem_alloc_profiling_enabled() -> bool { false }
#[cfg(not(CONFIG_MEM_ALLOC_PROFILING))]
#[inline] pub fn mem_alloc_profiling_permanently_disabled() -> bool { true }
#[cfg(not(CONFIG_MEM_ALLOC_PROFILING))]
#[inline] pub unsafe fn alloc_tag_add(_ref_: *mut crate::codetag::CodeTagRef, _tag: *mut AllocTag, _bytes: usize) {}
#[cfg(not(CONFIG_MEM_ALLOC_PROFILING))]
#[inline] pub unsafe fn alloc_tag_sub(_ref_: *mut crate::codetag::CodeTagRef, _bytes: usize) {}

#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline]
pub unsafe fn alloc_tag_set_inaccurate(tag: *mut AllocTag) { (*tag).ct.flags |= crate::codetag::CODETAG_FLAG_INACCURATE; }
#[cfg(CONFIG_MEM_ALLOC_PROFILING)]
#[inline]
pub unsafe fn alloc_tag_is_inaccurate(tag: *mut AllocTag) -> bool { ((*tag).ct.flags & crate::codetag::CODETAG_FLAG_INACCURATE) != 0 }
#[cfg(not(CONFIG_MEM_ALLOC_PROFILING))]
#[inline] pub unsafe fn alloc_tag_set_inaccurate(_tag: *mut AllocTag) {}
#[cfg(not(CONFIG_MEM_ALLOC_PROFILING))]
#[inline] pub unsafe fn alloc_tag_is_inaccurate(_tag: *mut AllocTag) -> bool { false }

// C macros DEFINE_ALLOC_TAG, alloc_tag_record, alloc_hooks_tag, and alloc_hooks
// are retained as declaration-level Rust macro equivalents for call-site use.
#[macro_export]
macro_rules! DEFINE_ALLOC_TAG { ($name:ident) => { static mut $name: $crate::AllocTag = unsafe { core::mem::zeroed() }; }; }
#[macro_export]
macro_rules! alloc_tag_record { ($p:expr) => { $p = unsafe { crate::current::current_alloc_tag() }; }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
