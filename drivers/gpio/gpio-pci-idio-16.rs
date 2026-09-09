// SPDX-License-Identifier: GPL-2.0-only
/*
 * GPIO driver for the ACCES PCI-IDIO-16
 * Copyright (C) 2017 William Breathitt Gray
 */

// Linux kernel dependencies supplied by the surrounding translation unit:
// linux/bits.h, linux/device.h, linux/err.h, linux/irq.h, linux/kernel.h,
// linux/module.h, linux/pci.h, linux/regmap.h, linux/types.h, gpio-idio-16.h

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    fn regmap_reg_range(first: c_uint, last: c_uint) -> regmap_range;
    fn pcim_enable_device(pdev: *mut pci_dev) -> c_int;
    fn pcim_iomap_region(pdev: *mut pci_dev, bar: usize, name: *const c_char) -> *mut c_void;
    fn pci_name(pdev: *mut pci_dev) -> *const c_char;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char) -> c_int;
    fn ptr_err(ptr: *const c_void) -> c_int;
    fn devm_idio_16_regmap_register(
        dev: *mut device,
        config: *mut idio_16_regmap_config,
    ) -> c_int;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub irq: c_int,
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap_range {
    pub range_min: c_uint,
    pub range_max: c_uint,
}
#[repr(C)]
pub struct regmap_access_table {
    pub yes_ranges: *const regmap_range,
    pub n_yes_ranges: usize,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub reg_stride: c_uint,
    pub val_bits: c_uint,
    pub io_port: bool,
    pub max_register: c_uint,
    pub wr_table: *const regmap_access_table,
    pub rd_table: *const regmap_access_table,
    pub volatile_table: *const regmap_access_table,
    pub precious_table: *const regmap_access_table,
    pub cache_type: c_uint,
    pub use_raw_spinlock: bool,
}
#[repr(C)]
pub struct regmap_irq_type {
    pub types_supported: c_ulong,
}
#[repr(C)]
pub struct regmap_irq {
    pub mask: c_ulong,
    pub type_: regmap_irq_type,
}
#[repr(C)]
pub struct idio_16_regmap_config {
    pub parent: *mut device,
    pub map: *mut regmap,
    pub regmap_irqs: *const regmap_irq,
    pub num_regmap_irqs: usize,
    pub irq: c_int,
    pub filters: bool,
}
#[repr(C)]
pub struct pci_device_id {
    pub vendor: u16,
    pub device: u16,
}
#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
}

const IRQ_TYPE_EDGE_BOTH: c_ulong = 0x3;
const REGCACHE_FLAT: c_uint = 2;

static IDIO_16_WR_RANGES: [regmap_range; 2] = [
    regmap_range { range_min: 0x0, range_max: 0x2 },
    regmap_range { range_min: 0x3, range_max: 0x4 },
];
static IDIO_16_RD_RANGES: [regmap_range; 2] = [
    regmap_range { range_min: 0x1, range_max: 0x2 },
    regmap_range { range_min: 0x5, range_max: 0x6 },
];
static IDIO_16_PRECIOUS_RANGES: [regmap_range; 1] = [
    regmap_range { range_min: 0x2, range_max: 0x2 },
];
static IDIO_16_WR_TABLE: regmap_access_table = regmap_access_table {
    yes_ranges: IDIO_16_WR_RANGES.as_ptr(), n_yes_ranges: 2,
};
static IDIO_16_RD_TABLE: regmap_access_table = regmap_access_table {
    yes_ranges: IDIO_16_RD_RANGES.as_ptr(), n_yes_ranges: 2,
};
static IDIO_16_PRECIOUS_TABLE: regmap_access_table = regmap_access_table {
    yes_ranges: IDIO_16_PRECIOUS_RANGES.as_ptr(), n_yes_ranges: 1,
};
static IDIO_16_REGMAP_CONFIG: regmap_config = regmap_config {
    reg_bits: 8, reg_stride: 1, val_bits: 8, io_port: true, max_register: 0x7,
    wr_table: &IDIO_16_WR_TABLE, rd_table: &IDIO_16_RD_TABLE,
    volatile_table: &IDIO_16_RD_TABLE, precious_table: &IDIO_16_PRECIOUS_TABLE,
    cache_type: REGCACHE_FLAT, use_raw_spinlock: true,
};

/* Only input lines (GPIO 16-31) support interrupts */
static IDIO_16_REGMAP_IRQS: [regmap_irq; 32] = {
    let mut irqs = [regmap_irq { mask: 0, type_: regmap_irq_type { types_supported: 0 } }; 32];
    let mut id = 0;
    while id < 16 {
        irqs[16 + id] = regmap_irq {
            mask: 1 << 2,
            type_: regmap_irq_type { types_supported: IRQ_TYPE_EDGE_BOTH },
        };
        id += 1;
    }
    irqs
};

static mut IDIO_16_DRIVER: pci_driver = pci_driver {
    name: b"pci-idio-16\0".as_ptr() as *const c_char,
    id_table: IDIO_16_PCI_DEV_ID.as_ptr(),
    probe: Some(idio_16_probe),
};

static IDIO_16_PCI_DEV_ID: [pci_device_id; 2] = [
    pci_device_id { vendor: 0x494F, device: 0x0DC8 },
    pci_device_id { vendor: 0, device: 0 },
];

unsafe extern "C" fn idio_16_probe(pdev: *mut pci_dev, _id: *const pci_device_id) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let pci_bar_index: usize = 2;
    let mut config: idio_16_regmap_config = core::mem::zeroed();

    let err = pcim_enable_device(pdev);
    if err != 0 {
        return dev_err_probe(dev, err, b"Failed to enable PCI device\n\0".as_ptr() as *const c_char);
    }

    let regs = pcim_iomap_region(pdev, pci_bar_index, pci_name(pdev));
    if (regs as isize) < 0 {
        return dev_err_probe(dev, ptr_err(regs), b"Unable to map PCI I/O addresses\n\0".as_ptr() as *const c_char);
    }

    let map = devm_regmap_init_mmio(dev, regs, &IDIO_16_REGMAP_CONFIG);
    if (map as isize) < 0 {
        return dev_err_probe(dev, ptr_err(map as *const c_void), b"Unable to initialize register map\n\0".as_ptr() as *const c_char);
    }

    config.parent = dev;
    config.map = map;
    config.regmap_irqs = IDIO_16_REGMAP_IRQS.as_ptr();
    config.num_regmap_irqs = IDIO_16_REGMAP_IRQS.len();
    config.irq = (*pdev).irq;
    config.filters = true;

    devm_idio_16_regmap_register(dev, &mut config)
}

// MODULE_DEVICE_TABLE(pci, idio_16_pci_dev_id);
// module_pci_driver(idio_16_driver);
// MODULE_AUTHOR("William Breathitt Gray <vilhelm.gray@gmail.com>");
// MODULE_DESCRIPTION("ACCES PCI-IDIO-16 GPIO driver");
// MODULE_LICENSE("GPL v2");
// MODULE_IMPORT_NS("GPIO_IDIO_16");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
