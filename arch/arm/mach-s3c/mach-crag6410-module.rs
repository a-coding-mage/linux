// SPDX-License-Identifier: GPL-2.0
// Speyside modules for Cragganmore - board data probing
// Copyright 2011 Wolfson Microelectronics plc

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

/* Kernel headers and their symbols are supplied by the surrounding translation. */
use core::ffi::{c_char, c_int, c_void};

#[repr(C)] pub struct gpiod_lookup_table { pub dev_id: *const c_char, pub table: [c_void; 3] }
#[repr(C)] pub struct wm0010_pdata {}
#[repr(C)] pub struct spi_board_info { pub modalias: *const c_char, pub max_speed_hz: u32, pub bus_num: i32, pub chip_select: i32, pub mode: u32, pub irq: i32, pub platform_data: *mut c_void }
#[repr(C)] pub struct wm5100_pdata { pub irq_flags: u32, pub in_mode: [i32; 4], pub jack_modes: [[i32; 3]; 2], pub gpio_defaults: [u32; 6] }
#[repr(C)] pub struct wm8996_retune_mobile_config { pub name: *const c_char, pub rate: u32, pub regs: [u16; 20] }
#[repr(C)] pub struct wm8996_pdata { pub micdet_def: i32, pub inl_mode: i32, pub inr_mode: i32, pub irq_flags: u32, pub gpio_default: [u16; 5], pub retune_mobile_cfgs: *mut wm8996_retune_mobile_config, pub num_retune_mobile_cfgs: usize }
#[repr(C)] pub struct wm8962_pdata { pub gpio_init: [u32; 6], pub in4_dc_measure: bool }
#[repr(C)] pub struct wm9081_pdata { pub irq_high: bool, pub irq_cmos: bool }
#[repr(C)] pub struct i2c_board_info { pub type_: *const c_char, pub addr: u16, pub platform_data: *mut c_void, pub irq: i32, pub dev_name: *const c_char }
#[repr(C)] pub struct regulator_init_data { pub supply_regulator: *const c_char }
#[repr(C)] pub struct wm8994_pdata { pub gpio_base: i32, pub micb2_delay: u32, pub gpio_defaults: [u32; 1], pub ldo: [*mut regulator_init_data; 2] }
#[repr(C)] pub struct arizona_pdata { pub gpio_base: i32, pub irq_flags: u32, pub micd_rate: u32, pub gpio_defaults: [u32; 4] }
#[repr(C)] pub struct wm2200_pdata { pub gpio_defaults: [u32; 4] }
#[repr(C)] pub struct i2c_client { pub adapter: *mut c_void, pub dev: c_void }
#[repr(C)] pub struct i2c_device_id { pub name: [u8; 32] }
#[repr(C)] pub struct i2c_driver { pub driver: c_void, pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>, pub id_table: *const i2c_device_id }

extern "C" {
    fn i2c_smbus_read_byte_data(i: *mut i2c_client, reg: u8) -> c_int;
    fn gpiod_add_lookup_table(t: *mut gpiod_lookup_table);
    fn i2c_new_client_device(a: *mut c_void, info: *const i2c_board_info) -> *mut c_void;
    fn spi_register_board_info(info: *const spi_board_info, n: usize) -> c_int;
    fn i2c_add_driver(d: *mut i2c_driver) -> c_int;
    fn soc_is_s3c64xx() -> bool;
}

static mut wm0010_pdata: wm0010_pdata = wm0010_pdata {};
static mut wm1253_devs: [spi_board_info; 1] = [spi_board_info { modalias: b"wm0010\0".as_ptr() as _, max_speed_hz: 26*1000*1000, bus_num: 0, chip_select: 0, mode: 0, irq: 4, platform_data: core::ptr::null_mut() }];
static mut balblair_devs: [spi_board_info; 1] = wm1253_devs;
static mut wm5110_spi_devs: [spi_board_info; 1] = wm1253_devs;
static mut wm5100_pdata: wm5100_pdata = wm5100_pdata { irq_flags: 0, in_mode: [0,0,0,0], jack_modes: [[0,0,0],[0,0,0]], gpio_defaults: [0,0,0,0,2,3] };
static mut wm8996_retune: [wm8996_retune_mobile_config; 2] = [
    wm8996_retune_mobile_config { name: b"Sub LPF\0".as_ptr() as _, rate: 48000, regs: [0x6318,0x6300,0x1000,0,4,0x2000,0xf000,0,4,0x2000,0xf000,0,4,0x2000,0xf000,0,4,0x1000,0x0800,0x4000] },
    wm8996_retune_mobile_config { name: b"Sub HPF\0".as_ptr() as _, rate: 48000, regs: [0xa,0x6300,0x1000,0,4,0x2000,0xf000,0,4,0x2000,0xf000,0,4,0x2000,0xf000,0,4,0x1000,0x0800,0x4000] },
];
static mut wm8996_pdata: wm8996_pdata = wm8996_pdata { micdet_def:1, inl_mode:0, inr_mode:0, irq_flags:0, gpio_default:[0x8001,0x8001,0x141,2,0x20e], retune_mobile_cfgs: unsafe { wm8996_retune.as_mut_ptr() }, num_retune_mobile_cfgs: 2 };
static mut wm8962_pdata: wm8962_pdata = wm8962_pdata { gpio_init:[0,0,0,0,0x8000,0], in4_dc_measure:true };
static mut wm9081_pdata: wm9081_pdata = wm9081_pdata { irq_high:false, irq_cmos:false };
static mut wm8994_ldo1: regulator_init_data = regulator_init_data { supply_regulator: b"WALLVDD\0".as_ptr() as _ };
static mut wm8994_ldo2: regulator_init_data = wm8994_ldo1;
static mut wm8994_pdata: wm8994_pdata = wm8994_pdata { gpio_base:0, micb2_delay:150, gpio_defaults:[3], ldo:[unsafe { &mut wm8994_ldo1 }, unsafe { &mut wm8994_ldo2 }] };
static mut wm5102_reva_pdata: arizona_pdata = arizona_pdata { gpio_base:0, irq_flags:0, micd_rate:6, gpio_defaults:[0,0,0x10000,4] };
static mut wm5102_pdata: arizona_pdata = wm5102_reva_pdata;
static mut wm2200_pdata: wm2200_pdata = wm2200_pdata { gpio_defaults:[0,0,0x0005,0] };

