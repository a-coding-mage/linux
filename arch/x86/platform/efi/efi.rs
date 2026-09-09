// SPDX-License-Identifier: GPL-2.0
/*
 * Common EFI (Extensible Firmware Interface) support functions
 * Based on Extensible Firmware Interface Specification version 1.0
 *
 * Copyright (C) 1999 VA Linux Systems
 * Copyright (C) 1999 Walt Drummond <drummond@valinux.com>
 * Copyright (C) 1999-2002 Hewlett-Packard Co.
 *	David Mosberger-Tang <davidm@hpl.hp.com>
 *	Stephane Eranian <eranian@hpl.hp.com>
 * Copyright (C) 2005-2008 Intel Co.
 *	Fenghua Yu <fenghua.yu@intel.com>
 *	Bibo Mao <bibo.mao@intel.com>
 *	Chandramouli Narayanan <mouli@linux.intel.com>
 *	Huang Ying <ying.huang@intel.com>
 * Copyright (C) 2013 SuSE Labs
 *	Borislav Petkov <bp@suse.de> - runtime services VA mapping
 *
 * Copied from efi_32.c to eliminate the duplicated code between EFI
 * 32/64 support code. --ying 2007-10-26
 *
 * All EFI Runtime Services are not implemented yet as EFI only
 * supports physical mode addressing on SoftSDV. This is to be fixed
 * in a future version.  --drummond 1999-07-20
 *
 * Implemented EFI runtime services and virtual mode calls.  --davidm
 *
 * Goutham Rao: <goutham.rao@intel.com>
 *	Skip non-WB memory and ignore empty memory ranges.
 */

// C headers and kernel headers are supplied by other translation units.

static mut EFI_SYSTAB_PHYS: c_ulong = 0;
static mut EFI_RUNTIME: c_ulong = 0;
static mut EFI_NR_TABLES: c_ulong = 0;

pub static mut efi_fw_vendor: c_ulong = 0;
pub static mut efi_config_table: c_ulong = 0;

static mut add_efi_memmap: c_int = 0;

unsafe extern "C" fn setup_add_efi_memmap(_arg: *mut c_char) -> c_int {
    add_efi_memmap = 1;
    0
}

// early_param("add_efi_memmap", setup_add_efi_memmap);

unsafe fn do_add_efi_memmap() {
    if !efi_enabled(EFI_MEMMAP) { return; }
    let mut md: *mut efi_memory_desc_t;
    for_each_efi_memory_desc!(md) {
        let start = (*md).phys_addr;
        let size = (*md).num_pages << EFI_PAGE_SHIFT;
        let e820_type: c_int;
        match (*md).type_ {
            EFI_LOADER_CODE | EFI_LOADER_DATA | EFI_BOOT_SERVICES_CODE |
            EFI_BOOT_SERVICES_DATA | EFI_CONVENTIONAL_MEMORY => {
                if efi_soft_reserve_enabled() && ((*md).attribute & EFI_MEMORY_SP) != 0 {
                    e820_type = E820_TYPE_SOFT_RESERVED;
                } else if ((*md).attribute & EFI_MEMORY_WB) != 0 {
                    e820_type = E820_TYPE_RAM;
                } else { e820_type = E820_TYPE_RESERVED; }
            }
            EFI_ACPI_RECLAIM_MEMORY => e820_type = E820_TYPE_ACPI,
            EFI_ACPI_MEMORY_NVS => e820_type = E820_TYPE_NVS,
            EFI_UNUSABLE_MEMORY => e820_type = E820_TYPE_UNUSABLE,
            EFI_PERSISTENT_MEMORY => e820_type = E820_TYPE_PMEM,
            _ => e820_type = E820_TYPE_RESERVED,
        }
        e820__range_add(start, size, e820_type);
    }
    e820__update_table(efi_table_ptr());
}

unsafe fn do_efi_soft_reserve() -> bool {
    if !efi_enabled(EFI_MEMMAP) || !efi_soft_reserve_enabled() { return false; }
    let mut md: *mut efi_memory_desc_t;
    for_each_efi_memory_desc!(md) {
        if (*md).type_ == EFI_CONVENTIONAL_MEMORY && ((*md).attribute & EFI_MEMORY_SP) != 0 { return true; }
    }
    false
}

