// SPDX-License-Identifier: GPL-2.0+
/*
 * HD44780 Character LCD driver for Linux
 *
 * Copyright (C) 2000-2008, Willy Tarreau <w@1wt.eu>
 * Copyright (C) 2016-2017 Glider bvba
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(usize)]
enum Hd44780Pin {
    // Order does matter due to writing to GPIO array subsets!
    PinData0, // Optional
    PinData1, // Optional
    PinData2, // Optional
    PinData3, // Optional
    PinData4,
    PinData5,
    PinData6,
    PinData7,
    PinCtrlRs,
    PinCtrlRw, // Optional
    PinCtrlE,
    PinCtrlBl, // Optional
    PinNum,
}

#[repr(C)]
struct Hd44780 {
    pins: [*mut GpioDesc; Hd44780Pin::PinNum as usize],
}

unsafe fn hd44780_backlight(lcd: *mut Charlcd, on: CharlcdOnoff) {
    let hdc = (*lcd).drvdata as *mut Hd44780Common;
    let hd = (*hdc).hd44780;

    if !(*hd).pins[Hd44780Pin::PinCtrlBl as usize].is_null() {
        gpiod_set_value_cansleep((*hd).pins[Hd44780Pin::PinCtrlBl as usize], on);
    }
}

unsafe fn hd44780_strobe_gpio(hd: *mut Hd44780) {
    // Maintain the data during 20 us before the strobe
    udelay(20);

    gpiod_set_value_cansleep((*hd).pins[Hd44780Pin::PinCtrlE as usize], 1);

    // Maintain the strobe during 40 us
    udelay(40);

    gpiod_set_value_cansleep((*hd).pins[Hd44780Pin::PinCtrlE as usize], 0);
}

/* write to an LCD panel register in 8 bit GPIO mode */
unsafe fn hd44780_write_gpio8(hd: *mut Hd44780, val: u8, rs: u32) {
    let mut values: [usize; 1] = [val as usize];
    // DECLARE_BITMAP(values, 10); __assign_bit(8, values, rs)
    values[0] = (values[0] & !(1usize << 8)) | (((rs != 0) as usize) << 8);
    let n = if !(*hd).pins[Hd44780Pin::PinCtrlRw as usize].is_null() { 10 } else { 9 };

    // Present the data to the port
    gpiod_set_array_value_cansleep(n, (*hd).pins.as_mut_ptr().add(Hd44780Pin::PinData0 as usize), core::ptr::null(), values.as_ptr());

    hd44780_strobe_gpio(hd);
}

/* write to an LCD panel register in 4 bit GPIO mode */
unsafe fn hd44780_write_gpio4(hd: *mut Hd44780, val: u8, rs: u32) {
    let mut values: [usize; 1] = [(val >> 4) as usize];
    // DECLARE_BITMAP(values, 6); __assign_bit(4, values, rs)
    values[0] = (values[0] & !(1usize << 4)) | (((rs != 0) as usize) << 4);
    let n = if !(*hd).pins[Hd44780Pin::PinCtrlRw as usize].is_null() { 6 } else { 5 };

    // Present the data to the port
    gpiod_set_array_value_cansleep(n, (*hd).pins.as_mut_ptr().add(Hd44780Pin::PinData4 as usize), core::ptr::null(), values.as_ptr());

    hd44780_strobe_gpio(hd);

    // Low nibble
    values[0] &= !0x0fusize;
    values[0] |= (val & 0x0f) as usize;

    // Present the data to the port
    gpiod_set_array_value_cansleep(n, (*hd).pins.as_mut_ptr().add(Hd44780Pin::PinData4 as usize), core::ptr::null(), values.as_ptr());

    hd44780_strobe_gpio(hd);
}

/* Send a command to the LCD panel in 8 bit GPIO mode */
unsafe fn hd44780_write_cmd_gpio8(hdc: *mut Hd44780Common, cmd: i32) {
    hd44780_write_gpio8((*hdc).hd44780, cmd as u8, 0);
    // The shortest command takes at least 120 us
    udelay(120);
}

