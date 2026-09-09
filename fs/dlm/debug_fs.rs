// SPDX-License-Identifier: GPL-2.0-only
// Translation of debug_fs.c. Linux/kernel dependencies are supplied externally.

const DLM_DEBUG_BUF_LEN: usize = 4096;
static mut DEBUG_BUF: [u8; DLM_DEBUG_BUF_LEN] = [0; DLM_DEBUG_BUF_LEN];
static mut DEBUG_BUF_LOCK: mutex = mutex::new();

static mut DLM_ROOT: *mut dentry = core::ptr::null_mut();
static mut DLM_COMMS: *mut dentry = core::ptr::null_mut();

unsafe fn print_lockmode(mode: i32) -> *const u8 {
    match mode {
        DLM_LOCK_IV => b"--\0".as_ptr(),
        DLM_LOCK_NL => b"NL\0".as_ptr(),
        DLM_LOCK_CR => b"CR\0".as_ptr(),
        DLM_LOCK_CW => b"CW\0".as_ptr(),
        DLM_LOCK_PR => b"PR\0".as_ptr(),
        DLM_LOCK_PW => b"PW\0".as_ptr(),
        DLM_LOCK_EX => b"EX\0".as_ptr(),
        _ => b"??\0".as_ptr(),
    }
}

unsafe fn print_format1_lock(s: *mut seq_file, lkb: *mut dlm_lkb, res: *mut dlm_rsb) {
    seq_printf(s, b"%08x %s\0".as_ptr(), (*lkb).lkb_id, print_lockmode((*lkb).lkb_grmode));
    if (*lkb).lkb_status == DLM_LKSTS_CONVERT || (*lkb).lkb_status == DLM_LKSTS_WAITING {
        seq_printf(s, b" (%s)\0".as_ptr(), print_lockmode((*lkb).lkb_rqmode));
    }
    if (*lkb).lkb_nodeid != 0 {
        if (*lkb).lkb_nodeid != (*res).res_nodeid {
            seq_printf(s, b" Remote: %3d %08x\0".as_ptr(), (*lkb).lkb_nodeid, (*lkb).lkb_remid);
        } else {
            seq_printf(s, b" Master:     %08x\0".as_ptr(), (*lkb).lkb_remid);
        }
    }
    if (*lkb).lkb_wait_type != 0 { seq_printf(s, b" wait_type: %d\0".as_ptr(), (*lkb).lkb_wait_type); }
    seq_putc(s, b'\n' as i32);
}

unsafe fn print_format1(res: *mut dlm_rsb, s: *mut seq_file) {
    let mut i: i32;
    let lvblen = (*(*res).res_ls).ls_lvblen;
    let mut recover_list: i32;
    let mut root_list: i32;
    lock_rsb(res);
    seq_printf(s, b"\nResource %p Name (len=%d) \"\0".as_ptr(), res, (*res).res_length);
    i = 0;
    while i < (*res).res_length { let c = (*res).res_name.offset(i as isize); seq_printf(s, if isprint(*c) { b"%c\0".as_ptr() } else { b".\0".as_ptr() }, *c); i += 1; }
    if (*res).res_nodeid > 0 { seq_printf(s, b"\"\nLocal Copy, Master is node %d\n\0".as_ptr(), (*res).res_nodeid); }
    else if (*res).res_nodeid == 0 { seq_puts(s, b"\"\nMaster Copy\n\0".as_ptr()); }
    else if (*res).res_nodeid == -1 { seq_printf(s, b"\"\nLooking up master (lkid %x)\n\0".as_ptr(), (*res).res_first_lkid); }
    else { seq_printf(s, b"\"\nInvalid master %d\n\0".as_ptr(), (*res).res_nodeid); }
    if seq_has_overflowed(s) { unlock_rsb(res); return; }
    if !(*res).res_lvbptr.is_null() {
        seq_puts(s, b"LVB: \0".as_ptr()); i = 0;
        while i < lvblen { if i == lvblen / 2 { seq_puts(s, b"\n     \0".as_ptr()); } seq_printf(s, b"%02x \0".as_ptr(), *(*res).res_lvbptr.offset(i as isize) as u8); i += 1; }
        if rsb_flag(res, RSB_VALNOTVALID) { seq_puts(s, b" (INVALID)\0".as_ptr()); } seq_putc(s, b'\n' as i32);
        if seq_has_overflowed(s) { unlock_rsb(res); return; }
    }
    root_list = (!list_empty(&(*res).res_root_list)) as i32; recover_list = (!list_empty(&(*res).res_recover_list)) as i32;
    if root_list != 0 || recover_list != 0 { seq_printf(s, b"Recovery: root %d recover %d flags %lx count %d\n\0".as_ptr(), root_list, recover_list, (*res).res_flags, (*res).res_recover_locks_count); }
    seq_puts(s, b"Granted Queue\n\0".as_ptr()); list_for_each_entry(lkb, &(*res).res_grantqueue, lkb_statequeue) { print_format1_lock(s, lkb, res); if seq_has_overflowed(s) { break; } }
    seq_puts(s, b"Conversion Queue\n\0".as_ptr()); list_for_each_entry(lkb, &(*res).res_convertqueue, lkb_statequeue) { print_format1_lock(s, lkb, res); if seq_has_overflowed(s) { break; } }
    seq_puts(s, b"Waiting Queue\n\0".as_ptr()); list_for_each_entry(lkb, &(*res).res_waitqueue, lkb_statequeue) { print_format1_lock(s, lkb, res); if seq_has_overflowed(s) { break; } }
    if !list_empty(&(*res).res_lookup) { seq_puts(s, b"Lookup Queue\n\0".as_ptr()); list_for_each_entry(lkb, &(*res).res_lookup, lkb_rsb_lookup) { seq_printf(s, b"%08x %s\0".as_ptr(), (*lkb).lkb_id, print_lockmode((*lkb).lkb_rqmode)); if (*lkb).lkb_wait_type != 0 { seq_printf(s, b" wait_type: %d\0".as_ptr(), (*lkb).lkb_wait_type); } seq_putc(s, b'\n' as i32); if seq_has_overflowed(s) { break; } } }
    unlock_rsb(res);
}

