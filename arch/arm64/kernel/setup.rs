// SPDX-License-Identifier: GPL-2.0-only
/*
 * Based on arch/arm/kernel/setup.c
 *
 * Copyright (C) 1995-2001 Russell King
 * Copyright (C) 2012 ARM Ltd.
 */

// Linux and architecture headers from the C translation unit provide the
// external types, constants, functions, and configuration symbols used here.

static mut NUM_STANDARD_RESOURCES: c_int = 0;
static mut STANDARD_RESOURCES: *mut resource = core::ptr::null_mut();

static mut __FDT_POINTER: phys_addr_t = 0;
static mut MMU_ENABLED_AT_BOOT: u64 = 0;

/* Standard memory resources */
static mut MEM_RES: [resource; 2] = [
    resource { name: b"Kernel code\0".as_ptr() as *const c_char, start: 0, end: 0, flags: IORESOURCE_SYSTEM_RAM },
    resource { name: b"Kernel data\0".as_ptr() as *const c_char, start: 0, end: 0, flags: IORESOURCE_SYSTEM_RAM },
];

/* The recorded values of x0 .. x3 upon kernel entry. */
#[no_mangle]
pub static mut boot_args: [u64; 4] = [0; 4];

pub unsafe fn smp_setup_processor_id() {
    let mpidr = read_cpuid_mpidr() & MPIDR_HWID_BITMASK;
    set_cpu_logical_map(0, mpidr);
    pr_info!("Booting Linux on physical CPU 0x%010lx [0x%08x]\n", mpidr as c_ulong, read_cpuid_id());
}

pub unsafe fn arch_match_cpu_phys_id(cpu: c_int, phys_id: u64) -> bool {
    phys_id == cpu_logical_map(cpu as c_uint)
}

pub static mut mpidr_hash: mpidr_hash = mpidr_hash { shift_aff: [0; 4], mask: 0, bits: 0 };

/** smp_build_mpidr_hash - Pre-compute shifts required at each affinity
 * level in order to build a linear index from an MPIDR value. Resulting
 * algorithm is a collision free hash carried out through shifting and ORing
 */
unsafe fn smp_build_mpidr_hash() {
    let mut i: u32;
    let mut affinity: u32;
    let mut fs = [0u32; 4];
    let mut bits = [0u32; 4];
    let mut ls: u32;
    let mut mask: u64 = 0;
    for_each_possible_cpu!(i) { mask |= cpu_logical_map(i) ^ cpu_logical_map(0); }
    pr_debug!("mask of set bits %#llx\n", mask);
    for i in 0..4 {
        affinity = MPIDR_AFFINITY_LEVEL(mask, i);
        ls = fls(affinity);
        fs[i as usize] = if affinity != 0 { ffs(affinity) - 1 } else { 0 };
        bits[i as usize] = ls - fs[i as usize];
    }
    mpidr_hash.shift_aff[0] = MPIDR_LEVEL_SHIFT(0) + fs[0];
    mpidr_hash.shift_aff[1] = MPIDR_LEVEL_SHIFT(1) + fs[1] - bits[0];
    mpidr_hash.shift_aff[2] = MPIDR_LEVEL_SHIFT(2) + fs[2] - (bits[1] + bits[0]);
    mpidr_hash.shift_aff[3] = MPIDR_LEVEL_SHIFT(3) + fs[3] - (bits[2] + bits[1] + bits[0]);
    mpidr_hash.mask = mask;
    mpidr_hash.bits = bits[3] + bits[2] + bits[1] + bits[0];
    pr_debug!("MPIDR hash: aff0[%u] aff1[%u] aff2[%u] aff3[%u] mask[%#llx] bits[%u]\n", mpidr_hash.shift_aff[0], mpidr_hash.shift_aff[1], mpidr_hash.shift_aff[2], mpidr_hash.shift_aff[3], mpidr_hash.mask, mpidr_hash.bits);
    if mpidr_hash_size() > 4 * num_possible_cpus() { pr_warn!("Large number of MPIDR hash buckets detected\n"); }
}

unsafe fn setup_machine_fdt(dt_phys: phys_addr_t) {
    let mut size: c_int = 0;
    let dt_virt = fixmap_remap_fdt(dt_phys, &mut size, PAGE_KERNEL);
    if !dt_virt.is_null() { memblock_reserve(dt_phys, size as u64); }
    if early_init_dt_scan(dt_virt, dt_phys) == 0 {
        pr_crit!("\nError: invalid device tree blob: PA=%pa, VA=%px, size=%d bytes\nThe dtb must be 8-byte aligned and must not exceed 2 MB in size.\n\nPlease check your bootloader.\n", &dt_phys, dt_virt, size);
        loop { cpu_relax(); }
    }
    fixmap_remap_fdt(dt_phys, &mut size, PAGE_KERNEL_RO);
    let name = of_flat_dt_get_machine_name();
    if name.is_null() { return; }
    pr_info!("Machine model: %s\n", name);
    dump_stack_set_arch_desc!("%s (DT)", name);
}

