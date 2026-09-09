// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Embedded Planet EP8248E support
 *
 * Copyright 2007 Freescale Semiconductor, Inc.
 * Author: Scott Wood <scottwood@freescale.com>
 */

// Kernel headers and symbols referenced below are supplied by the surrounding
// Rust kernel environment.

static mut EP8248E_BCSR: *mut u8 = core::ptr::null_mut();
static mut EP8248E_BCSR_NODE: *mut device_node = core::ptr::null_mut();

const BCSR7_SCC2_ENABLE: u8 = 0x10;

const BCSR8_PHY1_ENABLE: u8 = 0x80;
const BCSR8_PHY1_POWER: u8 = 0x40;
const BCSR8_PHY2_ENABLE: u8 = 0x20;
const BCSR8_PHY2_POWER: u8 = 0x10;
const BCSR8_MDIO_READ: u8 = 0x04;
const BCSR8_MDIO_CLOCK: u8 = 0x02;
const BCSR8_MDIO_DATA: u8 = 0x01;

const BCSR9_USB_ENABLE: u8 = 0x80;
const BCSR9_USB_POWER: u8 = 0x40;
const BCSR9_USB_HOST: u8 = 0x20;
const BCSR9_USB_FULL_SPEED_TARGET: u8 = 0x10;

unsafe fn ep8248e_pic_init() {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), c"fsl,pq2-pic".as_ptr());
    if np.is_null() {
        printk(KERN_ERR, c"PIC init: can not find cpm-pic node\n".as_ptr());
        return;
    }

    cpm2_pic_init(np);
    of_node_put(np);
}

unsafe fn ep8248e_set_mdc(_ctrl: *mut mdiobb_ctrl, level: i32) {
    if level != 0 {
        setbits8(EP8248E_BCSR.add(8), BCSR8_MDIO_CLOCK);
    } else {
        clrbits8(EP8248E_BCSR.add(8), BCSR8_MDIO_CLOCK);
    }
    // Read back to flush the write.
    in_8(EP8248E_BCSR.add(8));
}

unsafe fn ep8248e_set_mdio_dir(_ctrl: *mut mdiobb_ctrl, output: i32) {
    if output != 0 {
        clrbits8(EP8248E_BCSR.add(8), BCSR8_MDIO_READ);
    } else {
        setbits8(EP8248E_BCSR.add(8), BCSR8_MDIO_READ);
    }
    // Read back to flush the write.
    in_8(EP8248E_BCSR.add(8));
}

unsafe fn ep8248e_set_mdio_data(_ctrl: *mut mdiobb_ctrl, data: i32) {
    if data != 0 {
        setbits8(EP8248E_BCSR.add(8), BCSR8_MDIO_DATA);
    } else {
        clrbits8(EP8248E_BCSR.add(8), BCSR8_MDIO_DATA);
    }
    // Read back to flush the write.
    in_8(EP8248E_BCSR.add(8));
}

unsafe fn ep8248e_get_mdio_data(_ctrl: *mut mdiobb_ctrl) -> i32 {
    (in_8(EP8248E_BCSR.add(8)) & BCSR8_MDIO_DATA) as i32
}

static EP8248E_MDIO_OPS: mdiobb_ops = mdiobb_ops {
    set_mdc: Some(ep8248e_set_mdc),
    set_mdio_dir: Some(ep8248e_set_mdio_dir),
    set_mdio_data: Some(ep8248e_set_mdio_data),
    get_mdio_data: Some(ep8248e_get_mdio_data),
    owner: THIS_MODULE,
};

static mut EP8248E_MDIO_CTRL: mdiobb_ctrl = mdiobb_ctrl {
    ops: &EP8248E_MDIO_OPS,
};

unsafe fn ep8248e_mdio_probe(ofdev: *mut platform_device) -> i32 {
    let mut bus: *mut mii_bus;
    let mut res: resource = core::mem::zeroed();
    let node: *mut device_node;
    let ret: i32;

    node = of_get_parent((*(*ofdev).dev.of_node));
    of_node_put(node);
    if node != EP8248E_BCSR_NODE {
        return -ENODEV;
    }

    ret = of_address_to_resource((*ofdev).dev.of_node, 0, &mut res);
    if ret != 0 {
        return ret;
    }

    bus = alloc_mdio_bitbang(&mut EP8248E_MDIO_CTRL);
    if bus.is_null() {
        return -ENOMEM;
    }

    (*bus).name = c"ep8248e-mdio-bitbang".as_ptr();
    (*bus).parent = &mut (*ofdev).dev;
    snprintf((*bus).id.as_mut_ptr(), MII_BUS_ID_SIZE, c"%pa".as_ptr(), &res.start);

    ret = of_mdiobus_register(bus, (*ofdev).dev.of_node);
    if ret != 0 {
        free_mdio_bitbang(bus);
        return ret;
    }
    0
}

static EP8248E_MDIO_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: c"fsl,ep8248e-mdio-bitbang".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

