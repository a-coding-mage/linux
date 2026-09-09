// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2011-2014 PLUMgrid, http://plumgrid.com
 * Copyright (c) 2016 Facebook
 * Copyright (c) 2018 Covalent IO, Inc. http://covalent.io
 */

// Dependencies supplied by the surrounding kernel translation.

#[inline]
unsafe fn bpf_verifier_log_attr_valid(log_level: u32, log_buf: *mut c_char, log_size: u32) -> bool {
    /* ubuf and len_total should both be specified (or not) together */
    if (log_buf != core::ptr::null_mut()) != (log_size != 0) { return false; }
    /* log buf without log_level is meaningless */
    if !log_buf.is_null() && log_level == 0 { return false; }
    if log_level & !BPF_LOG_MASK != 0 { return false; }
    if log_size > u32::MAX >> 2 { return false; }
    true
}

pub unsafe fn bpf_vlog_init(log: *mut bpf_verifier_log, log_level: u32,
                            log_buf: *mut c_char, log_size: u32) -> i32 {
    (*log).level = log_level;
    (*log).ubuf = log_buf;
    (*log).len_total = log_size;
    /* log attributes have to be sane */
    if !bpf_verifier_log_attr_valid(log_level, log_buf, log_size) { return -EINVAL; }
    0
}

unsafe fn bpf_vlog_update_len_max(log: *mut bpf_verifier_log, add_len: u32) {
    /* add_len includes terminal \0, so no need for +1. */
    let len = (*log).end_pos.wrapping_add(add_len as u64);
    /* log->len_max could be larger than our current len due to
     * bpf_vlog_reset() calls, so we maintain the max of any length at any
     * previous point
     */
    if len > u32::MAX as u64 { (*log).len_max = u32::MAX; }
    else if len > (*log).len_max as u64 { (*log).len_max = len as u32; }
}

pub unsafe fn bpf_verifier_vlog(log: *mut bpf_verifier_log, fmt: *const c_char, args: va_list) {
    let mut cur_pos: u64;
    let mut new_n: u32;
    let mut n = vscnprintf((*log).kbuf.as_mut_ptr(), BPF_VERIFIER_TMP_LOG_SIZE, fmt, args);
    if (*log).level == BPF_LOG_KERNEL {
        let newline = n > 0 && (*log).kbuf[(n - 1) as usize] == b'\n' as c_char;
        pr_err(cstr!("BPF: %s%s"), (*log).kbuf.as_ptr(), if newline { cstr!("") } else { cstr!("\n") });
        return;
    }
    n += 1;
    bpf_vlog_update_len_max(log, n);
    if (*log).level & BPF_LOG_FIXED != 0 {
        new_n = 0;
        if (*log).end_pos < (*log).len_total as u64 {
            new_n = core::cmp::min((*log).len_total as u64 - (*log).end_pos, n as u64) as u32;
            (*log).kbuf[(new_n - 1) as usize] = 0;
        }
        cur_pos = (*log).end_pos;
        (*log).end_pos += (n - 1) as u64;
        if !(*log).ubuf.is_null() && new_n != 0 && copy_to_user((*log).ubuf.add(cur_pos as usize), (*log).kbuf.as_ptr(), new_n as usize) != 0 { (*log).ubuf = core::ptr::null_mut(); }
    } else {
        let new_end = (*log).end_pos + n as u64;
        let new_start = if new_end - (*log).start_pos >= (*log).len_total as u64 { new_end - (*log).len_total as u64 } else { (*log).start_pos };
        (*log).start_pos = new_start;
        (*log).end_pos = new_end - 1;
        if (*log).ubuf.is_null() { return; }
        new_n = core::cmp::min(n, (*log).len_total);
        cur_pos = new_end - new_n as u64;
        let mut buf_start = 0u32;
        let mut buf_end = 0u32;
        div_u64_rem(cur_pos, (*log).len_total as u64, &mut buf_start);
        div_u64_rem(new_end, (*log).len_total as u64, &mut buf_end);
        if buf_end == 0 { buf_end = (*log).len_total; }
        if buf_start < buf_end {
            if copy_to_user((*log).ubuf.add(buf_start as usize), (*log).kbuf.as_ptr().add((n - new_n) as usize), (buf_end - buf_start) as usize) != 0 { (*log).ubuf = core::ptr::null_mut(); }
        } else {
            if copy_to_user((*log).ubuf.add(buf_start as usize), (*log).kbuf.as_ptr().add((n - new_n) as usize), ((*log).len_total - buf_start) as usize) != 0 || copy_to_user((*log).ubuf, (*log).kbuf.as_ptr().add((n - buf_end) as usize), buf_end as usize) != 0 { (*log).ubuf = core::ptr::null_mut(); }
        }
    }
}

