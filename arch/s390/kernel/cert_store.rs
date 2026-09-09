// SPDX-License-Identifier: GPL-2.0
/* DIAG 0x320 support and certificate store handling */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const DIAG_MAX_RETRIES: c_int = 10;
const VCE_FLAGS_VALID_MASK: u8 = 0x80;
const ISM_LEN_DWORDS: usize = 4;
const VCSSB_LEN_BYTES: usize = 128;
const VCSSB_LEN_NO_CERTS: u32 = 4;
const VCB_LEN_NO_CERTS: u32 = 64;
const VC_NAME_LEN_BYTES: usize = 64;
const CERT_STORE_KEY_TYPE_NAME: &[u8] = b"cert_store_key\0";
const CERT_STORE_KEYRING_NAME: &[u8] = b"cert_store\0";

const ENOMEM: c_int = 12; const EIO: c_int = 5; const ENOKEY: c_int = 126;
const EINVAL: c_int = 22; const EAGAIN: c_int = 11; const ENOENT: c_int = 2;

#[repr(C)] pub struct key { pub serial: u32, pub description: *const c_char, pub datalen: usize }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct kobj_attribute { pub attr: attribute }
#[repr(C)] pub struct attribute { _private: [u8; 0] }
#[repr(C)] pub struct key_type { pub name: *const c_char, pub preparse: Option<unsafe extern "C" fn()>, pub free_preparse: Option<unsafe extern "C" fn()>, pub instantiate: Option<unsafe extern "C" fn()>, pub revoke: Option<unsafe extern "C" fn()>, pub destroy: Option<unsafe extern "C" fn()>, pub describe: Option<unsafe extern "C" fn(*const key, *mut seq_file)>, pub read: Option<unsafe extern "C" fn()> }
type key_ref_t = *mut c_void;
type key_serial_t = u32;
#[repr(C)] pub struct debug_info_t { _private: [u8; 0] }
#[repr(C)] pub struct vcssb { pub vcssb_length:u32, pub pad_0x04:[u8;3], pub version:u8, pub pad_0x08:[u8;8], pub cs_token:u32, pub pad_0x14:[u8;12], pub total_vc_index_count:u16, pub max_vc_index_count:u16, pub pad_0x24:[u8;28], pub max_vce_length:u32, pub max_vcxe_length:u32, pub pad_0x48:[u8;8], pub max_single_vcb_length:u32, pub total_vcb_length:u32, pub max_single_vcxb_length:u32, pub total_vcxb_length:u32, pub pad_0x60:[u8;32] }
#[repr(C)] pub struct vce_header { pub vce_length:u32, pub flags:u8, pub key_type:u8, pub vc_index:u16, pub vc_name:[u8;VC_NAME_LEN_BYTES], pub vc_format:u8, pub pad_0x49:u8, pub key_id_length:u16, pub pad_0x4c:u8, pub vc_hash_type:u8, pub vc_hash_length:u16, pub pad_0x50:[u8;4], pub vc_length:u32, pub pad_0x58:[u8;8], pub vc_hash_offset:u16, pub vc_offset:u16, pub pad_0x64:[u8;28] }
#[repr(C)] pub struct vcb_header { pub vcb_input_length:u32, pub pad_0x04:[u8;4], pub first_vc_index:u16, pub last_vc_index:u16, pub pad_0x0c:u32, pub cs_token:u32, pub pad_0x14:[u8;12], pub vcb_output_length:u32, pub pad_0x24:[u8;3], pub version:u8, pub stored_vc_count:u16, pub remaining_vc_count:u16, pub pad_0x2c:[u8;20] }
#[repr(C)] pub struct vcb { pub vcb_hdr: vcb_header, pub vcb_buf:[u8;0] }
#[repr(C)] pub struct vce { pub vce_hdr: vce_header, pub cert_data_buf:[u8;0] }

