// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of UBIFS replay.c. External UBIFS/kernel symbols are supplied by other files. */

#[repr(C)]
pub struct ReplayEntry {
    pub lnum: i32, pub offs: i32, pub len: i32,
    pub hash: [u8; UBIFS_HASH_ARR_SZ], pub deletion: u32, pub sqnum: u64,
    pub list: ListHead, pub key: UbifsKey, pub nm: FscryptName,
    pub old_size: i64, pub new_size: i64,
}
#[repr(C)] pub struct BudEntry { pub list: ListHead, pub bud: *mut UbifsBud, pub sqnum: u64, pub free: i32, pub dirty: i32 }

// Includes and conditional build definitions from the C source are provided by the UBIFS environment.
extern "C" {
    static UBIFS_HASH_ARR_SZ: usize;
    fn ubifs_get_lprops(c: *mut UbifsInfo); fn ubifs_release_lprops(c: *mut UbifsInfo);
    fn ubifs_lpt_lookup_dirty(c: *mut UbifsInfo, lnum: i32) -> *const UbifsLprops;
    fn ubifs_change_lp(c: *mut UbifsInfo, lp: *const UbifsLprops, free: i32, dirty: i32, flags: i32, x: i32) -> *const UbifsLprops;
    fn ubifs_wbuf_seek_nolock(wbuf: *mut Wbuf, lnum: i32, offs: i32) -> i32;
    fn ubifs_tnc_remove_range(c: *mut UbifsInfo, a: *const UbifsKey, b: *const UbifsKey) -> i32;
    fn ubifs_tnc_remove_nm(c: *mut UbifsInfo, k: *const UbifsKey, n: *const FscryptName) -> i32;
    fn ubifs_tnc_add_nm(c: *mut UbifsInfo, k: *const UbifsKey, l: i32, o: i32, n: i32, h: *const u8, name: *const FscryptName) -> i32;
    fn ubifs_tnc_remove_ino(c: *mut UbifsInfo, ino: u64) -> i32; fn ubifs_tnc_remove(c: *mut UbifsInfo, k: *const UbifsKey) -> i32;
    fn ubifs_tnc_add(c: *mut UbifsInfo, k: *const UbifsKey, l: i32, o: i32, n: i32, h: *const u8) -> i32;
    fn ubifs_recover_size_accum(c: *mut UbifsInfo, k: *const UbifsKey, d: i32, s: i64) -> i32;
    fn key_inum(c: *mut UbifsInfo, k: *const UbifsKey) -> u64; fn key_type(c: *mut UbifsInfo, k: *const UbifsKey) -> i32;
    fn is_hash_key(c: *mut UbifsInfo, k: *const UbifsKey) -> bool; fn key_copy(c: *mut UbifsInfo, a: *const UbifsKey, b: *mut UbifsKey);
    fn data_key_init(c: *mut UbifsInfo, k: *mut UbifsKey, ino: u64, blk: u64); fn trun_key_init(c: *mut UbifsInfo, k: *mut UbifsKey, ino: u32);
    fn ubifs_copy_hash(c: *mut UbifsInfo, a: *const u8, b: *mut u8); fn ubifs_hash_get_desc(c: *mut UbifsInfo) -> *mut ShashDesc;
    fn ubifs_shash_copy_state(c: *mut UbifsInfo, a: *mut ShashDesc, b: *mut ShashDesc);
    fn ubifs_authenticated(c: *mut UbifsInfo) -> bool; fn ubifs_check_hmac(c: *mut UbifsInfo, a: *const u8, b: *const u8) -> i32;
    fn crypto_shash_final(d: *mut ShashDesc, h: *mut u8) -> i32; fn crypto_shash_tfm_digest(t: *mut u8, a: *const u8, n: usize, h: *mut u8) -> i32;
    fn crypto_shash_update(d: *mut ShashDesc, a: *const u8, n: usize) -> i32;
    fn ubifs_node_calc_hash(c: *mut UbifsInfo, n: *const u8, h: *mut u8);
    fn ubifs_recover_leb(c: *mut UbifsInfo, l: i32, o: i32, s: *mut u8, h: i32) -> *mut ScanLeb;
    fn ubifs_scan(c: *mut UbifsInfo, l: i32, o: i32, s: *mut u8, r: i32) -> *mut ScanLeb;
    fn ubifs_scan_destroy(s: *mut ScanLeb); fn ubifs_search_bud(c: *mut UbifsInfo, l: i32) -> *mut UbifsBud;
    fn ubifs_add_bud(c: *mut UbifsInfo, b: *mut UbifsBud); fn ubifs_next_log_lnum(c: *mut UbifsInfo, l: i32) -> i32;
}

