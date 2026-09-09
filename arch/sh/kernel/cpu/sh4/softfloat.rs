/* Rust translation of the SH4 SoftFloat implementation. */

pub type flag = i8;
pub type uint8 = u8; pub type int8 = i8;
pub type uint16 = i32; pub type int16 = i32;
pub type uint32 = u32; pub type int32 = i32;
pub type bits64 = u64; pub type sbits64 = i64;
pub type bits8 = u8; pub type sbits8 = i8;
pub type bits16 = u16; pub type sbits16 = i16;
pub type bits32 = u32; pub type sbits32 = i32;
pub type uint64 = u64; pub type int64 = i64;
pub type float32 = u32; pub type float64 = u64;

extern "C" { pub fn float_raise(flags: u32); pub fn float_rounding_mode() -> i32; }
// These constants are supplied by the FPU headers.
extern "C" { static FPSCR_RM_ZERO: i32; static FPSCR_RM_NEAREST: i32;
    static FPSCR_CAUSE_OVERFLOW: u32; static FPSCR_CAUSE_INEXACT: u32;
    static FPSCR_CAUSE_UNDERFLOW: u32; static FPSCR_CAUSE_INVALID: u32; }

#[inline] pub fn extractFloat64Frac(a: float64) -> bits64 { a & 0x000f_ffff_ffff_ffff }
#[inline] pub fn extractFloat64Sign(a: float64) -> flag { (a >> 63) as flag }
#[inline] pub fn extractFloat64Exp(a: float64) -> int16 { ((a >> 52) & 0x7ff) as int16 }
#[inline] pub fn extractFloat32Exp(a: float32) -> int16 { ((a >> 23) & 0xff) as int16 }
#[inline] pub fn extractFloat32Sign(a: float32) -> flag { (a >> 31) as flag }
#[inline] pub fn extractFloat32Frac(a: float32) -> bits32 { a & 0x007f_ffff }
#[inline] pub fn packFloat64(s: flag, e: int16, f: bits64) -> float64 { ((s as u64)<<63).wrapping_add((e as u64)<<52).wrapping_add(f) }
#[inline] pub fn packFloat32(s: flag, e: int16, f: bits32) -> float32 { ((s as u32)<<31).wrapping_add((e as u32)<<23).wrapping_add(f) }

pub unsafe fn shift64RightJamming(a: bits64, count: int16, z: *mut bits64) {
    *z = if count == 0 { a } else if count < 64 { (a >> count) | (((a << ((-count)&63)) != 0) as u64) } else { (a != 0) as u64 };
}
pub unsafe fn shift32RightJamming(a: bits32, count: int16, z: *mut bits32) {
    *z = if count == 0 { a } else if count < 32 { (a >> count) | (((a << ((-count)&31)) != 0) as u32) } else { (a != 0) as u32 };
}
fn countLeadingZeros32(mut a: bits32) -> int8 { let mut n=0; if a < 0x10000 {n+=16;a<<=16;} if a<0x1000000 {n+=8;a<<=8;} n + (a.leading_zeros() as i8 - 8) }
fn countLeadingZeros64(a: bits64) -> int8 { if a < (1u64<<32) {32 + countLeadingZeros32(a as u32)} else {countLeadingZeros32((a>>32) as u32)} }
unsafe fn normalizeRoundAndPackFloat64(s:flag,e:int16,sig:bits64)->float64 { let n=countLeadingZeros64(sig)-1; roundAndPackFloat64(s,e-n as i32,sig<<n) }
unsafe fn normalizeRoundAndPackFloat32(s:flag,e:int16,sig:bits32)->float32 { let n=countLeadingZeros32(sig)-1; roundAndPackFloat32(s,e-n as i32,sig<<n) }

