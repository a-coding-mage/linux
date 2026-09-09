// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/setup.c
 *
 * This file handles the architecture-dependent parts of initialization
 *
 *  Copyright (C) 1999  Niibe Yutaka
 *  Copyright (C) 2002 - 2010 Paul Mundt
 */

// C headers and build-time configuration symbols are supplied by the
// surrounding kernel translation unit.

pub static mut cpu_data: [sh_cpuinfo; NR_CPUS] = {
    let mut data = [sh_cpuinfo::default(); NR_CPUS];
    data[0] = sh_cpuinfo {
        type_: CPU_SH_NONE,
        family: CPU_FAMILY_UNKNOWN,
        loops_per_jiffy: 10000000,
        phys_bits: MAX_PHYSMEM_BITS,
        ..sh_cpuinfo::default()
    };
    data
};

pub static mut sh_mv: sh_machine_vector = sh_machine_vector { mv_name: "generic", ..sh_machine_vector::default() };

extern "C" {
    static mut root_mountflags: c_int;
}

const RAMDISK_IMAGE_START_MASK: c_ulong = 0x07FF;
const RAMDISK_PROMPT_FLAG: c_ulong = 0x8000;
const RAMDISK_LOAD_FLAG: c_ulong = 0x4000;

static mut command_line: [c_char; COMMAND_LINE_SIZE] = [0; COMMAND_LINE_SIZE];

static mut code_resource: resource = resource {
    name: "Kernel code",
    flags: IORESOURCE_BUSY | IORESOURCE_SYSTEM_RAM,
    ..resource::default()
};
static mut data_resource: resource = resource {
    name: "Kernel data",
    flags: IORESOURCE_BUSY | IORESOURCE_SYSTEM_RAM,
    ..resource::default()
};
static mut bss_resource: resource = resource {
    name: "Kernel bss",
    flags: IORESOURCE_BUSY | IORESOURCE_SYSTEM_RAM,
    ..resource::default()
};

pub static mut memory_start: c_ulong = 0;
pub static mut memory_end: c_ulong = 0;
static mut memory_limit: c_ulong = 0;
static mut mem_resources: [resource; MAX_NUMNODES] = [resource::default(); MAX_NUMNODES];

pub static mut l1i_cache_shape: c_int = 0;
pub static mut l1d_cache_shape: c_int = 0;
pub static mut l2_cache_shape: c_int = 0;

unsafe extern "C" fn early_parse_mem(mut p: *mut c_char) -> c_int {
    if p.is_null() { return 1; }
    memory_limit = PAGE_ALIGN(memparse(p, &mut p));
    pr_notice!("Memory limited to %ldMB\n", memory_limit >> 20);
    0
}

pub unsafe extern "C" fn check_for_initrd() {
    #[cfg(CONFIG_BLK_DEV_INITRD)]
    {
        let (mut start, mut end): (c_ulong, c_ulong);
        if LOADER_TYPE == 0 || INITRD_START == 0 || INITRD_SIZE == 0 { initrd_disabled!(); return; }
        start = INITRD_START + __MEMORY_START;
        end = start + INITRD_SIZE;
        if end <= start { initrd_disabled!(); return; }
        if start & !PAGE_MASK != 0 { pr_err!("initrd must be page aligned\n"); initrd_disabled!(); return; }
        if start < __MEMORY_START { pr_err!("initrd start (%08lx) < __MEMORY_START(%x)\n", start, __MEMORY_START); initrd_disabled!(); return; }
        if end > memblock_end_of_DRAM() { pr_err!("initrd extends beyond end of memory (0x%08lx > 0x%08lx)\ndisabling initrd\n", end, memblock_end_of_DRAM() as c_ulong); initrd_disabled!(); return; }
        ROOT_DEV = Root_RAM0;
        initrd_start = __va(start) as c_ulong;
        initrd_end = initrd_start + INITRD_SIZE;
        memblock_reserve(__pa(initrd_start), INITRD_SIZE);
    }
}

#[cfg(not(CONFIG_GENERIC_CALIBRATE_DELAY))]
pub unsafe extern "C" fn calibrate_delay() {
    let clk = clk_get(core::ptr::null_mut(), "cpu_clk");
    if IS_ERR(clk) { panic!("Need a sane CPU clock definition!"); }
    loops_per_jiffy = (clk_get_rate(clk) >> 1) / HZ;
    printk!(KERN_INFO "Calibrating delay loop (skipped)... %lu.%02lu BogoMIPS PRESET (lpj=%lu)\n", loops_per_jiffy/(500000/HZ), (loops_per_jiffy/(5000/HZ)) % 100, loops_per_jiffy);
}

pub unsafe extern "C" fn __add_active_range(nid: c_uint, start_pfn: c_ulong, end_pfn: c_ulong) {
    let res = &mut mem_resources[nid as usize];
    WARN_ON!(!res.name.is_null());
    let start = start_pfn << PAGE_SHIFT;
    let end = end_pfn << PAGE_SHIFT;
    res.name = "System RAM"; res.start = start; res.end = end - 1; res.flags = IORESOURCE_SYSTEM_RAM | IORESOURCE_BUSY;
    if request_resource(&mut iomem_resource, res) != 0 { pr_err!("unable to request memory_resource 0x%lx 0x%lx\n", start_pfn, end_pfn); return; }
    request_resource(res, &mut code_resource); request_resource(res, &mut data_resource); request_resource(res, &mut bss_resource);
    #[cfg(CONFIG_CRASH_RESERVE)] request_resource(res, &mut crashk_res);
    pmb_bolt_mapping(__va(start), start, end - start, PAGE_KERNEL);
    memblock_set_node(PFN_PHYS(start_pfn), PFN_PHYS(end_pfn - start_pfn), &mut memblock.memory, nid);
}

