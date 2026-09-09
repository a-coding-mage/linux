/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/efi.h. External kernel types and functions are
 * intentionally referenced rather than implemented here. */

use core::ffi::{c_char, c_int, c_void};

pub type efi_status_t = usize;
pub type efi_bool_t = u8;
pub type efi_char16_t = u16;
pub type efi_physical_addr_t = u64;
pub type efi_handle_t = *mut c_void;
pub type phys_addr_t = usize;
pub type umode_t = u16;

#[repr(C)] #[derive(Copy, Clone)] pub struct guid_t { pub b: [u8; 16] }
pub type efi_guid_t = guid_t;
#[repr(C)] pub struct screen_info;
#[repr(C)] pub struct page;
#[repr(C)] pub struct range { pub start: u64, pub end: u64 }
#[repr(C)] pub struct mm_struct;
#[repr(C)] pub struct kobject;
#[repr(C)] pub struct attribute;
#[repr(C)] pub struct kset;
#[repr(C)] pub struct work_struct;
#[repr(C)] pub struct completion;
#[repr(C)] pub struct workqueue_struct;
#[repr(C)] pub struct blocking_notifier_head;
#[repr(C)] pub struct atomic_t { pub counter: c_int }

pub const EFI_SUCCESS: efi_status_t = 0;
const EFI_ERROR_BIT: efi_status_t = 1usize << (usize::BITS - 1);
pub const EFI_LOAD_ERROR: efi_status_t = 1 | EFI_ERROR_BIT;
pub const EFI_INVALID_PARAMETER: efi_status_t = 2 | EFI_ERROR_BIT;
pub const EFI_UNSUPPORTED: efi_status_t = 3 | EFI_ERROR_BIT;
pub const EFI_BAD_BUFFER_SIZE: efi_status_t = 4 | EFI_ERROR_BIT;
pub const EFI_BUFFER_TOO_SMALL: efi_status_t = 5 | EFI_ERROR_BIT;
pub const EFI_NOT_READY: efi_status_t = 6 | EFI_ERROR_BIT;
pub const EFI_DEVICE_ERROR: efi_status_t = 7 | EFI_ERROR_BIT;
pub const EFI_WRITE_PROTECTED: efi_status_t = 8 | EFI_ERROR_BIT;
pub const EFI_OUT_OF_RESOURCES: efi_status_t = 9 | EFI_ERROR_BIT;
pub const EFI_NOT_FOUND: efi_status_t = 14 | EFI_ERROR_BIT;
pub const EFI_ACCESS_DENIED: efi_status_t = 15 | EFI_ERROR_BIT;
pub const EFI_TIMEOUT: efi_status_t = 18 | EFI_ERROR_BIT;
pub const EFI_ABORTED: efi_status_t = 21 | EFI_ERROR_BIT;
pub const EFI_SECURITY_VIOLATION: efi_status_t = 26 | EFI_ERROR_BIT;

#[macro_export] macro_rules! EFI_GUID { ($a:expr,$b:expr,$c:expr,$($d:expr),+ $(,)?) => { $crate::efi_guid_t { b: [($a as u32) as u8,(($a as u32)>>8) as u8,(($a as u32)>>16) as u8,(($a as u32)>>24) as u8,($b as u16) as u8,(($b as u16)>>8) as u8,($c as u16) as u8,(($c as u16)>>8) as u8,$($d as u8),+] } } }

