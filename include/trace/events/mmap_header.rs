/* SPDX-License-Identifier: GPL-2.0 */

//! Rust translation of the `mmap` trace-event header.
//!
//! The Linux tracepoint and `define_trace` machinery is supplied by the
//! surrounding kernel translation.  The declarations below preserve the two
//! event payload layouts and their assignment/printing behavior.

/// Payload captured by the `vm_unmapped_area` trace event.
#[repr(C)]
pub struct VmUnmappedAreaEntry {
    pub addr: ::kernel::ffi::c_ulong,
    pub total_vm: ::kernel::ffi::c_ulong,
    pub flags: ::kernel::ffi::c_ulong,
    pub length: ::kernel::ffi::c_ulong,
    pub low_limit: ::kernel::ffi::c_ulong,
    pub high_limit: ::kernel::ffi::c_ulong,
    pub align_mask: ::kernel::ffi::c_ulong,
    pub align_offset: ::kernel::ffi::c_ulong,
}

/// Payload captured by the `exit_mmap` trace event.
#[repr(C)]
pub struct ExitMmapEntry {
    pub mm: *mut crate::mm_struct,
    pub mt: *mut crate::maple_tree,
}

/// Assignment performed by `vm_unmapped_area`'s `TP_fast_assign` block.
///
/// `info` and the current task's memory descriptor are supplied by the kernel
/// tracepoint environment.
#[inline]
pub unsafe fn vm_unmapped_area_fast_assign(
    entry: *mut VmUnmappedAreaEntry,
    addr: ::kernel::ffi::c_ulong,
    info: *const crate::vm_unmapped_area_info,
    current_mm: *const crate::mm_struct,
) {
    (*entry).addr = addr;
    (*entry).total_vm = (*current_mm).total_vm;
    (*entry).flags = (*info).flags;
    (*entry).length = (*info).length;
    (*entry).low_limit = (*info).low_limit;
    (*entry).high_limit = (*info).high_limit;
    (*entry).align_mask = (*info).align_mask;
    (*entry).align_offset = (*info).align_offset;
}

/// Values printed by `vm_unmapped_area`'s `TP_printk` block.
#[inline]
pub unsafe fn vm_unmapped_area_print_values(
    entry: *const VmUnmappedAreaEntry,
) -> (
    ::kernel::ffi::c_ulong,
    ::kernel::ffi::c_long,
    ::kernel::ffi::c_ulong,
    ::kernel::ffi::c_ulong,
    ::kernel::ffi::c_ulong,
    ::kernel::ffi::c_ulong,
    ::kernel::ffi::c_ulong,
    ::kernel::ffi::c_ulong,
    ::kernel::ffi::c_ulong,
) {
    let addr = (*entry).addr;
    // IS_ERR_VALUE(addr) is the kernel's unsigned-long error-pointer test.
    let is_err = addr >= (-(crate::MAX_ERRNO as isize) as ::kernel::ffi::c_ulong);
    (
        if is_err { 0 } else { addr },
        if is_err { addr as ::kernel::ffi::c_long } else { 0 },
        (*entry).total_vm,
        (*entry).flags,
        (*entry).length,
        (*entry).low_limit,
        (*entry).high_limit,
        (*entry).align_mask,
        (*entry).align_offset,
    )
}

/// Assignment performed by `exit_mmap`'s `TP_fast_assign` block.
#[inline]
pub unsafe fn exit_mmap_fast_assign(entry: *mut ExitMmapEntry, mm: *mut crate::mm_struct) {
    (*entry).mm = mm;
    (*entry).mt = &mut (*mm).mm_mt;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
