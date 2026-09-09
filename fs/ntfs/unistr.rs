// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * NTFS Unicode string handling.
 *
 * Copyright (c) 2001-2006 Anton Altaparmakov
 */

/* IMPORTANT: all Unicode characters are little endian inside the strings. */

/* Used by the name collation functions to quickly determine invalid characters. */
static LEGAL_ANSI_CHAR_ARRAY: [u8; 0x40] = [
    0x00, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10,
    0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10,
    0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10,
    0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10,
    0x17, 0x07, 0x18, 0x17, 0x17, 0x17, 0x17, 0x17,
    0x17, 0x17, 0x18, 0x16, 0x16, 0x17, 0x07, 0x00,
    0x17, 0x17, 0x17, 0x17, 0x17, 0x17, 0x17, 0x17,
    0x17, 0x17, 0x04, 0x16, 0x18, 0x16, 0x18, 0x18,
];

pub unsafe fn ntfs_are_names_equal(s1: *const __le16, s1_len: usize, s2: *const __le16, s2_len: usize, ic: u32, upcase: *const __le16, upcase_size: u32) -> bool {
    if s1_len != s2_len { return false; }
    if ic == CASE_SENSITIVE { return ntfs_ucsncmp(s1, s2, s1_len) == 0; }
    ntfs_ucsncasecmp(s1, s2, s1_len, upcase, upcase_size) == 0
}

pub unsafe fn ntfs_collate_names(name1: *const __le16, name1_len: u32, name2: *const __le16, name2_len: u32, err_val: i32, ic: u32, upcase: *const __le16, upcase_len: u32) -> i32 {
    let min_len = if name1_len > name2_len { name2_len } else { name1_len };
    for cnt in 0..min_len as usize {
        let mut c1 = le16_to_cpu(*name1.add(cnt));
        let mut c2 = le16_to_cpu(*name2.add(cnt));
        if ic != 0 {
            if c1 as u32 < upcase_len { c1 = le16_to_cpu(*upcase.add(c1 as usize)); }
            if c2 as u32 < upcase_len { c2 = le16_to_cpu(*upcase.add(c2 as usize)); }
        }
        if c1 < 64 && (LEGAL_ANSI_CHAR_ARRAY[c1 as usize] & 8) != 0 { return err_val; }
        if c1 < c2 { return -1; }
        if c1 > c2 { return 1; }
    }
    if name1_len < name2_len { return -1; }
    if name1_len == name2_len { return 0; }
    let c1 = le16_to_cpu(*name1.add(min_len as usize));
    if c1 < 64 && (LEGAL_ANSI_CHAR_ARRAY[c1 as usize] & 8) != 0 { return err_val; }
    1
}

pub unsafe fn ntfs_ucsncmp(s1: *const __le16, s2: *const __le16, n: usize) -> i32 {
    for i in 0..n {
        let c1 = le16_to_cpu(*s1.add(i));
        let c2 = le16_to_cpu(*s2.add(i));
        if c1 < c2 { return -1; }
        if c1 > c2 { return 1; }
        if c1 == 0 { break; }
    }
    0
}

pub unsafe fn ntfs_ucsncasecmp(s1: *const __le16, s2: *const __le16, n: usize, upcase: *const __le16, upcase_size: u32) -> i32 {
    for i in 0..n {
        let mut c1 = le16_to_cpu(*s1.add(i));
        if c1 as u32 < upcase_size { c1 = le16_to_cpu(*upcase.add(c1 as usize)); }
        let mut c2 = le16_to_cpu(*s2.add(i));
        if c2 as u32 < upcase_size { c2 = le16_to_cpu(*upcase.add(c2 as usize)); }
        if c1 < c2 { return -1; }
        if c1 > c2 { return 1; }
        if c1 == 0 { break; }
    }
    0
}

pub unsafe fn ntfs_file_compare_values(file_name_attr1: *const struct_file_name_attr, file_name_attr2: *const struct_file_name_attr, err_val: i32, ic: u32, upcase: *const __le16, upcase_len: u32) -> i32 {
    ntfs_collate_names((&(*file_name_attr1).file_name) as *const _ as *const __le16, (*file_name_attr1).file_name_length, (&(*file_name_attr2).file_name) as *const _ as *const __le16, (*file_name_attr2).file_name_length, err_val, ic, upcase, upcase_len)
}

