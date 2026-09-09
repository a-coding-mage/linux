// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2009 Sunplus Core Technology Co., Ltd.
 *  Chen Liqin <liqin.chen@sunplusct.com>
 *  Lennox Wu <lennox.wu@sunplusct.com>
 * Copyright (C) 2012 Regents of the University of California
 * Copyright (C) 2020 FORTH-ICS/CARV
 *  Nick Kossifidis <mick@ics.forth.gr>
 */

// Linux and architecture headers are supplied by the surrounding kernel bindings.

/// The lucky hart to first increment this variable will boot the other cores.
/// This is used before the kernel initializes the BSS so it can't be in the BSS.
#[no_mangle]
pub static mut hart_lottery: atomic_t = atomic_t { counter: 0 };
#[no_mangle]
pub static mut boot_cpu_hartid: c_ulong = 0;

static mut kimage_res: resource = resource { name: c_str!("Kernel image"), ..resource::default() };
static mut code_res: resource = resource { name: c_str!("Kernel code"), ..resource::default() };
static mut data_res: resource = resource { name: c_str!("Kernel data"), ..resource::default() };
static mut rodata_res: resource = resource { name: c_str!("Kernel rodata"), ..resource::default() };
static mut bss_res: resource = resource { name: c_str!("Kernel bss"), ..resource::default() };
// CONFIG_CRASH_DUMP
static mut elfcorehdr_res: resource = resource { name: c_str!("ELF Core hdr"), ..resource::default() };

static mut num_standard_resources: c_int = 0;
static mut standard_resources: *mut resource = core::ptr::null_mut();

unsafe fn add_resource(parent: *mut resource, res: *mut resource) -> c_int {
    let ret = insert_resource(parent, res);
    if ret < 0 { pr_err!("Failed to add resource %s %pR\n", (*res).name, res); }
    ret
}

unsafe fn add_kernel_resources() -> c_int {
    (*core::ptr::addr_of_mut!(code_res)).start = __pa_symbol(_text);
    code_res.end = __pa_symbol(_etext) - 1;
    code_res.flags = IORESOURCE_SYSTEM_RAM | IORESOURCE_BUSY;
    rodata_res.start = __pa_symbol(__start_rodata);
    rodata_res.end = __pa_symbol(__end_rodata) - 1;
    rodata_res.flags = IORESOURCE_SYSTEM_RAM | IORESOURCE_BUSY;
    data_res.start = __pa_symbol(_data);
    data_res.end = __pa_symbol(_edata) - 1;
    data_res.flags = IORESOURCE_SYSTEM_RAM | IORESOURCE_BUSY;
    bss_res.start = __pa_symbol(__bss_start);
    bss_res.end = __pa_symbol(__bss_stop) - 1;
    bss_res.flags = IORESOURCE_SYSTEM_RAM | IORESOURCE_BUSY;
    kimage_res.start = code_res.start;
    kimage_res.end = bss_res.end;
    kimage_res.flags = IORESOURCE_SYSTEM_RAM | IORESOURCE_BUSY;
    let mut ret = add_resource(&mut iomem_resource, &mut kimage_res);
    if ret < 0 { return ret; }
    ret = add_resource(&mut kimage_res, &mut code_res); if ret < 0 { return ret; }
    ret = add_resource(&mut kimage_res, &mut rodata_res); if ret < 0 { return ret; }
    ret = add_resource(&mut kimage_res, &mut data_res); if ret < 0 { return ret; }
    add_resource(&mut kimage_res, &mut bss_res)
}

unsafe fn init_resources() {
    let num_resources = memblock.memory.cnt + memblock.reserved.cnt + 1;
    let mut res_idx = num_resources - 1;
    let mem_res_sz = num_resources * core::mem::size_of::<resource>();
    let mem_res = memblock_alloc_or_panic(mem_res_sz, SMP_CACHE_BYTES) as *mut resource;
    if add_kernel_resources() < 0 { release_child_resources(&mut iomem_resource); memblock_free(mem_res as *mut _, mem_res_sz); return; }
    // CONFIG_CRASH_DUMP
    if elfcorehdr_size > 0 {
        elfcorehdr_res.start = elfcorehdr_addr;
        elfcorehdr_res.end = elfcorehdr_addr + elfcorehdr_size - 1;
        elfcorehdr_res.flags = IORESOURCE_SYSTEM_RAM | IORESOURCE_BUSY;
        add_resource(&mut iomem_resource, &mut elfcorehdr_res);
    }
    for_each_reserved_mem_region!(region => {
        let res = &mut *mem_res.add(res_idx as usize); res_idx -= 1;
        res.name = c_str!("Reserved"); res.flags = IORESOURCE_MEM | IORESOURCE_EXCLUSIVE;
        res.start = __pfn_to_phys(memblock_region_reserved_base_pfn(region));
        res.end = __pfn_to_phys(memblock_region_reserved_end_pfn(region)) - 1;
        if memblock_is_memory(res.start) { res_idx += 1; continue; }
        if add_resource(&mut iomem_resource, res) < 0 { break; }
    });
    let mut non_resv_res = 0;
    for_each_mem_region!(region => {
        let res = &mut *mem_res.add(res_idx as usize); res_idx -= 1; non_resv_res += 1;
        if unlikely(memblock_is_nomap(region)) { res.name = c_str!("Reserved"); res.flags = IORESOURCE_MEM | IORESOURCE_EXCLUSIVE; }
        else { res.name = c_str!("System RAM"); res.flags = IORESOURCE_SYSTEM_RAM | IORESOURCE_BUSY; }
        res.start = __pfn_to_phys(memblock_region_memory_base_pfn(region));
        res.end = __pfn_to_phys(memblock_region_memory_end_pfn(region)) - 1;
        add_resource(&mut iomem_resource, res);
    });
    num_standard_resources = non_resv_res;
    standard_resources = mem_res.add((res_idx + 1) as usize);
    if res_idx >= 0 { memblock_free(mem_res as *mut _, (res_idx + 1) as usize * core::mem::size_of::<resource>()); }
}

