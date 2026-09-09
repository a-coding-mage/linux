// SPDX-License-Identifier: GPL-2.0-only
/*
 * FPGA Manager Driver for Lattice iCE40.
 *
 *  Copyright (c) 2016 Joel Holdsworth
 *
 * This driver adds support to the FPGA manager for configuring the SRAM of
 * Lattice iCE40 FPGAs through slave SPI.
 */

// Dependencies supplied by the surrounding kernel bindings are intentionally
// referenced here rather than implemented in this translation unit.

const ICE40_SPI_MAX_SPEED: u32 = 25_000_000; // Hz
const ICE40_SPI_MIN_SPEED: u32 = 1_000_000; // Hz

const ICE40_SPI_RESET_DELAY: u32 = 1; // us (>200ns)
const ICE40_SPI_HOUSEKEEPING_DELAY: u32 = 1200; // us

const ICE40_SPI_NUM_ACTIVATION_BYTES: usize = (49 + 8 - 1) / 8;

#[repr(C)]
struct Ice40FpgaPriv {
    dev: *mut SpiDevice,
    reset: *mut GpioDesc,
    cdone: *mut GpioDesc,
}

unsafe fn ice40_fpga_ops_state(mgr: *mut FpgaManager) -> FpgaMgrStates {
    let priv_ = (*mgr).priv_ as *mut Ice40FpgaPriv;

    if gpiod_get_value((*priv_).cdone) != 0 {
        FpgaMgrStates::Operating
    } else {
        FpgaMgrStates::Unknown
    }
}

unsafe fn ice40_fpga_ops_write_init(
    mgr: *mut FpgaManager,
    info: *mut FpgaImageInfo,
    _buf: *const libc::c_char,
    _count: usize,
) -> libc::c_int {
    let priv_ = (*mgr).priv_ as *mut Ice40FpgaPriv;
    let dev = (*priv_).dev;
    let mut message = SpiMessage::default();
    let mut assert_cs_then_reset_delay = SpiTransfer::default();
    assert_cs_then_reset_delay.cs_change = 1;
    assert_cs_then_reset_delay.delay.value = ICE40_SPI_RESET_DELAY as u16;
    assert_cs_then_reset_delay.delay.unit = SpiDelayUnit::Usecs;
    let mut housekeeping_delay_then_release_cs = SpiTransfer::default();
    housekeeping_delay_then_release_cs.delay.value = ICE40_SPI_HOUSEKEEPING_DELAY as u16;
    housekeeping_delay_then_release_cs.delay.unit = SpiDelayUnit::Usecs;
    let mut ret: libc::c_int;

    if ((*info).flags & FpgaMgrFlags::PartialReconfig.bits()) != 0 {
        dev_err((*dev).dev, "Partial reconfiguration is not supported\n");
        return -ENOTSUPP;
    }

    /* Lock the bus, assert CRESET_B and SS_B and delay >200ns */
    spi_bus_lock((*dev).controller);
    gpiod_set_value((*priv_).reset, 1);
    spi_message_init(&mut message);
    spi_message_add_tail(&mut assert_cs_then_reset_delay, &mut message);
    ret = spi_sync_locked(dev, &mut message);
    /* Come out of reset */
    gpiod_set_value((*priv_).reset, 0);
    /* Abort if the chip-select failed */
    if ret != 0 {
        spi_bus_unlock((*dev).controller);
        return ret;
    }
    /* Check CDONE is de-asserted i.e. the FPGA is reset */
    if gpiod_get_value((*priv_).cdone) != 0 {
        dev_err((*dev).dev, "Device reset failed, CDONE is asserted\n");
        ret = -EIO;
        spi_bus_unlock((*dev).controller);
        return ret;
    }
    /* Wait for the housekeeping to complete, and release SS_B */
    spi_message_init(&mut message);
    spi_message_add_tail(&mut housekeeping_delay_then_release_cs, &mut message);
    ret = spi_sync_locked(dev, &mut message);
    spi_bus_unlock((*dev).controller);
    ret
}

