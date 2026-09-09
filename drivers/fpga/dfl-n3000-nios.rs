// SPDX-License-Identifier: GPL-2.0
/* Rust translation of dfl-n3000-nios.c. Kernel dependencies are external. */

const N3000_NS_PARAM: u64 = 0x8;
const N3000_NS_PARAM_SHIFT_MODE_MSK: u64 = 1 << 1;
const N3000_NS_PARAM_SHIFT_MODE_MSB: u64 = 0;
const N3000_NS_PARAM_SHIFT_MODE_LSB: u64 = 1;
const N3000_NS_PARAM_DATA_WIDTH: u64 = 0xfc;
const N3000_NS_PARAM_NUM_CS: u64 = 0x3f00;
const N3000_NS_PARAM_CLK_POL: u64 = 1 << 14;
const N3000_NS_PARAM_CLK_PHASE: u64 = 1 << 15;
const N3000_NS_PARAM_PERIPHERAL_ID: u64 = 0xffff00000000;
const N3000_NS_CTRL: u64 = 0x10;
const N3000_NS_CTRL_WR_DATA: u64 = 0xffff_ffff;
const N3000_NS_CTRL_ADDR: u64 = 0x1fff00000000;
const N3000_NS_CTRL_CMD_MSK: u64 = 0xc000000000000000;
const N3000_NS_CTRL_CMD_NOP: u64 = 0;
const N3000_NS_CTRL_CMD_RD: u64 = 1;
const N3000_NS_CTRL_CMD_WR: u64 = 2;
const N3000_NS_STAT: u64 = 0x18;
const N3000_NS_STAT_RD_DATA: u64 = 0xffff_ffff;
const N3000_NS_STAT_RW_VAL: u64 = 1 << 32;

const N3000_NIOS_INIT: u32 = 0x1000;
const N3000_NIOS_INIT_DONE: u32 = 1;
const N3000_NIOS_INIT_START: u32 = 1 << 1;
const N3000_NIOS_INIT_REQ_FEC_MODE_A0_MSK: u32 = 0x300;
const N3000_NIOS_INIT_REQ_FEC_MODE_A1_MSK: u32 = 0xc00;
const N3000_NIOS_INIT_REQ_FEC_MODE_A2_MSK: u32 = 0x3000;
const N3000_NIOS_INIT_REQ_FEC_MODE_A3_MSK: u32 = 0xc000;
const N3000_NIOS_INIT_REQ_FEC_MODE_B0_MSK: u32 = 0x30000;
const N3000_NIOS_INIT_REQ_FEC_MODE_B1_MSK: u32 = 0xc0000;
const N3000_NIOS_INIT_REQ_FEC_MODE_B2_MSK: u32 = 0x300000;
const N3000_NIOS_INIT_REQ_FEC_MODE_B3_MSK: u32 = 0xc00000;
const N3000_NIOS_INIT_REQ_FEC_MODE_NO: u32 = 0;
const N3000_NIOS_INIT_REQ_FEC_MODE_KR: u32 = 1;
const N3000_NIOS_INIT_REQ_FEC_MODE_RS: u32 = 2;
const N3000_NIOS_FW_VERSION: u32 = 0x1004;
const N3000_NIOS_FW_VERSION_PATCH: u32 = 0x00f00000;
const N3000_NIOS_FW_VERSION_MINOR: u32 = 0x0f000000;
const N3000_NIOS_FW_VERSION_MAJOR: u32 = 0xf0000000;
const N3000_NIOS_PKVL_A_MODE_STS: u32 = 0x1020;
const N3000_NIOS_PKVL_B_MODE_STS: u32 = 0x1024;
const N3000_NIOS_PKVL_MODE_STS_GROUP_MSK: u32 = 0xff00;
const N3000_NIOS_PKVL_MODE_STS_GROUP_OK: u32 = 0;
const N3000_NIOS_PKVL_MODE_STS_ID_MSK: u32 = 0xff;
const N3000_NIOS_PKVL_MODE_ID_RESET: u32 = 0;
const N3000_NIOS_PKVL_MODE_ID_4X10G: u32 = 1;
const N3000_NIOS_PKVL_MODE_ID_4X25G: u32 = 2;
const N3000_NIOS_PKVL_MODE_ID_2X25G: u32 = 3;
const N3000_NIOS_PKVL_MODE_ID_2X25G_2X10G: u32 = 4;
const N3000_NIOS_PKVL_MODE_ID_1X25G: u32 = 5;
const N3000_NIOS_REGBUS_RETRY_COUNT: i32 = 10000;
const N3000_NIOS_INIT_TIMEOUT: u32 = 10000000;
const N3000_NIOS_INIT_TIME_INTV: u32 = 100000;

