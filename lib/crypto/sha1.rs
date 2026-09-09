// SPDX-License-Identifier: GPL-2.0
/* SHA-1 and HMAC-SHA1 library functions. */

// Types, constants, and external helpers below are supplied by the corresponding
// kernel headers (and by the architecture-specific implementation when enabled).

const SHA1_WORKSPACE_WORDS: usize = 16;

static SHA1_IV: sha1_block_state = sha1_block_state { h: [SHA1_H0, SHA1_H1, SHA1_H2, SHA1_H3, SHA1_H4] };

#[inline]
unsafe fn get_be32(p: *const u8) -> u32 {
    u32::from_be_bytes([*p, *p.add(1), *p.add(2), *p.add(3)])
}
#[inline]
unsafe fn put_be32(v: u32, p: *mut u8) { p.copy_from_nonoverlapping(v.to_be_bytes().as_ptr(), 4); }
#[inline]
unsafe fn put_be64(v: u64, p: *mut u8) { p.copy_from_nonoverlapping(v.to_be_bytes().as_ptr(), 8); }

unsafe fn sha1_block_generic(state: *mut sha1_block_state, data: *const u8,
                             workspace: *mut [u32; SHA1_WORKSPACE_WORDS]) {
    let mut a = (*state).h[0]; let mut b = (*state).h[1]; let mut c = (*state).h[2];
    let mut d = (*state).h[3]; let mut e = (*state).h[4];
    let mut i = 0usize;
    while i < 80 {
        let x = if i < 16 { get_be32(data.add(i * 4)) } else {
            let w = &mut *workspace;
            let x = w[(i + 13) & 15] ^ w[(i + 8) & 15] ^ w[(i + 2) & 15] ^ w[i & 15];
            x.rotate_left(1)
        };
        (*workspace)[i & 15] = x;
        let f = if i < 20 { ((c ^ d) & b) ^ d }
                else if i < 40 { b ^ c ^ d }
                else if i < 60 { (b & c) + (d & (b ^ c)) }
                else { b ^ c ^ d };
        let k = if i < 20 { 0x5a827999 } else if i < 40 { 0x6ed9eba1 }
                else if i < 60 { 0x8f1bbcdc } else { 0xca62c1d6 };
        let t = e.wrapping_add(x).wrapping_add(a.rotate_left(5)).wrapping_add(f).wrapping_add(k);
        b = b.rotate_right(2);
        let old_e = e; e = d; d = c; c = b; b = a; a = t;
        i += 1;
    }
    (*state).h[0] = (*state).h[0].wrapping_add(a);
    (*state).h[1] = (*state).h[1].wrapping_add(b);
    (*state).h[2] = (*state).h[2].wrapping_add(c);
    (*state).h[3] = (*state).h[3].wrapping_add(d);
    (*state).h[4] = (*state).h[4].wrapping_add(e);
}

unsafe fn sha1_blocks_generic(state: *mut sha1_block_state, mut data: *const u8, mut nblocks: usize) {
    let mut workspace = [0u32; SHA1_WORKSPACE_WORDS];
    while nblocks != 0 { sha1_block_generic(state, data, &mut workspace); data = data.add(SHA1_BLOCK_SIZE); nblocks -= 1; }
    core::ptr::write_bytes(workspace.as_mut_ptr(), 0, workspace.len());
}

#[inline] unsafe fn sha1_blocks(state: *mut sha1_block_state, data: *const u8, n: usize) { sha1_blocks_generic(state, data, n); }

pub unsafe fn sha1_init(ctx: *mut sha1_ctx) { (*ctx).state = SHA1_IV; (*ctx).bytecount = 0; }

pub unsafe fn sha1_update(ctx: *mut sha1_ctx, mut data: *const u8, mut len: usize) {
    let mut partial = (*ctx).bytecount % SHA1_BLOCK_SIZE;
    (*ctx).bytecount += len;
    if partial + len >= SHA1_BLOCK_SIZE {
        if partial != 0 { let l = SHA1_BLOCK_SIZE - partial; core::ptr::copy_nonoverlapping(data, (*ctx).buf.as_mut_ptr().add(partial), l); data = data.add(l); len -= l; sha1_blocks(&mut (*ctx).state, (*ctx).buf.as_ptr(), 1); }
        let nblocks = len / SHA1_BLOCK_SIZE; len %= SHA1_BLOCK_SIZE;
        if nblocks != 0 { sha1_blocks(&mut (*ctx).state, data, nblocks); data = data.add(nblocks * SHA1_BLOCK_SIZE); }
        partial = 0;
    }
    if len != 0 { core::ptr::copy_nonoverlapping(data, (*ctx).buf.as_mut_ptr().add(partial), len); }
}

