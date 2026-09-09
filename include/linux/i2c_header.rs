/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of linux/i2c.h. External kernel types and functions are
 * intentionally referenced but not implemented here. */

use core::ffi::c_void;

pub const I2C_MAX_STANDARD_MODE_FREQ: u32 = 100000;
pub const I2C_MAX_FAST_MODE_FREQ: u32 = 400000;
pub const I2C_MAX_FAST_MODE_PLUS_FREQ: u32 = 1000000;
pub const I2C_MAX_TURBO_MODE_FREQ: u32 = 1400000;
pub const I2C_MAX_HIGH_SPEED_MODE_FREQ: u32 = 3400000;
pub const I2C_MAX_ULTRA_FAST_MODE_FREQ: u32 = 5000000;

extern "C" {
    pub static i2c_bus_type: bus_type;
    pub static i2c_adapter_type: device_type;
    pub static i2c_client_type: device_type;
}

#[repr(C)] pub struct i2c_msg { pub addr: u16, pub flags: u16, pub len: u16, pub buf: *mut u8 }
#[repr(C)] pub struct i2c_adapter;
#[repr(C)] pub struct i2c_client;
#[repr(C)] pub struct i2c_driver;
#[repr(C)] pub struct i2c_device_identity;
#[repr(C)] pub union i2c_smbus_data { pub byte: u8, pub word: u16, pub block: [u8; 34] }
#[repr(C)] pub struct i2c_board_info;
#[repr(C)] pub struct bus_type;
#[repr(C)] pub struct device_type;
#[repr(C)] pub struct device;
#[repr(C)] pub struct device_driver;
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct module;
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct kobject;
#[repr(C)] pub struct fwnode_handle;
#[repr(C)] pub struct software_node;
#[repr(C)] pub struct resource;
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct gpio_desc;
#[repr(C)] pub struct pinctrl;
#[repr(C)] pub struct pinctrl_state;
#[repr(C)] pub struct rt_mutex;
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct completion;
#[repr(C)] pub struct irq_domain;
#[repr(C)] pub struct regulator;
#[repr(C)] pub struct i2c_device_id;
pub type acpi_handle = *mut c_void;
pub type u8_ = u8; pub type u16_ = u16; pub type u32_ = u32; pub type u64_ = u64;
pub type i2c_slave_cb_t = Option<unsafe extern "C" fn(*mut i2c_client, i2c_slave_event, *mut u8) -> i32>;

#[repr(C)] #[derive(Copy, Clone)] pub enum i2c_slave_event { I2C_SLAVE_READ_REQUESTED, I2C_SLAVE_WRITE_REQUESTED, I2C_SLAVE_READ_PROCESSED, I2C_SLAVE_WRITE_RECEIVED, I2C_SLAVE_STOP }
#[repr(C)] #[derive(Copy, Clone)] pub enum i2c_alert_protocol { I2C_PROTOCOL_SMBUS_ALERT, I2C_PROTOCOL_SMBUS_HOST_NOTIFY }

pub const I2C_DEVICE_ID_NXP_SEMICONDUCTORS: u16 = 0;
pub const I2C_DEVICE_ID_NXP_SEMICONDUCTORS_1: u16 = 1;
pub const I2C_DEVICE_ID_NXP_SEMICONDUCTORS_2: u16 = 2;
pub const I2C_DEVICE_ID_NXP_SEMICONDUCTORS_3: u16 = 3;
pub const I2C_DEVICE_ID_RAMTRON_INTERNATIONAL: u16 = 4;
pub const I2C_DEVICE_ID_ANALOG_DEVICES: u16 = 5;
pub const I2C_DEVICE_ID_STMICROELECTRONICS: u16 = 6;
pub const I2C_DEVICE_ID_ON_SEMICONDUCTOR: u16 = 7;
pub const I2C_DEVICE_ID_SPRINTEK_CORPORATION: u16 = 8;
pub const I2C_DEVICE_ID_ESPROS_PHOTONICS_AG: u16 = 9;
pub const I2C_DEVICE_ID_FUJITSU_SEMICONDUCTOR: u16 = 10;
pub const I2C_DEVICE_ID_FLIR: u16 = 11;
pub const I2C_DEVICE_ID_O2MICRO: u16 = 12;
pub const I2C_DEVICE_ID_ATMEL: u16 = 13;
pub const I2C_DEVICE_ID_NONE: u16 = 0xffff;

#[repr(C)] pub struct i2c_device_identity { pub manufacturer_id: u16, pub part_id: u16, pub die_revision: u8 }
#[repr(C)] pub struct i2c_driver_flags; pub const I2C_DRV_ACPI_WAIVE_D0_PROBE: u32 = 1 << 0;

