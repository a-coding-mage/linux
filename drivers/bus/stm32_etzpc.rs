// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2023, STMicroelectronics - All Rights Reserved
 */

/* Dependencies are supplied by the surrounding kernel translation. */

/*
 * ETZPC registers
 */
const ETZPC_DECPROT: u32 = 0x10;
const ETZPC_HWCFGR: u32 = 0x3f0;

/*
 * HWCFGR register
 */
const ETZPC_HWCFGR_NUM_TZMA: u32 = 0xff;
const ETZPC_HWCFGR_NUM_PER_SEC: u32 = 0xff00;
const ETZPC_HWCFGR_NUM_AHB_SEC: u32 = 0xff0000;
const ETZPC_HWCFGR_CHUNKS1N4: u32 = 0xff000000;

/*
 * ETZPC miscellaneous
 */
const ETZPC_PROT_MASK: u32 = 0x3;
const ETZPC_PROT_A7NS: u32 = 0x3;
const ETZPC_DECPROT_SHIFT: u32 = 1;

const IDS_PER_DECPROT_REGS: u32 = 16;

unsafe fn stm32_etzpc_grant_access(
    ctrl: *mut stm32_firewall_controller,
    firewall_id: u32,
) -> i32 {
    let mut offset: u32;
    let reg_offset: u32;
    let sec_val: u32;

    if firewall_id >= (*ctrl).max_entries {
        dev_err((*ctrl).dev, "Invalid sys bus ID %u", firewall_id);
        return -EINVAL;
    }

    /* Check access configuration, 16 peripherals per register */
    reg_offset = ETZPC_DECPROT
        .wrapping_add(0x4u32.wrapping_mul(firewall_id / IDS_PER_DECPROT_REGS));
    offset = (firewall_id % IDS_PER_DECPROT_REGS) << ETZPC_DECPROT_SHIFT;

    /* Verify peripheral is non-secure and attributed to cortex A7 */
    sec_val = (readl((*ctrl).mmio.add(reg_offset as usize)) >> offset) & ETZPC_PROT_MASK;
    if sec_val != ETZPC_PROT_A7NS {
        dev_dbg(
            (*ctrl).dev,
            "Invalid bus configuration: reg_offset %#x, value %d\n",
            reg_offset,
            sec_val,
        );
        return -EACCES;
    }

    0
}

unsafe fn stm32_etzpc_release_access(
    _ctrl: *mut stm32_firewall_controller,
    _firewall_id: u32,
) {
}

unsafe fn stm32_etzpc_probe(pdev: *mut platform_device) -> i32 {
    let etzpc_controller: *mut stm32_firewall_controller;
    let np: *mut device_node = (*pdev).dev.of_node;
    let nb_per: u32;
    let nb_master: u32;
    let mut res: *mut resource = core::ptr::null_mut();
    let mmio: *mut u8;
    let mut rc: i32;

    etzpc_controller = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<stm32_firewall_controller>(),
        GFP_KERNEL,
    ) as *mut stm32_firewall_controller;
    if etzpc_controller.is_null() {
        return -ENOMEM;
    }

    mmio = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if is_err(mmio) {
        return ptr_err(mmio);
    }

    (*etzpc_controller).dev = &mut (*pdev).dev;
    (*etzpc_controller).mmio = mmio;
    (*etzpc_controller).name = dev_driver_string((*etzpc_controller).dev);
    (*etzpc_controller).type_ = STM32_PERIPHERAL_FIREWALL | STM32_MEMORY_FIREWALL;
    (*etzpc_controller).grant_access = Some(stm32_etzpc_grant_access);
    (*etzpc_controller).release_access = Some(stm32_etzpc_release_access);

    /* Get number of etzpc entries*/
    nb_per = field_get(ETZPC_HWCFGR_NUM_PER_SEC, readl(mmio.add(ETZPC_HWCFGR as usize)));
    nb_master = field_get(ETZPC_HWCFGR_NUM_AHB_SEC, readl(mmio.add(ETZPC_HWCFGR as usize)));
    (*etzpc_controller).max_entries = nb_per + nb_master;

    platform_set_drvdata(pdev, etzpc_controller);

    rc = stm32_firewall_controller_register(etzpc_controller);
    if rc != 0 {
        dev_err(
            (*etzpc_controller).dev,
            "Couldn't register as a firewall controller: %d",
            rc,
        );
        return rc;
    }

    rc = stm32_firewall_populate_bus(etzpc_controller);
    if rc != 0 {
        dev_err(
            (*etzpc_controller).dev,
            "Couldn't populate ETZPC bus: %d",
            rc,
        );
        return rc;
    }

    /* Populate all allowed nodes */
    of_platform_populate(np, core::ptr::null(), core::ptr::null(), &mut (*pdev).dev)
}

static stm32_etzpc_of_match: [of_device_id; 2] = [
    of_device_id { compatible: "st,stm32-etzpc" },
    of_device_id { ..Default::default() },
];

static mut stm32_etzpc_driver: platform_driver = platform_driver {
    probe: Some(stm32_etzpc_probe),
    driver: device_driver {
        name: "stm32-etzpc",
        of_match_table: stm32_etzpc_of_match.as_ptr(),
    },
};

module_platform_driver!(stm32_etzpc_driver);

module_author!("Gatien Chevallier <gatien.chevallier@foss.st.com>");
module_description!("STMicroelectronics ETZPC driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