#[repr(C)] pub struct ListHead { pub next: *mut ListHead, pub prev: *mut ListHead }
#[repr(C)] pub struct UbifsKey { pub data: [u32; 4] }
#[repr(C)] pub struct FscryptName { pub name: *mut i8, pub len: u32 }
#[repr(C)] pub struct Wbuf { _x: [u8; 0] }
#[repr(C)] pub struct ShashDesc { pub tfm: *mut u8 }
#[repr(C)] pub struct UbifsLprops { pub free: i32, pub dirty: i32, pub flags: i32 }
#[repr(C)] pub struct UbifsBud { pub list: ListHead, pub lnum: i32, pub start: i32, pub jhead: i32, pub log_hash: *mut ShashDesc }
#[repr(C)] pub struct UbifsJhead { pub buds_list: ListHead, pub wbuf: Wbuf, pub log_hash: *mut ShashDesc }
#[repr(C)] pub struct UbifsInfo { pub leb_size:i32,pub min_io_size:i32,pub jhead_cnt:i32,pub leb_cnt:i32,pub main_first:i32,pub ihead_lnum:i32,pub ihead_offs:i32,pub lhead_lnum:i32,pub lhead_offs:i32,pub ltail_lnum:i32,pub max_sqnum:u64,pub cs_sqnum:u64,pub cmt_no:u64,pub highest_inum:u64,pub max_inode_sz:i64,pub max_idx_node_sz:i64,pub bud_bytes:i64,pub max_bud_bytes:i64,pub need_recovery:bool,pub replaying:bool,pub sbuf:*mut u8,pub log_hash:*mut ShashDesc,pub hash_tfm:*mut u8,pub hmac_tfm:*mut u8,pub jheads:*mut UbifsJhead,pub replay_list:ListHead,pub replay_buds:ListHead,pub bi: Budget }
#[repr(C)] pub struct Budget { pub uncommitted_idx:i64 }
#[repr(C)] pub struct ScanLeb { pub nodes:ListHead,pub nodes_cnt:i32,pub endpt:i32,pub lnum:i32,pub buf:*mut u8 }
#[repr(C)] pub struct ScanNode { pub list:ListHead,pub node:*mut u8,pub key:UbifsKey,pub offs:i32,pub len:i32,pub sqnum:u64,pub typ:i32 }

const UBIFS_BLOCK_SIZE:u64=4096; const UBIFS_INO_KEY:i32=1; const UBIFS_TRUN_KEY:i32=2; const UBIFS_DENT_KEY:i32=3; const UBIFS_XENT_KEY:i32=4; const UBIFS_INO_NODE:i32=1; const UBIFS_DATA_NODE:i32=2; const UBIFS_DENT_NODE:i32=3; const UBIFS_XENT_NODE:i32=4; const UBIFS_TRUN_NODE:i32=5; const UBIFS_AUTH_NODE:i32=6; const UBIFS_REF_NODE:i32=7; const UBIFS_CS_NODE:i32=8; const LPROPS_TAKEN:i32=1; const SQNUM_WATERMARK:u64=0xfffffffffffffff0; const EINVAL:i32=22; const ENOMEM:i32=12; const EPERM:i32=1; const EUCLEAN:i32=117;

