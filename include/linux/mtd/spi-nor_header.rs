/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2014 Freescale Semiconductor, Inc.
 */

// C dependencies: linux/bitops.h, linux/mtd/mtd.h, linux/spi/spi-mem.h

/* Flash opcodes. */
pub const SPINOR_OP_WRDI: u8 = 0x04;
pub const SPINOR_OP_WREN: u8 = 0x06;
pub const SPINOR_OP_RDSR: u8 = 0x05;
pub const SPINOR_OP_WRSR: u8 = 0x01;
pub const SPINOR_OP_RDSR2: u8 = 0x3f;
pub const SPINOR_OP_WRSR2: u8 = 0x3e;
pub const SPINOR_OP_READ: u8 = 0x03;
pub const SPINOR_OP_READ_FAST: u8 = 0x0b;
pub const SPINOR_OP_READ_1_1_2: u8 = 0x3b;
pub const SPINOR_OP_READ_1_2_2: u8 = 0xbb;
pub const SPINOR_OP_READ_1_1_4: u8 = 0x6b;
pub const SPINOR_OP_READ_1_4_4: u8 = 0xeb;
pub const SPINOR_OP_READ_1_1_8: u8 = 0x8b;
pub const SPINOR_OP_READ_1_8_8: u8 = 0xcb;
pub const SPINOR_OP_PP: u8 = 0x02;
pub const SPINOR_OP_PP_1_1_4: u8 = 0x32;
pub const SPINOR_OP_PP_1_4_4: u8 = 0x38;
pub const SPINOR_OP_PP_1_1_8: u8 = 0x82;
pub const SPINOR_OP_PP_1_8_8: u8 = 0xc2;
pub const SPINOR_OP_BE_4K: u8 = 0x20;
pub const SPINOR_OP_BE_4K_PMC: u8 = 0xd7;
pub const SPINOR_OP_BE_32K: u8 = 0x52;
pub const SPINOR_OP_CHIP_ERASE: u8 = 0xc7;
pub const SPINOR_OP_SE: u8 = 0xd8;
pub const SPINOR_OP_RDID: u8 = 0x9f;
pub const SPINOR_OP_RDSFDP: u8 = 0x5a;
pub const SPINOR_OP_RDCR: u8 = 0x35;
pub const SPINOR_OP_SRSTEN: u8 = 0x66;
pub const SPINOR_OP_SRST: u8 = 0x99;
pub const SPINOR_OP_GBULK: u8 = 0x98;

pub const SPINOR_OP_READ_4B: u8 = 0x13;
pub const SPINOR_OP_READ_FAST_4B: u8 = 0x0c;
pub const SPINOR_OP_READ_1_1_2_4B: u8 = 0x3c;
pub const SPINOR_OP_READ_1_2_2_4B: u8 = 0xbc;
pub const SPINOR_OP_READ_1_1_4_4B: u8 = 0x6c;
pub const SPINOR_OP_READ_1_4_4_4B: u8 = 0xec;
pub const SPINOR_OP_READ_1_1_8_4B: u8 = 0x7c;
pub const SPINOR_OP_READ_1_8_8_4B: u8 = 0xcc;
pub const SPINOR_OP_PP_4B: u8 = 0x12;
pub const SPINOR_OP_PP_1_1_4_4B: u8 = 0x34;
pub const SPINOR_OP_PP_1_4_4_4B: u8 = 0x3e;
pub const SPINOR_OP_PP_1_1_8_4B: u8 = 0x84;
pub const SPINOR_OP_PP_1_8_8_4B: u8 = 0x8e;
pub const SPINOR_OP_BE_4K_4B: u8 = 0x21;
pub const SPINOR_OP_BE_32K_4B: u8 = 0x5c;
pub const SPINOR_OP_SE_4B: u8 = 0xdc;

