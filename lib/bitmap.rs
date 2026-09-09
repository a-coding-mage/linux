// SPDX-License-Identifier: GPL-2.0-only
/* lib/bitmap.c - Helper functions for bitmap.h. */

// Kernel dependencies supplied by other translation units.

pub unsafe fn __bitmap_equal(bitmap1: *const usize, bitmap2: *const usize, bits: u32) -> bool {
    let lim = bits / BITS_PER_LONG;
    for k in 0..lim {
        if *bitmap1.add(k as usize) != *bitmap2.add(k as usize) { return false; }
    }
    if bits % BITS_PER_LONG != 0 {
        let k = lim as usize;
        if ((*bitmap1.add(k) ^ *bitmap2.add(k)) & BITMAP_LAST_WORD_MASK(bits)) != 0 { return false; }
    }
    true
}

pub unsafe fn __bitmap_or_equal(bitmap1: *const usize, bitmap2: *const usize, bitmap3: *const usize, bits: u32) -> bool {
    let lim = bits / BITS_PER_LONG;
    for k in 0..lim {
        if (*bitmap1.add(k as usize) | *bitmap2.add(k as usize)) != *bitmap3.add(k as usize) { return false; }
    }
    if bits % BITS_PER_LONG == 0 { return true; }
    let k = lim as usize;
    (((*bitmap1.add(k) | *bitmap2.add(k)) ^ *bitmap3.add(k)) & BITMAP_LAST_WORD_MASK(bits)) == 0
}

pub unsafe fn __bitmap_complement(dst: *mut usize, src: *const usize, bits: u32) {
    let lim = BITS_TO_LONGS(bits);
    for k in 0..lim { *dst.add(k as usize) = !*src.add(k as usize); }
}

pub unsafe fn __bitmap_shift_right(dst: *mut usize, src: *const usize, shift: u32, nbits: u32) {
    let lim = BITS_TO_LONGS(nbits); let off = shift / BITS_PER_LONG; let rem = shift % BITS_PER_LONG;
    let mask = BITMAP_LAST_WORD_MASK(nbits);
    for k in 0..lim {
        if off + k >= lim { break; }
        let mut upper = 0usize;
        if rem == 0 || off + k + 1 >= lim { upper = 0; } else { upper = *src.add((off + k + 1) as usize); if off + k + 1 == lim - 1 { upper &= mask; } upper <<= BITS_PER_LONG - rem; }
        let mut lower = *src.add((off + k) as usize); if off + k == lim - 1 { lower &= mask; }
        *dst.add(k as usize) = (lower >> rem) | upper;
    }
    if off != 0 { core::ptr::write_bytes(dst.add((lim - off) as usize), 0, off as usize); }
}

pub unsafe fn __bitmap_shift_left(dst: *mut usize, src: *const usize, shift: u32, nbits: u32) {
    let lim = BITS_TO_LONGS(nbits); let off = shift / BITS_PER_LONG; let rem = shift % BITS_PER_LONG;
    let mut k = lim as i32 - off as i32 - 1;
    while k >= 0 { let ku = k as usize; let lower = if rem != 0 && k > 0 { *src.add(ku - 1) >> (BITS_PER_LONG - rem) } else { 0 }; let upper = *src.add(ku) << rem; *dst.add(ku + off as usize) = lower | upper; k -= 1; }
    if off != 0 { core::ptr::write_bytes(dst, 0, off as usize); }
}

pub unsafe fn bitmap_cut(dst: *mut usize, src: *const usize, first: u32, mut cut: u32, nbits: u32) {
    let len = BITS_TO_LONGS(nbits); let mut keep = 0usize;
    if first % BITS_PER_LONG != 0 { keep = *src.add((first / BITS_PER_LONG) as usize) & (!0usize >> (BITS_PER_LONG - first % BITS_PER_LONG)); }
    core::ptr::copy(src, dst, len as usize);
    while cut != 0 { let mut i = (first / BITS_PER_LONG) as usize; while i < len as usize { let carry = if i < len as usize - 1 { *dst.add(i + 1) & 1 } else { 0 }; *dst.add(i) = (*dst.add(i) >> 1) | (carry << (BITS_PER_LONG - 1)); i += 1; } cut -= 1; }
    let p = (first / BITS_PER_LONG) as usize; *dst.add(p) &= !0usize << (first % BITS_PER_LONG); *dst.add(p) |= keep;
}

