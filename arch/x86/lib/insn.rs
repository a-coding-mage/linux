// SPDX-License-Identifier: GPL-2.0-or-later
/* x86 instruction analysis */

// Linux headers and architecture helpers are supplied by the surrounding crate.

extern "C" {
    fn inat_get_opcode_attribute(b: insn_byte_t) -> insn_attr_t;
    fn inat_is_legacy_prefix(a: insn_attr_t) -> bool;
    fn inat_is_address_size_prefix(a: insn_attr_t) -> bool;
    fn inat_is_operand_size_prefix(a: insn_attr_t) -> bool;
    fn inat_is_rex_prefix(a: insn_attr_t) -> bool;
    fn inat_is_rex2_prefix(a: insn_attr_t) -> bool;
    fn inat_is_vex_prefix(a: insn_attr_t) -> bool;
    fn inat_is_xop_prefix(a: insn_attr_t) -> bool;
    fn inat_is_evex_prefix(a: insn_attr_t) -> bool;
    fn inat_is_vex3_prefix(a: insn_attr_t) -> bool;
    fn inat_accept_xop(a: insn_attr_t) -> bool;
    fn inat_get_xop_attribute(op: insn_byte_t, m: insn_byte_t) -> insn_attr_t;
    fn inat_get_avx_attribute(op: insn_byte_t, m: insn_byte_t, p: insn_byte_t) -> insn_attr_t;
    fn inat_evex_scalable(a: insn_attr_t) -> bool;
    fn inat_must_evex(a: insn_attr_t) -> bool;
    fn inat_accept_vex(a: insn_attr_t) -> bool;
    fn inat_is_group(a: insn_attr_t) -> bool;
    fn inat_must_vex(a: insn_attr_t) -> bool;
    fn inat_is_invalid64(a: insn_attr_t) -> bool;
    fn inat_is_escape(a: insn_attr_t) -> bool;
    fn inat_get_escape_attribute(op: insn_byte_t, pfx: insn_byte_t, a: insn_attr_t) -> insn_attr_t;
    fn inat_has_modrm(a: insn_attr_t) -> bool;
    fn inat_get_group_attribute(m: insn_byte_t, p: insn_byte_t, a: insn_attr_t) -> insn_attr_t;
    fn inat_is_force64(a: insn_attr_t) -> bool;
    fn inat_has_moffset(a: insn_attr_t) -> bool;
    fn inat_has_immediate(a: insn_attr_t) -> bool;
    fn inat_immediate_size(a: insn_attr_t) -> i32;
    fn inat_has_second_immediate(a: insn_attr_t) -> bool;
    fn insn_set_byte(f: *mut insn_field, n: i32, b: insn_byte_t);
    fn insn_field_set(f: *mut insn_field, v: i64, n: i32);
    fn insn_is_avx_or_xop(i: *const insn) -> bool;
    fn avx_insn_is_xop(i: *const insn) -> bool;
    fn insn_xop_map_bits(i: *const insn) -> insn_byte_t;
    fn insn_vex_m_bits(i: *const insn) -> insn_byte_t;
    fn insn_vex_p_bits(i: *const insn) -> insn_byte_t;
    fn insn_vex_w_bit(i: *const insn) -> bool;
    fn insn_is_evex(i: *const insn) -> bool;
    fn insn_is_rex2(i: *const insn) -> bool;
    fn insn_rex2_m_bit(i: *const insn) -> bool;
    fn insn_last_prefix_id(i: *const insn) -> insn_byte_t;
}

// These declarations mirror the included architecture headers.
type insn_byte_t = u8;
type insn_attr_t = u32;

