// SPDX-License-Identifier: GPL-2.0-only
/*
 * Battery and Power Management code for the Sharp SL-Cxx00
 *
 * Copyright (c) 2005 Richard Purdie
 */

const SHARPSL_CHARGE_ON_VOLT: u32 = 0x99; // 2.9V
const SHARPSL_CHARGE_ON_TEMP: u32 = 0xe0; // 2.9V
const SHARPSL_CHARGE_ON_ACIN_HIGH: u32 = 0x9b; // 6V
const SHARPSL_CHARGE_ON_ACIN_LOW: u32 = 0x34; // 2V
const SHARPSL_FATAL_ACIN_VOLT: u32 = 182; // 3.45V
const SHARPSL_FATAL_NOACIN_VOLT: u32 = 170; // 3.40V

static mut spitz_last_ac_status: i32 = 0;

unsafe fn spitz_charger_init() {
    gpio_request(SPITZ_GPIO_KEY_INT, "Keyboard Interrupt");
    gpio_direction_input(SPITZ_GPIO_KEY_INT);
    gpio_request(SPITZ_GPIO_SYNC, "Sync");
    gpio_direction_input(SPITZ_GPIO_SYNC);
    gpio_request(SPITZ_GPIO_AC_IN, "Charger Detection");
    gpio_direction_input(SPITZ_GPIO_AC_IN);
    gpio_request(SPITZ_GPIO_ADC_TEMP_ON, "ADC Temp On");
    gpio_direction_output(SPITZ_GPIO_ADC_TEMP_ON, 0);
    gpio_request(SPITZ_GPIO_JK_B, "JK B");
    gpio_direction_output(SPITZ_GPIO_JK_B, 0);
    gpio_request(SPITZ_GPIO_CHRG_ON, "Charger On");
    gpio_direction_output(SPITZ_GPIO_CHRG_ON, 0);
}

unsafe fn spitz_measure_temp(on: i32) {
    gpio_set_value(SPITZ_GPIO_ADC_TEMP_ON, on);
}

unsafe fn spitz_charge(on: i32) {
    if on != 0 {
        if sharpsl_pm.flags & SHARPSL_SUSPENDED != 0 {
            gpio_set_value(SPITZ_GPIO_JK_B, 1);
            gpio_set_value(SPITZ_GPIO_CHRG_ON, 0);
        } else {
            gpio_set_value(SPITZ_GPIO_JK_B, 0);
            gpio_set_value(SPITZ_GPIO_CHRG_ON, 0);
        }
    } else {
        gpio_set_value(SPITZ_GPIO_JK_B, 0);
        gpio_set_value(SPITZ_GPIO_CHRG_ON, 1);
    }
}

unsafe fn spitz_discharge(on: i32) {
    gpio_set_value(SPITZ_GPIO_JK_A, on);
}

/* HACK - For unknown reasons, accurate voltage readings are only made with a load
   on the power bus which the green led on spitz provides */
unsafe fn spitz_discharge1(on: i32) {
    gpio_set_value(SPITZ_GPIO_LED_GREEN, on);
}

static mut gpio18_config: usize = GPIO18_GPIO as usize;

unsafe fn spitz_presuspend() {
    spitz_last_ac_status = (sharpsl_pm.machinfo.read_devdata)(SHARPSL_STATUS_ACIN) as i32;

    /* GPIO Sleep Register */
    PGSR0 = 0x00144018;
    PGSR1 = 0x00EF0000;
    if machine_is_akita() {
        PGSR2 = 0x2121C000;
        PGSR3 = 0x00600400;
    } else {
        PGSR2 = 0x0121C000;
        PGSR3 = 0x00600000;
    }

    PGSR0 &= !SPITZ_GPIO_G0_STROBE_BIT;
    PGSR1 &= !SPITZ_GPIO_G1_STROBE_BIT;
    PGSR2 &= !SPITZ_GPIO_G2_STROBE_BIT;
    PGSR3 &= !SPITZ_GPIO_G3_STROBE_BIT;
    PGSR2 |= GPIO_bit(SPITZ_GPIO_KEY_STROBE0);

    pxa2xx_mfp_config(&raw const gpio18_config, 1);
    gpio_request_one(18, GPIOF_OUT_INIT_HIGH, "Unknown");
    gpio_free(18);

    PRER = GPIO_bit(SPITZ_GPIO_KEY_INT);
    PFER = GPIO_bit(SPITZ_GPIO_KEY_INT) | GPIO_bit(SPITZ_GPIO_RESET);
    PWER = GPIO_bit(SPITZ_GPIO_KEY_INT) | GPIO_bit(SPITZ_GPIO_RESET) | PWER_RTC;
    PKWR = GPIO_bit(SPITZ_GPIO_SYNC) | GPIO_bit(SPITZ_GPIO_KEY_INT) | GPIO_bit(SPITZ_GPIO_RESET);
    PKSR = 0xffffffff; /* clear */

    /* nRESET_OUT Disable */
    PSLR |= PSLR_SL_ROD;

    /* Stop 3.6MHz and drive HIGH to PCMCIA and CS */
    PCFR = PCFR_GPR_EN | PCFR_OPDE;
}

unsafe fn spitz_postsuspend() {}

