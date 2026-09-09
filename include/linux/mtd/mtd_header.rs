/* SPDX-License-Identifier: GPL-2.0-or-later */
/* C header translation; included dependencies are supplied externally. */

pub const MTD_FAIL_ADDR_UNKNOWN: i64 = -1;

pub type UChar = u8;
pub type U64 = u64;
pub type U32 = u32;
pub type LoFF = i64;
pub type ResourceSize = usize;

#[repr(C)] pub struct mtd_info { pub type_: UChar, pub flags: u32, pub size: u64, pub erasesize: u32, pub writesize: u32, pub writebufsize: u32, pub oobsize: u32, pub oobavail: u32, pub erasesize_shift: u32, pub writesize_shift: u32, pub erasesize_mask: u32, pub writesize_mask: u32, pub bitflip_threshold: u32, pub name: *const i8, pub index: i32, pub ooblayout: *const mtd_ooblayout_ops, pub pairing: *const mtd_pairing_scheme, pub ecc_step_size: u32, pub ecc_strength: u32, pub numeraseregions: i32, pub eraseregions: *mut mtd_erase_region_info,
    pub _erase: Option<unsafe extern "C" fn(*mut mtd_info,*mut erase_info)->i32>, pub _point: Option<unsafe extern "C" fn(*mut mtd_info,LoFF,usize,*mut usize,*mut *mut core::ffi::c_void,*mut ResourceSize)->i32>, pub _unpoint: Option<unsafe extern "C" fn(*mut mtd_info,LoFF,usize)->i32>, pub _read: Option<unsafe extern "C" fn(*mut mtd_info,LoFF,usize,*mut usize,*mut UChar)->i32>, pub _write: Option<unsafe extern "C" fn(*mut mtd_info,LoFF,usize,*mut usize,*const UChar)->i32>, pub _panic_write: Option<unsafe extern "C" fn(*mut mtd_info,LoFF,usize,*mut usize,*const UChar)->i32>, pub _read_oob: Option<unsafe extern "C" fn(*mut mtd_info,LoFF,*mut mtd_oob_ops)->i32>, pub _write_oob: Option<unsafe extern "C" fn(*mut mtd_info,LoFF,*mut mtd_oob_ops)->i32>, pub _get_fact_prot_info: Option<unsafe extern "C" fn(*mut mtd_info,usize,*mut usize,*mut otp_info)->i32>, pub _read_fact_prot_reg: Option<unsafe extern "C" fn(*mut mtd_info,LoFF,usize,*mut usize,*mut UChar)->i32>, pub _get_user_prot_info: Option<unsafe extern "C" fn(*mut mtd_info,usize,*mut usize,*mut otp_info)->i32>, pub _read_user_prot_reg: Option<unsafe extern "C" fn(*mut mtd_info,LoFF,usize,*mut usize,*mut UChar)->i32>, pub _write_user_prot_reg: Option<unsafe extern "C" fn(*mut mtd_info,LoFF,usize,*mut usize,*const UChar)->i32>, pub _lock_user_prot_reg: Option<unsafe extern "C" fn(*mut mtd_info,LoFF,usize)->i32>, pub _erase_user_prot_reg: Option<unsafe extern "C" fn(*mut mtd_info,LoFF,usize)->i32>, pub _writev: Option<unsafe extern "C" fn(*mut mtd_info,*const kvec,usize,LoFF,*mut usize)->i32>, pub _sync: Option<unsafe extern "C" fn(*mut mtd_info)>, pub _lock: Option<unsafe extern "C" fn(*mut mtd_info,LoFF,u64)->i32>, pub _unlock: Option<unsafe extern "C" fn(*mut mtd_info,LoFF,u64)->i32>, pub _is_locked: Option<unsafe extern "C" fn(*mut mtd_info,LoFF,u64)->i32>, pub _block_isreserved: Option<unsafe extern "C" fn(*mut mtd_info,LoFF)->i32>, pub _block_isbad: Option<unsafe extern "C" fn(*mut mtd_info,LoFF)->i32>, pub _block_markbad: Option<unsafe extern "C" fn(*mut mtd_info,LoFF)->i32>, pub _max_bad_blocks: Option<unsafe extern "C" fn(*mut mtd_info,LoFF,usize)->i32>, pub _suspend: Option<unsafe extern "C" fn(*mut mtd_info)->i32>, pub _resume: Option<unsafe extern "C" fn(*mut mtd_info)>, pub _reboot: Option<unsafe extern "C" fn(*mut mtd_info)>, pub _get_device: Option<unsafe extern "C" fn(*mut mtd_info)->i32>, pub _put_device: Option<unsafe extern "C" fn(*mut mtd_info)>, pub oops_panic_write: bool, pub reboot_notifier: notifier_block, pub ecc_stats: mtd_ecc_stats, pub subpage_sft: i32, pub priv_: *mut core::ffi::c_void, pub owner: *mut module, pub dev: device, pub refcnt: kref, pub dbg: mtd_debug_info, pub nvmem: *mut nvmem_device, pub otp_user_nvmem: *mut nvmem_device, pub otp_factory_nvmem: *mut nvmem_device, pub parent: *mut mtd_info, pub partitions: list_head, pub part: mtd_part, pub master: mtd_master }