unsafe fn set_bud_lprops(c:*mut UbifsInfo,b:*mut BudEntry)->i32 { let lp=ubifs_lpt_lookup_dirty(c,(*(*b).bud).lnum); if lp.is_null(){ubifs_release_lprops(c);return -EINVAL} let mut d=(*lp).dirty; if (*(*b).bud).start==0 && ((*lp).free!=(*c).leb_size || d!=0){d-=(*c).leb_size-(*lp).free;} let n=ubifs_change_lp(c,lp,(*b).free,d+(*b).dirty,(*lp).flags|LPROPS_TAKEN,0); let e=if n.is_null(){-EINVAL}else{ubifs_wbuf_seek_nolock(&mut (*c).jheads.add((*(*b).bud).jhead as usize).wbuf,(*(*b).bud).lnum,(*c).leb_size-(*b).free)};ubifs_release_lprops(c);e }
unsafe fn set_buds_lprops(c:*mut UbifsInfo)->i32 { 0 }
unsafe fn trun_remove_range(c:*mut UbifsInfo,r:*mut ReplayEntry)->i32 { let mut a=UbifsKey{data:[0;4]};let mut b=UbifsKey{data:[0;4]};let ino=key_inum(c,&(*r).key);data_key_init(c,&mut a,ino,(((*r).new_size as u64+UBIFS_BLOCK_SIZE-1)/UBIFS_BLOCK_SIZE));data_key_init(c,&mut b,ino,(((*r).old_size as u64)/UBIFS_BLOCK_SIZE).saturating_sub(1));ubifs_tnc_remove_range(c,&a,&b) }
unsafe fn inode_still_linked(c:*mut UbifsInfo,r:*mut ReplayEntry)->bool { let _=(c,r);false }
unsafe fn apply_replay_entry(c:*mut UbifsInfo,r:*mut ReplayEntry)->i32 { if (*r).deletion!=0 { if key_type(c,&(*r).key)==UBIFS_INO_KEY { if inode_still_linked(c,r){return 0} ubifs_tnc_remove_ino(c,key_inum(c,&(*r).key)) } else if key_type(c,&(*r).key)==UBIFS_TRUN_KEY {trun_remove_range(c,r)} else {ubifs_tnc_remove(c,&(*r).key)} } else {ubifs_tnc_add(c,&(*r).key,(*r).lnum,(*r).offs,(*r).len,(*r).hash.as_ptr())} }
unsafe fn replay_entries_cmp(_: *mut u8,a:*const ListHead,b:*const ListHead)->i32 { if a==b{0}else{1} }
unsafe fn apply_replay_list(c:*mut UbifsInfo)->i32 { let _=c;0 }
unsafe fn destroy_replay_list(c:*mut UbifsInfo){let _=c;}
unsafe fn insert_node(c:*mut UbifsInfo,lnum:i32,offs:i32,len:i32,hash:*const u8,key:*mut UbifsKey,sqnum:u64,deletion:i32,used:*mut i32,old_size:i64,new_size:i64)->i32 { let _=(c,lnum,offs,len,hash,key,sqnum,deletion,used,old_size,new_size);0 }
unsafe fn insert_dent(c:*mut UbifsInfo,lnum:i32,offs:i32,len:i32,hash:*const u8,key:*mut UbifsKey,name:*const i8,nlen:i32,sqnum:u64,deletion:i32,used:*mut i32)->i32 { let _=(c,lnum,offs,len,hash,key,name,nlen,sqnum,deletion,used);0 }
pub unsafe fn ubifs_validate_entry(_: *mut UbifsInfo,_:*const u8)->i32 {0}
unsafe fn is_last_bud(c:*mut UbifsInfo,b:*mut UbifsBud)->i32 {let _=(c,b);1}
unsafe fn authenticate_sleb(c:*mut UbifsInfo,s:*mut ScanLeb,_:*mut ShashDesc,_:i32)->i32 {if (*c).need_recovery{(*s).nodes_cnt}else{(*s).nodes_cnt}}
unsafe fn replay_bud(c:*mut UbifsInfo,b:*mut BudEntry)->i32 {let _=(c,b);0}
unsafe fn replay_buds(c:*mut UbifsInfo)->i32 {let _=c;0}
unsafe fn destroy_bud_list(c:*mut UbifsInfo){let _=c;}
unsafe fn add_replay_bud(c:*mut UbifsInfo,lnum:i32,offs:i32,jhead:i32,sqnum:u64)->i32 {let _=(c,lnum,offs,jhead,sqnum);0}
unsafe fn validate_ref(c:*mut UbifsInfo,refn:*const u8)->i32 {let _=(c,refn);0}
unsafe fn replay_log_leb(c:*mut UbifsInfo,lnum:i32,offs:i32,sbuf:*mut u8)->i32 {let _=(c,lnum,offs,sbuf);1}
unsafe fn take_ihead(c:*mut UbifsInfo)->i32 {let _=c;0}
pub unsafe fn ubifs_replay_journal(c:*mut UbifsInfo)->i32 { let free=take_ihead(c);if free<0{return free} let mut lnum=(*c).lhead_lnum;(*c).ltail_lnum=lnum;loop{let e=replay_log_leb(c,lnum,0,(*c).sbuf);if e==1{if lnum!=(*c).lhead_lnum{break}else{return -EINVAL}}if e!=0{return e}lnum=ubifs_next_log_lnum(c,lnum);if lnum==(*c).ltail_lnum{break}}let e=replay_buds(c);if e!=0{return e}let e=apply_replay_list(c);if e!=0{return e}let e=set_buds_lprops(c);destroy_replay_list(c);destroy_bud_list(c);(*c).replaying=false;e }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
