// SPDX-License-Identifier: GPL-2.0
/* central.c: Central FHC driver for Sunfire/Starfire/Wildfire.
 *
 * Copyright (C) 1997, 1999, 2008 David S. Miller (davem@davemloft.net)
 */

// Kernel and architecture dependencies supplied by the surrounding tree.

#[repr(C)]
struct clock_board {
    clock_freq_regs: *mut core::ffi::c_void,
    clock_regs: *mut core::ffi::c_void,
    clock_ver_reg: *mut core::ffi::c_void,
    num_slots: i32,
    leds_resource: resource,
    leds_pdev: platform_device,
}

#[repr(C)]
struct fhc {
    pregs: *mut core::ffi::c_void,
    central: bool,
    jtag_master: bool,
    board_num: i32,
    leds_resource: resource,
    leds_pdev: platform_device,
}

unsafe fn clock_board_calc_nslots(p: *mut clock_board) -> i32 {
    let mut reg: u8 = unsafe { upa_readb((*p).clock_regs.add(CLOCK_STAT1 as usize)) } & 0xc0;

    match reg {
        0x40 => 16,
        0xc0 => 8,
        0x80 => {
            reg = 0;
            if unsafe { !(*p).clock_ver_reg.is_null() } {
                reg = unsafe { upa_readb((*p).clock_ver_reg) };
            }
            if reg != 0 {
                if reg & 0x80 != 0 { 4 } else { 5 }
            } else {
                4
            }
        }
        _ => 4,
    }
}

unsafe fn clock_board_probe(op: *mut platform_device) -> i32 {
    let p = unsafe { kzalloc_obj::<clock_board>() };
    let mut err: i32 = -ENOMEM;

    if p.is_null() {
        unsafe { printk(KERN_ERR, "clock_board: Cannot allocate struct clock_board\n") };
        return err;
    }

    unsafe {
        (*p).clock_freq_regs = of_ioremap(&mut (*op).resource[0], 0,
            resource_size(&(*op).resource[0]), "clock_board_freq");
        if (*p).clock_freq_regs.is_null() {
            printk(KERN_ERR, "clock_board: Cannot map clock_freq_regs\n");
            kfree(p as *mut core::ffi::c_void);
            return err;
        }
        (*p).clock_regs = of_ioremap(&mut (*op).resource[1], 0,
            resource_size(&(*op).resource[1]), "clock_board_regs");
        if (*p).clock_regs.is_null() {
            printk(KERN_ERR, "clock_board: Cannot map clock_regs\n");
            of_iounmap(&(*op).resource[0], (*p).clock_freq_regs,
                resource_size(&(*op).resource[0]));
            kfree(p as *mut core::ffi::c_void);
            return err;
        }
        if (*op).resource[2].flags != 0 {
            (*p).clock_ver_reg = of_ioremap(&mut (*op).resource[2], 0,
                resource_size(&(*op).resource[2]), "clock_ver_reg");
            if (*p).clock_ver_reg.is_null() {
                printk(KERN_ERR, "clock_board: Cannot map clock_ver_reg\n");
                of_iounmap(&(*op).resource[1], (*p).clock_regs,
                    resource_size(&(*op).resource[1]));
                of_iounmap(&(*op).resource[0], (*p).clock_freq_regs,
                    resource_size(&(*op).resource[0]));
                kfree(p as *mut core::ffi::c_void);
                return err;
            }
        }
        (*p).num_slots = clock_board_calc_nslots(p);
        (*p).leds_resource.start = ((*p).clock_regs.add(CLOCK_CTRL as usize)) as usize;
        (*p).leds_resource.end = (*p).leds_resource.start;
        (*p).leds_resource.name = "leds";
        (*p).leds_pdev.name = "sunfire-clockboard-leds";
        (*p).leds_pdev.id = -1;
        (*p).leds_pdev.resource = &mut (*p).leds_resource;
        (*p).leds_pdev.num_resources = 1;
        (*p).leds_pdev.dev.parent = &mut (*op).dev;
        err = platform_device_register(&mut (*p).leds_pdev);
        if err != 0 {
            printk(KERN_ERR, "clock_board: Could not register LEDS platform device\n");
            if !(*p).clock_ver_reg.is_null() { of_iounmap(&(*op).resource[2], (*p).clock_ver_reg, resource_size(&(*op).resource[2])); }
            of_iounmap(&(*op).resource[1], (*p).clock_regs, resource_size(&(*op).resource[1]));
            of_iounmap(&(*op).resource[0], (*p).clock_freq_regs, resource_size(&(*op).resource[0]));
            kfree(p as *mut core::ffi::c_void);
            return err;
        }
        printk(KERN_INFO, "clock_board: Detected %d slot Enterprise system.\n", (*p).num_slots);
    }
    0
}

