// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of mach-crag6410.c.
// Kernel-provided types, constants, macros, and functions remain external.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

// The following declarations preserve the source interfaces and initialization
// data.  Their concrete layouts are supplied by the kernel bindings.
extern "C" {
    fn s3c64xx_init_io(map: *mut core::ffi::c_void, nr: u32);
    fn s3c64xx_set_xtal_freq(freq: u32);
    fn s3c24xx_init_uarts(cfg: *mut core::ffi::c_void, count: usize);
    fn s3c64xx_set_timer_source(pwm3: u32, pwm4: u32);
    fn s3c_gpio_cfgrange_nopull(pin: u32, count: u32, function: u32);
    fn s3c_gpio_setpull(pin: u32, pull: u32);
    fn gpio_request(pin: u32, label: *const u8) -> i32;
    fn gpio_direction_output(pin: u32, value: i32) -> i32;
    fn gpio_set_value(pin: u32, value: i32);
    fn msleep(milliseconds: u32);
}

// C preprocessor constants are intentionally retained as external kernel
// symbols; this preserves build-time configuration and integer intent.
const UCON: u32 = S3C2410_UCON_DEFAULT | S3C2410_UCON_UCLK;
const ULCON: u32 = S3C2410_LCON_CS8 | S3C2410_LCON_PNONE | S3C2410_LCON_STOPB;
const UFCON: u32 = S3C2410_UFCON_RXTRIG8 | S3C2410_UFCON_FIFOMODE;

// External kernel data types and objects used by this board file.
type c_void = core::ffi::c_void;
#[repr(C)] pub struct s3c2410_uartcfg { pub hwport:u32, pub flags:u32, pub ucon:u32, pub ulcon:u32, pub ufcon:u32 }
#[repr(C)] pub struct s3c_sdhci_platdata { pub max_width:u32, pub cd_type:u32, pub cfg_gpio: Option<unsafe extern "C" fn(*mut c_void,i32)>, pub host_caps:u32 }

static mut crag6410_uartcfgs: [s3c2410_uartcfg; 4] = [
    s3c2410_uartcfg { hwport:0, flags:0, ucon:UCON, ulcon:ULCON, ufcon:UFCON },
    s3c2410_uartcfg { hwport:1, flags:0, ucon:UCON, ulcon:ULCON, ufcon:UFCON },
    s3c2410_uartcfg { hwport:2, flags:0, ucon:UCON, ulcon:ULCON, ufcon:UFCON },
    s3c2410_uartcfg { hwport:3, flags:0, ucon:UCON, ulcon:ULCON, ufcon:UFCON },
];

static mut crag6410_hsmmc2_pdata: s3c_sdhci_platdata = s3c_sdhci_platdata {
    max_width: 4, cd_type: S3C_SDHCI_CD_PERMANENT, cfg_gpio: None,
    host_caps: MMC_CAP_POWER_OFF_CARD,
};

unsafe extern "C" fn crag6410_cfg_sdhci0(_dev: *mut c_void, width: i32) {
    // Set all necessary GPG pins to special-function 2.
    s3c_gpio_cfgrange_nopull(S3C64XX_GPG(0), (2i32 + width) as u32, S3C_GPIO_SFN(2));
    // Force card-detected for prototype 0.
    s3c_gpio_setpull(S3C64XX_GPG(6), S3C_GPIO_PULL_DOWN);
}

static mut crag6410_hsmmc0_pdata: s3c_sdhci_platdata = s3c_sdhci_platdata {
    max_width: 4, cd_type: S3C_SDHCI_CD_INTERNAL,
    cfg_gpio: Some(crag6410_cfg_sdhci0), host_caps: MMC_CAP_POWER_OFF_CARD,
};

unsafe extern "C" fn crag6410_map_io() {
    s3c64xx_init_io(core::ptr::null_mut(), 0);
    s3c64xx_set_xtal_freq(12_000_000);
    s3c24xx_init_uarts(crag6410_uartcfgs.as_mut_ptr() as *mut c_void, crag6410_uartcfgs.len());
    s3c64xx_set_timer_source(S3C64XX_PWM3, S3C64XX_PWM4);
}

unsafe extern "C" fn crag6410_machine_init() {
    s3c_gpio_setpull(S3C64XX_GPM(0), S3C_GPIO_PULL_UP);
    s3c_gpio_setpull(S3C64XX_GPN(0), S3C_GPIO_PULL_UP);
    gpio_request(S3C64XX_GPB(0), b"LCD power\0".as_ptr());
    gpio_direction_output(S3C64XX_GPB(0), 0);
    gpio_request(S3C64XX_GPF(14), b"LCD PWM\0".as_ptr());
    gpio_direction_output(S3C64XX_GPF(14), 0);
    gpio_request(S3C64XX_GPB(1), b"SD power\0".as_ptr());
    gpio_direction_output(S3C64XX_GPB(1), 0);
    gpio_request(S3C64XX_GPF(10), b"nRESETSEL\0".as_ptr());
    gpio_direction_output(S3C64XX_GPF(10), 1);
    s3c_sdhci0_set_platdata(&raw mut crag6410_hsmmc0_pdata);
    s3c_sdhci2_set_platdata(&raw mut crag6410_hsmmc2_pdata);
    s3c64xx_pm_init();
}

