// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2016 Broadcom
 */

// Translated from util.c. Kernel and driver symbols are supplied by other files.

const SPU_OFIFO_CTRL: u32 = 0x40;
const SPU_FIFO_WATERMARK: u32 = 0x1FF;

pub unsafe fn spu_sg_at_offset(
    mut sg: *mut scatterlist,
    skip: u32,
    sge: *mut *mut scatterlist,
    sge_offset: *mut u32,
) -> i32 {
    let mut index: u32 = 0;
    let mut next_index: u32 = (*sg).length;
    while next_index <= skip {
        sg = sg_next(sg);
        index = next_index;
        if sg.is_null() {
            return -EINVAL;
        }
        next_index += (*sg).length;
    }
    *sge_offset = skip - index;
    *sge = sg;
    0
}

pub unsafe fn sg_copy_part_to_buf(
    src: *mut scatterlist,
    dest: *mut u8,
    len: u32,
    skip: u32,
) {
    let nents: u32 = sg_nents(src);
    let copied: usize = sg_pcopy_to_buffer(src, nents, dest, len, skip);
    if copied != len as usize {
        flow_log!("%s copied %u bytes of %u requested. ", __func__, copied as u32, len);
        flow_log!("sg with %u entries and skip %u\n", nents, skip);
    }
}

pub unsafe fn sg_copy_part_from_buf(
    dest: *mut scatterlist,
    src: *mut u8,
    len: u32,
    skip: u32,
) {
    let nents: u32 = sg_nents(dest);
    let copied: usize = sg_pcopy_from_buffer(dest, nents, src, len, skip);
    if copied != len as usize {
        flow_log!("%s copied %u bytes of %u requested. ", __func__, copied as u32, len);
        flow_log!("sg with %u entries and skip %u\n", nents, skip);
    }
}

pub unsafe fn spu_sg_count(sg_list: *mut scatterlist, skip: u32, mut nbytes: i32) -> i32 {
    if sg_list.is_null() {
        return 0;
    }
    let mut sg: *mut scatterlist = core::ptr::null_mut();
    let mut offset: u32 = 0;
    if spu_sg_at_offset(sg_list, skip, &mut sg, &mut offset) < 0 {
        return 0;
    }
    let mut sg_nents = 0;
    while !sg.is_null() && nbytes > 0 {
        sg_nents += 1;
        nbytes -= ((*sg).length - offset) as i32;
        offset = 0;
        sg = sg_next(sg);
    }
    sg_nents
}

pub unsafe fn spu_msg_sg_add(
    to_sg: *mut *mut scatterlist,
    from_sg: *mut *mut scatterlist,
    from_skip: *mut u32,
    from_nents: u8,
    length: u32,
) -> u32 {
    let mut to = *to_sg;
    let mut from = *from_sg;
    let mut skip = *from_skip;
    let mut frag_len = 0u32;
    let mut copied = 0u32;
    if length == 0 { return 0; }
    let mut i = 0;
    while i < from_nents {
        let entry_len = (*from).length - skip;
        frag_len = core::cmp::min(entry_len, length - copied);
        let offset = (*from).offset + skip;
        if frag_len != 0 {
            sg_set_page(to, sg_page(from), frag_len, offset);
            to = to.add(1);
        }
        copied += frag_len;
        if copied == entry_len { skip = 0; }
        if copied == length { break; }
        from = sg_next(from);
        i += 1;
    }
    *to_sg = to;
    *from_sg = from;
    *from_skip = if frag_len < (*from).length - skip { skip + frag_len } else { 0 };
    copied
}

pub unsafe fn add_to_ctr(ctr_pos: *mut u8, increment: u32) {
    let high_be = ctr_pos as *mut u64;
    let low_be = high_be.add(1);
    let orig_low = u64::from_be(*low_be);
    let new_low = orig_low.wrapping_add(increment as u64);
    *low_be = new_low.to_be();
    if new_low < orig_low {
        *high_be = u64::from_be(*high_be).wrapping_add(1).to_be();
    }
}

