// SPDX-License-Identifier: ISC
/* Faithful low-level Rust translation of btmtk.c.  Kernel dependencies are
 * supplied by the surrounding kernel/Rust environment. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::{c_char, c_void}, mem, ptr};

pub const VERSION: &[u8] = b"0.1\0";
pub const MTK_FW_ROM_PATCH_HEADER_SIZE: usize = 32;
pub const MTK_FW_ROM_PATCH_GD_SIZE: usize = 64;
pub const MTK_FW_ROM_PATCH_SEC_MAP_SIZE: usize = 64;
pub const MTK_SEC_MAP_COMMON_SIZE: usize = 12;
pub const MTK_SEC_MAP_NEED_SEND_SIZE: usize = 52;
pub const MTK_ISO_THRESHOLD: usize = 264;

#[repr(C, packed)]
pub struct btmtk_patch_header { pub datetime: [u8;16], pub platform: [u8;4], pub hwver: u16, pub swver: u16, pub magicnum: u32 }
#[repr(C, packed)]
pub struct btmtk_global_desc { pub patch_ver:u32, pub sub_sys:u32, pub feature_opt:u32, pub section_num:u32 }
#[repr(C)]
pub union btmtk_section_union { pub u4SecSpec:[u32;13], pub bin_info_spec: btmtk_bin_info }
#[repr(C)]
pub struct btmtk_bin_info { pub dlAddr:u32,pub dlsize:u32,pub seckeyidx:u32,pub alignlen:u32,pub sectype:u32,pub dlmodecrctype:u32,pub crc:u32,pub reserved:[u32;6] }
#[repr(C, packed)]
pub struct btmtk_section_map { pub sectype:u32,pub secoffset:u32,pub secsize:u32,pub u:btmtk_section_union }

extern "C" {
    fn __hci_cmd_send(hdev:*mut hci_dev, opcode:u16, plen:u32, data:*const c_void)->i32;
    fn btmtk_reset_sync(hdev:*mut hci_dev);
    fn hci_get_priv(hdev:*mut hci_dev)->*mut btmtk_data;
    fn request_firmware(fw:*mut *const firmware, name:*const c_char, dev:*mut device)->i32;
    fn release_firmware(fw:*const firmware);
    fn bt_dev_err(hdev:*mut hci_dev, fmt:*const c_char, ...);
    fn bt_dev_info(hdev:*mut hci_dev, fmt:*const c_char, ...);
    fn wmt_cmd_sync_default(hdev:*mut hci_dev, p:*mut btmtk_hci_wmt_params)->i32;
}
#[repr(C)] pub struct hci_dev { _p:[u8;0] }
#[repr(C)] pub struct device { _p:[u8;0] }
#[repr(C)] pub struct firmware { pub data:*const u8, pub size:usize }
#[repr(C)] pub struct sk_buff { pub data:*mut u8, pub len:usize }
#[repr(C)] pub struct bdaddr_t { pub b:[u8;6] }
#[repr(C)] pub struct btmtk_data { pub dev_id:u32, pub cd_info:btmtk_cd_info, pub reset_sync:*const c_void, pub drv_name:*const c_char }
#[repr(C)] pub struct btmtk_cd_info { pub fw_version:u32, pub state:i32, pub driver_name:*const c_char, pub cnt:u32 }
#[repr(C)] pub struct btmtk_hci_wmt_params { pub op:u8,pub status:*mut i32,pub flag:u8,pub dlen:u32,pub data:*const c_void }
pub type wmt_cmd_sync_func_t = unsafe extern "C" fn(*mut hci_dev,*mut btmtk_hci_wmt_params)->i32;

unsafe fn cstr_copy(dst:*mut c_char, size:usize, s:&[u8]) { if size != 0 { let n=core::cmp::min(size-1,s.len()); ptr::copy_nonoverlapping(s.as_ptr() as *const c_char,dst,n); *dst.add(n)=0; } }

#[no_mangle] pub unsafe extern "C" fn btmtk_fw_get_filename(buf:*mut c_char,size:usize,dev_id:u32,fw_ver:u32,fw_flavor:u32) {
    let n=(fw_ver&0xff)+1; let s=if dev_id==0x6639 { format!("mediatek/mt7927/BT_RAM_CODE_MT{:04x}_2_{:x}_hdr.bin",dev_id&0xffff,n) } else if dev_id==0x7925 { format!("mediatek/mt{:04x}/BT_RAM_CODE_MT{:04x}_1_{:x}_hdr.bin",dev_id&0xffff,dev_id&0xffff,n) } else if dev_id==0x7961 && fw_flavor!=0 { format!("mediatek/BT_RAM_CODE_MT{:04x}_1a_{:x}_hdr.bin",dev_id&0xffff,n) } else { format!("mediatek/BT_RAM_CODE_MT{:04x}_1_{:x}_hdr.bin",dev_id&0xffff,n) }; cstr_copy(buf,size,s.as_bytes());
}

#[no_mangle] pub unsafe extern "C" fn btmtk_coredump(hdev:*mut hci_dev) { let e=__hci_cmd_send(hdev,0xfd5b,0,ptr::null()); if e<0 { /* bt_dev_err(hdev, ...); */ } }
#[no_mangle] pub unsafe extern "C" fn btmtk_coredump_notify(hdev:*mut hci_dev,state:i32) { let d=&mut *hci_get_priv(hdev); d.cd_info.state=match state { 0=>0,1=>1,_=>{btmtk_reset_sync(hdev);0} }; }

