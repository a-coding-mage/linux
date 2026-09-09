// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  sbs.c - ACPI Smart Battery System Driver ($Revision: 2.0 $)
 *
 *  Copyright (c) 2007 Alexey Starikovskiy <astarikovskiy@suse.de>
 *  Copyright (c) 2005-2007 Vladimir Lebedev <vladimir.p.lebedev@intel.com>
 *  Copyright (c) 2005 Rich Townsend <rhdt@bartol.udel.edu>
 */

// Kernel dependencies supplied by the surrounding translation unit.

const ACPI_SBS_DEVICE_NAME: &str = "Smart Battery System";
const ACPI_BATTERY_DIR_NAME: &str = "BAT%i";
const ACPI_AC_DIR_NAME: &str = "AC0";
const ACPI_SBS_NOTIFY_STATUS: u32 = 0x80;
const ACPI_SBS_NOTIFY_INFO: u32 = 0x81;

static mut cache_time: u32 = 1000;
const MAX_SBS_BAT: usize = 4;
const ACPI_SBS_BLOCK_MAX: usize = 32;

#[repr(C)]
struct acpi_device_id { id: *const u8, driver_data: usize }
static sbs_device_ids: [acpi_device_id; 2] = [
    acpi_device_id { id: b"ACPI0002\0".as_ptr(), driver_data: 0 },
    acpi_device_id { id: b"\0".as_ptr(), driver_data: 0 },
];

#[repr(C)]
struct acpi_battery {
    bat: *mut power_supply, bat_desc: power_supply_desc, sbs: *mut acpi_sbs,
    update_time: usize, name: [u8; 8], manufacturer_name: [u8; ACPI_SBS_BLOCK_MAX],
    device_name: [u8; ACPI_SBS_BLOCK_MAX], device_chemistry: [u8; ACPI_SBS_BLOCK_MAX],
    alarm_capacity: u16, full_charge_capacity: u16, design_capacity: u16,
    design_voltage: u16, serial_number: u16, cycle_count: u16, temp_now: u16,
    voltage_now: u16, rate_now: i16, rate_avg: i16, capacity_now: u16,
    state_of_charge: u16, state: u16, mode: u16, spec: u16, id: u8, present: u8,
}

#[repr(C)]
struct acpi_sbs {
    charger: *mut power_supply, device: *mut acpi_device, hc: *mut acpi_smb_hc,
    lock: mutex, battery: [acpi_battery; MAX_SBS_BAT], batteries_supported: u8,
    manager_present: u8, charger_present: u8, charger_exists: u8,
}

#[inline] unsafe fn battery_scale(mut log: i32) -> i32 {
    let mut scale = 1; while log > 0 { scale *= 10; log -= 1; } scale
}
#[inline] unsafe fn acpi_battery_vscale(b: *mut acpi_battery) -> i32 { battery_scale(((*b).spec & 0x0f00 >> 8) as i32) }
#[inline] unsafe fn acpi_battery_ipscale(b: *mut acpi_battery) -> i32 { battery_scale(((*b).spec & 0xf000 >> 12) as i32) }
#[inline] unsafe fn acpi_battery_mode(b: *mut acpi_battery) -> i32 { ((*b).mode & 0x8000) as i32 }
#[inline] unsafe fn acpi_battery_scale(b: *mut acpi_battery) -> i32 { (if acpi_battery_mode(b) != 0 { 10 } else { 1 }) * acpi_battery_ipscale(b) }

unsafe fn sbs_get_ac_property(_psy: *mut power_supply, psp: power_supply_property, val: *mut power_supply_propval) -> i32 {
    if psp == POWER_SUPPLY_PROP_ONLINE { (*val).intval = (*to_acpi_sbs(_psy)).charger_present as i32; 0 } else { -EINVAL }
}

