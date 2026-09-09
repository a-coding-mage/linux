// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * lib/ts_fsm.c - A naive finite state machine text search approach
 *
 * Rust translation of the implementation source.  The declarations used
 * below (ts_config, ts_state, ts_fsm_token, ts_ops, and kernel helpers) are
 * supplied by the surrounding textsearch implementation.
 */

use core::ffi::c_void;

#[repr(C)]
pub struct ts_fsm {
    pub ntokens: u32,
    pub tokens: *mut ts_fsm_token,
}

const _A: u16 = 0x100;
const _W: u16 = 0x200;

/* Map to _ctype flags and some magic numbers. */
const token_map: [u16; 15] = [
    0, _W, _C, _L, _U, _P, _S, _D, _D | _X, _U | _L,
    _U | _L | _D, _P | _U | _L | _D | _SP, _P | _U | _L | _D, _A,
    0,
];

/* The kernel ctype lookup table, expressed without C designated initializers. */
const fn make_token_lookup_tbl() -> [u16; 256] {
    let mut t = [_W; 256];
    let mut i = 0usize;
    while i < 128 {
        t[i] = _W | _A | if i < 32 || i == 127 { _C } else { 0 };
        if i == 9 || (i >= 10 && i <= 13) || i == 32 { t[i] |= _S; }
        if i == 32 { t[i] |= _SP; }
        if i >= 33 && i <= 47 || i >= 58 && i <= 64 || i >= 91 && i <= 96 || i >= 123 { t[i] |= _P; }
        if i >= 48 && i <= 57 { t[i] |= _D; }
        if i >= 65 && i <= 90 { t[i] |= _U; }
        if i >= 97 && i <= 122 { t[i] |= _L; }
        if i >= 65 && i <= 70 || i >= 97 && i <= 102 { t[i] |= _X; }
        i += 1;
    }
    while i < 256 { t[i] = _W | if i < 192 { _P } else if i < 224 { _U } else { _L }; i += 1; }
    t
}
const token_lookup_tbl: [u16; 256] = make_token_lookup_tbl();

#[inline]
unsafe fn match_token(t: *mut ts_fsm_token, d: u8) -> bool {
    if (*t).type_ != 0 {
        (token_lookup_tbl[d as usize] & (*t).type_) != 0
    } else { (*t).value == d }
}

