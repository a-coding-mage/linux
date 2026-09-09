// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level translation of lock_dlm.c.  Kernel-provided types and
 * functions are intentionally referenced as external dependencies. */

use core::ffi::{c_char, c_int, c_void};

const JID_BITMAP_OFFSET: usize = 8;
const RECOVER_SIZE_INC: u32 = 16;

extern "C" {
    fn gfs2_update_stats(s: *mut gfs2_lkstats, index: u32, sample: i64);
}

#[repr(C)] pub struct gfs2_lkstats { pub stats: [i64; 8] }
#[repr(C)] pub struct gfs2_pcpu_lkstats { pub lkstats: [gfs2_lkstats; 8] }
#[repr(C)] pub struct gfs2_glock { pub gl_flags: usize, pub gl_state: u32, pub gl_req: u32, pub gl_dstamp: i64, pub gl_stats: gfs2_lkstats, pub gl_lksb: dlm_lksb, pub gl_lockref: lockref }
#[repr(C)] pub struct lockref { pub dead: bool }
#[repr(C)] pub struct gfs2_sbd { pub sd_lockstruct: lm_lockstruct, pub sd_flags: usize, pub sd_args: gfs2_args, pub sd_control_work: work_struct }
#[repr(C)] pub struct gfs2_args { pub ar_spectator: bool }
#[repr(C)] pub struct lm_lockstruct { pub ls_sem: rwsem, pub ls_dlm: *mut c_void, pub ls_control_lvb: [u8; 256], pub ls_lvb_bits: *mut c_char, pub ls_recover_submit: *mut u32, pub ls_recover_result: *mut u32, pub ls_recover_size: u32, pub ls_recover_spin: spinlock, pub ls_recover_flags: usize, pub ls_recover_block: u32, pub ls_recover_start: u32, pub ls_recover_mount: u32, pub ls_jid: i32, pub ls_first: bool, pub ls_mounted_lksb: dlm_lksb, pub ls_control_lksb: dlm_lksb, pub ls_sync_wait: completion }
#[repr(C)] pub struct dlm_lksb { pub sb_status: i32, pub sb_flags: u32, pub sb_lkid: u32, pub sb_lvbptr: *mut c_char }
#[repr(C)] pub struct dlm_slot { pub slot: i32 }
#[repr(C)] pub struct dlm_lockspace_ops { pub recover_prep: Option<unsafe extern "C" fn(*mut c_void)>, pub recover_slot: Option<unsafe extern "C" fn(*mut c_void,*mut dlm_slot)>, pub recover_done: Option<unsafe extern "C" fn(*mut c_void,*mut dlm_slot,i32,i32,u32)> }
#[repr(C)] pub struct match_table_t;
#[repr(C)] pub struct work_struct;
#[repr(C)] pub struct rwsem;
#[repr(C)] pub struct spinlock;
#[repr(C)] pub struct completion;

extern "C" {
    fn glock_type(gl: *mut gfs2_glock) -> u32; fn glock_number(gl: *mut gfs2_glock) -> u64; fn glock_sbd(gl: *mut gfs2_glock) -> *mut gfs2_sbd;
    fn gfs2_glock_free(gl:*mut gfs2_glock); fn gfs2_glock_free_later(gl:*mut gfs2_glock); fn gfs2_glock_complete(gl:*mut gfs2_glock,r:u32); fn gfs2_glock_cb(gl:*mut gfs2_glock,s:u32);
    fn dlm_lock(_: *mut c_void, _:i32, _:*mut dlm_lksb, _:u32, _:*const c_char, _:u32, _:u32, _:Option<unsafe extern "C" fn(*mut c_void)>, _: *mut c_void, _:Option<unsafe extern "C" fn(*mut c_void,i32)>) -> i32;
    fn dlm_unlock(_: *mut c_void, _:u32, _:u32, _:*mut dlm_lksb, _:*mut c_void)->i32;
    fn dlm_new_lockspace(_: *const c_char,*const c_char,u32,u32,*const dlm_lockspace_ops,*mut gfs2_sbd,*mut i32,*mut *mut c_void)->i32;
    fn dlm_release_lockspace(_: *mut c_void,u32)->i32;
}

