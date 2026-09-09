// SPDX-License-Identifier: GPL-2.0-only
/* Architecture-dependent early kernel initialization. */

// Symbols and types supplied by the surrounding kernel are intentionally left external.

pub static mut max_low_pfn_mapped: c_ulong = 0;
pub static mut max_pfn_mapped: c_ulong = 0;
pub static mut _brk_start: c_ulong = __brk_base as c_ulong;
pub static mut _brk_end: c_ulong = __brk_base as c_ulong;
pub static mut boot_params: boot_params = boot_params::default();

static mut rodata_resource: resource = resource { name: b"Kernel rodata\0".as_ptr() as *const c_char, start: 0, end: 0, flags: IORESOURCE_BUSY | IORESOURCE_SYSTEM_RAM };
static mut data_resource: resource = resource { name: b"Kernel data\0".as_ptr() as *const c_char, start: 0, end: 0, flags: IORESOURCE_BUSY | IORESOURCE_SYSTEM_RAM };
static mut code_resource: resource = resource { name: b"Kernel code\0".as_ptr() as *const c_char, start: 0, end: 0, flags: IORESOURCE_BUSY | IORESOURCE_SYSTEM_RAM };
static mut bss_resource: resource = resource { name: b"Kernel bss\0".as_ptr() as *const c_char, start: 0, end: 0, flags: IORESOURCE_BUSY | IORESOURCE_SYSTEM_RAM };

pub static mut boot_cpu_data: cpuinfo_x86 = cpuinfo_x86::default();
pub static mut mmu_cr4_features: c_ulong = 0;
pub static mut bootloader_type: c_int = 0;
pub static mut bootloader_version: c_int = 0;
pub static mut sysfb_primary_display: sysfb_display_info = sysfb_display_info::default();
extern "C" { static mut root_mountflags: c_int; }
pub static mut saved_video_mode: c_ulong = 0;
const RAMDISK_IMAGE_START_MASK: u16 = 0x07ff;
const RAMDISK_PROMPT_FLAG: u16 = 0x8000;
const RAMDISK_LOAD_FLAG: u16 = 0x4000;
static mut command_line: [c_char; COMMAND_LINE_SIZE] = [0; COMMAND_LINE_SIZE];

#[inline]
unsafe fn copy_edd() {
    #[cfg(feature = "CONFIG_EDD")]
    { memcpy((*(&mut edd)).mbr_signature.as_mut_ptr() as *mut c_void, boot_params.edd_mbr_sig_buffer.as_ptr() as *const c_void, size_of_val(&edd.mbr_signature)); memcpy(edd.edd_info.as_mut_ptr() as *mut c_void, boot_params.eddbuf.as_ptr() as *const c_void, size_of_val(&edd.edd_info)); edd.mbr_signature_nr = boot_params.edd_mbr_sig_buf_entries; edd.edd_info_nr = boot_params.eddbuf_entries; }
}

pub unsafe fn extend_brk(size: usize, align: usize) -> *mut c_void {
    let mask = align - 1;
    BUG_ON(_brk_start == 0); BUG_ON(align & mask != 0);
    _brk_end = (_brk_end + mask) & !mask;
    BUG_ON((_brk_end + size) as *mut c_char > __brk_limit as *mut c_char);
    let ret = _brk_end as *mut c_void; _brk_end += size; memset(ret, 0, size); ret
}

unsafe fn cleanup_highmap() {}

unsafe fn reserve_brk() {
    if _brk_end > _brk_start { memblock_reserve_kern(__pa_symbol(_brk_start), _brk_end - _brk_start); }
    _brk_start = 0;
}

unsafe fn get_ramdisk_image() -> u64 { let mut v = boot_params.hdr.ramdisk_image as u64 | ((boot_params.ext_ramdisk_image as u64) << 32); if v == 0 { v = phys_initrd_start; } v }
unsafe fn get_ramdisk_size() -> u64 { let mut v = boot_params.hdr.ramdisk_size as u64 | ((boot_params.ext_ramdisk_size as u64) << 32); if v == 0 { v = phys_initrd_size; } v }

