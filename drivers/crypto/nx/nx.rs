// SPDX-License-Identifier: GPL-2.0-only
/*
 * Routines supporting the Power 7+ Nest Accelerators driver
 *
 * Copyright (C) 2011-2012 International Business Machines Inc.
 *
 * Author: Kent Yoder <yoder1@us.ibm.com>
 */

// Dependencies are supplied by the surrounding kernel/Rust translation.

pub unsafe fn nx_hcall_sync(nx_ctx: *mut nx_crypto_ctx, op: *mut vio_pfo_op, may_sleep: u32) -> i32 {
    let mut rc: i32;
    let mut retries: i32 = 10;
    let viodev = nx_driver.viodev;

    atomic_inc(&mut (*(*nx_ctx).stats).sync_ops);
    loop {
        rc = vio_h_cop_sync(viodev, op);
        if !(rc == -EBUSY && may_sleep == 0 && { retries -= 1; retries + 1 != 0 }) { break; }
    }
    if rc != 0 {
        dev_dbg(&(*viodev).dev, "vio_h_cop_sync failed: rc: %d hcall rc: %ld\n", rc, (*op).hcall_err);
        atomic_inc(&mut (*(*nx_ctx).stats).errors);
        atomic_set(&mut (*(*nx_ctx).stats).last_error, (*op).hcall_err);
        atomic_set(&mut (*(*nx_ctx).stats).last_error_pid, current.pid);
    }
    rc
}

pub unsafe fn nx_build_sg_list(mut sg_head: *mut nx_sg, start_addr: *mut u8, len: *mut u32, sgmax: u32) -> *mut nx_sg {
    let mut sg_len: u32 = 0;
    let mut sg = sg_head;
    let mut sg_addr = start_addr as u64;
    let mut end_addr: u64;
    if is_vmalloc_addr(start_addr) {
        sg_addr = page_to_phys(vmalloc_to_page(start_addr)) + offset_in_page(sg_addr);
    } else { sg_addr = __pa(sg_addr); }
    end_addr = sg_addr + *len as u64;
    while sg_len < *len {
        let next_page: u64;
        (*sg).addr = sg_addr;
        sg_addr = min_t(sg_addr + NX_PAGE_SIZE as u64, end_addr);
        next_page = (sg_addr & PAGE_MASK as u64) + PAGE_SIZE as u64;
        (*sg).len = (min_t(sg_addr, next_page) - (*sg).addr) as u32;
        sg_len += (*sg).len;
        if sg_addr >= next_page && is_vmalloc_addr(start_addr.add(sg_len as usize)) {
            sg_addr = page_to_phys(vmalloc_to_page(start_addr.add(sg_len as usize)));
            end_addr = sg_addr + *len as u64 - sg_len as u64;
        }
        sg = sg.add(1);
        if sg.offset_from(sg_head) == sgmax as isize { break; }
    }
    *len = sg_len;
    sg
}

pub unsafe fn nx_walk_and_build(mut nx_dst: *mut nx_sg, sglen: u32, sg_src: *mut scatterlist, start: u32, src_len: *mut u32) -> *mut nx_sg {
    let mut walk: scatter_walk = core::mem::zeroed();
    let nx_head = nx_dst;
    let mut len = *src_len;
    scatterwalk_start_at_pos(&mut walk, sg_src, start);
    while len != 0 && nx_dst.offset_from(nx_head) < sglen as isize {
        let mut n = scatterwalk_next(&mut walk, len);
        nx_dst = nx_build_sg_list(nx_dst, walk.addr, &mut n, sglen - nx_dst.offset_from(nx_head) as u32);
        scatterwalk_done_src(&mut walk, n);
        len -= n;
    }
    *src_len -= len;
    nx_dst
}

unsafe fn trim_sg_list(mut sg: *mut nx_sg, mut end: *mut nx_sg, mut delta: u32, nbytes: *mut u32) -> isize {
    let is_delta = delta;
    while delta != 0 && end > sg {
        let last = end.sub(1);
        if (*last).len > delta { (*last).len -= delta; delta = 0; }
        else { end = end.sub(1); delta -= (*last).len; }
    }
    let oplen = sg.offset_from(end) * core::mem::size_of::<nx_sg>() as isize;
    if is_delta != 0 {
        let data_back = *nbytes - ((oplen.unsigned_abs() as u32 / AES_BLOCK_SIZE) * (*sg).len & !(AES_BLOCK_SIZE - 1));
        *nbytes -= data_back;
    }
    oplen
}

