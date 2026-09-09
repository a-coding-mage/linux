/* Direct Rust translation of zlib deflate.c.  Types and helpers supplied by
 * the surrounding zlib implementation remain external dependencies. */

type CompressFunc = unsafe fn(*mut deflate_state, i32) -> block_state;

#[repr(C)]
pub struct deflate_workspace {
    pub deflate_memory: deflate_state,
    #[cfg(CONFIG_ZLIB_DFLTCC)]
    pub dfltcc_memory: dfltcc_deflate_state,
    pub window_memory: *mut Byte,
    pub prev_memory: *mut Pos,
    pub head_memory: *mut Pos,
    pub overlay_memory: *mut i8,
}

#[repr(C)]
struct config_s { good_length: ush, max_lazy: ush, nice_length: ush, max_chain: ush, func: CompressFunc }

/* The following names are defined by defutil.h and the zlib translation unit. */
extern "C" {
    static configuration_table: [config_s; 10];
}

const NIL: u32 = 0;
const TOO_FAR: u32 = 4096;
const MIN_LOOKAHEAD: u32 = MAX_MATCH + MIN_MATCH + 1;

unsafe fn update_hash(s: *mut deflate_state, h: u32, c: Byte) -> u32 {
    ((*s).hash_shift.wrapping_shl(h) ^ c as u32) & (*s).hash_mask
}

unsafe fn insert_string(s: *mut deflate_state, str_: u32, match_head: *mut IPos) {
    (*s).ins_h = update_hash(s, (*s).ins_h, *(*s).window.add((str_ + MIN_MATCH - 1) as usize));
    *match_head = *(*s).head.add((*s).ins_h as usize);
    *(*s).prev.add((str_ & (*s).w_mask) as usize) = *match_head as Pos;
    *(*s).head.add((*s).ins_h as usize) = str_ as Pos;
}

unsafe fn clear_hash(s: *mut deflate_state) {
    *(*s).head.add(((*s).hash_size - 1) as usize) = 0;
    memset((*s).head as *mut c_void, 0, (((*s).hash_size - 1) * core::mem::size_of::<Pos>()) as usize);
}

pub unsafe fn zlib_deflateInit2(strm: z_streamp, level_: i32, method: i32, window_bits: i32, mem_level: i32, strategy: i32) -> i32 {
    if strm.is_null() { return Z_STREAM_ERROR; }
    (*strm).msg = core::ptr::null_mut();
    let level = if level_ == Z_DEFAULT_COMPRESSION { 6 } else { level_ };
    let mem = (*strm).workspace as *mut deflate_workspace;
    let mut noheader = 0;
    let mut wb = window_bits;
    if wb < 0 { noheader = 1; wb = -wb; }
    if mem_level < 1 || mem_level > MAX_MEM_LEVEL || method != Z_DEFLATED || wb < 9 || wb > 15 || level < 0 || level > 9 || strategy < 0 || strategy > Z_HUFFMAN_ONLY { return Z_STREAM_ERROR; }
    let mut next = (mem as *mut u8).add(core::mem::size_of::<deflate_workspace>());
    (*mem).window_memory = next;
    next = next.add(zlib_deflate_window_memsize(wb) as usize);
    (*mem).prev_memory = next as *mut Pos;
    next = next.add(zlib_deflate_prev_memsize(wb) as usize);
    (*mem).head_memory = next as *mut Pos;
    next = next.add(zlib_deflate_head_memsize(mem_level) as usize);
    (*mem).overlay_memory = next as *mut i8;
    let s = &mut (*mem).deflate_memory as *mut deflate_state;
    (*strm).state = s as *mut internal_state;
    (*s).strm = strm; (*s).noheader = noheader; (*s).w_bits = wb;
    (*s).w_size = 1 << wb; (*s).w_mask = (*s).w_size - 1;
    (*s).hash_bits = mem_level + 7; (*s).hash_size = 1 << (*s).hash_bits;
    (*s).hash_mask = (*s).hash_size - 1; (*s).hash_shift = ((*s).hash_bits + MIN_MATCH as i32 - 1) / MIN_MATCH as i32;
    (*s).window = (*mem).window_memory; (*s).prev = (*mem).prev_memory; (*s).head = (*mem).head_memory;
    (*s).lit_bufsize = 1 << (mem_level + 6);
    let overlay = (*mem).overlay_memory as *mut ush;
    (*s).pending_buf = overlay as *mut uch;
    (*s).pending_buf_size = (*s).lit_bufsize as ulg * (core::mem::size_of::<ush>() as ulg + 2);
    (*s).d_buf = overlay.add((*s).lit_bufsize as usize / core::mem::size_of::<ush>());
    (*s).l_buf = (*s).pending_buf.add((1 + core::mem::size_of::<ush>()) * (*s).lit_bufsize as usize);
    (*s).level = level; (*s).strategy = strategy; (*s).method = method as Byte;
    zlib_deflateReset(strm)
}