pub unsafe fn efi_memblock_x86_reserve_range() -> c_int {
    let e = &mut boot_params.efi_info;
    if efi_enabled(EFI_PARAVIRT) { return 0; }
    if IS_ENABLED!(CONFIG_X86_32) && e.efi_memmap_hi > 0 {
        pr_err!("Memory map is above 4GB, disabling EFI.\n");
        return -EINVAL;
    }
    let pmap: phys_addr_t = (e.efi_memmap as u64) | ((e.efi_memmap_hi as u64) << 32);
    let mut data = efi_memory_map_data { phys_map: pmap, size: e.efi_memmap_size,
        desc_size: e.efi_memdesc_size, desc_version: e.efi_memdesc_version, flags: 0 };
    let rv = efi_memmap_init_early(&mut data);
    if rv != 0 { return rv; }
    if add_efi_memmap != 0 || do_efi_soft_reserve() { do_add_efi_memmap(); }
    WARN!(efi.memmap.desc_version != 1, "Unexpected EFI_MEMORY_DESCRIPTOR version %ld", efi.memmap.desc_version);
    memblock_reserve(pmap, efi.memmap.nr_map * efi.memmap.desc_size);
    set_bit(EFI_PRESERVE_BS_REGIONS, &mut efi.flags);
    0
}

const OVERFLOW_ADDR_SHIFT: u32 = 64 - EFI_PAGE_SHIFT;
const OVERFLOW_ADDR_MASK: u64 = u64::MAX << OVERFLOW_ADDR_SHIFT;
const U64_HIGH_BIT: u64 = !(u64::MAX >> 1);

unsafe fn efi_memmap_entry_valid(md: *const efi_memory_desc_t, i: c_int) -> bool {
    let mut end = ((*md).num_pages << EFI_PAGE_SHIFT).wrapping_add((*md).phys_addr).wrapping_sub(1);
    let mut end_hi = 0u64;
    let mut buf = [0i8; 64];
    if (*md).num_pages == 0 { end = 0; }
    else if (*md).num_pages > EFI_PAGES_MAX || EFI_PAGES_MAX - (*md).num_pages < ((*md).phys_addr >> EFI_PAGE_SHIFT) {
        end_hi = (((*md).num_pages & OVERFLOW_ADDR_MASK) >> OVERFLOW_ADDR_SHIFT);
        if ((*md).phys_addr & U64_HIGH_BIT) != 0 && (end & U64_HIGH_BIT) == 0 { end_hi += 1; }
    } else { return true; }
    pr_warn_once!(FW_BUG "Invalid EFI memory map entries:\n");
    if end_hi != 0 { pr_warn!("mem%02u: %s range=[0x%016llx-0x%llx%016llx] (invalid)\n", i, efi_md_typeattr_format(buf.as_mut_ptr(), buf.len(), md), (*md).phys_addr, end_hi, end); }
    else { pr_warn!("mem%02u: %s range=[0x%016llx-0x%016llx] (invalid)\n", i, efi_md_typeattr_format(buf.as_mut_ptr(), buf.len(), md), (*md).phys_addr, end); }
    false
}

unsafe fn efi_clean_memmap() {
    let mut out = efi.memmap.map;
    let mut input = out;
    let end = efi.memmap.map_end;
    let mut i = 0;
    let mut n_removal = 0;
    while input < end {
        if efi_memmap_entry_valid(input, i) {
            if out != input { memcpy(out as *mut c_void, input as *const c_void, efi.memmap.desc_size); }
            out = (out as *mut u8).add(efi.memmap.desc_size) as *mut efi_memory_desc_t;
        } else { n_removal += 1; }
        input = (input as *mut u8).add(efi.memmap.desc_size) as *const efi_memory_desc_t;
        i += 1;
    }
    if n_removal > 0 {
        let data = efi_memory_map_data { phys_map: efi.memmap.phys_map, desc_version: efi.memmap.desc_version,
            desc_size: efi.memmap.desc_size, size: efi.memmap.desc_size * (efi.memmap.nr_map - n_removal), flags: 0 };
        pr_warn!("Removing %d invalid memory map entries.\n", n_removal);
        efi_memmap_install(&data);
    }
}

unsafe fn efi_remove_e820_mmio() {
    let mut md: *mut efi_memory_desc_t; let mut i = 0;
    for_each_efi_memory_desc!(md) {
        if (*md).type_ == EFI_MEMORY_MAPPED_IO {
            let size = (*md).num_pages << EFI_PAGE_SHIFT; let start = (*md).phys_addr; let end = start + size - 1;
            if size >= 256 * 1024 { pr_info!("Remove mem%02u: MMIO range=[0x%08llx-0x%08llx] (%lluMB) from e820 map\n", i, start, end, size >> 20); e820__range_remove(start, size, E820_TYPE_RESERVED); }
            else { pr_info!("Not removing mem%02u: MMIO range=[0x%08llx-0x%08llx] (%lluKB) from e820 map\n", i, start, end, size >> 10); }
        } i += 1;
    }
}

