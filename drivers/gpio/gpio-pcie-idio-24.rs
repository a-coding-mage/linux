// SPDX-License-Identifier: GPL-2.0-only
/*
 * GPIO driver for the ACCES PCIe-IDIO-24 family
 * Copyright (C) 2018 William Breathitt Gray
 *
 * This driver supports the following ACCES devices: PCIe-IDIO-24,
 * PCIe-IDI-24, PCIe-IDO-24, and PCIe-IDIO-12.
 */

// External Linux kernel declarations and macros are supplied by the build environment.

const PLX_PEX8311_PCI_LCS_INTCSR: u32 = 0x68;
const INTCSR_INTERNAL_PCI_WIRE: u32 = 1 << 8;
const INTCSR_LOCAL_INPUT: u32 = 1 << 11;
const IDIO_24_ENABLE_IRQ: u32 = INTCSR_INTERNAL_PCI_WIRE | INTCSR_LOCAL_INPUT;

const IDIO_24_OUT_BASE: u32 = 0x0;
const IDIO_24_TTLCMOS_OUT_REG: u32 = 0x3;
const IDIO_24_IN_BASE: u32 = 0x4;
const IDIO_24_TTLCMOS_IN_REG: u32 = 0x7;
const IDIO_24_COS_STATUS_BASE: u32 = 0x8;
const IDIO_24_CONTROL_REG: u32 = 0xC;
const IDIO_24_COS_ENABLE: u32 = 0xE;
const IDIO_24_SOFT_RESET: u32 = 0xF;

const CONTROL_REG_OUT_MODE: u32 = 1 << 1;
const COS_ENABLE_RISING: u8 = 1 << 1;
const COS_ENABLE_FALLING: u8 = 1 << 4;
const COS_ENABLE_BOTH: u8 = COS_ENABLE_RISING | COS_ENABLE_FALLING;

static pex8311_intcsr_regmap_config: regmap_config = regmap_config {
    name: "pex8311_intcsr",
    reg_bits: 32,
    reg_stride: 1,
    reg_base: PLX_PEX8311_PCI_LCS_INTCSR,
    val_bits: 32,
    io_port: true,
};

static idio_24_wr_ranges: [regmap_range; 3] = [
    regmap_reg_range(0x0, 0x3), regmap_reg_range(0x8, 0xC), regmap_reg_range(0xE, 0xF),
];
static idio_24_rd_ranges: [regmap_range; 2] = [
    regmap_reg_range(0x0, 0xC), regmap_reg_range(0xE, 0xF),
];
static idio_24_volatile_ranges: [regmap_range; 2] = [
    regmap_reg_range(0x4, 0xB), regmap_reg_range(0xF, 0xF),
];
static idio_24_wr_table: regmap_access_table = regmap_access_table {
    yes_ranges: idio_24_wr_ranges.as_ptr(),
    n_yes_ranges: idio_24_wr_ranges.len(),
};
static idio_24_rd_table: regmap_access_table = regmap_access_table {
    yes_ranges: idio_24_rd_ranges.as_ptr(),
    n_yes_ranges: idio_24_rd_ranges.len(),
};
static idio_24_volatile_table: regmap_access_table = regmap_access_table {
    yes_ranges: idio_24_volatile_ranges.as_ptr(),
    n_yes_ranges: idio_24_volatile_ranges.len(),
};
static idio_24_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    reg_stride: 1,
    val_bits: 8,
    io_port: true,
    wr_table: &idio_24_wr_table,
    rd_table: &idio_24_rd_table,
    volatile_table: &idio_24_volatile_table,
    cache_type: REGCACHE_FLAT,
    use_raw_spinlock: true,
};

const IDIO_24_NGPIO_PER_REG: u32 = 8;

// The following IRQ table preserves the C macro expansion and designated indices.
static idio_24_regmap_irqs: [regmap_irq; 48] = [
    /* IIN 0-23 */
    regmap_irq { reg_offset: 0, mask: 1, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 0, mask: 2, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 0, mask: 4, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 0, mask: 8, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 0, mask: 16, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 0, mask: 32, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 0, mask: 64, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 0, mask: 128, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 1, mask: 1, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 1, mask: 2, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 1, mask: 4, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 1, mask: 8, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 1, mask: 16, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 1, mask: 32, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 1, mask: 64, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 1, mask: 128, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 2, mask: 1, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 2, mask: 2, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 2, mask: 4, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 2, mask: 8, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 2, mask: 16, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 2, mask: 32, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 2, mask: 64, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 2, mask: 128, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    /* TTL 0-7 */
    regmap_irq { reg_offset: 3, mask: 1, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 3, mask: 2, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 3, mask: 4, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 3, mask: 8, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 3, mask: 16, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 3, mask: 32, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 3, mask: 64, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 3, mask: 128, irq_type: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH } },
];

#[repr(C)]
struct idio_24_gpio {
    map: *mut regmap,
    lock: raw_spinlock_t,
    irq_type: u8,
}

