// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-omap2/board-n8x0.c
 *
 * Copyright (C) 2005-2009 Nokia Corporation
 * Author: Juha Yrjola <juha.yrjola@nokia.com>
 *
 * Modified from mach-omap2/board-generic.c
 */

// Kernel dependencies supplied by the surrounding translation unit.

const TUSB6010_ASYNC_CS: i32 = 1;
const TUSB6010_SYNC_CS: i32 = 4;
const TUSB6010_DMACHAN: i32 = 0x3f;

const NOKIA_N810_WIMAX: u32 = 1 << 2;
const NOKIA_N810: u32 = 1 << 1;
const NOKIA_N800: u32 = 1 << 0;

static mut board_caps: u32 = 0;

#[inline]
unsafe fn board_is_n800() -> bool { board_caps & NOKIA_N800 != 0 }
#[inline]
unsafe fn board_is_n810() -> bool { board_caps & NOKIA_N810 != 0 }
#[inline]
unsafe fn board_is_n810_wimax() -> bool { board_caps & NOKIA_N810_WIMAX != 0 }

unsafe fn board_check_revision() {
    if of_machine_is_compatible("nokia,n800") { board_caps = NOKIA_N800; }
    else if of_machine_is_compatible("nokia,n810") { board_caps = NOKIA_N810; }
    else if of_machine_is_compatible("nokia,n810-wimax") { board_caps = NOKIA_N810_WIMAX; }
    if board_caps == 0 { pr_err!("Unknown board\n"); }
}

static mut tusb_gpio_table: gpiod_lookup_table = gpiod_lookup_table {
    dev_id: "musb-tusb", table: [],
};
static mut nokia800_mmc_gpio_table: gpiod_lookup_table = gpiod_lookup_table {
    dev_id: "mmci-omap.0", table: [],
};
static mut nokia810_mmc_gpio_table: gpiod_lookup_table = gpiod_lookup_table {
    dev_id: "mmci-omap.0", table: [],
};
static mut nokia810_asoc_gpio_table: gpiod_lookup_table = gpiod_lookup_table {
    dev_id: "soc-audio", table: [],
};

// CONFIG_USB_MUSB_TUSB6010 conditional section.
#[cfg(feature = "CONFIG_USB_MUSB_TUSB6010")]
static mut musb_config: musb_hdrc_config = musb_hdrc_config {
    multipoint: 1, dyn_fifo: 1, num_eps: 16, ram_bits: 12,
};
#[cfg(feature = "CONFIG_USB_MUSB_TUSB6010")]
static mut tusb_data: musb_hdrc_platform_data = musb_hdrc_platform_data {
    mode: MUSB_OTG, min_power: 25, power: 100,
    config: unsafe { &raw mut musb_config },
};
#[cfg(feature = "CONFIG_USB_MUSB_TUSB6010")]
unsafe fn n8x0_usb_init() {
    gpiod_add_lookup_table(&raw mut tusb_gpio_table);
    let ret = tusb6010_setup_interface(&raw mut tusb_data, TUSB6010_REFCLK_19, 2,
        TUSB6010_ASYNC_CS, TUSB6010_SYNC_CS, TUSB6010_DMACHAN);
    if ret != 0 { return; }
    pr_info!("TUSB 6010\n");
}
#[cfg(not(feature = "CONFIG_USB_MUSB_TUSB6010"))]
unsafe fn n8x0_usb_init() {}

// The following MMC implementation is present when CONFIG_MENELAUS and
// CONFIG_MMC_OMAP are enabled.
#[cfg(all(feature = "CONFIG_MENELAUS", feature = "CONFIG_MMC_OMAP"))]
static mut slot1_cover_open: i32 = 0;
#[cfg(all(feature = "CONFIG_MENELAUS", feature = "CONFIG_MMC_OMAP"))]
static mut slot2_cover_open: i32 = 0;
#[cfg(all(feature = "CONFIG_MENELAUS", feature = "CONFIG_MMC_OMAP"))]
static mut mmc_device: *mut device = core::ptr::null_mut();

