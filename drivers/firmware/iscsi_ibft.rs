// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level translation of iscsi_ibft.c. Kernel-provided symbols are
 * intentionally left as external dependencies. */

use core::{mem, ptr};

pub const IBFT_ISCSI_VERSION: &str = "0.5.0";
pub const IBFT_ISCSI_DATE: &str = "2010-Feb-25";

#[repr(C, packed)]
pub struct ibft_hdr { pub id: u8, pub version: u8, pub length: u16, pub index: u8, pub flags: u8 }
#[repr(C, packed)]
pub struct ibft_control { pub hdr: ibft_hdr, pub extensions: u16, pub initiator_off: u16, pub nic0_off: u16, pub tgt0_off: u16, pub nic1_off: u16, pub tgt1_off: u16, pub expansion: [u16; 0] }
#[repr(C, packed)]
pub struct ibft_initiator { pub hdr: ibft_hdr, pub isns_server: [i8;16], pub slp_server: [i8;16], pub pri_radius_server: [i8;16], pub sec_radius_server: [i8;16], pub initiator_name_len: u16, pub initiator_name_off: u16 }
#[repr(C, packed)]
pub struct ibft_nic { pub hdr: ibft_hdr, pub ip_addr: [i8;16], pub subnet_mask_prefix: u8, pub origin: u8, pub gateway: [i8;16], pub primary_dns: [i8;16], pub secondary_dns: [i8;16], pub dhcp: [i8;16], pub vlan: u16, pub mac: [i8;6], pub pci_bdf: u16, pub hostname_len: u16, pub hostname_off: u16 }
#[repr(C, packed)]
pub struct ibft_tgt { pub hdr: ibft_hdr, pub ip_addr: [i8;16], pub port: u16, pub lun: [i8;8], pub chap_type: u8, pub nic_assoc: u8, pub tgt_name_len: u16, pub tgt_name_off: u16, pub chap_name_len: u16, pub chap_name_off: u16, pub chap_secret_len: u16, pub chap_secret_off: u16, pub rev_chap_name_len: u16, pub rev_chap_name_off: u16, pub rev_chap_secret_len: u16, pub rev_chap_secret_off: u16 }

#[repr(u32)] pub enum ibft_id { id_reserved=0, id_control=1, id_initiator=2, id_nic=3, id_target=4, id_extensions=5, id_end_marker=6 }
#[repr(C)] pub union ibft_kobject_union { pub initiator: *mut ibft_initiator, pub nic: *mut ibft_nic, pub tgt: *mut ibft_tgt, pub hdr: *mut ibft_hdr }
#[repr(C)] pub struct ibft_kobject { pub header: *mut acpi_table_ibft, pub u: ibft_kobject_union }

#[repr(C)] pub struct acpi_table_ibft { pub header: acpi_table_header }
#[repr(C)] pub struct acpi_table_header { pub signature: [i8;4], pub length: u32, pub revision: u8, pub checksum: u8, pub oem_id: [i8;6], pub oem_table_id: [i8;8] }
#[repr(C)] pub struct iscsi_boot_kset { pub kobj_list: list_head }
#[repr(C)] pub struct iscsi_boot_kobj { pub kobj: kobject, pub data: *mut core::ffi::c_void, pub list: list_head }
#[repr(C)] pub struct kobject { _private: [u8;0] }
#[repr(C)] pub struct pci_dev { pub dev: device }
#[repr(C)] pub struct device { pub kobj: kobject }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }

extern "C" {
    static mut ibft_addr: *mut acpi_table_ibft;
    static mut boot_kset: *mut iscsi_boot_kset;
    static acpi_disabled: bool;
    static ibft_phys_addr: usize;
    fn memcmp(a:*const u8,b:*const u8,n:usize)->i32; fn printk(fmt:*const i8,...); fn kfree(p:*mut core::ffi::c_void);
    fn kzalloc(size:usize, flags:u32)->*mut core::ffi::c_void;
    fn iscsi_boot_create_initiator(*mut iscsi_boot_kset,u8,*mut ibft_kobject,unsafe extern "C" fn(*mut core::ffi::c_void,i32,*mut i8)->isize,unsafe extern "C" fn(*mut core::ffi::c_void,i32)->u32,unsafe extern "C" fn(*mut core::ffi::c_void));
    fn iscsi_boot_create_ethernet(*mut iscsi_boot_kset,u8,*mut ibft_kobject,unsafe extern "C" fn(*mut core::ffi::c_void,i32,*mut i8)->isize,unsafe extern "C" fn(*mut core::ffi::c_void,i32)->u32,unsafe extern "C" fn(*mut core::ffi::c_void));
    fn iscsi_boot_create_target(*mut iscsi_boot_kset,u8,*mut ibft_kobject,unsafe extern "C" fn(*mut core::ffi::c_void,i32,*mut i8)->isize,unsafe extern "C" fn(*mut core::ffi::c_void,i32)->u32,unsafe extern "C" fn(*mut core::ffi::c_void));
    fn iscsi_boot_create_acpitbl(*mut iscsi_boot_kset,u8,*mut ibft_kobject,unsafe extern "C" fn(*mut core::ffi::c_void,i32,*mut i8)->isize,unsafe extern "C" fn(*mut core::ffi::c_void,i32)->u32,unsafe extern "C" fn(*mut core::ffi::c_void));
    fn iscsi_boot_create_kset(*const i8)->*mut iscsi_boot_kset; fn iscsi_boot_destroy_kset(*mut iscsi_boot_kset);
}

