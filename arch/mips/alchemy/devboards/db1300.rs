// SPDX-License-Identifier: GPL-2.0
/*
 * DBAu1300 init and platform device setup.
 *
 * (c) 2009 Manuel Lauss <manuel.lauss@googlemail.com>
 */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Kernel headers and symbols are supplied by the surrounding kernel bindings.
extern "C" {
    fn au1300_pinfunc_to_dev(pin: c_int);
    fn au1300_gpio_direction_input(pin: c_int);
    fn au1300_set_dbdma_gpio(n: c_int, pin: c_int);
    fn alchemy_rdsmem(addr: c_ulong) -> c_uint;
    fn __raw_writeb(value: c_int, addr: *mut c_void);
    fn __raw_writel(value: c_uint, addr: *mut c_void);
    fn wmb();
    fn bcsr_read(reg: c_int) -> c_uint;
    fn bcsr_mod(reg: c_int, set: c_uint, clear: c_uint);
    fn bcsr_init(base: c_ulong, hexled: c_ulong);
    fn bcsr_init_irq(first: c_int, last: c_int, irq: c_int);
    fn au1300_gpio_to_irq(pin: c_int) -> c_int;
    fn irq_set_irq_type(irq: c_int, kind: c_int);
    fn irq_set_status_flags(irq: c_int, flags: c_uint);
    fn i2c_register_board_info(bus: c_int, info: *mut c_void, count: usize) -> c_int;
    fn platform_driver_register(driver: *mut c_void) -> c_int;
    fn platform_add_devices(devs: *mut *mut c_void, count: usize) -> c_int;
    fn db1x_register_pcmcia_socket(a: c_ulong, b: c_ulong, c: c_ulong, d: c_ulong,
        e: c_ulong, f: c_ulong, irq: c_int, ins: c_int, x: c_int, eject: c_int, y: c_int);
    fn db1x_register_norflash(size: c_ulong, width: c_int, swapped: c_int);
    fn prom_get_ethernet_addr(addr: *mut u8);
    fn printk(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn clk_get(dev: *mut c_void, name: *const c_char) -> *mut c_void;
    fn clk_set_rate(clk: *mut c_void, rate: c_ulong) -> c_int;
    fn clk_prepare_enable(clk: *mut c_void) -> c_int;
    fn clk_put(clk: *mut c_void);
    fn platform_device_register_full(info: *const c_void) -> *mut c_void;
    fn software_node_register_node_group(nodes: *const *const c_void) -> c_int;
    fn PTR_ERR_OR_ZERO(ptr: *mut c_void) -> c_int;
    fn mmc_detect_change(host: *mut c_void, delay: c_ulong);
    fn msecs_to_jiffies(ms: c_uint) -> c_ulong;
    fn msleep(ms: c_uint);
    fn disable_irq_nosync(irq: c_int);
    fn enable_irq(irq: c_int);
    fn request_threaded_irq(irq: c_int, top: *const c_void, thread: *const c_void,
        flags: c_uint, name: *const c_char, data: *mut c_void) -> c_int;
    fn free_irq(irq: c_int, data: *mut c_void);
    fn wm97xx_config_gpio(wm: *mut c_void, gpio: c_int, dir: c_int, pol: c_int, sticky: c_int, wake: c_int);
    fn wm97xx_register_mach_ops(wm: *mut c_void, ops: *mut c_void) -> c_int;
    fn platform_get_drvdata(pdev: *mut c_void) -> *mut wm97xx;
    fn alchemy_uart_enable(addr: c_ulong);
}

#[repr(C)] pub struct wm97xx { pub pen_irq: c_int }
#[repr(C)] pub struct opaque { _private: [u8; 0] }
type irqreturn_t = c_int;
type led_brightness = c_int;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQ_WAKE_THREAD: irqreturn_t = 2;
const LED_OFF: led_brightness = 0;

const DB1300_FIRST_INT: c_int = ALCHEMY_GPIC_INT_LAST + 1;
const DB1300_IDE_INT: c_int = DB1300_FIRST_INT + 0;
const DB1300_ETH_INT: c_int = DB1300_FIRST_INT + 1;
const DB1300_CF_INT: c_int = DB1300_FIRST_INT + 2;
const DB1300_VIDEO_INT: c_int = DB1300_FIRST_INT + 4;
const DB1300_HDMI_INT: c_int = DB1300_FIRST_INT + 5;
const DB1300_DC_INT: c_int = DB1300_FIRST_INT + 6;
const DB1300_FLASH_INT: c_int = DB1300_FIRST_INT + 7;
const DB1300_CF_INSERT_INT: c_int = DB1300_FIRST_INT + 8;
const DB1300_CF_EJECT_INT: c_int = DB1300_FIRST_INT + 9;
const DB1300_AC97_INT: c_int = DB1300_FIRST_INT + 10;
const DB1300_AC97_PEN_INT: c_int = DB1300_FIRST_INT + 11;
const DB1300_SD1_INSERT_INT: c_int = DB1300_FIRST_INT + 12;
const DB1300_SD1_EJECT_INT: c_int = DB1300_FIRST_INT + 13;
const DB1300_OTG_VBUS_OC_INT: c_int = DB1300_FIRST_INT + 14;
const DB1300_HOST_VBUS_OC_INT: c_int = DB1300_FIRST_INT + 15;
const DB1300_LAST_INT: c_int = DB1300_FIRST_INT + 15;
const DB1300_ETH_PHYS_ADDR: c_ulong = 0x19000000;
const DB1300_ETH_PHYS_END: c_ulong = 0x197fffff;
const DB1300_IDE_PHYS_ADDR: c_ulong = 0x18800000;
const DB1300_IDE_REG_SHIFT: c_int = 5;
const DB1300_IDE_PHYS_LEN: c_ulong = 16 << DB1300_IDE_REG_SHIFT;
const DB1300_NAND_PHYS_ADDR: c_ulong = 0x20000000;
const DB1300_NAND_PHYS_END: c_ulong = 0x20000fff;

static mut db1300_i2c_devs: [opaque; 2] = [opaque { _private: [] }; 2];
static mut db1300_gpio_pins: [c_int; 8] = [AU1300_PIN_LCDPWM0, AU1300_PIN_PSC2SYNC1, AU1300_PIN_WAKE1, AU1300_PIN_WAKE2, AU1300_PIN_WAKE3, AU1300_PIN_FG3AUX, AU1300_PIN_EXTCLK1, -1];
static mut db1300_dev_pins: [c_int; 64] = [
    AU1300_PIN_WAKE0, AU1300_PIN_EXTCLK0, AU1300_PIN_SD0DAT4, AU1300_PIN_SD0DAT5,
    AU1300_PIN_SD0DAT6, AU1300_PIN_SD0DAT7, AU1300_PIN_U1RI, AU1300_PIN_U1DCD,
    AU1300_PIN_U1DSR, AU1300_PIN_U1CTS, AU1300_PIN_U1RTS, AU1300_PIN_U1DTR,
    AU1300_PIN_U1RX, AU1300_PIN_U1TX, AU1300_PIN_U0RI, AU1300_PIN_U0DCD,
    AU1300_PIN_U0DSR, AU1300_PIN_U0CTS, AU1300_PIN_U0RTS, AU1300_PIN_U0DTR,
    AU1300_PIN_U2RX, AU1300_PIN_U2TX, AU1300_PIN_U3RX, AU1300_PIN_U3TX,
    AU1300_PIN_LCDPWM1, AU1300_PIN_LCDCLKIN, AU1300_PIN_SD1DAT0, AU1300_PIN_SD1DAT1,
    AU1300_PIN_SD1DAT2, AU1300_PIN_SD1DAT3, AU1300_PIN_SD1CMD, AU1300_PIN_SD1CLK,
    AU1300_PIN_SD2DAT0, AU1300_PIN_SD2DAT1, AU1300_PIN_SD2DAT2, AU1300_PIN_SD2DAT3,
    AU1300_PIN_SD2CMD, AU1300_PIN_SD2CLK, AU1300_PIN_PSC0CLK, AU1300_PIN_PSC1CLK,
    AU1300_PIN_PSC0SYNC0, AU1300_PIN_PSC0SYNC1, AU1300_PIN_PSC0D0, AU1300_PIN_PSC0D1,
    AU1300_PIN_PSC1SYNC0, AU1300_PIN_PSC1SYNC1, AU1300_PIN_PSC1D0, AU1300_PIN_PSC1D1,
    AU1300_PIN_PSC2SYNC0, AU1300_PIN_PSC2D0, AU1300_PIN_PSC2D1, AU1300_PIN_PSC3SYNC0,
    AU1300_PIN_PSC3SYNC1, AU1300_PIN_PSC3D0, AU1300_PIN_PSC3D1, AU1300_PIN_PCE2,
    AU1300_PIN_PCE1, AU1300_PIN_PIOS16, AU1300_PIN_PIOR, AU1300_PIN_PWE,
    AU1300_PIN_PWAIT, AU1300_PIN_PREG, AU1300_PIN_POE, AU1300_PIN_PIOW,
];

unsafe fn db1300_gpio_config() {
    let mut i = 0; while db1300_dev_pins[i] != -1 { au1300_pinfunc_to_dev(db1300_dev_pins[i]); i += 1; }
    i = 0; while db1300_gpio_pins[i] != -1 { au1300_gpio_direction_input(db1300_gpio_pins[i]); i += 1; }
    au1300_set_dbdma_gpio(1, AU1300_PIN_FG3AUX);
}

static mut au1300_all_dmamask: u64 = 0xffff_ffff;
#[no_mangle] pub unsafe extern "C" fn au1300_nand_cmd_ctrl(this: *mut opaque, cmd: c_int, ctrl: c_uint) {
    // The nand_chip legacy IO_ADDR fields and NAND control constants are supplied by bindings.
    let _ = (this, cmd, ctrl);
}
#[no_mangle] pub unsafe extern "C" fn au1300_nand_device_ready(_this: *mut opaque) -> c_int { (alchemy_rdsmem(AU1000_MEM_STSTAT) & 1) as c_int }

