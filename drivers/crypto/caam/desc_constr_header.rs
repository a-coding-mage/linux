/* SPDX-License-Identifier: GPL-2.0 */
/*
 * caam descriptor construction helper functions
 *
 * Copyright 2008-2012 Freescale Semiconductor, Inc.
 * Copyright 2019, 2025 NXP
 */

// Dependencies supplied by the surrounding translation unit: desc.h, regs.h

pub const IMMEDIATE: u32 = 1 << 23;
pub const CAAM_CMD_SZ: usize = core::mem::size_of::<u32>();
pub const CAAM_PTR_SZ_MAX: usize = core::mem::size_of::<dma_addr_t>();
pub const CAAM_PTR_SZ_MIN: usize = core::mem::size_of::<u32>();
pub const CAAM_DESC_BYTES_MAX: usize = CAAM_CMD_SZ * MAX_CAAM_DESCSIZE as usize;

pub static mut caam_little_end: bool = false;
pub static mut caam_ptr_sz: usize = 0;

#[inline]
pub unsafe fn pad_sg_nents(sg_nents: i32) -> i32 { (sg_nents + 3) & !3 }

#[inline]
pub unsafe fn desc_len(desc: *mut u32) -> i32 {
    (caam32_to_cpu(*desc) & HDR_DESCLEN_MASK) as i32
}

#[inline]
pub unsafe fn desc_bytes(desc: *mut core::ffi::c_void) -> usize {
    desc_len(desc as *mut u32) as usize * CAAM_CMD_SZ
}

#[inline]
pub unsafe fn desc_end(desc: *mut u32) -> *mut u32 { desc.add(desc_len(desc) as usize) }

#[inline]
pub unsafe fn sh_desc_pdb(desc: *mut u32) -> *mut core::ffi::c_void {
    desc.add(1) as *mut core::ffi::c_void
}

#[inline]
pub unsafe fn init_desc(desc: *mut u32, options: u32) {
    *desc = cpu_to_caam32((options | HDR_ONE) + 1);
}

#[inline]
pub unsafe fn init_sh_desc(desc: *mut u32, options: u32) {
    init_desc(desc, CMD_SHARED_DESC_HDR | options);
}

#[inline]
pub unsafe fn init_sh_desc_pdb(desc: *mut u32, options: u32, pdb_bytes: usize) {
    let pdb_len = (pdb_bytes + CAAM_CMD_SZ - 1) / CAAM_CMD_SZ;
    init_sh_desc(desc, (((pdb_len as u32 + 1) << HDR_START_IDX_SHIFT) + pdb_len as u32) | options);
}

#[inline]
pub unsafe fn init_job_desc(desc: *mut u32, options: u32) { init_desc(desc, CMD_DESC_HDR | options); }

#[inline]
pub unsafe fn init_job_desc_pdb(desc: *mut u32, options: u32, pdb_bytes: usize) {
    let pdb_len = (pdb_bytes + CAAM_CMD_SZ - 1) / CAAM_CMD_SZ;
    init_job_desc(desc, ((pdb_len as u32 + 1) << HDR_START_IDX_SHIFT) | options);
}

#[inline]
pub unsafe fn append_ptr(desc: *mut u32, ptr: dma_addr_t) {
    if caam_ptr_sz == core::mem::size_of::<dma_addr_t>() {
        let offset = desc_end(desc) as *mut dma_addr_t;
        *offset = cpu_to_caam_dma(ptr);
    } else {
        let offset = desc_end(desc);
        *offset = cpu_to_caam_dma(ptr) as u32;
    }
    *desc = cpu_to_caam32(caam32_to_cpu(*desc) + (caam_ptr_sz / CAAM_CMD_SZ) as u32);
}

#[inline]
pub unsafe fn init_job_desc_shared(desc: *mut u32, ptr: dma_addr_t, len: i32, options: u32) {
    init_job_desc(desc, HDR_SHARED | options | ((len as u32) << HDR_START_IDX_SHIFT));
    append_ptr(desc, ptr);
}

