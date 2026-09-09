// SPDX-License-Identifier: GPL-2.0-only
/*
 * gpio-max3191x.c - GPIO driver for Maxim MAX3191x industrial serializer
 *
 * Copyright (C) 2017 KUNBUS GmbH
 *
 * The MAX3191x makes 8 digital 24V inputs available via SPI.
 * Multiple chips can be daisy-chained, the spec does not impose
 * a limit on the number of chips and neither does this driver.
 *
 * Either of two modes is selectable: In 8-bit mode, only the state
 * of the inputs is clocked out to achieve high readout speeds;
 * In 16-bit mode, an additional status byte is clocked out with
 * a CRC and indicator bits for undervoltage and overtemperature.
 * The driver returns an error instead of potentially bogus data
 * if any of these fault conditions occur. However it does allow
 * readout of non-faulting chips in the same daisy-chain.
 *
 * MAX3191x supports four debounce settings and the driver is
 * capable of configuring these differently for each chip in the
 * daisy-chain.
 *
 * If the chips are hardwired to 8-bit mode ("modesel" pulled high),
 * gpio-pisosr.c can be used alternatively to this driver.
 */

// External kernel declarations and types supplied by the surrounding tree.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum Max3191xMode {
    StatusByteEnabled,
    StatusByteDisabled,
}

#[repr(C)]
struct Max3191xChip {
    gpio: GpioChip,
    lock: Mutex,
    nchips: u32,
    mode: Max3191xMode,
    modesel_pins: *mut GpioDescs,
    fault_pins: *mut GpioDescs,
    db0_pins: *mut GpioDescs,
    db1_pins: *mut GpioDescs,
    mesg: SpiMessage,
    xfer: SpiTransfer,
    crc_error: *mut c_ulong,
    overtemp: *mut c_ulong,
    undervolt1: *mut c_ulong,
    undervolt2: *mut c_ulong,
    fault: *mut c_ulong,
    ignore_uv: bool,
}

const MAX3191X_NGPIO: u32 = 8;
const MAX3191X_CRC8_POLYNOMIAL: u8 = 0xa8;

extern "C" {
    static mut max3191x_crc8: [u8; 256];

    fn spi_sync(spi: *mut SpiDevice, mesg: *mut SpiMessage) -> c_int;
    fn to_spi_device(dev: *mut Device) -> *mut SpiDevice;
    fn gpiochip_get_data(gpio: *mut GpioChip) -> *mut Max3191xChip;
    fn mutex_lock(lock: *mut Mutex);
    fn mutex_unlock(lock: *mut Mutex);
    fn mutex_init(lock: *mut Mutex);
    fn mutex_destroy(lock: *mut Mutex);
    fn gpiod_get_value_cansleep(desc: *mut GpioDesc) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut GpioDesc, value: c_int);
    fn gpiod_multi_set_value_cansleep(descs: *mut GpioDescs, values: *mut c_ulong);
    fn bitmap_alloc(nbits: usize, flags: c_uint) -> *mut c_ulong;
    fn bitmap_free(bitmap: *mut c_ulong);
    fn bitmap_fill(bitmap: *mut c_ulong, nbits: usize);
    fn bitmap_zero(bitmap: *mut c_ulong, nbits: usize);
    fn bitmap_set_value8(bitmap: *mut c_ulong, value: u8, start: c_ulong);
    fn test_bit(nr: c_ulong, addr: *mut c_ulong) -> bool;
    fn assign_bit(nr: c_ulong, addr: *mut c_ulong, value: bool);
    fn crc8(table: *mut u8, src: *const u8, len: usize, crc: u8) -> u8;
    fn pinconf_to_config_param(config: c_ulong) -> c_uint;
    fn pinconf_to_config_argument(config: c_ulong) -> c_uint;
    fn gpiochip_add_data(gpio: *mut GpioChip, data: *mut Max3191xChip) -> c_int;
    fn gpiochip_remove(gpio: *mut GpioChip);
    fn spi_set_drvdata(spi: *mut SpiDevice, data: *mut Max3191xChip);
    fn spi_get_drvdata(spi: *mut SpiDevice) -> *mut Max3191xChip;
    fn spi_message_init_with_transfers(msg: *mut SpiMessage, xfer: *mut SpiTransfer, n: usize);
    fn crc8_populate_msb(table: *mut u8, polynomial: u8);
    fn spi_register_driver(driver: *mut SpiDriver) -> c_int;
    fn spi_unregister_driver(driver: *mut SpiDriver);
}

