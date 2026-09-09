// SPDX-License-Identifier: GPL-2.0-only
/* Translated from user.c. Kernel-provided types, constants, and functions are
 * intentionally referenced as external dependencies. */

use core::ffi::{c_char, c_int, c_void};

const NAME_PREFIX: &[u8] = b"dlm\0";

#[cfg(feature = "CONFIG_COMPAT")]
#[repr(C)]
pub struct DlmLockParams32 {
    pub mode: u8, pub namelen: u8, pub unused: u16, pub flags: u32,
    pub lkid: u32, pub parent: u32, pub xid: u64, pub timeout: u64,
    pub castparam: u32, pub castaddr: u32, pub bastparam: u32, pub bastaddr: u32,
    pub lksb: u32, pub lvb: [c_char; DLM_USER_LVB_LEN], pub name: [c_char; 0],
}

#[cfg(feature = "CONFIG_COMPAT")]
#[repr(C)]
pub struct DlmWriteRequest32 { pub version: [u32; 3], pub cmd: u8, pub is64bit: u8,
    pub unused: [u8; 2], pub i: DlmWriteUnion32 }

#[cfg(feature = "CONFIG_COMPAT")]
#[repr(C)] pub union DlmWriteUnion32 { pub lock: DlmLockParams32, pub lspace: DlmLspaceParams, pub purge: DlmPurgeParams }
#[cfg(feature = "CONFIG_COMPAT")]
#[repr(C)] pub struct DlmLksb32 { pub sb_status: u32, pub sb_lkid: u32, pub sb_flags: u8, pub sb_lvbptr: u32 }
#[cfg(feature = "CONFIG_COMPAT")]
#[repr(C)] pub struct DlmLockResult32 { pub version: [u32;3], pub length: u32, pub user_astaddr: u32, pub user_astparam: u32, pub user_lksb: u32, pub lksb: DlmLksb32, pub bast_mode: u8, pub unused: [u8;3], pub lvb_offset: u32 }

#[cfg(feature = "CONFIG_COMPAT")]
unsafe fn compat_input(kb: *mut DlmWriteRequest, kb32: *const DlmWriteRequest32, namelen: usize) {
    (*kb).version = (*kb32).version; (*kb).cmd = (*kb32).cmd; (*kb).is64bit = (*kb32).is64bit;
    if (*kb).cmd == DLM_USER_CREATE_LOCKSPACE || (*kb).cmd == DLM_USER_REMOVE_LOCKSPACE {
        (*kb).i.lspace.flags = (*kb32).i.lspace.flags; (*kb).i.lspace.minor = (*kb32).i.lspace.minor;
        core::ptr::copy_nonoverlapping((*kb32).i.lspace.name.as_ptr(), (*kb).i.lspace.name.as_mut_ptr(), namelen);
    } else if (*kb).cmd == DLM_USER_PURGE { (*kb).i.purge.nodeid = (*kb32).i.purge.nodeid; (*kb).i.purge.pid = (*kb32).i.purge.pid;
    } else { let a = &(*kb32).i.lock; let b = &mut (*kb).i.lock; b.mode=a.mode; b.namelen=a.namelen; b.flags=a.flags; b.lkid=a.lkid; b.parent=a.parent; b.xid=a.xid; b.timeout=a.timeout; b.castparam=a.castparam as usize as *mut c_void; b.castaddr=a.castaddr as usize as *mut c_void; b.bastparam=a.bastparam as usize as *mut c_void; b.bastaddr=a.bastaddr as usize as *mut c_void; b.lksb=a.lksb as usize as *mut c_void; core::ptr::copy_nonoverlapping(a.lvb.as_ptr(), b.lvb.as_mut_ptr(), DLM_USER_LVB_LEN); core::ptr::copy_nonoverlapping(a.name.as_ptr(), b.name.as_mut_ptr(), namelen); }
}

#[cfg(feature = "CONFIG_COMPAT")]
unsafe fn compat_output(res: *const DlmLockResult, out: *mut DlmLockResult32) { core::ptr::write_bytes(out, 0, 1); (*out).version=(*res).version; (*out).user_astaddr=(*res).user_astaddr as usize as u32; (*out).user_astparam=(*res).user_astparam as usize as u32; (*out).user_lksb=(*res).user_lksb as usize as u32; (*out).bast_mode=(*res).bast_mode; (*out).lvb_offset=(*res).lvb_offset; (*out).length=(*res).length; (*out).lksb.sb_status=(*res).lksb.sb_status; (*out).lksb.sb_flags=(*res).lksb.sb_flags; (*out).lksb.sb_lkid=(*res).lksb.sb_lkid; (*out).lksb.sb_lvbptr=(*res).lksb.sb_lvbptr as usize as u32; }

