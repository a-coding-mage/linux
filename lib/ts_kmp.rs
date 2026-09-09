// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * lib/ts_kmp.c		Knuth-Morris-Pratt text search implementation
 *
 * Authors:	Thomas Graf <tgraf@suug.ch>
 *
 * ==========================================================================
 *
 *   Implements a linear-time string-matching algorithm due to Knuth,
 *   Morris, and Pratt [1]. Their algorithm avoids the explicit
 *   computation of the transition function DELTA altogether. Its
 *   matching time is O(n), for n being length(text), using just an
 *   auxiliary function PI[1..m], for m being length(pattern),
 *   precomputed from the pattern in time O(m). The array PI allows
 *   the transition function DELTA to be computed efficiently
 *   "on the fly" as needed. Roughly speaking, for any state
 *   "q" = 0,1,...,m and any character "a" in SIGMA, the value
 *   PI["q"] contains the information that is independent of "a" and
 *   is needed to compute DELTA("q", "a") [2]. Since the array PI
 *   has only m entries, whereas DELTA has O(m|SIGMA|) entries, we
 *   save a factor of |SIGMA| in the preprocessing time by computing
 *   PI rather than DELTA.
 *
 *   [1] Cormen, Leiserson, Rivest, Stein
 *       Introdcution to Algorithms, 2nd Edition, MIT Press
 *   [2] See finite automaton theory
 */

use core::ffi::c_void;

type U8 = u8;
type GfpT = usize;

#[repr(C)]
pub struct TsConfig {
    pub flags: i32,
    // Other fields are supplied by the textsearch implementation.
    pub get_next_block: unsafe extern "C" fn(u32, *mut *const U8, *mut TsConfig, *mut TsState) -> u32,
}

#[repr(C)]
pub struct TsState {
    pub offset: u32,
}

#[repr(C)]
pub struct TsOps {
    pub name: *const u8,
    pub find: unsafe extern "C" fn(*mut TsConfig, *mut TsState) -> u32,
    pub init: unsafe extern "C" fn(*const c_void, u32, GfpT, i32) -> *mut TsConfig,
    pub get_pattern: unsafe extern "C" fn(*mut TsConfig) -> *mut c_void,
    pub get_pattern_len: unsafe extern "C" fn(*mut TsConfig) -> u32,
    pub owner: *mut c_void,
    pub list: ListHead,
}

#[repr(C)]
pub struct ListHead {
    pub next: *mut ListHead,
    pub prev: *mut ListHead,
}

#[repr(C)]
struct TsKmp {
    pattern: *mut U8,
    pattern_len: u32,
    prefix_tbl: [u32; 0],
}

extern "C" {
    static THIS_MODULE: c_void;
    fn ts_config_priv(conf: *mut TsConfig) -> *mut c_void;
    fn alloc_ts_config(size: usize, gfp_mask: GfpT) -> *mut TsConfig;
    fn textsearch_register(ops: *mut TsOps) -> i32;
    fn textsearch_unregister(ops: *mut TsOps);
    fn toupper(c: i32) -> i32;
}

const TS_IGNORECASE: i32 = 1 << 0;
const UINT_MAX: u32 = u32::MAX;

unsafe fn kmp_find(conf: *mut TsConfig, state: *mut TsState) -> u32 {
    let kmp = ts_config_priv(conf) as *mut TsKmp;
    let mut i: u32;
    let mut q: u32 = 0;
    let mut text_len: u32;
    let mut consumed = (*state).offset;
    let text: *const U8 = core::ptr::null();
    let icase = (*conf).flags & TS_IGNORECASE;

    loop {
        text_len = ((*conf).get_next_block)(consumed, &text as *const _ as *mut _, conf, state);
        if text_len == 0 { break; }
        i = 0;
        while i < text_len {
            while q > 0 && *(*kmp).prefix_tbl.as_ptr().add(q as usize) != 0 {
                let pattern = *((*kmp).pattern.add(q as usize));
                let ch = if icase != 0 { toupper(*text.add(i as usize) as i32) as U8 } else { *text.add(i as usize) };
                if pattern == ch { break; }
                q = *(*kmp).prefix_tbl.as_ptr().add((q - 1) as usize);
            }
            let ch = if icase != 0 { toupper(*text.add(i as usize) as i32) as U8 } else { *text.add(i as usize) };
            if *(*kmp).pattern.add(q as usize) == ch { q += 1; }
            if q == (*kmp).pattern_len {
                (*state).offset = consumed + i + 1;
                return (*state).offset - (*kmp).pattern_len;
            }
            i += 1;
        }
        consumed += text_len;
    }
    UINT_MAX
}

