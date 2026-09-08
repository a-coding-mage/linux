// SPDX-License-Identifier: GPL-2.0
/*
 * security/tomoyo/audit.c
 *
 * Copyright (C) 2005-2011  NTT DATA CORPORATION
 */

/* C dependencies from common.h and linux/slab.h are supplied externally. */

unsafe fn tomoyo_print_bprm(
    bprm: *mut linux_binprm,
    dump: *mut tomoyo_page_dump,
) -> *mut core::ffi::c_char {
    const TOMOYO_BUFFER_LEN: usize = 4096 * 2;
    let buffer = kzalloc(TOMOYO_BUFFER_LEN, GFP_NOFS);
    let mut cp: *mut core::ffi::c_char;
    let mut last_start: *mut core::ffi::c_char;
    let mut len: i32;
    let mut pos = (*bprm).p;
    let mut offset = pos % PAGE_SIZE;
    let mut argv_count = (*bprm).argc;
    let mut envp_count = (*bprm).envc;
    let mut truncated = false;

    if buffer.is_null() { return core::ptr::null_mut(); }
    len = snprintf(buffer, TOMOYO_BUFFER_LEN - 1, c"argv[]={ ");
    cp = buffer.add(len as usize);
    if argv_count == 0 {
        memmove(cp.cast(), c"} envp[]={ ".as_ptr().cast(), 11);
        cp = cp.add(11);
    }
    last_start = cp;
    while argv_count != 0 || envp_count != 0 {
        if tomoyo_dump_page(bprm, pos, dump) == 0 { break; }
        pos += PAGE_SIZE - offset;
        while offset < PAGE_SIZE {
            let kaddr = (*dump).data;
            let c = *kaddr.add(offset);
            offset += 1;
            if cp == last_start { *cp = b'"' as i8; cp = cp.add(1); }
            if cp >= buffer.add(TOMOYO_BUFFER_LEN - 32) {
                truncated = true;
            } else if c == b'\\' {
                *cp = b'\\' as i8; cp = cp.add(1); *cp = b'\\' as i8; cp = cp.add(1);
            } else if c > b' ' && c < 127 {
                *cp = c as i8; cp = cp.add(1);
            } else if c == 0 {
                *cp = b'"' as i8; cp = cp.add(1); *cp = b' ' as i8; cp = cp.add(1); last_start = cp;
            } else {
                *cp = b'\\' as i8; cp = cp.add(1);
                *cp = ((c >> 6) + b'0') as i8; cp = cp.add(1);
                *cp = (((c >> 3) & 7) + b'0') as i8; cp = cp.add(1);
                *cp = ((c & 7) + b'0') as i8; cp = cp.add(1);
            }
            if c != 0 { continue; }
            if argv_count != 0 {
                argv_count -= 1;
                if argv_count == 0 {
                    if truncated { cp = last_start; memmove(cp.cast(), c"... ".as_ptr().cast(), 4); cp = cp.add(4); }
                    memmove(cp.cast(), c"} envp[]={ ".as_ptr().cast(), 11); cp = cp.add(11); last_start = cp; truncated = false;
                }
            } else if envp_count != 0 {
                envp_count -= 1;
                if envp_count == 0 && truncated { cp = last_start; memmove(cp.cast(), c"... ".as_ptr().cast(), 4); cp = cp.add(4); }
            }
            if argv_count == 0 && envp_count == 0 { break; }
        }
        offset = 0;
    }
    *cp = b'}' as i8; cp = cp.add(1); *cp = 0;
    return buffer;
}

#[inline]
unsafe fn tomoyo_filetype(mode: umode_t) -> *const core::ffi::c_char {
    match mode & S_IFMT {
        S_IFREG | 0 => tomoyo_condition_keyword[TOMOYO_TYPE_IS_FILE as usize],
        S_IFDIR => tomoyo_condition_keyword[TOMOYO_TYPE_IS_DIRECTORY as usize],
        S_IFLNK => tomoyo_condition_keyword[TOMOYO_TYPE_IS_SYMLINK as usize],
        S_IFIFO => tomoyo_condition_keyword[TOMOYO_TYPE_IS_FIFO as usize],
        S_IFSOCK => tomoyo_condition_keyword[TOMOYO_TYPE_IS_SOCKET as usize],
        S_IFBLK => tomoyo_condition_keyword[TOMOYO_TYPE_IS_BLOCK_DEV as usize],
        S_IFCHR => tomoyo_condition_keyword[TOMOYO_TYPE_IS_CHAR_DEV as usize],
        _ => c"unknown".as_ptr().cast(),
    }
}

