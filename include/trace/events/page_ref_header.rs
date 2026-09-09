/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM page_ref
// The Linux tracepoint and included dependency definitions are supplied by
// the surrounding translation unit.

#[repr(C)]
pub struct Page {
    pub flags: PageFlags,
    pub _mapcount: libc::c_int,
    pub mapping: *mut libc::c_void,
}

#[repr(C)]
pub union PageFlags {
    pub f: libc::c_ulong,
    _opaque: [u8; core::mem::size_of::<libc::c_ulong>()],
}

unsafe extern "C" {
    fn page_to_pfn(page: *mut Page) -> libc::c_ulong;
    fn page_ref_count(page: *mut Page) -> libc::c_int;
    fn atomic_read(mapcount: *const libc::c_int) -> libc::c_int;
    fn get_pageblock_migratetype(page: *mut Page) -> libc::c_int;
    fn show_page_flags(flags: libc::c_ulong) -> *const libc::c_char;
}

#[repr(C)]
pub struct PageRefModTemplateEntry {
    pub pfn: libc::c_ulong,
    pub flags: libc::c_ulong,
    pub count: libc::c_int,
    pub mapcount: libc::c_int,
    pub mapping: *mut libc::c_void,
    pub mt: libc::c_int,
    pub val: libc::c_int,
}

#[repr(C)]
pub struct PageRefModAndTestTemplateEntry {
    pub pfn: libc::c_ulong,
    pub flags: libc::c_ulong,
    pub count: libc::c_int,
    pub mapcount: libc::c_int,
    pub mapping: *mut libc::c_void,
    pub mt: libc::c_int,
    pub val: libc::c_int,
    pub ret: libc::c_int,
}

// TP_fast_assign for page_ref_mod_template.
#[inline]
pub unsafe fn page_ref_mod_template_fast_assign(
    entry: *mut PageRefModTemplateEntry,
    page: *mut Page,
    v: libc::c_int,
) {
    (*entry).pfn = page_to_pfn(page);
    (*entry).flags = (*page).flags.f;
    (*entry).count = page_ref_count(page);
    (*entry).mapcount = atomic_read(&(*page)._mapcount);
    (*entry).mapping = (*page).mapping;
    (*entry).mt = get_pageblock_migratetype(page);
    (*entry).val = v;
}

// TP_fast_assign for page_ref_mod_and_test_template.
#[inline]
pub unsafe fn page_ref_mod_and_test_template_fast_assign(
    entry: *mut PageRefModAndTestTemplateEntry,
    page: *mut Page,
    v: libc::c_int,
    ret: libc::c_int,
) {
    (*entry).pfn = page_to_pfn(page);
    (*entry).flags = (*page).flags.f;
    (*entry).count = page_ref_count(page);
    (*entry).mapcount = atomic_read(&(*page)._mapcount);
    (*entry).mapping = (*page).mapping;
    (*entry).mt = get_pageblock_migratetype(page);
    (*entry).val = v;
    (*entry).ret = ret;
}

// TP_printk formats:
// page_ref_mod_template:
// "pfn=0x%lx flags=%s count=%d mapcount=%d mapping=%p mt=%d val=%d"
// page_ref_mod_and_test_template:
// "pfn=0x%lx flags=%s count=%d mapcount=%d mapping=%p mt=%d val=%d ret=%d"

// DECLARE_EVENT_CLASS(page_ref_mod_template)
pub struct PageRefModTemplate;

// DEFINE_EVENT(page_ref_mod_template, page_ref_set)
pub struct PageRefSet;
// DEFINE_EVENT(page_ref_mod_template, page_ref_mod)
pub struct PageRefMod;

// DECLARE_EVENT_CLASS(page_ref_mod_and_test_template)
pub struct PageRefModAndTestTemplate;

// DEFINE_EVENT(page_ref_mod_and_test_template, page_ref_mod_and_test)
pub struct PageRefModAndTest;
// DEFINE_EVENT(page_ref_mod_and_test_template, page_ref_mod_and_return)
pub struct PageRefModAndReturn;
// DEFINE_EVENT(page_ref_mod_and_test_template, page_ref_mod_unless)
pub struct PageRefModUnless;
// DEFINE_EVENT(page_ref_mod_and_test_template, page_ref_freeze)
pub struct PageRefFreeze;

// DEFINE_EVENT(page_ref_mod_template, page_ref_unfreeze)
pub struct PageRefUnfreeze;

// #include <trace/define_trace.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
