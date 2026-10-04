// SPDX-License-Identifier: GPL-2.0-only
/*
 * Low level x86 E820 memory map handling functions.
 *
 * The firmware and bootloader passes us the "E820 table", which is the primary
 * physical memory layout description available about x86 systems.
 *
 * The kernel takes the E820 memory layout and optionally modifies it with
 * quirks and other tweaks, and feeds that into the generic Linux memory
 * allocation code routines via a platform independent interface (memblock, etc.).
 */
// Rust production owner translated from the unchanged adjacent e820.c.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    unreachable_pub
)]

#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/x86_e820_generated.rs"
    ));
}
use bindings as b;
use core::cmp::{max, min};
use core::mem::{offset_of, size_of, MaybeUninit};
use core::ptr::{addr_of, addr_of_mut, copy_nonoverlapping, null_mut, write_bytes};
use kernel::ffi::{c_char, c_int, c_ulong, c_void};

#[path = "../../../rust/ffi_export.rs"]
mod ffi_export;

// Firmware is kept unmodified for the firmware checksum; kexec has its own
// independently modifiable map. All three initial backing stores are init-only.
#[link_section = ".init.data"]
static mut e820_table_init: b::e820_table = unsafe { MaybeUninit::zeroed().assume_init() };
#[link_section = ".init.data"]
static mut e820_table_kexec_init: b::e820_table = unsafe { MaybeUninit::zeroed().assume_init() };
#[link_section = ".init.data"]
static mut e820_table_firmware_init: b::e820_table = unsafe { MaybeUninit::zeroed().assume_init() };
#[no_mangle]
#[link_section = ".ref.data"]
pub static mut e820_table: *mut b::e820_table = addr_of_mut!(e820_table_init);
#[no_mangle]
#[link_section = ".ref.data"]
pub static mut e820_table_kexec: *mut b::e820_table = addr_of_mut!(e820_table_kexec_init);
#[no_mangle]
#[link_section = ".ref.data"]
pub static mut e820_table_firmware: *mut b::e820_table = addr_of_mut!(e820_table_firmware_init);
#[no_mangle]
pub static mut pci_mem_start: c_ulong = 0xaeedbabe;
#[cfg(CONFIG_PCI)]
ffi_export::export_symbol!(pci_mem_start, pci_mem_start, "", "");
ffi_export::export_symbol!(e820__mapped_any, e820__mapped_any, "GPL", "");
// The configured EXPORT_SYMBOL_FOR_KVM registration is macro-only C glue.

// Match printk.h when CONFIG_PRINTK is disabled: argument expressions still
// evaluate, while the canonical inline returns zero without an external call.
// PRINTK_INDEX is rejected by the owner rule until callsite metadata is ported.
macro_rules! printk {
    ($($arg:expr),* $(,)?) => {{
        #[cfg(CONFIG_PRINTK)]
        { b::_printk($($arg),*); }
        #[cfg(not(CONFIG_PRINTK))]
        { let _ = ($($arg),*); }
    }};
}

const E820_MAX_ENTRIES: usize = b::RUST_E820_MAX_ENTRIES as usize;
const _: () = assert!(size_of::<b::boot_e820_entry>() == 20);

// Never form a reference to a packed member, or a reference to an entire table:
// after reallocation only nr_entries elements of the trailing array exist.
#[inline(always)]
unsafe fn entry_at(table: *mut b::e820_table, index: u32) -> *mut b::e820_entry {
    addr_of_mut!((*table).entries)
        .cast::<b::e820_entry>()
        .add(index as usize)
}
#[inline(always)]
unsafe fn end_of(entry: *const b::e820_entry) -> u64 {
    (*entry).addr.wrapping_add((*entry).size)
}