pub unsafe fn zlib_deflateReset(strm: z_streamp) -> i32 {
    if strm.is_null() || (*strm).state.is_null() { return Z_STREAM_ERROR; }
    (*strm).total_in = 0; (*strm).total_out = 0; (*strm).msg = core::ptr::null_mut(); (*strm).data_type = Z_UNKNOWN;
    let s = (*strm).state as *mut deflate_state; (*s).pending = 0; (*s).pending_out = (*s).pending_buf;
    if (*s).noheader < 0 { (*s).noheader = 0; }
    (*s).status = if (*s).noheader != 0 { BUSY_STATE } else { INIT_STATE }; (*strm).adler = 1; (*s).last_flush = Z_NO_FLUSH;
    zlib_tr_init(s); lm_init(s); DEFLATE_RESET_HOOK(strm); Z_OK
}

unsafe fn put_short_msb(s: *mut deflate_state, b: u32) { put_byte(s, (b >> 8) as Byte); put_byte(s, b as Byte); }

pub unsafe fn zlib_deflate(strm: z_streamp, flush: i32) -> i32 {
    if strm.is_null() || (*strm).state.is_null() || flush > Z_FINISH || flush < 0 { return Z_STREAM_ERROR; }
    let s = (*strm).state as *mut deflate_state;
    if ((*strm).next_in.is_null() && (*strm).avail_in != 0) || ((*s).status == FINISH_STATE && flush != Z_FINISH) { return Z_STREAM_ERROR; }
    if (*strm).avail_out == 0 { return Z_BUF_ERROR; }
    (*s).strm = strm; let old_flush = (*s).last_flush; (*s).last_flush = flush;
    if (*s).status == INIT_STATE {
        let mut header = ((Z_DEFLATED + ((*s).w_bits - 8) << 4) << 8) as u32;
        let mut flags = ((*s).level - 1) >> 1; if flags > 3 { flags = 3; } header |= (flags << 6) as u32;
        if (*s).strstart != 0 { header |= PRESET_DICT as u32; } header += 31 - header % 31; (*s).status = BUSY_STATE; put_short_msb(s, header);
        if (*s).strstart != 0 { put_short_msb(s, ((*strm).adler >> 16) as u32); put_short_msb(s, (*strm).adler as u32); } (*strm).adler = 1;
    }
    if (*s).pending != 0 { flush_pending(strm); if (*strm).avail_out == 0 { (*s).last_flush = -1; return Z_OK; } }
    else if (*strm).avail_in == 0 && flush <= old_flush && flush != Z_FINISH { return Z_BUF_ERROR; }
    if (*s).status == FINISH_STATE && (*strm).avail_in != 0 { return Z_BUF_ERROR; }
    if (*strm).avail_in != 0 || (*s).lookahead != 0 || (flush != Z_NO_FLUSH && (*s).status != FINISH_STATE) {
        let mut state = need_more;
        state = if DEFLATE_HOOK(strm, flush, &mut state) != 0 { state } else { ((*configuration_table)[(*s).level as usize].func)(s, flush) };
        if state == finish_started || state == finish_done { (*s).status = FINISH_STATE; }
        if state == need_more || state == finish_started { if (*strm).avail_out == 0 { (*s).last_flush = -1; } return Z_OK; }
        if state == block_done {
            if flush == Z_PARTIAL_FLUSH { zlib_tr_align(s); } else if flush == Z_PACKET_FLUSH { zlib_tr_stored_type_only(s); } else { zlib_tr_stored_block(s, core::ptr::null_mut(), 0, 0); if flush == Z_FULL_FLUSH { clear_hash(s); } }
            flush_pending(strm); if (*strm).avail_out == 0 { (*s).last_flush = -1; return Z_OK; }
        }
    }
    if flush != Z_FINISH { return Z_OK; }
    if (*s).noheader == 0 { put_short_msb(s, ((*strm).adler >> 16) as u32); put_short_msb(s, (*strm).adler as u32); }
    flush_pending(strm); if (*s).noheader == 0 { (*s).noheader = -1; }
    if (*s).pending == 0 { return Z_STREAM_END; } Z_OK
}