unsafe fn reserve_memblock_reserved_regions() -> c_int {
    for i in 0..num_standard_resources {
        let mem = &mut *standard_resources.add(i as usize);
        let mem_size = resource_size(mem);
        if !memblock_is_region_reserved(mem.start, mem_size) { continue; }
        for_each_reserved_mem_range!(j, r_start, r_end => {
            let start = max(PFN_PHYS(PFN_DOWN(r_start)), mem.start);
            let end = min(PFN_PHYS(PFN_UP(r_end)) - 1, mem.end);
            if start <= mem.end && end >= mem.start { reserve_region_with_split(mem, start, end, c_str!("Reserved")); }
        });
    }
    0
}

unsafe fn parse_dtb() {
    if early_init_dt_scan(dtb_early_va, dtb_early_pa) {
        let name = of_flat_dt_get_machine_name();
        if !name.is_null() { pr_info!("Machine model: %s\n", name); dump_stack_set_arch_desc(c_str!("%s (DT)"), name); }
    } else { pr_err!("No DTB passed to the kernel\n"); }
}

unsafe fn riscv_spinlock_init() {
    let mut using_ext: *const c_char = core::ptr::null();
    if IS_ENABLED!(CONFIG_RISCV_TICKET_SPINLOCKS) { pr_info!("Ticket spinlock: enabled\n"); return; }
    if IS_ENABLED!(CONFIG_RISCV_ISA_ZABHA) && IS_ENABLED!(CONFIG_RISCV_ISA_ZACAS) && IS_ENABLED!(CONFIG_TOOLCHAIN_HAS_ZACAS) && riscv_isa_extension_available(core::ptr::null_mut(), ZABHA) && riscv_isa_extension_available(core::ptr::null_mut(), ZACAS) { using_ext = c_str!("using Zabha"); }
    else if riscv_isa_extension_available(core::ptr::null_mut(), ZICCRSE) { using_ext = c_str!("using Ziccrse"); }
    // CONFIG_RISCV_COMBO_SPINLOCKS: disable queued spinlocks when no extension is available.
    if using_ext.is_null() { pr_err!("Queued spinlock without Zabha or Ziccrse"); } else { pr_info!("Queued spinlock %s: enabled\n", using_ext); }
}

unsafe extern "C" { fn init_rt_signal_env(); }

pub unsafe fn setup_arch(cmdline_p: *mut *mut c_char) {
    parse_dtb(); setup_initial_init_mm(_stext, _etext, _edata, _end); *cmdline_p = boot_command_line;
    early_ioremap_setup(); sbi_init(); jump_label_init(); parse_early_param(); efi_init(); paging_init(); acpi_table_upgrade(); acpi_boot_table_init();
    if acpi_disabled { unflatten_device_tree(); }
    misc_mem_init(); init_resources();
    // CONFIG_KASAN
    // CONFIG_SMP
    if !acpi_disabled { acpi_init_rintc_map(); acpi_map_cpus_to_nodes(); }
    riscv_init_cbo_blocksizes(); riscv_fill_hwcap(); apply_boot_alternatives(); init_rt_signal_env();
    if IS_ENABLED!(CONFIG_RISCV_ISA_ZICBOM) && riscv_isa_extension_available(core::ptr::null_mut(), ZICBOM) { riscv_noncoherent_supported(); }
    riscv_set_dma_cache_alignment(); riscv_user_isa_enable(); riscv_spinlock_init();
    if !IS_ENABLED!(CONFIG_RISCV_ISA_ZBB) || !riscv_isa_extension_available(core::ptr::null_mut(), ZBB) { static_branch_disable(&mut efficient_ffs_key); }
}

pub unsafe fn arch_cpu_is_hotpluggable(cpu: c_int) -> bool { cpu_has_hotplug(cpu) }

pub unsafe fn free_initmem() {
    if IS_ENABLED!(CONFIG_STRICT_KERNEL_RWX) { set_kernel_memory(lm_alias(__init_begin), lm_alias(__init_end), set_memory_rw_nx); if IS_ENABLED!(CONFIG_64BIT) { set_kernel_memory(__init_begin, __init_end, set_memory_nx); } }
    free_initmem_default(POISON_FREE_INITMEM);
}

unsafe fn dump_kernel_offset(_self: *mut notifier_block, _v: c_ulong, _p: *mut c_void) -> c_int { pr_emerg!("Kernel Offset: 0x%lx from 0x%lx\n", kernel_map.virt_offset, KERNEL_LINK_ADDR); 0 }
static mut kernel_offset_notifier: notifier_block = notifier_block { notifier_call: Some(dump_kernel_offset), ..notifier_block::default() };

unsafe fn register_kernel_offset_dumper() -> c_int {
    if IS_ENABLED!(CONFIG_RANDOMIZE_BASE) { atomic_notifier_chain_register(&mut panic_notifier_list, &mut kernel_offset_notifier); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