/* Send data to the LCD panel in 8 bit GPIO mode */
unsafe fn hd44780_write_data_gpio8(hdc: *mut Hd44780Common, data: i32) {
    hd44780_write_gpio8((*hdc).hd44780, data as u8, 1);
    // The shortest data takes at least 45 us
    udelay(45);
}

static HD44780_OPS_GPIO8: CharlcdOps = Char lcdOps {
    backlight: Some(hd44780_backlight),
    print: Some(hd44780_common_print), gotoxy: Some(hd44780_common_gotoxy),
    home: Some(hd44780_common_home), clear_display: Some(hd44780_common_clear_display),
    init_display: Some(hd44780_common_init_display), shift_cursor: Some(hd44780_common_shift_cursor),
    shift_display: Some(hd44780_common_shift_display), display: Some(hd44780_common_display),
    cursor: Some(hd44780_common_cursor), blink: Some(hd44780_common_blink),
    fontsize: Some(hd44780_common_fontsize), lines: Some(hd44780_common_lines),
    redefine_char: Some(hd44780_common_redefine_char),
};

/* Send a command to the LCD panel in 4 bit GPIO mode */
unsafe fn hd44780_write_cmd_gpio4(hdc: *mut Hd44780Common, cmd: i32) {
    hd44780_write_gpio4((*hdc).hd44780, cmd as u8, 0);
    // The shortest command takes at least 120 us
    udelay(120);
}

/* Send 4-bits of a command to the LCD panel in raw 4 bit GPIO mode */
unsafe fn hd44780_write_cmd_raw_gpio4(hdc: *mut Hd44780Common, cmd: i32) {
    let mut values: [usize; 1] = [(cmd & 0x0f) as usize];
    let hd = (*hdc).hd44780;
    let n = if !(*hd).pins[Hd44780Pin::PinCtrlRw as usize].is_null() { 6 } else { 5 };
    // Present the data to the port
    gpiod_set_array_value_cansleep(n, (*hd).pins.as_mut_ptr().add(Hd44780Pin::PinData4 as usize), core::ptr::null(), values.as_ptr());
    hd44780_strobe_gpio(hd);
}

/* Send data to the LCD panel in 4 bit GPIO mode */
unsafe fn hd44780_write_data_gpio4(hdc: *mut Hd44780Common, data: i32) {
    hd44780_write_gpio4((*hdc).hd44780, data as u8, 1);
    // The shortest data takes at least 45 us
    udelay(45);
}

static HD44780_OPS_GPIO4: Char lcdOps = Char lcdOps {
    backlight: Some(hd44780_backlight),
    print: Some(hd44780_common_print), gotoxy: Some(hd44780_common_gotoxy),
    home: Some(hd44780_common_home), clear_display: Some(hd44780_common_clear_display),
    init_display: Some(hd44780_common_init_display), shift_cursor: Some(hd44780_common_shift_cursor),
    shift_display: Some(hd44780_common_shift_display), display: Some(hd44780_common_display),
    cursor: Some(hd44780_common_cursor), blink: Some(hd44780_common_blink),
    fontsize: Some(hd44780_common_fontsize), lines: Some(hd44780_common_lines),
    redefine_char: Some(hd44780_common_redefine_char),
};