unsafe fn ice40_fpga_ops_write(
    mgr: *mut FpgaManager,
    buf: *const libc::c_char,
    count: usize,
) -> libc::c_int {
    let priv_ = (*mgr).priv_ as *mut Ice40FpgaPriv;
    spi_write((*priv_).dev, buf as *const libc::c_void, count)
}

unsafe fn ice40_fpga_ops_write_complete(
    mgr: *mut FpgaManager,
    _info: *mut FpgaImageInfo,
) -> libc::c_int {
    let priv_ = (*mgr).priv_ as *mut Ice40FpgaPriv;
    let dev = (*priv_).dev;
    let padding = [0u8; ICE40_SPI_NUM_ACTIVATION_BYTES];
    /* Check CDONE is asserted */
    if gpiod_get_value((*priv_).cdone) == 0 {
        dev_err((*dev).dev, "CDONE was not asserted after firmware transfer\n");
        return -EIO;
    }
    /* Send of zero-padding to activate the firmware */
    spi_write(dev, padding.as_ptr() as *const libc::c_void, padding.len())
}

static ICE40_FPGA_OPS: FpgaManagerOps = FpgaManagerOps {
    state: Some(ice40_fpga_ops_state),
    write_init: Some(ice40_fpga_ops_write_init),
    write: Some(ice40_fpga_ops_write),
    write_complete: Some(ice40_fpga_ops_write_complete),
};

unsafe fn ice40_fpga_probe(spi: *mut SpiDevice) -> libc::c_int {
    let dev = (*spi).dev;
    let priv_ = devm_kzalloc(dev, core::mem::size_of::<Ice40FpgaPriv>(), GFP_KERNEL)
        as *mut Ice40FpgaPriv;
    if priv_.is_null() {
        return -ENOMEM;
    }
    (*priv_).dev = spi;
    /* Check board setup data. */
    if (*spi).max_speed_hz > ICE40_SPI_MAX_SPEED {
        dev_err(dev, "SPI speed is too high, maximum speed is 25000000\n");
        return -EINVAL;
    }
    if (*spi).max_speed_hz < ICE40_SPI_MIN_SPEED {
        dev_err(dev, "SPI speed is too low, minimum speed is 1000000\n");
        return -EINVAL;
    }
    if ((*spi).mode & SPI_CPHA) != 0 {
        dev_err(dev, "Bad SPI mode, CPHA not supported\n");
        return -EINVAL;
    }
    /* Set up the GPIOs */
    (*priv_).cdone = devm_gpiod_get(dev, "cdone", GPIOD_IN);
    if is_err((*priv_).cdone) {
        let ret = ptr_err((*priv_).cdone);
        dev_err(dev, "Failed to get CDONE GPIO: %d\n", ret);
        return ret;
    }
    (*priv_).reset = devm_gpiod_get(dev, "reset", GPIOD_OUT_HIGH);
    if is_err((*priv_).reset) {
        let ret = ptr_err((*priv_).reset);
        dev_err(dev, "Failed to get CRESET_B GPIO: %d\n", ret);
        return ret;
    }
    let mgr = devm_fpga_mgr_register(dev, "Lattice iCE40 FPGA Manager", &ICE40_FPGA_OPS, priv_);
    ptr_err_or_zero(mgr)
}

static ICE40_FPGA_OF_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { compatible: "lattice,ice40-fpga-mgr" },
    OfDeviceId { compatible: "" },
];

static ICE40_FPGA_SPI_IDS: [SpiDeviceId; 2] = [
    SpiDeviceId { name: "ice40-fpga-mgr" },
    SpiDeviceId { name: "" },
];

static ICE40_FPGA_DRIVER: SpiDriver = SpiDriver {
    probe: Some(ice40_fpga_probe),
    driver: DeviceDriver {
        name: "ice40spi",
        of_match_table: &ICE40_FPGA_OF_MATCH,
    },
    id_table: &ICE40_FPGA_SPI_IDS,
};

module_spi_driver!(ICE40_FPGA_DRIVER);

module_author!("Joel Holdsworth <joel@airwebreathe.org.uk>");
module_description!("Lattice iCE40 FPGA Manager");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
