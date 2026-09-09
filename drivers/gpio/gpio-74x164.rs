// SPDX-License-Identifier: GPL-2.0-only
/*
 *  74Hx164 - Generic serial-in/parallel-out 8-bits shift register GPIO driver
 *
 *  Copyright (C) 2010 Gabor Juhos <juhosg@openwrt.org>
 *  Copyright (C) 2010 Miguel Gaio <miguel.gaio@efixo.com>
 */

// Linux kernel dependencies supplied by other translation units.
use crate::linux::*;

const GEN_74X164_NUMBER_GPIOS: u32 = 8;

#[repr(C)]
pub struct gen_74x164_chip {
    pub gpio_chip: gpio_chip,
    pub lock: mutex,
    pub gpiod_oe: *mut gpio_desc,
    pub registers: u32,
    /*
     * Since the registers are chained, every byte sent will make
     * the previous byte shift to the next register in the
     * chain. Thus, the first byte sent will end up in the last
     * register at the end of the transfer. So, to have a logical
     * numbering, store the bytes in reverse order.
     */
    pub buffer: [u8; 0],
}

unsafe fn __gen_74x164_write_config(chip: *mut gen_74x164_chip) -> i32 {
    spi_write(
        to_spi_device((*(*chip).gpio_chip.parent)),
        (*chip).buffer.as_ptr(),
        (*chip).registers as usize,
    )
}

unsafe fn gen_74x164_get_value(gc: *mut gpio_chip, offset: u32) -> i32 {
    let chip = gpiochip_get_data(gc) as *mut gen_74x164_chip;
    let bank: u8 = ((*chip).registers - 1 - offset / 8) as u8;
    let pin: u8 = (offset % 8) as u8;

    let _guard = mutex_guard(&mut (*chip).lock);
    if (*chip).buffer[bank as usize] & (1u8 << pin) != 0 { 1 } else { 0 }
}

unsafe fn gen_74x164_set_value(gc: *mut gpio_chip, offset: u32, val: i32) -> i32 {
    let chip = gpiochip_get_data(gc) as *mut gen_74x164_chip;
    let bank: u8 = ((*chip).registers - 1 - offset / 8) as u8;
    let pin: u8 = (offset % 8) as u8;

    let _guard = mutex_guard(&mut (*chip).lock);

    if val != 0 {
        (*chip).buffer[bank as usize] |= 1u8 << pin;
    } else {
        (*chip).buffer[bank as usize] &= !(1u8 << pin);
    }

    __gen_74x164_write_config(chip)
}

unsafe fn gen_74x164_set_multiple(
    gc: *mut gpio_chip,
    mask: *mut c_ulong,
    bits: *mut c_ulong,
) -> i32 {
    let chip = gpiochip_get_data(gc) as *mut gen_74x164_chip;
    let _guard = mutex_guard(&mut (*chip).lock);

    let mut offset: usize = 0;
    while offset < (*chip).registers as usize * 8 {
        let bankmask = for_each_set_clump8(mask, offset, (*chip).registers as usize * 8);
        let bank = (*chip).registers as usize - 1 - offset / 8;
        let bitmask = bitmap_get_value8(bits, offset) & bankmask;

        (*chip).buffer[bank] &= !bankmask;
        (*chip).buffer[bank] |= bitmask;
        offset += 8;
    }
    __gen_74x164_write_config(chip)
}

unsafe fn gen_74x164_direction_output(gc: *mut gpio_chip, offset: u32, val: i32) -> i32 {
    gen_74x164_set_value(gc, offset, val);
    0
}

unsafe extern "C" fn gen_74x164_deactivate(data: *mut c_void) {
    let chip = data as *mut gen_74x164_chip;
    gpiod_set_value_cansleep((*chip).gpiod_oe, 0);
}

