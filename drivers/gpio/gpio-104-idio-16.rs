// SPDX-License-Identifier: GPL-2.0-only
/*
 * GPIO driver for the ACCES 104-IDIO-16 family
 * Copyright (C) 2015 William Breathitt Gray
 *
 * This driver supports the following ACCES devices: 104-IDIO-16,
 * 104-IDIO-16E, 104-IDO-16, 104-IDIO-8, 104-IDIO-8E, and 104-IDO-8.
 */

// Kernel dependencies are supplied by the surrounding Rust kernel bindings.
use crate::gpio_idio_16::*;

const IDIO_16_EXTENT: usize = 8;
const MAX_NUM_IDIO_16: usize = max_num_isa_dev(IDIO_16_EXTENT);

static mut BASE: [u32; MAX_NUM_IDIO_16] = [0; MAX_NUM_IDIO_16];
static mut NUM_IDIO_16: u32 = 0;

static mut IRQ: [u32; MAX_NUM_IDIO_16] = [0; MAX_NUM_IDIO_16];
static mut NUM_IRQ: u32 = 0;

static IDIO_16_WR_RANGES: [RegmapRange; 2] = [
    regmap_reg_range(0x0, 0x2),
    regmap_reg_range(0x4, 0x4),
];
static IDIO_16_RD_RANGES: [RegmapRange; 2] = [
    regmap_reg_range(0x1, 0x2),
    regmap_reg_range(0x5, 0x5),
];
static IDIO_16_PRECIOUS_RANGES: [RegmapRange; 1] = [
    regmap_reg_range(0x2, 0x2),
];
static IDIO_16_WR_TABLE: RegmapAccessTable = RegmapAccessTable {
    yes_ranges: IDIO_16_WR_RANGES.as_ptr(),
    n_yes_ranges: IDIO_16_WR_RANGES.len(),
};
static IDIO_16_RD_TABLE: RegmapAccessTable = RegmapAccessTable {
    yes_ranges: IDIO_16_RD_RANGES.as_ptr(),
    n_yes_ranges: IDIO_16_RD_RANGES.len(),
};
static IDIO_16_PRECIOUS_TABLE: RegmapAccessTable = RegmapAccessTable {
    yes_ranges: IDIO_16_PRECIOUS_RANGES.as_ptr(),
    n_yes_ranges: IDIO_16_PRECIOUS_RANGES.len(),
};
static IDIO_16_REGMAP_CONFIG: RegmapConfig = RegmapConfig {
    reg_bits: 8,
    reg_stride: 1,
    val_bits: 8,
    io_port: true,
    max_register: 0x5,
    wr_table: &IDIO_16_WR_TABLE,
    rd_table: &IDIO_16_RD_TABLE,
    volatile_table: &IDIO_16_RD_TABLE,
    precious_table: &IDIO_16_PRECIOUS_TABLE,
    cache_type: REGCACHE_FLAT,
    use_raw_spinlock: true,
};

/* Only input lines (GPIO 16-31) support interrupts */
static IDIO_16_REGMAP_IRQS: [RegmapIrq; 32] = {
    let mut irqs = [RegmapIrq::default(); 32];
    let mut id = 0;
    while id < 16 {
        irqs[16 + id] = RegmapIrq {
            mask: 1u32 << id,
            irq_type: RegmapIrqType {
                types_supported: IRQ_TYPE_EDGE_BOTH,
            },
            ..RegmapIrq::default()
        };
        id += 1;
    }
    irqs
};

unsafe fn idio_16_probe(dev: *mut Device, id: usize) -> i32 {
    let name = dev_name(dev);
    let mut config = Idio16RegmapConfig::default();
    let regs: *mut core::ffi::c_void;
    let map: *mut Regmap;

    if devm_request_region(dev, BASE[id], IDIO_16_EXTENT, name).is_null() {
        dev_err(
            dev,
            "Unable to lock port addresses (0x%X-0x%X)\n",
            BASE[id],
            BASE[id] + IDIO_16_EXTENT as u32,
        );
        return -EBUSY;
    }

    regs = devm_ioport_map(dev, BASE[id], IDIO_16_EXTENT);
    if regs.is_null() {
        return -ENOMEM;
    }

    map = devm_regmap_init_mmio(dev, regs, &IDIO_16_REGMAP_CONFIG);
    if IS_ERR(map) {
        return dev_err_probe(dev, PTR_ERR(map), "Unable to initialize register map\n");
    }

    config.parent = dev;
    config.map = map;
    config.regmap_irqs = IDIO_16_REGMAP_IRQS.as_ptr();
    config.num_regmap_irqs = IDIO_16_REGMAP_IRQS.len();
    config.irq = IRQ[id];
    config.no_status = true;

    devm_idio_16_regmap_register(dev, &config)
}

static mut IDIO_16_DRIVER: IsaDriver = IsaDriver {
    probe: Some(idio_16_probe),
    driver: Driver {
        name: "104-idio-16",
    },
};

module_isa_driver_with_irq!(IDIO_16_DRIVER, NUM_IDIO_16, NUM_IRQ);

module_author!("William Breathitt Gray <vilhelm.gray@gmail.com>");
module_description!("ACCES 104-IDIO-16 GPIO driver");
module_license!("GPL v2");
module_import_ns!("GPIO_IDIO_16");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
