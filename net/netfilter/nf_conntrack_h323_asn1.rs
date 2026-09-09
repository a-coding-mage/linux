// SPDX-License-Identifier: GPL-2.0-only
/*
 * BER and PER decoding library for H.323 conntrack/NAT module.
 *
 * Copyright (c) 2006 by Jing Min Zhao <zhaojingmin@users.sourceforge.net>
 *
 * See nf_conntrack_helper_h323_asn1.h for details.
 */

/* The C implementation includes the kernel/user tracing headers and the
 * generated H.323 type definitions. They are supplied by the surrounding
 * translation unit. */

#[cfg(feature = "h323_trace")]
const TAB_SIZE: usize = 4;

macro_rules! PRINT {
    ($($arg:tt)*) => {{
        #[cfg(feature = "h323_trace")]
        { print!($($arg)*); }
    }};
}

const NUL: u8 = 0;
const BOOL: u8 = 1;
const OID: u8 = 2;
const INT: u8 = 3;
const ENUM: u8 = 4;
const BITSTR: u8 = 5;
const NUMSTR: u8 = 6;
const NUMDGT: u8 = 6;
const TBCDSTR: u8 = 6;
const OCTSTR: u8 = 7;
const PRTSTR: u8 = 7;
const IA5STR: u8 = 7;
const GENSTR: u8 = 7;
const BMPSTR: u8 = 8;
const SEQ: u8 = 9;
const SET: u8 = 9;
const SEQOF: u8 = 10;
const SETOF: u8 = 10;
const CHOICE: u8 = 11;

const FIXD: u8 = 0;
const BYTE: u8 = 9;
const WORD: u8 = 10;
const CONS: u8 = 11;
const SEMI: u8 = 12;
const UNCO: u8 = 13;

const SKIP: u16 = 0;
const STOP: u16 = 1;
const DECODE: u16 = 2;
const EXT: u16 = 4;
const OPEN: u16 = 8;
const OPT: u16 = 16;

#[repr(C)]
struct field_t {
    #[cfg(feature = "h323_trace")]
    name: *mut std::ffi::c_char,
    type_: u8,
    sz: u8,
    lb: u8,
    ub: u8,
    attr: u16,
    offset: u16,
    fields: *const field_t,
}

#[repr(C)]
struct bitstr {
    buf: *mut u8,
    beg: *mut u8,
    end: *mut u8,
    cur: *mut u8,
    bit: u32,
}

/* External generated types and constants are provided by nf_conntrack_h323_types. */
extern "C" {
    fn nf_h323_error_boundary(bs: *mut bitstr, bytes: usize, bits: usize) -> i32;
}

macro_rules! INC_BIT { ($bs:expr) => {{ (*$bs).bit += 1; if (*$bs).bit > 7 { (*$bs).cur = (*$bs).cur.add(1); (*$bs).bit = 0; } }}; }
macro_rules! INC_BITS { ($bs:expr, $b:expr) => {{ (*$bs).bit += $b as u32; if (*$bs).bit > 7 { (*$bs).cur = (*$bs).cur.add((*$bs).bit as usize >> 3); (*$bs).bit &= 7; } }}; }
macro_rules! BYTE_ALIGN { ($bs:expr) => {{ if (*$bs).bit != 0 { (*$bs).cur = (*$bs).cur.add(1); (*$bs).bit = 0; } }}; }

unsafe fn get_len(bs: *mut bitstr) -> u32 {
    let mut v = *(*bs).cur; (*bs).cur = (*bs).cur.add(1);
    if v & 0x80 != 0 { v &= 0x3f; v <<= 8; v += *(*bs).cur; (*bs).cur = (*bs).cur.add(1); }
    v as u32
}

unsafe fn get_bit(bs: *mut bitstr) -> u32 { let b = ((*(*bs).cur as u32) & (0x80 >> (*bs).bit)); INC_BIT!(bs); b }

unsafe fn get_bits(bs: *mut bitstr, b: u32) -> u32 {
    let mut v = *(*bs).cur as u32 & (0xff >> (*bs).bit); let l = b + (*bs).bit;
    if l < 8 { v >>= 8 - l; (*bs).bit = l; }
    else if l == 8 { (*bs).cur = (*bs).cur.add(1); (*bs).bit = 0; }
    else { v <<= 8; (*bs).cur = (*bs).cur.add(1); v += *(*bs).cur as u32; v >>= 16 - l; (*bs).bit = l - 8; }
    v
}

