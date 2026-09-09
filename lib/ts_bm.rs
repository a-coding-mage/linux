// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * lib/ts_bm.c        Boyer-Moore text search implementation
 *
 * Rust translation of the source implementation.
 */

const ASIZE: usize = 256;

#[repr(C)]
pub struct ts_bm {
    pub pattern: *mut u8,
    pub patlen: u32,
    pub bad_shift: [u32; ASIZE],
    pub good_shift: [u32; 0],
}

unsafe fn matchpat(pattern: *const u8, patlen: u32, text: *const u8, icase: bool) -> u32 {
    let mut i: u32 = 0;
    while i < patlen {
        let mut t = *text.offset(-(i as isize));
        if icase {
            t = t.to_ascii_uppercase();
        }
        if t != *pattern.offset(-(i as isize)) {
            break;
        }
        i += 1;
    }
    i
}

unsafe fn bm_find(conf: *mut ts_config, state: *mut ts_state) -> u32 {
    let bm = ts_config_priv(conf);
    let mut text_len: u32;
    let mut consumed = (*state).offset;
    let mut text: *const u8 = core::ptr::null();
    let icase = ((*conf).flags & TS_IGNORECASE) != 0;

    loop {
        let mut shift: i32 = (*bm).patlen as i32 - 1;
        text_len = ((*conf).get_next_block)(consumed, &mut text, conf, state);
        if text_len == 0 {
            break;
        }
        while shift < text_len as i32 {
            let i = matchpat(
                (*bm).pattern.offset((*bm).patlen as isize - 1),
                (*bm).patlen,
                text.offset(shift as isize),
                icase,
            );
            if i == (*bm).patlen {
                return consumed + (shift as u32 - ((*bm).patlen - 1));
            }
            let bs = (*bm).bad_shift[*text.offset((shift as u32 - i) as isize) as usize] as i32;
            let a = shift - i as i32 + bs;
            let b = shift + (*bm).good_shift[i as usize] as i32;
            shift = if a > b { a } else { b };
        }
        consumed += text_len;
    }
    u32::MAX
}

unsafe fn subpattern(pattern: *mut u8, i: i32, j: i32, mut g: i32) -> i32 {
    let mut x = i + g - 1;
    let mut y = j + g - 1;
    let mut ret = 0;
    while *pattern.offset(x as isize) == *pattern.offset(y as isize) {
        x -= 1;
        y -= 1;
        if y < 0 {
            ret = 1;
            break;
        }
        g -= 1;
        if g == 0 {
            ret = if *pattern.offset((i - 1) as isize) != *pattern.offset((j - 1) as isize) { 1 } else { 0 };
            break;
        }
    }
    ret
}

unsafe fn compute_prefix_tbl(bm: *mut ts_bm, flags: i32) {
    for i in 0..ASIZE {
        (*bm).bad_shift[i] = (*bm).patlen;
    }
    for i in 0..((*bm).patlen - 1) as usize {
        (*bm).bad_shift[*(*bm).pattern.add(i) as usize] = (*bm).patlen - 1 - i as u32;
        if (flags & TS_IGNORECASE) != 0 {
            (*bm).bad_shift[(*(*bm).pattern.add(i)).to_ascii_lowercase() as usize] = (*bm).patlen - 1 - i as u32;
        }
    }
    (*bm).good_shift[0] = 1;
    for i in 1..(*bm).patlen as usize {
        *(*bm).good_shift.as_mut_ptr().add(i) = (*bm).patlen;
    }
    let mut g: i32 = 1;
    let mut i = (*bm).patlen as i32 - 1;
    while i > 0 {
        let mut j = i - 1;
        while j >= 1 - g {
            if subpattern((*bm).pattern, i, j, g) != 0 {
                *(*bm).good_shift.as_mut_ptr().add(g as usize) = (*bm).patlen - j as u32 - g as u32;
                break;
            }
            j -= 1;
        }
        g += 1;
        i -= 1;
    }
}

unsafe fn bm_init(pattern: *const core::ffi::c_void, len: u32, gfp_mask: gfp_t, flags: i32) -> *mut ts_config {
    if len == 0 {
        return ERR_PTR(-EINVAL);
    }
    let prefix_tbl_len = match len.checked_mul(core::mem::size_of::<u32>() as u32) {
        Some(v) => v,
        None => return ERR_PTR(-EINVAL),
    };
    let priv_size = match core::mem::size_of::<ts_bm>().checked_add(len as usize).and_then(|v| v.checked_add(prefix_tbl_len as usize)) {
        Some(v) => v,
        None => return ERR_PTR(-EINVAL),
    };
    let conf = alloc_ts_config(priv_size, gfp_mask);
    if IS_ERR(conf) { return conf; }
    (*conf).flags = flags;
    let bm = ts_config_priv(conf);
    (*bm).patlen = len;
    (*bm).pattern = (*bm).good_shift.as_mut_ptr().add((prefix_tbl_len / 4) as usize) as *mut u8;
    if (flags & TS_IGNORECASE) != 0 {
        for i in 0..len as usize { *(*bm).pattern.add(i) = (*(pattern as *const u8).add(i)).to_ascii_uppercase(); }
    } else {
        core::ptr::copy_nonoverlapping(pattern as *const u8, (*bm).pattern, len as usize);
    }
    compute_prefix_tbl(bm, flags);
    conf
}

unsafe fn bm_get_pattern(conf: *mut ts_config) -> *mut core::ffi::c_void { (*ts_config_priv(conf)).pattern as *mut core::ffi::c_void }
unsafe fn bm_get_pattern_len(conf: *mut ts_config) -> u32 { (*ts_config_priv(conf)).patlen }

extern "C" {
    type ts_config;
    type ts_state;
    type gfp_t;
    static TS_IGNORECASE: i32;
    static bm_ops: ts_ops;
    fn ts_config_priv(conf: *mut ts_config) -> *mut ts_bm;
    fn alloc_ts_config(size: usize, mask: gfp_t) -> *mut ts_config;
    fn ERR_PTR(err: i32) -> *mut ts_config;
    fn IS_ERR(ptr: *mut ts_config) -> bool;
    fn textsearch_register(ops: *const ts_ops) -> i32;
    fn textsearch_unregister(ops: *const ts_ops);
}

#[repr(C)] pub struct ts_ops { _private: [u8; 0] }
const EINVAL: i32 = 22;
unsafe fn init_bm() -> i32 { textsearch_register(&bm_ops) }
unsafe fn exit_bm() { textsearch_unregister(&bm_ops); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
