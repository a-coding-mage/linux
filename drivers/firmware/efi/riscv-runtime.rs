// SPDX-License-Identifier: GPL-2.0
/*
 * Extensible Firmware Interface
 *
 * Copyright (C) 2020 Western Digital Corporation or its affiliates.
 *
 * Based on Extensible Firmware Interface Specification version 2.4
 * Adapted from drivers/firmware/efi/arm-runtime.c
 */

// Linux and architecture headers from the original implementation provide the
// declarations used below.

unsafe fn efi_virtmap_init() -> bool {
    let mut md: *mut efi_memory_desc_t;

    efi_mm.pgd = pgd_alloc(&mut efi_mm);
    mm_init_cpumask(&mut efi_mm);
    init_new_context(core::ptr::null_mut(), &mut efi_mm);

    // for_each_efi_memory_desc(md)
    for md in efi_memory_descs() {
        if (*md).attribute & EFI_MEMORY_RUNTIME == 0 {
            continue;
        }
        if (*md).virt_addr == u64::MAX {
            return false;
        }

        efi_create_mapping(&mut efi_mm, md);
    }

    if efi_memattr_apply_permissions(&mut efi_mm, efi_set_mapping_permissions) != 0 {
        return false;
    }

    true
}

/*
 * Enable the UEFI Runtime Services if all prerequisites are in place, i.e.,
 * non-early mapping of the UEFI system table and virtual mappings for all
 * EFI_MEMORY_RUNTIME regions.
 */
unsafe fn riscv_enable_runtime_services() -> i32 {
    let mapsize: u64;

    if !efi_enabled(EFI_BOOT) {
        pr_info!("EFI services will not be available.\n");
        return 0;
    }

    efi_memmap_unmap();

    mapsize = efi.memmap.desc_size * efi.memmap.nr_map;

    if efi_memmap_init_late(efi.memmap.phys_map, mapsize) != 0 {
        pr_err!("Failed to remap EFI memory map\n");
        return 0;
    }

    if efi_soft_reserve_enabled() {
        // for_each_efi_memory_desc(md)
        for md in efi_memory_descs() {
            let md_size: u64 = (*md).num_pages << EFI_PAGE_SHIFT;
            let res: *mut resource;

            if (*md).attribute & EFI_MEMORY_SP == 0 {
                continue;
            }

            res = kzalloc_obj::<resource>();
            if WARN_ON(res.is_null()) {
                break;
            }

            (*res).start = (*md).phys_addr;
            (*res).end = (*md).phys_addr + md_size - 1;
            (*res).name = c"Soft Reserved".as_ptr();
            (*res).flags = IORESOURCE_MEM;
            (*res).desc = IORES_DESC_SOFT_RESERVED;

            insert_resource(&mut iomem_resource, res);
        }
    }

    if efi_runtime_disabled() {
        pr_info!("EFI runtime services will be disabled.\n");
        return 0;
    }

    if efi_enabled(EFI_RUNTIME_SERVICES) {
        pr_info!("EFI runtime services access via paravirt.\n");
        return 0;
    }

    pr_info!("Remapping and enabling EFI services.\n");

    if !efi_virtmap_init() {
        pr_err!("UEFI virtual mapping missing or invalid -- runtime services will not be available\n");
        return -ENOMEM;
    }

    // Set up runtime services function pointers.
    efi_native_runtime_setup();
    set_bit(EFI_RUNTIME_SERVICES, &mut efi.flags);

    0
}

// early_initcall(riscv_enable_runtime_services);

unsafe fn efi_virtmap_load() {
    preempt_disable();
    switch_mm((*current).active_mm, &mut efi_mm, core::ptr::null_mut());
}

unsafe fn efi_virtmap_unload() {
    switch_mm(&mut efi_mm, (*current).active_mm, core::ptr::null_mut());
    preempt_enable();
}

pub unsafe fn arch_efi_call_virt_setup() {
    sync_kernel_mappings(efi_mm.pgd);
    efi_virtmap_load();
}

pub unsafe fn arch_efi_call_virt_teardown() {
    efi_virtmap_unload();
}

unsafe fn riscv_dmi_init() -> i32 {
    /*
     * On riscv, DMI depends on UEFI, and dmi_setup() needs to
     * be called early because dmi_id_init(), which is an arch_initcall
     * itself, depends on dmi_scan_machine() having been called already.
     */
    dmi_setup();

    0
}

// core_initcall(riscv_dmi_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
