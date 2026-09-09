// SPDX-License-Identifier: GPL-2.0-or-later
/* Cryptographic API: RIPEMD-160 implementation. */

// External kernel/header symbols are supplied by the surrounding translation unit.

#[repr(C)]
pub struct Rmd160Ctx {
    pub byte_count: u64,
    pub state: [u32; 5],
}

#[inline] fn f1(x: u32, y: u32, z: u32) -> u32 { x ^ y ^ z }
#[inline] fn f2(x: u32, y: u32, z: u32) -> u32 { z ^ (x & (y ^ z)) }
#[inline] fn f3(x: u32, y: u32, z: u32) -> u32 { (x | !y) ^ z }
#[inline] fn f4(x: u32, y: u32, z: u32) -> u32 { y ^ (z & (x ^ y)) }
#[inline] fn f5(x: u32, y: u32, z: u32) -> u32 { x ^ (y | !z) }

macro_rules! round { ($a:ident,$b:ident,$c:ident,$d:ident,$e:ident,$f:ident,$k:expr,$x:expr,$s:expr) => {{
    $a = $a.wrapping_add($f($b,$c,$d)).wrapping_add(u32::from_le($x)).wrapping_add($k);
    $a = $a.rotate_left($s).wrapping_add($e);
    $c = $c.rotate_left(10);
}} }

