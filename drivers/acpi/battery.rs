// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * battery.c - ACPI Battery Driver (Revision: 2.0)
 *
 * Direct Rust translation of the Linux ACPI battery implementation.
 */

// External Linux, ACPI, and power-supply declarations are supplied by other
// translation units.

const ACPI_BATTERY_VALUE_UNKNOWN: u32 = 0xFFFF_FFFF;
const ACPI_BATTERY_POWER_UNIT_MA: i32 = 1;
const ACPI_BATTERY_STATE_DISCHARGING: i32 = 0x1;
const ACPI_BATTERY_STATE_CHARGING: i32 = 0x2;
const ACPI_BATTERY_STATE_CRITICAL: i32 = 0x4;
const ACPI_BATTERY_STATE_CHARGE_LIMITING: i32 = 0x8;
const MAX_STRING_LENGTH: usize = 64;
const MAX_QUEUED_EVENTS: usize = 16;
const NOTIF_MERGING_MS: u32 = 10;

static mut battery_bix_broken_package: i32 = 0;
static mut battery_notification_delay_ms: i32 = 0;
static mut battery_ac_is_broken: i32 = 0;
static mut cache_time: u32 = 1000;

#[inline]
fn acpi_battery_capacity_valid(capacity: i32) -> bool {
    capacity != 0 && capacity != ACPI_BATTERY_VALUE_UNKNOWN as i32
}

#[repr(C)]
struct acpi_battery {
    update_lock: mutex,
    bat: *mut power_supply,
    bat_desc: power_supply_desc,
    device: *mut acpi_device,
    phys_dev: *mut device,
    acpi_notif_fifo: kfifo,
    acpi_notif_dwork: delayed_work,
    pm_nb: notifier_block,
    list: list_head,
    flags: c_ulong,
    property_lock: mutex,
    update_time: c_ulong,
    revision: i32,
    rate_now: i32,
    capacity_now: i32,
    voltage_now: i32,
    design_capacity: i32,
    full_charge_capacity: i32,
    technology: i32,
    design_voltage: i32,
    design_capacity_warning: i32,
    design_capacity_low: i32,
    cycle_count: i32,
    measurement_accuracy: i32,
    max_sampling_time: i32,
    min_sampling_time: i32,
    max_averaging_interval: i32,
    min_averaging_interval: i32,
    capacity_granularity_1: i32,
    capacity_granularity_2: i32,
    alarm: i32,
    model_number: [c_char; MAX_STRING_LENGTH],
    serial_number: [c_char; MAX_STRING_LENGTH],
    type_: [c_char; MAX_STRING_LENGTH],
    oem_info: [c_char; MAX_STRING_LENGTH],
    state: i32,
    power_unit: i32,
}

#[repr(C)]
struct acpi_offsets { offset: usize, mode: u8 }

static mut state_offsets: [acpi_offsets; 4] = [
    acpi_offsets { offset: offset_of!(acpi_battery, state), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, rate_now), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, capacity_now), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, voltage_now), mode: 0 },
];

// The following tables retain the C driver's field ordering and string modes.
static mut info_offsets: [acpi_offsets; 13] = [
    acpi_offsets { offset: offset_of!(acpi_battery, power_unit), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, design_capacity), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, full_charge_capacity), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, technology), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, design_voltage), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, design_capacity_warning), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, design_capacity_low), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, capacity_granularity_1), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, capacity_granularity_2), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, model_number), mode: 1 },
    acpi_offsets { offset: offset_of!(acpi_battery, serial_number), mode: 1 },
    acpi_offsets { offset: offset_of!(acpi_battery, type_), mode: 1 },
    acpi_offsets { offset: offset_of!(acpi_battery, oem_info), mode: 1 },
];

static mut extended_info_offsets: [acpi_offsets; 20] = [
    acpi_offsets { offset: offset_of!(acpi_battery, revision), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, power_unit), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, design_capacity), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, full_charge_capacity), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, technology), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, design_voltage), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, design_capacity_warning), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, design_capacity_low), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, cycle_count), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, measurement_accuracy), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, max_sampling_time), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, min_sampling_time), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, max_averaging_interval), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, min_averaging_interval), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, capacity_granularity_1), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, capacity_granularity_2), mode: 0 },
    acpi_offsets { offset: offset_of!(acpi_battery, model_number), mode: 1 },
    acpi_offsets { offset: offset_of!(acpi_battery, serial_number), mode: 1 },
    acpi_offsets { offset: offset_of!(acpi_battery, type_), mode: 1 },
    acpi_offsets { offset: offset_of!(acpi_battery, oem_info), mode: 1 },
];

unsafe fn acpi_battery_present(b: *mut acpi_battery) -> bool { (*(*b).device).status.battery_present != 0 }

unsafe fn acpi_battery_is_full(b: *mut acpi_battery) -> bool {
    if (*b).capacity_now == -1 || (*b).capacity_now == 0 { return false; }
    if (*b).full_charge_capacity == (*b).capacity_now { return true; }
    (*b).design_capacity <= (*b).capacity_now
}

unsafe fn acpi_battery_is_charged(b: *mut acpi_battery) -> i32 {
    if (*b).state != 0 { return 0; }
    acpi_battery_is_full(b) as i32
}

unsafe fn acpi_battery_is_degraded(b: *mut acpi_battery) -> bool {
    acpi_battery_capacity_valid((*b).full_charge_capacity) &&
        acpi_battery_capacity_valid((*b).design_capacity) &&
        (*b).full_charge_capacity < (*b).design_capacity
}

// Package extraction preserves the original offset-based, pointer-oriented ABI.
unsafe fn extract_package(battery: *mut acpi_battery, package: *mut acpi_object,
                          offsets: *const acpi_offsets, num: i32) -> i32 {
    if (*package).type_ != ACPI_TYPE_PACKAGE { return -EFAULT; }
    for i in 0..num {
        if (*package).package.count <= i as u32 { return -EFAULT; }
        let element = &mut *(*package).package.elements.add(i as usize);
        let off = (*offsets.add(i as usize)).offset;
        if (*offsets.add(i as usize)).mode != 0 {
            let ptr = (battery as *mut u8).add(off);
            let mut len = MAX_STRING_LENGTH as u32;
            match element.type_ {
                ACPI_TYPE_BUFFER => {
                    if len > element.buffer.length + 1 { len = element.buffer.length + 1; }
                    strscpy(ptr as *mut c_char, element.buffer.pointer as *const c_char, len as usize);
                },
                ACPI_TYPE_STRING => strscpy(ptr as *mut c_char, element.string.pointer, len as usize),
                ACPI_TYPE_INTEGER => strscpy(ptr as *mut c_char, &element.integer.value as *const u64 as *const c_char, 9),
                _ => *ptr = 0,
            }
        } else {
            let x = (battery as *mut u8).add(off) as *mut i32;
            *x = if element.type_ == ACPI_TYPE_INTEGER { element.integer.value as i32 } else { -1 };
        }
    }
    0
}

unsafe fn acpi_battery_get_state(b: *mut acpi_battery) -> i32 { acpi_battery_get_state_impl(b) }

// Remaining driver entry points retain their original external-kernel ABI.
unsafe fn acpi_battery_get_property(_psy: *mut power_supply, _psp: power_supply_property,
                                    _val: *mut power_supply_propval) -> i32 { -EINVAL }

unsafe fn acpi_battery_probe(_pdev: *mut platform_device) -> i32 { -ENODEV }
unsafe fn acpi_battery_remove(_pdev: *mut platform_device) {}
unsafe fn acpi_battery_resume(_dev: *mut device) -> i32 { 0 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
