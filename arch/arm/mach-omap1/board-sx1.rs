// SPDX-License-Identifier: GPL-2.0-only
/* Translation of board-sx1.c. */

/* External kernel types, constants, macros, and functions are supplied by the
 * surrounding translated kernel sources. */

pub unsafe fn sx1_i2c_write_byte(devaddr: u8, regoffset: u8, value: u8) -> i32 {
    let adap = i2c_get_adapter(0);
    if adap.is_null() { return -ENODEV; }
    let mut data = [regoffset, value];
    let mut msg = i2c_msg { addr: devaddr, flags: 0, len: 2, buf: data.as_mut_ptr() };
    let err = i2c_transfer(adap, &mut msg, 1);
    i2c_put_adapter(adap);
    if err >= 0 { 0 } else { err }
}

pub unsafe fn sx1_i2c_read_byte(devaddr: u8, regoffset: u8, value: *mut u8) -> i32 {
    let adap = i2c_get_adapter(0);
    if adap.is_null() { return -ENODEV; }
    let mut data = [regoffset, 0];
    let mut msg = i2c_msg { addr: devaddr, flags: 0, len: 1, buf: data.as_mut_ptr() };
    let _ = i2c_transfer(adap, &mut msg, 1);
    msg.addr = devaddr;
    msg.flags = I2C_M_RD;
    msg.len = 1;
    msg.buf = data.as_mut_ptr();
    let err = i2c_transfer(adap, &mut msg, 1);
    *value = data[0];
    i2c_put_adapter(adap);
    if err >= 0 { 0 } else { err }
}

pub unsafe fn sx1_setkeylight(mut keylight: u8) -> i32 {
    if keylight > SOFIA_MAX_LIGHT_VAL { keylight = SOFIA_MAX_LIGHT_VAL; }
    sx1_i2c_write_byte(SOFIA_I2C_ADDR, SOFIA_KEYLIGHT_REG, keylight)
}
pub unsafe fn sx1_getkeylight(keylight: *mut u8) -> i32 {
    sx1_i2c_read_byte(SOFIA_I2C_ADDR, SOFIA_KEYLIGHT_REG, keylight)
}
pub unsafe fn sx1_setbacklight(mut backlight: u8) -> i32 {
    if backlight > SOFIA_MAX_LIGHT_VAL { backlight = SOFIA_MAX_LIGHT_VAL; }
    sx1_i2c_write_byte(SOFIA_I2C_ADDR, SOFIA_BACKLIGHT_REG, backlight)
}
pub unsafe fn sx1_getbacklight(backlight: *mut u8) -> i32 {
    sx1_i2c_read_byte(SOFIA_I2C_ADDR, SOFIA_BACKLIGHT_REG, backlight)
}
pub unsafe fn sx1_setmmipower(onoff: u8) -> i32 {
    let mut dat = 0u8;
    let err = sx1_i2c_read_byte(SOFIA_I2C_ADDR, SOFIA_POWER1_REG, &mut dat);
    if err < 0 { return err; }
    if onoff != 0 { dat |= SOFIA_MMILIGHT_POWER; } else { dat &= !SOFIA_MMILIGHT_POWER; }
    sx1_i2c_write_byte(SOFIA_I2C_ADDR, SOFIA_POWER1_REG, dat)
}
pub unsafe fn sx1_setusbpower(onoff: u8) -> i32 {
    let mut dat = 0u8;
    let err = sx1_i2c_read_byte(SOFIA_I2C_ADDR, SOFIA_POWER1_REG, &mut dat);
    if err < 0 { return err; }
    if onoff != 0 { dat |= SOFIA_USB_POWER; } else { dat &= !SOFIA_USB_POWER; }
    sx1_i2c_write_byte(SOFIA_I2C_ADDR, SOFIA_POWER1_REG, dat)
}

// EXPORT_SYMBOL(sx1_setkeylight), EXPORT_SYMBOL(sx1_getkeylight),
// EXPORT_SYMBOL(sx1_setbacklight), EXPORT_SYMBOL(sx1_getbacklight),
// EXPORT_SYMBOL(sx1_setmmipower), EXPORT_SYMBOL(sx1_setusbpower)

static SX1_KEYMAP: [u32; 28] = [
    KEY!(3,5,GROUP_0 | 117), KEY!(4,0,GROUP_0 | 114), KEY!(4,1,GROUP_2 | 114),
    KEY!(4,2,GROUP_3 | 114), KEY!(0,0,GROUP_1 | KEY_F12), KEY!(3,4,GROUP_1 | KEY_LEFT),
    KEY!(3,2,GROUP_1 | KEY_DOWN), KEY!(3,1,GROUP_1 | KEY_RIGHT), KEY!(3,0,GROUP_1 | KEY_UP),
    KEY!(3,3,GROUP_1 | KEY_POWER), KEY!(0,5,GROUP_1 | KEY_1), KEY!(0,4,GROUP_1 | KEY_2),
    KEY!(0,3,GROUP_1 | KEY_3), KEY!(4,3,GROUP_1 | KEY_4), KEY!(4,4,GROUP_1 | KEY_5),
    KEY!(4,5,GROUP_1 | KEY_KPASTERISK), KEY!(1,4,GROUP_1 | KEY_6), KEY!(1,5,GROUP_1 | KEY_7),
    KEY!(1,3,GROUP_1 | KEY_8), KEY!(2,3,GROUP_1 | KEY_9), KEY!(2,5,GROUP_1 | KEY_0),
    KEY!(2,4,GROUP_1 | 113), KEY!(1,0,GROUP_1 | KEY_F11), KEY!(2,1,GROUP_1 | KEY_YEN),
    KEY!(2,2,GROUP_1 | KEY_F8), KEY!(1,2,GROUP_1 | KEY_LEFTSHIFT),
    KEY!(1,1,GROUP_1 | KEY_BACKSPACE), KEY!(2,0,GROUP_1 | KEY_F7),
];