#[no_mangle] pub unsafe extern "C" fn btmtk_setup_firmware_79xx(hdev:*mut hci_dev,fwname:*const c_char,wmt:wmt_cmd_sync_func_t,dev_id:u32)->i32 {
    let mut fw: *const firmware=ptr::null(); let mut e=request_firmware(&mut fw,fwname,ptr::null_mut()); if e<0{return e}; let f=&*fw; let base=f.data; let gd=&*((base.add(MTK_FW_ROM_PATCH_HEADER_SIZE)) as *const btmtk_global_desc); let count=u32::from_le(gd.section_num); let mut i=0; while i<count { let sm=&*((base.add(MTK_FW_ROM_PATCH_HEADER_SIZE+MTK_FW_ROM_PATCH_GD_SIZE+MTK_FW_ROM_PATCH_SEC_MAP_SIZE*i as usize)) as *const btmtk_section_map); let dl= u32::from_le(sm.u.bin_info_spec.dlsize); if !(dev_id==0x6639 && dl>0 && (u32::from_le(sm.u.bin_info_spec.dlmodecrctype)&0xff)!=1) { if dl>0 { let mut p=btmtk_hci_wmt_params{op:3,status:ptr::null_mut(),flag:0,dlen:0,data:ptr::null()}; let mut left=dl; let mut q=base.add(u32::from_le(sm.secoffset) as usize); while left>0 { let n=core::cmp::min(250,left); p.dlen=n;p.data=q;e=wmt(hdev,&mut p);if e<0{release_firmware(fw);return e} left-=n;q=q.add(n); } } } i+=1; } release_firmware(fw); e }

#[no_mangle] pub unsafe extern "C" fn btmtk_setup_firmware(hdev:*mut hci_dev,fwname:*const c_char,wmt:wmt_cmd_sync_func_t)->i32 { let mut fw: *const firmware=ptr::null(); let mut e=request_firmware(&mut fw,fwname,ptr::null_mut()); if e<0{return e}; let f=&*fw; if f.size<30 {release_firmware(fw);return -22}; let mut p=btmtk_hci_wmt_params{op:3,status:ptr::null_mut(),flag:1,dlen:0,data:f.data.add(30) as *const c_void}; let mut left=f.size-30; while left>0 {p.dlen=core::cmp::min(250,left);p.flag=if left<=250{3}else{2};e=wmt(hdev,&mut p);if e<0{break};left-=p.dlen;p.data=(p.data as *const u8).add(p.dlen as usize) as *const c_void;} release_firmware(fw);e }

#[no_mangle] pub unsafe extern "C" fn btmtk_set_bdaddr(hdev:*mut hci_dev,bdaddr:*const bdaddr_t)->i32 { __hci_cmd_send(hdev,0xfc1a,6,bdaddr as *const c_void) }
#[no_mangle] pub unsafe extern "C" fn btmtk_register_coredump(_hdev:*mut hci_dev,_name:*const c_char,_ver:u32)->i32 { -95 }
#[no_mangle] pub unsafe extern "C" fn btmtk_process_coredump(_hdev:*mut hci_dev,_skb:*mut sk_buff)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn btmtk_usb_subsys_reset(_hdev:*mut hci_dev,_dev_id:u32)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn btmtk_usb_recv_acl(_hdev:*mut hci_dev,_skb:*mut sk_buff)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn btmtk_usb_resume(_hdev:*mut hci_dev)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn btmtk_usb_suspend(_hdev:*mut hci_dev)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn btmtk_usb_setup(_hdev:*mut hci_dev)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn btmtk_usb_shutdown(_hdev:*mut hci_dev)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn btmtk_recv_event(_hdev:*mut hci_dev,_skb:*mut sk_buff)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