#[repr(C)] pub struct efi_table_hdr_t { pub signature:u64, pub revision:u32, pub headersize:u32, pub crc32:u32, pub reserved:u32 }
#[repr(C)] pub struct efi_memory_desc_t { pub typ:u32, pub pad:u32, pub phys_addr:u64, pub virt_addr:u64, pub num_pages:u64, pub attribute:u64 }
#[repr(C)] pub struct efi_capsule_header_t { pub guid:efi_guid_t, pub headersize:u32, pub flags:u32, pub imagesize:u32 }
#[repr(C, packed)] pub struct efi_manage_capsule_header { pub ver:u32, pub emb_drv_cnt:u16, pub payload_cnt:u16, pub offset_list:[u64;0] }
#[repr(C, packed)] pub struct efi_manage_capsule_image_header { pub ver:u32, pub image_type_id:efi_guid_t, pub image_index:u8, pub reserved_bytes:[u8;3], pub image_size:u32, pub vendor_code_size:u32, pub hw_ins:u64, pub capsule_support:u64 }
#[repr(C)] pub struct win_cert { pub len:u32, pub rev:u16, pub cert_type:u16 }
#[repr(C)] pub struct win_cert_uefi_guid { pub hdr:win_cert, pub cert_type:efi_guid_t, pub cert_data:[u8;0] }
#[repr(C)] pub struct efi_image_auth { pub mon_count:u64, pub auth_info:win_cert_uefi_guid }
#[repr(C)] pub struct capsule_info { pub header:efi_capsule_header_t, pub capsule:*mut efi_capsule_header_t, pub reset_type:c_int, pub index:isize, pub count:usize, pub total_size:usize, pub pages:*mut *mut page, pub phys:*mut phys_addr_t, pub page_bytes_remain:usize }

pub const EFI_PAGE_SHIFT:u32=12; pub const EFI_PAGE_SIZE:usize=1usize<<EFI_PAGE_SHIFT; pub const EFI_MEMORY_DESCRIPTOR_VERSION:u32=1;
pub const EFI_RESET_COLD:c_int=0; pub const EFI_RESET_WARM:c_int=1; pub const EFI_RESET_SHUTDOWN:c_int=2;
pub const EFI_RUNTIME_SERVICES_SIGNATURE:u64=0x5652453544e5552; pub const EFI_RUNTIME_SERVICES_REVISION:u32=0x10000;
pub const EFI_CAPSULE_PERSIST_ACROSS_RESET:u32=0x10000; pub const EFI_CAPSULE_POPULATE_SYSTEM_TABLE:u32=0x20000; pub const EFI_CAPSULE_INITIATE_RESET:u32=0x40000;

#[repr(C)] pub struct efi_time_t { pub year:u16,pub month:u8,pub day:u8,pub hour:u8,pub minute:u8,pub second:u8,pub pad1:u8,pub nanosecond:u32,pub timezone:i16,pub daylight:u8,pub pad2:u8 }
#[repr(C)] pub struct efi_time_cap_t { pub resolution:u32,pub accuracy:u32,pub sets_to_zero:u8 }
pub type efi_get_time_t=unsafe extern "C" fn(*mut efi_time_t,*mut efi_time_cap_t)->efi_status_t;
pub type efi_set_time_t=unsafe extern "C" fn(*mut efi_time_t)->efi_status_t;
pub type efi_get_wakeup_time_t=unsafe extern "C" fn(*mut efi_bool_t,*mut efi_bool_t,*mut efi_time_t)->efi_status_t;
pub type efi_set_wakeup_time_t=unsafe extern "C" fn(efi_bool_t,*mut efi_time_t)->efi_status_t;
pub type efi_get_variable_t=unsafe extern "C" fn(*mut efi_char16_t,*mut efi_guid_t,*mut u32,*mut usize,*mut c_void)->efi_status_t;
pub type efi_get_next_variable_t=unsafe extern "C" fn(*mut usize,*mut efi_char16_t,*mut efi_guid_t)->efi_status_t;
pub type efi_set_variable_t=unsafe extern "C" fn(*mut efi_char16_t,*mut efi_guid_t,u32,usize,*mut c_void)->efi_status_t;
pub type efi_get_next_high_mono_count_t=unsafe extern "C" fn(*mut u32)->efi_status_t;
pub type efi_reset_system_t=unsafe extern "C" fn(c_int,efi_status_t,usize,*mut efi_char16_t)->!;
pub type efi_query_variable_info_t=unsafe extern "C" fn(u32,*mut u64,*mut u64,*mut u64)->efi_status_t;
pub type efi_update_capsule_t=unsafe extern "C" fn(*mut *mut efi_capsule_header_t,usize,usize)->efi_status_t;
pub type efi_query_capsule_caps_t=unsafe extern "C" fn(*mut *mut efi_capsule_header_t,usize,*mut u64,*mut c_int)->efi_status_t;
pub type efi_query_variable_store_t=unsafe extern "C" fn(u32,usize,bool)->efi_status_t;