pub unsafe fn rmd160_transform(state: *mut u32, input: *const u32) {
    let mut aa = *state; let mut bb = *state.add(1); let mut cc = *state.add(2);
    let mut dd = *state.add(3); let mut ee = *state.add(4);
    let mut aaa = aa; let mut bbb = bb; let mut ccc = cc; let mut ddd = dd; let mut eee = ee;
    macro_rules! r { ($a:ident,$b:ident,$c:ident,$d:ident,$e:ident,$f:ident,$k:expr,$i:expr,$s:expr) => {
        round!($a,$b,$c,$d,$e,$f,$k,*input.add($i),$s)
    }; }
    // The constants are defined by ripemd.h in the source environment.
    r!(aa,bb,cc,dd,ee,f1,RMD_K1,0,11); r!(ee,aa,bb,cc,dd,f1,RMD_K1,1,14); r!(dd,ee,aa,bb,cc,f1,RMD_K1,2,15); r!(cc,dd,ee,aa,bb,f1,RMD_K1,3,12); r!(bb,cc,dd,ee,aa,f1,RMD_K1,4,5); r!(aa,bb,cc,dd,ee,f1,RMD_K1,5,8); r!(ee,aa,bb,cc,dd,f1,RMD_K1,6,7); r!(dd,ee,aa,bb,cc,f1,RMD_K1,7,9); r!(cc,dd,ee,aa,bb,f1,RMD_K1,8,11); r!(bb,cc,dd,ee,aa,f1,RMD_K1,9,13); r!(aa,bb,cc,dd,ee,f1,RMD_K1,10,14); r!(ee,aa,bb,cc,dd,f1,RMD_K1,11,15); r!(dd,ee,aa,bb,cc,f1,RMD_K1,12,6); r!(cc,dd,ee,aa,bb,f1,RMD_K1,13,7); r!(bb,cc,dd,ee,aa,f1,RMD_K1,14,9); r!(aa,bb,cc,dd,ee,f1,RMD_K1,15,8);
    r!(ee,aa,bb,cc,dd,f2,RMD_K2,7,7); r!(dd,ee,aa,bb,cc,f2,RMD_K2,4,6); r!(cc,dd,ee,aa,bb,f2,RMD_K2,13,8); r!(bb,cc,dd,ee,aa,f2,RMD_K2,1,13); r!(aa,bb,cc,dd,ee,f2,RMD_K2,10,11); r!(ee,aa,bb,cc,dd,f2,RMD_K2,6,9); r!(dd,ee,aa,bb,cc,f2,RMD_K2,15,7); r!(cc,dd,ee,aa,bb,f2,RMD_K2,3,15); r!(bb,cc,dd,ee,aa,f2,RMD_K2,12,7); r!(aa,bb,cc,dd,ee,f2,RMD_K2,0,12); r!(ee,aa,bb,cc,dd,f2,RMD_K2,9,15); r!(dd,ee,aa,bb,cc,f2,RMD_K2,5,9); r!(cc,dd,ee,aa,bb,f2,RMD_K2,2,11); r!(bb,cc,dd,ee,aa,f2,RMD_K2,14,7); r!(aa,bb,cc,dd,ee,f2,RMD_K2,11,13); r!(ee,aa,bb,cc,dd,f2,RMD_K2,8,12);
    r!(dd,ee,aa,bb,cc,f3,RMD_K3,3,11); r!(cc,dd,ee,aa,bb,f3,RMD_K3,10,13); r!(bb,cc,dd,ee,aa,f3,RMD_K3,14,6); r!(aa,bb,cc,dd,ee,f3,RMD_K3,4,7); r!(ee,aa,bb,cc,dd,f3,RMD_K3,9,14); r!(dd,ee,aa,bb,cc,f3,RMD_K3,15,9); r!(cc,dd,ee,aa,bb,f3,RMD_K3,8,13); r!(bb,cc,dd,ee,aa,f3,RMD_K3,1,15); r!(aa,bb,cc,dd,ee,f3,RMD_K3,2,14); r!(ee,aa,bb,cc,dd,f3,RMD_K3,7,8); r!(dd,ee,aa,bb,cc,f3,RMD_K3,0,13); r!(cc,dd,ee,aa,bb,f3,RMD_K3,6,6); r!(bb,cc,dd,ee,aa,f3,RMD_K3,13,5); r!(aa,bb,cc,dd,ee,f3,RMD_K3,11,12); r!(ee,aa,bb,cc,dd,f3,RMD_K3,5,7); r!(dd,ee,aa,bb,cc,f3,RMD_K3,12,5);
    r!(cc,dd,ee,aa,bb,f4,RMD_K4,1,11); r!(bb,cc,dd,ee,aa,f4,RMD_K4,9,12); r!(aa,bb,cc,dd,ee,f4,RMD_K4,11,14); r!(ee,aa,bb,cc,dd,f4,RMD_K4,10,15); r!(dd,ee,aa,bb,cc,f4,RMD_K4,0,14); r!(cc,dd,ee,aa,bb,f4,RMD_K4,8,15); r!(bb,cc,dd,ee,aa,f4,RMD_K4,12,9); r!(aa,bb,cc,dd,ee,f4,RMD_K4,4,8); r!(ee,aa,bb,cc,dd,f4,RMD_K4,13,9); r!(dd,ee,aa,bb,cc,f4,RMD_K4,3,14); r!(cc,dd,ee,aa,bb,f4,RMD_K4,7,5); r!(bb,cc,dd,ee,aa,f4,RMD_K4,15,6); r!(aa,bb,cc,dd,ee,f4,RMD_K4,14,8); r!(ee,aa,bb,cc,dd,f4,RMD_K4,5,6); r!(dd,ee,aa,bb,cc,f4,RMD_K4,6,5); r!(cc,dd,ee,aa,bb,f4,RMD_K4,2,12);
    r!(bb,cc,dd,ee,aa,f5,RMD_K5,4,9); r!(aa,bb,cc,dd,ee,f5,RMD_K5,0,15); r!(ee,aa,bb,cc,dd,f5,RMD_K5,5,5); r!(dd,ee,aa,bb,cc,f5,RMD_K5,9,11); r!(cc,dd,ee,aa,bb,f5,RMD_K5,7,6); r!(bb,cc,dd,ee,aa,f5,RMD_K5,12,8); r!(aa,bb,cc,dd,ee,f5,RMD_K5,2,13); r!(ee,aa,bb,cc,dd,f5,RMD_K5,10,12); r!(dd,ee,aa,bb,cc,f5,RMD_K5,14,5); r!(cc,dd,ee,aa,bb,f5,RMD_K5,1,12); r!(bb,cc,dd,ee,aa,f5,RMD_K5,3,13); r!(aa,bb,cc,dd,ee,f5,RMD_K5,8,14); r!(ee,aa,bb,cc,dd,f5,RMD_K5,11,11); r!(dd,ee,aa,bb,cc,f5,RMD_K5,6,8); r!(cc,dd,ee,aa,bb,f5,RMD_K5,15,5); r!(bb,cc,dd,ee,aa,f5,RMD_K5,13,6);
    // Right lane, followed by the state combination.
    r!(aaa,bbb,ccc,ddd,eee,f5,RMD_K6,5,8); r!(eee,aaa,bbb,ccc,ddd,f5,RMD_K6,14,9); r!(ddd,eee,aaa,bbb,ccc,f5,RMD_K6,7,9); r!(ccc,ddd,eee,aaa,bbb,f5,RMD_K6,0,11); r!(bbb,ccc,ddd,eee,aaa,f5,RMD_K6,9,13); r!(aaa,bbb,ccc,ddd,eee,f5,RMD_K6,2,15); r!(eee,aaa,bbb,ccc,ddd,f5,RMD_K6,11,15); r!(ddd,eee,aaa,bbb,ccc,f5,RMD_K6,4,5); r!(ccc,ddd,eee,aaa,bbb,f5,RMD_K6,13,7); r!(bbb,ccc,ddd,eee,aaa,f5,RMD_K6,6,7); r!(aaa,bbb,ccc,ddd,eee,f5,RMD_K6,15,8); r!(eee,aaa,bbb,ccc,ddd,f5,RMD_K6,8,11); r!(ddd,eee,aaa,bbb,ccc,f5,RMD_K6,1,14); r!(ccc,ddd,eee,aaa,bbb,f5,RMD_K6,10,14); r!(bbb,ccc,ddd,eee,aaa,f5,RMD_K6,3,12); r!(aaa,bbb,ccc,ddd,eee,f5,RMD_K6,12,6);
    r!(eee,aaa,bbb,ccc,ddd,f4,RMD_K7,6,9); r!(ddd,eee,aaa,bbb,ccc,f4,RMD_K7,11,13); r!(ccc,ddd,eee,aaa,bbb,f4,RMD_K7,3,15); r!(bbb,ccc,ddd,eee,aaa,f4,RMD_K7,7,7); r!(aaa,bbb,ccc,ddd,eee,f4,RMD_K7,0,12); r!(eee,aaa,bbb,ccc,ddd,f4,RMD_K7,13,8); r!(ddd,eee,aaa,bbb,ccc,f4,RMD_K7,5,9); r!(ccc,ddd,eee,aaa,bbb,f4,RMD_K7,10,11); r!(bbb,ccc,ddd,eee,aaa,f4,RMD_K7,14,7); r!(aaa,bbb,ccc,ddd,eee,f4,RMD_K7,15,7); r!(eee,aaa,bbb,ccc,ddd,f4,RMD_K7,8,12); r!(ddd,eee,aaa,bbb,ccc,f4,RMD_K7,12,7); r!(ccc,ddd,eee,aaa,bbb,f4,RMD_K7,4,6); r!(bbb,ccc,ddd,eee,aaa,f4,RMD_K7,9,15); r!(aaa,bbb,ccc,ddd,eee,f4,RMD_K7,1,13); r!(eee,aaa,bbb,ccc,ddd,f4,RMD_K7,2,11);
    r!(ddd,eee,aaa,bbb,ccc,f3,RMD_K8,15,9); r!(ccc,ddd,eee,aaa,bbb,f3,RMD_K8,5,7); r!(bbb,ccc,ddd,eee,aaa,f3,RMD_K8,1,15); r!(aaa,bbb,ccc,ddd,eee,f3,RMD_K8,3,11); r!(eee,aaa,bbb,ccc,ddd,f3,RMD_K8,7,8); r!(ddd,eee,aaa,bbb,ccc,f3,RMD_K8,14,6); r!(ccc,ddd,eee,aaa,bbb,f3,RMD_K8,6,6); r!(bbb,ccc,ddd,eee,aaa,f3,RMD_K8,9,14); r!(aaa,bbb,ccc,ddd,eee,f3,RMD_K8,11,12); r!(eee,aaa,bbb,ccc,ddd,f3,RMD_K8,8,13); r!(ddd,eee,aaa,bbb,ccc,f3,RMD_K8,12,5); r!(ccc,ddd,eee,aaa,bbb,f3,RMD_K8,2,14); r!(bbb,ccc,ddd,eee,aaa,f3,RMD_K8,10,13); r!(aaa,bbb,ccc,ddd,eee,f3,RMD_K8,0,13); r!(eee,aaa,bbb,ccc,ddd,f3,RMD_K8,4,7); r!(ddd,eee,aaa,bbb,ccc,f3,RMD_K8,13,5);
    r!(ccc,ddd,eee,aaa,bbb,f2,RMD_K9,8,15); r!(bbb,ccc,ddd,eee,aaa,f2,RMD_K9,6,5); r!(aaa,bbb,ccc,ddd,eee,f2,RMD_K9,4,8); r!(eee,aaa,bbb,ccc,ddd,f2,RMD_K9,1,11); r!(ddd,eee,aaa,bbb,ccc,f2,RMD_K9,3,14); r!(ccc,ddd,eee,aaa,bbb,f2,RMD_K9,11,14); r!(bbb,ccc,ddd,eee,aaa,f2,RMD_K9,15,6); r!(aaa,bbb,ccc,ddd,eee,f2,RMD_K9,0,14); r!(eee,aaa,bbb,ccc,ddd,f2,RMD_K9,5,6); r!(ddd,eee,aaa,bbb,ccc,f2,RMD_K9,12,9); r!(ccc,ddd,eee,aaa,bbb,f2,RMD_K9,2,12); r!(bbb,ccc,ddd,eee,aaa,f2,RMD_K9,13,9); r!(aaa,bbb,ccc,ddd,eee,f2,RMD_K9,9,12); r!(eee,aaa,bbb,ccc,ddd,f2,RMD_K9,7,5); r!(ddd,eee,aaa,bbb,ccc,f2,RMD_K9,10,15); r!(ccc,ddd,eee,aaa,bbb,f2,RMD_K9,14,8);
    r!(bbb,ccc,ddd,eee,aaa,f1,RMD_K1,12,8); r!(aaa,bbb,ccc,ddd,eee,f1,RMD_K1,15,5); r!(eee,aaa,bbb,ccc,ddd,f1,RMD_K1,10,12); r!(ddd,eee,aaa,bbb,ccc,f1,RMD_K1,4,9); r!(ccc,ddd,eee,aaa,bbb,f1,RMD_K1,1,12); r!(bbb,ccc,ddd,eee,aaa,f1,RMD_K1,5,5); r!(aaa,bbb,ccc,ddd,eee,f1,RMD_K1,8,14); r!(eee,aaa,bbb,ccc,ddd,f1,RMD_K1,7,6); r!(ddd,eee,aaa,bbb,ccc,f1,RMD_K1,6,8); r!(ccc,ddd,eee,aaa,bbb,f1,RMD_K1,2,13); r!(bbb,ccc,ddd,eee,aaa,f1,RMD_K1,13,6); r!(aaa,bbb,ccc,ddd,eee,f1,RMD_K1,14,5); r!(eee,aaa,bbb,ccc,ddd,f1,RMD_K1,0,15); r!(ddd,eee,aaa,bbb,ccc,f1,RMD_K1,3,13); r!(ccc,ddd,eee,aaa,bbb,f1,RMD_K1,9,11); r!(bbb,ccc,ddd,eee,aaa,f1,RMD_K1,11,11);
    let old0 = *state; let old1 = *state.add(1); let old2 = *state.add(2); let old3 = *state.add(3); let old4 = *state.add(4);
    ddd = ddd.wrapping_add(cc).wrapping_add(old1); *state.add(1) = old2.wrapping_add(dd).wrapping_add(eee); *state.add(2) = old3.wrapping_add(ee).wrapping_add(aaa); *state.add(3) = old4.wrapping_add(aa).wrapping_add(bbb); *state.add(4) = old0.wrapping_add(bb).wrapping_add(ccc); *state = ddd;
}