unsafe fn tomoyo_print_header(r: *mut tomoyo_request_info) -> *mut core::ffi::c_char {
    let mut stamp: tomoyo_time = core::mem::zeroed();
    let gpid = task_pid_nr(current);
    let obj = (*r).obj;
    const TOMOYO_BUFFER_LEN: usize = 4096;
    let buffer = kmalloc(TOMOYO_BUFFER_LEN, GFP_NOFS);
    let mut pos: i32;
    if buffer.is_null() { return core::ptr::null_mut(); }
    tomoyo_convert_time(ktime_get_real_seconds(), &mut stamp);
    pos = snprintf(buffer, TOMOYO_BUFFER_LEN - 1, c"#%04u/%02u/%02u %02u:%02u:%02u# profile=%u mode=%s granted=%s (global-pid=%u) task={ pid=%u ppid=%u uid=%u gid=%u euid=%u egid=%u suid=%u sgid=%u fsuid=%u fsgid=%u }", stamp.year, stamp.month, stamp.day, stamp.hour, stamp.min, stamp.sec, (*r).profile, tomoyo_mode[(*r).mode as usize], str_yes_no((*r).granted), gpid, tomoyo_sys_getpid(), tomoyo_sys_getppid(), from_kuid(&init_user_ns, current_uid()), from_kgid(&init_user_ns, current_gid()), from_kuid(&init_user_ns, current_euid()), from_kgid(&init_user_ns, current_egid()), from_kuid(&init_user_ns, current_suid()), from_kgid(&init_user_ns, current_sgid()), from_kuid(&init_user_ns, current_fsuid()), from_kgid(&init_user_ns, current_fsgid()));
    if obj.is_null() { return buffer; }
    if !(*obj).validate_done { tomoyo_get_attributes(obj); (*obj).validate_done = true; }
    for i in 0..TOMOYO_MAX_PATH_STAT {
        if !(*obj).stat_valid[i as usize] { continue; }
        let stat = &(*obj).stat[i as usize];
        let mut dev = stat.dev; let mode = stat.mode;
        if i & 1 != 0 { pos += snprintf(buffer.add(pos as usize), TOMOYO_BUFFER_LEN - 1 - pos as usize, c" path%u.parent={ uid=%u gid=%u ino=%llu perm=0%o }", (i >> 1) + 1, from_kuid(&init_user_ns, stat.uid), from_kgid(&init_user_ns, stat.gid), stat.ino, stat.mode & S_IALLUGO); continue; }
        pos += snprintf(buffer.add(pos as usize), TOMOYO_BUFFER_LEN - 1 - pos as usize, c" path%u={ uid=%u gid=%u ino=%llu major=%u minor=%u perm=0%o type=%s", (i >> 1) + 1, from_kuid(&init_user_ns, stat.uid), from_kgid(&init_user_ns, stat.gid), stat.ino, MAJOR(dev), MINOR(dev), mode & S_IALLUGO, tomoyo_filetype(mode));
        if S_ISCHR(mode) || S_ISBLK(mode) { dev = stat.rdev; pos += snprintf(buffer.add(pos as usize), TOMOYO_BUFFER_LEN - 1 - pos as usize, c" dev_major=%u dev_minor=%u", MAJOR(dev), MINOR(dev)); }
        pos += snprintf(buffer.add(pos as usize), TOMOYO_BUFFER_LEN - 1 - pos as usize, c" }");
    }
    if pos < TOMOYO_BUFFER_LEN as i32 - 1 { buffer } else { kfree(buffer.cast()); core::ptr::null_mut() }
}

pub unsafe fn tomoyo_init_log(r: *mut tomoyo_request_info, mut len: i32, fmt: *const core::ffi::c_char, args: va_list) -> *mut core::ffi::c_char {
    let mut buf = core::ptr::null_mut(); let mut bprm_info = core::ptr::null_mut(); let mut header = tomoyo_print_header(r); let mut realpath = core::ptr::null_mut(); let mut symlink: *const core::ffi::c_char = core::ptr::null();
    if header.is_null() { return core::ptr::null_mut(); }
    let domainname = (*(*r).domain).domainname.name;
    len += strlen(domainname) as i32 + strlen(header) as i32 + 10;
    if !(*r).ee.is_null() { let file = (*(*r).ee).bprm.file; realpath = tomoyo_realpath_from_path(&(*file).f_path); bprm_info = tomoyo_print_bprm((*(*r).ee).bprm, &mut (*(*r).ee).dump); if realpath.is_null() || bprm_info.is_null() { kfree(realpath); kfree(bprm_info); kfree(header); return core::ptr::null_mut(); } len += strlen(realpath) as i32 + 80 + strlen(bprm_info) as i32; } else if !(*r).obj.is_null() && !(*(*r).obj).symlink_target.is_null() { symlink = (*(*(*r).obj).symlink_target).name; len += 18 + strlen(symlink) as i32; }
    len = kmalloc_size_roundup(len); buf = kzalloc(len as usize, GFP_NOFS); if buf.is_null() { kfree(realpath); kfree(bprm_info); kfree(header); return buf; }
    len -= 1; let mut pos = snprintf(buf, len as usize, c"%s", header);
    if !realpath.is_null() { let bprm = (*(*r).ee).bprm; pos += snprintf(buf.add(pos as usize), (len-pos) as usize, c" exec={ realpath=\"%s\" argc=%d envc=%d %s }", realpath, (*bprm).argc, (*bprm).envc, bprm_info); } else if !symlink.is_null() { pos += snprintf(buf.add(pos as usize), (len-pos) as usize, c" symlink.target=\"%s\"", symlink); }
    pos += snprintf(buf.add(pos as usize), (len-pos) as usize, c"\n%s\n", domainname); vsnprintf(buf.add(pos as usize), (len-pos) as usize, fmt, args);
    kfree(realpath); kfree(bprm_info); kfree(header); buf
}