#[repr(C)] pub struct efi_runtime_services_32_t { pub hdr:efi_table_hdr_t, pub get_time:u32,pub set_time:u32,pub get_wakeup_time:u32,pub set_wakeup_time:u32,pub set_virtual_address_map:u32,pub convert_pointer:u32,pub get_variable:u32,pub get_next_variable:u32,pub set_variable:u32,pub get_next_high_mono_count:u32,pub reset_system:u32,pub update_capsule:u32,pub query_capsule_caps:u32,pub query_variable_info:u32 }
#[repr(C)] pub struct efi_runtime_services_t { pub hdr:efi_table_hdr_t,pub get_time:Option<efi_get_time_t>,pub set_time:Option<efi_set_time_t>,pub get_wakeup_time:Option<efi_get_wakeup_time_t>,pub set_wakeup_time:Option<efi_set_wakeup_time_t>,pub set_virtual_address_map:*mut c_void,pub convert_pointer:*mut c_void,pub get_variable:Option<efi_get_variable_t>,pub get_next_variable:Option<efi_get_next_variable_t>,pub set_variable:Option<efi_set_variable_t>,pub get_next_high_mono_count:Option<efi_get_next_high_mono_count_t>,pub reset_system:Option<efi_reset_system_t>,pub update_capsule:Option<efi_update_capsule_t>,pub query_capsule_caps:Option<efi_query_capsule_caps_t>,pub query_variable_info:Option<efi_query_variable_info_t> }

#[repr(C)] pub struct efi_memory_map_data { pub phys_map:phys_addr_t,pub size:usize,pub desc_version:usize,pub desc_size:usize,pub flags:usize }
#[repr(C)] pub struct efi_memory_map { pub phys_map:phys_addr_t,pub map:*mut c_void,pub map_end:*mut c_void,pub nr_map:c_int,pub desc_version:usize,pub desc_size:usize,pub flags:usize }
#[repr(C)] pub struct efi_mem_range { pub range:range,pub attribute:u64 }
#[repr(C)] pub struct efi_config_table_64_t { pub guid:efi_guid_t,pub table:u64 }
#[repr(C)] pub struct efi_config_table_32_t { pub guid:efi_guid_t,pub table:u32 }
#[repr(C)] pub struct efi_config_table_t { pub guid:efi_guid_t,pub table:*mut c_void }
#[repr(C)] pub struct efi_system_table_t { pub hdr:efi_table_hdr_t,pub fw_vendor:usize,pub fw_revision:u32,pub con_in_handle:usize,pub con_in:*mut c_void,pub con_out_handle:usize,pub con_out:*mut c_void,pub stderr_handle:usize,pub stderr:usize,pub runtime:*mut efi_runtime_services_t,pub boottime:*mut c_void,pub nr_tables:usize,pub tables:usize }

pub const EFI_SYSTEM_TABLE_SIGNATURE:u64=0x5453595320494249; pub const EFI_DXE_SERVICES_TABLE_SIGNATURE:u64=0x565245535f455844;
pub const EFI_RT_SUPPORTED_ALL:u32=0x3fff; pub const EFI_RT_SUPPORTED_TIME_SERVICES:u32=3; pub const EFI_RT_SUPPORTED_WAKEUP_SERVICES:u32=0xc; pub const EFI_RT_SUPPORTED_VARIABLE_SERVICES:u32=0x70;
pub const EFI_VARIABLE_NON_VOLATILE:u64=1; pub const EFI_VARIABLE_BOOTSERVICE_ACCESS:u64=2; pub const EFI_VARIABLE_RUNTIME_ACCESS:u64=4; pub const EFI_VARIABLE_HARDWARE_ERROR_RECORD:u64=8; pub const EFI_VARIABLE_AUTHENTICATED_WRITE_ACCESS:u64=16; pub const EFI_VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS:u64=32; pub const EFI_VARIABLE_APPEND_WRITE:u64=64;
pub const EFI_VAR_NAME_LEN:usize=1024; pub const EFI_RANDOM_SEED_SIZE:u32=32;

