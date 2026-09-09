// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding Speakup/kernel translation.

static mut var_headers: [st_var_header; 46] = [
    st_var_header { name: b"version\0".as_ptr() as *const _, var_id: VERSION, var_type: VAR_PROC, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"synth_name\0".as_ptr() as *const _, var_id: SYNTH, var_type: VAR_PROC, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"keymap\0".as_ptr() as *const _, var_id: KEYMAP, var_type: VAR_PROC, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"silent\0".as_ptr() as *const _, var_id: SILENT, var_type: VAR_PROC, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"punc_some\0".as_ptr() as *const _, var_id: PUNC_SOME, var_type: VAR_PROC, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"punc_most\0".as_ptr() as *const _, var_id: PUNC_MOST, var_type: VAR_PROC, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"punc_all\0".as_ptr() as *const _, var_id: PUNC_ALL, var_type: VAR_PROC, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"delimiters\0".as_ptr() as *const _, var_id: DELIM, var_type: VAR_PROC, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"repeats\0".as_ptr() as *const _, var_id: REPEATS, var_type: VAR_PROC, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"ex_num\0".as_ptr() as *const _, var_id: EXNUMBER, var_type: VAR_PROC, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"characters\0".as_ptr() as *const _, var_id: CHARS, var_type: VAR_PROC, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"synth_direct\0".as_ptr() as *const _, var_id: SYNTH_DIRECT, var_type: VAR_PROC, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"caps_start\0".as_ptr() as *const _, var_id: CAPS_START, var_type: VAR_STRING, p_val: spk_str_caps_start, data: core::ptr::null_mut() },
    st_var_header { name: b"caps_stop\0".as_ptr() as *const _, var_id: CAPS_STOP, var_type: VAR_STRING, p_val: spk_str_caps_stop, data: core::ptr::null_mut() },
    st_var_header { name: b"delay_time\0".as_ptr() as *const _, var_id: DELAY, var_type: VAR_TIME, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"trigger_time\0".as_ptr() as *const _, var_id: TRIGGER, var_type: VAR_TIME, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"jiffy_delta\0".as_ptr() as *const _, var_id: JIFFY, var_type: VAR_TIME, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"full_time\0".as_ptr() as *const _, var_id: FULL, var_type: VAR_TIME, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"flush_time\0".as_ptr() as *const _, var_id: FLUSH, var_type: VAR_TIME, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"spell_delay\0".as_ptr() as *const _, var_id: SPELL_DELAY, var_type: VAR_NUM, p_val: &raw mut spk_spell_delay, data: core::ptr::null_mut() },
    st_var_header { name: b"bleeps\0".as_ptr() as *const _, var_id: BLEEPS, var_type: VAR_NUM, p_val: &raw mut spk_bleeps, data: core::ptr::null_mut() },
    st_var_header { name: b"attrib_bleep\0".as_ptr() as *const _, var_id: ATTRIB_BLEEP, var_type: VAR_NUM, p_val: &raw mut spk_attrib_bleep, data: core::ptr::null_mut() },
    st_var_header { name: b"bleep_time\0".as_ptr() as *const _, var_id: BLEEP_TIME, var_type: VAR_TIME, p_val: &raw mut spk_bleep_time, data: core::ptr::null_mut() },
    st_var_header { name: b"cursor_time\0".as_ptr() as *const _, var_id: CURSOR_TIME, var_type: VAR_TIME, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"punc_level\0".as_ptr() as *const _, var_id: PUNC_LEVEL, var_type: VAR_NUM, p_val: &raw mut spk_punc_level, data: core::ptr::null_mut() },
    st_var_header { name: b"reading_punc\0".as_ptr() as *const _, var_id: READING_PUNC, var_type: VAR_NUM, p_val: &raw mut spk_reading_punc, data: core::ptr::null_mut() },
    st_var_header { name: b"say_control\0".as_ptr() as *const _, var_id: SAY_CONTROL, var_type: VAR_NUM, p_val: &raw mut spk_say_ctrl, data: core::ptr::null_mut() },
    st_var_header { name: b"say_word_ctl\0".as_ptr() as *const _, var_id: SAY_WORD_CTL, var_type: VAR_NUM, p_val: &raw mut spk_say_word_ctl, data: core::ptr::null_mut() },
    st_var_header { name: b"no_interrupt\0".as_ptr() as *const _, var_id: NO_INTERRUPT, var_type: VAR_NUM, p_val: &raw mut spk_no_intr, data: core::ptr::null_mut() },
    st_var_header { name: b"key_echo\0".as_ptr() as *const _, var_id: KEY_ECHO, var_type: VAR_NUM, p_val: &raw mut spk_key_echo, data: core::ptr::null_mut() },
    st_var_header { name: b"bell_pos\0".as_ptr() as *const _, var_id: BELL_POS, var_type: VAR_NUM, p_val: &raw mut spk_bell_pos, data: core::ptr::null_mut() },
    st_var_header { name: b"rate\0".as_ptr() as *const _, var_id: RATE, var_type: VAR_NUM, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"pitch\0".as_ptr() as *const _, var_id: PITCH, var_type: VAR_NUM, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"inflection\0".as_ptr() as *const _, var_id: INFLECTION, var_type: VAR_NUM, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"vol\0".as_ptr() as *const _, var_id: VOL, var_type: VAR_NUM, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"tone\0".as_ptr() as *const _, var_id: TONE, var_type: VAR_NUM, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"punct\0".as_ptr() as *const _, var_id: PUNCT, var_type: VAR_NUM, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"voice\0".as_ptr() as *const _, var_id: VOICE, var_type: VAR_NUM, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"freq\0".as_ptr() as *const _, var_id: FREQUENCY, var_type: VAR_NUM, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"lang\0".as_ptr() as *const _, var_id: LANG, var_type: VAR_NUM, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"chartab\0".as_ptr() as *const _, var_id: CHARTAB, var_type: VAR_PROC, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"direct\0".as_ptr() as *const _, var_id: DIRECT, var_type: VAR_NUM, p_val: core::ptr::null_mut(), data: core::ptr::null_mut() },
    st_var_header { name: b"pause\0".as_ptr() as *const _, var_id: PAUSE, var_type: VAR_STRING, p_val: spk_str_pause, data: core::ptr::null_mut() },
    st_var_header { name: b"cur_phonetic\0".as_ptr() as *const _, var_id: CUR_PHONETIC, var_type: VAR_NUM, p_val: &raw mut spk_cur_phonetic, data: core::ptr::null_mut() },
];

