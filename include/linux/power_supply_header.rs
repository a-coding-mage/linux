/* SPDX-License-Identifier: GPL-2.0-only */
/* Translation of linux/power_supply.h. Kernel-provided types and functions remain external dependencies. */

use core::ffi::{c_char, c_void};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PowerSupplyStatus { Unknown = 0, Charging, Discharging, NotCharging, Full }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum power_supply_charge_type {
    Unknown = 0, None, Trickle, Fast, Standard, Adaptive, Custom, Longlife, Bypass,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PowerSupplyHealth {
    Unknown = 0, Good, Overheat, Dead, Overvoltage, Undervoltage, UnspecFailure, Cold,
    WatchdogTimerExpire, SafetyTimerExpire, Overcurrent, CalibrationRequired, Warm, Cool,
    Hot, NoBattery, BlownFuse, CellImbalance,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PowerSupplyTechnology {
    Unknown = 0, NiMH, LION, LIPO, LiFe, NiCd, LiMn, PbAc, NiZn, RAM, ZnAr,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PowerSupplyCapacityLevel { Unknown = 0, Critical, Low, Normal, High, Full }
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PowerSupplyScope { Unknown = 0, System, Device }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum power_supply_property {
    Status = 0, ChargeType, ChargeTypes, Health, Present, Online, Authentic, Technology,
    CycleCount, VoltageMax, VoltageMin, VoltageMaxDesign, VoltageMinDesign, VoltageNow,
    VoltageAvg, VoltageOcv, VoltageBoot, CurrentMax, CurrentNow, CurrentAvg, CurrentBoot,
    PowerNow, PowerAvg, ChargeFullDesign, ChargeEmptyDesign, ChargeFull, ChargeEmpty,
    ChargeNow, ChargeAvg, ChargeCounter, ConstantChargeCurrent, ConstantChargeCurrentMax,
    ConstantChargeVoltage, ConstantChargeVoltageMax, ChargeControlLimit, ChargeControlLimitMax,
    ChargeControlStartThreshold, ChargeControlEndThreshold, ChargeBehaviour, InputCurrentLimit,
    InputVoltageLimit, InputPowerLimit, EnergyFullDesign, EnergyEmptyDesign, EnergyFull,
    EnergyEmpty, EnergyNow, EnergyAvg, Capacity, CapacityAlertMin, CapacityAlertMax,
    CapacityErrorMargin, CapacityLevel, Temp, TempMax, TempMin, TempAlertMin, TempAlertMax,
    TempAmbient, TempAmbientAlertMin, TempAmbientAlertMax, TimeToEmptyNow, TimeToEmptyAvg,
    TimeToFullNow, TimeToFullAvg, Type, UsbType, Scope, PrechargeCurrent, ChargeTermCurrent,
    Calibrate, ManufactureYear, ManufactureMonth, ManufactureDay, InternalResistance,
    StateOfHealth, ModelName, Manufacturer, SerialNumber,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum power_supply_type { Unknown = 0, Battery, Ups, Mains, Usb, UsbDcp, UsbCdp, UsbAca, UsbTypeC, UsbPd, UsbPdDrp, AppleBrickId, Wireless }
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum power_supply_usb_type { Unknown = 0, Sdp, Dcp, Cdp, Aca, C, Pd, PdDrp, PdPps, PdSprAvs, PdPpsSprAvs, AppleBrickId }
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum power_supply_charge_behaviour { Auto = 0, InhibitCharge, InhibitChargeAwake, ForceDischarge }
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum power_supply_notifier_events { PropChanged = 0 }

#[repr(C)]
pub union power_supply_propval { pub intval: i32, pub strval: *const c_char }

pub enum device_node {}
pub enum fwnode_handle {}
pub enum attribute_group {}
pub enum notifier_block {}
pub enum device {}
pub enum work_struct {}
pub enum delayed_work {}
pub enum spinlock_t {}
pub enum atomic_t {}
pub enum rw_semaphore {}
pub enum list_head {}
pub enum led_trigger {}
pub enum thermal_zone_device {}
pub enum thermal_cooling_device {}

#[repr(C)]
pub struct power_supply_config {
    pub fwnode: *mut fwnode_handle,
    pub drv_data: *mut c_void,
    pub attr_grp: *const *const attribute_group,
    pub supplied_to: *mut *mut c_char,
    pub num_supplicants: usize,
    pub no_wakeup_source: bool,
}

#[repr(C)]
pub struct power_supply_desc {
    pub name: *const c_char, pub type_: power_supply_type, pub charge_behaviours: u8,
    pub charge_types: u32, pub usb_types: u32, pub properties: *const power_supply_property,
    pub num_properties: usize,
    pub get_property: Option<unsafe extern "C" fn(*mut power_supply, power_supply_property, *mut power_supply_propval) -> i32>,
    pub set_property: Option<unsafe extern "C" fn(*mut power_supply, power_supply_property, *const power_supply_propval) -> i32>,
    pub property_is_writeable: Option<unsafe extern "C" fn(*mut power_supply, power_supply_property) -> i32>,
    pub external_power_changed: Option<unsafe extern "C" fn(*mut power_supply)>,
    pub init: Option<unsafe extern "C" fn(*mut power_supply) -> i32>,
    pub no_thermal: bool, pub use_for_apm: i32,
}

#[repr(C)]
pub struct power_supply_ext {
    pub name: *const c_char, pub charge_behaviours: u8, pub charge_types: u32,
    pub properties: *const power_supply_property, pub num_properties: usize,
    pub get_property: Option<unsafe extern "C" fn(*mut power_supply, *const power_supply_ext, *mut c_void, power_supply_property, *mut power_supply_propval) -> i32>,
    pub set_property: Option<unsafe extern "C" fn(*mut power_supply, *const power_supply_ext, *mut c_void, power_supply_property, *const power_supply_propval) -> i32>,
    pub property_is_writeable: Option<unsafe extern "C" fn(*mut power_supply, *const power_supply_ext, *mut c_void, power_supply_property) -> i32>,
}

#[repr(C)]
pub struct power_supply {
    pub desc: *const power_supply_desc, pub supplied_to: *mut *mut c_char, pub num_supplicants: usize,
    pub supplied_from: *mut *mut c_char, pub num_supplies: usize, pub drv_data: *mut c_void,
    pub dev: device, pub changed_work: work_struct, pub deferred_register_work: delayed_work,
    pub changed_lock: spinlock_t, pub changed: bool, pub update_groups: bool, pub initialized: bool,
    pub removing: bool, pub use_cnt: atomic_t, pub battery_info: *mut power_supply_battery_info,
    pub extensions_sem: rw_semaphore, pub extensions: list_head,
    #[cfg(CONFIG_THERMAL)] pub tzd: *mut thermal_zone_device,
    #[cfg(CONFIG_THERMAL)] pub tcd: *mut thermal_cooling_device,
    #[cfg(CONFIG_LEDS_TRIGGERS)] pub charging_or_full_trig: *mut led_trigger,
    #[cfg(CONFIG_LEDS_TRIGGERS)] pub online_trig: *mut led_trigger,
    #[cfg(CONFIG_LEDS_TRIGGERS)] pub charging_trig: *mut led_trigger,
    #[cfg(CONFIG_LEDS_TRIGGERS)] pub full_trig: *mut led_trigger,
    #[cfg(CONFIG_LEDS_TRIGGERS)] pub charging_blink_full_solid_trig: *mut led_trigger,
    #[cfg(CONFIG_LEDS_TRIGGERS)] pub charging_orange_full_green_trig: *mut led_trigger,
}

#[repr(C)] pub struct power_supply_info { pub name: *const c_char, pub technology: i32, pub voltage_max_design: i32, pub voltage_min_design: i32, pub charge_full_design: i32, pub charge_empty_design: i32, pub energy_full_design: i32, pub energy_empty_design: i32, pub use_for_apm: i32 }
#[repr(C)] pub struct power_supply_battery_ocv_table { pub ocv: i32, pub capacity: i32 }
#[repr(C)] pub struct power_supply_resistance_temp_table { pub temp: i32, pub resistance: i32 }
#[repr(C)] pub struct power_supply_vbat_ri_table { pub vbat_uv: i32, pub ri_uohm: i32 }
#[repr(C)] pub struct power_supply_maintenance_charge_table { pub charge_current_max_ua: i32, pub charge_voltage_max_uv: i32, pub charge_safety_timer_minutes: i32 }

pub const POWER_SUPPLY_OCV_TEMP_MAX: usize = 20;
#[repr(C)]
pub struct power_supply_battery_info {
    pub technology: u32, pub energy_full_design_uwh: i32, pub charge_full_design_uah: i32,
    pub voltage_min_design_uv: i32, pub voltage_max_design_uv: i32, pub tricklecharge_current_ua: i32,
    pub precharge_current_ua: i32, pub precharge_voltage_max_uv: i32, pub charge_term_current_ua: i32,
    pub charge_restart_voltage_uv: i32, pub overvoltage_limit_uv: i32, pub constant_charge_current_max_ua: i32,
    pub constant_charge_voltage_max_uv: i32, pub maintenance_charge: *const power_supply_maintenance_charge_table,
    pub maintenance_charge_size: i32, pub alert_low_temp_charge_current_ua: i32, pub alert_low_temp_charge_voltage_uv: i32,
    pub alert_high_temp_charge_current_ua: i32, pub alert_high_temp_charge_voltage_uv: i32,
    pub factory_internal_resistance_uohm: i32, pub factory_internal_resistance_charging_uohm: i32,
    pub ocv_temp: [i32; 20], pub temp_ambient_alert_min: i32, pub temp_ambient_alert_max: i32,
    pub temp_alert_min: i32, pub temp_alert_max: i32, pub temp_min: i32, pub temp_max: i32,
    pub ocv_table: [*const power_supply_battery_ocv_table; 20], pub ocv_table_size: [i32; 20],
    pub resist_table: *const power_supply_resistance_temp_table, pub resist_table_size: i32,
    pub vbat2ri_discharging: *const power_supply_vbat_ri_table, pub vbat2ri_discharging_size: i32,
    pub vbat2ri_charging: *const power_supply_vbat_ri_table, pub vbat2ri_charging_size: i32,
    pub bti_resistance_ohm: i32, pub bti_resistance_tolerance: i32,
}

extern "C" {
    pub fn power_supply_get_maintenance_charging_setting(info: *mut power_supply_battery_info, index: i32) -> *const power_supply_maintenance_charge_table;
    pub fn power_supply_reg_notifier(nb: *mut notifier_block) -> i32;
    pub fn power_supply_unreg_notifier(nb: *mut notifier_block);
    pub fn power_supply_get_by_name(name: *const c_char) -> *mut power_supply;
    pub fn power_supply_get_system_batteries(dev: *mut device, psys: *mut *mut *mut power_supply) -> i32;
    pub fn power_supply_put_system_batteries(psys: *mut *mut power_supply, count: i32);
    pub fn power_supply_put(psy: *mut power_supply);
    pub fn power_supply_get_by_reference(fwnode: *mut fwnode_handle, property: *const c_char) -> *mut power_supply;
    pub fn devm_power_supply_get_by_reference(dev: *mut device, property: *const c_char) -> *mut power_supply;
    pub static power_supply_battery_info_properties: [power_supply_property; 0];
    pub static power_supply_battery_info_properties_size: usize;
    pub fn power_supply_get_battery_info(psy: *mut power_supply, info_out: *mut *mut power_supply_battery_info) -> i32;
    pub fn power_supply_put_battery_info(psy: *mut power_supply, info: *mut power_supply_battery_info);
    pub fn power_supply_battery_info_has_prop(info: *mut power_supply_battery_info, psp: power_supply_property) -> bool;
    pub fn power_supply_battery_info_get_prop(info: *mut power_supply_battery_info, psp: power_supply_property, val: *mut power_supply_propval) -> i32;
    pub fn power_supply_ocv2cap_simple(table: *const power_supply_battery_ocv_table, table_len: i32, ocv: i32) -> i32;
    pub fn power_supply_find_ocv2cap_table(info: *mut power_supply_battery_info, temp: i32, table_len: *mut i32) -> *const power_supply_battery_ocv_table;
    pub fn power_supply_batinfo_ocv2cap(info: *mut power_supply_battery_info, ocv: i32, temp: i32) -> i32;
    pub fn power_supply_temp2resist_simple(table: *const power_supply_resistance_temp_table, table_len: i32, temp: i32) -> i32;
    pub fn power_supply_vbat2ri(info: *mut power_supply_battery_info, vbat_uv: i32, charging: bool) -> i32;
    pub fn power_supply_battery_bti_in_range(info: *mut power_supply_battery_info, resistance: i32) -> bool;
    pub fn power_supply_changed(psy: *mut power_supply);
    pub fn power_supply_am_i_supplied(psy: *mut power_supply) -> i32;
    pub fn power_supply_get_property_from_supplier(psy: *mut power_supply, psp: power_supply_property, val: *mut power_supply_propval) -> i32;
    pub fn power_supply_get_property(psy: *mut power_supply, psp: power_supply_property, val: *mut power_supply_propval) -> i32;
    pub fn power_supply_get_property_direct(psy: *mut power_supply, psp: power_supply_property, val: *mut power_supply_propval) -> i32;
    pub fn power_supply_set_property(psy: *mut power_supply, psp: power_supply_property, val: *const power_supply_propval) -> i32;
    pub fn power_supply_set_property_direct(psy: *mut power_supply, psp: power_supply_property, val: *const power_supply_propval) -> i32;
    pub fn power_supply_external_power_changed(psy: *mut power_supply);
    pub fn power_supply_register(parent: *mut device, desc: *const power_supply_desc, cfg: *const power_supply_config) -> *mut power_supply;
    pub fn devm_power_supply_register(parent: *mut device, desc: *const power_supply_desc, cfg: *const power_supply_config) -> *mut power_supply;
    pub fn power_supply_unregister(psy: *mut power_supply);
    pub fn power_supply_powers(psy: *mut power_supply, dev: *mut device) -> i32;
    pub fn power_supply_register_extension(psy: *mut power_supply, ext: *const power_supply_ext, dev: *mut device, data: *mut c_void) -> i32;
    pub fn power_supply_unregister_extension(psy: *mut power_supply, ext: *const power_supply_ext);
    pub fn power_supply_get_drvdata(psy: *mut power_supply) -> *mut c_void;
    pub fn power_supply_for_each_psy(data: *mut c_void, f: Option<unsafe extern "C" fn(*mut power_supply, *mut c_void) -> i32>) -> i32;
    pub fn power_supply_is_system_supplied() -> i32;
    pub fn power_supply_charge_behaviour_show(dev: *mut device, available_behaviours: u32, behaviour: power_supply_charge_behaviour, buf: *mut c_char) -> isize;
    pub fn power_supply_charge_behaviour_parse(available_behaviours: u32, buf: *const c_char) -> i32;
    pub fn power_supply_charge_types_show(dev: *mut device, available_types: u32, current_type: power_supply_charge_type, buf: *mut c_char) -> isize;
    pub fn power_supply_charge_types_parse(available_types: u32, buf: *const c_char) -> i32;
}

#[inline] pub unsafe fn power_supply_supports_maintenance_charging(info: *mut power_supply_battery_info) -> bool { !power_supply_get_maintenance_charging_setting(info, 0).is_null() }
#[inline] pub unsafe fn power_supply_supports_vbat2ri(info: *mut power_supply_battery_info) -> bool { !(*info).vbat2ri_discharging.is_null() && (*info).vbat2ri_discharging_size > 0 }
#[inline] pub unsafe fn power_supply_supports_temp2ri(info: *mut power_supply_battery_info) -> bool { !(*info).resist_table.is_null() && (*info).resist_table_size > 0 }

#[inline] pub const fn power_supply_is_amp_property(psp: power_supply_property) -> bool { matches!(psp,
    power_supply_property::ChargeFullDesign | power_supply_property::ChargeEmptyDesign | power_supply_property::ChargeFull |
    power_supply_property::ChargeEmpty | power_supply_property::ChargeNow | power_supply_property::ChargeAvg |
    power_supply_property::ChargeCounter | power_supply_property::PrechargeCurrent | power_supply_property::ChargeTermCurrent |
    power_supply_property::ConstantChargeCurrent | power_supply_property::ConstantChargeCurrentMax |
    power_supply_property::CurrentMax | power_supply_property::CurrentNow | power_supply_property::CurrentAvg |
    power_supply_property::CurrentBoot) }

#[inline] pub const fn power_supply_is_watt_property(psp: power_supply_property) -> bool { matches!(psp,
    power_supply_property::EnergyFullDesign | power_supply_property::EnergyEmptyDesign | power_supply_property::EnergyFull |
    power_supply_property::EnergyEmpty | power_supply_property::EnergyNow | power_supply_property::EnergyAvg |
    power_supply_property::VoltageMax | power_supply_property::VoltageMin | power_supply_property::VoltageMaxDesign |
    power_supply_property::VoltageMinDesign | power_supply_property::VoltageNow | power_supply_property::VoltageAvg |
    power_supply_property::VoltageOcv | power_supply_property::VoltageBoot | power_supply_property::ConstantChargeVoltage |
    power_supply_property::ConstantChargeVoltageMax | power_supply_property::PowerNow) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
