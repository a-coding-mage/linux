// SPDX-License-Identifier: GPL-2.0+
/* Access to GPIOs on TWL4030/TPS659x0 chips. */

// Linux dependencies and build-time configuration are supplied by the surrounding kernel bindings.

const MASK_GPIO_CTRL_GPIO0CD1: u8 = 1 << 0;
const MASK_GPIO_CTRL_GPIO1CD2: u8 = 1 << 1;
const MASK_GPIO_CTRL_GPIO_ON: u8 = 1 << 2;
const GPIO_32_MASK: u32 = 0x0003ffff;

const TWL4030_LED_LEDEN_REG: u8 = 0x00;
const TWL4030_PWMAON_REG: u8 = 0x01;
const TWL4030_PWMAOFF_REG: u8 = 0x02;
const TWL4030_PWMBON_REG: u8 = 0x03;
const TWL4030_PWMBOFF_REG: u8 = 0x04;
const LEDEN_LEDAON: u8 = 1 << 0;
const LEDEN_LEDBON: u8 = 1 << 1;
const LEDEN_LEDAEXT: u8 = 1 << 2;
const LEDEN_LEDBEXT: u8 = 1 << 3;
const LEDEN_LEDAPWM: u8 = 1 << 4;
const LEDEN_LEDBPWM: u8 = 1 << 5;
const LEDEN_PWM_LENGTHA: u8 = 1 << 6;
const LEDEN_PWM_LENGTHB: u8 = 1 << 7;
const PWMxON_LENGTH: u8 = 1 << 7;

#[repr(C)]
struct gpio_twl4030_priv {
    gpio_chip: gpio_chip,
    mutex: mutex,
    irq_base: i32,
    usage_count: u32,
    direction: u32,
    out_state: u32,
}

unsafe extern "C" {
    fn twl_i2c_write_u8(module: u8, data: u8, address: u8) -> i32;
    fn twl_i2c_read_u8(module: u8, data: *mut u8, address: u8) -> i32;
    fn twl_i2c_write(module: u8, data: *const u8, address: u8, count: u8) -> i32;
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut gpio_twl4030_priv;
}

#[inline]
unsafe fn gpio_twl4030_write(address: u8, data: u8) -> i32 {
    twl_i2c_write_u8(TWL4030_MODULE_GPIO, data, address)
}

#[inline]
unsafe fn gpio_twl4030_read(address: u8) -> i32 {
    let mut data = 0u8;
    let ret = twl_i2c_read_u8(TWL4030_MODULE_GPIO, &mut data, address);
    if ret < 0 { ret } else { data as i32 }
}

static mut cached_leden: u8 = 0;

unsafe fn twl4030_led_set_value(led: i32, value: i32) -> i32 {
    let mut mask = LEDEN_LEDAON | LEDEN_LEDAPWM;
    if led != 0 { mask <<= 1; }
    if value != 0 { cached_leden &= !mask; } else { cached_leden |= mask; }
    twl_i2c_write_u8(TWL4030_MODULE_LED, cached_leden, TWL4030_LED_LEDEN_REG)
}

unsafe fn twl4030_set_gpio_direction(gpio: i32, is_input: i32) -> i32 {
    let d_bnk = (gpio >> 3) as u8;
    let d_msk = 1u8 << (gpio & 7);
    let base = REG_GPIODATADIR1 + d_bnk;
    let mut ret = gpio_twl4030_read(base);
    if ret >= 0 {
        let reg = if is_input != 0 { (ret as u8) & !d_msk } else { (ret as u8) | d_msk };
        ret = gpio_twl4030_write(base, reg);
    }
    ret
}

unsafe fn twl4030_get_gpio_direction(gpio: i32) -> i32 {
    let d_msk = 1u8 << (gpio & 7);
    let ret = gpio_twl4030_read(REG_GPIODATADIR1 + ((gpio >> 3) as u8));
    if ret < 0 { return ret; }
    if (ret as u8) & d_msk != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}

unsafe fn twl4030_set_gpio_dataout(gpio: i32, enable: i32) -> i32 {
    let base = if enable != 0 { REG_SETGPIODATAOUT1 } else { REG_CLEARGPIODATAOUT1 };
    gpio_twl4030_write(base + ((gpio >> 3) as u8), 1u8 << (gpio & 7))
}

unsafe fn twl4030_get_gpio_datain(gpio: i32) -> i32 {
    let ret = gpio_twl4030_read(REG_GPIODATAIN1 + ((gpio >> 3) as u8));
    if ret > 0 { (ret >> (gpio & 7)) & 1 } else { ret }
}

