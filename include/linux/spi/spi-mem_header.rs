/* SPDX-License-Identifier: GPL-2.0+ */
/* Rust translation of linux/spi/spi-mem.h. */

/* Dependency supplied by the surrounding kernel translation. */

#[repr(C)]
#[derive(Copy, Clone)]
pub enum spi_mem_data_dir {
    SPI_MEM_NO_DATA,
    SPI_MEM_DATA_IN,
    SPI_MEM_DATA_OUT,
}

#[repr(C)]
pub union spi_mem_buf {
    pub in_: *mut core::ffi::c_void,
    pub out: *const core::ffi::c_void,
}

#[repr(C)]
pub struct spi_mem_op_cmd { pub nbytes: u8, pub buswidth: u8, pub dtr: u8, pub __pad: u8, pub opcode: u16 }
#[repr(C)]
pub struct spi_mem_op_addr { pub nbytes: u8, pub buswidth: u8, pub dtr: u8, pub __pad: u8, pub val: u64 }
#[repr(C)]
pub struct spi_mem_op_dummy { pub nbytes: u8, pub buswidth: u8, pub dtr: u8, pub __pad: u8 }
#[repr(C)]
pub struct spi_mem_op_data {
    pub buswidth: u8, pub dtr: u8, pub ecc: u8, pub swap16: u8,
    pub __pad: u8, pub dir: spi_mem_data_dir, pub nbytes: core::ffi::c_uint,
    pub buf: spi_mem_buf,
}
#[repr(C)]
pub struct spi_mem_op {
    pub cmd: spi_mem_op_cmd,
    pub addr: spi_mem_op_addr,
    pub dummy: spi_mem_op_dummy,
    pub data: spi_mem_op_data,
    pub max_freq: core::ffi::c_uint,
}

#[macro_export] macro_rules! SPI_MEM_OP_CMD { ($opcode:expr, $buswidth:expr) => { spi_mem_op_cmd { nbytes: 1, buswidth: $buswidth, dtr: 0, __pad: 0, opcode: $opcode } }; }
#[macro_export] macro_rules! SPI_MEM_DTR_OP_RPT_CMD { ($opcode:expr, $buswidth:expr) => { spi_mem_op_cmd { nbytes: 2, buswidth: $buswidth, dtr: 1, __pad: 0, opcode: ($opcode | (($opcode as u16) << 8)) } }; }
#[macro_export] macro_rules! SPI_MEM_DTR_OP_PACKED_CMD { ($opcode:expr, $addr:expr, $buswidth:expr) => { spi_mem_op_cmd { nbytes: 2, buswidth: $buswidth, dtr: 1, __pad: 0, opcode: (($opcode as u16) << 8) | $addr } }; }
#[macro_export] macro_rules! SPI_MEM_OP_ADDR { ($nbytes:expr, $val:expr, $buswidth:expr) => { spi_mem_op_addr { nbytes: $nbytes, buswidth: $buswidth, dtr: 0, __pad: 0, val: $val } }; }
#[macro_export] macro_rules! SPI_MEM_DTR_OP_ADDR { ($nbytes:expr, $val:expr, $buswidth:expr) => { spi_mem_op_addr { nbytes: $nbytes, buswidth: $buswidth, dtr: 1, __pad: 0, val: $val } }; }
#[macro_export] macro_rules! SPI_MEM_DTR_OP_RPT_ADDR { ($val:expr, $buswidth:expr) => { spi_mem_op_addr { nbytes: 2, buswidth: $buswidth, dtr: 1, __pad: 0, val: $val | (($val as u64) << 8) } }; }
#[macro_export] macro_rules! SPI_MEM_OP_NO_ADDR { () => { spi_mem_op_addr { nbytes: 0, buswidth: 0, dtr: 0, __pad: 0, val: 0 } }; }
#[macro_export] macro_rules! SPI_MEM_OP_DUMMY { ($nbytes:expr, $buswidth:expr) => { spi_mem_op_dummy { nbytes: $nbytes, buswidth: $buswidth, dtr: 0, __pad: 0 } }; }
#[macro_export] macro_rules! SPI_MEM_DTR_OP_DUMMY { ($nbytes:expr, $buswidth:expr) => { spi_mem_op_dummy { nbytes: $nbytes, buswidth: $buswidth, dtr: 1, __pad: 0 } }; }
#[macro_export] macro_rules! SPI_MEM_OP_NO_DUMMY { () => { spi_mem_op_dummy { nbytes: 0, buswidth: 0, dtr: 0, __pad: 0 } }; }
#[macro_export] macro_rules! SPI_MEM_OP_NO_DATA { () => { spi_mem_op_data { buswidth: 0, dtr: 0, ecc: 0, swap16: 0, __pad: 0, dir: spi_mem_data_dir::SPI_MEM_NO_DATA, nbytes: 0, buf: spi_mem_buf { in_: core::ptr::null_mut() } } }; }
#[macro_export] macro_rules! SPI_MEM_OP_DATA_IN { ($nbytes:expr, $buf:expr, $buswidth:expr) => { spi_mem_op_data { buswidth: $buswidth, dtr: 0, ecc: 0, swap16: 0, __pad: 0, dir: spi_mem_data_dir::SPI_MEM_DATA_IN, nbytes: $nbytes, buf: spi_mem_buf { in_: $buf } } }; }
#[macro_export] macro_rules! SPI_MEM_DTR_OP_DATA_IN { ($nbytes:expr, $buf:expr, $buswidth:expr) => { spi_mem_op_data { buswidth: $buswidth, dtr: 1, ecc: 0, swap16: 0, __pad: 0, dir: spi_mem_data_dir::SPI_MEM_DATA_IN, nbytes: $nbytes, buf: spi_mem_buf { in_: $buf } } }; }
#[macro_export] macro_rules! SPI_MEM_OP_DATA_OUT { ($nbytes:expr, $buf:expr, $buswidth:expr) => { spi_mem_op_data { buswidth: $buswidth, dtr: 0, ecc: 0, swap16: 0, __pad: 0, dir: spi_mem_data_dir::SPI_MEM_DATA_OUT, nbytes: $nbytes, buf: spi_mem_buf { out: $buf } } }; }
#[macro_export] macro_rules! SPI_MEM_DTR_OP_DATA_OUT { ($nbytes:expr, $buf:expr, $buswidth:expr) => { spi_mem_op_data { buswidth: $buswidth, dtr: 1, ecc: 0, swap16: 0, __pad: 0, dir: spi_mem_data_dir::SPI_MEM_DATA_OUT, nbytes: $nbytes, buf: spi_mem_buf { out: $buf } } }; }
#[macro_export] macro_rules! SPI_MEM_OP { ($cmd:expr, $addr:expr, $dummy:expr, $data:expr $(, $rest:tt)*) => { spi_mem_op { cmd: $cmd, addr: $addr, dummy: $dummy, data: $data, $($rest)* } }; }
#[macro_export] macro_rules! SPI_MEM_OP_MAX_FREQ { ($freq:expr) => { max_freq: $freq }; }

