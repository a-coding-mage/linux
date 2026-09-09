// SPDX-License-Identifier: GPL-2.0-only
/* ARM64 Specific Low-Level ACPI Boot Support */

// Kernel headers and configuration-provided symbols are external dependencies.

pub static mut acpi_noirq: i32 = 1;
pub static mut acpi_disabled: i32 = 1;
pub static mut acpi_pci_disabled: i32 = 1;

static mut param_acpi_off: bool = false;
static mut param_acpi_on: bool = false;
static mut param_acpi_force: bool = false;
static mut param_acpi_nospcr: bool = false;

unsafe fn parse_acpi(arg: *mut core::ffi::c_char) -> i32 {
    if arg.is_null() { return -22; }
    if c_str_eq(arg, b"off\0") { param_acpi_off = true; }
    else if c_str_eq(arg, b"on\0") { param_acpi_on = true; }
    else if c_str_eq(arg, b"force\0") { param_acpi_force = true; }
    else if c_str_eq(arg, b"nospcr\0") { param_acpi_nospcr = true; }
    else { return -22; }
    0
}

unsafe fn c_str_eq(_a: *const core::ffi::c_char, _b: &[u8]) -> bool { todo!("external kernel string comparison") }

unsafe fn dt_is_stub() -> bool {
    let mut node: i32 = 0;
    // fdt_for_each_subnode(node, initial_boot_params, 0)
    while fdt_next_subnode(&mut node) {
        let name = fdt_get_name(node);
        if c_str_eq(name, b"chosen\0") { continue; }
        if c_str_eq(name, b"hypervisor\0") && of_flat_dt_is_compatible(node, b"xen,xen\0") { continue; }
        return false;
    }
    true
}

pub unsafe fn __acpi_map_table(phys: u64, size: u64) -> *mut core::ffi::c_void {
    if size == 0 { return core::ptr::null_mut(); }
    early_memremap(phys, size)
}

pub unsafe fn __acpi_unmap_table(map: *mut core::ffi::c_void, size: u64) {
    if map.is_null() || size == 0 { return; }
    early_memunmap(map, size);
}

pub unsafe fn acpi_psci_present() -> bool { (acpi_gbl_FADT.arm_boot_flags & ACPI_FADT_PSCI_COMPLIANT) != 0 }
pub unsafe fn acpi_psci_use_hvc() -> bool { (acpi_gbl_FADT.arm_boot_flags & ACPI_FADT_PSCI_USE_HVC) != 0 }

unsafe fn acpi_fadt_sanity_check() -> i32 {
    let mut table: *mut acpi_table_header = core::ptr::null_mut();
    let status = acpi_get_table(ACPI_SIG_FADT, 0, &mut table);
    if acpi_failure(status) { return -19; }
    let fadt = table as *mut acpi_table_fadt;
    let mut ret = 0;
    if (*table).revision < 5 || ((*table).revision == 5 && (*fadt).minor_revision < 1) {
        if (*fadt).arm_boot_flags == 0 { ret = -22; }
    }
    if ((*fadt).flags & ACPI_FADT_HW_REDUCED) == 0 { ret = -22; }
    acpi_put_table(table);
    ret
}

pub unsafe fn acpi_boot_table_init() {
    if param_acpi_off || (!param_acpi_on && !param_acpi_force && !dt_is_stub()) { return; }
    enable_acpi();
    if acpi_table_init() != 0 || acpi_fadt_sanity_check() != 0 {
        if !param_acpi_force { disable_acpi(); }
    }
    if acpi_disabled != 0 {
        if earlycon_acpi_spcr_enable { early_init_dt_scan_chosen_stdout(); }
    } else {
        // CONFIG_HIBERNATION: retrieve FACS and hardware signature when enabled.
        acpi_parse_spcr(earlycon_acpi_spcr_enable, !param_acpi_nospcr);
        // CONFIG_ACPI_BGRT: parse BGRT when enabled.
    }
}

unsafe fn __acpi_get_writethrough_mem_attribute() -> pgprot_t { __pgprot(PROT_NORMAL_NC) }

pub unsafe fn __acpi_get_mem_attribute(addr: phys_addr_t) -> pgprot_t {
    let attr = efi_mem_attributes(addr);
    if attr & EFI_MEMORY_WB != 0 { return PAGE_KERNEL; }
    if attr & EFI_MEMORY_WC != 0 { return __pgprot(PROT_NORMAL_NC); }
    if attr & EFI_MEMORY_WT != 0 { return __acpi_get_writethrough_mem_attribute(); }
    __pgprot(PROT_DEVICE_nGnRnE)
}

