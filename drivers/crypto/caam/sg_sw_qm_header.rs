/* SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause) */
/*
 * Copyright 2013-2016 Freescale Semiconductor, Inc.
 * Copyright 2016-2017 NXP
 */

// Dependencies supplied by <soc/fsl/qman.h> and "regs.h" remain external.

#[inline]
pub unsafe fn __dma_to_qm_sg(
    qm_sg_ptr: *mut qm_sg_entry,
    dma: dma_addr_t,
    offset: u16,
) {
    qm_sg_entry_set64(qm_sg_ptr, dma);
    (*qm_sg_ptr).__reserved2 = 0;
    (*qm_sg_ptr).bpid = 0;
    (*qm_sg_ptr).offset = cpu_to_be16(offset & QM_SG_OFF_MASK);
}

#[inline]
pub unsafe fn dma_to_qm_sg_one(
    qm_sg_ptr: *mut qm_sg_entry,
    dma: dma_addr_t,
    len: u32,
    offset: u16,
) {
    __dma_to_qm_sg(qm_sg_ptr, dma, offset);
    qm_sg_entry_set_len(qm_sg_ptr, len);
}

#[inline]
pub unsafe fn dma_to_qm_sg_one_last(
    qm_sg_ptr: *mut qm_sg_entry,
    dma: dma_addr_t,
    len: u32,
    offset: u16,
) {
    __dma_to_qm_sg(qm_sg_ptr, dma, offset);
    qm_sg_entry_set_f(qm_sg_ptr, len);
}

#[inline]
pub unsafe fn dma_to_qm_sg_one_ext(
    qm_sg_ptr: *mut qm_sg_entry,
    dma: dma_addr_t,
    len: u32,
    offset: u16,
) {
    __dma_to_qm_sg(qm_sg_ptr, dma, offset);
    (*qm_sg_ptr).cfg = cpu_to_be32(QM_SG_EXT | (len & QM_SG_LEN_MASK));
}

#[inline]
pub unsafe fn dma_to_qm_sg_one_last_ext(
    qm_sg_ptr: *mut qm_sg_entry,
    dma: dma_addr_t,
    len: u32,
    offset: u16,
) {
    __dma_to_qm_sg(qm_sg_ptr, dma, offset);
    (*qm_sg_ptr).cfg = cpu_to_be32(QM_SG_EXT | QM_SG_FIN | (len & QM_SG_LEN_MASK));
}

/*
 * convert scatterlist to h/w link table format
 * but does not have final bit; instead, returns last entry
 */
#[inline]
pub unsafe fn sg_to_qm_sg(
    mut sg: *mut scatterlist,
    mut len: i32,
    mut qm_sg_ptr: *mut qm_sg_entry,
    offset: u16,
) -> *mut qm_sg_entry {
    let mut ent_len: i32;

    while len != 0 {
        ent_len = core::cmp::min(sg_dma_len(sg), len);

        dma_to_qm_sg_one(qm_sg_ptr, sg_dma_address(sg), ent_len as u32, offset);
        qm_sg_ptr = qm_sg_ptr.add(1);
        sg = sg_next(sg);
        len -= ent_len;
    }
    qm_sg_ptr.sub(1)
}

/*
 * convert scatterlist to h/w link table format
 * scatterlist must have been previously dma mapped
 */
#[inline]
pub unsafe fn sg_to_qm_sg_last(
    sg: *mut scatterlist,
    len: i32,
    qm_sg_ptr: *mut qm_sg_entry,
    offset: u16,
) {
    let qm_sg_ptr = sg_to_qm_sg(sg, len, qm_sg_ptr, offset);
    qm_sg_entry_set_f(qm_sg_ptr, qm_sg_entry_get_len(qm_sg_ptr));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
