// SPDX-License-Identifier: GPL-2.0-only
/* LZO1X Compressor from LZO; translated from lzo1x_compress.c. */

// Symbols supplied by the LZO/kernel translation unit are intentionally external.
extern "C" {
    fn get_unaligned_le32(p: *const u8) -> u32;
    fn put_unaligned_le32(v: u32, p: *mut u8);
    fn memset(p: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
}

// Build-time constants and types are supplied by lzodefs.h/linux/lzo.h.
type lzo_dict_t = u16;
extern "C" {
    static LZO_E_OK: i32;
    static LZO_E_OUTPUT_OVERRUN: i32;
    static LZO_VERSION: u8;
}

const LZO_UNSAFE: i32 = 1;

#[inline(always)] unsafe fn need_op(op: *mut u8, end: *mut u8, n: usize) -> bool {
    op.add(n) <= end
}

unsafe fn lzo1x_1_do_compress(
    input: *const u8, in_len: usize, out: *mut *mut u8, op_end: *mut u8,
    tp: *mut usize, wrkmem: *mut core::ffi::c_void, state_offset: *mut i8,
    bitstream_version: u8,
) -> i32 {
    let in_end = input.add(in_len);
    let ip_end = input.add(in_len - 20);
    let dict = wrkmem as *mut lzo_dict_t;
    let mut ti = *tp;
    let mut op = *out;
    let mut ip = input;
    let mut ii = ip;
    ip = ip.add(if ti < 4 { 4 - ti } else { 0 });

    'outer: loop {
        let mut m_pos: *const u8 = core::ptr::null();
        let mut t: usize;
        let mut m_len: usize;
        let mut m_off: usize;
        let mut run_length: usize = 0;
        'literal: {
            ip = ip.add(1 + ((ip.offset_from(ii) as usize) >> 5));
            if ip >= ip_end { break 'outer; }
            let dv = get_unaligned_le32(ip);
            if dv == 0 && bitstream_version != 0 {
                let mut ir = ip.add(4);
                let limit = core::cmp::min(ip_end, ip.add(MAX_ZERO_RUN_LENGTH + 1));
                while ir < limit && *ir == 0 { ir = ir.add(1); }
                run_length = ir.offset_from(ip) as usize;
                if run_length > MAX_ZERO_RUN_LENGTH { run_length = MAX_ZERO_RUN_LENGTH; }
            } else {
                t = (((dv.wrapping_mul(0x1824429d)) >> (32 - D_BITS)) & D_MASK) as usize;
                m_pos = input.add(*dict.add(t) as usize);
                *dict.add(t) = ip.offset_from(input) as lzo_dict_t;
                if dv != get_unaligned_le32(m_pos) { continue 'literal; }
            }
        }
        ii = ii.sub(ti); ti = 0; t = ip.offset_from(ii) as usize;
        if t != 0 {
            if t <= 3 { *op.add(*state_offset as usize) |= t as u8; if !need_op(op, op_end, 4) { return LZO_E_OUTPUT_OVERRUN; } core::ptr::copy_nonoverlapping(ii, op, 4); op = op.add(t); }
            else if t <= 16 { if !need_op(op, op_end, 17) { return LZO_E_OUTPUT_OVERRUN; } *op = (t - 3) as u8; op = op.add(1); core::ptr::copy_nonoverlapping(ii, op, 8); core::ptr::copy_nonoverlapping(ii.add(8), op.add(8), 8); op = op.add(t); }
            else { if t <= 18 { if !need_op(op, op_end, 1) { return LZO_E_OUTPUT_OVERRUN; } *op = (t-3) as u8; op=op.add(1); } else { let mut tt=t-18; if !need_op(op,op_end,1){return LZO_E_OUTPUT_OVERRUN;} *op=0;op=op.add(1); while tt>255 {tt-=255;if !need_op(op,op_end,1){return LZO_E_OUTPUT_OVERRUN;}*op=0;op=op.add(1);}if !need_op(op,op_end,1){return LZO_E_OUTPUT_OVERRUN;}*op=tt as u8;op=op.add(1);} if !need_op(op,op_end,t){return LZO_E_OUTPUT_OVERRUN;} while t>=16 {core::ptr::copy_nonoverlapping(ii,op,8);core::ptr::copy_nonoverlapping(ii.add(8),op.add(8),8);op=op.add(16);ii=ii.add(16);t-=16;}while t>0{*op=*ii;op=op.add(1);ii=ii.add(1);t-=1;} }
        }
        if run_length != 0 { ip=ip.add(run_length); run_length-=MIN_ZERO_RUN_LENGTH; if !need_op(op,op_end,4){return LZO_E_OUTPUT_OVERRUN;} put_unaligned_le32(((run_length as u32)<<21)|0xfffc18|((run_length as u32)&7),op);op=op.add(4);*state_offset=-3;ii=ip;continue; }
        m_len=4; while ip.add(m_len) < ip_end && *ip.add(m_len)==*m_pos.add(m_len){m_len+=1;} m_off=ip.offset_from(m_pos) as usize; ip=ip.add(m_len);
        if m_len<=M2_MAX_LEN && m_off<=M2_MAX_OFFSET {m_off-=1;if !need_op(op,op_end,2){return LZO_E_OUTPUT_OVERRUN;}*op=(((m_len-1)<<5)|((m_off&7)<<2))as u8;*op.add(1)=(m_off>>3)as u8;op=op.add(2);}
        else if m_off<=M3_MAX_OFFSET {m_off-=1;if !need_op(op,op_end,1){return LZO_E_OUTPUT_OVERRUN;}if m_len<=M3_MAX_LEN{*op=(M3_MARKER|(m_len-2))as u8;op=op.add(1);}else{m_len-=M3_MAX_LEN;*op=M3_MARKER as u8;op=op.add(1);while m_len>255{m_len-=255;if !need_op(op,op_end,1){return LZO_E_OUTPUT_OVERRUN;}*op=0;op=op.add(1);}if !need_op(op,op_end,1){return LZO_E_OUTPUT_OVERRUN;}*op=m_len as u8;op=op.add(1);}if !need_op(op,op_end,2){return LZO_E_OUTPUT_OVERRUN;}*op=(m_off<<2)as u8;*op.add(1)=(m_off>>6)as u8;op=op.add(2);}
        else {m_off-=0x4000;if !need_op(op,op_end,1){return LZO_E_OUTPUT_OVERRUN;}if m_len<=M4_MAX_LEN{*op=(M4_MARKER|((m_off>>11)&8)|(m_len-2))as u8;op=op.add(1);}else{m_len-=M4_MAX_LEN;*op=(M4_MARKER|((m_off>>11)&8))as u8;op=op.add(1);while m_len>255{m_len-=255;if !need_op(op,op_end,1){return LZO_E_OUTPUT_OVERRUN;}*op=0;op=op.add(1);}if !need_op(op,op_end,1){return LZO_E_OUTPUT_OVERRUN;}*op=m_len as u8;op=op.add(1);}if !need_op(op,op_end,2){return LZO_E_OUTPUT_OVERRUN;}*op=(m_off<<2)as u8;*op.add(1)=(m_off>>6)as u8;op=op.add(2);}
        *state_offset=-2; ii=ip;
    }
    *out=op;*tp=in_end.offset_from(ii) as usize-ti;LZO_E_OK
}

