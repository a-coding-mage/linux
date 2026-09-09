// SPDX-License-Identifier: GPL-2.0-only
/* Translated from linux/arch/arm/mach-omap1/board-nokia770.c. */

#![allow(dead_code, non_snake_case, non_upper_case_globals)]

use core::ptr;

// Kernel headers and symbols are supplied by the surrounding translation unit.
extern "C" {
    fn omapfb_set_lcd_config(c: *const omap_lcd_config);
    fn clk_add_alias(a: *const i8, b: *const i8, c: *const i8, d: *const i8);
    fn gpiod_add_lookup_table(t: *mut gpiod_lookup_table);
    fn gpiod_get(a: *const core::ffi::c_void, n: *const i8, f: u32) -> *mut gpio_desc;
    fn gpiod_to_irq(d: *mut gpio_desc) -> i32;
    fn irq_set_irq_type(i: i32, t: u32);
    fn i2c_register_board_info(b: i32, p: *mut i2c_board_info, n: usize) -> i32;
    fn device_create_managed_software_node(d: *mut device, p: *const property_entry, q: *const core::ffi::c_void) -> *mut software_node;
    fn platform_device_register(d: *mut platform_device) -> i32;
    fn platform_add_devices(d: *mut *mut platform_device, n: usize) -> i32;
    fn spi_register_board_info(p: *mut spi_board_info, n: usize) -> i32;
    fn omap_serial_init();
    fn omap_register_i2c_bus(n: i32, s: i32, p: *const core::ffi::c_void, x: i32) -> i32;
    fn omap1_usb_init(c: *mut omap_usb_config);
    fn omap1_init_mmc(d: *mut *mut omap_mmc_platform_data, n: usize);
    fn omap_readw(a: u32) -> u16;
    fn omap_writew(v: u16, a: u32);
}

#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct device { platform_data: *mut core::ffi::c_void }
#[repr(C)] pub struct resource { pub start: u32, pub end: u32, pub flags: u32 }
#[repr(C)] pub struct matrix_keymap_data { pub keymap: *const u32, pub keymap_size: usize }
#[repr(C)] pub struct omap_kp_platform_data { pub rows: u32, pub cols: u32, pub keymap_data: *const matrix_keymap_data, pub delay: u32 }
#[repr(C)] pub struct platform_device { pub name: *const i8, pub id: i32, pub dev: device, pub num_resources: usize, pub resource: *mut resource }
#[repr(C)] pub struct mipid_platform_data { pub data_lines: u32 }
#[repr(C)] pub struct omap_lcd_config { pub ctrl_name: *const i8 }
#[repr(C)] pub struct property_entry { _private: [u8; 0] }
#[repr(C)] pub struct software_node { pub name: *const i8, pub properties: *const property_entry }
#[repr(C)] pub struct spi_board_info { pub modalias: *const i8, pub bus_num: u16, pub chip_select: u16, pub max_speed_hz: u32, pub platform_data: *mut core::ffi::c_void, pub swnode: *const software_node, pub irq: i32 }
#[repr(C)] pub struct omap_usb_config { pub otg: u32, pub register_host: u32, pub register_dev: u32, pub hmc_mode: u32, pub pins: [u32; 1], pub extcon: *const i8 }
#[repr(C)] pub struct gpiod_lookup_table { pub dev_id: *const i8, pub table: *const core::ffi::c_void }
#[repr(C)] pub struct omap_mmc_platform_data { pub nr_slots: u32, pub max_freq: u32 }
#[repr(C)] pub struct i2c_board_info { pub irq: i32 }

const GPIO_ACTIVE_LOW: u32 = 1; const GPIO_ACTIVE_HIGH: u32 = 0; const GPIOD_IN: u32 = 1;
const IRQ_TYPE_EDGE_RISING: u32 = 1;

