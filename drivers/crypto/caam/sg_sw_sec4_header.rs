/* SPDX-License-Identifier: GPL-2.0 */
/*
 * CAAM/SEC 4.x functions for using scatterlists in caam driver
 *
 * Copyright 2008-2011 Freescale Semiconductor, Inc.
 */

// C dependencies supplied by the surrounding translation unit:
// ctrl.h, regs.h, sg_sw_qm2.h, and soc/fsl/dpaa2-fd.h.

#[repr(C)]
pub struct sec4_sg_entry {
    pub ptr: u64,
    pub len: u32,
    pub bpid_offset: u32,
}

// The following types and functions are provided by the translated dependencies.
extern "C" {
    static mut caam_dpaa2: bool;
    fn dma_to_qm_sg_one(
        entry: *mut dpaa2_sg_entry,
        dma: dma_addr_t,
        len: u32,
        offset: u16,
    );
    fn cpu_to_caam_dma64(value: dma_addr_t) -> u64;
    fn cpu_to_caam32(value: u32) -> u32;
    fn print_hex_dump_debug(
        prefix: *const core::ffi::c_char,
        prefix_type: i32,
        rowsize: i32,
        groupsize: i32,
        buffer: *const core::ffi::c_void,
        length: usize,
        ascii: i32,
    );
    fn sg_dma_len(sg: *mut scatterlist) -> u32;
    fn sg_dma_address(sg: *mut scatterlist) -> dma_addr_t;
    fn sg_next(sg: *mut scatterlist) -> *mut scatterlist;
    fn dpaa2_sg_set_final(entry: *mut dpaa2_sg_entry, final_entry: bool);
}

#[repr(C)]
pub struct scatterlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dpaa2_sg_entry {
    _private: [u8; 0],
}

pub type dma_addr_t = u64;

// These constants are supplied by the translated CAAM register dependencies.
extern "C" {
    static SEC4_SG_OFFSET_MASK: u32;
    static SEC4_SG_LEN_FIN: u32;
    static DUMP_PREFIX_ADDRESS: i32;
}

/*
 * convert single dma address to h/w link table format
 */
#[inline]
pub unsafe fn dma_to_sec4_sg_one(
    sec4_sg_ptr: *mut sec4_sg_entry,
    dma: dma_addr_t,
    len: u32,
    offset: u16,
) {
    if caam_dpaa2 {
        dma_to_qm_sg_one(sec4_sg_ptr as *mut dpaa2_sg_entry, dma, len, offset);
    } else {
        (*sec4_sg_ptr).ptr = cpu_to_caam_dma64(dma);
        (*sec4_sg_ptr).len = cpu_to_caam32(len);
        (*sec4_sg_ptr).bpid_offset = cpu_to_caam32((offset as u32) & SEC4_SG_OFFSET_MASK);
    }

    print_hex_dump_debug(
        b"sec4_sg_ptr@: \0".as_ptr() as *const core::ffi::c_char,
        DUMP_PREFIX_ADDRESS,
        16,
        4,
        sec4_sg_ptr as *const core::ffi::c_void,
        core::mem::size_of::<sec4_sg_entry>(),
        1,
    );
}

/*
 * convert scatterlist to h/w link table format
 * but does not have final bit; instead, returns last entry
 */
#[inline]
pub unsafe fn sg_to_sec4_sg(
    mut sg: *mut scatterlist,
    mut len: i32,
    mut sec4_sg_ptr: *mut sec4_sg_entry,
    offset: u16,
) -> *mut sec4_sg_entry {
    while len != 0 {
        let ent_len = core::cmp::min(sg_dma_len(sg) as i32, len);

        dma_to_sec4_sg_one(sec4_sg_ptr, sg_dma_address(sg), ent_len as u32, offset);
        sec4_sg_ptr = sec4_sg_ptr.add(1);
        sg = sg_next(sg);
        len -= ent_len;
    }
    sec4_sg_ptr.sub(1)
}

#[inline]
pub unsafe fn sg_to_sec4_set_last(sec4_sg_ptr: *mut sec4_sg_entry) {
    if caam_dpaa2 {
        dpaa2_sg_set_final(sec4_sg_ptr as *mut dpaa2_sg_entry, true);
    } else {
        (*sec4_sg_ptr).len |= cpu_to_caam32(SEC4_SG_LEN_FIN);
    }
}

/*
 * convert scatterlist to h/w link table format
 * scatterlist must have been previously dma mapped
 */
#[inline]
pub unsafe fn sg_to_sec4_sg_last(
    sg: *mut scatterlist,
    len: i32,
    sec4_sg_ptr: *mut sec4_sg_entry,
    offset: u16,
) {
    let sec4_sg_ptr = sg_to_sec4_sg(sg, len, sec4_sg_ptr, offset);
    sg_to_sec4_set_last(sec4_sg_ptr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
