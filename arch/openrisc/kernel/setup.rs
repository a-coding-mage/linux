// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OpenRISC setup.c
 *
 * Linux architectural port borrowing liberally from similar works of
 * others. All original copyrights apply as per the original source
 * declaration.
 *
 * Modifications for the OpenRISC architecture:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 *
 * This file handles the architecture-dependent parts of initialization
 */

// Linux and architecture header dependencies are supplied by the surrounding kernel.

unsafe fn setup_memory() {
    let mut ram_start_pfn: c_ulong;
    let mut ram_end_pfn: c_ulong;
    let mut memory_start: phys_addr_t;
    let mut memory_end: phys_addr_t;

    memory_end = 0;
    memory_start = memory_end;

    /* Find main memory where is the kernel, we assume its the only one */
    memory_start = memblock_start_of_DRAM();
    memory_end = memblock_end_of_DRAM();

    if memory_end == 0 {
        panic("No memory!");
    }

    ram_start_pfn = PFN_UP(memory_start);
    ram_end_pfn = PFN_DOWN(memblock_end_of_DRAM());

    /* setup bootmem globals (we use no_bootmem, but mm still depends on this) */
    min_low_pfn = ram_start_pfn;
    max_low_pfn = ram_end_pfn;
    max_pfn = ram_end_pfn;

    /*
     * initialize the boot-time allocator (with low memory only).
     *
     * This makes the memory from the end of the kernel to the end of
     * RAM usable.
     */
    memblock_reserve(__pa(_stext), _end - _stext);

    // CONFIG_BLK_DEV_INITRD: reserve the initrd when it is present.
    if initrd_start != 0 && initrd_end > initrd_start {
        let aligned_start = ALIGN_DOWN(initrd_start, PAGE_SIZE);
        let aligned_end = ALIGN(initrd_end, PAGE_SIZE);
        memblock_reserve(__pa(aligned_start), aligned_end - aligned_start);
    }

    early_init_fdt_reserve_self();
    early_init_fdt_scan_reserved_mem();
    memblock_dump_all();
}

static mut cpuinfo_or1k: [cpuinfo_or1k; NR_CPUS] = [cpuinfo_or1k::default(); NR_CPUS];

unsafe fn print_cpuinfo() {
    let upr = mfspr(SPR_UPR);
    let vr = mfspr(SPR_VR);
    let version = (vr & SPR_VR_VER) >> 24;
    let revision = vr & SPR_VR_REV;
    let cpuinfo = &cpuinfo_or1k[smp_processor_id() as usize];

    printk(KERN_INFO, "CPU: OpenRISC-%x (revision %d) @%d MHz\n", version, revision,
           cpuinfo.clock_frequency / 1000000);

    if upr & SPR_UPR_UP == 0 {
        printk(KERN_INFO, "-- no UPR register... unable to detect configuration\n");
        return;
    }
    if upr & SPR_UPR_DMP != 0 {
        printk(KERN_INFO, "-- dmmu: %4d entries, %lu way(s)\n",
               1 << ((mfspr(SPR_DMMUCFGR) & SPR_DMMUCFGR_NTS) >> 2),
               1 + (mfspr(SPR_DMMUCFGR) & SPR_DMMUCFGR_NTW));
    }
    if upr & SPR_UPR_IMP != 0 {
        printk(KERN_INFO, "-- immu: %4d entries, %lu way(s)\n",
               1 << ((mfspr(SPR_IMMUCFGR) & SPR_IMMUCFGR_NTS) >> 2),
               1 + (mfspr(SPR_IMMUCFGR) & SPR_IMMUCFGR_NTW));
    }
    printk(KERN_INFO, "-- additional features:\n");
    if upr & SPR_UPR_DUP != 0 { printk(KERN_INFO, "-- debug unit\n"); }
    if upr & SPR_UPR_PCUP != 0 { printk(KERN_INFO, "-- performance counters\n"); }
    if upr & SPR_UPR_PMP != 0 { printk(KERN_INFO, "-- power management\n"); }
    if upr & SPR_UPR_PICP != 0 { printk(KERN_INFO, "-- PIC\n"); }
    if upr & SPR_UPR_TTP != 0 { printk(KERN_INFO, "-- timer\n"); }
    if upr & SPR_UPR_CUP != 0 { printk(KERN_INFO, "-- custom unit(s)\n"); }
}

pub unsafe fn setup_cpuinfo() {
    let cpu_id = smp_processor_id();
    let cpu = of_get_cpu_node(cpu_id, core::ptr::null_mut());
    if cpu.is_null() { panic("Couldn't find CPU%d in device tree...\n", cpu_id); }
    if of_property_read_u32(cpu, "clock-frequency", &mut cpuinfo_or1k[cpu_id as usize].clock_frequency) != 0 {
        printk(KERN_WARNING, "Device tree missing CPU 'clock-frequency' parameter.Assuming frequency 25MHZThis is probably not what you want.");
    }
    cpuinfo_or1k[cpu_id as usize].coreid = mfspr(SPR_COREID);
    of_node_put(cpu);
    print_cpuinfo();
}

pub unsafe fn or1k_early_setup(mut fdt: *mut c_void) {
    if !fdt.is_null() { pr_info("FDT at %p\n", fdt); }
    else { fdt = __dtb_start; pr_info("Compiled-in FDT at %p\n", fdt); }
    early_init_devtree(fdt);
}

unsafe fn extract_value_bits(reg: c_ulong, bit_nr: c_short, width: c_short) -> c_ulong {
    (reg >> bit_nr) & (0 << width)
}

