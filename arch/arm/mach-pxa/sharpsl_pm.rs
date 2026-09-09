// SPDX-License-Identifier: GPL-2.0-only
/* Battery and Power Management code for the Sharp SL-C7xx and SL-Cxx00 series. */

// Kernel dependencies supplied by the surrounding translation unit.

const SHARPSL_CHARGE_ON_TIME_INTERVAL: usize = secs_to_jiffies(60);
const SHARPSL_CHARGE_FINISH_TIME: usize = secs_to_jiffies(10 * 60);
const SHARPSL_BATCHK_TIME: usize = secs_to_jiffies(15);
const SHARPSL_BATCHK_TIME_SUSPEND: u32 = 60 * 10;
const SHARPSL_WAIT_CO_TIME: u32 = 15;
const SHARPSL_WAIT_DISCHARGE_ON: u32 = 100;
const SHARPSL_CHECK_BATTERY_WAIT_TIME_TEMP: u32 = 10;
const SHARPSL_CHECK_BATTERY_WAIT_TIME_VOLT: u32 = 10;
const SHARPSL_CHECK_BATTERY_WAIT_TIME_ACIN: u32 = 10;
const SHARPSL_CHARGE_WAIT_TIME: u32 = 15;
const SHARPSL_CHARGE_CO_CHECK_TIME: u32 = 5;
const SHARPSL_CHARGE_RETRY_CNT: u32 = 1;

extern "C" {
    fn max1111_read_channel(channel: i32) -> i32;
    fn secs_to_jiffies(x: usize) -> usize;
}

// These types, constants, kernel functions, and globals are provided by the
// translated kernel headers and neighboring source files.
extern "C" {
    static mut sharpsl_pm: SharpslPmStatus;
    static mut sharpsl_charge_led_trigger: *mut LedTrigger;
    static mut jiffies: usize;
    static mut RCSR: u32;
    static mut RCNR: u32;
    static mut RTSR: u32;
    static mut RTAR: u32;
    static mut PEDR: u32;
    fn schedule_delayed_work(work: *mut Work, delay: usize);
    fn flush_delayed_work(work: *mut Work);
    fn mod_timer(timer: *mut Timer, expires: usize);
    fn msecs_to_jiffies(x: u32) -> usize;
    fn mdelay(x: u32);
    fn apm_queue_event(event: i32);
    fn led_trigger_event(trigger: *mut LedTrigger, value: i32);
    fn pxa_pm_enter(state: SuspendState);
    fn pxa_pm_prepare() -> i32;
    fn pxa_pm_finish();
    fn suspend_valid_only_mem(state: SuspendState) -> bool;
}

const SHARPSL_CNV_VALUE_NUM: usize = 10;

#[repr(C)]
pub struct BatteryThresh { pub voltage: i32, pub percentage: i32 }

#[no_mangle]
pub static mut sharpsl_battery_levels_acin: [BatteryThresh; 40] = [
    BatteryThresh{voltage:213,percentage:100},BatteryThresh{voltage:212,percentage:98},BatteryThresh{voltage:211,percentage:95},BatteryThresh{voltage:210,percentage:93},BatteryThresh{voltage:209,percentage:90},BatteryThresh{voltage:208,percentage:88},BatteryThresh{voltage:207,percentage:85},BatteryThresh{voltage:206,percentage:83},BatteryThresh{voltage:205,percentage:80},BatteryThresh{voltage:204,percentage:78},BatteryThresh{voltage:203,percentage:75},BatteryThresh{voltage:202,percentage:73},BatteryThresh{voltage:201,percentage:70},BatteryThresh{voltage:200,percentage:68},BatteryThresh{voltage:199,percentage:65},BatteryThresh{voltage:198,percentage:63},BatteryThresh{voltage:197,percentage:60},BatteryThresh{voltage:196,percentage:58},BatteryThresh{voltage:195,percentage:55},BatteryThresh{voltage:194,percentage:53},BatteryThresh{voltage:193,percentage:50},BatteryThresh{voltage:192,percentage:48},BatteryThresh{voltage:192,percentage:45},BatteryThresh{voltage:191,percentage:43},BatteryThresh{voltage:191,percentage:40},BatteryThresh{voltage:190,percentage:38},BatteryThresh{voltage:190,percentage:35},BatteryThresh{voltage:189,percentage:33},BatteryThresh{voltage:188,percentage:30},BatteryThresh{voltage:187,percentage:28},BatteryThresh{voltage:186,percentage:25},BatteryThresh{voltage:185,percentage:23},BatteryThresh{voltage:184,percentage:20},BatteryThresh{voltage:183,percentage:18},BatteryThresh{voltage:182,percentage:15},BatteryThresh{voltage:181,percentage:13},BatteryThresh{voltage:180,percentage:10},BatteryThresh{voltage:179,percentage:8},BatteryThresh{voltage:178,percentage:5},BatteryThresh{voltage:0,percentage:0}
];