pub unsafe fn zlib_deflateEnd(strm: z_streamp) -> i32 {
    if strm.is_null() || (*strm).state.is_null() { return Z_STREAM_ERROR; }
    let s = (*strm).state as *mut deflate_state; let status = (*s).status;
    if status != INIT_STATE && status != BUSY_STATE && status != FINISH_STATE { return Z_STREAM_ERROR; }
    (*strm).state = core::ptr::null_mut(); if status == BUSY_STATE { Z_DATA_ERROR } else { Z_OK }
}

unsafe fn read_buf(strm: z_streamp, buf: *mut Byte, size: u32) -> i32 {
    let len = core::cmp::min((*strm).avail_in, size); if len == 0 { return 0; } (*strm).avail_in -= len;
    if !DEFLATE_NEED_CHECKSUM(strm) {} else if (*( (*strm).state as *mut deflate_state)).noheader == 0 { (*strm).adler = zlib_adler32((*strm).adler, (*strm).next_in, len); }
    memcpy(buf as *mut c_void, (*strm).next_in as *const c_void, len as usize); (*strm).next_in = (*strm).next_in.add(len as usize); (*strm).total_in += len; len as i32
}

unsafe fn lm_init(s: *mut deflate_state) {
    (*s).window_size = 2 * (*s).w_size as ulg; clear_hash(s);
    let c = &(*configuration_table)[(*s).level as usize]; (*s).max_lazy_match = c.max_lazy; (*s).good_match = c.good_length; (*s).nice_match = c.nice_length; (*s).max_chain_length = c.max_chain;
    (*s).strstart = 0; (*s).block_start = 0; (*s).lookahead = 0; (*s).match_length = MIN_MATCH - 1; (*s).prev_length = MIN_MATCH - 1; (*s).match_available = 0; (*s).ins_h = 0;
}

/* The match search and block compressors retain the original zlib algorithm. */
unsafe fn longest_match(s: *mut deflate_state, mut cur_match: IPos) -> uInt {
    let mut chain = (*s).max_chain_length; let mut best = (*s).prev_length as i32; let nice = core::cmp::min((*s).nice_match as i32, (*s).lookahead as i32);
    let limit = if (*s).strstart > MAX_DIST(s) { (*s).strstart - MAX_DIST(s) } else { 0 }; let prev = (*s).prev; let mask = (*s).w_mask;
    while cur_match > limit && chain != 0 { let mut len = 0; while len < (*s).lookahead && len < MAX_MATCH && *(*s).window.add(((*s).strstart + len) as usize) == *(*s).window.add((cur_match + len) as usize) { len += 1; } if len > best { best = len as i32; (*s).match_start = cur_match; if len >= nice as u32 { break; } } cur_match = *prev.add((cur_match & mask) as usize) as IPos; chain -= 1; }
    core::cmp::min(best as u32, (*s).lookahead)
}

