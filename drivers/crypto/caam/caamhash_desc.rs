// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
/*
 * Shared descriptors for ahash algorithms
 *
 * Copyright 2017-2019 NXP
 */

// C dependencies supplied by the surrounding CAAM implementation are kept as
// external symbols here.

#[repr(C)]
pub struct alginfo {
    pub algtype: u32,
    pub key_virt: *const u8,
    pub key_dma: u64,
    pub keylen: u32,
    pub keylen_pad: u32,
}

extern "C" {
    fn init_sh_desc(desc: *mut u32, hdr: u32);
    fn append_jump(desc: *mut u32, options: u32) -> *mut u32;
    fn append_key_as_imm(desc: *mut u32, key: *const u8, keylen: u32,
                         padlen: u32, options: u32);
    fn append_proto_dkp(desc: *mut u32, adata: *mut alginfo);
    fn set_jump_tgt_here(desc: *mut u32, jump: *mut u32);
    fn append_seq_load(desc: *mut u32, len: i32, options: u32);
    fn append_operation(desc: *mut u32, operation: u32);
    fn append_math_add(desc: *mut u32, dest: u32, src: u32, reg: u32, value: u32);
    fn append_seq_fifo_load(desc: *mut u32, len: i32, options: u32);
    fn append_seq_store(desc: *mut u32, len: i32, options: u32);
    fn append_key(desc: *mut u32, key: u64, keylen: u32, options: u32);
    fn append_fifo_store(desc: *mut u32, key: u64, keylen: u32, options: u32);
    fn is_xcbc_aes(algtype: u32) -> bool;
}

/// cnstr_shdsc_ahash - ahash shared descriptor
pub unsafe extern "C" fn cnstr_shdsc_ahash(
    desc: *mut u32,
    adata: *mut alginfo,
    state: u32,
    digestsize: i32,
    ctx_len: i32,
    import_ctx: bool,
    era: i32,
) {
    let mut op = (*adata).algtype;

    init_sh_desc(desc, HDR_SHARE_SERIAL);

    /* Append key if it has been set; ahash update excluded */
    if state != OP_ALG_AS_UPDATE && (*adata).keylen != 0 {
        let skip_key_load: *mut u32;

        /* Skip key loading if already shared */
        skip_key_load = append_jump(desc, JUMP_JSL | JUMP_TEST_ALL | JUMP_COND_SHRD);

        if era < 6 {
            append_key_as_imm(desc, (*adata).key_virt, (*adata).keylen_pad,
                              (*adata).keylen,
                              CLASS_2 | KEY_DEST_MDHA_SPLIT | KEY_ENC);
        } else {
            append_proto_dkp(desc, adata);
        }

        set_jump_tgt_here(desc, skip_key_load);
        op |= OP_ALG_AAI_HMAC_PRECOMP;
    }

    /* If needed, import context from software */
    if import_ctx {
        append_seq_load(desc, ctx_len, LDST_CLASS_2_CCB | LDST_SRCDST_BYTE_CONTEXT);
    }

    /* Class 2 operation */
    append_operation(desc, op | state | OP_ALG_ENCRYPT);

    /* Load from buf and/or src and write to req->result or state->context */
    append_math_add(desc, VARSEQINLEN, SEQINLEN, REG0, CAAM_CMD_SZ);
    /* Read remaining bytes */
    append_seq_fifo_load(desc, 0, FIFOLD_CLASS_CLASS2 | FIFOLD_TYPE_LAST2 |
                         FIFOLD_TYPE_MSG | KEY_VLF);
    /* Store class2 context bytes */
    append_seq_store(desc, digestsize, LDST_CLASS_2_CCB | LDST_SRCDST_BYTE_CONTEXT);
}

/// cnstr_shdsc_sk_hash - shared descriptor for symmetric key cipher-based hash algorithms
pub unsafe extern "C" fn cnstr_shdsc_sk_hash(
    desc: *mut u32,
    adata: *mut alginfo,
    state: u32,
    digestsize: i32,
    ctx_len: i32,
) {
    let skip_key_load: *mut u32;

    init_sh_desc(desc, HDR_SHARE_SERIAL | HDR_SAVECTX);
    /* Skip loading of key, context if already shared */
    skip_key_load = append_jump(desc, JUMP_TEST_ALL | JUMP_COND_SHRD);

    if state == OP_ALG_AS_INIT || state == OP_ALG_AS_INITFINALIZE {
        append_key_as_imm(desc, (*adata).key_virt, (*adata).keylen,
                          (*adata).keylen, CLASS_1 | KEY_DEST_CLASS_REG);
    } else {
        /* UPDATE, FINALIZE */
        if is_xcbc_aes((*adata).algtype) {
            /* Load K1 */
            append_key(desc, (*adata).key_dma, (*adata).keylen,
                       CLASS_1 | KEY_DEST_CLASS_REG | KEY_ENC);
        } else {
            /* CMAC */
            append_key_as_imm(desc, (*adata).key_virt, (*adata).keylen,
                              (*adata).keylen, CLASS_1 | KEY_DEST_CLASS_REG);
        }
        /* Restore context */
        append_seq_load(desc, ctx_len, LDST_CLASS_1_CCB | LDST_SRCDST_BYTE_CONTEXT);
    }

    set_jump_tgt_here(desc, skip_key_load);
    /* Class 1 operation */
    append_operation(desc, (*adata).algtype | state | OP_ALG_ENCRYPT);
    /* Load from buf and/or src and write to req->result or state->context */
    append_math_add(desc, VARSEQINLEN, SEQINLEN, REG0, CAAM_CMD_SZ);
    /* Read remaining bytes */
    append_seq_fifo_load(desc, 0, FIFOLD_CLASS_CLASS1 | FIFOLD_TYPE_LAST1 |
                         FIFOLD_TYPE_MSG | FIFOLDST_VLF);
    /* Save context: xcbc partial hash, keys K2 and K3; cmac partial hash, L */
    append_seq_store(desc, digestsize, LDST_CLASS_1_CCB | LDST_SRCDST_BYTE_CONTEXT);
    if is_xcbc_aes((*adata).algtype) && state == OP_ALG_AS_INIT {
        /* Save K1 */
        append_fifo_store(desc, (*adata).key_dma, (*adata).keylen,
                          LDST_CLASS_1_CCB | FIFOST_TYPE_KEY_KEK);
    }
}

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("FSL CAAM ahash descriptors support");
// MODULE_AUTHOR("NXP Semiconductors");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