static mut EP8248E_MDIO_DRIVER: platform_driver = platform_driver {
    driver: driver {
        name: c"ep8248e-mdio-bitbang".as_ptr(),
        of_match_table: EP8248E_MDIO_MATCH.as_ptr(),
        suppress_bind_attrs: true,
    },
    probe: Some(ep8248e_mdio_probe),
};

#[repr(C)]
struct cpm_pin {
    port: i32,
    pin: i32,
    flags: i32,
}

static EP8248E_PINS: [cpm_pin; 48] = [
    cpm_pin { port: 2, pin: 4, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 2, pin: 5, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 2, pin: 14, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 2, pin: 15, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 3, pin: 29, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 3, pin: 30, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 3, pin: 31, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 14, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 15, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 16, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 17, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 18, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 19, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 20, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 21, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 26, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 0, pin: 27, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 0, pin: 28, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 0, pin: 29, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 0, pin: 30, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 0, pin: 31, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 2, pin: 21, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 2, pin: 22, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 18, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 19, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 20, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 21, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 22, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 23, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 24, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 25, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 26, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 27, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 28, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 29, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 1, pin: 30, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 1, pin: 31, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 2, pin: 18, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 2, pin: 19, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 4, pin: 14, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 4, pin: 15, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 2, pin: 10, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 2, pin: 11, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 2, pin: 20, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 2, pin: 24, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 3, pin: 23, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 3, pin: 24, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 3, pin: 25, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
];

unsafe fn init_ioports() {
    for pin in EP8248E_PINS.iter() {
        cpm2_set_pin(pin.port, pin.pin, pin.flags);
    }
    cpm2_smc_clk_setup(CPM_CLK_SMC1, CPM_BRG7);
    cpm2_clk_setup(CPM_CLK_SCC1, CPM_BRG1, CPM_CLK_RX);
    cpm2_clk_setup(CPM_CLK_SCC1, CPM_BRG1, CPM_CLK_TX);
    cpm2_clk_setup(CPM_CLK_SCC3, CPM_CLK8, CPM_CLK_TX); // USB
    cpm2_clk_setup(CPM_CLK_FCC1, CPM_CLK11, CPM_CLK_RX);
    cpm2_clk_setup(CPM_CLK_FCC1, CPM_CLK10, CPM_CLK_TX);
    cpm2_clk_setup(CPM_CLK_FCC2, CPM_CLK13, CPM_CLK_RX);
    cpm2_clk_setup(CPM_CLK_FCC2, CPM_CLK14, CPM_CLK_TX);
}

unsafe fn ep8248e_setup_arch() {
    if ppc_md.progress.is_some() {
        ppc_md.progress.unwrap()(c"ep8248e_setup_arch()".as_ptr(), 0);
    }
    cpm2_reset();
    // When this is set, snooping CPM DMA from RAM causes machine checks.
    // See erratum SIU18.
    clrbits32(&mut (*cpm2_immr).im_siu_conf.siu_82xx.sc_bcr, MPC82XX_BCR_PLDP);

    EP8248E_BCSR_NODE = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), c"fsl,ep8248e-bcsr".as_ptr());
    if EP8248E_BCSR_NODE.is_null() {
        printk(KERN_ERR, c"No bcsr in device tree\n".as_ptr());
        return;
    }
    EP8248E_BCSR = of_iomap(EP8248E_BCSR_NODE, 0);
    if EP8248E_BCSR.is_null() {
        printk(KERN_ERR, c"Cannot map BCSR registers\n".as_ptr());
        of_node_put(EP8248E_BCSR_NODE);
        EP8248E_BCSR_NODE = core::ptr::null_mut();
        return;
    }
    setbits8(EP8248E_BCSR.add(7), BCSR7_SCC2_ENABLE);
    setbits8(EP8248E_BCSR.add(8), BCSR8_PHY1_ENABLE | BCSR8_PHY1_POWER | BCSR8_PHY2_ENABLE | BCSR8_PHY2_POWER);
    init_ioports();
    if ppc_md.progress.is_some() {
        ppc_md.progress.unwrap()(c"ep8248e_setup_arch(), finish".as_ptr(), 0);
    }
}

static OF_BUS_IDS: [of_device_id; 3] = [
    of_device_id { compatible: c"simple-bus".as_ptr() },
    of_device_id { compatible: c"fsl,ep8248e-bcsr".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

unsafe fn declare_of_platform_devices() -> i32 {
    of_platform_bus_probe(core::ptr::null_mut(), OF_BUS_IDS.as_ptr(), core::ptr::null_mut());
    if IS_ENABLED_CONFIG_MDIO_BITBANG {
        platform_driver_register(&mut EP8248E_MDIO_DRIVER);
    }
    0
}

machine_device_initcall!(ep8248e, declare_of_platform_devices);

define_machine!(ep8248e, machine_desc {
    name: c"Embedded Planet EP8248E".as_ptr(),
    compatible: c"fsl,ep8248e".as_ptr(),
    setup_arch: Some(ep8248e_setup_arch),
    init_IRQ: Some(ep8248e_pic_init),
    get_irq: Some(cpm2_get_irq),
    restart: Some(pq2_restart),
    progress: Some(udbg_progress),
});

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
