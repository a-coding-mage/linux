// SPDX-License-Identifier: GPL-2.0
// Translation of the Alpha implementation.  The operations below correspond
// to the Alpha unaligned load/store and extract/insert instructions.

type __wsum = u32;

#[inline]
unsafe fn ldq_u(p: *const u8) -> usize { core::ptr::read_unaligned(p as *const usize) }
#[inline]
unsafe fn stq_u(v: usize, p: *mut u8) { core::ptr::write_unaligned(p as *mut usize, v); }
#[inline] unsafe fn extql(x: usize, y: usize) -> usize { x >> ((y & 7) * 8) }
#[inline] unsafe fn extqh(x: usize, y: usize) -> usize { x << ((8 - (y & 7)) * 8) }
#[inline] unsafe fn mskql(x: usize, y: usize) -> usize { x & (!0usize >> ((8 - (y & 7)) * 8)) }
#[inline] unsafe fn mskqh(x: usize, y: usize) -> usize { x & (!0usize << ((y & 7) * 8)) }
#[inline] unsafe fn insql(x: usize, y: usize) -> usize { x << ((y & 7) * 8) }
#[inline] unsafe fn insqh(x: usize, y: usize) -> usize { x >> ((8 - (y & 7)) * 8) }

#[inline]
unsafe fn get_word(p: *const u8, unaligned: bool) -> Result<usize, i64> {
    let _ = unaligned;
    Ok(ldq_u(p))
}

#[inline]
unsafe fn from64to16(x: usize) -> u16 {
    let a = (x as u32).wrapping_add((x >> 32) as u32) as u64;
    let b = (a as u16 as u64).wrapping_add((a >> 16) as u16 as u64)
        .wrapping_add((a >> 32) as u16 as u64);
    (b as u16).wrapping_add((b >> 16) as u16) as u16
}

#[inline]
unsafe fn csum_partial_cfu_aligned(src: *const u8, dst: *mut usize, mut len: i64) -> usize {
    let mut checksum: usize = !0u32 as usize; let mut carry = 0usize; let mut s = src; let mut d = dst;
    while len >= 0 { let word = match get_word(s, false) { Ok(v) => v, Err(_) => return 0 };
        checksum = checksum.wrapping_add(carry).wrapping_add(word); s = s.add(8); len -= 8; carry = (checksum < word) as usize; *d = word; d = d.add(1); }
    len += 8; checksum = checksum.wrapping_add(carry);
    if len != 0 { let word0 = match get_word(s, false) { Ok(v) => v, Err(_) => return 0 }; let tmp = *d; let word = mskql(word0, len as usize); checksum = checksum.wrapping_add(word); let tmp = mskqh(tmp, len as usize); carry = (checksum < word) as usize; *d = word | tmp; checksum = checksum.wrapping_add(carry); }
    checksum
}

#[inline]
unsafe fn csum_partial_cfu_dest_aligned(src: *const u8, dst: *mut usize, soff: usize, mut len: i64) -> usize {
    let mut first = get_word(src, true).unwrap_or(0); let lastsrc = src.offset(7 + len) ; let mut carry=0; let mut checksum=!0u32 as usize; let mut s=src; let mut d=dst;
    while len >= 0 { let second=get_word(s.add(8),true).unwrap_or(0); let mut word=extql(first,soff); len-=8; s=s.add(8); first=extqh(second,soff); checksum=checksum.wrapping_add(carry); word|=first; first=second; checksum=checksum.wrapping_add(word); *d=word; d=d.add(1); carry=(checksum<word) as usize; }
    len+=8; checksum=checksum.wrapping_add(carry); if len!=0 { let second=get_word(lastsrc,true).unwrap_or(0); let tmp=*d; let mut word=extql(first,soff)|extqh(second,soff); word=mskql(word,len as usize); checksum=checksum.wrapping_add(word); let tmp=mskqh(tmp,len as usize); carry=(checksum<word) as usize; *d=word|tmp; checksum=checksum.wrapping_add(carry); } checksum
}

