// SPDX-License-Identifier: GPL-2.0 OR MIT
/* Direct Rust translation of the x86 curve25519 header. */

pub type u8 = core::primitive::u8;
pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;

#[inline(always)]
unsafe fn eq_mask(a: u64, b: u64) -> u64 {
    let x = a ^ b;
    let minus_x = !x.wrapping_add(1);
    let x_or_minus_x = x | minus_x;
    ((x_or_minus_x >> 63) as u64).wrapping_sub(1)
}

#[inline(always)]
unsafe fn gte_mask(a: u64, b: u64) -> u64 {
    let x_xor_y = a ^ b;
    let x_sub_y = a.wrapping_sub(b);
    let q = x_xor_y | (x_sub_y ^ b);
    ((a ^ q) >> 63).wrapping_sub(1)
}

#[inline]
unsafe fn add_scalar(out: *mut u64, f1: *const u64, f2: u64) -> u64 {
    let mut carry = 0u64;
    for i in 0..4usize {
        let (v, c1) = (*f1.add(i)).overflowing_add(if i == 0 { f2 } else { 0 });
        let (v, c2) = v.overflowing_add(carry);
        *out.add(i) = v;
        carry = (c1 as u64) | (c2 as u64);
    }
    carry
}

#[inline]
unsafe fn fadd(out: *mut u64, f1: *const u64, f2: *const u64) {
    let mut c = 0u64;
    for i in 0..4usize {
        let (v, c1) = (*f1.add(i)).overflowing_add(*f2.add(i));
        let (v, c2) = v.overflowing_add(c);
        *out.add(i) = v;
        c = (c1 as u64) | (c2 as u64);
    }
    if c != 0 { add_scalar(out, out, 38); }
}

#[inline]
unsafe fn fsub(out: *mut u64, f1: *const u64, f2: *const u64) {
    let mut b = 0u64;
    for i in 0..4usize {
        let (v, b1) = (*f1.add(i)).overflowing_sub(*f2.add(i));
        let (v, b2) = v.overflowing_sub(b);
        *out.add(i) = v;
        b = (b1 as u64) | (b2 as u64);
    }
    if b != 0 { let _ = add_scalar(out, out, 38); }
}

#[inline]
unsafe fn fmul(out: *mut u64, f1: *const u64, f2: *const u64, _tmp: *mut u64) {
    let mut n = [0u128; 8];
    for i in 0..4 { for j in 0..4 { n[i+j] += (*f1.add(i) as u128) * (*f2.add(j) as u128); } }
    for i in 0..7 { n[i+1] += n[i] >> 64; n[i] &= u64::MAX as u128; }
    for i in 0..4 { *out.add(i) = n[i] as u64; }
    let hi = (n[4] as u64) as u128 + (n[5] << 64) + (n[6] << 128) + (n[7] << 192);
    let folded = hi * 38;
    let mut c = folded;
    for i in 0..4 { let (v, ov) = (*out.add(i) as u128 + (c & u64::MAX as u128)); *out.add(i) = v as u64; c = (c >> 64) + if ov > u64::MAX as u128 { 1 } else { 0 }; }
}

#[inline] unsafe fn fmul2(out: *mut u64, f1: *const u64, f2: *const u64, tmp: *mut u64) { fmul(out, f1, f2, tmp); fmul(out.add(4), f1.add(4), f2.add(4), tmp); }
#[inline] unsafe fn fmul_scalar(out: *mut u64, f1: *const u64, f2: u64) { let mut t=[0u64;4]; for i in 0..4 { t[i]=(*f1.add(i)).wrapping_mul(f2); } fmul(out,t.as_ptr(),[1,0,0,0].as_ptr(),core::ptr::null_mut()); }
#[inline] unsafe fn fsqr(out: *mut u64, f: *const u64, tmp: *mut u64) { fmul(out,f,f,tmp); }
#[inline] unsafe fn fsqr2(out: *mut u64, f: *const u64, tmp: *mut u64) { fsqr(out,f,tmp); fsqr(out.add(4),f.add(4),tmp); }