#[inline]
pub unsafe fn append_data(desc: *mut u32, data: *const core::ffi::c_void, len: i32) {
    let offset = desc_end(desc) as *mut u8;
    if data.is_null() { /* CONFIG_CRYPTO_DEV_FSL_CAAM_DEBUG may permit NULL */ }
    else { core::ptr::copy_nonoverlapping(data as *const u8, offset, len as usize); }
    *desc = cpu_to_caam32(caam32_to_cpu(*desc) + ((len as usize + CAAM_CMD_SZ - 1) / CAAM_CMD_SZ) as u32);
}

#[inline]
pub unsafe fn append_cmd(desc: *mut u32, command: u32) {
    *desc_end(desc) = cpu_to_caam32(command);
    *desc = cpu_to_caam32(caam32_to_cpu(*desc) + 1);
}

pub use append_cmd as append_u32;

#[inline]
pub unsafe fn append_u64(desc: *mut u32, data: u64) {
    let offset = desc_end(desc);
    if caam_little_end {
        *offset = cpu_to_caam32(lower_32_bits(data));
        *offset.add(1) = cpu_to_caam32(upper_32_bits(data));
    } else {
        *offset = cpu_to_caam32(upper_32_bits(data));
        *offset.add(1) = cpu_to_caam32(lower_32_bits(data));
    }
    *desc = cpu_to_caam32(caam32_to_cpu(*desc) + 2);
}

#[inline]
pub unsafe fn write_cmd(desc: *mut u32, command: u32) -> *mut u32 {
    *desc = cpu_to_caam32(command); desc.add(1)
}

#[inline]
pub unsafe fn append_cmd_ptr(desc: *mut u32, ptr: dma_addr_t, len: i32, command: u32) {
    append_cmd(desc, command | len as u32); append_ptr(desc, ptr);
}

#[inline]
pub unsafe fn append_cmd_ptr_extlen(desc: *mut u32, ptr: dma_addr_t, len: u32, command: u32) {
    append_cmd(desc, command);
    if command & (SQIN_RTO | SQIN_PRE) == 0 { append_ptr(desc, ptr); }
    append_cmd(desc, len);
}

#[inline]
pub unsafe fn append_cmd_data(desc: *mut u32, data: *const core::ffi::c_void, len: i32, command: u32) {
    append_cmd(desc, command | IMMEDIATE | len as u32); append_data(desc, data, len);
}

#[inline] pub unsafe fn append_jump(desc: *mut u32, options: u32) -> *mut u32 { let cmd = desc_end(desc); append_cmd(desc, CMD_JUMP | options); cmd }
#[inline] pub unsafe fn append_move(desc: *mut u32, options: u32) -> *mut u32 { let cmd = desc_end(desc); append_cmd(desc, CMD_MOVE | options); cmd }
#[inline] pub unsafe fn append_move_len(desc: *mut u32, options: u32) -> *mut u32 { let cmd = desc_end(desc); append_cmd(desc, CMD_MOVE_LEN | options); cmd }

#[inline]
pub unsafe fn set_jump_tgt_here(desc: *mut u32, jump_cmd: *mut u32) { *jump_cmd = cpu_to_caam32(caam32_to_cpu(*jump_cmd) | (desc_len(desc) as u32 - jump_cmd.offset_from(desc) as u32)); }

#[inline]
pub unsafe fn set_move_tgt_here(desc: *mut u32, move_cmd: *mut u32) {
    let mut val = caam32_to_cpu(*move_cmd); val &= !MOVE_OFFSET_MASK; val |= (desc_len(desc) as u32 << (MOVE_OFFSET_SHIFT + 2)) & MOVE_OFFSET_MASK; *move_cmd = cpu_to_caam32(val);
}