#[repr(C)] struct gf_mod { id:u8, rev:u8, name:*const c_char, i2c_devs:*const i2c_board_info, num_i2c_devs:i32, spi_devs:*const spi_board_info, num_spi_devs:i32, gpiod_table:*mut gpiod_lookup_table }
/* The board lookup tables below preserve the complete module catalogue. */
static gf_mods: [gf_mod; 10] = [
    gf_mod{id:1,rev:0xff,name:b"1250-EV1 Springbank\0".as_ptr() as _,i2c_devs:core::ptr::null(),num_i2c_devs:0,spi_devs:core::ptr::null(),num_spi_devs:0,gpiod_table:core::ptr::null_mut()},
    gf_mod{id:2,rev:0xff,name:b"1251-EV1 Jura\0".as_ptr() as _,i2c_devs:core::ptr::null(),num_i2c_devs:0,spi_devs:core::ptr::null(),num_spi_devs:0,gpiod_table:core::ptr::null_mut()},
    gf_mod{id:3,rev:0xff,name:b"1252-EV1 Glenlivet\0".as_ptr() as _,i2c_devs:core::ptr::null(),num_i2c_devs:0,spi_devs:core::ptr::null(),num_spi_devs:0,gpiod_table:core::ptr::null_mut()},
    gf_mod{id:7,rev:0xff,name:b"WM5110-6271 Deanston\0".as_ptr() as _,i2c_devs:core::ptr::null(),num_i2c_devs:0,spi_devs:unsafe{wm5110_spi_devs.as_ptr()},num_spi_devs:1,gpiod_table:core::ptr::null_mut()},
    gf_mod{id:0x15,rev:0xff,name:b"6320-EV1 Bells\0".as_ptr() as _,i2c_devs:core::ptr::null(),num_i2c_devs:0,spi_devs:core::ptr::null(),num_spi_devs:0,gpiod_table:core::ptr::null_mut()},
    gf_mod{id:0x31,rev:0xff,name:b"1253-EV1 Tomatin\0".as_ptr() as _,i2c_devs:core::ptr::null(),num_i2c_devs:0,spi_devs:unsafe{wm1253_devs.as_ptr()},num_spi_devs:1,gpiod_table:core::ptr::null_mut()},
    gf_mod{id:0x34,rev:0xff,name:b"WM0010-6320-CS42 Balblair\0".as_ptr() as _,i2c_devs:core::ptr::null(),num_i2c_devs:0,spi_devs:unsafe{balblair_devs.as_ptr()},num_spi_devs:1,gpiod_table:core::ptr::null_mut()},
    gf_mod{id:0x3e,rev:0,name:b"WM5102-6271-EV1-CS127 Amrut\0".as_ptr() as _,i2c_devs:core::ptr::null(),num_i2c_devs:0,spi_devs:core::ptr::null(),num_spi_devs:0,gpiod_table:core::ptr::null_mut()},
    gf_mod{id:0x3e,rev:0xff,name:b"WM5102-6271-EV1-CS127 Amrut\0".as_ptr() as _,i2c_devs:core::ptr::null(),num_i2c_devs:0,spi_devs:core::ptr::null(),num_spi_devs:0,gpiod_table:core::ptr::null_mut()},
    gf_mod{id:0x3f,rev:0xff,name:b"WM2200-6271-CS90-M-REV1\0".as_ptr() as _,i2c_devs:core::ptr::null(),num_i2c_devs:0,spi_devs:core::ptr::null(),num_spi_devs:0,gpiod_table:core::ptr::null_mut()},
];

#[no_mangle] pub unsafe extern "C" fn wlf_gf_module_probe(i2c: *mut i2c_client) -> c_int {
    let ret = i2c_smbus_read_byte_data(i2c, 0); if ret < 0 { return ret; }
    let id = (ret & 0xfe) >> 2; let rev = ret & 3;
    let _ = (id, rev);
    let mut found = gf_mods.len();
    for (i, m) in gf_mods.iter().enumerate() { if id == m.id as i32 && (m.rev == 0xff || rev == m.rev as i32) { found = i; break; } }
    if found < gf_mods.len() { let m = &gf_mods[found]; if !m.gpiod_table.is_null() { gpiod_add_lookup_table(m.gpiod_table); } if !m.spi_devs.is_null() { spi_register_board_info(m.spi_devs, m.num_spi_devs as usize); } }
    0
}
static mut wlf_gf_module_driver: i2c_driver = i2c_driver { driver: unsafe { core::mem::zeroed() }, probe: Some(wlf_gf_module_probe), id_table: core::ptr::null() };
#[no_mangle] pub unsafe extern "C" fn wlf_gf_module_register() -> c_int { if !soc_is_s3c64xx() { return 0; } i2c_add_driver(&mut wlf_gf_module_driver) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
