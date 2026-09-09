/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  hwmon-sysfs.h - hardware monitoring chip driver sysfs defines
 *
 *  Copyright (C) 2005 Yani Ioannou <yani.ioannou@gmail.com>
 */

// C dependencies: <linux/device.h> and <linux/kstrtox.h>.

#[repr(C)]
pub struct sensor_device_attribute {
    pub dev_attr: device_attribute,
    pub index: i32,
}

#[macro_export]
macro_rules! to_sensor_dev_attr {
    ($dev_attr:expr) => {
        container_of_const!($dev_attr, sensor_device_attribute, dev_attr)
    };
}

#[macro_export]
macro_rules! SENSOR_ATTR {
    ($name:ident, $mode:expr, $show:expr, $store:expr, $index:expr) => {
        sensor_device_attribute {
            dev_attr: __DEVICE_ATTR!($name, $mode, $show, $store),
            index: $index,
        }
    };
}

#[macro_export]
macro_rules! SENSOR_ATTR_RO {
    ($name:ident, $func:ident, $index:expr) => {
        SENSOR_ATTR!($name, 0o444, $func##_show, NULL, $index)
    };
}

#[macro_export]
macro_rules! SENSOR_ATTR_RW {
    ($name:ident, $func:ident, $index:expr) => {
        SENSOR_ATTR!($name, 0o644, $func##_show, $func##_store, $index)
    };
}

#[macro_export]
macro_rules! SENSOR_ATTR_WO {
    ($name:ident, $func:ident, $index:expr) => {
        SENSOR_ATTR!($name, 0o200, NULL, $func##_store, $index)
    };
}

#[macro_export]
macro_rules! SENSOR_DEVICE_ATTR {
    ($name:ident, $mode:expr, $show:expr, $store:expr, $index:expr) => {
        pub static mut sensor_dev_attr_$name: sensor_device_attribute =
            SENSOR_ATTR!($name, $mode, $show, $store, $index);
    };
}

#[macro_export]
macro_rules! SENSOR_DEVICE_ATTR_RO {
    ($name:ident, $func:ident, $index:expr) => {
        SENSOR_DEVICE_ATTR!($name, 0o444, $func##_show, NULL, $index)
    };
}

#[macro_export]
macro_rules! SENSOR_DEVICE_ATTR_RW {
    ($name:ident, $func:ident, $index:expr) => {
        SENSOR_DEVICE_ATTR!($name, 0o644, $func##_show, $func##_store, $index)
    };
}

#[macro_export]
macro_rules! SENSOR_DEVICE_ATTR_WO {
    ($name:ident, $func:ident, $index:expr) => {
        SENSOR_DEVICE_ATTR!($name, 0o200, NULL, $func##_store, $index)
    };
}

#[repr(C)]
pub struct sensor_device_attribute_2 {
    pub dev_attr: device_attribute,
    pub index: u8,
    pub nr: u8,
}

#[macro_export]
macro_rules! to_sensor_dev_attr_2 {
    ($dev_attr:expr) => {
        container_of_const!($dev_attr, sensor_device_attribute_2, dev_attr)
    };
}

#[macro_export]
macro_rules! SENSOR_ATTR_2 {
    ($name:ident, $mode:expr, $show:expr, $store:expr, $nr:expr, $index:expr) => {
        sensor_device_attribute_2 {
            dev_attr: __DEVICE_ATTR!($name, $mode, $show, $store),
            index: $index,
            nr: $nr,
        }
    };
}

#[macro_export]
macro_rules! SENSOR_ATTR_2_RO {
    ($name:ident, $func:ident, $nr:expr, $index:expr) => {
        SENSOR_ATTR_2!($name, 0o444, $func##_show, NULL, $nr, $index)
    };
}

#[macro_export]
macro_rules! SENSOR_ATTR_2_RW {
    ($name:ident, $func:ident, $nr:expr, $index:expr) => {
        SENSOR_ATTR_2!($name, 0o644, $func##_show, $func##_store, $nr, $index)
    };
}

#[macro_export]
macro_rules! SENSOR_ATTR_2_WO {
    ($name:ident, $func:ident, $nr:expr, $index:expr) => {
        SENSOR_ATTR_2!($name, 0o200, NULL, $func##_store, $nr, $index)
    };
}

#[macro_export]
macro_rules! SENSOR_DEVICE_ATTR_2 {
    ($name:ident, $mode:expr, $show:expr, $store:expr, $nr:expr, $index:expr) => {
        pub static mut sensor_dev_attr_$name: sensor_device_attribute_2 =
            SENSOR_ATTR_2!($name, $mode, $show, $store, $nr, $index);
    };
}

#[macro_export]
macro_rules! SENSOR_DEVICE_ATTR_2_RO {
    ($name:ident, $func:ident, $nr:expr, $index:expr) => {
        SENSOR_DEVICE_ATTR_2!($name, 0o444, $func##_show, NULL, $nr, $index)
    };
}

#[macro_export]
macro_rules! SENSOR_DEVICE_ATTR_2_RW {
    ($name:ident, $func:ident, $nr:expr, $index:expr) => {
        SENSOR_DEVICE_ATTR_2!($name, 0o644, $func##_show, $func##_store, $nr, $index)
    };
}

#[macro_export]
macro_rules! SENSOR_DEVICE_ATTR_2_WO {
    ($name:ident, $func:ident, $nr:expr, $index:expr) => {
        SENSOR_DEVICE_ATTR_2!($name, 0o200, NULL, $func##_store, $nr, $index)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
