// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * FSL SoC setup code
 *
 * Maintained by Kumar Gala (see MAINTAINERS for contact information)
 *
 * 2006 (c) MontaVista Software, Inc.
 * Vitaly Bordug <vbordug@ru.mvista.com>
 */

// Linux and architecture dependencies supplied by the surrounding repository.

static mut immrbase: phys_addr_t = (-1i32) as phys_addr_t;

pub unsafe fn get_immrbase() -> phys_addr_t {
    let mut soc: *mut device_node;

    if immrbase != (-1i32) as phys_addr_t {
        return immrbase;
    }

    soc = of_find_node_by_type(core::ptr::null_mut(), c"soc".as_ptr());
    if !soc.is_null() {
        let mut res: resource;

        if of_range_to_resource(soc, 0, &mut res) == 0 {
            immrbase = res.start;
        }

        of_node_put(soc);
    }

    immrbase
}

// EXPORT_SYMBOL(get_immrbase);

pub unsafe fn fsl_get_sys_freq() -> u32 {
    static mut sysfreq: u32 = (-1i32) as u32;
    let soc: *mut device_node;

    if sysfreq != (-1i32) as u32 {
        return sysfreq;
    }

    soc = of_find_node_by_type(core::ptr::null_mut(), c"soc".as_ptr());
    if soc.is_null() {
        return (-1i32) as u32;
    }

    of_property_read_u32(soc, c"clock-frequency".as_ptr(), &mut sysfreq);
    if sysfreq == (-1i32) as u32 || sysfreq == 0 {
        of_property_read_u32(soc, c"bus-frequency".as_ptr(), &mut sysfreq);
    }

    of_node_put(soc);
    sysfreq
}
// EXPORT_SYMBOL(fsl_get_sys_freq);

// CONFIG_CPM || CONFIG_QUICC_ENGINE
#[cfg(any(feature = "CONFIG_CPM", feature = "CONFIG_QUICC_ENGINE"))]
pub unsafe fn get_brgfreq() -> u32 {
    static mut brgfreq: u32 = (-1i32) as u32;
    let mut node: *mut device_node;

    if brgfreq != (-1i32) as u32 {
        return brgfreq;
    }

    node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), c"fsl,cpm-brg".as_ptr());
    if !node.is_null() {
        of_property_read_u32(node, c"clock-frequency".as_ptr(), &mut brgfreq);
        of_node_put(node);
        return brgfreq;
    }

    /* Legacy device binding -- will go away when no users are left. */
    node = of_find_node_by_type(core::ptr::null_mut(), c"cpm".as_ptr());
    if node.is_null() {
        node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), c"fsl,qe".as_ptr());
    }
    if node.is_null() {
        node = of_find_node_by_type(core::ptr::null_mut(), c"qe".as_ptr());
    }

    if !node.is_null() {
        of_property_read_u32(node, c"brg-frequency".as_ptr(), &mut brgfreq);
        if brgfreq == (-1i32) as u32 || brgfreq == 0 {
            if of_property_read_u32(node, c"bus-frequency".as_ptr(), &mut brgfreq) == 0 {
                brgfreq /= 2;
            }
        }
        of_node_put(node);
    }

    brgfreq
}

// EXPORT_SYMBOL(get_brgfreq);

#[cfg(any(feature = "CONFIG_CPM", feature = "CONFIG_QUICC_ENGINE"))]
pub unsafe fn get_baudrate() -> u32 {
    static mut fs_baudrate: u32 = (-1i32) as u32;
    let node: *mut device_node;

    if fs_baudrate != (-1i32) as u32 {
        return fs_baudrate;
    }

    node = of_find_node_by_type(core::ptr::null_mut(), c"serial".as_ptr());
    if !node.is_null() {
        of_property_read_u32(node, c"current-speed".as_ptr(), &mut fs_baudrate);
        of_node_put(node);
    }

    fs_baudrate
}

// EXPORT_SYMBOL(get_baudrate);

// CONFIG_FSL_SOC_BOOKE || CONFIG_PPC_86xx
#[cfg(any(feature = "CONFIG_FSL_SOC_BOOKE", feature = "CONFIG_PPC_86xx"))]
static mut rstcr: *mut __be32 = core::ptr::null_mut();

#[cfg(any(feature = "CONFIG_FSL_SOC_BOOKE", feature = "CONFIG_PPC_86xx"))]
unsafe fn fsl_rstcr_restart(_this: *mut notifier_block, _mode: c_ulong, _cmd: *mut c_void) -> c_int {
    local_irq_disable();
    /* set reset control register */
    out_be32(rstcr, 0x2); /* HRESET_REQ */
    NOTIFY_DONE
}

#[cfg(any(feature = "CONFIG_FSL_SOC_BOOKE", feature = "CONFIG_PPC_86xx"))]
unsafe fn setup_rstcr() -> c_int {
    let mut np: *mut device_node = core::ptr::null_mut();
    static mut restart_handler: notifier_block = notifier_block {
        notifier_call: Some(fsl_rstcr_restart),
        priority: 128,
    };

    // for_each_node_by_name(np, "global-utilities")
    while !({ np = of_find_node_by_name(np, c"global-utilities".as_ptr()); np }).is_null() {
        if of_property_read_bool(np, c"fsl,has-rstcr".as_ptr()) {
            rstcr = of_iomap(np, 0).add(0xb0);
            if rstcr.is_null() {
                printk(c"Error: reset control register not mapped!\n".as_ptr());
            } else {
                register_restart_handler(&mut restart_handler);
            }
            break;
        }
    }

    of_node_put(np);
    0
}

// arch_initcall(setup_rstcr);

// CONFIG_FB_FSL_DIU || CONFIG_FB_FSL_DIU_MODULE
#[cfg(any(feature = "CONFIG_FB_FSL_DIU", feature = "CONFIG_FB_FSL_DIU_MODULE"))]
pub static mut diu_ops: platform_diu_data_ops = platform_diu_data_ops {};
// EXPORT_SYMBOL(diu_ops);

// CONFIG_EPAPR_PARAVIRT
#[cfg(feature = "CONFIG_EPAPR_PARAVIRT")]
pub unsafe fn fsl_hv_restart(_cmd: *mut c_char) -> ! {
    pr_info(c"hv restart\n".as_ptr());
    fh_partition_restart(-1);
    loop {}
}

#[cfg(feature = "CONFIG_EPAPR_PARAVIRT")]
pub unsafe fn fsl_hv_halt() -> ! {
    pr_info(c"hv exit\n".as_ptr());
    fh_partition_stop(-1);
    loop {}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
