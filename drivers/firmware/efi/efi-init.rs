// SPDX-License-Identifier: GPL-2.0
/*
 * Extensible Firmware Interface
 *
 * Based on Extensible Firmware Interface Specification version 2.4
 *
 * Copyright (C) 2013 - 2015 Linaro Ltd.
 */

// C includes and build-time configuration are supplied by the surrounding kernel.

pub static mut primary_display_table: ::core::ffi::c_ulong = EFI_INVALID_TABLE_ADDR;

unsafe fn is_memory(md: *mut efi_memory_desc_t) -> ::core::ffi::c_int {
    if (*md).attribute & (EFI_MEMORY_WB | EFI_MEMORY_WT | EFI_MEMORY_WC) != 0 {
        return 1;
    }
    0
}

/*
 * Translate a EFI virtual address into a physical address: this is necessary,
 * as some data members of the EFI system table are virtually remapped after
 * SetVirtualAddressMap() has been called.
 */
unsafe fn efi_to_phys(addr: ::core::ffi::c_ulong) -> phys_addr_t {
    let mut md: *mut efi_memory_desc_t;
    // for_each_efi_memory_desc(md)
    for_each_efi_memory_desc!(md, {
        if (*md).attribute & EFI_MEMORY_RUNTIME == 0 {
            continue;
        }
        if (*md).virt_addr == 0 {
            break;
        }
        if (*md).virt_addr <= addr
            && (addr - (*md).virt_addr) < ((*md).num_pages << EFI_PAGE_SHIFT)
        {
            return (*md).phys_addr + addr - (*md).virt_addr;
        }
    });
    addr
}

extern "C" {
    pub static efi_arch_tables: efi_config_table_type_t;
}

// x86 defines its own instance of sysfb_primary_display.
#[cfg(any(feature = "CONFIG_SYSFB", feature = "CONFIG_EFI_EARLYCON", feature = "CONFIG_FIRMWARE_EDID"))]
#[cfg(not(feature = "CONFIG_X86"))]
#[no_mangle]
pub static mut sysfb_primary_display: sysfb_display_info = unsafe { ::core::mem::zeroed() };

unsafe fn init_primary_display() {
    let mut dpy: *mut sysfb_display_info;

    if primary_display_table != EFI_INVALID_TABLE_ADDR {
        dpy = early_memremap(primary_display_table, ::core::mem::size_of::<sysfb_display_info>());
        if dpy.is_null() {
            pr_err!("Could not map primary_display config table\n");
            return;
        }
        sysfb_primary_display = *dpy;
        ::core::ptr::write_bytes(dpy, 0, 1);
        early_memunmap(dpy, ::core::mem::size_of::<sysfb_display_info>());

        if memblock_is_map_memory(sysfb_primary_display.screen.lfb_base) {
            memblock_mark_nomap(
                sysfb_primary_display.screen.lfb_base,
                sysfb_primary_display.screen.lfb_size,
            );
        }

        if cfg!(feature = "CONFIG_EFI_EARLYCON") {
            efi_earlycon_reprobe();
        }
    }
}

unsafe fn uefi_init(efi_system_table: u64) -> ::core::ffi::c_int {
    let mut config_tables: *mut efi_config_table_t;
    let mut systab: *mut efi_system_table_t;
    let table_size: usize;
    let mut retval: ::core::ffi::c_int;

    systab = early_memremap_ro(efi_system_table, ::core::mem::size_of::<efi_system_table_t>());
    if systab.is_null() {
        pr_warn!("Unable to map EFI system table.\n");
        return -ENOMEM;
    }

    set_bit(EFI_BOOT, &mut efi.flags);
    if cfg!(target_pointer_width = "64") {
        set_bit(EFI_64BIT, &mut efi.flags);
    }

    retval = efi_systab_check_header(&(*systab).hdr);
    if retval != 0 {
        early_memunmap(systab, ::core::mem::size_of::<efi_system_table_t>());
        return retval;
    }

    efi.runtime = (*systab).runtime;
    efi.runtime_version = (*systab).hdr.revision;
    efi_systab_report_header(&(*systab).hdr, efi_to_phys((*systab).fw_vendor));

    table_size = ::core::mem::size_of::<efi_config_table_t>() * (*systab).nr_tables;
    config_tables = early_memremap_ro(efi_to_phys((*systab).tables), table_size);
    if config_tables.is_null() {
        pr_warn!("Unable to map EFI config table array.\n");
        retval = -ENOMEM;
        early_memunmap(systab, ::core::mem::size_of::<efi_system_table_t>());
        return retval;
    }
    retval = efi_config_parse_tables(config_tables, (*systab).nr_tables, &efi_arch_tables);
    early_memunmap(config_tables, table_size);
    early_memunmap(systab, ::core::mem::size_of::<efi_system_table_t>());
    retval
}

