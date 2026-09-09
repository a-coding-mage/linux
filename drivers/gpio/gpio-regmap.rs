// SPDX-License-Identifier: GPL-2.0-only
/* regmap based generic GPIO driver */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct gpio_regmap {
    pub parent: *mut device,
    pub regmap: *mut regmap,
    pub gpio_chip: gpio_chip,
    pub reg_stride: i32,
    pub ngpio_per_reg: i32,
    pub reg_dat_base: u32,
    pub reg_set_base: u32,
    pub reg_clr_base: u32,
    pub reg_dir_in_base: u32,
    pub reg_dir_out_base: u32,
    pub fixed_direction_mask: *mut c_ulong,
    pub fixed_direction_output: *mut c_ulong,
    #[cfg(CONFIG_REGMAP_IRQ)]
    pub regmap_irq_line: i32,
    #[cfg(CONFIG_REGMAP_IRQ)]
    pub irq_chip_data: *mut regmap_irq_chip_data,
    pub reg_mask_xlate: Option<unsafe extern "C" fn(*mut gpio_regmap, gpio_regmap_operation, u32, u32, *mut u32, *mut u32) -> i32>,
    pub value_xlate: Option<unsafe extern "C" fn(*mut gpio_regmap, gpio_regmap_operation, u32, u32, u32, *mut u32, *mut u32) -> i32>,
    pub set_config: Option<unsafe extern "C" fn(*mut gpio_regmap, *mut gpio_chip, u32, c_ulong) -> i32>,
    pub driver_data: *mut c_void,
}

unsafe fn gpio_regmap_addr(addr: u32) -> u32 {
    if addr == GPIO_REGMAP_ADDR_ZERO { 0 } else { addr }
}

unsafe extern "C" fn gpio_regmap_simple_xlate(gpio: *mut gpio_regmap, _op: gpio_regmap_operation, base: u32, offset: u32, reg: *mut u32, mask: *mut u32) -> i32 {
    let line = offset % (*gpio).ngpio_per_reg as u32;
    let stride = offset / (*gpio).ngpio_per_reg as u32;
    *reg = base + stride * (*gpio).reg_stride as u32;
    *mask = 1u32.wrapping_shl(line);
    0
}

unsafe extern "C" fn gpio_regmap_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip);
    let base = if (*gpio).reg_dat_base != 0 { gpio_regmap_addr((*gpio).reg_dat_base) } else { gpio_regmap_addr((*gpio).reg_set_base) };
    let mut val = 0u32; let mut reg = 0u32; let mut mask = 0u32;
    let mut ret = ((*gpio).reg_mask_xlate.unwrap())(gpio, GPIO_REGMAP_GET_OP, base, offset, &mut reg, &mut mask);
    if ret != 0 { return ret; }
    ret = if (*gpio).reg_dat_base == (*gpio).reg_set_base { regmap_read_bypassed((*gpio).regmap, reg, &mut val) } else { regmap_read((*gpio).regmap, reg, &mut val) };
    if ret != 0 { return ret; }
    if val & mask != 0 { 1 } else { 0 }
}

unsafe extern "C" fn gpio_regmap_set(chip: *mut gpio_chip, offset: u32, val: i32) -> i32 {
    let gpio = gpiochip_get_data(chip); let base = gpio_regmap_addr((*gpio).reg_set_base);
    let (mut reg, mut mask, mut mask_val) = (0u32, 0u32, if val != 0 { 0u32 } else { 0u32 });
    let mut ret = ((*gpio).reg_mask_xlate.unwrap())(gpio, GPIO_REGMAP_SET_OP, base, offset, &mut reg, &mut mask); if ret != 0 { return ret; }
    mask_val = if val != 0 { mask } else { 0 };
    if let Some(xlate) = (*gpio).value_xlate { ret = xlate(gpio, GPIO_REGMAP_SET_OP, base, offset, reg, &mut mask, &mut mask_val); if ret != 0 { return ret; } }
    if (*gpio).reg_dat_base == (*gpio).reg_set_base { regmap_write_bits((*gpio).regmap, reg, mask, mask_val) } else { regmap_update_bits((*gpio).regmap, reg, mask, mask_val) }
}

unsafe extern "C" fn gpio_regmap_set_with_clear(chip: *mut gpio_chip, offset: u32, val: i32) -> i32 {
    let gpio = gpiochip_get_data(chip); let base = if val != 0 { gpio_regmap_addr((*gpio).reg_set_base) } else { gpio_regmap_addr((*gpio).reg_clr_base) };
    let (mut reg, mut mask, mut value) = (0u32, 0u32, 0u32);
    let mut ret = ((*gpio).reg_mask_xlate.unwrap())(gpio, GPIO_REGMAP_SET_OP, base, offset, &mut reg, &mut mask); if ret != 0 { return ret; }
    if let Some(xlate) = (*gpio).value_xlate { ret = xlate(gpio, GPIO_REGMAP_SET_OP, base, offset, reg, &mut mask, &mut value); if ret != 0 { return ret; } }
    regmap_write((*gpio).regmap, reg, mask)
}