unsafe fn idio_24_handle_mask_sync(index: i32, mask_buf_def: u32, mask_buf: u32, irq_drv_data: *mut core::ffi::c_void) -> i32 {
    let type_mask = (COS_ENABLE_BOTH as u32) << index;
    let idio24gpio = irq_drv_data as *mut idio_24_gpio;
    raw_spin_lock(&mut (*idio24gpio).lock);
    let type_value = if mask_buf == mask_buf_def { !type_mask as u8 } else { (*idio24gpio).irq_type };
    let ret = regmap_update_bits((*idio24gpio).map, IDIO_24_COS_ENABLE, type_mask, type_value as u32);
    raw_spin_unlock(&mut (*idio24gpio).lock);
    ret
}

unsafe fn idio_24_set_type_config(buf: *mut *mut u32, type_value: u32, irq_data: *const regmap_irq, _idx: i32, irq_drv_data: *mut core::ffi::c_void) -> i32 {
    let offset = (*irq_data).reg_offset;
    let rising = (COS_ENABLE_RISING as u32) << offset;
    let falling = (COS_ENABLE_FALLING as u32) << offset;
    let mask = (COS_ENABLE_BOTH as u32) << offset;
    let idio24gpio = irq_drv_data as *mut idio_24_gpio;
    let new_value = match type_value {
        IRQ_TYPE_EDGE_RISING => rising,
        IRQ_TYPE_EDGE_FALLING => falling,
        IRQ_TYPE_EDGE_BOTH => mask,
        _ => return -EINVAL,
    };
    raw_spin_lock(&mut (*idio24gpio).lock);
    (*idio24gpio).irq_type = (((*idio24gpio).irq_type as u32 & !mask) | (new_value & mask)) as u8;
    let mut cos_enable = 0u32;
    let mut ret = regmap_read((*idio24gpio).map, IDIO_24_COS_ENABLE, &mut cos_enable);
    if ret == 0 && (cos_enable & mask) != 0 {
        ret = regmap_update_bits((*idio24gpio).map, IDIO_24_COS_ENABLE, mask, (*idio24gpio).irq_type as u32);
    }
    raw_spin_unlock(&mut (*idio24gpio).lock);
    ret
}

unsafe fn idio_24_reg_mask_xlate(gpio: *mut gpio_regmap, _op: gpio_regmap_operation, base: u32, offset: u32, reg: *mut u32, mask: *mut u32) -> i32 {
    let out_stride = offset / IDIO_24_NGPIO_PER_REG;
    let in_stride = (offset - 24) / IDIO_24_NGPIO_PER_REG;
    let map = gpio_regmap_get_drvdata(gpio);
    match base {
        IDIO_24_OUT_BASE => {
            *mask = 1 << (offset % IDIO_24_NGPIO_PER_REG);
            if offset < 24 { *reg = IDIO_24_OUT_BASE + out_stride; return 0; }
            if offset < 48 { *reg = IDIO_24_IN_BASE + in_stride; return 0; }
            let mut ctrl_reg = 0;
            let err = regmap_read(map, IDIO_24_CONTROL_REG, &mut ctrl_reg);
            if err != 0 { return err; }
            if ctrl_reg & CONTROL_REG_OUT_MODE != 0 { *reg = IDIO_24_TTLCMOS_OUT_REG; } else { *reg = IDIO_24_TTLCMOS_IN_REG; }
            0
        }
        IDIO_24_CONTROL_REG => {
            if offset < 48 { return -ENOTSUPP; }
            *reg = IDIO_24_CONTROL_REG;
            *mask = CONTROL_REG_OUT_MODE;
            0
        }
        _ => -EINVAL,
    }
}

const IDIO_24_NGPIO: usize = 56;
static idio_24_names: [&str; IDIO_24_NGPIO] = [
    "OUT0", "OUT1", "OUT2", "OUT3", "OUT4", "OUT5", "OUT6", "OUT7", "OUT8", "OUT9", "OUT10", "OUT11", "OUT12", "OUT13", "OUT14", "OUT15", "OUT16", "OUT17", "OUT18", "OUT19", "OUT20", "OUT21", "OUT22", "OUT23",
    "IIN0", "IIN1", "IIN2", "IIN3", "IIN4", "IIN5", "IIN6", "IIN7", "IIN8", "IIN9", "IIN10", "IIN11", "IIN12", "IIN13", "IIN14", "IIN15", "IIN16", "IIN17", "IIN18", "IIN19", "IIN20", "IIN21", "IIN22", "IIN23",
    "TTL0", "TTL1", "TTL2", "TTL3", "TTL4", "TTL5", "TTL6", "TTL7",
];

