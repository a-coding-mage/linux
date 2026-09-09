// SPDX-License-Identifier: GPL-2.0
/* Rust translation of xfs_error.c.  C header dependencies are supplied by the
 * surrounding XFS translation and are intentionally not reimplemented here. */

#[cfg(debug_assertions)]
extern "C" {
    static xfs_errortag_random_default: *const u32;
    static xfs_errortag_names: *const *const core::ffi::c_char;
}

extern "C" {
    static xfs_error_level: i32;
}

#[cfg(debug_assertions)]
#[no_mangle]
pub unsafe extern "C" fn xfs_errortag_init(mp: *mut xfs_mount) -> i32 {
    xfs_sysfs_init(&mut (*mp).m_errortag_kobj, &xfs_errortag_ktype,
        &mut (*mp).m_kobj, b"errortag\0".as_ptr() as *const _)
}

#[cfg(debug_assertions)]
#[no_mangle]
pub unsafe extern "C" fn xfs_errortag_del(mp: *mut xfs_mount) {
    xfs_sysfs_del(&mut (*mp).m_errortag_kobj);
}

#[cfg(debug_assertions)]
#[no_mangle]
pub unsafe extern "C" fn xfs_errortag_test(
    mp: *mut xfs_mount, file: *const core::ffi::c_char, line: i32,
    error_tag: u32,
) -> bool {
    let randfactor = core::ptr::read_volatile((*mp).m_errortag.as_ptr().add(error_tag as usize));
    if randfactor == 0 || get_random_u32_below(randfactor) != 0 {
        return false;
    }
    xfs_warn_ratelimited(mp, b"Injecting error at file %s, line %d, on filesystem \"%s\"\0".as_ptr() as *const _, file, line, (*(*mp).m_super).s_id);
    true
}

#[cfg(debug_assertions)]
#[no_mangle]
pub unsafe extern "C" fn xfs_errortag_delay(
    mp: *mut xfs_mount, file: *const core::ffi::c_char, line: i32,
    error_tag: u32,
) {
    might_sleep();
    let delay = core::ptr::read_volatile((*mp).m_errortag.as_ptr().add(error_tag as usize));
    if delay == 0 { return; }
    xfs_warn_ratelimited(mp, b"Injecting %ums delay at file %s, line %d, on filesystem \"%s\"\0".as_ptr() as *const _, delay, file, line, (*(*mp).m_super).s_id);
    mdelay(delay);
}

#[cfg(debug_assertions)]
#[no_mangle]
pub unsafe extern "C" fn xfs_errortag_add(mp: *mut xfs_mount, error_tag: u32) -> i32 {
    if error_tag >= XFS_ERRTAG_MAX { return -EINVAL; }
    if error_tag == XFS_ERRTAG_DROP_WRITES { return -EINVAL; }
    let val = core::ptr::read(xfs_errortag_random_default.add(error_tag as usize));
    core::ptr::write_volatile((*mp).m_errortag.as_mut_ptr().add(error_tag as usize), val);
    0
}

#[cfg(debug_assertions)]
#[no_mangle]
pub unsafe extern "C" fn xfs_errortag_add_name(mp: *mut xfs_mount, tag_name: *const core::ffi::c_char) -> i32 {
    for i in 0..XFS_ERRTAG_MAX {
        let name = *xfs_errortag_names.add(i as usize);
        if !name.is_null() && strcmp(name, tag_name) == 0 { return xfs_errortag_add(mp, i); }
    }
    -EINVAL
}

#[cfg(debug_assertions)]
#[no_mangle]
pub unsafe extern "C" fn xfs_errortag_copy(dst_mp: *mut xfs_mount, src_mp: *mut xfs_mount) {
    for i in 0..XFS_ERRTAG_MAX {
        let val = core::ptr::read_volatile((*src_mp).m_errortag.as_ptr().add(i as usize));
        if val != 0 { core::ptr::write_volatile((*dst_mp).m_errortag.as_mut_ptr().add(i as usize), val); }
    }
}