pub unsafe fn nx_build_sg_lists(nx_ctx: *mut nx_crypto_ctx, iv: *const u8, dst: *mut scatterlist, src: *mut scatterlist, nbytes: *mut u32, offset: u32, oiv: *mut u8) -> i32 {
    let mut delta = 0;
    let total = *nbytes;
    let mut nx_insg = (*nx_ctx).in_sg;
    let mut nx_outsg = (*nx_ctx).out_sg;
    let mut max_sg_len = min_t((*nx_ctx).ap.sglen, nx_driver.of.max_sg_len / core::mem::size_of::<nx_sg>() as u32);
    max_sg_len = min_t(max_sg_len, (*nx_ctx).ap.databytelen / NX_PAGE_SIZE);
    if !oiv.is_null() { core::ptr::copy_nonoverlapping(iv, oiv, AES_BLOCK_SIZE as usize); }
    *nbytes = min_t(*nbytes, (*nx_ctx).ap.databytelen);
    nx_outsg = nx_walk_and_build(nx_outsg, max_sg_len, dst, offset, nbytes);
    nx_insg = nx_walk_and_build(nx_insg, max_sg_len, src, offset, nbytes);
    if *nbytes < total { delta = *nbytes - (*nbytes & !(AES_BLOCK_SIZE - 1)); }
    (*nx_ctx).op.inlen = trim_sg_list((*nx_ctx).in_sg, nx_insg, delta, nbytes);
    (*nx_ctx).op.outlen = trim_sg_list((*nx_ctx).out_sg, nx_outsg, delta, nbytes);
    0
}

pub unsafe fn nx_ctx_init(nx_ctx: *mut nx_crypto_ctx, function: u32) {
    spin_lock_init(&mut (*nx_ctx).lock);
    core::ptr::write_bytes((*nx_ctx).kmem, 0, (*nx_ctx).kmem_len as usize);
    (*(*nx_ctx).csbcpb).csb.valid |= NX_CSB_VALID_BIT;
    (*nx_ctx).op.flags = function;
    (*nx_ctx).op.csbcpb = __pa((*nx_ctx).csbcpb);
    (*nx_ctx).op.in_ = __pa((*nx_ctx).in_sg);
    (*nx_ctx).op.out = __pa((*nx_ctx).out_sg);
    if !(*nx_ctx).csbcpb_aead.is_null() {
        (*(*nx_ctx).csbcpb_aead).csb.valid |= NX_CSB_VALID_BIT;
        (*nx_ctx).op_aead.flags = function;
        (*nx_ctx).op_aead.csbcpb = __pa((*nx_ctx).csbcpb_aead);
        (*nx_ctx).op_aead.in_ = __pa((*nx_ctx).in_sg);
        (*nx_ctx).op_aead.out = __pa((*nx_ctx).out_sg);
    }
}

unsafe fn nx_of_update_status(dev: *mut device, p: *mut property, props: *mut nx_of) {
    if !strncmp((*p).value, b"okay\0".as_ptr(), (*p).length) { (*props).status = NX_WAITING; (*props).flags |= NX_OF_FLAG_STATUS_SET; }
    else { dev_info(dev, "%s: status '%s' is not 'okay'\n", __func__, (*p).value); }
}
unsafe fn nx_of_update_sglen(dev: *mut device, p: *mut property, props: *mut nx_of) {
    if (*p).length != core::mem::size_of_val(&(*props).max_sg_len) as u32 { dev_err(dev, "%s: unexpected format for ibm,max-sg-len property\n", __func__); dev_dbg(dev, "%s: ibm,max-sg-len is %d bytes long, expected %zd bytes\n", __func__, (*p).length, core::mem::size_of_val(&(*props).max_sg_len)); return; }
    (*props).max_sg_len = *( (*p).value as *const u32); (*props).flags |= NX_OF_FLAG_MAXSGLEN_SET;
}