pub unsafe fn efi_print_memmap() {
    let mut md: *mut efi_memory_desc_t; let mut i = 0;
    for_each_efi_memory_desc!(md) {
        let mut buf = [0i8; 64];
        pr_info!("mem%02u: %s range=[0x%016llx-0x%016llx] (%lluMB)\n", i, efi_md_typeattr_format(buf.as_mut_ptr(), buf.len(), md), (*md).phys_addr, (*md).phys_addr + ((*md).num_pages << EFI_PAGE_SHIFT) - 1, (*md).num_pages >> (20 - EFI_PAGE_SHIFT)); i += 1;
    }
}

// The remaining EFI setup and runtime mapping routines retain their C ABI and
// kernel-provided types; declarations below preserve their source interfaces.
pub unsafe fn efi_init() {
    if IS_ENABLED!(CONFIG_X86_32) && (boot_params.efi_info.efi_systab_hi != 0 || boot_params.efi_info.efi_memmap_hi != 0) { pr_info!("Table located above 4GB, disabling EFI.\n"); return; }
    EFI_SYSTAB_PHYS = boot_params.efi_info.efi_systab as c_ulong | ((boot_params.efi_info.efi_systab_hi as u64) << 32) as c_ulong;
    if efi_systab_init(EFI_SYSTAB_PHYS) != 0 { return; }
    if efi_reuse_config(efi_config_table, EFI_NR_TABLES) != 0 { return; }
    if efi_config_init(arch_tables()) != 0 { return; }
    if !efi_runtime_supported() { pr_err!("No EFI runtime due to 32/64-bit mismatch with kernel\n"); }
    if !efi_runtime_supported() || efi_runtime_disabled() { efi_memmap_unmap(); return; }
    set_bit(EFI_RUNTIME_SERVICES, &mut efi.flags); efi_clean_memmap(); efi_remove_e820_mmio();
    if efi_enabled(EFI_DBG) { efi_print_memmap(); }
}

pub unsafe fn efi_enter_virtual_mode() {
    if efi_enabled(EFI_PARAVIRT) { return; }
    efi.runtime = efi_runtime as *mut efi_runtime_services_t;
    if efi_setup != 0 { kexec_enter_virtual_mode(); } else { __efi_enter_virtual_mode(); }
    efi_dump_pagetable();
}

unsafe fn efi_merge_regions() {
    let mut md: *mut efi_memory_desc_t; let mut prev: *mut efi_memory_desc_t = core::ptr::null_mut();
    for_each_efi_memory_desc!(md) {
        if prev.is_null() { prev = md; continue; }
        if (*prev).type_ != (*md).type_ || (*prev).attribute != (*md).attribute { prev = md; continue; }
        let prev_size = (*prev).num_pages << EFI_PAGE_SHIFT;
        if (*md).phys_addr == (*prev).phys_addr + prev_size { (*prev).num_pages += (*md).num_pages; (*md).type_ = EFI_RESERVED_TYPE; (*md).attribute = 0; continue; }
        prev = md;
    }
}

unsafe fn realloc_pages(old_memmap: *mut c_void, old_shift: c_int) -> *mut c_void {
    let ret = __get_free_pages(GFP_KERNEL, old_shift + 1) as *mut c_void;
    if ret.is_null() { free_pages(old_memmap as c_ulong, old_shift); return ret; }
    if !old_memmap.is_null() { memcpy(ret, old_memmap, PAGE_SIZE << old_shift); }
    free_pages(old_memmap as c_ulong, old_shift); ret
}

unsafe fn efi_map_next_entry_reverse(entry: *mut c_void) -> *mut c_void {
    if entry.is_null() { return (efi.memmap.map_end as *mut u8).sub(efi.memmap.desc_size) as *mut c_void; }
    let next = (entry as *mut u8).sub(efi.memmap.desc_size) as *mut c_void;
    if next < efi.memmap.map as *mut c_void { core::ptr::null_mut() } else { next }
}

unsafe fn efi_map_next_entry(entry: *mut c_void) -> *mut c_void {
    if efi_enabled(EFI_64BIT) { return efi_map_next_entry_reverse(entry); }
    if entry.is_null() { return efi.memmap.map as *mut c_void; }
    let next = (entry as *mut u8).add(efi.memmap.desc_size) as *mut c_void;
    if next >= efi.memmap.map_end as *mut c_void { core::ptr::null_mut() } else { next }
}

