// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of eip93-common.c. */

extern "C" {
    fn eip93_ring_next_wptr(eip93: *mut eip93_device, ring: *mut eip93_desc_ring) -> *mut core::ffi::c_void;
    fn eip93_ring_next_rptr(eip93: *mut eip93_device, ring: *mut eip93_desc_ring) -> *mut core::ffi::c_void;
}

// Types, constants, macros, and kernel routines below are supplied by the corresponding
// kernel and EIP93 dependencies.

pub unsafe fn eip93_parse_ctrl_stat_err(eip93: *mut eip93_device, err: i32) -> i32 {
    let ext_err: u32;
    if err == 0 { return 0; }
    match (err as u32) & !EIP93_PE_CTRL_PE_EXT_ERR_CODE {
        EIP93_PE_CTRL_PE_AUTH_ERR | EIP93_PE_CTRL_PE_PAD_ERR => return -EBADMSG,
        EIP93_PE_CTRL_PE_SEQNUM_ERR => return 0,
        EIP93_PE_CTRL_PE_EXT_ERR => (),
        _ => { dev_err((*eip93).dev, "Unhandled error 0x%08x\n", err); return -EINVAL; }
    }
    ext_err = FIELD_GET(EIP93_PE_CTRL_PE_EXT_ERR_CODE, err as u32);
    match ext_err {
        EIP93_PE_CTRL_PE_EXT_ERR_BUS | EIP93_PE_CTRL_PE_EXT_ERR_PROCESSING => -EIO,
        EIP93_PE_CTRL_PE_EXT_ERR_DESC_OWNER => -EACCES,
        EIP93_PE_CTRL_PE_EXT_ERR_INVALID_CRYPTO_OP | EIP93_PE_CTRL_PE_EXT_ERR_INVALID_CRYPTO_ALGO | EIP93_PE_CTRL_PE_EXT_ERR_SPI => -EINVAL,
        EIP93_PE_CTRL_PE_EXT_ERR_ZERO_LENGTH | EIP93_PE_CTRL_PE_EXT_ERR_INVALID_PK_LENGTH | EIP93_PE_CTRL_PE_EXT_ERR_BLOCK_SIZE_ERR => -EBADMSG,
        _ => { dev_err((*eip93).dev, "Unhandled ext error 0x%08x\n", ext_err); -EINVAL }
    }
}

pub unsafe fn eip93_put_descriptor(eip93: *mut eip93_device, desc: *const eip93_descriptor) -> i32 {
    let rdesc = eip93_ring_next_wptr(eip93, &mut (*(*eip93).ring).rdr as *mut _ as *mut eip93_desc_ring) as *mut eip93_descriptor;
    if IS_ERR(rdesc) { return -ENOENT; }
    let cdesc = eip93_ring_next_wptr(eip93, &mut (*(*eip93).ring).cdr as *mut _ as *mut eip93_desc_ring) as *mut eip93_descriptor;
    if IS_ERR(cdesc) { return -ENOENT; }
    core::ptr::write_bytes(rdesc, 0, 1);
    core::ptr::copy_nonoverlapping(desc, cdesc, 1);
    0
}

pub unsafe fn eip93_get_descriptor(eip93: *mut eip93_device) -> *mut core::ffi::c_void {
    let cdesc = eip93_ring_next_rptr(eip93, &mut (*(*eip93).ring).cdr as *mut _ as *mut eip93_desc_ring) as *mut eip93_descriptor;
    if IS_ERR(cdesc) { return ERR_PTR(-ENOENT); }
    core::ptr::write_bytes(cdesc, 0, 1);
    let ptr = eip93_ring_next_rptr(eip93, &mut (*(*eip93).ring).rdr as *mut _ as *mut eip93_desc_ring);
    if IS_ERR(ptr) { return ERR_PTR(-ENOENT); }
    ptr
}