// Equivalent shash callbacks; kernel-provided descriptor access and endian helpers
// remain external dependencies, as in the original implementation.
#[repr(C)] pub struct ShashDesc { _private: [u8; 0] }
extern "C" {
    fn shash_desc_ctx(desc: *mut ShashDesc) -> *mut Rmd160Ctx;
    fn crypto_register_shash(alg: *mut ShashAlg) -> i32;
    fn crypto_unregister_shash(alg: *mut ShashAlg);
}

pub unsafe fn rmd160_init(desc: *mut ShashDesc) -> i32 {
    let c = &mut *shash_desc_ctx(desc);
    c.byte_count = 0;
    c.state = [RMD_H0, RMD_H1, RMD_H2, RMD_H3, RMD_H4];
    0
}

pub unsafe fn rmd160_update(desc: *mut ShashDesc, data: *const u8, mut len: usize) -> i32 {
    let c = &mut *shash_desc_ctx(desc);
    let remain = len % RMD160_BLOCK_SIZE;
    c.byte_count = c.byte_count.wrapping_add((len - remain) as u64);
    let mut buffer = [0u32; 16];
    while len >= core::mem::size_of_val(&buffer) {
        core::ptr::copy_nonoverlapping(data, buffer.as_mut_ptr() as *mut u8, core::mem::size_of_val(&buffer));
        rmd160_transform(c.state.as_mut_ptr(), buffer.as_ptr());
        data = data.add(core::mem::size_of_val(&buffer));
        len -= core::mem::size_of_val(&buffer);
    }
    core::ptr::write_bytes(buffer.as_mut_ptr() as *mut u8, 0, core::mem::size_of_val(&buffer));
    remain as i32
}