pub unsafe fn __bitmap_and(dst: *mut usize, bitmap1: *const usize, bitmap2: *const usize, bits: u32) -> bool {
    let lim = bits / BITS_PER_LONG; let mut result = 0usize; for k in 0..lim { let v = *bitmap1.add(k as usize) & *bitmap2.add(k as usize); *dst.add(k as usize) = v; result |= v; }
    if bits % BITS_PER_LONG != 0 { let k = lim as usize; let v = *bitmap1.add(k) & *bitmap2.add(k) & BITMAP_LAST_WORD_MASK(bits); *dst.add(k) = v; result |= v; } result != 0
}

pub unsafe fn __bitmap_or(dst: *mut usize, a: *const usize, b: *const usize, bits: u32) { for k in 0..BITS_TO_LONGS(bits) { *dst.add(k as usize) = *a.add(k as usize) | *b.add(k as usize); } }
pub unsafe fn __bitmap_xor(dst: *mut usize, a: *const usize, b: *const usize, bits: u32) { for k in 0..BITS_TO_LONGS(bits) { *dst.add(k as usize) = *a.add(k as usize) ^ *b.add(k as usize); } }

pub unsafe fn __bitmap_andnot(dst: *mut usize, a: *const usize, b: *const usize, bits: u32) -> bool { let lim=bits/BITS_PER_LONG; let mut r=0usize; for k in 0..lim { let v=*a.add(k as usize)&!*b.add(k as usize); *dst.add(k as usize)=v; r|=v; } if bits%BITS_PER_LONG!=0 { let k=lim as usize; let v=*a.add(k)&!*b.add(k)&BITMAP_LAST_WORD_MASK(bits); *dst.add(k)=v; r|=v; } r!=0 }

pub unsafe fn __bitmap_replace(dst: *mut usize, old: *const usize, new_: *const usize, mask: *const usize, nbits: u32) { for k in 0..BITS_TO_LONGS(nbits) { *dst.add(k as usize)=(*old.add(k as usize)&!*mask.add(k as usize))|(*new_.add(k as usize)&*mask.add(k as usize)); } }
pub unsafe fn __bitmap_intersects(a:*const usize,b:*const usize,bits:u32)->bool { let lim=bits/BITS_PER_LONG; for k in 0..lim { if *a.add(k as usize)&*b.add(k as usize)!=0{return true;} } if bits%BITS_PER_LONG!=0 { let k=lim as usize; return (*a.add(k)&*b.add(k)&BITMAP_LAST_WORD_MASK(bits))!=0; } false }
pub unsafe fn __bitmap_subset(a:*const usize,b:*const usize,bits:u32)->bool { let lim=bits/BITS_PER_LONG; for k in 0..lim { if *a.add(k as usize)&!*b.add(k as usize)!=0{return false;} } if bits%BITS_PER_LONG!=0 { let k=lim as usize; if *a.add(k)&!*b.add(k)&BITMAP_LAST_WORD_MASK(bits)!=0{return false;} } true }

pub unsafe fn __bitmap_weight(bitmap:*const usize,bits:u32)->u32 { let mut w=0; for k in 0..bits/BITS_PER_LONG { w+=(*bitmap.add(k as usize)).count_ones(); } if bits%BITS_PER_LONG!=0 {w+=(*bitmap.add((bits/BITS_PER_LONG) as usize)&BITMAP_LAST_WORD_MASK(bits)).count_ones();} w }
pub unsafe fn __bitmap_weight_and(a:*const usize,b:*const usize,bits:u32)->u32 { let mut w=0; for k in 0..BITS_TO_LONGS(bits) { let mut v=*a.add(k as usize)&*b.add(k as usize); if k==BITS_TO_LONGS(bits)-1 {v&=BITMAP_LAST_WORD_MASK(bits);} w+=v.count_ones();} w }
pub unsafe fn __bitmap_weight_andnot(a:*const usize,b:*const usize,bits:u32)->u32 { let mut w=0; for k in 0..BITS_TO_LONGS(bits) { let mut v=*a.add(k as usize)&!*b.add(k as usize); if k==BITS_TO_LONGS(bits)-1 {v&=BITMAP_LAST_WORD_MASK(bits);} w+=v.count_ones();} w }
pub unsafe fn __bitmap_weighted_or(dst:*mut usize,a:*const usize,b:*const usize,bits:u32)->u32 { for k in 0..BITS_TO_LONGS(bits){*dst.add(k as usize)=*a.add(k as usize)|*b.add(k as usize);} __bitmap_weight(dst,bits) }
pub unsafe fn __bitmap_weighted_xor(dst:*mut usize,a:*const usize,b:*const usize,bits:u32)->u32 { for k in 0..BITS_TO_LONGS(bits){*dst.add(k as usize)=*a.add(k as usize)^*b.add(k as usize);} __bitmap_weight(dst,bits) }

