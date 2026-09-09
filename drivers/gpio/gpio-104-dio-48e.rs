// SPDX-License-Identifier: GPL-2.0-only
/*
 * GPIO driver for the ACCES 104-DIO-48E series
 * Copyright (C) 2016 William Breathitt Gray
 *
 * This driver supports the following ACCES devices: 104-DIO-48E and
 * 104-DIO-24E.
 */

// C dependencies supplied by the Linux kernel and gpio-i8255.h.

const DIO48E_EXTENT: u32 = 16;
const MAX_NUM_DIO48E: usize = max_num_isa_dev(DIO48E_EXTENT);

static mut base: [u32; MAX_NUM_DIO48E] = [0; MAX_NUM_DIO48E];
static mut num_dio48e: u32 = 0;
static mut irq: [u32; MAX_NUM_DIO48E] = [0; MAX_NUM_DIO48E];
static mut num_irq: u32 = 0;

const DIO48E_ENABLE_INTERRUPT: u32 = 0xB;
const DIO48E_DISABLE_INTERRUPT: u32 = DIO48E_ENABLE_INTERRUPT;
const DIO48E_ENABLE_COUNTER_TIMER_ADDRESSING: u32 = 0xD;
const DIO48E_DISABLE_COUNTER_TIMER_ADDRESSING: u32 = DIO48E_ENABLE_COUNTER_TIMER_ADDRESSING;
const DIO48E_CLEAR_INTERRUPT: u32 = 0xF;
const DIO48E_NUM_PPI: usize = 2;

static dio48e_wr_ranges: [regmap_range; 4] = [
    regmap_reg_range(0x0, 0x9), regmap_reg_range(0xB, 0xB),
    regmap_reg_range(0xD, 0xD), regmap_reg_range(0xF, 0xF),
];
static dio48e_rd_ranges: [regmap_range; 5] = [
    regmap_reg_range(0x0, 0x2), regmap_reg_range(0x4, 0x6),
    regmap_reg_range(0xB, 0xB), regmap_reg_range(0xD, 0xD),
    regmap_reg_range(0xF, 0xF),
];
static dio48e_volatile_ranges: [regmap_range; 5] = [
    i8255_volatile_regmap_range(0x0), i8255_volatile_regmap_range(0x4),
    regmap_reg_range(0xB, 0xB), regmap_reg_range(0xD, 0xD),
    regmap_reg_range(0xF, 0xF),
];
static dio48e_precious_ranges: [regmap_range; 3] = [
    regmap_reg_range(0xB, 0xB), regmap_reg_range(0xD, 0xD),
    regmap_reg_range(0xF, 0xF),
];

static dio48e_wr_table: regmap_access_table = regmap_access_table {
    yes_ranges: dio48e_wr_ranges.as_ptr(), n_yes_ranges: dio48e_wr_ranges.len(),
};
static dio48e_rd_table: regmap_access_table = regmap_access_table {
    yes_ranges: dio48e_rd_ranges.as_ptr(), n_yes_ranges: dio48e_rd_ranges.len(),
};
static dio48e_volatile_table: regmap_access_table = regmap_access_table {
    yes_ranges: dio48e_volatile_ranges.as_ptr(), n_yes_ranges: dio48e_volatile_ranges.len(),
};
static dio48e_precious_table: regmap_access_table = regmap_access_table {
    yes_ranges: dio48e_precious_ranges.as_ptr(), n_yes_ranges: dio48e_precious_ranges.len(),
};

static pit_wr_ranges: [regmap_range; 1] = [regmap_reg_range(0x0, 0x3)];
static pit_rd_ranges: [regmap_range; 1] = [regmap_reg_range(0x0, 0x2)];
static pit_wr_table: regmap_access_table = regmap_access_table {
    yes_ranges: pit_wr_ranges.as_ptr(), n_yes_ranges: pit_wr_ranges.len(),
};
static pit_rd_table: regmap_access_table = regmap_access_table {
    yes_ranges: pit_rd_ranges.as_ptr(), n_yes_ranges: pit_rd_ranges.len(),
};

/* only bit 3 on each respective Port C supports interrupts */
const fn dio48e_regmap_irq(ppi: usize) -> regmap_irq {
    regmap_irq { index: 19 + ppi * 24, mask: 1 << ppi, types_supported: IRQ_TYPE_EDGE_RISING }
}
static dio48e_regmap_irqs: [regmap_irq; 2] = [dio48e_regmap_irq(0), dio48e_regmap_irq(1)];

#[repr(C)]
struct dio48e_gpio {
    lock: raw_spinlock_t,
    map: *mut regmap,
    regs: *mut core::ffi::c_void,
    flags: c_ulong,
    irq_mask: c_uint,
}

