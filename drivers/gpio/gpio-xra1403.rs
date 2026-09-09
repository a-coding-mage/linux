// SPDX-License-Identifier: GPL-2.0-only
/*
 * GPIO driver for EXAR XRA1403 16-bit GPIO expander
 *
 * Copyright (c) 2017, General Electric Company
 */

// Linux dependencies: bitops, gpio/driver, kernel, module, mutex, seq_file,
// spi, string_choices, and regmap.

/* XRA1403 registers */
const XRA_GSR: u32 = 0x00; // GPIO State
const XRA_OCR: u32 = 0x02; // Output Control
const XRA_PIR: u32 = 0x04; // Input Polarity Inversion
const XRA_GCR: u32 = 0x06; // GPIO Configuration
const XRA_PUR: u32 = 0x08; // Input Internal Pull-up Resistor Enable/Disable
const XRA_IER: u32 = 0x0A; // Input Interrupt Enable
const XRA_TSCR: u32 = 0x0C; // Output Three-State Control
const XRA_ISR: u32 = 0x0E; // Input Interrupt Status
const XRA_REIR: u32 = 0x10; // Input Rising Edge Interrupt Enable
const XRA_FEIR: u32 = 0x12; // Input Falling Edge Interrupt Enable
const XRA_IFR: u32 = 0x14; // Input Filter Enable/Disable
const XRA_LAST: u32 = 0x15; // Bounds

#[repr(C)]
struct Xra1403 {
    chip: GpioChip,
    regmap: *mut Regmap,
}

static XRA1403_REGMAP_CFG: RegmapConfig = RegmapConfig {
    reg_bits: 7,
    pad_bits: 1,
    val_bits: 8,
    max_register: XRA_LAST,
};

fn to_reg(reg: u32, offset: u32) -> u32 {
    reg + u32::from(offset > 7)
}

unsafe fn xra1403_direction_input(chip: *mut GpioChip, offset: u32) -> i32 {
    let xra = gpiochip_get_data(chip);
    regmap_update_bits((*xra).regmap, to_reg(XRA_GCR, offset),
        1u32 << (offset % 8), 1u32 << (offset % 8))
}

unsafe fn xra1403_direction_output(chip: *mut GpioChip, offset: u32, value: i32) -> i32 {
    let xra = gpiochip_get_data(chip);
    let mut ret = regmap_update_bits((*xra).regmap, to_reg(XRA_GCR, offset),
        1u32 << (offset % 8), 0);
    if ret != 0 { return ret; }
    ret = regmap_update_bits((*xra).regmap, to_reg(XRA_OCR, offset),
        1u32 << (offset % 8), if value != 0 { 1u32 << (offset % 8) } else { 0 });
    ret
}

unsafe fn xra1403_get_direction(chip: *mut GpioChip, offset: u32) -> i32 {
    let xra = gpiochip_get_data(chip);
    let mut val: u32 = 0;
    let ret = regmap_read((*xra).regmap, to_reg(XRA_GCR, offset), &mut val);
    if ret != 0 { return ret; }
    if val & (1u32 << (offset % 8)) != 0 { GPIO_LINE_DIRECTION_IN } else { GPIO_LINE_DIRECTION_OUT }
}

unsafe fn xra1403_get(chip: *mut GpioChip, offset: u32) -> i32 {
    let xra = gpiochip_get_data(chip);
    let mut val: u32 = 0;
    let ret = regmap_read((*xra).regmap, to_reg(XRA_GSR, offset), &mut val);
    if ret != 0 { return ret; }
    i32::from((val & (1u32 << (offset % 8))) != 0)
}