#[repr(C)] pub struct spi_mem_dirmap_info { pub op_tmpl: *mut spi_mem_op, pub primary_op_tmpl: spi_mem_op, pub secondary_op_tmpl: spi_mem_op, pub offset: u64, pub length: u64 }
#[repr(C)] pub struct spi_mem_dirmap_desc { pub mem: *mut spi_mem, pub info: spi_mem_dirmap_info, pub nodirmap: core::ffi::c_uint, pub priv_: *mut core::ffi::c_void }
#[repr(C)] pub struct spi_mem { pub spi: *mut spi_device, pub drvpriv: *mut core::ffi::c_void, pub name: *const core::ffi::c_char, pub dqs: bool }

#[inline] pub unsafe fn spi_mem_set_drvdata(mem: *mut spi_mem, data: *mut core::ffi::c_void) { (*mem).drvpriv = data; }
#[inline] pub unsafe fn spi_mem_get_drvdata(mem: *mut spi_mem) -> *mut core::ffi::c_void { (*mem).drvpriv }

#[repr(C)] pub struct spi_controller_mem_ops {
    pub adjust_op_size: Option<unsafe extern "C" fn(*mut spi_mem, *mut spi_mem_op) -> core::ffi::c_int>,
    pub supports_op: Option<unsafe extern "C" fn(*mut spi_mem, *const spi_mem_op) -> bool>,
    pub exec_op: Option<unsafe extern "C" fn(*mut spi_mem, *const spi_mem_op) -> core::ffi::c_int>,
    pub get_name: Option<unsafe extern "C" fn(*mut spi_mem) -> *const core::ffi::c_char>,
    pub dirmap_create: Option<unsafe extern "C" fn(*mut spi_mem_dirmap_desc) -> core::ffi::c_int>,
    pub dirmap_destroy: Option<unsafe extern "C" fn(*mut spi_mem_dirmap_desc)>,
    pub dirmap_read: Option<unsafe extern "C" fn(*mut spi_mem_dirmap_desc, u64, usize, *mut core::ffi::c_void) -> isize>,
    pub dirmap_write: Option<unsafe extern "C" fn(*mut spi_mem_dirmap_desc, u64, usize, *const core::ffi::c_void) -> isize>,
    pub poll_status: Option<unsafe extern "C" fn(*mut spi_mem, *const spi_mem_op, u16, u16, usize, usize, usize) -> core::ffi::c_int>,
}
#[repr(C)] pub struct spi_controller_mem_caps { pub dtr: bool, pub ecc: bool, pub swap16: bool, pub per_op_freq: bool, pub secondary_op_tmpl: bool, pub no_cs_assertion: bool }
#[macro_export] macro_rules! spi_mem_controller_is_capable { ($ctlr:expr, $cap:ident) => { unsafe { !(*$ctlr).mem_caps.is_null() && (*(*$ctlr).mem_caps).$cap } }; }