unsafe fn compute_prefix_tbl(pattern: *const U8, len: u32, prefix_tbl: *mut u32, flags: i32) {
    let mut k = 0u32;
    let mut q = 1u32;
    let icase = flags & TS_IGNORECASE;
    while q < len {
        while k > 0 {
            let a = if icase != 0 { toupper(*pattern.add(k as usize) as i32) as U8 } else { *pattern.add(k as usize) };
            let b = if icase != 0 { toupper(*pattern.add(q as usize) as i32) as U8 } else { *pattern.add(q as usize) };
            if a == b { break; }
            k = *prefix_tbl.add((k - 1) as usize);
        }
        let a = if icase != 0 { toupper(*pattern.add(k as usize) as i32) as U8 } else { *pattern.add(k as usize) };
        let b = if icase != 0 { toupper(*pattern.add(q as usize) as i32) as U8 } else { *pattern.add(q as usize) };
        if a == b { k += 1; }
        *prefix_tbl.add(q as usize) = k;
        q += 1;
    }
}

unsafe fn kmp_init(pattern: *const c_void, len: u32, gfp_mask: GfpT, flags: i32) -> *mut TsConfig {
    if len == 0 { return (-22isize) as *mut TsConfig; }
    let prefix_tbl_len = (len as usize).wrapping_mul(core::mem::size_of::<u32>());
    let priv_size = core::mem::size_of::<TsKmp>().wrapping_add(len as usize).wrapping_add(prefix_tbl_len);
    let conf = alloc_ts_config(priv_size, gfp_mask);
    if conf.is_null() { return conf; }
    (*conf).flags = flags;
    let kmp = ts_config_priv(conf) as *mut TsKmp;
    (*kmp).pattern_len = len;
    compute_prefix_tbl(pattern as *const U8, len, (*kmp).prefix_tbl.as_mut_ptr(), flags);
    (*kmp).pattern = (*kmp).prefix_tbl.as_mut_ptr().add(prefix_tbl_len / core::mem::size_of::<u32>()) as *mut U8;
    for i in 0..len as usize {
        *(*kmp).pattern.add(i) = if flags & TS_IGNORECASE != 0 { toupper(*(pattern as *const U8).add(i) as i32) as U8 } else { *(pattern as *const U8).add(i) };
    }
    conf
}

unsafe fn kmp_get_pattern(conf: *mut TsConfig) -> *mut c_void { (ts_config_priv(conf) as *mut TsKmp).as_ref().unwrap().pattern as *mut c_void }
unsafe fn kmp_get_pattern_len(conf: *mut TsConfig) -> u32 { (*(ts_config_priv(conf) as *mut TsKmp)).pattern_len }

static mut KMP_OPS: TsOps = TsOps {
    name: b"kmp\0".as_ptr(),
    find: kmp_find,
    init: kmp_init,
    get_pattern: kmp_get_pattern,
    get_pattern_len: kmp_get_pattern_len,
    owner: core::ptr::null_mut(),
    list: ListHead { next: core::ptr::null_mut(), prev: core::ptr::null_mut() },
};

unsafe fn init_kmp() -> i32 { textsearch_register(&mut KMP_OPS) }
unsafe fn exit_kmp() { textsearch_unregister(&mut KMP_OPS); }

// MODULE_DESCRIPTION("Knuth-Morris-Pratt text search implementation");
// MODULE_LICENSE("GPL");
// module_init(init_kmp);
// module_exit(exit_kmp);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