extern "C" {
    static D_SIZE: usize; static D_BITS: u32; static D_MASK: u32;
    static MAX_ZERO_RUN_LENGTH: usize; static MIN_ZERO_RUN_LENGTH: usize;
    static M2_MAX_LEN: usize; static M2_MAX_OFFSET: usize; static M3_MAX_LEN: usize;
    static M3_MAX_OFFSET: usize; static M4_MAX_LEN: usize; static M4_MARKER: usize;
    static M3_MARKER: usize; static M4_MAX_OFFSET_V1: usize; static M4_MAX_OFFSET_V0: usize;
}

#[no_mangle] pub unsafe extern "C" fn lzo1x_1_compress(i:*const u8,n:usize,o:*mut u8,l:*mut usize,w:*mut core::ffi::c_void)->i32{ lzogeneric1x_1_compress(i,n,o,l,w,0) }
#[no_mangle] pub unsafe extern "C" fn lzorle1x_1_compress(i:*const u8,n:usize,o:*mut u8,l:*mut usize,w:*mut core::ffi::c_void)->i32{ lzogeneric1x_1_compress(i,n,o,l,w,LZO_VERSION) }

unsafe fn lzogeneric1x_1_compress(i:*const u8,n:usize,o:*mut u8,l:*mut usize,w:*mut core::ffi::c_void,v:u8)->i32 { let mut op=o; if v!=0{*op=17;*op.add(1)=v;op=op.add(2);} let start=op;let mut t=0usize;let mut state=-2i8;let mut ip=i;let mut left=n;while left>20{let ll=core::cmp::min(left,M4_MAX_OFFSET_V1+1);memset(w,0,D_SIZE*core::mem::size_of::<lzo_dict_t>());let e=lzo1x_1_do_compress(ip,ll,&mut op,o.add(*l),&mut t,w,&mut state,v);if e!=LZO_E_OK{return e;}ip=ip.add(ll);left-=ll;}t+=left;if t>0{if !need_op(op,o.add(*l),t+1){return LZO_E_OUTPUT_OVERRUN;}*op=(17+t)as u8;op=op.add(1);core::ptr::copy_nonoverlapping(i.add(n-t),op,t);op=op.add(t);}if !need_op(op,o.add(*l),3){return LZO_E_OUTPUT_OVERRUN;}*op=(M4_MARKER|1)as u8;*op.add(1)=0;*op.add(2)=0;*l=op.offset_from(o)as usize;LZO_E_OK }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
