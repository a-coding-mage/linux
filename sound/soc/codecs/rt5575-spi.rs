// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt5575-spi.c  --  ALC5575 SPI driver
 *
 * Copyright(c) 2025 Realtek Semiconductor Corp.
 *
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const RT5575_SPI_CMD_BURST_WRITE: u8 = 5;
const RT5575_SPI_BUF_LEN: usize = 240;

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_controller {
    pub num_chipselect: u32,
}

#[repr(C)]
pub struct spi_device {
    pub dev: device,
}

#[repr(C)]
pub struct spi_board_info {
    pub modalias: *const c_char,
    pub chip_select: u32,
    pub max_speed_hz: u32,
}

#[repr(C)]
pub struct firmware {
    pub size: usize,
    pub data: *const u8,
}

#[repr(C, packed)]
struct rt5575_spi_burst_write {
    cmd: u8,
    addr: u32,
    data: [u8; RT5575_SPI_BUF_LEN],
    dummy: u8,
}

unsafe extern "C" {
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_property_read_u32_index(
        np: *mut device_node,
        propname: *const c_char,
        index: u32,
        out_value: *mut u32,
    ) -> c_int;
    fn of_find_spi_controller_by_node(node: *mut device_node) -> *mut spi_controller;
    fn of_node_put(node: *mut device_node);
    fn spi_controller_put(ctlr: *mut spi_controller);
    fn spi_new_device(ctlr: *mut spi_controller, chip: *const spi_board_info) -> *mut spi_device;
    fn spi_write(spi: *mut spi_device, buf: *const c_void, len: usize) -> c_int;
    fn request_firmware(
        firmware_p: *mut *const firmware,
        name: *const c_char,
        device: *mut device,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rt5575_spi_get_device(dev: *mut device) -> *mut spi_device {
    let mut spi: *mut spi_device;
    let ctlr: *mut spi_controller;
    let spi_np: *mut device_node;
    let mut cs: u32;

    spi_np = unsafe { of_parse_phandle((*dev).of_node, c"spi-parent".as_ptr()) };
    if spi_np.is_null() {
        unsafe { dev_err(dev, c"Failed to get spi-parent phandle\n".as_ptr()) };
        return ptr::null_mut();
    }

    if unsafe { of_property_read_u32_index((*dev).of_node, c"spi-parent".as_ptr(), 1, &mut cs) }
        != 0
    {
        cs = 0;
    }

    ctlr = unsafe { of_find_spi_controller_by_node(spi_np) };
    unsafe { of_node_put(spi_np) };
    if ctlr.is_null() {
        unsafe { dev_err(dev, c"Failed to get spi_controller\n".as_ptr()) };
        return ptr::null_mut();
    }

    if cs >= unsafe { (*ctlr).num_chipselect } {
        unsafe { dev_err(dev, c"Chip select has wrong number %d\n".as_ptr(), cs as c_uint) };
        unsafe { spi_controller_put(ctlr) };
        return ptr::null_mut();
    }

    spi = unsafe {
        spi_new_device(
            ctlr,
            &spi_board_info {
                modalias: c"rt5575".as_ptr(),
                chip_select: cs,
                max_speed_hz: 10000000,
            },
        )
    };

    unsafe { spi_controller_put(ctlr) };
    spi
}

/**
 * rt5575_spi_burst_write - Write data to SPI by rt5575 address.
 * @spi: SPI device.
 * @addr: Start address.
 * @txbuf: Data buffer for writing.
 * @len: Data length.
 *
 */
unsafe fn rt5575_spi_burst_write(
    spi: *mut spi_device,
    addr: u32,
    txbuf: *const u8,
    len: usize,
) {
    let mut buf = rt5575_spi_burst_write {
        cmd: RT5575_SPI_CMD_BURST_WRITE,
        addr: 0,
        data: [0; RT5575_SPI_BUF_LEN],
        dummy: 0,
    };
    let mut end: c_uint;
    let mut offset: c_uint = 0;

    while (offset as usize) < len {
        if (offset as usize) + RT5575_SPI_BUF_LEN <= len {
            end = RT5575_SPI_BUF_LEN as c_uint;
        } else {
            end = (len % RT5575_SPI_BUF_LEN) as c_uint;
        }

        buf.addr = addr.wrapping_add(offset).to_le();
        unsafe {
            ptr::copy_nonoverlapping(
                txbuf.add(offset as usize),
                buf.data.as_mut_ptr(),
                end as usize,
            )
        };
        unsafe {
            spi_write(
                spi,
                &buf as *const rt5575_spi_burst_write as *const c_void,
                size_of::<rt5575_spi_burst_write>(),
            )
        };

        offset = offset.wrapping_add(RT5575_SPI_BUF_LEN as c_uint);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rt5575_spi_fw_load(spi: *mut spi_device) -> c_int {
    let dev: *mut device = unsafe { &mut (*spi).dev };
    let mut i: c_int;
    let mut ret: c_int;
    static FW_PATH: [*const c_char; 4] = [
        c"realtek/rt5575/rt5575_fw1.bin".as_ptr(),
        c"realtek/rt5575/rt5575_fw2.bin".as_ptr(),
        c"realtek/rt5575/rt5575_fw3.bin".as_ptr(),
        c"realtek/rt5575/rt5575_fw4.bin".as_ptr(),
    ];
    static FW_ADDR: [u32; 4] = [0x5f400000, 0x5f600000, 0x5f7fe000, 0x5f7ff000];

    i = 0;
    while (i as usize) < FW_ADDR.len() {
        /* C source uses __free(firmware) cleanup attribute for firmware lifetime. */
        let mut firmware: *const firmware = ptr::null();
        ret = unsafe { request_firmware(&mut firmware, FW_PATH[i as usize], dev) };
        if ret != 0 {
            unsafe { dev_err(dev, c"Request firmware failure: %d\n".as_ptr(), ret) };
            return ret;
        }

        unsafe {
            rt5575_spi_burst_write(
                spi,
                FW_ADDR[i as usize],
                (*firmware).data,
                (*firmware).size,
            )
        };
        i += 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