pub const SPINOR_OP_READ_1_1_1_DTR: u8 = 0x0d;
pub const SPINOR_OP_READ_1_2_2_DTR: u8 = 0xbd;
pub const SPINOR_OP_READ_1_4_4_DTR: u8 = 0xed;
pub const SPINOR_OP_READ_1_1_1_DTR_4B: u8 = 0x0e;
pub const SPINOR_OP_READ_1_2_2_DTR_4B: u8 = 0xbe;
pub const SPINOR_OP_READ_1_4_4_DTR_4B: u8 = 0xee;
pub const SPINOR_OP_BP: u8 = 0x02;
pub const SPINOR_OP_AAI_WP: u8 = 0xad;
pub const SPINOR_OP_EN4B: u8 = 0xb7;
pub const SPINOR_OP_EX4B: u8 = 0xe9;
pub const SPINOR_OP_BRWR: u8 = 0x17;
pub const SPINOR_OP_RD_EVCR: u8 = 0x65;
pub const SPINOR_OP_WD_EVCR: u8 = 0x61;
pub const SPINOR_OP_ESECR: u8 = 0x44;
pub const SPINOR_OP_PSECR: u8 = 0x42;
pub const SPINOR_OP_RSECR: u8 = 0x48;

pub const SR_WIP: u32 = 1 << 0; pub const SR_WEL: u32 = 1 << 1;
pub const SR_BP0: u32 = 1 << 2; pub const SR_BP1: u32 = 1 << 3;
pub const SR_BP2: u32 = 1 << 4; pub const SR_BP3: u32 = 1 << 5;
pub const SR_TB_BIT5: u32 = 1 << 5; pub const SR_BP3_BIT6: u32 = 1 << 6;
pub const SR_TB_BIT6: u32 = 1 << 6; pub const SR_SRWD: u32 = 1 << 7;
pub const SR_E_ERR: u32 = 1 << 5; pub const SR_P_ERR: u32 = 1 << 6;
pub const SR1_QUAD_EN_BIT6: u32 = 1 << 6;
pub const SR_BP_SHIFT: u32 = 2;
pub const EVCR_QUAD_EN_MICRON: u32 = 1 << 7;
pub const SR2_QUAD_EN_BIT1: u32 = 1 << 1; pub const SR2_LB1: u32 = 1 << 3;
pub const SR2_LB2: u32 = 1 << 4; pub const SR2_LB3: u32 = 1 << 5;
pub const SR2_CMP_BIT6: u32 = 1 << 6; pub const SR2_QUAD_EN_BIT7: u32 = 1 << 7;

pub const SNOR_PROTO_INST_MASK: usize = 0xff << 16;
pub const SNOR_PROTO_INST_SHIFT: usize = 16;
pub const SNOR_PROTO_ADDR_MASK: usize = 0xff << 8;
pub const SNOR_PROTO_ADDR_SHIFT: usize = 8;
pub const SNOR_PROTO_DATA_MASK: usize = 0xff;
pub const SNOR_PROTO_DATA_SHIFT: usize = 0;
pub const SNOR_PROTO_IS_DTR: usize = 1 << 24;
pub const fn snor_proto_inst(n: usize) -> usize { (n << SNOR_PROTO_INST_SHIFT) & SNOR_PROTO_INST_MASK }
pub const fn snor_proto_addr(n: usize) -> usize { (n << SNOR_PROTO_ADDR_SHIFT) & SNOR_PROTO_ADDR_MASK }
pub const fn snor_proto_data(n: usize) -> usize { n & SNOR_PROTO_DATA_MASK }
pub const fn snor_proto_str(i: usize, a: usize, d: usize) -> usize { snor_proto_inst(i) | snor_proto_addr(a) | snor_proto_data(d) }
pub const fn snor_proto_dtr(i: usize, a: usize, d: usize) -> usize { SNOR_PROTO_IS_DTR | snor_proto_str(i, a, d) }

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum spi_nor_protocol {
    SNOR_PROTO_1_1_1 = snor_proto_str(1,1,1), SNOR_PROTO_1_1_2 = snor_proto_str(1,1,2),
    SNOR_PROTO_1_1_4 = snor_proto_str(1,1,4), SNOR_PROTO_1_1_8 = snor_proto_str(1,1,8),
    SNOR_PROTO_1_2_2 = snor_proto_str(1,2,2), SNOR_PROTO_1_4_4 = snor_proto_str(1,4,4),
    SNOR_PROTO_1_8_8 = snor_proto_str(1,8,8), SNOR_PROTO_2_2_2 = snor_proto_str(2,2,2),
    SNOR_PROTO_4_4_4 = snor_proto_str(4,4,4), SNOR_PROTO_8_8_8 = snor_proto_str(8,8,8),
    SNOR_PROTO_1_1_1_DTR = snor_proto_dtr(1,1,1), SNOR_PROTO_1_2_2_DTR = snor_proto_dtr(1,2,2),
    SNOR_PROTO_1_4_4_DTR = snor_proto_dtr(1,4,4), SNOR_PROTO_1_8_8_DTR = snor_proto_dtr(1,8,8),
    SNOR_PROTO_8_8_8_DTR = snor_proto_dtr(8,8,8),
}