pub unsafe fn bpf_vlog_reset(log: *mut bpf_verifier_log, new_pos: u64) {
    let zero: c_char = 0;
    if WARN_ON_ONCE(new_pos > (*log).end_pos) { return; }
    if !bpf_verifier_log_needed(log) || (*log).level == BPF_LOG_KERNEL { return; }
    (*log).end_pos = new_pos;
    if (*log).end_pos < (*log).start_pos { (*log).start_pos = (*log).end_pos; }
    if (*log).ubuf.is_null() { return; }
    let pos = if (*log).level & BPF_LOG_FIXED != 0 { (*log).end_pos as u32 + 1 } else { let mut p=0; div_u64_rem(new_pos, (*log).len_total as u64, &mut p); p };
    if pos < (*log).len_total && put_user(zero, (*log).ubuf.add(pos as usize)) != 0 { (*log).ubuf = core::ptr::null_mut(); }
}

unsafe fn bpf_vlog_reverse_kbuf(buf: *mut c_char, len: i32) {
    let (mut i, mut j) = (0, len - 1);
    while i < j { core::ptr::swap(buf.add(i as usize), buf.add(j as usize)); i += 1; j -= 1; }
}

unsafe fn bpf_vlog_reverse_ubuf(log: *mut bpf_verifier_log, mut start: i32, mut end: i32) -> i32 {
    let n = core::mem::size_of_val(&(*log).kbuf) as i32 / 2;
    let lbuf = (*log).kbuf.as_mut_ptr(); let rbuf = lbuf.add(n as usize);
    while end - start > 1 {
        let nn = core::cmp::min(n, (end - start) / 2);
        if copy_from_user(lbuf, (*log).ubuf.add(start as usize), nn as usize) != 0 || copy_from_user(rbuf, (*log).ubuf.add((end - nn) as usize), nn as usize) != 0 { return -EFAULT; }
        bpf_vlog_reverse_kbuf(lbuf, nn); bpf_vlog_reverse_kbuf(rbuf, nn);
        if copy_to_user((*log).ubuf.add(start as usize), rbuf, nn as usize) != 0 || copy_to_user((*log).ubuf.add((end - nn) as usize), lbuf, nn as usize) != 0 { return -EFAULT; }
        start += nn; end -= nn;
    }
    0
}

pub unsafe fn bpf_vlog_finalize(log: *mut bpf_verifier_log, log_size_actual: *mut u32) -> i32 {
    *log_size_actual = 0;
    if log.is_null() || (*log).level == 0 || (*log).level == BPF_LOG_KERNEL { return 0; }
    if !(*log).ubuf.is_null() && (*log).start_pos != 0 {
        let mut sublen=0; div_u64_rem((*log).start_pos, (*log).len_total as u64, &mut sublen); sublen = (*log).len_total - sublen;
        let mut err = bpf_vlog_reverse_ubuf(log, 0, (*log).len_total as i32);
        if err == 0 { err = bpf_vlog_reverse_ubuf(log, 0, sublen as i32); }
        if err == 0 { err = bpf_vlog_reverse_ubuf(log, sublen as i32, (*log).len_total as i32); }
        if err != 0 { (*log).ubuf = core::ptr::null_mut(); }
    }
    *log_size_actual = (*log).len_max;
    if ((*log).ubuf.is_null()) != ((*log).len_total == 0) { return -EFAULT; }
    if !(*log).ubuf.is_null() && (*log).len_max > (*log).len_total { return -ENOSPC; }
    0
}

pub unsafe fn bpf_verifier_log_write(env: *mut bpf_verifier_env, fmt: *const c_char, mut args: va_list) {
    if !bpf_verifier_log_needed(&mut (*env).log) { return; }
    bpf_verifier_vlog(&mut (*env).log, fmt, args);
}

pub unsafe fn bpf_log(log: *mut bpf_verifier_log, fmt: *const c_char, mut args: va_list) {
    if !bpf_verifier_log_needed(log) { return; }
    bpf_verifier_vlog(log, fmt, args);
}

unsafe fn ltrim(mut s: *const c_char) -> *const c_char {
    while isspace(*s as i32) != 0 { s = s.add(1); } s
}