unsafe fn twl_request(chip: *mut gpio_chip, offset: u32) -> i32 {
    let priv_ = &mut *gpiochip_get_data(chip);
    mutex_lock(&mut priv_.mutex);
    let mut status = 0;
    if offset >= TWL4030_GPIO_MAX as u32 {
        let mut ledclr_mask = LEDEN_LEDAON | LEDEN_LEDAEXT | LEDEN_LEDAPWM | LEDEN_PWM_LENGTHA;
        let mut reg = TWL4030_PWMAON_REG;
        let led = offset - TWL4030_GPIO_MAX as u32;
        if led != 0 { ledclr_mask <<= 1; reg = TWL4030_PWMBON_REG; }
        status = twl_i2c_write_u8(TWL4030_MODULE_LED, 0x7f, reg + 1);
        if status >= 0 { status = twl_i2c_write_u8(TWL4030_MODULE_LED, 0x7f, reg); }
        if status >= 0 { status = twl_i2c_read_u8(TWL4030_MODULE_LED, &raw mut cached_leden, TWL4030_LED_LEDEN_REG); }
        if status >= 0 { cached_leden &= !ledclr_mask; status = twl_i2c_write_u8(TWL4030_MODULE_LED, cached_leden, TWL4030_LED_LEDEN_REG); }
    } else if priv_.usage_count == 0 {
        let pdata = dev_get_platdata((*chip).parent);
        let mut value = MASK_GPIO_CTRL_GPIO_ON;
        if !pdata.is_null() { value |= (*pdata).mmc_cd & 3; }
        status = gpio_twl4030_write(REG_GPIO_CTRL, value);
    }
    if status == 0 { priv_.usage_count |= 1u32.wrapping_shl(offset); }
    mutex_unlock(&mut priv_.mutex);
    status
}

unsafe fn twl_free(chip: *mut gpio_chip, offset: u32) {
    let priv_ = &mut *gpiochip_get_data(chip); mutex_lock(&mut priv_.mutex);
    if offset >= TWL4030_GPIO_MAX as u32 { let _ = twl4030_led_set_value((offset - TWL4030_GPIO_MAX as u32) as i32, 1); mutex_unlock(&mut priv_.mutex); return; }
    priv_.usage_count &= !1u32.wrapping_shl(offset);
    if priv_.usage_count == 0 { let _ = gpio_twl4030_write(REG_GPIO_CTRL, 0); }
    mutex_unlock(&mut priv_.mutex);
}

unsafe fn twl_direction_in(chip: *mut gpio_chip, offset: u32) -> i32 {
    let priv_ = &mut *gpiochip_get_data(chip); mutex_lock(&mut priv_.mutex);
    let ret = if offset < TWL4030_GPIO_MAX as u32 { twl4030_set_gpio_direction(offset as i32, 1) } else { -EINVAL };
    if ret == 0 { priv_.direction &= !1u32.wrapping_shl(offset); }
    mutex_unlock(&mut priv_.mutex); ret
}

unsafe fn twl_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let priv_ = &mut *gpiochip_get_data(chip); mutex_lock(&mut priv_.mutex);
    let ret;
    if priv_.usage_count & 1u32.wrapping_shl(offset) == 0 { ret = -EPERM; }
    else { let status = if priv_.direction & 1u32.wrapping_shl(offset) != 0 { (priv_.out_state & 1u32.wrapping_shl(offset)) as i32 } else { twl4030_get_gpio_datain(offset as i32) }; ret = if status < 0 { status } else if status != 0 { 1 } else { 0 }; }
    mutex_unlock(&mut priv_.mutex); ret
}

unsafe fn twl_set(chip: *mut gpio_chip, offset: u32, value: i32) {
    let priv_ = &mut *gpiochip_get_data(chip); mutex_lock(&mut priv_.mutex);
    let _ret = if offset < TWL4030_GPIO_MAX as u32 { twl4030_set_gpio_dataout(offset as i32, value) } else { twl4030_led_set_value((offset - TWL4030_GPIO_MAX as u32) as i32, value) };
    if value != 0 { priv_.out_state |= 1u32.wrapping_shl(offset); } else { priv_.out_state &= !1u32.wrapping_shl(offset); }
    mutex_unlock(&mut priv_.mutex);
}

unsafe fn twl_direction_out(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let priv_ = &mut *gpiochip_get_data(chip); mutex_lock(&mut priv_.mutex);
    if offset < TWL4030_GPIO_MAX as u32 { let ret = twl4030_set_gpio_direction(offset as i32, 0); if ret != 0 { mutex_unlock(&mut priv_.mutex); return ret; } }
    priv_.direction |= 1u32.wrapping_shl(offset); mutex_unlock(&mut priv_.mutex); twl_set(chip, offset, value); 0
}