#[repr(C)] pub struct spi_mem_driver { pub spidrv: spi_driver, pub probe: Option<unsafe extern "C" fn(*mut spi_mem) -> core::ffi::c_int>, pub remove: Option<unsafe extern "C" fn(*mut spi_mem) -> core::ffi::c_int>, pub shutdown: Option<unsafe extern "C" fn(*mut spi_mem)> }

/* Types supplied by linux/spi/spi.h and other kernel headers. */
pub enum spi_device {}
pub enum spi_driver {}
pub enum spi_controller {}
pub enum sg_table {}
pub enum device {}
pub enum module {}

extern "C" {
    pub fn spi_controller_dma_map_mem_op_data(ctlr: *mut spi_controller, op: *const spi_mem_op, sg: *mut sg_table) -> core::ffi::c_int;
    pub fn spi_controller_dma_unmap_mem_op_data(ctlr: *mut spi_controller, op: *const spi_mem_op, sg: *mut sg_table);
    pub fn spi_mem_default_supports_op(mem: *mut spi_mem, op: *const spi_mem_op) -> bool;
    pub fn spi_mem_set_dqs(mem: *mut spi_mem);
    pub fn spi_mem_has_dqs(mem: *mut spi_mem) -> bool;
    pub fn spi_mem_adjust_op_size(mem: *mut spi_mem, op: *mut spi_mem_op) -> core::ffi::c_int;
    pub fn spi_mem_adjust_op_freq(mem: *mut spi_mem, op: *mut spi_mem_op);
    pub fn spi_mem_calc_op_duration(mem: *mut spi_mem, op: *mut spi_mem_op) -> u64;
    pub fn spi_mem_supports_op(mem: *mut spi_mem, op: *const spi_mem_op) -> bool;
    pub fn spi_mem_exec_op(mem: *mut spi_mem, op: *const spi_mem_op) -> core::ffi::c_int;
    pub fn spi_mem_get_name(mem: *mut spi_mem) -> *const core::ffi::c_char;
    pub fn spi_mem_dirmap_create(mem: *mut spi_mem, info: *const spi_mem_dirmap_info) -> *mut spi_mem_dirmap_desc;
    pub fn spi_mem_dirmap_destroy(desc: *mut spi_mem_dirmap_desc);
    pub fn spi_mem_dirmap_read(desc: *mut spi_mem_dirmap_desc, offs: u64, len: usize, buf: *mut core::ffi::c_void) -> isize;
    pub fn spi_mem_dirmap_write(desc: *mut spi_mem_dirmap_desc, offs: u64, len: usize, buf: *const core::ffi::c_void) -> isize;
    pub fn devm_spi_mem_dirmap_create(dev: *mut device, mem: *mut spi_mem, info: *const spi_mem_dirmap_info) -> *mut spi_mem_dirmap_desc;
    pub fn devm_spi_mem_dirmap_destroy(dev: *mut device, desc: *mut spi_mem_dirmap_desc);
    pub fn spi_mem_poll_status(mem: *mut spi_mem, op: *const spi_mem_op, mask: u16, match_: u16, initial_delay_us: usize, polling_delay_us: usize, timeout_ms: u16) -> core::ffi::c_int;
    pub fn spi_mem_driver_register_with_owner(drv: *mut spi_mem_driver, owner: *mut module) -> core::ffi::c_int;
    pub fn spi_mem_driver_unregister(drv: *mut spi_mem_driver);
}

/* CONFIG_SPI_MEM conditional declarations are preserved as external interfaces. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