unsafe fn gpio_regmap_fixed_direction(gpio: *mut gpio_regmap, offset: u32) -> bool {
    if (*gpio).fixed_direction_output.is_null() { return false; }
    if !(*gpio).fixed_direction_mask.is_null() && test_bit(offset, (*gpio).fixed_direction_mask) == 0 { return false; }
    true
}

unsafe extern "C" fn gpio_regmap_get_direction(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip);
    if gpio_regmap_fixed_direction(gpio, offset) { return if test_bit(offset, (*gpio).fixed_direction_output) != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }; }
    if (*gpio).reg_dat_base != 0 && (*gpio).reg_set_base == 0 { return GPIO_LINE_DIRECTION_IN; }
    if (*gpio).reg_set_base != 0 && (*gpio).reg_dat_base == 0 { return GPIO_LINE_DIRECTION_OUT; }
    let (base, invert) = if (*gpio).reg_dir_out_base != 0 { (gpio_regmap_addr((*gpio).reg_dir_out_base), 0) } else if (*gpio).reg_dir_in_base != 0 { (gpio_regmap_addr((*gpio).reg_dir_in_base), 1) } else { return -ENOTSUPP; };
    let (mut val, mut reg, mut mask) = (0u32, 0u32, 0u32); let mut ret = ((*gpio).reg_mask_xlate.unwrap())(gpio, GPIO_REGMAP_GET_DIR_OP, base, offset, &mut reg, &mut mask); if ret != 0 { return ret; }
    ret = regmap_read((*gpio).regmap, reg, &mut val); if ret != 0 { return ret; }
    if ((if val & mask != 0 { 1 } else { 0 }) ^ invert) != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}

unsafe fn gpio_regmap_try_direction_fixed(gpio: *mut gpio_regmap, offset: u32, output: bool) -> i32 {
    let is_output = test_bit(offset, (*gpio).fixed_direction_output) != 0;
    if is_output == output { 0 } else { -EINVAL }
}

unsafe extern "C" fn gpio_regmap_set_direction(chip: *mut gpio_chip, offset: u32, output: bool) -> i32 {
    let gpio = gpiochip_get_data(chip);
    if gpio_regmap_fixed_direction(gpio, offset) { return gpio_regmap_try_direction_fixed(gpio, offset, output); }
    let (base, invert) = if (*gpio).reg_dir_out_base != 0 { (gpio_regmap_addr((*gpio).reg_dir_out_base), false) } else if (*gpio).reg_dir_in_base != 0 { (gpio_regmap_addr((*gpio).reg_dir_in_base), true) } else { return -ENOTSUPP; };
    let (mut val, mut reg, mut mask) = (0u32, 0u32, 0u32); let mut ret = ((*gpio).reg_mask_xlate.unwrap())(gpio, GPIO_REGMAP_SET_DIR_OP, base, offset, &mut reg, &mut mask); if ret != 0 { return ret; }
    val = if invert { if output { 0 } else { mask } } else if output { mask } else { 0 };
    if let Some(xlate) = (*gpio).value_xlate { ret = xlate(gpio, GPIO_REGMAP_SET_DIR_OP, base, offset, reg, &mut mask, &mut val); if ret != 0 { return ret; } }
    regmap_update_bits((*gpio).regmap, reg, mask, val)
}

unsafe extern "C" fn gpio_regmap_direction_input(chip: *mut gpio_chip, offset: u32) -> i32 { gpio_regmap_set_direction(chip, offset, false) }
unsafe extern "C" fn gpio_regmap_direction_output(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let gpio = gpiochip_get_data(chip); if gpio_regmap_fixed_direction(gpio, offset) { let ret = gpio_regmap_try_direction_fixed(gpio, offset, true); if ret != 0 { return ret; } }
    gpio_regmap_set(chip, offset, value); gpio_regmap_set_direction(chip, offset, true)
}
unsafe extern "C" fn gpio_regmap_set_config(chip: *mut gpio_chip, offset: u32, cfg: c_ulong) -> i32 { let gpio = gpiochip_get_data(chip); ((*gpio).set_config.unwrap())(gpio, chip, offset, cfg) }

