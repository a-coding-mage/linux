// SPDX-License-Identifier: GPL-2.0+
/* Direct low-level Rust translation of aspeed-hace-hash.c. */

// Kernel dependencies supplied externally by the surrounding translation.

static SHA1_IV: [u32; 8] = [cpu_to_be32(SHA1_H0), cpu_to_be32(SHA1_H1), cpu_to_be32(SHA1_H2), cpu_to_be32(SHA1_H3), cpu_to_be32(SHA1_H4), 0, 0, 0];
static SHA224_IV: [u32; 8] = [cpu_to_be32(SHA224_H0), cpu_to_be32(SHA224_H1), cpu_to_be32(SHA224_H2), cpu_to_be32(SHA224_H3), cpu_to_be32(SHA224_H4), cpu_to_be32(SHA224_H5), cpu_to_be32(SHA224_H6), cpu_to_be32(SHA224_H7)];
static SHA256_IV: [u32; 8] = [cpu_to_be32(SHA256_H0), cpu_to_be32(SHA256_H1), cpu_to_be32(SHA256_H2), cpu_to_be32(SHA256_H3), cpu_to_be32(SHA256_H4), cpu_to_be32(SHA256_H5), cpu_to_be32(SHA256_H6), cpu_to_be32(SHA256_H7)];
static SHA384_IV: [u64; 8] = [cpu_to_be64(SHA384_H0), cpu_to_be64(SHA384_H1), cpu_to_be64(SHA384_H2), cpu_to_be64(SHA384_H3), cpu_to_be64(SHA384_H4), cpu_to_be64(SHA384_H5), cpu_to_be64(SHA384_H6), cpu_to_be64(SHA384_H7)];
static SHA512_IV: [u64; 8] = [cpu_to_be64(SHA512_H0), cpu_to_be64(SHA512_H1), cpu_to_be64(SHA512_H2), cpu_to_be64(SHA512_H3), cpu_to_be64(SHA512_H4), cpu_to_be64(SHA512_H5), cpu_to_be64(SHA512_H6), cpu_to_be64(SHA512_H7)];

unsafe fn aspeed_sham_export(req: *mut ahash_request, out: *mut u8) -> i32 {
    let r = ahash_request_ctx(req); let c = &mut *r;
    memcpy(out, c.digest.as_mut_ptr() as *mut _, c.ivsize as usize);
    let p = out.add(c.ivsize as usize) as *mut u64;
    put_unaligned(c.digcnt[0], p);
    if c.ivsize == 64 { put_unaligned(c.digcnt[1], p.add(1)); } 0
}

unsafe fn aspeed_sham_import(req: *mut ahash_request, input: *const u8) -> i32 {
    let r = ahash_request_ctx(req); let c = &mut *r;
    let e = aspeed_sham_init(req); if e != 0 { return e; }
    memcpy(c.digest.as_mut_ptr() as *mut _, input as *const _, c.ivsize as usize);
    let p = input.add(c.ivsize as usize) as *const u64;
    c.digcnt[0] = get_unaligned(p); if c.ivsize == 64 { c.digcnt[1] = get_unaligned(p.add(1)); } 0
}

unsafe fn aspeed_ahash_fill_padding(h: *mut aspeed_hace_dev, r: *mut aspeed_sham_reqctx, b: *mut u8) -> i32 {
    let c=&mut *r; let (pad,bitslen,bits) = match c.flags & SHA_FLAGS_MASK {
        SHA_FLAGS_SHA1|SHA_FLAGS_SHA224|SHA_FLAGS_SHA256 => { let x=(c.digcnt[0] << 3).to_be(); let i=c.digcnt[0]&0x3f; ((if i<56 {56-i}else{120-i}),8,[0,x]) },
        _ => { let x=(c.digcnt[0] << 3).to_be(); let y=((c.digcnt[1]<<3)|(c.digcnt[0]>>61)).to_be(); let i=c.digcnt[0]&0x7f; ((if i<112 {112-i}else{240-i}),16,[y,x]) }
    }; let _=h; *b=0x80; memset(b.add(1),0,(pad-1) as usize); memcpy(b.add(pad as usize) as *mut _, bits.as_ptr() as *const _, bitslen as usize); pad+bitslen
}