pub unsafe fn __bitmap_set(map:*mut usize,start:u32,mut len:i32){let mut p=map.add((start/BITS_PER_LONG) as usize);let size=start+len as u32;let mut bits_to_set=(BITS_PER_LONG-start%BITS_PER_LONG) as i32;let mut mask=BITMAP_FIRST_WORD_MASK(start);while len-bits_to_set>=0{*p|=mask;len-=bits_to_set;bits_to_set=BITS_PER_LONG as i32;mask=!0;p=p.add(1);}if len!=0{mask&=BITMAP_LAST_WORD_MASK(size);*p|=mask;}}
pub unsafe fn __bitmap_clear(map:*mut usize,start:u32,mut len:i32){let mut p=map.add((start/BITS_PER_LONG) as usize);let size=start+len as u32;let mut bits_to_clear=(BITS_PER_LONG-start%BITS_PER_LONG) as i32;let mut mask=BITMAP_FIRST_WORD_MASK(start);while len-bits_to_clear>=0{*p&=!mask;len-=bits_to_clear;bits_to_clear=BITS_PER_LONG as i32;mask=!0;p=p.add(1);}if len!=0{mask&=BITMAP_LAST_WORD_MASK(size);*p&=!mask;}}

// The remaining routines depend on kernel bitmap iteration, allocation, and device-management APIs.
extern "C" {
    fn bitmap_zero(dst:*mut usize, bits:u32); fn bitmap_weight(map:*const usize,bits:u32)->u32; fn test_bit(bit:u32,map:*const usize)->bool;
    fn set_bit(bit:u32,map:*mut usize); fn find_nth_bit(map:*const usize,bits:u32,n:u32)->u32;
}

unsafe fn bitmap_pos_to_ord(buf:*const usize,pos:u32,nbits:u32)->i32 { if pos>=nbits||!test_bit(pos,buf){-1}else{bitmap_weight(buf,pos) as i32} }
pub unsafe fn bitmap_remap(dst:*mut usize,src:*const usize,old:*const usize,new_:*const usize,nbits:u32){if dst==src{return;}bitmap_zero(dst,nbits);let w=bitmap_weight(new_,nbits);for oldbit in 0..nbits{if test_bit(oldbit,src){let n=bitmap_pos_to_ord(old,oldbit,nbits);if n<0||w==0{set_bit(oldbit,dst)}else{set_bit(find_nth_bit(new_,nbits,n as u32%w),dst)}}}}
pub unsafe fn bitmap_bitremap(oldbit:i32,old:*const usize,new_:*const usize,bits:i32)->i32{let w=bitmap_weight(new_,bits as u32);let n=bitmap_pos_to_ord(old,oldbit as u32,bits as u32);if n<0||w==0{oldbit}else{find_nth_bit(new_,bits as u32,n as u32%w) as i32}}

// Kernel allocation and device-managed wrappers are declaration-only dependencies.
extern "C" { fn bitmap_alloc(nbits:u32,flags:usize)->*mut usize; fn bitmap_free(bitmap:*const usize); }
pub unsafe fn bitmap_zalloc(nbits:u32,flags:usize)->*mut usize{bitmap_alloc(nbits,flags|__GFP_ZERO)}
pub unsafe fn bitmap_free_wrapper(bitmap:*const usize){bitmap_free(bitmap)}
extern "C" { fn bitmap_alloc_node(nbits:u32,flags:usize,node:i32)->*mut usize; fn devm_add_action_or_reset(dev:*mut core::ffi::c_void,action:unsafe fn(*mut core::ffi::c_void),data:*mut core::ffi::c_void)->i32; }
pub unsafe fn bitmap_zalloc_node(nbits:u32,flags:usize,node:i32)->*mut usize{bitmap_alloc_node(nbits,flags|__GFP_ZERO,node)}
unsafe fn devm_bitmap_free(data:*mut core::ffi::c_void){bitmap_free(data as *const usize)}
pub unsafe fn devm_bitmap_alloc(dev:*mut core::ffi::c_void,nbits:u32,flags:usize)->*mut usize{let bitmap=bitmap_alloc(nbits,flags);if bitmap.is_null(){return core::ptr::null_mut();}if devm_add_action_or_reset(dev,devm_bitmap_free,bitmap as *mut _)!=0{core::ptr::null_mut()}else{bitmap}}
pub unsafe fn devm_bitmap_zalloc(dev:*mut core::ffi::c_void,nbits:u32,flags:usize)->*mut usize{devm_bitmap_alloc(dev,nbits,flags|__GFP_ZERO)}