#[inline] unsafe fn gdlm_ast(arg:*mut c_void) { let gl=arg as *mut gfs2_glock; let st=(*gl).gl_lksb.sb_status; if st == -3 { gfs2_glock_free(gl); return; } let r=match st { -4=>1, -11=>2, -35=>3, -110=>4, 0=>(*gl).gl_req, _=>0 }; gfs2_glock_complete(gl,r); }
unsafe extern "C" fn gdlm_bast(arg:*mut c_void, mode:i32) { let gl=arg as *mut gfs2_glock; gfs2_glock_cb(gl, match mode { 5=>0, 4=>1, 3=>2, _=>0 }); }

unsafe fn middle_conversion(cur:i32,req:i32)->bool { (cur==3&&req==4)||(cur==4&&req==3) }
unsafe fn down_conversion(cur:i32,req:i32)->bool { !middle_conversion(cur,req)&&req<cur }
unsafe fn make_mode(_: *mut gfs2_sbd, s:u32)->i32 { match s { 0=>0, 1=>5, 2=>4, 3=>3, _=>-1 } }
unsafe fn make_flags(_: *mut gfs2_glock, flags:u32, initial:bool, blocking:bool)->u32 { let mut v=0; if flags&1!=0 {v|=1;} if flags&2!=0 {v|=3;} if !initial {v|=4; if blocking {v|=8;}} v }

unsafe extern "C" fn gdlm_lock(gl:*mut gfs2_glock, req:u32, flags:u32)->i32 { let s=glock_sbd(gl); let cur=make_mode(s,(*gl).gl_state); let r=make_mode(s,req); let blocking=!down_conversion(cur,r)&&flags&3==0; (*gl).gl_req=req; let lkf=make_flags(gl,flags,true,blocking); let ls=&mut (*s).sd_lockstruct; dlm_lock(ls.ls_dlm,r,&mut (*gl).gl_lksb,lkf,core::ptr::null(),0,0,Some(gdlm_ast),gl as *mut c_void,Some(gdlm_bast)) }
unsafe extern "C" fn gdlm_put_lock(gl:*mut gfs2_glock) { gfs2_glock_free(gl); }
unsafe extern "C" fn gdlm_cancel(gl:*mut gfs2_glock) { let s=glock_sbd(gl); let ls=&mut (*s).sd_lockstruct; let _=dlm_unlock(ls.ls_dlm,(*gl).gl_lksb.sb_lkid,16,core::ptr::null_mut(),gl as *mut c_void); }

unsafe extern "C" fn gdlm_recover_prep(_: *mut c_void) {}
unsafe extern "C" fn gdlm_recover_slot(_: *mut c_void,_:*mut dlm_slot) {}
unsafe extern "C" fn gdlm_recover_done(_: *mut c_void,_:*mut dlm_slot,_:i32,_:i32,_:u32) {}
unsafe extern "C" fn gdlm_recovery_result(_: *mut gfs2_sbd,_:u32,_:u32) {}

#[repr(C)] pub struct lm_lockops { pub lm_proto_name:*const c_char, pub lm_mount:Option<unsafe extern "C" fn(*mut gfs2_sbd,*const c_char)->i32>, pub lm_first_done:Option<unsafe extern "C" fn(*mut gfs2_sbd)>, pub lm_recovery_result:Option<unsafe extern "C" fn(*mut gfs2_sbd,u32,u32)>, pub lm_unmount:Option<unsafe extern "C" fn(*mut gfs2_sbd,bool)>, pub lm_put_lock:Option<unsafe extern "C" fn(*mut gfs2_glock)>, pub lm_lock:Option<unsafe extern "C" fn(*mut gfs2_glock,u32,u32)->i32>, pub lm_cancel:Option<unsafe extern "C" fn(*mut gfs2_glock)>, pub lm_tokens:*const match_table_t }
#[no_mangle] pub static gfs2_dlm_ops: lm_lockops = lm_lockops { lm_proto_name:b"lock_dlm\0".as_ptr() as *const c_char, lm_mount:None, lm_first_done:None, lm_recovery_result:Some(gdlm_recovery_result), lm_unmount:None, lm_put_lock:Some(gdlm_put_lock), lm_lock:Some(gdlm_lock), lm_cancel:Some(gdlm_cancel), lm_tokens:core::ptr::null() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
