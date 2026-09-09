// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Initial setup-routines for HP 9000 based hardware.
 *
 * Rust translation of the corresponding implementation source.
 */

// The declarations below are supplied by the surrounding kernel translation.

static mut boot_cpu_data: system_cpuinfo_parisc = system_cpuinfo_parisc::zeroed();
#[cfg(CONFIG_PA8X00)]
static mut _parisc_requires_coherency: i32 = 0;

extern "C" {
    static mut cpu_data: cpuinfo_parisc;
}

/// init_percpu_prof - enable/setup per cpu profiling hooks.
/// @cpunum: The processor instance.
///
/// FIXME: doesn't do much yet...
unsafe fn init_percpu_prof(_cpunum: u64) {}

/// processor_probe - Determine if processor driver should claim this device.
unsafe extern "C" fn processor_probe(dev: *mut parisc_device) -> i32 {
    let mut txn_addr: u64;
    let mut cpuid: u64;
    let p: *mut cpuinfo_parisc;
    let mut cpu_info: pdc_pat_cpu_num = core::mem::zeroed();

    #[cfg(CONFIG_SMP)]
    {
        if num_online_cpus() >= nr_cpu_ids {
            printk(KERN_INFO as _, c"num_online_cpus() >= nr_cpu_ids\n".as_ptr());
            return 1;
        }
    }
    #[cfg(not(CONFIG_SMP))]
    {
        if boot_cpu_data.cpu_count > 0 {
            printk(KERN_INFO as _, c"CONFIG_SMP=n  ignoring additional CPUs\n".as_ptr());
            return 1;
        }
    }

    cpuid = boot_cpu_data.cpu_count;
    txn_addr = (*dev).hpa.start;
    cpu_info.cpu_num = cpuid;
    cpu_info.cpu_loc = cpuid;

    #[cfg(CONFIG_64BIT)]
    if is_pdc_pat() {
        let mut status: u64;
        let mut bytecnt: u64 = 0;
        let pa_pdc_cell = kmalloc_obj::<pdc_pat_cell_mod_maddr_block_t>();
        if pa_pdc_cell.is_null() {
            panic!("couldn't allocate memory for PDC_PAT_CELL!");
        }
        status = pdc_pat_cell_module(&mut bytecnt, (*dev).pcell_loc,
            (*dev).mod_index, PA_VIEW, pa_pdc_cell);
        BUG_ON(PDC_OK != status);
        BUG_ON((*dev).mod_info != (*pa_pdc_cell).mod_info);
        BUG_ON((*dev).pmod_loc != (*pa_pdc_cell).mod_location);
        txn_addr = (*pa_pdc_cell).mod_[0];
        kfree(pa_pdc_cell as *mut core::ffi::c_void);
        status = pdc_pat_cpu_get_number(&mut cpu_info, (*dev).hpa.start);
        BUG_ON(PDC_OK != status);
        pr_info(c"Logical CPU #%lu is physical cpu #%lu at location 0x%lx with hpa %pa\n".as_ptr(),
            cpuid, cpu_info.cpu_num, cpu_info.cpu_loc, &(*dev).hpa.start);
    }

    p = per_cpu_ptr(&mut cpu_data, cpuid as usize);
    boot_cpu_data.cpu_count += 1;
    if cpuid != 0 {
        memset(p as *mut core::ffi::c_void, 0, core::mem::size_of::<cpuinfo_parisc>());
    }
    (*p).dev = dev;
    (*p).hpa = (*dev).hpa.start;
    (*p).cpuid = cpuid;
    (*p).txn_addr = txn_addr;
    (*p).cpu_num = cpu_info.cpu_num;
    (*p).cpu_loc = cpu_info.cpu_loc;
    store_cpu_topology(cpuid as usize);
    #[cfg(CONFIG_SMP)]
    init_percpu_prof(cpuid);
    #[cfg(CONFIG_SMP)]
    if cpuid != 0 {
        set_cpu_present(cpuid as usize, true);
        add_cpu(cpuid as usize);
    }
    0
}