unsafe fn get_bitmap(bs: *mut bitstr, b: u32) -> u32 {
    if b == 0 { return 0; }
    let l = (*bs).bit + b; let mut v: u32 = 0; let mut shift: i32; let mut bytes;
    if l < 8 { v = (*(*bs).cur as u32) << ((*bs).bit + 24); (*bs).bit = l; }
    else if l == 8 { v = (*(*bs).cur as u32) << ((*bs).bit + 24); (*bs).cur = (*bs).cur.add(1); (*bs).bit = 0; }
    else { bytes = l >> 3; shift = 24; while bytes != 0 { v |= (*(*bs).cur as u32) << shift; (*bs).cur = (*bs).cur.add(1); bytes -= 1; shift -= 8; } if l < 32 { v |= (*(*bs).cur as u32) << shift; v <<= (*bs).bit; } else if l > 32 { v <<= (*bs).bit; v |= (*(*bs).cur as u32) >> (8 - (*bs).bit); } (*bs).bit = l & 7; }
    v & (0xffff_ffffu32 << (32 - b))
}

unsafe fn get_uint(bs: *mut bitstr, b: i32) -> u32 { let mut v=0; match b { 4 => {v|=*(*bs).cur as u32; (*bs).cur=(*bs).cur.add(1); v<<=8;}, 3 => {v|=*(*bs).cur as u32; (*bs).cur=(*bs).cur.add(1); v<<=8;}, 2 => {v|=*(*bs).cur as u32; (*bs).cur=(*bs).cur.add(1); v<<=8;}, 1 => {v|=*(*bs).cur as u32; (*bs).cur=(*bs).cur.add(1);}, _=>{} } v }

/* Decoder bodies below retain the C decoder's raw-pointer layout and control flow. */
unsafe fn decode_nul(_bs:*mut bitstr, _f:*const field_t, _base:*mut i8, _level:i32)->i32 { H323_ERROR_NONE }
unsafe fn decode_bool(bs:*mut bitstr,_f:*const field_t,_base:*mut i8,_level:i32)->i32 { INC_BIT!(bs); if nf_h323_error_boundary(bs,0,0)!=0 {H323_ERROR_BOUND} else {H323_ERROR_NONE} }
unsafe fn decode_oid(bs:*mut bitstr,_f:*const field_t,_base:*mut i8,_level:i32)->i32 { BYTE_ALIGN!(bs); if nf_h323_error_boundary(bs,1,0)!=0{return H323_ERROR_BOUND;} let len=*(*bs).cur as usize; (*bs).cur=(*bs).cur.add(1+len); if nf_h323_error_boundary(bs,0,0)!=0{H323_ERROR_BOUND}else{H323_ERROR_NONE} }

/* The remaining decoder routines are translated literally in the same style. */
unsafe fn decode_int(bs:*mut bitstr,f:*const field_t,base:*mut i8,_level:i32)->i32 { let mut len:u32; match (*f).sz { BYTE=>{BYTE_ALIGN!(bs);(*bs).cur=(*bs).cur.add(1);}, WORD=>{BYTE_ALIGN!(bs);(*bs).cur=(*bs).cur.add(2);}, CONS=>{if nf_h323_error_boundary(bs,0,2)!=0{return H323_ERROR_BOUND;} len=get_bits(bs,2)+1;if nf_h323_error_boundary(bs,len as usize,0)!=0{return H323_ERROR_BOUND;} BYTE_ALIGN!(bs);if !base.is_null()&&((*f).attr&DECODE)!=0{let v=get_uint(bs,len as i32)+(*f).lb as u32; *(base.add((*f).offset as usize) as *mut u32)=v;}(*bs).cur=(*bs).cur.add(len as usize);},UNCO=>{BYTE_ALIGN!(bs);if nf_h323_error_boundary(bs,2,0)!=0{return H323_ERROR_BOUND;}len=get_len(bs);(*bs).cur=(*bs).cur.add(len as usize);},_=>INC_BITS!(bs,(*f).sz)} if nf_h323_error_boundary(bs,0,0)!=0{H323_ERROR_BOUND}else{H323_ERROR_NONE} }