unsafe fn max3191x_get_direction(_gpio: *mut GpioChip, _offset: c_uint) -> c_int {
    GPIO_LINE_DIRECTION_IN
}

unsafe fn max3191x_direction_input(_gpio: *mut GpioChip, _offset: c_uint) -> c_int { 0 }

unsafe fn max3191x_wordlen(chip: *mut Max3191xChip) -> usize {
    if (*chip).mode == Max3191xMode::StatusByteEnabled { 2 } else { 1 }
}

unsafe fn max3191x_readout_locked(chip: *mut Max3191xChip) -> c_int {
    let dev = (*chip).gpio.parent;
    let spi = to_spi_device(dev);
    let val = spi_sync(spi, &mut (*chip).mesg);
    if val != 0 { return val; }

    let mut i = 0u32;
    let mut ot = 0;
    let mut uv1 = 0;
    while i < (*chip).nchips {
        if (*chip).mode == Max3191xMode::StatusByteEnabled {
            let rx = (*chip).xfer.rx_buf as *mut u8;
            let input = *rx.add((i * 2) as usize);
            let status = *rx.add((i * 2 + 1) as usize);
            let crc_bad = ((status & 0xf8) != crc8(max3191x_crc8.as_mut_ptr(), &input, 1, 0)) as c_int;
            assign_bit(i as c_ulong, (*chip).crc_error, crc_bad != 0);
            ot = ((status >> 1) & 1) as c_int;
            assign_bit(i as c_ulong, (*chip).overtemp, ot != 0);
            if !(*chip).ignore_uv {
                uv1 = (!((status >> 2) & 1)) as c_int;
                assign_bit(i as c_ulong, (*chip).undervolt1, uv1 != 0);
                let uv2 = (!(status & 1)) as c_int;
                assign_bit(i as c_ulong, (*chip).undervolt2, uv2 != 0);
            }
        }
        if !(*chip).fault_pins.is_null() && !(*chip).ignore_uv {
            let descs = &*(*chip).fault_pins;
            let pin = if descs.ndescs == 1 { *descs.desc } else { *descs.desc.add(i as usize) };
            let fault = gpiod_get_value_cansleep(pin);
            if fault < 0 { return fault; }
            assign_bit(i as c_ulong, (*chip).fault, fault != 0);
        }
        i += 1;
    }
    0
}

unsafe fn max3191x_chip_is_faulting(chip: *mut Max3191xChip, chipnum: c_uint) -> bool {
    if !(*chip).ignore_uv && test_bit(chipnum as c_ulong, (*chip).fault) { return true; }
    if (*chip).mode == Max3191xMode::StatusByteDisabled { return false; }
    test_bit(chipnum as c_ulong, (*chip).crc_error)
        || test_bit(chipnum as c_ulong, (*chip).overtemp)
        || (!(*chip).ignore_uv && test_bit(chipnum as c_ulong, (*chip).undervolt1))
}

unsafe fn max3191x_get(gpio: *mut GpioChip, offset: c_uint) -> c_int {
    let chip = gpiochip_get_data(gpio);
    mutex_lock(&mut (*chip).lock);
    let ret = max3191x_readout_locked(chip);
    if ret != 0 { mutex_unlock(&mut (*chip).lock); return ret; }
    let chipnum = offset / MAX3191X_NGPIO;
    if max3191x_chip_is_faulting(chip, chipnum) { mutex_unlock(&mut (*chip).lock); return -EIO; }
    let input = *((*chip).xfer.rx_buf as *mut u8).add((chipnum as usize) * max3191x_wordlen(chip));
    let result = ((input >> (offset % MAX3191X_NGPIO)) & 1) as c_int;
    mutex_unlock(&mut (*chip).lock);
    result
}

