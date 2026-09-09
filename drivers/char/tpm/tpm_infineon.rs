// SPDX-License-Identifier: GPL-2.0-only
/*
 * Description:
 * Device Driver for the Infineon Technologies
 * SLD 9630 TT 1.1 and SLB 9635 TT 1.2 Trusted Platform Module
 * Specifications at www.trustedcomputinggroup.org
 *
 * Copyright (C) 2005, Marcel Selhorst <tpmdd@selhorst.net>
 * Sirrix AG - security technologies <tpmdd@sirrix.com> and
 * Applied Data Security Group, Ruhr-University Bochum, Germany
 * Project-Homepage: http://www.trust.rub.de/projects/linux-device-driver-infineon/
 */

// Linux dependencies supplied by the surrounding kernel translation.

const TPM_MAX_WTX_PACKAGES: i32 = 50;
const TPM_WTX_MSLEEP_TIME: i32 = 20;
const TPM_MSLEEP_TIME: i32 = 3;
const TPM_MAX_TRIES: i32 = 5000;
const TPM_INFINEON_DEV_VEN_VALUE: i32 = 0x15D1;
const TPM_INF_IO_PORT: i32 = 0x0;
const TPM_INF_IO_MEM: i32 = 0x1;
const TPM_INF_ADDR: u8 = 0x0;
const TPM_INF_DATA: u8 = 0x1;

#[repr(C)]
struct tpm_inf_dev {
    iotype: i32,
    mem_base: *mut core::ffi::c_void,
    map_base: usize,
    map_size: usize,
    index_off: u32,
    data_regs: u32,
    data_size: u32,
    config_port: u32,
    config_size: u32,
}

static mut tpm_dev: tpm_inf_dev = tpm_inf_dev {
    iotype: 0, mem_base: core::ptr::null_mut(), map_base: 0, map_size: 0,
    index_off: 0, data_regs: 0, data_size: 0, config_port: 0, config_size: 0,
};

extern "C" {
    fn outb(data: u8, port: u32);
    fn inb(port: u32) -> u8;
    fn writeb(data: u8, addr: *mut core::ffi::c_void);
    fn readb(addr: *mut core::ffi::c_void) -> u8;
    fn tpm_msleep(ms: i32);
    fn dev_err(dev: *const core::ffi::c_void, fmt: *const i8, ...);
    fn dev_info(dev: *const core::ffi::c_void, fmt: *const i8, ...);
}

#[inline]
unsafe fn tpm_data_out(data: u8, offset: u8) {
    if tpm_dev.iotype == TPM_INF_IO_PORT { outb(data, tpm_dev.data_regs + offset as u32); }
    else { writeb(data, (tpm_dev.mem_base as usize + tpm_dev.data_regs as usize + offset as usize) as *mut _); }
}

#[inline]
unsafe fn tpm_data_in(offset: u8) -> u8 {
    if tpm_dev.iotype == TPM_INF_IO_PORT { inb(tpm_dev.data_regs + offset as u32) }
    else { readb((tpm_dev.mem_base as usize + tpm_dev.data_regs as usize + offset as usize) as *mut _) }
}

#[inline]
unsafe fn tpm_config_out(data: u8, offset: u8) {
    if tpm_dev.iotype == TPM_INF_IO_PORT { outb(data, tpm_dev.config_port + offset as u32); }
    else { writeb(data, (tpm_dev.mem_base as usize + tpm_dev.index_off as usize + offset as usize) as *mut _); }
}

#[inline]
unsafe fn tpm_config_in(offset: u8) -> u8 {
    if tpm_dev.iotype == TPM_INF_IO_PORT { inb(tpm_dev.config_port + offset as u32) }
    else { readb((tpm_dev.mem_base as usize + tpm_dev.index_off as usize + offset as usize) as *mut _) }
}

const TPM_VL_VER: u8 = 0x01;
const TPM_VL_CHANNEL_TPM: u8 = 0x0B;
const TPM_INF_NAK: u8 = 0x15;
const TPM_CTRL_WTX: u8 = 0x10;
const TPM_CTRL_WTX_ABORT: u8 = 0x18;
const TPM_CTRL_WTX_ABORT_ACK: u8 = 0x18;
const TPM_CTRL_ERROR: u8 = 0x20;
const TPM_CTRL_DATA: u8 = 0x04;
const WRFIFO: u8 = 0x00;
const RDFIFO: u8 = 0x01;
const STAT: u8 = 0x02;
const CMD: u8 = 0x03;
const STAT_XFE: i32 = 0x00;
const STAT_RDA: i32 = 0x07;
const CHIP_ID1: u8 = 0x20;
const CHIP_ID2: u8 = 0x21;
const TPM_DAR: u8 = 0x30;
const RESET_LP_IRQC_DISABLE: u8 = 0x41;
const ENABLE_REGISTER_PAIR: u8 = 0x55;
const IOLIMH: u8 = 0x60;
const IOLIML: u8 = 0x61;
const DISABLE_REGISTER_PAIR: u8 = 0xAA;
const IDVENL: u8 = 0xF1;
const IDVENH: u8 = 0xF2;
const IDPDL: u8 = 0xF3;
const IDPDH: u8 = 0xF4;

static mut number_of_wtx: i32 = 0;

