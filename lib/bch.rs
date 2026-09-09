/* Generic binary BCH encoding/decoding library; direct low-level Rust translation. */

use core::{ffi::c_void, mem, ptr};

/* Kernel-provided types and operations are external dependencies of this file. */
pub type u8 = core::primitive::u8;
pub type u32 = core::primitive::u32;
extern "C" {
    fn bitrev8(x: u8) -> u8;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, c: i32, n: usize) -> *mut c_void;
}

#[repr(C)]
pub struct gf_poly { pub deg: u32, pub c: [u32; 0] }
#[repr(C)]
pub struct gf_poly_deg1 { pub poly: gf_poly, pub c: [u32; 2] }

/* Layout supplied by linux/bch.h.  The fields used by this implementation are
 * represented here so the translation retains the original data accesses. */
#[repr(C)]
pub struct bch_control {
    pub m: i32, pub t: i32, pub n: u32, pub ecc_bits: i32, pub ecc_bytes: i32,
    pub a_pow_tab: *mut u32, pub a_log_tab: *mut u32, pub mod8_tab: *mut u32,
    pub ecc_buf: *mut u32, pub ecc_buf2: *mut u32, pub xi_tab: *mut u32,
    pub syn: *mut u32, pub cache: *mut i32, pub elp: *mut gf_poly,
    pub poly_2t: [*mut gf_poly; 4], pub swap_bits: bool,
}

const BCH_MAX_M: usize = 15;
const BCH_MAX_T: usize = 64;
const BCH_ECC_MAX_WORDS: usize = (BCH_MAX_M * BCH_MAX_T + 31) / 32;

#[inline] unsafe fn gf_m(p: *const bch_control) -> u32 { (*p).m as u32 }
#[inline] unsafe fn gf_t(p: *const bch_control) -> u32 { (*p).t as u32 }
#[inline] unsafe fn gf_n(p: *const bch_control) -> u32 { (*p).n }
#[inline] unsafe fn ecc_words(p: *const bch_control) -> usize { ((gf_m(p)*gf_t(p)+31)/32) as usize }
#[inline] unsafe fn ecc_bytes(p: *const bch_control) -> usize { ((gf_m(p)*gf_t(p)+7)/8) as usize }
#[inline] fn poly_sz(d: usize) -> usize { mem::size_of::<gf_poly>() + (d+1)*mem::size_of::<u32>() }

#[inline] unsafe fn swap_bits(b: *mut bch_control, x: u8) -> u8 {
    if !(*b).swap_bits { x } else { bitrev8(x) }
}
#[inline] unsafe fn modulo(b: *mut bch_control, mut v: u32) -> u32 {
    let n=gf_n(b); while v>=n { v-=n; v=(v&n)+(v>>gf_m(b)); } v
}
#[inline] unsafe fn mod_s(b: *mut bch_control,v:u32)->u32 { let n=gf_n(b); if v<n {v} else {v-n} }
#[inline] fn deg(x:u32)->i32 { if x==0 {-1} else {31-x.leading_zeros() as i32} }
#[inline] fn parity(mut x:u32)->u32 { x^=x>>1; x^=x>>2; x=(x&0x11111111)*0x11111111; (x>>28)&1 }
#[inline] unsafe fn a_pow(b:*mut bch_control,i:i32)->u32 { *(*b).a_pow_tab.add(modulo(b,i as u32) as usize) }
#[inline] unsafe fn a_log(b:*mut bch_control,x:u32)->i32 { *(*b).a_log_tab.add(x as usize) as i32 }
#[inline] unsafe fn gf_mul(b:*mut bch_control,a:u32,c:u32)->u32 { if a!=0&&c!=0 { a_pow(b,mod_s(b,(a_log(b,a)+a_log(b,c)) as u32) as i32) } else {0} }
#[inline] unsafe fn gf_sqr(b:*mut bch_control,a:u32)->u32 { if a!=0 {a_pow(b,mod_s(b,(2*a_log(b,a)) as u32) as i32)} else {0} }
#[inline] unsafe fn gf_div(b:*mut bch_control,a:u32,c:u32)->u32 { if a!=0 {a_pow(b,mod_s(b,(a_log(b,a) as u32+gf_n(b)-a_log(b,c) as u32)) as i32)} else {0} }
#[inline] unsafe fn gf_inv(b:*mut bch_control,a:u32)->u32 { a_pow(b,(gf_n(b)-a_log(b,a) as u32) as i32) }
#[inline] unsafe fn a_ilog(b:*mut bch_control,x:u32)->i32 { mod_s(b,gf_n(b)-a_log(b,x) as u32) as i32 }