unsafe fn _e820__mapped_any(
    table: *mut b::e820_table,
    start: u64,
    end: u64,
    type_: b::e820_type,
) -> bool {
    for idx in 0..(*table).nr_entries {
        let entry = entry_at(table, idx);
        if type_ != 0 && (*entry).type_ != type_ {
            continue;
        }
        if (*entry).addr >= end || end_of(entry) <= start {
            continue;
        }
        return true;
    }
    false
}
#[no_mangle]
pub unsafe extern "C" fn e820__mapped_raw_any(start: u64, end: u64, type_: b::e820_type) -> bool {
    _e820__mapped_any(e820_table_firmware, start, end, type_)
}
#[no_mangle]
pub unsafe extern "C" fn e820__mapped_any(start: u64, end: u64, type_: b::e820_type) -> bool {
    _e820__mapped_any(e820_table, start, end, type_)
}
unsafe fn __e820__mapped_all(mut start: u64, end: u64, type_: b::e820_type) -> *mut b::e820_entry {
    for idx in 0..(*e820_table).nr_entries {
        let entry = entry_at(e820_table, idx);
        if type_ != 0 && (*entry).type_ != type_ {
            continue;
        }
        if (*entry).addr >= end || end_of(entry) <= start {
            continue;
        }
        if (*entry).addr <= start {
            start = end_of(entry);
        }
        if start >= end {
            return entry;
        }
    }
    null_mut()
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__mapped_all(start: u64, end: u64, type_: b::e820_type) -> bool {
    !__e820__mapped_all(start, end, type_).is_null()
}
#[no_mangle]
pub unsafe extern "C" fn e820__get_entry_type(start: u64, end: u64) -> c_int {
    let entry = __e820__mapped_all(start, end, 0);
    if entry.is_null() {
        -(b::EINVAL as c_int)
    } else {
        (*entry).type_ as c_int
    }
}

#[cold]
#[link_section = ".init.text"]
unsafe fn __e820__range_add(table: *mut b::e820_table, start: u64, size: u64, type_: b::e820_type) {
    let idx = (*table).nr_entries;
    if idx as usize >= E820_MAX_ENTRIES {
        printk!(
            c"\x013E820 table full; ignoring [mem %#010llx-%#010llx]\n"
                .as_ptr()
                .cast::<c_char>(),
            start,
            start.wrapping_add(size).wrapping_sub(1)
        );
        return;
    }
    let entry_new = entry_at(table, idx);
    (*entry_new).addr = start;
    (*entry_new).size = size;
    (*entry_new).type_ = type_;
    (*table).nr_entries = idx.wrapping_add(1);
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__range_add(start: u64, size: u64, type_: b::e820_type) {
    __e820__range_add(e820_table, start, size, type_);
}
#[cold]
#[link_section = ".init.text"]
unsafe fn e820_print_type(type_: b::e820_type) {
    match type_ {
        b::E820_TYPE_RAM => {
            printk!(c"\x01c System RAM".as_ptr().cast::<c_char>());
        }
        b::E820_TYPE_RESERVED => {
            printk!(c"\x01c device reserved".as_ptr().cast::<c_char>());
        }
        b::E820_TYPE_SOFT_RESERVED => {
            printk!(c"\x01c soft reserved".as_ptr().cast::<c_char>());
        }
        b::E820_TYPE_ACPI => {
            printk!(c"\x01c ACPI data".as_ptr().cast::<c_char>());
        }
        b::E820_TYPE_NVS => {
            printk!(c"\x01c ACPI NVS".as_ptr().cast::<c_char>());
        }
        b::E820_TYPE_UNUSABLE => {
            printk!(c"\x01c unusable".as_ptr().cast::<c_char>());
        }
        b::E820_TYPE_PMEM | b::E820_TYPE_PRAM => {
            printk!(
                c"\x01c persistent RAM (type %u)".as_ptr().cast::<c_char>(),
                type_
            );
        }
        _ => {
            printk!(c"\x01c type %u".as_ptr().cast::<c_char>(), type_);
        }
    }
}
#[cold]
#[link_section = ".init.text"]
unsafe fn e820__print_table(who: *const c_char) {
    let mut range_end_prev = 0;
    for idx in 0..(*e820_table).nr_entries {
        let entry = entry_at(e820_table, idx);
        let range_start = (*entry).addr;
        let range_end = end_of(entry);
        if range_start < range_end_prev {
            printk!(c"\x016[Firmware Bug]: out of order E820 entry!\n"
                .as_ptr()
                .cast::<c_char>());
        }
        if range_start > range_end_prev {
            printk!(
                c"\x016%s: [gap %#018Lx-%#018Lx]\n"
                    .as_ptr()
                    .cast::<c_char>(),
                who,
                range_end_prev,
                range_start.wrapping_sub(1)
            );
        }
        printk!(
            c"\x016%s: [mem %#018Lx-%#018Lx] ".as_ptr().cast::<c_char>(),
            who,
            range_start,
            range_end.wrapping_sub(1)
        );
        e820_print_type((*entry).type_);
        printk!(c"\x01c\n".as_ptr().cast::<c_char>());
        range_end_prev = range_end;
    }
}

// Private sweep bookkeeping; no replacement ABI layouts for kernel structures.
#[derive(Clone, Copy)]
struct change_member {
    entry: *mut b::e820_entry,
    addr: u64,
}
#[link_section = ".init.data"]
static mut change_point_list: [change_member; 2 * E820_MAX_ENTRIES] = [change_member {
    entry: null_mut(),
    addr: 0,
}; 2 * E820_MAX_ENTRIES];
#[link_section = ".init.data"]
static mut change_point: [*mut change_member; 2 * E820_MAX_ENTRIES] =
    [null_mut(); 2 * E820_MAX_ENTRIES];
#[link_section = ".init.data"]
static mut overlap_list: [*mut b::e820_entry; E820_MAX_ENTRIES] = [null_mut(); E820_MAX_ENTRIES];
#[link_section = ".init.data"]
static mut new_entries: [b::e820_entry; E820_MAX_ENTRIES] =
    unsafe { MaybeUninit::zeroed().assume_init() };

#[cold]
#[link_section = ".init.text"]
unsafe extern "C" fn cpcompare(a: *const c_void, b: *const c_void) -> c_int {
    let ap = *a.cast::<*mut change_member>();
    let bp = *b.cast::<*mut change_member>();
    if (*ap).addr != (*bp).addr {
        return if (*ap).addr > (*bp).addr { 1 } else { -1 };
    }
    ((*ap).addr != (*(*ap).entry).addr) as c_int - ((*bp).addr != (*(*bp).entry).addr) as c_int
}
fn e820_type_mergeable(type_: b::e820_type) -> bool {
    type_ != b::E820_TYPE_PRAM && type_ != b::E820_TYPE_SOFT_RESERVED
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__update_table(table: *mut b::e820_table) -> c_int {
    if (*table).nr_entries < 2 {
        return -1;
    }
    if (*table).nr_entries as usize > E820_MAX_ENTRIES {
        b::rust_e820_bug();
    }
    for idx in 0..(*table).nr_entries {
        let entry = entry_at(table, idx);
        if end_of(entry) < (*entry).addr {
            return -1;
        }
    }
    let points = addr_of_mut!(change_point).cast::<*mut change_member>();
    let members = addr_of_mut!(change_point_list).cast::<change_member>();
    for idx in 0..2 * (*table).nr_entries as usize {
        *points.add(idx) = members.add(idx);
    }
    let mut chg_idx = 0usize;
    for idx in 0..(*table).nr_entries {
        let entry = entry_at(table, idx);
        if (*entry).size != 0 {
            (**points.add(chg_idx)).addr = (*entry).addr;
            (**points.add(chg_idx)).entry = entry;
            chg_idx += 1;
            (**points.add(chg_idx)).addr = end_of(entry);
            (**points.add(chg_idx)).entry = entry;
            chg_idx += 1;
        }
    }
    let chg_nr = chg_idx;
    b::sort(
        points.cast(),
        chg_nr,
        size_of::<*mut change_member>(),
        Some(cpcompare),
        None,
    );
    let overlap = addr_of_mut!(overlap_list).cast::<*mut b::e820_entry>();
    let output = addr_of_mut!(new_entries).cast::<b::e820_entry>();
    let mut overlap_entries = 0usize;
    let mut new_nr_entries = 0usize;
    let mut last_type = 0;
    let mut last_addr = 0u64;
    for chg_idx in 0..chg_nr {
        let point = *points.add(chg_idx);
        if (*point).addr == (*(*point).entry).addr {
            *overlap.add(overlap_entries) = (*point).entry;
            overlap_entries += 1;
        } else {
            // Match the original complete scan and swap-with-last order.
            for idx in 0..overlap_entries {
                if *overlap.add(idx) == (*point).entry {
                    *overlap.add(idx) = *overlap.add(overlap_entries - 1);
                }
            }
            overlap_entries -= 1;
        }
        let mut current_type = 0;
        for idx in 0..overlap_entries {
            if (**overlap.add(idx)).type_ > current_type {
                current_type = (**overlap.add(idx)).type_;
            }
        }
        if current_type != last_type || !e820_type_mergeable(current_type) {
            if last_type != 0 {
                (*output.add(new_nr_entries)).size = (*point).addr.wrapping_sub(last_addr);
                if (*output.add(new_nr_entries)).size != 0 {
                    new_nr_entries += 1;
                    if new_nr_entries >= E820_MAX_ENTRIES {
                        break;
                    }
                }
            }
            if current_type != 0 {
                (*output.add(new_nr_entries)).addr = (*point).addr;
                (*output.add(new_nr_entries)).type_ = current_type;
                last_addr = (*point).addr;
            }
            last_type = current_type;
        }
    }
    copy_nonoverlapping(output, entry_at(table, 0), new_nr_entries);
    (*table).nr_entries = new_nr_entries as u32;
    0
}

#[cold]
#[link_section = ".init.text"]
unsafe fn append_e820_table(mut entry: *mut b::boot_e820_entry, mut nr_entries: u32) -> c_int {
    if nr_entries == 0 {
        return -(b::ENOENT as c_int);
    }
    while nr_entries != 0 {
        let start = (*entry).addr;
        let size = (*entry).size;
        let end = start.wrapping_add(size).wrapping_sub(1);
        if start > end && size != 0 {
            return -(b::EINVAL as c_int);
        }
        e820__range_add(start, size, (*entry).type_);
        entry = entry.add(1);
        nr_entries -= 1;
    }
    0
}
#[cold]
#[link_section = ".init.text"]
unsafe fn __e820__range_update(
    table: *mut b::e820_table,
    start: u64,
    mut size: u64,
    old_type: b::e820_type,
    new_type: b::e820_type,
) -> u64 {
    if old_type == new_type {
        b::rust_e820_bug();
    }
    size = min(size, u64::MAX - start);
    let end = start.wrapping_add(size);
    printk!(
        c"\x017e820: update [mem %#010Lx-%#010Lx]"
            .as_ptr()
            .cast::<c_char>(),
        start,
        end.wrapping_sub(1)
    );
    e820_print_type(old_type);
    printk!(c"\x01c ==>".as_ptr().cast::<c_char>());
    e820_print_type(new_type);
    printk!(c"\x01c\n".as_ptr().cast::<c_char>());
    let mut real_updated_size = 0u64;
    let mut idx = 0;
    // The bound is reread because appending changes nr_entries during the scan.
    while idx < (*table).nr_entries {
        let entry = entry_at(table, idx);
        idx += 1;
        if (*entry).type_ != old_type {
            continue;
        }
        let entry_end = end_of(entry);
        if (*entry).addr >= start && entry_end <= end {
            (*entry).type_ = new_type;
            real_updated_size = real_updated_size.wrapping_add((*entry).size);
            continue;
        }
        if (*entry).addr < start && entry_end > end {
            __e820__range_add(table, start, size, new_type);
            __e820__range_add(table, end, entry_end.wrapping_sub(end), (*entry).type_);
            (*entry).size = start.wrapping_sub((*entry).addr);
            real_updated_size = real_updated_size.wrapping_add(size);
            continue;
        }
        let final_start = max(start, (*entry).addr);
        let final_end = min(end, entry_end);
        if final_start >= final_end {
            continue;
        }
        __e820__range_add(table, final_start, final_end - final_start, new_type);
        real_updated_size = real_updated_size.wrapping_add(final_end - final_start);
        (*entry).size = (*entry).size.wrapping_sub(final_end - final_start);
        if (*entry).addr < final_start {
            continue;
        }
        (*entry).addr = final_end;
    }
    real_updated_size
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__range_update(
    start: u64,
    size: u64,
    old_type: b::e820_type,
    new_type: b::e820_type,
) -> u64 {
    __e820__range_update(e820_table, start, size, old_type, new_type)
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__range_update_table(
    t: *mut b::e820_table,
    start: u64,
    size: u64,
    old_type: b::e820_type,
    new_type: b::e820_type,
) -> u64 {
    __e820__range_update(t, start, size, old_type, new_type)
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__range_remove(start: u64, mut size: u64, filter_type: b::e820_type) {
    size = min(size, u64::MAX - start);
    let end = start.wrapping_add(size);
    printk!(
        c"\x017e820: remove [mem %#010Lx-%#010Lx]"
            .as_ptr()
            .cast::<c_char>(),
        start,
        end.wrapping_sub(1)
    );
    if filter_type != 0 {
        e820_print_type(filter_type);
    }
    printk!(c"\x01c\n".as_ptr().cast::<c_char>());
    let mut idx = 0;
    while idx < (*e820_table).nr_entries {
        let entry = entry_at(e820_table, idx);
        idx += 1;
        if filter_type != 0 && (*entry).type_ != filter_type {
            continue;
        }
        let entry_end = end_of(entry);
        if (*entry).addr >= start && entry_end <= end {
            write_bytes(entry, 0, 1);
            continue;
        }
        if (*entry).addr < start && entry_end > end {
            e820__range_add(end, entry_end - end, (*entry).type_);
            (*entry).size = start - (*entry).addr;
            continue;
        }
        let final_start = max(start, (*entry).addr);
        let final_end = min(end, entry_end);
        if final_start >= final_end {
            continue;
        }
        (*entry).size = (*entry).size.wrapping_sub(final_end - final_start);
        if (*entry).addr < final_start {
            continue;
        }
        (*entry).addr = final_end;
    }
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__update_table_print() {
    if e820__update_table(e820_table) != 0 {
        return;
    }
    printk!(c"\x016modified physical RAM map:\n"
        .as_ptr()
        .cast::<c_char>());
    e820__print_table(c"modified".as_ptr().cast::<c_char>());
}
#[cold]
#[link_section = ".init.text"]
unsafe fn e820__update_table_kexec() {
    e820__update_table(e820_table_kexec);
}

#[cold]
#[link_section = ".init.text"]
unsafe fn e820_search_gap(max_gap_start: *mut c_ulong, max_gap_size: *mut c_ulong) -> c_int {
    let mut range_end_prev = 0u64;
    let mut found = 0;
    // Like C, this routine requires the already populated boot memory map.
    let mut entry = MaybeUninit::<*mut b::e820_entry>::uninit();
    for idx in 0..(*e820_table).nr_entries {
        let current = entry_at(e820_table, idx);
        entry.write(current);
        let range_start = (*current).addr;
        let range_end = end_of(current);
        if range_start > range_end_prev {
            let gap_start = range_end_prev;
            if gap_start < b::RUST_E820_MAX_GAP_END {
                let gap_end = min(range_start, b::RUST_E820_MAX_GAP_END);
                let gap_size = gap_end - gap_start;
                if gap_size >= *max_gap_size as u64 {
                    *max_gap_start = gap_start as c_ulong;
                    *max_gap_size = gap_size as c_ulong;
                    found = 1;
                }
            }
        }
        range_end_prev = range_end;
    }
    let entry = entry.assume_init();
    if end_of(entry) < b::RUST_E820_MAX_GAP_END {
        let gap_start = end_of(entry);
        let gap_size = b::RUST_E820_MAX_GAP_END - gap_start;
        if gap_size >= *max_gap_size as u64 {
            *max_gap_start = gap_start as c_ulong;
            *max_gap_size = gap_size as c_ulong;
            found = 1;
        }
    }
    found
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__setup_pci_gap() {
    let mut max_gap_start = MaybeUninit::<c_ulong>::uninit();
    let mut max_gap_size: c_ulong = b::RUST_E820_SZ_4M as c_ulong;
    let found = e820_search_gap(max_gap_start.as_mut_ptr(), &mut max_gap_size);
    if found == 0 {
        #[cfg(CONFIG_X86_64)]
        {
            max_gap_start.write(
                b::max_pfn
                    .wrapping_shl(b::RUST_E820_PAGE_SHIFT)
                    .wrapping_add(b::RUST_E820_SZ_1M as c_ulong),
            );
            printk!(
                c"\x013Cannot find an available gap in the 32-bit address range\n"
                    .as_ptr()
                    .cast::<c_char>()
            );
            printk!(
                c"\x013PCI devices with unassigned 32-bit BARs may not work!\n"
                    .as_ptr()
                    .cast::<c_char>()
            );
        }
        #[cfg(CONFIG_X86_32)]
        {
            max_gap_start.write(b::RUST_E820_SZ_256M as c_ulong);
        }
    }
    let max_gap_start = max_gap_start.assume_init();
    pci_mem_start = max_gap_start;
    printk!(
        c"\x016[gap %#010lx-%#010lx] available for PCI devices\n"
            .as_ptr()
            .cast::<c_char>(),
        max_gap_start,
        max_gap_start.wrapping_add(max_gap_size).wrapping_sub(1)
    );
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__reallocate_tables() {
    let size = (offset_of!(b::e820_table, entries)
        + size_of::<b::e820_entry>() * (*e820_table).nr_entries as usize) as c_int;
    let n = b::rust_e820_kmemdup_main(e820_table.cast(), size as usize).cast::<b::e820_table>();
    if n.is_null() {
        b::rust_e820_bug();
    }
    e820_table = n;
    let size = (offset_of!(b::e820_table, entries)
        + size_of::<b::e820_entry>() * (*e820_table_kexec).nr_entries as usize)
        as c_int;
    let n =
        b::rust_e820_kmemdup_kexec(e820_table_kexec.cast(), size as usize).cast::<b::e820_table>();
    if n.is_null() {
        b::rust_e820_bug();
    }
    e820_table_kexec = n;
    let size = (offset_of!(b::e820_table, entries)
        + size_of::<b::e820_entry>() * (*e820_table_firmware).nr_entries as usize)
        as c_int;
    let n = b::rust_e820_kmemdup_firmware(e820_table_firmware.cast(), size as usize)
        .cast::<b::e820_table>();
    if n.is_null() {
        b::rust_e820_bug();
    }
    e820_table_firmware = n;
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__memory_setup_extended(phys_addr: u64, data_len: u32) {
    let sdata = b::early_memremap(phys_addr as b::resource_size_t, data_len as usize)
        .cast::<b::setup_data>();
    let entries = ((*sdata).len as usize / size_of::<b::boot_e820_entry>()) as c_int;
    let extmap = addr_of_mut!((*sdata).data).cast::<b::boot_e820_entry>();
    append_e820_table(extmap, entries as u32);
    e820__update_table(e820_table);
    copy_nonoverlapping(e820_table, e820_table_kexec, 1);
    copy_nonoverlapping(e820_table, e820_table_firmware, 1);
    b::early_memunmap(sdata.cast(), data_len as usize);
    printk!(c"\x016extended physical RAM map:\n"
        .as_ptr()
        .cast::<c_char>());
    e820__print_table(c"extended".as_ptr().cast::<c_char>());
}
#[allow(unused_variables, unused_assignments)]
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__register_nosave_regions(limit_pfn: c_ulong) {
    let mut last_addr = 0u64;
    for idx in 0..(*e820_table).nr_entries {
        let entry = entry_at(e820_table, idx);
        if (*entry).type_ != b::E820_TYPE_RAM {
            continue;
        }
        if last_addr < (*entry).addr {
            // include/linux/suspend.h defines an empty inline without hibernation.
            #[cfg(CONFIG_HIBERNATION)]
            b::register_nosave_region(
                (last_addr >> b::RUST_E820_PAGE_SHIFT) as c_ulong,
                ((*entry)
                    .addr
                    .wrapping_add(b::RUST_E820_PAGE_SIZE as u64 - 1)
                    >> b::RUST_E820_PAGE_SHIFT) as c_ulong,
            );
        }
        last_addr = end_of(entry);
    }
    #[cfg(CONFIG_HIBERNATION)]
    b::register_nosave_region((last_addr >> b::RUST_E820_PAGE_SHIFT) as c_ulong, limit_pfn);
}
#[cfg(CONFIG_ACPI)]
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__register_nvs_regions() -> c_int {
    for idx in 0..(*e820_table).nr_entries {
        let entry = entry_at(e820_table, idx);
        if (*entry).type_ == b::E820_TYPE_NVS {
            b::acpi_nvs_register((*entry).addr, (*entry).size);
        }
    }
    0
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__memblock_alloc_reserved(size: u64, align: u64) -> u64 {
    // Exact expansion of include/linux/memblock.h:memblock_phys_alloc().
    let addr = b::memblock_phys_alloc_range(
        size as b::phys_addr_t,
        align as b::phys_addr_t,
        0,
        b::RUST_E820_MEMBLOCK_ALLOC_ACCESSIBLE as b::phys_addr_t,
    ) as u64;
    if addr != 0 {
        e820__range_update_table(
            e820_table_kexec,
            addr,
            size,
            b::E820_TYPE_RAM,
            b::E820_TYPE_RESERVED,
        );
        printk!(
            c"\x016update e820_table_kexec for e820__memblock_alloc_reserved()\n"
                .as_ptr()
                .cast::<c_char>()
        );
        e820__update_table_kexec();
    }
    addr
}
#[inline(always)]
unsafe fn max_arch_pfn() -> c_ulong {
    #[cfg(CONFIG_X86_32)]
    {
        b::RUST_E820_MAX_ARCH_PFN as c_ulong
    }
    #[cfg(CONFIG_X86_64)]
    {
        // Normal kernel pgtable_l5_enabled, not the startup-only global flag.
        if b::rust_e820_cpu_has_la57() {
            b::RUST_E820_MAX_ARCH_PFN_L5 as c_ulong
        } else {
            b::RUST_E820_MAX_ARCH_PFN_L4 as c_ulong
        }
    }
}
#[cold]
#[link_section = ".init.text"]
unsafe fn e820__end_ram_pfn(limit_pfn: c_ulong) -> c_ulong {
    let mut last_pfn = 0;
    let max_arch_pfn = max_arch_pfn();
    for idx in 0..(*e820_table).nr_entries {
        let entry = entry_at(e820_table, idx);
        if (*entry).type_ != b::E820_TYPE_RAM && (*entry).type_ != b::E820_TYPE_ACPI {
            continue;
        }
        let start_pfn = ((*entry).addr >> b::RUST_E820_PAGE_SHIFT) as c_ulong;
        let end_pfn = (end_of(entry) >> b::RUST_E820_PAGE_SHIFT) as c_ulong;
        if start_pfn >= limit_pfn {
            continue;
        }
        if end_pfn > limit_pfn {
            last_pfn = limit_pfn;
            break;
        }
        if end_pfn > last_pfn {
            last_pfn = end_pfn;
        }
    }
    if last_pfn > max_arch_pfn {
        last_pfn = max_arch_pfn;
    }
    printk!(
        c"\x016last_pfn = %#lx max_arch_pfn = %#lx\n"
            .as_ptr()
            .cast::<c_char>(),
        last_pfn,
        max_arch_pfn
    );
    last_pfn
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__end_of_ram_pfn() -> c_ulong {
    e820__end_ram_pfn(max_arch_pfn())
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__end_of_low_ram_pfn() -> c_ulong {
    e820__end_ram_pfn((1 as c_ulong) << (32 - b::RUST_E820_PAGE_SHIFT))
}

#[link_section = ".init.data"]
static mut userdef: c_int = 0;
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn parse_memopt(mut p: *mut c_char) -> c_int {
    if p.is_null() {
        return -(b::EINVAL as c_int);
    }
    if b::strcmp(p, c"nopentium".as_ptr().cast::<c_char>()) == 0 {
        #[cfg(CONFIG_X86_32)]
        {
            b::setup_clear_cpu_cap(b::RUST_E820_X86_FEATURE_PSE);
            return 0;
        }
        #[cfg(CONFIG_X86_64)]
        {
            printk!(c"\x014mem=nopentium ignored! (only supported on x86_32)\n"
                .as_ptr()
                .cast::<c_char>());
            return -(b::EINVAL as c_int);
        }
    }
    userdef = 1;
    let mem_size = b::memparse(p, &mut p);
    if mem_size == 0 {
        return -(b::EINVAL as c_int);
    }
    e820__range_remove(mem_size, u64::MAX - mem_size, b::E820_TYPE_RAM);
    #[cfg(CONFIG_MEMORY_HOTPLUG)]
    {
        b::max_mem_size = mem_size;
    }
    0
}
#[cold]
#[link_section = ".init.text"]
unsafe fn parse_memmap_one(mut p: *mut c_char) -> c_int {
    if p.is_null() {
        return -(b::EINVAL as c_int);
    }
    if b::strncmp(p, c"exactmap".as_ptr().cast::<c_char>(), 8) == 0 {
        (*e820_table).nr_entries = 0;
        userdef = 1;
        return 0;
    }
    let oldp = p;
    let mem_size = b::memparse(p, &mut p);
    if p == oldp {
        return -(b::EINVAL as c_int);
    }
    userdef = 1;
    match *p as u8 {
        b'@' | b'#' | b'$' | b'!' => {
            let marker = *p as u8;
            let start_at = b::memparse(p.add(1), &mut p);
            let type_ = match marker {
                b'@' => b::E820_TYPE_RAM,
                b'#' => b::E820_TYPE_ACPI,
                b'$' => b::E820_TYPE_RESERVED,
                _ => b::E820_TYPE_PRAM,
            };
            e820__range_add(start_at, mem_size, type_);
        }
        b'%' => {
            let mut from: b::e820_type = 0;
            let mut to: b::e820_type = 0;
            let start_at = b::memparse(p.add(1), &mut p);
            if *p as u8 == b'-' {
                from = b::simple_strtoull(p.add(1), &mut p, 0) as b::e820_type;
            }
            if *p as u8 == b'+' {
                to = b::simple_strtoull(p.add(1), &mut p, 0) as b::e820_type;
            }
            if *p != 0 {
                return -(b::EINVAL as c_int);
            }
            if from != 0 && to != 0 {
                e820__range_update(start_at, mem_size, from, to);
            } else if to != 0 {
                e820__range_add(start_at, mem_size, to);
            } else {
                e820__range_remove(start_at, mem_size, from);
            }
        }
        _ => {
            e820__range_remove(mem_size, u64::MAX - mem_size, b::E820_TYPE_RAM);
        }
    }
    if *p == 0 {
        0
    } else {
        -(b::EINVAL as c_int)
    }
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn parse_memmap_opt(mut str_: *mut c_char) -> c_int {
    while !str_.is_null() {
        let mut k = b::strchr(str_, b',' as c_int);
        if !k.is_null() {
            *k = 0;
            k = k.add(1);
        }
        // The original intentionally ignores errors from each component.
        parse_memmap_one(str_);
        str_ = k;
    }
    0
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__finish_early_params() {
    if userdef != 0 {
        if e820__update_table(e820_table) < 0 {
            b::panic(
                c"Invalid user supplied memory map"
                    .as_ptr()
                    .cast::<c_char>(),
            );
        }
        printk!(c"\x016user-defined physical RAM map:\n"
            .as_ptr()
            .cast::<c_char>());
        e820__print_table(c"user".as_ptr().cast::<c_char>());
    }
}

// Canonical __setup_param layout, section, string alignment and early flag.
#[used]
#[link_section = ".init.rodata"]
static __setup_str_parse_memopt: [u8; 4] = *b"mem\0";
#[used]
#[link_section = ".init.setup"]
static mut __setup_parse_memopt: b::obs_kernel_param = b::obs_kernel_param {
    str_: addr_of!(__setup_str_parse_memopt).cast(),
    setup_func: Some(parse_memopt),
    early: 1,
};
#[used]
#[link_section = ".init.rodata"]
static __setup_str_parse_memmap_opt: [u8; 7] = *b"memmap\0";
#[used]
#[link_section = ".init.setup"]
static mut __setup_parse_memmap_opt: b::obs_kernel_param = b::obs_kernel_param {
    str_: addr_of!(__setup_str_parse_memmap_opt).cast(),
    setup_func: Some(parse_memmap_opt),
    early: 1,
};

#[cold]
#[link_section = ".init.text"]
unsafe fn e820_type_to_string(entry: *mut b::e820_entry) -> *const c_char {
    match (*entry).type_ {
        b::E820_TYPE_RAM => c"System RAM".as_ptr().cast::<c_char>(),
        b::E820_TYPE_ACPI => c"ACPI Tables".as_ptr().cast::<c_char>(),
        b::E820_TYPE_NVS => c"ACPI Non-volatile Storage".as_ptr().cast::<c_char>(),
        b::E820_TYPE_UNUSABLE => c"Unusable memory".as_ptr().cast::<c_char>(),
        b::E820_TYPE_PRAM => c"Persistent Memory (legacy)".as_ptr().cast::<c_char>(),
        b::E820_TYPE_PMEM => c"Persistent Memory".as_ptr().cast::<c_char>(),
        b::E820_TYPE_RESERVED => c"Reserved".as_ptr().cast::<c_char>(),
        b::E820_TYPE_SOFT_RESERVED => c"Soft Reserved".as_ptr().cast::<c_char>(),
        _ => c"Unknown E820 type".as_ptr().cast::<c_char>(),
    }
}
#[cold]
#[link_section = ".init.text"]
unsafe fn e820_type_to_iomem_type(entry: *mut b::e820_entry) -> c_ulong {
    if (*entry).type_ == b::E820_TYPE_RAM {
        b::RUST_E820_IORESOURCE_SYSTEM_RAM as c_ulong
    } else {
        b::RUST_E820_IORESOURCE_MEM as c_ulong
    }
}
#[cold]
#[link_section = ".init.text"]
unsafe fn e820_type_to_iores_desc(entry: *mut b::e820_entry) -> c_ulong {
    (match (*entry).type_ {
        b::E820_TYPE_ACPI => b::IORES_DESC_ACPI_TABLES,
        b::E820_TYPE_NVS => b::IORES_DESC_ACPI_NV_STORAGE,
        b::E820_TYPE_PMEM => b::IORES_DESC_PERSISTENT_MEMORY,
        b::E820_TYPE_PRAM => b::IORES_DESC_PERSISTENT_MEMORY_LEGACY,
        b::E820_TYPE_RESERVED => b::IORES_DESC_RESERVED,
        b::E820_TYPE_SOFT_RESERVED => b::IORES_DESC_SOFT_RESERVED,
        _ => b::IORES_DESC_NONE,
    }) as c_ulong
}
#[link_section = ".init.data"]
static mut e820_res: *mut b::resource = null_mut();
#[cold]
#[link_section = ".init.text"]
unsafe fn e820_device_region(type_: b::e820_type, res: *mut b::resource) -> bool {
    if (*res).start < b::RUST_E820_SZ_1M as b::resource_size_t {
        return false;
    }
    matches!(
        type_,
        b::E820_TYPE_RESERVED | b::E820_TYPE_SOFT_RESERVED | b::E820_TYPE_PRAM | b::E820_TYPE_PMEM
    )
}
#[allow(unused_variables)]
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__reserve_resources() {
    // Expand memblock_alloc_or_panic while preserving its original __func__.
    let mut res = b::__memblock_alloc_or_panic(
        (size_of::<b::resource>() * (*e820_table).nr_entries as usize) as b::phys_addr_t,
        b::RUST_E820_SMP_CACHE_BYTES as b::phys_addr_t,
        c"e820__reserve_resources".as_ptr().cast::<c_char>(),
    )
    .cast::<b::resource>();
    e820_res = res;
    for idx in 0..(*e820_table).nr_entries {
        let entry = entry_at(e820_table, idx);
        let end = end_of(entry).wrapping_sub(1);
        if end != end as b::resource_size_t as u64 {
            res = res.add(1);
            continue;
        }
        (*res).start = (*entry).addr as b::resource_size_t;
        (*res).end = end as b::resource_size_t;
        (*res).name = e820_type_to_string(entry);
        (*res).flags = e820_type_to_iomem_type(entry);
        (*res).desc = e820_type_to_iores_desc(entry);
        if !e820_device_region((*entry).type_, res) {
            (*res).flags |= b::RUST_E820_IORESOURCE_BUSY as c_ulong;
            b::insert_resource(addr_of_mut!(b::iomem_resource), res);
        }
        res = res.add(1);
    }
    for idx in 0..(*e820_table_kexec).nr_entries {
        let entry = entry_at(e820_table_kexec, idx);
        // Canonical firmware-map.h is an empty inline when this is disabled.
        #[cfg(CONFIG_FIRMWARE_MEMMAP)]
        b::firmware_map_add_early((*entry).addr, end_of(entry), e820_type_to_string(entry));
    }
}
#[cold]
#[link_section = ".init.text"]
fn ram_alignment(pos: b::resource_size_t) -> c_ulong {
    let mb = (pos >> 20) as c_ulong;
    if mb == 0 {
        return 64 * 1024;
    }
    if mb < 16 {
        return 1024 * 1024;
    }
    64 * 1024 * 1024
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__reserve_resources_late() {
    for idx in 0..(*e820_table).nr_entries {
        let res = e820_res.add(idx as usize);
        if !(*res).parent.is_null() || (*res).end == 0 {
            continue;
        }
        if (*res).desc == b::IORES_DESC_SOFT_RESERVED as c_ulong {
            b::insert_resource_expand_to_fit(addr_of_mut!(b::soft_reserve_resource), res);
        } else {
            b::insert_resource_expand_to_fit(addr_of_mut!(b::iomem_resource), res);
        }
    }
    for idx in 0..(*e820_table).nr_entries {
        let entry = entry_at(e820_table, idx);
        if (*entry).type_ != b::E820_TYPE_RAM {
            continue;
        }
        let start = end_of(entry);
        let mask = ram_alignment(start as b::resource_size_t).wrapping_sub(1) as u64;
        // round_up(start, alignment) - 1, including native unsigned wrap.
        let mut end = ((start.wrapping_sub(1) | mask).wrapping_add(1)).wrapping_sub(1);
        end = min(end, b::resource_size_t::MAX as u64);
        if start >= end {
            continue;
        }
        printk!(
            c"\x016e820: register RAM buffer resource [mem %#010llx-%#010llx]\n"
                .as_ptr()
                .cast::<c_char>(),
            start,
            end
        );
        b::reserve_region_with_split(
            addr_of_mut!(b::iomem_resource),
            start as b::resource_size_t,
            end as b::resource_size_t,
            c"RAM buffer".as_ptr().cast::<c_char>(),
        );
    }
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__memory_setup_default() -> *mut c_char {
    let mut who = c"BIOS-e820".as_ptr().cast::<c_char>().cast_mut();
    let boot = addr_of_mut!(b::boot_params);
    if append_e820_table(
        addr_of_mut!((*boot).e820_table).cast(),
        (*boot).e820_entries as u32,
    ) < 0
    {
        let mem_size;
        // Preserve the original e801 preference when the two values tie.
        if (*boot).alt_mem_k < (*boot).screen_info.ext_mem_k as u32 {
            mem_size = (*boot).screen_info.ext_mem_k as u64;
            who = c"BIOS-88".as_ptr().cast::<c_char>().cast_mut();
        } else {
            mem_size = (*boot).alt_mem_k as u64;
            who = c"BIOS-e801".as_ptr().cast::<c_char>().cast_mut();
        }
        (*e820_table).nr_entries = 0;
        e820__range_add(0, b::RUST_E820_LOWMEMSIZE, b::E820_TYPE_RAM);
        e820__range_add(b::RUST_E820_HIGH_MEMORY, mem_size << 10, b::E820_TYPE_RAM);
    }
    e820__update_table(e820_table);
    who
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__memory_setup() {
    // BUILD_BUG_ON boot ABI size is the compile-time assertion above.
    let who = b::x86_init.resources.memory_setup.unwrap_unchecked()();
    copy_nonoverlapping(e820_table, e820_table_kexec, 1);
    copy_nonoverlapping(e820_table, e820_table_firmware, 1);
    printk!(c"\x016BIOS-provided physical RAM map:\n"
        .as_ptr()
        .cast::<c_char>());
    e820__print_table(who);
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn e820__memblock_setup() {
    // Exact header bodies of movable_node_is_enabled/memblock_set_bottom_up.
    #[cfg(CONFIG_MEMORY_HOTPLUG)]
    if b::movable_node_enabled {
        b::memblock.bottom_up = true;
    }
    b::memblock_set_current_limit(b::RUST_E820_ISA_END_ADDRESS as b::phys_addr_t);
    b::memblock_allow_resize();
    for idx in 0..(*e820_table).nr_entries {
        let entry = entry_at(e820_table, idx);
        let end = end_of(entry);
        if end != end as b::resource_size_t as u64 {
            continue;
        }
        if (*entry).type_ == b::E820_TYPE_SOFT_RESERVED {
            // Exact header body of memblock_reserve(), not the KERN variant.
            b::__memblock_reserve(
                (*entry).addr as b::phys_addr_t,
                (*entry).size as b::phys_addr_t,
                b::RUST_E820_NUMA_NO_NODE,
                0,
            );
        }
        if (*entry).type_ != b::E820_TYPE_RAM {
            continue;
        }
        b::memblock_add(
            (*entry).addr as b::phys_addr_t,
            (*entry).size as b::phys_addr_t,
        );
    }
    b::memblock_mark_kho_scratch(0, b::RUST_E820_SZ_1M as b::phys_addr_t);
    #[cfg(CONFIG_X86_32)]
    b::memblock_remove(
        (b::max_pfn as b::phys_addr_t).wrapping_shl(b::RUST_E820_PAGE_SHIFT),
        b::phys_addr_t::MAX,
    );
    b::memblock_trim_memory(b::RUST_E820_PAGE_SIZE as b::phys_addr_t);
    b::memblock_dump_all();
}
