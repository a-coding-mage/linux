// SPDX-License-Identifier: GPL-2.0
/*
 * Lattice MachXO2 Slave SPI Driver
 *
 * Manage Lattice FPGA firmware that is loaded over SPI using
 * the slave serial configuration interface.
 *
 * Copyright (C) 2018 Paolo Pisati <p.pisati@gmail.com>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const IDCODE_PUB: [u8; 4] = [0xe0, 0x00, 0x00, 0x00];
const ISC_ENABLE: [u8; 4] = [0xc6, 0x08, 0x00, 0x00];
const ISC_ERASE: [u8; 4] = [0x0e, 0x04, 0x00, 0x00];
const ISC_PROGRAMDONE: [u8; 4] = [0x5e, 0x00, 0x00, 0x00];
const LSC_INITADDRESS: [u8; 4] = [0x46, 0x00, 0x00, 0x00];
const LSC_PROGINCRNV: [u8; 4] = [0x70, 0x00, 0x00, 0x01];
const LSC_READ_STATUS: [u8; 4] = [0x3c, 0x00, 0x00, 0x00];
const LSC_REFRESH: [u8; 4] = [0x79, 0x00, 0x00, 0x00];

const MACHXO2_MAX_SPEED: u32 = 66000000;
const MACHXO2_LOW_DELAY_USEC: u32 = 5;
const MACHXO2_HIGH_DELAY_USEC: u32 = 200;
const MACHXO2_REFRESH_USEC: u32 = 4800;
const MACHXO2_MAX_BUSY_LOOP: i32 = 128;
const MACHXO2_MAX_REFRESH_LOOP: i32 = 16;
const MACHXO2_PAGE_SIZE: usize = 16;
const MACHXO2_BUF_SIZE: usize = MACHXO2_PAGE_SIZE + 4;

const BUSY: usize = 12;
const DONE: usize = 8;
const DVER: usize = 27;
const ENAB: usize = 9;
const ERRBITS: usize = 23;
const ERRMASK: usize = 7;
const FAIL: usize = 13;

const ENOERR: u8 = 0;
const EID: u8 = 1;
const ECMD: u8 = 2;
const ECRC: u8 = 3;
const EPREAM: u8 = 4;
const EABRT: u8 = 5;
const EOVERFL: u8 = 6;
const ESDMEOF: u8 = 7;

#[inline]
unsafe fn get_err(status: *mut c_ulong) -> u8 {
    ((*status >> ERRBITS) & ERRMASK as c_ulong) as u8
}

unsafe fn get_status(spi: *mut spi_device, status: *mut c_ulong) -> c_int {
    let mut msg: spi_message = core::mem::zeroed();
    let mut rx: spi_transfer = core::mem::zeroed();
    let mut tx: spi_transfer = core::mem::zeroed();
    tx.tx_buf = LSC_READ_STATUS.as_ptr() as *const c_void;
    tx.len = LSC_READ_STATUS.len();
    rx.rx_buf = status as *mut c_void;
    rx.len = 4;
    spi_message_init(&mut msg);
    spi_message_add_tail(&mut tx, &mut msg);
    spi_message_add_tail(&mut rx, &mut msg);
    let ret = spi_sync(spi, &mut msg);
    if ret != 0 { return ret; }
    *status = u32::from_be(*status as u32) as c_ulong;
    0
}

#[cfg(feature = "DEBUG")]
unsafe fn get_err_string(err: u8) -> *const c_char {
    match err {
        ENOERR => b"No Error\0".as_ptr() as *const c_char,
        EID => b"ID ERR\0".as_ptr() as *const c_char,
        ECMD => b"CMD ERR\0".as_ptr() as *const c_char,
        ECRC => b"CRC ERR\0".as_ptr() as *const c_char,
        EPREAM => b"Preamble ERR\0".as_ptr() as *const c_char,
        EABRT => b"Abort ERR\0".as_ptr() as *const c_char,
        EOVERFL => b"Overflow ERR\0".as_ptr() as *const c_char,
        ESDMEOF => b"SDM EOF\0".as_ptr() as *const c_char,
        _ => b"Default switch case\0".as_ptr() as *const c_char,
    }
}

unsafe fn dump_status_reg(status: *mut c_ulong) {
    #[cfg(feature = "DEBUG")]
    {
        let _ = (status, DVER, ENAB, FAIL, get_err(status));
    }
}

unsafe fn wait_until_not_busy(spi: *mut spi_device) -> c_int {
    let mut status: c_ulong = 0;
    let mut loop_: c_int = 0;
    loop {
        let ret = get_status(spi, &mut status);
        if ret != 0 { return ret; }
        loop_ += 1;
        if loop_ >= MACHXO2_MAX_BUSY_LOOP { return -EBUSY; }
        if status & (1 << BUSY) == 0 { break; }
    }
    0
}

unsafe fn machxo2_cleanup(mgr: *mut fpga_manager) -> c_int {
    let spi = (*mgr).priv_ as *mut spi_device; let mut msg: spi_message = core::mem::zeroed(); let mut tx: [spi_transfer; 2] = core::mem::zeroed();
    tx[0].tx_buf = ISC_ERASE.as_ptr() as *const c_void; tx[0].len = 4; spi_message_init(&mut msg); spi_message_add_tail(&mut tx[0], &mut msg);
    let mut ret = spi_sync(spi, &mut msg); if ret != 0 { return ret; } ret = wait_until_not_busy(spi); if ret != 0 { return ret; }
    spi_message_init(&mut msg); tx[1].tx_buf = LSC_REFRESH.as_ptr() as *const c_void; tx[1].len = 4; spi_message_add_tail(&mut tx[1], &mut msg); spi_sync(spi, &mut msg)
}
unsafe fn machxo2_spi_state(mgr: *mut fpga_manager) -> fpga_mgr_states { let spi = (*mgr).priv_ as *mut spi_device; let mut status = 0; get_status(spi, &mut status); if status & (1 << BUSY) == 0 && status & (1 << DONE) != 0 && get_err(&mut status) == ENOERR { FPGA_MGR_STATE_OPERATING } else { FPGA_MGR_STATE_UNKNOWN } }
unsafe fn machxo2_write_init(mgr: *mut fpga_manager, info: *mut fpga_image_info, _buf: *const c_char, _count: usize) -> c_int { if (*info).flags & FPGA_MGR_PARTIAL_RECONFIG != 0 { return -ENOTSUPP; } let spi = (*mgr).priv_ as *mut spi_device; let mut msg: spi_message = core::mem::zeroed(); let mut tx: [spi_transfer; 3] = core::mem::zeroed(); tx[0].tx_buf = ISC_ENABLE.as_ptr() as *const c_void; tx[0].len = 4; spi_message_init(&mut msg); spi_message_add_tail(&mut tx[0], &mut msg); tx[1].tx_buf = ISC_ERASE.as_ptr() as *const c_void; tx[1].len = 4; spi_message_add_tail(&mut tx[1], &mut msg); let mut ret = spi_sync(spi, &mut msg); if ret != 0 { return ret; } ret = wait_until_not_busy(spi); if ret != 0 { return ret; } let mut status = 0; get_status(spi, &mut status); if status & (1 << FAIL) != 0 { return -EINVAL; } spi_message_init(&mut msg); tx[2].tx_buf = LSC_INITADDRESS.as_ptr() as *const c_void; tx[2].len = 4; spi_message_add_tail(&mut tx[2], &mut msg); spi_sync(spi, &mut msg) }
unsafe fn machxo2_write(mgr: *mut fpga_manager, buf: *const c_char, count: usize) -> c_int { if count % MACHXO2_PAGE_SIZE != 0 { return -EINVAL; } let spi = (*mgr).priv_ as *mut spi_device; let mut payload = [0u8; MACHXO2_BUF_SIZE]; payload[..4].copy_from_slice(&LSC_PROGINCRNV); let mut i = 0; while i < count { core::ptr::copy_nonoverlapping(buf.add(i) as *const u8, payload[4..].as_mut_ptr(), MACHXO2_PAGE_SIZE); let mut msg: spi_message = core::mem::zeroed(); let mut tx: spi_transfer = core::mem::zeroed(); tx.tx_buf = payload.as_ptr() as *const c_void; tx.len = MACHXO2_BUF_SIZE; spi_message_init(&mut msg); spi_message_add_tail(&mut tx, &mut msg); let ret = spi_sync(spi, &mut msg); if ret != 0 { return ret; } i += MACHXO2_PAGE_SIZE; } 0 }
unsafe fn machxo2_write_complete(mgr: *mut fpga_manager, _info: *mut fpga_image_info) -> c_int { let spi = (*mgr).priv_ as *mut spi_device; let mut msg: spi_message = core::mem::zeroed(); let mut tx: spi_transfer = core::mem::zeroed(); tx.tx_buf = ISC_PROGRAMDONE.as_ptr() as *const c_void; tx.len = 4; spi_message_init(&mut msg); spi_message_add_tail(&mut tx, &mut msg); let mut ret = spi_sync(spi, &mut msg); if ret != 0 { return ret; } ret = wait_until_not_busy(spi); if ret != 0 { return ret; } let mut status = 0; get_status(spi, &mut status); if status & (1 << DONE) == 0 { machxo2_cleanup(mgr); return -EINVAL; } 0 }

// External kernel types and functions referenced by the source file.
type c_int = i32;
type c_ulong = usize;
type c_char = i8;
type c_void = core::ffi::c_void;
const EBUSY: c_int = 16;
#[repr(C)] pub struct spi_device { pub max_speed_hz: u32, pub dev: device, pub priv_: *mut c_void }
#[repr(C)] pub struct device;
#[repr(C)] pub struct fpga_manager { pub priv_: *mut c_void, pub dev: device }
#[repr(C)] pub struct fpga_image_info { pub flags: u32 }
#[repr(C)] pub struct spi_message;
#[repr(C)] pub struct spi_transfer { pub tx_buf: *const c_void, pub rx_buf: *mut c_void, pub len: usize }
#[repr(C)] pub enum fpga_mgr_states { FPGA_MGR_STATE_UNKNOWN, FPGA_MGR_STATE_OPERATING }
extern "C" { fn spi_message_init(msg: *mut spi_message); fn spi_message_add_tail(xfer: *mut spi_transfer, msg: *mut spi_message); fn spi_sync(spi: *mut spi_device, msg: *mut spi_message) -> c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
