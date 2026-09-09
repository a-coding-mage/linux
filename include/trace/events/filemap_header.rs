/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM: filemap
// The C tracepoint framework declarations and included Linux definitions are
// supplied by the surrounding kernel translation.

#[repr(C)]
pub struct MmFilemapOpPageCacheEntry {
    pub i_ino: u64,
    pub pfn: ::core::ffi::c_ulong,
    pub index: ::core::ffi::c_ulong,
    pub s_dev: ::core::ffi::c_ulong,
    pub order: u8,
}

#[repr(C)]
pub struct MmFilemapOpPageCacheRangeEntry {
    pub i_ino: u64,
    pub s_dev: ::core::ffi::c_ulong,
    pub index: ::core::ffi::c_ulong,
    pub last_index: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct MmFilemapFaultEntry {
    pub i_ino: u64,
    pub s_dev: ::core::ffi::c_ulong,
    pub index: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct FilemapSetWbErrEntry {
    pub i_ino: u64,
    pub s_dev: ::core::ffi::c_ulong,
    pub errseq: u32,
}

#[repr(C)]
pub struct FileCheckAndAdvanceWbErrEntry {
    pub i_ino: u64,
    pub file: *mut File,
    pub s_dev: ::core::ffi::c_ulong,
    pub old: u32,
    pub new: u32,
}

// External kernel types and helpers referenced by the tracepoint prototypes.
#[repr(C)]
pub struct Folio {
    pub mapping: *mut AddressSpace,
    pub index: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct AddressSpace {
    pub host: *mut Inode,
}

#[repr(C)]
pub struct Inode {
    pub i_ino: u64,
    pub i_sb: *mut SuperBlock,
    pub i_rdev: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct SuperBlock {
    pub s_dev: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct File {
    pub f_mapping: *mut AddressSpace,
    pub f_wb_err: u32,
}

extern "C" {
    pub fn folio_pfn(folio: *const Folio) -> ::core::ffi::c_ulong;
    pub fn folio_order(folio: *const Folio) -> u8;
}

// DECLARE_EVENT_CLASS(mm_filemap_op_page_cache)
pub unsafe fn mm_filemap_op_page_cache_assign(
    entry: *mut MmFilemapOpPageCacheEntry,
    folio: *mut Folio,
) {
    (*entry).pfn = folio_pfn(folio);
    (*entry).i_ino = (*(*folio).mapping).host.as_ref().unwrap().i_ino;
    (*entry).index = (*folio).index;
    let host = (*(*folio).mapping).host.as_ref().unwrap();
    if !host.i_sb.is_null() {
        (*entry).s_dev = (*host.i_sb).s_dev;
    } else {
        (*entry).s_dev = host.i_rdev;
    }
    (*entry).order = folio_order(folio);
}

// DEFINE_EVENT(mm_filemap_op_page_cache, mm_filemap_delete_from_page_cache)
// DEFINE_EVENT(mm_filemap_op_page_cache, mm_filemap_add_to_page_cache)

// DECLARE_EVENT_CLASS(mm_filemap_op_page_cache_range)
pub unsafe fn mm_filemap_op_page_cache_range_assign(
    entry: *mut MmFilemapOpPageCacheRangeEntry,
    mapping: *mut AddressSpace,
    index: ::core::ffi::c_ulong,
    last_index: ::core::ffi::c_ulong,
) {
    (*entry).i_ino = (*mapping).host.as_ref().unwrap().i_ino;
    let host = (*mapping).host.as_ref().unwrap();
    if !host.i_sb.is_null() {
        (*entry).s_dev = (*host.i_sb).s_dev;
    } else {
        (*entry).s_dev = host.i_rdev;
    }
    (*entry).index = index;
    (*entry).last_index = last_index;
}

// DEFINE_EVENT(mm_filemap_op_page_cache_range, mm_filemap_get_pages)
// DEFINE_EVENT(mm_filemap_op_page_cache_range, mm_filemap_map_pages)

pub unsafe fn mm_filemap_fault_assign(
    entry: *mut MmFilemapFaultEntry,
    mapping: *mut AddressSpace,
    index: ::core::ffi::c_ulong,
) {
    (*entry).i_ino = (*mapping).host.as_ref().unwrap().i_ino;
    let host = (*mapping).host.as_ref().unwrap();
    (*entry).s_dev = if !host.i_sb.is_null() { (*host.i_sb).s_dev } else { host.i_rdev };
    (*entry).index = index;
}

pub unsafe fn filemap_set_wb_err_assign(
    entry: *mut FilemapSetWbErrEntry,
    mapping: *mut AddressSpace,
    eseq: u32,
) {
    (*entry).i_ino = (*mapping).host.as_ref().unwrap().i_ino;
    (*entry).errseq = eseq;
    let host = (*mapping).host.as_ref().unwrap();
    (*entry).s_dev = if !host.i_sb.is_null() { (*host.i_sb).s_dev } else { host.i_rdev };
}

pub unsafe fn file_check_and_advance_wb_err_assign(
    entry: *mut FileCheckAndAdvanceWbErrEntry,
    file: *mut File,
    old: u32,
) {
    (*entry).file = file;
    let mapping = (*file).f_mapping;
    (*entry).i_ino = (*mapping).host.as_ref().unwrap().i_ino;
    let host = (*mapping).host.as_ref().unwrap();
    (*entry).s_dev = if !host.i_sb.is_null() { (*host.i_sb).s_dev } else { host.i_rdev };
    (*entry).old = old;
    (*entry).new = (*file).f_wb_err;
}

// TP_printk format strings are retained as comments because formatting is
// performed by the external tracepoint framework.
// mm_filemap_op_page_cache: "dev %d:%d ino %llx pfn=0x%lx ofs=%lu order=%u"
// mm_filemap_op_page_cache_range: "dev=%d:%d ino=%llx ofs=%lld-%lld"
// mm_filemap_fault: "dev=%d:%d ino=%llx ofs=%lld"
// filemap_set_wb_err: "dev=%d:%d ino=0x%llx errseq=0x%x"
// file_check_and_advance_wb_err: "file=%p dev=%d:%d ino=0x%llx old=0x%x new=0x%x"

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