unsafe fn extract_value(mut reg: c_ulong, mut mask: c_ulong) -> c_ulong {
    while mask & 0x1 == 0 { reg >>= 1; mask >>= 1; }
    mask & reg
}

/* Lightweight calibrate_delay implementation calculating loops_per_jiffy from the device tree. */
pub unsafe fn calibrate_delay() {
    let cpu = of_get_cpu_node(smp_processor_id(), core::ptr::null_mut());
    let val = of_get_property(cpu, "clock-frequency", core::ptr::null_mut());
    if val.is_null() { panic("no cpu 'clock-frequency' parameter in device tree"); }
    loops_per_jiffy = (*val as c_int as c_ulong) / HZ;
    pr_cont("%lu.%02lu BogoMIPS (lpj=%lu)\n", loops_per_jiffy / (500000 / HZ),
            (loops_per_jiffy / (5000 / HZ)) % 100, loops_per_jiffy);
    of_node_put(cpu);
}

pub unsafe fn setup_arch(cmdline_p: *mut *mut c_char) {
    setup_memory();
    unflatten_and_copy_device_tree();
    setup_cpuinfo();
    // CONFIG_SMP: smp_init_cpus();
    setup_initial_init_mm(_stext, _etext, _edata, _end);
    if initrd_start == initrd_end {
        printk(KERN_INFO, "Initial ramdisk not found\n"); initrd_start = 0; initrd_end = 0;
    } else {
        printk(KERN_INFO, "Initial ramdisk at: 0x%p (%lu bytes)\n", initrd_start as *mut c_void, initrd_end - initrd_start);
        initrd_below_start_ok = 1;
    }
    jump_label_init();
    paging_init();
    *cmdline_p = boot_command_line;
    printk(KERN_INFO, "OpenRISC Linux -- http://openrisc.io\n");
}

unsafe fn show_cpuinfo(m: *mut seq_file, _v: *mut c_void) -> c_int {
    let mut vr = mfspr(SPR_VR);
    let cpucfgr = mfspr(SPR_CPUCFGR);
    let mut version;
    let avr;
    // CONFIG_SMP: print the processor core id from the sequence value.
    if vr & SPR_VR_UVRP != 0 {
        vr = mfspr(SPR_VR2); version = vr & SPR_VR2_VER; avr = mfspr(SPR_AVR);
        seq_printf(m, "cpu architecture\t: OpenRISC 1000 (%d.%d-rev%d)\n", (avr >> 24) & 0xff, (avr >> 16) & 0xff, (avr >> 8) & 0xff);
        seq_printf(m, "cpu implementation id\t: 0x%x\n", (vr & SPR_VR2_CPUID) >> 24);
        seq_printf(m, "cpu version\t\t: 0x%x\n", version);
    } else {
        version = (vr & SPR_VR_VER) >> 24;
        seq_printf(m, "cpu\t\t\t: OpenRISC-%x\n", version);
        seq_printf(m, "revision\t\t: %d\n", vr & SPR_VR_REV);
    }
    seq_printf(m, "frequency\t\t: %ld\n", loops_per_jiffy * HZ);
    seq_printf(m, "immu\t\t\t: %d entries, %lu ways\n", 1 << ((mfspr(SPR_DMMUCFGR) & SPR_DMMUCFGR_NTS) >> 2), 1 + (mfspr(SPR_DMMUCFGR) & SPR_DMMUCFGR_NTW));
    seq_printf(m, "dmmu\t\t\t: %d entries, %lu ways\n", 1 << ((mfspr(SPR_IMMUCFGR) & SPR_IMMUCFGR_NTS) >> 2), 1 + (mfspr(SPR_IMMUCFGR) & SPR_IMMUCFGR_NTW));
    seq_printf(m, "bogomips\t\t: %lu.%02lu\n", (loops_per_jiffy * HZ) / 500000, ((loops_per_jiffy * HZ) / 5000) % 100);
    seq_puts(m, "features\t\t: ");
    seq_printf(m, "%s ", if cpucfgr & SPR_CPUCFGR_OB32S != 0 { "orbis32" } else { "" });
    seq_printf(m, "%s ", if cpucfgr & SPR_CPUCFGR_OB64S != 0 { "orbis64" } else { "" });
    seq_printf(m, "%s ", if cpucfgr & SPR_CPUCFGR_OF32S != 0 { "orfpx32" } else { "" });
    seq_printf(m, "%s ", if cpucfgr & SPR_CPUCFGR_OF64S != 0 { "orfpx64" } else { "" });
    seq_printf(m, "%s ", if cpucfgr & SPR_CPUCFGR_OV64S != 0 { "orvdx64" } else { "" });
    seq_puts(m, "\n\n");
    0
}

unsafe fn c_start(_m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    *pos = cpumask_next(*pos - 1, cpu_online_mask);
    if *pos < nr_cpu_ids { &mut cpuinfo_or1k[*pos as usize] as *mut _ as *mut c_void } else { core::ptr::null_mut() }
}
unsafe fn c_next(m: *mut seq_file, _v: *mut c_void, pos: *mut loff_t) -> *mut c_void { *pos += 1; c_start(m, pos) }
unsafe fn c_stop(_m: *mut seq_file, _v: *mut c_void) {}

#[no_mangle]
pub static cpuinfo_op: seq_operations = seq_operations { start: Some(c_start), next: Some(c_next), stop: Some(c_stop), show: Some(show_cpuinfo) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
