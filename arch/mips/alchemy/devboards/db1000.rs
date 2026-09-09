// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * DBAu1000/1500/1100 PBAu1100/1500 board support
 *
 * Copyright 2000, 2008 MontaVista Software Inc.
 * Author: MontaVista Software, Inc. <source@mvista.com>
 */

// Translated from the Linux kernel C implementation.  Kernel types, constants,
// and functions supplied by the included headers are external dependencies.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct resource { pub start: c_ulong, pub end: c_ulong, pub flags: c_ulong }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device_info { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct led_classdev { _private: [u8; 0] }
#[repr(C)] pub struct property_entry { _private: [u8; 0] }
#[repr(C)] pub struct software_node { _private: [u8; 0] }
#[repr(C)] pub struct spi_board_info { _private: [u8; 0] }
#[repr(C)] pub struct au1xmmc_platform_data { _private: [u8; 0] }

type u8_ = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type irqreturn_t = c_int;
type led_brightness = c_int;

extern "C" {
    fn bcsr_init(addr: c_ulong, led_addr: c_ulong);
    fn bcsr_read(reg: c_int) -> c_ulong;
    fn bcsr_mod(reg: c_int, clear: c_int, set: c_int);
    fn get_system_type() -> *const c_char;
    fn pr_info(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn platform_device_register(dev: *mut platform_device) -> c_int;
    fn mmc_detect_change(host: *mut c_void, delay: c_ulong);
    fn msecs_to_jiffies(ms: c_ulong) -> c_ulong;
    fn irq_set_irq_type(irq: c_int, kind: c_int) -> c_int;
    fn request_irq(irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
                   flags: c_ulong, name: *const c_char, dev: *mut c_void) -> c_int;
    fn free_irq(irq: c_int, dev: *mut c_void);
    fn alchemy_gpio_get_value(gpio: c_int) -> c_int;
    fn msleep(ms: c_ulong);
    fn gpio_request(gpio: c_int, label: *const c_char) -> c_int;
    fn gpio_direction_input(gpio: c_int) -> c_int;
    fn alchemy_rdsys(reg: c_ulong) -> c_ulong;
    fn alchemy_wrsys(value: c_ulong, reg: c_ulong);
    fn spi_register_board_info(info: *mut spi_board_info, count: usize) -> c_int;
    fn clk_get(dev: *mut c_void, name: *const c_char) -> *mut clk;
    fn clk_set_parent(child: *mut clk, parent: *mut clk) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn clk_put(clk: *mut clk);
    fn platform_add_devices(devs: *mut *mut platform_device, count: usize) -> c_int;
    fn platform_device_register_full(info: *const platform_device_info) -> *mut platform_device;
    fn ptr_err_or_zero(ptr: *mut platform_device) -> c_int;
    fn db1x_register_pcmcia_socket(attr_start: c_ulong, attr_end: c_ulong,
        mem_start: c_ulong, mem_end: c_ulong, io_start: c_ulong, io_end: c_ulong,
        c: c_int, d: c_int, s: c_int, gpio: c_int, socket: c_int) -> c_int;
    fn db1x_register_norflash(size: c_ulong, width: c_int, swapped: c_int) -> c_int;
}

// External constants/macros from the kernel headers.
extern "C" {
    static alchemy_gpio2_node: software_node;
}

const F_SWAPPED: c_int = 0; // bcsr_read(BCSR_STATUS) & BCSR_STATUS_DB1000_SWAPBOOT

pub unsafe extern "C" fn db1000_board_setup() -> c_int {
    bcsr_init(DB1000_BCSR_PHYS_ADDR, DB1000_BCSR_PHYS_ADDR + DB1000_BCSR_HEXLED_OFS);
    match BCSR_WHOAMI_BOARD(bcsr_read(BCSR_WHOAMI)) {
        BCSR_WHOAMI_DB1000 | BCSR_WHOAMI_DB1500 | BCSR_WHOAMI_DB1100 |
        BCSR_WHOAMI_PB1500 | BCSR_WHOAMI_PB1500R2 | BCSR_WHOAMI_PB1100 => {
            pr_info(b"AMD Alchemy %s Board\n\0".as_ptr() as *const c_char, get_system_type());
            0
        }
        _ => -ENODEV,
    }
}

unsafe extern "C" fn db1500_map_pci_irq(_d: *const pci_dev, slot: u8_, pin: u8_) -> c_int {
    if slot < 12 || slot > 13 || pin == 0 { return -1; }
    if slot == 12 { return if pin == 1 { AU1500_PCI_INTA } else { 0xff }; }
    match pin { 1 => AU1500_PCI_INTA, 2 => AU1500_PCI_INTB, 3 => AU1500_PCI_INTC,
        4 => AU1500_PCI_INTD, _ => -1 }
}

static mut au1xxx_all_dmamask: u64 = 0xffff_ffff;

#[repr(C)] struct alchemy_pci_platdata { board_map_irq: Option<unsafe extern "C" fn(*const pci_dev, u8_, u8_) -> c_int> }

static mut db1500_pci_pd: alchemy_pci_platdata = alchemy_pci_platdata { board_map_irq: Some(db1500_map_pci_irq) };

// Resource and platform-device layouts are supplied by the kernel dependency.
// The following declarations preserve the original objects and their linkage.
extern "C" {
    static mut alchemy_pci_host_res: [resource; 1];
    static mut db1500_pci_host_dev: platform_device;
    static mut au1100_lcd_resources: [resource; 2];
    static mut au1100_lcd_device: platform_device;
    static mut alchemy_ac97c_res: [resource; 3];
    static mut alchemy_ac97c_dev: platform_device;
    static mut alchemy_ac97c_dma_dev: platform_device;
    static mut db1x00_codec_dev: platform_device;
    static mut db1x00_audio_dev: platform_device;
}

pub unsafe extern "C" fn db1500_pci_setup() -> c_int { platform_device_register(&mut db1500_pci_host_dev) }

// CONFIG_MMC_AU1X conditional section retained as a Rust conditional intent.
#[cfg(feature = "CONFIG_MMC_AU1X")]
mod mmc {
    use super::*;
    unsafe extern "C" fn db1100_mmc_cd(_irq: c_int, ptr: *mut c_void) -> irqreturn_t {
        mmc_detect_change(ptr, msecs_to_jiffies(500)); IRQ_HANDLED
    }
    unsafe extern "C" fn db1100_mmc_cd_setup(host: *mut c_void, en: c_int) -> c_int {
        let irq = if BCSR_WHOAMI_BOARD(bcsr_read(BCSR_WHOAMI)) == BCSR_WHOAMI_DB1100 { AU1100_GPIO19_INT } else { AU1100_GPIO14_INT };
        if en != 0 { irq_set_irq_type(irq, IRQ_TYPE_EDGE_BOTH); request_irq(irq, db1100_mmc_cd, 0, b"sd0_cd\0".as_ptr() as *const c_char, host) } else { free_irq(irq, host); 0 }
    }
    unsafe extern "C" fn db1100_mmc1_cd_setup(host: *mut c_void, en: c_int) -> c_int {
        let irq = if BCSR_WHOAMI_BOARD(bcsr_read(BCSR_WHOAMI)) == BCSR_WHOAMI_DB1100 { AU1100_GPIO20_INT } else { AU1100_GPIO15_INT };
        if en != 0 { irq_set_irq_type(irq, IRQ_TYPE_EDGE_BOTH); request_irq(irq, db1100_mmc_cd, 0, b"sd1_cd\0".as_ptr() as *const c_char, host) } else { free_irq(irq, host); 0 }
    }
    unsafe extern "C" fn db1100_mmc_card_readonly(_host: *mut c_void) -> c_int { if bcsr_read(BCSR_STATUS) & BCSR_STATUS_SD0WP != 0 { 0 } else { 1 } }
    unsafe extern "C" fn db1100_mmc_card_inserted(_host: *mut c_void) -> c_int { (alchemy_gpio_get_value(19) == 0) as c_int }
    unsafe extern "C" fn db1100_mmc_set_power(_host: *mut c_void, state: c_int) { let bit = if BCSR_WHOAMI_BOARD(bcsr_read(BCSR_WHOAMI)) == BCSR_WHOAMI_DB1100 { BCSR_BOARD_SD0PWR } else { BCSR_BOARD_PB1100_SD0PWR }; if state != 0 { bcsr_mod(BCSR_BOARD, 0, bit); msleep(400); } else { bcsr_mod(BCSR_BOARD, bit, 0); } }
    unsafe extern "C" fn db1100_mmcled_set(_led: *mut led_classdev, b: led_brightness) { if b != LED_OFF { bcsr_mod(BCSR_LEDS, BCSR_LEDS_LED0, 0); } else { bcsr_mod(BCSR_LEDS, 0, BCSR_LEDS_LED0); } }
    unsafe extern "C" fn db1100_mmc1_card_readonly(_host: *mut c_void) -> c_int { if bcsr_read(BCSR_BOARD) & BCSR_BOARD_SD1WP != 0 { 1 } else { 0 } }
    unsafe extern "C" fn db1100_mmc1_card_inserted(_host: *mut c_void) -> c_int { (alchemy_gpio_get_value(20) == 0) as c_int }
    unsafe extern "C" fn db1100_mmc1_set_power(_host: *mut c_void, state: c_int) { let bit = if BCSR_WHOAMI_BOARD(bcsr_read(BCSR_WHOAMI)) == BCSR_WHOAMI_DB1100 { BCSR_BOARD_SD1PWR } else { BCSR_BOARD_PB1100_SD1PWR }; if state != 0 { bcsr_mod(BCSR_BOARD, 0, bit); msleep(400); } else { bcsr_mod(BCSR_BOARD, bit, 0); } }
    unsafe extern "C" fn db1100_mmc1led_set(_led: *mut led_classdev, b: led_brightness) { if b != LED_OFF { bcsr_mod(BCSR_LEDS, BCSR_LEDS_LED1, 0); } else { bcsr_mod(BCSR_LEDS, 0, BCSR_LEDS_LED1); } }
}

// Property, SPI, resource, and platform-device objects are represented by the
// corresponding external kernel layouts; their initializers remain dependency-defined.
extern "C" {
    static db1100_ads7846_props: [property_entry; 3];
    static db1100_ads7846_swnode: software_node;
    static mut db1100_spi_info: [spi_board_info; 1];
    static db1100_spi_dev_properties: [property_entry; 5];
    static db1100_spi_dev_info: platform_device_info;
    static mut db1x00_devs: [*mut platform_device; 4];
    static mut db1100_devs: [*mut platform_device; 3];
}

pub unsafe extern "C" fn db1000_dev_setup() -> c_int {
    let board = BCSR_WHOAMI_BOARD(bcsr_read(BCSR_WHOAMI));
    let (c0, c1, d0, d1, s0, s1, twosocks, flashsize): (c_int,c_int,c_int,c_int,c_int,c_int,c_int,c_ulong);
    let mut err: c_int;
    if board == BCSR_WHOAMI_DB1500 { c0=AU1500_GPIO2_INT; c1=AU1500_GPIO5_INT; d0=0; d1=3; s0=AU1500_GPIO1_INT; s1=AU1500_GPIO4_INT; twosocks=1; flashsize=32; }
    else if board == BCSR_WHOAMI_DB1100 { c0=AU1100_GPIO2_INT; c1=AU1100_GPIO5_INT; d0=0; d1=3; s0=AU1100_GPIO1_INT; s1=AU1100_GPIO4_INT;
        gpio_request(19,b"sd0_cd\0".as_ptr() as *const c_char); gpio_request(20,b"sd1_cd\0".as_ptr() as *const c_char); gpio_direction_input(19); gpio_direction_input(20);
        let mut pfc=alchemy_rdsys(AU1000_SYS_PINFUNC); pfc |= 1; alchemy_wrsys(pfc,AU1000_SYS_PINFUNC);
        spi_register_board_info(db1100_spi_info.as_mut_ptr(),1);
        let p=clk_get(core::ptr::null_mut(),b"auxpll_clk\0".as_ptr() as *const c_char); let c=clk_get(core::ptr::null_mut(),b"lcd_intclk\0".as_ptr() as *const c_char);
        if !c.is_null() && !p.is_null() { clk_set_parent(c,p); clk_set_rate(c,clk_get_rate(p)); } if !c.is_null(){clk_put(c);} if !p.is_null(){clk_put(p);}
        platform_add_devices(db1100_devs.as_mut_ptr(),3); err=ptr_err_or_zero(platform_device_register_full(&db1100_spi_dev_info)); if err!=0 { pr_err(b"failed to register SPI controller: %d\n\0".as_ptr() as *const c_char,err); }
        twosocks=1; flashsize=32;
    } else if board == BCSR_WHOAMI_DB1000 { c0=AU1000_GPIO2_INT;c1=AU1000_GPIO5_INT;d0=0;d1=3;s0=AU1000_GPIO1_INT;s1=AU1000_GPIO4_INT;twosocks=1;flashsize=32; }
    else if board == BCSR_WHOAMI_PB1500 || board == BCSR_WHOAMI_PB1500R2 { c0=AU1500_GPIO203_INT;c1=0;d0=1;d1=0;s0=AU1500_GPIO202_INT;s1=0;twosocks=0;flashsize=64;irq_set_irq_type(AU1500_GPIO204_INT,IRQ_TYPE_LEVEL_LOW);irq_set_irq_type(AU1500_GPIO205_INT,IRQ_TYPE_LEVEL_LOW); }
    else if board == BCSR_WHOAMI_PB1100 { c0=AU1100_GPIO11_INT;c1=0;d0=9;d1=0;s0=AU1100_GPIO10_INT;s1=0;twosocks=0;flashsize=64;irq_set_irq_type(AU1100_GPIO8_INT,IRQ_TYPE_LEVEL_LOW);irq_set_irq_type(AU1100_GPIO12_INT,IRQ_TYPE_LEVEL_LOW);irq_set_irq_type(AU1100_GPIO13_INT,IRQ_TYPE_LEVEL_LOW);platform_add_devices(db1100_devs.as_mut_ptr(),3); }
    else { return 0; }
    irq_set_irq_type(c0,IRQ_TYPE_LEVEL_LOW); irq_set_irq_type(s0,IRQ_TYPE_LEVEL_LOW);
    db1x_register_pcmcia_socket(AU1000_PCMCIA_ATTR_PHYS_ADDR,AU1000_PCMCIA_ATTR_PHYS_ADDR+0x400000-1,AU1000_PCMCIA_MEM_PHYS_ADDR,AU1000_PCMCIA_MEM_PHYS_ADDR+0x400000-1,AU1000_PCMCIA_IO_PHYS_ADDR,AU1000_PCMCIA_IO_PHYS_ADDR+0x10000-1,c0,d0,0,0,0);
    if twosocks!=0 { irq_set_irq_type(c1,IRQ_TYPE_LEVEL_LOW);irq_set_irq_type(s1,IRQ_TYPE_LEVEL_LOW);db1x_register_pcmcia_socket(AU1000_PCMCIA_ATTR_PHYS_ADDR+0x4000000,AU1000_PCMCIA_ATTR_PHYS_ADDR+0x4400000-1,AU1000_PCMCIA_MEM_PHYS_ADDR+0x4000000,AU1000_PCMCIA_MEM_PHYS_ADDR+0x4400000-1,AU1000_PCMCIA_IO_PHYS_ADDR+0x4000000,AU1000_PCMCIA_IO_PHYS_ADDR+0x4010000-1,c1,d1,0,0,1); }
    platform_add_devices(db1x00_devs.as_mut_ptr(),4); db1x_register_norflash(flashsize<<20,4,F_SWAPPED); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
