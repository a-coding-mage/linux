/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Regulator uapi header
 *
 * Author: Naresh Solanki <Naresh.Solanki@9elements.com>
 */

/* Dependency equivalent of <linux/types.h>. */

/*
 * Regulator notifier events.
 *
 * UNDER_VOLTAGE  Regulator output is under voltage.
 * OVER_CURRENT   Regulator output current is too high.
 * REGULATION_OUT Regulator output is out of regulation.
 * FAIL           Regulator output has failed.
 * OVER_TEMP      Regulator over temp.
 * FORCE_DISABLE  Regulator forcibly shut down by software.
 * VOLTAGE_CHANGE Regulator voltage changed.
 *                Data passed is old voltage cast to (void *).
 * DISABLE        Regulator was disabled.
 * PRE_VOLTAGE_CHANGE   Regulator is about to have voltage changed.
 *                      Data passed is "struct pre_voltage_change_data"
 * ABORT_VOLTAGE_CHANGE Regulator voltage change failed for some reason.
 *                      Data passed is old voltage cast to (void *).
 * PRE_DISABLE    Regulator is about to be disabled
 * ABORT_DISABLE  Regulator disable failed for some reason
 *
 * NOTE: These events can be OR'ed together when passed into handler.
 */

pub const REGULATOR_EVENT_UNDER_VOLTAGE: u32 = 0x01;
pub const REGULATOR_EVENT_OVER_CURRENT: u32 = 0x02;
pub const REGULATOR_EVENT_REGULATION_OUT: u32 = 0x04;
pub const REGULATOR_EVENT_FAIL: u32 = 0x08;
pub const REGULATOR_EVENT_OVER_TEMP: u32 = 0x10;
pub const REGULATOR_EVENT_FORCE_DISABLE: u32 = 0x20;
pub const REGULATOR_EVENT_VOLTAGE_CHANGE: u32 = 0x40;
pub const REGULATOR_EVENT_DISABLE: u32 = 0x80;
pub const REGULATOR_EVENT_PRE_VOLTAGE_CHANGE: u32 = 0x100;
pub const REGULATOR_EVENT_ABORT_VOLTAGE_CHANGE: u32 = 0x200;
pub const REGULATOR_EVENT_PRE_DISABLE: u32 = 0x400;
pub const REGULATOR_EVENT_ABORT_DISABLE: u32 = 0x800;
pub const REGULATOR_EVENT_ENABLE: u32 = 0x1000;

/*
 * Following notifications should be emitted only if detected condition
 * is such that the HW is likely to still be working but consumers should
 * take a recovery action to prevent problems escalating into errors.
 */
pub const REGULATOR_EVENT_UNDER_VOLTAGE_WARN: u32 = 0x2000;
pub const REGULATOR_EVENT_OVER_CURRENT_WARN: u32 = 0x4000;
pub const REGULATOR_EVENT_OVER_VOLTAGE_WARN: u32 = 0x8000;
pub const REGULATOR_EVENT_OVER_TEMP_WARN: u32 = 0x10000;
pub const REGULATOR_EVENT_WARN_MASK: u32 = 0x1E000;

#[repr(C)]
pub struct reg_genl_event {
    pub reg_name: [std::ffi::c_char; 32],
    pub event: u64,
}

/* attributes of reg_genl_family */
#[repr(i32)]
pub enum reg_genl_attr {
    REG_GENL_ATTR_UNSPEC,
    REG_GENL_ATTR_EVENT, /* reg event info needed by user space */
    __REG_GENL_ATTR_MAX,
}

pub const REG_GENL_ATTR_MAX: i32 = __REG_GENL_ATTR_MAX as i32 - 1;

/* commands supported by the reg_genl_family */
#[repr(i32)]
pub enum reg_genl_cmd {
    REG_GENL_CMD_UNSPEC,
    REG_GENL_CMD_EVENT, /* kernel->user notifications for reg events */
    __REG_GENL_CMD_MAX,
}

pub const REG_GENL_CMD_MAX: i32 = __REG_GENL_CMD_MAX as i32 - 1;

pub const REG_GENL_FAMILY_NAME: &str = "reg_event";
pub const REG_GENL_VERSION: u32 = 0x01;
pub const REG_GENL_MCAST_GROUP_NAME: &str = "reg_mc_group";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