unsafe fn relocate_initrd() {
    let image = get_ramdisk_image(); let size = get_ramdisk_size(); let area = PAGE_ALIGN(size);
    let relocated = memblock_phys_alloc_range(area, PAGE_SIZE, 0, PFN_PHYS(max_pfn_mapped));
    if relocated == 0 { panic!("Cannot find place for new RAMDISK of size %lld\n", size); }
    initrd_start = relocated + PAGE_OFFSET; initrd_end = initrd_start + size;
    printk(KERN_INFO, "Allocated new RAMDISK: [mem %#010llx-%#010llx]\n", relocated, relocated + size - 1);
    if copy_from_early_mem(initrd_start as *mut c_void, image, size) != 0 { panic!("Copy RAMDISK failed\n"); }
    printk(KERN_INFO, "Move RAMDISK from [mem %#010llx-%#010llx] to [mem %#010llx-%#010llx]\n", image, image + size - 1, relocated, relocated + size - 1);
}

unsafe fn early_reserve_initrd() {
    let image = get_ramdisk_image(); let size = get_ramdisk_size(); let end = PAGE_ALIGN(image + size);
    if boot_params.hdr.type_of_loader == 0 || image == 0 || size == 0 { return; }
    memblock_reserve_kern(image, end - image);
}
unsafe fn reserve_initrd() {
    let image = get_ramdisk_image(); let size = get_ramdisk_size(); let end = PAGE_ALIGN(image + size);
    if boot_params.hdr.type_of_loader == 0 || image == 0 || size == 0 { return; }
    initrd_start = 0; printk(KERN_INFO, "RAMDISK: [mem %#010llx-%#010llx]\n", image, end - 1);
    if pfn_range_is_mapped(PFN_DOWN(image), PFN_DOWN(end)) { initrd_start = image + PAGE_OFFSET; initrd_end = initrd_start + size; return; }
    relocate_initrd(); memblock_phys_free(image, end - image);
}

unsafe fn add_early_ima_buffer(phys_addr: u64) {
    #[cfg(feature = "CONFIG_IMA")]
    { let data = early_memremap(phys_addr + size_of::<setup_data>() as u64, size_of::<ima_setup_data>()); if data.is_null() { pr_warn!("setup: failed to memremap ima_setup_data entry\n"); return; } let d = &mut *(data as *mut ima_setup_data); if d.size != 0 { memblock_reserve_kern(d.addr, d.size); ima_kexec_buffer_phys = d.addr; ima_kexec_buffer_size = d.size as usize; } early_memunmap(data, size_of::<ima_setup_data>()); }
    #[cfg(not(feature = "CONFIG_IMA"))] pr_warn!("Passed IMA kexec data, but CONFIG_IMA not set. Ignoring.\n");
}

unsafe fn add_kho(phys_addr: u64, data_len: u32) {
    if !IS_ENABLED(CONFIG_KEXEC_HANDOVER) { pr_warn!("Passed KHO data, but CONFIG_KEXEC_HANDOVER not set. Ignoring.\n"); return; }
    let addr = phys_addr + size_of::<setup_data>() as u64; let size = data_len as u64 - size_of::<setup_data>() as u64;
    let p = early_memremap(addr, size as usize); if p.is_null() { pr_warn!("setup: failed to memremap kho data (0x%llx, 0x%llx)\n", addr, size); return; }
    let kho = &*(p as *const kho_data); kho_populate(kho.fdt_addr, kho.fdt_size, kho.scratch_addr, kho.scratch_size); early_memunmap(p, size as usize);
}

unsafe fn parse_setup_data() {
    let mut pa = boot_params.hdr.setup_data;
    while pa != 0 { let p = early_memremap(pa, size_of::<setup_data>()); let d = &*(p as *const setup_data); let len = d.len + size_of::<setup_data>() as u32; let typ = d.type_; let next = d.next; early_memunmap(p, size_of::<setup_data>());
        match typ { SETUP_E820_EXT => e820__memory_setup_extended(pa, len), SETUP_DTB => add_dtb(pa), SETUP_EFI => parse_efi_setup(pa, len), SETUP_IMA => add_early_ima_buffer(pa), SETUP_KEXEC_KHO => add_kho(pa, len), SETUP_RNG_SEED => { let q = early_memremap(pa, len as usize); let x = &mut *(q as *mut setup_data); add_bootloader_randomness(x.data.as_ptr() as *const c_void, x.len as usize); memzero_explicit(x.data.as_mut_ptr() as *mut c_void, x.len as usize); memzero_explicit(&mut x.len as *mut _ as *mut c_void, size_of_val(&x.len)); early_memunmap(q, len as usize); }, _ => {} } pa = next;
    }
}