unsafe fn xra1403_set(chip: *mut GpioChip, offset: u32, value: i32) -> i32 {
    let xra = gpiochip_get_data(chip);
    regmap_update_bits((*xra).regmap, to_reg(XRA_OCR, offset),
        1u32 << (offset % 8), if value != 0 { 1u32 << (offset % 8) } else { 0 })
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe fn xra1403_dbg_show(s: *mut SeqFile, chip: *mut GpioChip) {
    let xra = gpiochip_get_data(chip);
    let mut value = [0i32; XRA_LAST as usize];
    let mut reg = 0u32;
    seq_puts(s, "xra reg:");
    while reg <= XRA_LAST { seq_printf(s, " %2.2x", reg); reg += 1; }
    seq_puts(s, "\n  value:");
    reg = 0;
    while reg < XRA_LAST { regmap_read((*xra).regmap, reg, &mut value[reg as usize] as *mut i32 as *mut u32); seq_printf(s, " %2.2x", value[reg as usize]); reg += 1; }
    seq_puts(s, "\n");
    let gcr = (value[(XRA_GCR + 1) as usize] << 8) | value[XRA_GCR as usize];
    let gsr = (value[(XRA_GSR + 1) as usize] << 8) | value[XRA_GSR as usize];
    for_each_requested_gpio!(chip, i, label, {
        seq_printf(s, " gpio-%-3d (%-12s) %s %s\n", i, label,
            if gcr & (1 << i) != 0 { "in" } else { "out" }, str_hi_lo(gsr & (1 << i)));
    });
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
const XRA1403_DBG_SHOW: Option<unsafe fn(*mut SeqFile, *mut GpioChip)> = None;

unsafe fn xra1403_probe(spi: *mut SpiDevice) -> i32 {
    let xra = devm_kzalloc(&mut (*spi).dev, core::mem::size_of::<Xra1403>(), GFP_KERNEL) as *mut Xra1403;
    if xra.is_null() { return -ENOMEM; }
    let reset_gpio = devm_gpiod_get_optional(&mut (*spi).dev, "reset", GPIOD_OUT_LOW);
    if is_err(reset_gpio) { dev_warn(&(*spi).dev, "Could not get reset-gpios\n"); }

    (*xra).chip.direction_input = Some(xra1403_direction_input);
    (*xra).chip.direction_output = Some(xra1403_direction_output);
    (*xra).chip.get_direction = Some(xra1403_get_direction);
    (*xra).chip.get = Some(xra1403_get);
    (*xra).chip.set = Some(xra1403_set);
    #[cfg(feature = "CONFIG_DEBUG_FS")]
    { (*xra).chip.dbg_show = Some(xra1403_dbg_show); }
    (*xra).chip.ngpio = 16;
    (*xra).chip.label = "xra1403";
    (*xra).chip.base = -1;
    (*xra).chip.can_sleep = true;
    (*xra).chip.parent = &mut (*spi).dev;
    (*xra).chip.owner = THIS_MODULE;

    (*xra).regmap = devm_regmap_init_spi(spi, &XRA1403_REGMAP_CFG);
    if is_err((*xra).regmap) {
        let ret = ptr_err((*xra).regmap);
        dev_err(&(*spi).dev, "Failed to allocate regmap: %d\n", ret);
        return ret;
    }
    devm_gpiochip_add_data(&mut (*spi).dev, &mut (*xra).chip, xra as *mut core::ffi::c_void)
}

static XRA1403_IDS: [SpiDeviceId; 2] = [
    SpiDeviceId { name: "xra1403" },
    SpiDeviceId { name: "" },
];

static XRA1403_SPI_OF_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { compatible: "exar,xra1403" },
    OfDeviceId { compatible: "" },
];

static mut XRA1403_DRIVER: SpiDriver = SpiDriver {
    probe: Some(xra1403_probe),
    id_table: &XRA1403_IDS,
    driver: Driver { name: "xra1403", of_match_table: &XRA1403_SPI_OF_MATCH },
};

// MODULE_DEVICE_TABLE(spi, xra1403_ids);
// MODULE_DEVICE_TABLE(of, xra1403_spi_of_match);
// module_spi_driver(xra1403_driver);
// MODULE_AUTHOR("Nandor Han <nandor.han@ge.com>");
// MODULE_AUTHOR("Semi Malinen <semi.malinen@ge.com>");
// MODULE_DESCRIPTION("GPIO expander driver for EXAR XRA1403");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