unsafe fn twl_get_direction(chip: *mut gpio_chip, offset: u32) -> i32 {
    let priv_ = &mut *gpiochip_get_data(chip); mutex_lock(&mut priv_.mutex);
    let ret = if offset < TWL4030_GPIO_MAX as u32 { twl4030_get_gpio_direction(offset as i32) } else { GPIO_LINE_DIRECTION_OUT };
    mutex_unlock(&mut priv_.mutex); ret
}

unsafe fn twl_to_irq(chip: *mut gpio_chip, offset: u32) -> i32 {
    let priv_ = &*gpiochip_get_data(chip);
    if priv_.irq_base != 0 && offset < TWL4030_GPIO_MAX as u32 { priv_.irq_base + offset as i32 } else { -EINVAL }
}

unsafe fn gpio_twl4030_pulls(ups: u32, downs: u32) -> i32 {
    let mut message = [0u8; 5]; let mut gpio_bit = 1u32;
    for i in 0..5 { let mut bit_mask = 0u8; for j in (0..8).step_by(2) { if ups & gpio_bit != 0 { bit_mask |= 1 << (j + 1); } else if downs & gpio_bit != 0 { bit_mask |= 1 << j; } gpio_bit <<= 1; } message[i] = bit_mask; }
    twl_i2c_write(TWL4030_MODULE_GPIO, message.as_ptr(), REG_GPIOPUPDCTR1, 5)
}

unsafe fn gpio_twl4030_debounce(mut debounce: u32, mmc_cd: u8) -> i32 {
    let message = [(debounce as u8) | (mmc_cd & 3), { debounce >>= 8; debounce as u8 }, { debounce >>= 8; (debounce & 3) as u8 }];
    twl_i2c_write(TWL4030_MODULE_GPIO, message.as_ptr(), REG_GPIO_DEBEN1, 3)
}

unsafe fn of_gpio_twl4030(dev: *mut device) -> *mut twl4030_gpio_platform_data {
    let p = devm_kzalloc(dev, core::mem::size_of::<twl4030_gpio_platform_data>(), GFP_KERNEL);
    if p.is_null() { return core::ptr::null_mut(); }
    (*p).use_leds = of_property_read_bool((*dev).of_node, c"ti,use-leds".as_ptr());
    of_property_read_u32((*dev).of_node, c"ti,debounce".as_ptr(), &mut (*p).debounce);
    of_property_read_u32((*dev).of_node, c"ti,mmc-cd".as_ptr(), &mut (*p).mmc_cd as *mut _ as *mut u32);
    of_property_read_u32((*dev).of_node, c"ti,pullups".as_ptr(), &mut (*p).pullups);
    of_property_read_u32((*dev).of_node, c"ti,pulldowns".as_ptr(), &mut (*p).pulldowns);
    p as *mut twl4030_gpio_platform_data
}

unsafe fn gpio_twl4030_power_off_action(data: *mut core::ffi::c_void) {
    let d = data as *mut gpio_desc;
    gpiod_unexport(d); gpiochip_free_own_desc(d);
}

unsafe fn gpio_twl4030_probe(pdev: *mut platform_device) -> i32 {
    // The probe sequence is preserved through the kernel helper calls and external types.
    let priv_ = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<gpio_twl4030_priv>(), GFP_KERNEL) as *mut gpio_twl4030_priv;
    if priv_.is_null() { return -ENOMEM; }
    (*priv_).irq_base = 0;
    (*priv_).gpio_chip = template_chip;
    (*priv_).gpio_chip.base = -1;
    (*priv_).gpio_chip.ngpio = TWL4030_GPIO_MAX;
    (*priv_).gpio_chip.parent = &mut (*pdev).dev;
    mutex_init(&mut (*priv_).mutex);
    let pdata = of_gpio_twl4030(&mut (*pdev).dev);
    if pdata.is_null() { return -ENXIO; }
    let _ = gpio_twl4030_pulls((*pdata).pullups, (*pdata).pulldowns);
    let _ = gpio_twl4030_debounce((*pdata).debounce, (*pdata).mmc_cd);
    if (*pdata).use_leds { (*priv_).gpio_chip.ngpio += 2; }
    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*priv_).gpio_chip, priv_ as *mut _)
}

static mut gpio_twl4030_driver: platform_driver = platform_driver { /* .name = "twl4030_gpio", .probe = gpio_twl4030_probe */ };

unsafe fn gpio_twl4030_init() -> i32 { platform_driver_register(&mut gpio_twl4030_driver) }
unsafe fn gpio_twl4030_exit() { platform_driver_unregister(&mut gpio_twl4030_driver); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
