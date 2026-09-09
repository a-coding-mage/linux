// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 Pengutronix, Steffen Trumtrar <kernel@pengutronix.de>
 * Copyright (C) 2021 Pengutronix, Ahmad Fatoum <kernel@pengutronix.de>
 * Copyright 2024-2025 NXP
 */

// External Linux kernel, CAAM, and compatibility declarations are supplied by
// the surrounding translation unit/build environment.

const CAAM_BLOB_DESC_BYTES_MAX: usize =
    CAAM_CMD_SZ as usize
    + CAAM_CMD_SZ as usize + CAAM_BLOB_KEYMOD_LENGTH as usize
    + CAAM_CMD_SZ as usize + CAAM_PTR_SZ_MAX as usize
    + CAAM_CMD_SZ as usize + CAAM_PTR_SZ_MAX as usize
    + CAAM_CMD_SZ as usize;

#[repr(C)]
pub struct caam_blob_priv {
    pub jrdev: device,
}

#[repr(C)]
struct caam_blob_job_result {
    err: c_int,
    completion: completion,
}

unsafe extern "C" fn caam_blob_job_done(
    dev: *mut device,
    _desc: *mut u32,
    err: u32,
    context: *mut c_void,
) {
    let res = &mut *(context as *mut caam_blob_job_result);
    let mut ecode: c_int = 0;

    dev_dbg(dev, "%s %d: err 0x%x\n", __FUNCTION__, __LINE__, err);

    if err != 0 {
        ecode = caam_jr_strstatus(dev, err);
    }

    res.err = ecode;

    /*
     * Upon completion, desc points to a buffer containing a CAAM job
     * descriptor which encapsulates data into an externally-storable
     * blob.
     */
    complete(&mut res.completion);
}

unsafe fn check_caam_state(jrdev: *mut device) -> u32 {
    let ctrlpriv: *const caam_drv_private = dev_get_drvdata((*jrdev).parent);
    FIELD_GET(CSTA_MOO, rd_reg32((*(*ctrlpriv).jr).perfmon.status))
}