#[repr(C)] pub struct efi_generic_dev_path { pub typ:u8,pub sub_type:u8,pub length:u16 }
#[repr(C)] pub struct efi_pci_dev_path { pub header:efi_generic_dev_path,pub fn_:u8,pub dev:u8 }
#[repr(C)] pub struct efi_vendor_dev_path { pub header:efi_generic_dev_path,pub vendorguid:efi_guid_t,pub vendordata:[u8;0] }
#[repr(C)] pub struct efi_file_path_dev_path { pub header:efi_generic_dev_path,pub filename:[efi_char16_t;0] }
#[repr(C)] pub struct linux_efi_random_seed { pub size:u32,pub bits:[u8;0] }
#[repr(C)] pub struct linux_efi_tpm_eventlog { pub size:u32,pub final_events_preboot_size:u32,pub version:u8,pub log:[u8;0] }
#[repr(C)] pub struct efi_tcg2_final_events_table { pub version:u64,pub nr_events:u64,pub events:[u8;0] }
#[repr(C)] pub struct linux_efi_coco_secret_area { pub base_pa:u64,pub size:u64 }
#[repr(C)] pub struct linux_efi_initrd { pub base:usize,pub size:usize }

extern "C" { pub static mut efi_rng_seed:usize; pub fn efi_native_runtime_setup(); pub fn efi_init(); pub fn efi_earlycon_reprobe(); pub fn efi_status_to_err(status:efi_status_t)->c_int; pub fn efi_tpm_eventlog_init()->c_int; pub static mut efi_tpm_final_log_size:c_int; pub static mut rci2_table_phys:usize; }

pub const EFI_MEMMAP_LATE:usize=1; pub const EFI_MEMMAP_MEMBLOCK:usize=2; pub const EFI_MEMMAP_SLAB:usize=4;
pub const EFI_RT_PROPERTIES_TABLE_VERSION:u16=1; pub const EFI_INVALID_TABLE_ADDR:usize=!0;
pub const EFI_MEMORY_ATTRIBUTES_FLAGS_RT_FORWARD_CONTROL_FLOW_GUARD:u32=1;
pub const EFI_BOOT:i32=0; pub const EFI_CONFIG_TABLES:i32=2; pub const EFI_RUNTIME_SERVICES:i32=3; pub const EFI_MEMMAP:i32=4; pub const EFI_64BIT:i32=5; pub const EFI_PARAVIRT:i32=6; pub const EFI_ARCH_1:i32=7; pub const EFI_DBG:i32=8; pub const EFI_MEM_ATTR:i32=9; pub const EFI_MEM_NO_SOFT_RESERVE:i32=10; pub const EFI_PRESERVE_BS_REGIONS:i32=11;
pub const EFI_DEV_HW:u8=1; pub const EFI_DEV_ACPI:u8=2; pub const EFI_DEV_MSG:u8=3; pub const EFI_DEV_MEDIA:u8=4; pub const EFI_DEV_BIOS_BOOT:u8=5; pub const EFI_DEV_END_PATH:u8=0x7f; pub const EFI_DEV_END_PATH2:u8=0xff;
pub const EFI_VAR_NAME_LIMIT:usize=1024;