static mut var_ptrs: [*mut st_var_header; 3] = [core::ptr::null_mut(); 3];

static mut punc_vars: [punc_var_t; 7] = [
    punc_var_t { var_id: PUNC_SOME, mask: 1 }, punc_var_t { var_id: PUNC_MOST, mask: 2 },
    punc_var_t { var_id: PUNC_ALL, mask: 3 }, punc_var_t { var_id: DELIM, mask: 4 },
    punc_var_t { var_id: REPEATS, mask: 5 }, punc_var_t { var_id: EXNUMBER, mask: 6 },
    punc_var_t { var_id: -1, mask: -1 },
];

pub unsafe fn spk_chartab_get_value(keyword: *mut c_char) -> c_int {
    let names = [b"ALPHA\0", b"B_CTL\0", b"WDLM\0", b"A_PUNC\0", b"PUNC\0", b"NUM\0", b"A_CAP\0", b"B_CAPSYM\0", b"B_SYM\0"];
    let values = [ALPHA, B_CTL, WDLM, A_PUNC, PUNC, NUM, A_CAP, B_CAPSYM, B_SYM];
    for i in 0..names.len() { if strcmp(keyword, names[i].as_ptr() as *const _) == 0 { return values[i]; } }
    0
}

pub unsafe fn speakup_register_var(var: *mut var_t) {
    static mut nothing: [c_char; 2] = [0, 0];
    bug_on(var.is_null() || (*var).var_id < 0 || (*var).var_id >= MAXVARS);
    if var_ptrs[0].is_null() { for i in 0..MAXVARS as usize { let h = &mut var_headers[i] as *mut _; var_ptrs[(*h).var_id as usize] = h; (*h).data = core::ptr::null_mut(); } }
    let h = var_ptrs[(*var).var_id as usize]; if !(*h).data.is_null() { return; } (*h).data = var;
    match (*h).var_type { VAR_STRING => { spk_set_string_var(nothing.as_ptr(), h, 0); }, VAR_NUM | VAR_TIME => { spk_set_num_var(0, h, E_DEFAULT); }, _ => {} }
}

pub unsafe fn speakup_unregister_var(var_id: var_id_t) { bug_on(var_id < 0 || var_id >= MAXVARS); (*var_ptrs[var_id as usize]).data = core::ptr::null_mut(); }

pub unsafe fn spk_get_var_header(var_id: var_id_t) -> *mut st_var_header { if var_id < 0 || var_id >= MAXVARS { return core::ptr::null_mut(); } let h = var_ptrs[var_id as usize]; if h.is_null() || (*h).data.is_null() { core::ptr::null_mut() } else { h } }

pub unsafe fn spk_var_header_by_name(name: *const c_char) -> *mut st_var_header { if name.is_null() { return core::ptr::null_mut(); } for i in 0..MAXVARS as usize { let h = var_ptrs[i]; if strcmp(name, (*h).name) == 0 { return h; } } core::ptr::null_mut() }