#[repr(C)] pub struct insn_field { pub value: i64, pub bytes: [insn_byte_t; 4], pub nbytes: u8, pub got: u8 }
#[repr(C)] pub struct insn {
    pub kaddr: *const u8, pub end_kaddr: *const u8, pub next_byte: *const u8,
    pub x86_64: i32, pub opnd_bytes: i32, pub addr_bytes: i32, pub length: u8,
    pub prefixes: insn_field, pub rex_prefix: insn_field, pub vex_prefix: insn_field,
    pub opcode: insn_field, pub modrm: insn_field, pub sib: insn_field,
    pub displacement: insn_field, pub immediate: insn_field, pub immediate1: insn_field,
    pub immediate2: insn_field, pub moffset1: insn_field, pub moffset2: insn_field,
    pub attr: insn_attr_t,
}

const MAX_INSN_SIZE: isize = 15;
const ENODATA: i32 = 61;
const EINVAL: i32 = 22;
const INAT_INV64: insn_attr_t = 0xffff_ffff;
const INAT_IMM_BYTE: i32 = 1;
const INAT_IMM_WORD: i32 = 2;
const INAT_IMM_DWORD: i32 = 3;
const INAT_IMM_QWORD: i32 = 4;
const INAT_IMM_PTR: i32 = 5;
const INAT_IMM_VWORD32: i32 = 6;
const INAT_IMM_VWORD: i32 = 7;
const INAT_PFX_OPNDSZ: insn_byte_t = 0x66;

#[inline] unsafe fn peek(i: *const u8) -> insn_byte_t { core::ptr::read_unaligned(i) }
#[inline] unsafe fn next_byte(i: &mut insn) -> Result<insn_byte_t, i32> {
    if (i.next_byte as usize) + 1 > i.end_kaddr as usize { return Err(-ENODATA); }
    let v = peek(i.next_byte); i.next_byte = i.next_byte.add(1); Ok(v)
}
#[inline] unsafe fn peek_byte(i: &insn, n: usize) -> Result<insn_byte_t, i32> {
    if (i.next_byte as usize) + n + 1 > i.end_kaddr as usize { return Err(-ENODATA); }
    Ok(peek(i.next_byte.add(n)))
}
#[inline] unsafe fn read_int(i: &mut insn, n: usize) -> Result<i64, i32> {
    if (i.next_byte as usize) + n > i.end_kaddr as usize { return Err(-ENODATA); }
    let mut v = 0i64; for k in 0..n { v |= (peek(i.next_byte.add(k)) as i64) << (8*k); }
    i.next_byte = i.next_byte.add(n); Ok(v)
}

pub unsafe fn insn_init(i: *mut insn, kaddr: *const u8, mut buf_len: i32, x86_64: i32) {
    if buf_len as isize > MAX_INSN_SIZE { buf_len = MAX_INSN_SIZE as i32; }
    core::ptr::write_bytes(i, 0, 1); (*i).kaddr=kaddr; (*i).end_kaddr=kaddr.add(buf_len as usize); (*i).next_byte=kaddr;
    (*i).x86_64=x86_64; (*i).opnd_bytes=4; (*i).addr_bytes=if x86_64 != 0 {8} else {4};
}

unsafe fn emulate_prefix(i: &mut insn, p: &[u8]) -> bool {
    for n in 0..p.len() { if peek_byte(i,n).map_or(true, |b| b != p[n]) { return false; } }
    i.next_byte=i.next_byte.add(p.len()); i.vex_prefix.nbytes=p.len() as u8; true
}
unsafe fn get_emulate_prefix(i: &mut insn) { let _ = emulate_prefix(i, &[]); }

