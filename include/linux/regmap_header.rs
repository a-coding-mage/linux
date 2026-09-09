/* SPDX-License-Identifier: GPL-2.0-only */
//! Rust source-level translation of Linux `regmap.h`.
//! C preprocessor configuration (`CONFIG_REGMAP`, `CONFIG_LOCKDEP`) is retained
//! by exposing the declarations and preserving the disabled-operation stubs.

use core::ffi::{c_char, c_int, c_void};

pub type U8 = u8;
pub type U32 = u32;
pub type SizeT = usize;
pub type KtimeT = i64;
pub type IrqHwNumberT = u32;

#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct fsi_device { _private: [u8; 0] }
#[repr(C)] pub struct i2c_client { _private: [u8; 0] }
#[repr(C)] pub struct i3c_device { pub dev: device }
#[repr(C)] pub struct irq_domain { _private: [u8; 0] }
#[repr(C)] pub struct mdio_device { _private: [u8; 0] }
#[repr(C)] pub struct slim_device { _private: [u8; 0] }
#[repr(C)] pub struct spi_device { _private: [u8; 0] }
#[repr(C)] pub struct spmi_device { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct regmap_field { _private: [u8; 0] }
#[repr(C)] pub struct snd_ac97 { _private: [u8; 0] }
#[repr(C)] pub struct sdw_slave { pub dev: device }
#[repr(C)] pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] pub struct lock_class_key { _private: [u8; 0] }
#[repr(C)] pub struct regmap_async { _private: [u8; 0] }

pub const REGMAP_MDIO_C45_DEVAD_SHIFT: u32 = 16;
pub const REGMAP_MDIO_C45_DEVAD_MASK: u32 = 0x1f << 16;
pub const REGMAP_MDIO_C45_REGNUM_MASK: u32 = 0xffff;
#[inline] pub const fn REGMAP_UPSHIFT(s: i32) -> i32 { -s }
#[inline] pub const fn REGMAP_DOWNSHIFT(s: i32) -> i32 { s }

#[repr(C)] #[derive(Copy, Clone)] pub enum regcache_type { REGCACHE_NONE, REGCACHE_RBTREE, REGCACHE_FLAT, REGCACHE_MAPLE, REGCACHE_FLAT_S }
#[repr(C)] #[derive(Copy, Clone)] pub struct reg_default { pub reg: u32, pub def: u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct reg_sequence { pub reg: u32, pub def: u32, pub delay_us: u32 }
#[inline] pub const fn REG_SEQ(reg: u32, def: u32, delay_us: u32) -> reg_sequence { reg_sequence { reg, def, delay_us } }
#[inline] pub const fn REG_SEQ0(reg: u32, def: u32) -> reg_sequence { REG_SEQ(reg, def, 0) }

#[repr(C)] #[derive(Copy, Clone)] pub enum regmap_endian { REGMAP_ENDIAN_DEFAULT = 0, REGMAP_ENDIAN_BIG, REGMAP_ENDIAN_LITTLE, REGMAP_ENDIAN_NATIVE }
#[repr(C)] pub struct regmap_range { pub range_min: u32, pub range_max: u32 }
#[inline] pub const fn regmap_reg_range(low: u32, high: u32) -> regmap_range { regmap_range { range_min: low, range_max: high } }
#[repr(C)] pub struct regmap_access_table { pub yes_ranges: *const regmap_range, pub n_yes_ranges: u32, pub no_ranges: *const regmap_range, pub n_no_ranges: u32 }
pub type regmap_lock = unsafe extern "C" fn(*mut c_void);
pub type regmap_unlock = unsafe extern "C" fn(*mut c_void);