#[cfg(debug_assertions)]
#[no_mangle]
pub unsafe extern "C" fn xfs_errortag_clearall(mp: *mut xfs_mount) -> i32 {
    for i in 0..XFS_ERRTAG_MAX { core::ptr::write_volatile((*mp).m_errortag.as_mut_ptr().add(i as usize), 0); }
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfs_error_report(tag: *const core::ffi::c_char, level: i32,
    mp: *mut xfs_mount, filename: *const core::ffi::c_char, linenum: i32,
    failaddr: xfs_failaddr_t) {
    if level <= xfs_error_level {
        xfs_alert_tag(mp, XFS_PTAG_ERROR_REPORT, b"Internal error %s at line %d of file %s.  Caller %pS\0".as_ptr() as *const _, tag, linenum, filename, failaddr);
        xfs_stack_trace();
    }
}

#[no_mangle]
pub unsafe extern "C" fn xfs_corruption_error(tag: *const core::ffi::c_char, level: i32,
    mp: *mut xfs_mount, buf: *const core::ffi::c_void, bufsize: usize,
    filename: *const core::ffi::c_char, linenum: i32, failaddr: xfs_failaddr_t) {
    if !buf.is_null() && level <= xfs_error_level { xfs_hex_dump(buf, bufsize); }
    xfs_error_report(tag, level, mp, filename, linenum, failaddr);
    xfs_alert(mp, b"Corruption detected. Unmount and run xfs_repair\0".as_ptr() as *const _);
}

#[no_mangle]
pub unsafe extern "C" fn xfs_buf_corruption_error(bp: *mut xfs_buf, fa: xfs_failaddr_t) {
    let mp = (*bp).b_mount;
    xfs_alert_tag(mp, XFS_PTAG_VERIFIER_ERROR, b"Metadata corruption detected at %pS, %s block 0x%llx\0".as_ptr() as *const _, fa, (*(*bp).b_ops).name, xfs_buf_daddr(bp));
    xfs_alert(mp, b"Unmount and run xfs_repair\0".as_ptr() as *const _);
    if xfs_error_level >= XFS_ERRLEVEL_HIGH { xfs_stack_trace(); }
}

#[no_mangle]
pub unsafe extern "C" fn xfs_buf_verifier_error(bp: *mut xfs_buf, error: i32,
    name: *const core::ffi::c_char, buf: *const core::ffi::c_void, bufsz: usize,
    failaddr: xfs_failaddr_t) {
    let mp = (*bp).b_mount;
    let fa = if failaddr != 0 { failaddr } else { __return_address() };
    __xfs_buf_ioerror(bp, error, fa);
    xfs_alert_tag(mp, XFS_PTAG_VERIFIER_ERROR, b"Metadata %s detected at %pS, %s block 0x%llx %s\0".as_ptr() as *const _, if (*bp).b_error == -EFSBADCRC { b"CRC error\0".as_ptr() } else { b"corruption\0".as_ptr() }, fa, (*(*bp).b_ops).name, xfs_buf_daddr(bp), name);
    xfs_alert(mp, b"Unmount and run xfs_repair\0".as_ptr() as *const _);
    if xfs_error_level >= XFS_ERRLEVEL_LOW { let sz = core::cmp::min(XFS_CORRUPTION_DUMP_LEN, bufsz); xfs_alert(mp, b"First %d bytes of corrupted metadata buffer:\0".as_ptr() as *const _, sz); xfs_hex_dump(buf, sz); }
    if xfs_error_level >= XFS_ERRLEVEL_HIGH { xfs_stack_trace(); }
}

#[no_mangle]
pub unsafe extern "C" fn xfs_verifier_error(bp: *mut xfs_buf, error: i32, failaddr: xfs_failaddr_t) {
    xfs_buf_verifier_error(bp, error, b"\0".as_ptr() as *const _, xfs_buf_offset(bp, 0), XFS_CORRUPTION_DUMP_LEN, failaddr);
}

#[no_mangle]
pub unsafe extern "C" fn xfs_inode_verifier_error(ip: *mut xfs_inode, error: i32,
    name: *const core::ffi::c_char, buf: *const core::ffi::c_void, bufsz: usize,
    failaddr: xfs_failaddr_t) {
    let mp = (*ip).i_mount;
    let fa = if failaddr != 0 { failaddr } else { __return_address() };
    xfs_alert(mp, b"Metadata %s detected at %pS, inode 0x%llx %s\0".as_ptr() as *const _, if error == -EFSBADCRC { b"CRC error\0".as_ptr() } else { b"corruption\0".as_ptr() }, fa, I_INO(ip), name);
    xfs_alert(mp, b"Unmount and run xfs_repair\0".as_ptr() as *const _);
    if !buf.is_null() && xfs_error_level >= XFS_ERRLEVEL_LOW { let sz = core::cmp::min(XFS_CORRUPTION_DUMP_LEN, bufsz); xfs_alert(mp, b"First %d bytes of corrupted metadata buffer:\0".as_ptr() as *const _, sz); xfs_hex_dump(buf, sz); }
    if xfs_error_level >= XFS_ERRLEVEL_HIGH { xfs_stack_trace(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