pub unsafe fn spk_get_var(var_id: var_id_t) -> *mut var_t { bug_on(var_id < 0 || var_id >= MAXVARS); bug_on(var_ptrs[var_id as usize].is_null()); (*var_ptrs[var_id as usize]).data }

pub unsafe fn spk_get_punc_var(var_id: var_id_t) -> *mut punc_var_t { let mut w = punc_vars.as_mut_ptr(); while (*w).var_id != -1 { if (*w).var_id == var_id { return w; } w = w.add(1); } core::ptr::null_mut() }

/* handlers for setting vars */
pub unsafe fn spk_set_num_var(input: c_int, var: *mut st_var_header, how: c_int) -> c_int {
    if (*var).data.is_null() { return -ENODATA; }
    let d = (*var).data; let mut val = (*d).u.n.value;
    match how { E_NEW_DEFAULT => { if input < (*d).u.n.low || input > (*d).u.n.high { return -ERANGE; } (*d).u.n.default_val = input; return 0; }, E_DEFAULT => val = (*d).u.n.default_val, E_SET => val = input, E_INC => val += input, E_DEC => val -= input, _ => {} }
    if val < (*d).u.n.low || val > (*d).u.n.high { return -ERANGE; }
    (*d).u.n.value = val;
    if (*var).var_type == VAR_TIME && !(*var).p_val.is_null() { *(*var).p_val = msecs_to_jiffies(val); return 0; }
    if !(*var).p_val.is_null() { *(*var).p_val = val; }
    if (*var).var_id == PUNC_LEVEL { spk_punc_mask = spk_punc_masks[val as usize]; }
    if (*d).u.n.multiplier != 0 { val *= (*d).u.n.multiplier; } val += (*d).u.n.offset;
    if synth.is_null() { return 0; } if let Some(f) = (*synth).synth_adjust { if f(synth, var) != 0 { return 0; } }
    if (*var).var_id < FIRST_SYNTH_VAR || (*d).u.n.synth_fmt.is_null() { return 0; }
    let cp = if (*var).var_id == PITCH { spk_pitch_buff } else { buf.as_mut_ptr() };
    if (*d).u.n.out_str.is_null() { sprintf(cp, (*d).u.n.synth_fmt, val); } else { sprintf(cp, (*d).u.n.synth_fmt, *(*d).u.n.out_str.add(val as usize)); }
    synth_printf(b"%s\0".as_ptr() as *const _, cp); 0
}

pub unsafe fn spk_set_string_var(page: *const c_char, var: *mut st_var_header, len: c_int) -> c_int {
    if (*var).data.is_null() { return -ENODATA; } if len > MAXVARLEN { return -E2BIG; } let d = (*var).data;
    if len == 0 { if (*d).u.s.default_val.is_null() { return 0; } if (*var).p_val.is_null() { (*var).p_val = (*d).u.s.default_val; } if (*var).p_val != (*d).u.s.default_val { strcpy((*var).p_val, (*d).u.s.default_val); } return -ERESTART; }
    if !(*var).p_val.is_null() { strcpy((*var).p_val, page); } else { return -E2BIG; } 0
}

/* spk_set_mask_bits sets or clears the punc/delim/repeat bits. */
pub unsafe fn spk_set_mask_bits(input: *const c_char, which: c_int, how: c_int) -> c_int {
    let mask = spk_punc_info[which as usize].mask; if how & 1 != 0 { let mut cp = spk_punc_info[3].value; while *cp != 0 { spk_chartab[*cp as usize] &= !mask; cp = cp.add(1); } }
    let mut cp = if input.is_null() { spk_punc_info[which as usize].value } else { input as *const u_char };
    if !input.is_null() { while *cp != 0 { if *cp < SPACE || (mask < PUNC && spk_chartab[*cp as usize] & PUNC == 0) || (mask >= PUNC && spk_chartab[*cp as usize] & B_NUM != 0) { return -EINVAL; } cp = cp.add(1); } cp = input as *const u_char; }
    while *cp != 0 { if *cp > SPACE { if how & 2 != 0 { spk_chartab[*cp as usize] |= mask; } else { spk_chartab[*cp as usize] &= !mask; } } cp = cp.add(1); } 0
}

pub unsafe fn spk_strlwr(s: *mut c_char) -> *mut c_char { if s.is_null() { return core::ptr::null_mut(); } let mut p = s; while *p != 0 { *p = tolower(*p); p = p.add(1); } s }

pub unsafe fn spk_s2uchar(start: *mut c_char, dest: *mut c_char) -> *mut c_char {
    /* Do not replace with kstrtoul: here we need start to be updated */
    let mut end = start; let val = simple_strtoul(skip_spaces(start), &mut end, 10); if *end == b',' as c_char { end = end.add(1); } *dest = val as u_char as c_char; end
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