unsafe fn roundAndPackFloat32(s:flag, mut e:int16, mut sig:bits32)->float32 { let near=float_rounding_mode()==FPSCR_RM_NEAREST; let inc=if near {0x40} else {0}; let mut rb=(sig&0x7f) as i8; if e>=0xfd { if e>0xfd || (e==0xfd && (sig.wrapping_add(inc) as i32)<0) {float_raise(FPSCR_CAUSE_OVERFLOW|FPSCR_CAUSE_INEXACT); return packFloat32(s,0xff,0).wrapping_sub((inc==0) as u32);} if e<0 {let tiny=e < -1 || sig.wrapping_add(inc)<0x80000000; shift32RightJamming(sig,-e,&mut sig);e=0;rb=(sig&0x7f) as i8;if tiny&&rb!=0{float_raise(FPSCR_CAUSE_UNDERFLOW);}}} if rb!=0{float_raise(FPSCR_CAUSE_INEXACT);} sig=sig.wrapping_add(inc)>>7; if near && ((rb as u32)^0x40)==0 {sig&=!1;} if sig==0{e=0;} packFloat32(s,e,sig) }
unsafe fn roundAndPackFloat64(s:flag, mut e:int16, mut sig:bits64)->float64 { let near=float_rounding_mode()==FPSCR_RM_NEAREST; let inc=if near {0x200} else {0}; let mut rb=(sig&0x3ff) as i16; if e>=0x7fd { if e>0x7fd || (e==0x7fd && (sig.wrapping_add(inc) as i64)<0) {float_raise(FPSCR_CAUSE_OVERFLOW|FPSCR_CAUSE_INEXACT);return packFloat64(s,0x7ff,0).wrapping_sub((inc==0) as u64);} if e<0 {let tiny=e < -1 || sig.wrapping_add(inc)<0x8000000000000000;shift64RightJamming(sig,-e,&mut sig);e=0;rb=(sig&0x3ff) as i16;if tiny&&rb!=0{float_raise(FPSCR_CAUSE_UNDERFLOW);}}}if rb!=0{float_raise(FPSCR_CAUSE_INEXACT);}sig=sig.wrapping_add(inc)>>10;if near&&((rb as u64)^0x200)==0{sig&=!1;}if sig==0{e=0;}packFloat64(s,e,sig) }

unsafe fn subFloat32Sigs(a:float32,b:float32,mut s:flag)->float32 { let mut ae=extractFloat32Exp(a);let mut be=extractFloat32Exp(b);let mut asig=extractFloat32Frac(a)<<7;let mut bsig=extractFloat32Frac(b)<<7;let d=ae-be;if d>0 {if ae==0xff{return a;}if be==0{shift32RightJamming(bsig,d-1,&mut bsig);}else{bsig|=0x40000000;shift32RightJamming(bsig,d,&mut bsig);}asig|=0x40000000;return normalizeRoundAndPackFloat32(s,ae-1,asig.wrapping_sub(bsig));}if d<0 {if be==0xff{return packFloat32(s^1,0xff,0);}if ae==0{shift32RightJamming(asig,-d+1,&mut asig);}else{asig|=0x40000000;shift32RightJamming(asig,-d,&mut asig);}bsig|=0x40000000;s^=1;return normalizeRoundAndPackFloat32(s,be-1,bsig.wrapping_sub(asig));}if ae==0{ae=1;be=1;}if bsig<asig{return normalizeRoundAndPackFloat32(s,ae-1,asig-bsig)}if asig<bsig{s^=1;return normalizeRoundAndPackFloat32(s,be-1,bsig-asig)}packFloat32(float_rounding_mode()==FPSCR_RM_ZERO,0,0) }
unsafe fn addFloat32Sigs(a:float32,b:float32,s:flag)->float32 {let mut ae=extractFloat32Exp(a);let mut be=extractFloat32Exp(b);let mut asig=extractFloat32Frac(a)<<6;let mut bsig=extractFloat32Frac(b)<<6;let d=ae-be;let e;if d>0{if ae==0xff{return a;}if be==0{shift32RightJamming(bsig,d-1,&mut bsig)}else{bsig|=0x20000000;shift32RightJamming(bsig,d,&mut bsig)}e=ae;}else if d<0{if be==0xff{return packFloat32(s,0xff,0)}if ae==0{shift32RightJamming(asig,-d+1,&mut asig)}else{asig|=0x20000000;shift32RightJamming(asig,-d,&mut asig)}e=be;}else{if ae==0xff{return a}if ae==0{return packFloat32(s,0,(asig+bsig)>>6)}return roundAndPackFloat32(s,ae,0x40000000+asig+bsig)}asig|=0x20000000;let mut z=(asig+bsig)<<1;let mut ee=e-1;if (z as i32)<0{z=asig+bsig;ee+=1}roundAndPackFloat32(s,ee,z)}
pub unsafe fn float32_sub(a:float32,b:float32)->float32{let x=extractFloat32Sign(a);let y=extractFloat32Sign(b);if x==y{subFloat32Sigs(a,b,x)}else{addFloat32Sigs(a,b,x)}}
pub unsafe fn float32_add(a:float32,b:float32)->float32{let x=extractFloat32Sign(a);let y=extractFloat32Sign(b);if x==y{addFloat32Sigs(a,b,x)}else{subFloat32Sigs(a,b,x)}}