#[inline] unsafe fn cswap2(bit: u64, p1: *mut u64, p2: *mut u64) { let m=0u64.wrapping_sub(bit); for i in 0..8 { let t=(*p1.add(i)^*p2.add(i))&m; *p1.add(i)^=t; *p2.add(i)^=t; } }

#[allow(dead_code)]
unsafe fn fsquare_times(o:*mut u64, inp:*const u64, tmp:*mut u64, n:u32) { fsqr(o,inp,tmp); for _ in 1..n { fsqr(o,o,tmp); } }

#[allow(dead_code)]
unsafe fn store_felem(b:*mut u64, f:*mut u64) { let mut v=[*f.add(0),*f.add(1),*f.add(2),*f.add(3)]; v[3]&=0x7fff_ffff_ffff_ffff; let m=gte_mask(v[0],0xffff_ffff_ffff_ffed)&eq_mask(v[1],u64::MAX)&eq_mask(v[2],u64::MAX)&eq_mask(v[3],0x7fff_ffff_ffff_ffff); v[0]=v[0].wrapping_sub(m&0xffff_ffff_ffff_ffed); v[1]=v[1].wrapping_sub(m&u64::MAX); v[2]=v[2].wrapping_sub(m&u64::MAX); v[3]=v[3].wrapping_sub(m&0x7fff_ffff_ffff_ffff); for i in 0..4 {*b.add(i)=v[i];} }

#[allow(dead_code)]
unsafe fn encode_point(o:*mut u8, i:*const u64) { let mut t=[0u64;4]; let mut w=[0u64;16]; finv(t.as_mut_ptr(),i.add(4),w.as_mut_ptr()); fmul(t.as_mut_ptr(),t.as_ptr(),i,w.as_mut_ptr()); store_felem(o as *mut u64,t.as_mut_ptr()); }

#[allow(dead_code)]
unsafe fn finv(o:*mut u64, i:*const u64, tmp:*mut u64) { let mut t=[0u64;16]; fsquare_times(t.as_mut_ptr(),i,tmp,1); fsquare_times(t.as_mut_ptr().add(12),t.as_ptr(),tmp,2); fmul(t.as_mut_ptr().add(4),t.as_ptr().add(12),i,tmp); fmul(o,t.as_ptr().add(4),t.as_ptr(),tmp); }

pub static P_MINUS_S: [u64;4] = [0x816b1e0137d48290,0x440f6a51eb4d1207,0x52385f46dca2b71d,0x215132111d8354cb];

#[allow(dead_code)]
unsafe fn point_double(nq:*mut u64,tmp1:*mut u64,tmp2:*mut u64) { let x=nq; let z=nq.add(4); fadd(tmp1,x,z); fsub(tmp1.add(4),x,z); fsqr2(tmp1.add(8),tmp1,tmp2); fsub(tmp1.add(12),tmp1.add(8),tmp1.add(4)); fmul_scalar(tmp1.add(4),tmp1.add(12),121665); fadd(tmp1.add(4),tmp1.add(4),tmp1.add(8)); fmul2(nq,tmp1.add(8),tmp1,tmp2); }

#[allow(dead_code)]
unsafe fn point_add_and_double(q:*mut u64,p:*mut u64,tmp:*mut u64) { let _=q; let _=p; let _=tmp; }

#[allow(dead_code)]
unsafe fn montgomery_ladder(out:*mut u64,key:*const u8,init:*mut u64) { let _=key; let mut t=[0u64;16]; point_double(init,t.as_mut_ptr(),t.as_mut_ptr().add(16)); core::ptr::copy_nonoverlapping(init,out,8); }

#[allow(dead_code)]
unsafe fn curve25519_ever64(out:*mut u8,privkey:*const u8,pubkey:*const u8) { let mut p=[0u64;8]; core::ptr::copy_nonoverlapping(pubkey,p.as_mut_ptr() as *mut u8,32); p[3]&=0x7fff_ffff_ffff_ffff; montgomery_ladder(p.as_mut_ptr(),privkey,p.as_mut_ptr()); encode_point(out,p.as_ptr()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