const fn field_get(mask: u32, value: u32) -> u32 { (value & mask) >> mask.trailing_zeros() }
const fn field_prep(mask: u64, value: u64) -> u64 { (value << mask.trailing_zeros()) & mask }
const N3000_NIOS_INIT_REQ_FEC_MODE_MSK_ALL: u32 = 0x00ffff00;
const N3000_NIOS_INIT_REQ_FEC_MODE_NO_ALL: u32 = 0;
const N3000_NIOS_INIT_REQ_FEC_MODE_KR_ALL: u32 = 0x00555500;
const N3000_NIOS_INIT_REQ_FEC_MODE_RS_ALL: u32 = 0x00aaaa00;

#[repr(C)]
pub struct n3000_nios { pub base: *mut core::ffi::c_void, pub regmap: *mut regmap, pub dev: *mut device, pub altera_spi: *mut platform_device }
#[repr(C)] pub struct regmap;
#[repr(C)] pub struct device;
#[repr(C)] pub struct platform_device;
#[repr(C)] pub struct dfl_device { pub dev: device, pub mmio_res: resource }
#[repr(C)] pub struct resource;
#[repr(C)] pub struct device_attribute;

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> i32;
    fn sysfs_emit(buf: *mut i8, fmt: *const i8, ...) -> isize;
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn readq(addr: *mut core::ffi::c_void) -> u64;
    fn writeq(value: u64, addr: *mut core::ffi::c_void);
    fn cpu_relax();
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    fn devm_ioremap_resource(dev: *mut device, res: *mut resource) -> *mut core::ffi::c_void;
    fn devm_regmap_init(dev: *mut device, bus: *const core::ffi::c_void, context: *mut n3000_nios, cfg: *const regmap_config) -> *mut regmap;
    fn platform_device_register_full(info: *const platform_device_info) -> *mut platform_device;
    fn platform_device_unregister(dev: *mut platform_device);
}
#[repr(C)] pub struct regmap_config { pub reg_bits: u32, pub reg_stride: u32, pub val_bits: u32, pub fast_io: bool, pub reg_write: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32,u32)->i32>, pub reg_read: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32,*mut u32)->i32> }
#[repr(C)] pub struct platform_device_info { pub name: *const i8, pub id: i32, pub parent: *mut device, pub data: *mut core::ffi::c_void, pub size_data: usize }
#[repr(C)] pub struct spi_board_info { pub modalias: *const i8, pub max_speed_hz: u32, pub bus_num: u16, pub chip_select: u16 }
#[repr(C)] pub struct altera_spi_platform_data { pub mode_bits: u32, pub num_chipselect: u32, pub bits_per_word_mask: u32, pub num_devices: u32, pub devices: *mut spi_board_info }