unsafe fn request_standard_resources() {
    kernel_code_mut().start = __pa_symbol!(_text);
    kernel_code_mut().end = __pa_symbol!(__init_begin - 1);
    kernel_data_mut().start = __pa_symbol!(_sdata);
    kernel_data_mut().end = __pa_symbol!(_end - 1);
    insert_resource(&mut iomem_resource, kernel_code_mut());
    insert_resource(&mut iomem_resource, kernel_data_mut());
    NUM_STANDARD_RESOURCES = memblock.memory.cnt;
    let res_size = NUM_STANDARD_RESOURCES as usize * core::mem::size_of::<resource>();
    STANDARD_RESOURCES = memblock_alloc_or_panic(res_size, SMP_CACHE_BYTES);
    let mut i = 0usize;
    for_each_mem_region!(region) {
        let res = &mut *STANDARD_RESOURCES.add(i); i += 1;
        if memblock_is_nomap(region) {
            res.name = b"reserved\0".as_ptr() as *const c_char;
            res.flags = IORESOURCE_MEM;
            res.start = __pfn_to_phys(memblock_region_reserved_base_pfn(region));
            res.end = __pfn_to_phys(memblock_region_reserved_end_pfn(region)) - 1;
        } else {
            res.name = b"System RAM\0".as_ptr() as *const c_char;
            res.flags = IORESOURCE_SYSTEM_RAM | IORESOURCE_BUSY;
            res.start = __pfn_to_phys(memblock_region_memory_base_pfn(region));
            res.end = __pfn_to_phys(memblock_region_memory_end_pfn(region)) - 1;
        }
        insert_resource(&mut iomem_resource, res);
    }
}

unsafe fn reserve_memblock_reserved_regions() -> c_int {
    for i in 0..NUM_STANDARD_RESOURCES as usize {
        let mem = &mut *STANDARD_RESOURCES.add(i);
        let mem_size = resource_size(mem);
        if !memblock_is_region_reserved(mem.start, mem_size) { continue; }
        for_each_reserved_mem_range!(j, r_start, r_end) {
            let start = core::cmp::max(PFN_PHYS(PFN_DOWN(r_start)), mem.start);
            let end = core::cmp::min(PFN_PHYS(PFN_UP(r_end)) - 1, mem.end);
            if start > mem.end || end < mem.start { continue; }
            reserve_region_with_split(mem, start, end, b"reserved\0".as_ptr() as *const c_char);
        }
    }
    0
}

// arch_initcall(reserve_memblock_reserved_regions)

#[no_mangle]
pub static mut __cpu_logical_map: [u64; NR_CPUS] = [INVALID_HWID; NR_CPUS];

pub unsafe fn cpu_logical_map(cpu: c_uint) -> u64 { __cpu_logical_map[cpu as usize] }

pub unsafe fn setup_arch(cmdline_p: *mut *mut c_char) {
    setup_initial_init_mm(_text, _etext, _edata, _end);
    *cmdline_p = boot_command_line;
    kaslr_init();
    early_fixmap_init(); early_ioremap_init();
    setup_machine_fdt(__FDT_POINTER);
    jump_label_init(); parse_early_param(); dynamic_scs_init();
    local_daif_restore(DAIF_PROCCTX_NOIRQ);
    cpu_uninstall_idmap(); xen_early_init(); efi_init();
    if !efi_enabled(EFI_BOOT) {
        if (_text as u64) % MIN_KIMG_ALIGN != 0 { pr_warn!(FW_BUG "Kernel image misaligned at boot, please fix your bootloader!"); }
        WARN_TAINT!(MMU_ENABLED_AT_BOOT, TAINT_FIRMWARE_WORKAROUND, FW_BUG "Booted with MMU enabled!");
    }
    arm64_memblock_init(); paging_init(); acpi_table_upgrade(); acpi_boot_table_init();
    if acpi_disabled { unflatten_device_tree(); }
    bootmem_init(); kasan_init(); request_standard_resources();
    if acpi_disabled { psci_dt_init(); } else { psci_acpi_init(); }
    arm64_rsi_init(); init_bootcpu_ops(); smp_init_cpus(); smp_build_mpidr_hash();
    if boot_args[1] != 0 || boot_args[2] != 0 || boot_args[3] != 0 {
        pr_err!("WARNING: x1-x3 nonzero in violation of boot protocol:\n\tx1: %016llx\n\tx2: %016llx\n\tx3: %016llx\nThis indicates a broken bootloader or old kernel\n", boot_args[1], boot_args[2], boot_args[3]);
    }
}

unsafe fn cpu_can_disable(cpu: c_uint) -> bool {
    // CONFIG_HOTPLUG_CPU condition from the C source is build-time dependent.
    #[cfg(CONFIG_HOTPLUG_CPU)]
    { let ops = get_cpu_ops(cpu); if !ops.is_null() && (*ops).cpu_can_disable.is_some() { return ((*ops).cpu_can_disable.unwrap())(cpu); } }
    false
}

pub unsafe fn arch_cpu_is_hotpluggable(num: c_int) -> bool { cpu_can_disable(num as c_uint) }

unsafe fn dump_kernel_offset() {
    let offset = kaslr_offset();
    if IS_ENABLED!(CONFIG_RANDOMIZE_BASE) && offset > 0 { pr_emerg!("Kernel Offset: 0x%lx from 0x%lx\n", offset, KIMAGE_VADDR); pr_emerg!("PHYS_OFFSET: 0x%llx\n", PHYS_OFFSET); } else { pr_emerg!("Kernel Offset: disabled\n"); }
}

unsafe fn arm64_panic_block_dump(_self: *mut notifier_block, _v: c_ulong, _p: *mut c_void) -> c_int { dump_kernel_offset(); dump_cpu_features(); dump_mem_limit(); 0 }

static mut arm64_panic_block: notifier_block = notifier_block { notifier_call: Some(arm64_panic_block_dump) };

unsafe fn register_arm64_panic_block() -> c_int { atomic_notifier_chain_register(&mut panic_notifier_list, &mut arm64_panic_block); 0 }

// device_initcall(register_arm64_panic_block)

unsafe fn check_mmu_enabled_at_boot() -> c_int { if !efi_enabled(EFI_BOOT) && MMU_ENABLED_AT_BOOT != 0 { panic!("Non-EFI boot detected with MMU and caches enabled"); } 0 }

// device_initcall_sync(check_mmu_enabled_at_boot)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