unsafe fn aspeed_ahash_update_counter(r:*mut aspeed_sham_reqctx,len:u32){let c=&mut *r;c.offset+=len;c.digcnt[0]=c.digcnt[0].wrapping_add(len as u64);if c.digcnt[0] < len as u64 {c.digcnt[1]+=1;}}

unsafe fn aspeed_ahash_complete(h:*mut aspeed_hace_dev)->i32{let e=&mut (*h).hash_engine;let req=e.req;let r=&mut *ahash_request_ctx(req);dma_unmap_single((*h).dev,r.digest_dma_addr,SHA512_DIGEST_SIZE,DMA_BIDIRECTIONAL);if r.total-r.offset>=r.block_size||(r.total!=r.offset&&(r.flags&SHA_FLAGS_FINUP)!=0){return aspeed_ahash_req_update(h)}e.flags&=!CRYPTO_FLAGS_BUSY;if r.flags&SHA_FLAGS_FINUP!=0{memcpy((*req).result as *mut _,r.digest.as_ptr() as *const _,r.digsize as usize);}crypto_finalize_hash_request((*h).crypt_engine_hash,req,r.total-r.offset);0}

unsafe fn aspeed_hace_ahash_trigger(h:*mut aspeed_hace_dev,resume:aspeed_hace_fn_t)->i32{let e=&mut (*h).hash_engine;let r=&mut *ahash_request_ctx(e.req);r.cmd|=HASH_CMD_INT_ENABLE;e.resume=resume;ast_hace_write(h,e.src_dma,ASPEED_HACE_HASH_SRC);ast_hace_write(h,e.digest_dma,ASPEED_HACE_HASH_DIGEST_BUFF);ast_hace_write(h,e.digest_dma,ASPEED_HACE_HASH_KEY_BUFF);ast_hace_write(h,e.src_length,ASPEED_HACE_HASH_DATA_LEN);mb();ast_hace_write(h,r.cmd,ASPEED_HACE_HASH_CMD);-EINPROGRESS}

unsafe fn aspeed_ahash_req_update(h:*mut aspeed_hace_dev)->i32{let e=&mut (*h).hash_engine;let r=&mut *ahash_request_ctx(e.req);let resume=if (*h).version==AST2600_VERSION{r.cmd|=HASH_CMD_HASH_SRC_SG_CTRL;aspeed_ahash_update_resume_sg}else{aspeed_ahash_complete};let ret=(e.dma_prepare)(h);if ret!=0{return ret}aspeed_hace_ahash_trigger(h,resume)}

unsafe fn aspeed_ahash_update_resume_sg(h:*mut aspeed_hace_dev)->i32{let e=&mut (*h).hash_engine;let r=&mut *ahash_request_ctx(e.req);dma_unmap_sg((*h).dev,r.src_sg,r.src_nents,DMA_TO_DEVICE);if r.flags&SHA_FLAGS_FINUP!=0&&r.total==r.offset{dma_unmap_single((*h).dev,r.buffer_dma_addr,core::mem::size_of_val(&r.buffer),DMA_TO_DEVICE)};r.cmd&=!HASH_CMD_HASH_SRC_SG_CTRL;aspeed_ahash_complete(h)}

unsafe fn aspeed_hace_hash_handle_queue(h:*mut aspeed_hace_dev,req:*mut ahash_request)->i32{crypto_transfer_hash_request_to_engine((*h).crypt_engine_hash,req)}

unsafe fn aspeed_sham_update(req:*mut ahash_request)->i32{let r=&mut *ahash_request_ctx(req);let tfm=crypto_ahash_reqtfm(req);let t=&mut *crypto_ahash_ctx(tfm);r.total=(*req).nbytes;r.src_sg=(*req).src;r.offset=0;r.src_nents=sg_nents_for_len((*req).src,(*req).nbytes);aspeed_hace_hash_handle_queue(t.hace_dev,req)}
unsafe fn aspeed_sham_finup(req:*mut ahash_request)->i32{(*ahash_request_ctx(req)).flags|=SHA_FLAGS_FINUP;aspeed_sham_update(req)}
unsafe fn aspeed_sham_digest(req:*mut ahash_request)->i32{let e=aspeed_sham_init(req);if e!=0{e}else{aspeed_sham_finup(req)}}