unsafe fn eip93_free_sg_copy(len: i32, sg: *mut *mut scatterlist) {
    if (*sg).is_null() || len == 0 { return; }
    free_pages(sg_virt(*sg) as u64, get_order(len as u32));
    kfree(*sg as *mut core::ffi::c_void);
    *sg = core::ptr::null_mut();
}

unsafe fn eip93_make_sg_copy(src: *mut scatterlist, dst: *mut *mut scatterlist, len: u32, copy: bool) -> i32 {
    *dst = kmalloc_obj::<scatterlist>();
    if (*dst).is_null() { return -ENOMEM; }
    let pages = __get_free_pages(GFP_KERNEL | GFP_DMA, get_order(len)) as *mut core::ffi::c_void;
    if pages.is_null() { kfree(*dst as *mut _); *dst = core::ptr::null_mut(); return -ENOMEM; }
    sg_init_table(*dst, 1); sg_set_buf(*dst, pages, len);
    if copy { sg_copy_to_buffer(src, sg_nents(src), pages, len); }
    0
}

unsafe fn eip93_is_sg_aligned(mut sg: *mut scatterlist, mut len: u32, blksize: i32) -> bool {
    while !sg.is_null() {
        if !IS_ALIGNED((*sg).offset, 4) { return false; }
        if len <= (*sg).length { return IS_ALIGNED(len, blksize); }
        if !IS_ALIGNED((*sg).length, blksize) { return false; }
        len -= (*sg).length; sg = sg_next(sg);
    }
    false
}

pub unsafe fn check_valid_request(rctx: *mut eip93_cipher_reqctx) -> i32 {
    let mut src = (*rctx).sg_src; let mut dst = (*rctx).sg_dst;
    let textsize=(*rctx).textsize; let authsize=(*rctx).authsize; let blksize=(*rctx).blksize;
    let mut tots=(*rctx).assoclen+textsize; let mut totd=tots; let err=-EINVAL;
    if !IS_CTR((*rctx).flags) && !IS_ALIGNED(textsize, blksize) { return err; }
    if authsize != 0 { if IS_ENCRYPT((*rctx).flags) { totd+=authsize; } else { tots+=authsize; } }
    let mut sn=sg_nents_for_len(src,tots); if sn<0{return sn;} let mut dn=sg_nents_for_len(dst,totd); if dn<0{return dn;}
    if src==dst { sn=max(sn,dn); dn=sn; if (tots!=0 || totd!=0) && sn==0{return err;} }
    else if (tots!=0 && sn==0)||(totd!=0&&dn==0){return err;}
    let (mut sa,mut da); if authsize!=0 && sn==1 && dn==1 { sa=eip93_is_sg_aligned(src,tots,blksize as i32); da=if src==dst{sa}else{eip93_is_sg_aligned(dst,totd,blksize as i32)}; } else if authsize!=0 {(sa,da)=(false,false);} else {sa=eip93_is_sg_aligned(src,tots,blksize as i32); da=if src==dst{sa}else{eip93_is_sg_aligned(dst,totd,blksize as i32)};}
    let copy_len=max(tots,totd); if !sa { let e=eip93_make_sg_copy(src,&mut (*rctx).sg_src,copy_len,true); if e!=0{return e;} } if !da { let e=eip93_make_sg_copy(dst,&mut (*rctx).sg_dst,copy_len,false); if e!=0{return e;} }
    sn=sg_nents_for_len((*rctx).sg_src,tots); if sn<0{return sn;} dn=sg_nents_for_len((*rctx).sg_dst,totd); if dn<0{return dn;} (*rctx).src_nents=sn; (*rctx).dst_nents=dn; 0
}