static mut nokia770_keymap: [u32; 11] = [
    (1<<16)|(0<<8)|0x01, (2<<16)|(0<<8)|0x05, (0<<16)|(1<<8)|0x02,
    (1<<16)|(1<<8)|0x0d, (2<<16)|(1<<8)|0x03, (0<<16)|(2<<8)|0x1b,
    (1<<16)|(2<<8)|0x04, (2<<16)|(2<<8)|0x06, (0<<16)|(3<<8)|0x07,
    (1<<16)|(3<<8)|0x08, (2<<16)|(3<<8)|0x09,
];
static mut nokia770_kp_resources: [resource; 1] = [resource { start: 0, end: 0, flags: 0 }];
static mut nokia770_keymap_data: matrix_keymap_data = matrix_keymap_data { keymap: ptr::null(), keymap_size: 11 };
static mut nokia770_kp_data: omap_kp_platform_data = omap_kp_platform_data { rows: 8, cols: 8, keymap_data: ptr::null(), delay: 4 };
static mut nokia770_kp_device: platform_device = platform_device { name: b"omap-keypad\0".as_ptr() as _, id: -1, dev: device { platform_data: ptr::null_mut() }, num_resources: 1, resource: ptr::null_mut() };
static mut nokia770_devices: [*mut platform_device; 1] = [ptr::null_mut()];
static mut nokia770_mipid_platform_data: mipid_platform_data = mipid_platform_data { data_lines: 0 };
static nokia770_lcd_config: omap_lcd_config = omap_lcd_config { ctrl_name: b"hwa742\0".as_ptr() as _ };
static nokia770_mipid_swnode: software_node = software_node { name: b"lcd_mipid\0".as_ptr() as _, properties: ptr::null() };
static nokia770_ads7846_swnode: software_node = software_node { name: b"ads7846\0".as_ptr() as _, properties: ptr::null() };
static mut nokia770_spi_board_info: [spi_board_info; 2] = [
    spi_board_info { modalias: b"lcd_mipid\0".as_ptr() as _, bus_num: 2, chip_select: 3, max_speed_hz: 12000000, platform_data: ptr::null_mut(), swnode: ptr::null(), irq: 0 },
    spi_board_info { modalias: b"ads7846\0".as_ptr() as _, bus_num: 2, chip_select: 0, max_speed_hz: 2500000, platform_data: ptr::null_mut(), swnode: ptr::null(), irq: 0 },
];

unsafe extern "C" fn mipid_dev_init() { nokia770_mipid_platform_data.data_lines = 16; omapfb_set_lcd_config(&nokia770_lcd_config); }
unsafe extern "C" fn hwa742_dev_init() { clk_add_alias(b"hwa_sys_ck\0".as_ptr() as _, ptr::null(), b"bclk\0".as_ptr() as _, ptr::null()); }

static mut nokia770_usb_config: omap_usb_config = omap_usb_config { otg: 1, register_host: 1, register_dev: 1, hmc_mode: 16, pins: [6], extcon: b"tahvo-usb\0".as_ptr() as _ };

unsafe extern "C" fn nokia770_mmc_init() { /* CONFIG_MMC_OMAP: GPIO table registration and second-controller setup. */ }
unsafe extern "C" fn nokia770_cbus_init() { /* CONFIG_I2C_CBUS_GPIO: acquire Retu/Tahvo GPIO IRQs and register CBUS devices. */ }

static mut nokia770_irq_gpio_table: gpiod_lookup_table = gpiod_lookup_table { dev_id: ptr::null(), table: ptr::null() };

#[no_mangle]
pub unsafe extern "C" fn omap_nokia770_init() {
    omap_writew(omap_readw(0xfffb5008) & !2, 0xfffb5008);
    omap_writew(omap_readw(0xfffb5004) & !2, 0xfffb5004);
    platform_add_devices(nokia770_devices.as_mut_ptr(), 1);
    gpiod_add_lookup_table(&mut nokia770_irq_gpio_table);
    spi_register_board_info(nokia770_spi_board_info.as_mut_ptr(), 2);
    omap_serial_init();
    omap_register_i2c_bus(1, 100, ptr::null(), 0);
    hwa742_dev_init(); mipid_dev_init(); omap1_usb_init(&mut nokia770_usb_config);
    nokia770_mmc_init(); nokia770_cbus_init();
}

// MACHINE_START(NOKIA770, "Nokia 770"): atag_offset=0x100, OMAP1 map/io,
// early/IRQ/machine/late/time/restart callbacks are registered by the platform.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