#[inline] pub unsafe fn append_operation(desc: *mut u32, options: u32) { append_cmd(desc, CMD_OPERATION | options); }
#[inline] pub unsafe fn append_seq_load(desc: *mut u32, len: u32, options: u32) { append_cmd(desc, CMD_SEQ_LOAD | len | options); }
#[inline] pub unsafe fn append_seq_store(desc: *mut u32, len: u32, options: u32) { append_cmd(desc, CMD_SEQ_STORE | len | options); }
#[inline] pub unsafe fn append_seq_fifo_load(desc: *mut u32, len: u32, options: u32) { append_cmd(desc, CMD_SEQ_FIFO_LOAD | len | options); }
#[inline] pub unsafe fn append_seq_fifo_store(desc: *mut u32, len: u32, options: u32) { append_cmd(desc, CMD_SEQ_FIFO_STORE | len | options); }

#[inline] pub unsafe fn append_key(desc: *mut u32, ptr: dma_addr_t, len: u32, options: u32) { append_cmd_ptr(desc, ptr, len as i32, CMD_KEY | options); }
#[inline] pub unsafe fn append_load(desc: *mut u32, ptr: dma_addr_t, len: u32, options: u32) { append_cmd_ptr(desc, ptr, len as i32, CMD_LOAD | options); }
#[inline] pub unsafe fn append_fifo_load(desc: *mut u32, ptr: dma_addr_t, len: u32, options: u32) { append_cmd_ptr(desc, ptr, len as i32, CMD_FIFO_LOAD | options); }
#[inline] pub unsafe fn append_fifo_store(desc: *mut u32, ptr: dma_addr_t, len: u32, options: u32) { append_cmd_ptr(desc, ptr, len as i32, CMD_FIFO_STORE | options); }

#[inline]
pub unsafe fn append_store(desc: *mut u32, ptr: dma_addr_t, len: u32, options: u32) {
    let cmd_src = options & LDST_SRCDST_MASK; append_cmd(desc, CMD_STORE | options | len);
    if cmd_src != LDST_SRCDST_WORD_DESCBUF_SHARED && cmd_src != LDST_SRCDST_WORD_DESCBUF_JOB && cmd_src != LDST_SRCDST_WORD_DESCBUF_JOB_WE && cmd_src != LDST_SRCDST_WORD_DESCBUF_SHARED_WE { append_ptr(desc, ptr); }
}

#[inline] pub unsafe fn append_seq_in_ptr_intlen(desc: *mut u32, ptr: dma_addr_t, len: u32, options: u32) { if options & (SQIN_RTO | SQIN_PRE) != 0 { append_cmd(desc, CMD_SEQ_IN_PTR | len | options); } else { append_cmd_ptr(desc, ptr, len as i32, CMD_SEQ_IN_PTR | options); } }
#[inline] pub unsafe fn append_seq_out_ptr_intlen(desc: *mut u32, ptr: dma_addr_t, len: u32, options: u32) { if options & (SQIN_RTO | SQIN_PRE) != 0 { append_cmd(desc, CMD_SEQ_OUT_PTR | len | options); } else { append_cmd_ptr(desc, ptr, len as i32, CMD_SEQ_OUT_PTR | options); } }

