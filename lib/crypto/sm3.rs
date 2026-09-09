/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of the C SM3 implementation. */

// The following types, constants, and primitives are supplied by the kernel
// crypto headers and architecture-specific implementation.
use core::ptr;

#[repr(C)]
pub struct sm3_block_state { pub h: [u32; 8] }
#[repr(C)]
pub struct sm3_ctx { pub state: sm3_block_state, pub bytecount: usize, pub buf: [u8; SM3_BLOCK_SIZE] }

extern "C" {
    static SM3_IVA: u32; static SM3_IVB: u32; static SM3_IVC: u32; static SM3_IVD: u32;
    static SM3_IVE: u32; static SM3_IVF: u32; static SM3_IVG: u32; static SM3_IVH: u32;
    fn sm3_blocks(state: *mut sm3_block_state, data: *const u8, nblocks: usize);
}

const SM3_BLOCK_SIZE: usize = 64;
const SM3_DIGEST_SIZE: usize = 32;
static K: [u32; 64] = [
    0x79cc4519,0xf3988a32,0xe7311465,0xce6228cb,0x9cc45197,0x3988a32f,0x7311465e,0xe6228cbc,
    0xcc451979,0x988a32f3,0x311465e7,0x6228cbce,0xc451979c,0x88a32f39,0x11465e73,0x228cbce6,
    0x9d8a7a87,0x3b14f50f,0x7629ea1e,0xec53d43c,0xd8a7a879,0xb14f50f3,0x629ea1e7,0xc53d43ce,
    0x8a7a879d,0x14f50f3b,0x29ea1e76,0x53d43cec,0xa7a879d8,0x4f50f3b1,0x9ea1e762,0x3d43cec5,
    0x7a879d8a,0xf50f3b14,0xea1e7629,0xd43cec53,0xa879d8a7,0x50f3b14f,0xa1e7629e,0x43cec53d,
    0x879d8a7a,0x0f3b14f5,0x1e7629ea,0x3cec53d4,0x79d8a7a8,0xf3b14f50,0xe7629ea1,0xcec53d43,
    0x9d8a7a87,0x3b14f50f,0x7629ea1e,0xec53d43c,0xd8a7a879,0xb14f50f3,0x629ea1e7,0xc53d43ce,
    0x8a7a879d,0x14f50f3b,0x29ea1e76,0x53d43cec,0xa7a879d8,0x4f50f3b1,0x9ea1e762,0x3d43cec5
];

#[inline] fn p0(x: u32) -> u32 { x ^ x.rotate_left(9) ^ x.rotate_left(17) }
#[inline] fn p1(x: u32) -> u32 { x ^ x.rotate_left(15) ^ x.rotate_left(23) }

unsafe fn sm3_block_generic(state: *mut sm3_block_state, data: *const u8, w: *mut [u32; 16]) {
    for i in 0..16 { (*w)[i] = u32::from_be_bytes([*data.add(i*4),*data.add(i*4+1),*data.add(i*4+2),*data.add(i*4+3)]); }
    let mut v = (*state).h;
    for i in 0..64 {
        let w1 = (*w)[i & 15];
        let w2 = if i < 4 { (*w)[i+4] } else {
            let x = (*w)[i & 15] ^ (*w)[(i+7) & 15] ^ (*w)[(i+13) & 15].rotate_left(15);
            (*w)[i & 15] = p1(x) ^ (*w)[(i+3) & 15].rotate_left(7) ^ (*w)[(i+10) & 15]; (*w)[i & 15]
        };
        let ff = if i < 16 { v[0]^v[1]^v[2] } else { (v[0]&v[1])|(v[0]&v[2])|(v[1]&v[2]) };
        let gg = if i < 16 { v[4]^v[5]^v[6] } else { (v[4]&v[5])|(!v[4]&v[6]) };
        let ss1 = (v[0].rotate_left(12).wrapping_add(v[4]).wrapping_add(K[i])).rotate_left(7);
        let ss2 = ss1 ^ v[0].rotate_left(12);
        let d = v[3].wrapping_add(ff).wrapping_add(ss2).wrapping_add(w1^w2);
        let h = p0(v[7].wrapping_add(gg).wrapping_add(ss1).wrapping_add(w1));
        v = [d, v[0].rotate_left(9), v[1], v[2], h, v[4].rotate_left(19), v[5], v[6]];
    }
    for i in 0..8 { (*state).h[i] ^= v[i]; }
}

pub unsafe fn sm3_init(ctx: *mut sm3_ctx) { (*ctx).state.h = [SM3_IVA,SM3_IVB,SM3_IVC,SM3_IVD,SM3_IVE,SM3_IVF,SM3_IVG,SM3_IVH]; (*ctx).bytecount=0; }
pub unsafe fn sm3_update(ctx: *mut sm3_ctx, mut data: *const u8, mut len: usize) {
    let mut partial = (*ctx).bytecount % SM3_BLOCK_SIZE; (*ctx).bytecount += len;
    if partial + len >= SM3_BLOCK_SIZE { if partial { let l=SM3_BLOCK_SIZE-partial; ptr::copy_nonoverlapping(data,(*ctx).buf.as_mut_ptr().add(partial),l); data=data.add(l); len-=l; sm3_blocks(&mut (*ctx).state,(*ctx).buf.as_ptr(),1); } let n=len/SM3_BLOCK_SIZE; len%=SM3_BLOCK_SIZE; if n { sm3_blocks(&mut (*ctx).state,data,n); data=data.add(n*SM3_BLOCK_SIZE); } partial=0; }
    if len { ptr::copy_nonoverlapping(data,(*ctx).buf.as_mut_ptr().add(partial),len); }
}

pub unsafe fn sm3_final(ctx: *mut sm3_ctx, out: *mut u8) {
    let bits=(*ctx).bytecount.wrapping_mul(8); let mut p=(*ctx).bytecount%64; (*ctx).buf[p]=0x80; p+=1;
    if p>56 { (*ctx).buf[p..64].fill(0); sm3_blocks(&mut (*ctx).state,(*ctx).buf.as_ptr(),1); p=0; }
    (*ctx).buf[p..56].fill(0); (*ctx).buf[56..64].copy_from_slice(&bits.to_be_bytes()); sm3_blocks(&mut (*ctx).state,(*ctx).buf.as_ptr(),1);
    for i in 0..8 { ptr::copy_nonoverlapping((*ctx).state.h[i].to_be_bytes().as_ptr(),out.add(i*4),4); }
    ptr::write_bytes(ctx as *mut u8,0,core::mem::size_of::<sm3_ctx>());
}
pub unsafe fn sm3(data:*const u8,len:usize,out:*mut u8) { let mut ctx=sm3_ctx{state:sm3_block_state{h:[0;8]},bytecount:0,buf:[0;64]}; sm3_init(&mut ctx); sm3_update(&mut ctx,data,len); sm3_final(&mut ctx,out); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