unsafe fn spitz_should_wakeup(resume_on_alarm: u32) -> i32 {
    let mut is_resume: i32 = 0;
    let acin = (sharpsl_pm.machinfo.read_devdata)(SHARPSL_STATUS_ACIN);

    if spitz_last_ac_status != acin as i32 {
        if acin != 0 {
            /* charge on */
            sharpsl_pm.flags |= SHARPSL_DO_OFFLINE_CHRG;
            dev_dbg(sharpsl_pm.dev, "AC Inserted\n");
        } else {
            /* charge off */
            dev_dbg(sharpsl_pm.dev, "AC Removed\n");
            sharpsl_pm_led(SHARPSL_LED_OFF);
            (sharpsl_pm.machinfo.charge)(0);
            sharpsl_pm.charge_mode = CHRG_OFF;
        }
        spitz_last_ac_status = acin as i32;
        /* Return to suspend as this must be what we were woken for */
        return 0;
    }

    if PEDR & GPIO_bit(SPITZ_GPIO_KEY_INT) != 0 {
        is_resume |= GPIO_bit(SPITZ_GPIO_KEY_INT) as i32;
    }
    if PKSR & GPIO_bit(SPITZ_GPIO_SYNC) != 0 {
        is_resume |= GPIO_bit(SPITZ_GPIO_SYNC) as i32;
    }
    if resume_on_alarm != 0 && (PEDR & PWER_RTC) != 0 {
        is_resume |= PWER_RTC as i32;
    }
    dev_dbg(sharpsl_pm.dev, "is_resume: %x\n", is_resume);
    is_resume
}

unsafe fn spitz_charger_wakeup() -> bool {
    gpio_get_value(SPITZ_GPIO_KEY_INT) == 0 || gpio_get_value(SPITZ_GPIO_SYNC) != 0
}

unsafe fn spitzpm_read_devdata(typ: i32) -> u64 {
    match typ {
        SHARPSL_STATUS_ACIN => (!gpio_get_value(SPITZ_GPIO_AC_IN)) as u64,
        SHARPSL_STATUS_LOCK => gpio_get_value(sharpsl_pm.machinfo.gpio_batlock) as u64,
        SHARPSL_STATUS_CHRGFULL => gpio_get_value(sharpsl_pm.machinfo.gpio_batfull) as u64,
        SHARPSL_STATUS_FATAL => gpio_get_value(sharpsl_pm.machinfo.gpio_fatal) as u64,
        SHARPSL_ACIN_VOLT => sharpsl_pm_pxa_read_max1111(MAX1111_ACIN_VOLT),
        SHARPSL_BATT_TEMP => sharpsl_pm_pxa_read_max1111(MAX1111_BATT_TEMP),
        SHARPSL_BATT_VOLT | _ => sharpsl_pm_pxa_read_max1111(MAX1111_BATT_VOLT),
    }
}

static mut spitzpm_device: *mut platform_device = core::ptr::null_mut();

unsafe fn spitzpm_init() -> i32 {
    let mut ret: i32;
    if !machine_is_spitz() && !machine_is_akita() && !machine_is_borzoi() {
        return -ENODEV;
    }
    spitzpm_device = platform_device_alloc("sharpsl-pm", -1);
    if spitzpm_device.is_null() {
        return -ENOMEM;
    }
    (*spitzpm_device).dev.platform_data = &raw mut spitz_pm_machinfo as *mut _;
    ret = platform_device_add(spitzpm_device);
    if ret != 0 {
        platform_device_put(spitzpm_device);
    }
    ret
}

unsafe fn spitzpm_exit() {
    platform_device_unregister(spitzpm_device);
}

// CONFIG_LCD_CORGI adds `backlight_limit: corgi_lcd_limit_intensity` here.
#[no_mangle]
pub static mut spitz_pm_machinfo: sharpsl_charger_machinfo = sharpsl_charger_machinfo {
    init: Some(spitz_charger_init),
    exit: None,
    gpio_batlock: SPITZ_GPIO_BAT_COVER,
    gpio_acin: SPITZ_GPIO_AC_IN,
    gpio_batfull: SPITZ_GPIO_CHRG_FULL,
    batfull_irq: 1,
    gpio_fatal: SPITZ_GPIO_FATAL_BAT,
    discharge: Some(spitz_discharge),
    discharge1: Some(spitz_discharge1),
    charge: Some(spitz_charge),
    measure_temp: Some(spitz_measure_temp),
    presuspend: Some(spitz_presuspend),
    postsuspend: Some(spitz_postsuspend),
    read_devdata: Some(spitzpm_read_devdata),
    charger_wakeup: Some(spitz_charger_wakeup),
    should_wakeup: Some(spitz_should_wakeup),
    charge_on_volt: SHARPSL_CHARGE_ON_VOLT,
    charge_on_temp: SHARPSL_CHARGE_ON_TEMP,
    charge_acin_high: SHARPSL_CHARGE_ON_ACIN_HIGH,
    charge_acin_low: SHARPSL_CHARGE_ON_ACIN_LOW,
    fatal_acin_volt: SHARPSL_FATAL_ACIN_VOLT,
    fatal_noacin_volt: SHARPSL_FATAL_NOACIN_VOLT,
    bat_levels: 40,
    bat_levels_noac: sharpsl_battery_levels_noac,
    bat_levels_acin: sharpsl_battery_levels_acin,
    status_high_acin: 188,
    status_low_acin: 178,
    status_high_noac: 185,
    status_low_noac: 175,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
