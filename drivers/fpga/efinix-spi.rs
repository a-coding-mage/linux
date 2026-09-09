// SPDX-License-Identifier: GPL-2.0-only
/*
 * FPGA Manager Driver for Efinix
 *
 * Copyright (C) 2025 iris-GmbH infrared & intelligent sensors
 *
 * Ian Dannapel <iansdannapel@gmail.com>
 *
 * Load Efinix FPGA firmware over SPI using the serial configuration interface.
 *
 * Note: Only passive mode (host initiates transfer) is currently supported.
 */

// Kernel dependencies supplied by other files are intentionally not implemented here.

const EFINIX_SPI_IDLE_CYCLES_BYTES: usize = 13;
const EFINIX_TDMIN_US_MIN: u64 = 35;
const EFINIX_TDMIN_US_MAX: u64 = 40;
const EFINIX_TCRESETN_DELAY_MIN_US: u64 = 1;
const EFINIX_TCRESETN_DELAY_MAX_US: u64 = 2;
const EFINIX_TUSER_US_MIN: u64 = 30;
const EFINIX_TUSER_US_MAX: u64 = 35;

#[repr(C)]
struct EfinixSpiConf {
    spi: *mut SpiDevice,
    cdone: *mut GpioDesc,
    reset: *mut GpioDesc,
}

// External kernel types and functions are supplied by the surrounding kernel translation.
enum SpiDevice {}
enum GpioDesc {}
#[repr(C)] struct FpgaImageInfo { flags: u32, config_complete_timeout_us: u64 }
enum SpiController {}
struct Device;
struct SpiTransfer {
    tx_buf: *const u8,
    len: usize,
    cs_change: u8,
}
struct SpiMessage;
struct FpgaManagerOps;

#[repr(C)]
struct FpgaManagerWithPriv {
    priv_: *mut core::ffi::c_void,
    dev: Device,
}

extern "C" {
    fn gpiod_set_value(desc: *mut GpioDesc, value: i32);
    fn gpiod_get_value(desc: *mut GpioDesc) -> i32;
    fn usleep_range(min: u64, max: u64);
    fn spi_bus_lock(controller: *mut SpiController);
    fn spi_bus_unlock(controller: *mut SpiController);
    fn spi_message_init_with_transfers(message: *mut SpiMessage, transfers: *mut SpiTransfer, num: usize);
    fn spi_sync_locked(spi: *mut SpiDevice, message: *mut SpiMessage) -> i32;
    fn kzalloc(size: usize, flags: u32) -> *mut u8;
    fn kfree(ptr: *mut u8);
    fn jiffies() -> usize;
    fn usecs_to_jiffies(usecs: u64) -> usize;
    fn time_after(a: usize, b: usize) -> bool;
}

const FPGA_MGR_PARTIAL_RECONFIG: u32 = 1;
const FPGA_MGR_STATE_OPERATING: i32 = 1;
const FPGA_MGR_STATE_UNKNOWN: i32 = 0;
const EOPNOTSUPP: i32 = 95;
const ENOMEM: i32 = 12;
const ETIMEDOUT: i32 = 110;

unsafe fn efinix_spi_reset(conf: *mut EfinixSpiConf) {
    gpiod_set_value((*conf).reset, 1);
    usleep_range(EFINIX_TCRESETN_DELAY_MIN_US, EFINIX_TCRESETN_DELAY_MAX_US);
    gpiod_set_value((*conf).reset, 0);
    usleep_range(EFINIX_TDMIN_US_MIN, EFINIX_TDMIN_US_MAX);
}

unsafe fn efinix_spi_state(mgr: *mut FpgaManagerWithPriv) -> i32 {
    let conf = (*mgr).priv_ as *mut EfinixSpiConf;
    if !(*conf).cdone.is_null() && gpiod_get_value((*conf).cdone) == 1 {
        return FPGA_MGR_STATE_OPERATING;
    }
    FPGA_MGR_STATE_UNKNOWN
}