unsafe fn nx_of_update_msc(dev: *mut device, p: *mut property, props: *mut nx_of) {
    let mut msc = (*p).value as *mut max_sync_cop; let lenp = (*p).length; let mut bytes_so_far = 0;
    while bytes_so_far + core::mem::size_of::<max_sync_cop>() as u32 <= lenp {
        bytes_so_far += core::mem::size_of::<max_sync_cop>() as u32; let mut trip = (*msc).trip;
        for i in 0..(*msc).triplets { if bytes_so_far + core::mem::size_of::<msc_triplet>() as u32 > lenp { break; }
            if (*msc).fc >= NX_MAX_FC || (*msc).mode >= NX_MAX_MODE { dev_err(dev, "unknown function code/mode combo: %d/%d (ignored)\n", (*msc).fc, (*msc).mode); bytes_so_far += core::mem::size_of::<msc_triplet>() as u32; trip = trip.add(1); continue; }
            if (*trip).sglen == 0 || (*trip).databytelen < NX_PAGE_SIZE { dev_warn(dev, "bogus sglen/databytelen: %u/%u (ignored)\n", (*trip).sglen, (*trip).databytelen); bytes_so_far += core::mem::size_of::<msc_triplet>() as u32; trip = trip.add(1); continue; }
            let slot = match (*trip).keybitlen { 128 | 160 => 0, 192 => 1, 256 if (*msc).fc == NX_FC_AES => 2, 256 if (*msc).fc == NX_FC_AES_HMAC || (*msc).fc == NX_FC_SHA => 1, 512 => 2, _ => { bytes_so_far += core::mem::size_of::<msc_triplet>() as u32; trip = trip.add(1); continue; } };
            (*props).ap[(*msc).fc as usize][(*msc).mode as usize][slot].databytelen = (*trip).databytelen; (*props).ap[(*msc).fc as usize][(*msc).mode as usize][slot].sglen = (*trip).sglen;
            bytes_so_far += core::mem::size_of::<msc_triplet>() as u32; trip = trip.add(1);
            let _ = i;
        }
        msc = trip as *mut max_sync_cop;
    }
    (*props).flags |= NX_OF_FLAG_MAXSYNCCOP_SET;
}

unsafe fn nx_of_init(dev: *mut device, props: *mut nx_of) {
    let base_node = (*dev).of_node;
    let p = of_find_property(base_node, "status", core::ptr::null_mut()); if !p.is_null() { nx_of_update_status(dev, p, props); }
    let p = of_find_property(base_node, "ibm,max-sg-len", core::ptr::null_mut()); if !p.is_null() { nx_of_update_sglen(dev, p, props); }
    let p = of_find_property(base_node, "ibm,max-sync-cop", core::ptr::null_mut()); if !p.is_null() { nx_of_update_msc(dev, p, props); }
}

unsafe fn nx_check_prop(dev: *mut device, fc: u32, mode: u32, slot: i32) -> bool { let p = &nx_driver.of.ap[fc as usize][mode as usize][slot as usize]; if p.sglen == 0 || p.databytelen < NX_PAGE_SIZE { if !dev.is_null() { dev_warn(dev, "bogus sglen/databytelen for %u/%u/%u: %u/%u (ignored)\n", fc, mode, slot, p.sglen, p.databytelen); } return false; } true }
unsafe fn nx_check_props(dev: *mut device, fc: u32, mode: u32) -> bool { for i in 0..3 { if !nx_check_prop(dev, fc, mode, i) { return false; } } true }
unsafe fn nx_register_skcipher(alg: *mut skcipher_alg, fc: u32, mode: u32) -> i32 { if nx_check_props(&mut nx_driver.viodev.dev, fc, mode) { crypto_register_skcipher(alg) } else { 0 } }
unsafe fn nx_register_aead(alg: *mut aead_alg, fc: u32, mode: u32) -> i32 { if nx_check_props(&mut nx_driver.viodev.dev, fc, mode) { crypto_register_aead(alg) } else { 0 } }
unsafe fn nx_register_shash(alg: *mut shash_alg, fc: u32, mode: u32, slot: i32) -> i32 { if if slot >= 0 { nx_check_prop(&mut nx_driver.viodev.dev, fc, mode, slot) } else { nx_check_props(&mut nx_driver.viodev.dev, fc, mode) } { crypto_register_shash(alg) } else { 0 } }
unsafe fn nx_unregister_skcipher(alg: *mut skcipher_alg, fc: u32, mode: u32) { if nx_check_props(core::ptr::null_mut(), fc, mode) { crypto_unregister_skcipher(alg); } }
unsafe fn nx_unregister_aead(alg: *mut aead_alg, fc: u32, mode: u32) { if nx_check_props(core::ptr::null_mut(), fc, mode) { crypto_unregister_aead(alg); } }
unsafe fn nx_unregister_shash(alg: *mut shash_alg, fc: u32, mode: u32, slot: i32) { if if slot >= 0 { nx_check_prop(core::ptr::null_mut(), fc, mode, slot) } else { nx_check_props(core::ptr::null_mut(), fc, mode) } { crypto_unregister_shash(alg); } }

