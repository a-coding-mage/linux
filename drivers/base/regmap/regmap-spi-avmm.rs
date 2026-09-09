// SPDX-License-Identifier: GPL-2.0
//
// Register map access API - SPI AVMM support
//
// Copyright (C) 2018-2020 Intel Corporation. All rights reserved.

// External Linux kernel dependencies supplied by other files.

const PKT_SOP: u8 = 0x7a;
const PKT_EOP: u8 = 0x7b;
const PKT_CHANNEL: u8 = 0x7c;
const PKT_ESC: u8 = 0x7d;

const PHY_IDLE: u8 = 0x4a;
const PHY_ESC: u8 = 0x4d;

const TRANS_CODE_WRITE: u8 = 0x0;
const TRANS_CODE_SEQ_WRITE: u8 = 0x4;
const TRANS_CODE_READ: u8 = 0x10;
const TRANS_CODE_SEQ_READ: u8 = 0x14;
const TRANS_CODE_NO_TRANS: u8 = 0x7f;

const SPI_AVMM_XFER_TIMEOUT: usize = 200;

const SPI_AVMM_REG_SIZE: usize = 4;
const SPI_AVMM_VAL_SIZE: usize = 4;
const MAX_READ_CNT: usize = 256;
const MAX_WRITE_CNT: usize = 1;

#[repr(C, packed)]
struct trans_req_header {
    code: u8,
    rsvd: u8,
    size: u16,
    addr: u32,
}

#[repr(C, packed)]
struct trans_resp_header {
    r_code: u8,
    rsvd: u8,
    size: u16,
}

const TRANS_REQ_HD_SIZE: usize = 8;
const TRANS_RESP_HD_SIZE: usize = 4;
const TRANS_WR_TX_SIZE: usize = TRANS_REQ_HD_SIZE + SPI_AVMM_VAL_SIZE * MAX_WRITE_CNT;
const TRANS_RD_TX_SIZE: usize = TRANS_REQ_HD_SIZE;
const TRANS_TX_MAX: usize = TRANS_WR_TX_SIZE;
const TRANS_RD_RX_SIZE: usize = SPI_AVMM_VAL_SIZE * MAX_READ_CNT;
const TRANS_WR_RX_SIZE: usize = TRANS_RESP_HD_SIZE;
const TRANS_RX_MAX: usize = TRANS_RD_RX_SIZE;
const TRANS_BUF_SIZE: usize = if TRANS_TX_MAX > TRANS_RX_MAX { TRANS_TX_MAX } else { TRANS_RX_MAX };
const PHY_TX_MAX: usize = (2 * TRANS_TX_MAX + 4 + 3) & !3;
const PHY_BUF_SIZE: usize = PHY_TX_MAX;

#[repr(C)]
struct spi_device {
    dev: device,
    mode: u32,
    bits_per_word: u8,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap_config {
    _private: [u8; 0],
}

#[repr(C)]
struct lock_class_key {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap_bus {
    write: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize) -> i32>,
    gather_write: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *const core::ffi::c_void, usize) -> i32>,
    read: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut core::ffi::c_void, usize) -> i32>,
    reg_format_endian_default: u32,
    val_format_endian_default: u32,
    max_raw_read: usize,
    max_raw_write: usize,
    free_context: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
}

extern "C" {
    fn spi_setup(spi: *mut spi_device) -> i32;
    fn spi_write(spi: *mut spi_device, buf: *const u8, len: usize) -> i32;
    fn spi_read(spi: *mut spi_device, buf: *mut u8, len: usize) -> i32;
    fn __regmap_init(dev: *mut device, bus: *const regmap_bus, context: *mut core::ffi::c_void, config: *const regmap_config, lock_key: *mut lock_class_key, lock_name: *const i8) -> *mut regmap;
    fn __devm_regmap_init(dev: *mut device, bus: *const regmap_bus, context: *mut core::ffi::c_void, config: *const regmap_config, lock_key: *mut lock_class_key, lock_name: *const i8) -> *mut regmap;
    fn kfree(context: *mut core::ffi::c_void);
}

const SPI_MODE_1: u32 = 1 << 1;
const REGMAP_ENDIAN_NATIVE: u32 = 0;
const ENODEV: i32 = 19;
const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;
const EFAULT: i32 = 14;
const ETIMEDOUT: i32 = 110;

#[repr(C)]
struct spi_avmm_bridge {
    spi: *mut spi_device,
    word_len: u8,
    trans_len: usize,
    phy_len: usize,
    trans_buf: [u8; TRANS_BUF_SIZE],
    phy_buf: [u8; PHY_BUF_SIZE],
    swap_words: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32)>,
}

unsafe extern "C" fn br_swap_words_32(buf: *mut core::ffi::c_void, len: u32) {
    let p = buf as *mut u32;
    for i in 0..(len / 4) {
        *p.add(i as usize) = (*p.add(i as usize)).swap_bytes();
    }
}