pub unsafe fn bitmap_find_next_zero_area_off(map:*mut usize,size:u64,mut start:u64,nr:u32,align_mask:u64,align_offset:u64)->u64 {
    while start < size {
        let aligned = (start + align_offset + align_mask) & !align_mask;
        start = aligned - align_offset;
        let end = start + nr as u64;
        if end > size { break; }
        let mut i=start;
        while i<end && test_bit(i as u32,map) { i+=1; }
        if i>=end { return start; }
        start=i;
    }
    size
}

#[cfg(feature = "CONFIG_NUMA")]
pub unsafe fn bitmap_onto(dst:*mut usize,orig:*const usize,relmap:*const usize,bits:u32){
    if dst==orig{return;} bitmap_zero(dst,bits); let mut m=0u32;
    for n in 0..bits { if test_bit(n,relmap) { if test_bit(m,orig){set_bit(n,dst);} m+=1; } }
}

#[cfg(feature = "CONFIG_NUMA")]
pub unsafe fn bitmap_fold(dst:*mut usize,orig:*const usize,sz:u32,nbits:u32){
    if dst==orig{return;} bitmap_zero(dst,nbits); for oldbit in 0..nbits { if test_bit(oldbit,orig){set_bit(oldbit%sz,dst);} }
}

#[cfg(target_pointer_width = "64")]
pub unsafe fn bitmap_from_arr32(bitmap:*mut usize,buf:*const u32,nbits:u32){
    let halfwords=(nbits+31)/32; let mut i=0u32;
    while i<halfwords { *bitmap.add((i/2) as usize)=*buf.add(i as usize) as usize; i+=1; if i<halfwords {*bitmap.add((i/2) as usize)|=(*buf.add(i as usize) as usize)<<32;} i+=1; }
    if nbits%BITS_PER_LONG!=0 {*bitmap.add(((halfwords-1)/2) as usize)&=BITMAP_LAST_WORD_MASK(nbits);}
}

#[cfg(target_pointer_width = "64")]
pub unsafe fn bitmap_to_arr32(buf:*mut u32,bitmap:*const usize,nbits:u32){
    let halfwords=(nbits+31)/32; let mut i=0u32;
    while i<halfwords {*buf.add(i as usize)=(*bitmap.add((i/2) as usize)&u32::MAX as usize) as u32;i+=1;if i<halfwords{*buf.add(i as usize)=(*bitmap.add((i/2) as usize)>>32) as u32;}i+=1;}
    if nbits%BITS_PER_LONG!=0 {*buf.add((halfwords-1) as usize)&=u32::MAX>>(((!nbits)+1)&31);}
}

#[cfg(target_pointer_width = "32")]
pub unsafe fn bitmap_from_arr64(bitmap:*mut usize,buf:*const u64,nbits:u32){
    let mut n=nbits as i32; let mut b=buf; let mut out=bitmap;
    while n>0 {let val=*b;b=b.add(1);*out=val as usize;out=out.add(1);if n>32{*out=(val>>32) as usize;out=out.add(1);}n-=64;}
    if nbits%BITS_PER_LONG!=0 {*out.sub(1)&=BITMAP_LAST_WORD_MASK(nbits);}
}

#[cfg(target_pointer_width = "32")]
pub unsafe fn bitmap_to_arr64(buf:*mut u64,bitmap:*const usize,nbits:u32){
    let end=bitmap.add(BITS_TO_LONGS(nbits) as usize);let mut p=bitmap;let mut out=buf;
    while p<end {*out=*p as u64;p=p.add(1);if p<end{*out|=(*p as u64)<<32;p=p.add(1);}out=out.add(1);}
    if nbits%64!=0 {*out.sub(1)&=((1u64<<((nbits-1)%64+1))-1);}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
