// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *
 *   Copyright (c) International Business Machines Corp., 2002,2008
 *   Author(s): Steve French (sfrench@us.ibm.com)
 *
 *   Error mapping routines from Samba libsmb/errormap.c
 *   Copyright (C) Andrew Tridgell 2001
 */

/* Kernel and CIFS dependencies are supplied by the surrounding translation unit. */

unsafe fn cifs_inet_pton(address_family: libc::c_int, cp: *const libc::c_char,
                         len: libc::c_int, dst: *mut libc::c_void) -> libc::c_int {
    let mut ret: libc::c_int = 0;

    /* calculate length by finding first slash or NULL */
    if address_family == AF_INET {
        ret = in4_pton(cp, len, dst, b'\\' as libc::c_char, core::ptr::null_mut());
    } else if address_family == AF_INET6 {
        ret = in6_pton(cp, len, dst, b'\\' as libc::c_char, core::ptr::null_mut());
    }

    cifs_dbg(NOISY, "address conversion returned %d for %*.*s\n", ret, len, len, cp);
    if ret > 0 {
        ret = 1;
    }
    ret
}

pub unsafe fn cifs_convert_address(dst: *mut sockaddr, src: *const libc::c_char,
                                   len: libc::c_int) -> libc::c_int {
    let mut rc: libc::c_int;
    let mut alen: libc::c_int;
    let mut slen: libc::c_int;
    let pct: *const libc::c_char;
    let mut scope_id = [0 as libc::c_char; 13];
    let s4 = dst as *mut sockaddr_in;
    let s6 = dst as *mut sockaddr_in6;

    if cifs_inet_pton(AF_INET, src, len, core::ptr::addr_of_mut!((*s4).sin_addr.s_addr) as *mut libc::c_void) != 0 {
        (*s4).sin_family = AF_INET as _;
        return 1;
    }

    pct = memchr(src as *const libc::c_void, b'%' as libc::c_int, len as usize) as *const libc::c_char;
    alen = if !pct.is_null() { pct.offset_from(src) as libc::c_int } else { len };
    rc = cifs_inet_pton(AF_INET6, src, alen,
                        core::ptr::addr_of_mut!((*s6).sin6_addr.s6_addr) as *mut libc::c_void);
    if rc == 0 { return rc; }

    (*s6).sin6_family = AF_INET6 as _;
    if !pct.is_null() {
        slen = len - (alen + 1);
        if slen <= 0 || slen > 12 { return 0; }
        core::ptr::copy_nonoverlapping(pct.add(1), scope_id.as_mut_ptr(), slen as usize);
        scope_id[slen as usize] = 0;
        rc = kstrtouint(scope_id.as_ptr(), 0, core::ptr::addr_of_mut!((*s6).sin6_scope_id));
        rc = if rc == 0 { 1 } else { 0 };
    }
    rc
}

pub unsafe fn cifs_set_port(addr: *mut sockaddr, port: libc::c_ushort) {
    match (*addr).sa_family as libc::c_int {
        AF_INET => (*((addr as *mut sockaddr_in))).sin_port = htons(port),
        AF_INET6 => (*((addr as *mut sockaddr_in6))).sin6_port = htons(port),
        _ => {}
    }
}

const NTFS_TIME_OFFSET: u64 = (369 * 365 + 89) as u64 * 24 * 3600 * 10000000;

pub unsafe fn cifs_NTtimeToUnix(ntutc: __le64) -> timespec64 {
    let mut ts: timespec64 = core::mem::zeroed();
    /* BB what about the timezone? BB */
    let t: i64 = le64_to_cpu(ntutc) as i64 - NTFS_TIME_OFFSET as i64;
    let mut abs_t: u64;
    if t < 0 {
        abs_t = (-t) as u64;
        ts.tv_nsec = (abs_t % 10000000 * 100) as time64_t;
        abs_t /= 10000000;
        ts.tv_nsec = -ts.tv_nsec;
        ts.tv_sec = -(abs_t as i64);
    } else {
        abs_t = t as u64;
        ts.tv_nsec = (abs_t % 10000000 * 100) as time64_t;
        abs_t /= 10000000;
        ts.tv_sec = abs_t as i64;
    }
    ts
}

pub fn cifs_UnixTimeToNT(t: timespec64) -> u64 {
    t.tv_sec as u64 * 10000000 + t.tv_nsec as u64 / 100 + NTFS_TIME_OFFSET
}

static TOTAL_DAYS_OF_PREV_MONTHS: [i32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

pub unsafe fn cnvrtDosUnixTm(le_date: __le16, le_time: __le16, offset: i32) -> timespec64 {
    let mut ts: timespec64 = core::mem::zeroed();
    let date = le16_to_cpu(le_date);
    let time = le16_to_cpu(le_time);
    let st = &*(core::ptr::addr_of!(time) as *const SMB_TIME);
    let sd = &*(core::ptr::addr_of!(date) as *const SMB_DATE);
    cifs_dbg(FYI, "date %d time %d\n", date, time);
    let mut sec: time64_t = 2 * st.TwoSeconds as time64_t;
    let min = st.Minutes as time64_t;
    if sec > 59 || min > 59 { cifs_dbg(VFS, "Invalid time min %d sec %lld\n", min, sec); }
    sec += min * 60 + 60 * 60 * st.Hours as time64_t;
    if st.Hours > 24 { cifs_dbg(VFS, "Invalid hours %d\n", st.Hours); }
    let mut day = sd.Day as i32;
    let mut month = sd.Month as i32;
    if day < 1 || day > 31 || month < 1 || month > 12 {
        cifs_dbg(VFS, "Invalid date, month %d day: %d\n", month, day);
        day = clamp(day, 1, 31); month = clamp(month, 1, 12);
    }
    month -= 1;
    let mut days: time64_t = (day + TOTAL_DAYS_OF_PREV_MONTHS[month as usize] + 3652) as time64_t;
    let year = sd.Year as time64_t;
    days += year * 365 + year / 4;
    if year >= 120 { days -= 1; }
    if year != 120 { days -= if (year & 3) == 0 && month < 2 { 1 } else { 0 }; }
    sec += 24 * 60 * 60 * days;
    ts.tv_sec = sec + offset as time64_t;
    ts.tv_nsec = 0;
    ts
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
