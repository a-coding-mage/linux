// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2010 John Crispin <john@phrozen.org>
 */

// C dependencies supplied by the surrounding kernel translation unit.

/* access to the ebu needs to be locked between different drivers */
DEFINE_SPINLOCK!(ebu_lock);
EXPORT_SYMBOL_GPL!(ebu_lock);

/*
 * this struct is filled by the soc specific detection code and holds
 * information about the specific soc type, revision and name
 */
static mut soc_info: ltq_soc_info = unsafe { core::mem::zeroed() };

/* These structs are used to override vsmp_init_secondary() */
#[cfg(CONFIG_MIPS_MT_SMP)]
extern "C" {
    static vsmp_smp_ops: plat_smp_ops;
}
#[cfg(CONFIG_MIPS_MT_SMP)]
static mut lantiq_smp_ops: plat_smp_ops = unsafe { core::mem::zeroed() };

pub unsafe extern "C" fn get_system_type() -> *const core::ffi::c_char {
    soc_info.sys_type.as_ptr() as *const core::ffi::c_char
}

pub unsafe extern "C" fn ltq_soc_type() -> i32 {
    soc_info.type_
}

unsafe fn prom_init_cmdline() {
    let argc: i32 = fw_arg0;
    let argv: *mut *mut core::ffi::c_char = KSEG1ADDR(fw_arg1) as *mut *mut core::ffi::c_char;
    let mut i = 0;

    arcs_cmdline[0] = 0;

    while i < argc {
        let p: *mut core::ffi::c_char = KSEG1ADDR(*argv.add(i as usize)) as *mut core::ffi::c_char;

        if CPHYSADDR(p as *const core::ffi::c_void) != 0 && *p != 0 {
            strlcat(arcs_cmdline.as_mut_ptr(), p, core::mem::size_of_val(&arcs_cmdline));
            strlcat(
                arcs_cmdline.as_mut_ptr(),
                b" \0".as_ptr() as *const core::ffi::c_char,
                core::mem::size_of_val(&arcs_cmdline),
            );
        }
        i += 1;
    }
}

pub unsafe extern "C" fn plat_mem_setup() {
    let mut dtb: *mut core::ffi::c_void;

    ioport_resource.start = IOPORT_RESOURCE_START;
    ioport_resource.end = IOPORT_RESOURCE_END;
    iomem_resource.start = IOMEM_RESOURCE_START;
    iomem_resource.end = IOMEM_RESOURCE_END;

    set_io_port_base(KSEG1 as usize as u64);

    dtb = get_fdt();
    if dtb.is_null() {
        panic!("no dtb found");
    }

    /*
     * Load the devicetree. This causes the chosen node to be
     * parsed resulting in our memory appearing
     */
    __dt_setup_arch(dtb);
}

#[cfg(CONFIG_MIPS_MT_SMP)]
unsafe fn lantiq_init_secondary() {
    /*
     * MIPS CPU startup function vsmp_init_secondary() will only
     * enable some of the interrupts for the second CPU/VPE.
     */
    set_c0_status(ST0_IM);
}

pub unsafe extern "C" fn prom_init() {
    /* call the soc specific detetcion code and get it to fill soc_info */
    ltq_soc_detect(&mut soc_info);
    snprintf(
        soc_info.sys_type.as_mut_ptr(),
        LTQ_SYS_TYPE_LEN - 1,
        b"%s rev %s\0".as_ptr() as *const core::ffi::c_char,
        soc_info.name.as_ptr(),
        soc_info.rev_type.as_ptr(),
    );
    soc_info.sys_type[LTQ_SYS_TYPE_LEN - 1] = 0;
    pr_info!("SoC: %s\n", soc_info.sys_type.as_ptr());
    prom_init_cmdline();

    #[cfg(CONFIG_MIPS_MT_SMP)]
    {
        lantiq_smp_ops = vsmp_smp_ops;
        if cpu_has_mipsmt {
            lantiq_smp_ops.init_secondary = Some(lantiq_init_secondary);
        }
        register_smp_ops(&mut lantiq_smp_ops);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