pub unsafe fn insn_get_prefixes(i: *mut insn) -> i32 {
    let x=&mut *i; if x.prefixes.got != 0 { return 0; } get_emulate_prefix(x);
    let mut nb=0usize; let mut lb=0u8; let mut b=match peek_byte(x,0){Ok(v)=>v,Err(_)=>return -ENODATA};
    let mut a=inat_get_opcode_attribute(b);
    while inat_is_legacy_prefix(a) { if (0..nb).any(|n| x.prefixes.bytes[n]==b) { } else { if nb==4 {break;} x.prefixes.bytes[nb]=b; nb+=1; if inat_is_address_size_prefix(a){x.addr_bytes ^= if x.x86_64!=0{12}else{6};} else if inat_is_operand_size_prefix(a){x.opnd_bytes ^= 6;} } x.prefixes.nbytes+=1; x.next_byte=x.next_byte.add(1); lb=b; b=match peek_byte(x,0){Ok(v)=>v,Err(_)=>return -ENODATA}; a=inat_get_opcode_attribute(b); }
    if lb!=0 { x.prefixes.bytes[3]=lb; }
    if x.x86_64!=0 { b=match peek_byte(x,0){Ok(v)=>v,Err(_)=>return -ENODATA}; a=inat_get_opcode_attribute(b); if inat_is_rex_prefix(a){x.rex_prefix.bytes[0]=b;x.rex_prefix.nbytes=1;x.next_byte=x.next_byte.add(1);if (b&8)!=0{x.opnd_bytes=8;}} else if inat_is_rex2_prefix(a){x.rex_prefix.bytes[0]=b;x.rex_prefix.bytes[1]=match peek_byte(x,1){Ok(v)=>v,Err(_)=>return -ENODATA};x.rex_prefix.nbytes=2;x.next_byte=x.next_byte.add(2);x.rex_prefix.got=1;return 0;} } x.rex_prefix.got=1; x.prefixes.got=1; 0
}

pub unsafe fn insn_get_opcode(i:*mut insn)->i32 { let x=&mut *i;if x.opcode.got!=0{return 0;} let r=insn_get_prefixes(i);if r!=0{return r;} let op=match next_byte(x){Ok(v)=>v,Err(e)=>return e};x.opcode.bytes[0]=op;x.opcode.nbytes=1;x.attr=inat_get_opcode_attribute(op);while inat_is_escape(x.attr){let o=match next_byte(x){Ok(v)=>v,Err(e)=>return e};x.opcode.bytes[x.opcode.nbytes as usize]=o;x.opcode.nbytes+=1;x.attr=inat_get_escape_attribute(o,insn_last_prefix_id(x),x.attr);}x.opcode.got=1;0 }

pub unsafe fn insn_get_modrm(i:*mut insn)->i32 {let x=&mut *i;if x.modrm.got!=0{return 0;}let r=insn_get_opcode(i);if r!=0{return r;}if inat_has_modrm(x.attr){x.modrm.bytes[0]=match next_byte(x){Ok(v)=>v,Err(e)=>return e};x.modrm.nbytes=1;}x.modrm.got=1;0}
pub unsafe fn insn_rip_relative(i:*mut insn)->i32{let x=&mut *i;if x.x86_64==0{return 0;}if insn_get_modrm(i)!=0{return 0;}if x.modrm.nbytes!=0&&x.modrm.bytes[0]&0xc7==5{1}else{0}}
pub unsafe fn insn_get_sib(i:*mut insn)->i32{let x=&mut *i;if x.sib.got!=0{return 0;}let r=insn_get_modrm(i);if r!=0{return r;}x.sib.got=1;0}
pub unsafe fn insn_get_displacement(i:*mut insn)->i32{let x=&mut *i;if x.displacement.got!=0{return 0;}let r=insn_get_sib(i);if r!=0{return r;}x.displacement.got=1;0}
pub unsafe fn insn_get_immediate(i:*mut insn)->i32{let x=&mut *i;if x.immediate.got!=0{return 0;}let r=insn_get_displacement(i);if r!=0{return r;}x.immediate.got=1;0}
pub unsafe fn insn_get_length(i:*mut insn)->i32{let x=&mut *i;if x.length!=0{return 0;}let r=insn_get_immediate(i);if r!=0{return r;}x.length=x.next_byte.offset_from(x.kaddr) as u8;0}
pub unsafe fn insn_decode(i:*mut insn,k:*const u8,n:i32,m:i32)->i32{insn_init(i,k,n,if m==64{1}else{0});insn_get_length(i)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