unsafe fn get_retimer_mode(nn: *mut n3000_nios, reg: u32, out: *mut u32) -> i32 { let mut v=0; let r=regmap_read((*nn).regmap,reg,&mut v); if r!=0{return r} if field_get(N3000_NIOS_PKVL_MODE_STS_GROUP_MSK,v)!=0{return -14} *out=field_get(N3000_NIOS_PKVL_MODE_STS_ID_MSK,v); 0 }
unsafe fn n3000_nios_poll_stat_timeout(base:*mut core::ffi::c_void,v:*mut u64)->i32 { let mut loops=N3000_NIOS_REGBUS_RETRY_COUNT; while loops>0 { *v=readq(base.add(N3000_NS_STAT as usize)); if *v & N3000_NS_STAT_RW_VAL != 0 {break} cpu_relax(); loops-=1; } if loops>0 {0} else {-110} }
pub unsafe extern "C" fn n3000_nios_reg_write(context:*mut core::ffi::c_void,reg:u32,val:u32)->i32 { let nn=context as *mut n3000_nios; let v=field_prep(N3000_NS_CTRL_CMD_MSK,N3000_NS_CTRL_CMD_WR)|field_prep(N3000_NS_CTRL_ADDR,reg as u64)|field_prep(N3000_NS_CTRL_WR_DATA,val as u64); writeq(v,(*nn).base.add(N3000_NS_CTRL as usize)); let mut s=0; n3000_nios_poll_stat_timeout((*nn).base,&mut s) }
pub unsafe extern "C" fn n3000_nios_reg_read(context:*mut core::ffi::c_void,reg:u32,val:*mut u32)->i32 { let nn=context as *mut n3000_nios; let v=field_prep(N3000_NS_CTRL_CMD_MSK,N3000_NS_CTRL_CMD_RD)|field_prep(N3000_NS_CTRL_ADDR,reg as u64); writeq(v,(*nn).base.add(N3000_NS_CTRL as usize)); let mut s=0; let r=n3000_nios_poll_stat_timeout((*nn).base,&mut s); if r==0 {*val=(s&N3000_NS_STAT_RD_DATA) as u32} r }

unsafe fn is_retimer_fec_supported(mode: u32) -> bool { mode != N3000_NIOS_PKVL_MODE_ID_RESET && mode != N3000_NIOS_PKVL_MODE_ID_4X10G }
unsafe fn n3000_nios_init_done_check(nn: *mut n3000_nios) -> i32 {
    let mut val=0; let r=regmap_read((*nn).regmap,N3000_NIOS_FW_VERSION,&mut val); if r!=0{return r}
    if val==0{return 0}
    if field_get(N3000_NIOS_FW_VERSION_MAJOR,val)>=3 {
        let r=regmap_read((*nn).regmap,N3000_NIOS_INIT,&mut val); if r!=0{return r}
        if val & (N3000_NIOS_INIT_DONE|N3000_NIOS_INIT_START)==0 { val=N3000_NIOS_INIT_START|N3000_NIOS_INIT_REQ_FEC_MODE_RS_ALL; let r=regmap_write((*nn).regmap,N3000_NIOS_INIT,val); if r!=0{return r} }
    }
    let mut state_a=0; let mut state_b=0; let r=regmap_read((*nn).regmap,N3000_NIOS_PKVL_A_MODE_STS,&mut state_a); if r!=0{return r}; let r=regmap_read((*nn).regmap,N3000_NIOS_PKVL_B_MODE_STS,&mut state_b); if r!=0{return r}; r
}
static mut M10_N3000_INFO: spi_board_info = spi_board_info { modalias: b"m10-n3000\0".as_ptr() as *const i8, max_speed_hz:12500000, bus_num:0, chip_select:0 };
unsafe fn create_altera_spi_controller(nn:*mut n3000_nios)->i32 { let mut p=altera_spi_platform_data{mode_bits:1,num_chipselect:0,bits_per_word_mask:0,num_devices:1,devices:&mut M10_N3000_INFO}; let v=readq((*nn).base.add(N3000_NS_PARAM as usize)); p.num_chipselect=field_get(N3000_NS_PARAM_NUM_CS as u32,v as u32); p.bits_per_word_mask=field_get(N3000_NS_PARAM_DATA_WIDTH as u32,v as u32); 0 }
unsafe fn destroy_altera_spi_controller(nn:*mut n3000_nios) { platform_device_unregister((*nn).altera_spi); }
unsafe fn n3000_nios_probe(ddev:*mut dfl_device)->i32 { let nn=devm_kzalloc(&mut (*ddev).dev as *mut _,core::mem::size_of::<n3000_nios>(),0) as *mut n3000_nios; if nn.is_null(){return -12}; dev_set_drvdata(&mut (*ddev).dev,nn as *mut _); (*nn).dev=&mut (*ddev).dev; (*nn).base=devm_ioremap_resource((*nn).dev,&mut (*ddev).mmio_res); n3000_nios_init_done_check(nn) }
unsafe fn n3000_nios_remove(ddev:*mut dfl_device) { destroy_altera_spi_controller(dev_get_drvdata(&mut (*ddev).dev) as *mut n3000_nios); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