unsafe fn __sha1_final(ctx: *mut sha1_ctx, out: *mut u8) {
    let bitcount = (*ctx).bytecount << 3; let mut partial = (*ctx).bytecount % SHA1_BLOCK_SIZE;
    (*ctx).buf[partial] = 0x80; partial += 1;
    if partial > SHA1_BLOCK_SIZE - 8 { core::ptr::write_bytes((*ctx).buf.as_mut_ptr().add(partial), 0, SHA1_BLOCK_SIZE - partial); sha1_blocks(&mut (*ctx).state, (*ctx).buf.as_ptr(), 1); partial = 0; }
    core::ptr::write_bytes((*ctx).buf.as_mut_ptr().add(partial), 0, SHA1_BLOCK_SIZE - 8 - partial);
    put_be64(bitcount, (*ctx).buf.as_mut_ptr().add(SHA1_BLOCK_SIZE - 8)); sha1_blocks(&mut (*ctx).state, (*ctx).buf.as_ptr(), 1);
    for i in (0..SHA1_DIGEST_SIZE).step_by(4) { put_be32((*ctx).state.h[i / 4], out.add(i)); }
}

pub unsafe fn sha1_final(ctx: *mut sha1_ctx, out: *mut u8) { __sha1_final(ctx, out); core::ptr::write_bytes(ctx as *mut u8, 0, core::mem::size_of::<sha1_ctx>()); }
pub unsafe fn sha1(data: *const u8, len: usize, out: *mut u8) { let mut ctx: sha1_ctx = core::mem::zeroed(); sha1_init(&mut ctx); sha1_update(&mut ctx, data, len); sha1_final(&mut ctx, out); }

pub unsafe fn hmac_sha1_preparekey(key: *mut hmac_sha1_key, raw_key: *const u8, raw_key_len: usize) {
    let mut block = [0u8; SHA1_BLOCK_SIZE];
    if raw_key_len > SHA1_BLOCK_SIZE { sha1(raw_key, raw_key_len, block.as_mut_ptr()); }
    else { core::ptr::copy_nonoverlapping(raw_key, block.as_mut_ptr(), raw_key_len); }
    for x in block.iter_mut() { *x ^= HMAC_IPAD_VALUE; }
    (*key).istate = SHA1_IV; sha1_blocks(&mut (*key).istate, block.as_ptr(), 1);
    for x in block.iter_mut() { *x ^= HMAC_OPAD_VALUE ^ HMAC_IPAD_VALUE; }
    (*key).ostate = SHA1_IV; sha1_blocks(&mut (*key).ostate, block.as_ptr(), 1);
}
pub unsafe fn hmac_sha1_init(ctx: *mut hmac_sha1_ctx, key: *const hmac_sha1_key) { (*ctx).sha_ctx.state = (*key).istate; (*ctx).sha_ctx.bytecount = SHA1_BLOCK_SIZE; (*ctx).ostate = (*key).ostate; }
pub unsafe fn hmac_sha1_init_usingrawkey(ctx: *mut hmac_sha1_ctx, raw_key: *const u8, len: usize) { let mut k: hmac_sha1_key = core::mem::zeroed(); hmac_sha1_preparekey(&mut k, raw_key, len); hmac_sha1_init(ctx, &k); }
pub unsafe fn hmac_sha1_final(ctx: *mut hmac_sha1_ctx, out: *mut u8) { __sha1_final(&mut (*ctx).sha_ctx, (*ctx).sha_ctx.buf.as_mut_ptr()); core::ptr::write_bytes((*ctx).sha_ctx.buf.as_mut_ptr().add(SHA1_DIGEST_SIZE), 0, SHA1_BLOCK_SIZE-SHA1_DIGEST_SIZE); (*ctx).sha_ctx.buf[SHA1_DIGEST_SIZE]=0x80; put_be32((8*(SHA1_BLOCK_SIZE+SHA1_DIGEST_SIZE)) as u32, (*ctx).sha_ctx.buf.as_mut_ptr().add(SHA1_BLOCK_SIZE-4)); sha1_blocks(&mut (*ctx).ostate, (*ctx).sha_ctx.buf.as_ptr(), 1); for i in (0..SHA1_DIGEST_SIZE).step_by(4) { put_be32((*ctx).ostate.h[i/4], out.add(i)); } }
pub unsafe fn hmac_sha1(key: *const hmac_sha1_key, data: *const u8, len: usize, out: *mut u8) { let mut c: hmac_sha1_ctx=core::mem::zeroed(); hmac_sha1_init(&mut c,key); hmac_sha1_update(&mut c,data,len); hmac_sha1_final(&mut c,out); }
pub unsafe fn hmac_sha1_usingrawkey(k:*const u8, kl:usize,d:*const u8,dl:usize,o:*mut u8){let mut c:hmac_sha1_ctx=core::mem::zeroed();hmac_sha1_init_usingrawkey(&mut c,k,kl);hmac_sha1_update(&mut c,d,dl);hmac_sha1_final(&mut c,o);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