#[repr(C)] pub struct erase_info { pub addr:u64, pub len:u64, pub fail_addr:u64 }
#[repr(C)] pub struct mtd_erase_region_info { pub offset:u64, pub erasesize:u32, pub numblocks:u32, pub lockmap:*mut usize }
#[repr(C)] pub struct mtd_req_stats { pub uncorrectable_errors:u32, pub corrected_bitflips:u32, pub max_bitflips:u32 }
#[repr(C)] pub struct mtd_oob_ops { pub mode:u32, pub len:usize, pub retlen:usize, pub ooblen:usize, pub oobretlen:usize, pub ooboffs:u32, pub datbuf:*mut u8, pub oobbuf:*mut u8, pub stats:*mut mtd_req_stats }
#[repr(C)] pub struct mtd_oob_region { pub offset:u32, pub length:u32 }
#[repr(C)] pub struct mtd_ooblayout_ops { pub ecc:Option<unsafe extern "C" fn(*mut mtd_info,i32,*mut mtd_oob_region)->i32>, pub free:Option<unsafe extern "C" fn(*mut mtd_info,i32,*mut mtd_oob_region)->i32> }
#[repr(C)] pub struct mtd_pairing_info { pub pair:i32, pub group:i32 }
#[repr(C)] pub struct mtd_pairing_scheme { pub ngroups:i32, pub get_info:Option<unsafe extern "C" fn(*mut mtd_info,i32,*mut mtd_pairing_info)->i32>, pub get_wunit:Option<unsafe extern "C" fn(*mut mtd_info,*const mtd_pairing_info)->i32> }
#[repr(C)] pub struct mtd_debug_info { pub dfs_dir:*mut dentry }
#[repr(C)] pub struct mtd_part { pub node:list_head, pub offset:u64, pub size:u64, pub flags:u32 }
#[repr(C)] pub struct mtd_master { pub partitions_lock:mutex, pub chrdev_lock:mutex, pub suspended:u32 }
#[repr(C)] pub struct mtd_notifier { pub add:Option<unsafe extern "C" fn(*mut mtd_info)>, pub remove:Option<unsafe extern "C" fn(*mut mtd_info)>, pub list:list_head }

#[repr(C)] pub struct mtd_ecc_stats { _private:[u8;0] }
#[repr(C)] pub struct notifier_block { _private:[u8;0] }
#[repr(C)] pub struct device { pub of_node:*mut device_node, _private:[u8;0] }
#[repr(C)] pub struct kref { _private:[u8;0] }
#[repr(C)] pub struct list_head { _private:[u8;0] }
#[repr(C)] pub struct mutex { _private:[u8;0] }
#[repr(C)] pub struct dentry { _private:[u8;0] }
#[repr(C)] pub struct module { _private:[u8;0] }
#[repr(C)] pub struct nvmem_device { _private:[u8;0] }
#[repr(C)] pub struct device_node { _private:[u8;0] }
#[repr(C)] pub struct otp_info { _private:[u8;0] }
#[repr(C)] pub struct kvec { _private:[u8;0] }
#[repr(C)] pub struct mtd_partition { _private:[u8;0] }
#[repr(C)] pub struct mtd_part_parser_data { _private:[u8;0] }

extern "C" { pub fn list_empty(head:*const list_head)->bool; pub fn dev_of_node(dev:*const device)->*mut device_node; pub fn of_property_read_string(np:*mut device_node,name:*const i8,out:*mut *const i8)->i32; }