#[cfg(all(feature = "CONFIG_MENELAUS", feature = "CONFIG_MMC_OMAP"))]
static mut mmc1_data: omap_mmc_platform_data = omap_mmc_platform_data {
    nr_slots: 0, init: n8x0_mmc_late_init, cleanup: n8x0_mmc_cleanup,
    shutdown: n8x0_mmc_shutdown, max_freq: 24000000, slots: [],
};
#[cfg(all(feature = "CONFIG_MENELAUS", feature = "CONFIG_MMC_OMAP"))]
static mut mmc_data: [*mut omap_mmc_platform_data; OMAP24XX_NR_MMC] =
    [core::ptr::null_mut(); OMAP24XX_NR_MMC];

#[cfg(not(all(feature = "CONFIG_MENELAUS", feature = "CONFIG_MMC_OMAP")))]
static mut mmc1_data: omap_mmc_platform_data = omap_mmc_platform_data {};

#[cfg(all(feature = "CONFIG_MENELAUS", feature = "CONFIG_MMC_OMAP"))]
unsafe fn n8x0_mmc_set_power_menelaus(dev: *mut device, slot: i32, power_on: i32, vdd: i32) -> i32 {
    let mv = if slot == 0 {
        if power_on == 0 { return menelaus_set_vmmc(0); }
        match 1 << vdd { MMC_VDD_33_34 | MMC_VDD_32_33 | MMC_VDD_31_32 => 3100,
            MMC_VDD_30_31 => 3000, MMC_VDD_28_29 => 2800, MMC_VDD_165_195 => 1850,
            _ => { BUG!(); 0 } }
    } else {
        if power_on == 0 { return menelaus_set_vdcdc(3, 0); }
        match 1 << vdd { MMC_VDD_33_34 | MMC_VDD_32_33 => 3300,
            MMC_VDD_30_31 | MMC_VDD_29_30 => 3000, MMC_VDD_28_29 | MMC_VDD_27_28 => 2800,
            MMC_VDD_24_25 | MMC_VDD_23_24 => 2400, MMC_VDD_22_23 | MMC_VDD_21_22 => 2200,
            MMC_VDD_20_21 => 2000, MMC_VDD_165_195 => 1800, _ => { BUG!(); 0 } }
    };
    if slot == 0 { menelaus_set_vmmc(mv) } else { menelaus_set_vdcdc(3, mv) }
}

#[cfg(all(feature = "CONFIG_MENELAUS", feature = "CONFIG_MMC_OMAP"))]
unsafe fn n8x0_mmc_set_power(dev: *mut device, slot: i32, power_on: i32, vdd: i32) -> i32 {
    if board_is_n800() || slot == 0 { n8x0_mmc_set_power_menelaus(dev, slot, power_on, vdd) } else { 0 }
}

#[cfg(all(feature = "CONFIG_MENELAUS", feature = "CONFIG_MMC_OMAP"))]
unsafe fn n8x0_mmc_set_bus_mode(dev: *mut device, slot: i32, bus_mode: i32) -> i32 {
    BUG_ON!(slot != 0 && slot != 1); let slot = slot + 1;
    let r = match bus_mode { MMC_BUSMODE_OPENDRAIN => menelaus_set_mmc_opendrain(slot, 1),
        MMC_BUSMODE_PUSHPULL => menelaus_set_mmc_opendrain(slot, 0), _ => { BUG!(); 0 } };
    if r != 0 && printk_ratelimit() { dev_err!(dev, "MMC: unable to set bus mode for slot %d\n", slot); } r
}

#[cfg(all(feature = "CONFIG_MENELAUS", feature = "CONFIG_MMC_OMAP"))]
unsafe fn n8x0_mmc_get_cover_state(_dev: *mut device, slot: i32) -> i32 {
    let slot = slot + 1; BUG_ON!(slot != 1 && slot != 2);
    if slot == 1 { slot1_cover_open } else { slot2_cover_open }
}