/// collect_boot_cpu_data - Fill the boot_cpu_data structure.
unsafe extern "C" fn collect_boot_cpu_data() {
    let mut cr16_seed: u64;
    let mut orig_prod_num = [0i8; 64];
    let mut current_prod_num = [0i8; 64];
    let mut serial_no = [0i8; 64];
    memset(&mut boot_cpu_data as *mut _ as *mut core::ffi::c_void, 0,
        core::mem::size_of::<system_cpuinfo_parisc>());
    cr16_seed = get_cycles();
    add_device_randomness(&cr16_seed as *const _ as *const core::ffi::c_void,
        core::mem::size_of::<u64>());
    boot_cpu_data.cpu_hz = 100 * (*PAGE0).mem_10msec;
    if pdc_model_info(&mut boot_cpu_data.pdc.model) == PDC_OK {
        pr_info(c"model information\n".as_ptr());
        add_device_randomness(&boot_cpu_data.pdc.model as *const _ as *const core::ffi::c_void,
            core::mem::size_of_val(&boot_cpu_data.pdc.model));
    }
    if pdc_model_versions(&mut boot_cpu_data.pdc.versions, 0) == PDC_OK {
        pr_info(c"vers  0x%04lx\n".as_ptr(), boot_cpu_data.pdc.versions);
        add_device_randomness(&boot_cpu_data.pdc.versions as *const _ as *const core::ffi::c_void,
            core::mem::size_of_val(&boot_cpu_data.pdc.versions));
    }
    if pdc_model_cpuid(&mut boot_cpu_data.pdc.cpuid) == PDC_OK {
        pr_info(c"CPUID vers %ld rev %ld (0x%04lx)\n".as_ptr(),
            (boot_cpu_data.pdc.cpuid >> 5) & 127, boot_cpu_data.pdc.cpuid & 31,
            boot_cpu_data.pdc.cpuid);
        add_device_randomness(&boot_cpu_data.pdc.cpuid as *const _ as *const core::ffi::c_void,
            core::mem::size_of_val(&boot_cpu_data.pdc.cpuid));
    }
    if pdc_model_capabilities(&mut boot_cpu_data.pdc.capabilities) == PDC_OK {
        printk(KERN_INFO as _, c"capabilities 0x%lx\n".as_ptr(), boot_cpu_data.pdc.capabilities);
    }
    if pdc_model_sysmodel(OS_ID_HPUX, boot_cpu_data.pdc.sys_model_name.as_mut_ptr()) == PDC_OK {
        pr_info(c"HP-UX model name: %s\n".as_ptr(), boot_cpu_data.pdc.sys_model_name.as_ptr());
    }
    serial_no[0] = 0;
    if pdc_model_sysmodel(OS_ID_MPEXL, serial_no.as_mut_ptr()) == PDC_OK && serial_no[0] != 0 {
        pr_info(c"MPE/iX model name: %s\n".as_ptr(), serial_no.as_ptr());
    }
    dump_stack_set_arch_desc(c"%s".as_ptr(), boot_cpu_data.pdc.sys_model_name.as_ptr());
    boot_cpu_data.hversion = boot_cpu_data.pdc.model.hversion;
    boot_cpu_data.sversion = boot_cpu_data.pdc.model.sversion;
    boot_cpu_data.cpu_type = parisc_get_cpu_type(boot_cpu_data.hversion);
    boot_cpu_data.cpu_name = cpu_name_version[boot_cpu_data.cpu_type][0];
    boot_cpu_data.family_name = cpu_name_version[boot_cpu_data.cpu_type][1];
    #[cfg(CONFIG_PA8X00)]
    { _parisc_requires_coherency = boot_cpu_data.cpu_type == mako || boot_cpu_data.cpu_type == mako2; }
    if pdc_model_platform_info(orig_prod_num.as_mut_ptr(), current_prod_num.as_mut_ptr(), serial_no.as_mut_ptr()) == PDC_OK {
        printk(KERN_INFO as _, c"product %s, original product %s, S/N: %s\n".as_ptr(),
            if current_prod_num[0] != 0 { current_prod_num.as_ptr() } else { c"n/a".as_ptr() },
            orig_prod_num.as_ptr(), serial_no.as_ptr());
        add_device_randomness(orig_prod_num.as_ptr() as *const _, strlen(orig_prod_num.as_ptr()));
        add_device_randomness(current_prod_num.as_ptr() as *const _, strlen(current_prod_num.as_ptr()));
        add_device_randomness(serial_no.as_ptr() as *const _, strlen(serial_no.as_ptr()));
    }
}

unsafe extern "C" fn init_per_cpu(cpunum: i32) -> i32 {
    let mut coproc_cfg: pdc_coproc_cfg = core::mem::zeroed();
    set_firmware_width();
    let ret = pdc_coproc_cfg(&mut coproc_cfg);
    if ret >= 0 && coproc_cfg.ccr_functional != 0 {
        mtctl(coproc_cfg.ccr_functional, 10);
        (*per_cpu_ptr(&mut cpu_data, cpunum as usize)).fp_rev = coproc_cfg.revision;
        (*per_cpu_ptr(&mut cpu_data, cpunum as usize)).fp_model = coproc_cfg.model;
    } else {
        printk(KERN_WARNING as _, c"WARNING: No FP CoProcessor?!\n".as_ptr());
        #[cfg(CONFIG_64BIT)] panic!("FP CoProc not reported");
    }
    init_percpu_prof(cpunum as u64);
    btlb_init_per_cpu();
    ret
}

unsafe extern "C" fn show_cpuinfo(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let cpu_name = per_cpu(&cpu_data, 0).dev.name;
    for_each_online_cpu!(cpu, {
        seq_printf(m, c"processor\t: %lu\n".as_ptr(), cpu);
        seq_printf(m, c"cpu family\t: PA-RISC %s\n".as_ptr(), boot_cpu_data.family_name);
        seq_printf(m, c"cpu\t\t: %s\n".as_ptr(), boot_cpu_data.cpu_name);
        seq_printf(m, c"cpu MHz\t\t: %d.%06d\n".as_ptr(), boot_cpu_data.cpu_hz / 1000000, boot_cpu_data.cpu_hz % 1000000);
    });
    0
}

static processor_tbl: [parisc_device_id; 2] = [
    parisc_device_id { hw_type: HPHW_NPROC, hversion_rev: HVERSION_REV_ANY_ID, hversion: HVERSION_ANY_ID, sversion: SVERSION_ANY_ID },
    parisc_device_id { hw_type: 0, hversion_rev: 0, hversion: 0, sversion: 0 },
];

static mut cpu_driver: parisc_driver = parisc_driver {
    name: c"CPU".as_ptr(),
    id_table: processor_tbl.as_ptr(),
    probe: Some(processor_probe),
};

unsafe extern "C" fn processor_init() {
    reset_cpu_topology();
    register_parisc_driver(&cpu_driver);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
