// SPDX-License-Identifier: GPL-2.0-only
/* Analog Devices ADP5585 GPIO driver */

// Dependencies supplied by the surrounding kernel translation.

const fn adp5585_bank(n: u32) -> i32 { if n >= 6 { 1 } else { 0 } }
const fn adp5585_bit(n: u32) -> i32 { if n >= 6 { 1 << (n - 6) } else { 1 << n } }
const fn adp5589_bank(n: u32) -> i32 { (n >> 3) as i32 }
const fn adp5589_bit(n: u32) -> i32 { 1 << (n & 7) }

#[repr(C)]
struct Adp5585GpioChip {
    bank: unsafe extern "C" fn(u32) -> i32,
    bit: unsafe extern "C" fn(u32) -> i32,
    debounce_dis_a: u32, rpull_cfg_a: u32, gpo_data_a: u32, gpo_out_a: u32,
    gpio_dir_a: u32, gpi_stat_a: u32, gpi_int_lvl_a: u32, gpi_ev_a: u32,
    gpi_ev_min: u32, gpi_ev_max: u32, has_bias_hole: bool,
}

#[repr(C)]
struct Adp5585GpioDev {
    gpio_chip: GpioChip,
    nb: NotifierBlock,
    info: *const Adp5585GpioChip,
    regmap: *mut Regmap,
    irq_mask: usize, irq_en: usize, irq_active_high: usize,
    bus_lock: Mutex,
}

unsafe extern "C" fn adp5585_gpio_bank(off: u32) -> i32 { adp5585_bank(off) }
unsafe extern "C" fn adp5585_gpio_bit(off: u32) -> i32 { adp5585_bit(off) }
unsafe extern "C" fn adp5589_gpio_bank(off: u32) -> i32 { adp5589_bank(off) }
unsafe extern "C" fn adp5589_gpio_bit(off: u32) -> i32 { adp5589_bit(off) }

unsafe extern "C" fn adp5585_gpio_get_direction(chip: *mut GpioChip, off: u32) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut Adp5585GpioDev;
    let info = &*(*gpio).info; let mut val = 0u32;
    regmap_read((*gpio).regmap, info.gpio_dir_a + (info.bank)(off) as u32, &mut val);
    if val & (info.bit)(off) as u32 != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}

unsafe extern "C" fn adp5585_gpio_direction_input(chip: *mut GpioChip, off: u32) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut Adp5585GpioDev; let info = &*(*gpio).info;
    regmap_clear_bits((*gpio).regmap, info.gpio_dir_a + (info.bank)(off) as u32, (info.bit)(off) as u32)
}

unsafe extern "C" fn adp5585_gpio_direction_output(chip: *mut GpioChip, off: u32, val: i32) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut Adp5585GpioDev; let info = &*(*gpio).info;
    let bank = (info.bank)(off) as u32; let bit = (info.bit)(off) as u32;
    let ret = regmap_update_bits((*gpio).regmap, info.gpo_data_a + bank, bit, if val != 0 { bit } else { 0 });
    if ret != 0 { return ret; } regmap_set_bits((*gpio).regmap, info.gpio_dir_a + bank, bit)
}

unsafe extern "C" fn adp5585_gpio_get_value(chip: *mut GpioChip, off: u32) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut Adp5585GpioDev; let info = &*(*gpio).info;
    let bank = (info.bank)(off) as u32; let bit = (info.bit)(off) as u32; let mut val = 0u32;
    regmap_read((*gpio).regmap, info.gpio_dir_a + bank, &mut val);
    let reg = if val & bit != 0 { info.gpo_data_a } else { info.gpi_stat_a };
    regmap_read((*gpio).regmap, reg + bank, &mut val); if val & bit != 0 { 1 } else { 0 }
}

unsafe extern "C" fn adp5585_gpio_set_value(chip: *mut GpioChip, off: u32, val: i32) {
    let gpio = gpiochip_get_data(chip) as *mut Adp5585GpioDev; let info = &*(*gpio).info;
    let bit = (info.bit)(off) as u32;
    regmap_update_bits((*gpio).regmap, info.gpo_data_a + (info.bank)(off) as u32, bit, if val != 0 { bit } else { 0 });
}

// The remaining callbacks and registration structures retain the kernel ABI and
// are declared in terms of the surrounding translation's kernel types.
unsafe fn adp5585_gpio_set_bias(gpio: *mut Adp5585GpioDev, off: u32, bias: u32) -> i32 {
    let info = &*(*gpio).info; let mut bit = off * 2;
    if info.has_bias_hole && off > 5 { bit += 4; }
    let reg = info.rpull_cfg_a + bit / 8;
    let mask = ADP5585_Rx_PULL_CFG_MASK << (bit % 8);
    regmap_update_bits((*gpio).regmap, reg, mask, bias << (bit % 8))
}