#[inline] pub fn spi_nor_protocol_is_dtr(proto: spi_nor_protocol) -> bool { (proto as u32 & SNOR_PROTO_IS_DTR as u32) != 0 }
#[inline] pub fn spi_nor_get_protocol_inst_nbits(proto: spi_nor_protocol) -> u8 { ((proto as usize & SNOR_PROTO_INST_MASK) >> SNOR_PROTO_INST_SHIFT) as u8 }
#[inline] pub fn spi_nor_get_protocol_addr_nbits(proto: spi_nor_protocol) -> u8 { ((proto as usize & SNOR_PROTO_ADDR_MASK) >> SNOR_PROTO_ADDR_SHIFT) as u8 }
#[inline] pub fn spi_nor_get_protocol_data_nbits(proto: spi_nor_protocol) -> u8 { ((proto as usize & SNOR_PROTO_DATA_MASK) >> SNOR_PROTO_DATA_SHIFT) as u8 }
#[inline] pub fn spi_nor_get_protocol_width(proto: spi_nor_protocol) -> u8 { spi_nor_get_protocol_data_nbits(proto) }

#[repr(C)] pub struct spi_nor_hwcaps { pub mask: u32 }
pub const SNOR_HWCAPS_READ_MASK: u32 = 0xffff; pub const SNOR_HWCAPS_READ: u32 = 1<<0; pub const SNOR_HWCAPS_READ_FAST: u32 = 1<<1; pub const SNOR_HWCAPS_READ_1_1_1_DTR: u32 = 1<<2;
pub const SNOR_HWCAPS_READ_DUAL: u32 = 0x78; pub const SNOR_HWCAPS_READ_1_1_2: u32=1<<3; pub const SNOR_HWCAPS_READ_1_2_2: u32=1<<4; pub const SNOR_HWCAPS_READ_2_2_2: u32=1<<5; pub const SNOR_HWCAPS_READ_1_2_2_DTR:u32=1<<6;
pub const SNOR_HWCAPS_READ_QUAD:u32=0x780; pub const SNOR_HWCAPS_READ_1_1_4:u32=1<<7; pub const SNOR_HWCAPS_READ_1_4_4:u32=1<<8; pub const SNOR_HWCAPS_READ_4_4_4:u32=1<<9; pub const SNOR_HWCAPS_READ_1_4_4_DTR:u32=1<<10;
pub const SNOR_HWCAPS_READ_OCTAL:u32=0xf800; pub const SNOR_HWCAPS_READ_1_1_8:u32=1<<11; pub const SNOR_HWCAPS_READ_1_8_8:u32=1<<12; pub const SNOR_HWCAPS_READ_8_8_8:u32=1<<13; pub const SNOR_HWCAPS_READ_1_8_8_DTR:u32=1<<14; pub const SNOR_HWCAPS_READ_8_8_8_DTR:u32=1<<15;
pub const SNOR_HWCAPS_PP_MASK:u32=0xff0000; pub const SNOR_HWCAPS_PP:u32=1<<16; pub const SNOR_HWCAPS_PP_QUAD:u32=0xe0000; pub const SNOR_HWCAPS_PP_1_1_4:u32=1<<17; pub const SNOR_HWCAPS_PP_1_4_4:u32=1<<18; pub const SNOR_HWCAPS_PP_4_4_4:u32=1<<19; pub const SNOR_HWCAPS_PP_OCTAL:u32=0xf00000; pub const SNOR_HWCAPS_PP_1_1_8:u32=1<<20; pub const SNOR_HWCAPS_PP_1_8_8:u32=1<<21; pub const SNOR_HWCAPS_PP_8_8_8:u32=1<<22; pub const SNOR_HWCAPS_PP_8_8_8_DTR:u32=1<<23;
pub const SNOR_HWCAPS_X_X_X:u32=SNOR_HWCAPS_READ_2_2_2|SNOR_HWCAPS_READ_4_4_4|SNOR_HWCAPS_READ_8_8_8|SNOR_HWCAPS_PP_4_4_4|SNOR_HWCAPS_PP_8_8_8;
pub const SNOR_HWCAPS_X_X_X_DTR:u32=SNOR_HWCAPS_READ_8_8_8_DTR|SNOR_HWCAPS_PP_8_8_8_DTR;
pub const SNOR_HWCAPS_DTR:u32=SNOR_HWCAPS_READ_1_1_1_DTR|SNOR_HWCAPS_READ_1_2_2_DTR|SNOR_HWCAPS_READ_1_4_4_DTR|SNOR_HWCAPS_READ_1_8_8_DTR|SNOR_HWCAPS_READ_8_8_8_DTR;
pub const SNOR_HWCAPS_ALL:u32=SNOR_HWCAPS_READ_MASK|SNOR_HWCAPS_PP_MASK;