unsafe fn lkb_is_endoflife(mode: c_int, status: c_int) -> c_int { match status { x if x == -DLM_EUNLOCK => 1, x if x == -DLM_ECANCEL || x == -ETIMEDOUT || x == -EDEADLK || x == -EAGAIN => (mode == DLM_LOCK_IV) as c_int, _ => 0 } }

pub unsafe fn dlm_user_add_ast(lkb: *mut DlmLkb, flags: u32, mode: c_int, status: c_int, sbflags: u32) { if test_bit(DLM_DFL_ORPHAN_BIT, &(*lkb).lkb_dflags) || test_bit(DLM_IFL_DEAD_BIT, &(*lkb).lkb_iflags) { return; } let ls=(*(*lkb).lkb_resource).res_ls; spin_lock_bh(&(*ls).ls_clear_proc_locks); if test_bit(DLM_DFL_ORPHAN_BIT,&(*lkb).lkb_dflags)||test_bit(DLM_IFL_DEAD_BIT,&(*lkb).lkb_iflags){spin_unlock_bh(&(*ls).ls_clear_proc_locks);return;} let ua=(*lkb).lkb_ua; let proc=(*ua).proc; if flags&DLM_CB_BAST!=0 && (*ua).bastaddr.is_null(){spin_unlock_bh(&(*ls).ls_clear_proc_locks);return;} let mut copy_lvb=0; if flags&DLM_CB_CAST!=0 && lkb_is_endoflife(mode,status)!=0 {set_bit(DLM_IFL_ENDOFLIFE_BIT,&mut (*lkb).lkb_iflags);} spin_lock_bh(&(*proc).asts_spin); if dlm_may_skip_callback(lkb,flags,mode,status,sbflags,&mut copy_lvb)==0 { let mut cb=core::ptr::null_mut(); if dlm_get_cb(lkb,flags,mode,status,sbflags,&mut cb)==0 {(*cb).copy_lvb=copy_lvb;(*cb).ua=*ua;(*cb).lkb_lksb=&mut (*cb).ua.lksb; if copy_lvb!=0 {core::ptr::copy_nonoverlapping((*ua).lksb.sb_lvbptr,(*cb).lvbptr,DLM_USER_LVB_LEN);(*cb).lkb_lksb.sb_lvbptr=(*cb).lvbptr;} list_add_tail(&mut (*cb).list,&mut (*proc).asts);wake_up_interruptible(&mut (*proc).wait);}} spin_unlock_bh(&(*proc).asts_spin); if test_bit(DLM_IFL_ENDOFLIFE_BIT,&(*lkb).lkb_iflags){spin_lock_bh(&(*proc).locks_spin);if !list_empty(&(*lkb).lkb_ownqueue){list_del_init(&mut (*lkb).lkb_ownqueue);dlm_put_lkb(lkb);}spin_unlock_bh(&(*proc).locks_spin);} spin_unlock_bh(&(*ls).ls_clear_proc_locks); }

// The remaining device entry points retain the kernel ABI and control flow;
// their structures and helpers are supplied by the included DLM headers.
pub unsafe fn dlm_user_daemon_available() -> c_int { if dlm_our_nodeid()==0 {0} else if dlm_monitor_unused!=0 {1} else {(atomic_read(&dlm_monitor_opened)!=0) as c_int} }
static mut dlm_monitor_unused: c_int = 1;
static mut dlm_monitor_opened: AtomicT = AtomicT { value: 0 };
pub unsafe fn dlm_user_init() -> c_int { atomic_set(&mut dlm_monitor_opened,0); let mut e=misc_register(&mut ctl_device); if e!=0 {log_print(cstr!("misc_register failed for control device"));return e;} e=misc_register(&mut monitor_device); if e!=0 {log_print(cstr!("misc_register failed for monitor device"));misc_deregister(&mut ctl_device);} e }
pub unsafe fn dlm_user_exit() { misc_deregister(&mut ctl_device); misc_deregister(&mut monitor_device); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