/* Wait queue, log structure, list, lock, and counters are supplied by the kernel support layer. */
static mut tomoyo_log_wait: wait_queue_head_t = DECLARE_WAIT_QUEUE_HEAD!();
#[repr(C)] struct tomoyo_log { list: list_head, log: *mut core::ffi::c_char, size: i32 }
static mut tomoyo_log: list_head = LIST_HEAD_INIT!();
static mut tomoyo_log_lock: spinlock_t = DEFINE_SPINLOCK!();
static mut tomoyo_log_count: u32 = 0;

unsafe fn tomoyo_get_audit(ns: *const tomoyo_policy_namespace, profile: u8, index: u8, matched_acl: *const tomoyo_acl_info, is_granted: bool) -> bool {
    if !tomoyo_policy_loaded { return false; }
    let p = tomoyo_profile(ns, profile); if tomoyo_log_count >= (*p).pref[TOMOYO_PREF_MAX_AUDIT_LOG as usize] { return false; }
    if is_granted && !matched_acl.is_null() && !(*matched_acl).cond.is_null() && (*(*matched_acl).cond).grant_log != TOMOYO_GRANTLOG_AUTO { return (*(*matched_acl).cond).grant_log == TOMOYO_GRANTLOG_YES; }
    let category = tomoyo_index2category[index as usize] + TOMOYO_MAX_MAC_INDEX; let mut mode = (*p).config[index as usize];
    if mode == TOMOYO_CONFIG_USE_DEFAULT { mode = (*p).config[category as usize]; } if mode == TOMOYO_CONFIG_USE_DEFAULT { mode = (*p).default_config; }
    if is_granted { mode & TOMOYO_CONFIG_WANT_GRANT_LOG != 0 } else { mode & TOMOYO_CONFIG_WANT_REJECT_LOG != 0 }
}

pub unsafe fn tomoyo_write_log2(r: *mut tomoyo_request_info, len: i32, fmt: *const core::ffi::c_char, args: va_list) { if !tomoyo_get_audit((*(*r).domain).ns, (*r).profile, (*r).type_, (*r).matched_acl, (*r).granted) { return; } let buf = tomoyo_init_log(r, len, fmt, args); if buf.is_null() { return; } let entry = kzalloc_obj::<tomoyo_log>(GFP_NOFS); if entry.is_null() { kfree(buf); return; } (*entry).log = buf; let size = kmalloc_size_roundup(strlen(buf) as i32 + 1); (*entry).size = size + kmalloc_size_roundup(core::mem::size_of::<tomoyo_log>() as i32); let mut quota_exceeded = false; spin_lock(&mut tomoyo_log_lock); if tomoyo_memory_quota[TOMOYO_MEMORY_AUDIT as usize] != 0 && tomoyo_memory_used[TOMOYO_MEMORY_AUDIT as usize] + (*entry).size as u32 >= tomoyo_memory_quota[TOMOYO_MEMORY_AUDIT as usize] { quota_exceeded = true; } else { tomoyo_memory_used[TOMOYO_MEMORY_AUDIT as usize] += (*entry).size as u32; list_add_tail(&mut (*entry).list, &mut tomoyo_log); tomoyo_log_count += 1; } spin_unlock(&mut tomoyo_log_lock); if quota_exceeded { kfree(buf); kfree(entry); return; } wake_up(&mut tomoyo_log_wait); }

pub unsafe fn tomoyo_write_log(r: *mut tomoyo_request_info, fmt: *const core::ffi::c_char, mut args: ...) { let len = vsnprintf(core::ptr::null_mut(), 0, fmt, args) + 1; tomoyo_write_log2(r, len, fmt, args); }

pub unsafe fn tomoyo_read_log(head: *mut tomoyo_io_buffer) { if (*head).r.w_pos != 0 { return; } kfree((*head).read_buf); (*head).read_buf = core::ptr::null_mut(); spin_lock(&mut tomoyo_log_lock); let ptr = if !list_empty(&tomoyo_log) { let p = list_entry(tomoyo_log.next, tomoyo_log, list); list_del(&mut (*p).list); tomoyo_log_count -= 1; tomoyo_memory_used[TOMOYO_MEMORY_AUDIT as usize] -= (*p).size as u32; p } else { core::ptr::null_mut() }; spin_unlock(&mut tomoyo_log_lock); if !ptr.is_null() { (*head).read_buf = (*ptr).log; (*head).r.w[(*head).r.w_pos as usize] = (*head).read_buf; (*head).r.w_pos += 1; kfree(ptr); } }

pub unsafe fn tomoyo_poll_log(file: *mut file, wait: *mut poll_table) -> __poll_t { if tomoyo_log_count != 0 { return EPOLLIN | EPOLLRDNORM; } poll_wait(file, &mut tomoyo_log_wait, wait); if tomoyo_log_count != 0 { EPOLLIN | EPOLLRDNORM } else { 0 } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