pub unsafe fn acpi_os_ioremap(phys: acpi_physical_address, size: acpi_size) -> *mut core::ffi::c_void {
    let mut region: *mut efi_memory_desc_t = core::ptr::null_mut();
    let mut prot = __pgprot(PROT_DEVICE_nGnRnE);
    if !efi_enabled(EFI_MEMMAP) { return core::ptr::null_mut(); }
    for_each_efi_memory_desc(|md: *mut efi_memory_desc_t| {
        let end = (*md).phys_addr + ((*md).num_pages << EFI_PAGE_SHIFT);
        if phys >= (*md).phys_addr && phys < end {
            if phys + size > end { region = core::ptr::null_mut(); return false; }
            region = md; return false;
        }
        true
    });
    if !region.is_null() {
        match (*region).type_ {
            EFI_LOADER_CODE | EFI_LOADER_DATA | EFI_BOOT_SERVICES_CODE | EFI_BOOT_SERVICES_DATA | EFI_CONVENTIONAL_MEMORY | EFI_PERSISTENT_MEMORY => {
                if memblock_is_map_memory(phys) || !memblock_is_region_memory(phys, size) { return core::ptr::null_mut(); }
                prot = PAGE_KERNEL_RO;
            }
            EFI_RUNTIME_SERVICES_CODE => prot = PAGE_KERNEL_RO,
            EFI_ACPI_RECLAIM_MEMORY => {
                if memblock_is_map_memory(phys) { return __phys_to_virt(phys); }
            }
            _ => {
                if (*region).attribute & EFI_MEMORY_WB != 0 { prot = PAGE_KERNEL; }
                else if (*region).attribute & EFI_MEMORY_WC != 0 { prot = __pgprot(PROT_NORMAL_NC); }
                else if (*region).attribute & EFI_MEMORY_WT != 0 { prot = __acpi_get_writethrough_mem_attribute(); }
            }
        }
    }
    __ioremap_prot(phys, size, prot)
}

pub unsafe fn apei_claim_sea(regs: *mut pt_regs) -> i32 {
    if !CONFIG_ACPI_APEI_GHES { return -2; }
    let mut err = -2;
    let current_flags = local_daif_save_flags();
    let mut return_to_irqs_enabled = !irqs_disabled_flags(arch_local_save_flags());
    if !regs.is_null() { return_to_irqs_enabled = !regs_irqs_disabled(regs); }
    local_daif_restore(DAIF_ERRCTX); nmi_enter(); err = ghes_notify_sea(); nmi_exit();
    if err == 0 {
        if return_to_irqs_enabled { local_daif_restore(DAIF_PROCCTX_NOIRQ); __irq_enter(); irq_work_run(); __irq_exit(); }
        else { err = -115; }
    }
    local_daif_restore(current_flags); err
}

pub unsafe fn arch_reserve_mem_area(addr: acpi_physical_address, size: usize) { memblock_mark_nomap(addr, size); }

#[cfg(CONFIG_ACPI_HOTPLUG_CPU)]
pub unsafe fn acpi_map_cpu(_handle: acpi_handle, _physid: phys_cpuid_t, _apci_id: u32, pcpu: *mut i32) -> i32 {
    if *pcpu < 0 { return *pcpu; } set_cpu_present(*pcpu, true); 0
}
#[cfg(CONFIG_ACPI_HOTPLUG_CPU)]
pub unsafe fn acpi_unmap_cpu(cpu: i32) -> i32 { set_cpu_present(cpu, false); 0 }

pub unsafe fn acpi_get_cpu_uid(cpu: u32, uid: *mut u32) -> i32 {
    if cpu >= nr_cpu_ids { return -22; }
    let gicc = acpi_cpu_get_madt_gicc(cpu); if gicc.is_null() { return -19; }
    *uid = (*gicc).uid; 0
}

pub unsafe fn get_cpu_for_acpi_id(uid: u32) -> i32 {
    for cpu in 0..nr_cpu_ids { let mut cpu_uid = 0; if acpi_get_cpu_uid(cpu, &mut cpu_uid) == 0 && uid == cpu_uid { return cpu as i32; } }
    -22
}

// External kernel types, constants, globals, and functions referenced above are supplied by other translation units.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