unsafe fn parse_boot_params() {
    ROOT_DEV = old_decode_dev(boot_params.hdr.root_dev); sysfb_primary_display.screen = boot_params.screen_info;
    saved_video_mode = boot_params.hdr.vid_mode; bootloader_type = boot_params.hdr.type_of_loader;
    if bootloader_type >> 4 == 0xe { bootloader_type = (bootloader_type & 0xf) | ((boot_params.hdr.ext_loader_type + 0x10) << 4); }
    bootloader_version = (bootloader_type & 0xf) | (boot_params.hdr.ext_loader_ver << 4);
    if boot_params.hdr.root_flags == 0 { root_mountflags &= !MS_RDONLY; }
}

unsafe fn memblock_x86_reserve_range_setup_data() { let mut pa = boot_params.hdr.setup_data; while pa != 0 { let p = early_memremap(pa, size_of::<setup_data>()); if p.is_null() { pr_warn!("setup: failed to memremap setup_data entry\n"); return; } let d = &*(p as *const setup_data); let next = d.next; memblock_reserve_kern(pa, size_of::<setup_data>() as u64 + d.len as u64); early_memunmap(p, size_of::<setup_data>()); pa = next; } }

unsafe fn arch_reserve_crashkernel() { if !IS_ENABLED(CONFIG_CRASH_RESERVE) { return; } let mut base=0u64; let mut size=0u64; let mut low=0u64; let mut cma=0u64; let mut high=false; if parse_crashkernel(boot_command_line, memblock_phys_mem_size(), &mut size, &mut base, &mut low, &mut cma, &mut high) != 0 { return; } if xen_pv_domain() { pr_info!("Ignoring crashkernel for a Xen PV domain\n"); return; } reserve_crashkernel_generic(size, base, low, high); reserve_crashkernel_cma(cma); }

pub unsafe fn reserve_standard_io_resources() { for r in standard_io_resources.iter_mut() { request_resource(&mut ioport_resource, r); } }
static mut standard_io_resources: [resource; 10] = [resource::default(); 10];

unsafe fn setup_kernel_resources() { code_resource.start=__pa_symbol(_text); code_resource.end=__pa_symbol(_etext)-1; rodata_resource.start=__pa_symbol(__start_rodata); rodata_resource.end=__pa_symbol(__end_rodata)-1; data_resource.start=__pa_symbol(_sdata); data_resource.end=__pa_symbol(_edata)-1; bss_resource.start=__pa_symbol(__bss_start); bss_resource.end=__pa_symbol(__bss_stop)-1; insert_resource(&mut iomem_resource,&mut code_resource); insert_resource(&mut iomem_resource,&mut rodata_resource); insert_resource(&mut iomem_resource,&mut data_resource); insert_resource(&mut iomem_resource,&mut bss_resource); }

unsafe fn snb_gfx_workaround_needed() -> bool { if !early_pci_allowed() { return false; } if read_pci_config_16(0,2,0,PCI_VENDOR_ID)!=0x8086 { return false; } [0x0102,0x0112,0x0122,0x0106,0x0116,0x0126,0x010a].contains(&read_pci_config_16(0,2,0,PCI_DEVICE_ID)) }
unsafe fn trim_snb_memory() { if !snb_gfx_workaround_needed() { return; } for p in [0x20050000,0x20110000,0x20130000,0x20138000,0x40004000] { if memblock_reserve(p,PAGE_SIZE)!=0 { printk(KERN_WARNING,"failed to reserve 0x%08lx\n",p); } } }
unsafe fn trim_bios_range() { e820__range_update(0,PAGE_SIZE,E820_TYPE_RAM,E820_TYPE_RESERVED); e820__range_remove(BIOS_BEGIN,BIOS_END-BIOS_BEGIN,E820_TYPE_RAM); e820__update_table(&mut e820_table); }
unsafe fn e820_add_kernel_range() { let start=__pa_symbol(_text); let size=__pa_symbol(_end)-start; if !e820__mapped_all(start,start+size,E820_TYPE_RAM) { pr_warn!(".text .data .bss are not marked as E820_TYPE_RAM!\n"); e820__range_remove(start,size,0); e820__range_add(start,size,E820_TYPE_RAM); } }
unsafe fn early_reserve_memory() { memblock_reserve_kern(__pa_symbol(_text), __end_of_kernel_reserve as u64 - _text as u64); memblock_reserve(0,SZ_64K); early_reserve_initrd(); memblock_x86_reserve_range_setup_data(); reserve_bios_regions(); trim_snb_memory(); }

