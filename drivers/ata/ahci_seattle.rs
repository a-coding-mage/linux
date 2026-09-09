// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Seattle AHCI SATA driver
 *
 * Copyright (c) 2015, Advanced Micro Devices
 * Author: Brijesh Singh <brijesh.singh@amd.com>
 *
 * based on the AHCI SATA platform driver by Jeff Garzik and Anton Vorontsov
 */

// Linux kernel dependencies: kernel, module, pm, device, platform_device,
// libata, ahci_platform, acpi, pci_ids, and "ahci.h".

const fn activity_bit_pos(x: u32) -> u32 { 8 + (3 * x) }
const fn locate_bit_pos(x: u32) -> u32 { activity_bit_pos(x) + 1 }
const fn fault_bit_pos(x: u32) -> u32 { locate_bit_pos(x) + 1 }

const ACTIVITY_MASK: u32 = 0x0001_0000;
const LOCATE_MASK: u32 = 0x0008_0000;
const FAULT_MASK: u32 = 0x0040_0000;
const DRV_NAME: &str = "ahci-seattle";

#[repr(C)]
struct seattle_plat_data {
    sgpio_ctrl: *mut core::ffi::c_void,
}

#[repr(C)]
static mut ahci_port_ops: ata_port_operations = ata_port_operations {
    inherits: unsafe { &ahci_ops },
};

#[repr(C)]
static ahci_port_info: ata_port_info = ata_port_info {
    flags: AHCI_FLAG_COMMON,
    pio_mask: ATA_PIO4,
    udma_mask: ATA_UDMA6,
    port_ops: unsafe { &ahci_port_ops },
};

#[repr(C)]
static mut ahci_seattle_ops: ata_port_operations = ata_port_operations {
    inherits: unsafe { &ahci_ops },
    transmit_led_message: Some(seattle_transmit_led_message),
};

#[repr(C)]
static ahci_port_seattle_info: ata_port_info = ata_port_info {
    flags: AHCI_FLAG_COMMON | ATA_FLAG_EM | ATA_FLAG_SW_ACTIVITY,
    link_flags: ATA_LFLAG_SW_ACTIVITY,
    pio_mask: ATA_PIO4,
    udma_mask: ATA_UDMA6,
    port_ops: unsafe { &ahci_seattle_ops },
};

static ahci_platform_sht: scsi_host_template = scsi_host_template {
    // AHCI_SHT(DRV_NAME)
};

unsafe extern "C" fn seattle_transmit_led_message_impl(
    ap: *mut ata_port,
    state: u32,
    size: isize,
) -> isize {
    let hpriv = (*(*ap).host).private_data as *mut ahci_host_priv;
    let pp = (*ap).private_data as *mut ahci_port_priv;
    let plat_data = (*hpriv).plat_data as *mut seattle_plat_data;
    let mut flags: c_ulong = 0;
    let pmp: i32;
    let emp: *mut ahci_em_priv;
    let mut val: u32;

    /* get the slot number from the message */
    pmp = ((state & EM_MSG_LED_PMP_SLOT) >> 8) as i32;
    if pmp >= EM_MAX_SLOTS {
        return -EINVAL as isize;
    }
    emp = (*pp).em_priv.as_mut_ptr().add(pmp as usize);

    val = ioread32((*plat_data).sgpio_ctrl as *const u32);
    if state & ACTIVITY_MASK != 0 {
        val |= 1u32 << activity_bit_pos((*ap).port_no as u32);
    } else {
        val &= !(1u32 << activity_bit_pos((*ap).port_no as u32));
    }

    if state & LOCATE_MASK != 0 {
        val |= 1u32 << locate_bit_pos((*ap).port_no as u32);
    } else {
        val &= !(1u32 << locate_bit_pos((*ap).port_no as u32));
    }

    if state & FAULT_MASK != 0 {
        val |= 1u32 << fault_bit_pos((*ap).port_no as u32);
    } else {
        val &= !(1u32 << fault_bit_pos((*ap).port_no as u32));
    }

    iowrite32(val, (*plat_data).sgpio_ctrl);
    spin_lock_irqsave((*ap).lock, &mut flags);

    /* save off new led state for port/slot */
    (*emp).led_state = state;

    spin_unlock_irqrestore((*ap).lock, flags);
    size
}

unsafe extern "C" fn seattle_transmit_led_message(
    ap: *mut ata_port,
    state: u32,
    size: isize,
) -> isize {
    seattle_transmit_led_message_impl(ap, state, size)
}

unsafe extern "C" fn ahci_seattle_get_port_info(
    pdev: *mut platform_device,
    hpriv: *mut ahci_host_priv,
) -> *const ata_port_info {
    let dev = &mut (*pdev).dev;
    let plat_data = devm_kzalloc(dev, core::mem::size_of::<seattle_plat_data>(), GFP_KERNEL)
        as *mut seattle_plat_data;
    let val: u32;

    if plat_data.is_null() {
        return &ahci_port_info;
    }

    (*plat_data).sgpio_ctrl = devm_platform_ioremap_resource(pdev, 1);
    if is_err((*plat_data).sgpio_ctrl) {
        return &ahci_port_info;
    }

    val = ioread32((*plat_data).sgpio_ctrl as *const u32);
    if val & 0xf == 0 {
        return &ahci_port_info;
    }

    (*hpriv).em_loc = 0;
    (*hpriv).em_buf_sz = 4;
    (*hpriv).em_msg_type = EM_MSG_TYPE_LED;
    (*hpriv).plat_data = plat_data as *mut core::ffi::c_void;

    dev_info(dev, "SGPIO LED control is enabled.\n");
    &ahci_port_seattle_info
}

unsafe extern "C" fn ahci_seattle_probe(pdev: *mut platform_device) -> i32 {
    let mut rc: i32;
    let hpriv = ahci_platform_get_resources(pdev, 0);
    if is_err(hpriv) {
        return ptr_err(hpriv);
    }

    rc = ahci_platform_enable_resources(hpriv);
    if rc != 0 {
        return rc;
    }

    rc = ahci_platform_init_host(
        pdev,
        hpriv,
        ahci_seattle_get_port_info(pdev, hpriv),
        &ahci_platform_sht,
    );
    if rc != 0 {
        ahci_platform_disable_resources(hpriv);
    }
    rc
}

// SIMPLE_DEV_PM_OPS(ahci_pm_ops, ahci_platform_suspend, ahci_platform_resume)
static ahci_pm_ops: dev_pm_ops = dev_pm_ops {
    suspend: Some(ahci_platform_suspend),
    resume: Some(ahci_platform_resume),
};

static ahci_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id { id: *b"AMDI0600\0", driver_data: 0 },
    acpi_device_id { id: [0; 9], driver_data: 0 },
];

static mut ahci_seattle_driver: platform_driver = platform_driver {
    probe: Some(ahci_seattle_probe),
    remove: Some(ata_platform_remove_one),
    driver: device_driver {
        name: DRV_NAME,
        acpi_match_table: &ahci_acpi_match,
        pm: &ahci_pm_ops,
    },
};

// module_platform_driver(ahci_seattle_driver);
// MODULE_DEVICE_TABLE(acpi, ahci_acpi_match);
// MODULE_DESCRIPTION("Seattle AHCI SATA platform driver");
// MODULE_AUTHOR("Brijesh Singh <brijesh.singh@amd.com>");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