unsafe fn fsm_find(conf: *mut ts_config, state: *mut ts_state) -> u32 {
    let fsm = ts_config_priv(conf) as *mut ts_fsm;
    let mut cur: *mut ts_fsm_token = core::ptr::null_mut();
    let mut next: *mut ts_fsm_token;
    let (mut match_start, mut block_idx, mut block_len) = (0u32, 0u32, 0u32);
    let mut strict: u32;
    let mut consumed = (*state).offset;
    let mut data: *const u8 = core::ptr::null();

    macro_rules! get_next_block { () => {{ consumed += block_idx; block_idx = 0; block_len = (*conf).get_next_block(consumed, &mut data, conf, state); }}; }
    macro_rules! end_of_data { () => {{ if block_idx >= block_len { get_next_block!(); } block_idx >= block_len }}; }
    macro_rules! token_mismatch { () => {{ if strict != 0 { return u32::MAX; } block_idx += 1; continue 'startover; }}; }

    if end_of_data!() { return u32::MAX; }
    strict = ((*fsm).tokens).as_ref().unwrap().recur != TS_FSM_HEAD_IGNORE as _ as u32 as _;

    'startover: loop {
        match_start = consumed + block_idx;
        let mut tok_idx = 0u32;
        while tok_idx < (*fsm).ntokens {
            cur = (*fsm).tokens.add(tok_idx as usize);
            next = if tok_idx < (*fsm).ntokens - 1 { cur.add(1) } else { core::ptr::null_mut() };
            match (*cur).recur {
                TS_FSM_SINGLE => { if end_of_data!() { return u32::MAX; } if !match_token(cur, *data.add(block_idx as usize)) { token_mismatch!(); } }
                TS_FSM_PERHAPS => { if end_of_data!() || !match_token(cur, *data.add(block_idx as usize)) { tok_idx += 1; continue; } }
                TS_FSM_MULTI => { if end_of_data!() { return u32::MAX; } if !match_token(cur, *data.add(block_idx as usize)) { token_mismatch!(); } block_idx += 1; }
                TS_FSM_ANY => { if next.is_null() { (*state).offset = consumed + block_idx; return match_start; } if end_of_data!() { tok_idx += 1; continue; } while !match_token(next, *data.add(block_idx as usize)) { if !match_token(cur, *data.add(block_idx as usize)) { token_mismatch!(); } block_idx += 1; if end_of_data!() { return u32::MAX; } } }
                TS_FSM_HEAD_IGNORE => { if end_of_data!() { tok_idx += 1; continue; } while !match_token(next, *data.add(block_idx as usize)) { if !match_token(cur, *data.add(block_idx as usize)) { return u32::MAX; } block_idx += 1; if end_of_data!() { return u32::MAX; } } match_start = consumed + block_idx; }
                _ => {}
            }
            block_idx += 1; tok_idx += 1;
        }
        if end_of_data!() { (*state).offset = consumed + block_idx; return match_start; }
        return u32::MAX;
    }
}

unsafe fn fsm_init(pattern: *const c_void, len: u32, gfp_mask: gfp_t, flags: i32) -> *mut ts_config {
    let tokens = pattern as *mut ts_fsm_token; let ntokens = len / core::mem::size_of::<ts_fsm_token>() as u32;
    if len % core::mem::size_of::<ts_fsm_token>() as u32 != 0 || ntokens < 1 || flags & TS_IGNORECASE != 0 { return ERR_PTR(-EINVAL); }
    for i in 0..ntokens { let t = tokens.add(i as usize); if (*t).type_ > TS_FSM_TYPE_MAX || (*t).recur > TS_FSM_RECUR_MAX || ((*t).recur == TS_FSM_HEAD_IGNORE && (i != 0 || i == ntokens - 1)) { return ERR_PTR(-EINVAL); } }
    let conf = alloc_ts_config(core::mem::size_of::<ts_fsm>() as u32 + len, gfp_mask); if IS_ERR(conf) { return conf; }
    (*conf).flags = flags; let fsm = ts_config_priv(conf) as *mut ts_fsm; (*fsm).ntokens = ntokens; core::ptr::copy_nonoverlapping(pattern as *const u8, (*fsm).tokens as *mut u8, len as usize);
    for i in 0..ntokens { let t = (*fsm).tokens.add(i as usize); (*t).type_ = token_map[(*t).type_ as usize]; } conf
}

unsafe fn fsm_get_pattern(conf: *mut ts_config) -> *mut c_void { (ts_config_priv(conf) as *mut ts_fsm).as_ref().unwrap().tokens as *mut c_void }
unsafe fn fsm_get_pattern_len(conf: *mut ts_config) -> u32 { (ts_config_priv(conf) as *mut ts_fsm).as_ref().unwrap().ntokens * core::mem::size_of::<ts_fsm_token>() as u32 }

/* Registration and module metadata are supplied by the kernel integration. */
unsafe fn init_fsm() -> i32 { textsearch_register(&mut fsm_ops) }
unsafe fn exit_fsm() { textsearch_unregister(&mut fsm_ops); }

static mut fsm_ops: ts_ops = ts_ops {
    name: b"fsm\0".as_ptr() as *const i8,
    find: Some(fsm_find),
    init: Some(fsm_init),
    get_pattern: Some(fsm_get_pattern),
    get_pattern_len: Some(fsm_get_pattern_len),
    owner: THIS_MODULE,
    list: LIST_HEAD_INIT,
};

/* MODULE_DESCRIPTION("naive finite state machine text search"); */
/* MODULE_LICENSE("GPL"); */
/* module_init(init_fsm); */
/* module_exit(exit_fsm); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