unsafe fn should_map_region(md: *mut efi_memory_desc_t) -> bool {
    if (*md).attribute & EFI_MEMORY_RUNTIME != 0 { return true; }
    if IS_ENABLED!(CONFIG_X86_32) { return false; }
    if (*md).type_ == EFI_CONVENTIONAL_MEMORY && efi_soft_reserve_enabled() && (*md).attribute & EFI_MEMORY_SP != 0 { return false; }
    if efi_is_mixed() && ((*md).type_ == EFI_CONVENTIONAL_MEMORY || (*md).type_ == EFI_LOADER_DATA || (*md).type_ == EFI_LOADER_CODE) { return true; }
    (*md).type_ == EFI_BOOT_SERVICES_CODE || (*md).type_ == EFI_BOOT_SERVICES_DATA
}

unsafe fn efi_map_regions(count: &mut c_int, pg_shift: &mut c_int) -> *mut c_void {
    let mut p: *mut c_void = core::ptr::null_mut(); let mut new_memmap: *mut c_void = core::ptr::null_mut(); let mut left = 0usize; let desc_size = efi.memmap.desc_size; let mut md;
    while { p = efi_map_next_entry(p); !p.is_null() } { md = p as *mut efi_memory_desc_t; if !should_map_region(md) { continue; } efi_map_region(md); if left < desc_size { new_memmap = realloc_pages(new_memmap, *pg_shift); if new_memmap.is_null() { return core::ptr::null_mut(); } left += PAGE_SIZE << *pg_shift; *pg_shift += 1; } memcpy((new_memmap as *mut u8).add((*count as usize) * desc_size) as *mut c_void, md as *const c_void, desc_size); left -= desc_size; *count += 1; }
    new_memmap
}

unsafe fn __efi_enter_virtual_mode() {
    let mut count = 0; let mut pg_shift = 0; if efi_alloc_page_tables() != 0 { clear_bit(EFI_RUNTIME_SERVICES, &mut efi.flags); return; }
    efi_merge_regions(); let new_memmap = efi_map_regions(&mut count, &mut pg_shift); if new_memmap.is_null() { clear_bit(EFI_RUNTIME_SERVICES, &mut efi.flags); return; }
    let pa = __pa(new_memmap); efi_memmap_unmap(); if efi_memmap_init_late(pa, efi.memmap.desc_size * count) != 0 { clear_bit(EFI_RUNTIME_SERVICES, &mut efi.flags); return; }
    if efi_setup_page_tables(pa, 1 << pg_shift) != 0 { clear_bit(EFI_RUNTIME_SERVICES, &mut efi.flags); return; }
    efi_sync_low_kernel_mappings(); let status = efi_set_virtual_address_map(efi.memmap.desc_size * count, efi.memmap.desc_size, efi.memmap.desc_version, pa as *mut efi_memory_desc_t, EFI_SYSTAB_PHYS);
    if status != EFI_SUCCESS { clear_bit(EFI_RUNTIME_SERVICES, &mut efi.flags); return; }
    efi_check_for_embedded_firmwares(); efi_unmap_boot_services(); if !efi_is_mixed() { efi_native_runtime_setup(); } else { efi_thunk_runtime_setup(); } efi_runtime_update_mappings(); efi_delete_dummy_variable();
}

pub unsafe fn efi_is_table_address(phys_addr: c_ulong) -> bool {
    if phys_addr == EFI_INVALID_TABLE_ADDR { return false; }
    for p in efi_tables() { if *p == phys_addr { return true; } }
    false
}

pub unsafe fn efi_attr_is_visible(_kobj: *mut kobject, attr: *mut attribute, _n: c_int) -> umode_t {
    if attr == &mut efi_attr_fw_vendor.attr && (efi_enabled(EFI_PARAVIRT) || efi_fw_vendor == EFI_INVALID_TABLE_ADDR) { return 0; }
    if attr == &mut efi_attr_runtime.attr && efi_runtime == EFI_INVALID_TABLE_ADDR { return 0; }
    if attr == &mut efi_attr_config_table.attr && efi_config_table == EFI_INVALID_TABLE_ADDR { return 0; }
    (*attr).mode
}

pub unsafe fn __x86_efi_boot_mode() -> efi_secureboot_mode { boot_params.secure_boot }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