pub unsafe fn mtd_get_master(mut mtd:*mut mtd_info)->*mut mtd_info { while !(*mtd).parent.is_null(){mtd=(*mtd).parent;} mtd }
pub unsafe fn mtd_get_master_ofs(mut mtd:*mut mtd_info,mut ofs:u64)->u64 { while !(*mtd).parent.is_null(){ofs=ofs.wrapping_add((*mtd).part.offset);mtd=(*mtd).parent;} ofs }
pub unsafe fn mtd_is_partition(mtd:*const mtd_info)->bool { !(*mtd).parent.is_null() }
pub unsafe fn mtd_has_partitions(mtd:*const mtd_info)->bool { !list_empty(&(*mtd).partitions) }
pub unsafe fn mtd_set_ooblayout(mtd:*mut mtd_info,v:*const mtd_ooblayout_ops){(*mtd).ooblayout=v}
pub unsafe fn mtd_set_pairing_scheme(mtd:*mut mtd_info,v:*const mtd_pairing_scheme){(*mtd).pairing=v}
pub unsafe fn mtd_set_of_node(mtd:*mut mtd_info,np:*mut device_node){(*mtd).dev.of_node=np;if (*mtd).name.is_null(){of_property_read_string(np,b"label\0".as_ptr() as *const i8,&mut (*mtd).name);}}
pub unsafe fn mtd_get_of_node(mtd:*mut mtd_info)->*mut device_node {dev_of_node(&(*mtd).dev)}
pub unsafe fn mtd_oobavail(mtd:*mut mtd_info,ops:*mut mtd_oob_ops)->u32 {if (*ops).mode==1 /* MTD_OPS_AUTO_OOB */ {(*mtd).oobavail}else{(*mtd).oobsize}}
pub unsafe fn mtd_sync(mtd:*mut mtd_info){let master=mtd_get_master(mtd);if let Some(f)=(*master)._sync{f(master)}}
pub unsafe fn mtd_suspend(mtd:*mut mtd_info)->i32{let master=mtd_get_master(mtd);if (*master).master.suspended!=0{return 0} let r=(*master)._suspend.map_or(0,|f|f(master));if r!=0{return r}(*master).master.suspended=1;0}
pub unsafe fn mtd_resume(mtd:*mut mtd_info){let master=mtd_get_master(mtd);if (*master).master.suspended==0{return}if let Some(f)=(*master)._resume{f(master)}(*master).master.suspended=0}
pub unsafe fn mtd_div_by_eb(sz:u64,mtd:*mut mtd_info)->u32{if (*mtd).erasesize_shift!=0{(sz>>(*mtd).erasesize_shift) as u32}else{(sz/(*mtd).erasesize as u64) as u32}}
pub unsafe fn mtd_mod_by_eb(sz:u64,mtd:*mut mtd_info)->u32{if (*mtd).erasesize_shift!=0{(sz&(*mtd).erasesize_mask as u64) as u32}else{(sz%(*mtd).erasesize as u64) as u32}}
pub unsafe fn mtd_div_by_ws(sz:u64,mtd:*mut mtd_info)->u32{if (*mtd).writesize_shift!=0{(sz>>(*mtd).writesize_shift) as u32}else{(sz/(*mtd).writesize as u64) as u32}}
pub unsafe fn mtd_mod_by_ws(sz:u64,mtd:*mut mtd_info)->u32{if (*mtd).writesize_shift!=0{(sz&(*mtd).writesize_mask as u64) as u32}else{(sz%(*mtd).writesize as u64) as u32}}
pub unsafe fn mtd_wunit_per_eb(mtd:*mut mtd_info)->i32{let master=mtd_get_master(mtd);((*master).erasesize/(*mtd).writesize) as i32}
pub unsafe fn mtd_offset_to_wunit(mtd:*mut mtd_info,offs:i64)->i32{mtd_div_by_ws(mtd_mod_by_eb(offs as u64,mtd) as u64,mtd) as i32}
pub unsafe fn mtd_wunit_to_offset(mtd:*mut mtd_info,base:i64,wunit:i32)->i64{base.wrapping_add((wunit as i64).wrapping_mul((*mtd).writesize as i64))}
pub unsafe fn mtd_has_oob(mtd:*const mtd_info)->bool{let master=mtd_get_master(mtd as *mut _);(*master)._read_oob.is_some()&&(*master)._write_oob.is_some()}
pub unsafe fn mtd_can_have_bb(mtd:*const mtd_info)->bool{mtd_get_master(mtd as *mut _).as_ref().unwrap()._block_isbad.is_some()}
pub unsafe fn mtd_type_is_nand(mtd:*const mtd_info)->bool{(*mtd).type_==4||(*mtd).type_==8}
pub unsafe fn mtd_is_bitflip(err:i32)->bool{err==-117}
pub unsafe fn mtd_is_eccerr(err:i32)->bool{err==-74}
pub unsafe fn mtd_is_bitflip_or_eccerr(err:i32)->bool{mtd_is_bitflip(err)||mtd_is_eccerr(err)}

