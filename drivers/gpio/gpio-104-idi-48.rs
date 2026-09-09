// SPDX-License-Identifier: GPL-2.0-only
/*
 * GPIO driver for the ACCES 104-IDI-48 family
 * Copyright (C) 2015 William Breathitt Gray
 *
 * This driver supports the following ACCES devices: 104-IDI-48A,
 * 104-IDI-48AC, 104-IDI-48B, and 104-IDI-48BC.
 */

// Kernel dependencies supplied by the surrounding Rust kernel environment.

const IDI_48_EXTENT: usize = 8;
const MAX_NUM_IDI_48: usize = max_num_isa_dev(IDI_48_EXTENT);

static mut BASE: [u32; MAX_NUM_IDI_48] = [0; MAX_NUM_IDI_48];
static mut NUM_IDI_48: u32 = 0;

static mut IRQ: [u32; MAX_NUM_IDI_48] = [0; MAX_NUM_IDI_48];
static mut NUM_IRQ: u32 = 0;

const IDI48_IRQ_STATUS: u32 = 0x7;
const IDI48_IRQ_ENABLE: u32 = IDI48_IRQ_STATUS;

unsafe fn idi_48_reg_mask_xlate(
    _gpio: *mut gpio_regmap,
    _op: gpio_regmap_operation,
    base: u32,
    offset: u32,
    reg: *mut u32,
    mask: *mut u32,
) -> i32 {
    let line = offset % 8;
    let stride = offset / 8;
    let port = (stride / 3) * 4;
    let port_stride = stride % 3;

    *reg = base + port + port_stride;
    *mask = 1u32.wrapping_shl(line);

    0
}

static IDI_48_WR_RANGES: [regmap_range; 1] = [regmap_reg_range(0x0, 0x6)];
static IDI_48_RD_RANGES: [regmap_range; 2] = [
    regmap_reg_range(0x0, 0x2),
    regmap_reg_range(0x4, 0x7),
];
static IDI_48_PRECIOUS_RANGES: [regmap_range; 1] = [regmap_reg_range(0x7, 0x7)];

static IDI_48_WR_TABLE: regmap_access_table = regmap_access_table {
    no_ranges: IDI_48_WR_RANGES.as_ptr(),
    n_no_ranges: IDI_48_WR_RANGES.len(),
};
static IDI_48_RD_TABLE: regmap_access_table = regmap_access_table {
    yes_ranges: IDI_48_RD_RANGES.as_ptr(),
    n_yes_ranges: IDI_48_RD_RANGES.len(),
};
static IDI_48_PRECIOUS_TABLE: regmap_access_table = regmap_access_table {
    yes_ranges: IDI_48_PRECIOUS_RANGES.as_ptr(),
    n_yes_ranges: IDI_48_PRECIOUS_RANGES.len(),
};
static IDI48_REGMAP_CONFIG: regmap_config = regmap_config {
    reg_bits: 8,
    reg_stride: 1,
    val_bits: 8,
    io_port: true,
    max_register: 0x6,
    wr_table: &IDI_48_WR_TABLE,
    rd_table: &IDI_48_RD_TABLE,
    precious_table: &IDI_48_PRECIOUS_TABLE,
    use_raw_spinlock: true,
};

const IDI48_NGPIO: usize = 48;

const fn idi48_regmap_irq(id: usize) -> regmap_irq {
    regmap_irq {
        mask: 1u32 << (id / 8),
        type_: irq_type { types_supported: IRQ_TYPE_EDGE_BOTH },
    }
}

static IDI48_REGMAP_IRQS: [regmap_irq; IDI48_NGPIO] = [
    idi48_regmap_irq(0), idi48_regmap_irq(1), idi48_regmap_irq(2),
    idi48_regmap_irq(3), idi48_regmap_irq(4), idi48_regmap_irq(5),
    idi48_regmap_irq(6), idi48_regmap_irq(7), idi48_regmap_irq(8),
    idi48_regmap_irq(9), idi48_regmap_irq(10), idi48_regmap_irq(11),
    idi48_regmap_irq(12), idi48_regmap_irq(13), idi48_regmap_irq(14),
    idi48_regmap_irq(15), idi48_regmap_irq(16), idi48_regmap_irq(17),
    idi48_regmap_irq(18), idi48_regmap_irq(19), idi48_regmap_irq(20),
    idi48_regmap_irq(21), idi48_regmap_irq(22), idi48_regmap_irq(23),
    idi48_regmap_irq(24), idi48_regmap_irq(25), idi48_regmap_irq(26),
    idi48_regmap_irq(27), idi48_regmap_irq(28), idi48_regmap_irq(29),
    idi48_regmap_irq(30), idi48_regmap_irq(31), idi48_regmap_irq(32),
    idi48_regmap_irq(33), idi48_regmap_irq(34), idi48_regmap_irq(35),
    idi48_regmap_irq(36), idi48_regmap_irq(37), idi48_regmap_irq(38),
    idi48_regmap_irq(39), idi48_regmap_irq(40), idi48_regmap_irq(41),
    idi48_regmap_irq(42), idi48_regmap_irq(43), idi48_regmap_irq(44),
    idi48_regmap_irq(45), idi48_regmap_irq(46), idi48_regmap_irq(47),
];

