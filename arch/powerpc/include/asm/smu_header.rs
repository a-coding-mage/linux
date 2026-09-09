/* SPDX-License-Identifier: GPL-2.0 */

/* Definitions for talking to the SMU chip in newer G5 PowerMacs. */

pub const SMU_CMD_PARTITION_COMMAND: u8 = 0x3e;
pub const SMU_CMD_PARTITION_LATEST: u8 = 0x01;
pub const SMU_CMD_PARTITION_BASE: u8 = 0x02;
pub const SMU_CMD_PARTITION_UPDATE: u8 = 0x03;

pub const SMU_CMD_FAN_COMMAND: u8 = 0x4a;
pub const SMU_CMD_BATTERY_COMMAND: u8 = 0x6f;
pub const SMU_CMD_GET_BATTERY_INFO: u8 = 0x00;

pub const SMU_CMD_RTC_COMMAND: u8 = 0x8e;
pub const SMU_CMD_RTC_SET_PWRUP_TIMER: u8 = 0x00;
pub const SMU_CMD_RTC_GET_PWRUP_TIMER: u8 = 0x01;
pub const SMU_CMD_RTC_STOP_PWRUP_TIMER: u8 = 0x02;
pub const SMU_CMD_RTC_SET_PRAM_BYTE_ACC: u8 = 0x20;
pub const SMU_CMD_RTC_SET_PRAM_AUTOINC: u8 = 0x21;
pub const SMU_CMD_RTC_SET_PRAM_LO_BYTES: u8 = 0x22;
pub const SMU_CMD_RTC_SET_PRAM_HI_BYTES: u8 = 0x23;
pub const SMU_CMD_RTC_GET_PRAM_BYTE: u8 = 0x28;
pub const SMU_CMD_RTC_GET_PRAM_LO_BYTES: u8 = 0x29;
pub const SMU_CMD_RTC_GET_PRAM_HI_BYTES: u8 = 0x2a;
pub const SMU_CMD_RTC_SET_DATETIME: u8 = 0x80;
pub const SMU_CMD_RTC_GET_DATETIME: u8 = 0x81;

pub const SMU_CMD_I2C_COMMAND: u8 = 0x9a;
pub const SMU_I2C_TRANSFER_SIMPLE: u8 = 0x00;
pub const SMU_I2C_TRANSFER_STDSUB: u8 = 0x01;
pub const SMU_I2C_TRANSFER_COMBINED: u8 = 0x02;

pub const SMU_CMD_POWER_COMMAND: u8 = 0xaa;
pub const SMU_CMD_POWER_RESTART: &str = "RESTART";
pub const SMU_CMD_POWER_SHUTDOWN: &str = "SHUTDOWN";
pub const SMU_CMD_POWER_VOLTAGE_SLEW: &str = "VSLEW";
pub const SMU_CMD_READ_ADC: u8 = 0xd8;
pub const SMU_CMD_MISC_df_COMMAND: u8 = 0xdf;
pub const SMU_CMD_MISC_df_SET_DISPLAY_LIT: u8 = 0x02;
pub const SMU_CMD_MISC_df_NMI_OPTION: u8 = 0x04;
pub const SMU_CMD_MISC_df_DIMM_OFFSET: u8 = 0x99;
pub const SMU_CMD_VERSION_COMMAND: u8 = 0xea;
pub const SMU_VERSION_RUNNING: u8 = 0x00;
pub const SMU_VERSION_BASE: u8 = 0x01;
pub const SMU_VERSION_UPDATE: u8 = 0x02;
pub const SMU_CMD_SWITCHES: u8 = 0xdc;
pub const SMU_SWITCH_CASE_CLOSED: u8 = 0x01;
pub const SMU_SWITCH_AC_POWER: u8 = 0x04;
pub const SMU_SWITCH_POWER_SWITCH: u8 = 0x08;
pub const SMU_CMD_MISC_ee_COMMAND: u8 = 0xee;
pub const SMU_CMD_MISC_ee_GET_DATABLOCK_REC: u8 = 0x02;
pub const SMU_CMD_MISC_ee_GET_WATTS: u8 = 0x03;
pub const SMU_CMD_MISC_ee_LEDS_CTRL: u8 = 0x04;
pub const SMU_CMD_MISC_ee_GET_DATA: u8 = 0x05;
pub const SMU_CMD_POWER_EVENTS_COMMAND: u8 = 0x8f;

pub const SMU_PWR_GET_POWERUP_EVENTS: u8 = 0x00;
pub const SMU_PWR_SET_POWERUP_EVENTS: u8 = 0x01;
pub const SMU_PWR_CLR_POWERUP_EVENTS: u8 = 0x02;
pub const SMU_PWR_GET_WAKEUP_EVENTS: u8 = 0x03;
pub const SMU_PWR_SET_WAKEUP_EVENTS: u8 = 0x04;
pub const SMU_PWR_CLR_WAKEUP_EVENTS: u8 = 0x05;
pub const SMU_PWR_LAST_SHUTDOWN_CAUSE: u8 = 0x07;
pub const SMU_PWR_SERVER_ID: u8 = 0x08;
pub const SMU_PWR_WAKEUP_KEY: u8 = 0x01;
pub const SMU_PWR_WAKEUP_AC_INSERT: u8 = 0x02;
pub const SMU_PWR_WAKEUP_AC_CHANGE: u8 = 0x04;
pub const SMU_PWR_WAKEUP_LID_OPEN: u8 = 0x08;
pub const SMU_PWR_WAKEUP_RING: u8 = 0x10;