#[cfg(all(feature = "CONFIG_MENELAUS", feature = "CONFIG_MMC_OMAP"))]
unsafe fn n8x0_mmc_late_init(dev: *mut device) -> i32 {
    mmc_device = dev;
    let mut r = menelaus_set_slot_sel(1); if r < 0 { return r; }
    let vs2sel = if board_is_n800() { 0 } else { 2 };
    r = menelaus_set_mmc_slot(2, 0, vs2sel, 1); if r < 0 { return r; }
    n8x0_mmc_set_power(dev, 0, MMC_POWER_ON, 16);
    n8x0_mmc_set_power(dev, 1, MMC_POWER_ON, 16);
    r = menelaus_set_mmc_slot(1, 1, 0, 1); if r < 0 { return r; }
    r = menelaus_set_mmc_slot(2, 1, vs2sel, 1); if r < 0 { return r; }
    r = menelaus_get_slot_pin_states(); if r < 0 { return r; }
    let (bit, openp) = if board_is_n800() { (1, &mut slot2_cover_open) }
        else { slot2_cover_open = 0; (1, &mut slot1_cover_open) };
    if r == 0xf || r == (0xf & !bit) { r = !r; }
    *openp = if r & bit != 0 { 1 } else { 0 };
    menelaus_register_mmc_callback(n8x0_mmc_callback, core::ptr::null_mut())
}
#[cfg(all(feature = "CONFIG_MENELAUS", feature = "CONFIG_MMC_OMAP"))]
unsafe fn n8x0_mmc_shutdown(_dev: *mut device) {
    let vs2sel = if board_is_n800() { 0 } else { 2 };
    menelaus_set_mmc_slot(1, 0, 0, 0); menelaus_set_mmc_slot(2, 0, vs2sel, 0);
}
#[cfg(all(feature = "CONFIG_MENELAUS", feature = "CONFIG_MMC_OMAP"))]
unsafe fn n8x0_mmc_cleanup(_dev: *mut device) { menelaus_unregister_mmc_callback(); }

#[cfg(all(feature = "CONFIG_MENELAUS", feature = "CONFIG_MMC_OMAP"))]
unsafe fn n8x0_mmc_callback(_data: *mut core::ffi::c_void, card_mask: u8) {
    let (bit, openp, index) = if board_is_n800() { (1, &mut slot2_cover_open, 1) }
        else { (1, &mut slot1_cover_open, 0) };
    *openp = if card_mask as i32 & bit != 0 { 1 } else { 0 };
    omap_mmc_notify_cover_event(mmc_device, index, *openp);
}

// Remaining platform-data initialization and late-init routines retain the
// original kernel objects and callback wiring.
#[cfg(all(feature = "CONFIG_MENELAUS", feature = "CONFIG_MMC_OMAP"))]
unsafe fn n8x0_mmc_init() { mmc1_data.nr_slots = 2; mmc_data[0] = &raw mut mmc1_data; }
#[cfg(not(all(feature = "CONFIG_MENELAUS", feature = "CONFIG_MMC_OMAP")))]
unsafe fn n8x0_mmc_init() {}

unsafe fn n8x0_menelaus_late_init(_dev: *mut device) -> i32 {
    #[cfg(feature = "CONFIG_MENELAUS")] {
        let ret = menelaus_set_vcore_hw(1400, 1050); if ret < 0 { return ret; }
        let val = EN_VPLL_SLEEP | EN_VMMC_SLEEP | EN_VAUX_SLEEP | EN_VIO_SLEEP |
            EN_VMEM_SLEEP | EN_DC3_SLEEP | EN_VC_SLEEP | EN_DC2_SLEEP;
        return menelaus_set_regulator_sleep(1, val);
    }
    0
}

static mut n8x0_menelaus_platform_data: menelaus_platform_data = menelaus_platform_data {
    late_init: n8x0_menelaus_late_init,
};

unsafe fn n8x0_late_initcall() -> i32 {
    if board_caps == 0 { return -ENODEV; }
    n8x0_mmc_init(); n8x0_usb_init();
    gpiod_add_lookup_table(&raw mut nokia810_asoc_gpio_table);
    0
}

unsafe fn n8x0_legacy_init() -> *mut core::ffi::c_void {
    board_check_revision(); &raw mut mmc1_data as *mut core::ffi::c_void
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