unsafe fn nx_register_algs() -> i32 {
    let mut rc = -1; if nx_driver.of.flags != NX_OF_FLAG_MASK_READY { return rc; }
    core::ptr::write_bytes(&mut nx_driver.stats as *mut _, 0, 1); NX_DEBUGFS_INIT(&mut nx_driver); nx_driver.of.status = NX_OKAY;
    let regs: [(i32, i32); 10] = [(nx_register_skcipher(&mut nx_ecb_aes_alg, NX_FC_AES, NX_MODE_AES_ECB),0),(nx_register_skcipher(&mut nx_cbc_aes_alg,NX_FC_AES,NX_MODE_AES_CBC),0),(nx_register_skcipher(&mut nx_ctr3686_aes_alg,NX_FC_AES,NX_MODE_AES_CTR),0),(nx_register_aead(&mut nx_gcm_aes_alg,NX_FC_AES,NX_MODE_AES_GCM),0),(nx_register_aead(&mut nx_gcm4106_aes_alg,NX_FC_AES,NX_MODE_AES_GCM),0),(nx_register_aead(&mut nx_ccm_aes_alg,NX_FC_AES,NX_MODE_AES_CCM),0),(nx_register_aead(&mut nx_ccm4309_aes_alg,NX_FC_AES,NX_MODE_AES_CCM),0),(nx_register_shash(&mut nx_shash_sha256_alg,NX_FC_SHA,NX_MODE_SHA,NX_PROPS_SHA256),0),(nx_register_shash(&mut nx_shash_sha512_alg,NX_FC_SHA,NX_MODE_SHA,NX_PROPS_SHA512),0),(nx_register_shash(&mut nx_shash_aes_xcbc_alg,NX_FC_AES,NX_MODE_AES_XCBC_MAC,-1),0)]; let _ = regs; rc = 0; rc
}

unsafe fn nx_crypto_ctx_init(nx_ctx: *mut nx_crypto_ctx, fc: u32, mode: u32) -> i32 {
    if nx_driver.of.status != NX_OKAY { pr_err("Attempt to initialize NX crypto context while device is not available!\n"); return -ENODEV; }
    (*nx_ctx).kmem_len = if mode == NX_MODE_AES_GCM || mode == NX_MODE_AES_CCM { 5 * NX_PAGE_SIZE + core::mem::size_of::<nx_csbcpb>() as u32 } else { 4 * NX_PAGE_SIZE + core::mem::size_of::<nx_csbcpb>() as u32 };
    (*nx_ctx).kmem = kmalloc((*nx_ctx).kmem_len, GFP_KERNEL); if (*nx_ctx).kmem.is_null() { return -ENOMEM; }
    (*nx_ctx).csbcpb = round_up((*nx_ctx).kmem as u64, NX_PAGE_SIZE as u64) as *mut nx_csbcpb;
    (*nx_ctx).in_sg = ((*nx_ctx).csbcpb as *mut u8).add(NX_PAGE_SIZE as usize) as *mut nx_sg; (*nx_ctx).out_sg = ((*nx_ctx).in_sg as *mut u8).add(NX_PAGE_SIZE as usize) as *mut nx_sg;
    if mode == NX_MODE_AES_GCM || mode == NX_MODE_AES_CCM { (*nx_ctx).csbcpb_aead = ((*nx_ctx).out_sg as *mut u8).add(NX_PAGE_SIZE as usize) as *mut nx_csbcpb; }
    (*nx_ctx).stats = &mut nx_driver.stats; core::ptr::copy_nonoverlapping(nx_driver.of.ap[fc as usize][mode as usize].as_ptr(), (*nx_ctx).props.as_mut_ptr(), 3); 0
}

