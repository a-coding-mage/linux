// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2006 Juergen Beisert, Pengutronix
 * Copyright (C) 2008 Guennadi Liakhovetski, Pengutronix
 * Copyright (C) 2009 Wolfram Sang, Pengutronix
 *
 * The Maxim MAX7300/1 device is an I2C/SPI driven GPIO expander. There are
 * 28 GPIOs. 8 of them can trigger an interrupt. See datasheet for more
 * details
 * Note:
 * - DIN must be stable at the rising edge of clock.
 * - when writing:
 *   - always clock in 16 clocks at once
 *   - at DIN: D15 first, D0 last
 *   - D0..D7 = databyte, D8..D14 = commandbyte
 *   - D15 = low -> write command
 * - when reading
 *   - always clock in 16 clocks at once
 *   - at DIN: D15 first, D0 last
 *   - D0..D7 = dummy, D8..D14 = register address
 *   - D15 = high -> read command
 *   - raise CS and assert it again
 *   - always clock in 16 clocks at once
 *   - at DOUT: D15 first, D0 last
 *   - D0..D7 contains the data from the first cycle
 *
 * The driver exports a standard gpiochip interface
 */

// C includes supply the external kernel types, functions, constants, and macros used below.

const PIN_CONFIG_MASK: u8 = 0x03;
const PIN_CONFIG_IN_PULLUP: u8 = 0x03;
const PIN_CONFIG_IN_WO_PULLUP: u8 = 0x02;
const PIN_CONFIG_OUT: u8 = 0x01;
const PIN_NUMBER: u32 = 28;

unsafe fn max7301_direction_input(chip: *mut gpio_chip, mut offset: u32) -> i32 {
    let ts: *mut max7301 = container_of!(chip, max7301, chip);
    let config: *mut u8;
    let offset_bits: u8;
    let pin_config: u8;
    let ret: i32;

    /* First 4 pins are unused in the controller */
    offset += 4;
    offset_bits = ((offset & 3) << 1) as u8;
    config = unsafe { (*ts).port_config.as_mut_ptr().add((offset >> 2) as usize) };

    if unsafe { (*ts).input_pullup_active & BIT(offset) } != 0 {
        pin_config = PIN_CONFIG_IN_PULLUP;
    } else {
        pin_config = PIN_CONFIG_IN_WO_PULLUP;
    }

    mutex_lock(unsafe { &mut (*ts).lock });
    unsafe { *config = (*config & !(PIN_CONFIG_MASK << offset_bits)) | (pin_config << offset_bits); }
    ret = unsafe { ((*ts).write)(unsafe { (*ts).dev }, 0x08 + (offset >> 2), *config) };
    mutex_unlock(unsafe { &mut (*ts).lock });
    ret
}

unsafe fn __max7301_set(ts: *mut max7301, offset: u32, value: i32) -> i32 {
    if value != 0 {
        unsafe { (*ts).out_level |= 1 << offset; }
        unsafe { ((*ts).write)((*ts).dev, 0x20 + offset, 0x01) }
    } else {
        unsafe { (*ts).out_level &= !(1 << offset); }
        unsafe { ((*ts).write)((*ts).dev, 0x20 + offset, 0x00) }
    }
}

unsafe fn max7301_direction_output(chip: *mut gpio_chip, mut offset: u32, value: i32) -> i32 {
    let ts: *mut max7301 = container_of!(chip, max7301, chip);
    let config: *mut u8;
    offset += 4;
    let offset_bits = ((offset & 3) << 1) as u8;
    config = unsafe { (*ts).port_config.as_mut_ptr().add((offset >> 2) as usize) };
    mutex_lock(unsafe { &mut (*ts).lock });
    unsafe { *config = (*config & !(PIN_CONFIG_MASK << offset_bits)) | (PIN_CONFIG_OUT << offset_bits); }
    let mut ret = __max7301_set(ts, offset, value);
    if ret == 0 {
        ret = unsafe { ((*ts).write)((*ts).dev, 0x08 + (offset >> 2), *config) };
    }
    mutex_unlock(unsafe { &mut (*ts).lock });
    ret
}

unsafe fn max7301_get(chip: *mut gpio_chip, mut offset: u32) -> i32 {
    let ts: *mut max7301 = gpiochip_get_data(chip);
    let mut level: i32 = -EINVAL;
    offset += 4;
    mutex_lock(unsafe { &mut (*ts).lock });
    let config = (unsafe { (*ts).port_config[(offset >> 2) as usize] } >> ((offset & 3) << 1)) & PIN_CONFIG_MASK;
    match config {
        PIN_CONFIG_OUT => level = if unsafe { (*ts).out_level & (1 << offset) } != 0 { 1 } else { 0 },
        PIN_CONFIG_IN_WO_PULLUP | PIN_CONFIG_IN_PULLUP => level = unsafe { ((*ts).read)((*ts).dev, 0x20 + offset) & 0x01 },
        _ => {}
    }
    mutex_unlock(unsafe { &mut (*ts).lock });
    level
}

unsafe fn max7301_set(chip: *mut gpio_chip, mut offset: u32, value: i32) {
    let ts: *mut max7301 = gpiochip_get_data(chip);
    offset += 4;
    mutex_lock(unsafe { &mut (*ts).lock });
    let _ret = __max7301_set(ts, offset, value);
    mutex_unlock(unsafe { &mut (*ts).lock });
}

pub unsafe fn __max730x_probe(ts: *mut max7301) -> i32 {
    let dev = (*ts).dev;
    let pdata = dev_get_platdata(dev);
    let ret = devm_mutex_init((*ts).dev, &mut (*ts).lock);
    if ret != 0 { return ret; }
    dev_set_drvdata(dev, ts);
    ((*ts).write)(dev, 0x04, 0x01);
    if !pdata.is_null() {
        (*ts).input_pullup_active = (*pdata).input_pullup_active;
        (*ts).chip.base = (*pdata).base;
    } else { (*ts).chip.base = -1; }
    (*ts).chip.label = (*dev).driver.name;
    (*ts).chip.direction_input = Some(max7301_direction_input);
    (*ts).chip.get = Some(max7301_get);
    (*ts).chip.direction_output = Some(max7301_direction_output);
    (*ts).chip.set = Some(max7301_set);
    (*ts).chip.ngpio = PIN_NUMBER;
    (*ts).chip.can_sleep = true;
    (*ts).chip.parent = dev;
    (*ts).chip.owner = THIS_MODULE;
    for i in 1..8 {
        (*ts).port_config[i] = 0xAA;
        for j in 0..4 {
            let offset = (i - 1) * 4 + j;
            let ret = max7301_direction_input(&mut (*ts).chip, offset);
            if ret != 0 { return ret; }
        }
    }
    devm_gpiochip_add_data((*ts).dev, &mut (*ts).chip, ts)
}

pub unsafe fn __max730x_remove(dev: *mut device) {
    let ts = dev_get_drvdata(dev);
    ((*ts).write)(dev, 0x04, 0x00);
}

// EXPORT_SYMBOL_GPL(__max730x_probe);
// EXPORT_SYMBOL_GPL(__max730x_remove);
// MODULE_AUTHOR("Juergen Beisert, Wolfram Sang");
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("MAX730x GPIO-Expanders, generic parts");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
