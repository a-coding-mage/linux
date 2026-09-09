/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const THERMAL_NAME_LENGTH: usize = 20;
pub const THERMAL_THRESHOLD_WAY_UP: u32 = 0x1;
pub const THERMAL_THRESHOLD_WAY_DOWN: u32 = 0x2;

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum thermal_device_mode {
    THERMAL_DEVICE_DISABLED = 0,
    THERMAL_DEVICE_ENABLED,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum thermal_trip_type {
    THERMAL_TRIP_ACTIVE = 0,
    THERMAL_TRIP_PASSIVE,
    THERMAL_TRIP_HOT,
    THERMAL_TRIP_CRITICAL,
}

/* Adding event notification support elements */
pub const THERMAL_GENL_FAMILY_NAME: &str = "thermal";
pub const THERMAL_GENL_VERSION: u32 = 0x02;
pub const THERMAL_GENL_SAMPLING_GROUP_NAME: &str = "sampling";
pub const THERMAL_GENL_EVENT_GROUP_NAME: &str = "event";

/* Attributes of thermal_genl_family */
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum thermal_genl_attr {
    THERMAL_GENL_ATTR_UNSPEC,
    THERMAL_GENL_ATTR_TZ,
    THERMAL_GENL_ATTR_TZ_ID,
    THERMAL_GENL_ATTR_TZ_TEMP,
    THERMAL_GENL_ATTR_TZ_TRIP,
    THERMAL_GENL_ATTR_TZ_TRIP_ID,
    THERMAL_GENL_ATTR_TZ_TRIP_TYPE,
    THERMAL_GENL_ATTR_TZ_TRIP_TEMP,
    THERMAL_GENL_ATTR_TZ_TRIP_HYST,
    THERMAL_GENL_ATTR_TZ_MODE,
    THERMAL_GENL_ATTR_TZ_NAME,
    THERMAL_GENL_ATTR_TZ_CDEV_WEIGHT,
    THERMAL_GENL_ATTR_TZ_GOV,
    THERMAL_GENL_ATTR_TZ_GOV_NAME,
    THERMAL_GENL_ATTR_CDEV,
    THERMAL_GENL_ATTR_CDEV_ID,
    THERMAL_GENL_ATTR_CDEV_CUR_STATE,
    THERMAL_GENL_ATTR_CDEV_MAX_STATE,
    THERMAL_GENL_ATTR_CDEV_NAME,
    THERMAL_GENL_ATTR_GOV_NAME,
    THERMAL_GENL_ATTR_CPU_CAPABILITY,
    THERMAL_GENL_ATTR_CPU_CAPABILITY_ID,
    THERMAL_GENL_ATTR_CPU_CAPABILITY_PERFORMANCE,
    THERMAL_GENL_ATTR_CPU_CAPABILITY_EFFICIENCY,
    THERMAL_GENL_ATTR_THRESHOLD,
    THERMAL_GENL_ATTR_THRESHOLD_TEMP,
    THERMAL_GENL_ATTR_THRESHOLD_DIRECTION,
    THERMAL_GENL_ATTR_TZ_PREV_TEMP,
    __THERMAL_GENL_ATTR_MAX,
}
pub const THERMAL_GENL_ATTR_MAX: i32 = __THERMAL_GENL_ATTR_MAX as i32 - 1;

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum thermal_genl_sampling {
    THERMAL_GENL_SAMPLING_TEMP,
    __THERMAL_GENL_SAMPLING_MAX,
}
pub const THERMAL_GENL_SAMPLING_MAX: i32 = __THERMAL_GENL_SAMPLING_MAX as i32 - 1;

/* Events of thermal_genl_family */
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum thermal_genl_event {
    THERMAL_GENL_EVENT_UNSPEC,
    THERMAL_GENL_EVENT_TZ_CREATE, /* Thermal zone creation */
    THERMAL_GENL_EVENT_TZ_DELETE, /* Thermal zone deletion */
    THERMAL_GENL_EVENT_TZ_DISABLE, /* Thermal zone disabled */
    THERMAL_GENL_EVENT_TZ_ENABLE, /* Thermal zone enabled */
    THERMAL_GENL_EVENT_TZ_TRIP_UP, /* Trip point crossed the way up */
    THERMAL_GENL_EVENT_TZ_TRIP_DOWN, /* Trip point crossed the way down */
    THERMAL_GENL_EVENT_TZ_TRIP_CHANGE, /* Trip point changed */
    THERMAL_GENL_EVENT_TZ_TRIP_ADD, /* Trip point added */
    THERMAL_GENL_EVENT_TZ_TRIP_DELETE, /* Trip point deleted */
    THERMAL_GENL_EVENT_CDEV_ADD, /* Cdev bound to the thermal zone */
    THERMAL_GENL_EVENT_CDEV_DELETE, /* Cdev unbound */
    THERMAL_GENL_EVENT_CDEV_STATE_UPDATE, /* Cdev state updated */
    THERMAL_GENL_EVENT_TZ_GOV_CHANGE, /* Governor policy changed  */
    THERMAL_GENL_EVENT_CPU_CAPABILITY_CHANGE, /* CPU capability changed */
    THERMAL_GENL_EVENT_THRESHOLD_ADD, /* A thresold has been added */
    THERMAL_GENL_EVENT_THRESHOLD_DELETE, /* A thresold has been deleted */
    THERMAL_GENL_EVENT_THRESHOLD_FLUSH, /* All thresolds have been deleted */
    THERMAL_GENL_EVENT_THRESHOLD_UP, /* A thresold has been crossed the way up */
    THERMAL_GENL_EVENT_THRESHOLD_DOWN, /* A thresold has been crossed the way down */
    __THERMAL_GENL_EVENT_MAX,
}
pub const THERMAL_GENL_EVENT_MAX: i32 = __THERMAL_GENL_EVENT_MAX as i32 - 1;

/* Commands supported by the thermal_genl_family */
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum thermal_genl_cmd {
    THERMAL_GENL_CMD_UNSPEC,
    THERMAL_GENL_CMD_TZ_GET_ID, /* List of thermal zones id */
    THERMAL_GENL_CMD_TZ_GET_TRIP, /* List of thermal trips */
    THERMAL_GENL_CMD_TZ_GET_TEMP, /* Get the thermal zone temperature */
    THERMAL_GENL_CMD_TZ_GET_GOV, /* Get the thermal zone governor */
    THERMAL_GENL_CMD_TZ_GET_MODE, /* Get the thermal zone mode */
    THERMAL_GENL_CMD_CDEV_GET, /* List of cdev id */
    THERMAL_GENL_CMD_THRESHOLD_GET, /* List of thresholds */
    THERMAL_GENL_CMD_THRESHOLD_ADD, /* Add a threshold */
    THERMAL_GENL_CMD_THRESHOLD_DELETE, /* Delete a threshold */
    THERMAL_GENL_CMD_THRESHOLD_FLUSH, /* Flush all the thresholds */
    __THERMAL_GENL_CMD_MAX,
}
pub const THERMAL_GENL_CMD_MAX: i32 = __THERMAL_GENL_CMD_MAX as i32 - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