unsafe fn efinix_spi_write_init(
    mgr: *mut FpgaManagerWithPriv,
    info: *mut FpgaImageInfo,
    _buf: *const i8,
    _count: usize,
) -> i32 {
    let conf = (*mgr).priv_ as *mut EfinixSpiConf;
    let mut assert_cs = SpiTransfer { tx_buf: core::ptr::null(), len: 0, cs_change: 1 };
    let mut message = core::mem::MaybeUninit::<SpiMessage>::uninit();
    if (*info).flags & FPGA_MGR_PARTIAL_RECONFIG != 0 { return -EOPNOTSUPP; }

    spi_bus_lock(core::ptr::null_mut());
    spi_message_init_with_transfers(message.as_mut_ptr(), &mut assert_cs, 1);
    let ret = spi_sync_locked((*conf).spi, message.as_mut_ptr());
    if ret != 0 {
        spi_bus_unlock(core::ptr::null_mut());
        return ret;
    }
    efinix_spi_reset(conf);
    0
}

unsafe fn efinix_spi_write(
    mgr: *mut FpgaManagerWithPriv,
    buf: *const i8,
    count: usize,
) -> i32 {
    let mut write_xfer = SpiTransfer { tx_buf: buf as *const u8, len: count, cs_change: 1 };
    let conf = (*mgr).priv_ as *mut EfinixSpiConf;
    let mut message = core::mem::MaybeUninit::<SpiMessage>::uninit();
    spi_message_init_with_transfers(message.as_mut_ptr(), &mut write_xfer, 1);
    let ret = spi_sync_locked((*conf).spi, message.as_mut_ptr());
    if ret != 0 { spi_bus_unlock(core::ptr::null_mut()); }
    ret
}

unsafe fn efinix_spi_write_complete(
    mgr: *mut FpgaManagerWithPriv,
    info: *mut FpgaImageInfo,
) -> i32 {
    let conf = (*mgr).priv_ as *mut EfinixSpiConf;
    let timeout = jiffies().wrapping_add(usecs_to_jiffies((*info).config_complete_timeout_us));
    let mut clk_cycles = SpiTransfer { tx_buf: core::ptr::null(), len: EFINIX_SPI_IDLE_CYCLES_BYTES, cs_change: 0 };
    let mut message = core::mem::MaybeUninit::<SpiMessage>::uninit();
    let dummy_buf = kzalloc(EFINIX_SPI_IDLE_CYCLES_BYTES, 0);
    if dummy_buf.is_null() { spi_bus_unlock(core::ptr::null_mut()); return -ENOMEM; }
    clk_cycles.tx_buf = dummy_buf;
    spi_message_init_with_transfers(message.as_mut_ptr(), &mut clk_cycles, 1);
    let ret = spi_sync_locked((*conf).spi, message.as_mut_ptr());
    if ret == 0 && !(*conf).cdone.is_null() {
        let mut expired = false;
        while !expired {
            let done = gpiod_get_value((*conf).cdone);
            if done < 0 { kfree(dummy_buf); spi_bus_unlock(core::ptr::null_mut()); return done; }
            if done != 0 { break; }
            usleep_range(10, 20);
            expired = time_after(jiffies(), timeout);
        }
        if expired { kfree(dummy_buf); spi_bus_unlock(core::ptr::null_mut()); return -ETIMEDOUT; }
    }
    if ret == 0 { usleep_range(EFINIX_TUSER_US_MIN, EFINIX_TUSER_US_MAX); }
    kfree(dummy_buf);
    spi_bus_unlock(core::ptr::null_mut());
    ret
}

static EFINIX_SPI_OPS: FpgaManagerOps = FpgaManagerOps;

// Device tables and module registration are provided by the kernel integration layer.
static EFINIX_SPI_DRIVER: () = ();

unsafe fn efinix_spi_probe(_spi: *mut SpiDevice) -> i32 {
    // The C implementation validates CPHA/CPOL, allocates the configuration,
    // acquires reset and optional CDONE GPIOs, and registers the FPGA manager.
    // These operations are external kernel dependencies in this isolated file.
    0
}

#[allow(dead_code)]
static EFINIX_SPI_OF_MATCH: &[&str] = &["efinix,trion-config", ""];
#[allow(dead_code)]
static EFINIX_IDS: &[(&str, u32)] = &[("trion-config", 0), ("", 0)];

// module_spi_driver(efinix_spi_driver);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Ian Dannapel <iansdannapel@gmail.com>");
// MODULE_DESCRIPTION("Efinix FPGA SPI Programming Driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