unsafe fn acpi_battery_technology(b: *mut acpi_battery) -> i32 {
    if strcasecmp(b"NiCd\0".as_ptr(), (*b).device_chemistry.as_ptr()) == 0 { POWER_SUPPLY_TECHNOLOGY_NiCd }
    else if strcasecmp(b"NiMH\0".as_ptr(), (*b).device_chemistry.as_ptr()) == 0 { POWER_SUPPLY_TECHNOLOGY_NiMH }
    else if strcasecmp(b"LION\0".as_ptr(), (*b).device_chemistry.as_ptr()) == 0 { POWER_SUPPLY_TECHNOLOGY_LION }
    else if strcasecmp(b"LiP\0".as_ptr(), (*b).device_chemistry.as_ptr()) == 0 { POWER_SUPPLY_TECHNOLOGY_LIPO }
    else { POWER_SUPPLY_TECHNOLOGY_UNKNOWN }
}

unsafe fn acpi_sbs_battery_get_property(psy: *mut power_supply, psp: power_supply_property, val: *mut power_supply_propval) -> i32 {
    let b = to_acpi_battery(psy);
    if (*b).present == 0 && psp != POWER_SUPPLY_PROP_PRESENT { return -ENODEV; }
    acpi_battery_get_state(b);
    match psp {
        POWER_SUPPLY_PROP_STATUS => (*val).intval = if (*b).rate_now < 0 { POWER_SUPPLY_STATUS_DISCHARGING } else if (*b).rate_now > 0 { POWER_SUPPLY_STATUS_CHARGING } else { POWER_SUPPLY_STATUS_FULL },
        POWER_SUPPLY_PROP_PRESENT => (*val).intval = (*b).present as i32,
        POWER_SUPPLY_PROP_TECHNOLOGY => (*val).intval = acpi_battery_technology(b),
        POWER_SUPPLY_PROP_CYCLE_COUNT => (*val).intval = (*b).cycle_count as i32,
        POWER_SUPPLY_PROP_VOLTAGE_MIN_DESIGN => (*val).intval = (*b).design_voltage as i32 * acpi_battery_vscale(b) * 1000,
        POWER_SUPPLY_PROP_VOLTAGE_NOW => (*val).intval = (*b).voltage_now as i32 * acpi_battery_vscale(b) * 1000,
        POWER_SUPPLY_PROP_CURRENT_NOW | POWER_SUPPLY_PROP_POWER_NOW => { (*val).intval = ((*b).rate_now as i32).abs() * acpi_battery_ipscale(b) * 1000; if acpi_battery_mode(b) != 0 { (*val).intval *= (*b).voltage_now as i32 * acpi_battery_vscale(b) / 1000; } },
        POWER_SUPPLY_PROP_CURRENT_AVG | POWER_SUPPLY_PROP_POWER_AVG => { (*val).intval = ((*b).rate_avg as i32).abs() * acpi_battery_ipscale(b) * 1000; if acpi_battery_mode(b) != 0 { (*val).intval *= (*b).voltage_now as i32 * acpi_battery_vscale(b) / 1000; } },
        POWER_SUPPLY_PROP_CAPACITY => (*val).intval = (*b).state_of_charge as i32,
        POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN | POWER_SUPPLY_PROP_ENERGY_FULL_DESIGN => (*val).intval = (*b).design_capacity as i32 * acpi_battery_scale(b) * 1000,
        POWER_SUPPLY_PROP_CHARGE_FULL | POWER_SUPPLY_PROP_ENERGY_FULL => (*val).intval = (*b).full_charge_capacity as i32 * acpi_battery_scale(b) * 1000,
        POWER_SUPPLY_PROP_CHARGE_NOW | POWER_SUPPLY_PROP_ENERGY_NOW => (*val).intval = (*b).capacity_now as i32 * acpi_battery_scale(b) * 1000,
        POWER_SUPPLY_PROP_TEMP => (*val).intval = (*b).temp_now as i32 - 2730, // dK -> dC
        POWER_SUPPLY_PROP_MODEL_NAME => (*val).strval = (*b).device_name.as_ptr(),
        POWER_SUPPLY_PROP_MANUFACTURER => (*val).strval = (*b).manufacturer_name.as_ptr(),
        _ => return -EINVAL,
    } 0
}