pub unsafe fn x86_configure_nx() { if boot_cpu_has(X86_FEATURE_NX) { __supported_pte_mask |= _PAGE_NX; } else { __supported_pte_mask &= !_PAGE_NX; } }
unsafe fn x86_report_nx() { if !boot_cpu_has(X86_FEATURE_NX) { printk(KERN_NOTICE,"Notice: NX (Execute Disable) protection missing in CPU!\n"); } else { printk(KERN_INFO,"NX (Execute Disable) protection: active\n"); } }

pub unsafe fn setup_arch(cmdline_p: *mut *mut c_char) {
    printk(KERN_INFO,"Command line: %s\n",boot_command_line); boot_cpu_data.x86_phys_bits=MAX_PHYSMEM_BITS;
    strscpy(command_line.as_mut_ptr(),boot_command_line,COMMAND_LINE_SIZE); *cmdline_p=command_line.as_mut_ptr();
    olpc_ofw_detect(); idt_setup_early_traps(); early_cpu_init(); jump_label_init(); static_call_init(); early_ioremap_init(); setup_olpc_ofw_pgd(); parse_boot_params(); x86_init.oem.arch_setup(); early_reserve_memory(); iomem_resource.end=(1u64<<boot_cpu_data.x86_phys_bits)-1; e820__memory_setup(); parse_setup_data(); copy_edd(); setup_initial_init_mm(_text,_etext,_edata,_brk_end as *mut c_void); x86_configure_nx(); parse_early_param(); if efi_enabled(EFI_BOOT) { efi_memblock_x86_reserve_range(); } x86_report_nx(); apic_setup_apic_calls(); e820__finish_early_params(); if efi_enabled(EFI_BOOT){efi_init();} reserve_ibft_region(); x86_init.resources.dmi_setup(); init_hypervisor_platform(); tsc_early_init(); x86_init.resources.probe_roms(); setup_kernel_resources(); e820_add_kernel_range(); trim_bios_range(); max_pfn=e820__end_of_ram_pfn(); cache_bp_init(); if mtrr_trim_uncached_memory(max_pfn){max_pfn=e820__end_of_ram_pfn();} max_possible_pfn=max_pfn; kernel_randomize_memory(); check_x2apic(); max_low_pfn=if max_pfn>(1u64<<(32-PAGE_SHIFT)){e820__end_of_low_ram_pfn()}else{max_pfn}; x86_init.mpparse.find_mptable(); early_alloc_pgt_buf(); reserve_brk(); cleanup_highmap(); e820__memblock_setup(); mem_encrypt_setup_arch(); cc_random_init(); efi_find_mirror(); efi_esrt_init(); efi_mokvar_table_init(); efi_reserve_boot_services(); e820__memblock_alloc_reserved_mpc_new(); x86_platform.realmode_reserve(); init_mem_mapping(); cpu_init_replace_early_idt(); mmu_cr4_features=__read_cr4()&!X86_CR4_PCIDE; memblock_set_current_limit(get_max_mapped()); setup_log_buf(1); reserve_initrd(); acpi_table_upgrade(); acpi_boot_table_init(); vsmp_init(); io_delay_init(); early_platform_quirks(); early_acpi_boot_init(); x86_init.mpparse.early_parse_smp_cfg(); x86_flattree_get_config(); initmem_init(); dma_contiguous_reserve(max_pfn_mapped<<PAGE_SHIFT); arch_reserve_crashkernel(); if !early_xdbc_setup_hardware(){early_xdbc_register_console();} x86_init.paging.pagetable_init(); kasan_init(); sync_initial_page_table(); tboot_probe(); map_vsyscall(); x86_32_probe_apic(); early_quirks(); topology_apply_cmdline_limits_early(); acpi_boot_init(); x86_init.mpparse.parse_smp_cfg(); init_apic_mappings(); topology_init_possible_cpus(); init_cpu_to_node(); init_gi_nodes(); io_apic_init_mappings(); x86_init.hyper.guest_late_init(); e820__reserve_resources(); e820__register_nosave_regions(max_pfn); x86_init.resources.reserve_resources(); e820__setup_pci_gap(); x86_init.oem.banner(); x86_init.timers.wallclock_init(); therm_lvt_init(); mcheck_init(); register_refined_jiffies(PIT_TICK_RATE); unwind_init();
}

#[cfg(feature = "CONFIG_X86_32")]
pub unsafe fn i386_reserve_resources() { request_resource(&mut iomem_resource,&mut video_ram_resource); reserve_standard_io_resources(); }
#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
pub fn arch_cpu_is_hotpluggable(cpu: c_int) -> bool { cpu > 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
