// SPDX-License-Identifier: GPL-2.0
/*
 * caam - Freescale FSL CAAM support for Public Key Cryptography descriptors
 *
 * Copyright 2016 Freescale Semiconductor, Inc.
 *
 * There is no Shared Descriptor for PKC so that the Job Descriptor must carry
 * all the desired key parameters, input and output pointers.
 */
// Dependencies supplied by the corresponding CAAM headers:
// caampkc.h, desc_constr.h

/* Descriptor for RSA Public operation */
pub unsafe fn init_rsa_pub_desc(desc: *mut u32, pdb: *mut rsa_pub_pdb) {
    init_job_desc_pdb(desc, 0, SIZEOF_RSA_PUB_PDB);
    append_cmd(desc, (*pdb).sgf);
    append_ptr(desc, (*pdb).f_dma);
    append_ptr(desc, (*pdb).g_dma);
    append_ptr(desc, (*pdb).n_dma);
    append_ptr(desc, (*pdb).e_dma);
    append_cmd(desc, (*pdb).f_len);
    append_operation(desc, OP_TYPE_UNI_PROTOCOL | OP_PCLID_RSAENC_PUBKEY);
}

/* Descriptor for RSA Private operation - Private Key Form #1 */
pub unsafe fn init_rsa_priv_f1_desc(desc: *mut u32, pdb: *mut rsa_priv_f1_pdb) {
    init_job_desc_pdb(desc, 0, SIZEOF_RSA_PRIV_F1_PDB);
    append_cmd(desc, (*pdb).sgf);
    append_ptr(desc, (*pdb).g_dma);
    append_ptr(desc, (*pdb).f_dma);
    append_ptr(desc, (*pdb).n_dma);
    append_ptr(desc, (*pdb).d_dma);
    append_operation(
        desc,
        OP_TYPE_UNI_PROTOCOL | OP_PCLID_RSADEC_PRVKEY | RSA_PRIV_KEY_FRM_1,
    );
}

/* Descriptor for RSA Private operation - Private Key Form #2 */
pub unsafe fn init_rsa_priv_f2_desc(desc: *mut u32, pdb: *mut rsa_priv_f2_pdb) {
    init_job_desc_pdb(desc, 0, SIZEOF_RSA_PRIV_F2_PDB);
    append_cmd(desc, (*pdb).sgf);
    append_ptr(desc, (*pdb).g_dma);
    append_ptr(desc, (*pdb).f_dma);
    append_ptr(desc, (*pdb).d_dma);
    append_ptr(desc, (*pdb).p_dma);
    append_ptr(desc, (*pdb).q_dma);
    append_ptr(desc, (*pdb).tmp1_dma);
    append_ptr(desc, (*pdb).tmp2_dma);
    append_cmd(desc, (*pdb).p_q_len);
    append_operation(
        desc,
        OP_TYPE_UNI_PROTOCOL | OP_PCLID_RSADEC_PRVKEY | RSA_PRIV_KEY_FRM_2,
    );
}

/* Descriptor for RSA Private operation - Private Key Form #3 */
pub unsafe fn init_rsa_priv_f3_desc(desc: *mut u32, pdb: *mut rsa_priv_f3_pdb) {
    init_job_desc_pdb(desc, 0, SIZEOF_RSA_PRIV_F3_PDB);
    append_cmd(desc, (*pdb).sgf);
    append_ptr(desc, (*pdb).g_dma);
    append_ptr(desc, (*pdb).f_dma);
    append_ptr(desc, (*pdb).c_dma);
    append_ptr(desc, (*pdb).p_dma);
    append_ptr(desc, (*pdb).q_dma);
    append_ptr(desc, (*pdb).dp_dma);
    append_ptr(desc, (*pdb).dq_dma);
    append_ptr(desc, (*pdb).tmp1_dma);
    append_ptr(desc, (*pdb).tmp2_dma);
    append_cmd(desc, (*pdb).p_q_len);
    append_operation(
        desc,
        OP_TYPE_UNI_PROTOCOL | OP_PCLID_RSADEC_PRVKEY | RSA_PRIV_KEY_FRM_3,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