// The remaining platform-data objects retain their C layout through the kernel's generated bindings.
// Their initializers are represented as opaque storage because the declarations come from external headers.
static mut db1300_nand_parts: [opaque; 2] = [opaque { _private: [] }; 2];
#[no_mangle] pub static mut db1300_nand_platdata: opaque = opaque { _private: [] };
#[no_mangle] pub static mut db1300_sd0_platdata: opaque = opaque { _private: [] };
#[no_mangle] pub static mut db1300_sd1_platdata: opaque = opaque { _private: [] };

unsafe fn db1300_mmc_cd(irq: c_int, _ptr: *mut c_void) -> irqreturn_t { disable_irq_nosync(irq); IRQ_WAKE_THREAD }
unsafe fn db1300_mmc_cdfn(irq: c_int, ptr: *mut c_void) -> irqreturn_t { mmc_detect_change(ptr, msecs_to_jiffies(200)); msleep(100); if irq == DB1300_SD1_INSERT_INT { enable_irq(DB1300_SD1_EJECT_INT) } else { enable_irq(DB1300_SD1_INSERT_INT) }; IRQ_HANDLED }
unsafe fn db1300_mmc_card_readonly(_host: *mut c_void) -> c_int { (bcsr_read(BCSR_STATUS) & BCSR_STATUS_SD0WP) as c_int }
unsafe fn db1300_mmc_card_inserted(_host: *mut c_void) -> c_int { ((bcsr_read(BCSR_SIGSTAT) & (1 << 12)) != 0) as c_int }
unsafe fn db1300_movinand_inserted(_host: *mut c_void) -> c_int { 0 }
unsafe fn db1300_movinand_readonly(_host: *mut c_void) -> c_int { 0 }