#[repr(C)]
pub struct sdesc {
    pub shash: shash_desc,
    pub ctx: [u8; 0],
}

pub unsafe fn do_shash(
    name: *mut u8, result: *mut u8,
    data1: *const u8, data1_len: u32,
    data2: *const u8, data2_len: u32,
    key: *const u8, key_len: u32,
) -> i32 {
    let hash = crypto_alloc_shash(name, 0, 0);
    if IS_ERR(hash) {
        let rc = PTR_ERR(hash);
        pr_err!("%s: Crypto %s allocation error %d\n", __func__, name, rc);
        return rc;
    }
    let size = core::mem::size_of::<shash_desc>() + crypto_shash_descsize(hash) as usize;
    let sdesc = kmalloc(size, GFP_KERNEL) as *mut sdesc;
    if sdesc.is_null() {
        crypto_free_shash(hash);
        return -ENOMEM;
    }
    (*sdesc).shash.tfm = hash;
    let mut rc = 0;
    if key_len > 0 {
        rc = crypto_shash_setkey(hash, key, key_len);
        if rc != 0 { pr_err!("%s: Could not setkey %s shash\n", __func__, name); }
    }
    if rc == 0 { rc = crypto_shash_init(&mut (*sdesc).shash); }
    if rc == 0 { rc = crypto_shash_update(&mut (*sdesc).shash, data1, data1_len); }
    if rc == 0 && !data2.is_null() && data2_len != 0 {
        rc = crypto_shash_update(&mut (*sdesc).shash, data2, data2_len);
    }
    if rc == 0 { rc = crypto_shash_final(&mut (*sdesc).shash, result); }
    crypto_free_shash(hash);
    kfree(sdesc as *mut core::ffi::c_void);
    rc
}

pub unsafe fn spu_alg_name(alg: spu_cipher_alg, mode: spu_cipher_mode) -> *mut u8 {
    match alg {
        CIPHER_ALG_RC4 => b"rc4\0".as_ptr() as *mut u8,
        CIPHER_ALG_AES => match mode {
            CIPHER_MODE_CBC => b"cbc(aes)\0", CIPHER_MODE_ECB => b"ecb(aes)\0",
            CIPHER_MODE_OFB => b"ofb(aes)\0", CIPHER_MODE_CFB => b"cfb(aes)\0",
            CIPHER_MODE_CTR => b"ctr(aes)\0", CIPHER_MODE_XTS => b"xts(aes)\0",
            CIPHER_MODE_GCM => b"gcm(aes)\0", _ => b"aes\0",
        }.as_ptr() as *mut u8,
        CIPHER_ALG_DES => match mode {
            CIPHER_MODE_CBC => b"cbc(des)\0", CIPHER_MODE_ECB => b"ecb(des)\0",
            CIPHER_MODE_CTR => b"ctr(des)\0", _ => b"des\0",
        }.as_ptr() as *mut u8,
        CIPHER_ALG_3DES => match mode {
            CIPHER_MODE_CBC => b"cbc(des3_ede)\0", CIPHER_MODE_ECB => b"ecb(des3_ede)\0",
            CIPHER_MODE_CTR => b"ctr(des3_ede)\0", _ => b"3des\0",
        }.as_ptr() as *mut u8,
        _ => b"other\0".as_ptr() as *mut u8,
    }
}

#[cfg(feature = "DEBUG")]
pub unsafe fn __dump_sg(sg: *mut scatterlist, skip: u32, len: u32) {
    let mut dbuf = [0u8; 16];
    let mut idx = skip;
    let mut num_out = 0u32;
    if packet_debug_logging {
        while num_out < len {
            let count = if len - num_out > 16 { 16 } else { len - num_out };
            sg_copy_part_to_buf(sg, dbuf.as_mut_ptr(), count, idx);
            num_out += count;
            print_hex_dump!(KERN_ALERT, "  sg: ", DUMP_PREFIX_NONE, 4, 1,
                            dbuf.as_ptr(), count, false);
            idx += 16;
        }
    }
    if debug_logging_sleep != 0 { msleep(debug_logging_sleep); }
}