#[repr(C)] pub struct regmap_config {
    pub name: *const c_char, pub reg_bits: c_int, pub reg_stride: c_int, pub reg_shift: c_int, pub reg_base: u32, pub pad_bits: c_int, pub val_bits: c_int,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device,u32)->bool>, pub readable_reg: Option<unsafe extern "C" fn(*mut device,u32)->bool>, pub volatile_reg: Option<unsafe extern "C" fn(*mut device,u32)->bool>, pub precious_reg: Option<unsafe extern "C" fn(*mut device,u32)->bool>, pub writeable_noinc_reg: Option<unsafe extern "C" fn(*mut device,u32)->bool>, pub readable_noinc_reg: Option<unsafe extern "C" fn(*mut device,u32)->bool>,
    pub reg_read: Option<unsafe extern "C" fn(*mut c_void,u32,*mut u32)->c_int>, pub reg_write: Option<unsafe extern "C" fn(*mut c_void,u32,u32)->c_int>, pub reg_update_bits: Option<unsafe extern "C" fn(*mut c_void,u32,u32,u32)->c_int>, pub read: Option<unsafe extern "C" fn(*mut c_void,*const c_void,SizeT,*mut c_void,SizeT)->c_int>, pub write: Option<unsafe extern "C" fn(*mut c_void,*const c_void,SizeT)->c_int>, pub max_raw_read: SizeT, pub max_raw_write: SizeT,
    pub can_sleep: bool, pub fast_io: bool, pub io_port: bool, pub disable_locking: bool, pub lock: Option<regmap_lock>, pub unlock: Option<regmap_unlock>, pub lock_arg: *mut c_void,
    pub max_register: u32, pub max_register_is_0: bool, pub wr_table: *const regmap_access_table, pub rd_table: *const regmap_access_table, pub volatile_table: *const regmap_access_table, pub precious_table: *const regmap_access_table, pub wr_noinc_table: *const regmap_access_table, pub rd_noinc_table: *const regmap_access_table, pub reg_defaults: *const reg_default, pub num_reg_defaults: u32, pub reg_default_cb: Option<unsafe extern "C" fn(*mut device,u32,*mut u32)->c_int>, pub cache_type: regcache_type, pub reg_defaults_raw: *const c_void, pub num_reg_defaults_raw: u32,
    pub read_flag_mask: usize, pub write_flag_mask: usize, pub zero_flag_mask: bool, pub use_single_read: bool, pub use_single_write: bool, pub use_relaxed_mmio: bool, pub can_multi_write: bool, pub use_hwlock: bool, pub use_raw_spinlock: bool, pub hwlock_id: u32, pub hwlock_mode: u32, pub reg_format_endian: regmap_endian, pub val_format_endian: regmap_endian, pub ranges: *const regmap_range_cfg, pub num_ranges: u32,
}
#[repr(C)] pub struct regmap_range_cfg { pub name: *const c_char, pub range_min: u32, pub range_max: u32, pub selector_reg: u32, pub selector_mask: u32, pub selector_shift: c_int, pub window_start: u32, pub window_len: u32 }
#[repr(C)] pub struct regmap_sdw_mbq_cfg { pub mbq_size: Option<unsafe extern "C" fn(*mut device,u32)->c_int>, pub deferrable: Option<unsafe extern "C" fn(*mut device,u32)->bool>, pub timeout_us: usize, pub retry_us: usize }

pub type regmap_hw_write = unsafe extern "C" fn(*mut c_void,*const c_void,SizeT)->c_int;
pub type regmap_hw_gather_write = unsafe extern "C" fn(*mut c_void,*const c_void,SizeT,*const c_void,SizeT)->c_int;
pub type regmap_hw_async_write = unsafe extern "C" fn(*mut c_void,*const c_void,SizeT,*const c_void,SizeT,*mut regmap_async)->c_int;
pub type regmap_hw_read = unsafe extern "C" fn(*mut c_void,*const c_void,SizeT,*mut c_void,SizeT)->c_int;
pub type regmap_hw_reg_read = unsafe extern "C" fn(*mut c_void,u32,*mut u32)->c_int;
pub type regmap_hw_reg_noinc_read = unsafe extern "C" fn(*mut c_void,u32,*mut c_void,SizeT)->c_int;
pub type regmap_hw_reg_write = unsafe extern "C" fn(*mut c_void,u32,u32)->c_int;
pub type regmap_hw_reg_noinc_write = unsafe extern "C" fn(*mut c_void,u32,*const c_void,SizeT)->c_int;
pub type regmap_hw_reg_update_bits = unsafe extern "C" fn(*mut c_void,u32,u32,u32)->c_int;
pub type regmap_hw_async_alloc = unsafe extern "C" fn()->*mut regmap_async;
pub type regmap_hw_free_context = unsafe extern "C" fn(*mut c_void);
#[repr(C)] pub struct regmap_bus { pub fast_io: bool, pub free_on_exit: bool, pub write: Option<regmap_hw_write>, pub gather_write: Option<regmap_hw_gather_write>, pub async_write: Option<regmap_hw_async_write>, pub reg_write: Option<regmap_hw_reg_write>, pub reg_noinc_write: Option<regmap_hw_reg_noinc_write>, pub reg_update_bits: Option<regmap_hw_reg_update_bits>, pub read: Option<regmap_hw_read>, pub reg_read: Option<regmap_hw_reg_read>, pub reg_noinc_read: Option<regmap_hw_reg_noinc_read>, pub free_context: Option<regmap_hw_free_context>, pub async_alloc: Option<regmap_hw_async_alloc>, pub read_flag_mask: u8, pub reg_format_endian_default: regmap_endian, pub val_format_endian_default: regmap_endian, pub max_raw_read: SizeT, pub max_raw_write: SizeT }