#[repr(C)] pub struct spi_nor_rww { pub wait: wait_queue_head_t, pub ongoing_io: bool, pub ongoing_rd: bool, pub ongoing_pe: bool, pub used_banks: u32 }
#[repr(C)] pub struct spi_nor {
    pub mtd: mtd_info, pub lock: mutex, pub rww: spi_nor_rww,
    pub dev: *mut device, pub spimem: *mut spi_mem, pub bouncebuf: *mut u8, pub bouncebuf_size: usize,
    pub id: *mut u8, pub info: *const flash_info, pub manufacturer: *const spi_nor_manufacturer,
    pub addr_nbytes: u8, pub erase_opcode: u8, pub read_opcode: u8, pub read_dummy: u8, pub program_opcode: u8,
    pub read_proto: spi_nor_protocol, pub write_proto: spi_nor_protocol, pub reg_proto: spi_nor_protocol,
    pub sst_write_second: bool, pub flags: u32, pub cmd_ext_type: spi_nor_cmd_ext,
    pub sfdp: *mut sfdp, pub debugfs_root: *mut dentry, pub dfs_sr_cache: [u8; 2],
    pub controller_ops: *const spi_nor_controller_ops, pub params: *mut spi_nor_flash_parameter,
    pub dirmap: spi_nor_dirmap, pub priv_: *mut core::ffi::c_void,
}
#[repr(C)] pub struct spi_nor_dirmap { pub rdesc: *mut spi_mem_dirmap_desc, pub wdesc: *mut spi_mem_dirmap_desc }
// Types supplied by the included Linux headers.
extern "C" {
    type wait_queue_head_t; type mtd_info; type mutex; type device; type spi_mem; type sfdp; type dentry;
    type spi_mem_dirmap_desc; type device_node;
    fn mtd_set_of_node(mtd: *mut mtd_info, np: *mut device_node);
    fn mtd_get_of_node(mtd: *mut mtd_info) -> *mut device_node;
}
#[inline] pub unsafe fn spi_nor_set_flash_node(nor: *mut spi_nor, np: *mut device_node) { mtd_set_of_node(&mut (*nor).mtd, np); }
#[inline] pub unsafe fn spi_nor_get_flash_node(nor: *mut spi_nor) -> *mut device_node { mtd_get_of_node(&mut (*nor).mtd) }
#[repr(C)] pub struct spi_nor_controller_ops { pub prepare: Option<unsafe extern "C" fn(*mut spi_nor)->i32>, pub unprepare: Option<unsafe extern "C" fn(*mut spi_nor)>, pub read_reg: Option<unsafe extern "C" fn(*mut spi_nor,u8,*mut u8,usize)->i32>, pub write_reg: Option<unsafe extern "C" fn(*mut spi_nor,u8,*const u8,usize)->i32>, pub read: Option<unsafe extern "C" fn(*mut spi_nor,i64,usize,*mut u8)->isize>, pub write: Option<unsafe extern "C" fn(*mut spi_nor,i64,usize,*const u8)->isize>, pub erase: Option<unsafe extern "C" fn(*mut spi_nor,i64)->i32> }
#[repr(u32)] pub enum spi_nor_cmd_ext { SPI_NOR_EXT_NONE=0, SPI_NOR_EXT_REPEAT, SPI_NOR_EXT_INVERT, SPI_NOR_EXT_HEX }
pub enum flash_info {} pub enum spi_nor_manufacturer {} pub enum spi_nor_flash_parameter {}

extern "C" { pub fn spi_nor_scan(nor: *mut spi_nor, name: *const i8, hwcaps: *const spi_nor_hwcaps) -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