extern "C" {
 pub fn mtd_ooblayout_ecc(_: *mut mtd_info, _:i32, _: *mut mtd_oob_region)->i32; pub fn mtd_ooblayout_find_eccregion(_: *mut mtd_info, _:i32, _: *mut i32, _: *mut mtd_oob_region)->i32; pub fn mtd_ooblayout_get_eccbytes(_: *mut mtd_info,*mut u8,*const u8,i32,i32)->i32; pub fn mtd_ooblayout_set_eccbytes(_: *mut mtd_info,*const u8,*mut u8,i32,i32)->i32; pub fn mtd_ooblayout_free(_: *mut mtd_info, _:i32, _: *mut mtd_oob_region)->i32; pub fn mtd_ooblayout_get_databytes(_: *mut mtd_info,*mut u8,*const u8,i32,i32)->i32; pub fn mtd_ooblayout_set_databytes(_: *mut mtd_info,*const u8,*mut u8,i32,i32)->i32; pub fn mtd_ooblayout_count_freebytes(_: *mut mtd_info)->i32; pub fn mtd_ooblayout_count_eccbytes(_: *mut mtd_info)->i32;
 pub fn mtd_wunit_to_pairing_info(_: *mut mtd_info,i32,*mut mtd_pairing_info)->i32; pub fn mtd_pairing_info_to_wunit(_: *mut mtd_info,*const mtd_pairing_info)->i32; pub fn mtd_pairing_groups(_: *mut mtd_info)->i32; pub fn mtd_erase(_: *mut mtd_info, _: *mut erase_info)->i32; pub fn mtd_point(_: *mut mtd_info,i64,usize,*mut usize,*mut *mut core::ffi::c_void,*mut ResourceSize)->i32; pub fn mtd_unpoint(_: *mut mtd_info,i64,usize)->i32; pub fn mtd_read(_: *mut mtd_info,i64,usize,*mut usize,*mut u8)->i32; pub fn mtd_write(_: *mut mtd_info,i64,usize,*mut usize,*const u8)->i32; pub fn mtd_panic_write(_: *mut mtd_info,i64,usize,*mut usize,*const u8)->i32; pub fn mtd_read_oob(_: *mut mtd_info,i64,*mut mtd_oob_ops)->i32; pub fn mtd_write_oob(_: *mut mtd_info,i64,*mut mtd_oob_ops)->i32;
 pub fn mtd_lock(_: *mut mtd_info,i64,u64)->i32; pub fn mtd_unlock(_: *mut mtd_info,i64,u64)->i32; pub fn mtd_is_locked(_: *mut mtd_info,i64,u64)->i32; pub fn mtd_block_isreserved(_: *mut mtd_info,i64)->i32; pub fn mtd_block_isbad(_: *mut mtd_info,i64)->i32; pub fn mtd_block_markbad(_: *mut mtd_info,i64)->i32; pub fn mtd_mmap_capabilities(_: *mut mtd_info)->u32; pub fn register_mtd_user(_: *mut mtd_notifier); pub fn unregister_mtd_user(_: *mut mtd_notifier)->i32; pub fn mtd_kmalloc_up_to(_: *const mtd_info,*mut usize)->*mut core::ffi::c_void;
 pub fn mtd_device_parse_register(_: *mut mtd_info,*const *const i8,*mut mtd_part_parser_data,*const mtd_partition,i32)->i32; pub fn mtd_device_unregister(_: *mut mtd_info)->i32; pub fn get_mtd_device(_: *mut mtd_info,i32)->*mut mtd_info; pub fn __get_mtd_device(_: *mut mtd_info)->i32; pub fn __put_mtd_device(_: *mut mtd_info); pub fn of_get_mtd_device_by_node(_: *mut device_node)->*mut mtd_info; pub fn get_mtd_device_nm(_: *const i8)->*mut mtd_info; pub fn put_mtd_device(_: *mut mtd_info);
}

/* CONFIG_DEBUG_FS selects the external implementation; otherwise this is false. */
#[inline] pub fn mtd_check_expert_analysis_mode()->bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
