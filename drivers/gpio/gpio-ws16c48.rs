// SPDX-License-Identifier: GPL-2.0-only
/*
 * GPIO driver for the WinSystems WS16C48
 * Copyright (C) 2016 William Breathitt Gray
 */

// Linux kernel headers and macros are supplied by the surrounding kernel bindings.

const WS16C48_EXTENT: usize = 11;
const MAX_NUM_WS16C48: usize = max_num_isa_dev(WS16C48_EXTENT);

static mut base: [u32; MAX_NUM_WS16C48] = [0; MAX_NUM_WS16C48];
static mut num_ws16c48: u32 = 0;
static mut irq: [u32; MAX_NUM_WS16C48] = [0; MAX_NUM_WS16C48];
static mut num_irq: u32 = 0;

const WS16C48_DAT_BASE: u8 = 0x0;
const WS16C48_PAGE_LOCK: u8 = 0x7;
const WS16C48_PAGE_BASE: u8 = 0x8;
const WS16C48_POL: u8 = WS16C48_PAGE_BASE;
const WS16C48_ENAB: u8 = WS16C48_PAGE_BASE;
const WS16C48_INT_ID: u8 = WS16C48_PAGE_BASE;

const PAGE_LOCK_PAGE_FIELD: u8 = 0xC0;
const POL_PAGE: u8 = 1 << 6;
const ENAB_PAGE: u8 = 2 << 6;
const INT_ID_PAGE: u8 = 3 << 6;

static mut ws16c48_wr_ranges: [regmap_range; 2] = [
    regmap_reg_range(0x0, 0x5), regmap_reg_range(0x7, 0xA),
];
static mut ws16c48_rd_ranges: [regmap_range; 1] = [regmap_reg_range(0x0, 0xA)];
static mut ws16c48_volatile_ranges: [regmap_range; 2] = [
    regmap_reg_range(0x0, 0x6), regmap_reg_range(0x8, 0xA),
];
static mut ws16c48_wr_table: regmap_access_table = regmap_access_table {
    yes_ranges: unsafe { ws16c48_wr_ranges.as_ptr() }, n_yes_ranges: 2,
};
static mut ws16c48_rd_table: regmap_access_table = regmap_access_table {
    yes_ranges: unsafe { ws16c48_rd_ranges.as_ptr() }, n_yes_ranges: 1,
};
static mut ws16c48_volatile_table: regmap_access_table = regmap_access_table {
    yes_ranges: unsafe { ws16c48_volatile_ranges.as_ptr() }, n_yes_ranges: 2,
};
static mut ws16c48_regmap_config: regmap_config = regmap_config {
    reg_bits: 8, reg_stride: 1, val_bits: 8, io_port: true,
    wr_table: unsafe { &ws16c48_wr_table }, rd_table: unsafe { &ws16c48_rd_table },
    volatile_table: unsafe { &ws16c48_volatile_table }, cache_type: REGCACHE_FLAT,
    use_raw_spinlock: true,
};

const WS16C48_NGPIO_PER_REG: usize = 8;
const WS16C48_NUM_IRQS: usize = 24;
const WS16C48_NGPIO: usize = 48;

#[repr(C)]
struct ws16c48_gpio {
    map: *mut regmap,
    lock: raw_spinlock_t,
    irq_mask: [u8; WS16C48_NUM_IRQS / WS16C48_NGPIO_PER_REG],
}