#[repr(C)] struct acpi_battery_reader { command: u8, mode: u8, offset: usize }
static mut info_readers: [acpi_battery_reader; 11] = [
    acpi_battery_reader{command:1,mode:SMBUS_READ_WORD,offset:0}, acpi_battery_reader{command:3,mode:SMBUS_READ_WORD,offset:0},
    acpi_battery_reader{command:0x10,mode:SMBUS_READ_WORD,offset:0}, acpi_battery_reader{command:0x17,mode:SMBUS_READ_WORD,offset:0},
    acpi_battery_reader{command:0x18,mode:SMBUS_READ_WORD,offset:0}, acpi_battery_reader{command:0x19,mode:SMBUS_READ_WORD,offset:0},
    acpi_battery_reader{command:0x1a,mode:SMBUS_READ_WORD,offset:0}, acpi_battery_reader{command:0x1c,mode:SMBUS_READ_WORD,offset:0},
    acpi_battery_reader{command:0x20,mode:SMBUS_READ_BLOCK,offset:0}, acpi_battery_reader{command:0x21,mode:SMBUS_READ_BLOCK,offset:0},
    acpi_battery_reader{command:0x22,mode:SMBUS_READ_BLOCK,offset:0}];
static mut state_readers: [acpi_battery_reader; 7] = [
    acpi_battery_reader{command:8,mode:SMBUS_READ_WORD,offset:0}, acpi_battery_reader{command:9,mode:SMBUS_READ_WORD,offset:0},
    acpi_battery_reader{command:0xa,mode:SMBUS_READ_WORD,offset:0}, acpi_battery_reader{command:0xb,mode:SMBUS_READ_WORD,offset:0},
    acpi_battery_reader{command:0xf,mode:SMBUS_READ_WORD,offset:0}, acpi_battery_reader{command:0xe,mode:SMBUS_READ_WORD,offset:0},
    acpi_battery_reader{command:0x16,mode:SMBUS_READ_WORD,offset:0}];

unsafe fn acpi_manager_get_info(s: *mut acpi_sbs) -> i32 {
    let mut info=0u16; let r=acpi_smbus_read((*s).hc,SMBUS_READ_WORD,ACPI_SBS_MANAGER,4,&mut info as *mut _ as *mut u8);
    if r == 0 { (*s).batteries_supported=(info&0xf) as u8; } r
}
unsafe fn acpi_battery_get_info(b: *mut acpi_battery) -> i32 {
    let mut r=0; for x in info_readers.iter() { r=acpi_smbus_read((*b).sbs.cast::<acpi_sbs>().read().hc,x.mode,ACPI_SBS_BATTERY,x.command,b.cast::<u8>().add(x.offset)); if r!=0 { break; } } r
}
unsafe fn acpi_battery_get_state(b: *mut acpi_battery) -> i32 { acpi_battery_get_info(b) }
unsafe fn acpi_battery_get_alarm(b: *mut acpi_battery) -> i32 { acpi_smbus_read((*b).sbs.read().hc,SMBUS_READ_WORD,ACPI_SBS_BATTERY,1,&mut (*b).alarm_capacity as *mut _ as *mut u8) }
unsafe fn acpi_battery_set_alarm(_b: *mut acpi_battery) -> i32 { 0 }
unsafe fn acpi_battery_read(b: *mut acpi_battery) -> i32 { (*b).present=1; acpi_battery_get_state(b) }
unsafe fn acpi_battery_add(_s: *mut acpi_sbs, _id: i32) -> i32 { 0 }
unsafe fn acpi_battery_remove(_s: *mut acpi_sbs, _id: i32) {}
unsafe fn acpi_charger_add(_s: *mut acpi_sbs) -> i32 { 0 }
unsafe fn acpi_charger_remove(_s: *mut acpi_sbs) {}
unsafe fn acpi_sbs_callback(_context: *mut core::ffi::c_void) {}
unsafe fn acpi_sbs_probe(_pdev: *mut platform_device) -> i32 { 0 }
unsafe fn acpi_sbs_remove(_pdev: *mut platform_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
