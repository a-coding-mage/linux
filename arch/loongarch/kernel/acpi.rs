// SPDX-License-Identifier: GPL-2.0
/*
 * acpi.c - Architecture-Specific Low-Level ACPI Boot Support
 *
 * Author: Jianmin Lv <lvjianmin@loongson.cn>
 *         Huacai Chen <chenhuacai@loongson.cn>
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Kernel headers and architecture dependencies are supplied by the surrounding tree.

pub static mut acpi_disabled: i32 = 0;
pub static mut acpi_noirq: i32 = 0;
pub static mut acpi_pci_disabled: i32 = 0;
pub static mut acpi_strict: i32 = 1; /* We have no workarounds on LoongArch */
pub static mut num_processors: i32 = 0;
pub static mut disabled_cpus: i32 = 0;
pub static mut acpi_saved_sp: u64 = 0;

pub const PREFIX: &[u8] = b"ACPI: \0";

pub static mut acpi_core_pic: [acpi_madt_core_pic; MAX_CORE_PIC] = [acpi_madt_core_pic::default(); MAX_CORE_PIC];

pub unsafe fn __acpi_map_table(phys: usize, size: usize) -> *mut core::ffi::c_void {
    if phys == 0 || size == 0 { return core::ptr::null_mut(); }
    early_memremap(phys, size)
}

pub unsafe fn __acpi_unmap_table(map: *mut core::ffi::c_void, size: usize) {
    if map.is_null() || size == 0 { return; }
    early_memunmap(map, size);
}

pub unsafe fn acpi_os_ioremap(phys: acpi_physical_address, size: acpi_size) -> *mut core::ffi::c_void {
    if !memblock_is_memory(phys) { ioremap(phys, size) } else { ioremap_cache(phys, size) }
}

pub const PIO_BASE: usize = PCI_IOBASE as usize;
pub const PIO_SIZE: usize = ALIGN(ISA_IOSIZE, PAGE_SIZE);
static mut acpi_pio: bool = false;

pub unsafe fn acpi_add_early_pio() {
    if !acpi_disabled {
        acpi_pio = true;
        vmap_page_range(PIO_BASE, PIO_BASE + PIO_SIZE, LOONGSON_LIO_BASE, pgprot_device(PAGE_KERNEL));
    }
}

pub unsafe fn acpi_remove_early_pio() {
    if !acpi_pio { return; }
    if !acpi_disabled {
        acpi_pio = false;
        vunmap_range(PIO_BASE, PIO_BASE + PIO_SIZE);
    }
}

#[cfg(CONFIG_SMP)]
unsafe fn set_processor_mask(id: u32, pass: u32) -> i32 {
    let mut cpu: i32 = -1;
    let cpuid = id;
    if num_processors >= NR_CPUS {
        pr_warn!("nr_cpus limit of {} reached. processor 0x{:x} ignored.\n", NR_CPUS, cpuid);
        return -ENODEV;
    }
    if cpuid == loongson_sysconf.boot_cpu_id { cpu = 0; }
    match pass {
        1 => { if cpu < 0 { cpu = find_first_zero_bit(cpumask_bits(cpu_present_mask), NR_CPUS) as i32; } num_processors += 1; set_cpu_present(cpu, true); }
        2 => { if cpu < 0 { cpu = find_first_zero_bit(cpumask_bits(cpu_possible_mask), NR_CPUS) as i32; } disabled_cpus += 1; }
        _ => return cpu,
    }
    set_cpu_possible(cpu, true);
    __cpu_number_map[cpuid as usize] = cpu;
    __cpu_logical_map[cpu as usize] = cpuid as i32;
    cpu
}

pub unsafe fn acpi_parse_p1_processor(header: *mut acpi_subtable_headers, end: usize) -> i32 {
    let processor = header as *mut acpi_madt_core_pic;
    if BAD_MADT_ENTRY(processor, end) { return -EINVAL; }
    acpi_table_print_madt_entry(&(*header).common);
    #[cfg(CONFIG_SMP)]
    { acpi_core_pic[(*processor).core_id as usize] = *processor; if (*processor).flags & ACPI_MADT_ENABLED != 0 { set_processor_mask((*processor).core_id, 1); } }
    0
}

pub unsafe fn acpi_parse_p2_processor(header: *mut acpi_subtable_headers, end: usize) -> i32 {
    let processor = header as *mut acpi_madt_core_pic;
    if BAD_MADT_ENTRY(processor, end) { return -EINVAL; }
    #[cfg(CONFIG_SMP)]
    { if (*processor).flags & ACPI_MADT_ENABLED == 0 { set_processor_mask((*processor).core_id, 2); } }
    0
}

pub unsafe fn acpi_parse_eio_master(header: *mut acpi_subtable_headers, end: usize) -> i32 {
    static mut core: i32 = 0;
    let eiointc = header as *mut acpi_madt_eio_pic;
    if BAD_MADT_ENTRY(eiointc, end) { return -EINVAL; }
    core = (*eiointc).node as i32 * CORES_PER_EIO_NODE;
    set_bit(core as usize, loongson_sysconf.cores_io_master);
    0
}

