// SPDX-License-Identifier: GPL-2.0-only
/*
 *  RISC-V Specific Low-Level ACPI Boot Support
 *
 *  Copyright (C) 2013-2014, Linaro Ltd.
 *	Author: Al Stone <al.stone@linaro.org>
 *	Author: Graeme Gregory <graeme.gregory@linaro.org>
 *	Author: Hanjun Guo <hanjun.guo@linaro.org>
 *	Author: Tomasz Nowicki <tomasz.nowicki@linaro.org>
 *	Author: Naresh Bhat <naresh.bhat@linaro.org>
 *
 *  Copyright (C) 2021-2023, Ventana Micro Systems Inc.
 *	Author: Sunil V L <sunilvl@ventanamicro.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

pub static mut acpi_noirq: i32 = 1; // skip ACPI IRQ initialization
pub static mut acpi_disabled: i32 = 1;
pub static mut acpi_pci_disabled: i32 = 1; // skip ACPI PCI scan and IRQ initialization

static mut param_acpi_off: bool = false;
static mut param_acpi_on: bool = false;
static mut param_acpi_force: bool = false;

static mut cpu_madt_rintc: [acpi_madt_rintc; NR_CPUS] = [acpi_madt_rintc::default(); NR_CPUS];

unsafe fn parse_acpi(arg: *mut c_char) -> i32 {
    if arg.is_null() {
        return -EINVAL;
    }

    // "acpi=off" disables both ACPI table parsing and interpreter
    if c_str_eq(arg, c"off".as_ptr()) {
        param_acpi_off = true;
    } else if c_str_eq(arg, c"on".as_ptr()) { // prefer ACPI over DT
        param_acpi_on = true;
    } else if c_str_eq(arg, c"force".as_ptr()) { // force ACPI to be enabled
        param_acpi_force = true;
    } else {
        return -EINVAL; // Core will print when we return error
    }

    0
}

// early_param("acpi", parse_acpi);

/*
 * acpi_fadt_sanity_check() - Check FADT presence and carry out sanity
 *                              checks on it
 *
 * Return 0 on success,  <0 on failure
 */
unsafe fn acpi_fadt_sanity_check() -> i32 {
    let mut table: *mut acpi_table_header = core::ptr::null_mut();
    let mut ret = 0;

    // FADT is required on riscv; retrieve it to check its presence
    // and carry out revision and ACPI HW reduced compliance tests
    let status = acpi_get_table(ACPI_SIG_FADT, 0, &mut table);
    if ACPI_FAILURE(status) {
        let msg = acpi_format_exception(status);
        pr_err!("Failed to get FADT table, {}\n", msg);
        return -ENODEV;
    }

    let fadt = table as *mut acpi_table_fadt;

    /*
     * The revision in the table header is the FADT's Major revision. The
     * FADT also has a minor revision, which is stored in the FADT itself.
     *
     * ACPI 6.6 is required for RISC-V as it introduces RISC-V specific
     * tables such as RHCT (RISC-V Hart Capabilities Table) and RIMT
     * (RISC-V I/O Mapping Table).
     */
    if (*table).revision < 6 || ((*table).revision == 6 && (*fadt).minor_revision < 6) {
        pr_err!(FW_BUG "Unsupported FADT revision {}.{}, should be 6.6+\n",
                (*table).revision, (*fadt).minor_revision);
    }

    if (*fadt).flags & ACPI_FADT_HW_REDUCED == 0 {
        pr_err!("FADT not ACPI hardware reduced compliant\n");
        ret = -EINVAL;
    }

    // acpi_get_table() creates FADT table mapping that should be released after parsing.
    acpi_put_table(table);
    ret
}

/*
 * acpi_boot_table_init() called from setup_arch(), always.
 *	1. find RSDP and get its address, and then find XSDT
 *	2. extract all tables and checksums them all
 *	3. check ACPI FADT HW reduced flag
 */
pub unsafe fn acpi_boot_table_init() {
    if param_acpi_off || (!param_acpi_on && !param_acpi_force && efi.acpi20 == EFI_INVALID_TABLE_ADDR) {
        goto_done!();
    }

    enable_acpi();

    if acpi_table_init() != 0 || acpi_fadt_sanity_check() != 0 {
        pr_err!("Failed to init ACPI tables\n");
        if !param_acpi_force {
            disable_acpi();
        }
    }

    if acpi_disabled != 0 {
        if earlycon_acpi_spcr_enable {
            early_init_dt_scan_chosen_stdout();
        }
    } else {
        acpi_parse_spcr(earlycon_acpi_spcr_enable, true);
        // CONFIG_ACPI_BGRT is a build-time condition supplied by the kernel configuration.
        if IS_ENABLED_CONFIG_ACPI_BGRT {
            acpi_table_parse(ACPI_SIG_BGRT, acpi_parse_bgrt);
        }
    }
}

