// SPDX-License-Identifier: GPL-2.0
/*
 * CAAM/SEC 4.x functions for handling key-generation jobs
 *
 * Copyright 2008-2011 Freescale Semiconductor, Inc.
 *
 */
// Dependencies supplied by the surrounding translation unit are intentionally
// left as external Rust symbols.

pub unsafe fn split_key_done(
    dev: *mut device,
    _desc: *mut u32,
    err: u32,
    context: *mut core::ffi::c_void,
) {
    let res = context as *mut split_key_result;
    let mut ecode: i32 = 0;

    dev_dbg(dev, "%s %d: err 0x%x\n", "split_key_done", 0, err);

    if err != 0 {
        ecode = caam_jr_strstatus(dev, err);
    }

    (*res).err = ecode;

    complete(&mut (*res).completion);
}

/*
get a split ipad/opad key

Split key generation-----------------------------------------------

[00] 0xb0810008    jobdesc: stidx=1 share=never len=8
[01] 0x04000014        key: class2->keyreg len=20
                        @0xffe01000
[03] 0x84410014  operation: cls2-op sha1 hmac init dec
[04] 0x24940000     fifold: class2 msgdata-last2 len=0 imm
[05] 0xa4000001       jump: class2 local all ->1 [06]
[06] 0x64260028    fifostr: class2 mdsplit-jdk len=40
                        @0xffe04000
*/
pub unsafe fn gen_split_key(
    jrdev: *mut device,
    key_out: *mut u8,
    adata: *mut alginfo,
    key_in: *const u8,
    keylen: u32,
    max_keylen: i32,
) -> i32 {
    let mut desc: *mut u32;
    let mut result: split_key_result = core::mem::zeroed();
    let mut dma_addr: dma_addr_t;
    let local_max: u32;
    let mut ret: i32 = -ENOMEM;

    (*adata).keylen = split_key_len((*adata).algtype & OP_ALG_ALGSEL_MASK);
    (*adata).keylen_pad = split_key_pad_len((*adata).algtype & OP_ALG_ALGSEL_MASK);
    local_max = if keylen > (*adata).keylen_pad {
        keylen
    } else {
        (*adata).keylen_pad
    };

    dev_dbg(
        jrdev,
        "split keylen %d split keylen padded %d\n",
        (*adata).keylen,
        (*adata).keylen_pad,
    );
    print_hex_dump_devel(
        "ctx.key@" __stringify!(__LINE__) ": ",
        DUMP_PREFIX_ADDRESS,
        16,
        4,
        key_in,
        keylen,
        1,
    );

    if (local_max > max_keylen as u32) {
        return -EINVAL;
    }

    desc = kmalloc(CAAM_CMD_SZ * 6 + CAAM_PTR_SZ * 2, GFP_KERNEL);
    if desc.is_null() {
        dev_err(jrdev, "unable to allocate key input memory\n");
        return ret;
    }

    core::ptr::copy_nonoverlapping(key_in, key_out, keylen as usize);

    dma_addr = dma_map_single(jrdev, key_out, local_max, DMA_BIDIRECTIONAL);
    if dma_mapping_error(jrdev, dma_addr) {
        dev_err(jrdev, "unable to map key memory\n");
        goto_out_free(desc);
        return ret;
    }

    init_job_desc(desc, 0);
    append_key(desc, dma_addr, keylen, CLASS_2 | KEY_DEST_CLASS_REG);

    /* Sets MDHA up into an HMAC-INIT */
    append_operation(
        desc,
        ((*adata).algtype & OP_ALG_ALGSEL_MASK)
            | OP_ALG_AAI_HMAC
            | OP_TYPE_CLASS2_ALG
            | OP_ALG_DECRYPT
            | OP_ALG_AS_INIT,
    );

    /*
     * do a FIFO_LOAD of zero, this will trigger the internal key expansion
     * into both pads inside MDHA
     */
    append_fifo_load_as_imm(
        desc,
        core::ptr::null_mut(),
        0,
        LDST_CLASS_2_CCB | FIFOLD_TYPE_MSG | FIFOLD_TYPE_LAST2,
    );

    /*
     * FIFO_STORE with the explicit split-key content store
     * (0x26 output type)
     */
    append_fifo_store(
        desc,
        dma_addr,
        (*adata).keylen,
        LDST_CLASS_2_CCB | FIFOST_TYPE_SPLIT_KEK,
    );

    print_hex_dump_debug(
        "jobdesc@" __stringify!(__LINE__) ": ",
        DUMP_PREFIX_ADDRESS,
        16,
        4,
        desc,
        desc_bytes(desc),
        1,
    );

    result.err = 0;
    init_completion(&mut result.completion);

    ret = caam_jr_enqueue(jrdev, desc, split_key_done, &mut result);
    if ret == -EINPROGRESS {
        /* in progress */
        wait_for_completion(&mut result.completion);
        ret = result.err;

        print_hex_dump_devel(
            "ctx.key@" __stringify!(__LINE__) ": ",
            DUMP_PREFIX_ADDRESS,
            16,
            4,
            key_out,
            (*adata).keylen_pad,
            1,
        );
    }

    dma_unmap_single(jrdev, dma_addr, local_max, DMA_BIDIRECTIONAL);
    goto_out_free(desc);
    ret
}

unsafe fn goto_out_free(desc: *mut u32) {
    kfree(desc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