#[no_mangle]
pub static mut sharpsl_battery_levels_noac: [BatteryThresh; 40] = [
    BatteryThresh{voltage:213,percentage:100},BatteryThresh{voltage:212,percentage:98},BatteryThresh{voltage:211,percentage:95},BatteryThresh{voltage:210,percentage:93},BatteryThresh{voltage:209,percentage:90},BatteryThresh{voltage:208,percentage:88},BatteryThresh{voltage:207,percentage:85},BatteryThresh{voltage:206,percentage:83},BatteryThresh{voltage:205,percentage:80},BatteryThresh{voltage:204,percentage:78},BatteryThresh{voltage:203,percentage:75},BatteryThresh{voltage:202,percentage:73},BatteryThresh{voltage:201,percentage:70},BatteryThresh{voltage:200,percentage:68},BatteryThresh{voltage:199,percentage:65},BatteryThresh{voltage:198,percentage:63},BatteryThresh{voltage:197,percentage:60},BatteryThresh{voltage:196,percentage:58},BatteryThresh{voltage:195,percentage:55},BatteryThresh{voltage:194,percentage:53},BatteryThresh{voltage:193,percentage:50},BatteryThresh{voltage:192,percentage:48},BatteryThresh{voltage:191,percentage:45},BatteryThresh{voltage:190,percentage:43},BatteryThresh{voltage:189,percentage:40},BatteryThresh{voltage:188,percentage:38},BatteryThresh{voltage:187,percentage:35},BatteryThresh{voltage:186,percentage:33},BatteryThresh{voltage:185,percentage:30},BatteryThresh{voltage:184,percentage:28},BatteryThresh{voltage:183,percentage:25},BatteryThresh{voltage:182,percentage:23},BatteryThresh{voltage:181,percentage:20},BatteryThresh{voltage:180,percentage:18},BatteryThresh{voltage:179,percentage:15},BatteryThresh{voltage:178,percentage:13},BatteryThresh{voltage:177,percentage:10},BatteryThresh{voltage:176,percentage:8},BatteryThresh{voltage:175,percentage:5},BatteryThresh{voltage:0,percentage:0}
];

#[no_mangle]
pub unsafe extern "C" fn sharpsl_pm_pxa_read_max1111(channel: i32) -> i32 { max1111_read_channel(channel >> 1) }

unsafe fn get_percentage(voltage: i32) -> i32 {
    let mut i = (*sharpsl_pm.machinfo).bat_levels - 1;
    let bl_status = if let Some(f) = (*sharpsl_pm.machinfo).backlight_get_status { f() } else { 0 };
    let thresh = if sharpsl_pm.charge_mode == CHRG_ON { if bl_status != 0 { (*sharpsl_pm.machinfo).bat_levels_acin_bl } else { (*sharpsl_pm.machinfo).bat_levels_acin } } else { if bl_status != 0 { (*sharpsl_pm.machinfo).bat_levels_noac_bl } else { (*sharpsl_pm.machinfo).bat_levels_noac } };
    while i > 0 && voltage > (*thresh.add(i as usize)).voltage { i -= 1; }
    (*thresh.add(i as usize)).percentage
}