pub unsafe fn ntfs_nlstoucs(vol: *const struct_ntfs_volume, ins: *const i8, ins_len: i32, outs: *mut *mut __le16, max_name_len: i32) -> i32 {
    let nls = (*vol).nls_map;
    let ucs: *mut __le16;
    if !ins.is_null() {
        if max_name_len > NTFS_MAX_NAME_LEN { ucs = kvmalloc(((max_name_len + 2) as usize) * core::mem::size_of::<__le16>(), GFP_NOFS | __GFP_ZERO) as *mut __le16; }
        else { ucs = kmem_cache_alloc(ntfs_name_cache, GFP_NOFS) as *mut __le16; }
        if !ucs.is_null() {
            let mut o: i32 = 0;
            let mut wc_len: i32 = 0;
            if (*vol).nls_utf8 {
                o = utf8s_to_utf16s(ins, ins_len, UTF16_LITTLE_ENDIAN, ucs as *mut wchar_t, max_name_len + 2);
                if o < 0 || o > max_name_len { wc_len = o; goto_name_err(vol, nls, ucs, max_name_len, wc_len); return if wc_len < 0 { -EILSEQ } else { -ENAMETOOLONG }; }
            } else {
                let mut i = 0;
                while i < ins_len {
                    let mut wc: wchar_t = 0;
                    wc_len = (*nls).char2uni(ins.add(i as usize), ins_len - i, &mut wc);
                    if wc_len >= 0 && o < max_name_len {
                        if wc != 0 { *ucs.add(o as usize) = cpu_to_le16(wc as u16); o += 1; i += wc_len; continue; }
                        break;
                    }
                    goto_name_err(vol, nls, ucs, max_name_len, wc_len); return if wc_len < 0 { -EILSEQ } else { -ENAMETOOLONG };
                }
            }
            *ucs.add(o as usize) = 0; *outs = ucs; return o;
        }
        ntfs_debug(c_str!("Failed to allocate buffer for converted name from ntfs_name_cache.")); return -ENOMEM;
    }
    ntfs_error((*vol).sb, c_str!("Received NULL pointer.")); -EINVAL
}

/* Allocation/error helper corresponding to the C name_err label. */
unsafe fn goto_name_err(_vol: *const struct_ntfs_volume, _nls: *mut struct_nls_table, ucs: *mut __le16, max_name_len: i32, _wc_len: i32) {
    if max_name_len > NTFS_MAX_NAME_LEN { kvfree(ucs as *mut core::ffi::c_void); } else { kmem_cache_free(ntfs_name_cache, ucs as *mut core::ffi::c_void); }
}

pub unsafe fn ntfs_ucstonls(vol: *const struct_ntfs_volume, ins: *const __le16, ins_len: i32, outs: *mut *mut u8, outs_len: i32) -> i32 {
    let nls = (*vol).nls_map;
    if ins.is_null() { ntfs_error((*vol).sb, c_str!("Received NULL pointer.")); return -EINVAL; }
    let mut ns = *outs; let mut ns_len = outs_len;
    if !ns.is_null() && ns_len == 0 { return -ENAMETOOLONG; }
    if ns.is_null() { ns_len = ins_len * NLS_MAX_CHARSET_SIZE; ns = kmalloc((ns_len + 1) as usize, GFP_NOFS) as *mut u8; if ns.is_null() { return -ENOMEM; } }
    let o = if (*vol).nls_utf8 { utf16s_to_utf8s(ins as *const wchar_t, ins_len, UTF16_LITTLE_ENDIAN, ns, ns_len) } else {
        let mut i = 0; let mut o = 0;
        while i < ins_len { let wc = (*nls).uni2char(le16_to_cpu(*ins.add(i as usize)), ns.add(o as usize), ns_len - o); if wc > 0 { o += wc; i += 1; } else if wc == 0 { break; } else { if wc == -ENAMETOOLONG && ns != *outs { let tc = kmalloc(((ns_len + 64) & !63) as usize, GFP_NOFS) as *mut u8; if !tc.is_null() { memcpy(tc as *mut core::ffi::c_void, ns as *const core::ffi::c_void, ns_len as usize); ns_len = ((ns_len + 64) & !63) - 1; kfree(ns as *mut core::ffi::c_void); ns = tc; continue; } } if ns != *outs { kfree(ns as *mut core::ffi::c_void); } return if wc == -ENAMETOOLONG { wc } else { -EILSEQ }; } }
        o
    };
    if o >= ns_len { if ns != *outs { kfree(ns as *mut core::ffi::c_void); } return -ENAMETOOLONG; }
    *ns.add(o as usize) = 0; *outs = ns; o
}

unsafe fn ntfs_ucsnlen(s: *const __le16, maxlen: u32) -> u32 { let mut i = 0; while i < maxlen && le16_to_cpu(*s.add(i as usize)) != 0 { i += 1; } i }

pub unsafe fn ntfs_ucsndup(s: *const __le16, maxlen: u32) -> *mut __le16 {
    let len = ntfs_ucsnlen(s, maxlen); let dst = kmalloc(((len + 1) as usize) * core::mem::size_of::<__le16>(), GFP_NOFS) as *mut __le16;
    if !dst.is_null() { memcpy(dst as *mut core::ffi::c_void, s as *const core::ffi::c_void, (len as usize) * core::mem::size_of::<__le16>()); *dst.add(len as usize) = cpu_to_le16(0); } dst
}

pub unsafe fn ntfs_names_are_equal(s1: *const __le16, s1_len: usize, s2: *const __le16, s2_len: usize, ic: u32, upcase: *const __le16, upcase_size: u32) -> bool {
    if s1_len != s2_len { return false; } if s1_len == 0 { return true; }
    if ic == CASE_SENSITIVE { ntfs_ucsncmp(s1, s2, s1_len) == 0 } else { ntfs_ucsncasecmp(s1, s2, s1_len, upcase, upcase_size) == 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