unsafe fn gen_74x164_activate(dev: *mut device, chip: *mut gen_74x164_chip) -> i32 {
    gpiod_set_value_cansleep((*chip).gpiod_oe, 1);
    devm_add_action_or_reset(dev, Some(gen_74x164_deactivate), chip as *mut c_void)
}

unsafe fn gen_74x164_probe(spi: *mut spi_device) -> i32 {
    let dev = &mut (*spi).dev as *mut device;
    let mut chip: *mut gen_74x164_chip;
    let mut nregs: u32 = 0;
    let mut init_state: u32 = 0;
    let mut ret: i32;

    /* bits_per_word cannot be configured in platform data */
    (*spi).bits_per_word = 8;

    ret = spi_setup(spi);
    if ret < 0 { return ret; }

    ret = device_property_read_u32(dev, "registers-number\0", &mut nregs);
    if ret != 0 { return dev_err_probe(dev, ret, "Missing 'registers-number' property.\0"); }

    chip = devm_kzalloc(dev, struct_size::<gen_74x164_chip>(nregs as usize), GFP_KERNEL) as *mut gen_74x164_chip;
    if chip.is_null() { return -12; }
    (*chip).registers = nregs;

    /* Seed the chain with a board-specified initial pattern when present. */
    if device_property_read_u32(dev, "lines-initial-states\0", &mut init_state) == 0 {
        for i in 0..core::cmp::min(nregs, 4) {
            (*chip).buffer[(nregs - 1 - i) as usize] = ((init_state >> (i * 8)) & 0xff) as u8;
        }
    }

    (*chip).gpiod_oe = devm_gpiod_get_optional(dev, "enable\0", GPIOD_OUT_LOW);
    if is_err((*chip).gpiod_oe) { return ptr_err((*chip).gpiod_oe); }

    (*chip).gpio_chip.label = (*spi).modalias;
    (*chip).gpio_chip.direction_output = Some(gen_74x164_direction_output);
    (*chip).gpio_chip.get = Some(gen_74x164_get_value);
    (*chip).gpio_chip.set = Some(gen_74x164_set_value);
    (*chip).gpio_chip.set_multiple = Some(gen_74x164_set_multiple);
    (*chip).gpio_chip.base = -1;
    (*chip).gpio_chip.ngpio = GEN_74X164_NUMBER_GPIOS * (*chip).registers;
    (*chip).gpio_chip.can_sleep = true;
    (*chip).gpio_chip.parent = dev;
    (*chip).gpio_chip.owner = THIS_MODULE;

    ret = devm_mutex_init(dev, &mut (*chip).lock);
    if ret != 0 { return ret; }
    ret = __gen_74x164_write_config(chip);
    if ret != 0 { return dev_err_probe(dev, ret, "Config write failed\0"); }
    ret = gen_74x164_activate(dev, chip);
    if ret != 0 { return ret; }
    devm_gpiochip_add_data(dev, &mut (*chip).gpio_chip, chip as *mut c_void)
}

static GEN_74X164_SPI_IDS: [spi_device_id; 3] = [
    spi_device_id { name: "74hc595\0" },
    spi_device_id { name: "74lvc594\0" },
    spi_device_id { name: "\0" },
];

static GEN_74X164_DT_IDS: [of_device_id; 3] = [
    of_device_id { compatible: "fairchild,74hc595\0" },
    of_device_id { compatible: "nxp,74lvc594\0" },
    of_device_id { compatible: "\0" },
];

static mut gen_74x164_driver: spi_driver = spi_driver {
    driver: driver { name: "74x164\0", of_match_table: GEN_74X164_DT_IDS.as_ptr() },
    probe: Some(gen_74x164_probe),
    id_table: GEN_74X164_SPI_IDS.as_ptr(),
};

module_spi_driver!(gen_74x164_driver);
module_author!("Gabor Juhos <juhosg@openwrt.org>");
module_author!("Miguel Gaio <miguel.gaio@efixo.com>");
module_description!("GPIO expander driver for 74X164 8-bits shift register");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