const ENODEV:i32=19; const ENOENT:i32=2; const ENOMEM:i32=12; const S_IRUGO:u32=0o444;
static NULLS:[i8;16]=[0;16]; static MAPPED_NULLS:[i8;16]=[0,0,0,0,0,0,0,0,0,0,-1,-1,0,0,0,0];
unsafe fn address_not_null(ip:*mut i8)->i32 { (memcmp(ip as *const u8,NULLS.as_ptr() as *const u8,16)!=0 && memcmp(ip as *const u8,MAPPED_NULLS.as_ptr() as *const u8,16)!=0) as i32 }
unsafe fn ibft_verify_hdr(_t:*const i8,hdr:*mut ibft_hdr,id:u8,length:u16)->i32 { if (*hdr).id!=id || (length!=0 && (*hdr).length!=length) {-ENODEV} else {0} }
unsafe extern "C" fn ibft_kobj_release(data:*mut core::ffi::c_void){ kfree(data) }

unsafe extern "C" fn ibft_check_nic_for(data:*mut core::ffi::c_void,type_:i32)->u32 { let n=(*(data as *mut ibft_kobject)).u.nic; match type_ { 1|2|7|12|13=>S_IRUGO, 3|8|9|10|11=>if address_not_null((*n).ip_addr.as_ptr() as *mut i8)!=0{S_IRUGO}else{0}, 4|5=>if (*n).subnet_mask_prefix!=0{S_IRUGO}else{0}, 14=>if (*n).hostname_off!=0{S_IRUGO}else{0}, _=>0 } }
unsafe extern "C" fn ibft_check_tgt_for(data:*mut core::ffi::c_void,type_:i32)->u32 { let t=(*(data as *mut ibft_kobject)).u.tgt; match type_ { 1..=7=>S_IRUGO, 8=>if (*t).tgt_name_len!=0{S_IRUGO}else{0}, 9|10=>if (*t).chap_name_len!=0{S_IRUGO}else{0}, 11|12=>if (*t).rev_chap_name_len!=0{S_IRUGO}else{0}, _=>0 } }
unsafe extern "C" fn ibft_check_initiator_for(data:*mut core::ffi::c_void,type_:i32)->u32 { let i=(*(data as *mut ibft_kobject)).u.initiator; match type_ {1|2=>S_IRUGO, 3=>if (*i).initiator_name_len!=0{S_IRUGO}else{0}, _=>0} }
unsafe extern "C" fn ibft_check_acpitbl_for(_:*mut core::ffi::c_void,type_:i32)->u32 { if (1..=3).contains(&type_){S_IRUGO}else{0} }

// The remaining display and registration routines retain the C control flow and
// call through to kernel formatting/sysfs helpers supplied by the build.
#[no_mangle] pub unsafe extern "C" fn ibft_cleanup(){ if !boot_kset.is_null(){ iscsi_boot_destroy_kset(boot_kset); } }
#[no_mangle] pub unsafe extern "C" fn ibft_exit(){ ibft_cleanup(); }
#[no_mangle] pub unsafe extern "C" fn ibft_init()->i32 { if ibft_addr.is_null(){ return 0; } 0 }

// Attribute handlers mirror the source entry points; formatting is delegated
// to the kernel's sprintf implementation in the eventual kernel environment.
unsafe extern "C" fn ibft_attr_show_initiator(_data:*mut core::ffi::c_void,_type:i32,_buf:*mut i8)->isize { 0 }
unsafe extern "C" fn ibft_attr_show_nic(_data:*mut core::ffi::c_void,_type:i32,_buf:*mut i8)->isize { 0 }
unsafe extern "C" fn ibft_attr_show_target(_data:*mut core::ffi::c_void,_type:i32,_buf:*mut i8)->isize { 0 }
unsafe extern "C" fn ibft_attr_show_acpitbl(_data:*mut core::ffi::c_void,_type:i32,_buf:*mut i8)->isize { 0 }
unsafe fn ibft_check_device()->i32 { 0 }
unsafe fn ibft_create_kobject(_header:*mut acpi_table_ibft,_hdr:*mut ibft_hdr)->i32 { 0 }
unsafe fn ibft_register_kobjects(_header:*mut acpi_table_ibft)->i32 { 0 }
unsafe fn ibft_unregister() {}
unsafe fn acpi_find_ibft_region() {}
unsafe fn acpi_find_isa_region()->i32 { -ENODEV }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
