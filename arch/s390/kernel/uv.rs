// SPDX-License-Identifier: GPL-2.0
/* Common Ultravisor functions and initialization. */

// Kernel dependencies supplied by other translation units.
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut prot_virt_guest: c_int;
    static mut uv_info: uv_info;
    static mut prot_virt_host: c_int;
    static mut init_mm: mm_struct;
    static mut firmware_kobj: *mut kobject;
    fn uv_call(cc: u64, uvcb: u64) -> u16;
    fn __uv_call(cc: u64, uvcb: u64) -> u16;
    fn is_prot_virt_host() -> bool;
    fn memblock_alloc_try_nid(size: usize, align: usize, max_addr: usize, flags: u32, nid: i32) -> *mut c_void;
    fn memblock_free(ptr: *mut c_void, size: usize);
    fn folio_get(folio: *mut folio); fn folio_put(folio: *mut folio);
    fn folio_order(folio: *mut folio) -> u32; fn folio_to_phys(folio: *mut folio) -> usize;
    fn clear_bit(n: u32, p: *mut u64); fn set_bit(n: u32, p: *mut u64); fn test_bit(n: u32, p: *const u64) -> bool;
    fn pte_present(pte: pte_t) -> bool; fn pte_pfn(pte: pte_t) -> usize; fn pfn_folio(pfn: usize) -> *mut folio;
    fn pte_page(pte: pte_t) -> *mut page; fn ptep_get(ptep: *mut pte_t) -> pte_t;
    fn __free_page(page: *mut page); fn page_to_phys(page: *mut page) -> usize;
    fn alloc_page(flags: u32) -> *mut page; fn set_pte(ptep: *mut pte_t, pte: pte_t);
    fn get_vm_area(size: usize, flags: u32) -> *mut vm_struct; fn free_vm_area(area: *mut vm_struct);
    fn find_vm_area(addr: *mut c_void) -> *mut vm_struct; fn get_vm_area_size(area: *mut vm_struct) -> usize;
    fn apply_to_existing_page_range(mm: *mut mm_struct, addr: usize, size: usize, cb: unsafe extern "C" fn(*mut pte_t, usize, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn apply_to_page_range(mm: *mut mm_struct, addr: usize, size: usize, cb: unsafe extern "C" fn(*mut pte_t, usize, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn folio_mapcount(f: *mut folio) -> i32; fn folio_test_swapcache(f: *mut folio) -> bool; fn folio_mapping(f: *mut folio) -> *mut address_space;
    fn folio_ref_freeze(f: *mut folio, n: i32) -> bool; fn folio_ref_unfreeze(f: *mut folio, n: i32);
    fn folio_test_writeback(f: *mut folio) -> bool; fn folio_test_large(f: *mut folio) -> bool; fn folio_wait_writeback(f: *mut folio);
    fn lru_add_drain_all(); fn folio_lock(f: *mut folio); fn folio_unlock(f: *mut folio); fn split_folio(f: *mut folio) -> c_int;
    fn folio_test_dirty(f: *mut folio) -> bool; fn folio_test_anon(f: *mut folio) -> bool; fn mapping_can_writeback(m: *mut address_space) -> bool;
    fn folio_pos(f: *mut folio) -> i64; fn folio_size(f: *mut folio) -> usize; fn igrab(i: *mut inode) -> *mut inode; fn iput(i: *mut inode);
    fn filemap_write_and_wait_range(m: *mut address_space, start: i64, end: i64) -> c_int;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize; fn sysfs_emit_at(buf: *mut c_char, at: isize, fmt: *const c_char, ...) -> isize;
    fn kset_create_and_add(name: *const c_char, u: *mut c_void, parent: *mut kobject) -> *mut kset; fn sysfs_create_group(k: *mut kobject, g: *const attribute_group) -> c_int; fn kset_unregister(k: *mut kset);
    fn kobject_create_and_add(name: *const c_char, parent: *mut kobject) -> *mut kobject; fn sysfs_create_files(k: *mut kobject, a: *const *mut attribute) -> c_int; fn sysfs_remove_files(k: *mut kobject, a: *const *mut attribute); fn kobject_del(k: *mut kobject); fn kobject_put(k: *mut kobject);
    fn test_facility(n: u32) -> bool; fn test_bit_inv(n: u32, p: *const u64) -> bool; fn uv_list_secrets(list: *mut uv_secret_list, start: u16, rc: *mut u16, x: *mut c_void);
    fn memcmp(a: *const u8, b: *const u8, n: usize) -> c_int;
}

#[repr(C)] pub struct uv_info { pub uv_base_stor_len: usize, pub inst_calls_list: [usize; 4], pub supp_se_hdr_ver: usize, pub supp_se_hdr_pcf: usize, pub guest_cpu_stor_len: usize, pub conf_dump_storage_state_len: usize, pub conf_dump_finalize_len: usize, pub uv_feature_indications: usize, pub max_guest_cpu_id: i32, pub max_num_sec_conf: i32, pub max_sec_stor_addr: usize, pub supp_att_req_hdr_ver: usize, pub supp_att_pflags: usize, pub supp_add_secret_req_ver: usize, pub supp_add_secret_pcf: usize, pub supp_secret_types: usize, pub max_assoc_secrets: i32, pub max_retr_secrets: i32 }
#[repr(C)] pub struct uv_cb_header { pub cmd:u16, pub len:u16, pub rc:u16, pub rrc:u16 }
#[repr(C)] pub struct uv_cb_init { pub header:uv_cb_header, pub stor_origin:usize, pub stor_len:usize }
#[repr(C)] pub struct uv_cb_cfs { pub header:uv_cb_header, pub paddr:usize }
#[repr(C)] pub struct uv_key_hash { pub dword:[u64;4] }
#[repr(C)] pub struct uv_cb_query_keys { pub header:uv_cb_header, pub key_hashes:[uv_key_hash;4] }
#[repr(C)] pub struct uv_cb_retr_secr { pub header:uv_cb_header, pub secret_idx:u16, pub buf_addr:u64, pub buf_size:usize }
#[repr(C)] pub struct uv_secret_list_item_hdr { _private:[u8;0] }
#[repr(C)] pub struct uv_secret_list { pub total_num_secrets:u16, pub next_secret_idx:u16, pub secrets:[uv_secret_item;1] }
#[repr(C)] pub struct uv_secret_item { pub id:[u8;32], pub hdr:uv_secret_list_item_hdr }
#[repr(C)] pub struct folio { pub flags:folio_flags, pub mapping:*mut address_space, pub private:usize }
#[repr(C)] pub struct folio_flags { pub f:u64 }
#[repr(C)] pub struct page; #[repr(C)] pub struct pte_t(pub usize); #[repr(C)] pub struct mm_struct; #[repr(C)] pub struct address_space { pub host:*mut inode } #[repr(C)] pub struct inode;
#[repr(C)] pub struct vm_struct { pub addr:*mut c_void, pub flags:u32 } #[repr(C)] pub struct kobject; #[repr(C)] pub struct kset { pub kobj:kobject } #[repr(C)] pub struct attribute; #[repr(C)] pub struct kobj_attribute; #[repr(C)] pub struct attribute_group;

unsafe fn uv_init(stor_base: usize, stor_len: usize) -> c_int { let mut uvcb=uv_cb_init{header:uv_cb_header{cmd:UVC_CMD_INIT_UV,len:core::mem::size_of::<uv_cb_init>() as u16,rc:0,rrc:0},stor_origin:stor_base,stor_len}; if uv_call(0,&mut uvcb as *mut _ as u64)!=0 { return -1; } 0 }
pub unsafe fn setup_uv() { if !is_prot_virt_host(){return;} let p=memblock_alloc_try_nid(uv_info.uv_base_stor_len, SZ_1M,SZ_2G,MEMBLOCK_ALLOC_ACCESSIBLE,NUMA_NO_NODE); if p.is_null(){prot_virt_host=0;return;} if uv_init(p as usize,uv_info.uv_base_stor_len)!=0 {memblock_free(p,uv_info.uv_base_stor_len);prot_virt_host=0;} }
pub unsafe fn uv_pin_shared(paddr:usize)->c_int { let mut x=uv_cb_cfs{header:uv_cb_header{cmd:UVC_CMD_PIN_PAGE_SHARED,len:core::mem::size_of::<uv_cb_cfs>() as u16,rc:0,rrc:0},paddr}; if uv_call(0,&mut x as *mut _ as u64)!=0{-22}else{0} }
unsafe fn uv_destroy(paddr:usize)->c_int { let mut x=uv_cb_cfs{header:uv_cb_header{cmd:UVC_CMD_DESTR_SEC_STOR,len:core::mem::size_of::<uv_cb_cfs>() as u16,rc:0,rrc:0},paddr}; if uv_call(0,&mut x as *mut _ as u64)!=0 && !(x.header.rc==0x107&&x.header.rrc==0xd){-22}else{0} }
pub unsafe fn uv_destroy_folio(f:*mut folio)->c_int { folio_get(f); let mut rc=0; for i in 0..(1usize<<folio_order(f)){rc=uv_destroy(folio_to_phys(f)+i*4096);if rc!=0{break;}} if rc==0{clear_bit(1,&mut (*f).flags.f);} folio_put(f);rc }
pub unsafe fn uv_destroy_pte(p:pte_t)->c_int { uv_destroy_folio(pfn_folio(pte_pfn(p))) }
pub unsafe fn uv_convert_from_secure(paddr:usize)->c_int { let mut x=uv_cb_cfs{header:uv_cb_header{cmd:UVC_CMD_CONV_FROM_SEC_STOR,len:core::mem::size_of::<uv_cb_cfs>() as u16,rc:0,rrc:0},paddr}; if uv_call(0,&mut x as *mut _ as u64)!=0{-22}else{0} }
pub unsafe fn uv_convert_from_secure_folio(f:*mut folio)->c_int { folio_get(f); let mut rc=0; for i in 0..(1usize<<folio_order(f)){rc=uv_convert_from_secure(folio_to_phys(f)+i*4096);if rc!=0{break;}} if rc==0{clear_bit(1,&mut (*f).flags.f);} folio_put(f);rc }
pub unsafe fn uv_convert_from_secure_pte(p:pte_t)->c_int { uv_convert_from_secure_folio(pfn_folio(pte_pfn(p))) }

unsafe fn expected_folio_refs(f:*mut folio)->i32 { let mut r=folio_mapcount(f); if folio_test_swapcache(f){r+=1;} else if !folio_mapping(f).is_null(){r+=1;if (*f).private!=0{r+=1;}} r }
pub unsafe fn __make_folio_secure(f:*mut folio,u:*mut uv_cb_header)->c_int { if folio_test_writeback(f){return -16;} let e=expected_folio_refs(f)+1;if !folio_ref_freeze(f,e){return -16;}set_bit(1,&mut (*f).flags.f);let cc=__uv_call(0,u as u64);folio_ref_unfreeze(f,e);if cc==UVC_CC_OK{0}else if cc==UVC_CC_BUSY||cc==UVC_CC_PARTIAL{-11}else if (*u).rc==0x10a{-6}else{-22} }
pub unsafe fn arch_make_folio_accessible(f:*mut folio)->c_int { if !test_bit(1,&(*f).flags.f){return 0;} if uv_pin_shared(folio_to_phys(f))==0||uv_convert_from_secure(folio_to_phys(f))==0{clear_bit(1,&mut (*f).flags.f);0}else{-5} }

unsafe extern "C" fn uv_free_range_cb(ptep:*mut pte_t,_addr:usize,_data:*mut c_void)->c_int { let p=ptep_get(ptep);if pte_present(p){__free_page(pte_page(p));}0 }
pub unsafe fn uv_free_stor_var(v:*mut c_void) { if v.is_null(){return;}let a=find_vm_area(v);if a.is_null(){return;}let size=get_vm_area_size(a);apply_to_existing_page_range(&mut init_mm,(*a).addr as usize,size,uv_free_range_cb,core::ptr::null_mut());free_vm_area(a); }
unsafe extern "C" fn uv_alloc_range_cb(ptep:*mut pte_t,_addr:usize,_data:*mut c_void)->c_int { let p=alloc_page(0);if p.is_null(){return -12;}set_pte(ptep,pte_t(page_to_phys(p)));0 }
pub unsafe fn uv_alloc_stor_var(mut size:usize)->*mut c_void { size=(size+4095)&!4095;let a=get_vm_area(size,0);if a.is_null(){return core::ptr::null_mut();}if apply_to_page_range(&mut init_mm,(*a).addr as usize,size,uv_alloc_range_cb,core::ptr::null_mut())!=0{uv_free_stor_var((*a).addr);return core::ptr::null_mut();}(*a).addr }

pub unsafe fn s390_wiggle_split_folio(_mm:*mut mm_struct, f:*mut folio)->c_int { folio_wait_writeback(f);lru_add_drain_all();if !folio_test_large(f){return 0;}for _ in 0..2{folio_lock(f);let rc=split_folio(f);folio_unlock(f);if rc!=-16{return rc;}if !folio_test_dirty(f)||folio_test_anon(f)||folio_mapping(f).is_null(){break;}}-11 }

unsafe fn find_secret_in_page(id:*const u8,list:*const uv_secret_list,out:*mut uv_secret_list_item_hdr)->c_int { for i in 0..(*list).total_num_secrets as usize { let s=&(*list).secrets[i];if memcmp(id,s.id.as_ptr(),32)==0{*out=s.hdr;return 0;}} -2 }
pub unsafe fn uv_find_secret(id:*const u8,list:*mut uv_secret_list,out:*mut uv_secret_list_item_hdr)->c_int { let mut start=0;loop{let mut rc=0;uv_list_secrets(list,start,&mut rc,core::ptr::null_mut());if rc!=UVC_RC_EXECUTED&&rc!=UVC_RC_MORE_DATA{return if rc==UVC_RC_INV_CMD{-19}else{-5}};if find_secret_in_page(id,list,out)==0{return 0;}let next=(*list).next_secret_idx;start=next;if !(rc==UVC_RC_MORE_DATA&&start<next){return -2;}} }
pub unsafe fn uv_retrieve_secret(idx:u16,buf:*mut u8,size:usize)->c_int { let mut x=uv_cb_retr_secr{header:uv_cb_header{cmd:UVC_CMD_RETR_SECRET,len:core::mem::size_of::<uv_cb_retr_secr>() as u16,rc:0,rrc:0},secret_idx:idx,buf_addr:buf as u64,buf_size:size};uv_call(0,&mut x as *mut _ as u64);match x.header.rc{UVC_RC_EXECUTED=>0,UVC_RC_INV_CMD=>-19,UVC_RC_RETR_SECR_STORE_EMPTY|UVC_RC_RETR_SECR_INV_SECRET|UVC_RC_RETR_SECR_INV_IDX=>-2,UVC_RC_RETR_SECR_BUF_SMALL=>-22,_=>-5} }

// Constants and build-time kernel macros are supplied by the surrounding kernel translation.
extern "C" { static SZ_1M:usize; static SZ_2G:usize; static MEMBLOCK_ALLOC_ACCESSIBLE:u32; static NUMA_NO_NODE:i32; static UVC_CMD_INIT_UV:u16; static UVC_CMD_PIN_PAGE_SHARED:u16; static UVC_CMD_DESTR_SEC_STOR:u16; static UVC_CMD_CONV_FROM_SEC_STOR:u16; static UVC_CMD_RETR_SECRET:u16; static UVC_CC_OK:u16; static UVC_CC_BUSY:u16; static UVC_CC_PARTIAL:u16; static UVC_RC_EXECUTED:u16; static UVC_RC_MORE_DATA:u16; static UVC_RC_INV_CMD:u16; static UVC_RC_RETR_SECR_STORE_EMPTY:u16; static UVC_RC_RETR_SECR_INV_SECRET:u16; static UVC_RC_RETR_SECR_INV_IDX:u16; static UVC_RC_RETR_SECR_BUF_SMALL:u16; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
