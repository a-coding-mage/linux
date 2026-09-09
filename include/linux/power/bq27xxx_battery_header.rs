/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <linux/power_supply.h>

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum bq27xxx_chip {
    BQ27000 = 1, /* bq27000, bq27200 */
    BQ27010, /* bq27010, bq27210 */
    BQ2750X, /* bq27500 deprecated alias */
    BQ2751X, /* bq27510, bq27520 deprecated alias */
    BQ2752X,
    BQ27500, /* bq27500/1 */
    BQ27510G1, /* bq27510G1 */
    BQ27510G2, /* bq27510G2 */
    BQ27510G3, /* bq27510G3 */
    BQ27520G1, /* bq27520G1 */
    BQ27520G2, /* bq27520G2 */
    BQ27520G3, /* bq27520G3 */
    BQ27520G4, /* bq27520G4 */
    BQ27521, /* bq27521 */
    BQ27530, /* bq27530, bq27531 */
    BQ27531,
    BQ27541, /* bq27541, bq27542, bq27546, bq27742 */
    BQ27542,
    BQ27546,
    BQ27742,
    BQ27545, /* bq27545 */
    BQ27411,
    BQ27421, /* bq27421, bq27441, bq27621 */
    BQ27425,
    BQ27426,
    BQ27441,
    BQ27621,
    BQ27Z561,
    BQ28Z610,
    BQ34Z100,
    BQ78Z100,
}

#[repr(C)]
pub struct bq27xxx_access_methods {
    pub read: Option<unsafe extern "C" fn(di: *mut bq27xxx_device_info, reg: u8, single: bool) -> i32>,
    pub write: Option<unsafe extern "C" fn(di: *mut bq27xxx_device_info, reg: u8, value: i32, single: bool) -> i32>,
    pub read_bulk: Option<unsafe extern "C" fn(di: *mut bq27xxx_device_info, reg: u8, data: *mut u8, len: i32) -> i32>,
    pub write_bulk: Option<unsafe extern "C" fn(di: *mut bq27xxx_device_info, reg: u8, data: *mut u8, len: i32) -> i32>,
}

#[repr(C)]
pub struct bq27xxx_reg_cache {
    pub capacity: i32,
    pub flags: i32,
}

// External Linux kernel types supplied by other files.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bq27xxx_dm_reg {
    _private: [u8; 0],
}
#[repr(C)]
pub struct delayed_work {
    _private: [u8; 0],
}
#[repr(C)]
pub struct power_supply {
    _private: [u8; 0],
}
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub union power_supply_propval {
    pub intval: i32,
    pub strval: *const u8,
}
#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bq27xxx_device_info {
    pub dev: *mut device,
    pub chip: bq27xxx_chip,
    pub opts: u32,
    pub name: *const u8,
    pub dm_regs: *mut bq27xxx_dm_reg,
    pub unseal_key: u32,
    pub bus: bq27xxx_access_methods,
    pub cache: bq27xxx_reg_cache,
    pub charge_design_full: i32,
    pub voltage_min_design: i32,
    pub voltage_max_design: i32,
    pub removed: bool,
    pub last_update: usize,
    pub last_status: power_supply_propval,
    pub work: delayed_work,
    pub bat: *mut power_supply,
    pub list: list_head,
    pub lock: mutex,
    pub regs: *mut u8,
}

unsafe extern "C" {
    pub fn bq27xxx_battery_update(di: *mut bq27xxx_device_info);
    pub fn bq27xxx_battery_setup(di: *mut bq27xxx_device_info) -> i32;
    pub fn bq27xxx_battery_teardown(di: *mut bq27xxx_device_info);
    pub static bq27xxx_battery_battery_pm_ops: dev_pm_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