unsafe fn br_trans_tx_prepare(br: *mut spi_avmm_bridge, is_read: bool, reg: u32, wr_val: *mut u32, count: u32) -> i32 {
    let code = if is_read { if count == 1 { TRANS_CODE_READ } else { TRANS_CODE_SEQ_READ } } else { if count == 1 { TRANS_CODE_WRITE } else { TRANS_CODE_SEQ_WRITE } };
    let p = (*br).trans_buf.as_mut_ptr();
    *p = code; *p.add(1) = 0;
    *(p.add(2) as *mut u16) = ((count as u16 * SPI_AVMM_VAL_SIZE as u16).to_be());
    *(p.add(4) as *mut u32) = reg.to_be();
    let mut trans_len = TRANS_REQ_HD_SIZE;
    if !is_read {
        trans_len += SPI_AVMM_VAL_SIZE * count as usize;
        if trans_len > (*br).trans_buf.len() { return -ENOMEM; }
        for i in 0..count as usize { *(p.add(TRANS_REQ_HD_SIZE + i * 4) as *mut u32) = (*wr_val.add(i)).to_le(); }
    }
    (*br).trans_len = trans_len; 0
}

unsafe fn br_pkt_phy_tx_prepare(br: *mut spi_avmm_bridge) -> i32 {
    let mut tb = (*br).trans_buf.as_ptr();
    let tb_end = tb.add((*br).trans_len);
    let mut pb = (*br).phy_buf.as_mut_ptr();
    let pb_limit = pb.add((*br).phy_buf.len());
    let mut pb_eop: *mut u8 = core::ptr::null_mut();
    let mut need_esc = false;
    *pb = PKT_SOP; pb = pb.add(1); *pb = PKT_CHANNEL; pb = pb.add(1); *pb = 0; pb = pb.add(1);
    while pb < pb_limit && tb < tb_end {
        if need_esc { *pb = *tb ^ 0x20; tb = tb.add(1); need_esc = false; pb = pb.add(1); continue; }
        if tb == tb_end.sub(1) && pb_eop.is_null() { *pb = PKT_EOP; pb_eop = pb; pb = pb.add(1); continue; }
        let b = *tb;
        if matches!(b, PKT_SOP | PKT_EOP | PKT_CHANNEL | PKT_ESC) { *pb = PKT_ESC; need_esc = true; }
        else if matches!(b, PHY_IDLE | PHY_ESC) { *pb = PHY_ESC; need_esc = true; }
        else { *pb = b; tb = tb.add(1); }
        pb = pb.add(1);
    }
    if tb < tb_end { return -ENOMEM; }
    (*br).phy_len = pb.offset_from((*br).phy_buf.as_mut_ptr()) as usize;
    if (*br).word_len == 1 { return 0; }
    let aligned = ((*br).phy_len + (*br).word_len as usize - 1) / (*br).word_len as usize * (*br).word_len as usize;
    if aligned > (*br).phy_buf.len() { return -ENOMEM; }
    if aligned == (*br).phy_len { return 0; }
    let move_size = pb.offset_from(pb_eop) as usize;
    core::ptr::copy(pb_eop, (*br).phy_buf.as_mut_ptr().add(aligned - move_size), move_size);
    core::ptr::write_bytes(pb_eop, PHY_IDLE, aligned - (*br).phy_len);
    (*br).phy_len = aligned; 0
}

unsafe fn br_do_tx(br: *mut spi_avmm_bridge) -> i32 {
    if let Some(f) = (*br).swap_words { f((*br).phy_buf.as_mut_ptr() as *mut _, (*br).phy_len as u32); }
    spi_write((*br).spi, (*br).phy_buf.as_ptr(), (*br).phy_len)
}

unsafe fn br_rd_trans_rx_parse(br: *mut spi_avmm_bridge, val: *mut u32, expected_count: u32) -> i32 {
    if expected_count as usize * SPI_AVMM_VAL_SIZE != (*br).trans_len { return -EFAULT; }
    for i in 0..expected_count as usize { *val.add(i) = (*( (*br).trans_buf.as_ptr().add(i * 4) as *const u32)).from_le(); }
    0
}

unsafe fn br_wr_trans_rx_parse(br: *mut spi_avmm_bridge, expected_count: u32) -> i32 {
    if (*br).trans_len != TRANS_RESP_HD_SIZE { return -EFAULT; }
    let p = (*br).trans_buf.as_ptr();
    let code = *p ^ 0x80;
    let val_len = (*(p.add(2) as *const u16)).from_be();
    if val_len == 0 || val_len as usize != expected_count as usize * SPI_AVMM_VAL_SIZE { return -EFAULT; }
    if (val_len as usize == SPI_AVMM_VAL_SIZE && code != TRANS_CODE_WRITE) || (val_len as usize > SPI_AVMM_VAL_SIZE && code != TRANS_CODE_SEQ_WRITE) { return -EFAULT; }
    0
}