#[repr(C)] pub struct reg_field { pub reg: u32, pub lsb: u32, pub msb: u32, pub id_size: u32, pub id_offset: u32 }
#[inline] pub const fn REG_FIELD(reg:u32,lsb:u32,msb:u32)->reg_field { reg_field{reg,lsb,msb,id_size:0,id_offset:0} }
#[inline] pub const fn REG_FIELD_ID(reg:u32,lsb:u32,msb:u32,size:u32,offset:u32)->reg_field { reg_field{reg,lsb,msb,id_size:size,id_offset:offset} }
#[repr(C)] pub struct regmap_irq_type { pub type_reg_offset:u32,pub type_reg_mask:u32,pub type_rising_val:u32,pub type_falling_val:u32,pub type_level_low_val:u32,pub type_level_high_val:u32,pub types_supported:u32 }
#[repr(C)] pub struct regmap_irq { pub reg_offset:u32,pub mask:u32,pub type_:regmap_irq_type }
#[repr(C)] pub struct regmap_irq_sub_irq_map { pub num_regs:u32,pub offset:*mut u32 }
#[repr(C)] pub struct regmap_irq_chip_data { _private:[u8;0] }
#[repr(C)] pub struct regmap_irq_chip { pub name:*const c_char,pub domain_suffix:*const c_char,pub main_status:u32,pub num_main_status_bits:u32,pub sub_reg_offsets:*const regmap_irq_sub_irq_map,pub num_main_regs:c_int,pub status_base:u32,pub mask_base:u32,pub unmask_base:u32,pub ack_base:u32,pub wake_base:u32,pub config_base:*const u32,pub irq_reg_stride:u32,pub init_ack_masked:u32,pub mask_unmask_non_inverted:u32,pub use_ack:u32,pub ack_invert:u32,pub clear_ack:u32,pub status_invert:u32,pub status_is_level:u32,pub wake_invert:u32,pub type_in_mask:u32,pub clear_on_unmask:u32,pub runtime_pm:u32,pub no_status:u32,pub num_regs:c_int,pub irqs:*const regmap_irq,pub num_irqs:c_int,pub num_config_bases:c_int,pub num_config_regs:c_int,pub handle_pre_irq:Option<unsafe extern "C" fn(*mut c_void)->c_int>,pub handle_post_irq:Option<unsafe extern "C" fn(*mut c_void)->c_int>,pub handle_mask_sync:Option<unsafe extern "C" fn(c_int,u32,u32,*mut c_void)->c_int>,pub set_type_config:Option<unsafe extern "C" fn(*mut *mut u32,u32,*const regmap_irq,c_int,*mut c_void)->c_int>,pub get_irq_reg:Option<unsafe extern "C" fn(*mut regmap_irq_chip_data,u32,c_int)->u32>,pub irq_reqres:Option<unsafe extern "C" fn(*mut c_void,IrqHwNumberT)->c_int>,pub irq_relres:Option<unsafe extern "C" fn(*mut c_void,IrqHwNumberT)>,pub irq_drv_data:*mut c_void }

// Declaration-only external API.  Bodies remain external dependencies.
extern "C" {
    pub fn __regmap_init(dev:*mut device,bus:*const regmap_bus,bus_context:*mut c_void,config:*const regmap_config,lock_key:*mut lock_class_key,lock_name:*const c_char)->*mut regmap;
    pub fn regmap_write(map:*mut regmap,reg:u32,val:u32)->c_int; pub fn regmap_read(map:*mut regmap,reg:u32,val:*mut u32)->c_int;
    pub fn regmap_update_bits_base(map:*mut regmap,reg:u32,mask:u32,val:u32,change:*mut bool,async_:bool,force:bool)->c_int;
    pub fn regmap_field_update_bits_base(field:*mut regmap_field,mask:u32,val:u32,change:*mut bool,async_:bool,force:bool)->c_int;
    pub fn regmap_field_read(field:*mut regmap_field,val:*mut u32)->c_int;
    pub fn regmap_field_alloc(map:*mut regmap,field:reg_field)->*mut regmap_field;
    pub fn regmap_field_free(field:*mut regmap_field);
}

#[inline] pub unsafe fn regmap_update_bits(map:*mut regmap,reg:u32,mask:u32,val:u32)->c_int { regmap_update_bits_base(map,reg,mask,val,core::ptr::null_mut(),false,false) }
#[inline] pub unsafe fn regmap_set_bits(map:*mut regmap,reg:u32,bits:u32)->c_int { regmap_update_bits(map,reg,bits,bits) }
#[inline] pub unsafe fn regmap_clear_bits(map:*mut regmap,reg:u32,bits:u32)->c_int { regmap_update_bits(map,reg,bits,0) }
#[inline] pub unsafe fn regmap_assign_bits(map:*mut regmap,reg:u32,bits:u32,value:bool)->c_int { if value {regmap_set_bits(map,reg,bits)} else {regmap_clear_bits(map,reg,bits)} }
#[inline] pub unsafe fn regmap_reg_in_range(reg:u32,range:*const regmap_range)->bool { reg >= (*range).range_min && reg <= (*range).range_max }
#[inline] pub unsafe fn regmap_field_write(field:*mut regmap_field,val:u32)->c_int { regmap_field_update_bits_base(field,!0,val,core::ptr::null_mut(),false,false) }
#[inline] pub unsafe fn regmap_field_update_bits(field:*mut regmap_field,mask:u32,val:u32)->c_int { regmap_field_update_bits_base(field,mask,val,core::ptr::null_mut(),false,false) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