#[repr(C)] pub struct efi_boot_memmap { pub map_size:usize,pub desc_size:usize,pub desc_ver:u32,pub map_key:usize,pub buff_size:usize,pub map:[efi_memory_desc_t;0] }
#[repr(C)] pub struct efi_unaccepted_memory { pub version:u32,pub unit_size:u32,pub phys_base:u64,pub size:u64,pub bitmap:[usize;0] }
#[repr(C)] pub struct efi_config_table_type_t { pub guid:efi_guid_t,pub ptr:*mut usize,pub name:[c_char;16] }
#[repr(C)] pub struct efi_signature_data_t { pub signature_owner:efi_guid_t,pub signature_data:[u8;0] }
#[repr(C)] pub struct efi_signature_list_t { pub signature_type:efi_guid_t,pub signature_list_size:u32,pub signature_header_size:u32,pub signature_size:u32,pub signature_header:[u8;0] }
pub type efi_sha256_hash_t=[u8;32];
#[repr(C)] pub struct efi_cert_x509_sha256_t { pub to_be_signed_hash:efi_sha256_hash_t,pub time_of_revocation:efi_time_t }
#[repr(C)] pub struct efivar_operations { pub get_variable:Option<efi_get_variable_t>,pub get_next_variable:Option<efi_get_next_variable_t>,pub set_variable:Option<efi_set_variable_t>,pub set_variable_nonblocking:Option<efi_set_variable_t>,pub query_variable_store:Option<efi_query_variable_store_t>,pub query_variable_info:Option<efi_query_variable_info_t> }
#[repr(C)] pub struct efivars { pub kset:*mut kset,pub ops:*const efivar_operations }
#[repr(C)] pub struct linux_efi_memreserve { pub size:c_int,pub count:atomic_t,pub next:phys_addr_t,pub entry:[efi_memreserve_entry;0] }
#[repr(C)] pub struct efi_memreserve_entry { pub base:phys_addr_t,pub size:phys_addr_t }
#[repr(C)] pub struct efi_mokvar_table_entry { pub name:[c_char;256],pub data_size:u64,pub data:[u8;0] }
#[repr(C)] pub struct efi_mokvar_table_entry_dummy;
#[repr(C)] pub struct efi_runtime_work { pub args:*mut c_void,pub status:efi_status_t,pub work:work_struct,pub efi_rts_id:efi_rts_ids,pub efi_rts_comp:completion,pub caller:*const c_void }
#[repr(C)] pub enum efi_rts_ids { EFI_NONE,EFI_GET_TIME,EFI_SET_TIME,EFI_GET_WAKEUP_TIME,EFI_SET_WAKEUP_TIME,EFI_GET_VARIABLE,EFI_GET_NEXT_VARIABLE,EFI_SET_VARIABLE,EFI_QUERY_VARIABLE_INFO,EFI_GET_NEXT_HIGH_MONO_COUNT,EFI_RESET_SYSTEM,EFI_UPDATE_CAPSULE,EFI_QUERY_CAPSULE_CAPS,EFI_ACPI_PRM_HANDLER }
pub const EFIVAR_OPS_RDONLY:i32=0; pub const EFIVAR_OPS_RDWR:i32=1;
pub const EFI_SECRET_TABLE_HEADER_GUID:efi_guid_t=EFI_GUID!(0x1e74f542,0x71dd,0x4d66,0x96,0x3e,0xef,0x42,0x87,0xff,0x17,0x3b);

extern "C" {
 pub fn efi_capsule_setup_info(cap_info:*mut capsule_info,kbuff:*mut c_void,hdr_bytes:usize)->c_int;
 pub fn __efi_capsule_setup_info(cap_info:*mut capsule_info)->c_int;
 pub fn efi_config_parse_tables(config_tables:*const efi_config_table_t,count:c_int,arch_tables:*const efi_config_table_type_t)->c_int;
 pub fn efi_mem_type(phys_addr:usize)->c_int; pub fn efi_mem_attributes(phys_addr:usize)->u64; pub fn efi_mem_attribute(phys_addr:usize,size:usize)->u64;
 pub fn efi_mem_reserve(addr:phys_addr_t,size:u64); pub fn efi_mem_reserve_persistent(addr:phys_addr_t,size:u64)->c_int;
 pub fn efivars_register(efivars:*mut efivars,ops:*const efivar_operations)->c_int; pub fn efivars_unregister(efivars:*mut efivars)->c_int;
 pub fn efivar_lock()->c_int; pub fn efivar_trylock()->c_int; pub fn efivar_unlock();
 pub fn efi_query_variable_store(attributes:u32,size:usize,nonblocking:bool)->efi_status_t;
 pub fn efi_call_virt_check_flags(flags:usize,caller:*const c_void); pub fn efi_call_virt_save_flags()->usize;
 pub fn efi_runtime_assert_lock_held(); pub fn efi_arch_mem_reserve(addr:phys_addr_t,size:u64);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