// The probe, remove, platform-driver registration, and module metadata below
// retain their C-defined external kernel interfaces in the surrounding port.
unsafe fn hd44780_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev;
    let mut i: u32;
    let mut base: usize;
    let mut lcd: *mut Charlcd;
    let mut hdc: *mut Hd44780Common;
    let mut hd: *mut Hd44780;
    let mut ifwidth: i32;
    let mut ret: i32 = -12;

    ifwidth = gpiod_count(dev, "data");
    if ifwidth < 0 { return ifwidth; }
    match ifwidth {
        4 => base = Hd44780Pin::PinData4 as usize,
        8 => base = Hd44780Pin::PinData0 as usize,
        _ => return -22,
    }

    lcd = hd44780_common_alloc();
    if lcd.is_null() { return -12; }
    hd = kzalloc_hd44780();
    if hd.is_null() { hd44780_common_free(lcd); return ret; }
    hdc = (*lcd).drvdata as *mut Hd44780Common;
    (*hdc).hd44780 = hd;

    i = 0;
    while i < ifwidth as u32 {
        (*hd).pins[base + i as usize] = devm_gpiod_get_index(dev, "data", i, GpiodFlags::OutLow);
        if is_err((*hd).pins[base + i as usize]) { ret = ptr_err((*hd).pins[base + i as usize]); kfree(hd); hd44780_common_free(lcd); return ret; }
        i += 1;
    }
    (*hd).pins[Hd44780Pin::PinCtrlE as usize] = devm_gpiod_get(dev, "enable", GpiodFlags::OutLow);
    if is_err((*hd).pins[Hd44780Pin::PinCtrlE as usize]) { ret = ptr_err((*hd).pins[Hd44780Pin::PinCtrlE as usize]); kfree(hd); hd44780_common_free(lcd); return ret; }
    (*hd).pins[Hd44780Pin::PinCtrlRs as usize] = devm_gpiod_get(dev, "rs", GpiodFlags::OutHigh);
    if is_err((*hd).pins[Hd44780Pin::PinCtrlRs as usize]) { ret = ptr_err((*hd).pins[Hd44780Pin::PinCtrlRs as usize]); kfree(hd); hd44780_common_free(lcd); return ret; }
    (*hd).pins[Hd44780Pin::PinCtrlRw as usize] = devm_gpiod_get_optional(dev, "rw", GpiodFlags::OutLow);
    if is_err((*hd).pins[Hd44780Pin::PinCtrlRw as usize]) { ret = ptr_err((*hd).pins[Hd44780Pin::PinCtrlRw as usize]); kfree(hd); hd44780_common_free(lcd); return ret; }
    (*hd).pins[Hd44780Pin::PinCtrlBl as usize] = devm_gpiod_get_optional(dev, "backlight", GpiodFlags::OutLow);
    if is_err((*hd).pins[Hd44780Pin::PinCtrlBl as usize]) { ret = ptr_err((*hd).pins[Hd44780Pin::PinCtrlBl as usize]); kfree(hd); hd44780_common_free(lcd); return ret; }

    ret = device_property_read_u32(dev, "display-height-chars", &mut (*lcd).height);
    if ret != 0 { kfree(hd); hd44780_common_free(lcd); return ret; }
    ret = device_property_read_u32(dev, "display-width-chars", &mut (*lcd).width);
    if ret != 0 { kfree(hd); hd44780_common_free(lcd); return ret; }
    // On displays with more than two rows, the internal buffer width is
    // usually equal to the display width
    if (*lcd).height > 2 { (*hdc).bwidth = (*lcd).width; }
    device_property_read_u32(dev, "internal-buffer-width", &mut (*hdc).bwidth);
    (*hdc).ifwidth = ifwidth;
    if ifwidth == 8 {
        (*lcd).ops = &HD44780_OPS_GPIO8;
        (*hdc).write_data = Some(hd44780_write_data_gpio8);
        (*hdc).write_cmd = Some(hd44780_write_cmd_gpio8);
    } else {
        (*lcd).ops = &HD44780_OPS_GPIO4;
        (*hdc).write_data = Some(hd44780_write_data_gpio4);
        (*hdc).write_cmd = Some(hd44780_write_cmd_gpio4);
        (*hdc).write_cmd_raw4 = Some(hd44780_write_cmd_raw_gpio4);
    }
    ret = charlcd_register(lcd);
    if ret != 0 { kfree(hd); hd44780_common_free(lcd); return ret; }
    platform_set_drvdata(pdev, lcd);
    0
}

unsafe fn hd44780_remove(pdev: *mut PlatformDevice) {
    let lcd = platform_get_drvdata(pdev);
    let hdc = (*lcd).drvdata as *mut Hd44780Common;
    charlcd_unregister(lcd);
    kfree((*hdc).hd44780);
    hd44780_common_free(lcd);
}

// Device matching, platform-driver registration, and module metadata are
// supplied by the surrounding kernel translation.
static HD44780_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(hd44780_probe),
    remove: Some(hd44780_remove),
    name: "hd44780",
    compatible: "hit,hd44780",
};

module_platform_driver!(HD44780_DRIVER);
module_description!("HD44780 Character LCD driver");
module_author!("Geert Uytterhoeven <geert@linux-m68k.org>");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
