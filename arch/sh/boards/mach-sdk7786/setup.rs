// SPDX-License-Identifier: GPL-2.0
/*
 * Renesas Technology Europe SDK7786 Support.
 *
 * Copyright (C) 2010  Matt Fleming
 * Copyright (C) 2010  Paul Mundt
 */

// Linux kernel headers and build-time macros from the original source are
// supplied by the surrounding kernel translation.

static mut HEARTBEAT_RESOURCE: Resource = Resource {
    start: 0x07fff8b0,
    end: 0x07fff8b0 + core::mem::size_of::<u16>() as u64 - 1,
    flags: IORESOURCE_MEM | IORESOURCE_MEM_16BIT,
};

static mut HEARTBEAT_DEVICE: PlatformDevice = PlatformDevice {
    name: "heartbeat",
    id: -1,
    num_resources: 1,
    resource: unsafe { &raw mut HEARTBEAT_RESOURCE },
};

/* Dummy supplies, where voltage doesn't matter */
static mut DUMMY_SUPPLIES: [RegulatorConsumerSupply; 2] = [
    RegulatorConsumerSupply { supply: "vddvario", dev_name: "smsc911x" },
    RegulatorConsumerSupply { supply: "vdd33a", dev_name: "smsc911x" },
];

static mut SMSC911X_RESOURCES: [Resource; 2] = [
    Resource {
        name: "smsc911x-memory",
        start: 0x07ffff00,
        end: 0x07ffff00 + SZ_256 - 1,
        flags: IORESOURCE_MEM,
    },
    Resource {
        name: "smsc911x-irq",
        start: evt2irq(0x2c0),
        end: evt2irq(0x2c0),
        flags: IORESOURCE_IRQ,
    },
];

static mut SMSC911X_CONFIG: SmsC911xPlatformConfig = SmsC911xPlatformConfig {
    irq_polarity: SMSC911X_IRQ_POLARITY_ACTIVE_LOW,
    irq_type: SMSC911X_IRQ_TYPE_OPEN_DRAIN,
    flags: SMSC911X_USE_32BIT,
    phy_interface: PHY_INTERFACE_MODE_MII,
};

static mut SMSC911X_DEVICE: PlatformDevice = PlatformDevice {
    name: "smsc911x",
    id: -1,
    num_resources: core::mem::size_of::<[Resource; 2]>() / core::mem::size_of::<Resource>(),
    resource: unsafe { &raw mut SMSC911X_RESOURCES[0] },
    dev: Device {
        platform_data: unsafe { &raw mut SMSC911X_CONFIG as *mut core::ffi::c_void },
    },
};

static mut SMBUS_FPGA_RESOURCE: Resource = Resource {
    start: 0x07fff9e0,
    end: 0x07fff9e0 + SZ_32 - 1,
    flags: IORESOURCE_MEM,
};

static mut SMBUS_FPGA_DEVICE: PlatformDevice = PlatformDevice {
    name: "i2c-sdk7786",
    id: 0,
    num_resources: 1,
    resource: unsafe { &raw mut SMBUS_FPGA_RESOURCE },
};

static mut SMBUS_PCIE_RESOURCE: Resource = Resource {
    start: 0x07fffc30,
    end: 0x07fffc30 + SZ_32 - 1,
    flags: IORESOURCE_MEM,
};

static mut SMBUS_PCIE_DEVICE: PlatformDevice = PlatformDevice {
    name: "i2c-sdk7786",
    id: 1,
    num_resources: 1,
    resource: unsafe { &raw mut SMBUS_PCIE_RESOURCE },
};

static mut SDK7786_I2C_DEVICES: [I2cBoardInfo; 1] = [I2cBoardInfo {
    type_: "max6900",
    addr: 0x68,
}];

static mut SH7786_DEVICES: [*mut PlatformDevice; 4] = [
    unsafe { &raw mut HEARTBEAT_DEVICE },
    unsafe { &raw mut SMSC911X_DEVICE },
    unsafe { &raw mut SMBUS_FPGA_DEVICE },
    unsafe { &raw mut SMBUS_PCIE_DEVICE },
];

unsafe fn sdk7786_i2c_setup() -> i32 {
    let mut tmp: u32;

    /*
     * Hand over I2C control to the FPGA.
     */
    tmp = fpga_read_reg(SBCR);
    tmp &= !SCBR_I2CCEN;
    tmp |= SCBR_I2CMEN;
    fpga_write_reg(tmp, SBCR);

    i2c_register_board_info(0, &raw mut SDK7786_I2C_DEVICES, 1)
}

