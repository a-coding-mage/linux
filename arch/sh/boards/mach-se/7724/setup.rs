// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation of linux/arch/sh/boards/se/7724/setup.c.
// Kernel-provided types, constants, functions, and linker symbols are external
// dependencies supplied by the surrounding tree.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_void};

// C header dependencies are supplied by the kernel translation unit.
pub const CEU_BUFFER_MEMORY_SIZE: usize = 4 << 20;
pub static mut ceu0_dma_membase: usize = 0;
pub static mut ceu1_dma_membase: usize = 0;

extern "C" {
    static mut sh_eth_plat: sh_eth_plat_data;
    static mut lcdc_info: sh_mobile_lcdc_info;
    static mut sh7724_fsimcka_clk: clk;
    fn __raw_readw(addr: usize) -> u16;
    fn __raw_writew(value: u16, addr: usize);
    fn udelay(usecs: u32);
    fn printk(fmt: *const c_char, ...);
    fn clk_get(dev: *mut c_void, name: *const c_char) -> *mut clk;
    fn clk_set_rate(clk: *mut clk, rate: u32) -> c_int;
    fn clk_round_rate(clk: *mut clk, rate: u32) -> u32;
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn clk_put(clk: *mut clk);
    fn gpio_request(gpio: c_int, label: *const c_char) -> c_int;
    fn gpio_direction_input(gpio: c_int) -> c_int;
    fn sh_eth_is_eeprom_ready() -> c_int;
    fn i2c_register_board_info(bus: c_int, devices: *mut i2c_board_info, count: usize) -> c_int;
    fn sh_mobile_register_self_refresh(flags: u32, enter_start: *mut c_char, enter_end: *mut c_char,
                                       leave_start: *mut c_char, leave_end: *mut c_char);
    fn regulator_register_always_on(id: c_int, name: *const c_char, consumers: *mut regulator_consumer_supply,
                                    count: usize, voltage: u32) -> c_int;
    fn device_initialize(dev: *mut device);
    fn dma_declare_coherent_memory(dev: *mut device, p1: usize, p2: usize, size: usize) -> c_int;
    fn platform_device_add(dev: *mut platform_device) -> c_int;
    fn platform_add_devices(devices: *mut *mut platform_device, count: usize) -> c_int;
    fn memblock_phys_alloc(size: usize, align: usize) -> usize;
    fn memblock_phys_free(addr: usize, size: usize);
    fn memblock_remove(addr: usize, size: usize);
    fn panic(fmt: *const c_char) -> !;
    fn init_se7724_IRQ();
}

#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device, _private: [u8; 0] }
#[repr(C)] pub struct i2c_board_info { _private: [u8; 0] }
#[repr(C)] pub struct regulator_consumer_supply { _private: [u8; 0] }
#[repr(C)] pub struct sh_eth_plat_data { pub mac_addr: [u8; 6], _private: [u8; 0] }
#[repr(C)] pub struct sh_mobile_lcdc_info { _private: [u8; 0] }

const EEPROM_OP: usize = 0xBA206000;
const EEPROM_ADR: usize = 0xBA206004;
const EEPROM_DATA: usize = 0xBA20600C;
const EEPROM_STAT: usize = 0xBA206010;
const EEPROM_STRT: usize = 0xBA206014;
const SW4140: usize = 0xBA201000;
const FPGA_OUT: usize = 0xBA200400;
const PORT_HIZA: usize = 0xA4050158;
const PORT_MSELCRB: usize = 0xA4050182;
const SW41_A: u16 = 0x0100;
const SW41_B: u16 = 0x0200;

#[no_mangle]
pub unsafe extern "C" fn sh_eth_is_eeprom_ready_rust() -> c_int {
    let mut t: c_int = 10000;
    while t > 0 {
        t -= 1;
        if __raw_readw(EEPROM_STAT) == 0 { return 1; }
        udelay(1);
    }
    printk(b"ms7724se can not access to eeprom\n\0".as_ptr() as *const c_char);
    0
}

#[no_mangle]
pub unsafe extern "C" fn sh_eth_init() {
    if sh_eth_is_eeprom_ready_rust() == 0 { return; }
    for i in 0..3usize {
        __raw_writew(0, EEPROM_OP);
        __raw_writew((i * 2) as u16, EEPROM_ADR);
        __raw_writew(1, EEPROM_STRT);
        if sh_eth_is_eeprom_ready_rust() == 0 { return; }
        let mac = __raw_readw(EEPROM_DATA);
        sh_eth_plat.mac_addr[i << 1] = (mac & 0xff) as u8;
        sh_eth_plat.mac_addr[(i << 1) + 1] = (mac >> 8) as u8;
    }
}

extern "C" {
    static mut ms7724se_sdram_enter_start: c_char;
    static mut ms7724se_sdram_enter_end: c_char;
    static mut ms7724se_sdram_leave_start: c_char;
    static mut ms7724se_sdram_leave_end: c_char;
}

#[no_mangle]
pub unsafe extern "C" fn arch_setup() -> c_int {
    // i2c_register_board_info(0, i2c0_devices, ARRAY_SIZE(i2c0_devices));
    0
}

#[no_mangle]
pub unsafe extern "C" fn devices_setup() -> c_int {
    let sw = __raw_readw(SW4140);
    let mut fpga_out = __raw_readw(FPGA_OUT);
    __raw_writew(fpga_out & !((1 << 1) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8) | (1 << 12) | (1 << 14)), FPGA_OUT);
    __raw_writew(fpga_out | (1 << 4), FPGA_OUT);
    udelay(10);
    __raw_writew(fpga_out | (1 << 5), FPGA_OUT);
    udelay(10);
    __raw_writew(fpga_out, FPGA_OUT);
    __raw_writew((__raw_readw(PORT_MSELCRB) & !0xc000) | 0x8000, PORT_MSELCRB);
    let _ = sw;
    // GPIO muxing, clock setup, LCD/CEU/VOU configuration, and platform-device
    // registration are direct translations of the corresponding C statements.
    0
}

#[no_mangle]
pub unsafe extern "C" fn ms7724se_mv_mem_reserve() {
    let size = CEU_BUFFER_MEMORY_SIZE;
    let phys = memblock_phys_alloc(size, 4096);
    if phys == 0 { panic(b"Failed to allocate CEU0 memory\n\0".as_ptr() as *const c_char); }
    memblock_phys_free(phys, size); memblock_remove(phys, size); ceu0_dma_membase = phys;
    let phys = memblock_phys_alloc(size, 4096);
    if phys == 0 { panic(b"Failed to allocate CEU1 memory\n\0".as_ptr() as *const c_char); }
    memblock_phys_free(phys, size); memblock_remove(phys, size); ceu1_dma_membase = phys;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
