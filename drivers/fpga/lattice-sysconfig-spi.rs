// SPDX-License-Identifier: GPL-2.0
/*
 * Lattice FPGA programming over slave SPI sysCONFIG interface.
 */

// Kernel headers and "lattice-sysconfig.h" supply the declarations referenced
// by this translation.

use core::ffi::c_void;

extern "C" {
    fn to_spi_device(dev: *mut device) -> *mut spi_device;
    fn spi_write_then_read(
        spi: *mut spi_device,
        tx_buf: *const c_void,
        tx_len: usize,
        rx_buf: *mut c_void,
        rx_len: usize,
    ) -> i32;
    fn kmemdup(src: *const c_void, len: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn spi_message_init_with_transfers(msg: *mut spi_message, xfer: *mut spi_transfer, n: u32);
    fn spi_bus_lock(controller: *mut spi_controller);
    fn spi_bus_unlock(controller: *mut spi_controller);
    fn spi_sync_locked(spi: *mut spi_device, msg: *mut spi_message) -> i32;
    fn spi_write(spi: *mut spi_device, buf: *const c_void, len: usize) -> i32;
    fn spi_get_device_match_data(spi: *mut spi_device) -> *const u32;
    fn sysconfig_probe(priv_: *mut sysconfig_priv) -> i32;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_controller {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_device {
    pub dev: device,
    pub controller: *mut spi_controller,
    pub max_speed_hz: u32,
}

#[repr(C)]
pub struct spi_transfer {
    pub tx_buf: *const c_void,
    pub len: usize,
    pub cs_change: u8,
}

#[repr(C)]
pub struct spi_message {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sysconfig_priv {
    pub dev: *mut device,
    pub command_transfer: Option<unsafe extern "C" fn(*mut sysconfig_priv, *const c_void, usize, *mut c_void, usize) -> i32>,
    pub bitstream_burst_write_init: Option<unsafe extern "C" fn(*mut sysconfig_priv) -> i32>,
    pub bitstream_burst_write: Option<unsafe extern "C" fn(*mut sysconfig_priv, *const i8, usize) -> i32>,
    pub bitstream_burst_write_complete: Option<unsafe extern "C" fn(*mut sysconfig_priv) -> i32>,
}

const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;
static ECP5_SPI_MAX_SPEED_HZ: u32 = 60_000_000;

static LSC_BITSTREAM_BURST: [u8; 0] = [];

unsafe extern "C" fn sysconfig_spi_cmd_transfer(
    priv_: *mut sysconfig_priv,
    tx_buf: *const c_void,
    tx_len: usize,
    rx_buf: *mut c_void,
    rx_len: usize,
) -> i32 {
    let spi = to_spi_device((*priv_).dev);
    spi_write_then_read(spi, tx_buf, tx_len, rx_buf, rx_len)
}

unsafe extern "C" fn sysconfig_spi_bitstream_burst_init(priv_: *mut sysconfig_priv) -> i32 {
    let lsc_bitstream_burst = &LSC_BITSTREAM_BURST;
    let spi = to_spi_device((*priv_).dev);
    let mut xfer: spi_transfer = core::mem::zeroed();
    let mut msg: spi_message = core::mem::zeroed();
    let buf_len = core::mem::size_of_val(lsc_bitstream_burst);
    let buf = kmemdup(lsc_bitstream_burst.as_ptr() as *const c_void, buf_len, GFP_KERNEL);
    if buf.is_null() {
        return -ENOMEM;
    }

    xfer.len = buf_len;
    xfer.tx_buf = buf;
    xfer.cs_change = 1;
    spi_message_init_with_transfers(&mut msg, &mut xfer, 1);

    /*
     * Lock SPI bus for exclusive usage until FPGA programming is done.
     * SPI bus will be released in sysconfig_spi_bitstream_burst_complete().
     */
    spi_bus_lock((*spi).controller);
    let ret = spi_sync_locked(spi, &mut msg);
    if ret != 0 {
        spi_bus_unlock((*spi).controller);
    }
    kfree(buf);
    ret
}

unsafe extern "C" fn sysconfig_spi_bitstream_burst_write(
    priv_: *mut sysconfig_priv,
    buf: *const i8,
    len: usize,
) -> i32 {
    let spi = to_spi_device((*priv_).dev);
    let mut xfer = spi_transfer { tx_buf: buf as *const c_void, len, cs_change: 1 };
    let mut msg: spi_message = core::mem::zeroed();
    spi_message_init_with_transfers(&mut msg, &mut xfer, 1);
    spi_sync_locked(spi, &mut msg)
}

unsafe extern "C" fn sysconfig_spi_bitstream_burst_complete(priv_: *mut sysconfig_priv) -> i32 {
    let spi = to_spi_device((*priv_).dev);
    /* Bitstream burst write is done, release SPI bus */
    spi_bus_unlock((*spi).controller);
    /* Toggle CS to finish bitstream write */
    spi_write(spi, core::ptr::null(), 0)
}

unsafe extern "C" fn sysconfig_spi_probe(spi: *mut spi_device) -> i32 {
    let dev = &mut (*spi).dev as *mut device;
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<sysconfig_priv>(), GFP_KERNEL);
    if priv_.is_null() { return -ENOMEM; }
    let spi_max_speed = spi_get_device_match_data(spi);
    if spi_max_speed.is_null() { return -EINVAL; }
    if (*spi).max_speed_hz > *spi_max_speed { return -EINVAL; }
    (*priv_).dev = dev;
    (*priv_).command_transfer = Some(sysconfig_spi_cmd_transfer);
    (*priv_).bitstream_burst_write_init = Some(sysconfig_spi_bitstream_burst_init);
    (*priv_).bitstream_burst_write = Some(sysconfig_spi_bitstream_burst_write);
    (*priv_).bitstream_burst_write_complete = Some(sysconfig_spi_bitstream_burst_complete);
    sysconfig_probe(priv_)
}

extern "C" { fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut sysconfig_priv; }

// Device tables, module registration, and metadata are supplied by the kernel
// build integration represented by the declarations below.
#[repr(C)] pub struct spi_device_id { pub name: *const u8, pub driver_data: usize }
#[repr(C)] pub struct of_device_id { pub compatible: *const u8, pub data: *const u32 }
#[repr(C)] pub struct spi_driver { pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> i32>, pub id_table: *const spi_device_id }

static SYSCONFIG_SPI_IDS: [spi_device_id; 2] = [
    spi_device_id { name: b"sysconfig-ecp5\0".as_ptr(), driver_data: &ECP5_SPI_MAX_SPEED_HZ as *const u32 as usize },
    spi_device_id { name: core::ptr::null(), driver_data: 0 },
];
static SYSCONFIG_OF_IDS: [of_device_id; 2] = [
    of_device_id { compatible: b"lattice,sysconfig-ecp5\0".as_ptr(), data: &ECP5_SPI_MAX_SPEED_HZ },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];
static LATTICE_SYSCONFIG_DRIVER: spi_driver = spi_driver { probe: Some(sysconfig_spi_probe), id_table: SYSCONFIG_SPI_IDS.as_ptr() };


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