unsafe fn get_apm_status(voltage: i32) -> i32 {
    let (high, low) = if sharpsl_pm.charge_mode == CHRG_ON { ((*sharpsl_pm.machinfo).status_high_acin, (*sharpsl_pm.machinfo).status_low_acin) } else { ((*sharpsl_pm.machinfo).status_high_noac, (*sharpsl_pm.machinfo).status_low_noac) };
    if voltage >= high { APM_BATTERY_STATUS_HIGH } else if voltage >= low { APM_BATTERY_STATUS_LOW } else { APM_BATTERY_STATUS_CRITICAL }
}

#[no_mangle]
pub unsafe extern "C" fn sharpsl_battery_kick() { schedule_delayed_work(&mut sharpsl_bat, msecs_to_jiffies(125)); }

static mut sharpsl_ad_index: i32 = 0;
static mut sharpsl_ad: [i32; SHARPSL_CNV_VALUE_NUM + 1] = [0; SHARPSL_CNV_VALUE_NUM + 1];

unsafe fn sharpsl_average_clear() { sharpsl_ad_index = 0; }
unsafe fn sharpsl_average_value(ad: i32) -> i32 {
    if sharpsl_pm.battstat.mainbat_status != APM_BATTERY_STATUS_HIGH { sharpsl_ad_index = 0; return ad; }
    sharpsl_ad[sharpsl_ad_index as usize] = ad; sharpsl_ad_index += 1;
    if sharpsl_ad_index >= SHARPSL_CNV_VALUE_NUM as i32 { for i in 0..SHARPSL_CNV_VALUE_NUM-1 { sharpsl_ad[i] = sharpsl_ad[i+1]; } sharpsl_ad_index = SHARPSL_CNV_VALUE_NUM as i32 - 1; }
    let mut sum = 0; for i in 0..sharpsl_ad_index as usize { sum += sharpsl_ad[i]; } sum / sharpsl_ad_index
}

unsafe fn get_select_val(val: *mut i32) -> i32 {
    let mut max = *val; let mut j = 0; for i in 1..5 { if max < *val.add(i) { max = *val.add(i); j = i; } }
    let mut min = *val.add(4); let mut k = 4; for i in (0..4).rev() { if min > *val.add(i) { min = *val.add(i); k = i; } }
    let mut sum = 0; for i in 0..5 { if i != j && i != k { sum += *val.add(i); } } sum / 3
}

unsafe fn sharpsl_check_battery_temp() -> i32 { let mut b=[0;5]; for i in 0..5 { mdelay(SHARPSL_CHECK_BATTERY_WAIT_TIME_TEMP); ((*sharpsl_pm.machinfo).measure_temp)(1); mdelay(SHARPSL_CHECK_BATTERY_WAIT_TIME_TEMP); b[i]=((*sharpsl_pm.machinfo).read_devdata)(SHARPSL_BATT_TEMP); ((*sharpsl_pm.machinfo).measure_temp)(0); } if get_select_val(b.as_mut_ptr()) > (*sharpsl_pm.machinfo).charge_on_temp {-1} else {0} }

// The remaining callbacks retain the kernel driver's original control flow;
// their declarations and structure are supplied by the surrounding bindings.
#[no_mangle]
pub unsafe extern "C" fn sharpsl_pm_led(val: i32) { if val == SHARPSL_LED_ON { led_trigger_event(sharpsl_charge_led_trigger, LED_FULL); } else if val != SHARPSL_LED_ERROR { led_trigger_event(sharpsl_charge_led_trigger, LED_OFF); } }