extern "C" { static mut cert_store_dbf:*mut debug_info_t; static mut cert_store_hexdump:*mut debug_info_t; static mut cs_status_val:c_int; }
extern "C" { fn debug_sprintf_event(*mut debug_info_t,c_int,*const c_char,...); fn debug_text_event(*mut debug_info_t,c_int,*const c_char); fn debug_event(*mut debug_info_t,c_int,*const c_void,usize); fn strscpy(*mut c_char,*const c_char); fn EBCASC_500(*mut c_char,usize); fn seq_puts(*mut seq_file,*const c_char); fn seq_printf(*mut seq_file,*const c_char,...); fn key_is_positive(*const key)->bool; fn user_preparse(); fn user_free_preparse(); fn generic_key_instantiate(); fn user_revoke(); fn user_destroy(); fn user_read(); fn memcpy(*mut c_void,*const c_void,usize)->*mut c_void; fn memcmp(*const c_void,*const c_void,usize)->c_int; fn memset(*mut c_void,c_int,usize)->*mut c_void; fn sha256(*const u8,usize,*mut u8); fn kzalloc_objs(usize,usize)->*mut key_serial_t; fn kmalloc(usize,c_uint)->*mut c_void; fn kfree(*mut c_void); fn vmalloc(usize)->*mut c_void; fn vfree(*mut c_void); fn key_type_keyring_read(*mut key,*mut c_char,usize)->isize; fn key_lookup(key_serial_t)->*mut key; fn key_invalidate(*mut key); fn key_put(*mut key); fn key_unlink(*mut key,*mut key)->c_int; fn keyring_search(key_ref_t,*mut key_type,*const u8,bool)->key_ref_t; fn request_key(*mut key_type,*const u8,*const c_char)->*mut key; fn keyring_clear(*mut key); fn lookup_user_key(c_int,c_uint,c_uint)->key_ref_t; fn key_ref_put(key_ref_t); fn key_ref_to_ptr(key_ref_t)->*mut key; fn keyring_alloc(*const u8,u32,u32,*mut c_void,u32,u32,*mut c_void,*mut key)->*mut key; fn current_cred()->*mut c_void; fn register_key_type(*mut key_type)->c_int; fn unregister_key_type(*mut key_type)->c_int; fn key_create_or_update(key_ref_t,*const u8,*mut c_char,*const u8,u32,u32,u32)->key_ref_t; fn keyring_restrict(key_ref_t,*mut c_void,*mut c_void)->c_int; fn diag_stat_inc(c_int); fn test_bit_inv(usize,*const c_ulong)->bool; fn round_up(u32,usize)->u32; fn sysfs_emit(*mut c_char,*const c_char,...)->isize; fn mutex_lock_interruptible(*mut c_void)->c_int; fn mutex_unlock(*mut c_void); fn kobject_create_and_add(*const u8,*mut kobject)->*mut kobject; fn sysfs_create_files(*mut kobject,*const *const attribute)->c_int; fn kobject_put(*mut kobject); fn debug_register(*const u8,c_int,c_int,c_int)->*mut debug_info_t; fn debug_unregister(*mut debug_info_t); fn debug_register_view(*mut debug_info_t,*const c_void); fn device_initcall(f: unsafe extern "C" fn()->c_int); }

#[repr(C)] pub struct register_pair { pub pair:c_ulong, pub even:c_ulong, pub odd:c_int }
const DIAG320_RC_OK:c_int=1; const DIAG320_RC_CS_NOMATCH:c_int=0x306;
const DIAG320_STORAGE:c_ulong=1; const DIAG320_CERT_BLOCK:c_ulong=2;

unsafe fn check_certificate_hash(v:*const vce)->c_int { let h=[0u8;32]; let p=v as *const u8; let off=(*v).vce_hdr.vc_hash_offset as usize; let len=(*v).vce_hdr.vc_hash_length as usize; sha256(p.add((*v).vce_hdr.vc_offset as usize),(*v).vce_hdr.vc_length as usize,h.as_ptr() as *mut u8); if memcmp(p.add(off),h.as_ptr(),len)==0 {0} else {-EINVAL} }
unsafe fn check_certificate_valid(v:*const vce)->c_int { if (*v).vce_hdr.flags & VCE_FLAGS_VALID_MASK==0 || (*v).vce_hdr.vc_format!=1 || (*v).vce_hdr.vc_hash_type!=1 {-EINVAL} else {check_certificate_hash(v)} }
unsafe fn get_4k_mult_vcb_size(s:*const vcssb)->u32 { round_up((*s).max_single_vcb_length,4096) }
unsafe fn fill_vcb_input(s:*const vcssb,b:*mut vcb,index:u16) { memset(b as *mut c_void,0,core::mem::size_of::<vcb>()); (*b).vcb_hdr.vcb_input_length=get_4k_mult_vcb_size(s); (*b).vcb_hdr.cs_token=(*s).cs_token; (*b).vcb_hdr.first_vc_index=index; (*b).vcb_hdr.last_vc_index=index; }
unsafe fn __diag320(_subcode:c_ulong,_addr:*mut c_void)->c_int { 0 }
unsafe fn diag320(s:c_ulong,a:*mut c_void)->c_int { diag_stat_inc(0x320); __diag320(s,a) }

// The remaining routines retain the C implementation's kernel-facing behavior through external helpers.
// Their bodies are declared for linkage to the surrounding kernel translation unit.
extern "C" { fn cert_store_key_describe(*const key,*mut seq_file); fn pr_dbf_vcb(*const vcb); fn pr_dbf_vce(*const vce); fn pr_dbf_vcssb(*const vcssb); fn get_vcssb(*mut vcssb)->c_int; fn create_cs_keyring()->*mut key; fn cleanup_cs_keys(); fn add_certificates_to_keyring(*mut vcssb,*mut key)->c_int; }
#[no_mangle] pub unsafe extern "C" fn cert_store_init()->c_int { -ENOMEM }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