unsafe fn spu_debugfs_read(
    filp: *mut file, ubuf: *mut core::ffi::c_char, count: usize, offp: *mut loff_t,
) -> isize {
    let out_count: isize = 2048;
    let buf = kmalloc(out_count as usize, GFP_KERNEL) as *mut core::ffi::c_char;
    if buf.is_null() { return -ENOMEM as isize; }
    let ipriv = (*filp).private_data as *mut bcm_device_private;
    let mut out_offset: isize = 0;
    out_offset += scnprintf!(buf.add(out_offset as usize), out_count - out_offset,
        "Number of SPUs.........%u\n", (*ipriv).spu.num_spu);
    out_offset += scnprintf!(buf.add(out_offset as usize), out_count - out_offset,
        "Current sessions.......%u\n", atomic_read(&(*ipriv).session_count));
    out_offset += scnprintf!(buf.add(out_offset as usize), out_count - out_offset,
        "Session count..........%u\n", atomic_read(&(*ipriv).stream_count));
    out_offset += scnprintf!(buf.add(out_offset as usize), out_count - out_offset,
        "Cipher setkey..........%u\n", atomic_read(&(*ipriv).setkey_cnt[SPU_OP_CIPHER]));
    out_offset += scnprintf!(buf.add(out_offset as usize), out_count - out_offset,
        "Cipher Ops.............%u\n", atomic_read(&(*ipriv).op_counts[SPU_OP_CIPHER]));
    for alg in 0..CIPHER_ALG_LAST {
        for mode in 0..CIPHER_MODE_LAST {
            let op_cnt = atomic_read(&(*ipriv).cipher_cnt[alg][mode]);
            if op_cnt != 0 { out_offset += scnprintf!(buf.add(out_offset as usize), out_count - out_offset,
                "  %-13s%11u\n", spu_alg_name(alg, mode), op_cnt); }
        }
    }
    out_offset += scnprintf!(buf.add(out_offset as usize), out_count - out_offset,
        "Hash Ops...............%u\n", atomic_read(&(*ipriv).op_counts[SPU_OP_HASH]));
    for alg in 0..HASH_ALG_LAST {
        let op_cnt = atomic_read(&(*ipriv).hash_cnt[alg]);
        if op_cnt != 0 { out_offset += scnprintf!(buf.add(out_offset as usize), out_count - out_offset,
            "  %-13s%11u\n", hash_alg_name[alg], op_cnt); }
    }
    if out_offset > out_count { out_offset = out_count; }
    let ret = simple_read_from_buffer(ubuf, count, offp, buf, out_offset as usize);
    kfree(buf as *mut core::ffi::c_void);
    ret
}

pub unsafe fn spu_setup_debugfs() {
    if !debugfs_initialized() { return; }
    if iproc_priv.debugfs_dir.is_null() {
        iproc_priv.debugfs_dir = debugfs_create_dir(KBUILD_MODNAME, core::ptr::null_mut());
    }
    if iproc_priv.debugfs_stats.is_null() {
        iproc_priv.debugfs_stats = debugfs_create_file("stats", 0o400, iproc_priv.debugfs_dir,
            &mut iproc_priv as *mut _, &spu_debugfs_stats);
    }
}

pub unsafe fn spu_free_debugfs() {
    debugfs_remove_recursive(iproc_priv.debugfs_dir);
    iproc_priv.debugfs_dir = core::ptr::null_mut();
}

pub unsafe fn format_value_ccm(val: u32, buf: *mut u8, len: u8) {
    core::ptr::write_bytes(buf, 0, len as usize);
    for i in 0..len {
        *buf.add((len - i - 1) as usize) = ((val >> (8 * i)) & 0xff) as u8;
        if i >= 3 { break; }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