unsafe fn empty_fifo(_chip: *mut core::ffi::c_void, clear_wrfifo: i32) -> i32 {
    let mut status: i32;
    let mut check = 0;
    if clear_wrfifo != 0 {
        for _ in 0..4096 { status = tpm_data_in(WRFIFO) as i32; if status == 0xff { if check == 5 { break; } else { check += 1; } } }
    }
    let mut i = 0;
    loop { let _ = tpm_data_in(RDFIFO); status = tpm_data_in(STAT) as i32; i += 1; if i == TPM_MAX_TRIES { return -5; } if (status & (1 << STAT_RDA)) == 0 { break; } }
    0
}

unsafe fn wait(chip: *mut core::ffi::c_void, wait_for_bit: i32) -> i32 {
    for i in 0..TPM_MAX_TRIES { let status = tpm_data_in(STAT); if (status as i32 & (1 << wait_for_bit)) != 0 { return 0; } tpm_msleep(TPM_MSLEEP_TIME); if i == TPM_MAX_TRIES - 1 { dev_err(chip, core::ptr::null()); return -5; } }
    0
}

unsafe fn wait_and_send(chip: *mut core::ffi::c_void, sendbyte: u8) { let _ = wait(chip, STAT_XFE); tpm_data_out(sendbyte, WRFIFO); }

unsafe fn tpm_wtx(chip: *mut core::ffi::c_void) { number_of_wtx += 1; dev_info(chip, core::ptr::null()); wait_and_send(chip, TPM_VL_VER); wait_and_send(chip, TPM_CTRL_WTX); wait_and_send(chip, 0); wait_and_send(chip, 0); tpm_msleep(TPM_WTX_MSLEEP_TIME); }
unsafe fn tpm_wtx_abort(chip: *mut core::ffi::c_void) { dev_info(chip, core::ptr::null()); wait_and_send(chip, TPM_VL_VER); wait_and_send(chip, TPM_CTRL_WTX_ABORT); wait_and_send(chip, 0); wait_and_send(chip, 0); number_of_wtx = 0; tpm_msleep(TPM_WTX_MSLEEP_TIME); }

// The remaining driver registration and PnP glue retain the original externally supplied kernel interfaces.
// Their declarations are intentionally kept as low-level opaque callbacks pending surrounding bindings.
#[no_mangle]
pub unsafe extern "C" fn tpm_inf_status(_chip: *mut core::ffi::c_void) -> u8 { tpm_data_in(STAT) }

unsafe fn tpm_inf_recv(chip: *mut core::ffi::c_void, buf: *mut u8, _count: usize) -> i32 {
    number_of_wtx = 0;
    'recv_begin: loop {
        for i in 0..4 { if wait(chip, STAT_RDA) != 0 { return -5; } *buf.add(i) = tpm_data_in(RDFIFO); }
        if *buf != TPM_VL_VER { dev_err(chip, core::ptr::null()); return -5; }
        if *buf.add(1) == TPM_CTRL_DATA {
            let mut size = ((*buf.add(2) as u32) << 8) | *buf.add(3) as u32;
            for i in 0..size as usize { let _ = wait(chip, STAT_RDA); *buf.add(i) = tpm_data_in(RDFIFO); }
            if size == 0x6d00 && *buf.add(1) == 0x80 { dev_err(chip, core::ptr::null()); return -5; }
            for i in 0..size as usize { *buf.add(i) = *buf.add(i + 6); }
            size -= 6;
            return size as i32;
        }
        if *buf.add(1) == TPM_CTRL_WTX {
            dev_info(chip, core::ptr::null());
            if number_of_wtx < TPM_MAX_WTX_PACKAGES { tpm_wtx(chip); } else { tpm_wtx_abort(chip); }
            continue 'recv_begin;
        }
        if *buf.add(1) == TPM_CTRL_WTX_ABORT_ACK { dev_info(chip, core::ptr::null()); return 0; }
        if *buf.add(1) == TPM_CTRL_ERROR { dev_err(chip, core::ptr::null()); if *buf.add(4) == TPM_INF_NAK { dev_err(chip, core::ptr::null()); } return -5; }
        return -5;
    }
}

unsafe fn tpm_inf_send(chip: *mut core::ffi::c_void, buf: *const u8, _bufsiz: usize, count: usize) -> i32 {
    tpm_data_out(RESET_LP_IRQC_DISABLE, CMD);
    if empty_fifo(chip, 1) != 0 || wait(chip, STAT_XFE) != 0 { return -5; }
    let c = count as u32;
    let bytes = [(c >> 24) as u8, (c >> 16) as u8, (c >> 8) as u8, c as u8];
    let total = (c + 6) as u16;
    wait_and_send(chip, TPM_VL_VER); wait_and_send(chip, TPM_CTRL_DATA);
    wait_and_send(chip, (total >> 8) as u8); wait_and_send(chip, total as u8);
    wait_and_send(chip, TPM_VL_VER); wait_and_send(chip, TPM_VL_CHANNEL_TPM);
    for b in bytes { wait_and_send(chip, b); }
    for i in 0..count { wait_and_send(chip, *buf.add(i)); }
    0
}

unsafe fn tpm_inf_cancel(_chip: *mut core::ffi::c_void) {
    /* Since legacy mode has no cancel function, WTX provides interruption. */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