#[no_mangle]
pub unsafe extern "C" fn caam_process_blob(
    priv_: *mut caam_blob_priv,
    info: *mut caam_blob_info,
    encap: bool,
) -> c_int {
    let mut testres: caam_blob_job_result = core::mem::zeroed();
    let jrdev: *mut device = &mut (*priv_).jrdev;
    let mut dma_in: dma_addr_t;
    let mut dma_out: dma_addr_t;
    let mut op: c_int = OP_PCLID_BLOB;
    let mut hwbk_caam_ovhd: c_int = 0;
    let output_len: usize;
    let desc: *mut u32;
    let moo: u32;
    let ret: c_int;
    let len: c_int;

    if (*info).key_mod_len > CAAM_BLOB_KEYMOD_LENGTH {
        return -EINVAL;
    }

    if encap {
        op |= OP_TYPE_ENCAP_PROTOCOL;
        output_len = (*info).input_len + CAAM_BLOB_OVERHEAD;
    } else {
        op |= OP_TYPE_DECAP_PROTOCOL;
        output_len = (*info).input_len - CAAM_BLOB_OVERHEAD;
        (*info).output_len = output_len;
    }

    if encap && (*info).pkey_info.is_pkey {
        op |= OP_PCL_BLOB_BLACK;
        if (*info).pkey_info.key_enc_algo == CAAM_ENC_ALGO_CCM {
            op |= OP_PCL_BLOB_EKT;
            hwbk_caam_ovhd = CAAM_CCM_OVERHEAD;
        }
        if ((*info).input_len as c_int + hwbk_caam_ovhd) as usize > MAX_KEY_SIZE {
            return -EINVAL;
        }
        len = (*info).input_len as c_int + hwbk_caam_ovhd;
    } else {
        len = (*info).input_len as c_int;
    }

    desc = kzalloc(CAAM_BLOB_DESC_BYTES_MAX, GFP_KERNEL) as *mut u32;
    if desc.is_null() {
        return -ENOMEM;
    }

    dma_in = dma_map_single(jrdev, (*info).input, len as usize,
                            if encap { DMA_BIDIRECTIONAL } else { DMA_TO_DEVICE });
    if dma_mapping_error(jrdev, dma_in) {
        dev_err(jrdev, "unable to map input DMA buffer\n");
        ret = -ENOMEM;
        goto out_free;
    }

    dma_out = dma_map_single(jrdev, (*info).output, output_len, DMA_FROM_DEVICE);
    if dma_mapping_error(jrdev, dma_out) {
        dev_err(jrdev, "unable to map output DMA buffer\n");
        ret = -ENOMEM;
        goto out_unmap_in;
    }

    moo = check_caam_state(jrdev);
    if moo != CSTA_MOO_SECURE && moo != CSTA_MOO_TRUSTED {
        dev_warn(jrdev, "using insecure test key, enable HAB to use unique device key!\n");
    }

    /* A data blob is encrypted using a random AES-CCM blob key. */
    init_job_desc(desc, 0);

    if encap && (*info).pkey_info.is_pkey {
        append_key(desc, dma_in, (*info).input_len,
                   CLASS_1 | KEY_DEST_CLASS_REG);
        if (*info).pkey_info.key_enc_algo == CAAM_ENC_ALGO_CCM {
            append_fifo_store(desc, dma_in, (*info).input_len,
                              LDST_CLASS_1_CCB | FIFOST_TYPE_KEY_CCM_JKEK);
        } else {
            append_fifo_store(desc, dma_in, (*info).input_len,
                              LDST_CLASS_1_CCB | FIFOST_TYPE_KEY_KEK);
        }
        append_jump(desc, JUMP_COND_NOP | (BIT(0) << JUMP_OFFSET_SHIFT));
    }

    append_key_as_imm(desc, (*info).key_mod, (*info).key_mod_len,
                      (*info).key_mod_len, CLASS_2 | KEY_DEST_CLASS_REG);
    append_seq_in_ptr(desc, dma_in, (*info).input_len, 0);
    append_seq_out_ptr(desc, dma_out, output_len, 0);
    append_operation(desc, op);

    print_hex_dump_debug("data@<line>: ", DUMP_PREFIX_ADDRESS, 16, 1,
                         (*info).input, len as usize, false);
    print_hex_dump_debug("jobdesc@<line>: ", DUMP_PREFIX_ADDRESS, 16, 1,
                         desc, desc_bytes(desc), false);

    testres.err = 0;
    init_completion(&mut testres.completion);
    ret = caam_jr_enqueue(jrdev, desc, Some(caam_blob_job_done),
                          &mut testres as *mut _ as *mut c_void);
    if ret == -EINPROGRESS {
        wait_for_completion(&mut testres.completion);
        ret = testres.err;
        print_hex_dump_debug("output@<line>: ", DUMP_PREFIX_ADDRESS, 16, 1,
                             (*info).output, output_len, false);
    }
    if ret == 0 {
        (*info).output_len = output_len;
    }
    dma_unmap_single(jrdev, dma_out, output_len, DMA_FROM_DEVICE);
out_unmap_in:
    dma_unmap_single(jrdev, dma_in, len as usize,
                     if encap { DMA_BIDIRECTIONAL } else { DMA_TO_DEVICE });
out_free:
    kfree(desc as *mut c_void);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn caam_blob_gen_init() -> *mut caam_blob_priv {
    let ctrlpriv: *mut caam_drv_private;
    let jrdev: *mut device;

    /* caam_blob_gen_init() may expectedly fail with -ENODEV. */
    jrdev = caam_jr_alloc();
    if IS_ERR(jrdev) {
        pr_info("job ring requested, but none currently available\n");
        return ERR_PTR(-ENODEV) as *mut caam_blob_priv;
    }
    ctrlpriv = dev_get_drvdata((*jrdev).parent);
    if !(*ctrlpriv).blob_present {
        dev_info(jrdev, "no hardware blob generation support\n");
        caam_jr_free(jrdev);
        return ERR_PTR(-ENODEV) as *mut caam_blob_priv;
    }
    container_of!(jrdev, caam_blob_priv, jrdev)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