static IDI48_NAMES: [&str; IDI48_NGPIO] = [
    "Bit 0 A", "Bit 1 A", "Bit 2 A", "Bit 3 A", "Bit 4 A", "Bit 5 A",
    "Bit 6 A", "Bit 7 A", "Bit 8 A", "Bit 9 A", "Bit 10 A", "Bit 11 A",
    "Bit 12 A", "Bit 13 A", "Bit 14 A", "Bit 15 A", "Bit 16 A", "Bit 17 A",
    "Bit 18 A", "Bit 19 A", "Bit 20 A", "Bit 21 A", "Bit 22 A", "Bit 23 A",
    "Bit 0 B", "Bit 1 B", "Bit 2 B", "Bit 3 B", "Bit 4 B", "Bit 5 B",
    "Bit 6 B", "Bit 7 B", "Bit 8 B", "Bit 9 B", "Bit 10 B", "Bit 11 B",
    "Bit 12 B", "Bit 13 B", "Bit 14 B", "Bit 15 B", "Bit 16 B", "Bit 17 B",
    "Bit 18 B", "Bit 19 B", "Bit 20 B", "Bit 21 B", "Bit 22 B", "Bit 23 B",
];

unsafe fn idi_48_probe(dev: *mut device, id: usize) -> i32 {
    let name = dev_name(dev);
    let mut config: gpio_regmap_config = core::mem::zeroed();
    let regs: *mut core::ffi::c_void;
    let map: *mut regmap;
    let chip: *mut regmap_irq_chip;
    let mut chip_data: *mut regmap_irq_chip_data = core::ptr::null_mut();
    let mut err: i32;

    if devm_request_region(dev, BASE[id], IDI_48_EXTENT, name).is_null() {
        dev_err(dev, "Unable to lock port addresses (0x%X-0x%X)\n", BASE[id], BASE[id] + IDI_48_EXTENT as u32);
        return -EBUSY;
    }

    regs = devm_ioport_map(dev, BASE[id], IDI_48_EXTENT);
    if regs.is_null() { return -ENOMEM; }

    map = devm_regmap_init_mmio(dev, regs, &IDI48_REGMAP_CONFIG);
    if IS_ERR(map) { return dev_err_probe(dev, PTR_ERR(map), "Unable to initialize register map\n"); }

    chip = devm_kzalloc(dev, core::mem::size_of::<regmap_irq_chip>(), GFP_KERNEL) as *mut regmap_irq_chip;
    if chip.is_null() { return -ENOMEM; }
    (*chip).name = name;
    (*chip).status_base = IDI48_IRQ_STATUS;
    (*chip).unmask_base = IDI48_IRQ_ENABLE;
    (*chip).clear_on_unmask = true;
    (*chip).num_regs = 1;
    (*chip).irqs = IDI48_REGMAP_IRQS.as_ptr();
    (*chip).num_irqs = IDI48_REGMAP_IRQS.len();

    err = devm_regmap_add_irq_chip(dev, map, IRQ[id], IRQF_SHARED, 0, chip, &mut chip_data);
    if err != 0 { return dev_err_probe(dev, err, "IRQ registration failed\n"); }

    config.parent = dev;
    config.regmap = map;
    config.ngpio = IDI48_NGPIO;
    config.names = IDI48_NAMES.as_ptr();
    config.reg_dat_base = GPIO_REGMAP_ADDR(0x0);
    config.ngpio_per_reg = 8;
    config.reg_mask_xlate = Some(idi_48_reg_mask_xlate);
    config.irq_domain = regmap_irq_get_domain(chip_data);

    PTR_ERR_OR_ZERO(devm_gpio_regmap_register(dev, &config))
}

static mut IDI_48_DRIVER: isa_driver = isa_driver {
    probe: Some(idi_48_probe),
    driver: driver { name: "104-idi-48" },
};

module_isa_driver_with_irq!(IDI_48_DRIVER, NUM_IDI_48, NUM_IRQ);

MODULE_AUTHOR!("William Breathitt Gray <vilhelm.gray@gmail.com>");
MODULE_DESCRIPTION!("ACCES 104-IDI-48 GPIO driver");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
