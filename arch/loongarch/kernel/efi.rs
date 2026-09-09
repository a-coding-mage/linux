// SPDX-License-Identifier: GPL-2.0
/*
 * EFI initialization
 *
 * Author: Jianmin Lv <lvjianmin@loongson.cn>
 *         Huacai Chen <chenhuacai@loongson.cn>
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies are supplied by the surrounding kernel translation.

static mut efi_nr_tables: usize = 0;
static mut efi_config_table: usize = 0;

static mut boot_memmap: usize = EFI_INVALID_TABLE_ADDR;
static mut fdt_pointer: usize = EFI_INVALID_TABLE_ADDR;

static mut efi_systab: *mut efi_system_table_t = core::ptr::null_mut();
static mut arch_tables: [efi_config_table_type_t; 3] = [
    efi_config_table_type_t { guid: LINUX_EFI_BOOT_MEMMAP_GUID, table: unsafe { &mut boot_memmap }, name: b"MEMMAP\0".as_ptr() as *const i8 },
    efi_config_table_type_t { guid: DEVICE_TREE_GUID, table: unsafe { &mut fdt_pointer }, name: b"FDTPTR\0".as_ptr() as *const i8 },
    efi_config_table_type_t { guid: 0, table: core::ptr::null_mut(), name: core::ptr::null() },
];

pub unsafe fn efi_fdt_pointer() -> *mut core::ffi::c_void {
    if efi_systab.is_null() {
        return core::ptr::null_mut();
    }

    if fdt_pointer == EFI_INVALID_TABLE_ADDR {
        return core::ptr::null_mut();
    }

    early_memremap_ro(fdt_pointer, SZ_64K)
}

pub unsafe fn efi_runtime_init() {
    if !efi_enabled(EFI_BOOT) || (*efi_systab).runtime.is_null() {
        return;
    }

    if efi_runtime_disabled() {
        pr_info!("EFI runtime services will be disabled.\n");
        return;
    }

    efi.runtime = (*efi_systab).runtime as *mut efi_runtime_services_t;
    efi.runtime_version = (*(*efi_systab).runtime).hdr.revision as u32;

    efi_native_runtime_setup();
    set_bit(EFI_RUNTIME_SERVICES, &mut efi.flags);
}

pub unsafe fn efi_poweroff_required() -> bool {
    efi_enabled(EFI_RUNTIME_SERVICES) && (acpi_gbl_reduced_hardware || acpi_no_s5)
}

static mut primary_display_table: usize = EFI_INVALID_TABLE_ADDR;

// Preserved under CONFIG_SYSFB || CONFIG_EFI_EARLYCON.
#[cfg(any(CONFIG_SYSFB, CONFIG_EFI_EARLYCON))]
#[no_mangle]
pub static mut sysfb_primary_display: sysfb_display_info = unsafe { core::mem::zeroed() };

#[cfg(any(CONFIG_SYSFB, CONFIG_EFI_EARLYCON))]
unsafe fn init_primary_display() {
    let dpy: *mut sysfb_display_info;

    if primary_display_table == EFI_INVALID_TABLE_ADDR {
        return;
    }

    dpy = early_memremap(primary_display_table, core::mem::size_of::<sysfb_display_info>()) as *mut sysfb_display_info;
    if dpy.is_null() {
        pr_err!("Could not map primary_display config table\n");
        return;
    }
    sysfb_primary_display = *dpy;
    core::ptr::write_bytes(dpy, 0, 1);
    early_memunmap(dpy as *mut core::ffi::c_void, core::mem::size_of::<sysfb_display_info>());

    memblock_reserve(
        __screen_info_lfb_base(&sysfb_primary_display.screen),
        sysfb_primary_display.screen.lfb_size,
    );
}

pub unsafe fn efi_init() {
    let mut size: i32;
    let mut config_tables: *mut core::ffi::c_void;
    let tbl: *mut efi_boot_memmap;

    if efi_system_table.is_null() {
        return;
    }

    efi_systab = early_memremap_ro(efi_system_table as usize, core::mem::size_of::<efi_system_table_t>()) as *mut efi_system_table_t;
    if efi_systab.is_null() {
        pr_err!("Can't find EFI system table.\n");
        return;
    }

    efi_systab_report_header(&(*efi_systab).hdr, (*efi_systab).fw_vendor);

    if IS_ENABLED(CONFIG_64BIT) {
        set_bit(EFI_64BIT, &mut efi.flags);
    }

    efi_nr_tables = (*efi_systab).nr_tables as usize;
    efi_config_table = (*efi_systab).tables as usize;

    size = core::mem::size_of::<efi_config_table_t>() as i32;
    config_tables = early_memremap(efi_config_table, efi_nr_tables * size as usize);
    efi_config_parse_tables(config_tables, (*efi_systab).nr_tables, arch_tables.as_mut_ptr());
    early_memunmap(config_tables, efi_nr_tables * size as usize);

    set_bit(EFI_CONFIG_TABLES, &mut efi.flags);

    if IS_ENABLED(CONFIG_EFI_EARLYCON) || IS_ENABLED(CONFIG_SYSFB) {
        init_primary_display();
    }

    if boot_memmap == EFI_INVALID_TABLE_ADDR {
        return;
    }

    tbl = early_memremap_ro(boot_memmap, core::mem::size_of::<efi_boot_memmap>()) as *mut efi_boot_memmap;
    if !tbl.is_null() {
        let mut data: efi_memory_map_data = core::mem::zeroed();

        data.phys_map = boot_memmap + core::mem::size_of::<efi_boot_memmap>();
        data.size = (*tbl).map_size;
        data.desc_size = (*tbl).desc_size;
        data.desc_version = (*tbl).desc_ver;

        if efi_memmap_init_early(&mut data) < 0 {
            panic!("Unable to map EFI memory map.\n");
        }

        /*
         * Reserve the physical memory region occupied by the EFI
         * memory map table (header + descriptors). This is crucial
         * for kdump, as the kdump kernel relies on this original
         * memmap passed by the bootloader. Without reservation,
         * this region could be overwritten by the primary kernel.
         * Also, set the EFI_PRESERVE_BS_REGIONS flag to indicate that
         * critical boot services code/data regions like this are preserved.
         */
        memblock_reserve(boot_memmap as phys_addr_t, core::mem::size_of::<efi_boot_memmap>() + data.size);
        set_bit(EFI_PRESERVE_BS_REGIONS, &mut efi.flags);

        early_memunmap(tbl as *mut core::ffi::c_void, core::mem::size_of::<efi_boot_memmap>());
    }

    efi_esrt_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