pub unsafe extern "C" fn gpio_regmap_reqres_irq(gpio: *mut gpio_regmap, offset: u32) -> i32 { gpiochip_reqres_irq(&mut (*gpio).gpio_chip, offset) }
pub unsafe extern "C" fn gpio_regmap_relres_irq(gpio: *mut gpio_regmap, offset: u32) { gpiochip_relres_irq(&mut (*gpio).gpio_chip, offset); }
pub unsafe extern "C" fn gpio_regmap_get_drvdata(gpio: *mut gpio_regmap) -> *mut c_void { (*gpio).driver_data }
pub unsafe extern "C" fn gpio_regmap_enable_irq(gpio: *mut gpio_regmap, hwirq: irq_hw_number_t) { gpiochip_enable_irq(&mut (*gpio).gpio_chip, hwirq); }
pub unsafe extern "C" fn gpio_regmap_disable_irq(gpio: *mut gpio_regmap, hwirq: irq_hw_number_t) { gpiochip_disable_irq(&mut (*gpio).gpio_chip, hwirq); }

pub unsafe extern "C" fn gpio_regmap_register(config: *const gpio_regmap_config) -> *mut gpio_regmap {
    if (*config).parent.is_null() || ((*config).reg_dat_base == 0 && (*config).reg_set_base == 0) || (((*config).reg_dir_out_base != 0 || (*config).reg_dir_in_base != 0) && ((*config).reg_dat_base == 0 || (*config).reg_set_base == 0)) || ((*config).reg_dir_out_base != 0 && (*config).reg_dir_in_base != 0) { return ERR_PTR(-EINVAL); }
    let gpio = kzalloc_obj::<gpio_regmap>(); if gpio.is_null() { return ERR_PTR(-ENOMEM); }
    (*gpio).parent = (*config).parent; (*gpio).driver_data = (*config).drvdata; (*gpio).regmap = (*config).regmap;
    (*gpio).reg_dat_base = (*config).reg_dat_base; (*gpio).reg_set_base = (*config).reg_set_base; (*gpio).reg_clr_base = (*config).reg_clr_base; (*gpio).reg_dir_in_base = (*config).reg_dir_in_base; (*gpio).reg_dir_out_base = (*config).reg_dir_out_base;
    let chip = &mut (*gpio).gpio_chip; chip.parent = (*config).parent; chip.fwnode = (*config).fwnode; chip.base = -1; chip.names = (*config).names; chip.label = if !(*config).label.is_null() { (*config).label } else { dev_name((*config).parent) }; chip.can_sleep = regmap_might_sleep((*config).regmap); chip.init_valid_mask = (*config).init_valid_mask;
    chip.request = Some(gpiochip_generic_request); chip.free = Some(gpiochip_generic_free); chip.get = Some(gpio_regmap_get); if (*gpio).reg_set_base != 0 && (*gpio).reg_clr_base != 0 { chip.set = Some(gpio_regmap_set_with_clear); } else if (*gpio).reg_set_base != 0 { chip.set = Some(gpio_regmap_set); }
    chip.get_direction = Some(gpio_regmap_get_direction); if (*gpio).reg_dir_in_base != 0 || (*gpio).reg_dir_out_base != 0 { chip.direction_input = Some(gpio_regmap_direction_input); chip.direction_output = Some(gpio_regmap_direction_output); }
    chip.ngpio = (*config).ngpio; if chip.ngpio == 0 { let ret = gpiochip_get_ngpios(chip, chip.parent); if ret != 0 { kfree(gpio as *mut c_void); return ERR_PTR(ret); } }
    (*gpio).ngpio_per_reg = if (*config).ngpio_per_reg != 0 { (*config).ngpio_per_reg } else { chip.ngpio as i32 }; (*gpio).reg_stride = if (*config).reg_stride != 0 { (*config).reg_stride } else { 1 }; (*gpio).reg_mask_xlate = Some((*config).reg_mask_xlate.unwrap_or(gpio_regmap_simple_xlate)); (*gpio).value_xlate = (*config).value_xlate; if let Some(f) = (*config).set_config { (*gpio).set_config = Some(f); chip.set_config = Some(gpio_regmap_set_config); }
    let ret = gpiochip_add_data(chip, gpio); if ret < 0 { kfree(gpio as *mut c_void); return ERR_PTR(ret); } gpio
}

pub unsafe extern "C" fn gpio_regmap_unregister(gpio: *mut gpio_regmap) { gpiochip_remove(&mut (*gpio).gpio_chip); kfree(gpio as *mut c_void); }
unsafe extern "C" fn devm_gpio_regmap_unregister(res: *mut c_void) { gpio_regmap_unregister(res as *mut gpio_regmap); }
pub unsafe extern "C" fn devm_gpio_regmap_register(dev: *mut device, config: *const gpio_regmap_config) -> *mut gpio_regmap { let gpio = gpio_regmap_register(config); if IS_ERR(gpio) { return gpio; } let ret = devm_add_action_or_reset(dev, Some(devm_gpio_regmap_unregister), gpio as *mut c_void); if ret != 0 { return ERR_PTR(ret); } gpio }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