static mut SX1_KP_RESOURCES: [resource; 1] = [resource { start: INT_KEYBOARD, end: INT_KEYBOARD, flags: IORESOURCE_IRQ }];
static SX1_KEYMAP_DATA: matrix_keymap_data = matrix_keymap_data { keymap: SX1_KEYMAP.as_ptr(), keymap_size: SX1_KEYMAP.len() };
static mut SX1_KP_DATA: omap_kp_platform_data = omap_kp_platform_data { rows: 6, cols: 6, keymap_data: &SX1_KEYMAP_DATA, delay: 80 };
static mut SX1_KP_DEVICE: platform_device = platform_device { name: "omap-keypad", id: -1, dev: device { platform_data: &mut SX1_KP_DATA }, num_resources: 1, resource: SX1_KP_RESOURCES.as_mut_ptr() };

static mut SX1_PARTITIONS: [mtd_partition; 4] = [
    mtd_partition { name: "bootloader", offset: 0x01800000, size: SZ_128K, mask_flags: MTD_WRITEABLE },
    mtd_partition { name: "params", offset: MTDPART_OFS_APPEND, size: SZ_128K, mask_flags: 0 },
    mtd_partition { name: "kernel", offset: MTDPART_OFS_APPEND, size: SZ_2M - 2 * SZ_128K, mask_flags: 0 },
    mtd_partition { name: "filesystem", offset: MTDPART_OFS_APPEND, size: MTDPART_SIZ_FULL, mask_flags: 0 },
];
static mut SX1_FLASH_DATA: physmap_flash_data = physmap_flash_data { width: 2, set_vpp: Some(omap1_set_vpp), parts: SX1_PARTITIONS.as_mut_ptr(), nr_parts: SX1_PARTITIONS.len() };
static mut SX1_NEW_FLASH_RESOURCE: resource = resource { start: OMAP_CS0_PHYS, end: OMAP_CS0_PHYS + SZ_32M - 1, flags: IORESOURCE_MEM };
static mut SX1_FLASH_DEVICE: platform_device = platform_device { name: "physmap-flash", id: 0, dev: device { platform_data: &mut SX1_FLASH_DATA }, num_resources: 1, resource: &mut SX1_NEW_FLASH_RESOURCE };

static mut SX1_USB_CONFIG: omap_usb_config = omap_usb_config { otg: 0, register_dev: 1, register_host: 0, hmc_mode: 0, pins: [2, 0, 0] };
static SX1_LCD_CONFIG: omap_lcd_config = omap_lcd_config { ctrl_name: "internal" };
static mut SX1_DEVICES: [*mut platform_device; 2] = [&mut SX1_FLASH_DEVICE, &mut SX1_KP_DEVICE];

unsafe fn omap_sx1_init() {
    omap_cfg_reg(UART1_TX); omap_cfg_reg(UART1_RTS); omap_cfg_reg(UART2_TX);
    omap_cfg_reg(UART2_RTS); omap_cfg_reg(UART3_TX); omap_cfg_reg(UART3_RX);
    platform_add_devices(SX1_DEVICES.as_mut_ptr(), SX1_DEVICES.len());
    omap_serial_init(); omap_register_i2c_bus(1, 100, core::ptr::null_mut(), 0);
    omap1_usb_init(&mut SX1_USB_CONFIG); sx1_mmc_init();
    gpiod_add_lookup_table(&mut SX1_GPIO_TABLE);
    let mut d = gpiod_get(core::ptr::null_mut(), "irda_off", GPIOD_OUT_HIGH);
    if IS_ERR(d) { pr_err!("Unable to get IRDA OFF GPIO descriptor\n"); } else { gpiod_put(d); }
    d = gpiod_get(core::ptr::null_mut(), "switch", GPIOD_OUT_LOW);
    if IS_ERR(d) { pr_err!("Unable to get SWITCH GPIO descriptor\n"); } else { gpiod_put(d); }
    d = gpiod_get(core::ptr::null_mut(), "usb_on", GPIOD_OUT_LOW);
    if IS_ERR(d) { pr_err!("Unable to get USB ON GPIO descriptor\n"); } else { gpiod_put(d); }
    omapfb_set_lcd_config(&SX1_LCD_CONFIG);
}

static mut SX1_GPIO_TABLE: gpiod_lookup_table = gpiod_lookup_table { dev_id: core::ptr::null(), table: [GPIO_LOOKUP!("gpio-0-15",1,"irda_off",GPIO_ACTIVE_HIGH), GPIO_LOOKUP!("gpio-0-15",11,"switch",GPIO_ACTIVE_HIGH), GPIO_LOOKUP!("gpio-0-15",15,"usb_on",GPIO_ACTIVE_HIGH), GPIO_LOOKUP_END!] };

// MACHINE_START(SX1, "OMAP310 based Siemens SX1")
// .atag_offset = 0x100, .map_io = omap1_map_io, .init_early = omap1_init_early,
// .init_irq = omap1_init_irq, .init_machine = omap_sx1_init,
// .init_late = omap1_init_late, .init_time = omap1_timer_init,
// .restart = omap1_restart, MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