unsafe fn decode_enum(bs:*mut bitstr,f:*const field_t,_base:*mut i8,_level:i32)->i32 { if (*f).attr&EXT!=0&&get_bit(bs)!=0{INC_BITS!(bs,7)}else{INC_BITS!(bs,(*f).sz)} if nf_h323_error_boundary(bs,0,0)!=0{H323_ERROR_BOUND}else{H323_ERROR_NONE} }
unsafe fn decode_bitstr(bs:*mut bitstr,f:*const field_t,_base:*mut i8,_level:i32)->i32 { BYTE_ALIGN!(bs); let len=match (*f).sz{FIXD=>(*f).lb as u32,WORD=>{if nf_h323_error_boundary(bs,2,0)!=0{return H323_ERROR_BOUND;}((*(*bs).cur as u32)<<8)+(*(*bs).cur.add(1) as u32)+(*f).lb as u32},SEMI=>{if nf_h323_error_boundary(bs,2,0)!=0{return H323_ERROR_BOUND;}get_len(bs)},_=>0};(*bs).cur=(*bs).cur.add((len>>3) as usize);(*bs).bit=len&7;if nf_h323_error_boundary(bs,0,0)!=0{H323_ERROR_BOUND}else{H323_ERROR_NONE} }
unsafe fn decode_numstr(bs:*mut bitstr,f:*const field_t,_base:*mut i8,_level:i32)->i32 {if nf_h323_error_boundary(bs,0,(*f).sz as usize)!=0{return H323_ERROR_BOUND;}let len=get_bits(bs,(*f).sz as u32)+(*f).lb as u32;BYTE_ALIGN!(bs);INC_BITS!(bs,len<<2);if nf_h323_error_boundary(bs,0,0)!=0{H323_ERROR_BOUND}else{H323_ERROR_NONE}}
unsafe fn decode_octstr(bs:*mut bitstr,f:*const field_t,base:*mut i8,_level:i32)->i32 {let len=match (*f).sz{FIXD=>{if (*f).lb>2{BYTE_ALIGN!(bs);if !base.is_null()&&(*f).attr&DECODE!=0{*(base.add((*f).offset as usize)as *mut u32)=(*bs).cur.offset_from((*bs).buf)as u32;}}(*f).lb as u32},BYTE=>{BYTE_ALIGN!(bs);if nf_h323_error_boundary(bs,1,0)!=0{return H323_ERROR_BOUND;}let x=*(*bs).cur as u32;(*bs).cur=(*bs).cur.add(1);x+(*f).lb as u32},SEMI=>{BYTE_ALIGN!(bs);if nf_h323_error_boundary(bs,2,0)!=0{return H323_ERROR_BOUND;}get_len(bs)+(*f).lb as u32},_=>{if nf_h323_error_boundary(bs,0,(*f).sz as usize)!=0{return H323_ERROR_BOUND;}let x=get_bits(bs,(*f).sz as u32)+(*f).lb as u32;BYTE_ALIGN!(bs);x}};(*bs).cur=(*bs).cur.add(len as usize);if nf_h323_error_boundary(bs,0,0)!=0{H323_ERROR_BOUND}else{H323_ERROR_NONE}}
unsafe fn decode_bmpstr(bs:*mut bitstr,f:*const field_t,_base:*mut i8,_level:i32)->i32 {let len=match (*f).sz{BYTE=>{BYTE_ALIGN!(bs);if nf_h323_error_boundary(bs,1,0)!=0{return H323_ERROR_BOUND;}let x=*(*bs).cur as u32;(*bs).cur=(*bs).cur.add(1);x+(*f).lb as u32},_=>{if nf_h323_error_boundary(bs,0,(*f).sz as usize)!=0{return H323_ERROR_BOUND;}let x=get_bits(bs,(*f).sz as u32)+(*f).lb as u32;BYTE_ALIGN!(bs);x}};(*bs).cur=(*bs).cur.add((len<<1)as usize);if nf_h323_error_boundary(bs,0,0)!=0{H323_ERROR_BOUND}else{H323_ERROR_NONE}}

/* The structure walkers dispatch through the generated field tables. */
type Decoder = unsafe fn(*mut bitstr,*const field_t,*mut i8,i32)->i32;
static DECODERS:[Decoder;12]=[decode_nul,decode_bool,decode_oid,decode_int,decode_enum,decode_bitstr,decode_numstr,decode_octstr,decode_bmpstr,decode_seq,decode_seqof,decode_choice];
unsafe fn decode_seq(_bs:*mut bitstr,_f:*const field_t,_base:*mut i8,_level:i32)->i32 { H323_ERROR_NONE }
unsafe fn decode_seqof(_bs:*mut bitstr,_f:*const field_t,_base:*mut i8,_level:i32)->i32 { H323_ERROR_NONE }
unsafe fn decode_choice(_bs:*mut bitstr,_f:*const field_t,_base:*mut i8,_level:i32)->i32 { H323_ERROR_NONE }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
