// SPDX-License-Identifier: GPL-2.0-only
/*
 * Altera Passive Serial SPI Driver
 *
 *  Copyright (c) 2017 United Western Technologies, Corporation
 *
 *  Joshua Clayton <stillcompiling@gmail.com>
 *
 * Manage Altera FPGA firmware that is loaded over SPI using the passive
 * serial configuration method.
 * Firmware must be in binary "rbf" format.
 * Works on Arria 10, Cyclone V and Stratix V. Should work on Cyclone series.
 * May work on other Altera FPGAs.
 */

// Linux kernel dependencies corresponding to the original C includes.

#[repr(C)]
#[derive(Copy, Clone)]
enum altera_ps_devtype {
    CYCLONE5,
    ARRIA10,
}

#[repr(C)]
struct altera_ps_data {
    devtype: altera_ps_devtype,
    status_wait_min_us: i32,
    status_wait_max_us: i32,
    t_cfg_us: i32,
    t_st2ck_us: i32,
}

#[repr(C)]
struct altera_ps_conf {
    config: *mut gpio_desc,
    confd: *mut gpio_desc,
    status: *mut gpio_desc,
    spi: *mut spi_device,
    data: *const altera_ps_data,
    info_flags: u32,
    mgr_name: [core::ffi::c_char; 64],
}

//          |   Arria 10  |   Cyclone5  |   Stratix5  |
// t_CF2ST0 |     [; 600] |     [; 600] |     [; 600] |ns
// t_CFG    |        [2;] |        [2;] |        [2;] |µs
// t_STATUS | [268; 3000] | [268; 1506] | [268; 1506] |µs
// t_CF2ST1 |    [; 3000] |    [; 1506] |    [; 1506] |µs
// t_CF2CK  |     [3010;] |     [1506;] |     [1506;] |µs
// t_ST2CK  |       [10;] |        [2;] |        [2;] |µs
// t_CD2UM  |  [175; 830] |  [175; 437] |  [175; 437] |µs
static mut c5_data: altera_ps_data = altera_ps_data {
    devtype: altera_ps_devtype::CYCLONE5,
    status_wait_min_us: 268,
    status_wait_max_us: 1506,
    t_cfg_us: 2,
    t_st2ck_us: 2,
};

static mut a10_data: altera_ps_data = altera_ps_data {
    devtype: altera_ps_devtype::ARRIA10,
    status_wait_min_us: 268,
    status_wait_max_us: 3000,
    t_cfg_us: 2,
    t_st2ck_us: 10,
};

// Device-match table: "altr,fpga-passive-serial" -> c5_data;
// "altr,fpga-arria10-passive-serial" -> a10_data; followed by a sentinel.

unsafe fn altera_ps_state(mgr: *mut fpga_manager) -> fpga_mgr_states {
    let conf = (*mgr).priv_data as *mut altera_ps_conf;
    if gpiod_get_value_cansleep((*conf).status) != 0 {
        FPGA_MGR_STATE_RESET
    } else {
        FPGA_MGR_STATE_UNKNOWN
    }
}

unsafe fn altera_ps_delay(delay_us: i32) {
    if delay_us > 10 {
        usleep_range(delay_us, delay_us + 5);
    } else {
        udelay(delay_us);
    }
}

unsafe fn altera_ps_write_init(
    mgr: *mut fpga_manager,
    info: *mut fpga_image_info,
    _buf: *const core::ffi::c_char,
    _count: usize,
) -> i32 {
    let conf = (*mgr).priv_data as *mut altera_ps_conf;
    let mut min: i32;
    let mut max: i32;
    let mut waits: i32;
    (*conf).info_flags = (*info).flags;

    if (*info).flags & FPGA_MGR_PARTIAL_RECONFIG != 0 {
        dev_err(&mut (*mgr).dev, "Partial reconfiguration not supported.\n");
        return -EINVAL;
    }
    gpiod_set_value_cansleep((*conf).config, 1);
    altera_ps_delay((*(*conf).data).t_cfg_us);
    if gpiod_get_value_cansleep((*conf).status) == 0 {
        dev_err(&mut (*mgr).dev, "Status pin failed to show a reset\n");
        return -EIO;
    }
    gpiod_set_value_cansleep((*conf).config, 0);
    min = (*(*conf).data).status_wait_min_us;
    max = (*(*conf).data).status_wait_max_us;
    waits = max / min;
    if max % min != 0 { waits += 1; }
    for _i in 0..waits {
        usleep_range(min, min + 10);
        if gpiod_get_value_cansleep((*conf).status) == 0 {
            altera_ps_delay((*(*conf).data).t_st2ck_us);
            return 0;
        }
    }
    dev_err(&mut (*mgr).dev, "Status pin not ready.\n");
    -EIO
}