#[inline] pub unsafe fn append_load_as_imm(desc: *mut u32, data: *const core::ffi::c_void, len: u32, options: u32) { append_cmd_data(desc, data, len as i32, CMD_LOAD | options); }
#[inline] pub unsafe fn append_fifo_load_as_imm(desc: *mut u32, data: *const core::ffi::c_void, len: u32, options: u32) { append_cmd_data(desc, data, len as i32, CMD_FIFO_LOAD | options); }
#[inline] pub unsafe fn append_seq_in_ptr_extlen(desc: *mut u32, ptr: dma_addr_t, len: u32, options: u32) { append_cmd_ptr_extlen(desc, ptr, len, CMD_SEQ_IN_PTR | SQIN_EXT | options); }
#[inline] pub unsafe fn append_seq_out_ptr_extlen(desc: *mut u32, ptr: dma_addr_t, len: u32, options: u32) { append_cmd_ptr_extlen(desc, ptr, len, CMD_SEQ_OUT_PTR | SQIN_EXT | options); }
#[inline] pub unsafe fn append_seq_in_ptr(desc: *mut u32, ptr: dma_addr_t, len: u32, options: u32) { append_seq_in_ptr_intlen(desc, ptr, len, options); }
#[inline] pub unsafe fn append_seq_out_ptr(desc: *mut u32, ptr: dma_addr_t, len: u32, options: u32) { append_seq_out_ptr_intlen(desc, ptr, len, options); }
#[inline] pub unsafe fn append_key_as_imm(desc: *mut u32, data: *const core::ffi::c_void, data_len: u32, len: u32, options: u32) { append_cmd(desc, CMD_KEY | IMMEDIATE | len | options); append_data(desc, data, data_len as i32); }
#[inline] pub unsafe fn append_load_imm_u32(desc: *mut u32, immediate: u32, options: u32) { append_cmd(desc, CMD_LOAD | IMMEDIATE | options | if options & LDST_LEN_MASK != 0 { 0 } else { core::mem::size_of::<u32>() as u32 }); append_cmd(desc, immediate); }
#[inline] pub unsafe fn append_load_imm_be32(desc: *mut u32, immediate: u32, options: u32) { let data = cpu_to_be32(immediate); append_cmd(desc, CMD_LOAD | IMMEDIATE | options | core::mem::size_of::<u32>() as u32); append_data(desc, &data as *const u32 as *const _, core::mem::size_of::<u32>() as i32); }

#[repr(C)] pub struct alginfo { pub algtype: u32, pub keylen: u32, pub keylen_pad: u32, pub key_dma: dma_addr_t, pub protected_key_dma: dma_addr_t, pub key_virt: *const core::ffi::c_void, pub key_inline: bool, pub plain_keylen: u32, pub key_cmd_opt: u32 }

#[inline]
pub unsafe fn desc_inline_query(sd_base_len: u32, jd_len: u32, data_len: *mut u32, inl_mask: *mut u32, count: u32) -> i32 {
    let mut rem_bytes = CAAM_DESC_BYTES_MAX as i32 - sd_base_len as i32 - jd_len as i32; *inl_mask = 0;
    let mut i = 0; while i < count && rem_bytes > 0 { let item = *data_len.add(i as usize); if rem_bytes - (item as i32 + ((count - i - 1) as usize * caam_ptr_sz) as i32) >= 0 { rem_bytes -= item as i32; *inl_mask |= 1 << i; } else { rem_bytes -= caam_ptr_sz as i32; } i += 1; } if rem_bytes >= 0 { 0 } else { -1 }
}

#[inline]
pub unsafe fn append_proto_dkp(desc: *mut u32, adata: *mut alginfo) {
    let protid = ((*adata).algtype & OP_ALG_ALGSEL_SUBMASK) | (0x20 << OP_ALG_ALGSEL_SHIFT);
    if (*adata).key_inline {
        let words: usize;
        if (*adata).keylen > (*adata).keylen_pad { append_operation(desc, OP_TYPE_UNI_PROTOCOL | protid | OP_PCL_DKP_SRC_PTR | OP_PCL_DKP_DST_IMM | (*adata).keylen); append_ptr(desc, (*adata).key_dma); words = (ALIGN((*adata).keylen_pad as usize, CAAM_CMD_SZ) - caam_ptr_sz) / CAAM_CMD_SZ; }
        else { append_operation(desc, OP_TYPE_UNI_PROTOCOL | protid | OP_PCL_DKP_SRC_IMM | OP_PCL_DKP_DST_IMM | (*adata).keylen); append_data(desc, (*adata).key_virt, (*adata).keylen as i32); words = (ALIGN((*adata).keylen_pad as usize, CAAM_CMD_SZ) - ALIGN((*adata).keylen as usize, CAAM_CMD_SZ)) / CAAM_CMD_SZ; }
        if words != 0 { *desc = cpu_to_caam32(caam32_to_cpu(*desc) + words as u32); }
    } else { append_operation(desc, OP_TYPE_UNI_PROTOCOL | protid | OP_PCL_DKP_SRC_PTR | OP_PCL_DKP_DST_PTR | (*adata).keylen); append_ptr(desc, (*adata).key_dma); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