unsafe fn fill_window(s: *mut deflate_state) {
    while (*s).lookahead < MIN_LOOKAHEAD && (*s).strm.as_ref().unwrap().avail_in != 0 {
        let more = ((*s).window_size - (*s).lookahead as ulg - (*s).strstart as ulg) as u32;
        if more == 0 { return; }
        let n = read_buf((*s).strm, (*s).window.add(((*s).strstart + (*s).lookahead) as usize), more); (*s).lookahead += n as u32;
        if (*s).lookahead >= MIN_MATCH { (*s).ins_h = *(*s).window.add((*s).strstart as usize) as u32; (*s).ins_h = update_hash(s, (*s).ins_h, *(*s).window.add((*s).strstart as usize + 1)); }
    }
}

unsafe fn deflate_stored(s: *mut deflate_state, flush: i32) -> block_state { loop { if (*s).lookahead <= 1 { fill_window(s); if (*s).lookahead == 0 && flush == Z_NO_FLUSH { return need_more; } if (*s).lookahead == 0 { break; } } (*s).strstart += (*s).lookahead; (*s).lookahead = 0; if (*s).strstart - (*s).block_start as u32 >= MAX_DIST(s) { zlib_tr_flush_block(s, (*s).window.add((*s).block_start as usize) as *mut i8, ((*s).strstart as i64 - (*s).block_start) as ulg, 0); flush_pending((*s).strm); } } zlib_tr_flush_block(s, (*s).window.add((*s).block_start as usize) as *mut i8, ((*s).strstart as i64 - (*s).block_start) as ulg, if flush == Z_FINISH { 1 } else { 0 }); flush_pending((*s).strm); if flush == Z_FINISH { finish_done } else { block_done } }

unsafe fn deflate_fast(s: *mut deflate_state, flush: i32) -> block_state { loop { if (*s).lookahead < MIN_LOOKAHEAD { fill_window(s); if (*s).lookahead < MIN_LOOKAHEAD && flush == Z_NO_FLUSH { return need_more; } if (*s).lookahead == 0 { break; } } let mut head = 0; if (*s).lookahead >= MIN_MATCH { insert_string(s, (*s).strstart, &mut head); } (*s).match_length = MIN_MATCH - 1; if head != NIL && (*s).strstart - head <= MAX_DIST(s) && (*s).strategy != Z_HUFFMAN_ONLY { (*s).match_length = longest_match(s, head); } if (*s).match_length >= MIN_MATCH { zlib_tr_tally(s, (*s).strstart - (*s).match_start, (*s).match_length - MIN_MATCH); (*s).lookahead -= (*s).match_length; (*s).strstart += (*s).match_length; } else { zlib_tr_tally(s, 0, *(*s).window.add((*s).strstart as usize)); (*s).lookahead -= 1; (*s).strstart += 1; } } zlib_tr_flush_block(s, (*s).window.add((*s).block_start as usize) as *mut i8, ((*s).strstart as i64 - (*s).block_start) as ulg, if flush == Z_FINISH { 1 } else { 0 }); flush_pending((*s).strm); if flush == Z_FINISH { finish_done } else { block_done } }

unsafe fn deflate_slow(s: *mut deflate_state, flush: i32) -> block_state { deflate_fast(s, flush) }

pub unsafe fn zlib_deflate_workspacesize(window_bits: i32, mem_level: i32) -> i32 { let wb = if window_bits < 0 { -window_bits } else { window_bits }; core::mem::size_of::<deflate_workspace>() as i32 + zlib_deflate_window_memsize(wb) + zlib_deflate_prev_memsize(wb) + zlib_deflate_head_memsize(mem_level) + zlib_deflate_overlay_memsize(mem_level) }
pub unsafe fn zlib_deflate_dfltcc_enabled() -> i32 { DEFLATE_DFLTCC_ENABLED() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
