/* Rust translation of linux/arch/arm/mach-omap1/board-osk.c. */

/* Kernel headers and build-time conditions are supplied by the surrounding tree. */

pub const OMAP_GPIO_LABEL: &str = "gpio-0-15";
pub const OMAP_OSK_ETHR_START: usize = 0x04800300;
pub const OSK_TPS_GPIO_USB_PWR_EN: u32 = 0;
pub const OSK_TPS_GPIO_LED_D3: u32 = 1;
pub const OSK_TPS_GPIO_LAN_RESET: u32 = 2;
pub const OSK_TPS_GPIO_DSP_PWR_EN: u32 = 3;
pub const OSK_TPS_GPIO_LED_D9: u32 = 4;
pub const OSK_TPS_GPIO_LED_D2: u32 = 5;

extern "C" {
    static mut osk_partitions: [mtd_partition; 4];
    static mut osk_flash_data: physmap_flash_data;
    static mut osk_flash_resource: resource;
    static mut osk5912_flash_device: platform_device;
    static mut osk5912_smc91x_info: smc91x_platdata;
    static mut osk5912_smc91x_resources: [resource; 2];
    static mut osk5912_smc91x_device: platform_device;
    static mut osk5912_cf_resources: [resource; 2];
    static mut osk5912_cf_device: platform_device;
    static mut osk5912_devices: [*mut platform_device; 3];
    static mut tps_leds: [gpio_led; 3];
    static mut tps_leds_gpio_table: gpiod_lookup_table;
    static mut tps_leds_data: gpio_led_platform_data;
    static mut osk5912_tps_leds: platform_device;
    static mut eth_reset: *mut gpio_desc;
    static mut vdd_dsp: *mut gpio_desc;
    static mut tps_board: tps65010_board;
    static mut osk_i2c_board_info: [i2c_board_info; 2];
    static mut osk_usb_gpio_table: gpiod_lookup_table;
    static mut osk_usb_config: omap_usb_config;
    static mut osk_irq_gpio_table: gpiod_lookup_table;
}

/* The following opaque types and constants are provided by the kernel dependencies. */
#[repr(C)] pub struct mtd_partition { pub name: *const u8, pub offset: usize, pub size: usize, pub mask_flags: u32 }
#[repr(C)] pub struct physmap_flash_data { pub width: u32, pub set_vpp: Option<unsafe extern "C" fn()>, pub parts: *mut mtd_partition, pub nr_parts: usize }
#[repr(C)] pub struct resource { pub start: usize, pub end: usize, pub flags: u64 }
#[repr(C)] pub struct platform_device { pub name: *const u8, pub id: i32, pub dev: device, pub num_resources: usize, pub resource: *mut resource }
#[repr(C)] pub struct device { pub platform_data: *mut core::ffi::c_void, pub parent: *mut device }
#[repr(C)] pub struct smc91x_platdata { pub flags: u32, pub leda: u32, pub ledb: u32 }
#[repr(C)] pub struct gpio_led { pub name: *const u8, pub default_trigger: *const u8 }
#[repr(C)] pub struct gpiod_lookup_table { pub dev_id: *const u8, pub table: [u8; 128] }
#[repr(C)] pub struct gpio_led_platform_data { pub num_leds: usize, pub leds: *const gpio_led }
#[repr(C)] pub struct gpio_desc;
#[repr(C)] pub struct gpio_chip;
#[repr(C)] pub struct i2c_client { pub dev: device }
#[repr(C)] pub struct i2c_board_info { pub irq: i32, pub platform_data: *mut core::ffi::c_void }
#[repr(C)] pub struct tps65010_board { pub outmask: u32, pub setup: Option<unsafe extern "C" fn(*mut i2c_client, *mut gpio_chip) -> i32>, pub teardown: Option<unsafe extern "C" fn(*mut i2c_client, *mut gpio_chip)> }
#[repr(C)] pub struct omap_usb_config { pub register_dev: u32, pub register_host: u32, pub hmc_mode: u32, pub rwc: u32, pub pins: [u32; 3] }