unsafe fn is_usable_memory(md: *mut efi_memory_desc_t) -> bool {
    match (*md).type_ {
        EFI_LOADER_CODE | EFI_LOADER_DATA | EFI_ACPI_RECLAIM_MEMORY |
        EFI_BOOT_SERVICES_CODE | EFI_BOOT_SERVICES_DATA |
        EFI_CONVENTIONAL_MEMORY | EFI_PERSISTENT_MEMORY =>
            (*md).attribute & EFI_MEMORY_WB != 0,
        _ => false,
    }
}

unsafe fn reserve_regions() {
    let mut md: *mut efi_memory_desc_t;
    let mut paddr: u64;
    let mut npages: u64;
    let mut size: u64;

    if efi_enabled(EFI_DBG) != 0 { pr_info!("Processing EFI memory map:\n"); }
    memblock_dump_all();

    if is_kho_boot() {
        let mut r: *mut memblock_region;
        for_each_mem_region!(r, {
            if !memblock_is_kho_scratch(r) {
                memblock_remove((*r).base, (*r).size);
                r = r.offset(-1);
            }
        });
    } else {
        memblock_remove(0, PHYS_ADDR_MAX);
    }

    for_each_efi_memory_desc!(md, {
        paddr = (*md).phys_addr;
        npages = (*md).num_pages;
        if efi_enabled(EFI_DBG) != 0 {
            let mut buf = [0i8; 64];
            pr_info!("  0x{:012x}-0x{:012x} {}\n", paddr,
                     paddr + (npages << EFI_PAGE_SHIFT) - 1,
                     efi_md_typeattr_format(buf.as_mut_ptr(), buf.len(), md));
        }
        memrange_efi_to_native(&mut paddr, &mut npages);
        size = npages << PAGE_SHIFT;
        if is_memory(md) != 0 {
            if efi_soft_reserve_enabled() && ((*md).attribute & EFI_MEMORY_SP != 0) { continue; }
            early_init_dt_add_memory_arch(paddr, size);
            if !is_usable_memory(md) { memblock_mark_nomap(paddr, size); }
            if (*md).type_ == EFI_ACPI_RECLAIM_MEMORY { memblock_reserve(paddr, size); }
        }
    });
}

#[no_mangle]
pub unsafe fn efi_init() {
    let mut data: efi_memory_map_data = ::core::mem::zeroed();
    let efi_system_table = efi_get_fdt_params(&mut data);
    if efi_system_table == 0 { return; }
    if efi_memmap_init_early(&mut data) < 0 { panic!("Unable to map EFI memory map.\n"); }
    WARN!(efi.memmap.desc_version != 1, "Unexpected EFI_MEMORY_DESCRIPTOR version {}", efi.memmap.desc_version);
    if uefi_init(efi_system_table) < 0 { efi_memmap_unmap(); return; }
    reserve_regions();
    early_init_dt_check_for_usable_mem_range();
    efi_find_mirror();
    efi_esrt_init();
    efi_mokvar_table_init();
    memblock_reserve(data.phys_map & PAGE_MASK, PAGE_ALIGN(data.size + (data.phys_map & !PAGE_MASK)));
    if cfg!(any(feature = "CONFIG_X86", feature = "CONFIG_SYSFB", feature = "CONFIG_EFI_EARLYCON")) {
        init_primary_display();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