unsafe fn db1300fb_panel_index() -> c_int { 9 }
unsafe fn db1300fb_panel_init() -> c_int { bcsr_mod(BCSR_BOARD, BCSR_BOARD_LCDVEE | BCSR_BOARD_LCDVDD, BCSR_BOARD_LCDBL); 0 }
unsafe fn db1300fb_panel_shutdown() -> c_int { bcsr_mod(BCSR_BOARD, BCSR_BOARD_LCDBL, BCSR_BOARD_LCDVEE | BCSR_BOARD_LCDVDD); 0 }

#[no_mangle] pub unsafe extern "C" fn db1300_dev_setup() -> c_int {
    let cpldirq = au1300_gpio_to_irq(AU1300_PIN_EXTCLK1); irq_set_irq_type(cpldirq, IRQ_TYPE_LEVEL_HIGH); bcsr_init_irq(DB1300_FIRST_INT, DB1300_LAST_INT, cpldirq);
    irq_set_status_flags(DB1300_SD1_INSERT_INT, IRQ_NOAUTOEN); irq_set_status_flags(DB1300_SD1_EJECT_INT, IRQ_NOAUTOEN);
    irq_set_status_flags(DB1300_CF_INSERT_INT, IRQ_NOAUTOEN); irq_set_status_flags(DB1300_CF_EJECT_INT, IRQ_NOAUTOEN);
    i2c_register_board_info(0, db1300_i2c_devs.as_mut_ptr() as *mut c_void, 2);
    __raw_writel(PSC_SEL_CLK_SERCLK, (KSEG1ADDR(AU1300_PSC1_PHYS_ADDR) + PSC_SEL_OFFSET) as *mut c_void); wmb();
    __raw_writel(PSC_SEL_CLK_SERCLK, (KSEG1ADDR(AU1300_PSC2_PHYS_ADDR) + PSC_SEL_OFFSET) as *mut c_void); wmb();
    bcsr_mod(BCSR_RESETS, 0, BCSR_RESETS_USBHPWR | BCSR_RESETS_OTGPWR);
    db1x_register_norflash(64 << 20, 2, (bcsr_read(BCSR_STATUS) & BCSR_STATUS_DB1200_SWAPBOOT) as c_int);
    0
}

#[no_mangle] pub unsafe extern "C" fn db1300_board_setup() -> c_int {
    bcsr_init(DB1300_BCSR_PHYS_ADDR, DB1300_BCSR_PHYS_ADDR + DB1300_BCSR_HEXLED_OFS);
    let whoami = bcsr_read(BCSR_WHOAMI) as u16;
    if BCSR_WHOAMI_BOARD(whoami) != BCSR_WHOAMI_DB1300 { return -ENODEV; }
    db1300_gpio_config();
    alchemy_uart_enable(AU1300_UART0_PHYS_ADDR); alchemy_uart_enable(AU1300_UART1_PHYS_ADDR); alchemy_uart_enable(AU1300_UART3_PHYS_ADDR); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