unsafe extern "C" fn dio48e_regmap_lock(lock_arg: *mut core::ffi::c_void) {
    let dio48egpio = lock_arg as *mut dio48e_gpio;
    let mut flags: c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*dio48egpio).lock, &mut flags);
    (*dio48egpio).flags = flags;
}

unsafe extern "C" fn dio48e_regmap_unlock(lock_arg: *mut core::ffi::c_void) {
    let dio48egpio = lock_arg as *mut dio48e_gpio;
    raw_spin_unlock_irqrestore(&mut (*dio48egpio).lock, (*dio48egpio).flags);
}

unsafe extern "C" fn pit_regmap_lock(lock_arg: *mut core::ffi::c_void) {
    let dio48egpio = lock_arg as *mut dio48e_gpio;
    let mut flags: c_ulong = 0;
    raw_spin_lock_irqsave(&mut (*dio48egpio).lock, &mut flags);
    (*dio48egpio).flags = flags;
    iowrite8(0x00, (*dio48egpio).regs.byte_add(DIO48E_ENABLE_COUNTER_TIMER_ADDRESSING as usize));
}

unsafe extern "C" fn pit_regmap_unlock(lock_arg: *mut core::ffi::c_void) {
    let dio48egpio = lock_arg as *mut dio48e_gpio;
    ioread8((*dio48egpio).regs.byte_add(DIO48E_DISABLE_COUNTER_TIMER_ADDRESSING as usize));
    raw_spin_unlock_irqrestore(&mut (*dio48egpio).lock, (*dio48egpio).flags);
}

unsafe extern "C" fn dio48e_handle_mask_sync(index: c_int, mask_buf_def: c_uint,
    mask_buf: c_uint, irq_drv_data: *mut core::ffi::c_void) -> c_int {
    let dio48egpio = irq_drv_data as *mut dio48e_gpio;
    let prev_mask = (*dio48egpio).irq_mask;
    let mut val: c_uint = 0;
    if mask_buf == prev_mask { return 0; }
    (*dio48egpio).irq_mask = mask_buf;
    if prev_mask == mask_buf_def {
        let err = regmap_write((*dio48egpio).map, DIO48E_CLEAR_INTERRUPT, 0x00);
        if err != 0 { return err; }
        return regmap_write((*dio48egpio).map, DIO48E_ENABLE_INTERRUPT, 0x00);
    }
    if mask_buf == mask_buf_def { return regmap_read((*dio48egpio).map, DIO48E_DISABLE_INTERRUPT, &mut val); }
    0
}

const DIO48E_NGPIO: usize = 48;
static dio48e_names: [&str; DIO48E_NGPIO] = [
    "PPI Group 0 Port A 0", "PPI Group 0 Port A 1", "PPI Group 0 Port A 2", "PPI Group 0 Port A 3", "PPI Group 0 Port A 4", "PPI Group 0 Port A 5", "PPI Group 0 Port A 6", "PPI Group 0 Port A 7",
    "PPI Group 0 Port B 0", "PPI Group 0 Port B 1", "PPI Group 0 Port B 2", "PPI Group 0 Port B 3", "PPI Group 0 Port B 4", "PPI Group 0 Port B 5", "PPI Group 0 Port B 6", "PPI Group 0 Port B 7",
    "PPI Group 0 Port C 0", "PPI Group 0 Port C 1", "PPI Group 0 Port C 2", "PPI Group 0 Port C 3", "PPI Group 0 Port C 4", "PPI Group 0 Port C 5", "PPI Group 0 Port C 6", "PPI Group 0 Port C 7",
    "PPI Group 1 Port A 0", "PPI Group 1 Port A 1", "PPI Group 1 Port A 2", "PPI Group 1 Port A 3", "PPI Group 1 Port A 4", "PPI Group 1 Port A 5", "PPI Group 1 Port A 6", "PPI Group 1 Port A 7",
    "PPI Group 1 Port B 0", "PPI Group 1 Port B 1", "PPI Group 1 Port B 2", "PPI Group 1 Port B 3", "PPI Group 1 Port B 4", "PPI Group 1 Port B 5", "PPI Group 1 Port B 6", "PPI Group 1 Port B 7",
    "PPI Group 1 Port C 0", "PPI Group 1 Port C 1", "PPI Group 1 Port C 2", "PPI Group 1 Port C 3", "PPI Group 1 Port C 4", "PPI Group 1 Port C 5", "PPI Group 1 Port C 6", "PPI Group 1 Port C 7",
];

unsafe extern "C" fn dio48e_irq_init_hw(map: *mut regmap) -> c_int {
    let mut val: c_uint = 0;
    regmap_read(map, DIO48E_DISABLE_INTERRUPT, &mut val)
}

