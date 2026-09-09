// SPDX-License-Identifier: GPL-2.0
/*
 * Extensible Firmware Interface
 *
 * Based on Extensible Firmware Interface Specification version 1.0
 *
 * Copyright (C) 1999 VA Linux Systems
 * Copyright (C) 1999 Walt Drummond <drummond@valinux.com>
 * Copyright (C) 1999-2002 Hewlett-Packard Co.
 *	David Mosberger-Tang <davidm@hpl.hp.com>
 *	Stephane Eranian <eranian@hpl.hp.com>
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

pub unsafe fn efi_map_region(md: *mut efi_memory_desc_t) {
    let start_pfn: u64;
    let end_pfn: u64;
    let end: u64;
    let size: usize;
    let va: *mut core::ffi::c_void;

    start_pfn = PFN_DOWN((*md).phys_addr);
    size = ((*md).num_pages << PAGE_SHIFT) as usize;
    end = (*md).phys_addr + size as u64;
    end_pfn = PFN_UP(end);

    if pfn_range_is_mapped(start_pfn, end_pfn) {
        va = __va((*md).phys_addr);

        if ((*md).attribute & EFI_MEMORY_WB) == 0 {
            set_memory_uc(va as usize, (*md).num_pages);
        }
    } else {
        va = ioremap_cache((*md).phys_addr, size);
    }

    (*md).virt_addr = va as usize as u64;
    if va.is_null() {
        pr_err!("ioremap of 0x{:llX} failed!\n", (*md).phys_addr);
    }
}

/*
 * To make EFI call EFI runtime service in physical addressing mode we need
 * prolog/epilog before/after the invocation to claim the EFI runtime service
 * handler exclusively and to duplicate a memory mapping in low memory space,
 * say 0 - 3G.
 */

pub unsafe fn efi_alloc_page_tables() -> i32 {
    0
}

pub unsafe fn efi_sync_low_kernel_mappings() {}

pub unsafe fn efi_dump_pagetable() {
    // CONFIG_EFI_PGT_DUMP conditional code is preserved here for the build configuration.
    #[cfg(CONFIG_EFI_PGT_DUMP)]
    {
        ptdump_walk_pgd_level(core::ptr::null_mut(), &mut init_mm);
    }
}

pub unsafe fn efi_setup_page_tables(_pa_memmap: usize, _num_pages: u32) -> i32 {
    0
}

pub unsafe fn efi_map_region_fixed(_md: *mut efi_memory_desc_t) {}
pub unsafe fn parse_efi_setup(_phys_addr: u64, _data_len: u32) {}

extern "C" {
    pub fn efi_call_svam(
        runtime: *const *mut efi_runtime_services_t,
        memory_map_size: u32,
        descriptor_size: u32,
        descriptor_version: u32,
        virtual_map: *mut core::ffi::c_void,
        runtime_addr: u32,
    ) -> efi_status_t;
}

pub unsafe fn efi_set_virtual_address_map(
    memory_map_size: usize,
    descriptor_size: usize,
    descriptor_version: u32,
    virtual_map: *mut efi_memory_desc_t,
    systab_phys: usize,
) -> efi_status_t {
    let systab = systab_phys as *const efi_system_table_t;
    let mut gdt_descr: desc_ptr;
    let status: efi_status_t;
    let mut flags: usize;
    let save_pgd: *mut pgd_t;

    /* Current pgd is swapper_pg_dir, we'll restore it later: */
    save_pgd = swapper_pg_dir;
    load_cr3(initial_page_table);
    __flush_tlb_all();

    gdt_descr.address = get_cpu_gdt_paddr(0);
    gdt_descr.size = GDT_SIZE - 1;
    load_gdt(&gdt_descr);

    /* Disable interrupts around EFI calls: */
    local_irq_save(&mut flags);
    status = efi_call_svam(
        &(*systab).runtime,
        memory_map_size as u32,
        descriptor_size as u32,
        descriptor_version,
        virtual_map as *mut core::ffi::c_void,
        __pa(&efi.runtime) as u32,
    );
    local_irq_restore(flags);

    load_fixmap_gdt(0);
    load_cr3(save_pgd);
    __flush_tlb_all();

    status
}

pub unsafe fn efi_runtime_update_mappings() {
    if (__supported_pte_mask & _PAGE_NX) != 0 {
        let mut md: *mut efi_memory_desc_t;

        /* Make EFI runtime service code area executable */
        for_each_efi_memory_desc!(md) {
            if (*md).type_ != EFI_RUNTIME_SERVICES_CODE {
                continue;
            }

            set_memory_x((*md).virt_addr as usize, (*md).num_pages);
        }
    }
}

pub unsafe fn arch_efi_call_virt_setup() {
    efi_fpu_begin();
    firmware_restrict_branch_speculation_start();
}

pub unsafe fn arch_efi_call_virt_teardown() {
    firmware_restrict_branch_speculation_end();
    efi_fpu_end();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