unsafe fn rev_buf(mut buf: *mut core::ffi::c_char, len: usize) {
    let mut fw32 = buf as *mut u32;
    let extra_bytes = len & 0x03;
    let fw_end = (buf.add(len - extra_bytes)) as *const u32;
    while fw32 < fw_end as *mut u32 {
        *fw32 = bitrev8x4(*fw32);
        fw32 = fw32.add(1);
    }
    if extra_bytes != 0 {
        buf = fw_end as *mut core::ffi::c_char;
        let mut remaining = extra_bytes;
        while remaining != 0 {
            *buf = bitrev8(*buf as u8) as core::ffi::c_char;
            buf = buf.add(1);
            remaining -= 1;
        }
    }
}

unsafe fn altera_ps_write(mgr: *mut fpga_manager, buf: *const core::ffi::c_char, count: usize) -> i32 {
    let conf = (*mgr).priv_data as *mut altera_ps_conf;
    let mut fw_data = buf;
    let fw_data_end = buf.add(count);
    while fw_data < fw_data_end {
        let stride = core::cmp::min(fw_data_end.offset_from(fw_data) as usize, 4096);
        if (*conf).info_flags & FPGA_MGR_BITSTREAM_LSB_FIRST == 0 {
            rev_buf(fw_data as *mut core::ffi::c_char, stride);
        }
        let ret = spi_write((*conf).spi, fw_data, stride);
        if ret != 0 {
            dev_err(&mut (*mgr).dev, "spi error in firmware write: %d\n", ret);
            return ret;
        }
        fw_data = fw_data.add(stride);
    }
    0
}

unsafe fn altera_ps_write_complete(mgr: *mut fpga_manager, _info: *mut fpga_image_info) -> i32 {
    let conf = (*mgr).priv_data as *mut altera_ps_conf;
    static DUMMY: [core::ffi::c_char; 1] = [0];
    if gpiod_get_value_cansleep((*conf).status) != 0 {
        dev_err(&mut (*mgr).dev, "Error during configuration.\n");
        return -EIO;
    }
    if !(*conf).confd.is_null() && gpiod_get_raw_value_cansleep((*conf).confd) == 0 {
        dev_err(&mut (*mgr).dev, "CONF_DONE is inactive!\n");
        return -EIO;
    }
    // After CONF_DONE goes high, send two additional falling edges on DCLK
    // to begin initialization and enter user mode.
    let ret = spi_write((*conf).spi, DUMMY.as_ptr(), 1);
    if ret != 0 {
        dev_err(&mut (*mgr).dev, "spi error during end sequence: %d\n", ret);
        return ret;
    }
    0
}

// External kernel types, constants, and functions are supplied by dependencies.
#[allow(non_camel_case_types)] type gpio_desc = core::ffi::c_void;
#[allow(non_camel_case_types)] type spi_device = core::ffi::c_void;
#[allow(non_camel_case_types)] type fpga_mgr_states = i32;
#[repr(C)] struct device { _private: [u8; 0] }
#[repr(C)] struct fpga_manager { dev: device, priv_data: *mut core::ffi::c_void }
#[repr(C)] struct fpga_image_info { flags: u32 }
extern "C" {
    fn gpiod_get_value_cansleep(gpio: *mut gpio_desc) -> i32;
    fn gpiod_get_raw_value_cansleep(gpio: *mut gpio_desc) -> i32;
    fn gpiod_set_value_cansleep(gpio: *mut gpio_desc, value: i32);
    fn usleep_range(min: i32, max: i32);
    fn udelay(usecs: i32);
    fn bitrev8x4(value: u32) -> u32;
    fn bitrev8(value: u8) -> u8;
    fn spi_write(spi: *mut spi_device, buf: *const core::ffi::c_char, len: usize) -> i32;
}
const FPGA_MGR_STATE_RESET: fpga_mgr_states = 0;
const FPGA_MGR_STATE_UNKNOWN: fpga_mgr_states = 1;
const FPGA_MGR_PARTIAL_RECONFIG: u32 = 1 << 0;
const FPGA_MGR_BITSTREAM_LSB_FIRST: u32 = 1 << 1;
const EINVAL: i32 = 22;
const EIO: i32 = 5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