extern "C" {
    pub fn i2c_transfer_buffer_flags(client: *const i2c_client, buf: *mut i8, count: i32, flags: u16) -> i32;
    pub fn i2c_transfer(adap: *mut i2c_adapter, msgs: *mut i2c_msg, num: i32) -> i32;
    pub fn __i2c_transfer(adap: *mut i2c_adapter, msgs: *mut i2c_msg, num: i32) -> i32;
    pub fn i2c_smbus_xfer(adapter: *mut i2c_adapter, addr: u16, flags: u16, read_write: i8, command: u8, protocol: i32, data: *mut i2c_smbus_data) -> i32;
    pub fn __i2c_smbus_xfer(adapter: *mut i2c_adapter, addr: u16, flags: u16, read_write: i8, command: u8, protocol: i32, data: *mut i2c_smbus_data) -> i32;
    pub fn i2c_smbus_pec(crc: u8, p: *mut u8, count: usize) -> u8;
    pub fn i2c_smbus_read_byte(client: *const i2c_client) -> i32;
    pub fn i2c_smbus_write_byte(client: *const i2c_client, value: u8) -> i32;
    pub fn i2c_smbus_read_byte_data(client: *const i2c_client, command: u8) -> i32;
    pub fn i2c_smbus_write_byte_data(client: *const i2c_client, command: u8, value: u8) -> i32;
    pub fn i2c_smbus_read_word_data(client: *const i2c_client, command: u8) -> i32;
    pub fn i2c_smbus_write_word_data(client: *const i2c_client, command: u8, value: u16) -> i32;
    pub fn i2c_smbus_read_block_data(client: *const i2c_client, command: u8, values: *mut u8) -> i32;
    pub fn i2c_smbus_write_block_data(client: *const i2c_client, command: u8, length: u8, values: *const u8) -> i32;
    pub fn i2c_smbus_read_i2c_block_data(client: *const i2c_client, command: u8, length: u8, values: *mut u8) -> i32;
    pub fn i2c_smbus_write_i2c_block_data(client: *const i2c_client, command: u8, length: u8, values: *const u8) -> i32;
    pub fn i2c_smbus_read_i2c_block_data_or_emulated(client: *const i2c_client, command: u8, length: u8, values: *mut u8) -> i32;
    pub fn i2c_get_device_id(client: *const i2c_client, id: *mut i2c_device_identity) -> i32;
    pub fn i2c_client_get_device_id(client: *const i2c_client) -> *const i2c_device_id;
}

pub const I2C_CLIENT_PEC: u16 = 0x04; pub const I2C_CLIENT_TEN: u16 = 0x10;
pub const I2C_CLIENT_SLAVE: u16 = 0x20; pub const I2C_CLIENT_HOST_NOTIFY: u16 = 0x40;
pub const I2C_CLIENT_WAKE: u16 = 0x80; pub const I2C_CLIENT_SCCB: u16 = 0x9000;

#[repr(C)] pub struct i2c_timings { pub bus_freq_hz:u32, pub scl_rise_ns:u32, pub scl_fall_ns:u32, pub scl_int_delay_ns:u32, pub sda_fall_ns:u32, pub sda_hold_ns:u32, pub digital_filter_width_ns:u32, pub analog_filter_cutoff_freq_hz:u32 }
#[repr(C)] pub struct i2c_adapter_quirks { pub flags:u64, pub max_num_msgs:i32, pub max_write_len:u16, pub max_read_len:u16, pub max_comb_1st_msg_len:u16, pub max_comb_2nd_msg_len:u16 }
pub const I2C_AQ_COMB:u64=1<<0; pub const I2C_AQ_COMB_WRITE_FIRST:u64=1<<1; pub const I2C_AQ_COMB_READ_SECOND:u64=1<<2; pub const I2C_AQ_COMB_SAME_ADDR:u64=1<<3;
pub const I2C_AQ_COMB_WRITE_THEN_READ:u64=I2C_AQ_COMB|I2C_AQ_COMB_WRITE_FIRST|I2C_AQ_COMB_READ_SECOND|I2C_AQ_COMB_SAME_ADDR;
pub const I2C_AQ_NO_CLK_STRETCH:u64=1<<4; pub const I2C_AQ_NO_ZERO_LEN_READ:u64=1<<5; pub const I2C_AQ_NO_ZERO_LEN_WRITE:u64=1<<6; pub const I2C_AQ_NO_ZERO_LEN:u64=I2C_AQ_NO_ZERO_LEN_READ|I2C_AQ_NO_ZERO_LEN_WRITE; pub const I2C_AQ_NO_REP_START:u64=1<<7;

/* Remaining kernel structures and APIs retain their C ABI through opaque
 * external declarations; configuration-dependent implementations are supplied
 * by the kernel build. */