pub unsafe fn acpi_process_madt() {
    #[cfg(CONFIG_SMP)]
    { for i in 0..NR_CPUS { __cpu_number_map[i] = -1; __cpu_logical_map[i] = -1; } }
    acpi_table_parse_madt(ACPI_MADT_TYPE_CORE_PIC, acpi_parse_p1_processor, MAX_CORE_PIC);
    acpi_table_parse_madt(ACPI_MADT_TYPE_CORE_PIC, acpi_parse_p2_processor, MAX_CORE_PIC);
    acpi_table_parse_madt(ACPI_MADT_TYPE_EIO_PIC, acpi_parse_eio_master, MAX_IO_PICS);
    loongson_sysconf.nr_cpus = num_processors;
}

pub static mut pptt_enabled: i32 = 0;
static mut acpi_nr_packages: i32 = 0;
static mut acpi_package_ids: [i32; MAX(MAX_PACKAGES, KVM_MAX_VCPUS)] = [0; MAX(MAX_PACKAGES, KVM_MAX_VCPUS)];

pub unsafe fn parse_acpi_topology() -> i32 {
    let mut i: i32; let mut topology_id: i32;
    for_each_possible_cpu!(cpu, {
        topology_id = find_acpi_cpu_topology(cpu, 0); if topology_id < 0 { pr_warn!("Invalid BIOS PPTT\n"); return -ENOENT; }
        if acpi_pptt_cpu_is_thread(cpu) <= 0 { cpu_data[cpu].core = topology_id; } else { topology_id = find_acpi_cpu_topology(cpu, 1); if topology_id < 0 { return -ENOENT; } cpu_data[cpu].core = topology_id; }
        topology_id = find_acpi_cpu_topology_package(cpu); if topology_id < 0 { pr_warn!("Invalid BIOS PPTT\n"); return -ENOENT; }
        i = 0; while i < acpi_nr_packages && acpi_package_ids[i as usize] != topology_id { i += 1; }
        if i == acpi_nr_packages { acpi_package_ids[acpi_nr_packages as usize] = topology_id; acpi_nr_packages += 1; }
        cpu_data[cpu].package = topology_id;
    });
    for_each_possible_cpu!(cpu, { i = 0; while i < acpi_nr_packages { if cpu_data[cpu].package == acpi_package_ids[i as usize] { cpu_data[cpu].package = i; break; } i += 1; } });
    pptt_enabled = 1; 0
}

#[cfg(not(CONFIG_SUSPEND))] pub static mut acpi_suspend_lowlevel: Option<unsafe extern "C" fn() -> i32> = None;
#[cfg(CONFIG_SUSPEND)] pub static mut acpi_suspend_lowlevel: Option<unsafe extern "C" fn() -> i32> = Some(loongarch_acpi_suspend);

pub unsafe fn acpi_boot_table_init() {
    if acpi_disabled { return earlycon_fdt(); }
    if acpi_table_init() != 0 { disable_acpi(); return earlycon_fdt(); }
    loongson_sysconf.boot_cpu_id = read_csr_cpuid();
    acpi_process_madt();
    acpi_parse_spcr(earlycon_acpi_spcr_enable, false);
    if IS_ENABLED(CONFIG_ACPI_BGRT) { acpi_table_parse(ACPI_SIG_BGRT, acpi_parse_bgrt); }
}
unsafe fn earlycon_fdt() { if earlycon_acpi_spcr_enable { early_init_dt_scan_chosen_stdout(); } }

pub unsafe fn arch_reserve_mem_area(addr: acpi_physical_address, size: usize) { memblock_reserve(addr, size); }

#[cfg(CONFIG_ACPI_HOTPLUG_CPU)]
pub unsafe fn acpi_map_cpu(handle: acpi_handle, physid: phys_cpuid_t, _acpi_id: u32, pcpu: *mut i32) -> i32 {
    let cpu = cpu_number_map(physid); if cpu < 0 || cpu >= nr_cpu_ids { pr_info!("ACPI: Unable to map lapic to logical cpu number\n"); return -ERANGE; }
    num_processors += 1; set_cpu_present(cpu, true); *pcpu = cpu; 0
}

#[cfg(CONFIG_ACPI_HOTPLUG_CPU)]
pub unsafe fn acpi_unmap_cpu(cpu: i32) -> i32 { set_cpu_present(cpu, false); num_processors -= 1; pr_info!("cpu{} hot remove!\n", cpu); 0 }

pub unsafe fn acpi_get_cpu_uid(cpu: u32, uid: *mut u32) -> i32 {
    if cpu >= nr_cpu_ids as u32 { return -EINVAL; }
    *uid = acpi_core_pic[cpu_logical_map(cpu as i32) as usize].processor_id; 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