extern "C" {
    fn omap1_set_vpp(); fn gpiochip_request_own_desc(*mut gpio_chip, u32, *const u8, u32, u32) -> *mut gpio_desc;
    fn gpiochip_free_own_desc(*mut gpio_desc); fn tps65010_set_gpio_out_value(u32, u32);
    fn tps65010_set_led(u32, u32); fn tps65010_set_low_pwr(u32); fn tps65010_config_vregs1(u32);
    fn gpiod_add_lookup_table(*mut gpiod_lookup_table); fn platform_device_register(*mut platform_device) -> i32;
    fn omap_readl(usize) -> u32; fn omap_writel(u32, usize); fn omap_cfg_reg(u32);
    fn omap_cs3_phys() -> usize; fn pr_debug(*const u8, ...); fn pr_err(*const u8, ...);
    fn gpiod_get(*mut device, *const u8, u32) -> *mut gpio_desc; fn gpiod_to_irq(*mut gpio_desc) -> i32;
    fn irq_set_irq_type(i32, u32); fn platform_add_devices(*mut *mut platform_device, usize) -> i32;
    fn omap1_usb_init(*mut omap_usb_config); fn omap_serial_init();
    fn omap_register_i2c_bus(i32, i32, *mut i2c_board_info, usize) -> i32;
}

unsafe extern "C" fn osk_tps_setup(client: *mut i2c_client, gc: *mut gpio_chip) -> i32 {
    if !cfg!(feature = "CONFIG_TPS65010") { return -38; }
    let d = gpiochip_request_own_desc(gc, OSK_TPS_GPIO_USB_PWR_EN, b"n_vbus_en\0".as_ptr(), 1, 1);
    gpiochip_free_own_desc(d);
    tps65010_set_gpio_out_value(2, 1);
    eth_reset = gpiochip_request_own_desc(gc, OSK_TPS_GPIO_LAN_RESET, b"smc_reset\0".as_ptr(), 1, 0);
    vdd_dsp = gpiochip_request_own_desc(gc, OSK_TPS_GPIO_DSP_PWR_EN, b"dsp_power\0".as_ptr(), 1, 1);
    tps65010_set_led(1, 1); tps65010_set_led(2, 0); tps65010_set_low_pwr(1);
    tps65010_config_vregs1(0x03 | 0x20);
    (*(&mut osk5912_tps_leds)).dev.parent = &mut (*client).dev;
    gpiod_add_lookup_table(&mut tps_leds_gpio_table); platform_device_register(&mut osk5912_tps_leds); 0
}

unsafe extern "C" fn osk_tps_teardown(_: *mut i2c_client, _: *mut gpio_chip) { gpiochip_free_own_desc(eth_reset); gpiochip_free_own_desc(vdd_dsp); }

unsafe extern "C" fn osk_init_smc91x() { let mut l = omap_readl(0); l |= 3; omap_writel(l, 0); }
unsafe extern "C" fn osk_init_cf(seg: i32) {
    let res = &mut osk5912_cf_resources[1]; omap_cfg_reg(0);
    res.start = match seg { 1 => 0, 2 => 0, 3 => omap_cs3_phys(), _ => res.start };
    res.end = res.start + 8192 - 1; osk5912_cf_device.dev.platform_data = seg as usize as *mut core::ffi::c_void;
    omap_writel(0x0004a1b3, seg as usize); omap_writel(0, seg as usize);
}

unsafe extern "C" fn osk_init() {
    osk_init_smc91x(); osk_init_cf(2); let l = omap_readl(3); if l != 0x88013141 { omap_writel(0x88013141, 3); }
    osk_flash_resource.start = omap_cs3_phys(); osk_flash_resource.end = osk_flash_resource.start + 0x02000000 - 1;
    gpiod_add_lookup_table(&mut osk_irq_gpio_table);
    let d = gpiod_get(core::ptr::null_mut(), b"smc_irq\0".as_ptr(), 0); if !d.is_null() { irq_set_irq_type(gpiod_to_irq(d), 1); osk5912_smc91x_resources[1].start = gpiod_to_irq(d) as usize; }
    let d = gpiod_get(core::ptr::null_mut(), b"cf_irq\0".as_ptr(), 0); if !d.is_null() { irq_set_irq_type(gpiod_to_irq(d), 2); osk5912_cf_resources[0].start = gpiod_to_irq(d) as usize; }
    platform_add_devices(osk5912_devices.as_mut_ptr(), 3); let mut l = omap_readl(0); l |= 3 << 1; omap_writel(l, 0);
    gpiod_add_lookup_table(&mut osk_usb_gpio_table); omap1_usb_init(&mut osk_usb_config); omap_serial_init();
    let d = gpiod_get(core::ptr::null_mut(), b"tps65010\0".as_ptr(), 0); if !d.is_null() { osk_i2c_board_info[0].irq = gpiod_to_irq(d); }
    omap_register_i2c_bus(1, 400, osk_i2c_board_info.as_mut_ptr(), 2);
}

/* MACHINE_START(OMAP_OSK, "TI-OSK"): atag_offset=0x100, map_io=omap1_map_io,
 * init_early=omap1_init_early, init_irq=omap1_init_irq, init_machine=osk_init,
 * init_late=omap1_init_late, init_time=omap1_timer_init, restart=omap1_restart. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