unsafe fn acpi_parse_madt_rintc(header: *mut acpi_subtable_headers, _end: c_ulong) -> i32 {
    let rintc = header as *mut acpi_madt_rintc;
    if (*rintc).flags & ACPI_MADT_ENABLED == 0 {
        return 0;
    }

    let cpuid = riscv_hartid_to_cpuid((*rintc).hart_id);
    // CPUs more than num_possible_cpus, will be ignored.
    if cpuid >= 0 && cpuid < num_possible_cpus() {
        cpu_madt_rintc[cpuid as usize] = *rintc;
    }
    0
}

pub unsafe fn acpi_init_rintc_map() {
    if acpi_table_parse_madt(ACPI_MADT_TYPE_RINTC, acpi_parse_madt_rintc, 0) <= 0 {
        pr_err!("No valid RINTC entries exist\n");
        BUG!();
    }
}

pub unsafe fn acpi_cpu_get_madt_rintc(cpu: i32) -> *mut acpi_madt_rintc {
    &mut cpu_madt_rintc[cpu as usize]
}

pub unsafe fn __acpi_map_table(phys: c_ulong, size: c_ulong) -> *mut core::ffi::c_void {
    if size == 0 { return core::ptr::null_mut(); }
    early_memremap(phys, size)
}

pub unsafe fn __acpi_unmap_table(map: *mut core::ffi::c_void, size: c_ulong) {
    if map.is_null() || size == 0 { return; }
    early_memunmap(map, size);
}

pub unsafe fn acpi_os_ioremap(phys: acpi_physical_address, size: acpi_size) -> *mut core::ffi::c_void {
    let mut region: *mut efi_memory_desc_t = core::ptr::null_mut();
    let mut prot = PAGE_KERNEL_IO;

    if WARN_ON_ONCE(!efi_enabled(EFI_MEMMAP)) { return core::ptr::null_mut(); }

    for_each_efi_memory_desc!(md) {
        let end = (*md).phys_addr + ((*md).num_pages << EFI_PAGE_SHIFT);
        if phys < (*md).phys_addr || phys >= end { continue; }
        if phys + size > end {
            pr_warn!(FW_BUG "requested region covers multiple EFI memory regions\n");
            return core::ptr::null_mut();
        }
        region = md;
        break;
    }

    if !region.is_null() {
        match (*region).type_ {
            EFI_LOADER_CODE | EFI_LOADER_DATA | EFI_BOOT_SERVICES_CODE |
            EFI_BOOT_SERVICES_DATA | EFI_CONVENTIONAL_MEMORY | EFI_PERSISTENT_MEMORY => {
                if memblock_is_map_memory(phys) || !memblock_is_region_memory(phys, size) {
                    pr_warn!(FW_BUG "requested region covers kernel memory\n");
                    return core::ptr::null_mut();
                }
                prot = PAGE_KERNEL_RO;
            }
            EFI_RUNTIME_SERVICES_CODE => prot = PAGE_KERNEL_RO,
            EFI_ACPI_RECLAIM_MEMORY => {
                if memblock_is_map_memory(phys) { return __va(phys) as *mut core::ffi::c_void; }
                if (*region).attribute & EFI_MEMORY_WB != 0 { prot = PAGE_KERNEL; }
            }
            _ => {
                if (*region).attribute & EFI_MEMORY_WB != 0 { prot = PAGE_KERNEL; }
                else if (*region).attribute & (EFI_MEMORY_WC | EFI_MEMORY_WT) != 0 {
                    prot = pgprot_writecombine(PAGE_KERNEL);
                }
            }
        }
    }
    ioremap_prot(phys, size, prot)
}

// The following PCI accessors are compiled when CONFIG_PCI is enabled.
#[cfg(CONFIG_PCI)]
pub unsafe fn raw_pci_read(domain: c_uint, bus: c_uint, devfn: c_uint, reg: i32, len: i32, val: *mut u32) -> i32 {
    let b = pci_find_bus(domain, bus);
    if b.is_null() { return PCIBIOS_DEVICE_NOT_FOUND; }
    ((*(*b).ops).read)(b, devfn, reg, len, val)
}

#[cfg(CONFIG_PCI)]
pub unsafe fn raw_pci_write(domain: c_uint, bus: c_uint, devfn: c_uint, reg: i32, len: i32, val: u32) -> i32 {
    let b = pci_find_bus(domain, bus);
    if b.is_null() { return PCIBIOS_DEVICE_NOT_FOUND; }
    ((*(*b).ops).write)(b, devfn, reg, len, val)
}

pub unsafe fn acpi_get_cpu_uid(cpu: c_uint, uid: *mut u32) -> i32 {
    if cpu >= nr_cpu_ids { return -EINVAL; }
    let rintc = acpi_cpu_get_madt_rintc(cpu as i32);
    if rintc.is_null() { return -ENODEV; }
    *uid = (*rintc).uid;
    0
}

pub unsafe fn arch_reserve_mem_area(addr: acpi_physical_address, size: usize) {
    memblock_mark_nomap(addr, size);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