// The remaining probe/registration declarations mirror the C driver's external kernel API.
unsafe extern "C" fn dio48e_probe(dev: *mut device, id: c_uint) -> c_int {
    let name = dev_name(dev);
    let mut config: i8255_regmap_config = core::mem::zeroed();
    let mut dio48egpio: *mut dio48e_gpio;
    if devm_request_region(dev, base[id as usize], DIO48E_EXTENT, name).is_null() {
        dev_err(dev, "Unable to lock port addresses (0x%X-0x%X)\n", base[id as usize], base[id as usize] + DIO48E_EXTENT);
        return -EBUSY;
    }
    dio48egpio = devm_kzalloc(dev, core::mem::size_of::<dio48e_gpio>(), GFP_KERNEL) as *mut dio48e_gpio;
    if dio48egpio.is_null() { return -ENOMEM; }
    let regs = devm_ioport_map(dev, base[id as usize], DIO48E_EXTENT);
    if regs.is_null() { return -ENOMEM; }
    (*dio48egpio).regs = regs;
    raw_spin_lock_init(&mut (*dio48egpio).lock);
    let mut dio48e_regmap_config: regmap_config = core::mem::zeroed();
    let mut pit_regmap_config: regmap_config = core::mem::zeroed();
    let mut pit_config: i8254_regmap_config = core::mem::zeroed();
    let mut chip: *mut regmap_irq_chip;
    let mut chip_data: *mut regmap_irq_chip_data = core::ptr::null_mut();
    dio48e_regmap_config = regmap_config { reg_bits: 8, reg_stride: 1, val_bits: 8,
        lock: Some(dio48e_regmap_lock), unlock: Some(dio48e_regmap_unlock),
        lock_arg: dio48egpio as *mut _, io_port: true, wr_table: &dio48e_wr_table,
        rd_table: &dio48e_rd_table, volatile_table: &dio48e_volatile_table,
        precious_table: &dio48e_precious_table, cache_type: REGCACHE_FLAT, ..core::mem::zeroed() };
    let map = devm_regmap_init_mmio(dev, regs, &dio48e_regmap_config);
    if is_err(map) { return dev_err_probe(dev, ptr_err(map), "Unable to initialize register map\n"); }
    (*dio48egpio).map = map;
    pit_regmap_config = regmap_config { name: "i8254", reg_bits: 8, reg_stride: 1, val_bits: 8,
        lock: Some(pit_regmap_lock), unlock: Some(pit_regmap_unlock), lock_arg: dio48egpio as *mut _,
        io_port: true, wr_table: &pit_wr_table, rd_table: &pit_rd_table, ..core::mem::zeroed() };
    pit_config.map = devm_regmap_init_mmio(dev, regs, &pit_regmap_config);
    if is_err(pit_config.map) { return dev_err_probe(dev, ptr_err(pit_config.map), "Unable to initialize i8254 register map\n"); }
    chip = devm_kzalloc(dev, core::mem::size_of::<regmap_irq_chip>(), GFP_KERNEL) as *mut regmap_irq_chip;
    if chip.is_null() { return -ENOMEM; }
    (*chip).name = name; (*chip).mask_base = DIO48E_ENABLE_INTERRUPT; (*chip).ack_base = DIO48E_CLEAR_INTERRUPT;
    (*chip).no_status = true; (*chip).num_regs = 1; (*chip).irqs = dio48e_regmap_irqs.as_ptr();
    (*chip).num_irqs = dio48e_regmap_irqs.len(); (*chip).handle_mask_sync = Some(dio48e_handle_mask_sync);
    (*chip).irq_drv_data = dio48egpio as *mut _;
    let err = dio48e_irq_init_hw(map); if err != 0 { return err; }
    let err = devm_regmap_add_irq_chip(dev, map, irq[id as usize], 0, 0, chip, &mut chip_data);
    if err != 0 { return dev_err_probe(dev, err, "IRQ registration failed\n"); }
    pit_config.parent = dev; let err = devm_i8254_regmap_register(dev, &pit_config); if err != 0 { return err; }
    config.parent = dev; config.map = map; config.num_ppi = DIO48E_NUM_PPI;
    config.names = dio48e_names.as_ptr(); config.domain = regmap_irq_get_domain(chip_data);
    devm_i8255_regmap_register(dev, &config)
}

static mut dio48e_driver: isa_driver = isa_driver { probe: Some(dio48e_probe), name: "104-dio-48e" };

// module_isa_driver_with_irq(dio48e_driver, num_dio48e, num_irq);
// MODULE_AUTHOR("William Breathitt Gray <vilhelm.gray@gmail.com>");
// MODULE_DESCRIPTION("ACCES 104-DIO-48E GPIO driver");
// MODULE_LICENSE("GPL v2");
// MODULE_IMPORT_NS("I8254");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
