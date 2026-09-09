// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-omap1/board-sx1-mmc.c
 *
 * Copyright (C) 2007 Instituto Nokia de Tecnologia - INdT
 * Author: Carlos Eduardo Aguiar <carlos.aguiar@indt.org.br>
 *
 * This code is based on linux/arch/arm/mach-omap1/board-h2-mmc.c, which is:
 * Copyright (C) 2007 Instituto Nokia de Tecnologia - INdT
 */

// Dependency declarations are supplied by the surrounding kernel translation.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub type SetPower = unsafe extern "C" fn(*mut device, i32, i32, i32) -> i32;

#[repr(C)]
pub struct omap_mmc_slot_data {
    pub set_power: Option<SetPower>,
    pub ocr_mask: u32,
    pub name: *const u8,
}

#[repr(C)]
pub struct omap_mmc_platform_data {
    pub nr_slots: i32,
    pub slots: [omap_mmc_slot_data; 1],
}

extern "C" {
    fn sx1_i2c_read_byte(addr: u8, reg: u8, data: *mut u8) -> i32;
    fn sx1_i2c_write_byte(addr: u8, reg: u8, data: u8) -> i32;
    fn omap1_init_mmc(data: *mut *mut omap_mmc_platform_data, count: i32);
}

// These values are provided by hardware.h, board-sx1.h, and mmc.h.
extern "C" {
    static SOFIA_I2C_ADDR: u8;
    static SOFIA_POWER1_REG: u8;
    static SOFIA_MMC_POWER: u8;
    static MMC_VDD_32_33: u32;
    static MMC_VDD_33_34: u32;
    static OMAP15XX_NR_MMC: i32;
}

// CONFIG_MMC_OMAP is a build-time condition corresponding to this cfg.
#[cfg(CONFIG_MMC_OMAP)]
unsafe extern "C" fn mmc_set_power(
    _dev: *mut device,
    _slot: i32,
    power_on: i32,
    _vdd: i32,
) -> i32 {
    let mut dat: u8 = 0;
    let err = sx1_i2c_read_byte(SOFIA_I2C_ADDR, SOFIA_POWER1_REG, &mut dat);
    if err < 0 {
        return err;
    }

    if power_on != 0 {
        dat |= SOFIA_MMC_POWER;
    } else {
        dat &= !SOFIA_MMC_POWER;
    }

    sx1_i2c_write_byte(SOFIA_I2C_ADDR, SOFIA_POWER1_REG, dat)
}

// Cover switch is at OMAP_MPUIO(3)
#[cfg(CONFIG_MMC_OMAP)]
static mut mmc1_data: omap_mmc_platform_data = omap_mmc_platform_data {
    nr_slots: 1,
    slots: [omap_mmc_slot_data {
        set_power: Some(mmc_set_power),
        ocr_mask: unsafe { MMC_VDD_32_33 | MMC_VDD_33_34 },
        name: b"mmcblk\0".as_ptr(),
    }],
};

#[cfg(CONFIG_MMC_OMAP)]
static mut mmc_data: [*mut omap_mmc_platform_data; 1] = [core::ptr::null_mut()];

#[cfg(CONFIG_MMC_OMAP)]
pub unsafe extern "C" fn sx1_mmc_init() {
    mmc_data[0] = &raw mut mmc1_data;
    omap1_init_mmc(mmc_data.as_mut_ptr(), OMAP15XX_NR_MMC);
}

#[cfg(not(CONFIG_MMC_OMAP))]
pub unsafe extern "C" fn sx1_mmc_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