/* Kernel-only declarations are retained as declarations under this feature. */
#[cfg(feature = "kernel")]
#[repr(C)]
pub struct smu_cmd {
    pub cmd: u8,
    pub data_len: i32,
    pub reply_len: i32,
    pub data_buf: *mut core::ffi::c_void,
    pub reply_buf: *mut core::ffi::c_void,
    pub status: i32,
    pub done: Option<unsafe extern "C" fn(*mut smu_cmd, *mut core::ffi::c_void)>,
    pub misc: *mut core::ffi::c_void,
    pub link: list_head,
}

#[cfg(feature = "kernel")]
#[repr(C)]
pub struct smu_simple_cmd { pub cmd: smu_cmd, pub buffer: [u8; 16] }

#[cfg(feature = "kernel")]
extern "C" {
    pub fn smu_queue_cmd(cmd: *mut smu_cmd) -> i32;
    pub fn smu_queue_simple(scmd: *mut smu_simple_cmd, command: u8, data_len: u32,
        done: Option<unsafe extern "C" fn(*mut smu_cmd, *mut core::ffi::c_void)>,
        misc: *mut core::ffi::c_void, ...) -> i32;
    pub fn smu_done_complete(cmd: *mut smu_cmd, misc: *mut core::ffi::c_void);
    pub fn smu_spinwait_cmd(cmd: *mut smu_cmd);
    pub fn smu_poll();
    pub fn smu_init() -> i32;
    pub fn smu_present() -> i32;
    pub fn smu_shutdown();
    pub fn smu_restart();
    pub fn smu_get_rtc_time(time: *mut rtc_time, spinwait: i32) -> i32;
    pub fn smu_set_rtc_time(time: *mut rtc_time, spinwait: i32) -> i32;
    pub fn smu_queue_i2c(cmd: *mut smu_i2c_cmd) -> i32;
}

#[cfg(feature = "kernel")]
#[inline]
pub unsafe fn smu_spinwait_simple(scmd: *mut smu_simple_cmd) { smu_spinwait_cmd(&mut (*scmd).cmd); }

pub const SMU_I2C_READ_MAX: usize = 0x1d;
pub const SMU_I2C_WRITE_MAX: usize = 0x15;

#[repr(C)]
pub struct smu_i2c_param {
    pub bus: u8, pub type_: u8, pub devaddr: u8, pub sublen: u8,
    pub subaddr: [u8; 3], pub caddr: u8, pub datalen: u8,
    pub data: [u8; SMU_I2C_READ_MAX],
}

#[cfg(feature = "kernel")]
#[repr(C)]
pub struct smu_i2c_cmd {
    pub info: smu_i2c_param,
    pub done: Option<unsafe extern "C" fn(*mut smu_i2c_cmd, *mut core::ffi::c_void)>,
    pub misc: *mut core::ffi::c_void,
    pub status: i32,
    pub scmd: smu_cmd,
    pub read: i32,
    pub stage: i32,
    pub retries: i32,
    pub pdata: [u8; 32],
    pub link: list_head,
}

#[repr(C)]
pub struct smu_sdbp_header { pub id: __u8, pub len: __u8, pub version: __u8, pub flags: __u8 }

/* External endian helper supplied by the surrounding environment. */
macro_rules! SMU_U16_MIX { ($x:expr) => { le16_to_cpu($x) }; }
macro_rules! SMU_U32_MIX { ($x:expr) => { (((($x) & 0xff00ff00u32) >> 8) | ((($x) & 0x00ff00ffu32) << 8)) }; }

pub const SMU_SDB_FVT_ID: u8 = 0x12;
#[repr(C)] pub struct smu_sdbp_fvt { pub sysclk: __u32, pub pad: __u8, pub maxtemp: __u8, pub volts: [__u16; 3] }
pub const SMU_SDB_CPUVCP_ID: u8 = 0x21;
#[repr(C)] pub struct smu_sdbp_cpuvcp { pub volt_scale: __u16, pub volt_offset: __s16, pub curr_scale: __u16, pub curr_offset: __s16, pub power_quads: [__s32; 3] }
pub const SMU_SDB_CPUDIODE_ID: u8 = 0x18;
#[repr(C)] pub struct smu_sdbp_cpudiode { pub m_value: __u16, pub b_value: __s16 }
pub const SMU_SDB_SLOTSPOW_ID: u8 = 0x78;
#[repr(C)] pub struct smu_sdbp_slotspow { pub pow_scale: __u16, pub pow_offset: __s16 }
pub const SMU_SDB_SENSORTREE_ID: u8 = 0x25;
#[repr(C)] pub struct smu_sdbp_sensortree { pub model_id: __u8, pub unknown: [__u8; 3] }
pub const SMU_SDB_CPUPIDDATA_ID: u8 = 0x17;
#[repr(C)] pub struct smu_sdbp_cpupiddata { pub unknown1: __u8, pub target_temp_delta: __u8, pub unknown2: __u8, pub history_len: __u8, pub power_adj: __s16, pub max_power: __u16, pub gp: __s32, pub gr: __s32, pub gd: __s32 }
pub const SMU_SDB_DEBUG_SWITCHES_ID: u8 = 0x05;

#[repr(C)]
pub struct smu_user_cmd_hdr { pub cmdtype: __u32, pub cmd: __u8, pub pad: [__u8; 3], pub data_len: __u32 }
pub const SMU_CMDTYPE_SMU: u32 = 0;
pub const SMU_CMDTYPE_WANTS_EVENTS: u32 = 1;
pub const SMU_CMDTYPE_GET_PARTITION: u32 = 2;
#[repr(C)] pub struct smu_user_reply_hdr { pub status: __u32, pub reply_len: __u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