unsafe fn adp5585_gpio_set_drive(gpio: *mut Adp5585GpioDev, off: u32, drive: u32) -> i32 {
    let info = &*(*gpio).info; let bit = (info.bit)(off) as u32;
    regmap_update_bits((*gpio).regmap, info.gpo_out_a + (info.bank)(off) as u32,
        bit, if drive == PIN_CONFIG_DRIVE_OPEN_DRAIN { bit } else { 0 })
}

unsafe fn adp5585_gpio_set_debounce(gpio: *mut Adp5585GpioDev, off: u32, debounce: u32) -> i32 {
    let info = &*(*gpio).info; let bit = (info.bit)(off) as u32;
    regmap_update_bits((*gpio).regmap, info.debounce_dis_a + (info.bank)(off) as u32,
        bit, if debounce != 0 { 0 } else { bit })
}

unsafe extern "C" fn adp5585_gpio_set_config(chip: *mut GpioChip, off: u32, config: usize) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut Adp5585GpioDev;
    let param = pinconf_to_config_param(config); let arg = pinconf_to_config_argument(config);
    match param {
        PIN_CONFIG_BIAS_DISABLE => adp5585_gpio_set_bias(gpio, off, ADP5585_Rx_PULL_CFG_DISABLE),
        PIN_CONFIG_BIAS_PULL_DOWN => adp5585_gpio_set_bias(gpio, off, if arg != 0 { ADP5585_Rx_PULL_CFG_PD_300K } else { ADP5585_Rx_PULL_CFG_DISABLE }),
        PIN_CONFIG_BIAS_PULL_UP => adp5585_gpio_set_bias(gpio, off, if arg != 0 { ADP5585_Rx_PULL_CFG_PU_300K } else { ADP5585_Rx_PULL_CFG_DISABLE }),
        PIN_CONFIG_DRIVE_OPEN_DRAIN | PIN_CONFIG_DRIVE_PUSH_PULL => adp5585_gpio_set_drive(gpio, off, param),
        PIN_CONFIG_INPUT_DEBOUNCE => adp5585_gpio_set_debounce(gpio, off, arg),
        _ => -ENOTSUPP,
    }
}

extern "C" {
    fn adp5585_gpio_request(chip: *mut GpioChip, off: u32) -> i32;
    fn adp5585_gpio_free(chip: *mut GpioChip, off: u32);
    fn adp5585_gpio_probe(pdev: *mut PlatformDevice) -> i32;
}

// External kernel types and functions referenced above are provided by other
// translated units; constants below mirror the C driver's local metadata.
static ADP5585_GPIO_CHIP_INFO: Adp5585GpioChip = Adp5585GpioChip {
    bank: adp5585_gpio_bank, bit: adp5585_gpio_bit,
    debounce_dis_a: ADP5585_DEBOUNCE_DIS_A, rpull_cfg_a: ADP5585_RPULL_CONFIG_A,
    gpo_data_a: ADP5585_GPO_DATA_OUT_A, gpo_out_a: ADP5585_GPO_OUT_MODE_A,
    gpio_dir_a: ADP5585_GPIO_DIRECTION_A, gpi_stat_a: ADP5585_GPI_STATUS_A,
    gpi_int_lvl_a: ADP5585_GPI_INT_LEVEL_A, gpi_ev_a: ADP5585_GPI_EVENT_EN_A,
    gpi_ev_min: ADP5585_GPI_EVENT_START, gpi_ev_max: ADP5585_GPI_EVENT_END,
    has_bias_hole: true,
};
static ADP5589_GPIO_CHIP_INFO: Adp5585GpioChip = Adp5585GpioChip {
    bank: adp5589_gpio_bank, bit: adp5589_gpio_bit,
    debounce_dis_a: ADP5589_DEBOUNCE_DIS_A, rpull_cfg_a: ADP5589_RPULL_CONFIG_A,
    gpo_data_a: ADP5589_GPO_DATA_OUT_A, gpo_out_a: ADP5589_GPO_OUT_MODE_A,
    gpio_dir_a: ADP5589_GPIO_DIRECTION_A, gpi_stat_a: ADP5589_GPI_STATUS_A,
    gpi_int_lvl_a: ADP5589_GPI_INT_LEVEL_A, gpi_ev_a: ADP5589_GPI_EVENT_EN_A,
    gpi_ev_min: ADP5589_GPI_EVENT_START, gpi_ev_max: ADP5589_GPI_EVENT_END,
    has_bias_hole: false,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
