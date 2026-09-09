// SPDX-License-Identifier: GPL-2.0
/*
 * June 2006 Steve Glendinning <steve.glendinning@shawell.net>
 *
 * Polaris-specific resource declaration
 *
 */

// Linux and platform dependencies supplied by other translation units.

const BCR2: u32 = 0xFFFFFF62;
const WCR2: u32 = 0xFFFFFF66;
const AREA5_WAIT_CTRL: u16 = 0x1C00;
const WAIT_STATES_10: u16 = 0x7;

/* Dummy supplies, where voltage doesn't matter */
static mut DUMMY_SUPPLIES: [regulator_consumer_supply; 2] = [
    regulator_consumer_supply { supply: "vddvario", dev_name: "smsc911x.0" },
    regulator_consumer_supply { supply: "vdd33a", dev_name: "smsc911x.0" },
];

static mut SMSC911X_RESOURCES: [resource; 2] = [
    resource {
        name: "smsc911x-memory",
        start: PA_EXT5,
        end: PA_EXT5 + 0x1fff,
        flags: IORESOURCE_MEM,
    },
    resource {
        name: "smsc911x-irq",
        start: IRQ0_IRQ,
        end: IRQ0_IRQ,
        flags: IORESOURCE_IRQ,
    },
];

static mut SMSC911X_CONFIG: smsc911x_platform_config = smsc911x_platform_config {
    irq_polarity: SMSC911X_IRQ_POLARITY_ACTIVE_LOW,
    irq_type: SMSC911X_IRQ_TYPE_OPEN_DRAIN,
    flags: SMSC911X_USE_32BIT,
    phy_interface: PHY_INTERFACE_MODE_MII,
};

static mut SMSC911X_DEVICE: platform_device = platform_device {
    name: "smsc911x",
    id: 0,
    num_resources: SMSC911X_RESOURCES.len(),
    resource: SMSC911X_RESOURCES.as_mut_ptr(),
    dev: device {
        platform_data: &mut SMSC911X_CONFIG as *mut _ as *mut core::ffi::c_void,
    },
};

static mut HEARTBEAT_BIT_POS: [u8; 4] = [0, 1, 2, 3];

static mut HEARTBEAT_DATA: heartbeat_data = heartbeat_data {
    bit_pos: HEARTBEAT_BIT_POS.as_mut_ptr(),
    nr_bits: HEARTBEAT_BIT_POS.len(),
};

static mut HEARTBEAT_RESOURCE: resource = resource {
    start: PORT_PCDR,
    end: PORT_PCDR,
    flags: IORESOURCE_MEM | IORESOURCE_MEM_8BIT,
};

static mut HEARTBEAT_DEVICE: platform_device = platform_device {
    name: "heartbeat",
    id: -1,
    dev: device {
        platform_data: &mut HEARTBEAT_DATA as *mut _ as *mut core::ffi::c_void,
    },
    num_resources: 1,
    resource: &mut HEARTBEAT_RESOURCE,
};

static mut POLARIS_DEVICES: [*mut platform_device; 2] = [
    &mut SMSC911X_DEVICE,
    &mut HEARTBEAT_DEVICE,
];

unsafe fn polaris_initialise() -> i32 {
    let mut wcr: u16;
    let mut bcr_mask: u16;

    printk(KERN_INFO, "Configuring Polaris external bus\0");

    regulator_register_fixed(0, DUMMY_SUPPLIES.as_mut_ptr(), DUMMY_SUPPLIES.len());

    /* Configure area 5 with 2 wait states */
    wcr = __raw_readw(WCR2 as *const u16);
    wcr &= !AREA5_WAIT_CTRL;
    wcr |= WAIT_STATES_10 << 10;
    __raw_writew(wcr, WCR2 as *mut u16);

    /* Configure area 5 for 32-bit access */
    bcr_mask = __raw_readw(BCR2 as *const u16);
    bcr_mask |= 1 << 10;
    __raw_writew(bcr_mask, BCR2 as *mut u16);

    platform_add_devices(POLARIS_DEVICES.as_mut_ptr(), POLARIS_DEVICES.len())
}

// arch_initcall(polaris_initialise);

static mut IPR_IRQ_TABLE: [ipr_data; 2] = [
    /* External IRQs */
    ipr_data { irq: IRQ0_IRQ, offset: 0, shift: 0, priority: 1 }, /* IRQ0 */
    ipr_data { irq: IRQ1_IRQ, offset: 0, shift: 4, priority: 1 }, /* IRQ1 */
];

static mut IPR_OFFSETS: [u32; 1] = [INTC_IPRC];

static mut IPR_IRQ_DESC: ipr_desc = ipr_desc {
    ipr_offsets: IPR_OFFSETS.as_mut_ptr(),
    nr_offsets: IPR_OFFSETS.len(),

    ipr_data: IPR_IRQ_TABLE.as_mut_ptr(),
    nr_irqs: IPR_IRQ_TABLE.len(),
    chip: irq_chip {
        name: "sh7709-ext",
    },
};

unsafe fn init_polaris_irq() {
    /* Disable all interrupts */
    __raw_writew(0, BCR_ILCRA as *mut u16);
    __raw_writew(0, BCR_ILCRB as *mut u16);
    __raw_writew(0, BCR_ILCRC as *mut u16);
    __raw_writew(0, BCR_ILCRD as *mut u16);
    __raw_writew(0, BCR_ILCRE as *mut u16);
    __raw_writew(0, BCR_ILCRF as *mut u16);
    __raw_writew(0, BCR_ILCRG as *mut u16);

    register_ipr_controller(&mut IPR_IRQ_DESC);
}

static mut MV_POLARIS: sh_machine_vector = sh_machine_vector {
    mv_name: "Polaris",
    mv_init_irq: Some(init_polaris_irq),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