unsafe fn sdk7786_devices_setup() -> i32 {
    let ret = platform_add_devices(&raw mut SH7786_DEVICES, 4);
    if ret != 0 {
        return ret;
    }

    sdk7786_i2c_setup()
}

// device_initcall(sdk7786_devices_setup);

unsafe fn sdk7786_mode_pins() -> i32 {
    fpga_read_reg(MODSWR) as i32
}

/*
 * FPGA-driven PCIe clocks
 *
 * Historically these include the oscillator, clock B (slots 2/3/4) and
 * clock A (slot 1 and the CPU clock). Newer revs of the PCB shove
 * everything under a single PCIe clocks enable bit that happens to map
 * to the same bit position as the oscillator bit for earlier FPGA
 * versions.
 *
 * Given that the legacy clocks have the side-effect of shutting the CPU
 * off through the FPGA along with the PCI slots, we simply leave them in
 * their initial state and don't bother registering them with the clock
 * framework.
 */
unsafe fn sdk7786_pcie_clk_enable(_clk: *mut Clk) -> i32 {
    fpga_write_reg(fpga_read_reg(PCIECR) | PCIECR_CLKEN, PCIECR);
    0
}

unsafe fn sdk7786_pcie_clk_disable(_clk: *mut Clk) {
    fpga_write_reg(fpga_read_reg(PCIECR) & !PCIECR_CLKEN, PCIECR);
}

static mut SDK7786_PCIE_CLK_OPS: ShClkOps = ShClkOps {
    enable: Some(sdk7786_pcie_clk_enable),
    disable: Some(sdk7786_pcie_clk_disable),
};

static mut SDK7786_PCIE_CLK: Clk = Clk {
    ops: unsafe { &raw mut SDK7786_PCIE_CLK_OPS },
};

static mut SDK7786_PCIE_CL: ClkLookup = ClkLookup {
    con_id: "pcie_plat_clk",
    clk: unsafe { &raw mut SDK7786_PCIE_CLK },
};

unsafe fn sdk7786_clk_init() -> i32 {
    /*
     * Only handle the EXTAL case, anyone interfacing a crystal
     * resonator will need to provide their own input clock.
     */
    if test_mode_pin(MODE_PIN9) {
        return -EINVAL;
    }

    let clk = clk_get(core::ptr::null_mut(), "extal");
    if is_err(clk) {
        return ptr_err(clk);
    }
    let ret = clk_set_rate(clk, 33333333);
    clk_put(clk);

    /*
     * Setup the FPGA clocks.
     */
    let ret = clk_register(unsafe { &raw mut SDK7786_PCIE_CLK });
    if ret != 0 {
        pr_err("FPGA clock registration failed\n");
        return ret;
    }

    clkdev_add(unsafe { &raw mut SDK7786_PCIE_CL });

    0
}

unsafe fn sdk7786_restart(_cmd: *mut core::ffi::c_char) {
    fpga_write_reg(0xa5a5, SRSTR);
}

unsafe fn sdk7786_power_off() {
    fpga_write_reg(fpga_read_reg(PWRCR) | PWRCR_PDWNREQ, PWRCR);

    /*
     * It can take up to 20us for the R8C to do its job, back off and
     * wait a bit until we've been shut off. Even though newer FPGA
     * versions don't set the ACK bit, the latency issue remains.
     */
    while (fpga_read_reg(PWRCR) & PWRCR_PDWNACK == 0 {
        cpu_sleep();
    }
}

/* Initialize the board */
unsafe fn sdk7786_setup(_cmdline_p: *mut *mut core::ffi::c_char) {
    pr_info("Renesas Technology Europe SDK7786 support:\n");

    regulator_register_fixed(0, &raw mut DUMMY_SUPPLIES, 2);

    sdk7786_fpga_init();
    sdk7786_nmi_init();

    pr_info("\tPCB revision:\t%d\n", fpga_read_reg(PCBRR) & 0xf);

    machine_ops.restart = Some(sdk7786_restart);
    pm_power_off = Some(sdk7786_power_off);

    register_smp_ops(&shx3_smp_ops);
}

/*
 * The Machine Vector
 */
static mut MV_SDK7786: ShMachineVector = ShMachineVector {
    mv_name: "SDK7786",
    mv_setup: Some(sdk7786_setup),
    mv_mode_pins: Some(sdk7786_mode_pins),
    mv_clk_init: Some(sdk7786_clk_init),
    mv_init_irq: Some(sdk7786_init_irq),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