// The remaining registration tables and their callbacks retain the kernel's
// original externally-defined layouts and symbols.
extern "C" { static mut aspeed_ahash_algs: [aspeed_hace_alg; 3]; static mut aspeed_ahash_algs_g6: [aspeed_hace_alg; 2]; }
pub unsafe fn aspeed_unregister_hace_hash_algs(h:*mut aspeed_hace_dev){for a in aspeed_ahash_algs.iter_mut(){crypto_engine_unregister_ahash(&mut a.alg.ahash);}if (*h).version!=AST2600_VERSION{return;}for a in aspeed_ahash_algs_g6.iter_mut(){crypto_engine_unregister_ahash(&mut a.alg.ahash);}}

unsafe fn aspeed_ahash_dma_prepare(h:*mut aspeed_hace_dev)->i32{let e=&mut (*h).hash_engine;let r=&mut *ahash_request_ctx(e.req);let mut len=r.total-r.offset;let rem=len-len/r.block_size*r.block_size;let mut final_=false;if len>ASPEED_HASH_SRC_DMA_BUF_LEN{len=ASPEED_HASH_SRC_DMA_BUF_LEN}else if r.flags&SHA_FLAGS_FINUP!=0{if ((len+r.block_size-1)/r.block_size)*r.block_size+r.block_size>ASPEED_CRYPTO_SRC_DMA_BUF_LEN{len=(len-1)/r.block_size*r.block_size}else{final_=true}}else{len-=rem}memcpy_from_sglist(e.ahash_src_addr,r.src_sg,r.offset,len);aspeed_ahash_update_counter(r,len);if final_{len+=aspeed_ahash_fill_padding(h,r,e.ahash_src_addr.add(len as usize));}r.digest_dma_addr=dma_map_single((*h).dev,r.digest.as_mut_ptr(),SHA512_DIGEST_SIZE,DMA_BIDIRECTIONAL);if dma_mapping_error((*h).dev,r.digest_dma_addr){return -ENOMEM}e.src_length=len;e.src_dma=e.ahash_src_dma_addr;e.digest_dma=r.digest_dma_addr;0}
unsafe fn aspeed_ahash_dma_prepare_sg(h:*mut aspeed_hace_dev)->i32{let e=&mut (*h).hash_engine;let r=&mut *ahash_request_ctx(e.req);let final_=r.flags&SHA_FLAGS_FINUP!=0;let mut len=r.total-r.offset;let rem=if final_{0}else{len-len/r.block_size*r.block_size};len-=rem;let n=dma_map_sg((*h).dev,r.src_sg,r.src_nents,DMA_TO_DEVICE);if n==0{return -ENOMEM}r.digest_dma_addr=dma_map_single((*h).dev,r.digest.as_mut_ptr(),SHA512_DIGEST_SIZE,DMA_BIDIRECTIONAL);if dma_mapping_error((*h).dev,r.digest_dma_addr){dma_unmap_sg((*h).dev,r.src_sg,r.src_nents,DMA_TO_DEVICE);return -ENOMEM}let list=e.ahash_src_addr as *mut aspeed_sg_list;let mut total=0;let mut off=r.offset;let mut i=0;for_each_sg(r.src_sg,n,i,{let mut a=sg_dma_address(s);let mut l=sg_dma_len(s);if l<=off{off-=l;continue}l-=off;a+=off;off=0;if len>l{len-=l}else{l=len;len=0}total+=l;(*list.add(i)).phy_addr=cpu_to_le32(a);(*list.add(i)).len=cpu_to_le32(l);});if len!=0{total=total/r.block_size*r.block_size;}aspeed_ahash_update_counter(r,total);if final_{let l=aspeed_ahash_fill_padding(h,r,r.buffer.as_mut_ptr());r.buffer_dma_addr=dma_map_single((*h).dev,r.buffer.as_mut_ptr(),r.buffer.len(),DMA_TO_DEVICE);(*list.add(i)).phy_addr=cpu_to_le32(r.buffer_dma_addr);(*list.add(i)).len=cpu_to_le32(l as u32);total+=l as u32;i+=1;}(*list.add(i-1)).len|=cpu_to_le32(HASH_SG_LAST_LIST);e.src_length=total;e.src_dma=e.ahash_src_dma_addr;e.digest_dma=r.digest_dma_addr;0}