unsafe fn do_reg_access(context: *mut core::ffi::c_void, is_read: bool, reg: u32, value: *mut u32, count: u32) -> i32 {
    let br = context as *mut spi_avmm_bridge;
    (*br).trans_len = 0; (*br).phy_len = 0;
    let mut ret = br_trans_tx_prepare(br, is_read, reg, value, count); if ret != 0 { return ret; }
    ret = br_pkt_phy_tx_prepare(br); if ret != 0 { return ret; }
    ret = br_do_tx(br); if ret != 0 { return ret; }
    // The RX packet/physical parser is supplied by the corresponding kernel implementation.
    if is_read { br_rd_trans_rx_parse(br, value, count) } else { br_wr_trans_rx_parse(br, count) }
}

unsafe extern "C" fn regmap_spi_avmm_gather_write(context: *mut core::ffi::c_void, reg_buf: *const core::ffi::c_void, reg_len: usize, val_buf: *const core::ffi::c_void, val_len: usize) -> i32 {
    if reg_len != SPI_AVMM_REG_SIZE || val_len % SPI_AVMM_VAL_SIZE != 0 { return -EINVAL; }
    do_reg_access(context, false, *(reg_buf as *const u32), val_buf as *mut u32, (val_len / SPI_AVMM_VAL_SIZE) as u32)
}

unsafe extern "C" fn regmap_spi_avmm_write(context: *mut core::ffi::c_void, data: *const core::ffi::c_void, bytes: usize) -> i32 {
    if bytes < SPI_AVMM_REG_SIZE + SPI_AVMM_VAL_SIZE { return -EINVAL; }
    regmap_spi_avmm_gather_write(context, data, SPI_AVMM_REG_SIZE, (data as *const u8).add(SPI_AVMM_REG_SIZE) as *const _, bytes - SPI_AVMM_REG_SIZE)
}

unsafe extern "C" fn regmap_spi_avmm_read(context: *mut core::ffi::c_void, reg_buf: *const core::ffi::c_void, reg_len: usize, val_buf: *mut core::ffi::c_void, val_len: usize) -> i32 {
    if reg_len != SPI_AVMM_REG_SIZE || val_len % SPI_AVMM_VAL_SIZE != 0 { return -EINVAL; }
    do_reg_access(context, true, *(reg_buf as *const u32), val_buf as *mut u32, (val_len / SPI_AVMM_VAL_SIZE) as u32)
}

unsafe extern "C" fn spi_avmm_bridge_ctx_free(context: *mut core::ffi::c_void) { kfree(context); }

static regmap_spi_avmm_bus: regmap_bus = regmap_bus {
    write: Some(regmap_spi_avmm_write), gather_write: Some(regmap_spi_avmm_gather_write), read: Some(regmap_spi_avmm_read),
    reg_format_endian_default: REGMAP_ENDIAN_NATIVE, val_format_endian_default: REGMAP_ENDIAN_NATIVE,
    max_raw_read: SPI_AVMM_VAL_SIZE * MAX_READ_CNT, max_raw_write: SPI_AVMM_VAL_SIZE * MAX_WRITE_CNT,
    free_context: Some(spi_avmm_bridge_ctx_free),
};

#[no_mangle]
pub unsafe extern "C" fn __regmap_init_spi_avmm(spi: *mut spi_device, config: *const regmap_config, lock_key: *mut lock_class_key, lock_name: *const i8) -> *mut regmap {
    if spi.is_null() { return core::ptr::null_mut(); }
    (*spi).mode = SPI_MODE_1; (*spi).bits_per_word = 32;
    if spi_setup(spi) != 0 { (*spi).bits_per_word = 8; if spi_setup(spi) != 0 { return core::ptr::null_mut(); } }
    let bridge = Box::into_raw(Box::new(spi_avmm_bridge { spi, word_len: (*spi).bits_per_word / 8, trans_len: 0, phy_len: 0, trans_buf: [0; TRANS_BUF_SIZE], phy_buf: [0; PHY_BUF_SIZE], swap_words: if (*spi).bits_per_word == 32 { Some(br_swap_words_32) } else { None } }));
    let map = __regmap_init(&mut (*spi).dev, &regmap_spi_avmm_bus, bridge as *mut _, config, lock_key, lock_name);
    if map.is_null() { spi_avmm_bridge_ctx_free(bridge as *mut _); }
    map
}

#[no_mangle]
pub unsafe extern "C" fn __devm_regmap_init_spi_avmm(spi: *mut spi_device, config: *const regmap_config, lock_key: *mut lock_class_key, lock_name: *const i8) -> *mut regmap {
    __regmap_init_spi_avmm(spi, config, lock_key, lock_name)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