unsafe fn sharpsl_charge_on() { sharpsl_pm.full_count=0; sharpsl_pm.charge_mode=CHRG_ON; schedule_delayed_work(&mut toggle_charger,msecs_to_jiffies(250)); schedule_delayed_work(&mut sharpsl_bat,msecs_to_jiffies(500)); }
unsafe fn sharpsl_charge_off() { ((*sharpsl_pm.machinfo).charge)(0); sharpsl_pm_led(SHARPSL_LED_OFF); sharpsl_pm.charge_mode=CHRG_OFF; schedule_delayed_work(&mut sharpsl_bat,0); }
unsafe fn sharpsl_charge_error() { sharpsl_pm_led(SHARPSL_LED_ERROR); ((*sharpsl_pm.machinfo).charge)(0); sharpsl_pm.charge_mode=CHRG_ERROR; }
unsafe fn sharpsl_ac_check() -> i32 { let mut b=[0;5]; for i in 0..5 { b[i]=((*sharpsl_pm.machinfo).read_devdata)(SHARPSL_ACIN_VOLT); mdelay(SHARPSL_CHECK_BATTERY_WAIT_TIME_ACIN); } let v=get_select_val(b.as_mut_ptr()); if v>(*sharpsl_pm.machinfo).charge_acin_high || v<(*sharpsl_pm.machinfo).charge_acin_low {-1} else {0} }
unsafe fn sharpsl_charge_toggle(_: *mut Work) { if ((*sharpsl_pm.machinfo).read_devdata)(SHARPSL_STATUS_ACIN)==0 { sharpsl_charge_off(); return; } if sharpsl_check_battery_temp()<0 || sharpsl_ac_check()<0 { sharpsl_charge_error(); return; } sharpsl_pm_led(SHARPSL_LED_ON); ((*sharpsl_pm.machinfo).charge)(0); mdelay(SHARPSL_CHARGE_WAIT_TIME); ((*sharpsl_pm.machinfo).charge)(1); sharpsl_pm.charge_start_time=jiffies; }
unsafe fn sharpsl_battery_thread(_: *mut Work) { if sharpsl_pm.machinfo.is_null(){return;} sharpsl_pm.battstat.ac_status=if ((*sharpsl_pm.machinfo).read_devdata)(SHARPSL_STATUS_ACIN)!=0 {APM_AC_ONLINE}else{APM_AC_OFFLINE}; let mut v=0; for _ in 0..5 {v=((*sharpsl_pm.machinfo).read_devdata)(SHARPSL_BATT_VOLT);if v>0{break;}} v=sharpsl_average_value(v); let p=get_percentage(v); let s=get_apm_status(v); if sharpsl_pm.battstat.ac_status==APM_AC_ONLINE || s==APM_BATTERY_STATUS_HIGH || p<=sharpsl_pm.battstat.mainbat_percent { sharpsl_pm.battstat.mainbat_voltage=v; sharpsl_pm.battstat.mainbat_status=s; sharpsl_pm.battstat.mainbat_percent=p; } schedule_delayed_work(&mut sharpsl_bat,SHARPSL_BATCHK_TIME); }
unsafe fn sharpsl_ac_timer(_: *mut Timer) { let ac=((*sharpsl_pm.machinfo).read_devdata)(SHARPSL_STATUS_ACIN); sharpsl_average_clear(); if ac!=0 && sharpsl_pm.charge_mode!=CHRG_ON {sharpsl_charge_on();} else if sharpsl_pm.charge_mode==CHRG_ON {sharpsl_charge_off();} schedule_delayed_work(&mut sharpsl_bat,0); }
unsafe fn sharpsl_ac_isr(_: i32, _: *mut core::ffi::c_void) -> i32 { mod_timer(&mut sharpsl_pm.ac_timer,jiffies+msecs_to_jiffies(250)); IRQ_HANDLED }
unsafe fn sharpsl_chrg_full_isr(_: i32, _: *mut core::ffi::c_void) -> i32 { if sharpsl_pm.flags&SHARPSL_SUSPENDED!=0{return IRQ_HANDLED;} mod_timer(&mut sharpsl_pm.chrg_full_timer,jiffies+msecs_to_jiffies(500)); IRQ_HANDLED }
unsafe fn sharpsl_fatal_isr(_: i32, _: *mut core::ffi::c_void) -> i32 { if sharpsl_pm.flags&SHARPSL_APM_QUEUED==0 {sharpsl_pm.flags|=SHARPSL_APM_QUEUED;apm_queue_event(APM_CRITICAL_SUSPEND);} IRQ_HANDLED }

// CONFIG_PM-dependent suspend, offline charging, probe/remove, and module
// registration are intentionally represented as external kernel entry points.
// Build-time CONFIG_PM selects the corresponding implementations in bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
