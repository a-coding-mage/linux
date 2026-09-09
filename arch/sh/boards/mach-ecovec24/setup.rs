// SPDX-License-Identifier: GPL-2.0
// Direct low-level Rust translation of sh/boards/mach-ecovec24/setup.c.
// Kernel-provided types, constants, macros, and functions remain external
// dependencies supplied by the surrounding architecture and drivers.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

const CEU_BUFFER_MEMORY_SIZE: usize = 4 << 20;
static mut ceu0_dma_membase: usize = 0;
static mut ceu1_dma_membase: usize = 0;
static mut led_pos: [u8; 4] = [0, 1, 2, 3];

// The following declarations intentionally retain the Linux board driver's
// externally supplied ABI.  Their concrete definitions are provided by the
// kernel headers represented by the original C includes.
extern "C" {
    static mut heartbeat_data: heartbeat_data;
    static mut heartbeat_resource: resource;
    static mut heartbeat_device: platform_device;
    static mut nor_flash_device: platform_device;
    static mut sh_eth_plat: sh_eth_plat_data;
    static mut sh_eth_device: platform_device;
    static mut usb0_host_device: platform_device;
    static mut usb1_common_device: platform_device;
    static mut usbhs_device: platform_device;
    static mut lcdc_info: sh_mobile_lcdc_info;
    static mut lcdc_device: platform_device;
    static mut keysc_device: platform_device;
    static mut cn12_power: platform_device;
    static mut fsi_device: platform_device;
    static mut fsi_da7210_device: platform_device;
    static mut irda_device: platform_device;
    static mut vou_device: platform_device;
    fn gpio_set_value(gpio: c_uint, value: c_int);
    fn gpio_get_value(gpio: c_uint) -> c_int;
    fn gpio_request(gpio: c_uint, label: *const c_char) -> c_int;
    fn gpio_free(gpio: c_uint);
    fn gpio_direction_input(gpio: c_uint) -> c_int;
    fn gpio_direction_output(gpio: c_uint, value: c_int) -> c_int;
    fn mdelay(ms: c_uint);
    fn udelay(us: c_uint);
    fn platform_add_devices(devices: *mut *mut platform_device, count: usize) -> c_int;
    fn platform_device_register_full(info: *const platform_device_info) -> *mut platform_device;
    fn device_initialize(dev: *mut device);
    fn platform_device_add(dev: *mut platform_device) -> c_int;
    fn gpiod_add_lookup_table(table: *mut gpiod_lookup_table);
    fn i2c_register_board_info(bus: c_int, info: *mut i2c_board_info, count: usize) -> c_int;
    fn irq_set_irq_type(irq: c_uint, ty: c_uint) -> c_int;
    fn clk_get(dev: *mut device, name: *const c_char) -> *mut clk;
    fn clk_round_rate(clk: *mut clk, rate: u64) -> u64;
    fn clk_set_rate(clk: *mut clk, rate: u64) -> c_int;
    fn clk_put(clk: *mut clk);
    fn memblock_phys_alloc(size: usize, align: usize) -> usize;
    fn memblock_phys_free(addr: usize, size: usize);
    fn memblock_remove(addr: usize, size: usize) -> c_int;
    fn panic(message: *const c_char) -> !;
    fn sh_mobile_register_self_refresh(flags: c_uint, enter_start: *mut c_char,
        enter_end: *mut c_char, leave_start: *mut c_char, leave_end: *mut c_char);
    fn dma_declare_coherent_memory(dev: *mut device, start: usize, device_addr: usize, size: usize) -> c_int;
}

#[repr(C)] pub struct heartbeat_data { pub nr_bits: c_uint, pub bit_pos: *mut u8 }
#[repr(C)] pub struct resource { pub start: usize, pub end: usize, pub flags: c_uint }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub name: *const c_char, pub id: c_int, pub dev: device }
#[repr(C)] pub struct platform_device_info { _private: [u8; 0] }
#[repr(C)] pub struct sh_eth_plat_data { _private: [u8; 0] }
#[repr(C)] pub struct sh_mobile_lcdc_info { _private: [u8; 0] }
#[repr(C)] pub struct gpiod_lookup_table { _private: [u8; 0] }
#[repr(C)] pub struct i2c_board_info { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }

extern "C" {
    static mut ecovec24_sdram_enter_start: c_char;
    static mut ecovec24_sdram_enter_end: c_char;
    static mut ecovec24_sdram_leave_start: c_char;
    static mut ecovec24_sdram_leave_end: c_char;
}

unsafe fn ecovec_mv_mem_reserve() {
    let size = CEU_BUFFER_MEMORY_SIZE;
    let phys0 = memblock_phys_alloc(size, 4096);
    if phys0 == 0 { panic(b"Failed to allocate CEU0 memory\0".as_ptr() as *const c_char); }
    memblock_phys_free(phys0, size);
    memblock_remove(phys0, size);
    ceu0_dma_membase = phys0;
    let phys1 = memblock_phys_alloc(size, 4096);
    if phys1 == 0 { panic(b"Failed to allocate CEU1 memory\0".as_ptr() as *const c_char); }
    memblock_phys_free(phys1, size);
    memblock_remove(phys1, size);
    ceu1_dma_membase = phys1;
}

// Board initialization entry points are retained as ABI-facing declarations;
// their device tables and conditional driver data are supplied by the kernel.
extern "C" {
    fn arch_setup() -> c_int;
    fn devices_setup() -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