pub unsafe fn verbose_linfo(env: *mut bpf_verifier_env, insn_off: u32, prefix_fmt: *const c_char, mut args: va_list) {
    if !bpf_verifier_log_needed(&mut (*env).log) { return; }
    let prev_linfo = (*env).prev_linfo;
    let linfo = bpf_find_linfo((*env).prog, insn_off);
    if linfo.is_null() || linfo == prev_linfo { return; }
    if !prev_linfo.is_null() && (*linfo).file_name_off == (*prev_linfo).file_name_off && BPF_LINE_INFO_LINE_NUM((*linfo).line_col) == BPF_LINE_INFO_LINE_NUM((*prev_linfo).line_col) { return; }
    if !prefix_fmt.is_null() { bpf_verifier_vlog(&mut (*env).log, prefix_fmt, args); }
    let btf = (*(*env).prog).aux.btf;
    let s = ltrim(btf_name_by_offset(btf, (*linfo).line_off));
    bpf_verifier_log_write(env, cstr!("%s"), va_list!(s));
    let s = btf_name_by_offset(btf, (*linfo).file_name_off);
    let slash = strrchr(s, b'/' as i32);
    let fname = if slash.is_null() { s } else { slash.add(1) };
    bpf_verifier_log_write(env, cstr!(" @ %s:%u\n"), va_list!(fname, BPF_LINE_INFO_LINE_NUM((*linfo).line_col)));
    (*env).prev_linfo = linfo;
}

unsafe fn btf_type_name(btf: *const btf, id: u32) -> *const c_char { btf_name_by_offset(btf, (*btf_type_by_id(btf, id)).name_off) }

pub unsafe fn reg_type_str(env: *mut bpf_verifier_env, type_: u32) -> *const c_char {
    let mut postfix = [0 as c_char; 16]; let mut prefix = [0 as c_char; 64];
    if type_ & PTR_MAYBE_NULL != 0 { strscpy(postfix.as_mut_ptr(), if base_type(type_) == PTR_TO_BTF_ID { cstr!("or_null_") } else { cstr!("_or_null") }, postfix.len()); }
    snprintf(prefix.as_mut_ptr(), prefix.len(), cstr!("%s%s%s%s%s%s%s"), if type_ & MEM_RDONLY != 0 { cstr!("rdonly_") } else { cstr!("") }, if type_ & MEM_RINGBUF != 0 { cstr!("ringbuf_") } else { cstr!("") }, if type_ & MEM_USER != 0 { cstr!("user_") } else { cstr!("") }, if type_ & MEM_PERCPU != 0 { cstr!("percpu_") } else { cstr!("") }, if type_ & MEM_RCU != 0 { cstr!("rcu_") } else { cstr!("") }, if type_ & PTR_UNTRUSTED != 0 { cstr!("untrusted_") } else { cstr!("") }, if type_ & PTR_TRUSTED != 0 { cstr!("trusted_") } else { cstr!("") });
    snprintf((*env).tmp_str_buf.as_mut_ptr(), TMP_STR_BUF_LEN, cstr!("%s%s%s"), prefix.as_ptr(), reg_type_names[base_type(type_) as usize], postfix.as_ptr());
    (*env).tmp_str_buf.as_ptr()
}

pub fn dynptr_type_str(type_: bpf_dynptr_type) -> *const c_char { match type_ { BPF_DYNPTR_TYPE_LOCAL => cstr!("local"), BPF_DYNPTR_TYPE_RINGBUF => cstr!("ringbuf"), BPF_DYNPTR_TYPE_SKB => cstr!("skb"), BPF_DYNPTR_TYPE_XDP => cstr!("xdp"), BPF_DYNPTR_TYPE_SKB_META => cstr!("skb_meta"), BPF_DYNPTR_TYPE_FILE => cstr!("file"), BPF_DYNPTR_TYPE_INVALID => cstr!("<invalid>"), _ => { WARN_ONCE(true, cstr!("unknown dynptr type %d\n"), type_); cstr!("<unknown>") } } }

pub unsafe fn iter_type_str(btf: *const btf, btf_id: u32) -> *const c_char {
    if btf.is_null() || btf_id == 0 { return cstr!("<invalid>"); }
    /* we already validated that type is valid and has conforming name */
    btf_type_name(btf, btf_id).add(core::mem::size_of::<ITER_PREFIX>() - 1)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