extern "C" {
    pub fn i2c_recover_bus(adap:*mut i2c_adapter)->i32;
    pub fn i2c_generic_scl_recovery(adap:*mut i2c_adapter)->i32;
    pub fn i2c_slave_register(client:*mut i2c_client, cb:i2c_slave_cb_t)->i32;
    pub fn i2c_slave_unregister(client:*mut i2c_client)->i32;
    pub fn i2c_slave_event(client:*mut i2c_client,event:i2c_slave_event,val:*mut u8)->i32;
    pub fn i2c_add_adapter(adap:*mut i2c_adapter)->i32;
    pub fn i2c_del_adapter(adap:*mut i2c_adapter);
    pub fn i2c_get_adapter(nr:i32)->*mut i2c_adapter;
    pub fn i2c_put_adapter(adap:*mut i2c_adapter);
    pub fn i2c_get_dma_safe_msg_buf(msg:*mut i2c_msg, threshold:u32)->*mut u8;
    pub fn i2c_put_dma_safe_msg_buf(buf:*mut u8,msg:*mut i2c_msg,xferred:bool);
}

#[repr(C)] pub struct i2c_driver {
    pub class: u32,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client)->i32>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub alert: Option<unsafe extern "C" fn(*mut i2c_client,i2c_alert_protocol,u32)>,
    pub command: Option<unsafe extern "C" fn(*mut i2c_client,u32,*mut c_void)->i32>,
    pub driver: device_driver,
    pub id_table: *const i2c_device_id,
    pub detect: Option<unsafe extern "C" fn(*mut i2c_client,*mut i2c_board_info)->i32>,
    pub address_list: *const u16,
    pub clients: list_head,
    pub flags: u32,
}

#[repr(C)] pub struct i2c_client {
    pub flags:u16, pub addr:u16, pub name:[i8;20], pub adapter:*mut i2c_adapter,
    pub dev:device, pub init_irq:i32, pub irq:i32, pub detected:list_head,
    pub slave_cb:i2c_slave_cb_t, pub devres_group_id:*mut c_void, pub debugfs:*mut dentry,
}
#[repr(C)] pub struct i2c_board_info {
    pub type_: [i8;20], pub flags:u16, pub addr:u16, pub dev_name:*const i8,
    pub platform_data:*mut c_void, pub fwnode:*mut fwnode_handle,
    pub swnode:*const software_node, pub resources:*const resource,
    pub num_resources:u32, pub irq:i32,
}
#[repr(C)] pub struct i2c_lock_operations {
    pub lock_bus:Option<unsafe extern "C" fn(*mut i2c_adapter,u32)>,
    pub trylock_bus:Option<unsafe extern "C" fn(*mut i2c_adapter,u32)->i32>,
    pub unlock_bus:Option<unsafe extern "C" fn(*mut i2c_adapter,u32)>,
}
#[repr(C)] pub struct i2c_bus_recovery_info {
    pub recover_bus:Option<unsafe extern "C" fn(*mut i2c_adapter)->i32>,
    pub get_scl:Option<unsafe extern "C" fn(*mut i2c_adapter)->i32>,
    pub set_scl:Option<unsafe extern "C" fn(*mut i2c_adapter,i32)>,
    pub get_sda:Option<unsafe extern "C" fn(*mut i2c_adapter)->i32>,
    pub set_sda:Option<unsafe extern "C" fn(*mut i2c_adapter,i32)>,
    pub get_bus_free:Option<unsafe extern "C" fn(*mut i2c_adapter)->i32>,
    pub prepare_recovery:Option<unsafe extern "C" fn(*mut i2c_adapter)>,
    pub unprepare_recovery:Option<unsafe extern "C" fn(*mut i2c_adapter)>,
    pub scl_gpiod:*mut gpio_desc, pub sda_gpiod:*mut gpio_desc,
    pub pinctrl:*mut pinctrl, pub pins_default:*mut pinctrl_state, pub pins_gpio:*mut pinctrl_state,
}
pub const I2C_LOCK_ROOT_ADAPTER:u32=1<<0; pub const I2C_LOCK_SEGMENT:u32=1<<1;
pub const I2C_CLASS_HWMON:u32=1<<0; pub const I2C_CLASS_DEPRECATED:u32=1<<8; pub const I2C_CLIENT_END:u16=0xfffe;

extern "C" {
    pub fn i2c_new_client_device(*mut i2c_adapter,*const i2c_board_info)->*mut i2c_client;
    pub fn i2c_new_dummy_device(*mut i2c_adapter,u16)->*mut i2c_client;
    pub fn i2c_unregister_device(*mut i2c_client);
    pub fn i2c_register_driver(*mut module,*mut i2c_driver)->i32;
    pub fn i2c_del_driver(*mut i2c_driver);
    pub fn i2c_for_each_dev(*mut c_void,*mut c_void)->i32;
    pub fn i2c_parse_fw_timings(*mut device,*mut i2c_timings,bool);
    pub fn of_fwnode_handle(*mut device_node)->*mut fwnode_handle;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
