// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2000-2003 Deep Blue Solutions Ltd
 */

// Kernel dependencies supplied by the surrounding translation unit/build.

/* Regmap to the AP system controller */
static mut ap_syscon_map: *mut regmap = core::ptr::null_mut();

/*
 * All IO addresses are mapped onto VA 0xFFFx.xxxx, where x.xxxx
 * is the (PA >> 12).
 *
 * Setup a VA for the Integrator interrupt controller (for header #0,
 * just for now).
 */
// #define VA_IC_BASE __io_address(INTEGRATOR_IC_BASE)

/*
 * Logical      Physical
 * f1400000     14000000       Interrupt controller
 * f1600000     16000000       UART 0
 */

#[repr(C)]
static mut ap_io_desc: [map_desc; 2] = [
    map_desc {
        virtual_: IO_ADDRESS(INTEGRATOR_IC_BASE),
        pfn: __phys_to_pfn(INTEGRATOR_IC_BASE),
        length: SZ_4K,
        type_: MT_DEVICE,
    },
    map_desc {
        virtual_: IO_ADDRESS(INTEGRATOR_UART0_BASE),
        pfn: __phys_to_pfn(INTEGRATOR_UART0_BASE),
        length: SZ_4K,
        type_: MT_DEVICE,
    },
];

unsafe fn ap_map_io() {
    iotable_init(ap_io_desc.as_ptr(), ARRAY_SIZE(ap_io_desc));
}

#[cfg(CONFIG_PM)]
static mut ic_irq_enable: c_ulong = 0;

#[cfg(CONFIG_PM)]
unsafe fn irq_suspend(_data: *mut core::ffi::c_void) -> c_int {
    ic_irq_enable = readl(__io_address(INTEGRATOR_IC_BASE) + IRQ_ENABLE);
    0
}

#[cfg(CONFIG_PM)]
unsafe fn irq_resume(_data: *mut core::ffi::c_void) {
    /* disable all irq sources */
    cm_clear_irqs();
    writel((-1i32) as u32, __io_address(INTEGRATOR_IC_BASE) + IRQ_ENABLE_CLEAR);
    writel((-1i32) as u32, __io_address(INTEGRATOR_IC_BASE) + FIQ_ENABLE_CLEAR);
    writel(ic_irq_enable, __io_address(INTEGRATOR_IC_BASE) + IRQ_ENABLE_SET);
}

#[cfg(not(CONFIG_PM))]
// #define irq_suspend NULL
// #define irq_resume NULL

static irq_syscore_ops: syscore_ops = syscore_ops {
    suspend: Some(irq_suspend),
    resume: Some(irq_resume),
};

static mut irq_syscore: syscore = syscore {
    ops: &irq_syscore_ops,
};

unsafe fn irq_syscore_init() -> c_int {
    register_syscore(&mut irq_syscore);
    0
}

device_initcall!(irq_syscore_init);

/*
 * For the PL010 found in the Integrator/AP some of the UART control is
 * implemented in the system controller and accessed using a callback
 * from the driver.
 */
unsafe fn integrator_uart_set_mctrl(
    dev: *mut amba_device,
    _base: *mut core::ffi::c_void,
    mctrl: c_uint,
) {
    let mut ctrls: c_uint = 0;
    let mut ctrlc: c_uint = 0;
    let rts_mask: c_uint;
    let dtr_mask: c_uint;
    let phybase: u32 = (*dev).res.start;
    let mut ret: c_int;

    if phybase == INTEGRATOR_UART0_BASE {
        /* UART0 */
        rts_mask = 1 << 4;
        dtr_mask = 1 << 5;
    } else {
        /* UART1 */
        rts_mask = 1 << 6;
        dtr_mask = 1 << 7;
    }

    if mctrl & TIOCM_RTS != 0 {
        ctrlc |= rts_mask;
    } else {
        ctrls |= rts_mask;
    }

    if mctrl & TIOCM_DTR != 0 {
        ctrlc |= dtr_mask;
    } else {
        ctrls |= dtr_mask;
    }

    ret = regmap_write(ap_syscon_map, INTEGRATOR_SC_CTRLS_OFFSET, ctrls);
    if ret != 0 {
        pr_err!("MODEM: unable to write PL010 UART CTRLS\n");
    }

    ret = regmap_write(ap_syscon_map, INTEGRATOR_SC_CTRLC_OFFSET, ctrlc);
    if ret != 0 {
        pr_err!("MODEM: unable to write PL010 UART CRTLC\n");
    }
}

static mut ap_uart_data: amba_pl010_data = amba_pl010_data {
    set_mctrl: Some(integrator_uart_set_mctrl),
};

unsafe fn ap_init_irq_of() {
    cm_init();
    irqchip_init();
}

/* For the Device Tree, add in the UART callbacks as AUXDATA */
static mut ap_auxdata_lookup: [of_dev_auxdata; 3] = [
    OF_DEV_AUXDATA!("arm,primecell", INTEGRATOR_UART0_BASE, "uart0", &mut ap_uart_data),
    OF_DEV_AUXDATA!("arm,primecell", INTEGRATOR_UART1_BASE, "uart1", &mut ap_uart_data),
    of_dev_auxdata::sentinel(),
];

static ap_syscon_match: [of_device_id; 2] = [
    of_device_id { compatible: "arm,integrator-ap-syscon" },
    of_device_id { compatible: core::ptr::null() },
];

unsafe fn ap_init_of() {
    let mut syscon: *mut device_node;

    of_platform_default_populate(core::ptr::null_mut(), ap_auxdata_lookup.as_ptr(), core::ptr::null_mut());

    syscon = of_find_matching_node(core::ptr::null_mut(), ap_syscon_match.as_ptr());
    if syscon.is_null() {
        return;
    }
    ap_syscon_map = syscon_node_to_regmap(syscon);
    if IS_ERR(ap_syscon_map) {
        pr_crit!("could not find Integrator/AP system controller\n");
        return;
    }
}

static ap_dt_board_compat: [*const c_char; 2] = [
    "arm,integrator-ap\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

DT_MACHINE_START!(INTEGRATOR_AP_DT, "ARM Integrator/AP (Device Tree)", MachineDesc {
    reserve: Some(integrator_reserve),
    map_io: Some(ap_map_io),
    init_irq: Some(ap_init_irq_of),
    init_machine: Some(ap_init_of),
    dt_compat: ap_dt_board_compat.as_ptr(),
});

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