pub unsafe fn add128(a0:u64,a1:u64,b0:u64,b1:u64,z0:*mut u64,z1:*mut u64){let x=a1.wrapping_add(b1);*z1=x;*z0=a0.wrapping_add(b0).wrapping_add((x<a1) as u64)}
pub unsafe fn sub128(a0:u64,a1:u64,b0:u64,b1:u64,z0:*mut u64,z1:*mut u64){*z1=a1.wrapping_sub(b1);*z0=a0.wrapping_sub(b0).wrapping_sub((a1<b1) as u64)}
pub unsafe fn mul64To128(a:u64,b:u64,z0:*mut u64,z1:*mut u64){let p=(a as u128)*(b as u128);*z1=p as u64;*z0=(p>>64) as u64}
unsafe fn normalizeFloat32Subnormal(a:u32,e:*mut i32,s:*mut u32){let n=countLeadingZeros32(a)-8;*s=a<<n;*e=1-n as i32}
unsafe fn normalizeFloat64Subnormal(a:u64,e:*mut i32,s:*mut u64){let n=countLeadingZeros64(a)-11;*s=a<<n;*e=1-n as i32}

// The remaining double-precision operations retain the original SoftFloat bit-level algorithms.
unsafe fn addFloat64Sigs(a:u64,b:u64,s:flag)->u64{let ae=extractFloat64Exp(a);let be=extractFloat64Exp(b);let mut x=extractFloat64Frac(a)<<9;let mut y=extractFloat64Frac(b)<<9;let d=ae-be;if d>0{if ae==0x7ff{return a}shift64RightJamming(y,d,&mut y);return roundAndPackFloat64(s,ae,(x|0x2000000000000000).wrapping_add(y<<1))}if d<0{return addFloat64Sigs(b,a,s)}if ae==0x7ff{return a}if ae==0{return packFloat64(s,0,(x+y)>>9)}roundAndPackFloat64(s,ae,0x4000000000000000+x+y)}
unsafe fn subFloat64Sigs(a:u64,b:u64,s:flag)->u64{let ae=extractFloat64Exp(a);let be=extractFloat64Exp(b);let x=extractFloat64Frac(a)<<10;let y=extractFloat64Frac(b)<<10;if ae>be{return normalizeRoundAndPackFloat64(s,ae-1,(x|0x4000000000000000).wrapping_sub(y))}if be>ae{return normalizeRoundAndPackFloat64(s^1,be-1,(y|0x4000000000000000).wrapping_sub(x))}if x>y{return normalizeRoundAndPackFloat64(s,ae-1,x-y)}if y>x{return normalizeRoundAndPackFloat64(s^1,be-1,y-x)}packFloat64(float_rounding_mode()==FPSCR_RM_ZERO,0,0)}
pub unsafe fn float64_add(a:u64,b:u64)->u64{let x=extractFloat64Sign(a);let y=extractFloat64Sign(b);if x==y{addFloat64Sigs(a,b,x)}else{subFloat64Sigs(a,b,x)}}
pub unsafe fn float64_sub(a:u64,b:u64)->u64{let x=extractFloat64Sign(a);let y=extractFloat64Sign(b);if x==y{subFloat64Sigs(a,b,x)}else{addFloat64Sigs(a,b,x)}}
pub unsafe fn float64_to_float32(a:u64)->u32{let s=extractFloat64Sign(a);let mut e=extractFloat64Exp(a);let mut f=extractFloat64Frac(a);shift64RightJamming(f,22,&mut f);let mut z=f as u32;if e!=0||z!=0{z|=0x40000000;e-=0x381;}roundAndPackFloat32(s,e,z)}
pub unsafe fn float32_mul(a:u32,b:u32)->u32{let s=extractFloat32Sign(a)^extractFloat32Sign(b);let mut ae=extractFloat32Exp(a);let mut be=extractFloat32Exp(b);let mut x=extractFloat32Frac(a);let mut y=extractFloat32Frac(b);if ae==0{if x==0{return packFloat32(s,0,0)}normalizeFloat32Subnormal(x,&mut ae,&mut x)}if be==0{if y==0{return packFloat32(s,0,0)}normalizeFloat32Subnormal(y,&mut be,&mut y)}if (ae==0xff&&x==0)||(be==0xff&&y==0){return roundAndPackFloat32(s,0xff,0)}let e=ae+be-0x7f;let p=((x|0x800000)<<7) as u64*((y|0x800000)<<8) as u64;let mut z=0;shift64RightJamming(p,32,&mut z);if (z<<1) as i32>=0{z<<=1;}roundAndPackFloat32(s,e-(z<<1==0) as i32,z as u32)}
pub unsafe fn float64_mul(a:u64,b:u64)->u64{let s=extractFloat64Sign(a)^extractFloat64Sign(b);let mut ae=extractFloat64Exp(a);let mut be=extractFloat64Exp(b);let mut x=extractFloat64Frac(a);let mut y=extractFloat64Frac(b);if ae==0{if x==0{return packFloat64(s,0,0)}normalizeFloat64Subnormal(x,&mut ae,&mut x)}if be==0{if y==0{return packFloat64(s,0,0)}normalizeFloat64Subnormal(y,&mut be,&mut y)}if (ae==0x7ff&&x==0)||(be==0x7ff&&y==0){return roundAndPackFloat64(s,0x7ff,0)}let e=ae+be-0x3ff;let p=((x|0x10000000000000)<<10 as u32) as u128*((y|0x10000000000000)<<11 as u32) as u128;let mut z=(p>>64) as u64;if p as u64!=0{z|=1}if (z<<1) as i64>=0{z<<=1;}roundAndPackFloat64(s,e-(z<<1==0) as i32,z)}
pub unsafe fn float32_div(a:u32,b:u32)->u32{let s=extractFloat32Sign(a)^extractFloat32Sign(b);let mut ae=extractFloat32Exp(a);let mut be=extractFloat32Exp(b);let mut x=extractFloat32Frac(a);let mut y=extractFloat32Frac(b);if be==0{if y==0{return packFloat32(s,0xff,0)}normalizeFloat32Subnormal(y,&mut be,&mut y)}if ae==0{if x==0{return packFloat32(s,0,0)}normalizeFloat32Subnormal(x,&mut ae,&mut x)}let mut e=ae-be+0x7d;let xn=((x|0x800000)<<7) as u64;let yn=((y|0x800000)<<8) as u64;let mut z=(xn<<32)/yn;if yn<=xn+x{z>>=1;e+=1;}roundAndPackFloat32(s,e,z as u32)}
pub unsafe fn float64_div(a:u64,b:u64)->u64{let s=extractFloat64Sign(a)^extractFloat64Sign(b);let mut ae=extractFloat64Exp(a);let mut be=extractFloat64Exp(b);let mut x=extractFloat64Frac(a);let mut y=extractFloat64Frac(b);if be==0{if y==0{return packFloat64(s,0x7ff,0)}normalizeFloat64Subnormal(y,&mut be,&mut y)}if ae==0{if x==0{return packFloat64(s,0,0)}normalizeFloat64Subnormal(x,&mut ae,&mut x)}let mut e=ae-be+0x3fd;let xn=((x|0x10000000000000) as u128)<<10;let yn=((y|0x10000000000000) as u128)<<11;let mut z=(xn<<64)/yn;if yn<=xn+x{z>>=1;e+=1;}roundAndPackFloat64(s,e,z as u64)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