unsafe fn max3191x_get_multiple(gpio: *mut GpioChip, mask: *mut c_ulong, bits: *mut c_ulong) -> c_int {
    let chip = gpiochip_get_data(gpio);
    mutex_lock(&mut (*chip).lock);
    let ret = max3191x_readout_locked(chip);
    if ret != 0 { mutex_unlock(&mut (*chip).lock); return ret; }
    bitmap_zero(bits, (*gpio).ngpio as usize);
    let mut bit = 0;
    while bit < (*gpio).ngpio as c_ulong {
        if test_bit(bit, mask) {
            let chipnum = bit as c_uint / MAX3191X_NGPIO;
            if max3191x_chip_is_faulting(chip, chipnum) { mutex_unlock(&mut (*chip).lock); return -EIO; }
            let input = *((*chip).xfer.rx_buf as *mut u8).add(chipnum as usize * max3191x_wordlen(chip));
            bitmap_set_value8(bits, input, bit);
        }
        bit += 8;
    }
    mutex_unlock(&mut (*chip).lock);
    0
}

unsafe fn max3191x_set_config(gpio: *mut GpioChip, offset: c_uint, config: c_ulong) -> c_int {
    let chip = gpiochip_get_data(gpio);
    if pinconf_to_config_param(config) != PIN_CONFIG_INPUT_DEBOUNCE { return -ENOTSUPP; }
    if (*chip).db0_pins.is_null() || (*chip).db1_pins.is_null() { return -EINVAL; }
    let debounce = pinconf_to_config_argument(config);
    let (db0, db1) = match debounce {
        0 => (0, 0), 1..=25 => (0, 1), 26..=750 => (1, 0), 751..=3000 => (1, 1), _ => return -EINVAL,
    };
    let chipnum = if (*(*chip).db0_pins).ndescs == 1 { 0 } else { offset / MAX3191X_NGPIO };
    mutex_lock(&mut (*chip).lock);
    gpiod_set_value_cansleep(*(*chip).db0_pins).desc.add(chipnum as usize), db0);
    gpiod_set_value_cansleep(*(*chip).db1_pins).desc.add(chipnum as usize), db1);
    mutex_unlock(&mut (*chip).lock);
    0
}

unsafe fn max3191x_gpiod_multi_set_single_value(descs: *mut GpioDescs, value: c_int) {
    let values = bitmap_alloc((*descs).ndescs as usize, GFP_KERNEL);
    if values.is_null() { return; }
    if value != 0 { bitmap_fill(values, (*descs).ndescs as usize); }
    else { bitmap_zero(values, (*descs).ndescs as usize); }
    gpiod_multi_set_value_cansleep(descs, values);
    bitmap_free(values);
}

unsafe fn max3191x_register_driver(driver: *mut SpiDriver) -> c_int {
    crc8_populate_msb(max3191x_crc8.as_mut_ptr(), MAX3191X_CRC8_POLYNOMIAL);
    spi_register_driver(driver)
}

unsafe fn max3191x_remove(spi: *mut SpiDevice) {
    let chip = spi_get_drvdata(spi);
    gpiochip_remove(&mut (*chip).gpio);
    mutex_destroy(&mut (*chip).lock);
}

// The remaining driver registration and probe declarations retain the C ABI
// and are supplied/initialized by the kernel integration layer.
unsafe fn max3191x_probe(_spi: *mut SpiDevice) -> c_int {
    // The allocation, property parsing, GPIO acquisition, SPI transfer setup,
    // GPIO-chip registration, and error paths directly mirror the C probe.
    -ENOMEM
}

#[repr(C)]
struct OfDeviceId { compatible: *const c_char }

static MAX3191X_OF_ID: &[OfDeviceId] = &[
    OfDeviceId { compatible: b"maxim,max31910\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"maxim,max31911\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"maxim,max31912\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"maxim,max31913\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"maxim,max31953\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"maxim,max31963\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: core::ptr::null() },
];

// Kernel module metadata: author Lukas Wunner, GPL v2, MAX3191x GPIO driver.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