pub unsafe fn nx_crypto_ctx_aes_ccm_init(tfm: *mut crypto_aead) -> i32 { crypto_aead_set_reqsize(tfm, core::mem::size_of::<nx_ccm_rctx>()); nx_crypto_ctx_init(crypto_aead_ctx(tfm), NX_FC_AES, NX_MODE_AES_CCM) }
pub unsafe fn nx_crypto_ctx_aes_gcm_init(tfm: *mut crypto_aead) -> i32 { crypto_aead_set_reqsize(tfm, core::mem::size_of::<nx_gcm_rctx>()); nx_crypto_ctx_init(crypto_aead_ctx(tfm), NX_FC_AES, NX_MODE_AES_GCM) }
pub unsafe fn nx_crypto_ctx_aes_ctr_init(tfm: *mut crypto_skcipher) -> i32 { nx_crypto_ctx_init(crypto_skcipher_ctx(tfm), NX_FC_AES, NX_MODE_AES_CTR) }
pub unsafe fn nx_crypto_ctx_aes_cbc_init(tfm: *mut crypto_skcipher) -> i32 { nx_crypto_ctx_init(crypto_skcipher_ctx(tfm), NX_FC_AES, NX_MODE_AES_CBC) }
pub unsafe fn nx_crypto_ctx_aes_ecb_init(tfm: *mut crypto_skcipher) -> i32 { nx_crypto_ctx_init(crypto_skcipher_ctx(tfm), NX_FC_AES, NX_MODE_AES_ECB) }
pub unsafe fn nx_crypto_ctx_sha_init(tfm: *mut crypto_shash) -> i32 { nx_crypto_ctx_init(crypto_shash_ctx(tfm), NX_FC_SHA, NX_MODE_SHA) }
pub unsafe fn nx_crypto_ctx_aes_xcbc_init(tfm: *mut crypto_shash) -> i32 { nx_crypto_ctx_init(crypto_shash_ctx(tfm), NX_FC_AES, NX_MODE_AES_XCBC_MAC) }
pub unsafe fn nx_crypto_ctx_exit(nx_ctx: *mut nx_crypto_ctx) { kfree_sensitive((*nx_ctx).kmem); (*nx_ctx).csbcpb = core::ptr::null_mut(); (*nx_ctx).csbcpb_aead = core::ptr::null_mut(); (*nx_ctx).in_sg = core::ptr::null_mut(); (*nx_ctx).out_sg = core::ptr::null_mut(); }
pub unsafe fn nx_crypto_ctx_skcipher_exit(tfm: *mut crypto_skcipher) { nx_crypto_ctx_exit(crypto_skcipher_ctx(tfm)); }
pub unsafe fn nx_crypto_ctx_aead_exit(tfm: *mut crypto_aead) { kfree_sensitive((*crypto_aead_ctx(tfm)).kmem); }
pub unsafe fn nx_crypto_ctx_shash_exit(tfm: *mut crypto_shash) { nx_crypto_ctx_exit(crypto_shash_ctx(tfm)); }

unsafe fn nx_probe(viodev: *mut vio_dev, _id: *const vio_device_id) -> i32 { if !nx_driver.viodev.is_null() { return -EINVAL; } nx_driver.viodev = viodev; nx_of_init(&mut (*viodev).dev, &mut nx_driver.of); nx_register_algs() }
unsafe fn nx_remove(_viodev: *mut vio_dev) { if nx_driver.of.status == NX_OKAY { NX_DEBUGFS_FINI(&mut nx_driver); nx_unregister_shash(&mut nx_shash_aes_xcbc_alg,NX_FC_AES,NX_MODE_AES_XCBC_MAC,-1); nx_unregister_shash(&mut nx_shash_sha512_alg,NX_FC_SHA,NX_MODE_SHA,NX_PROPS_SHA256); nx_unregister_shash(&mut nx_shash_sha256_alg,NX_FC_SHA,NX_MODE_SHA,NX_PROPS_SHA512); nx_unregister_aead(&mut nx_ccm4309_aes_alg,NX_FC_AES,NX_MODE_AES_CCM); nx_unregister_aead(&mut nx_ccm_aes_alg,NX_FC_AES,NX_MODE_AES_CCM); nx_unregister_aead(&mut nx_gcm4106_aes_alg,NX_FC_AES,NX_MODE_AES_GCM); nx_unregister_aead(&mut nx_gcm_aes_alg,NX_FC_AES,NX_MODE_AES_GCM); nx_unregister_skcipher(&mut nx_ctr3686_aes_alg,NX_FC_AES,NX_MODE_AES_CTR); nx_unregister_skcipher(&mut nx_cbc_aes_alg,NX_FC_AES,NX_MODE_AES_CBC); nx_unregister_skcipher(&mut nx_ecb_aes_alg,NX_FC_AES,NX_MODE_AES_ECB); } }
unsafe fn nx_init() -> i32 { vio_register_driver(&mut nx_driver.viodriver) }
unsafe fn nx_fini() { vio_unregister_driver(&mut nx_driver.viodriver); }

static mut nx_crypto_driver_ids: [vio_device_id; 2] = [vio_device_id { type_: "ibm,sym-encryption-v1", compat: "ibm,sym-encryption" }, vio_device_id { type_: "", compat: "" }];
pub static mut nx_driver: nx_crypto_driver = nx_crypto_driver { viodriver: vio_driver { id_table: nx_crypto_driver_ids.as_ptr(), probe: Some(nx_probe), remove: Some(nx_remove), name: NX_NAME }, ..unsafe { core::mem::zeroed() } };

// module_init(nx_init); module_exit(nx_fini);
// MODULE_AUTHOR("Kent Yoder <yoder1@us.ibm.com>");
// MODULE_DESCRIPTION(NX_STRING); MODULE_LICENSE("GPL"); MODULE_VERSION(NX_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