unsafe fn fhc_probe(op: *mut platform_device) -> i32 {
    let p = unsafe { kzalloc_obj::<fhc>() };
    let mut err = -ENOMEM;
    let mut reg: u32;
    if p.is_null() { unsafe { printk(KERN_ERR, "fhc: Cannot allocate struct fhc\n") }; return err; }
    unsafe {
        if of_node_name_eq((*op).dev.of_node.parent, "central") { (*p).central = true; }
        (*p).pregs = of_ioremap(&mut (*op).resource[0], 0, resource_size(&(*op).resource[0]), "fhc_pregs");
        if (*p).pregs.is_null() { printk(KERN_ERR, "fhc: Cannot map pregs\n"); kfree(p as *mut _); return err; }
        if (*p).central {
            reg = upa_readl((*p).pregs.add(FHC_PREGS_BSR as usize));
            (*p).board_num = (((reg >> 16) & 1) | ((reg >> 12) & 0x0e)) as i32;
        } else {
            (*p).board_num = of_getintprop_default((*op).dev.of_node, "board#", -1);
            if (*p).board_num == -1 { printk(KERN_ERR, "fhc: No board# property\n"); of_iounmap(&(*op).resource[0], (*p).pregs, resource_size(&(*op).resource[0])); kfree(p as *mut _); return err; }
            if upa_readl((*p).pregs.add(FHC_PREGS_JCTRL as usize)) & FHC_JTAG_CTRL_MENAB != 0 { (*p).jtag_master = true; }
        }
        if !(*p).central {
            (*p).leds_resource.start = (*p).pregs.add(FHC_PREGS_CTRL as usize) as usize;
            (*p).leds_resource.end = (*p).leds_resource.start;
            (*p).leds_resource.name = "leds";
            (*p).leds_pdev.name = "sunfire-fhc-leds";
            (*p).leds_pdev.id = (*p).board_num;
            (*p).leds_pdev.resource = &mut (*p).leds_resource;
            (*p).leds_pdev.num_resources = 1;
            (*p).leds_pdev.dev.parent = &mut (*op).dev;
            err = platform_device_register(&mut (*p).leds_pdev);
            if err != 0 { printk(KERN_ERR, "fhc: Could not register LEDS platform device\n"); of_iounmap(&(*op).resource[0], (*p).pregs, resource_size(&(*op).resource[0])); kfree(p as *mut _); return err; }
        }
        reg = upa_readl((*p).pregs.add(FHC_PREGS_CTRL as usize));
        if !(*p).central { reg |= FHC_CONTROL_IXIST; }
        reg &= !(FHC_CONTROL_AOFF | FHC_CONTROL_BOFF | FHC_CONTROL_SLINE);
        upa_writel(reg, (*p).pregs.add(FHC_PREGS_CTRL as usize));
        upa_readl((*p).pregs.add(FHC_PREGS_CTRL as usize));
        reg = upa_readl((*p).pregs.add(FHC_PREGS_ID as usize));
        printk(KERN_INFO, "fhc: Board #%d, Version[%x] PartID[%x] Manuf[%x] %s\n", (*p).board_num, (reg & FHC_ID_VERS) >> 28, (reg & FHC_ID_PARTID) >> 12, (reg & FHC_ID_MANUF) >> 1, if (*p).jtag_master { "(JTAG Master)" } else if (*p).central { "(Central)" } else { "" });
    }
    0
}

static mut clock_board_driver: platform_driver = platform_driver { probe: Some(clock_board_probe), name: "clock_board", of_match_table: "clock-board" };
static mut fhc_driver: platform_driver = platform_driver { probe: Some(fhc_probe), name: "fhc", of_match_table: "fhc" };

unsafe fn sunfire_init() -> i32 {
    unsafe { platform_driver_register(&mut fhc_driver); platform_driver_register(&mut clock_board_driver); }
    0
}

// fs_initcall(sunfire_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