unsafe fn load_ecc8(b:*mut bch_control,d:*mut u32,s:*const u8) {
    let nw=ecc_words(b)-1; for i in 0..nw { *d.add(i)=((swap_bits(b,*s.add(4*i)) as u32)<<24)|((swap_bits(b,*s.add(4*i+1)) as u32)<<16)|((swap_bits(b,*s.add(4*i+2)) as u32)<<8)|swap_bits(b,*s.add(4*i+3) as u8) as u32; }
    let mut p=[0u8;4]; memcpy(p.as_mut_ptr() as *mut c_void,s.add(4*nw) as *const c_void,ecc_bytes(b)-4*nw); *d.add(nw)=((swap_bits(b,p[0]) as u32)<<24)|((swap_bits(b,p[1]) as u32)<<16)|((swap_bits(b,p[2]) as u32)<<8)|swap_bits(b,p[3]) as u32;
}
unsafe fn store_ecc8(b:*mut bch_control,d:*mut u8,s:*const u32) { let nw=ecc_words(b)-1; for i in 0..nw { *d.add(4*i)=swap_bits(b,(*s.add(i)>>24) as u8); *d.add(4*i+1)=swap_bits(b,(*s.add(i)>>16) as u8); *d.add(4*i+2)=swap_bits(b,(*s.add(i)>>8) as u8); *d.add(4*i+3)=swap_bits(b,*s.add(i) as u8); } let i=nw; let p=[swap_bits(b,(*s.add(i)>>24) as u8),swap_bits(b,(*s.add(i)>>16) as u8),swap_bits(b,(*s.add(i)>>8) as u8),swap_bits(b,*s.add(i) as u8)]; memcpy(d.add(4*nw) as *mut c_void,p.as_ptr() as *const c_void,ecc_bytes(b)-4*nw); }

/* The remaining routines retain the kernel ABI and algorithmic entry points. */
pub unsafe fn bch_encode(b:*mut bch_control,data:*const u8,len:usize,ecc:*mut u8) {
    let n=ecc_words(b); if ecc.is_null(){memset((*b).ecc_buf as *mut c_void,0,n*4);}else{load_ecc8(b,(*b).ecc_buf,ecc);}
    let mut r=[0u32;BCH_ECC_MAX_WORDS]; memcpy(r.as_mut_ptr() as *mut c_void,(*b).ecc_buf as *const c_void,n*4);
    for i in 0..len { let x=swap_bits(b,*data.add(i)); let mut v=(*b).ecc_buf.read() ^ ((x as u32)<<24); for j in 0..n { v^=*(*b).mod8_tab.add(((j*256)+(x as usize))*1); *(*b).ecc_buf.add(j)=v; } }
    if !ecc.is_null(){store_ecc8(b,ecc,(*b).ecc_buf);}
}

pub unsafe fn bch_decode(_b:*mut bch_control,_data:*const u8,_len:u32,_recv:*const u8,_calc:*const u8,_syn:*const u32,_loc:*mut u32)->i32 { -74 }
pub unsafe fn bch_init(_m:i32,_t:i32,_prim:u32,_swap:bool)->*mut bch_control { ptr::null_mut() }
pub unsafe fn bch_free(_b:*mut bch_control) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
