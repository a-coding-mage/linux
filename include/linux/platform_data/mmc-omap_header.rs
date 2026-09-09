/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * MMC definitions for OMAP2
 *
 * Copyright (C) 2006 Nokia Corporation
 */

pub const OMAP_MMC_MAX_SLOTS: usize = 2;

// Supplied by another header/dependency.
#[repr(C)]
pub struct mmc_card {
    _private: [u8; 0],
}

// Supplied by another header/dependency.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct omap_mmc_slot_data {
    /*
     * 4/8 wires and any additional host capabilities
     * need to OR'd all capabilities (ref. linux/mmc/host.h)
     */
    pub wires: u8, /* Used for the MMC driver on omap1 and 2420 */
    pub caps: u32, /* Used for the MMC driver on 2430 and later */
    pub pm_caps: u32, /* PM capabilities of the mmc */

    /*
     * nomux means "standard" muxing is wrong on this board, and
     * that board-specific code handled it before common init logic.
     */
    pub nomux: u8,

    /* switch pin can be for card detect (default) or card cover */
    pub cover: u8,

    /* use the internal clock */
    pub internal_clock: u8,

    /* nonremovable e.g. eMMC */
    pub nonremovable: u8,

    /* Try to sleep or power off when possible */
    pub power_saving: u8,

    /* If using power_saving and the MMC power is not to go off */
    pub no_off: u8,

    /* eMMC does not handle power off when not in sleep state */
    pub no_regulator_off_init: u8,

    /* Regulator off remapped to sleep */
    pub vcc_aux_disable_is_sleep: u8,

    /* we can put the features above into this variable */
    pub features: u32,

    pub switch_pin: i32, /* gpio (card detect) */
    pub gpio_wp: i32, /* gpio (write protect) */

    pub set_bus_mode: Option<unsafe extern "C" fn(*mut device, i32, i32) -> i32>,
    pub set_power: Option<unsafe extern "C" fn(*mut device, i32, i32, i32) -> i32>,
    pub get_ro: Option<unsafe extern "C" fn(*mut device, i32) -> i32>,
    pub remux: Option<unsafe extern "C" fn(*mut device, i32, i32)>,
    /* Call back before enabling / disabling regulators */
    pub before_set_reg: Option<unsafe extern "C" fn(*mut device, i32, i32, i32)>,
    /* Call back after enabling / disabling regulators */
    pub after_set_reg: Option<unsafe extern "C" fn(*mut device, i32, i32, i32)>,
    /* if we have special card, init it using this callback */
    pub init_card: Option<unsafe extern "C" fn(*mut mmc_card)>,

    /* return MMC cover switch state, can be NULL if not supported.
     *
     * possible return values:
     *   0 - closed
     *   1 - open
     */
    pub get_cover_state: Option<unsafe extern "C" fn(*mut device, i32) -> i32>,

    pub name: *const core::ffi::c_char,
    pub ocr_mask: u32,

    /* Card detection */
    pub card_detect: Option<unsafe extern "C" fn(*mut device, i32) -> i32>,

    pub ban_openended: u8,
}

pub const MMC_OMAP7XX: u32 = 1 << 3;
pub const MMC_OMAP15XX: u32 = 1 << 4;
pub const MMC_OMAP16XX: u32 = 1 << 5;

#[repr(C)]
pub struct omap_mmc_platform_data {
    /* back-link to device */
    pub dev: *mut device,

    /* number of slots per controller */
    pub nr_slots: u8,

    /* set if your board has components or wiring that limits the
     * maximum frequency on the MMC bus */
    pub max_freq: u32,

    /* initialize board-specific MMC functionality, can be NULL if
     * not supported */
    pub init: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub cleanup: Option<unsafe extern "C" fn(*mut device)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut device)>,

    /* Return context loss count due to PM states changing */
    pub get_context_loss_count: Option<unsafe extern "C" fn(*mut device) -> i32>,

    /* Integrating attributes from the omap_hwmod layer */
    pub controller_flags: u8,

    /* Register offset deviation */
    pub reg_offset: u16,

    pub slots: [omap_mmc_slot_data; OMAP_MMC_MAX_SLOTS],
}

unsafe extern "C" {
    pub fn omap_mmc_notify_cover_event(dev: *mut device, slot: i32, is_closed: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