pub unsafe extern "C" fn plat_early_device_setup() {}

#[cfg(CONFIG_OF_EARLY_FLATTREE)]
pub unsafe extern "C" fn sh_fdt_init(dt_phys: phys_addr_t) {
    static mut done: c_int = 0;
    if done != 0 { return; }
    let dt_virt = {
        #[cfg(CONFIG_BUILTIN_DTB)] { __dtb_start }
        #[cfg(not(CONFIG_BUILTIN_DTB))] { phys_to_virt(dt_phys) }
    };
    if dt_virt.is_null() || early_init_dt_scan(dt_virt, __pa(dt_virt)) == 0 {
        pr_crit!("Error: invalid device tree blob at physical address %p\n", dt_phys as *mut c_void);
        loop { cpu_relax(); }
    }
    done = 1;
}

pub unsafe extern "C" fn setup_arch(cmdline_p: *mut *mut c_char) {
    enable_mmu(); ROOT_DEV = old_decode_dev(ORIG_ROOT_DEV);
    printk!(KERN_NOTICE "Boot params:\n... MOUNT_ROOT_RDONLY - %08lx\n... RAMDISK_FLAGS     - %08lx\n... ORIG_ROOT_DEV     - %08lx\n... LOADER_TYPE       - %08lx\n... INITRD_START      - %08lx\n... INITRD_SIZE       - %08lx\n", MOUNT_ROOT_RDONLY, RAMDISK_FLAGS, ORIG_ROOT_DEV, LOADER_TYPE, INITRD_START, INITRD_SIZE);
    #[cfg(CONFIG_BLK_DEV_RAM)] { rd_image_start = RAMDISK_FLAGS & RAMDISK_IMAGE_START_MASK; }
    if MOUNT_ROOT_RDONLY == 0 { root_mountflags &= !MS_RDONLY; }
    setup_initial_init_mm(_text, _etext, _edata, _end);
    code_resource.start = virt_to_phys(_text); code_resource.end = virt_to_phys(_etext) - 1;
    data_resource.start = virt_to_phys(_etext); data_resource.end = virt_to_phys(_edata) - 1;
    bss_resource.start = virt_to_phys(__bss_start); bss_resource.end = virt_to_phys(__bss_stop) - 1;
    #[cfg(CONFIG_CMDLINE_OVERWRITE)] strscpy(&mut command_line, CONFIG_CMDLINE, COMMAND_LINE_SIZE);
    #[cfg(not(CONFIG_CMDLINE_OVERWRITE))] { strscpy(&mut command_line, COMMAND_LINE, COMMAND_LINE_SIZE); #[cfg(CONFIG_CMDLINE_EXTEND)] { strlcat(&mut command_line, " ", COMMAND_LINE_SIZE); strlcat(&mut command_line, CONFIG_CMDLINE, COMMAND_LINE_SIZE); } }
    memcpy(boot_command_line, command_line.as_ptr(), COMMAND_LINE_SIZE); *cmdline_p = command_line.as_mut_ptr();
    parse_early_param(); plat_early_device_setup(); sh_mv_setup(); sh_early_platform_driver_probe("earlyprintk", 1, 1);
    #[cfg(CONFIG_OF_EARLY_FLATTREE)] { #[cfg(CONFIG_BUILTIN_DTB)] unflatten_and_copy_device_tree(); #[cfg(not(CONFIG_BUILTIN_DTB))] unflatten_device_tree(); }
    paging_init(); if let Some(f) = sh_mv.mv_setup { f(cmdline_p); } plat_smp_setup();
}

pub unsafe extern "C" fn generic_mode_pins() -> c_int { pr_warn!("generic_mode_pins(): missing mode pin configuration\n"); 0 }
pub unsafe extern "C" fn test_mode_pin(pin: c_int) -> c_int { sh_mv.mv_mode_pins() & pin }

pub unsafe extern "C" fn arch_cpu_finalize_init() {
    let mut p = init_utsname().machine.as_mut_ptr().add(2); select_idle_routine(); current_cpu_data.loops_per_jiffy = loops_per_jiffy;
    match current_cpu_data.family { CPU_FAMILY_SH2 => { *p = b'2' as c_char; p = p.add(1); }, CPU_FAMILY_SH2A => { *p = b'2' as c_char; *p.add(1) = b'a' as c_char; p = p.add(2); }, CPU_FAMILY_SH3 => { *p = b'3' as c_char; p = p.add(1); }, CPU_FAMILY_SH4 => { *p = b'4' as c_char; p = p.add(1); }, CPU_FAMILY_SH4A => { *p = b'4' as c_char; *p.add(1) = b'a' as c_char; p = p.add(2); }, CPU_FAMILY_SH4AL_DSP => { for c in b"4al-dsp" { *p = *c as c_char; p = p.add(1); } }, CPU_FAMILY_UNKNOWN => {} }
    pr_info!("CPU: %s\n", get_cpu_subtype(&current_cpu_data));
    #[cfg(not(__LITTLE_ENDIAN__))] { *p = b'e' as c_char; *p.add(1) = b'b' as c_char; p = p.add(2); }
    *p = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