#[inline]
unsafe fn csum_partial_cfu_src_aligned(src: *const u8, dst: *mut usize, doff: usize, len: i64, partial_dest: usize) -> usize {
    // Literal low-level counterpart of the Alpha source-aligned path.
    let mut checksum = !0u32 as usize; let mut carry = 0usize; let mut s=src; let mut d=dst; let mut pd=mskql(partial_dest,doff);
    let mut n=len;
    while n>=0 { let word=get_word(s,false).unwrap_or(0); let second=insql(word,doff); checksum=checksum.wrapping_add(carry); stq_u(pd|second,d as *mut u8); s=s.add(8); checksum=checksum.wrapping_add(word); pd=insqh(word,doff); carry=(checksum<word) as usize; d=d.add(1); n-=8; }
    n+=8; if n!=0 { checksum=checksum.wrapping_add(carry); let word=mskql(get_word(s,false).unwrap_or(0),n as usize); checksum=checksum.wrapping_add(word); let second=insql(word,doff); let q=n+doff as i64; carry=(checksum<word) as usize; pd|=second; if q>=0 { stq_u(pd,d as *mut u8); if q!=0 { d=d.add(1); pd=insqh(word,doff); } } else { let old=ldq_u(d as *const u8); stq_u(pd|mskqh(old, q as usize),d as *mut u8); } } checksum.wrapping_add(carry)
}

#[inline]
unsafe fn csum_partial_cfu_unaligned(src: *const u8, dst: *mut usize, soff: usize, doff: usize, len: i64, partial_dest: usize) -> usize {
    // Preserve the same unaligned load/merge ordering as the C routine.
    let mut checksum=!0u32 as usize; let mut carry=0; let mut s=src; let mut d=dst; let mut first=get_word(s,true).unwrap_or(0); let last=src.offset(7+len); let mut pd=mskql(partial_dest,doff); let mut n=len;
    while n>=0 { let second=get_word(s.add(8),true).unwrap_or(0); let mut word=extql(first,soff)|extqh(second,soff); n-=8; s=s.add(8); first=second; let out=insql(word,doff); checksum=checksum.wrapping_add(carry).wrapping_add(word); stq_u(pd|out,d as *mut u8); carry=(checksum<word) as usize; pd=insqh(word,doff); d=d.add(1); }
    n+=doff as i64; checksum=checksum.wrapping_add(carry); let second=get_word(last,true).unwrap_or(0); let mut word=extql(first,soff)|extqh(second,soff); word=mskql(word,(n-doff as i64) as usize); checksum=checksum.wrapping_add(word); let out=insql(word,doff); carry=(checksum<word) as usize; stq_u(pd|out,d as *mut u8); checksum.wrapping_add(carry)
}

#[inline]
unsafe fn __csum_and_copy(src: *const u8, dst: *mut u8, len: i32) -> __wsum {
    let soff=(src as usize)&7; let doff=(dst as usize)&7; let checksum: usize;
    if doff==0 { checksum=if soff==0 { csum_partial_cfu_aligned(src,dst as *mut usize,(len-8) as i64) } else { csum_partial_cfu_dest_aligned(src,dst as *mut usize,soff,(len-8) as i64) }; }
    else { let partial=ldq_u(dst); checksum=if soff==0 { csum_partial_cfu_src_aligned(src,dst as *mut usize,doff,(len-8) as i64,partial) } else { csum_partial_cfu_unaligned(src,dst as *mut usize,soff,doff,(len-8) as i64,partial) }; }
    from64to16(checksum) as __wsum
}

extern "C" { fn access_ok(addr: *const u8, size: i32) -> bool; }

#[no_mangle]
pub unsafe extern "C" fn csum_and_copy_from_user(src: *const u8, dst: *mut u8, len: i32) -> __wsum {
    if !access_ok(src,len) { return 0; } __csum_and_copy(src,dst,len)
}

#[no_mangle]
pub unsafe extern "C" fn csum_partial_copy_nocheck(src: *const u8, dst: *mut u8, len: i32) -> __wsum {
    __csum_and_copy(src,dst,len)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