pub unsafe fn rmd160_finup(desc: *mut ShashDesc, src: *const u8, len: usize, out: *mut u8) -> i32 {
    let c = &mut *shash_desc_ctx(desc);
    let mut block = [0u8; RMD160_BLOCK_SIZE * 2];
    c.byte_count = c.byte_count.wrapping_add(len as u64);
    core::ptr::copy_nonoverlapping(src, block.as_mut_ptr(), len);
    block[len] = 0x80;
    let bit_offset = if len >= (RMD160_BLOCK_SIZE - 8) { RMD160_BLOCK_SIZE + 56 } else { 56 };
    block[bit_offset..bit_offset + 8].copy_from_slice(&(c.byte_count << 3).to_le_bytes());
    rmd160_transform(c.state.as_mut_ptr(), block.as_ptr() as *const u32);
    if bit_offset > 56 { rmd160_transform(c.state.as_mut_ptr(), block.as_ptr().add(64) as *const u32); }
    for i in 0..5 { core::ptr::write_unaligned(out.add(i * 4) as *mut u32, c.state[i].to_le()); }
    core::ptr::write_bytes(block.as_mut_ptr(), 0, block.len());
    0
}

#[repr(C)] pub struct ShashAlg { pub digestsize: u32, pub init: Option<unsafe fn(*mut ShashDesc)->i32>, pub update: Option<unsafe fn(*mut ShashDesc,*const u8,usize)->i32>, pub finup: Option<unsafe fn(*mut ShashDesc,*const u8,usize,*mut u8)->i32>, pub descsize: usize }
pub static mut ALG: ShashAlg = ShashAlg { digestsize: RMD160_DIGEST_SIZE, init: Some(rmd160_init), update: Some(rmd160_update), finup: Some(rmd160_finup), descsize: core::mem::size_of::<Rmd160Ctx>() };
pub unsafe fn rmd160_mod_init() -> i32 { crypto_register_shash(&mut ALG) }
pub unsafe fn rmd160_mod_fini() { crypto_unregister_shash(&mut ALG); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