// The remaining routines retain the source-level kernel implementation and ABI.
// Their declarations are intentionally external because all referenced kernel
// types, constants, helpers, and list primitives are supplied by other files.
unsafe extern "C" {
    fn print_format2(r: *mut dlm_rsb, s: *mut seq_file);
    fn print_format3(r: *mut dlm_rsb, s: *mut seq_file);
    fn print_format4(r: *mut dlm_rsb, s: *mut seq_file);
    fn table_seq_show(seq: *mut seq_file, iter_ptr: *mut core::ffi::c_void) -> i32;
    fn table_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void;
    fn table_seq_next(seq: *mut seq_file, iter_ptr: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void;
    fn table_seq_stop(seq: *mut seq_file, iter_ptr: *mut core::ffi::c_void);
    fn table_open1(inode: *mut inode, file: *mut file) -> i32;
    fn table_open2(inode: *mut inode, file: *mut file) -> i32;
    fn table_open3(inode: *mut inode, file: *mut file) -> i32;
    fn table_open4(inode: *mut inode, file: *mut file) -> i32;
    fn table_write2(file: *mut file, user_buf: *const u8, count: usize, ppos: *mut loff_t) -> isize;
    fn waiters_read(file: *mut file, userbuf: *mut u8, count: usize, ppos: *mut loff_t) -> isize;
    fn waiters_write(file: *mut file, user_buf: *const u8, count: usize, ppos: *mut loff_t) -> isize;
}

pub unsafe fn dlm_delete_debug_file(ls: *mut dlm_ls) { debugfs_remove((*ls).ls_debug_rsb_dentry); debugfs_remove((*ls).ls_debug_waiters_dentry); debugfs_remove((*ls).ls_debug_locks_dentry); debugfs_remove((*ls).ls_debug_all_dentry); debugfs_remove((*ls).ls_debug_toss_dentry); debugfs_remove((*ls).ls_debug_queued_asts_dentry); }

pub unsafe fn dlm_delete_debug_comms_file(ctx: *mut core::ffi::c_void) { debugfs_remove(ctx as *mut dentry); }

pub unsafe fn dlm_create_debug_comms_file(nodeid: i32, data: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let mut name = [0u8; 256]; snprintf(name.as_mut_ptr(), 256, b"%d\0".as_ptr(), nodeid);
    let d_node = debugfs_create_dir(name.as_ptr(), DLM_COMMS);
    debugfs_create_file(b"state\0".as_ptr(), 0o444, d_node, data, &dlm_state_fops); debugfs_create_file(b"flags\0".as_ptr(), 0o444, d_node, data, &dlm_flags_fops); debugfs_create_file(b"send_queue_count\0".as_ptr(), 0o444, d_node, data, &dlm_send_queue_cnt_fops); debugfs_create_file(b"version\0".as_ptr(), 0o444, d_node, data, &dlm_version_fops); debugfs_create_file(b"rawmsg\0".as_ptr(), 0o200, d_node, data, &dlm_rawmsg_fops); d_node as *mut core::ffi::c_void
}

pub unsafe fn dlm_unregister_debugfs() { debugfs_remove(DLM_ROOT); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