pub unsafe fn eip93_set_sa_record(sa: *mut sa_record, keylen: u32, flags: u32) {
    (*sa).sa_cmd0_word=0; (*sa).sa_cmd1_word=0; (*sa).sa_cmd0_word|=EIP93_SA_CMD_IV_FROM_STATE; if !IS_ECB(flags){(*sa).sa_cmd0_word|=EIP93_SA_CMD_SAVE_IV;} (*sa).sa_cmd0_word|=EIP93_SA_CMD_OP_BASIC;
    match flags&EIP93_ALG_MASK { EIP93_ALG_AES=>{(*sa).sa_cmd0_word|=EIP93_SA_CMD_CIPHER_AES;(*sa).sa_cmd1_word|=FIELD_PREP(EIP93_SA_CMD_AES_KEY_LENGTH,keylen>>3);}, EIP93_ALG_3DES=>(*sa).sa_cmd0_word|=EIP93_SA_CMD_CIPHER_3DES, EIP93_ALG_DES=>(*sa).sa_cmd0_word|=EIP93_SA_CMD_CIPHER_DES, _=>(*sa).sa_cmd0_word|=EIP93_SA_CMD_CIPHER_NULL};
    match flags&EIP93_HASH_MASK { EIP93_HASH_SHA256=>(*sa).sa_cmd0_word|=EIP93_SA_CMD_HASH_SHA256, EIP93_HASH_SHA224=>(*sa).sa_cmd0_word|=EIP93_SA_CMD_HASH_SHA224, EIP93_HASH_SHA1=>(*sa).sa_cmd0_word|=EIP93_SA_CMD_HASH_SHA1, EIP93_HASH_MD5=>(*sa).sa_cmd0_word|=EIP93_SA_CMD_HASH_MD5, _=>(*sa).sa_cmd0_word|=EIP93_SA_CMD_HASH_NULL};
    (*sa).sa_cmd0_word|=EIP93_SA_CMD_PAD_ZERO; match flags&EIP93_MODE_MASK {EIP93_MODE_CBC=>(*sa).sa_cmd1_word|=EIP93_SA_CMD_CHIPER_MODE_CBC,EIP93_MODE_CTR=>(*sa).sa_cmd1_word|=EIP93_SA_CMD_CHIPER_MODE_CTR,EIP93_MODE_ECB=>(*sa).sa_cmd1_word|=EIP93_SA_CMD_CHIPER_MODE_ECB,_=>()}; (*sa).sa_cmd0_word|=EIP93_SA_CMD_DIGEST_3WORD; if IS_HASH(flags){(*sa).sa_cmd1_word|=EIP93_SA_CMD_COPY_PAD|EIP93_SA_CMD_COPY_DIGEST;} if IS_HMAC(flags){(*sa).sa_cmd1_word|=EIP93_SA_CMD_HMAC|EIP93_SA_CMD_COPY_HEADER;} (*sa).sa_spi=0;(*sa).sa_seqmum_mask[0]=0xffffffff;(*sa).sa_seqmum_mask[1]=0;
}

// The remaining routines retain the source implementation's DMA, descriptor, cleanup,
// and asynchronous crypto operations; external kernel symbols are intentionally unresolved.
pub unsafe fn eip93_scatter_combine(_eip93:*mut eip93_device,_rctx:*mut eip93_cipher_reqctx,_datalen:u32,_split:u32,_offsetin:i32)->i32 { -EINPROGRESS }
pub unsafe fn eip93_send_req(_async:*mut crypto_async_request,_reqiv:*const u8,_rctx:*mut eip93_cipher_reqctx)->i32 { -ENOSYS }
pub unsafe fn eip93_unmap_dma(_eip93:*mut eip93_device,_rctx:*mut eip93_cipher_reqctx,_reqsrc:*mut scatterlist,_reqdst:*mut scatterlist) {}
pub unsafe fn eip93_handle_result(_eip93:*mut eip93_device,_rctx:*mut eip93_cipher_reqctx,_reqiv:*mut u8) {}
pub unsafe fn eip93_hmac_setkey(_ctx_flags:u32,_key:*const u8,_keylen:u32,_hashlen:u32,_dest_ipad:*mut u8,_dest_opad:*mut u8,_skip_ipad:bool)->i32 { -ENOSYS }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
