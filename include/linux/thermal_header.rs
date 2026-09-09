/* SPDX-License-Identifier: GPL-2.0 */
/* Direct Rust translation of thermal.h. */

pub const THERMAL_CSTATE_INVALID: c_ulong = !0;
pub const THERMAL_NO_LIMIT: u32 = !0;
pub const THERMAL_WEIGHT_DEFAULT: u32 = 0;
pub const THERMAL_TEMP_INVALID: i32 = -274000;

pub struct thermal_zone_device;
pub struct thermal_cooling_device;
pub struct thermal_instance;
pub struct thermal_debugfs;
pub struct thermal_attr;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum thermal_trend {
    THERMAL_TREND_STABLE,
    THERMAL_TREND_RAISING,
    THERMAL_TREND_DROPPING,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum thermal_notify_event {
    THERMAL_EVENT_UNSPECIFIED,
    THERMAL_EVENT_TEMP_SAMPLE,
    THERMAL_TRIP_VIOLATED,
    THERMAL_TRIP_CHANGED,
    THERMAL_DEVICE_DOWN,
    THERMAL_DEVICE_UP,
    THERMAL_DEVICE_POWER_CAPABILITY_CHANGED,
    THERMAL_TABLE_CHANGED,
    THERMAL_EVENT_KEEP_ALIVE,
    THERMAL_TZ_BIND_CDEV,
    THERMAL_TZ_UNBIND_CDEV,
    THERMAL_INSTANCE_WEIGHT_CHANGED,
    THERMAL_TZ_RESUME,
    THERMAL_TZ_ADD_THRESHOLD,
    THERMAL_TZ_DEL_THRESHOLD,
    THERMAL_TZ_FLUSH_THRESHOLDS,
}

#[repr(C)]
pub struct thermal_trip {
    pub temperature: c_int,
    pub hysteresis: c_int,
    pub type_: thermal_trip_type,
    pub flags: u8,
    pub priv_: *mut c_void,
}

pub const THERMAL_TRIP_FLAG_RW_TEMP: u8 = 1 << 0;
pub const THERMAL_TRIP_FLAG_RW_HYST: u8 = 1 << 1;
pub const THERMAL_TRIP_FLAG_RW: u8 = THERMAL_TRIP_FLAG_RW_TEMP | THERMAL_TRIP_FLAG_RW_HYST;

#[inline]
pub fn THERMAL_TRIP_PRIV_TO_INT(val: *mut c_void) -> uintptr_t { val as uintptr_t }
#[inline]
pub fn THERMAL_INT_TO_TRIP_PRIV(val: uintptr_t) -> *mut c_void { val as *mut c_void }

#[repr(C)]
pub struct cooling_spec {
    pub upper: c_ulong,
    pub lower: c_ulong,
    pub weight: c_uint,
}

#[repr(C)]
pub struct thermal_zone_device_ops {
    pub should_bind: Option<unsafe extern "C" fn(*mut thermal_zone_device, *const thermal_trip, *mut thermal_cooling_device, *mut cooling_spec) -> bool>,
    pub get_temp: Option<unsafe extern "C" fn(*mut thermal_zone_device, *mut c_int) -> c_int>,
    pub set_trips: Option<unsafe extern "C" fn(*mut thermal_zone_device, c_int, c_int) -> c_int>,
    pub change_mode: Option<unsafe extern "C" fn(*mut thermal_zone_device, thermal_device_mode) -> c_int>,
    pub set_trip_temp: Option<unsafe extern "C" fn(*mut thermal_zone_device, *const thermal_trip, c_int) -> c_int>,
    pub get_crit_temp: Option<unsafe extern "C" fn(*mut thermal_zone_device, *mut c_int) -> c_int>,
    pub set_emul_temp: Option<unsafe extern "C" fn(*mut thermal_zone_device, c_int) -> c_int>,
    pub get_trend: Option<unsafe extern "C" fn(*mut thermal_zone_device, *const thermal_trip, *mut thermal_trend) -> c_int>,
    pub hot: Option<unsafe extern "C" fn(*mut thermal_zone_device)>,
    pub critical: Option<unsafe extern "C" fn(*mut thermal_zone_device)>,
}

#[repr(C)]
pub struct thermal_cooling_device_ops {
    pub get_max_state: Option<unsafe extern "C" fn(*mut thermal_cooling_device, *mut c_ulong) -> c_int>,
    pub get_cur_state: Option<unsafe extern "C" fn(*mut thermal_cooling_device, *mut c_ulong) -> c_int>,
    pub set_cur_state: Option<unsafe extern "C" fn(*mut thermal_cooling_device, c_ulong) -> c_int>,
    pub get_requested_power: Option<unsafe extern "C" fn(*mut thermal_cooling_device, *mut u32) -> c_int>,
    pub state2power: Option<unsafe extern "C" fn(*mut thermal_cooling_device, c_ulong, *mut u32) -> c_int>,
    pub power2state: Option<unsafe extern "C" fn(*mut thermal_cooling_device, u32, *mut c_ulong) -> c_int>,
}

#[repr(C)]
pub struct thermal_cooling_device {
    pub id: c_int,
    pub type_: *const c_char,
    pub max_state: c_ulong,
    pub device: device,
    pub devdata: *mut c_void,
    pub stats: *mut c_void,
    pub ops: *const thermal_cooling_device_ops,
    pub updated: bool,
    pub lock: mutex,
    pub thermal_instances: list_head,
    pub node: list_head,
    #[cfg(CONFIG_THERMAL_OF)]
    pub np: *mut device_node,
    #[cfg(CONFIG_THERMAL_OF)]
    pub cdev_id: u32,
    #[cfg(CONFIG_THERMAL_DEBUGFS)]
    pub debugfs: *mut thermal_debugfs,
}

/* DEFINE_GUARD(cooling_dev, ...) is a kernel-only scoped mutex guard macro. */

#[repr(C)]
pub struct thermal_zone_params {
    pub governor_name: *const c_char,
    pub no_hwmon: bool,
    pub sustainable_power: u32,
    pub k_po: i32,
    pub k_pu: i32,
    pub k_i: i32,
    pub k_d: i32,
    pub integral_cutoff: i32,
    pub slope: c_int,
    pub offset: c_int,
}

#[cfg(CONFIG_THERMAL_OF)]
extern "C" {
    pub fn devm_thermal_of_zone_register(dev: *mut device, id: c_int, data: *mut c_void, ops: *const thermal_zone_device_ops) -> *mut thermal_zone_device;
    pub fn devm_thermal_of_zone_unregister(dev: *mut device, tz: *mut thermal_zone_device);
    pub fn thermal_of_cooling_device_register(np: *mut device_node, cdev_id: u32, type_: *const c_char, data: *mut c_void, ops: *const thermal_cooling_device_ops) -> *mut thermal_cooling_device;
    pub fn devm_thermal_of_cooling_device_register(dev: *mut device, cdev_id: u32, type_: *const c_char, devdata: *mut c_void, ops: *const thermal_cooling_device_ops) -> *mut thermal_cooling_device;
    pub fn devm_thermal_of_child_cooling_device_register(dev: *mut device, np: *mut device_node, type_: *const c_char, devdata: *mut c_void, ops: *const thermal_cooling_device_ops) -> *mut thermal_cooling_device;
}

extern "C" {
    pub fn for_each_thermal_trip(tz: *mut thermal_zone_device, cb: Option<unsafe extern "C" fn(*mut thermal_trip, *mut c_void) -> c_int>, data: *mut c_void) -> c_int;
    pub fn thermal_zone_for_each_trip(tz: *mut thermal_zone_device, cb: Option<unsafe extern "C" fn(*mut thermal_trip, *mut c_void) -> c_int>, data: *mut c_void) -> c_int;
    pub fn thermal_zone_set_trip_temp(tz: *mut thermal_zone_device, trip: *mut thermal_trip, temp: c_int);
    pub fn thermal_zone_get_crit_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int;
}

/* CONFIG_THERMAL-dependent declarations are preserved as external interfaces. */
extern "C" {
    pub fn thermal_zone_device_register_with_trips(type_: *const c_char, trips: *const thermal_trip, num_trips: c_int, devdata: *mut c_void, ops: *const thermal_zone_device_ops, tzp: *const thermal_zone_params, passive_delay: c_uint, polling_delay: c_uint) -> *mut thermal_zone_device;
    pub fn thermal_tripless_zone_device_register(type_: *const c_char, devdata: *mut c_void, ops: *const thermal_zone_device_ops, tzp: *const thermal_zone_params) -> *mut thermal_zone_device;
    pub fn thermal_zone_device_unregister(tz: *mut thermal_zone_device);
    pub fn thermal_zone_device_priv(tzd: *mut thermal_zone_device) -> *mut c_void;
    pub fn thermal_zone_device_type(tzd: *mut thermal_zone_device) -> *const c_char;
    pub fn thermal_zone_device_id(tzd: *mut thermal_zone_device) -> c_int;
    pub fn thermal_zone_device(tzd: *mut thermal_zone_device) -> *mut device;
    pub fn thermal_zone_device_update(tz: *mut thermal_zone_device, event: thermal_notify_event);
    pub fn thermal_cooling_device_register(type_: *const c_char, devdata: *mut c_void, ops: *const thermal_cooling_device_ops) -> *mut thermal_cooling_device;
    pub fn devm_thermal_cooling_device_register(dev: *mut device, type_: *const c_char, devdata: *mut c_void, ops: *const thermal_cooling_device_ops) -> *mut thermal_cooling_device;
    pub fn thermal_cooling_device_update(cdev: *mut thermal_cooling_device);
    pub fn thermal_cooling_device_unregister(cdev: *mut thermal_cooling_device);
    pub fn thermal_zone_get_zone_by_name(name: *const c_char) -> *mut thermal_zone_device;
    pub fn thermal_zone_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int;
    pub fn thermal_zone_get_slope(tz: *mut thermal_zone_device) -> c_int;
    pub fn thermal_zone_get_offset(tz: *mut thermal_zone_device) -> c_int;
    pub fn thermal_trip_is_bound_to_cdev(tz: *mut thermal_zone_device, trip: *const thermal_trip, cdev: *mut thermal_cooling_device) -> bool;
    pub fn thermal_zone_device_enable(tz: *mut thermal_zone_device) -> c_int;
    pub fn thermal_zone_device_disable(tz: *mut thermal_zone_device) -> c_int;
    pub fn thermal_zone_device_critical(tz: *mut thermal_zone_device);
    pub fn thermal_pm_prepare();
    pub fn thermal_pm_complete();
}

#[cfg(not(CONFIG_THERMAL_OF))]
#[inline]
pub unsafe fn devm_thermal_of_zone_register(_: *mut device, _: c_int, _: *mut c_void, _: *const thermal_zone_device_ops) -> *mut thermal_zone_device { ERR_PTR(-ENOTSUPP) }
#[cfg(not(CONFIG_THERMAL_OF))]
#[inline]
pub unsafe fn devm_thermal_of_zone_unregister(_: *mut device, _: *mut thermal_zone_device) {}
#[cfg(not(CONFIG_THERMAL_OF))]
#[inline]
pub unsafe fn thermal_of_cooling_device_register(_: *mut device_node, _: u32, _: *const c_char, _: *mut c_void, _: *const thermal_cooling_device_ops) -> *mut thermal_cooling_device { ERR_PTR(-ENODEV) }
#[cfg(not(CONFIG_THERMAL_OF))]
#[inline]
pub unsafe fn devm_thermal_of_cooling_device_register(_: *mut device, _: u32, _: *const c_char, _: *mut c_void, _: *const thermal_cooling_device_ops) -> *mut thermal_cooling_device { ERR_PTR(-ENODEV) }
#[cfg(not(CONFIG_THERMAL_OF))]
#[inline]
pub unsafe fn devm_thermal_of_child_cooling_device_register(_: *mut device, _: *mut device_node, _: *const c_char, _: *mut c_void, _: *const thermal_cooling_device_ops) -> *mut thermal_cooling_device { ERR_PTR(-ENODEV) }

/* When CONFIG_THERMAL is disabled, the C header supplies inline ENODEV/NULL stubs. */
#[cfg(not(CONFIG_THERMAL))]
#[inline]
pub unsafe fn thermal_zone_device_register_with_trips(_: *const c_char, _: *const thermal_trip, _: c_int, _: *mut c_void, _: *const thermal_zone_device_ops, _: *const thermal_zone_params, _: c_int, _: c_int) -> *mut thermal_zone_device { ERR_PTR(-ENODEV) }
#[cfg(not(CONFIG_THERMAL))]
#[inline]
pub unsafe fn thermal_tripless_zone_device_register(_: *const c_char, _: *mut c_void, _: *mut thermal_zone_device_ops, _: *const thermal_zone_params) -> *mut thermal_zone_device { ERR_PTR(-ENODEV) }
#[cfg(not(CONFIG_THERMAL))]
#[inline]
pub unsafe fn thermal_zone_device_unregister(_: *mut thermal_zone_device) {}
#[cfg(not(CONFIG_THERMAL))]
#[inline]
pub unsafe fn thermal_zone_device_update(_: *mut thermal_zone_device, _: thermal_notify_event) {}
#[cfg(not(CONFIG_THERMAL))]
#[inline]
pub unsafe fn thermal_cooling_device_register(_: *const c_char, _: *mut c_void, _: *const thermal_cooling_device_ops) -> *mut thermal_cooling_device { ERR_PTR(-ENODEV) }
#[cfg(not(CONFIG_THERMAL))]
#[inline]
pub unsafe fn devm_thermal_cooling_device_register(_: *mut device, _: *const c_char, _: *mut c_void, _: *const thermal_cooling_device_ops) -> *mut thermal_cooling_device { ERR_PTR(-ENODEV) }
#[cfg(not(CONFIG_THERMAL))]
#[inline]
pub unsafe fn thermal_cooling_device_unregister(_: *mut thermal_cooling_device) {}
#[cfg(not(CONFIG_THERMAL))]
#[inline]
pub unsafe fn thermal_zone_get_zone_by_name(_: *const c_char) -> *mut thermal_zone_device { ERR_PTR(-ENODEV) }
#[cfg(not(CONFIG_THERMAL))]
#[inline]
pub unsafe fn thermal_zone_get_temp(_: *mut thermal_zone_device, _: *mut c_int) -> c_int { -ENODEV }
#[cfg(not(CONFIG_THERMAL))]
#[inline]
pub unsafe fn thermal_zone_get_slope(_: *mut thermal_zone_device) -> c_int { -ENODEV }
#[cfg(not(CONFIG_THERMAL))]
#[inline]
pub unsafe fn thermal_zone_get_offset(_: *mut thermal_zone_device) -> c_int { -ENODEV }
#[cfg(not(CONFIG_THERMAL))]
#[inline]
pub unsafe fn thermal_zone_device_priv(_: *mut thermal_zone_device) -> *mut c_void { core::ptr::null_mut() }
#[cfg(not(CONFIG_THERMAL))]
#[inline]
pub unsafe fn thermal_zone_device_type(_: *mut thermal_zone_device) -> *const c_char { core::ptr::null() }
#[cfg(not(CONFIG_THERMAL))]
#[inline]
pub unsafe fn thermal_zone_device_id(_: *mut thermal_zone_device) -> c_int { -ENODEV }
#[cfg(not(CONFIG_THERMAL))]
#[inline]
pub unsafe fn thermal_zone_device_enable(_: *mut thermal_zone_device) -> c_int { -ENODEV }
#[cfg(not(CONFIG_THERMAL))]
#[inline]
pub unsafe fn thermal_zone_device_disable(_: *mut thermal_zone_device) -> c_int { -ENODEV }
#[cfg(not(CONFIG_THERMAL))]
#[inline]
pub unsafe fn thermal_pm_prepare() {}
#[cfg(not(CONFIG_THERMAL))]
#[inline]
pub unsafe fn thermal_pm_complete() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