unsafe fn aspeed_ahash_prepare_request(_: *mut crypto_engine, areq:*mut core::ffi::c_void){let req=ahash_request_cast(areq);let tfm=crypto_ahash_reqtfm(req);let t=&mut *crypto_ahash_ctx(tfm);let e=&mut (*t.hace_dev).hash_engine;e.req=req;e.dma_prepare=if (*t.hace_dev).version==AST2600_VERSION{aspeed_ahash_dma_prepare_sg}else{aspeed_ahash_dma_prepare};}
unsafe fn aspeed_ahash_do_request(_: *mut crypto_engine,areq:*mut core::ffi::c_void)->i32{let req=ahash_request_cast(areq);let tfm=crypto_ahash_reqtfm(req);let t=&mut *crypto_ahash_ctx(tfm);(*t.hace_dev).hash_engine.flags|=CRYPTO_FLAGS_BUSY;let ret=aspeed_ahash_req_update(t.hace_dev);if ret!=-EINPROGRESS{aspeed_ahash_fallback(req)}else{0}}
unsafe fn aspeed_ahash_do_one(e:*mut crypto_engine,a:*mut core::ffi::c_void)->i32{aspeed_ahash_prepare_request(e,a);aspeed_ahash_do_request(e,a)}
unsafe fn aspeed_sham_init(req:*mut ahash_request)->i32{let r=&mut *ahash_request_ctx(req);let tfm=crypto_ahash_reqtfm(req);let t=&mut *crypto_ahash_ctx(tfm);r.cmd=HASH_CMD_ACC_MODE;r.flags=0;match crypto_ahash_digestsize(tfm){SHA1_DIGEST_SIZE=>{r.cmd|=HASH_CMD_SHA1|HASH_CMD_SHA_SWAP;r.flags|=SHA_FLAGS_SHA1;r.digsize=SHA1_DIGEST_SIZE;r.block_size=SHA1_BLOCK_SIZE;r.ivsize=32;r.digest[..32].copy_from_slice(&SHA1_IV.map(|x|x.to_ne_bytes()).concat())},SHA224_DIGEST_SIZE=>{r.cmd|=HASH_CMD_SHA224|HASH_CMD_SHA_SWAP;r.flags|=SHA_FLAGS_SHA224;r.digsize=SHA224_DIGEST_SIZE;r.block_size=SHA224_BLOCK_SIZE;r.ivsize=32},SHA256_DIGEST_SIZE=>{r.cmd|=HASH_CMD_SHA256|HASH_CMD_SHA_SWAP;r.flags|=SHA_FLAGS_SHA256;r.digsize=SHA256_DIGEST_SIZE;r.block_size=SHA256_BLOCK_SIZE;r.ivsize=32},SHA384_DIGEST_SIZE=>{r.cmd|=HASH_CMD_SHA512_SER|HASH_CMD_SHA384|HASH_CMD_SHA_SWAP;r.flags|=SHA_FLAGS_SHA384;r.digsize=SHA384_DIGEST_SIZE;r.block_size=SHA384_BLOCK_SIZE;r.ivsize=64},SHA512_DIGEST_SIZE=>{r.cmd|=HASH_CMD_SHA512_SER|HASH_CMD_SHA512|HASH_CMD_SHA_SWAP;r.flags|=SHA_FLAGS_SHA512;r.digsize=SHA512_DIGEST_SIZE;r.block_size=SHA512_BLOCK_SIZE;r.ivsize=64},_=>{dev_warn(t.hace_dev.dev,"digest size not support");return -EINVAL}}r.total=0;r.digcnt=[0,0];0}
pub unsafe fn aspeed_register_hace_hash_algs(h:*mut aspeed_hace_dev){for a in aspeed_ahash_algs.iter_mut(){a.hace_dev=h;crypto_engine_register_ahash(&mut a.alg.ahash);}if (*h).version==AST2600_VERSION{for a in aspeed_ahash_algs_g6.iter_mut(){a.hace_dev=h;crypto_engine_register_ahash(&mut a.alg.ahash);}}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