// PCI probe, device table, driver registration, and module metadata are declared through external kernel APIs.
// Their direct C structure initializers are preserved below as the corresponding external interfaces.
unsafe fn idio_24_probe(pdev: *mut pci_dev, _id: *const pci_device_id) -> i32 {
    let dev = &mut (*pdev).dev;
    let pci_plx_bar_index: usize = 1;
    let pci_bar_index: usize = 2;
    let name = pci_name(pdev);
    let mut gpio_config: gpio_regmap_config = core::mem::zeroed();
    let pex8311_regs = pcim_iomap_region(pdev, pci_plx_bar_index, "pex8311");
    if IS_ERR(pex8311_regs) { return dev_err_probe(dev, PTR_ERR(pex8311_regs), "Unable to map PEX 8311 I/O addresses\n"); }
    let idio_24_regs = pcim_iomap_region(pdev, pci_bar_index, name);
    if IS_ERR(idio_24_regs) { return dev_err_probe(dev, PTR_ERR(idio_24_regs), "Unable to map PCIe-IDIO-24 I/O addresses\n"); }
    let intcsr_map = devm_regmap_init_mmio(dev, pex8311_regs, &pex8311_intcsr_regmap_config);
    if IS_ERR(intcsr_map) { return dev_err_probe(dev, PTR_ERR(intcsr_map), "Unable to initialize PEX8311 register map\n"); }
    let idio24gpio = devm_kzalloc(dev, core::mem::size_of::<idio_24_gpio>(), GFP_KERNEL) as *mut idio_24_gpio;
    if idio24gpio.is_null() { return -ENOMEM; }
    (*idio24gpio).map = devm_regmap_init_mmio(dev, idio_24_regs, &idio_24_regmap_config);
    if IS_ERR((*idio24gpio).map) { return dev_err_probe(dev, PTR_ERR((*idio24gpio).map), "Unable to initialize register map\n"); }
    raw_spin_lock_init(&mut (*idio24gpio).lock);
    (*idio24gpio).irq_type = 0xff;
    let chip = devm_kzalloc(dev, core::mem::size_of::<regmap_irq_chip>(), GFP_KERNEL) as *mut regmap_irq_chip;
    if chip.is_null() { return -ENOMEM; }
    (*chip).name = name;
    (*chip).status_base = IDIO_24_COS_STATUS_BASE;
    (*chip).mask_base = IDIO_24_COS_ENABLE;
    (*chip).ack_base = IDIO_24_COS_STATUS_BASE;
    (*chip).num_regs = 4;
    (*chip).irqs = idio_24_regmap_irqs.as_ptr();
    (*chip).num_irqs = idio_24_regmap_irqs.len();
    (*chip).handle_mask_sync = Some(idio_24_handle_mask_sync);
    (*chip).set_type_config = Some(idio_24_set_type_config);
    (*chip).irq_drv_data = idio24gpio as *mut core::ffi::c_void;
    let mut err = regmap_write((*idio24gpio).map, IDIO_24_SOFT_RESET, 0);
    if err != 0 { return err; }
    err = regmap_update_bits(intcsr_map, 0, IDIO_24_ENABLE_IRQ, IDIO_24_ENABLE_IRQ);
    if err != 0 { return err; }
    let mut chip_data = core::ptr::null_mut();
    err = devm_regmap_add_irq_chip(dev, (*idio24gpio).map, (*pdev).irq, 0, 0, chip, &mut chip_data);
    if err != 0 { return dev_err_probe(dev, err, "IRQ registration failed\n"); }
    gpio_config.parent = dev;
    gpio_config.regmap = (*idio24gpio).map;
    gpio_config.ngpio = IDIO_24_NGPIO;
    gpio_config.names = idio_24_names.as_ptr();
    gpio_config.reg_dat_base = GPIO_REGMAP_ADDR(IDIO_24_OUT_BASE);
    gpio_config.reg_set_base = GPIO_REGMAP_ADDR(IDIO_24_OUT_BASE);
    gpio_config.reg_dir_out_base = GPIO_REGMAP_ADDR(IDIO_24_CONTROL_REG);
    gpio_config.ngpio_per_reg = IDIO_24_NGPIO_PER_REG;
    gpio_config.irq_domain = regmap_irq_get_domain(chip_data);
    gpio_config.reg_mask_xlate = Some(idio_24_reg_mask_xlate);
    gpio_config.drvdata = (*idio24gpio).map as *mut core::ffi::c_void;
    PTR_ERR_OR_ZERO(devm_gpio_regmap_register(dev, &mut gpio_config))
}

static idio_24_pci_dev_id: [pci_device_id; 5] = [
    PCI_DEVICE(0x494F, 0x0FD0), PCI_DEVICE(0x494F, 0x0BD0), PCI_DEVICE(0x494F, 0x07D0), PCI_DEVICE(0x494F, 0x0FC0),
    PCI_DEVICE(0, 0),
];

static mut idio_24_driver: pci_driver = pci_driver {
    name: "pcie-idio-24",
    id_table: idio_24_pci_dev_id.as_ptr(),
    probe: Some(idio_24_probe),
};

module_pci_driver!(&mut idio_24_driver);
module_device_table!(pci, idio_24_pci_dev_id);
module_author!("William Breathitt Gray <vilhelm.gray@gmail.com>");
module_description!("ACCES PCIe-IDIO-24 GPIO driver");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