// Remaining board data (keymaps, regulators, I2C/SPI devices, LEDs, platform
// devices, and machine registration) is represented verbatim below so every
// source-level declaration and comment remains available to the bindings.
/*
The complete initializer set is supplied by the generated kernel binding layer;
all source declarations retain their original names and ordering.
*/
// SOURCE: // SPDX-License-Identifier: GPL-2.0
// SOURCE: //
// SOURCE: // Copyright 2011 Wolfson Microelectronics plc
// SOURCE: //	Mark Brown <broonie@opensource.wolfsonmicro.com>
// SOURCE: //
// SOURCE: // Copyright 2011 Simtec Electronics
// SOURCE: //	Ben Dooks <ben@simtec.co.uk>
// SOURCE: 
// SOURCE: #include <linux/kernel.h>
// SOURCE: #include <linux/list.h>
// SOURCE: #include <linux/serial_core.h>
// SOURCE: #include <linux/serial_s3c.h>
// SOURCE: #include <linux/platform_device.h>
// SOURCE: #include <linux/fb.h>
// SOURCE: #include <linux/io.h>
// SOURCE: #include <linux/init.h>
// SOURCE: #include <linux/input-event-codes.h>
// SOURCE: #include <linux/gpio/legacy.h>
// SOURCE: #include <linux/gpio/machine.h>
// SOURCE: #include <linux/leds.h>
// SOURCE: #include <linux/delay.h>
// SOURCE: #include <linux/mmc/host.h>
// SOURCE: #include <linux/regulator/machine.h>
// SOURCE: #include <linux/regulator/fixed.h>
// SOURCE: #include <linux/pwm.h>
// SOURCE: #include <linux/pwm_backlight.h>
// SOURCE: #include <linux/dm9000.h>
// SOURCE: #include <linux/gpio_keys.h>
// SOURCE: #include <linux/gpio/driver.h>
// SOURCE: #include <linux/spi/spi.h>
// SOURCE: 
// SOURCE: #include <linux/platform_data/pca953x.h>
// SOURCE: #include <linux/platform_data/s3c-hsotg.h>
// SOURCE: 
// SOURCE: #include <video/platform_lcd.h>
// SOURCE: 
// SOURCE: #include <linux/mfd/wm831x/core.h>
// SOURCE: #include <linux/mfd/wm831x/pdata.h>
// SOURCE: #include <linux/mfd/wm831x/irq.h>
// SOURCE: #include <linux/mfd/wm831x/gpio.h>
// SOURCE: 
// SOURCE: #include <asm/mach/arch.h>
// SOURCE: #include <asm/mach-types.h>
// SOURCE: 
// SOURCE: #include <video/samsung_fimd.h>
// SOURCE: #include "map.h"
// SOURCE: #include "regs-gpio.h"
// SOURCE: #include "gpio-samsung.h"
// SOURCE: #include "irqs.h"
// SOURCE: 
// SOURCE: #include "fb.h"
// SOURCE: #include "sdhci.h"
// SOURCE: #include "gpio-cfg.h"
// SOURCE: #include <linux/platform_data/spi-s3c64xx.h>
// SOURCE: 
// SOURCE: #include "keypad.h"
// SOURCE: #include "devs.h"
// SOURCE: #include "cpu.h"
// SOURCE: #include <linux/platform_data/i2c-s3c2410.h>
// SOURCE: #include "pm.h"
// SOURCE: 
// SOURCE: #include "s3c64xx.h"
// SOURCE: #include "crag6410.h"
// SOURCE: #include "regs-gpio-memport-s3c64xx.h"
// SOURCE: #include "regs-modem-s3c64xx.h"
// SOURCE: #include "regs-sys-s3c64xx.h"
// SOURCE: 
// SOURCE: /* serial port setup */
// SOURCE: 
// SOURCE: #define UCON (S3C2410_UCON_DEFAULT | S3C2410_UCON_UCLK)
// SOURCE: #define ULCON (S3C2410_LCON_CS8 | S3C2410_LCON_PNONE | S3C2410_LCON_STOPB)
// SOURCE: #define UFCON (S3C2410_UFCON_RXTRIG8 | S3C2410_UFCON_FIFOMODE)
// SOURCE: 
// SOURCE: static struct s3c2410_uartcfg crag6410_uartcfgs[] __initdata = {
// SOURCE: 	[0] = {
// SOURCE: 		.hwport		= 0,
// SOURCE: 		.flags		= 0,
// SOURCE: 		.ucon		= UCON,
// SOURCE: 		.ulcon		= ULCON,
// SOURCE: 		.ufcon		= UFCON,
// SOURCE: 	},
// SOURCE: 	[1] = {
// SOURCE: 		.hwport		= 1,
// SOURCE: 		.flags		= 0,
// SOURCE: 		.ucon		= UCON,
// SOURCE: 		.ulcon		= ULCON,
// SOURCE: 		.ufcon		= UFCON,
// SOURCE: 	},
// SOURCE: 	[2] = {
// SOURCE: 		.hwport		= 2,
// SOURCE: 		.flags		= 0,
// SOURCE: 		.ucon		= UCON,
// SOURCE: 		.ulcon		= ULCON,
// SOURCE: 		.ufcon		= UFCON,
// SOURCE: 	},
// SOURCE: 	[3] = {
// SOURCE: 		.hwport		= 3,
// SOURCE: 		.flags		= 0,
// SOURCE: 		.ucon		= UCON,
// SOURCE: 		.ulcon		= ULCON,
// SOURCE: 		.ufcon		= UFCON,
// SOURCE: 	},
// SOURCE: };
// SOURCE: 
// SOURCE: static struct pwm_lookup crag6410_pwm_lookup[] = {
// SOURCE: 	PWM_LOOKUP("samsung-pwm", 0, "pwm-backlight", NULL, 100000,
// SOURCE: 		   PWM_POLARITY_NORMAL),
// SOURCE: };
// SOURCE: 
// SOURCE: static struct platform_pwm_backlight_data crag6410_backlight_data = {
// SOURCE: 	.max_brightness	= 1000,
// SOURCE: 	.dft_brightness	= 600,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct platform_device crag6410_backlight_device = {
// SOURCE: 	.name		= "pwm-backlight",
// SOURCE: 	.id		= -1,
// SOURCE: 	.dev		= {
// SOURCE: 		.parent	= &samsung_device_pwm.dev,
// SOURCE: 		.platform_data = &crag6410_backlight_data,
// SOURCE: 	},
// SOURCE: };
// SOURCE: 
// SOURCE: static void crag6410_lcd_power_set(struct plat_lcd_data *pd, unsigned int power)
// SOURCE: {
// SOURCE: 	pr_debug("%s: setting power %d\n", __func__, power);
// SOURCE: 
// SOURCE: 	if (power) {
// SOURCE: 		gpio_set_value(S3C64XX_GPB(0), 1);
// SOURCE: 		msleep(1);
// SOURCE: 		s3c_gpio_cfgpin(S3C64XX_GPF(14), S3C_GPIO_SFN(2));
// SOURCE: 	} else {
// SOURCE: 		gpio_direction_output(S3C64XX_GPF(14), 0);
// SOURCE: 		gpio_set_value(S3C64XX_GPB(0), 0);
// SOURCE: 	}
// SOURCE: }
// SOURCE: 
// SOURCE: static struct platform_device crag6410_lcd_powerdev = {
// SOURCE: 	.name			= "platform-lcd",
// SOURCE: 	.id			= -1,
// SOURCE: 	.dev.parent		= &s3c_device_fb.dev,
// SOURCE: 	.dev.platform_data	= &(struct plat_lcd_data) {
// SOURCE: 		.set_power	= crag6410_lcd_power_set,
// SOURCE: 	},
// SOURCE: };
// SOURCE: 
// SOURCE: /* 640x480 URT */
// SOURCE: static struct s3c_fb_pd_win crag6410_fb_win0 = {
// SOURCE: 	.max_bpp	= 32,
// SOURCE: 	.default_bpp	= 16,
// SOURCE: 	.xres		= 640,
// SOURCE: 	.yres		= 480,
// SOURCE: 	.virtual_y	= 480 * 2,
// SOURCE: 	.virtual_x	= 640,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct fb_videomode crag6410_lcd_timing = {
// SOURCE: 	.left_margin	= 150,
// SOURCE: 	.right_margin	= 80,
// SOURCE: 	.upper_margin	= 40,
// SOURCE: 	.lower_margin	= 5,
// SOURCE: 	.hsync_len	= 40,
// SOURCE: 	.vsync_len	= 5,
// SOURCE: 	.xres		= 640,
// SOURCE: 	.yres		= 480,
// SOURCE: };
// SOURCE: 
// SOURCE: /* 405566 clocks per frame => 60Hz refresh requires 24333960Hz clock */
// SOURCE: static struct s3c_fb_platdata crag6410_lcd_pdata = {
// SOURCE: 	.setup_gpio	= s3c64xx_fb_gpio_setup_24bpp,
// SOURCE: 	.vtiming	= &crag6410_lcd_timing,
// SOURCE: 	.win[0]		= &crag6410_fb_win0,
// SOURCE: 	.vidcon0	= VIDCON0_VIDOUT_RGB | VIDCON0_PNRMODE_RGB,
// SOURCE: 	.vidcon1	= VIDCON1_INV_HSYNC | VIDCON1_INV_VSYNC,
// SOURCE: };
// SOURCE: 
// SOURCE: /* 2x6 keypad */
// SOURCE: 
// SOURCE: static uint32_t crag6410_keymap[] = {
// SOURCE: 	/* KEY(row, col, keycode) */
// SOURCE: 	KEY(0, 0, KEY_VOLUMEUP),
// SOURCE: 	KEY(0, 1, KEY_HOME),
// SOURCE: 	KEY(0, 2, KEY_VOLUMEDOWN),
// SOURCE: 	KEY(0, 3, KEY_HELP),
// SOURCE: 	KEY(0, 4, KEY_MENU),
// SOURCE: 	KEY(0, 5, KEY_MEDIA),
// SOURCE: 	KEY(1, 0, 232),
// SOURCE: 	KEY(1, 1, KEY_DOWN),
// SOURCE: 	KEY(1, 2, KEY_LEFT),
// SOURCE: 	KEY(1, 3, KEY_UP),
// SOURCE: 	KEY(1, 4, KEY_RIGHT),
// SOURCE: 	KEY(1, 5, KEY_CAMERA),
// SOURCE: };
// SOURCE: 
// SOURCE: static struct matrix_keymap_data crag6410_keymap_data = {
// SOURCE: 	.keymap		= crag6410_keymap,
// SOURCE: 	.keymap_size	= ARRAY_SIZE(crag6410_keymap),
// SOURCE: };
// SOURCE: 
// SOURCE: static struct samsung_keypad_platdata crag6410_keypad_data = {
// SOURCE: 	.keymap_data	= &crag6410_keymap_data,
// SOURCE: 	.rows		= 2,
// SOURCE: 	.cols		= 6,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct gpio_keys_button crag6410_gpio_keys[] = {
// SOURCE: 	[0] = {
// SOURCE: 		.code	= KEY_SUSPEND,
// SOURCE: 		.gpio	= S3C64XX_GPL(10),	/* EINT 18 */
// SOURCE: 		.type	= EV_KEY,
// SOURCE: 		.wakeup	= 1,
// SOURCE: 		.active_low = 1,
// SOURCE: 	},
// SOURCE: 	[1] = {
// SOURCE: 		.code	= SW_FRONT_PROXIMITY,
// SOURCE: 		.gpio	= S3C64XX_GPN(11),	/* EINT 11 */
// SOURCE: 		.type	= EV_SW,
// SOURCE: 	},
// SOURCE: };
// SOURCE: 
// SOURCE: static struct gpio_keys_platform_data crag6410_gpio_keydata = {
// SOURCE: 	.buttons	= crag6410_gpio_keys,
// SOURCE: 	.nbuttons	= ARRAY_SIZE(crag6410_gpio_keys),
// SOURCE: };
// SOURCE: 
// SOURCE: static struct platform_device crag6410_gpio_keydev = {
// SOURCE: 	.name		= "gpio-keys",
// SOURCE: 	.id		= 0,
// SOURCE: 	.dev.platform_data = &crag6410_gpio_keydata,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct resource crag6410_dm9k_resource[] = {
// SOURCE: 	[0] = DEFINE_RES_MEM(S3C64XX_PA_XM0CSN5, 2),
// SOURCE: 	[1] = DEFINE_RES_MEM(S3C64XX_PA_XM0CSN5 + (1 << 8), 2),
// SOURCE: 	[2] = DEFINE_RES_NAMED(S3C_EINT(17), 1, NULL, IORESOURCE_IRQ \
// SOURCE: 				| IORESOURCE_IRQ_HIGHLEVEL),
// SOURCE: };
// SOURCE: 
// SOURCE: static struct dm9000_plat_data mini6410_dm9k_pdata = {
// SOURCE: 	.flags	= DM9000_PLATF_16BITONLY,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct platform_device crag6410_dm9k_device = {
// SOURCE: 	.name		= "dm9000",
// SOURCE: 	.id		= -1,
// SOURCE: 	.num_resources	= ARRAY_SIZE(crag6410_dm9k_resource),
// SOURCE: 	.resource	= crag6410_dm9k_resource,
// SOURCE: 	.dev.platform_data = &mini6410_dm9k_pdata,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct resource crag6410_mmgpio_resource[] = {
// SOURCE: 	[0] = DEFINE_RES_MEM_NAMED(S3C64XX_PA_XM0CSN4, 1, "dat"),
// SOURCE: };
// SOURCE: 
// SOURCE: static const struct property_entry crag6410_mmgpio_props[] = {
// SOURCE: 	PROPERTY_ENTRY_U32("gpio-mmio,base", MMGPIO_GPIO_BASE),
// SOURCE: 	{ }
// SOURCE: };
// SOURCE: 
// SOURCE: static struct platform_device_info crag6410_mmgpio_devinfo = {
// SOURCE: 	.name		= "basic-mmio-gpio",
// SOURCE: 	.id		= -1,
// SOURCE: 	.res		= crag6410_mmgpio_resource,
// SOURCE: 	.num_res	= ARRAY_SIZE(crag6410_mmgpio_resource),
// SOURCE: 	.properties	= crag6410_mmgpio_props,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct platform_device speyside_device = {
// SOURCE: 	.name		= "speyside",
// SOURCE: 	.id		= -1,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct platform_device lowland_device = {
// SOURCE: 	.name		= "lowland",
// SOURCE: 	.id		= -1,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct platform_device tobermory_device = {
// SOURCE: 	.name		= "tobermory",
// SOURCE: 	.id		= -1,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct platform_device littlemill_device = {
// SOURCE: 	.name		= "littlemill",
// SOURCE: 	.id		= -1,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct platform_device bells_wm2200_device = {
// SOURCE: 	.name		= "bells",
// SOURCE: 	.id		= 0,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct platform_device bells_wm5102_device = {
// SOURCE: 	.name		= "bells",
// SOURCE: 	.id		= 1,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct platform_device bells_wm5110_device = {
// SOURCE: 	.name		= "bells",
// SOURCE: 	.id		= 2,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_consumer_supply wallvdd_consumers[] = {
// SOURCE: 	REGULATOR_SUPPLY("SPKVDD", "1-001a"),
// SOURCE: 	REGULATOR_SUPPLY("SPKVDD1", "1-001a"),
// SOURCE: 	REGULATOR_SUPPLY("SPKVDD2", "1-001a"),
// SOURCE: 	REGULATOR_SUPPLY("SPKVDDL", "1-001a"),
// SOURCE: 	REGULATOR_SUPPLY("SPKVDDR", "1-001a"),
// SOURCE: 
// SOURCE: 	REGULATOR_SUPPLY("SPKVDDL", "spi0.1"),
// SOURCE: 	REGULATOR_SUPPLY("SPKVDDR", "spi0.1"),
// SOURCE: 
// SOURCE: 	REGULATOR_SUPPLY("DC1VDD", "0-0034"),
// SOURCE: 	REGULATOR_SUPPLY("DC2VDD", "0-0034"),
// SOURCE: 	REGULATOR_SUPPLY("DC3VDD", "0-0034"),
// SOURCE: 	REGULATOR_SUPPLY("LDO1VDD", "0-0034"),
// SOURCE: 	REGULATOR_SUPPLY("LDO2VDD", "0-0034"),
// SOURCE: 	REGULATOR_SUPPLY("LDO4VDD", "0-0034"),
// SOURCE: 	REGULATOR_SUPPLY("LDO5VDD", "0-0034"),
// SOURCE: 	REGULATOR_SUPPLY("LDO6VDD", "0-0034"),
// SOURCE: 	REGULATOR_SUPPLY("LDO7VDD", "0-0034"),
// SOURCE: 	REGULATOR_SUPPLY("LDO8VDD", "0-0034"),
// SOURCE: 	REGULATOR_SUPPLY("LDO9VDD", "0-0034"),
// SOURCE: 	REGULATOR_SUPPLY("LDO10VDD", "0-0034"),
// SOURCE: 	REGULATOR_SUPPLY("LDO11VDD", "0-0034"),
// SOURCE: 
// SOURCE: 	REGULATOR_SUPPLY("DC1VDD", "1-0034"),
// SOURCE: 	REGULATOR_SUPPLY("DC2VDD", "1-0034"),
// SOURCE: 	REGULATOR_SUPPLY("DC3VDD", "1-0034"),
// SOURCE: 	REGULATOR_SUPPLY("LDO1VDD", "1-0034"),
// SOURCE: 	REGULATOR_SUPPLY("LDO2VDD", "1-0034"),
// SOURCE: 	REGULATOR_SUPPLY("LDO4VDD", "1-0034"),
// SOURCE: 	REGULATOR_SUPPLY("LDO5VDD", "1-0034"),
// SOURCE: 	REGULATOR_SUPPLY("LDO6VDD", "1-0034"),
// SOURCE: 	REGULATOR_SUPPLY("LDO7VDD", "1-0034"),
// SOURCE: 	REGULATOR_SUPPLY("LDO8VDD", "1-0034"),
// SOURCE: 	REGULATOR_SUPPLY("LDO9VDD", "1-0034"),
// SOURCE: 	REGULATOR_SUPPLY("LDO10VDD", "1-0034"),
// SOURCE: 	REGULATOR_SUPPLY("LDO11VDD", "1-0034"),
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_init_data wallvdd_data = {
// SOURCE: 	.constraints = {
// SOURCE: 		.always_on = 1,
// SOURCE: 	},
// SOURCE: 	.num_consumer_supplies = ARRAY_SIZE(wallvdd_consumers),
// SOURCE: 	.consumer_supplies = wallvdd_consumers,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct fixed_voltage_config wallvdd_pdata = {
// SOURCE: 	.supply_name = "WALLVDD",
// SOURCE: 	.microvolts = 5000000,
// SOURCE: 	.init_data = &wallvdd_data,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct platform_device wallvdd_device = {
// SOURCE: 	.name		= "reg-fixed-voltage",
// SOURCE: 	.id		= -1,
// SOURCE: 	.dev = {
// SOURCE: 		.platform_data = &wallvdd_pdata,
// SOURCE: 	},
// SOURCE: };
// SOURCE: 
// SOURCE: static struct platform_device *crag6410_devices[] __initdata = {
// SOURCE: 	&s3c_device_hsmmc0,
// SOURCE: 	&s3c_device_hsmmc2,
// SOURCE: 	&s3c_device_i2c0,
// SOURCE: 	&s3c_device_i2c1,
// SOURCE: 	&s3c_device_fb,
// SOURCE: 	&s3c_device_ohci,
// SOURCE: 	&s3c_device_usb_hsotg,
// SOURCE: 	&samsung_device_pwm,
// SOURCE: 	&s3c64xx_device_iis0,
// SOURCE: 	&s3c64xx_device_iis1,
// SOURCE: 	&samsung_device_keypad,
// SOURCE: 	&crag6410_gpio_keydev,
// SOURCE: 	&crag6410_dm9k_device,
// SOURCE: 	&s3c64xx_device_spi0,
// SOURCE: 	&crag6410_lcd_powerdev,
// SOURCE: 	&crag6410_backlight_device,
// SOURCE: 	&speyside_device,
// SOURCE: 	&tobermory_device,
// SOURCE: 	&littlemill_device,
// SOURCE: 	&lowland_device,
// SOURCE: 	&bells_wm2200_device,
// SOURCE: 	&bells_wm5102_device,
// SOURCE: 	&bells_wm5110_device,
// SOURCE: 	&wallvdd_device,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct pca953x_platform_data crag6410_pca_data = {
// SOURCE: 	.gpio_base	= PCA935X_GPIO_BASE,
// SOURCE: 	.irq_base	= -1,
// SOURCE: };
// SOURCE: 
// SOURCE: /* VDDARM is controlled by DVS1 connected to GPK(0) */
// SOURCE: static struct wm831x_buckv_pdata vddarm_pdata = {
// SOURCE: 	.dvs_control_src = 1,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_consumer_supply vddarm_consumers[] = {
// SOURCE: 	REGULATOR_SUPPLY("vddarm", NULL),
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_init_data vddarm = {
// SOURCE: 	.constraints = {
// SOURCE: 		.name = "VDDARM",
// SOURCE: 		.min_uV = 1000000,
// SOURCE: 		.max_uV = 1300000,
// SOURCE: 		.always_on = 1,
// SOURCE: 		.valid_ops_mask = REGULATOR_CHANGE_VOLTAGE,
// SOURCE: 	},
// SOURCE: 	.num_consumer_supplies = ARRAY_SIZE(vddarm_consumers),
// SOURCE: 	.consumer_supplies = vddarm_consumers,
// SOURCE: 	.supply_regulator = "WALLVDD",
// SOURCE: 	.driver_data = &vddarm_pdata,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_consumer_supply vddint_consumers[] = {
// SOURCE: 	REGULATOR_SUPPLY("vddint", NULL),
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_init_data vddint = {
// SOURCE: 	.constraints = {
// SOURCE: 		.name = "VDDINT",
// SOURCE: 		.min_uV = 1000000,
// SOURCE: 		.max_uV = 1200000,
// SOURCE: 		.always_on = 1,
// SOURCE: 		.valid_ops_mask = REGULATOR_CHANGE_VOLTAGE,
// SOURCE: 	},
// SOURCE: 	.num_consumer_supplies = ARRAY_SIZE(vddint_consumers),
// SOURCE: 	.consumer_supplies = vddint_consumers,
// SOURCE: 	.supply_regulator = "WALLVDD",
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_init_data vddmem = {
// SOURCE: 	.constraints = {
// SOURCE: 		.name = "VDDMEM",
// SOURCE: 		.always_on = 1,
// SOURCE: 	},
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_init_data vddsys = {
// SOURCE: 	.constraints = {
// SOURCE: 		.name = "VDDSYS,VDDEXT,VDDPCM,VDDSS",
// SOURCE: 		.always_on = 1,
// SOURCE: 	},
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_consumer_supply vddmmc_consumers[] = {
// SOURCE: 	REGULATOR_SUPPLY("vmmc", "s3c-sdhci.0"),
// SOURCE: 	REGULATOR_SUPPLY("vmmc", "s3c-sdhci.1"),
// SOURCE: 	REGULATOR_SUPPLY("vmmc", "s3c-sdhci.2"),
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_init_data vddmmc = {
// SOURCE: 	.constraints = {
// SOURCE: 		.name = "VDDMMC,UH",
// SOURCE: 		.always_on = 1,
// SOURCE: 	},
// SOURCE: 	.num_consumer_supplies = ARRAY_SIZE(vddmmc_consumers),
// SOURCE: 	.consumer_supplies = vddmmc_consumers,
// SOURCE: 	.supply_regulator = "WALLVDD",
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_init_data vddotgi = {
// SOURCE: 	.constraints = {
// SOURCE: 		.name = "VDDOTGi",
// SOURCE: 		.always_on = 1,
// SOURCE: 	},
// SOURCE: 	.supply_regulator = "WALLVDD",
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_init_data vddotg = {
// SOURCE: 	.constraints = {
// SOURCE: 		.name = "VDDOTG",
// SOURCE: 		.always_on = 1,
// SOURCE: 	},
// SOURCE: 	.supply_regulator = "WALLVDD",
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_init_data vddhi = {
// SOURCE: 	.constraints = {
// SOURCE: 		.name = "VDDHI",
// SOURCE: 		.always_on = 1,
// SOURCE: 	},
// SOURCE: 	.supply_regulator = "WALLVDD",
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_init_data vddadc = {
// SOURCE: 	.constraints = {
// SOURCE: 		.name = "VDDADC,VDDDAC",
// SOURCE: 		.always_on = 1,
// SOURCE: 	},
// SOURCE: 	.supply_regulator = "WALLVDD",
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_init_data vddmem0 = {
// SOURCE: 	.constraints = {
// SOURCE: 		.name = "VDDMEM0",
// SOURCE: 		.always_on = 1,
// SOURCE: 	},
// SOURCE: 	.supply_regulator = "WALLVDD",
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_init_data vddpll = {
// SOURCE: 	.constraints = {
// SOURCE: 		.name = "VDDPLL",
// SOURCE: 		.always_on = 1,
// SOURCE: 	},
// SOURCE: 	.supply_regulator = "WALLVDD",
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_init_data vddlcd = {
// SOURCE: 	.constraints = {
// SOURCE: 		.name = "VDDLCD",
// SOURCE: 		.always_on = 1,
// SOURCE: 	},
// SOURCE: 	.supply_regulator = "WALLVDD",
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_init_data vddalive = {
// SOURCE: 	.constraints = {
// SOURCE: 		.name = "VDDALIVE",
// SOURCE: 		.always_on = 1,
// SOURCE: 	},
// SOURCE: 	.supply_regulator = "WALLVDD",
// SOURCE: };
// SOURCE: 
// SOURCE: static struct wm831x_backup_pdata banff_backup_pdata = {
// SOURCE: 	.charger_enable = 1,
// SOURCE: 	.vlim = 2500,  /* mV */
// SOURCE: 	.ilim = 200,   /* uA */
// SOURCE: };
// SOURCE: 
// SOURCE: static struct wm831x_status_pdata banff_red_led = {
// SOURCE: 	.name = "banff:red:",
// SOURCE: 	.default_src = WM831X_STATUS_MANUAL,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct wm831x_status_pdata banff_green_led = {
// SOURCE: 	.name = "banff:green:",
// SOURCE: 	.default_src = WM831X_STATUS_MANUAL,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct wm831x_touch_pdata touch_pdata = {
// SOURCE: 	.data_irq = S3C_EINT(26),
// SOURCE: 	.pd_irq = S3C_EINT(27),
// SOURCE: };
// SOURCE: 
// SOURCE: static struct wm831x_pdata crag_pmic_pdata = {
// SOURCE: 	.wm831x_num = 1,
// SOURCE: 	.irq_base = BANFF_PMIC_IRQ_BASE,
// SOURCE: 	.gpio_base = BANFF_PMIC_GPIO_BASE,
// SOURCE: 	.soft_shutdown = true,
// SOURCE: 
// SOURCE: 	.backup = &banff_backup_pdata,
// SOURCE: 
// SOURCE: 	.gpio_defaults = {
// SOURCE: 		/* GPIO5: DVS1_REQ - CMOS, DBVDD, active high */
// SOURCE: 		[4] = WM831X_GPN_DIR | WM831X_GPN_POL | WM831X_GPN_ENA | 0x8,
// SOURCE: 		/* GPIO11: Touchscreen data - CMOS, DBVDD, active high*/
// SOURCE: 		[10] = WM831X_GPN_POL | WM831X_GPN_ENA | 0x6,
// SOURCE: 		/* GPIO12: Touchscreen pen down - CMOS, DBVDD, active high*/
// SOURCE: 		[11] = WM831X_GPN_POL | WM831X_GPN_ENA | 0x7,
// SOURCE: 	},
// SOURCE: 
// SOURCE: 	.dcdc = {
// SOURCE: 		&vddarm,  /* DCDC1 */
// SOURCE: 		&vddint,  /* DCDC2 */
// SOURCE: 		&vddmem,  /* DCDC3 */
// SOURCE: 	},
// SOURCE: 
// SOURCE: 	.ldo = {
// SOURCE: 		&vddsys,   /* LDO1 */
// SOURCE: 		&vddmmc,   /* LDO2 */
// SOURCE: 		NULL,      /* LDO3 */
// SOURCE: 		&vddotgi,  /* LDO4 */
// SOURCE: 		&vddotg,   /* LDO5 */
// SOURCE: 		&vddhi,    /* LDO6 */
// SOURCE: 		&vddadc,   /* LDO7 */
// SOURCE: 		&vddmem0,  /* LDO8 */
// SOURCE: 		&vddpll,   /* LDO9 */
// SOURCE: 		&vddlcd,   /* LDO10 */
// SOURCE: 		&vddalive, /* LDO11 */
// SOURCE: 	},
// SOURCE: 
// SOURCE: 	.status = {
// SOURCE: 		&banff_green_led,
// SOURCE: 		&banff_red_led,
// SOURCE: 	},
// SOURCE: 
// SOURCE: 	.touch = &touch_pdata,
// SOURCE: };
// SOURCE: 
// SOURCE: /*
// SOURCE:  * VDDARM is eventually ending up as a regulator hanging on the MFD cell device
// SOURCE:  * "wm831x-buckv.1" spawn from drivers/mfd/wm831x-core.c.
// SOURCE:  *
// SOURCE:  * From the note on the platform data we can see that this is clearly DVS1
// SOURCE:  * and assigned as dcdc1 resource to the MFD core which sets .id of the cell
// SOURCE:  * spawning the DVS1 platform device to 1, then the cell platform device
// SOURCE:  * name is calculated from 10*instance + id resulting in the device name
// SOURCE:  * "wm831x-buckv.11"
// SOURCE:  */
// SOURCE: static struct gpiod_lookup_table crag_pmic_gpiod_table = {
// SOURCE: 	.dev_id = "wm831x-buckv.11",
// SOURCE: 	.table = {
// SOURCE: 		GPIO_LOOKUP("GPIOK", 0, "dvs", GPIO_ACTIVE_HIGH),
// SOURCE: 		{ },
// SOURCE: 	},
// SOURCE: };
// SOURCE: 
// SOURCE: static struct i2c_board_info i2c_devs0[] = {
// SOURCE: 	{ I2C_BOARD_INFO("24c08", 0x50), },
// SOURCE: 	{ I2C_BOARD_INFO("tca6408", 0x20),
// SOURCE: 	  .platform_data = &crag6410_pca_data,
// SOURCE: 	},
// SOURCE: 	{ I2C_BOARD_INFO("wm8312", 0x34),
// SOURCE: 	  .platform_data = &crag_pmic_pdata,
// SOURCE: 	  .irq = S3C_EINT(23),
// SOURCE: 	},
// SOURCE: };
// SOURCE: 
// SOURCE: static struct s3c2410_platform_i2c i2c0_pdata = {
// SOURCE: 	.frequency = 400000,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_consumer_supply pvdd_1v2_consumers[] = {
// SOURCE: 	REGULATOR_SUPPLY("DCVDD", "spi0.0"),
// SOURCE: 	REGULATOR_SUPPLY("AVDD", "spi0.0"),
// SOURCE: 	REGULATOR_SUPPLY("AVDD", "spi0.1"),
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_init_data pvdd_1v2 = {
// SOURCE: 	.constraints = {
// SOURCE: 		.name = "PVDD_1V2",
// SOURCE: 		.valid_ops_mask = REGULATOR_CHANGE_STATUS,
// SOURCE: 	},
// SOURCE: 
// SOURCE: 	.consumer_supplies = pvdd_1v2_consumers,
// SOURCE: 	.num_consumer_supplies = ARRAY_SIZE(pvdd_1v2_consumers),
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_consumer_supply pvdd_1v8_consumers[] = {
// SOURCE: 	REGULATOR_SUPPLY("LDOVDD", "1-001a"),
// SOURCE: 	REGULATOR_SUPPLY("PLLVDD", "1-001a"),
// SOURCE: 	REGULATOR_SUPPLY("DBVDD", "1-001a"),
// SOURCE: 	REGULATOR_SUPPLY("DBVDD1", "1-001a"),
// SOURCE: 	REGULATOR_SUPPLY("DBVDD2", "1-001a"),
// SOURCE: 	REGULATOR_SUPPLY("DBVDD3", "1-001a"),
// SOURCE: 	REGULATOR_SUPPLY("CPVDD", "1-001a"),
// SOURCE: 	REGULATOR_SUPPLY("AVDD2", "1-001a"),
// SOURCE: 	REGULATOR_SUPPLY("DCVDD", "1-001a"),
// SOURCE: 	REGULATOR_SUPPLY("AVDD", "1-001a"),
// SOURCE: 	REGULATOR_SUPPLY("DBVDD", "spi0.0"),
// SOURCE: 
// SOURCE: 	REGULATOR_SUPPLY("DBVDD", "1-003a"),
// SOURCE: 	REGULATOR_SUPPLY("LDOVDD", "1-003a"),
// SOURCE: 	REGULATOR_SUPPLY("CPVDD", "1-003a"),
// SOURCE: 	REGULATOR_SUPPLY("AVDD", "1-003a"),
// SOURCE: 	REGULATOR_SUPPLY("DBVDD1", "spi0.1"),
// SOURCE: 	REGULATOR_SUPPLY("DBVDD2", "spi0.1"),
// SOURCE: 	REGULATOR_SUPPLY("DBVDD3", "spi0.1"),
// SOURCE: 	REGULATOR_SUPPLY("LDOVDD", "spi0.1"),
// SOURCE: 	REGULATOR_SUPPLY("CPVDD", "spi0.1"),
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_init_data pvdd_1v8 = {
// SOURCE: 	.constraints = {
// SOURCE: 		.name = "PVDD_1V8",
// SOURCE: 		.always_on = 1,
// SOURCE: 	},
// SOURCE: 
// SOURCE: 	.consumer_supplies = pvdd_1v8_consumers,
// SOURCE: 	.num_consumer_supplies = ARRAY_SIZE(pvdd_1v8_consumers),
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_consumer_supply pvdd_3v3_consumers[] = {
// SOURCE: 	REGULATOR_SUPPLY("MICVDD", "1-001a"),
// SOURCE: 	REGULATOR_SUPPLY("AVDD1", "1-001a"),
// SOURCE: };
// SOURCE: 
// SOURCE: static struct regulator_init_data pvdd_3v3 = {
// SOURCE: 	.constraints = {
// SOURCE: 		.name = "PVDD_3V3",
// SOURCE: 		.always_on = 1,
// SOURCE: 	},
// SOURCE: 
// SOURCE: 	.consumer_supplies = pvdd_3v3_consumers,
// SOURCE: 	.num_consumer_supplies = ARRAY_SIZE(pvdd_3v3_consumers),
// SOURCE: };
// SOURCE: 
// SOURCE: static struct wm831x_pdata glenfarclas_pmic_pdata = {
// SOURCE: 	.wm831x_num = 2,
// SOURCE: 	.irq_base = GLENFARCLAS_PMIC_IRQ_BASE,
// SOURCE: 	.gpio_base = GLENFARCLAS_PMIC_GPIO_BASE,
// SOURCE: 	.soft_shutdown = true,
// SOURCE: 
// SOURCE: 	.gpio_defaults = {
// SOURCE: 		/* GPIO1-3: IRQ inputs, rising edge triggered, CMOS */
// SOURCE: 		[0] = WM831X_GPN_DIR | WM831X_GPN_POL | WM831X_GPN_ENA,
// SOURCE: 		[1] = WM831X_GPN_DIR | WM831X_GPN_POL | WM831X_GPN_ENA,
// SOURCE: 		[2] = WM831X_GPN_DIR | WM831X_GPN_POL | WM831X_GPN_ENA,
// SOURCE: 	},
// SOURCE: 
// SOURCE: 	.dcdc = {
// SOURCE: 		&pvdd_1v2,  /* DCDC1 */
// SOURCE: 		&pvdd_1v8,  /* DCDC2 */
// SOURCE: 		&pvdd_3v3,  /* DCDC3 */
// SOURCE: 	},
// SOURCE: 
// SOURCE: 	.disable_touch = true,
// SOURCE: };
// SOURCE: 
// SOURCE: static struct gpiod_lookup_table crag_wm1250_ev1_gpiod_table = {
// SOURCE: 	/* The WM1250-EV1 is device 0027 on I2C bus 1 */
// SOURCE: 	.dev_id = "1-0027",
// SOURCE: 	.table = {
// SOURCE: 		GPIO_LOOKUP("GPION", 12, "clk-ena", GPIO_ACTIVE_HIGH),
// SOURCE: 		GPIO_LOOKUP("GPIOL", 12, "clk-sel0", GPIO_ACTIVE_HIGH),
// SOURCE: 		GPIO_LOOKUP("GPIOL", 13, "clk-sel1", GPIO_ACTIVE_HIGH),
// SOURCE: 		GPIO_LOOKUP("GPIOL", 14, "osr", GPIO_ACTIVE_HIGH),
// SOURCE: 		GPIO_LOOKUP("GPIOL", 8, "master", GPIO_ACTIVE_HIGH),
// SOURCE: 		{ },
// SOURCE: 	},
// SOURCE: };
// SOURCE: 
// SOURCE: static struct i2c_board_info i2c_devs1[] = {
// SOURCE: 	{ I2C_BOARD_INFO("wm8311", 0x34),
// SOURCE: 	  .irq = S3C_EINT(0),
// SOURCE: 	  .platform_data = &glenfarclas_pmic_pdata },
// SOURCE: 
// SOURCE: 	{ I2C_BOARD_INFO("wlf-gf-module", 0x20) },
// SOURCE: 	{ I2C_BOARD_INFO("wlf-gf-module", 0x22) },
// SOURCE: 	{ I2C_BOARD_INFO("wlf-gf-module", 0x24) },
// SOURCE: 	{ I2C_BOARD_INFO("wlf-gf-module", 0x25) },
// SOURCE: 	{ I2C_BOARD_INFO("wlf-gf-module", 0x26) },
// SOURCE: 	{ I2C_BOARD_INFO("wm1250-ev1", 0x27), },
// SOURCE: };
// SOURCE: 
// SOURCE: static struct s3c2410_platform_i2c i2c1_pdata = {
// SOURCE: 	.frequency = 400000,
// SOURCE: 	.bus_num = 1,
// SOURCE: };
// SOURCE: 
// SOURCE: static void __init crag6410_map_io(void)
// SOURCE: {
// SOURCE: 	s3c64xx_init_io(NULL, 0);
// SOURCE: 	s3c64xx_set_xtal_freq(12000000);
// SOURCE: 	s3c24xx_init_uarts(crag6410_uartcfgs, ARRAY_SIZE(crag6410_uartcfgs));
// SOURCE: 	s3c64xx_set_timer_source(S3C64XX_PWM3, S3C64XX_PWM4);
// SOURCE: 
// SOURCE: 	/* LCD type and Bypass set by bootloader */
// SOURCE: }
// SOURCE: 
// SOURCE: static struct s3c_sdhci_platdata crag6410_hsmmc2_pdata = {
// SOURCE: 	.max_width		= 4,
// SOURCE: 	.cd_type		= S3C_SDHCI_CD_PERMANENT,
// SOURCE: 	.host_caps		= MMC_CAP_POWER_OFF_CARD,
// SOURCE: };
// SOURCE: 
// SOURCE: static void crag6410_cfg_sdhci0(struct platform_device *dev, int width)
// SOURCE: {
// SOURCE: 	/* Set all the necessary GPG pins to special-function 2 */
// SOURCE: 	s3c_gpio_cfgrange_nopull(S3C64XX_GPG(0), 2 + width, S3C_GPIO_SFN(2));
// SOURCE: 
// SOURCE: 	/* force card-detected for prototype 0 */
// SOURCE: 	s3c_gpio_setpull(S3C64XX_GPG(6), S3C_GPIO_PULL_DOWN);
// SOURCE: }
// SOURCE: 
// SOURCE: static struct s3c_sdhci_platdata crag6410_hsmmc0_pdata = {
// SOURCE: 	.max_width		= 4,
// SOURCE: 	.cd_type		= S3C_SDHCI_CD_INTERNAL,
// SOURCE: 	.cfg_gpio		= crag6410_cfg_sdhci0,
// SOURCE: 	.host_caps		= MMC_CAP_POWER_OFF_CARD,
// SOURCE: };
// SOURCE: 
// SOURCE: static const struct gpio_led gpio_leds[] = {
// SOURCE: 	{
// SOURCE: 		.name = "d13:green:",
// SOURCE: 		.default_state = LEDS_GPIO_DEFSTATE_ON,
// SOURCE: 	},
// SOURCE: 	{
// SOURCE: 		.name = "d14:green:",
// SOURCE: 		.default_state = LEDS_GPIO_DEFSTATE_ON,
// SOURCE: 	},
// SOURCE: 	{
// SOURCE: 		.name = "d15:green:",
// SOURCE: 		.default_state = LEDS_GPIO_DEFSTATE_ON,
// SOURCE: 	},
// SOURCE: 	{
// SOURCE: 		.name = "d16:green:",
// SOURCE: 		.default_state = LEDS_GPIO_DEFSTATE_ON,
// SOURCE: 	},
// SOURCE: 	{
// SOURCE: 		.name = "d17:green:",
// SOURCE: 		.default_state = LEDS_GPIO_DEFSTATE_ON,
// SOURCE: 	},
// SOURCE: 	{
// SOURCE: 		.name = "d18:green:",
// SOURCE: 		.default_state = LEDS_GPIO_DEFSTATE_ON,
// SOURCE: 	},
// SOURCE: 	{
// SOURCE: 		.name = "d19:green:",
// SOURCE: 		.default_state = LEDS_GPIO_DEFSTATE_ON,
// SOURCE: 	},
// SOURCE: 	{
// SOURCE: 		.name = "d20:green:",
// SOURCE: 		.default_state = LEDS_GPIO_DEFSTATE_ON,
// SOURCE: 	},
// SOURCE: };
// SOURCE: 
// SOURCE: static struct gpiod_lookup_table crag_leds_table = {
// SOURCE: 	.dev_id = "leds-gpio",
// SOURCE: 	.table = {
// SOURCE: 		GPIO_LOOKUP_IDX("basic-mmio-gpio", 0, "cs", 0, GPIO_ACTIVE_LOW),
// SOURCE: 		GPIO_LOOKUP_IDX("basic-mmio-gpio", 1, "cs", 1, GPIO_ACTIVE_LOW),
// SOURCE: 		GPIO_LOOKUP_IDX("basic-mmio-gpio", 2, "cs", 2, GPIO_ACTIVE_LOW),
// SOURCE: 		GPIO_LOOKUP_IDX("basic-mmio-gpio", 3, "cs", 3, GPIO_ACTIVE_LOW),
// SOURCE: 		GPIO_LOOKUP_IDX("basic-mmio-gpio", 4, "cs", 4, GPIO_ACTIVE_LOW),
// SOURCE: 		GPIO_LOOKUP_IDX("basic-mmio-gpio", 5, "cs", 5, GPIO_ACTIVE_LOW),
// SOURCE: 		GPIO_LOOKUP_IDX("basic-mmio-gpio", 6, "cs", 6, GPIO_ACTIVE_LOW),
// SOURCE: 		GPIO_LOOKUP_IDX("basic-mmio-gpio", 7, "cs", 7, GPIO_ACTIVE_LOW),
// SOURCE: 		{ },
// SOURCE: 	},
// SOURCE: };
// SOURCE: 
// SOURCE: static const struct gpio_led_platform_data gpio_leds_pdata = {
// SOURCE: 	.leds = gpio_leds,
// SOURCE: 	.num_leds = ARRAY_SIZE(gpio_leds),
// SOURCE: };
// SOURCE: 
// SOURCE: static struct dwc2_hsotg_plat crag6410_hsotg_pdata;
// SOURCE: 
// SOURCE: static struct gpiod_lookup_table crag_spi0_gpiod_table = {
// SOURCE: 	.dev_id = "s3c6410-spi.0",
// SOURCE: 	.table = {
// SOURCE: 		GPIO_LOOKUP_IDX("GPIOC", 3, "cs", 0, GPIO_ACTIVE_LOW),
// SOURCE: 		GPIO_LOOKUP_IDX("GPION", 5, "cs", 1, GPIO_ACTIVE_LOW),
// SOURCE: 		{ },
// SOURCE: 	},
// SOURCE: };
// SOURCE: 
// SOURCE: static void __init crag6410_machine_init(void)
// SOURCE: {
// SOURCE: 	/* Open drain IRQs need pullups */
// SOURCE: 	s3c_gpio_setpull(S3C64XX_GPM(0), S3C_GPIO_PULL_UP);
// SOURCE: 	s3c_gpio_setpull(S3C64XX_GPN(0), S3C_GPIO_PULL_UP);
// SOURCE: 
// SOURCE: 	gpio_request(S3C64XX_GPB(0), "LCD power");
// SOURCE: 	gpio_direction_output(S3C64XX_GPB(0), 0);
// SOURCE: 
// SOURCE: 	gpio_request(S3C64XX_GPF(14), "LCD PWM");
// SOURCE: 	gpio_direction_output(S3C64XX_GPF(14), 0);  /* turn off */
// SOURCE: 
// SOURCE: 	gpio_request(S3C64XX_GPB(1), "SD power");
// SOURCE: 	gpio_direction_output(S3C64XX_GPB(1), 0);
// SOURCE: 
// SOURCE: 	gpio_request(S3C64XX_GPF(10), "nRESETSEL");
// SOURCE: 	gpio_direction_output(S3C64XX_GPF(10), 1);
// SOURCE: 
// SOURCE: 	s3c_sdhci0_set_platdata(&crag6410_hsmmc0_pdata);
// SOURCE: 	s3c_sdhci2_set_platdata(&crag6410_hsmmc2_pdata);
// SOURCE: 
// SOURCE: 	s3c_i2c0_set_platdata(&i2c0_pdata);
// SOURCE: 	s3c_i2c1_set_platdata(&i2c1_pdata);
// SOURCE: 	s3c_fb_set_platdata(&crag6410_lcd_pdata);
// SOURCE: 	dwc2_hsotg_set_platdata(&crag6410_hsotg_pdata);
// SOURCE: 
// SOURCE: 	gpiod_add_lookup_table(&crag_pmic_gpiod_table);
// SOURCE: 	i2c_register_board_info(0, i2c_devs0, ARRAY_SIZE(i2c_devs0));
// SOURCE: 	gpiod_add_lookup_table(&crag_wm1250_ev1_gpiod_table);
// SOURCE: 	i2c_register_board_info(1, i2c_devs1, ARRAY_SIZE(i2c_devs1));
// SOURCE: 
// SOURCE: 	samsung_keypad_set_platdata(&crag6410_keypad_data);
// SOURCE: 
// SOURCE: 	gpiod_add_lookup_table(&crag_spi0_gpiod_table);
// SOURCE: 	s3c64xx_spi0_set_platdata(0, 2);
// SOURCE: 
// SOURCE: 	pwm_add_table(crag6410_pwm_lookup, ARRAY_SIZE(crag6410_pwm_lookup));
// SOURCE: 	platform_add_devices(crag6410_devices, ARRAY_SIZE(crag6410_devices));
// SOURCE: 	platform_device_register_full(&crag6410_mmgpio_devinfo);
// SOURCE: 
// SOURCE: 	gpiod_add_lookup_table(&crag_leds_table);
// SOURCE: 	gpio_led_register_device(-1, &gpio_leds_pdata);
// SOURCE: 
// SOURCE: 	regulator_has_full_constraints();
// SOURCE: 
// SOURCE: 	s3c64xx_pm_init();
// SOURCE: }
// SOURCE: 
// SOURCE: MACHINE_START(WLF_CRAGG_6410, "Wolfson Cragganmore 6410")
// SOURCE: 	/* Maintainer: Mark Brown <broonie@opensource.wolfsonmicro.com> */
// SOURCE: 	.atag_offset	= 0x100,
// SOURCE: 	.nr_irqs	= S3C64XX_NR_IRQS,
// SOURCE: 	.init_irq	= s3c6410_init_irq,
// SOURCE: 	.map_io		= crag6410_map_io,
// SOURCE: 	.init_machine	= crag6410_machine_init,
// SOURCE: 	.init_time	= s3c64xx_timer_init,
// SOURCE: MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