static mut ws16c48_regmap_irqs: [regmap_irq; WS16C48_NUM_IRQS] = [
    regmap_irq { reg_offset: 0, mask: 1, type_: regmap_irq_type { type_reg_offset: 0, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 0, mask: 2, type_: regmap_irq_type { type_reg_offset: 0, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 0, mask: 4, type_: regmap_irq_type { type_reg_offset: 0, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 0, mask: 8, type_: regmap_irq_type { type_reg_offset: 0, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 0, mask: 16, type_: regmap_irq_type { type_reg_offset: 0, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 0, mask: 32, type_: regmap_irq_type { type_reg_offset: 0, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 0, mask: 64, type_: regmap_irq_type { type_reg_offset: 0, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 0, mask: 128, type_: regmap_irq_type { type_reg_offset: 0, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 1, mask: 1, type_: regmap_irq_type { type_reg_offset: 1, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 1, mask: 2, type_: regmap_irq_type { type_reg_offset: 1, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 1, mask: 4, type_: regmap_irq_type { type_reg_offset: 1, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 1, mask: 8, type_: regmap_irq_type { type_reg_offset: 1, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 1, mask: 16, type_: regmap_irq_type { type_reg_offset: 1, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 1, mask: 32, type_: regmap_irq_type { type_reg_offset: 1, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 1, mask: 64, type_: regmap_irq_type { type_reg_offset: 1, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 1, mask: 128, type_: regmap_irq_type { type_reg_offset: 1, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 2, mask: 1, type_: regmap_irq_type { type_reg_offset: 2, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 2, mask: 2, type_: regmap_irq_type { type_reg_offset: 2, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 2, mask: 4, type_: regmap_irq_type { type_reg_offset: 2, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 2, mask: 8, type_: regmap_irq_type { type_reg_offset: 2, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 2, mask: 16, type_: regmap_irq_type { type_reg_offset: 2, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 2, mask: 32, type_: regmap_irq_type { type_reg_offset: 2, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 2, mask: 64, type_: regmap_irq_type { type_reg_offset: 2, types_supported: IRQ_TYPE_EDGE_BOTH } },
    regmap_irq { reg_offset: 2, mask: 128, type_: regmap_irq_type { type_reg_offset: 2, types_supported: IRQ_TYPE_EDGE_BOTH } },
];

unsafe fn ws16c48_handle_pre_irq(irq_drv_data: *mut core::ffi::c_void) -> i32 {
    let gpio = irq_drv_data as *mut ws16c48_gpio;
    raw_spin_lock(&mut (*gpio).lock);
    0
}

unsafe fn ws16c48_handle_post_irq(irq_drv_data: *mut core::ffi::c_void) -> i32 {
    let gpio = irq_drv_data as *mut ws16c48_gpio;
    raw_spin_unlock(&mut (*gpio).lock);
    0
}

unsafe fn ws16c48_handle_mask_sync(index: i32, _mask_buf_def: u32, mask_buf: u32, irq_drv_data: *mut core::ffi::c_void) -> i32 {
    let gpio = irq_drv_data as *mut ws16c48_gpio;
    let mut flags: usize = 0;
    raw_spin_lock_irqsave(&mut (*gpio).lock, &mut flags);
    if mask_buf as u8 == (*gpio).irq_mask[index as usize] { raw_spin_unlock_irqrestore(&mut (*gpio).lock, flags); return 0; }
    (*gpio).irq_mask[index as usize] = mask_buf as u8;
    let mut ret = regmap_write((*gpio).map, WS16C48_PAGE_LOCK, ENAB_PAGE);
    if ret == 0 { ret = regmap_write((*gpio).map, WS16C48_ENAB + index as u8, !mask_buf as u8); }
    if ret == 0 { ret = regmap_write((*gpio).map, WS16C48_PAGE_LOCK, INT_ID_PAGE); }
    raw_spin_unlock_irqrestore(&mut (*gpio).lock, flags);
    ret
}

unsafe fn ws16c48_set_type_config(_buf: *mut *mut u32, type_: u32, irq_data: *const regmap_irq, idx: i32, irq_drv_data: *mut core::ffi::c_void) -> i32 {
    let gpio = irq_drv_data as *mut ws16c48_gpio;
    let polarity = match type_ { IRQ_TYPE_EDGE_RISING => (*irq_data).mask, IRQ_TYPE_EDGE_FALLING => 0, _ => return -EINVAL };
    let mut flags: usize = 0;
    raw_spin_lock_irqsave(&mut (*gpio).lock, &mut flags);
    let mut ret = regmap_write((*gpio).map, WS16C48_PAGE_LOCK, POL_PAGE);
    if ret == 0 { ret = regmap_update_bits((*gpio).map, WS16C48_POL + idx as u8, (*irq_data).mask, polarity); }
    if ret == 0 { ret = regmap_write((*gpio).map, WS16C48_PAGE_LOCK, INT_ID_PAGE); }
    raw_spin_unlock_irqrestore(&mut (*gpio).lock, flags);
    ret
}

static ws16c48_names: [&str; WS16C48_NGPIO] = [
    "Port 0 Bit 0", "Port 0 Bit 1", "Port 0 Bit 2", "Port 0 Bit 3", "Port 0 Bit 4", "Port 0 Bit 5", "Port 0 Bit 6", "Port 0 Bit 7",
    "Port 1 Bit 0", "Port 1 Bit 1", "Port 1 Bit 2", "Port 1 Bit 3", "Port 1 Bit 4", "Port 1 Bit 5", "Port 1 Bit 6", "Port 1 Bit 7",
    "Port 2 Bit 0", "Port 2 Bit 1", "Port 2 Bit 2", "Port 2 Bit 3", "Port 2 Bit 4", "Port 2 Bit 5", "Port 2 Bit 6", "Port 2 Bit 7",
    "Port 3 Bit 0", "Port 3 Bit 1", "Port 3 Bit 2", "Port 3 Bit 3", "Port 3 Bit 4", "Port 3 Bit 5", "Port 3 Bit 6", "Port 3 Bit 7",
    "Port 4 Bit 0", "Port 4 Bit 1", "Port 4 Bit 2", "Port 4 Bit 3", "Port 4 Bit 4", "Port 4 Bit 5", "Port 4 Bit 6", "Port 4 Bit 7",
    "Port 5 Bit 0", "Port 5 Bit 1", "Port 5 Bit 2", "Port 5 Bit 3", "Port 5 Bit 4", "Port 5 Bit 5", "Port 5 Bit 6", "Port 5 Bit 7",
];

unsafe fn ws16c48_irq_init_hw(map: *mut regmap) -> i32 {
    let mut err = regmap_write(map, WS16C48_PAGE_LOCK, ENAB_PAGE); if err != 0 { return err; }
    for index in 0..3 { err = regmap_write(map, WS16C48_ENAB + index, 0); if err != 0 { return err; } }
    regmap_write(map, WS16C48_PAGE_LOCK, INT_ID_PAGE)
}

unsafe fn ws16c48_probe(dev: *mut device, id: u32) -> i32 {
    let gpio = devm_kzalloc(dev, core::mem::size_of::<ws16c48_gpio>(), GFP_KERNEL) as *mut ws16c48_gpio;
    if gpio.is_null() { return -ENOMEM; }
    let name = dev_name(dev);
    if devm_request_region(dev, base[id as usize], WS16C48_EXTENT as u32, name).is_null() { dev_err(dev, "Unable to lock port addresses (0x%X-0x%X)\n", base[id as usize], base[id as usize] + WS16C48_EXTENT as u32); return -EBUSY; }
    let regs = devm_ioport_map(dev, base[id as usize], WS16C48_EXTENT as u32); if regs.is_null() { return -ENOMEM; }
    (*gpio).map = devm_regmap_init_mmio(dev, regs, &ws16c48_regmap_config);
    if is_err((*gpio).map) { return dev_err_probe(dev, ptr_err((*gpio).map), "Unable to initialize register map\n"); }
    let chip = devm_kzalloc(dev, core::mem::size_of::<regmap_irq_chip>(), GFP_KERNEL) as *mut regmap_irq_chip; if chip.is_null() { return -ENOMEM; }
    (*chip).name = name; (*chip).status_base = WS16C48_INT_ID; (*chip).mask_base = WS16C48_ENAB; (*chip).ack_base = WS16C48_INT_ID; (*chip).num_regs = 3;
    (*chip).irqs = ws16c48_regmap_irqs.as_ptr(); (*chip).num_irqs = WS16C48_NUM_IRQS; (*chip).handle_pre_irq = Some(ws16c48_handle_pre_irq); (*chip).handle_post_irq = Some(ws16c48_handle_post_irq); (*chip).handle_mask_sync = Some(ws16c48_handle_mask_sync); (*chip).set_type_config = Some(ws16c48_set_type_config); (*chip).irq_drv_data = gpio;
    raw_spin_lock_init(&mut (*gpio).lock);
    let err = ws16c48_irq_init_hw((*gpio).map); if err != 0 { return err; }
    let mut chip_data: *mut regmap_irq_chip_data = core::ptr::null_mut();
    let err = devm_regmap_add_irq_chip(dev, (*gpio).map, irq[id as usize], 0, 0, chip, &mut chip_data); if err != 0 { return dev_err_probe(dev, err, "IRQ registration failed\n"); }
    let mut gpio_config: gpio_regmap_config = core::mem::zeroed(); gpio_config.parent = dev; gpio_config.regmap = (*gpio).map; gpio_config.ngpio = WS16C48_NGPIO as u32; gpio_config.names = ws16c48_names.as_ptr(); gpio_config.reg_dat_base = GPIO_REGMAP_ADDR(WS16C48_DAT_BASE); gpio_config.reg_set_base = GPIO_REGMAP_ADDR(WS16C48_DAT_BASE); gpio_config.reg_dir_out_base = GPIO_REGMAP_ADDR(WS16C48_DAT_BASE); gpio_config.ngpio_per_reg = WS16C48_NGPIO_PER_REG as u32; gpio_config.irq_domain = regmap_irq_get_domain(chip_data);
    ptr_err_or_zero(devm_gpio_regmap_register(dev, &mut gpio_config))
}

static mut ws16c48_driver: isa_driver = isa_driver { probe: Some(ws16c48_probe), driver: driver { name: "ws16c48" } };

// module_isa_driver_with_irq(ws16c48_driver, num_ws16c48, num_irq);
// MODULE_AUTHOR("William Breathitt Gray <vilhelm.gray@gmail.com>");
// MODULE_DESCRIPTION("WinSystems WS16C48 GPIO driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
