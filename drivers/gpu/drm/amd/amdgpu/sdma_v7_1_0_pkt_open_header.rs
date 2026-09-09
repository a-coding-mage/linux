/*
 * Copyright 2025 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 */

pub const SDMA_OP_NOP: u32 = 0;
pub const SDMA_OP_COPY: u32 = 1;
pub const SDMA_OP_WRITE: u32 = 2;
pub const SDMA_OP_INDIRECT: u32 = 4;
pub const SDMA_OP_FENCE: u32 = 5;
pub const SDMA_OP_TRAP: u32 = 6;
pub const SDMA_OP_SEM: u32 = 7;
pub const SDMA_OP_POLL_REGMEM: u32 = 8;
pub const SDMA_OP_COND_EXE: u32 = 9;
pub const SDMA_OP_ATOMIC: u32 = 10;
pub const SDMA_OP_CONST_FILL: u32 = 11;
pub const SDMA_OP_PTEPDE: u32 = 12;
pub const SDMA_OP_TIMESTAMP: u32 = 13;
pub const SDMA_OP_SRBM_WRITE: u32 = 14;
pub const SDMA_OP_PRE_EXE: u32 = 15;
pub const SDMA_OP_GPUVM_INV: u32 = 16;
pub const SDMA_OP_GCR_REQ: u32 = 17;
pub const SDMA_OP_DUMMY_TRAP: u32 = 32;
pub const SDMA_SUBOP_TIMESTAMP_SET: u32 = 0;
pub const SDMA_SUBOP_TIMESTAMP_GET: u32 = 1;
pub const SDMA_SUBOP_TIMESTAMP_GET_GLOBAL: u32 = 2;
pub const SDMA_SUBOP_COPY_LINEAR: u32 = 0;
pub const SDMA_SUBOP_COPY_LINEAR_SUB_WIND: u32 = 4;
pub const SDMA_SUBOP_COPY_TILED: u32 = 1;
pub const SDMA_SUBOP_COPY_TILED_SUB_WIND: u32 = 5;
pub const SDMA_SUBOP_COPY_T2T_SUB_WIND: u32 = 6;
pub const SDMA_SUBOP_COPY_SOA: u32 = 3;
pub const SDMA_SUBOP_COPY_DIRTY_PAGE: u32 = 7;
pub const SDMA_SUBOP_COPY_LINEAR_PHY: u32 = 8;
pub const SDMA_SUBOP_COPY_LINEAR_SUB_WIND_LARGE: u32 = 36;
pub const SDMA_SUBOP_COPY_LINEAR_BC: u32 = 16;
pub const SDMA_SUBOP_COPY_TILED_BC: u32 = 17;
pub const SDMA_SUBOP_COPY_LINEAR_SUB_WIND_BC: u32 = 20;
pub const SDMA_SUBOP_COPY_TILED_SUB_WIND_BC: u32 = 21;
pub const SDMA_SUBOP_COPY_T2T_SUB_WIND_BC: u32 = 22;
pub const SDMA_SUBOP_WRITE_LINEAR: u32 = 0;
pub const SDMA_SUBOP_WRITE_TILED: u32 = 1;
pub const SDMA_SUBOP_WRITE_TILED_BC: u32 = 17;
pub const SDMA_SUBOP_PTEPDE_GEN: u32 = 0;
pub const SDMA_SUBOP_PTEPDE_COPY: u32 = 1;
pub const SDMA_SUBOP_PTEPDE_RMW: u32 = 2;
pub const SDMA_SUBOP_PTEPDE_COPY_BACKWARDS: u32 = 3;
pub const SDMA_SUBOP_MEM_INCR: u32 = 1;
pub const SDMA_SUBOP_DATA_FILL_MULTI: u32 = 1;
pub const SDMA_SUBOP_POLL_REG_WRITE_MEM: u32 = 1;
pub const SDMA_SUBOP_POLL_DBIT_WRITE_MEM: u32 = 2;
pub const SDMA_SUBOP_POLL_MEM_VERIFY: u32 = 3;
pub const SDMA_SUBOP_VM_INVALIDATION: u32 = 4;
pub const HEADER_AGENT_DISPATCH: u32 = 4;
pub const HEADER_BARRIER: u32 = 5;
pub const SDMA_OP_AQL_COPY: u32 = 0;
pub const SDMA_OP_AQL_BARRIER_OR: u32 = 0;

pub const SDMA_GCR_RANGE_IS_PA: u32 = (1 << 18);
macro_rules! SDMA_GCR_SEQ {
    ($x:expr) => { ((($x) & 0$x3) << 16) };
}
pub const SDMA_GCR_GL2_WB: u32 = (1 << 15);
pub const SDMA_GCR_GL2_INV: u32 = (1 << 14);
pub const SDMA_GCR_GL2_DISCARD: u32 = (1 << 13);
macro_rules! SDMA_GCR_GL2_RANGE {
    ($x:expr) => { ((($x) & 0$x3) << 11) };
}
pub const SDMA_GCR_GL2_US: u32 = (1 << 10);
pub const SDMA_GCR_GL1_INV: u32 = (1 << 9);
pub const SDMA_GCR_GLV_INV: u32 = (1 << 8);
pub const SDMA_GCR_GLK_INV: u32 = (1 << 7);
pub const SDMA_GCR_GLK_WB: u32 = (1 << 6);
pub const SDMA_GCR_GLM_INV: u32 = (1 << 5);
pub const SDMA_GCR_GLM_WB: u32 = (1 << 4);
macro_rules! SDMA_GCR_GL1_RANGE {
    ($x:expr) => { ((($x) & 0$x3) << 2) };
}
macro_rules! SDMA_GCR_GLI_INV {
    ($x:expr) => { ((($x) & 0$x3) << 0) };
}

macro_rules! SDMA_DCC_DATA_FORMAT {
    ($x:expr) => { (($x) & 0$x3f) };
}
macro_rules! SDMA_DCC_NUM_TYPE {
    ($x:expr) => { ((($x) & 0$x7) << 9) };
}
macro_rules! SDMA_DCC_READ_CM {
    ($x:expr) => { ((($x) & 0$x3) << 16) };
}
macro_rules! SDMA_DCC_WRITE_CM {
    ($x:expr) => { ((($x) & 0$x3) << 18) };
}
macro_rules! SDMA_DCC_MAX_COM {
    ($x:expr) => { ((($x) & 0$x3) << 24) };
}
macro_rules! SDMA_DCC_MAX_UCOM {
    ($x:expr) => { ((($x) & 0$x1) << 26) };
}

/*
** Definitions for SDMA_PKT_COPY_LINEAR packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_COPY_LINEAR_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_LINEAR_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_HEADER_op_mask) << SDMA_PKT_COPY_LINEAR_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_COPY_LINEAR_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_LINEAR_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_LINEAR_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_HEADER_sub_op_mask) << SDMA_PKT_COPY_LINEAR_HEADER_sub_op_shift) };
}

/*define for encrypt field*/
pub const SDMA_PKT_COPY_LINEAR_HEADER_encrypt_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_HEADER_encrypt_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_HEADER_encrypt_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_LINEAR_HEADER_ENCRYPT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_HEADER_encrypt_mask) << SDMA_PKT_COPY_LINEAR_HEADER_encrypt_shift) };
}

/*define for tmz field*/
pub const SDMA_PKT_COPY_LINEAR_HEADER_tmz_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_HEADER_tmz_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_HEADER_tmz_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_LINEAR_HEADER_TMZ {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_HEADER_tmz_mask) << SDMA_PKT_COPY_LINEAR_HEADER_tmz_shift) };
}

/*define for npd field*/
pub const SDMA_PKT_COPY_LINEAR_HEADER_npd_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_HEADER_npd_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_HEADER_npd_shift: u32 = 28;
macro_rules! SDMA_PKT_COPY_LINEAR_HEADER_NPD {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_HEADER_npd_mask) << SDMA_PKT_COPY_LINEAR_HEADER_npd_shift) };
}

/*define for backwards field*/
pub const SDMA_PKT_COPY_LINEAR_HEADER_backwards_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_HEADER_backwards_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_HEADER_backwards_shift: u32 = 25;
macro_rules! SDMA_PKT_COPY_LINEAR_HEADER_BACKWARDS {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_HEADER_backwards_mask) << SDMA_PKT_COPY_LINEAR_HEADER_backwards_shift) };
}

/*define for broadcast field*/
pub const SDMA_PKT_COPY_LINEAR_HEADER_broadcast_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_HEADER_broadcast_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_HEADER_broadcast_shift: u32 = 27;
macro_rules! SDMA_PKT_COPY_LINEAR_HEADER_BROADCAST {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_HEADER_broadcast_mask) << SDMA_PKT_COPY_LINEAR_HEADER_broadcast_shift) };
}

/*define for COUNT word*/
/*define for count field*/
pub const SDMA_PKT_COPY_LINEAR_COUNT_count_offset: u32 = 1;
pub const SDMA_PKT_COPY_LINEAR_COUNT_count_mask: u32 = 0x3FFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_COUNT_count_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_COUNT_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_COUNT_count_mask) << SDMA_PKT_COPY_LINEAR_COUNT_count_shift) };
}

/*define for PARAMETER word*/
/*define for dst_sw field*/
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_dst_sw_offset: u32 = 2;
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_dst_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_dst_sw_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_LINEAR_PARAMETER_DST_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_PARAMETER_dst_sw_mask) << SDMA_PKT_COPY_LINEAR_PARAMETER_dst_sw_shift) };
}

/*define for dst_cache_policy field*/
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_dst_cache_policy_offset: u32 = 2;
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_dst_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_dst_cache_policy_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_LINEAR_PARAMETER_DST_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_PARAMETER_dst_cache_policy_mask) << SDMA_PKT_COPY_LINEAR_PARAMETER_dst_cache_policy_shift) };
}

/*define for src_sw field*/
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_src_sw_offset: u32 = 2;
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_src_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_src_sw_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_LINEAR_PARAMETER_SRC_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_PARAMETER_src_sw_mask) << SDMA_PKT_COPY_LINEAR_PARAMETER_src_sw_shift) };
}

/*define for src_cache_policy field*/
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_src_cache_policy_offset: u32 = 2;
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_src_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_LINEAR_PARAMETER_src_cache_policy_shift: u32 = 26;
macro_rules! SDMA_PKT_COPY_LINEAR_PARAMETER_SRC_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_PARAMETER_src_cache_policy_mask) << SDMA_PKT_COPY_LINEAR_PARAMETER_src_cache_policy_shift) };
}

/*define for SRC_ADDR_LO word*/
/*define for src_addr_31_0 field*/
pub const SDMA_PKT_COPY_LINEAR_SRC_ADDR_LO_src_addr_31_0_offset: u32 = 3;
pub const SDMA_PKT_COPY_LINEAR_SRC_ADDR_LO_src_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SRC_ADDR_LO_src_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SRC_ADDR_LO_SRC_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SRC_ADDR_LO_src_addr_31_0_mask) << SDMA_PKT_COPY_LINEAR_SRC_ADDR_LO_src_addr_31_0_shift) };
}

/*define for SRC_ADDR_HI word*/
/*define for src_addr_63_32 field*/
pub const SDMA_PKT_COPY_LINEAR_SRC_ADDR_HI_src_addr_63_32_offset: u32 = 4;
pub const SDMA_PKT_COPY_LINEAR_SRC_ADDR_HI_src_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SRC_ADDR_HI_src_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SRC_ADDR_HI_SRC_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SRC_ADDR_HI_src_addr_63_32_mask) << SDMA_PKT_COPY_LINEAR_SRC_ADDR_HI_src_addr_63_32_shift) };
}

/*define for DST_ADDR_LO word*/
/*define for dst_addr_31_0 field*/
pub const SDMA_PKT_COPY_LINEAR_DST_ADDR_LO_dst_addr_31_0_offset: u32 = 5;
pub const SDMA_PKT_COPY_LINEAR_DST_ADDR_LO_dst_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_DST_ADDR_LO_dst_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_DST_ADDR_LO_DST_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_DST_ADDR_LO_dst_addr_31_0_mask) << SDMA_PKT_COPY_LINEAR_DST_ADDR_LO_dst_addr_31_0_shift) };
}

/*define for DST_ADDR_HI word*/
/*define for dst_addr_63_32 field*/
pub const SDMA_PKT_COPY_LINEAR_DST_ADDR_HI_dst_addr_63_32_offset: u32 = 6;
pub const SDMA_PKT_COPY_LINEAR_DST_ADDR_HI_dst_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_DST_ADDR_HI_dst_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_DST_ADDR_HI_DST_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_DST_ADDR_HI_dst_addr_63_32_mask) << SDMA_PKT_COPY_LINEAR_DST_ADDR_HI_dst_addr_63_32_shift) };
}


/*
** Definitions for SDMA_PKT_COPY_LINEAR_BC packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_COPY_LINEAR_BC_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_BC_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_LINEAR_BC_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_BC_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_BC_HEADER_op_mask) << SDMA_PKT_COPY_LINEAR_BC_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_COPY_LINEAR_BC_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_BC_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_LINEAR_BC_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_LINEAR_BC_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_BC_HEADER_sub_op_mask) << SDMA_PKT_COPY_LINEAR_BC_HEADER_sub_op_shift) };
}

/*define for COUNT word*/
/*define for count field*/
pub const SDMA_PKT_COPY_LINEAR_BC_COUNT_count_offset: u32 = 1;
pub const SDMA_PKT_COPY_LINEAR_BC_COUNT_count_mask: u32 = 0x003FFFFF;
pub const SDMA_PKT_COPY_LINEAR_BC_COUNT_count_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_BC_COUNT_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_BC_COUNT_count_mask) << SDMA_PKT_COPY_LINEAR_BC_COUNT_count_shift) };
}

/*define for PARAMETER word*/
/*define for dst_sw field*/
pub const SDMA_PKT_COPY_LINEAR_BC_PARAMETER_dst_sw_offset: u32 = 2;
pub const SDMA_PKT_COPY_LINEAR_BC_PARAMETER_dst_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_LINEAR_BC_PARAMETER_dst_sw_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_LINEAR_BC_PARAMETER_DST_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_BC_PARAMETER_dst_sw_mask) << SDMA_PKT_COPY_LINEAR_BC_PARAMETER_dst_sw_shift) };
}

/*define for dst_ha field*/
pub const SDMA_PKT_COPY_LINEAR_BC_PARAMETER_dst_ha_offset: u32 = 2;
pub const SDMA_PKT_COPY_LINEAR_BC_PARAMETER_dst_ha_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_BC_PARAMETER_dst_ha_shift: u32 = 19;
macro_rules! SDMA_PKT_COPY_LINEAR_BC_PARAMETER_DST_HA {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_BC_PARAMETER_dst_ha_mask) << SDMA_PKT_COPY_LINEAR_BC_PARAMETER_dst_ha_shift) };
}

/*define for src_sw field*/
pub const SDMA_PKT_COPY_LINEAR_BC_PARAMETER_src_sw_offset: u32 = 2;
pub const SDMA_PKT_COPY_LINEAR_BC_PARAMETER_src_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_LINEAR_BC_PARAMETER_src_sw_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_LINEAR_BC_PARAMETER_SRC_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_BC_PARAMETER_src_sw_mask) << SDMA_PKT_COPY_LINEAR_BC_PARAMETER_src_sw_shift) };
}

/*define for src_ha field*/
pub const SDMA_PKT_COPY_LINEAR_BC_PARAMETER_src_ha_offset: u32 = 2;
pub const SDMA_PKT_COPY_LINEAR_BC_PARAMETER_src_ha_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_BC_PARAMETER_src_ha_shift: u32 = 27;
macro_rules! SDMA_PKT_COPY_LINEAR_BC_PARAMETER_SRC_HA {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_BC_PARAMETER_src_ha_mask) << SDMA_PKT_COPY_LINEAR_BC_PARAMETER_src_ha_shift) };
}

/*define for SRC_ADDR_LO word*/
/*define for src_addr_31_0 field*/
pub const SDMA_PKT_COPY_LINEAR_BC_SRC_ADDR_LO_src_addr_31_0_offset: u32 = 3;
pub const SDMA_PKT_COPY_LINEAR_BC_SRC_ADDR_LO_src_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_BC_SRC_ADDR_LO_src_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_BC_SRC_ADDR_LO_SRC_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_BC_SRC_ADDR_LO_src_addr_31_0_mask) << SDMA_PKT_COPY_LINEAR_BC_SRC_ADDR_LO_src_addr_31_0_shift) };
}

/*define for SRC_ADDR_HI word*/
/*define for src_addr_63_32 field*/
pub const SDMA_PKT_COPY_LINEAR_BC_SRC_ADDR_HI_src_addr_63_32_offset: u32 = 4;
pub const SDMA_PKT_COPY_LINEAR_BC_SRC_ADDR_HI_src_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_BC_SRC_ADDR_HI_src_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_BC_SRC_ADDR_HI_SRC_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_BC_SRC_ADDR_HI_src_addr_63_32_mask) << SDMA_PKT_COPY_LINEAR_BC_SRC_ADDR_HI_src_addr_63_32_shift) };
}

/*define for DST_ADDR_LO word*/
/*define for dst_addr_31_0 field*/
pub const SDMA_PKT_COPY_LINEAR_BC_DST_ADDR_LO_dst_addr_31_0_offset: u32 = 5;
pub const SDMA_PKT_COPY_LINEAR_BC_DST_ADDR_LO_dst_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_BC_DST_ADDR_LO_dst_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_BC_DST_ADDR_LO_DST_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_BC_DST_ADDR_LO_dst_addr_31_0_mask) << SDMA_PKT_COPY_LINEAR_BC_DST_ADDR_LO_dst_addr_31_0_shift) };
}

/*define for DST_ADDR_HI word*/
/*define for dst_addr_63_32 field*/
pub const SDMA_PKT_COPY_LINEAR_BC_DST_ADDR_HI_dst_addr_63_32_offset: u32 = 6;
pub const SDMA_PKT_COPY_LINEAR_BC_DST_ADDR_HI_dst_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_BC_DST_ADDR_HI_dst_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_BC_DST_ADDR_HI_DST_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_BC_DST_ADDR_HI_dst_addr_63_32_mask) << SDMA_PKT_COPY_LINEAR_BC_DST_ADDR_HI_dst_addr_63_32_shift) };
}


/*
** Definitions for SDMA_PKT_COPY_DIRTY_PAGE packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_HEADER_op_mask) << SDMA_PKT_COPY_DIRTY_PAGE_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_HEADER_sub_op_mask) << SDMA_PKT_COPY_DIRTY_PAGE_HEADER_sub_op_shift) };
}

/*define for tmz field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_tmz_offset: u32 = 0;
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_tmz_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_tmz_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_HEADER_TMZ {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_HEADER_tmz_mask) << SDMA_PKT_COPY_DIRTY_PAGE_HEADER_tmz_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_cpv_shift: u32 = 19;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_HEADER_cpv_mask) << SDMA_PKT_COPY_DIRTY_PAGE_HEADER_cpv_shift) };
}

/*define for all field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_all_offset: u32 = 0;
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_all_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_HEADER_all_shift: u32 = 31;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_HEADER_ALL {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_HEADER_all_mask) << SDMA_PKT_COPY_DIRTY_PAGE_HEADER_all_shift) };
}

/*define for COUNT word*/
/*define for count field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_COUNT_count_offset: u32 = 1;
pub const SDMA_PKT_COPY_DIRTY_PAGE_COUNT_count_mask: u32 = 0x003FFFFF;
pub const SDMA_PKT_COPY_DIRTY_PAGE_COUNT_count_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_COUNT_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_COUNT_count_mask) << SDMA_PKT_COPY_DIRTY_PAGE_COUNT_count_shift) };
}

/*define for PARAMETER word*/
/*define for dst_mtype field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_mtype_offset: u32 = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_mtype_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_mtype_shift: u32 = 3;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_DST_MTYPE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_mtype_mask) << SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_mtype_shift) };
}

/*define for dst_l2_policy field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_l2_policy_offset: u32 = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_l2_policy_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_l2_policy_shift: u32 = 6;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_DST_L2_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_l2_policy_mask) << SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_l2_policy_shift) };
}

/*define for dst_llc field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_llc_offset: u32 = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_llc_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_llc_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_DST_LLC {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_llc_mask) << SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_llc_shift) };
}

/*define for src_mtype field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_mtype_offset: u32 = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_mtype_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_mtype_shift: u32 = 11;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_SRC_MTYPE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_mtype_mask) << SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_mtype_shift) };
}

/*define for src_l2_policy field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_l2_policy_offset: u32 = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_l2_policy_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_l2_policy_shift: u32 = 14;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_SRC_L2_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_l2_policy_mask) << SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_l2_policy_shift) };
}

/*define for src_llc field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_llc_offset: u32 = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_llc_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_llc_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_SRC_LLC {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_llc_mask) << SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_llc_shift) };
}

/*define for dst_sw field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_sw_offset: u32 = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_sw_shift: u32 = 17;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_DST_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_sw_mask) << SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_sw_shift) };
}

/*define for dst_gcc field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_gcc_offset: u32 = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_gcc_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_gcc_shift: u32 = 19;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_DST_GCC {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_gcc_mask) << SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_gcc_shift) };
}

/*define for dst_sys field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_sys_offset: u32 = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_sys_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_sys_shift: u32 = 20;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_DST_SYS {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_sys_mask) << SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_sys_shift) };
}

/*define for dst_snoop field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_snoop_offset: u32 = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_snoop_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_snoop_shift: u32 = 22;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_DST_SNOOP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_snoop_mask) << SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_snoop_shift) };
}

/*define for dst_gpa field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_gpa_offset: u32 = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_gpa_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_gpa_shift: u32 = 23;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_DST_GPA {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_gpa_mask) << SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_dst_gpa_shift) };
}

/*define for src_sw field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_sw_offset: u32 = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_sw_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_SRC_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_sw_mask) << SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_sw_shift) };
}

/*define for src_sys field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_sys_offset: u32 = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_sys_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_sys_shift: u32 = 28;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_SRC_SYS {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_sys_mask) << SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_sys_shift) };
}

/*define for src_snoop field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_snoop_offset: u32 = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_snoop_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_snoop_shift: u32 = 30;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_SRC_SNOOP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_snoop_mask) << SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_snoop_shift) };
}

/*define for src_gpa field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_gpa_offset: u32 = 2;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_gpa_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_gpa_shift: u32 = 31;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_SRC_GPA {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_gpa_mask) << SDMA_PKT_COPY_DIRTY_PAGE_PARAMETER_src_gpa_shift) };
}

/*define for SRC_ADDR_LO word*/
/*define for src_addr_31_0 field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_SRC_ADDR_LO_src_addr_31_0_offset: u32 = 3;
pub const SDMA_PKT_COPY_DIRTY_PAGE_SRC_ADDR_LO_src_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_DIRTY_PAGE_SRC_ADDR_LO_src_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_SRC_ADDR_LO_SRC_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_SRC_ADDR_LO_src_addr_31_0_mask) << SDMA_PKT_COPY_DIRTY_PAGE_SRC_ADDR_LO_src_addr_31_0_shift) };
}

/*define for SRC_ADDR_HI word*/
/*define for src_addr_63_32 field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_SRC_ADDR_HI_src_addr_63_32_offset: u32 = 4;
pub const SDMA_PKT_COPY_DIRTY_PAGE_SRC_ADDR_HI_src_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_DIRTY_PAGE_SRC_ADDR_HI_src_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_SRC_ADDR_HI_SRC_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_SRC_ADDR_HI_src_addr_63_32_mask) << SDMA_PKT_COPY_DIRTY_PAGE_SRC_ADDR_HI_src_addr_63_32_shift) };
}

/*define for DST_ADDR_LO word*/
/*define for dst_addr_31_0 field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_DST_ADDR_LO_dst_addr_31_0_offset: u32 = 5;
pub const SDMA_PKT_COPY_DIRTY_PAGE_DST_ADDR_LO_dst_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_DIRTY_PAGE_DST_ADDR_LO_dst_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_DST_ADDR_LO_DST_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_DST_ADDR_LO_dst_addr_31_0_mask) << SDMA_PKT_COPY_DIRTY_PAGE_DST_ADDR_LO_dst_addr_31_0_shift) };
}

/*define for DST_ADDR_HI word*/
/*define for dst_addr_63_32 field*/
pub const SDMA_PKT_COPY_DIRTY_PAGE_DST_ADDR_HI_dst_addr_63_32_offset: u32 = 6;
pub const SDMA_PKT_COPY_DIRTY_PAGE_DST_ADDR_HI_dst_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_DIRTY_PAGE_DST_ADDR_HI_dst_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_DIRTY_PAGE_DST_ADDR_HI_DST_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_DIRTY_PAGE_DST_ADDR_HI_dst_addr_63_32_mask) << SDMA_PKT_COPY_DIRTY_PAGE_DST_ADDR_HI_dst_addr_63_32_shift) };
}


/*
** Definitions for SDMA_PKT_COPY_PHYSICAL_LINEAR packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_op_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_sub_op_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_sub_op_shift) };
}

/*define for tmz field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_tmz_offset: u32 = 0;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_tmz_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_tmz_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_TMZ {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_tmz_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_tmz_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_cpv_shift: u32 = 19;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_cpv_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_HEADER_cpv_shift) };
}

/*define for COUNT word*/
/*define for count field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_COUNT_count_offset: u32 = 1;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_COUNT_count_mask: u32 = 0x003FFFFF;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_COUNT_count_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_COUNT_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_COUNT_count_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_COUNT_count_shift) };
}

/*define for addr_pair_num field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_COUNT_addr_pair_num_offset: u32 = 1;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_COUNT_addr_pair_num_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_COUNT_addr_pair_num_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_COUNT_ADDR_PAIR_NUM {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_COUNT_addr_pair_num_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_COUNT_addr_pair_num_shift) };
}

/*define for PARAMETER word*/
/*define for dst_mtype field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_mtype_offset: u32 = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_mtype_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_mtype_shift: u32 = 3;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_DST_MTYPE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_mtype_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_mtype_shift) };
}

/*define for dst_l2_policy field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_l2_policy_offset: u32 = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_l2_policy_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_l2_policy_shift: u32 = 6;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_DST_L2_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_l2_policy_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_l2_policy_shift) };
}

/*define for dst_llc field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_llc_offset: u32 = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_llc_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_llc_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_DST_LLC {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_llc_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_llc_shift) };
}

/*define for src_mtype field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_mtype_offset: u32 = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_mtype_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_mtype_shift: u32 = 11;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_SRC_MTYPE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_mtype_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_mtype_shift) };
}

/*define for src_l2_policy field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_l2_policy_offset: u32 = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_l2_policy_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_l2_policy_shift: u32 = 14;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_SRC_L2_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_l2_policy_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_l2_policy_shift) };
}

/*define for src_llc field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_llc_offset: u32 = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_llc_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_llc_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_SRC_LLC {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_llc_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_llc_shift) };
}

/*define for dst_sw field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_sw_offset: u32 = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_sw_shift: u32 = 17;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_DST_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_sw_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_sw_shift) };
}

/*define for dst_gcc field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_gcc_offset: u32 = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_gcc_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_gcc_shift: u32 = 19;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_DST_GCC {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_gcc_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_gcc_shift) };
}

/*define for dst_sys field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_sys_offset: u32 = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_sys_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_sys_shift: u32 = 20;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_DST_SYS {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_sys_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_sys_shift) };
}

/*define for dst_log field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_log_offset: u32 = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_log_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_log_shift: u32 = 21;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_DST_LOG {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_log_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_log_shift) };
}

/*define for dst_snoop field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_snoop_offset: u32 = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_snoop_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_snoop_shift: u32 = 22;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_DST_SNOOP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_snoop_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_snoop_shift) };
}

/*define for dst_gpa field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_gpa_offset: u32 = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_gpa_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_gpa_shift: u32 = 23;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_DST_GPA {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_gpa_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_dst_gpa_shift) };
}

/*define for src_sw field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_sw_offset: u32 = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_sw_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_SRC_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_sw_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_sw_shift) };
}

/*define for src_gcc field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_gcc_offset: u32 = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_gcc_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_gcc_shift: u32 = 27;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_SRC_GCC {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_gcc_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_gcc_shift) };
}

/*define for src_sys field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_sys_offset: u32 = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_sys_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_sys_shift: u32 = 28;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_SRC_SYS {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_sys_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_sys_shift) };
}

/*define for src_snoop field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_snoop_offset: u32 = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_snoop_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_snoop_shift: u32 = 30;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_SRC_SNOOP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_snoop_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_snoop_shift) };
}

/*define for src_gpa field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_gpa_offset: u32 = 2;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_gpa_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_gpa_shift: u32 = 31;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_SRC_GPA {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_gpa_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_PARAMETER_src_gpa_shift) };
}

/*define for SRC_ADDR_LO word*/
/*define for src_addr_31_0 field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_SRC_ADDR_LO_src_addr_31_0_offset: u32 = 3;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_SRC_ADDR_LO_src_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_SRC_ADDR_LO_src_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_SRC_ADDR_LO_SRC_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_SRC_ADDR_LO_src_addr_31_0_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_SRC_ADDR_LO_src_addr_31_0_shift) };
}

/*define for SRC_ADDR_HI word*/
/*define for src_addr_63_32 field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_SRC_ADDR_HI_src_addr_63_32_offset: u32 = 4;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_SRC_ADDR_HI_src_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_SRC_ADDR_HI_src_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_SRC_ADDR_HI_SRC_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_SRC_ADDR_HI_src_addr_63_32_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_SRC_ADDR_HI_src_addr_63_32_shift) };
}

/*define for DST_ADDR_LO word*/
/*define for dst_addr_31_0 field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_DST_ADDR_LO_dst_addr_31_0_offset: u32 = 5;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_DST_ADDR_LO_dst_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_DST_ADDR_LO_dst_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_DST_ADDR_LO_DST_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_DST_ADDR_LO_dst_addr_31_0_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_DST_ADDR_LO_dst_addr_31_0_shift) };
}

/*define for DST_ADDR_HI word*/
/*define for dst_addr_63_32 field*/
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_DST_ADDR_HI_dst_addr_63_32_offset: u32 = 6;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_DST_ADDR_HI_dst_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_PHYSICAL_LINEAR_DST_ADDR_HI_dst_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_PHYSICAL_LINEAR_DST_ADDR_HI_DST_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_PHYSICAL_LINEAR_DST_ADDR_HI_dst_addr_63_32_mask) << SDMA_PKT_COPY_PHYSICAL_LINEAR_DST_ADDR_HI_dst_addr_63_32_shift) };
}


/*
** Definitions for SDMA_PKT_COPY_BROADCAST_LINEAR packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_op_mask) << SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_sub_op_mask) << SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_sub_op_shift) };
}

/*define for encrypt field*/
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_encrypt_offset: u32 = 0;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_encrypt_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_encrypt_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_ENCRYPT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_encrypt_mask) << SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_encrypt_shift) };
}

/*define for tmz field*/
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_tmz_offset: u32 = 0;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_tmz_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_tmz_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_TMZ {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_tmz_mask) << SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_tmz_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_cpv_shift: u32 = 19;
macro_rules! SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_cpv_mask) << SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_cpv_shift) };
}

/*define for broadcast field*/
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_broadcast_offset: u32 = 0;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_broadcast_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_broadcast_shift: u32 = 27;
macro_rules! SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_BROADCAST {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_broadcast_mask) << SDMA_PKT_COPY_BROADCAST_LINEAR_HEADER_broadcast_shift) };
}

/*define for COUNT word*/
/*define for count field*/
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_COUNT_count_offset: u32 = 1;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_COUNT_count_mask: u32 = 0x3FFFFFFF;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_COUNT_count_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_BROADCAST_LINEAR_COUNT_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_BROADCAST_LINEAR_COUNT_count_mask) << SDMA_PKT_COPY_BROADCAST_LINEAR_COUNT_count_shift) };
}

/*define for PARAMETER word*/
/*define for dst2_sw field*/
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst2_sw_offset: u32 = 2;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst2_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst2_sw_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_DST2_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst2_sw_mask) << SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst2_sw_shift) };
}

/*define for dst2_cache_policy field*/
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst2_cache_policy_offset: u32 = 2;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst2_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst2_cache_policy_shift: u32 = 10;
macro_rules! SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_DST2_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst2_cache_policy_mask) << SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst2_cache_policy_shift) };
}

/*define for dst1_sw field*/
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst1_sw_offset: u32 = 2;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst1_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst1_sw_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_DST1_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst1_sw_mask) << SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst1_sw_shift) };
}

/*define for dst1_cache_policy field*/
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst1_cache_policy_offset: u32 = 2;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst1_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst1_cache_policy_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_DST1_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst1_cache_policy_mask) << SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_dst1_cache_policy_shift) };
}

/*define for src_sw field*/
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_src_sw_offset: u32 = 2;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_src_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_src_sw_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_SRC_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_src_sw_mask) << SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_src_sw_shift) };
}

/*define for src_cache_policy field*/
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_src_cache_policy_offset: u32 = 2;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_src_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_src_cache_policy_shift: u32 = 26;
macro_rules! SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_SRC_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_src_cache_policy_mask) << SDMA_PKT_COPY_BROADCAST_LINEAR_PARAMETER_src_cache_policy_shift) };
}

/*define for SRC_ADDR_LO word*/
/*define for src_addr_31_0 field*/
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_SRC_ADDR_LO_src_addr_31_0_offset: u32 = 3;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_SRC_ADDR_LO_src_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_SRC_ADDR_LO_src_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_BROADCAST_LINEAR_SRC_ADDR_LO_SRC_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_BROADCAST_LINEAR_SRC_ADDR_LO_src_addr_31_0_mask) << SDMA_PKT_COPY_BROADCAST_LINEAR_SRC_ADDR_LO_src_addr_31_0_shift) };
}

/*define for SRC_ADDR_HI word*/
/*define for src_addr_63_32 field*/
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_SRC_ADDR_HI_src_addr_63_32_offset: u32 = 4;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_SRC_ADDR_HI_src_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_SRC_ADDR_HI_src_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_BROADCAST_LINEAR_SRC_ADDR_HI_SRC_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_BROADCAST_LINEAR_SRC_ADDR_HI_src_addr_63_32_mask) << SDMA_PKT_COPY_BROADCAST_LINEAR_SRC_ADDR_HI_src_addr_63_32_shift) };
}

/*define for DST1_ADDR_LO word*/
/*define for dst1_addr_31_0 field*/
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST1_ADDR_LO_dst1_addr_31_0_offset: u32 = 5;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST1_ADDR_LO_dst1_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST1_ADDR_LO_dst1_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_BROADCAST_LINEAR_DST1_ADDR_LO_DST1_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_BROADCAST_LINEAR_DST1_ADDR_LO_dst1_addr_31_0_mask) << SDMA_PKT_COPY_BROADCAST_LINEAR_DST1_ADDR_LO_dst1_addr_31_0_shift) };
}

/*define for DST1_ADDR_HI word*/
/*define for dst1_addr_63_32 field*/
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST1_ADDR_HI_dst1_addr_63_32_offset: u32 = 6;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST1_ADDR_HI_dst1_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST1_ADDR_HI_dst1_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_BROADCAST_LINEAR_DST1_ADDR_HI_DST1_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_BROADCAST_LINEAR_DST1_ADDR_HI_dst1_addr_63_32_mask) << SDMA_PKT_COPY_BROADCAST_LINEAR_DST1_ADDR_HI_dst1_addr_63_32_shift) };
}

/*define for DST2_ADDR_LO word*/
/*define for dst2_addr_31_0 field*/
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST2_ADDR_LO_dst2_addr_31_0_offset: u32 = 7;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST2_ADDR_LO_dst2_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST2_ADDR_LO_dst2_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_BROADCAST_LINEAR_DST2_ADDR_LO_DST2_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_BROADCAST_LINEAR_DST2_ADDR_LO_dst2_addr_31_0_mask) << SDMA_PKT_COPY_BROADCAST_LINEAR_DST2_ADDR_LO_dst2_addr_31_0_shift) };
}

/*define for DST2_ADDR_HI word*/
/*define for dst2_addr_63_32 field*/
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST2_ADDR_HI_dst2_addr_63_32_offset: u32 = 8;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST2_ADDR_HI_dst2_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_BROADCAST_LINEAR_DST2_ADDR_HI_dst2_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_BROADCAST_LINEAR_DST2_ADDR_HI_DST2_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_BROADCAST_LINEAR_DST2_ADDR_HI_dst2_addr_63_32_mask) << SDMA_PKT_COPY_BROADCAST_LINEAR_DST2_ADDR_HI_dst2_addr_63_32_shift) };
}


/*
** Definitions for SDMA_PKT_COPY_LINEAR_SUBWIN packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_op_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_sub_op_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_sub_op_shift) };
}

/*define for tmz field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_tmz_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_tmz_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_tmz_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_TMZ {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_tmz_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_tmz_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_cpv_shift: u32 = 19;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_cpv_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_cpv_shift) };
}

/*define for elementsize field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_elementsize_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_elementsize_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_elementsize_shift: u32 = 29;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_ELEMENTSIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_elementsize_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_HEADER_elementsize_shift) };
}

/*define for SRC_ADDR_LO word*/
/*define for src_addr_31_0 field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_SRC_ADDR_LO_src_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_SRC_ADDR_LO_src_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_SRC_ADDR_LO_src_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_SRC_ADDR_LO_SRC_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_SRC_ADDR_LO_src_addr_31_0_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_SRC_ADDR_LO_src_addr_31_0_shift) };
}

/*define for SRC_ADDR_HI word*/
/*define for src_addr_63_32 field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_SRC_ADDR_HI_src_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_SRC_ADDR_HI_src_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_SRC_ADDR_HI_src_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_SRC_ADDR_HI_SRC_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_SRC_ADDR_HI_src_addr_63_32_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_SRC_ADDR_HI_src_addr_63_32_shift) };
}

/*define for DW_3 word*/
/*define for src_x field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_3_src_x_offset: u32 = 3;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_3_src_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_3_src_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_DW_3_SRC_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_DW_3_src_$x_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_DW_3_src_$x_shift) };
}

/*define for src_y field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_3_src_y_offset: u32 = 3;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_3_src_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_3_src_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_DW_3_SRC_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_DW_3_src_y_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_DW_3_src_y_shift) };
}

/*define for DW_4 word*/
/*define for src_z field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_4_src_z_offset: u32 = 4;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_4_src_z_mask: u32 = 0x00001FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_4_src_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_DW_4_SRC_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_DW_4_src_z_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_DW_4_src_z_shift) };
}

/*define for src_pitch field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_4_src_pitch_offset: u32 = 4;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_4_src_pitch_mask: u32 = 0x0007FFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_4_src_pitch_shift: u32 = 13;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_DW_4_SRC_PITCH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_DW_4_src_pitch_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_DW_4_src_pitch_shift) };
}

/*define for DW_5 word*/
/*define for src_slice_pitch field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_5_src_slice_pitch_offset: u32 = 5;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_5_src_slice_pitch_mask: u32 = 0x0FFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_5_src_slice_pitch_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_DW_5_SRC_SLICE_PITCH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_DW_5_src_slice_pitch_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_DW_5_src_slice_pitch_shift) };
}

/*define for DST_ADDR_LO word*/
/*define for dst_addr_31_0 field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DST_ADDR_LO_dst_addr_31_0_offset: u32 = 6;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DST_ADDR_LO_dst_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DST_ADDR_LO_dst_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_DST_ADDR_LO_DST_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_DST_ADDR_LO_dst_addr_31_0_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_DST_ADDR_LO_dst_addr_31_0_shift) };
}

/*define for DST_ADDR_HI word*/
/*define for dst_addr_63_32 field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DST_ADDR_HI_dst_addr_63_32_offset: u32 = 7;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DST_ADDR_HI_dst_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DST_ADDR_HI_dst_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_DST_ADDR_HI_DST_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_DST_ADDR_HI_dst_addr_63_32_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_DST_ADDR_HI_dst_addr_63_32_shift) };
}

/*define for DW_8 word*/
/*define for dst_x field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_8_dst_x_offset: u32 = 8;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_8_dst_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_8_dst_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_DW_8_DST_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_DW_8_dst_$x_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_DW_8_dst_$x_shift) };
}

/*define for dst_y field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_8_dst_y_offset: u32 = 8;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_8_dst_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_8_dst_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_DW_8_DST_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_DW_8_dst_y_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_DW_8_dst_y_shift) };
}

/*define for DW_9 word*/
/*define for dst_z field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_9_dst_z_offset: u32 = 9;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_9_dst_z_mask: u32 = 0x00001FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_9_dst_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_DW_9_DST_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_DW_9_dst_z_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_DW_9_dst_z_shift) };
}

/*define for dst_pitch field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_9_dst_pitch_offset: u32 = 9;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_9_dst_pitch_mask: u32 = 0x0007FFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_9_dst_pitch_shift: u32 = 13;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_DW_9_DST_PITCH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_DW_9_dst_pitch_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_DW_9_dst_pitch_shift) };
}

/*define for DW_10 word*/
/*define for dst_slice_pitch field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_10_dst_slice_pitch_offset: u32 = 10;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_10_dst_slice_pitch_mask: u32 = 0x0FFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_10_dst_slice_pitch_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_DW_10_DST_SLICE_PITCH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_DW_10_dst_slice_pitch_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_DW_10_dst_slice_pitch_shift) };
}

/*define for DW_11 word*/
/*define for rect_x field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_11_rect_x_offset: u32 = 11;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_11_rect_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_11_rect_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_DW_11_RECT_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_DW_11_rect_$x_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_DW_11_rect_$x_shift) };
}

/*define for rect_y field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_11_rect_y_offset: u32 = 11;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_11_rect_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_11_rect_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_DW_11_RECT_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_DW_11_rect_y_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_DW_11_rect_y_shift) };
}

/*define for DW_12 word*/
/*define for rect_z field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_rect_z_offset: u32 = 12;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_rect_z_mask: u32 = 0x00001FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_rect_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_RECT_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_rect_z_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_rect_z_shift) };
}

/*define for dst_sw field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_dst_sw_offset: u32 = 12;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_dst_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_dst_sw_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_DST_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_dst_sw_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_dst_sw_shift) };
}

/*define for dst_cache_policy field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_dst_cache_policy_offset: u32 = 12;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_dst_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_dst_cache_policy_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_DST_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_dst_cache_policy_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_dst_cache_policy_shift) };
}

/*define for src_sw field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_src_sw_offset: u32 = 12;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_src_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_src_sw_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_SRC_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_src_sw_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_src_sw_shift) };
}

/*define for src_cache_policy field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_src_cache_policy_offset: u32 = 12;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_src_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_src_cache_policy_shift: u32 = 26;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_SRC_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_src_cache_policy_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_DW_12_src_cache_policy_shift) };
}


/*
** Definitions for SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_op_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_sub_op_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_sub_op_shift) };
}

/*define for tmz field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_tmz_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_tmz_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_tmz_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_TMZ {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_tmz_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_tmz_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_cpv_shift: u32 = 19;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_cpv_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_HEADER_cpv_shift) };
}

/*define for SRC_ADDR_LO word*/
/*define for src_addr_31_0 field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_SRC_ADDR_LO_src_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_SRC_ADDR_LO_src_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_SRC_ADDR_LO_src_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_SRC_ADDR_LO_SRC_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_SRC_ADDR_LO_src_addr_31_0_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_SRC_ADDR_LO_src_addr_31_0_shift) };
}

/*define for SRC_ADDR_HI word*/
/*define for src_addr_63_32 field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_SRC_ADDR_HI_src_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_SRC_ADDR_HI_src_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_SRC_ADDR_HI_src_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_SRC_ADDR_HI_SRC_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_SRC_ADDR_HI_src_addr_63_32_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_SRC_ADDR_HI_src_addr_63_32_shift) };
}

/*define for DW_3 word*/
/*define for src_x field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_3_src_x_offset: u32 = 3;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_3_src_x_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_3_src_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_3_SRC_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_3_src_$x_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_3_src_$x_shift) };
}

/*define for DW_4 word*/
/*define for src_y field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_4_src_y_offset: u32 = 4;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_4_src_y_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_4_src_y_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_4_SRC_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_4_src_y_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_4_src_y_shift) };
}

/*define for DW_5 word*/
/*define for src_z field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_5_src_z_offset: u32 = 5;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_5_src_z_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_5_src_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_5_SRC_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_5_src_z_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_5_src_z_shift) };
}

/*define for DW_6 word*/
/*define for src_pitch field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_6_src_pitch_offset: u32 = 6;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_6_src_pitch_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_6_src_pitch_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_6_SRC_PITCH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_6_src_pitch_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_6_src_pitch_shift) };
}

/*define for DW_7 word*/
/*define for src_slice_pitch_31_0 field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_7_src_slice_pitch_31_0_offset: u32 = 7;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_7_src_slice_pitch_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_7_src_slice_pitch_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_7_SRC_SLICE_PITCH_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_7_src_slice_pitch_31_0_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_7_src_slice_pitch_31_0_shift) };
}

/*define for DW_8 word*/
/*define for src_slice_pitch_47_32 field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_8_src_slice_pitch_47_32_offset: u32 = 8;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_8_src_slice_pitch_47_32_mask: u32 = 0x0000FFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_8_src_slice_pitch_47_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_8_SRC_SLICE_PITCH_47_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_8_src_slice_pitch_47_32_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_8_src_slice_pitch_47_32_shift) };
}

/*define for DST_ADDR_LO word*/
/*define for dst_addr_31_0 field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DST_ADDR_LO_dst_addr_31_0_offset: u32 = 9;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DST_ADDR_LO_dst_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DST_ADDR_LO_dst_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DST_ADDR_LO_DST_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DST_ADDR_LO_dst_addr_31_0_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DST_ADDR_LO_dst_addr_31_0_shift) };
}

/*define for DST_ADDR_HI word*/
/*define for dst_addr_63_32 field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DST_ADDR_HI_dst_addr_63_32_offset: u32 = 10;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DST_ADDR_HI_dst_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DST_ADDR_HI_dst_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DST_ADDR_HI_DST_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DST_ADDR_HI_dst_addr_63_32_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DST_ADDR_HI_dst_addr_63_32_shift) };
}

/*define for DW_11 word*/
/*define for dst_x field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_11_dst_x_offset: u32 = 11;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_11_dst_x_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_11_dst_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_11_DST_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_11_dst_$x_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_11_dst_$x_shift) };
}

/*define for DW_12 word*/
/*define for dst_y field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_12_dst_y_offset: u32 = 12;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_12_dst_y_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_12_dst_y_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_12_DST_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_12_dst_y_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_12_dst_y_shift) };
}

/*define for DW_13 word*/
/*define for dst_z field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_13_dst_z_offset: u32 = 13;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_13_dst_z_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_13_dst_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_13_DST_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_13_dst_z_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_13_dst_z_shift) };
}

/*define for DW_14 word*/
/*define for dst_pitch field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_14_dst_pitch_offset: u32 = 14;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_14_dst_pitch_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_14_dst_pitch_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_14_DST_PITCH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_14_dst_pitch_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_14_dst_pitch_shift) };
}

/*define for DW_15 word*/
/*define for dst_slice_pitch_31_0 field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_15_dst_slice_pitch_31_0_offset: u32 = 15;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_15_dst_slice_pitch_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_15_dst_slice_pitch_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_15_DST_SLICE_PITCH_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_15_dst_slice_pitch_31_0_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_15_dst_slice_pitch_31_0_shift) };
}

/*define for DW_16 word*/
/*define for dst_slice_pitch_47_32 field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_dst_slice_pitch_47_32_offset: u32 = 16;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_dst_slice_pitch_47_32_mask: u32 = 0x0000FFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_dst_slice_pitch_47_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_DST_SLICE_PITCH_47_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_dst_slice_pitch_47_32_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_dst_slice_pitch_47_32_shift) };
}

/*define for dst_sw field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_dst_sw_offset: u32 = 16;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_dst_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_dst_sw_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_DST_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_dst_sw_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_dst_sw_shift) };
}

/*define for dst_policy field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_dst_policy_offset: u32 = 16;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_dst_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_dst_policy_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_DST_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_dst_policy_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_dst_policy_shift) };
}

/*define for src_sw field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_src_sw_offset: u32 = 16;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_src_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_src_sw_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_SRC_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_src_sw_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_src_sw_shift) };
}

/*define for src_policy field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_src_policy_offset: u32 = 16;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_src_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_src_policy_shift: u32 = 26;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_SRC_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_src_policy_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_16_src_policy_shift) };
}

/*define for DW_17 word*/
/*define for rect_x field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_17_rect_x_offset: u32 = 17;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_17_rect_x_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_17_rect_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_17_RECT_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_17_rect_$x_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_17_rect_$x_shift) };
}

/*define for DW_18 word*/
/*define for rect_y field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_18_rect_y_offset: u32 = 18;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_18_rect_y_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_18_rect_y_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_18_RECT_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_18_rect_y_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_18_rect_y_shift) };
}

/*define for DW_19 word*/
/*define for rect_z field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_19_rect_z_offset: u32 = 19;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_19_rect_z_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_19_rect_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_19_RECT_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_19_rect_z_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_LARGE_DW_19_rect_z_shift) };
}


/*
** Definitions for SDMA_PKT_COPY_LINEAR_SUBWIN_BC packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_HEADER_op_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_HEADER_sub_op_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_HEADER_sub_op_shift) };
}

/*define for elementsize field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_HEADER_elementsize_offset: u32 = 0;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_HEADER_elementsize_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_HEADER_elementsize_shift: u32 = 29;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_HEADER_ELEMENTSIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_HEADER_elementsize_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_HEADER_elementsize_shift) };
}

/*define for SRC_ADDR_LO word*/
/*define for src_addr_31_0 field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_SRC_ADDR_LO_src_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_SRC_ADDR_LO_src_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_SRC_ADDR_LO_src_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_SRC_ADDR_LO_SRC_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_SRC_ADDR_LO_src_addr_31_0_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_SRC_ADDR_LO_src_addr_31_0_shift) };
}

/*define for SRC_ADDR_HI word*/
/*define for src_addr_63_32 field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_SRC_ADDR_HI_src_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_SRC_ADDR_HI_src_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_SRC_ADDR_HI_src_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_SRC_ADDR_HI_SRC_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_SRC_ADDR_HI_src_addr_63_32_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_SRC_ADDR_HI_src_addr_63_32_shift) };
}

/*define for DW_3 word*/
/*define for src_x field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_3_src_x_offset: u32 = 3;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_3_src_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_3_src_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_3_SRC_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_3_src_$x_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_3_src_$x_shift) };
}

/*define for src_y field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_3_src_y_offset: u32 = 3;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_3_src_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_3_src_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_3_SRC_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_3_src_y_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_3_src_y_shift) };
}

/*define for DW_4 word*/
/*define for src_z field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_4_src_z_offset: u32 = 4;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_4_src_z_mask: u32 = 0x000007FF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_4_src_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_4_SRC_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_4_src_z_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_4_src_z_shift) };
}

/*define for src_pitch field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_4_src_pitch_offset: u32 = 4;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_4_src_pitch_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_4_src_pitch_shift: u32 = 13;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_4_SRC_PITCH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_4_src_pitch_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_4_src_pitch_shift) };
}

/*define for DW_5 word*/
/*define for src_slice_pitch field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_5_src_slice_pitch_offset: u32 = 5;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_5_src_slice_pitch_mask: u32 = 0x0FFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_5_src_slice_pitch_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_5_SRC_SLICE_PITCH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_5_src_slice_pitch_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_5_src_slice_pitch_shift) };
}

/*define for DST_ADDR_LO word*/
/*define for dst_addr_31_0 field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DST_ADDR_LO_dst_addr_31_0_offset: u32 = 6;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DST_ADDR_LO_dst_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DST_ADDR_LO_dst_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DST_ADDR_LO_DST_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DST_ADDR_LO_dst_addr_31_0_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DST_ADDR_LO_dst_addr_31_0_shift) };
}

/*define for DST_ADDR_HI word*/
/*define for dst_addr_63_32 field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DST_ADDR_HI_dst_addr_63_32_offset: u32 = 7;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DST_ADDR_HI_dst_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DST_ADDR_HI_dst_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DST_ADDR_HI_DST_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DST_ADDR_HI_dst_addr_63_32_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DST_ADDR_HI_dst_addr_63_32_shift) };
}

/*define for DW_8 word*/
/*define for dst_x field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_8_dst_x_offset: u32 = 8;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_8_dst_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_8_dst_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_8_DST_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_8_dst_$x_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_8_dst_$x_shift) };
}

/*define for dst_y field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_8_dst_y_offset: u32 = 8;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_8_dst_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_8_dst_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_8_DST_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_8_dst_y_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_8_dst_y_shift) };
}

/*define for DW_9 word*/
/*define for dst_z field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_9_dst_z_offset: u32 = 9;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_9_dst_z_mask: u32 = 0x000007FF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_9_dst_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_9_DST_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_9_dst_z_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_9_dst_z_shift) };
}

/*define for dst_pitch field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_9_dst_pitch_offset: u32 = 9;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_9_dst_pitch_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_9_dst_pitch_shift: u32 = 13;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_9_DST_PITCH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_9_dst_pitch_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_9_dst_pitch_shift) };
}

/*define for DW_10 word*/
/*define for dst_slice_pitch field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_10_dst_slice_pitch_offset: u32 = 10;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_10_dst_slice_pitch_mask: u32 = 0x0FFFFFFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_10_dst_slice_pitch_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_10_DST_SLICE_PITCH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_10_dst_slice_pitch_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_10_dst_slice_pitch_shift) };
}

/*define for DW_11 word*/
/*define for rect_x field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_11_rect_x_offset: u32 = 11;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_11_rect_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_11_rect_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_11_RECT_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_11_rect_$x_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_11_rect_$x_shift) };
}

/*define for rect_y field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_11_rect_y_offset: u32 = 11;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_11_rect_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_11_rect_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_11_RECT_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_11_rect_y_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_11_rect_y_shift) };
}

/*define for DW_12 word*/
/*define for rect_z field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_rect_z_offset: u32 = 12;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_rect_z_mask: u32 = 0x000007FF;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_rect_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_RECT_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_rect_z_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_rect_z_shift) };
}

/*define for dst_sw field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_dst_sw_offset: u32 = 12;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_dst_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_dst_sw_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_DST_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_dst_sw_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_dst_sw_shift) };
}

/*define for dst_ha field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_dst_ha_offset: u32 = 12;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_dst_ha_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_dst_ha_shift: u32 = 19;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_DST_HA {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_dst_ha_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_dst_ha_shift) };
}

/*define for src_sw field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_src_sw_offset: u32 = 12;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_src_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_src_sw_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_SRC_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_src_sw_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_src_sw_shift) };
}

/*define for src_ha field*/
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_src_ha_offset: u32 = 12;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_src_ha_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_src_ha_shift: u32 = 27;
macro_rules! SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_SRC_HA {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_src_ha_mask) << SDMA_PKT_COPY_LINEAR_SUBWIN_BC_DW_12_src_ha_shift) };
}


/*
** Definitions for SDMA_PKT_COPY_TILED packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_COPY_TILED_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_TILED_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_TILED_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_HEADER_op_mask) << SDMA_PKT_COPY_TILED_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_COPY_TILED_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_TILED_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_TILED_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_TILED_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_HEADER_sub_op_mask) << SDMA_PKT_COPY_TILED_HEADER_sub_op_shift) };
}

/*define for encrypt field*/
pub const SDMA_PKT_COPY_TILED_HEADER_encrypt_offset: u32 = 0;
pub const SDMA_PKT_COPY_TILED_HEADER_encrypt_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_TILED_HEADER_encrypt_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_HEADER_ENCRYPT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_HEADER_encrypt_mask) << SDMA_PKT_COPY_TILED_HEADER_encrypt_shift) };
}

/*define for tmz field*/
pub const SDMA_PKT_COPY_TILED_HEADER_tmz_offset: u32 = 0;
pub const SDMA_PKT_COPY_TILED_HEADER_tmz_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_TILED_HEADER_tmz_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_TILED_HEADER_TMZ {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_HEADER_tmz_mask) << SDMA_PKT_COPY_TILED_HEADER_tmz_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_COPY_TILED_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_COPY_TILED_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_TILED_HEADER_cpv_shift: u32 = 19;
macro_rules! SDMA_PKT_COPY_TILED_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_HEADER_cpv_mask) << SDMA_PKT_COPY_TILED_HEADER_cpv_shift) };
}

/*define for detile field*/
pub const SDMA_PKT_COPY_TILED_HEADER_detile_offset: u32 = 0;
pub const SDMA_PKT_COPY_TILED_HEADER_detile_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_TILED_HEADER_detile_shift: u32 = 31;
macro_rules! SDMA_PKT_COPY_TILED_HEADER_DETILE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_HEADER_detile_mask) << SDMA_PKT_COPY_TILED_HEADER_detile_shift) };
}

/*define for TILED_ADDR_LO word*/
/*define for tiled_addr_31_0 field*/
pub const SDMA_PKT_COPY_TILED_TILED_ADDR_LO_tiled_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_COPY_TILED_TILED_ADDR_LO_tiled_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_TILED_ADDR_LO_tiled_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_TILED_ADDR_LO_TILED_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_TILED_ADDR_LO_tiled_addr_31_0_mask) << SDMA_PKT_COPY_TILED_TILED_ADDR_LO_tiled_addr_31_0_shift) };
}

/*define for TILED_ADDR_HI word*/
/*define for tiled_addr_63_32 field*/
pub const SDMA_PKT_COPY_TILED_TILED_ADDR_HI_tiled_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_COPY_TILED_TILED_ADDR_HI_tiled_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_TILED_ADDR_HI_tiled_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_TILED_ADDR_HI_TILED_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_TILED_ADDR_HI_tiled_addr_63_32_mask) << SDMA_PKT_COPY_TILED_TILED_ADDR_HI_tiled_addr_63_32_shift) };
}

/*define for DW_3 word*/
/*define for width field*/
pub const SDMA_PKT_COPY_TILED_DW_3_width_offset: u32 = 3;
pub const SDMA_PKT_COPY_TILED_DW_3_width_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_DW_3_width_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_DW_3_WIDTH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_DW_3_width_mask) << SDMA_PKT_COPY_TILED_DW_3_width_shift) };
}

/*define for DW_4 word*/
/*define for height field*/
pub const SDMA_PKT_COPY_TILED_DW_4_height_offset: u32 = 4;
pub const SDMA_PKT_COPY_TILED_DW_4_height_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_DW_4_height_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_DW_4_HEIGHT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_DW_4_height_mask) << SDMA_PKT_COPY_TILED_DW_4_height_shift) };
}

/*define for depth field*/
pub const SDMA_PKT_COPY_TILED_DW_4_depth_offset: u32 = 4;
pub const SDMA_PKT_COPY_TILED_DW_4_depth_mask: u32 = 0x00001FFF;
pub const SDMA_PKT_COPY_TILED_DW_4_depth_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_DW_4_DEPTH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_DW_4_depth_mask) << SDMA_PKT_COPY_TILED_DW_4_depth_shift) };
}

/*define for DW_5 word*/
/*define for element_size field*/
pub const SDMA_PKT_COPY_TILED_DW_5_element_size_offset: u32 = 5;
pub const SDMA_PKT_COPY_TILED_DW_5_element_size_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_TILED_DW_5_element_size_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_DW_5_ELEMENT_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_DW_5_element_size_mask) << SDMA_PKT_COPY_TILED_DW_5_element_size_shift) };
}

/*define for swizzle_mode field*/
pub const SDMA_PKT_COPY_TILED_DW_5_swizzle_mode_offset: u32 = 5;
pub const SDMA_PKT_COPY_TILED_DW_5_swizzle_mode_mask: u32 = 0x0000001F;
pub const SDMA_PKT_COPY_TILED_DW_5_swizzle_mode_shift: u32 = 3;
macro_rules! SDMA_PKT_COPY_TILED_DW_5_SWIZZLE_MODE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_DW_5_swizzle_mode_mask) << SDMA_PKT_COPY_TILED_DW_5_swizzle_mode_shift) };
}

/*define for dimension field*/
pub const SDMA_PKT_COPY_TILED_DW_5_dimension_offset: u32 = 5;
pub const SDMA_PKT_COPY_TILED_DW_5_dimension_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_DW_5_dimension_shift: u32 = 9;
macro_rules! SDMA_PKT_COPY_TILED_DW_5_DIMENSION {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_DW_5_dimension_mask) << SDMA_PKT_COPY_TILED_DW_5_dimension_shift) };
}

/*define for mip_max field*/
pub const SDMA_PKT_COPY_TILED_DW_5_mip_max_offset: u32 = 5;
pub const SDMA_PKT_COPY_TILED_DW_5_mip_max_mask: u32 = 0x0000000F;
pub const SDMA_PKT_COPY_TILED_DW_5_mip_max_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_DW_5_MIP_MAX {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_DW_5_mip_ma$x_mask) << SDMA_PKT_COPY_TILED_DW_5_mip_ma$x_shift) };
}

/*define for DW_6 word*/
/*define for x field*/
pub const SDMA_PKT_COPY_TILED_DW_6_x_offset: u32 = 6;
pub const SDMA_PKT_COPY_TILED_DW_6_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_DW_6_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_DW_6_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_DW_6_$x_mask) << SDMA_PKT_COPY_TILED_DW_6_$x_shift) };
}

/*define for y field*/
pub const SDMA_PKT_COPY_TILED_DW_6_y_offset: u32 = 6;
pub const SDMA_PKT_COPY_TILED_DW_6_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_DW_6_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_DW_6_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_DW_6_y_mask) << SDMA_PKT_COPY_TILED_DW_6_y_shift) };
}

/*define for DW_7 word*/
/*define for z field*/
pub const SDMA_PKT_COPY_TILED_DW_7_z_offset: u32 = 7;
pub const SDMA_PKT_COPY_TILED_DW_7_z_mask: u32 = 0x00001FFF;
pub const SDMA_PKT_COPY_TILED_DW_7_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_DW_7_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_DW_7_z_mask) << SDMA_PKT_COPY_TILED_DW_7_z_shift) };
}

/*define for linear_sw field*/
pub const SDMA_PKT_COPY_TILED_DW_7_linear_sw_offset: u32 = 7;
pub const SDMA_PKT_COPY_TILED_DW_7_linear_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_DW_7_linear_sw_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_DW_7_LINEAR_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_DW_7_linear_sw_mask) << SDMA_PKT_COPY_TILED_DW_7_linear_sw_shift) };
}

/*define for linear_cache_policy field*/
pub const SDMA_PKT_COPY_TILED_DW_7_linear_cache_policy_offset: u32 = 7;
pub const SDMA_PKT_COPY_TILED_DW_7_linear_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_TILED_DW_7_linear_cache_policy_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_TILED_DW_7_LINEAR_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_DW_7_linear_cache_policy_mask) << SDMA_PKT_COPY_TILED_DW_7_linear_cache_policy_shift) };
}

/*define for tile_sw field*/
pub const SDMA_PKT_COPY_TILED_DW_7_tile_sw_offset: u32 = 7;
pub const SDMA_PKT_COPY_TILED_DW_7_tile_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_DW_7_tile_sw_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_TILED_DW_7_TILE_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_DW_7_tile_sw_mask) << SDMA_PKT_COPY_TILED_DW_7_tile_sw_shift) };
}

/*define for tile_cache_policy field*/
pub const SDMA_PKT_COPY_TILED_DW_7_tile_cache_policy_offset: u32 = 7;
pub const SDMA_PKT_COPY_TILED_DW_7_tile_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_TILED_DW_7_tile_cache_policy_shift: u32 = 26;
macro_rules! SDMA_PKT_COPY_TILED_DW_7_TILE_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_DW_7_tile_cache_policy_mask) << SDMA_PKT_COPY_TILED_DW_7_tile_cache_policy_shift) };
}

/*define for LINEAR_ADDR_LO word*/
/*define for linear_addr_31_0 field*/
pub const SDMA_PKT_COPY_TILED_LINEAR_ADDR_LO_linear_addr_31_0_offset: u32 = 8;
pub const SDMA_PKT_COPY_TILED_LINEAR_ADDR_LO_linear_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_LINEAR_ADDR_LO_linear_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_LINEAR_ADDR_LO_LINEAR_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_LINEAR_ADDR_LO_linear_addr_31_0_mask) << SDMA_PKT_COPY_TILED_LINEAR_ADDR_LO_linear_addr_31_0_shift) };
}

/*define for LINEAR_ADDR_HI word*/
/*define for linear_addr_63_32 field*/
pub const SDMA_PKT_COPY_TILED_LINEAR_ADDR_HI_linear_addr_63_32_offset: u32 = 9;
pub const SDMA_PKT_COPY_TILED_LINEAR_ADDR_HI_linear_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_LINEAR_ADDR_HI_linear_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_LINEAR_ADDR_HI_LINEAR_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_LINEAR_ADDR_HI_linear_addr_63_32_mask) << SDMA_PKT_COPY_TILED_LINEAR_ADDR_HI_linear_addr_63_32_shift) };
}

/*define for LINEAR_PITCH word*/
/*define for linear_pitch field*/
pub const SDMA_PKT_COPY_TILED_LINEAR_PITCH_linear_pitch_offset: u32 = 10;
pub const SDMA_PKT_COPY_TILED_LINEAR_PITCH_linear_pitch_mask: u32 = 0x0007FFFF;
pub const SDMA_PKT_COPY_TILED_LINEAR_PITCH_linear_pitch_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_LINEAR_PITCH_LINEAR_PITCH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_LINEAR_PITCH_linear_pitch_mask) << SDMA_PKT_COPY_TILED_LINEAR_PITCH_linear_pitch_shift) };
}

/*define for LINEAR_SLICE_PITCH word*/
/*define for linear_slice_pitch field*/
pub const SDMA_PKT_COPY_TILED_LINEAR_SLICE_PITCH_linear_slice_pitch_offset: u32 = 11;
pub const SDMA_PKT_COPY_TILED_LINEAR_SLICE_PITCH_linear_slice_pitch_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_LINEAR_SLICE_PITCH_linear_slice_pitch_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_LINEAR_SLICE_PITCH_LINEAR_SLICE_PITCH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_LINEAR_SLICE_PITCH_linear_slice_pitch_mask) << SDMA_PKT_COPY_TILED_LINEAR_SLICE_PITCH_linear_slice_pitch_shift) };
}

/*define for COUNT word*/
/*define for count field*/
pub const SDMA_PKT_COPY_TILED_COUNT_count_offset: u32 = 12;
pub const SDMA_PKT_COPY_TILED_COUNT_count_mask: u32 = 0x3FFFFFFF;
pub const SDMA_PKT_COPY_TILED_COUNT_count_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_COUNT_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_COUNT_count_mask) << SDMA_PKT_COPY_TILED_COUNT_count_shift) };
}


/*
** Definitions for SDMA_PKT_COPY_TILED_BC packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_COPY_TILED_BC_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_TILED_BC_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_TILED_BC_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_BC_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_HEADER_op_mask) << SDMA_PKT_COPY_TILED_BC_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_COPY_TILED_BC_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_TILED_BC_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_TILED_BC_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_TILED_BC_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_HEADER_sub_op_mask) << SDMA_PKT_COPY_TILED_BC_HEADER_sub_op_shift) };
}

/*define for detile field*/
pub const SDMA_PKT_COPY_TILED_BC_HEADER_detile_offset: u32 = 0;
pub const SDMA_PKT_COPY_TILED_BC_HEADER_detile_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_TILED_BC_HEADER_detile_shift: u32 = 31;
macro_rules! SDMA_PKT_COPY_TILED_BC_HEADER_DETILE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_HEADER_detile_mask) << SDMA_PKT_COPY_TILED_BC_HEADER_detile_shift) };
}

/*define for TILED_ADDR_LO word*/
/*define for tiled_addr_31_0 field*/
pub const SDMA_PKT_COPY_TILED_BC_TILED_ADDR_LO_tiled_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_COPY_TILED_BC_TILED_ADDR_LO_tiled_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_BC_TILED_ADDR_LO_tiled_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_BC_TILED_ADDR_LO_TILED_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_TILED_ADDR_LO_tiled_addr_31_0_mask) << SDMA_PKT_COPY_TILED_BC_TILED_ADDR_LO_tiled_addr_31_0_shift) };
}

/*define for TILED_ADDR_HI word*/
/*define for tiled_addr_63_32 field*/
pub const SDMA_PKT_COPY_TILED_BC_TILED_ADDR_HI_tiled_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_COPY_TILED_BC_TILED_ADDR_HI_tiled_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_BC_TILED_ADDR_HI_tiled_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_BC_TILED_ADDR_HI_TILED_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_TILED_ADDR_HI_tiled_addr_63_32_mask) << SDMA_PKT_COPY_TILED_BC_TILED_ADDR_HI_tiled_addr_63_32_shift) };
}

/*define for DW_3 word*/
/*define for width field*/
pub const SDMA_PKT_COPY_TILED_BC_DW_3_width_offset: u32 = 3;
pub const SDMA_PKT_COPY_TILED_BC_DW_3_width_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_BC_DW_3_width_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_BC_DW_3_WIDTH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_DW_3_width_mask) << SDMA_PKT_COPY_TILED_BC_DW_3_width_shift) };
}

/*define for DW_4 word*/
/*define for height field*/
pub const SDMA_PKT_COPY_TILED_BC_DW_4_height_offset: u32 = 4;
pub const SDMA_PKT_COPY_TILED_BC_DW_4_height_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_BC_DW_4_height_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_BC_DW_4_HEIGHT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_DW_4_height_mask) << SDMA_PKT_COPY_TILED_BC_DW_4_height_shift) };
}

/*define for depth field*/
pub const SDMA_PKT_COPY_TILED_BC_DW_4_depth_offset: u32 = 4;
pub const SDMA_PKT_COPY_TILED_BC_DW_4_depth_mask: u32 = 0x000007FF;
pub const SDMA_PKT_COPY_TILED_BC_DW_4_depth_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_BC_DW_4_DEPTH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_DW_4_depth_mask) << SDMA_PKT_COPY_TILED_BC_DW_4_depth_shift) };
}

/*define for DW_5 word*/
/*define for element_size field*/
pub const SDMA_PKT_COPY_TILED_BC_DW_5_element_size_offset: u32 = 5;
pub const SDMA_PKT_COPY_TILED_BC_DW_5_element_size_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_TILED_BC_DW_5_element_size_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_BC_DW_5_ELEMENT_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_DW_5_element_size_mask) << SDMA_PKT_COPY_TILED_BC_DW_5_element_size_shift) };
}

/*define for array_mode field*/
pub const SDMA_PKT_COPY_TILED_BC_DW_5_array_mode_offset: u32 = 5;
pub const SDMA_PKT_COPY_TILED_BC_DW_5_array_mode_mask: u32 = 0x0000000F;
pub const SDMA_PKT_COPY_TILED_BC_DW_5_array_mode_shift: u32 = 3;
macro_rules! SDMA_PKT_COPY_TILED_BC_DW_5_ARRAY_MODE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_DW_5_array_mode_mask) << SDMA_PKT_COPY_TILED_BC_DW_5_array_mode_shift) };
}

/*define for mit_mode field*/
pub const SDMA_PKT_COPY_TILED_BC_DW_5_mit_mode_offset: u32 = 5;
pub const SDMA_PKT_COPY_TILED_BC_DW_5_mit_mode_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_TILED_BC_DW_5_mit_mode_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_TILED_BC_DW_5_MIT_MODE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_DW_5_mit_mode_mask) << SDMA_PKT_COPY_TILED_BC_DW_5_mit_mode_shift) };
}

/*define for tilesplit_size field*/
pub const SDMA_PKT_COPY_TILED_BC_DW_5_tilesplit_size_offset: u32 = 5;
pub const SDMA_PKT_COPY_TILED_BC_DW_5_tilesplit_size_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_TILED_BC_DW_5_tilesplit_size_shift: u32 = 11;
macro_rules! SDMA_PKT_COPY_TILED_BC_DW_5_TILESPLIT_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_DW_5_tilesplit_size_mask) << SDMA_PKT_COPY_TILED_BC_DW_5_tilesplit_size_shift) };
}

/*define for bank_w field*/
pub const SDMA_PKT_COPY_TILED_BC_DW_5_bank_w_offset: u32 = 5;
pub const SDMA_PKT_COPY_TILED_BC_DW_5_bank_w_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_BC_DW_5_bank_w_shift: u32 = 15;
macro_rules! SDMA_PKT_COPY_TILED_BC_DW_5_BANK_W {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_DW_5_bank_w_mask) << SDMA_PKT_COPY_TILED_BC_DW_5_bank_w_shift) };
}

/*define for bank_h field*/
pub const SDMA_PKT_COPY_TILED_BC_DW_5_bank_h_offset: u32 = 5;
pub const SDMA_PKT_COPY_TILED_BC_DW_5_bank_h_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_BC_DW_5_bank_h_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_TILED_BC_DW_5_BANK_H {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_DW_5_bank_h_mask) << SDMA_PKT_COPY_TILED_BC_DW_5_bank_h_shift) };
}

/*define for num_bank field*/
pub const SDMA_PKT_COPY_TILED_BC_DW_5_num_bank_offset: u32 = 5;
pub const SDMA_PKT_COPY_TILED_BC_DW_5_num_bank_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_BC_DW_5_num_bank_shift: u32 = 21;
macro_rules! SDMA_PKT_COPY_TILED_BC_DW_5_NUM_BANK {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_DW_5_num_bank_mask) << SDMA_PKT_COPY_TILED_BC_DW_5_num_bank_shift) };
}

/*define for mat_aspt field*/
pub const SDMA_PKT_COPY_TILED_BC_DW_5_mat_aspt_offset: u32 = 5;
pub const SDMA_PKT_COPY_TILED_BC_DW_5_mat_aspt_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_BC_DW_5_mat_aspt_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_TILED_BC_DW_5_MAT_ASPT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_DW_5_mat_aspt_mask) << SDMA_PKT_COPY_TILED_BC_DW_5_mat_aspt_shift) };
}

/*define for pipe_config field*/
pub const SDMA_PKT_COPY_TILED_BC_DW_5_pipe_config_offset: u32 = 5;
pub const SDMA_PKT_COPY_TILED_BC_DW_5_pipe_config_mask: u32 = 0x0000001F;
pub const SDMA_PKT_COPY_TILED_BC_DW_5_pipe_config_shift: u32 = 26;
macro_rules! SDMA_PKT_COPY_TILED_BC_DW_5_PIPE_CONFIG {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_DW_5_pipe_config_mask) << SDMA_PKT_COPY_TILED_BC_DW_5_pipe_config_shift) };
}

/*define for DW_6 word*/
/*define for x field*/
pub const SDMA_PKT_COPY_TILED_BC_DW_6_x_offset: u32 = 6;
pub const SDMA_PKT_COPY_TILED_BC_DW_6_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_BC_DW_6_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_BC_DW_6_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_DW_6_$x_mask) << SDMA_PKT_COPY_TILED_BC_DW_6_$x_shift) };
}

/*define for y field*/
pub const SDMA_PKT_COPY_TILED_BC_DW_6_y_offset: u32 = 6;
pub const SDMA_PKT_COPY_TILED_BC_DW_6_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_BC_DW_6_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_BC_DW_6_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_DW_6_y_mask) << SDMA_PKT_COPY_TILED_BC_DW_6_y_shift) };
}

/*define for DW_7 word*/
/*define for z field*/
pub const SDMA_PKT_COPY_TILED_BC_DW_7_z_offset: u32 = 7;
pub const SDMA_PKT_COPY_TILED_BC_DW_7_z_mask: u32 = 0x000007FF;
pub const SDMA_PKT_COPY_TILED_BC_DW_7_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_BC_DW_7_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_DW_7_z_mask) << SDMA_PKT_COPY_TILED_BC_DW_7_z_shift) };
}

/*define for linear_sw field*/
pub const SDMA_PKT_COPY_TILED_BC_DW_7_linear_sw_offset: u32 = 7;
pub const SDMA_PKT_COPY_TILED_BC_DW_7_linear_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_BC_DW_7_linear_sw_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_BC_DW_7_LINEAR_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_DW_7_linear_sw_mask) << SDMA_PKT_COPY_TILED_BC_DW_7_linear_sw_shift) };
}

/*define for tile_sw field*/
pub const SDMA_PKT_COPY_TILED_BC_DW_7_tile_sw_offset: u32 = 7;
pub const SDMA_PKT_COPY_TILED_BC_DW_7_tile_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_BC_DW_7_tile_sw_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_TILED_BC_DW_7_TILE_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_DW_7_tile_sw_mask) << SDMA_PKT_COPY_TILED_BC_DW_7_tile_sw_shift) };
}

/*define for LINEAR_ADDR_LO word*/
/*define for linear_addr_31_0 field*/
pub const SDMA_PKT_COPY_TILED_BC_LINEAR_ADDR_LO_linear_addr_31_0_offset: u32 = 8;
pub const SDMA_PKT_COPY_TILED_BC_LINEAR_ADDR_LO_linear_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_BC_LINEAR_ADDR_LO_linear_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_BC_LINEAR_ADDR_LO_LINEAR_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_LINEAR_ADDR_LO_linear_addr_31_0_mask) << SDMA_PKT_COPY_TILED_BC_LINEAR_ADDR_LO_linear_addr_31_0_shift) };
}

/*define for LINEAR_ADDR_HI word*/
/*define for linear_addr_63_32 field*/
pub const SDMA_PKT_COPY_TILED_BC_LINEAR_ADDR_HI_linear_addr_63_32_offset: u32 = 9;
pub const SDMA_PKT_COPY_TILED_BC_LINEAR_ADDR_HI_linear_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_BC_LINEAR_ADDR_HI_linear_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_BC_LINEAR_ADDR_HI_LINEAR_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_LINEAR_ADDR_HI_linear_addr_63_32_mask) << SDMA_PKT_COPY_TILED_BC_LINEAR_ADDR_HI_linear_addr_63_32_shift) };
}

/*define for LINEAR_PITCH word*/
/*define for linear_pitch field*/
pub const SDMA_PKT_COPY_TILED_BC_LINEAR_PITCH_linear_pitch_offset: u32 = 10;
pub const SDMA_PKT_COPY_TILED_BC_LINEAR_PITCH_linear_pitch_mask: u32 = 0x0007FFFF;
pub const SDMA_PKT_COPY_TILED_BC_LINEAR_PITCH_linear_pitch_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_BC_LINEAR_PITCH_LINEAR_PITCH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_LINEAR_PITCH_linear_pitch_mask) << SDMA_PKT_COPY_TILED_BC_LINEAR_PITCH_linear_pitch_shift) };
}

/*define for LINEAR_SLICE_PITCH word*/
/*define for linear_slice_pitch field*/
pub const SDMA_PKT_COPY_TILED_BC_LINEAR_SLICE_PITCH_linear_slice_pitch_offset: u32 = 11;
pub const SDMA_PKT_COPY_TILED_BC_LINEAR_SLICE_PITCH_linear_slice_pitch_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_BC_LINEAR_SLICE_PITCH_linear_slice_pitch_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_BC_LINEAR_SLICE_PITCH_LINEAR_SLICE_PITCH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_LINEAR_SLICE_PITCH_linear_slice_pitch_mask) << SDMA_PKT_COPY_TILED_BC_LINEAR_SLICE_PITCH_linear_slice_pitch_shift) };
}

/*define for COUNT word*/
/*define for count field*/
pub const SDMA_PKT_COPY_TILED_BC_COUNT_count_offset: u32 = 12;
pub const SDMA_PKT_COPY_TILED_BC_COUNT_count_mask: u32 = 0x000FFFFF;
pub const SDMA_PKT_COPY_TILED_BC_COUNT_count_shift: u32 = 2;
macro_rules! SDMA_PKT_COPY_TILED_BC_COUNT_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_BC_COUNT_count_mask) << SDMA_PKT_COPY_TILED_BC_COUNT_count_shift) };
}


/*
** Definitions for SDMA_PKT_COPY_L2T_BROADCAST packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_HEADER_op_mask) << SDMA_PKT_COPY_L2T_BROADCAST_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_HEADER_sub_op_mask) << SDMA_PKT_COPY_L2T_BROADCAST_HEADER_sub_op_shift) };
}

/*define for encrypt field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_encrypt_offset: u32 = 0;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_encrypt_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_encrypt_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_HEADER_ENCRYPT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_HEADER_encrypt_mask) << SDMA_PKT_COPY_L2T_BROADCAST_HEADER_encrypt_shift) };
}

/*define for tmz field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_tmz_offset: u32 = 0;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_tmz_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_tmz_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_HEADER_TMZ {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_HEADER_tmz_mask) << SDMA_PKT_COPY_L2T_BROADCAST_HEADER_tmz_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_cpv_shift: u32 = 19;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_HEADER_cpv_mask) << SDMA_PKT_COPY_L2T_BROADCAST_HEADER_cpv_shift) };
}

/*define for videocopy field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_videocopy_offset: u32 = 0;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_videocopy_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_videocopy_shift: u32 = 26;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_HEADER_VIDEOCOPY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_HEADER_videocopy_mask) << SDMA_PKT_COPY_L2T_BROADCAST_HEADER_videocopy_shift) };
}

/*define for broadcast field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_broadcast_offset: u32 = 0;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_broadcast_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_L2T_BROADCAST_HEADER_broadcast_shift: u32 = 27;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_HEADER_BROADCAST {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_HEADER_broadcast_mask) << SDMA_PKT_COPY_L2T_BROADCAST_HEADER_broadcast_shift) };
}

/*define for TILED_ADDR_LO_0 word*/
/*define for tiled_addr0_31_0 field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_LO_0_tiled_addr0_31_0_offset: u32 = 1;
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_LO_0_tiled_addr0_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_LO_0_tiled_addr0_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_LO_0_TILED_ADDR0_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_LO_0_tiled_addr0_31_0_mask) << SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_LO_0_tiled_addr0_31_0_shift) };
}

/*define for TILED_ADDR_HI_0 word*/
/*define for tiled_addr0_63_32 field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_HI_0_tiled_addr0_63_32_offset: u32 = 2;
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_HI_0_tiled_addr0_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_HI_0_tiled_addr0_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_HI_0_TILED_ADDR0_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_HI_0_tiled_addr0_63_32_mask) << SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_HI_0_tiled_addr0_63_32_shift) };
}

/*define for TILED_ADDR_LO_1 word*/
/*define for tiled_addr1_31_0 field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_LO_1_tiled_addr1_31_0_offset: u32 = 3;
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_LO_1_tiled_addr1_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_LO_1_tiled_addr1_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_LO_1_TILED_ADDR1_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_LO_1_tiled_addr1_31_0_mask) << SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_LO_1_tiled_addr1_31_0_shift) };
}

/*define for TILED_ADDR_HI_1 word*/
/*define for tiled_addr1_63_32 field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_HI_1_tiled_addr1_63_32_offset: u32 = 4;
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_HI_1_tiled_addr1_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_HI_1_tiled_addr1_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_HI_1_TILED_ADDR1_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_HI_1_tiled_addr1_63_32_mask) << SDMA_PKT_COPY_L2T_BROADCAST_TILED_ADDR_HI_1_tiled_addr1_63_32_shift) };
}

/*define for DW_5 word*/
/*define for width field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_5_width_offset: u32 = 5;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_5_width_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_5_width_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_DW_5_WIDTH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_DW_5_width_mask) << SDMA_PKT_COPY_L2T_BROADCAST_DW_5_width_shift) };
}

/*define for DW_6 word*/
/*define for height field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_6_height_offset: u32 = 6;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_6_height_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_6_height_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_DW_6_HEIGHT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_DW_6_height_mask) << SDMA_PKT_COPY_L2T_BROADCAST_DW_6_height_shift) };
}

/*define for depth field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_6_depth_offset: u32 = 6;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_6_depth_mask: u32 = 0x00001FFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_6_depth_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_DW_6_DEPTH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_DW_6_depth_mask) << SDMA_PKT_COPY_L2T_BROADCAST_DW_6_depth_shift) };
}

/*define for DW_7 word*/
/*define for element_size field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_element_size_offset: u32 = 7;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_element_size_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_element_size_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_DW_7_ELEMENT_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_DW_7_element_size_mask) << SDMA_PKT_COPY_L2T_BROADCAST_DW_7_element_size_shift) };
}

/*define for swizzle_mode field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_swizzle_mode_offset: u32 = 7;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_swizzle_mode_mask: u32 = 0x0000001F;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_swizzle_mode_shift: u32 = 3;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_DW_7_SWIZZLE_MODE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_DW_7_swizzle_mode_mask) << SDMA_PKT_COPY_L2T_BROADCAST_DW_7_swizzle_mode_shift) };
}

/*define for dimension field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_dimension_offset: u32 = 7;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_dimension_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_dimension_shift: u32 = 9;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_DW_7_DIMENSION {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_DW_7_dimension_mask) << SDMA_PKT_COPY_L2T_BROADCAST_DW_7_dimension_shift) };
}

/*define for mip_max field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_mip_max_offset: u32 = 7;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_mip_max_mask: u32 = 0x0000000F;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_7_mip_max_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_DW_7_MIP_MAX {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_DW_7_mip_ma$x_mask) << SDMA_PKT_COPY_L2T_BROADCAST_DW_7_mip_ma$x_shift) };
}

/*define for DW_8 word*/
/*define for x field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_8_x_offset: u32 = 8;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_8_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_8_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_DW_8_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_DW_8_$x_mask) << SDMA_PKT_COPY_L2T_BROADCAST_DW_8_$x_shift) };
}

/*define for y field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_8_y_offset: u32 = 8;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_8_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_8_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_DW_8_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_DW_8_y_mask) << SDMA_PKT_COPY_L2T_BROADCAST_DW_8_y_shift) };
}

/*define for DW_9 word*/
/*define for z field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_9_z_offset: u32 = 9;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_9_z_mask: u32 = 0x00001FFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_9_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_DW_9_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_DW_9_z_mask) << SDMA_PKT_COPY_L2T_BROADCAST_DW_9_z_shift) };
}

/*define for DW_10 word*/
/*define for dst2_sw field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_dst2_sw_offset: u32 = 10;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_dst2_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_dst2_sw_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_DW_10_DST2_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_DW_10_dst2_sw_mask) << SDMA_PKT_COPY_L2T_BROADCAST_DW_10_dst2_sw_shift) };
}

/*define for dst2_cache_policy field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_dst2_cache_policy_offset: u32 = 10;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_dst2_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_dst2_cache_policy_shift: u32 = 10;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_DW_10_DST2_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_DW_10_dst2_cache_policy_mask) << SDMA_PKT_COPY_L2T_BROADCAST_DW_10_dst2_cache_policy_shift) };
}

/*define for linear_sw field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_linear_sw_offset: u32 = 10;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_linear_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_linear_sw_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_DW_10_LINEAR_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_DW_10_linear_sw_mask) << SDMA_PKT_COPY_L2T_BROADCAST_DW_10_linear_sw_shift) };
}

/*define for linear_cache_policy field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_linear_cache_policy_offset: u32 = 10;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_linear_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_linear_cache_policy_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_DW_10_LINEAR_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_DW_10_linear_cache_policy_mask) << SDMA_PKT_COPY_L2T_BROADCAST_DW_10_linear_cache_policy_shift) };
}

/*define for tile_sw field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_tile_sw_offset: u32 = 10;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_tile_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_tile_sw_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_DW_10_TILE_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_DW_10_tile_sw_mask) << SDMA_PKT_COPY_L2T_BROADCAST_DW_10_tile_sw_shift) };
}

/*define for tile_cache_policy field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_tile_cache_policy_offset: u32 = 10;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_tile_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_L2T_BROADCAST_DW_10_tile_cache_policy_shift: u32 = 26;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_DW_10_TILE_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_DW_10_tile_cache_policy_mask) << SDMA_PKT_COPY_L2T_BROADCAST_DW_10_tile_cache_policy_shift) };
}

/*define for LINEAR_ADDR_LO word*/
/*define for linear_addr_31_0 field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_ADDR_LO_linear_addr_31_0_offset: u32 = 11;
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_ADDR_LO_linear_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_ADDR_LO_linear_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_ADDR_LO_LINEAR_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_ADDR_LO_linear_addr_31_0_mask) << SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_ADDR_LO_linear_addr_31_0_shift) };
}

/*define for LINEAR_ADDR_HI word*/
/*define for linear_addr_63_32 field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_ADDR_HI_linear_addr_63_32_offset: u32 = 12;
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_ADDR_HI_linear_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_ADDR_HI_linear_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_ADDR_HI_LINEAR_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_ADDR_HI_linear_addr_63_32_mask) << SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_ADDR_HI_linear_addr_63_32_shift) };
}

/*define for LINEAR_PITCH word*/
/*define for linear_pitch field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_PITCH_linear_pitch_offset: u32 = 13;
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_PITCH_linear_pitch_mask: u32 = 0x0007FFFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_PITCH_linear_pitch_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_PITCH_LINEAR_PITCH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_PITCH_linear_pitch_mask) << SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_PITCH_linear_pitch_shift) };
}

/*define for LINEAR_SLICE_PITCH word*/
/*define for linear_slice_pitch field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_SLICE_PITCH_linear_slice_pitch_offset: u32 = 14;
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_SLICE_PITCH_linear_slice_pitch_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_SLICE_PITCH_linear_slice_pitch_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_SLICE_PITCH_LINEAR_SLICE_PITCH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_SLICE_PITCH_linear_slice_pitch_mask) << SDMA_PKT_COPY_L2T_BROADCAST_LINEAR_SLICE_PITCH_linear_slice_pitch_shift) };
}

/*define for COUNT word*/
/*define for count field*/
pub const SDMA_PKT_COPY_L2T_BROADCAST_COUNT_count_offset: u32 = 15;
pub const SDMA_PKT_COPY_L2T_BROADCAST_COUNT_count_mask: u32 = 0x3FFFFFFF;
pub const SDMA_PKT_COPY_L2T_BROADCAST_COUNT_count_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_L2T_BROADCAST_COUNT_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_L2T_BROADCAST_COUNT_count_mask) << SDMA_PKT_COPY_L2T_BROADCAST_COUNT_count_shift) };
}


/*
** Definitions for SDMA_PKT_COPY_T2T packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_COPY_T2T_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_T2T_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_T2T_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_HEADER_op_mask) << SDMA_PKT_COPY_T2T_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_COPY_T2T_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_T2T_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_T2T_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_T2T_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_HEADER_sub_op_mask) << SDMA_PKT_COPY_T2T_HEADER_sub_op_shift) };
}

/*define for tmz field*/
pub const SDMA_PKT_COPY_T2T_HEADER_tmz_offset: u32 = 0;
pub const SDMA_PKT_COPY_T2T_HEADER_tmz_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_T2T_HEADER_tmz_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_T2T_HEADER_TMZ {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_HEADER_tmz_mask) << SDMA_PKT_COPY_T2T_HEADER_tmz_shift) };
}

/*define for dcc field*/
pub const SDMA_PKT_COPY_T2T_HEADER_dcc_offset: u32 = 0;
pub const SDMA_PKT_COPY_T2T_HEADER_dcc_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_T2T_HEADER_dcc_shift: u32 = 19;
macro_rules! SDMA_PKT_COPY_T2T_HEADER_DCC {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_HEADER_dcc_mask) << SDMA_PKT_COPY_T2T_HEADER_dcc_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_COPY_T2T_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_COPY_T2T_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_T2T_HEADER_cpv_shift: u32 = 28;
macro_rules! SDMA_PKT_COPY_T2T_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_HEADER_cpv_mask) << SDMA_PKT_COPY_T2T_HEADER_cpv_shift) };
}

/*define for dcc_dir field*/
pub const SDMA_PKT_COPY_T2T_HEADER_dcc_dir_offset: u32 = 0;
pub const SDMA_PKT_COPY_T2T_HEADER_dcc_dir_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_T2T_HEADER_dcc_dir_shift: u32 = 31;
macro_rules! SDMA_PKT_COPY_T2T_HEADER_DCC_DIR {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_HEADER_dcc_dir_mask) << SDMA_PKT_COPY_T2T_HEADER_dcc_dir_shift) };
}

/*define for SRC_ADDR_LO word*/
/*define for src_addr_31_0 field*/
pub const SDMA_PKT_COPY_T2T_SRC_ADDR_LO_src_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_COPY_T2T_SRC_ADDR_LO_src_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_T2T_SRC_ADDR_LO_src_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_SRC_ADDR_LO_SRC_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_SRC_ADDR_LO_src_addr_31_0_mask) << SDMA_PKT_COPY_T2T_SRC_ADDR_LO_src_addr_31_0_shift) };
}

/*define for SRC_ADDR_HI word*/
/*define for src_addr_63_32 field*/
pub const SDMA_PKT_COPY_T2T_SRC_ADDR_HI_src_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_COPY_T2T_SRC_ADDR_HI_src_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_T2T_SRC_ADDR_HI_src_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_SRC_ADDR_HI_SRC_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_SRC_ADDR_HI_src_addr_63_32_mask) << SDMA_PKT_COPY_T2T_SRC_ADDR_HI_src_addr_63_32_shift) };
}

/*define for DW_3 word*/
/*define for src_x field*/
pub const SDMA_PKT_COPY_T2T_DW_3_src_x_offset: u32 = 3;
pub const SDMA_PKT_COPY_T2T_DW_3_src_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_DW_3_src_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_DW_3_SRC_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_3_src_$x_mask) << SDMA_PKT_COPY_T2T_DW_3_src_$x_shift) };
}

/*define for src_y field*/
pub const SDMA_PKT_COPY_T2T_DW_3_src_y_offset: u32 = 3;
pub const SDMA_PKT_COPY_T2T_DW_3_src_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_DW_3_src_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_T2T_DW_3_SRC_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_3_src_y_mask) << SDMA_PKT_COPY_T2T_DW_3_src_y_shift) };
}

/*define for DW_4 word*/
/*define for src_z field*/
pub const SDMA_PKT_COPY_T2T_DW_4_src_z_offset: u32 = 4;
pub const SDMA_PKT_COPY_T2T_DW_4_src_z_mask: u32 = 0x00001FFF;
pub const SDMA_PKT_COPY_T2T_DW_4_src_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_DW_4_SRC_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_4_src_z_mask) << SDMA_PKT_COPY_T2T_DW_4_src_z_shift) };
}

/*define for src_width field*/
pub const SDMA_PKT_COPY_T2T_DW_4_src_width_offset: u32 = 4;
pub const SDMA_PKT_COPY_T2T_DW_4_src_width_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_DW_4_src_width_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_T2T_DW_4_SRC_WIDTH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_4_src_width_mask) << SDMA_PKT_COPY_T2T_DW_4_src_width_shift) };
}

/*define for DW_5 word*/
/*define for src_height field*/
pub const SDMA_PKT_COPY_T2T_DW_5_src_height_offset: u32 = 5;
pub const SDMA_PKT_COPY_T2T_DW_5_src_height_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_DW_5_src_height_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_DW_5_SRC_HEIGHT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_5_src_height_mask) << SDMA_PKT_COPY_T2T_DW_5_src_height_shift) };
}

/*define for src_depth field*/
pub const SDMA_PKT_COPY_T2T_DW_5_src_depth_offset: u32 = 5;
pub const SDMA_PKT_COPY_T2T_DW_5_src_depth_mask: u32 = 0x00001FFF;
pub const SDMA_PKT_COPY_T2T_DW_5_src_depth_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_T2T_DW_5_SRC_DEPTH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_5_src_depth_mask) << SDMA_PKT_COPY_T2T_DW_5_src_depth_shift) };
}

/*define for DW_6 word*/
/*define for src_element_size field*/
pub const SDMA_PKT_COPY_T2T_DW_6_src_element_size_offset: u32 = 6;
pub const SDMA_PKT_COPY_T2T_DW_6_src_element_size_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_T2T_DW_6_src_element_size_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_DW_6_SRC_ELEMENT_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_6_src_element_size_mask) << SDMA_PKT_COPY_T2T_DW_6_src_element_size_shift) };
}

/*define for src_swizzle_mode field*/
pub const SDMA_PKT_COPY_T2T_DW_6_src_swizzle_mode_offset: u32 = 6;
pub const SDMA_PKT_COPY_T2T_DW_6_src_swizzle_mode_mask: u32 = 0x0000001F;
pub const SDMA_PKT_COPY_T2T_DW_6_src_swizzle_mode_shift: u32 = 3;
macro_rules! SDMA_PKT_COPY_T2T_DW_6_SRC_SWIZZLE_MODE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_6_src_swizzle_mode_mask) << SDMA_PKT_COPY_T2T_DW_6_src_swizzle_mode_shift) };
}

/*define for src_dimension field*/
pub const SDMA_PKT_COPY_T2T_DW_6_src_dimension_offset: u32 = 6;
pub const SDMA_PKT_COPY_T2T_DW_6_src_dimension_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_T2T_DW_6_src_dimension_shift: u32 = 9;
macro_rules! SDMA_PKT_COPY_T2T_DW_6_SRC_DIMENSION {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_6_src_dimension_mask) << SDMA_PKT_COPY_T2T_DW_6_src_dimension_shift) };
}

/*define for src_mip_max field*/
pub const SDMA_PKT_COPY_T2T_DW_6_src_mip_max_offset: u32 = 6;
pub const SDMA_PKT_COPY_T2T_DW_6_src_mip_max_mask: u32 = 0x0000000F;
pub const SDMA_PKT_COPY_T2T_DW_6_src_mip_max_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_T2T_DW_6_SRC_MIP_MAX {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_6_src_mip_ma$x_mask) << SDMA_PKT_COPY_T2T_DW_6_src_mip_ma$x_shift) };
}

/*define for src_mip_id field*/
pub const SDMA_PKT_COPY_T2T_DW_6_src_mip_id_offset: u32 = 6;
pub const SDMA_PKT_COPY_T2T_DW_6_src_mip_id_mask: u32 = 0x0000000F;
pub const SDMA_PKT_COPY_T2T_DW_6_src_mip_id_shift: u32 = 20;
macro_rules! SDMA_PKT_COPY_T2T_DW_6_SRC_MIP_ID {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_6_src_mip_id_mask) << SDMA_PKT_COPY_T2T_DW_6_src_mip_id_shift) };
}

/*define for DST_ADDR_LO word*/
/*define for dst_addr_31_0 field*/
pub const SDMA_PKT_COPY_T2T_DST_ADDR_LO_dst_addr_31_0_offset: u32 = 7;
pub const SDMA_PKT_COPY_T2T_DST_ADDR_LO_dst_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_T2T_DST_ADDR_LO_dst_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_DST_ADDR_LO_DST_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DST_ADDR_LO_dst_addr_31_0_mask) << SDMA_PKT_COPY_T2T_DST_ADDR_LO_dst_addr_31_0_shift) };
}

/*define for DST_ADDR_HI word*/
/*define for dst_addr_63_32 field*/
pub const SDMA_PKT_COPY_T2T_DST_ADDR_HI_dst_addr_63_32_offset: u32 = 8;
pub const SDMA_PKT_COPY_T2T_DST_ADDR_HI_dst_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_T2T_DST_ADDR_HI_dst_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_DST_ADDR_HI_DST_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DST_ADDR_HI_dst_addr_63_32_mask) << SDMA_PKT_COPY_T2T_DST_ADDR_HI_dst_addr_63_32_shift) };
}

/*define for DW_9 word*/
/*define for dst_x field*/
pub const SDMA_PKT_COPY_T2T_DW_9_dst_x_offset: u32 = 9;
pub const SDMA_PKT_COPY_T2T_DW_9_dst_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_DW_9_dst_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_DW_9_DST_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_9_dst_$x_mask) << SDMA_PKT_COPY_T2T_DW_9_dst_$x_shift) };
}

/*define for dst_y field*/
pub const SDMA_PKT_COPY_T2T_DW_9_dst_y_offset: u32 = 9;
pub const SDMA_PKT_COPY_T2T_DW_9_dst_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_DW_9_dst_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_T2T_DW_9_DST_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_9_dst_y_mask) << SDMA_PKT_COPY_T2T_DW_9_dst_y_shift) };
}

/*define for DW_10 word*/
/*define for dst_z field*/
pub const SDMA_PKT_COPY_T2T_DW_10_dst_z_offset: u32 = 10;
pub const SDMA_PKT_COPY_T2T_DW_10_dst_z_mask: u32 = 0x00001FFF;
pub const SDMA_PKT_COPY_T2T_DW_10_dst_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_DW_10_DST_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_10_dst_z_mask) << SDMA_PKT_COPY_T2T_DW_10_dst_z_shift) };
}

/*define for dst_width field*/
pub const SDMA_PKT_COPY_T2T_DW_10_dst_width_offset: u32 = 10;
pub const SDMA_PKT_COPY_T2T_DW_10_dst_width_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_DW_10_dst_width_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_T2T_DW_10_DST_WIDTH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_10_dst_width_mask) << SDMA_PKT_COPY_T2T_DW_10_dst_width_shift) };
}

/*define for DW_11 word*/
/*define for dst_height field*/
pub const SDMA_PKT_COPY_T2T_DW_11_dst_height_offset: u32 = 11;
pub const SDMA_PKT_COPY_T2T_DW_11_dst_height_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_DW_11_dst_height_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_DW_11_DST_HEIGHT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_11_dst_height_mask) << SDMA_PKT_COPY_T2T_DW_11_dst_height_shift) };
}

/*define for dst_depth field*/
pub const SDMA_PKT_COPY_T2T_DW_11_dst_depth_offset: u32 = 11;
pub const SDMA_PKT_COPY_T2T_DW_11_dst_depth_mask: u32 = 0x00001FFF;
pub const SDMA_PKT_COPY_T2T_DW_11_dst_depth_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_T2T_DW_11_DST_DEPTH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_11_dst_depth_mask) << SDMA_PKT_COPY_T2T_DW_11_dst_depth_shift) };
}

/*define for DW_12 word*/
/*define for dst_element_size field*/
pub const SDMA_PKT_COPY_T2T_DW_12_dst_element_size_offset: u32 = 12;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_element_size_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_element_size_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_DW_12_DST_ELEMENT_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_12_dst_element_size_mask) << SDMA_PKT_COPY_T2T_DW_12_dst_element_size_shift) };
}

/*define for dst_swizzle_mode field*/
pub const SDMA_PKT_COPY_T2T_DW_12_dst_swizzle_mode_offset: u32 = 12;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_swizzle_mode_mask: u32 = 0x0000001F;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_swizzle_mode_shift: u32 = 3;
macro_rules! SDMA_PKT_COPY_T2T_DW_12_DST_SWIZZLE_MODE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_12_dst_swizzle_mode_mask) << SDMA_PKT_COPY_T2T_DW_12_dst_swizzle_mode_shift) };
}

/*define for dst_dimension field*/
pub const SDMA_PKT_COPY_T2T_DW_12_dst_dimension_offset: u32 = 12;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_dimension_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_dimension_shift: u32 = 9;
macro_rules! SDMA_PKT_COPY_T2T_DW_12_DST_DIMENSION {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_12_dst_dimension_mask) << SDMA_PKT_COPY_T2T_DW_12_dst_dimension_shift) };
}

/*define for dst_mip_max field*/
pub const SDMA_PKT_COPY_T2T_DW_12_dst_mip_max_offset: u32 = 12;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_mip_max_mask: u32 = 0x0000000F;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_mip_max_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_T2T_DW_12_DST_MIP_MAX {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_12_dst_mip_ma$x_mask) << SDMA_PKT_COPY_T2T_DW_12_dst_mip_ma$x_shift) };
}

/*define for dst_mip_id field*/
pub const SDMA_PKT_COPY_T2T_DW_12_dst_mip_id_offset: u32 = 12;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_mip_id_mask: u32 = 0x0000000F;
pub const SDMA_PKT_COPY_T2T_DW_12_dst_mip_id_shift: u32 = 20;
macro_rules! SDMA_PKT_COPY_T2T_DW_12_DST_MIP_ID {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_12_dst_mip_id_mask) << SDMA_PKT_COPY_T2T_DW_12_dst_mip_id_shift) };
}

/*define for DW_13 word*/
/*define for rect_x field*/
pub const SDMA_PKT_COPY_T2T_DW_13_rect_x_offset: u32 = 13;
pub const SDMA_PKT_COPY_T2T_DW_13_rect_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_DW_13_rect_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_DW_13_RECT_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_13_rect_$x_mask) << SDMA_PKT_COPY_T2T_DW_13_rect_$x_shift) };
}

/*define for rect_y field*/
pub const SDMA_PKT_COPY_T2T_DW_13_rect_y_offset: u32 = 13;
pub const SDMA_PKT_COPY_T2T_DW_13_rect_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_DW_13_rect_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_T2T_DW_13_RECT_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_13_rect_y_mask) << SDMA_PKT_COPY_T2T_DW_13_rect_y_shift) };
}

/*define for DW_14 word*/
/*define for rect_z field*/
pub const SDMA_PKT_COPY_T2T_DW_14_rect_z_offset: u32 = 14;
pub const SDMA_PKT_COPY_T2T_DW_14_rect_z_mask: u32 = 0x00001FFF;
pub const SDMA_PKT_COPY_T2T_DW_14_rect_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_DW_14_RECT_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_14_rect_z_mask) << SDMA_PKT_COPY_T2T_DW_14_rect_z_shift) };
}

/*define for dst_sw field*/
pub const SDMA_PKT_COPY_T2T_DW_14_dst_sw_offset: u32 = 14;
pub const SDMA_PKT_COPY_T2T_DW_14_dst_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_T2T_DW_14_dst_sw_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_T2T_DW_14_DST_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_14_dst_sw_mask) << SDMA_PKT_COPY_T2T_DW_14_dst_sw_shift) };
}

/*define for dst_cache_policy field*/
pub const SDMA_PKT_COPY_T2T_DW_14_dst_cache_policy_offset: u32 = 14;
pub const SDMA_PKT_COPY_T2T_DW_14_dst_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_T2T_DW_14_dst_cache_policy_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_T2T_DW_14_DST_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_14_dst_cache_policy_mask) << SDMA_PKT_COPY_T2T_DW_14_dst_cache_policy_shift) };
}

/*define for src_sw field*/
pub const SDMA_PKT_COPY_T2T_DW_14_src_sw_offset: u32 = 14;
pub const SDMA_PKT_COPY_T2T_DW_14_src_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_T2T_DW_14_src_sw_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_T2T_DW_14_SRC_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_14_src_sw_mask) << SDMA_PKT_COPY_T2T_DW_14_src_sw_shift) };
}

/*define for src_cache_policy field*/
pub const SDMA_PKT_COPY_T2T_DW_14_src_cache_policy_offset: u32 = 14;
pub const SDMA_PKT_COPY_T2T_DW_14_src_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_T2T_DW_14_src_cache_policy_shift: u32 = 26;
macro_rules! SDMA_PKT_COPY_T2T_DW_14_SRC_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_DW_14_src_cache_policy_mask) << SDMA_PKT_COPY_T2T_DW_14_src_cache_policy_shift) };
}

/*define for META_ADDR_LO word*/
/*define for meta_addr_31_0 field*/
pub const SDMA_PKT_COPY_T2T_META_ADDR_LO_meta_addr_31_0_offset: u32 = 15;
pub const SDMA_PKT_COPY_T2T_META_ADDR_LO_meta_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_T2T_META_ADDR_LO_meta_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_META_ADDR_LO_META_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_META_ADDR_LO_meta_addr_31_0_mask) << SDMA_PKT_COPY_T2T_META_ADDR_LO_meta_addr_31_0_shift) };
}

/*define for META_ADDR_HI word*/
/*define for meta_addr_63_32 field*/
pub const SDMA_PKT_COPY_T2T_META_ADDR_HI_meta_addr_63_32_offset: u32 = 16;
pub const SDMA_PKT_COPY_T2T_META_ADDR_HI_meta_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_T2T_META_ADDR_HI_meta_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_META_ADDR_HI_META_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_META_ADDR_HI_meta_addr_63_32_mask) << SDMA_PKT_COPY_T2T_META_ADDR_HI_meta_addr_63_32_shift) };
}

/*define for META_CONFIG word*/
/*define for data_format field*/
pub const SDMA_PKT_COPY_T2T_META_CONFIG_data_format_offset: u32 = 17;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_data_format_mask: u32 = 0x0000007F;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_data_format_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_META_CONFIG_DATA_FORMAT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_META_CONFIG_data_format_mask) << SDMA_PKT_COPY_T2T_META_CONFIG_data_format_shift) };
}

/*define for color_transform_disable field*/
pub const SDMA_PKT_COPY_T2T_META_CONFIG_color_transform_disable_offset: u32 = 17;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_color_transform_disable_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_color_transform_disable_shift: u32 = 7;
macro_rules! SDMA_PKT_COPY_T2T_META_CONFIG_COLOR_TRANSFORM_DISABLE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_META_CONFIG_color_transform_disable_mask) << SDMA_PKT_COPY_T2T_META_CONFIG_color_transform_disable_shift) };
}

/*define for alpha_is_on_msb field*/
pub const SDMA_PKT_COPY_T2T_META_CONFIG_alpha_is_on_msb_offset: u32 = 17;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_alpha_is_on_msb_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_alpha_is_on_msb_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_T2T_META_CONFIG_ALPHA_IS_ON_MSB {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_META_CONFIG_alpha_is_on_msb_mask) << SDMA_PKT_COPY_T2T_META_CONFIG_alpha_is_on_msb_shift) };
}

/*define for number_type field*/
pub const SDMA_PKT_COPY_T2T_META_CONFIG_number_type_offset: u32 = 17;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_number_type_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_number_type_shift: u32 = 9;
macro_rules! SDMA_PKT_COPY_T2T_META_CONFIG_NUMBER_TYPE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_META_CONFIG_number_type_mask) << SDMA_PKT_COPY_T2T_META_CONFIG_number_type_shift) };
}

/*define for surface_type field*/
pub const SDMA_PKT_COPY_T2T_META_CONFIG_surface_type_offset: u32 = 17;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_surface_type_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_surface_type_shift: u32 = 12;
macro_rules! SDMA_PKT_COPY_T2T_META_CONFIG_SURFACE_TYPE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_META_CONFIG_surface_type_mask) << SDMA_PKT_COPY_T2T_META_CONFIG_surface_type_shift) };
}

/*define for meta_llc field*/
pub const SDMA_PKT_COPY_T2T_META_CONFIG_meta_llc_offset: u32 = 17;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_meta_llc_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_meta_llc_shift: u32 = 14;
macro_rules! SDMA_PKT_COPY_T2T_META_CONFIG_META_LLC {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_META_CONFIG_meta_llc_mask) << SDMA_PKT_COPY_T2T_META_CONFIG_meta_llc_shift) };
}

/*define for max_comp_block_size field*/
pub const SDMA_PKT_COPY_T2T_META_CONFIG_max_comp_block_size_offset: u32 = 17;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_max_comp_block_size_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_max_comp_block_size_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_T2T_META_CONFIG_MAX_COMP_BLOCK_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_META_CONFIG_ma$x_comp_block_size_mask) << SDMA_PKT_COPY_T2T_META_CONFIG_ma$x_comp_block_size_shift) };
}

/*define for max_uncomp_block_size field*/
pub const SDMA_PKT_COPY_T2T_META_CONFIG_max_uncomp_block_size_offset: u32 = 17;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_max_uncomp_block_size_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_max_uncomp_block_size_shift: u32 = 26;
macro_rules! SDMA_PKT_COPY_T2T_META_CONFIG_MAX_UNCOMP_BLOCK_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_META_CONFIG_ma$x_uncomp_block_size_mask) << SDMA_PKT_COPY_T2T_META_CONFIG_ma$x_uncomp_block_size_shift) };
}

/*define for write_compress_enable field*/
pub const SDMA_PKT_COPY_T2T_META_CONFIG_write_compress_enable_offset: u32 = 17;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_write_compress_enable_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_write_compress_enable_shift: u32 = 28;
macro_rules! SDMA_PKT_COPY_T2T_META_CONFIG_WRITE_COMPRESS_ENABLE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_META_CONFIG_write_compress_enable_mask) << SDMA_PKT_COPY_T2T_META_CONFIG_write_compress_enable_shift) };
}

/*define for meta_tmz field*/
pub const SDMA_PKT_COPY_T2T_META_CONFIG_meta_tmz_offset: u32 = 17;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_meta_tmz_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_meta_tmz_shift: u32 = 29;
macro_rules! SDMA_PKT_COPY_T2T_META_CONFIG_META_TMZ {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_META_CONFIG_meta_tmz_mask) << SDMA_PKT_COPY_T2T_META_CONFIG_meta_tmz_shift) };
}

/*define for pipe_aligned field*/
pub const SDMA_PKT_COPY_T2T_META_CONFIG_pipe_aligned_offset: u32 = 17;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_pipe_aligned_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_T2T_META_CONFIG_pipe_aligned_shift: u32 = 31;
macro_rules! SDMA_PKT_COPY_T2T_META_CONFIG_PIPE_ALIGNED {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_META_CONFIG_pipe_aligned_mask) << SDMA_PKT_COPY_T2T_META_CONFIG_pipe_aligned_shift) };
}


/*
** Definitions for SDMA_PKT_COPY_T2T_BC packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_COPY_T2T_BC_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_T2T_BC_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_T2T_BC_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_BC_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_HEADER_op_mask) << SDMA_PKT_COPY_T2T_BC_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_COPY_T2T_BC_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_T2T_BC_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_T2T_BC_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_T2T_BC_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_HEADER_sub_op_mask) << SDMA_PKT_COPY_T2T_BC_HEADER_sub_op_shift) };
}

/*define for SRC_ADDR_LO word*/
/*define for src_addr_31_0 field*/
pub const SDMA_PKT_COPY_T2T_BC_SRC_ADDR_LO_src_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_COPY_T2T_BC_SRC_ADDR_LO_src_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_T2T_BC_SRC_ADDR_LO_src_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_BC_SRC_ADDR_LO_SRC_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_SRC_ADDR_LO_src_addr_31_0_mask) << SDMA_PKT_COPY_T2T_BC_SRC_ADDR_LO_src_addr_31_0_shift) };
}

/*define for SRC_ADDR_HI word*/
/*define for src_addr_63_32 field*/
pub const SDMA_PKT_COPY_T2T_BC_SRC_ADDR_HI_src_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_COPY_T2T_BC_SRC_ADDR_HI_src_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_T2T_BC_SRC_ADDR_HI_src_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_BC_SRC_ADDR_HI_SRC_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_SRC_ADDR_HI_src_addr_63_32_mask) << SDMA_PKT_COPY_T2T_BC_SRC_ADDR_HI_src_addr_63_32_shift) };
}

/*define for DW_3 word*/
/*define for src_x field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_3_src_x_offset: u32 = 3;
pub const SDMA_PKT_COPY_T2T_BC_DW_3_src_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_BC_DW_3_src_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_3_SRC_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_3_src_$x_mask) << SDMA_PKT_COPY_T2T_BC_DW_3_src_$x_shift) };
}

/*define for src_y field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_3_src_y_offset: u32 = 3;
pub const SDMA_PKT_COPY_T2T_BC_DW_3_src_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_BC_DW_3_src_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_3_SRC_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_3_src_y_mask) << SDMA_PKT_COPY_T2T_BC_DW_3_src_y_shift) };
}

/*define for DW_4 word*/
/*define for src_z field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_4_src_z_offset: u32 = 4;
pub const SDMA_PKT_COPY_T2T_BC_DW_4_src_z_mask: u32 = 0x000007FF;
pub const SDMA_PKT_COPY_T2T_BC_DW_4_src_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_4_SRC_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_4_src_z_mask) << SDMA_PKT_COPY_T2T_BC_DW_4_src_z_shift) };
}

/*define for src_width field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_4_src_width_offset: u32 = 4;
pub const SDMA_PKT_COPY_T2T_BC_DW_4_src_width_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_BC_DW_4_src_width_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_4_SRC_WIDTH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_4_src_width_mask) << SDMA_PKT_COPY_T2T_BC_DW_4_src_width_shift) };
}

/*define for DW_5 word*/
/*define for src_height field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_5_src_height_offset: u32 = 5;
pub const SDMA_PKT_COPY_T2T_BC_DW_5_src_height_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_BC_DW_5_src_height_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_5_SRC_HEIGHT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_5_src_height_mask) << SDMA_PKT_COPY_T2T_BC_DW_5_src_height_shift) };
}

/*define for src_depth field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_5_src_depth_offset: u32 = 5;
pub const SDMA_PKT_COPY_T2T_BC_DW_5_src_depth_mask: u32 = 0x000007FF;
pub const SDMA_PKT_COPY_T2T_BC_DW_5_src_depth_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_5_SRC_DEPTH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_5_src_depth_mask) << SDMA_PKT_COPY_T2T_BC_DW_5_src_depth_shift) };
}

/*define for DW_6 word*/
/*define for src_element_size field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_element_size_offset: u32 = 6;
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_element_size_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_element_size_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_6_SRC_ELEMENT_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_6_src_element_size_mask) << SDMA_PKT_COPY_T2T_BC_DW_6_src_element_size_shift) };
}

/*define for src_array_mode field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_array_mode_offset: u32 = 6;
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_array_mode_mask: u32 = 0x0000000F;
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_array_mode_shift: u32 = 3;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_6_SRC_ARRAY_MODE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_6_src_array_mode_mask) << SDMA_PKT_COPY_T2T_BC_DW_6_src_array_mode_shift) };
}

/*define for src_mit_mode field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_mit_mode_offset: u32 = 6;
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_mit_mode_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_mit_mode_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_6_SRC_MIT_MODE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_6_src_mit_mode_mask) << SDMA_PKT_COPY_T2T_BC_DW_6_src_mit_mode_shift) };
}

/*define for src_tilesplit_size field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_tilesplit_size_offset: u32 = 6;
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_tilesplit_size_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_tilesplit_size_shift: u32 = 11;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_6_SRC_TILESPLIT_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_6_src_tilesplit_size_mask) << SDMA_PKT_COPY_T2T_BC_DW_6_src_tilesplit_size_shift) };
}

/*define for src_bank_w field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_bank_w_offset: u32 = 6;
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_bank_w_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_bank_w_shift: u32 = 15;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_6_SRC_BANK_W {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_6_src_bank_w_mask) << SDMA_PKT_COPY_T2T_BC_DW_6_src_bank_w_shift) };
}

/*define for src_bank_h field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_bank_h_offset: u32 = 6;
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_bank_h_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_bank_h_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_6_SRC_BANK_H {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_6_src_bank_h_mask) << SDMA_PKT_COPY_T2T_BC_DW_6_src_bank_h_shift) };
}

/*define for src_num_bank field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_num_bank_offset: u32 = 6;
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_num_bank_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_num_bank_shift: u32 = 21;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_6_SRC_NUM_BANK {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_6_src_num_bank_mask) << SDMA_PKT_COPY_T2T_BC_DW_6_src_num_bank_shift) };
}

/*define for src_mat_aspt field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_mat_aspt_offset: u32 = 6;
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_mat_aspt_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_mat_aspt_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_6_SRC_MAT_ASPT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_6_src_mat_aspt_mask) << SDMA_PKT_COPY_T2T_BC_DW_6_src_mat_aspt_shift) };
}

/*define for src_pipe_config field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_pipe_config_offset: u32 = 6;
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_pipe_config_mask: u32 = 0x0000001F;
pub const SDMA_PKT_COPY_T2T_BC_DW_6_src_pipe_config_shift: u32 = 26;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_6_SRC_PIPE_CONFIG {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_6_src_pipe_config_mask) << SDMA_PKT_COPY_T2T_BC_DW_6_src_pipe_config_shift) };
}

/*define for DST_ADDR_LO word*/
/*define for dst_addr_31_0 field*/
pub const SDMA_PKT_COPY_T2T_BC_DST_ADDR_LO_dst_addr_31_0_offset: u32 = 7;
pub const SDMA_PKT_COPY_T2T_BC_DST_ADDR_LO_dst_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_T2T_BC_DST_ADDR_LO_dst_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_BC_DST_ADDR_LO_DST_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DST_ADDR_LO_dst_addr_31_0_mask) << SDMA_PKT_COPY_T2T_BC_DST_ADDR_LO_dst_addr_31_0_shift) };
}

/*define for DST_ADDR_HI word*/
/*define for dst_addr_63_32 field*/
pub const SDMA_PKT_COPY_T2T_BC_DST_ADDR_HI_dst_addr_63_32_offset: u32 = 8;
pub const SDMA_PKT_COPY_T2T_BC_DST_ADDR_HI_dst_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_T2T_BC_DST_ADDR_HI_dst_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_BC_DST_ADDR_HI_DST_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DST_ADDR_HI_dst_addr_63_32_mask) << SDMA_PKT_COPY_T2T_BC_DST_ADDR_HI_dst_addr_63_32_shift) };
}

/*define for DW_9 word*/
/*define for dst_x field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_9_dst_x_offset: u32 = 9;
pub const SDMA_PKT_COPY_T2T_BC_DW_9_dst_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_BC_DW_9_dst_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_9_DST_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_9_dst_$x_mask) << SDMA_PKT_COPY_T2T_BC_DW_9_dst_$x_shift) };
}

/*define for dst_y field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_9_dst_y_offset: u32 = 9;
pub const SDMA_PKT_COPY_T2T_BC_DW_9_dst_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_BC_DW_9_dst_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_9_DST_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_9_dst_y_mask) << SDMA_PKT_COPY_T2T_BC_DW_9_dst_y_shift) };
}

/*define for DW_10 word*/
/*define for dst_z field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_10_dst_z_offset: u32 = 10;
pub const SDMA_PKT_COPY_T2T_BC_DW_10_dst_z_mask: u32 = 0x000007FF;
pub const SDMA_PKT_COPY_T2T_BC_DW_10_dst_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_10_DST_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_10_dst_z_mask) << SDMA_PKT_COPY_T2T_BC_DW_10_dst_z_shift) };
}

/*define for dst_width field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_10_dst_width_offset: u32 = 10;
pub const SDMA_PKT_COPY_T2T_BC_DW_10_dst_width_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_BC_DW_10_dst_width_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_10_DST_WIDTH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_10_dst_width_mask) << SDMA_PKT_COPY_T2T_BC_DW_10_dst_width_shift) };
}

/*define for DW_11 word*/
/*define for dst_height field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_11_dst_height_offset: u32 = 11;
pub const SDMA_PKT_COPY_T2T_BC_DW_11_dst_height_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_BC_DW_11_dst_height_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_11_DST_HEIGHT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_11_dst_height_mask) << SDMA_PKT_COPY_T2T_BC_DW_11_dst_height_shift) };
}

/*define for dst_depth field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_11_dst_depth_offset: u32 = 11;
pub const SDMA_PKT_COPY_T2T_BC_DW_11_dst_depth_mask: u32 = 0x00000FFF;
pub const SDMA_PKT_COPY_T2T_BC_DW_11_dst_depth_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_11_DST_DEPTH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_11_dst_depth_mask) << SDMA_PKT_COPY_T2T_BC_DW_11_dst_depth_shift) };
}

/*define for DW_12 word*/
/*define for dst_element_size field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_element_size_offset: u32 = 12;
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_element_size_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_element_size_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_12_DST_ELEMENT_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_12_dst_element_size_mask) << SDMA_PKT_COPY_T2T_BC_DW_12_dst_element_size_shift) };
}

/*define for dst_array_mode field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_array_mode_offset: u32 = 12;
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_array_mode_mask: u32 = 0x0000000F;
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_array_mode_shift: u32 = 3;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_12_DST_ARRAY_MODE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_12_dst_array_mode_mask) << SDMA_PKT_COPY_T2T_BC_DW_12_dst_array_mode_shift) };
}

/*define for dst_mit_mode field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_mit_mode_offset: u32 = 12;
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_mit_mode_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_mit_mode_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_12_DST_MIT_MODE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_12_dst_mit_mode_mask) << SDMA_PKT_COPY_T2T_BC_DW_12_dst_mit_mode_shift) };
}

/*define for dst_tilesplit_size field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_tilesplit_size_offset: u32 = 12;
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_tilesplit_size_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_tilesplit_size_shift: u32 = 11;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_12_DST_TILESPLIT_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_12_dst_tilesplit_size_mask) << SDMA_PKT_COPY_T2T_BC_DW_12_dst_tilesplit_size_shift) };
}

/*define for dst_bank_w field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_bank_w_offset: u32 = 12;
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_bank_w_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_bank_w_shift: u32 = 15;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_12_DST_BANK_W {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_12_dst_bank_w_mask) << SDMA_PKT_COPY_T2T_BC_DW_12_dst_bank_w_shift) };
}

/*define for dst_bank_h field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_bank_h_offset: u32 = 12;
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_bank_h_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_bank_h_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_12_DST_BANK_H {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_12_dst_bank_h_mask) << SDMA_PKT_COPY_T2T_BC_DW_12_dst_bank_h_shift) };
}

/*define for dst_num_bank field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_num_bank_offset: u32 = 12;
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_num_bank_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_num_bank_shift: u32 = 21;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_12_DST_NUM_BANK {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_12_dst_num_bank_mask) << SDMA_PKT_COPY_T2T_BC_DW_12_dst_num_bank_shift) };
}

/*define for dst_mat_aspt field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_mat_aspt_offset: u32 = 12;
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_mat_aspt_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_mat_aspt_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_12_DST_MAT_ASPT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_12_dst_mat_aspt_mask) << SDMA_PKT_COPY_T2T_BC_DW_12_dst_mat_aspt_shift) };
}

/*define for dst_pipe_config field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_pipe_config_offset: u32 = 12;
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_pipe_config_mask: u32 = 0x0000001F;
pub const SDMA_PKT_COPY_T2T_BC_DW_12_dst_pipe_config_shift: u32 = 26;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_12_DST_PIPE_CONFIG {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_12_dst_pipe_config_mask) << SDMA_PKT_COPY_T2T_BC_DW_12_dst_pipe_config_shift) };
}

/*define for DW_13 word*/
/*define for rect_x field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_13_rect_x_offset: u32 = 13;
pub const SDMA_PKT_COPY_T2T_BC_DW_13_rect_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_BC_DW_13_rect_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_13_RECT_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_13_rect_$x_mask) << SDMA_PKT_COPY_T2T_BC_DW_13_rect_$x_shift) };
}

/*define for rect_y field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_13_rect_y_offset: u32 = 13;
pub const SDMA_PKT_COPY_T2T_BC_DW_13_rect_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_T2T_BC_DW_13_rect_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_13_RECT_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_13_rect_y_mask) << SDMA_PKT_COPY_T2T_BC_DW_13_rect_y_shift) };
}

/*define for DW_14 word*/
/*define for rect_z field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_14_rect_z_offset: u32 = 14;
pub const SDMA_PKT_COPY_T2T_BC_DW_14_rect_z_mask: u32 = 0x000007FF;
pub const SDMA_PKT_COPY_T2T_BC_DW_14_rect_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_14_RECT_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_14_rect_z_mask) << SDMA_PKT_COPY_T2T_BC_DW_14_rect_z_shift) };
}

/*define for dst_sw field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_14_dst_sw_offset: u32 = 14;
pub const SDMA_PKT_COPY_T2T_BC_DW_14_dst_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_T2T_BC_DW_14_dst_sw_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_14_DST_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_14_dst_sw_mask) << SDMA_PKT_COPY_T2T_BC_DW_14_dst_sw_shift) };
}

/*define for src_sw field*/
pub const SDMA_PKT_COPY_T2T_BC_DW_14_src_sw_offset: u32 = 14;
pub const SDMA_PKT_COPY_T2T_BC_DW_14_src_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_T2T_BC_DW_14_src_sw_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_T2T_BC_DW_14_SRC_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_T2T_BC_DW_14_src_sw_mask) << SDMA_PKT_COPY_T2T_BC_DW_14_src_sw_shift) };
}


/*
** Definitions for SDMA_PKT_COPY_TILED_SUBWIN packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_HEADER_op_mask) << SDMA_PKT_COPY_TILED_SUBWIN_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_HEADER_sub_op_mask) << SDMA_PKT_COPY_TILED_SUBWIN_HEADER_sub_op_shift) };
}

/*define for tmz field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_tmz_offset: u32 = 0;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_tmz_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_tmz_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_HEADER_TMZ {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_HEADER_tmz_mask) << SDMA_PKT_COPY_TILED_SUBWIN_HEADER_tmz_shift) };
}

/*define for dcc field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_dcc_offset: u32 = 0;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_dcc_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_dcc_shift: u32 = 19;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_HEADER_DCC {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_HEADER_dcc_mask) << SDMA_PKT_COPY_TILED_SUBWIN_HEADER_dcc_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_cpv_shift: u32 = 28;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_HEADER_cpv_mask) << SDMA_PKT_COPY_TILED_SUBWIN_HEADER_cpv_shift) };
}

/*define for detile field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_detile_offset: u32 = 0;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_detile_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_TILED_SUBWIN_HEADER_detile_shift: u32 = 31;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_HEADER_DETILE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_HEADER_detile_mask) << SDMA_PKT_COPY_TILED_SUBWIN_HEADER_detile_shift) };
}

/*define for TILED_ADDR_LO word*/
/*define for tiled_addr_31_0 field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_TILED_ADDR_LO_tiled_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_COPY_TILED_SUBWIN_TILED_ADDR_LO_tiled_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_TILED_ADDR_LO_tiled_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_TILED_ADDR_LO_TILED_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_TILED_ADDR_LO_tiled_addr_31_0_mask) << SDMA_PKT_COPY_TILED_SUBWIN_TILED_ADDR_LO_tiled_addr_31_0_shift) };
}

/*define for TILED_ADDR_HI word*/
/*define for tiled_addr_63_32 field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_TILED_ADDR_HI_tiled_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_COPY_TILED_SUBWIN_TILED_ADDR_HI_tiled_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_TILED_ADDR_HI_tiled_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_TILED_ADDR_HI_TILED_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_TILED_ADDR_HI_tiled_addr_63_32_mask) << SDMA_PKT_COPY_TILED_SUBWIN_TILED_ADDR_HI_tiled_addr_63_32_shift) };
}

/*define for DW_3 word*/
/*define for tiled_x field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_3_tiled_x_offset: u32 = 3;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_3_tiled_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_3_tiled_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_3_TILED_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_3_tiled_$x_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_3_tiled_$x_shift) };
}

/*define for tiled_y field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_3_tiled_y_offset: u32 = 3;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_3_tiled_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_3_tiled_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_3_TILED_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_3_tiled_y_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_3_tiled_y_shift) };
}

/*define for DW_4 word*/
/*define for tiled_z field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_4_tiled_z_offset: u32 = 4;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_4_tiled_z_mask: u32 = 0x00001FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_4_tiled_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_4_TILED_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_4_tiled_z_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_4_tiled_z_shift) };
}

/*define for width field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_4_width_offset: u32 = 4;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_4_width_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_4_width_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_4_WIDTH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_4_width_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_4_width_shift) };
}

/*define for DW_5 word*/
/*define for height field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_5_height_offset: u32 = 5;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_5_height_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_5_height_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_5_HEIGHT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_5_height_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_5_height_shift) };
}

/*define for depth field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_5_depth_offset: u32 = 5;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_5_depth_mask: u32 = 0x00001FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_5_depth_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_5_DEPTH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_5_depth_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_5_depth_shift) };
}

/*define for DW_6 word*/
/*define for element_size field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_element_size_offset: u32 = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_element_size_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_element_size_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_6_ELEMENT_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_6_element_size_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_6_element_size_shift) };
}

/*define for swizzle_mode field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_swizzle_mode_offset: u32 = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_swizzle_mode_mask: u32 = 0x0000001F;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_swizzle_mode_shift: u32 = 3;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_6_SWIZZLE_MODE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_6_swizzle_mode_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_6_swizzle_mode_shift) };
}

/*define for dimension field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_dimension_offset: u32 = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_dimension_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_dimension_shift: u32 = 9;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_6_DIMENSION {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_6_dimension_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_6_dimension_shift) };
}

/*define for mip_max field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_mip_max_offset: u32 = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_mip_max_mask: u32 = 0x0000000F;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_mip_max_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_6_MIP_MAX {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_6_mip_ma$x_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_6_mip_ma$x_shift) };
}

/*define for mip_id field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_mip_id_offset: u32 = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_mip_id_mask: u32 = 0x0000000F;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_6_mip_id_shift: u32 = 20;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_6_MIP_ID {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_6_mip_id_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_6_mip_id_shift) };
}

/*define for LINEAR_ADDR_LO word*/
/*define for linear_addr_31_0 field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_LINEAR_ADDR_LO_linear_addr_31_0_offset: u32 = 7;
pub const SDMA_PKT_COPY_TILED_SUBWIN_LINEAR_ADDR_LO_linear_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_LINEAR_ADDR_LO_linear_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_LINEAR_ADDR_LO_LINEAR_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_LINEAR_ADDR_LO_linear_addr_31_0_mask) << SDMA_PKT_COPY_TILED_SUBWIN_LINEAR_ADDR_LO_linear_addr_31_0_shift) };
}

/*define for LINEAR_ADDR_HI word*/
/*define for linear_addr_63_32 field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_LINEAR_ADDR_HI_linear_addr_63_32_offset: u32 = 8;
pub const SDMA_PKT_COPY_TILED_SUBWIN_LINEAR_ADDR_HI_linear_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_LINEAR_ADDR_HI_linear_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_LINEAR_ADDR_HI_LINEAR_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_LINEAR_ADDR_HI_linear_addr_63_32_mask) << SDMA_PKT_COPY_TILED_SUBWIN_LINEAR_ADDR_HI_linear_addr_63_32_shift) };
}

/*define for DW_9 word*/
/*define for linear_x field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_9_linear_x_offset: u32 = 9;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_9_linear_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_9_linear_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_9_LINEAR_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_9_linear_$x_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_9_linear_$x_shift) };
}

/*define for linear_y field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_9_linear_y_offset: u32 = 9;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_9_linear_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_9_linear_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_9_LINEAR_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_9_linear_y_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_9_linear_y_shift) };
}

/*define for DW_10 word*/
/*define for linear_z field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_10_linear_z_offset: u32 = 10;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_10_linear_z_mask: u32 = 0x00001FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_10_linear_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_10_LINEAR_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_10_linear_z_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_10_linear_z_shift) };
}

/*define for linear_pitch field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_10_linear_pitch_offset: u32 = 10;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_10_linear_pitch_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_10_linear_pitch_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_10_LINEAR_PITCH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_10_linear_pitch_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_10_linear_pitch_shift) };
}

/*define for DW_11 word*/
/*define for linear_slice_pitch field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_11_linear_slice_pitch_offset: u32 = 11;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_11_linear_slice_pitch_mask: u32 = 0x0FFFFFFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_11_linear_slice_pitch_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_11_LINEAR_SLICE_PITCH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_11_linear_slice_pitch_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_11_linear_slice_pitch_shift) };
}

/*define for DW_12 word*/
/*define for rect_x field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_12_rect_x_offset: u32 = 12;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_12_rect_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_12_rect_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_12_RECT_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_12_rect_$x_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_12_rect_$x_shift) };
}

/*define for rect_y field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_12_rect_y_offset: u32 = 12;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_12_rect_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_12_rect_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_12_RECT_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_12_rect_y_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_12_rect_y_shift) };
}

/*define for DW_13 word*/
/*define for rect_z field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_rect_z_offset: u32 = 13;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_rect_z_mask: u32 = 0x00001FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_rect_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_13_RECT_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_13_rect_z_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_13_rect_z_shift) };
}

/*define for linear_sw field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_linear_sw_offset: u32 = 13;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_linear_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_linear_sw_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_13_LINEAR_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_13_linear_sw_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_13_linear_sw_shift) };
}

/*define for linear_cache_policy field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_linear_cache_policy_offset: u32 = 13;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_linear_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_linear_cache_policy_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_13_LINEAR_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_13_linear_cache_policy_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_13_linear_cache_policy_shift) };
}

/*define for tile_sw field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_tile_sw_offset: u32 = 13;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_tile_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_tile_sw_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_13_TILE_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_13_tile_sw_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_13_tile_sw_shift) };
}

/*define for tile_cache_policy field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_tile_cache_policy_offset: u32 = 13;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_tile_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_TILED_SUBWIN_DW_13_tile_cache_policy_shift: u32 = 26;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_DW_13_TILE_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_DW_13_tile_cache_policy_mask) << SDMA_PKT_COPY_TILED_SUBWIN_DW_13_tile_cache_policy_shift) };
}

/*define for META_ADDR_LO word*/
/*define for meta_addr_31_0 field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_ADDR_LO_meta_addr_31_0_offset: u32 = 14;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_ADDR_LO_meta_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_ADDR_LO_meta_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_META_ADDR_LO_META_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_META_ADDR_LO_meta_addr_31_0_mask) << SDMA_PKT_COPY_TILED_SUBWIN_META_ADDR_LO_meta_addr_31_0_shift) };
}

/*define for META_ADDR_HI word*/
/*define for meta_addr_63_32 field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_ADDR_HI_meta_addr_63_32_offset: u32 = 15;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_ADDR_HI_meta_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_ADDR_HI_meta_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_META_ADDR_HI_META_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_META_ADDR_HI_meta_addr_63_32_mask) << SDMA_PKT_COPY_TILED_SUBWIN_META_ADDR_HI_meta_addr_63_32_shift) };
}

/*define for META_CONFIG word*/
/*define for data_format field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_data_format_offset: u32 = 16;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_data_format_mask: u32 = 0x0000007F;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_data_format_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_DATA_FORMAT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_data_format_mask) << SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_data_format_shift) };
}

/*define for color_transform_disable field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_color_transform_disable_offset: u32 = 16;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_color_transform_disable_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_color_transform_disable_shift: u32 = 7;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_COLOR_TRANSFORM_DISABLE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_color_transform_disable_mask) << SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_color_transform_disable_shift) };
}

/*define for alpha_is_on_msb field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_alpha_is_on_msb_offset: u32 = 16;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_alpha_is_on_msb_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_alpha_is_on_msb_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_ALPHA_IS_ON_MSB {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_alpha_is_on_msb_mask) << SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_alpha_is_on_msb_shift) };
}

/*define for number_type field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_number_type_offset: u32 = 16;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_number_type_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_number_type_shift: u32 = 9;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_NUMBER_TYPE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_number_type_mask) << SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_number_type_shift) };
}

/*define for surface_type field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_surface_type_offset: u32 = 16;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_surface_type_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_surface_type_shift: u32 = 12;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_SURFACE_TYPE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_surface_type_mask) << SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_surface_type_shift) };
}

/*define for meta_llc field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_meta_llc_offset: u32 = 16;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_meta_llc_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_meta_llc_shift: u32 = 14;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_META_LLC {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_meta_llc_mask) << SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_meta_llc_shift) };
}

/*define for max_comp_block_size field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_max_comp_block_size_offset: u32 = 16;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_max_comp_block_size_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_max_comp_block_size_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_MAX_COMP_BLOCK_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_ma$x_comp_block_size_mask) << SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_ma$x_comp_block_size_shift) };
}

/*define for max_uncomp_block_size field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_max_uncomp_block_size_offset: u32 = 16;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_max_uncomp_block_size_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_max_uncomp_block_size_shift: u32 = 26;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_MAX_UNCOMP_BLOCK_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_ma$x_uncomp_block_size_mask) << SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_ma$x_uncomp_block_size_shift) };
}

/*define for write_compress_enable field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_write_compress_enable_offset: u32 = 16;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_write_compress_enable_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_write_compress_enable_shift: u32 = 28;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_WRITE_COMPRESS_ENABLE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_write_compress_enable_mask) << SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_write_compress_enable_shift) };
}

/*define for meta_tmz field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_meta_tmz_offset: u32 = 16;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_meta_tmz_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_meta_tmz_shift: u32 = 29;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_META_TMZ {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_meta_tmz_mask) << SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_meta_tmz_shift) };
}

/*define for pipe_aligned field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_pipe_aligned_offset: u32 = 16;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_pipe_aligned_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_pipe_aligned_shift: u32 = 31;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_PIPE_ALIGNED {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_pipe_aligned_mask) << SDMA_PKT_COPY_TILED_SUBWIN_META_CONFIG_pipe_aligned_shift) };
}


/*
** Definitions for SDMA_PKT_COPY_TILED_SUBWIN_BC packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_HEADER_op_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_HEADER_sub_op_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_HEADER_sub_op_shift) };
}

/*define for detile field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_HEADER_detile_offset: u32 = 0;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_HEADER_detile_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_HEADER_detile_shift: u32 = 31;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_HEADER_DETILE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_HEADER_detile_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_HEADER_detile_shift) };
}

/*define for TILED_ADDR_LO word*/
/*define for tiled_addr_31_0 field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_TILED_ADDR_LO_tiled_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_TILED_ADDR_LO_tiled_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_TILED_ADDR_LO_tiled_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_TILED_ADDR_LO_TILED_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_TILED_ADDR_LO_tiled_addr_31_0_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_TILED_ADDR_LO_tiled_addr_31_0_shift) };
}

/*define for TILED_ADDR_HI word*/
/*define for tiled_addr_63_32 field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_TILED_ADDR_HI_tiled_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_TILED_ADDR_HI_tiled_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_TILED_ADDR_HI_tiled_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_TILED_ADDR_HI_TILED_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_TILED_ADDR_HI_tiled_addr_63_32_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_TILED_ADDR_HI_tiled_addr_63_32_shift) };
}

/*define for DW_3 word*/
/*define for tiled_x field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_3_tiled_x_offset: u32 = 3;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_3_tiled_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_3_tiled_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_3_TILED_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_3_tiled_$x_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_3_tiled_$x_shift) };
}

/*define for tiled_y field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_3_tiled_y_offset: u32 = 3;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_3_tiled_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_3_tiled_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_3_TILED_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_3_tiled_y_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_3_tiled_y_shift) };
}

/*define for DW_4 word*/
/*define for tiled_z field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_4_tiled_z_offset: u32 = 4;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_4_tiled_z_mask: u32 = 0x000007FF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_4_tiled_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_4_TILED_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_4_tiled_z_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_4_tiled_z_shift) };
}

/*define for width field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_4_width_offset: u32 = 4;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_4_width_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_4_width_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_4_WIDTH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_4_width_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_4_width_shift) };
}

/*define for DW_5 word*/
/*define for height field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_5_height_offset: u32 = 5;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_5_height_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_5_height_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_5_HEIGHT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_5_height_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_5_height_shift) };
}

/*define for depth field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_5_depth_offset: u32 = 5;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_5_depth_mask: u32 = 0x000007FF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_5_depth_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_5_DEPTH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_5_depth_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_5_depth_shift) };
}

/*define for DW_6 word*/
/*define for element_size field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_element_size_offset: u32 = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_element_size_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_element_size_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_ELEMENT_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_element_size_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_element_size_shift) };
}

/*define for array_mode field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_array_mode_offset: u32 = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_array_mode_mask: u32 = 0x0000000F;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_array_mode_shift: u32 = 3;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_ARRAY_MODE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_array_mode_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_array_mode_shift) };
}

/*define for mit_mode field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_mit_mode_offset: u32 = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_mit_mode_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_mit_mode_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_MIT_MODE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_mit_mode_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_mit_mode_shift) };
}

/*define for tilesplit_size field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_tilesplit_size_offset: u32 = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_tilesplit_size_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_tilesplit_size_shift: u32 = 11;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_TILESPLIT_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_tilesplit_size_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_tilesplit_size_shift) };
}

/*define for bank_w field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_bank_w_offset: u32 = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_bank_w_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_bank_w_shift: u32 = 15;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_BANK_W {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_bank_w_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_bank_w_shift) };
}

/*define for bank_h field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_bank_h_offset: u32 = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_bank_h_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_bank_h_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_BANK_H {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_bank_h_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_bank_h_shift) };
}

/*define for num_bank field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_num_bank_offset: u32 = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_num_bank_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_num_bank_shift: u32 = 21;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_NUM_BANK {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_num_bank_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_num_bank_shift) };
}

/*define for mat_aspt field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_mat_aspt_offset: u32 = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_mat_aspt_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_mat_aspt_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_MAT_ASPT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_mat_aspt_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_mat_aspt_shift) };
}

/*define for pipe_config field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_pipe_config_offset: u32 = 6;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_pipe_config_mask: u32 = 0x0000001F;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_pipe_config_shift: u32 = 26;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_PIPE_CONFIG {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_pipe_config_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_6_pipe_config_shift) };
}

/*define for LINEAR_ADDR_LO word*/
/*define for linear_addr_31_0 field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_LINEAR_ADDR_LO_linear_addr_31_0_offset: u32 = 7;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_LINEAR_ADDR_LO_linear_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_LINEAR_ADDR_LO_linear_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_LINEAR_ADDR_LO_LINEAR_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_LINEAR_ADDR_LO_linear_addr_31_0_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_LINEAR_ADDR_LO_linear_addr_31_0_shift) };
}

/*define for LINEAR_ADDR_HI word*/
/*define for linear_addr_63_32 field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_LINEAR_ADDR_HI_linear_addr_63_32_offset: u32 = 8;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_LINEAR_ADDR_HI_linear_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_LINEAR_ADDR_HI_linear_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_LINEAR_ADDR_HI_LINEAR_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_LINEAR_ADDR_HI_linear_addr_63_32_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_LINEAR_ADDR_HI_linear_addr_63_32_shift) };
}

/*define for DW_9 word*/
/*define for linear_x field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_9_linear_x_offset: u32 = 9;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_9_linear_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_9_linear_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_9_LINEAR_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_9_linear_$x_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_9_linear_$x_shift) };
}

/*define for linear_y field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_9_linear_y_offset: u32 = 9;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_9_linear_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_9_linear_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_9_LINEAR_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_9_linear_y_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_9_linear_y_shift) };
}

/*define for DW_10 word*/
/*define for linear_z field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_10_linear_z_offset: u32 = 10;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_10_linear_z_mask: u32 = 0x000007FF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_10_linear_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_10_LINEAR_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_10_linear_z_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_10_linear_z_shift) };
}

/*define for linear_pitch field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_10_linear_pitch_offset: u32 = 10;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_10_linear_pitch_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_10_linear_pitch_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_10_LINEAR_PITCH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_10_linear_pitch_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_10_linear_pitch_shift) };
}

/*define for DW_11 word*/
/*define for linear_slice_pitch field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_11_linear_slice_pitch_offset: u32 = 11;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_11_linear_slice_pitch_mask: u32 = 0x0FFFFFFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_11_linear_slice_pitch_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_11_LINEAR_SLICE_PITCH {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_11_linear_slice_pitch_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_11_linear_slice_pitch_shift) };
}

/*define for DW_12 word*/
/*define for rect_x field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_12_rect_x_offset: u32 = 12;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_12_rect_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_12_rect_x_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_12_RECT_X {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_12_rect_$x_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_12_rect_$x_shift) };
}

/*define for rect_y field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_12_rect_y_offset: u32 = 12;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_12_rect_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_12_rect_y_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_12_RECT_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_12_rect_y_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_12_rect_y_shift) };
}

/*define for DW_13 word*/
/*define for rect_z field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_13_rect_z_offset: u32 = 13;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_13_rect_z_mask: u32 = 0x000007FF;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_13_rect_z_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_13_RECT_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_13_rect_z_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_13_rect_z_shift) };
}

/*define for linear_sw field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_13_linear_sw_offset: u32 = 13;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_13_linear_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_13_linear_sw_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_13_LINEAR_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_13_linear_sw_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_13_linear_sw_shift) };
}

/*define for tile_sw field*/
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_13_tile_sw_offset: u32 = 13;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_13_tile_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_13_tile_sw_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_13_TILE_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_13_tile_sw_mask) << SDMA_PKT_COPY_TILED_SUBWIN_BC_DW_13_tile_sw_shift) };
}


/*
** Definitions for SDMA_PKT_COPY_STRUCT packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_COPY_STRUCT_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_STRUCT_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_STRUCT_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_STRUCT_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_STRUCT_HEADER_op_mask) << SDMA_PKT_COPY_STRUCT_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_COPY_STRUCT_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_COPY_STRUCT_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COPY_STRUCT_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_COPY_STRUCT_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_STRUCT_HEADER_sub_op_mask) << SDMA_PKT_COPY_STRUCT_HEADER_sub_op_shift) };
}

/*define for tmz field*/
pub const SDMA_PKT_COPY_STRUCT_HEADER_tmz_offset: u32 = 0;
pub const SDMA_PKT_COPY_STRUCT_HEADER_tmz_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_STRUCT_HEADER_tmz_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_STRUCT_HEADER_TMZ {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_STRUCT_HEADER_tmz_mask) << SDMA_PKT_COPY_STRUCT_HEADER_tmz_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_COPY_STRUCT_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_COPY_STRUCT_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_STRUCT_HEADER_cpv_shift: u32 = 28;
macro_rules! SDMA_PKT_COPY_STRUCT_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_STRUCT_HEADER_cpv_mask) << SDMA_PKT_COPY_STRUCT_HEADER_cpv_shift) };
}

/*define for detile field*/
pub const SDMA_PKT_COPY_STRUCT_HEADER_detile_offset: u32 = 0;
pub const SDMA_PKT_COPY_STRUCT_HEADER_detile_mask: u32 = 0x00000001;
pub const SDMA_PKT_COPY_STRUCT_HEADER_detile_shift: u32 = 31;
macro_rules! SDMA_PKT_COPY_STRUCT_HEADER_DETILE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_STRUCT_HEADER_detile_mask) << SDMA_PKT_COPY_STRUCT_HEADER_detile_shift) };
}

/*define for SB_ADDR_LO word*/
/*define for sb_addr_31_0 field*/
pub const SDMA_PKT_COPY_STRUCT_SB_ADDR_LO_sb_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_COPY_STRUCT_SB_ADDR_LO_sb_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_STRUCT_SB_ADDR_LO_sb_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_STRUCT_SB_ADDR_LO_SB_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_STRUCT_SB_ADDR_LO_sb_addr_31_0_mask) << SDMA_PKT_COPY_STRUCT_SB_ADDR_LO_sb_addr_31_0_shift) };
}

/*define for SB_ADDR_HI word*/
/*define for sb_addr_63_32 field*/
pub const SDMA_PKT_COPY_STRUCT_SB_ADDR_HI_sb_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_COPY_STRUCT_SB_ADDR_HI_sb_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_STRUCT_SB_ADDR_HI_sb_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_STRUCT_SB_ADDR_HI_SB_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_STRUCT_SB_ADDR_HI_sb_addr_63_32_mask) << SDMA_PKT_COPY_STRUCT_SB_ADDR_HI_sb_addr_63_32_shift) };
}

/*define for START_INDEX word*/
/*define for start_index field*/
pub const SDMA_PKT_COPY_STRUCT_START_INDEX_start_index_offset: u32 = 3;
pub const SDMA_PKT_COPY_STRUCT_START_INDEX_start_index_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_STRUCT_START_INDEX_start_index_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_STRUCT_START_INDEX_START_INDEX {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_STRUCT_START_INDEX_start_inde$x_mask) << SDMA_PKT_COPY_STRUCT_START_INDEX_start_inde$x_shift) };
}

/*define for COUNT word*/
/*define for count field*/
pub const SDMA_PKT_COPY_STRUCT_COUNT_count_offset: u32 = 4;
pub const SDMA_PKT_COPY_STRUCT_COUNT_count_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_STRUCT_COUNT_count_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_STRUCT_COUNT_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_STRUCT_COUNT_count_mask) << SDMA_PKT_COPY_STRUCT_COUNT_count_shift) };
}

/*define for DW_5 word*/
/*define for stride field*/
pub const SDMA_PKT_COPY_STRUCT_DW_5_stride_offset: u32 = 5;
pub const SDMA_PKT_COPY_STRUCT_DW_5_stride_mask: u32 = 0x000007FF;
pub const SDMA_PKT_COPY_STRUCT_DW_5_stride_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_STRUCT_DW_5_STRIDE {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_STRUCT_DW_5_stride_mask) << SDMA_PKT_COPY_STRUCT_DW_5_stride_shift) };
}

/*define for linear_sw field*/
pub const SDMA_PKT_COPY_STRUCT_DW_5_linear_sw_offset: u32 = 5;
pub const SDMA_PKT_COPY_STRUCT_DW_5_linear_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_STRUCT_DW_5_linear_sw_shift: u32 = 16;
macro_rules! SDMA_PKT_COPY_STRUCT_DW_5_LINEAR_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_STRUCT_DW_5_linear_sw_mask) << SDMA_PKT_COPY_STRUCT_DW_5_linear_sw_shift) };
}

/*define for linear_cache_policy field*/
pub const SDMA_PKT_COPY_STRUCT_DW_5_linear_cache_policy_offset: u32 = 5;
pub const SDMA_PKT_COPY_STRUCT_DW_5_linear_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_STRUCT_DW_5_linear_cache_policy_shift: u32 = 18;
macro_rules! SDMA_PKT_COPY_STRUCT_DW_5_LINEAR_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_STRUCT_DW_5_linear_cache_policy_mask) << SDMA_PKT_COPY_STRUCT_DW_5_linear_cache_policy_shift) };
}

/*define for struct_sw field*/
pub const SDMA_PKT_COPY_STRUCT_DW_5_struct_sw_offset: u32 = 5;
pub const SDMA_PKT_COPY_STRUCT_DW_5_struct_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_COPY_STRUCT_DW_5_struct_sw_shift: u32 = 24;
macro_rules! SDMA_PKT_COPY_STRUCT_DW_5_STRUCT_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_STRUCT_DW_5_struct_sw_mask) << SDMA_PKT_COPY_STRUCT_DW_5_struct_sw_shift) };
}

/*define for struct_cache_policy field*/
pub const SDMA_PKT_COPY_STRUCT_DW_5_struct_cache_policy_offset: u32 = 5;
pub const SDMA_PKT_COPY_STRUCT_DW_5_struct_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COPY_STRUCT_DW_5_struct_cache_policy_shift: u32 = 26;
macro_rules! SDMA_PKT_COPY_STRUCT_DW_5_STRUCT_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_STRUCT_DW_5_struct_cache_policy_mask) << SDMA_PKT_COPY_STRUCT_DW_5_struct_cache_policy_shift) };
}

/*define for LINEAR_ADDR_LO word*/
/*define for linear_addr_31_0 field*/
pub const SDMA_PKT_COPY_STRUCT_LINEAR_ADDR_LO_linear_addr_31_0_offset: u32 = 6;
pub const SDMA_PKT_COPY_STRUCT_LINEAR_ADDR_LO_linear_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_STRUCT_LINEAR_ADDR_LO_linear_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_STRUCT_LINEAR_ADDR_LO_LINEAR_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_STRUCT_LINEAR_ADDR_LO_linear_addr_31_0_mask) << SDMA_PKT_COPY_STRUCT_LINEAR_ADDR_LO_linear_addr_31_0_shift) };
}

/*define for LINEAR_ADDR_HI word*/
/*define for linear_addr_63_32 field*/
pub const SDMA_PKT_COPY_STRUCT_LINEAR_ADDR_HI_linear_addr_63_32_offset: u32 = 7;
pub const SDMA_PKT_COPY_STRUCT_LINEAR_ADDR_HI_linear_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COPY_STRUCT_LINEAR_ADDR_HI_linear_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COPY_STRUCT_LINEAR_ADDR_HI_LINEAR_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COPY_STRUCT_LINEAR_ADDR_HI_linear_addr_63_32_mask) << SDMA_PKT_COPY_STRUCT_LINEAR_ADDR_HI_linear_addr_63_32_shift) };
}


/*
** Definitions for SDMA_PKT_WRITE_UNTILED packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_WRITE_UNTILED_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_WRITE_UNTILED_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_WRITE_UNTILED_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_UNTILED_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_UNTILED_HEADER_op_mask) << SDMA_PKT_WRITE_UNTILED_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_WRITE_UNTILED_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_WRITE_UNTILED_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_WRITE_UNTILED_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_WRITE_UNTILED_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_UNTILED_HEADER_sub_op_mask) << SDMA_PKT_WRITE_UNTILED_HEADER_sub_op_shift) };
}

/*define for encrypt field*/
pub const SDMA_PKT_WRITE_UNTILED_HEADER_encrypt_offset: u32 = 0;
pub const SDMA_PKT_WRITE_UNTILED_HEADER_encrypt_mask: u32 = 0x00000001;
pub const SDMA_PKT_WRITE_UNTILED_HEADER_encrypt_shift: u32 = 16;
macro_rules! SDMA_PKT_WRITE_UNTILED_HEADER_ENCRYPT {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_UNTILED_HEADER_encrypt_mask) << SDMA_PKT_WRITE_UNTILED_HEADER_encrypt_shift) };
}

/*define for tmz field*/
pub const SDMA_PKT_WRITE_UNTILED_HEADER_tmz_offset: u32 = 0;
pub const SDMA_PKT_WRITE_UNTILED_HEADER_tmz_mask: u32 = 0x00000001;
pub const SDMA_PKT_WRITE_UNTILED_HEADER_tmz_shift: u32 = 18;
macro_rules! SDMA_PKT_WRITE_UNTILED_HEADER_TMZ {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_UNTILED_HEADER_tmz_mask) << SDMA_PKT_WRITE_UNTILED_HEADER_tmz_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_WRITE_UNTILED_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_WRITE_UNTILED_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_WRITE_UNTILED_HEADER_cpv_shift: u32 = 28;
macro_rules! SDMA_PKT_WRITE_UNTILED_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_UNTILED_HEADER_cpv_mask) << SDMA_PKT_WRITE_UNTILED_HEADER_cpv_shift) };
}

/*define for DST_ADDR_LO word*/
/*define for dst_addr_31_0 field*/
pub const SDMA_PKT_WRITE_UNTILED_DST_ADDR_LO_dst_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_WRITE_UNTILED_DST_ADDR_LO_dst_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_UNTILED_DST_ADDR_LO_dst_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_UNTILED_DST_ADDR_LO_DST_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_UNTILED_DST_ADDR_LO_dst_addr_31_0_mask) << SDMA_PKT_WRITE_UNTILED_DST_ADDR_LO_dst_addr_31_0_shift) };
}

/*define for DST_ADDR_HI word*/
/*define for dst_addr_63_32 field*/
pub const SDMA_PKT_WRITE_UNTILED_DST_ADDR_HI_dst_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_WRITE_UNTILED_DST_ADDR_HI_dst_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_UNTILED_DST_ADDR_HI_dst_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_UNTILED_DST_ADDR_HI_DST_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_UNTILED_DST_ADDR_HI_dst_addr_63_32_mask) << SDMA_PKT_WRITE_UNTILED_DST_ADDR_HI_dst_addr_63_32_shift) };
}

/*define for DW_3 word*/
/*define for count field*/
pub const SDMA_PKT_WRITE_UNTILED_DW_3_count_offset: u32 = 3;
pub const SDMA_PKT_WRITE_UNTILED_DW_3_count_mask: u32 = 0x000FFFFF;
pub const SDMA_PKT_WRITE_UNTILED_DW_3_count_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_UNTILED_DW_3_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_UNTILED_DW_3_count_mask) << SDMA_PKT_WRITE_UNTILED_DW_3_count_shift) };
}

/*define for sw field*/
pub const SDMA_PKT_WRITE_UNTILED_DW_3_sw_offset: u32 = 3;
pub const SDMA_PKT_WRITE_UNTILED_DW_3_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_WRITE_UNTILED_DW_3_sw_shift: u32 = 24;
macro_rules! SDMA_PKT_WRITE_UNTILED_DW_3_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_UNTILED_DW_3_sw_mask) << SDMA_PKT_WRITE_UNTILED_DW_3_sw_shift) };
}

/*define for cache_policy field*/
pub const SDMA_PKT_WRITE_UNTILED_DW_3_cache_policy_offset: u32 = 3;
pub const SDMA_PKT_WRITE_UNTILED_DW_3_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_WRITE_UNTILED_DW_3_cache_policy_shift: u32 = 26;
macro_rules! SDMA_PKT_WRITE_UNTILED_DW_3_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_UNTILED_DW_3_cache_policy_mask) << SDMA_PKT_WRITE_UNTILED_DW_3_cache_policy_shift) };
}

/*define for DATA0 word*/
/*define for data0 field*/
pub const SDMA_PKT_WRITE_UNTILED_DATA0_data0_offset: u32 = 4;
pub const SDMA_PKT_WRITE_UNTILED_DATA0_data0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_UNTILED_DATA0_data0_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_UNTILED_DATA0_DATA0 {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_UNTILED_DATA0_data0_mask) << SDMA_PKT_WRITE_UNTILED_DATA0_data0_shift) };
}


/*
** Definitions for SDMA_PKT_WRITE_TILED packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_WRITE_TILED_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_WRITE_TILED_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_WRITE_TILED_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_TILED_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_HEADER_op_mask) << SDMA_PKT_WRITE_TILED_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_WRITE_TILED_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_WRITE_TILED_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_WRITE_TILED_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_WRITE_TILED_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_HEADER_sub_op_mask) << SDMA_PKT_WRITE_TILED_HEADER_sub_op_shift) };
}

/*define for encrypt field*/
pub const SDMA_PKT_WRITE_TILED_HEADER_encrypt_offset: u32 = 0;
pub const SDMA_PKT_WRITE_TILED_HEADER_encrypt_mask: u32 = 0x00000001;
pub const SDMA_PKT_WRITE_TILED_HEADER_encrypt_shift: u32 = 16;
macro_rules! SDMA_PKT_WRITE_TILED_HEADER_ENCRYPT {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_HEADER_encrypt_mask) << SDMA_PKT_WRITE_TILED_HEADER_encrypt_shift) };
}

/*define for tmz field*/
pub const SDMA_PKT_WRITE_TILED_HEADER_tmz_offset: u32 = 0;
pub const SDMA_PKT_WRITE_TILED_HEADER_tmz_mask: u32 = 0x00000001;
pub const SDMA_PKT_WRITE_TILED_HEADER_tmz_shift: u32 = 18;
macro_rules! SDMA_PKT_WRITE_TILED_HEADER_TMZ {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_HEADER_tmz_mask) << SDMA_PKT_WRITE_TILED_HEADER_tmz_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_WRITE_TILED_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_WRITE_TILED_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_WRITE_TILED_HEADER_cpv_shift: u32 = 28;
macro_rules! SDMA_PKT_WRITE_TILED_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_HEADER_cpv_mask) << SDMA_PKT_WRITE_TILED_HEADER_cpv_shift) };
}

/*define for DST_ADDR_LO word*/
/*define for dst_addr_31_0 field*/
pub const SDMA_PKT_WRITE_TILED_DST_ADDR_LO_dst_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_WRITE_TILED_DST_ADDR_LO_dst_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_TILED_DST_ADDR_LO_dst_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_TILED_DST_ADDR_LO_DST_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_DST_ADDR_LO_dst_addr_31_0_mask) << SDMA_PKT_WRITE_TILED_DST_ADDR_LO_dst_addr_31_0_shift) };
}

/*define for DST_ADDR_HI word*/
/*define for dst_addr_63_32 field*/
pub const SDMA_PKT_WRITE_TILED_DST_ADDR_HI_dst_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_WRITE_TILED_DST_ADDR_HI_dst_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_TILED_DST_ADDR_HI_dst_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_TILED_DST_ADDR_HI_DST_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_DST_ADDR_HI_dst_addr_63_32_mask) << SDMA_PKT_WRITE_TILED_DST_ADDR_HI_dst_addr_63_32_shift) };
}

/*define for DW_3 word*/
/*define for width field*/
pub const SDMA_PKT_WRITE_TILED_DW_3_width_offset: u32 = 3;
pub const SDMA_PKT_WRITE_TILED_DW_3_width_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_WRITE_TILED_DW_3_width_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_TILED_DW_3_WIDTH {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_DW_3_width_mask) << SDMA_PKT_WRITE_TILED_DW_3_width_shift) };
}

/*define for DW_4 word*/
/*define for height field*/
pub const SDMA_PKT_WRITE_TILED_DW_4_height_offset: u32 = 4;
pub const SDMA_PKT_WRITE_TILED_DW_4_height_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_WRITE_TILED_DW_4_height_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_TILED_DW_4_HEIGHT {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_DW_4_height_mask) << SDMA_PKT_WRITE_TILED_DW_4_height_shift) };
}

/*define for depth field*/
pub const SDMA_PKT_WRITE_TILED_DW_4_depth_offset: u32 = 4;
pub const SDMA_PKT_WRITE_TILED_DW_4_depth_mask: u32 = 0x00001FFF;
pub const SDMA_PKT_WRITE_TILED_DW_4_depth_shift: u32 = 16;
macro_rules! SDMA_PKT_WRITE_TILED_DW_4_DEPTH {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_DW_4_depth_mask) << SDMA_PKT_WRITE_TILED_DW_4_depth_shift) };
}

/*define for DW_5 word*/
/*define for element_size field*/
pub const SDMA_PKT_WRITE_TILED_DW_5_element_size_offset: u32 = 5;
pub const SDMA_PKT_WRITE_TILED_DW_5_element_size_mask: u32 = 0x00000007;
pub const SDMA_PKT_WRITE_TILED_DW_5_element_size_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_TILED_DW_5_ELEMENT_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_DW_5_element_size_mask) << SDMA_PKT_WRITE_TILED_DW_5_element_size_shift) };
}

/*define for swizzle_mode field*/
pub const SDMA_PKT_WRITE_TILED_DW_5_swizzle_mode_offset: u32 = 5;
pub const SDMA_PKT_WRITE_TILED_DW_5_swizzle_mode_mask: u32 = 0x0000001F;
pub const SDMA_PKT_WRITE_TILED_DW_5_swizzle_mode_shift: u32 = 3;
macro_rules! SDMA_PKT_WRITE_TILED_DW_5_SWIZZLE_MODE {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_DW_5_swizzle_mode_mask) << SDMA_PKT_WRITE_TILED_DW_5_swizzle_mode_shift) };
}

/*define for dimension field*/
pub const SDMA_PKT_WRITE_TILED_DW_5_dimension_offset: u32 = 5;
pub const SDMA_PKT_WRITE_TILED_DW_5_dimension_mask: u32 = 0x00000003;
pub const SDMA_PKT_WRITE_TILED_DW_5_dimension_shift: u32 = 9;
macro_rules! SDMA_PKT_WRITE_TILED_DW_5_DIMENSION {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_DW_5_dimension_mask) << SDMA_PKT_WRITE_TILED_DW_5_dimension_shift) };
}

/*define for mip_max field*/
pub const SDMA_PKT_WRITE_TILED_DW_5_mip_max_offset: u32 = 5;
pub const SDMA_PKT_WRITE_TILED_DW_5_mip_max_mask: u32 = 0x0000000F;
pub const SDMA_PKT_WRITE_TILED_DW_5_mip_max_shift: u32 = 16;
macro_rules! SDMA_PKT_WRITE_TILED_DW_5_MIP_MAX {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_DW_5_mip_ma$x_mask) << SDMA_PKT_WRITE_TILED_DW_5_mip_ma$x_shift) };
}

/*define for DW_6 word*/
/*define for x field*/
pub const SDMA_PKT_WRITE_TILED_DW_6_x_offset: u32 = 6;
pub const SDMA_PKT_WRITE_TILED_DW_6_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_WRITE_TILED_DW_6_x_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_TILED_DW_6_X {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_DW_6_$x_mask) << SDMA_PKT_WRITE_TILED_DW_6_$x_shift) };
}

/*define for y field*/
pub const SDMA_PKT_WRITE_TILED_DW_6_y_offset: u32 = 6;
pub const SDMA_PKT_WRITE_TILED_DW_6_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_WRITE_TILED_DW_6_y_shift: u32 = 16;
macro_rules! SDMA_PKT_WRITE_TILED_DW_6_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_DW_6_y_mask) << SDMA_PKT_WRITE_TILED_DW_6_y_shift) };
}

/*define for DW_7 word*/
/*define for z field*/
pub const SDMA_PKT_WRITE_TILED_DW_7_z_offset: u32 = 7;
pub const SDMA_PKT_WRITE_TILED_DW_7_z_mask: u32 = 0x00001FFF;
pub const SDMA_PKT_WRITE_TILED_DW_7_z_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_TILED_DW_7_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_DW_7_z_mask) << SDMA_PKT_WRITE_TILED_DW_7_z_shift) };
}

/*define for sw field*/
pub const SDMA_PKT_WRITE_TILED_DW_7_sw_offset: u32 = 7;
pub const SDMA_PKT_WRITE_TILED_DW_7_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_WRITE_TILED_DW_7_sw_shift: u32 = 24;
macro_rules! SDMA_PKT_WRITE_TILED_DW_7_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_DW_7_sw_mask) << SDMA_PKT_WRITE_TILED_DW_7_sw_shift) };
}

/*define for cache_policy field*/
pub const SDMA_PKT_WRITE_TILED_DW_7_cache_policy_offset: u32 = 7;
pub const SDMA_PKT_WRITE_TILED_DW_7_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_WRITE_TILED_DW_7_cache_policy_shift: u32 = 26;
macro_rules! SDMA_PKT_WRITE_TILED_DW_7_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_DW_7_cache_policy_mask) << SDMA_PKT_WRITE_TILED_DW_7_cache_policy_shift) };
}

/*define for COUNT word*/
/*define for count field*/
pub const SDMA_PKT_WRITE_TILED_COUNT_count_offset: u32 = 8;
pub const SDMA_PKT_WRITE_TILED_COUNT_count_mask: u32 = 0x000FFFFF;
pub const SDMA_PKT_WRITE_TILED_COUNT_count_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_TILED_COUNT_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_COUNT_count_mask) << SDMA_PKT_WRITE_TILED_COUNT_count_shift) };
}

/*define for DATA0 word*/
/*define for data0 field*/
pub const SDMA_PKT_WRITE_TILED_DATA0_data0_offset: u32 = 9;
pub const SDMA_PKT_WRITE_TILED_DATA0_data0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_TILED_DATA0_data0_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_TILED_DATA0_DATA0 {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_DATA0_data0_mask) << SDMA_PKT_WRITE_TILED_DATA0_data0_shift) };
}


/*
** Definitions for SDMA_PKT_WRITE_TILED_BC packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_WRITE_TILED_BC_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_WRITE_TILED_BC_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_WRITE_TILED_BC_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_TILED_BC_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_HEADER_op_mask) << SDMA_PKT_WRITE_TILED_BC_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_WRITE_TILED_BC_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_WRITE_TILED_BC_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_WRITE_TILED_BC_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_WRITE_TILED_BC_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_HEADER_sub_op_mask) << SDMA_PKT_WRITE_TILED_BC_HEADER_sub_op_shift) };
}

/*define for DST_ADDR_LO word*/
/*define for dst_addr_31_0 field*/
pub const SDMA_PKT_WRITE_TILED_BC_DST_ADDR_LO_dst_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_WRITE_TILED_BC_DST_ADDR_LO_dst_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_TILED_BC_DST_ADDR_LO_dst_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_TILED_BC_DST_ADDR_LO_DST_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_DST_ADDR_LO_dst_addr_31_0_mask) << SDMA_PKT_WRITE_TILED_BC_DST_ADDR_LO_dst_addr_31_0_shift) };
}

/*define for DST_ADDR_HI word*/
/*define for dst_addr_63_32 field*/
pub const SDMA_PKT_WRITE_TILED_BC_DST_ADDR_HI_dst_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_WRITE_TILED_BC_DST_ADDR_HI_dst_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_TILED_BC_DST_ADDR_HI_dst_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_TILED_BC_DST_ADDR_HI_DST_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_DST_ADDR_HI_dst_addr_63_32_mask) << SDMA_PKT_WRITE_TILED_BC_DST_ADDR_HI_dst_addr_63_32_shift) };
}

/*define for DW_3 word*/
/*define for width field*/
pub const SDMA_PKT_WRITE_TILED_BC_DW_3_width_offset: u32 = 3;
pub const SDMA_PKT_WRITE_TILED_BC_DW_3_width_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_WRITE_TILED_BC_DW_3_width_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_TILED_BC_DW_3_WIDTH {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_DW_3_width_mask) << SDMA_PKT_WRITE_TILED_BC_DW_3_width_shift) };
}

/*define for DW_4 word*/
/*define for height field*/
pub const SDMA_PKT_WRITE_TILED_BC_DW_4_height_offset: u32 = 4;
pub const SDMA_PKT_WRITE_TILED_BC_DW_4_height_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_WRITE_TILED_BC_DW_4_height_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_TILED_BC_DW_4_HEIGHT {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_DW_4_height_mask) << SDMA_PKT_WRITE_TILED_BC_DW_4_height_shift) };
}

/*define for depth field*/
pub const SDMA_PKT_WRITE_TILED_BC_DW_4_depth_offset: u32 = 4;
pub const SDMA_PKT_WRITE_TILED_BC_DW_4_depth_mask: u32 = 0x000007FF;
pub const SDMA_PKT_WRITE_TILED_BC_DW_4_depth_shift: u32 = 16;
macro_rules! SDMA_PKT_WRITE_TILED_BC_DW_4_DEPTH {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_DW_4_depth_mask) << SDMA_PKT_WRITE_TILED_BC_DW_4_depth_shift) };
}

/*define for DW_5 word*/
/*define for element_size field*/
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_element_size_offset: u32 = 5;
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_element_size_mask: u32 = 0x00000007;
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_element_size_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_TILED_BC_DW_5_ELEMENT_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_DW_5_element_size_mask) << SDMA_PKT_WRITE_TILED_BC_DW_5_element_size_shift) };
}

/*define for array_mode field*/
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_array_mode_offset: u32 = 5;
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_array_mode_mask: u32 = 0x0000000F;
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_array_mode_shift: u32 = 3;
macro_rules! SDMA_PKT_WRITE_TILED_BC_DW_5_ARRAY_MODE {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_DW_5_array_mode_mask) << SDMA_PKT_WRITE_TILED_BC_DW_5_array_mode_shift) };
}

/*define for mit_mode field*/
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_mit_mode_offset: u32 = 5;
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_mit_mode_mask: u32 = 0x00000007;
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_mit_mode_shift: u32 = 8;
macro_rules! SDMA_PKT_WRITE_TILED_BC_DW_5_MIT_MODE {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_DW_5_mit_mode_mask) << SDMA_PKT_WRITE_TILED_BC_DW_5_mit_mode_shift) };
}

/*define for tilesplit_size field*/
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_tilesplit_size_offset: u32 = 5;
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_tilesplit_size_mask: u32 = 0x00000007;
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_tilesplit_size_shift: u32 = 11;
macro_rules! SDMA_PKT_WRITE_TILED_BC_DW_5_TILESPLIT_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_DW_5_tilesplit_size_mask) << SDMA_PKT_WRITE_TILED_BC_DW_5_tilesplit_size_shift) };
}

/*define for bank_w field*/
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_bank_w_offset: u32 = 5;
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_bank_w_mask: u32 = 0x00000003;
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_bank_w_shift: u32 = 15;
macro_rules! SDMA_PKT_WRITE_TILED_BC_DW_5_BANK_W {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_DW_5_bank_w_mask) << SDMA_PKT_WRITE_TILED_BC_DW_5_bank_w_shift) };
}

/*define for bank_h field*/
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_bank_h_offset: u32 = 5;
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_bank_h_mask: u32 = 0x00000003;
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_bank_h_shift: u32 = 18;
macro_rules! SDMA_PKT_WRITE_TILED_BC_DW_5_BANK_H {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_DW_5_bank_h_mask) << SDMA_PKT_WRITE_TILED_BC_DW_5_bank_h_shift) };
}

/*define for num_bank field*/
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_num_bank_offset: u32 = 5;
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_num_bank_mask: u32 = 0x00000003;
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_num_bank_shift: u32 = 21;
macro_rules! SDMA_PKT_WRITE_TILED_BC_DW_5_NUM_BANK {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_DW_5_num_bank_mask) << SDMA_PKT_WRITE_TILED_BC_DW_5_num_bank_shift) };
}

/*define for mat_aspt field*/
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_mat_aspt_offset: u32 = 5;
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_mat_aspt_mask: u32 = 0x00000003;
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_mat_aspt_shift: u32 = 24;
macro_rules! SDMA_PKT_WRITE_TILED_BC_DW_5_MAT_ASPT {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_DW_5_mat_aspt_mask) << SDMA_PKT_WRITE_TILED_BC_DW_5_mat_aspt_shift) };
}

/*define for pipe_config field*/
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_pipe_config_offset: u32 = 5;
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_pipe_config_mask: u32 = 0x0000001F;
pub const SDMA_PKT_WRITE_TILED_BC_DW_5_pipe_config_shift: u32 = 26;
macro_rules! SDMA_PKT_WRITE_TILED_BC_DW_5_PIPE_CONFIG {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_DW_5_pipe_config_mask) << SDMA_PKT_WRITE_TILED_BC_DW_5_pipe_config_shift) };
}

/*define for DW_6 word*/
/*define for x field*/
pub const SDMA_PKT_WRITE_TILED_BC_DW_6_x_offset: u32 = 6;
pub const SDMA_PKT_WRITE_TILED_BC_DW_6_x_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_WRITE_TILED_BC_DW_6_x_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_TILED_BC_DW_6_X {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_DW_6_$x_mask) << SDMA_PKT_WRITE_TILED_BC_DW_6_$x_shift) };
}

/*define for y field*/
pub const SDMA_PKT_WRITE_TILED_BC_DW_6_y_offset: u32 = 6;
pub const SDMA_PKT_WRITE_TILED_BC_DW_6_y_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_WRITE_TILED_BC_DW_6_y_shift: u32 = 16;
macro_rules! SDMA_PKT_WRITE_TILED_BC_DW_6_Y {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_DW_6_y_mask) << SDMA_PKT_WRITE_TILED_BC_DW_6_y_shift) };
}

/*define for DW_7 word*/
/*define for z field*/
pub const SDMA_PKT_WRITE_TILED_BC_DW_7_z_offset: u32 = 7;
pub const SDMA_PKT_WRITE_TILED_BC_DW_7_z_mask: u32 = 0x000007FF;
pub const SDMA_PKT_WRITE_TILED_BC_DW_7_z_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_TILED_BC_DW_7_Z {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_DW_7_z_mask) << SDMA_PKT_WRITE_TILED_BC_DW_7_z_shift) };
}

/*define for sw field*/
pub const SDMA_PKT_WRITE_TILED_BC_DW_7_sw_offset: u32 = 7;
pub const SDMA_PKT_WRITE_TILED_BC_DW_7_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_WRITE_TILED_BC_DW_7_sw_shift: u32 = 24;
macro_rules! SDMA_PKT_WRITE_TILED_BC_DW_7_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_DW_7_sw_mask) << SDMA_PKT_WRITE_TILED_BC_DW_7_sw_shift) };
}

/*define for COUNT word*/
/*define for count field*/
pub const SDMA_PKT_WRITE_TILED_BC_COUNT_count_offset: u32 = 8;
pub const SDMA_PKT_WRITE_TILED_BC_COUNT_count_mask: u32 = 0x000FFFFF;
pub const SDMA_PKT_WRITE_TILED_BC_COUNT_count_shift: u32 = 2;
macro_rules! SDMA_PKT_WRITE_TILED_BC_COUNT_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_COUNT_count_mask) << SDMA_PKT_WRITE_TILED_BC_COUNT_count_shift) };
}

/*define for DATA0 word*/
/*define for data0 field*/
pub const SDMA_PKT_WRITE_TILED_BC_DATA0_data0_offset: u32 = 9;
pub const SDMA_PKT_WRITE_TILED_BC_DATA0_data0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_TILED_BC_DATA0_data0_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_TILED_BC_DATA0_DATA0 {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_TILED_BC_DATA0_data0_mask) << SDMA_PKT_WRITE_TILED_BC_DATA0_data0_shift) };
}


/*
** Definitions for SDMA_PKT_PTEPDE_COPY packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_PTEPDE_COPY_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_PTEPDE_COPY_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_PTEPDE_COPY_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_COPY_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_HEADER_op_mask) << SDMA_PKT_PTEPDE_COPY_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_PTEPDE_COPY_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_PTEPDE_COPY_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_PTEPDE_COPY_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_PTEPDE_COPY_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_HEADER_sub_op_mask) << SDMA_PKT_PTEPDE_COPY_HEADER_sub_op_shift) };
}

/*define for mtype field*/
pub const SDMA_PKT_PTEPDE_COPY_HEADER_mtype_offset: u32 = 0;
pub const SDMA_PKT_PTEPDE_COPY_HEADER_mtype_mask: u32 = 0x00000003;
pub const SDMA_PKT_PTEPDE_COPY_HEADER_mtype_shift: u32 = 16;
macro_rules! SDMA_PKT_PTEPDE_COPY_HEADER_MTYPE {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_HEADER_mtype_mask) << SDMA_PKT_PTEPDE_COPY_HEADER_mtype_shift) };
}

/*define for snoop field*/
pub const SDMA_PKT_PTEPDE_COPY_HEADER_snoop_offset: u32 = 0;
pub const SDMA_PKT_PTEPDE_COPY_HEADER_snoop_mask: u32 = 0x00000001;
pub const SDMA_PKT_PTEPDE_COPY_HEADER_snoop_shift: u32 = 22;
macro_rules! SDMA_PKT_PTEPDE_COPY_HEADER_SNOOP {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_HEADER_snoop_mask) << SDMA_PKT_PTEPDE_COPY_HEADER_snoop_shift) };
}

/*define for scope field*/
pub const SDMA_PKT_PTEPDE_COPY_HEADER_scope_offset: u32 = 0;
pub const SDMA_PKT_PTEPDE_COPY_HEADER_scope_mask: u32 = 0x00000003;
pub const SDMA_PKT_PTEPDE_COPY_HEADER_scope_shift: u32 = 24;
macro_rules! SDMA_PKT_PTEPDE_COPY_HEADER_SCOPE {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_HEADER_scope_mask) << SDMA_PKT_PTEPDE_COPY_HEADER_scope_shift) };
}

/*define for SRC_ADDR_LO word*/
/*define for src_addr_31_0 field*/
pub const SDMA_PKT_PTEPDE_COPY_SRC_ADDR_LO_src_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_PTEPDE_COPY_SRC_ADDR_LO_src_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_COPY_SRC_ADDR_LO_src_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_COPY_SRC_ADDR_LO_SRC_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_SRC_ADDR_LO_src_addr_31_0_mask) << SDMA_PKT_PTEPDE_COPY_SRC_ADDR_LO_src_addr_31_0_shift) };
}

/*define for SRC_ADDR_HI word*/
/*define for src_addr_63_32 field*/
pub const SDMA_PKT_PTEPDE_COPY_SRC_ADDR_HI_src_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_PTEPDE_COPY_SRC_ADDR_HI_src_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_COPY_SRC_ADDR_HI_src_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_COPY_SRC_ADDR_HI_SRC_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_SRC_ADDR_HI_src_addr_63_32_mask) << SDMA_PKT_PTEPDE_COPY_SRC_ADDR_HI_src_addr_63_32_shift) };
}

/*define for DST_ADDR_LO word*/
/*define for dst_addr_31_0 field*/
pub const SDMA_PKT_PTEPDE_COPY_DST_ADDR_LO_dst_addr_31_0_offset: u32 = 3;
pub const SDMA_PKT_PTEPDE_COPY_DST_ADDR_LO_dst_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_COPY_DST_ADDR_LO_dst_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_COPY_DST_ADDR_LO_DST_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_DST_ADDR_LO_dst_addr_31_0_mask) << SDMA_PKT_PTEPDE_COPY_DST_ADDR_LO_dst_addr_31_0_shift) };
}

/*define for DST_ADDR_HI word*/
/*define for dst_addr_63_32 field*/
pub const SDMA_PKT_PTEPDE_COPY_DST_ADDR_HI_dst_addr_63_32_offset: u32 = 4;
pub const SDMA_PKT_PTEPDE_COPY_DST_ADDR_HI_dst_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_COPY_DST_ADDR_HI_dst_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_COPY_DST_ADDR_HI_DST_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_DST_ADDR_HI_dst_addr_63_32_mask) << SDMA_PKT_PTEPDE_COPY_DST_ADDR_HI_dst_addr_63_32_shift) };
}

/*define for MASK_DW0 word*/
/*define for mask_dw0 field*/
pub const SDMA_PKT_PTEPDE_COPY_MASK_DW0_mask_dw0_offset: u32 = 5;
pub const SDMA_PKT_PTEPDE_COPY_MASK_DW0_mask_dw0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_COPY_MASK_DW0_mask_dw0_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_COPY_MASK_DW0_MASK_DW0 {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_MASK_DW0_mask_dw0_mask) << SDMA_PKT_PTEPDE_COPY_MASK_DW0_mask_dw0_shift) };
}

/*define for MASK_DW1 word*/
/*define for mask_dw1 field*/
pub const SDMA_PKT_PTEPDE_COPY_MASK_DW1_mask_dw1_offset: u32 = 6;
pub const SDMA_PKT_PTEPDE_COPY_MASK_DW1_mask_dw1_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_COPY_MASK_DW1_mask_dw1_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_COPY_MASK_DW1_MASK_DW1 {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_MASK_DW1_mask_dw1_mask) << SDMA_PKT_PTEPDE_COPY_MASK_DW1_mask_dw1_shift) };
}

/*define for COUNT word*/
/*define for count field*/
pub const SDMA_PKT_PTEPDE_COPY_COUNT_count_offset: u32 = 7;
pub const SDMA_PKT_PTEPDE_COPY_COUNT_count_mask: u32 = 0x0007FFFF;
pub const SDMA_PKT_PTEPDE_COPY_COUNT_count_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_COPY_COUNT_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_COUNT_count_mask) << SDMA_PKT_PTEPDE_COPY_COUNT_count_shift) };
}

/*define for dst_cache_policy field*/
pub const SDMA_PKT_PTEPDE_COPY_COUNT_dst_cache_policy_offset: u32 = 7;
pub const SDMA_PKT_PTEPDE_COPY_COUNT_dst_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_PTEPDE_COPY_COUNT_dst_cache_policy_shift: u32 = 22;
macro_rules! SDMA_PKT_PTEPDE_COPY_COUNT_DST_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_COUNT_dst_cache_policy_mask) << SDMA_PKT_PTEPDE_COPY_COUNT_dst_cache_policy_shift) };
}

/*define for src_cache_policy field*/
pub const SDMA_PKT_PTEPDE_COPY_COUNT_src_cache_policy_offset: u32 = 7;
pub const SDMA_PKT_PTEPDE_COPY_COUNT_src_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_PTEPDE_COPY_COUNT_src_cache_policy_shift: u32 = 29;
macro_rules! SDMA_PKT_PTEPDE_COPY_COUNT_SRC_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_COUNT_src_cache_policy_mask) << SDMA_PKT_PTEPDE_COPY_COUNT_src_cache_policy_shift) };
}


/*
** Definitions for SDMA_PKT_PTEPDE_COPY_BACKWARDS packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_op_mask) << SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_sub_op_mask) << SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_sub_op_shift) };
}

/*define for pte_size field*/
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_pte_size_offset: u32 = 0;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_pte_size_mask: u32 = 0x00000003;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_pte_size_shift: u32 = 28;
macro_rules! SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_PTE_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_pte_size_mask) << SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_pte_size_shift) };
}

/*define for direction field*/
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_direction_offset: u32 = 0;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_direction_mask: u32 = 0x00000001;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_direction_shift: u32 = 30;
macro_rules! SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_DIRECTION {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_direction_mask) << SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_direction_shift) };
}

/*define for ptepde_op field*/
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_ptepde_op_offset: u32 = 0;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_ptepde_op_mask: u32 = 0x00000001;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_ptepde_op_shift: u32 = 31;
macro_rules! SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_PTEPDE_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_ptepde_op_mask) << SDMA_PKT_PTEPDE_COPY_BACKWARDS_HEADER_ptepde_op_shift) };
}

/*define for SRC_ADDR_LO word*/
/*define for src_addr_31_0 field*/
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_SRC_ADDR_LO_src_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_SRC_ADDR_LO_src_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_SRC_ADDR_LO_src_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_COPY_BACKWARDS_SRC_ADDR_LO_SRC_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_BACKWARDS_SRC_ADDR_LO_src_addr_31_0_mask) << SDMA_PKT_PTEPDE_COPY_BACKWARDS_SRC_ADDR_LO_src_addr_31_0_shift) };
}

/*define for SRC_ADDR_HI word*/
/*define for src_addr_63_32 field*/
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_SRC_ADDR_HI_src_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_SRC_ADDR_HI_src_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_SRC_ADDR_HI_src_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_COPY_BACKWARDS_SRC_ADDR_HI_SRC_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_BACKWARDS_SRC_ADDR_HI_src_addr_63_32_mask) << SDMA_PKT_PTEPDE_COPY_BACKWARDS_SRC_ADDR_HI_src_addr_63_32_shift) };
}

/*define for DST_ADDR_LO word*/
/*define for dst_addr_31_0 field*/
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_DST_ADDR_LO_dst_addr_31_0_offset: u32 = 3;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_DST_ADDR_LO_dst_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_DST_ADDR_LO_dst_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_COPY_BACKWARDS_DST_ADDR_LO_DST_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_BACKWARDS_DST_ADDR_LO_dst_addr_31_0_mask) << SDMA_PKT_PTEPDE_COPY_BACKWARDS_DST_ADDR_LO_dst_addr_31_0_shift) };
}

/*define for DST_ADDR_HI word*/
/*define for dst_addr_63_32 field*/
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_DST_ADDR_HI_dst_addr_63_32_offset: u32 = 4;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_DST_ADDR_HI_dst_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_DST_ADDR_HI_dst_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_COPY_BACKWARDS_DST_ADDR_HI_DST_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_BACKWARDS_DST_ADDR_HI_dst_addr_63_32_mask) << SDMA_PKT_PTEPDE_COPY_BACKWARDS_DST_ADDR_HI_dst_addr_63_32_shift) };
}

/*define for MASK_BIT_FOR_DW word*/
/*define for mask_first_xfer field*/
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_MASK_BIT_FOR_DW_mask_first_xfer_offset: u32 = 5;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_MASK_BIT_FOR_DW_mask_first_xfer_mask: u32 = 0x000000FF;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_MASK_BIT_FOR_DW_mask_first_xfer_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_COPY_BACKWARDS_MASK_BIT_FOR_DW_MASK_FIRST_XFER {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_BACKWARDS_MASK_BIT_FOR_DW_mask_first_$xfer_mask) << SDMA_PKT_PTEPDE_COPY_BACKWARDS_MASK_BIT_FOR_DW_mask_first_$xfer_shift) };
}

/*define for mask_last_xfer field*/
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_MASK_BIT_FOR_DW_mask_last_xfer_offset: u32 = 5;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_MASK_BIT_FOR_DW_mask_last_xfer_mask: u32 = 0x000000FF;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_MASK_BIT_FOR_DW_mask_last_xfer_shift: u32 = 8;
macro_rules! SDMA_PKT_PTEPDE_COPY_BACKWARDS_MASK_BIT_FOR_DW_MASK_LAST_XFER {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_BACKWARDS_MASK_BIT_FOR_DW_mask_last_$xfer_mask) << SDMA_PKT_PTEPDE_COPY_BACKWARDS_MASK_BIT_FOR_DW_mask_last_$xfer_shift) };
}

/*define for COUNT_IN_32B_XFER word*/
/*define for count field*/
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_COUNT_IN_32B_XFER_count_offset: u32 = 6;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_COUNT_IN_32B_XFER_count_mask: u32 = 0x0001FFFF;
pub const SDMA_PKT_PTEPDE_COPY_BACKWARDS_COUNT_IN_32B_XFER_count_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_COPY_BACKWARDS_COUNT_IN_32B_XFER_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_COPY_BACKWARDS_COUNT_IN_32B_XFER_count_mask) << SDMA_PKT_PTEPDE_COPY_BACKWARDS_COUNT_IN_32B_XFER_count_shift) };
}


/*
** Definitions for SDMA_PKT_PTEPDE_RMW packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_PTEPDE_RMW_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_RMW_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_RMW_HEADER_op_mask) << SDMA_PKT_PTEPDE_RMW_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_PTEPDE_RMW_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_PTEPDE_RMW_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_RMW_HEADER_sub_op_mask) << SDMA_PKT_PTEPDE_RMW_HEADER_sub_op_shift) };
}

/*define for mtype field*/
pub const SDMA_PKT_PTEPDE_RMW_HEADER_mtype_offset: u32 = 0;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_mtype_mask: u32 = 0x00000007;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_mtype_shift: u32 = 16;
macro_rules! SDMA_PKT_PTEPDE_RMW_HEADER_MTYPE {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_RMW_HEADER_mtype_mask) << SDMA_PKT_PTEPDE_RMW_HEADER_mtype_shift) };
}

/*define for gcc field*/
pub const SDMA_PKT_PTEPDE_RMW_HEADER_gcc_offset: u32 = 0;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_gcc_mask: u32 = 0x00000001;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_gcc_shift: u32 = 19;
macro_rules! SDMA_PKT_PTEPDE_RMW_HEADER_GCC {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_RMW_HEADER_gcc_mask) << SDMA_PKT_PTEPDE_RMW_HEADER_gcc_shift) };
}

/*define for sys field*/
pub const SDMA_PKT_PTEPDE_RMW_HEADER_sys_offset: u32 = 0;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_sys_mask: u32 = 0x00000001;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_sys_shift: u32 = 20;
macro_rules! SDMA_PKT_PTEPDE_RMW_HEADER_SYS {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_RMW_HEADER_sys_mask) << SDMA_PKT_PTEPDE_RMW_HEADER_sys_shift) };
}

/*define for snp field*/
pub const SDMA_PKT_PTEPDE_RMW_HEADER_snp_offset: u32 = 0;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_snp_mask: u32 = 0x00000001;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_snp_shift: u32 = 22;
macro_rules! SDMA_PKT_PTEPDE_RMW_HEADER_SNP {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_RMW_HEADER_snp_mask) << SDMA_PKT_PTEPDE_RMW_HEADER_snp_shift) };
}

/*define for gpa field*/
pub const SDMA_PKT_PTEPDE_RMW_HEADER_gpa_offset: u32 = 0;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_gpa_mask: u32 = 0x00000001;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_gpa_shift: u32 = 23;
macro_rules! SDMA_PKT_PTEPDE_RMW_HEADER_GPA {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_RMW_HEADER_gpa_mask) << SDMA_PKT_PTEPDE_RMW_HEADER_gpa_shift) };
}

/*define for l2_policy field*/
pub const SDMA_PKT_PTEPDE_RMW_HEADER_l2_policy_offset: u32 = 0;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_l2_policy_mask: u32 = 0x00000003;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_l2_policy_shift: u32 = 24;
macro_rules! SDMA_PKT_PTEPDE_RMW_HEADER_L2_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_RMW_HEADER_l2_policy_mask) << SDMA_PKT_PTEPDE_RMW_HEADER_l2_policy_shift) };
}

/*define for llc_policy field*/
pub const SDMA_PKT_PTEPDE_RMW_HEADER_llc_policy_offset: u32 = 0;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_llc_policy_mask: u32 = 0x00000001;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_llc_policy_shift: u32 = 26;
macro_rules! SDMA_PKT_PTEPDE_RMW_HEADER_LLC_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_RMW_HEADER_llc_policy_mask) << SDMA_PKT_PTEPDE_RMW_HEADER_llc_policy_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_PTEPDE_RMW_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_PTEPDE_RMW_HEADER_cpv_shift: u32 = 28;
macro_rules! SDMA_PKT_PTEPDE_RMW_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_RMW_HEADER_cpv_mask) << SDMA_PKT_PTEPDE_RMW_HEADER_cpv_shift) };
}

/*define for ADDR_LO word*/
/*define for addr_31_0 field*/
pub const SDMA_PKT_PTEPDE_RMW_ADDR_LO_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_PTEPDE_RMW_ADDR_LO_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_RMW_ADDR_LO_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_RMW_ADDR_LO_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_RMW_ADDR_LO_addr_31_0_mask) << SDMA_PKT_PTEPDE_RMW_ADDR_LO_addr_31_0_shift) };
}

/*define for ADDR_HI word*/
/*define for addr_63_32 field*/
pub const SDMA_PKT_PTEPDE_RMW_ADDR_HI_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_PTEPDE_RMW_ADDR_HI_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_RMW_ADDR_HI_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_RMW_ADDR_HI_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_RMW_ADDR_HI_addr_63_32_mask) << SDMA_PKT_PTEPDE_RMW_ADDR_HI_addr_63_32_shift) };
}

/*define for MASK_LO word*/
/*define for mask_31_0 field*/
pub const SDMA_PKT_PTEPDE_RMW_MASK_LO_mask_31_0_offset: u32 = 3;
pub const SDMA_PKT_PTEPDE_RMW_MASK_LO_mask_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_RMW_MASK_LO_mask_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_RMW_MASK_LO_MASK_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_RMW_MASK_LO_mask_31_0_mask) << SDMA_PKT_PTEPDE_RMW_MASK_LO_mask_31_0_shift) };
}

/*define for MASK_HI word*/
/*define for mask_63_32 field*/
pub const SDMA_PKT_PTEPDE_RMW_MASK_HI_mask_63_32_offset: u32 = 4;
pub const SDMA_PKT_PTEPDE_RMW_MASK_HI_mask_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_RMW_MASK_HI_mask_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_RMW_MASK_HI_MASK_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_RMW_MASK_HI_mask_63_32_mask) << SDMA_PKT_PTEPDE_RMW_MASK_HI_mask_63_32_shift) };
}

/*define for VALUE_LO word*/
/*define for value_31_0 field*/
pub const SDMA_PKT_PTEPDE_RMW_VALUE_LO_value_31_0_offset: u32 = 5;
pub const SDMA_PKT_PTEPDE_RMW_VALUE_LO_value_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_RMW_VALUE_LO_value_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_RMW_VALUE_LO_VALUE_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_RMW_VALUE_LO_value_31_0_mask) << SDMA_PKT_PTEPDE_RMW_VALUE_LO_value_31_0_shift) };
}

/*define for VALUE_HI word*/
/*define for value_63_32 field*/
pub const SDMA_PKT_PTEPDE_RMW_VALUE_HI_value_63_32_offset: u32 = 6;
pub const SDMA_PKT_PTEPDE_RMW_VALUE_HI_value_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_RMW_VALUE_HI_value_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_RMW_VALUE_HI_VALUE_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_RMW_VALUE_HI_value_63_32_mask) << SDMA_PKT_PTEPDE_RMW_VALUE_HI_value_63_32_shift) };
}

/*define for COUNT word*/
/*define for num_of_pte field*/
pub const SDMA_PKT_PTEPDE_RMW_COUNT_num_of_pte_offset: u32 = 7;
pub const SDMA_PKT_PTEPDE_RMW_COUNT_num_of_pte_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_PTEPDE_RMW_COUNT_num_of_pte_shift: u32 = 0;
macro_rules! SDMA_PKT_PTEPDE_RMW_COUNT_NUM_OF_PTE {
    ($x:expr) => { ((($x) & SDMA_PKT_PTEPDE_RMW_COUNT_num_of_pte_mask) << SDMA_PKT_PTEPDE_RMW_COUNT_num_of_pte_shift) };
}


/*
** Definitions for SDMA_PKT_REGISTER_RMW packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_REGISTER_RMW_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_REGISTER_RMW_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_REGISTER_RMW_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_REGISTER_RMW_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_REGISTER_RMW_HEADER_op_mask) << SDMA_PKT_REGISTER_RMW_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_REGISTER_RMW_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_REGISTER_RMW_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_REGISTER_RMW_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_REGISTER_RMW_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_REGISTER_RMW_HEADER_sub_op_mask) << SDMA_PKT_REGISTER_RMW_HEADER_sub_op_shift) };
}

/*define for ADDR word*/
/*define for addr field*/
pub const SDMA_PKT_REGISTER_RMW_ADDR_addr_offset: u32 = 1;
pub const SDMA_PKT_REGISTER_RMW_ADDR_addr_mask: u32 = 0x000FFFFF;
pub const SDMA_PKT_REGISTER_RMW_ADDR_addr_shift: u32 = 0;
macro_rules! SDMA_PKT_REGISTER_RMW_ADDR_ADDR {
    ($x:expr) => { ((($x) & SDMA_PKT_REGISTER_RMW_ADDR_addr_mask) << SDMA_PKT_REGISTER_RMW_ADDR_addr_shift) };
}

/*define for aperture_id field*/
pub const SDMA_PKT_REGISTER_RMW_ADDR_aperture_id_offset: u32 = 1;
pub const SDMA_PKT_REGISTER_RMW_ADDR_aperture_id_mask: u32 = 0x00000FFF;
pub const SDMA_PKT_REGISTER_RMW_ADDR_aperture_id_shift: u32 = 20;
macro_rules! SDMA_PKT_REGISTER_RMW_ADDR_APERTURE_ID {
    ($x:expr) => { ((($x) & SDMA_PKT_REGISTER_RMW_ADDR_aperture_id_mask) << SDMA_PKT_REGISTER_RMW_ADDR_aperture_id_shift) };
}

/*define for MASK word*/
/*define for mask field*/
pub const SDMA_PKT_REGISTER_RMW_MASK_mask_offset: u32 = 2;
pub const SDMA_PKT_REGISTER_RMW_MASK_mask_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_REGISTER_RMW_MASK_mask_shift: u32 = 0;
macro_rules! SDMA_PKT_REGISTER_RMW_MASK_MASK {
    ($x:expr) => { ((($x) & SDMA_PKT_REGISTER_RMW_MASK_mask_mask) << SDMA_PKT_REGISTER_RMW_MASK_mask_shift) };
}

/*define for VALUE word*/
/*define for value field*/
pub const SDMA_PKT_REGISTER_RMW_VALUE_value_offset: u32 = 3;
pub const SDMA_PKT_REGISTER_RMW_VALUE_value_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_REGISTER_RMW_VALUE_value_shift: u32 = 0;
macro_rules! SDMA_PKT_REGISTER_RMW_VALUE_VALUE {
    ($x:expr) => { ((($x) & SDMA_PKT_REGISTER_RMW_VALUE_value_mask) << SDMA_PKT_REGISTER_RMW_VALUE_value_shift) };
}

/*define for MISC word*/
/*define for stride field*/
pub const SDMA_PKT_REGISTER_RMW_MISC_stride_offset: u32 = 4;
pub const SDMA_PKT_REGISTER_RMW_MISC_stride_mask: u32 = 0x000FFFFF;
pub const SDMA_PKT_REGISTER_RMW_MISC_stride_shift: u32 = 0;
macro_rules! SDMA_PKT_REGISTER_RMW_MISC_STRIDE {
    ($x:expr) => { ((($x) & SDMA_PKT_REGISTER_RMW_MISC_stride_mask) << SDMA_PKT_REGISTER_RMW_MISC_stride_shift) };
}

/*define for num_of_reg field*/
pub const SDMA_PKT_REGISTER_RMW_MISC_num_of_reg_offset: u32 = 4;
pub const SDMA_PKT_REGISTER_RMW_MISC_num_of_reg_mask: u32 = 0x00000FFF;
pub const SDMA_PKT_REGISTER_RMW_MISC_num_of_reg_shift: u32 = 20;
macro_rules! SDMA_PKT_REGISTER_RMW_MISC_NUM_OF_REG {
    ($x:expr) => { ((($x) & SDMA_PKT_REGISTER_RMW_MISC_num_of_reg_mask) << SDMA_PKT_REGISTER_RMW_MISC_num_of_reg_shift) };
}


/*
** Definitions for SDMA_PKT_WRITE_INCR packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_WRITE_INCR_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_WRITE_INCR_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_WRITE_INCR_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_INCR_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_INCR_HEADER_op_mask) << SDMA_PKT_WRITE_INCR_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_WRITE_INCR_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_WRITE_INCR_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_WRITE_INCR_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_WRITE_INCR_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_INCR_HEADER_sub_op_mask) << SDMA_PKT_WRITE_INCR_HEADER_sub_op_shift) };
}

/*define for cache_policy field*/
pub const SDMA_PKT_WRITE_INCR_HEADER_cache_policy_offset: u32 = 0;
pub const SDMA_PKT_WRITE_INCR_HEADER_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_WRITE_INCR_HEADER_cache_policy_shift: u32 = 24;
macro_rules! SDMA_PKT_WRITE_INCR_HEADER_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_INCR_HEADER_cache_policy_mask) << SDMA_PKT_WRITE_INCR_HEADER_cache_policy_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_WRITE_INCR_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_WRITE_INCR_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_WRITE_INCR_HEADER_cpv_shift: u32 = 28;
macro_rules! SDMA_PKT_WRITE_INCR_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_INCR_HEADER_cpv_mask) << SDMA_PKT_WRITE_INCR_HEADER_cpv_shift) };
}

/*define for DST_ADDR_LO word*/
/*define for dst_addr_31_0 field*/
pub const SDMA_PKT_WRITE_INCR_DST_ADDR_LO_dst_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_WRITE_INCR_DST_ADDR_LO_dst_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_INCR_DST_ADDR_LO_dst_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_INCR_DST_ADDR_LO_DST_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_INCR_DST_ADDR_LO_dst_addr_31_0_mask) << SDMA_PKT_WRITE_INCR_DST_ADDR_LO_dst_addr_31_0_shift) };
}

/*define for DST_ADDR_HI word*/
/*define for dst_addr_63_32 field*/
pub const SDMA_PKT_WRITE_INCR_DST_ADDR_HI_dst_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_WRITE_INCR_DST_ADDR_HI_dst_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_INCR_DST_ADDR_HI_dst_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_INCR_DST_ADDR_HI_DST_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_INCR_DST_ADDR_HI_dst_addr_63_32_mask) << SDMA_PKT_WRITE_INCR_DST_ADDR_HI_dst_addr_63_32_shift) };
}

/*define for MASK_DW0 word*/
/*define for mask_dw0 field*/
pub const SDMA_PKT_WRITE_INCR_MASK_DW0_mask_dw0_offset: u32 = 3;
pub const SDMA_PKT_WRITE_INCR_MASK_DW0_mask_dw0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_INCR_MASK_DW0_mask_dw0_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_INCR_MASK_DW0_MASK_DW0 {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_INCR_MASK_DW0_mask_dw0_mask) << SDMA_PKT_WRITE_INCR_MASK_DW0_mask_dw0_shift) };
}

/*define for MASK_DW1 word*/
/*define for mask_dw1 field*/
pub const SDMA_PKT_WRITE_INCR_MASK_DW1_mask_dw1_offset: u32 = 4;
pub const SDMA_PKT_WRITE_INCR_MASK_DW1_mask_dw1_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_INCR_MASK_DW1_mask_dw1_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_INCR_MASK_DW1_MASK_DW1 {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_INCR_MASK_DW1_mask_dw1_mask) << SDMA_PKT_WRITE_INCR_MASK_DW1_mask_dw1_shift) };
}

/*define for INIT_DW0 word*/
/*define for init_dw0 field*/
pub const SDMA_PKT_WRITE_INCR_INIT_DW0_init_dw0_offset: u32 = 5;
pub const SDMA_PKT_WRITE_INCR_INIT_DW0_init_dw0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_INCR_INIT_DW0_init_dw0_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_INCR_INIT_DW0_INIT_DW0 {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_INCR_INIT_DW0_init_dw0_mask) << SDMA_PKT_WRITE_INCR_INIT_DW0_init_dw0_shift) };
}

/*define for INIT_DW1 word*/
/*define for init_dw1 field*/
pub const SDMA_PKT_WRITE_INCR_INIT_DW1_init_dw1_offset: u32 = 6;
pub const SDMA_PKT_WRITE_INCR_INIT_DW1_init_dw1_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_INCR_INIT_DW1_init_dw1_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_INCR_INIT_DW1_INIT_DW1 {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_INCR_INIT_DW1_init_dw1_mask) << SDMA_PKT_WRITE_INCR_INIT_DW1_init_dw1_shift) };
}

/*define for INCR_DW0 word*/
/*define for incr_dw0 field*/
pub const SDMA_PKT_WRITE_INCR_INCR_DW0_incr_dw0_offset: u32 = 7;
pub const SDMA_PKT_WRITE_INCR_INCR_DW0_incr_dw0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_INCR_INCR_DW0_incr_dw0_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_INCR_INCR_DW0_INCR_DW0 {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_INCR_INCR_DW0_incr_dw0_mask) << SDMA_PKT_WRITE_INCR_INCR_DW0_incr_dw0_shift) };
}

/*define for INCR_DW1 word*/
/*define for incr_dw1 field*/
pub const SDMA_PKT_WRITE_INCR_INCR_DW1_incr_dw1_offset: u32 = 8;
pub const SDMA_PKT_WRITE_INCR_INCR_DW1_incr_dw1_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_WRITE_INCR_INCR_DW1_incr_dw1_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_INCR_INCR_DW1_INCR_DW1 {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_INCR_INCR_DW1_incr_dw1_mask) << SDMA_PKT_WRITE_INCR_INCR_DW1_incr_dw1_shift) };
}

/*define for COUNT word*/
/*define for count field*/
pub const SDMA_PKT_WRITE_INCR_COUNT_count_offset: u32 = 9;
pub const SDMA_PKT_WRITE_INCR_COUNT_count_mask: u32 = 0x0007FFFF;
pub const SDMA_PKT_WRITE_INCR_COUNT_count_shift: u32 = 0;
macro_rules! SDMA_PKT_WRITE_INCR_COUNT_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_WRITE_INCR_COUNT_count_mask) << SDMA_PKT_WRITE_INCR_COUNT_count_shift) };
}


/*
** Definitions for SDMA_PKT_INDIRECT packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_INDIRECT_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_INDIRECT_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_INDIRECT_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_INDIRECT_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_INDIRECT_HEADER_op_mask) << SDMA_PKT_INDIRECT_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_INDIRECT_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_INDIRECT_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_INDIRECT_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_INDIRECT_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_INDIRECT_HEADER_sub_op_mask) << SDMA_PKT_INDIRECT_HEADER_sub_op_shift) };
}

/*define for vmid field*/
pub const SDMA_PKT_INDIRECT_HEADER_vmid_offset: u32 = 0;
pub const SDMA_PKT_INDIRECT_HEADER_vmid_mask: u32 = 0x0000000F;
pub const SDMA_PKT_INDIRECT_HEADER_vmid_shift: u32 = 16;
macro_rules! SDMA_PKT_INDIRECT_HEADER_VMID {
    ($x:expr) => { ((($x) & SDMA_PKT_INDIRECT_HEADER_vmid_mask) << SDMA_PKT_INDIRECT_HEADER_vmid_shift) };
}

/*define for priv field*/
pub const SDMA_PKT_INDIRECT_HEADER_priv_offset: u32 = 0;
pub const SDMA_PKT_INDIRECT_HEADER_priv_mask: u32 = 0x00000001;
pub const SDMA_PKT_INDIRECT_HEADER_priv_shift: u32 = 31;
macro_rules! SDMA_PKT_INDIRECT_HEADER_PRIV {
    ($x:expr) => { ((($x) & SDMA_PKT_INDIRECT_HEADER_priv_mask) << SDMA_PKT_INDIRECT_HEADER_priv_shift) };
}

/*define for BASE_LO word*/
/*define for ib_base_31_0 field*/
pub const SDMA_PKT_INDIRECT_BASE_LO_ib_base_31_0_offset: u32 = 1;
pub const SDMA_PKT_INDIRECT_BASE_LO_ib_base_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_INDIRECT_BASE_LO_ib_base_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_INDIRECT_BASE_LO_IB_BASE_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_INDIRECT_BASE_LO_ib_base_31_0_mask) << SDMA_PKT_INDIRECT_BASE_LO_ib_base_31_0_shift) };
}

/*define for BASE_HI word*/
/*define for ib_base_63_32 field*/
pub const SDMA_PKT_INDIRECT_BASE_HI_ib_base_63_32_offset: u32 = 2;
pub const SDMA_PKT_INDIRECT_BASE_HI_ib_base_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_INDIRECT_BASE_HI_ib_base_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_INDIRECT_BASE_HI_IB_BASE_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_INDIRECT_BASE_HI_ib_base_63_32_mask) << SDMA_PKT_INDIRECT_BASE_HI_ib_base_63_32_shift) };
}

/*define for IB_SIZE word*/
/*define for ib_size field*/
pub const SDMA_PKT_INDIRECT_IB_SIZE_ib_size_offset: u32 = 3;
pub const SDMA_PKT_INDIRECT_IB_SIZE_ib_size_mask: u32 = 0x000FFFFF;
pub const SDMA_PKT_INDIRECT_IB_SIZE_ib_size_shift: u32 = 0;
macro_rules! SDMA_PKT_INDIRECT_IB_SIZE_IB_SIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_INDIRECT_IB_SIZE_ib_size_mask) << SDMA_PKT_INDIRECT_IB_SIZE_ib_size_shift) };
}

/*define for CSA_ADDR_LO word*/
/*define for csa_addr_31_0 field*/
pub const SDMA_PKT_INDIRECT_CSA_ADDR_LO_csa_addr_31_0_offset: u32 = 4;
pub const SDMA_PKT_INDIRECT_CSA_ADDR_LO_csa_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_INDIRECT_CSA_ADDR_LO_csa_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_INDIRECT_CSA_ADDR_LO_CSA_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_INDIRECT_CSA_ADDR_LO_csa_addr_31_0_mask) << SDMA_PKT_INDIRECT_CSA_ADDR_LO_csa_addr_31_0_shift) };
}

/*define for CSA_ADDR_HI word*/
/*define for csa_addr_63_32 field*/
pub const SDMA_PKT_INDIRECT_CSA_ADDR_HI_csa_addr_63_32_offset: u32 = 5;
pub const SDMA_PKT_INDIRECT_CSA_ADDR_HI_csa_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_INDIRECT_CSA_ADDR_HI_csa_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_INDIRECT_CSA_ADDR_HI_CSA_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_INDIRECT_CSA_ADDR_HI_csa_addr_63_32_mask) << SDMA_PKT_INDIRECT_CSA_ADDR_HI_csa_addr_63_32_shift) };
}


/*
** Definitions for SDMA_PKT_SEMAPHORE packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_SEMAPHORE_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_SEMAPHORE_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_SEMAPHORE_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_SEMAPHORE_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_SEMAPHORE_HEADER_op_mask) << SDMA_PKT_SEMAPHORE_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_SEMAPHORE_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_SEMAPHORE_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_SEMAPHORE_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_SEMAPHORE_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_SEMAPHORE_HEADER_sub_op_mask) << SDMA_PKT_SEMAPHORE_HEADER_sub_op_shift) };
}

/*define for write_one field*/
pub const SDMA_PKT_SEMAPHORE_HEADER_write_one_offset: u32 = 0;
pub const SDMA_PKT_SEMAPHORE_HEADER_write_one_mask: u32 = 0x00000001;
pub const SDMA_PKT_SEMAPHORE_HEADER_write_one_shift: u32 = 29;
macro_rules! SDMA_PKT_SEMAPHORE_HEADER_WRITE_ONE {
    ($x:expr) => { ((($x) & SDMA_PKT_SEMAPHORE_HEADER_write_one_mask) << SDMA_PKT_SEMAPHORE_HEADER_write_one_shift) };
}

/*define for signal field*/
pub const SDMA_PKT_SEMAPHORE_HEADER_signal_offset: u32 = 0;
pub const SDMA_PKT_SEMAPHORE_HEADER_signal_mask: u32 = 0x00000001;
pub const SDMA_PKT_SEMAPHORE_HEADER_signal_shift: u32 = 30;
macro_rules! SDMA_PKT_SEMAPHORE_HEADER_SIGNAL {
    ($x:expr) => { ((($x) & SDMA_PKT_SEMAPHORE_HEADER_signal_mask) << SDMA_PKT_SEMAPHORE_HEADER_signal_shift) };
}

/*define for mailbox field*/
pub const SDMA_PKT_SEMAPHORE_HEADER_mailbox_offset: u32 = 0;
pub const SDMA_PKT_SEMAPHORE_HEADER_mailbox_mask: u32 = 0x00000001;
pub const SDMA_PKT_SEMAPHORE_HEADER_mailbox_shift: u32 = 31;
macro_rules! SDMA_PKT_SEMAPHORE_HEADER_MAILBOX {
    ($x:expr) => { ((($x) & SDMA_PKT_SEMAPHORE_HEADER_mailbo$x_mask) << SDMA_PKT_SEMAPHORE_HEADER_mailbo$x_shift) };
}

/*define for ADDR_LO word*/
/*define for addr_31_0 field*/
pub const SDMA_PKT_SEMAPHORE_ADDR_LO_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_SEMAPHORE_ADDR_LO_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_SEMAPHORE_ADDR_LO_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_SEMAPHORE_ADDR_LO_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_SEMAPHORE_ADDR_LO_addr_31_0_mask) << SDMA_PKT_SEMAPHORE_ADDR_LO_addr_31_0_shift) };
}

/*define for ADDR_HI word*/
/*define for addr_63_32 field*/
pub const SDMA_PKT_SEMAPHORE_ADDR_HI_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_SEMAPHORE_ADDR_HI_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_SEMAPHORE_ADDR_HI_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_SEMAPHORE_ADDR_HI_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_SEMAPHORE_ADDR_HI_addr_63_32_mask) << SDMA_PKT_SEMAPHORE_ADDR_HI_addr_63_32_shift) };
}


/*
** Definitions for SDMA_PKT_MEM_INCR packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_MEM_INCR_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_MEM_INCR_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_MEM_INCR_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_MEM_INCR_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_MEM_INCR_HEADER_op_mask) << SDMA_PKT_MEM_INCR_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_MEM_INCR_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_MEM_INCR_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_MEM_INCR_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_MEM_INCR_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_MEM_INCR_HEADER_sub_op_mask) << SDMA_PKT_MEM_INCR_HEADER_sub_op_shift) };
}

/*define for l2_policy field*/
pub const SDMA_PKT_MEM_INCR_HEADER_l2_policy_offset: u32 = 0;
pub const SDMA_PKT_MEM_INCR_HEADER_l2_policy_mask: u32 = 0x00000003;
pub const SDMA_PKT_MEM_INCR_HEADER_l2_policy_shift: u32 = 24;
macro_rules! SDMA_PKT_MEM_INCR_HEADER_L2_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_MEM_INCR_HEADER_l2_policy_mask) << SDMA_PKT_MEM_INCR_HEADER_l2_policy_shift) };
}

/*define for llc_policy field*/
pub const SDMA_PKT_MEM_INCR_HEADER_llc_policy_offset: u32 = 0;
pub const SDMA_PKT_MEM_INCR_HEADER_llc_policy_mask: u32 = 0x00000001;
pub const SDMA_PKT_MEM_INCR_HEADER_llc_policy_shift: u32 = 26;
macro_rules! SDMA_PKT_MEM_INCR_HEADER_LLC_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_MEM_INCR_HEADER_llc_policy_mask) << SDMA_PKT_MEM_INCR_HEADER_llc_policy_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_MEM_INCR_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_MEM_INCR_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_MEM_INCR_HEADER_cpv_shift: u32 = 28;
macro_rules! SDMA_PKT_MEM_INCR_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_MEM_INCR_HEADER_cpv_mask) << SDMA_PKT_MEM_INCR_HEADER_cpv_shift) };
}

/*define for ADDR_LO word*/
/*define for addr_31_0 field*/
pub const SDMA_PKT_MEM_INCR_ADDR_LO_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_MEM_INCR_ADDR_LO_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_MEM_INCR_ADDR_LO_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_MEM_INCR_ADDR_LO_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_MEM_INCR_ADDR_LO_addr_31_0_mask) << SDMA_PKT_MEM_INCR_ADDR_LO_addr_31_0_shift) };
}

/*define for ADDR_HI word*/
/*define for addr_63_32 field*/
pub const SDMA_PKT_MEM_INCR_ADDR_HI_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_MEM_INCR_ADDR_HI_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_MEM_INCR_ADDR_HI_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_MEM_INCR_ADDR_HI_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_MEM_INCR_ADDR_HI_addr_63_32_mask) << SDMA_PKT_MEM_INCR_ADDR_HI_addr_63_32_shift) };
}


/*
** Definitions for SDMA_PKT_VM_INVALIDATION packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_VM_INVALIDATION_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_VM_INVALIDATION_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_VM_INVALIDATION_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_VM_INVALIDATION_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_VM_INVALIDATION_HEADER_op_mask) << SDMA_PKT_VM_INVALIDATION_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_VM_INVALIDATION_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_VM_INVALIDATION_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_VM_INVALIDATION_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_VM_INVALIDATION_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_VM_INVALIDATION_HEADER_sub_op_mask) << SDMA_PKT_VM_INVALIDATION_HEADER_sub_op_shift) };
}

/*define for gfx_eng_id field*/
pub const SDMA_PKT_VM_INVALIDATION_HEADER_gfx_eng_id_offset: u32 = 0;
pub const SDMA_PKT_VM_INVALIDATION_HEADER_gfx_eng_id_mask: u32 = 0x0000001F;
pub const SDMA_PKT_VM_INVALIDATION_HEADER_gfx_eng_id_shift: u32 = 16;
macro_rules! SDMA_PKT_VM_INVALIDATION_HEADER_GFX_ENG_ID {
    ($x:expr) => { ((($x) & SDMA_PKT_VM_INVALIDATION_HEADER_gf$x_eng_id_mask) << SDMA_PKT_VM_INVALIDATION_HEADER_gf$x_eng_id_shift) };
}

/*define for mm_eng_id field*/
pub const SDMA_PKT_VM_INVALIDATION_HEADER_mm_eng_id_offset: u32 = 0;
pub const SDMA_PKT_VM_INVALIDATION_HEADER_mm_eng_id_mask: u32 = 0x0000001F;
pub const SDMA_PKT_VM_INVALIDATION_HEADER_mm_eng_id_shift: u32 = 24;
macro_rules! SDMA_PKT_VM_INVALIDATION_HEADER_MM_ENG_ID {
    ($x:expr) => { ((($x) & SDMA_PKT_VM_INVALIDATION_HEADER_mm_eng_id_mask) << SDMA_PKT_VM_INVALIDATION_HEADER_mm_eng_id_shift) };
}

/*define for INVALIDATEREQ word*/
/*define for invalidatereq field*/
pub const SDMA_PKT_VM_INVALIDATION_INVALIDATEREQ_invalidatereq_offset: u32 = 1;
pub const SDMA_PKT_VM_INVALIDATION_INVALIDATEREQ_invalidatereq_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_VM_INVALIDATION_INVALIDATEREQ_invalidatereq_shift: u32 = 0;
macro_rules! SDMA_PKT_VM_INVALIDATION_INVALIDATEREQ_INVALIDATEREQ {
    ($x:expr) => { ((($x) & SDMA_PKT_VM_INVALIDATION_INVALIDATEREQ_invalidatereq_mask) << SDMA_PKT_VM_INVALIDATION_INVALIDATEREQ_invalidatereq_shift) };
}

/*define for ADDRESSRANGELO word*/
/*define for addressrangelo field*/
pub const SDMA_PKT_VM_INVALIDATION_ADDRESSRANGELO_addressrangelo_offset: u32 = 2;
pub const SDMA_PKT_VM_INVALIDATION_ADDRESSRANGELO_addressrangelo_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_VM_INVALIDATION_ADDRESSRANGELO_addressrangelo_shift: u32 = 0;
macro_rules! SDMA_PKT_VM_INVALIDATION_ADDRESSRANGELO_ADDRESSRANGELO {
    ($x:expr) => { ((($x) & SDMA_PKT_VM_INVALIDATION_ADDRESSRANGELO_addressrangelo_mask) << SDMA_PKT_VM_INVALIDATION_ADDRESSRANGELO_addressrangelo_shift) };
}

/*define for ADDRESSRANGEHI word*/
/*define for invalidateack field*/
pub const SDMA_PKT_VM_INVALIDATION_ADDRESSRANGEHI_invalidateack_offset: u32 = 3;
pub const SDMA_PKT_VM_INVALIDATION_ADDRESSRANGEHI_invalidateack_mask: u32 = 0x0000FFFF;
pub const SDMA_PKT_VM_INVALIDATION_ADDRESSRANGEHI_invalidateack_shift: u32 = 0;
macro_rules! SDMA_PKT_VM_INVALIDATION_ADDRESSRANGEHI_INVALIDATEACK {
    ($x:expr) => { ((($x) & SDMA_PKT_VM_INVALIDATION_ADDRESSRANGEHI_invalidateack_mask) << SDMA_PKT_VM_INVALIDATION_ADDRESSRANGEHI_invalidateack_shift) };
}

/*define for addressrangehi field*/
pub const SDMA_PKT_VM_INVALIDATION_ADDRESSRANGEHI_addressrangehi_offset: u32 = 3;
pub const SDMA_PKT_VM_INVALIDATION_ADDRESSRANGEHI_addressrangehi_mask: u32 = 0x0000001F;
pub const SDMA_PKT_VM_INVALIDATION_ADDRESSRANGEHI_addressrangehi_shift: u32 = 16;
macro_rules! SDMA_PKT_VM_INVALIDATION_ADDRESSRANGEHI_ADDRESSRANGEHI {
    ($x:expr) => { ((($x) & SDMA_PKT_VM_INVALIDATION_ADDRESSRANGEHI_addressrangehi_mask) << SDMA_PKT_VM_INVALIDATION_ADDRESSRANGEHI_addressrangehi_shift) };
}

/*define for reserved field*/
pub const SDMA_PKT_VM_INVALIDATION_ADDRESSRANGEHI_reserved_offset: u32 = 3;
pub const SDMA_PKT_VM_INVALIDATION_ADDRESSRANGEHI_reserved_mask: u32 = 0x000001FF;
pub const SDMA_PKT_VM_INVALIDATION_ADDRESSRANGEHI_reserved_shift: u32 = 23;
macro_rules! SDMA_PKT_VM_INVALIDATION_ADDRESSRANGEHI_RESERVED {
    ($x:expr) => { ((($x) & SDMA_PKT_VM_INVALIDATION_ADDRESSRANGEHI_reserved_mask) << SDMA_PKT_VM_INVALIDATION_ADDRESSRANGEHI_reserved_shift) };
}


/*
** Definitions for SDMA_PKT_FENCE packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_FENCE_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_FENCE_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_FENCE_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_FENCE_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_FENCE_HEADER_op_mask) << SDMA_PKT_FENCE_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_FENCE_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_FENCE_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_FENCE_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_FENCE_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_FENCE_HEADER_sub_op_mask) << SDMA_PKT_FENCE_HEADER_sub_op_shift) };
}

/*define for mtype field*/
pub const SDMA_PKT_FENCE_HEADER_mtype_offset: u32 = 0;
pub const SDMA_PKT_FENCE_HEADER_mtype_mask: u32 = 0x00000007;
pub const SDMA_PKT_FENCE_HEADER_mtype_shift: u32 = 16;
macro_rules! SDMA_PKT_FENCE_HEADER_MTYPE {
    ($x:expr) => { ((($x) & SDMA_PKT_FENCE_HEADER_mtype_mask) << SDMA_PKT_FENCE_HEADER_mtype_shift) };
}

/*define for gcc field*/
pub const SDMA_PKT_FENCE_HEADER_gcc_offset: u32 = 0;
pub const SDMA_PKT_FENCE_HEADER_gcc_mask: u32 = 0x00000001;
pub const SDMA_PKT_FENCE_HEADER_gcc_shift: u32 = 19;
macro_rules! SDMA_PKT_FENCE_HEADER_GCC {
    ($x:expr) => { ((($x) & SDMA_PKT_FENCE_HEADER_gcc_mask) << SDMA_PKT_FENCE_HEADER_gcc_shift) };
}

/*define for sys field*/
pub const SDMA_PKT_FENCE_HEADER_sys_offset: u32 = 0;
pub const SDMA_PKT_FENCE_HEADER_sys_mask: u32 = 0x00000001;
pub const SDMA_PKT_FENCE_HEADER_sys_shift: u32 = 20;
macro_rules! SDMA_PKT_FENCE_HEADER_SYS {
    ($x:expr) => { ((($x) & SDMA_PKT_FENCE_HEADER_sys_mask) << SDMA_PKT_FENCE_HEADER_sys_shift) };
}

/*define for snp field*/
pub const SDMA_PKT_FENCE_HEADER_snp_offset: u32 = 0;
pub const SDMA_PKT_FENCE_HEADER_snp_mask: u32 = 0x00000001;
pub const SDMA_PKT_FENCE_HEADER_snp_shift: u32 = 22;
macro_rules! SDMA_PKT_FENCE_HEADER_SNP {
    ($x:expr) => { ((($x) & SDMA_PKT_FENCE_HEADER_snp_mask) << SDMA_PKT_FENCE_HEADER_snp_shift) };
}

/*define for gpa field*/
pub const SDMA_PKT_FENCE_HEADER_gpa_offset: u32 = 0;
pub const SDMA_PKT_FENCE_HEADER_gpa_mask: u32 = 0x00000001;
pub const SDMA_PKT_FENCE_HEADER_gpa_shift: u32 = 23;
macro_rules! SDMA_PKT_FENCE_HEADER_GPA {
    ($x:expr) => { ((($x) & SDMA_PKT_FENCE_HEADER_gpa_mask) << SDMA_PKT_FENCE_HEADER_gpa_shift) };
}

/*define for l2_policy field*/
pub const SDMA_PKT_FENCE_HEADER_l2_policy_offset: u32 = 0;
pub const SDMA_PKT_FENCE_HEADER_l2_policy_mask: u32 = 0x00000003;
pub const SDMA_PKT_FENCE_HEADER_l2_policy_shift: u32 = 24;
macro_rules! SDMA_PKT_FENCE_HEADER_L2_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_FENCE_HEADER_l2_policy_mask) << SDMA_PKT_FENCE_HEADER_l2_policy_shift) };
}

/*define for llc_policy field*/
pub const SDMA_PKT_FENCE_HEADER_llc_policy_offset: u32 = 0;
pub const SDMA_PKT_FENCE_HEADER_llc_policy_mask: u32 = 0x00000001;
pub const SDMA_PKT_FENCE_HEADER_llc_policy_shift: u32 = 26;
macro_rules! SDMA_PKT_FENCE_HEADER_LLC_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_FENCE_HEADER_llc_policy_mask) << SDMA_PKT_FENCE_HEADER_llc_policy_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_FENCE_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_FENCE_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_FENCE_HEADER_cpv_shift: u32 = 28;
macro_rules! SDMA_PKT_FENCE_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_FENCE_HEADER_cpv_mask) << SDMA_PKT_FENCE_HEADER_cpv_shift) };
}

/*define for ADDR_LO word*/
/*define for addr_31_0 field*/
pub const SDMA_PKT_FENCE_ADDR_LO_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_FENCE_ADDR_LO_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_FENCE_ADDR_LO_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_FENCE_ADDR_LO_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_FENCE_ADDR_LO_addr_31_0_mask) << SDMA_PKT_FENCE_ADDR_LO_addr_31_0_shift) };
}

/*define for ADDR_HI word*/
/*define for addr_63_32 field*/
pub const SDMA_PKT_FENCE_ADDR_HI_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_FENCE_ADDR_HI_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_FENCE_ADDR_HI_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_FENCE_ADDR_HI_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_FENCE_ADDR_HI_addr_63_32_mask) << SDMA_PKT_FENCE_ADDR_HI_addr_63_32_shift) };
}

/*define for DATA word*/
/*define for data field*/
pub const SDMA_PKT_FENCE_DATA_data_offset: u32 = 3;
pub const SDMA_PKT_FENCE_DATA_data_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_FENCE_DATA_data_shift: u32 = 0;
macro_rules! SDMA_PKT_FENCE_DATA_DATA {
    ($x:expr) => { ((($x) & SDMA_PKT_FENCE_DATA_data_mask) << SDMA_PKT_FENCE_DATA_data_shift) };
}


/*
** Definitions for SDMA_PKT_SRBM_WRITE packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_SRBM_WRITE_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_SRBM_WRITE_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_SRBM_WRITE_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_SRBM_WRITE_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_SRBM_WRITE_HEADER_op_mask) << SDMA_PKT_SRBM_WRITE_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_SRBM_WRITE_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_SRBM_WRITE_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_SRBM_WRITE_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_SRBM_WRITE_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_SRBM_WRITE_HEADER_sub_op_mask) << SDMA_PKT_SRBM_WRITE_HEADER_sub_op_shift) };
}

/*define for byte_en field*/
pub const SDMA_PKT_SRBM_WRITE_HEADER_byte_en_offset: u32 = 0;
pub const SDMA_PKT_SRBM_WRITE_HEADER_byte_en_mask: u32 = 0x0000000F;
pub const SDMA_PKT_SRBM_WRITE_HEADER_byte_en_shift: u32 = 28;
macro_rules! SDMA_PKT_SRBM_WRITE_HEADER_BYTE_EN {
    ($x:expr) => { ((($x) & SDMA_PKT_SRBM_WRITE_HEADER_byte_en_mask) << SDMA_PKT_SRBM_WRITE_HEADER_byte_en_shift) };
}

/*define for ADDR word*/
/*define for addr field*/
pub const SDMA_PKT_SRBM_WRITE_ADDR_addr_offset: u32 = 1;
pub const SDMA_PKT_SRBM_WRITE_ADDR_addr_mask: u32 = 0x0003FFFF;
pub const SDMA_PKT_SRBM_WRITE_ADDR_addr_shift: u32 = 0;
macro_rules! SDMA_PKT_SRBM_WRITE_ADDR_ADDR {
    ($x:expr) => { ((($x) & SDMA_PKT_SRBM_WRITE_ADDR_addr_mask) << SDMA_PKT_SRBM_WRITE_ADDR_addr_shift) };
}

/*define for apertureid field*/
pub const SDMA_PKT_SRBM_WRITE_ADDR_apertureid_offset: u32 = 1;
pub const SDMA_PKT_SRBM_WRITE_ADDR_apertureid_mask: u32 = 0x00000FFF;
pub const SDMA_PKT_SRBM_WRITE_ADDR_apertureid_shift: u32 = 20;
macro_rules! SDMA_PKT_SRBM_WRITE_ADDR_APERTUREID {
    ($x:expr) => { ((($x) & SDMA_PKT_SRBM_WRITE_ADDR_apertureid_mask) << SDMA_PKT_SRBM_WRITE_ADDR_apertureid_shift) };
}

/*define for DATA word*/
/*define for data field*/
pub const SDMA_PKT_SRBM_WRITE_DATA_data_offset: u32 = 2;
pub const SDMA_PKT_SRBM_WRITE_DATA_data_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_SRBM_WRITE_DATA_data_shift: u32 = 0;
macro_rules! SDMA_PKT_SRBM_WRITE_DATA_DATA {
    ($x:expr) => { ((($x) & SDMA_PKT_SRBM_WRITE_DATA_data_mask) << SDMA_PKT_SRBM_WRITE_DATA_data_shift) };
}


/*
** Definitions for SDMA_PKT_PRE_EXE packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_PRE_EXE_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_PRE_EXE_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_PRE_EXE_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_PRE_EXE_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_PRE_EXE_HEADER_op_mask) << SDMA_PKT_PRE_EXE_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_PRE_EXE_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_PRE_EXE_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_PRE_EXE_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_PRE_EXE_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_PRE_EXE_HEADER_sub_op_mask) << SDMA_PKT_PRE_EXE_HEADER_sub_op_shift) };
}

/*define for dev_sel field*/
pub const SDMA_PKT_PRE_EXE_HEADER_dev_sel_offset: u32 = 0;
pub const SDMA_PKT_PRE_EXE_HEADER_dev_sel_mask: u32 = 0x000000FF;
pub const SDMA_PKT_PRE_EXE_HEADER_dev_sel_shift: u32 = 16;
macro_rules! SDMA_PKT_PRE_EXE_HEADER_DEV_SEL {
    ($x:expr) => { ((($x) & SDMA_PKT_PRE_EXE_HEADER_dev_sel_mask) << SDMA_PKT_PRE_EXE_HEADER_dev_sel_shift) };
}

/*define for EXEC_COUNT word*/
/*define for exec_count field*/
pub const SDMA_PKT_PRE_EXE_EXEC_COUNT_exec_count_offset: u32 = 1;
pub const SDMA_PKT_PRE_EXE_EXEC_COUNT_exec_count_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_PRE_EXE_EXEC_COUNT_exec_count_shift: u32 = 0;
macro_rules! SDMA_PKT_PRE_EXE_EXEC_COUNT_EXEC_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_PRE_EXE_EXEC_COUNT_e$xec_count_mask) << SDMA_PKT_PRE_EXE_EXEC_COUNT_e$xec_count_shift) };
}


/*
** Definitions for SDMA_PKT_COND_EXE packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_COND_EXE_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_COND_EXE_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COND_EXE_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_COND_EXE_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COND_EXE_HEADER_op_mask) << SDMA_PKT_COND_EXE_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_COND_EXE_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_COND_EXE_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_COND_EXE_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_COND_EXE_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_COND_EXE_HEADER_sub_op_mask) << SDMA_PKT_COND_EXE_HEADER_sub_op_shift) };
}

/*define for cache_policy field*/
pub const SDMA_PKT_COND_EXE_HEADER_cache_policy_offset: u32 = 0;
pub const SDMA_PKT_COND_EXE_HEADER_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_COND_EXE_HEADER_cache_policy_shift: u32 = 24;
macro_rules! SDMA_PKT_COND_EXE_HEADER_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_COND_EXE_HEADER_cache_policy_mask) << SDMA_PKT_COND_EXE_HEADER_cache_policy_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_COND_EXE_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_COND_EXE_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_COND_EXE_HEADER_cpv_shift: u32 = 28;
macro_rules! SDMA_PKT_COND_EXE_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_COND_EXE_HEADER_cpv_mask) << SDMA_PKT_COND_EXE_HEADER_cpv_shift) };
}

/*define for ADDR_LO word*/
/*define for addr_31_0 field*/
pub const SDMA_PKT_COND_EXE_ADDR_LO_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_COND_EXE_ADDR_LO_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COND_EXE_ADDR_LO_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_COND_EXE_ADDR_LO_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_COND_EXE_ADDR_LO_addr_31_0_mask) << SDMA_PKT_COND_EXE_ADDR_LO_addr_31_0_shift) };
}

/*define for ADDR_HI word*/
/*define for addr_63_32 field*/
pub const SDMA_PKT_COND_EXE_ADDR_HI_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_COND_EXE_ADDR_HI_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COND_EXE_ADDR_HI_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_COND_EXE_ADDR_HI_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_COND_EXE_ADDR_HI_addr_63_32_mask) << SDMA_PKT_COND_EXE_ADDR_HI_addr_63_32_shift) };
}

/*define for REFERENCE word*/
/*define for reference field*/
pub const SDMA_PKT_COND_EXE_REFERENCE_reference_offset: u32 = 3;
pub const SDMA_PKT_COND_EXE_REFERENCE_reference_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_COND_EXE_REFERENCE_reference_shift: u32 = 0;
macro_rules! SDMA_PKT_COND_EXE_REFERENCE_REFERENCE {
    ($x:expr) => { ((($x) & SDMA_PKT_COND_EXE_REFERENCE_reference_mask) << SDMA_PKT_COND_EXE_REFERENCE_reference_shift) };
}

/*define for EXEC_COUNT word*/
/*define for exec_count field*/
pub const SDMA_PKT_COND_EXE_EXEC_COUNT_exec_count_offset: u32 = 4;
pub const SDMA_PKT_COND_EXE_EXEC_COUNT_exec_count_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_COND_EXE_EXEC_COUNT_exec_count_shift: u32 = 0;
macro_rules! SDMA_PKT_COND_EXE_EXEC_COUNT_EXEC_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_COND_EXE_EXEC_COUNT_e$xec_count_mask) << SDMA_PKT_COND_EXE_EXEC_COUNT_e$xec_count_shift) };
}


/*
** Definitions for SDMA_PKT_CONSTANT_FILL packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_CONSTANT_FILL_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_CONSTANT_FILL_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_CONSTANT_FILL_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_CONSTANT_FILL_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_CONSTANT_FILL_HEADER_op_mask) << SDMA_PKT_CONSTANT_FILL_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_CONSTANT_FILL_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_CONSTANT_FILL_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_CONSTANT_FILL_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_CONSTANT_FILL_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_CONSTANT_FILL_HEADER_sub_op_mask) << SDMA_PKT_CONSTANT_FILL_HEADER_sub_op_shift) };
}

/*define for sw field*/
pub const SDMA_PKT_CONSTANT_FILL_HEADER_sw_offset: u32 = 0;
pub const SDMA_PKT_CONSTANT_FILL_HEADER_sw_mask: u32 = 0x00000003;
pub const SDMA_PKT_CONSTANT_FILL_HEADER_sw_shift: u32 = 16;
macro_rules! SDMA_PKT_CONSTANT_FILL_HEADER_SW {
    ($x:expr) => { ((($x) & SDMA_PKT_CONSTANT_FILL_HEADER_sw_mask) << SDMA_PKT_CONSTANT_FILL_HEADER_sw_shift) };
}

/*define for cache_policy field*/
pub const SDMA_PKT_CONSTANT_FILL_HEADER_cache_policy_offset: u32 = 0;
pub const SDMA_PKT_CONSTANT_FILL_HEADER_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_CONSTANT_FILL_HEADER_cache_policy_shift: u32 = 24;
macro_rules! SDMA_PKT_CONSTANT_FILL_HEADER_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_CONSTANT_FILL_HEADER_cache_policy_mask) << SDMA_PKT_CONSTANT_FILL_HEADER_cache_policy_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_CONSTANT_FILL_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_CONSTANT_FILL_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_CONSTANT_FILL_HEADER_cpv_shift: u32 = 28;
macro_rules! SDMA_PKT_CONSTANT_FILL_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_CONSTANT_FILL_HEADER_cpv_mask) << SDMA_PKT_CONSTANT_FILL_HEADER_cpv_shift) };
}

/*define for fillsize field*/
pub const SDMA_PKT_CONSTANT_FILL_HEADER_fillsize_offset: u32 = 0;
pub const SDMA_PKT_CONSTANT_FILL_HEADER_fillsize_mask: u32 = 0x00000003;
pub const SDMA_PKT_CONSTANT_FILL_HEADER_fillsize_shift: u32 = 30;
macro_rules! SDMA_PKT_CONSTANT_FILL_HEADER_FILLSIZE {
    ($x:expr) => { ((($x) & SDMA_PKT_CONSTANT_FILL_HEADER_fillsize_mask) << SDMA_PKT_CONSTANT_FILL_HEADER_fillsize_shift) };
}

/*define for DST_ADDR_LO word*/
/*define for dst_addr_31_0 field*/
pub const SDMA_PKT_CONSTANT_FILL_DST_ADDR_LO_dst_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_CONSTANT_FILL_DST_ADDR_LO_dst_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_CONSTANT_FILL_DST_ADDR_LO_dst_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_CONSTANT_FILL_DST_ADDR_LO_DST_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_CONSTANT_FILL_DST_ADDR_LO_dst_addr_31_0_mask) << SDMA_PKT_CONSTANT_FILL_DST_ADDR_LO_dst_addr_31_0_shift) };
}

/*define for DST_ADDR_HI word*/
/*define for dst_addr_63_32 field*/
pub const SDMA_PKT_CONSTANT_FILL_DST_ADDR_HI_dst_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_CONSTANT_FILL_DST_ADDR_HI_dst_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_CONSTANT_FILL_DST_ADDR_HI_dst_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_CONSTANT_FILL_DST_ADDR_HI_DST_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_CONSTANT_FILL_DST_ADDR_HI_dst_addr_63_32_mask) << SDMA_PKT_CONSTANT_FILL_DST_ADDR_HI_dst_addr_63_32_shift) };
}

/*define for DATA word*/
/*define for src_data_31_0 field*/
pub const SDMA_PKT_CONSTANT_FILL_DATA_src_data_31_0_offset: u32 = 3;
pub const SDMA_PKT_CONSTANT_FILL_DATA_src_data_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_CONSTANT_FILL_DATA_src_data_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_CONSTANT_FILL_DATA_SRC_DATA_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_CONSTANT_FILL_DATA_src_data_31_0_mask) << SDMA_PKT_CONSTANT_FILL_DATA_src_data_31_0_shift) };
}

/*define for COUNT word*/
/*define for count field*/
pub const SDMA_PKT_CONSTANT_FILL_COUNT_count_offset: u32 = 4;
pub const SDMA_PKT_CONSTANT_FILL_COUNT_count_mask: u32 = 0x3FFFFFFF;
pub const SDMA_PKT_CONSTANT_FILL_COUNT_count_shift: u32 = 0;
macro_rules! SDMA_PKT_CONSTANT_FILL_COUNT_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_CONSTANT_FILL_COUNT_count_mask) << SDMA_PKT_CONSTANT_FILL_COUNT_count_shift) };
}


/*
** Definitions for SDMA_PKT_DATA_FILL_MULTI packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_DATA_FILL_MULTI_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_DATA_FILL_MULTI_HEADER_op_mask) << SDMA_PKT_DATA_FILL_MULTI_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_DATA_FILL_MULTI_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_DATA_FILL_MULTI_HEADER_sub_op_mask) << SDMA_PKT_DATA_FILL_MULTI_HEADER_sub_op_shift) };
}

/*define for cache_policy field*/
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_cache_policy_offset: u32 = 0;
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_cache_policy_shift: u32 = 24;
macro_rules! SDMA_PKT_DATA_FILL_MULTI_HEADER_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_DATA_FILL_MULTI_HEADER_cache_policy_mask) << SDMA_PKT_DATA_FILL_MULTI_HEADER_cache_policy_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_cpv_shift: u32 = 28;
macro_rules! SDMA_PKT_DATA_FILL_MULTI_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_DATA_FILL_MULTI_HEADER_cpv_mask) << SDMA_PKT_DATA_FILL_MULTI_HEADER_cpv_shift) };
}

/*define for memlog_clr field*/
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_memlog_clr_offset: u32 = 0;
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_memlog_clr_mask: u32 = 0x00000001;
pub const SDMA_PKT_DATA_FILL_MULTI_HEADER_memlog_clr_shift: u32 = 31;
macro_rules! SDMA_PKT_DATA_FILL_MULTI_HEADER_MEMLOG_CLR {
    ($x:expr) => { ((($x) & SDMA_PKT_DATA_FILL_MULTI_HEADER_memlog_clr_mask) << SDMA_PKT_DATA_FILL_MULTI_HEADER_memlog_clr_shift) };
}

/*define for BYTE_STRIDE word*/
/*define for byte_stride field*/
pub const SDMA_PKT_DATA_FILL_MULTI_BYTE_STRIDE_byte_stride_offset: u32 = 1;
pub const SDMA_PKT_DATA_FILL_MULTI_BYTE_STRIDE_byte_stride_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_DATA_FILL_MULTI_BYTE_STRIDE_byte_stride_shift: u32 = 0;
macro_rules! SDMA_PKT_DATA_FILL_MULTI_BYTE_STRIDE_BYTE_STRIDE {
    ($x:expr) => { ((($x) & SDMA_PKT_DATA_FILL_MULTI_BYTE_STRIDE_byte_stride_mask) << SDMA_PKT_DATA_FILL_MULTI_BYTE_STRIDE_byte_stride_shift) };
}

/*define for DMA_COUNT word*/
/*define for dma_count field*/
pub const SDMA_PKT_DATA_FILL_MULTI_DMA_COUNT_dma_count_offset: u32 = 2;
pub const SDMA_PKT_DATA_FILL_MULTI_DMA_COUNT_dma_count_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_DATA_FILL_MULTI_DMA_COUNT_dma_count_shift: u32 = 0;
macro_rules! SDMA_PKT_DATA_FILL_MULTI_DMA_COUNT_DMA_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_DATA_FILL_MULTI_DMA_COUNT_dma_count_mask) << SDMA_PKT_DATA_FILL_MULTI_DMA_COUNT_dma_count_shift) };
}

/*define for DST_ADDR_LO word*/
/*define for dst_addr_31_0 field*/
pub const SDMA_PKT_DATA_FILL_MULTI_DST_ADDR_LO_dst_addr_31_0_offset: u32 = 3;
pub const SDMA_PKT_DATA_FILL_MULTI_DST_ADDR_LO_dst_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_DATA_FILL_MULTI_DST_ADDR_LO_dst_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_DATA_FILL_MULTI_DST_ADDR_LO_DST_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_DATA_FILL_MULTI_DST_ADDR_LO_dst_addr_31_0_mask) << SDMA_PKT_DATA_FILL_MULTI_DST_ADDR_LO_dst_addr_31_0_shift) };
}

/*define for DST_ADDR_HI word*/
/*define for dst_addr_63_32 field*/
pub const SDMA_PKT_DATA_FILL_MULTI_DST_ADDR_HI_dst_addr_63_32_offset: u32 = 4;
pub const SDMA_PKT_DATA_FILL_MULTI_DST_ADDR_HI_dst_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_DATA_FILL_MULTI_DST_ADDR_HI_dst_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_DATA_FILL_MULTI_DST_ADDR_HI_DST_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_DATA_FILL_MULTI_DST_ADDR_HI_dst_addr_63_32_mask) << SDMA_PKT_DATA_FILL_MULTI_DST_ADDR_HI_dst_addr_63_32_shift) };
}

/*define for BYTE_COUNT word*/
/*define for count field*/
pub const SDMA_PKT_DATA_FILL_MULTI_BYTE_COUNT_count_offset: u32 = 5;
pub const SDMA_PKT_DATA_FILL_MULTI_BYTE_COUNT_count_mask: u32 = 0x03FFFFFF;
pub const SDMA_PKT_DATA_FILL_MULTI_BYTE_COUNT_count_shift: u32 = 0;
macro_rules! SDMA_PKT_DATA_FILL_MULTI_BYTE_COUNT_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_DATA_FILL_MULTI_BYTE_COUNT_count_mask) << SDMA_PKT_DATA_FILL_MULTI_BYTE_COUNT_count_shift) };
}


/*
** Definitions for SDMA_PKT_POLL_REGMEM packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_POLL_REGMEM_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_POLL_REGMEM_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_POLL_REGMEM_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_REGMEM_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_REGMEM_HEADER_op_mask) << SDMA_PKT_POLL_REGMEM_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_POLL_REGMEM_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_POLL_REGMEM_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_POLL_REGMEM_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_POLL_REGMEM_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_REGMEM_HEADER_sub_op_mask) << SDMA_PKT_POLL_REGMEM_HEADER_sub_op_shift) };
}

/*define for cache_policy field*/
pub const SDMA_PKT_POLL_REGMEM_HEADER_cache_policy_offset: u32 = 0;
pub const SDMA_PKT_POLL_REGMEM_HEADER_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_POLL_REGMEM_HEADER_cache_policy_shift: u32 = 20;
macro_rules! SDMA_PKT_POLL_REGMEM_HEADER_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_REGMEM_HEADER_cache_policy_mask) << SDMA_PKT_POLL_REGMEM_HEADER_cache_policy_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_POLL_REGMEM_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_POLL_REGMEM_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_POLL_REGMEM_HEADER_cpv_shift: u32 = 24;
macro_rules! SDMA_PKT_POLL_REGMEM_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_REGMEM_HEADER_cpv_mask) << SDMA_PKT_POLL_REGMEM_HEADER_cpv_shift) };
}

/*define for hdp_flush field*/
pub const SDMA_PKT_POLL_REGMEM_HEADER_hdp_flush_offset: u32 = 0;
pub const SDMA_PKT_POLL_REGMEM_HEADER_hdp_flush_mask: u32 = 0x00000001;
pub const SDMA_PKT_POLL_REGMEM_HEADER_hdp_flush_shift: u32 = 26;
macro_rules! SDMA_PKT_POLL_REGMEM_HEADER_HDP_FLUSH {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_REGMEM_HEADER_hdp_flush_mask) << SDMA_PKT_POLL_REGMEM_HEADER_hdp_flush_shift) };
}

/*define for func field*/
pub const SDMA_PKT_POLL_REGMEM_HEADER_func_offset: u32 = 0;
pub const SDMA_PKT_POLL_REGMEM_HEADER_func_mask: u32 = 0x00000007;
pub const SDMA_PKT_POLL_REGMEM_HEADER_func_shift: u32 = 28;
macro_rules! SDMA_PKT_POLL_REGMEM_HEADER_FUNC {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_REGMEM_HEADER_func_mask) << SDMA_PKT_POLL_REGMEM_HEADER_func_shift) };
}

/*define for mem_poll field*/
pub const SDMA_PKT_POLL_REGMEM_HEADER_mem_poll_offset: u32 = 0;
pub const SDMA_PKT_POLL_REGMEM_HEADER_mem_poll_mask: u32 = 0x00000001;
pub const SDMA_PKT_POLL_REGMEM_HEADER_mem_poll_shift: u32 = 31;
macro_rules! SDMA_PKT_POLL_REGMEM_HEADER_MEM_POLL {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_REGMEM_HEADER_mem_poll_mask) << SDMA_PKT_POLL_REGMEM_HEADER_mem_poll_shift) };
}

/*define for ADDR_LO word*/
/*define for addr_31_0 field*/
pub const SDMA_PKT_POLL_REGMEM_ADDR_LO_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_POLL_REGMEM_ADDR_LO_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_REGMEM_ADDR_LO_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_REGMEM_ADDR_LO_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_REGMEM_ADDR_LO_addr_31_0_mask) << SDMA_PKT_POLL_REGMEM_ADDR_LO_addr_31_0_shift) };
}

/*define for ADDR_HI word*/
/*define for addr_63_32 field*/
pub const SDMA_PKT_POLL_REGMEM_ADDR_HI_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_POLL_REGMEM_ADDR_HI_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_REGMEM_ADDR_HI_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_REGMEM_ADDR_HI_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_REGMEM_ADDR_HI_addr_63_32_mask) << SDMA_PKT_POLL_REGMEM_ADDR_HI_addr_63_32_shift) };
}

/*define for VALUE word*/
/*define for value field*/
pub const SDMA_PKT_POLL_REGMEM_VALUE_value_offset: u32 = 3;
pub const SDMA_PKT_POLL_REGMEM_VALUE_value_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_REGMEM_VALUE_value_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_REGMEM_VALUE_VALUE {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_REGMEM_VALUE_value_mask) << SDMA_PKT_POLL_REGMEM_VALUE_value_shift) };
}

/*define for MASK word*/
/*define for mask field*/
pub const SDMA_PKT_POLL_REGMEM_MASK_mask_offset: u32 = 4;
pub const SDMA_PKT_POLL_REGMEM_MASK_mask_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_REGMEM_MASK_mask_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_REGMEM_MASK_MASK {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_REGMEM_MASK_mask_mask) << SDMA_PKT_POLL_REGMEM_MASK_mask_shift) };
}

/*define for DW5 word*/
/*define for interval field*/
pub const SDMA_PKT_POLL_REGMEM_DW5_interval_offset: u32 = 5;
pub const SDMA_PKT_POLL_REGMEM_DW5_interval_mask: u32 = 0x0000FFFF;
pub const SDMA_PKT_POLL_REGMEM_DW5_interval_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_REGMEM_DW5_INTERVAL {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_REGMEM_DW5_interval_mask) << SDMA_PKT_POLL_REGMEM_DW5_interval_shift) };
}

/*define for retry_count field*/
pub const SDMA_PKT_POLL_REGMEM_DW5_retry_count_offset: u32 = 5;
pub const SDMA_PKT_POLL_REGMEM_DW5_retry_count_mask: u32 = 0x00000FFF;
pub const SDMA_PKT_POLL_REGMEM_DW5_retry_count_shift: u32 = 16;
macro_rules! SDMA_PKT_POLL_REGMEM_DW5_RETRY_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_REGMEM_DW5_retry_count_mask) << SDMA_PKT_POLL_REGMEM_DW5_retry_count_shift) };
}


/*
** Definitions for SDMA_PKT_POLL_REG_WRITE_MEM packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_op_mask) << SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_sub_op_mask) << SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_sub_op_shift) };
}

/*define for cache_policy field*/
pub const SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_cache_policy_offset: u32 = 0;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_cache_policy_shift: u32 = 24;
macro_rules! SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_cache_policy_mask) << SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_cache_policy_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_cpv_shift: u32 = 28;
macro_rules! SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_cpv_mask) << SDMA_PKT_POLL_REG_WRITE_MEM_HEADER_cpv_shift) };
}

/*define for SRC_ADDR word*/
/*define for addr_31_2 field*/
pub const SDMA_PKT_POLL_REG_WRITE_MEM_SRC_ADDR_addr_31_2_offset: u32 = 1;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_SRC_ADDR_addr_31_2_mask: u32 = 0x3FFFFFFF;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_SRC_ADDR_addr_31_2_shift: u32 = 2;
macro_rules! SDMA_PKT_POLL_REG_WRITE_MEM_SRC_ADDR_ADDR_31_2 {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_REG_WRITE_MEM_SRC_ADDR_addr_31_2_mask) << SDMA_PKT_POLL_REG_WRITE_MEM_SRC_ADDR_addr_31_2_shift) };
}

/*define for DST_ADDR_LO word*/
/*define for addr_31_0 field*/
pub const SDMA_PKT_POLL_REG_WRITE_MEM_DST_ADDR_LO_addr_31_0_offset: u32 = 2;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_DST_ADDR_LO_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_DST_ADDR_LO_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_REG_WRITE_MEM_DST_ADDR_LO_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_REG_WRITE_MEM_DST_ADDR_LO_addr_31_0_mask) << SDMA_PKT_POLL_REG_WRITE_MEM_DST_ADDR_LO_addr_31_0_shift) };
}

/*define for DST_ADDR_HI word*/
/*define for addr_63_32 field*/
pub const SDMA_PKT_POLL_REG_WRITE_MEM_DST_ADDR_HI_addr_63_32_offset: u32 = 3;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_DST_ADDR_HI_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_REG_WRITE_MEM_DST_ADDR_HI_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_REG_WRITE_MEM_DST_ADDR_HI_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_REG_WRITE_MEM_DST_ADDR_HI_addr_63_32_mask) << SDMA_PKT_POLL_REG_WRITE_MEM_DST_ADDR_HI_addr_63_32_shift) };
}


/*
** Definitions for SDMA_PKT_POLL_DBIT_WRITE_MEM packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_op_mask) << SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_sub_op_mask) << SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_sub_op_shift) };
}

/*define for ea field*/
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_ea_offset: u32 = 0;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_ea_mask: u32 = 0x00000003;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_ea_shift: u32 = 16;
macro_rules! SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_EA {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_ea_mask) << SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_ea_shift) };
}

/*define for cache_policy field*/
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_cache_policy_offset: u32 = 0;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_cache_policy_shift: u32 = 24;
macro_rules! SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_cache_policy_mask) << SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_cache_policy_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_cpv_shift: u32 = 28;
macro_rules! SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_cpv_mask) << SDMA_PKT_POLL_DBIT_WRITE_MEM_HEADER_cpv_shift) };
}

/*define for DST_ADDR_LO word*/
/*define for addr_31_0 field*/
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_DST_ADDR_LO_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_DST_ADDR_LO_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_DST_ADDR_LO_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_DBIT_WRITE_MEM_DST_ADDR_LO_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_DBIT_WRITE_MEM_DST_ADDR_LO_addr_31_0_mask) << SDMA_PKT_POLL_DBIT_WRITE_MEM_DST_ADDR_LO_addr_31_0_shift) };
}

/*define for DST_ADDR_HI word*/
/*define for addr_63_32 field*/
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_DST_ADDR_HI_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_DST_ADDR_HI_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_DST_ADDR_HI_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_DBIT_WRITE_MEM_DST_ADDR_HI_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_DBIT_WRITE_MEM_DST_ADDR_HI_addr_63_32_mask) << SDMA_PKT_POLL_DBIT_WRITE_MEM_DST_ADDR_HI_addr_63_32_shift) };
}

/*define for START_PAGE word*/
/*define for addr_31_4 field*/
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_START_PAGE_addr_31_4_offset: u32 = 3;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_START_PAGE_addr_31_4_mask: u32 = 0x0FFFFFFF;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_START_PAGE_addr_31_4_shift: u32 = 4;
macro_rules! SDMA_PKT_POLL_DBIT_WRITE_MEM_START_PAGE_ADDR_31_4 {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_DBIT_WRITE_MEM_START_PAGE_addr_31_4_mask) << SDMA_PKT_POLL_DBIT_WRITE_MEM_START_PAGE_addr_31_4_shift) };
}

/*define for PAGE_NUM word*/
/*define for page_num_31_0 field*/
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_PAGE_NUM_page_num_31_0_offset: u32 = 4;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_PAGE_NUM_page_num_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_DBIT_WRITE_MEM_PAGE_NUM_page_num_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_DBIT_WRITE_MEM_PAGE_NUM_PAGE_NUM_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_DBIT_WRITE_MEM_PAGE_NUM_page_num_31_0_mask) << SDMA_PKT_POLL_DBIT_WRITE_MEM_PAGE_NUM_page_num_31_0_shift) };
}


/*
** Definitions for SDMA_PKT_POLL_MEM_VERIFY packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_MEM_VERIFY_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_MEM_VERIFY_HEADER_op_mask) << SDMA_PKT_POLL_MEM_VERIFY_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_POLL_MEM_VERIFY_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_MEM_VERIFY_HEADER_sub_op_mask) << SDMA_PKT_POLL_MEM_VERIFY_HEADER_sub_op_shift) };
}

/*define for cache_policy field*/
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_cache_policy_offset: u32 = 0;
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_cache_policy_shift: u32 = 24;
macro_rules! SDMA_PKT_POLL_MEM_VERIFY_HEADER_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_MEM_VERIFY_HEADER_cache_policy_mask) << SDMA_PKT_POLL_MEM_VERIFY_HEADER_cache_policy_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_cpv_shift: u32 = 28;
macro_rules! SDMA_PKT_POLL_MEM_VERIFY_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_MEM_VERIFY_HEADER_cpv_mask) << SDMA_PKT_POLL_MEM_VERIFY_HEADER_cpv_shift) };
}

/*define for mode field*/
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_mode_offset: u32 = 0;
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_mode_mask: u32 = 0x00000001;
pub const SDMA_PKT_POLL_MEM_VERIFY_HEADER_mode_shift: u32 = 31;
macro_rules! SDMA_PKT_POLL_MEM_VERIFY_HEADER_MODE {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_MEM_VERIFY_HEADER_mode_mask) << SDMA_PKT_POLL_MEM_VERIFY_HEADER_mode_shift) };
}

/*define for PATTERN word*/
/*define for pattern field*/
pub const SDMA_PKT_POLL_MEM_VERIFY_PATTERN_pattern_offset: u32 = 1;
pub const SDMA_PKT_POLL_MEM_VERIFY_PATTERN_pattern_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_PATTERN_pattern_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_MEM_VERIFY_PATTERN_PATTERN {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_MEM_VERIFY_PATTERN_pattern_mask) << SDMA_PKT_POLL_MEM_VERIFY_PATTERN_pattern_shift) };
}

/*define for CMP0_ADDR_START_LO word*/
/*define for cmp0_start_31_0 field*/
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_START_LO_cmp0_start_31_0_offset: u32 = 2;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_START_LO_cmp0_start_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_START_LO_cmp0_start_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_START_LO_CMP0_START_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_START_LO_cmp0_start_31_0_mask) << SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_START_LO_cmp0_start_31_0_shift) };
}

/*define for CMP0_ADDR_START_HI word*/
/*define for cmp0_start_63_32 field*/
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_START_HI_cmp0_start_63_32_offset: u32 = 3;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_START_HI_cmp0_start_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_START_HI_cmp0_start_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_START_HI_CMP0_START_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_START_HI_cmp0_start_63_32_mask) << SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_START_HI_cmp0_start_63_32_shift) };
}

/*define for CMP0_ADDR_END_LO word*/
/*define for cmp0_end_31_0 field*/
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_END_LO_cmp0_end_31_0_offset: u32 = 4;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_END_LO_cmp0_end_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_END_LO_cmp0_end_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_END_LO_CMP0_END_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_END_LO_cmp0_end_31_0_mask) << SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_END_LO_cmp0_end_31_0_shift) };
}

/*define for CMP0_ADDR_END_HI word*/
/*define for cmp0_end_63_32 field*/
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_END_HI_cmp0_end_63_32_offset: u32 = 5;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_END_HI_cmp0_end_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_END_HI_cmp0_end_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_END_HI_CMP0_END_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_END_HI_cmp0_end_63_32_mask) << SDMA_PKT_POLL_MEM_VERIFY_CMP0_ADDR_END_HI_cmp0_end_63_32_shift) };
}

/*define for CMP1_ADDR_START_LO word*/
/*define for cmp1_start_31_0 field*/
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_START_LO_cmp1_start_31_0_offset: u32 = 6;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_START_LO_cmp1_start_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_START_LO_cmp1_start_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_START_LO_CMP1_START_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_START_LO_cmp1_start_31_0_mask) << SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_START_LO_cmp1_start_31_0_shift) };
}

/*define for CMP1_ADDR_START_HI word*/
/*define for cmp1_start_63_32 field*/
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_START_HI_cmp1_start_63_32_offset: u32 = 7;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_START_HI_cmp1_start_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_START_HI_cmp1_start_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_START_HI_CMP1_START_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_START_HI_cmp1_start_63_32_mask) << SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_START_HI_cmp1_start_63_32_shift) };
}

/*define for CMP1_ADDR_END_LO word*/
/*define for cmp1_end_31_0 field*/
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_END_LO_cmp1_end_31_0_offset: u32 = 8;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_END_LO_cmp1_end_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_END_LO_cmp1_end_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_END_LO_CMP1_END_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_END_LO_cmp1_end_31_0_mask) << SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_END_LO_cmp1_end_31_0_shift) };
}

/*define for CMP1_ADDR_END_HI word*/
/*define for cmp1_end_63_32 field*/
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_END_HI_cmp1_end_63_32_offset: u32 = 9;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_END_HI_cmp1_end_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_END_HI_cmp1_end_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_END_HI_CMP1_END_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_END_HI_cmp1_end_63_32_mask) << SDMA_PKT_POLL_MEM_VERIFY_CMP1_ADDR_END_HI_cmp1_end_63_32_shift) };
}

/*define for REC_ADDR_LO word*/
/*define for rec_31_0 field*/
pub const SDMA_PKT_POLL_MEM_VERIFY_REC_ADDR_LO_rec_31_0_offset: u32 = 10;
pub const SDMA_PKT_POLL_MEM_VERIFY_REC_ADDR_LO_rec_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_REC_ADDR_LO_rec_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_MEM_VERIFY_REC_ADDR_LO_REC_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_MEM_VERIFY_REC_ADDR_LO_rec_31_0_mask) << SDMA_PKT_POLL_MEM_VERIFY_REC_ADDR_LO_rec_31_0_shift) };
}

/*define for REC_ADDR_HI word*/
/*define for rec_63_32 field*/
pub const SDMA_PKT_POLL_MEM_VERIFY_REC_ADDR_HI_rec_63_32_offset: u32 = 11;
pub const SDMA_PKT_POLL_MEM_VERIFY_REC_ADDR_HI_rec_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_REC_ADDR_HI_rec_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_MEM_VERIFY_REC_ADDR_HI_REC_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_MEM_VERIFY_REC_ADDR_HI_rec_63_32_mask) << SDMA_PKT_POLL_MEM_VERIFY_REC_ADDR_HI_rec_63_32_shift) };
}

/*define for RESERVED word*/
/*define for reserved field*/
pub const SDMA_PKT_POLL_MEM_VERIFY_RESERVED_reserved_offset: u32 = 12;
pub const SDMA_PKT_POLL_MEM_VERIFY_RESERVED_reserved_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_POLL_MEM_VERIFY_RESERVED_reserved_shift: u32 = 0;
macro_rules! SDMA_PKT_POLL_MEM_VERIFY_RESERVED_RESERVED {
    ($x:expr) => { ((($x) & SDMA_PKT_POLL_MEM_VERIFY_RESERVED_reserved_mask) << SDMA_PKT_POLL_MEM_VERIFY_RESERVED_reserved_shift) };
}


/*
** Definitions for SDMA_PKT_ATOMIC packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_ATOMIC_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_ATOMIC_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_ATOMIC_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_ATOMIC_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_ATOMIC_HEADER_op_mask) << SDMA_PKT_ATOMIC_HEADER_op_shift) };
}

/*define for loop field*/
pub const SDMA_PKT_ATOMIC_HEADER_loop_offset: u32 = 0;
pub const SDMA_PKT_ATOMIC_HEADER_loop_mask: u32 = 0x00000001;
pub const SDMA_PKT_ATOMIC_HEADER_loop_shift: u32 = 16;
macro_rules! SDMA_PKT_ATOMIC_HEADER_LOOP {
    ($x:expr) => { ((($x) & SDMA_PKT_ATOMIC_HEADER_loop_mask) << SDMA_PKT_ATOMIC_HEADER_loop_shift) };
}

/*define for tmz field*/
pub const SDMA_PKT_ATOMIC_HEADER_tmz_offset: u32 = 0;
pub const SDMA_PKT_ATOMIC_HEADER_tmz_mask: u32 = 0x00000001;
pub const SDMA_PKT_ATOMIC_HEADER_tmz_shift: u32 = 18;
macro_rules! SDMA_PKT_ATOMIC_HEADER_TMZ {
    ($x:expr) => { ((($x) & SDMA_PKT_ATOMIC_HEADER_tmz_mask) << SDMA_PKT_ATOMIC_HEADER_tmz_shift) };
}

/*define for cache_policy field*/
pub const SDMA_PKT_ATOMIC_HEADER_cache_policy_offset: u32 = 0;
pub const SDMA_PKT_ATOMIC_HEADER_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_PKT_ATOMIC_HEADER_cache_policy_shift: u32 = 20;
macro_rules! SDMA_PKT_ATOMIC_HEADER_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_ATOMIC_HEADER_cache_policy_mask) << SDMA_PKT_ATOMIC_HEADER_cache_policy_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_ATOMIC_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_ATOMIC_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_ATOMIC_HEADER_cpv_shift: u32 = 24;
macro_rules! SDMA_PKT_ATOMIC_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_ATOMIC_HEADER_cpv_mask) << SDMA_PKT_ATOMIC_HEADER_cpv_shift) };
}

/*define for atomic_op field*/
pub const SDMA_PKT_ATOMIC_HEADER_atomic_op_offset: u32 = 0;
pub const SDMA_PKT_ATOMIC_HEADER_atomic_op_mask: u32 = 0x0000007F;
pub const SDMA_PKT_ATOMIC_HEADER_atomic_op_shift: u32 = 25;
macro_rules! SDMA_PKT_ATOMIC_HEADER_ATOMIC_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_ATOMIC_HEADER_atomic_op_mask) << SDMA_PKT_ATOMIC_HEADER_atomic_op_shift) };
}

/*define for ADDR_LO word*/
/*define for addr_31_0 field*/
pub const SDMA_PKT_ATOMIC_ADDR_LO_addr_31_0_offset: u32 = 1;
pub const SDMA_PKT_ATOMIC_ADDR_LO_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_ATOMIC_ADDR_LO_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_ATOMIC_ADDR_LO_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_ATOMIC_ADDR_LO_addr_31_0_mask) << SDMA_PKT_ATOMIC_ADDR_LO_addr_31_0_shift) };
}

/*define for ADDR_HI word*/
/*define for addr_63_32 field*/
pub const SDMA_PKT_ATOMIC_ADDR_HI_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_ATOMIC_ADDR_HI_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_ATOMIC_ADDR_HI_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_ATOMIC_ADDR_HI_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_ATOMIC_ADDR_HI_addr_63_32_mask) << SDMA_PKT_ATOMIC_ADDR_HI_addr_63_32_shift) };
}

/*define for SRC_DATA_LO word*/
/*define for src_data_31_0 field*/
pub const SDMA_PKT_ATOMIC_SRC_DATA_LO_src_data_31_0_offset: u32 = 3;
pub const SDMA_PKT_ATOMIC_SRC_DATA_LO_src_data_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_ATOMIC_SRC_DATA_LO_src_data_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_ATOMIC_SRC_DATA_LO_SRC_DATA_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_ATOMIC_SRC_DATA_LO_src_data_31_0_mask) << SDMA_PKT_ATOMIC_SRC_DATA_LO_src_data_31_0_shift) };
}

/*define for SRC_DATA_HI word*/
/*define for src_data_63_32 field*/
pub const SDMA_PKT_ATOMIC_SRC_DATA_HI_src_data_63_32_offset: u32 = 4;
pub const SDMA_PKT_ATOMIC_SRC_DATA_HI_src_data_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_ATOMIC_SRC_DATA_HI_src_data_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_ATOMIC_SRC_DATA_HI_SRC_DATA_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_ATOMIC_SRC_DATA_HI_src_data_63_32_mask) << SDMA_PKT_ATOMIC_SRC_DATA_HI_src_data_63_32_shift) };
}

/*define for CMP_DATA_LO word*/
/*define for cmp_data_31_0 field*/
pub const SDMA_PKT_ATOMIC_CMP_DATA_LO_cmp_data_31_0_offset: u32 = 5;
pub const SDMA_PKT_ATOMIC_CMP_DATA_LO_cmp_data_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_ATOMIC_CMP_DATA_LO_cmp_data_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_ATOMIC_CMP_DATA_LO_CMP_DATA_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_ATOMIC_CMP_DATA_LO_cmp_data_31_0_mask) << SDMA_PKT_ATOMIC_CMP_DATA_LO_cmp_data_31_0_shift) };
}

/*define for CMP_DATA_HI word*/
/*define for cmp_data_63_32 field*/
pub const SDMA_PKT_ATOMIC_CMP_DATA_HI_cmp_data_63_32_offset: u32 = 6;
pub const SDMA_PKT_ATOMIC_CMP_DATA_HI_cmp_data_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_ATOMIC_CMP_DATA_HI_cmp_data_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_ATOMIC_CMP_DATA_HI_CMP_DATA_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_ATOMIC_CMP_DATA_HI_cmp_data_63_32_mask) << SDMA_PKT_ATOMIC_CMP_DATA_HI_cmp_data_63_32_shift) };
}

/*define for LOOP_INTERVAL word*/
/*define for loop_interval field*/
pub const SDMA_PKT_ATOMIC_LOOP_INTERVAL_loop_interval_offset: u32 = 7;
pub const SDMA_PKT_ATOMIC_LOOP_INTERVAL_loop_interval_mask: u32 = 0x00001FFF;
pub const SDMA_PKT_ATOMIC_LOOP_INTERVAL_loop_interval_shift: u32 = 0;
macro_rules! SDMA_PKT_ATOMIC_LOOP_INTERVAL_LOOP_INTERVAL {
    ($x:expr) => { ((($x) & SDMA_PKT_ATOMIC_LOOP_INTERVAL_loop_interval_mask) << SDMA_PKT_ATOMIC_LOOP_INTERVAL_loop_interval_shift) };
}


/*
** Definitions for SDMA_PKT_TIMESTAMP_SET packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_TIMESTAMP_SET_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_TIMESTAMP_SET_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_TIMESTAMP_SET_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_TIMESTAMP_SET_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_TIMESTAMP_SET_HEADER_op_mask) << SDMA_PKT_TIMESTAMP_SET_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_TIMESTAMP_SET_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_TIMESTAMP_SET_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_TIMESTAMP_SET_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_TIMESTAMP_SET_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_TIMESTAMP_SET_HEADER_sub_op_mask) << SDMA_PKT_TIMESTAMP_SET_HEADER_sub_op_shift) };
}

/*define for INIT_DATA_LO word*/
/*define for init_data_31_0 field*/
pub const SDMA_PKT_TIMESTAMP_SET_INIT_DATA_LO_init_data_31_0_offset: u32 = 1;
pub const SDMA_PKT_TIMESTAMP_SET_INIT_DATA_LO_init_data_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_TIMESTAMP_SET_INIT_DATA_LO_init_data_31_0_shift: u32 = 0;
macro_rules! SDMA_PKT_TIMESTAMP_SET_INIT_DATA_LO_INIT_DATA_31_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_TIMESTAMP_SET_INIT_DATA_LO_init_data_31_0_mask) << SDMA_PKT_TIMESTAMP_SET_INIT_DATA_LO_init_data_31_0_shift) };
}

/*define for INIT_DATA_HI word*/
/*define for init_data_63_32 field*/
pub const SDMA_PKT_TIMESTAMP_SET_INIT_DATA_HI_init_data_63_32_offset: u32 = 2;
pub const SDMA_PKT_TIMESTAMP_SET_INIT_DATA_HI_init_data_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_TIMESTAMP_SET_INIT_DATA_HI_init_data_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_TIMESTAMP_SET_INIT_DATA_HI_INIT_DATA_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_TIMESTAMP_SET_INIT_DATA_HI_init_data_63_32_mask) << SDMA_PKT_TIMESTAMP_SET_INIT_DATA_HI_init_data_63_32_shift) };
}


/*
** Definitions for SDMA_PKT_TIMESTAMP_GET packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_TIMESTAMP_GET_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_TIMESTAMP_GET_HEADER_op_mask) << SDMA_PKT_TIMESTAMP_GET_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_TIMESTAMP_GET_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_TIMESTAMP_GET_HEADER_sub_op_mask) << SDMA_PKT_TIMESTAMP_GET_HEADER_sub_op_shift) };
}

/*define for l2_policy field*/
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_l2_policy_offset: u32 = 0;
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_l2_policy_mask: u32 = 0x00000003;
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_l2_policy_shift: u32 = 24;
macro_rules! SDMA_PKT_TIMESTAMP_GET_HEADER_L2_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_TIMESTAMP_GET_HEADER_l2_policy_mask) << SDMA_PKT_TIMESTAMP_GET_HEADER_l2_policy_shift) };
}

/*define for llc_policy field*/
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_llc_policy_offset: u32 = 0;
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_llc_policy_mask: u32 = 0x00000001;
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_llc_policy_shift: u32 = 26;
macro_rules! SDMA_PKT_TIMESTAMP_GET_HEADER_LLC_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_TIMESTAMP_GET_HEADER_llc_policy_mask) << SDMA_PKT_TIMESTAMP_GET_HEADER_llc_policy_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_TIMESTAMP_GET_HEADER_cpv_shift: u32 = 28;
macro_rules! SDMA_PKT_TIMESTAMP_GET_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_TIMESTAMP_GET_HEADER_cpv_mask) << SDMA_PKT_TIMESTAMP_GET_HEADER_cpv_shift) };
}

/*define for WRITE_ADDR_LO word*/
/*define for write_addr_31_3 field*/
pub const SDMA_PKT_TIMESTAMP_GET_WRITE_ADDR_LO_write_addr_31_3_offset: u32 = 1;
pub const SDMA_PKT_TIMESTAMP_GET_WRITE_ADDR_LO_write_addr_31_3_mask: u32 = 0x1FFFFFFF;
pub const SDMA_PKT_TIMESTAMP_GET_WRITE_ADDR_LO_write_addr_31_3_shift: u32 = 3;
macro_rules! SDMA_PKT_TIMESTAMP_GET_WRITE_ADDR_LO_WRITE_ADDR_31_3 {
    ($x:expr) => { ((($x) & SDMA_PKT_TIMESTAMP_GET_WRITE_ADDR_LO_write_addr_31_3_mask) << SDMA_PKT_TIMESTAMP_GET_WRITE_ADDR_LO_write_addr_31_3_shift) };
}

/*define for WRITE_ADDR_HI word*/
/*define for write_addr_63_32 field*/
pub const SDMA_PKT_TIMESTAMP_GET_WRITE_ADDR_HI_write_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_TIMESTAMP_GET_WRITE_ADDR_HI_write_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_TIMESTAMP_GET_WRITE_ADDR_HI_write_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_TIMESTAMP_GET_WRITE_ADDR_HI_WRITE_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_TIMESTAMP_GET_WRITE_ADDR_HI_write_addr_63_32_mask) << SDMA_PKT_TIMESTAMP_GET_WRITE_ADDR_HI_write_addr_63_32_shift) };
}


/*
** Definitions for SDMA_PKT_TIMESTAMP_GET_GLOBAL packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_op_mask) << SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_sub_op_mask) << SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_sub_op_shift) };
}

/*define for l2_policy field*/
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_l2_policy_offset: u32 = 0;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_l2_policy_mask: u32 = 0x00000003;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_l2_policy_shift: u32 = 24;
macro_rules! SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_L2_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_l2_policy_mask) << SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_l2_policy_shift) };
}

/*define for llc_policy field*/
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_llc_policy_offset: u32 = 0;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_llc_policy_mask: u32 = 0x00000001;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_llc_policy_shift: u32 = 26;
macro_rules! SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_LLC_POLICY {
    ($x:expr) => { ((($x) & SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_llc_policy_mask) << SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_llc_policy_shift) };
}

/*define for cpv field*/
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_cpv_offset: u32 = 0;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_cpv_shift: u32 = 28;
macro_rules! SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_cpv_mask) << SDMA_PKT_TIMESTAMP_GET_GLOBAL_HEADER_cpv_shift) };
}

/*define for WRITE_ADDR_LO word*/
/*define for write_addr_31_3 field*/
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_WRITE_ADDR_LO_write_addr_31_3_offset: u32 = 1;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_WRITE_ADDR_LO_write_addr_31_3_mask: u32 = 0x1FFFFFFF;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_WRITE_ADDR_LO_write_addr_31_3_shift: u32 = 3;
macro_rules! SDMA_PKT_TIMESTAMP_GET_GLOBAL_WRITE_ADDR_LO_WRITE_ADDR_31_3 {
    ($x:expr) => { ((($x) & SDMA_PKT_TIMESTAMP_GET_GLOBAL_WRITE_ADDR_LO_write_addr_31_3_mask) << SDMA_PKT_TIMESTAMP_GET_GLOBAL_WRITE_ADDR_LO_write_addr_31_3_shift) };
}

/*define for WRITE_ADDR_HI word*/
/*define for write_addr_63_32 field*/
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_WRITE_ADDR_HI_write_addr_63_32_offset: u32 = 2;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_WRITE_ADDR_HI_write_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_TIMESTAMP_GET_GLOBAL_WRITE_ADDR_HI_write_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_PKT_TIMESTAMP_GET_GLOBAL_WRITE_ADDR_HI_WRITE_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_TIMESTAMP_GET_GLOBAL_WRITE_ADDR_HI_write_addr_63_32_mask) << SDMA_PKT_TIMESTAMP_GET_GLOBAL_WRITE_ADDR_HI_write_addr_63_32_shift) };
}


/*
** Definitions for SDMA_PKT_TRAP packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_TRAP_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_TRAP_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_TRAP_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_TRAP_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_TRAP_HEADER_op_mask) << SDMA_PKT_TRAP_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_TRAP_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_TRAP_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_TRAP_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_TRAP_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_TRAP_HEADER_sub_op_mask) << SDMA_PKT_TRAP_HEADER_sub_op_shift) };
}

/*define for INT_CONTEXT word*/
/*define for int_context field*/
pub const SDMA_PKT_TRAP_INT_CONTEXT_int_context_offset: u32 = 1;
pub const SDMA_PKT_TRAP_INT_CONTEXT_int_context_mask: u32 = 0x0FFFFFFF;
pub const SDMA_PKT_TRAP_INT_CONTEXT_int_context_shift: u32 = 0;
macro_rules! SDMA_PKT_TRAP_INT_CONTEXT_INT_CONTEXT {
    ($x:expr) => { ((($x) & SDMA_PKT_TRAP_INT_CONTEXT_int_conte$xt_mask) << SDMA_PKT_TRAP_INT_CONTEXT_int_conte$xt_shift) };
}


/*
** Definitions for SDMA_PKT_DUMMY_TRAP packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_DUMMY_TRAP_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_DUMMY_TRAP_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_DUMMY_TRAP_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_DUMMY_TRAP_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_DUMMY_TRAP_HEADER_op_mask) << SDMA_PKT_DUMMY_TRAP_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_DUMMY_TRAP_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_DUMMY_TRAP_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_DUMMY_TRAP_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_DUMMY_TRAP_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_DUMMY_TRAP_HEADER_sub_op_mask) << SDMA_PKT_DUMMY_TRAP_HEADER_sub_op_shift) };
}

/*define for INT_CONTEXT word*/
/*define for int_context field*/
pub const SDMA_PKT_DUMMY_TRAP_INT_CONTEXT_int_context_offset: u32 = 1;
pub const SDMA_PKT_DUMMY_TRAP_INT_CONTEXT_int_context_mask: u32 = 0x0FFFFFFF;
pub const SDMA_PKT_DUMMY_TRAP_INT_CONTEXT_int_context_shift: u32 = 0;
macro_rules! SDMA_PKT_DUMMY_TRAP_INT_CONTEXT_INT_CONTEXT {
    ($x:expr) => { ((($x) & SDMA_PKT_DUMMY_TRAP_INT_CONTEXT_int_conte$xt_mask) << SDMA_PKT_DUMMY_TRAP_INT_CONTEXT_int_conte$xt_shift) };
}


/*
** Definitions for SDMA_PKT_GPUVM_INV packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_GPUVM_INV_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_GPUVM_INV_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_GPUVM_INV_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_GPUVM_INV_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_GPUVM_INV_HEADER_op_mask) << SDMA_PKT_GPUVM_INV_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_GPUVM_INV_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_GPUVM_INV_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_GPUVM_INV_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_GPUVM_INV_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_GPUVM_INV_HEADER_sub_op_mask) << SDMA_PKT_GPUVM_INV_HEADER_sub_op_shift) };
}

/*define for PAYLOAD1 word*/
/*define for per_vmid_inv_req field*/
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_per_vmid_inv_req_offset: u32 = 1;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_per_vmid_inv_req_mask: u32 = 0x0000FFFF;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_per_vmid_inv_req_shift: u32 = 0;
macro_rules! SDMA_PKT_GPUVM_INV_PAYLOAD1_PER_VMID_INV_REQ {
    ($x:expr) => { ((($x) & SDMA_PKT_GPUVM_INV_PAYLOAD1_per_vmid_inv_req_mask) << SDMA_PKT_GPUVM_INV_PAYLOAD1_per_vmid_inv_req_shift) };
}

/*define for flush_type field*/
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_flush_type_offset: u32 = 1;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_flush_type_mask: u32 = 0x00000007;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_flush_type_shift: u32 = 16;
macro_rules! SDMA_PKT_GPUVM_INV_PAYLOAD1_FLUSH_TYPE {
    ($x:expr) => { ((($x) & SDMA_PKT_GPUVM_INV_PAYLOAD1_flush_type_mask) << SDMA_PKT_GPUVM_INV_PAYLOAD1_flush_type_shift) };
}

/*define for l2_ptes field*/
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_l2_ptes_offset: u32 = 1;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_l2_ptes_mask: u32 = 0x00000001;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_l2_ptes_shift: u32 = 19;
macro_rules! SDMA_PKT_GPUVM_INV_PAYLOAD1_L2_PTES {
    ($x:expr) => { ((($x) & SDMA_PKT_GPUVM_INV_PAYLOAD1_l2_ptes_mask) << SDMA_PKT_GPUVM_INV_PAYLOAD1_l2_ptes_shift) };
}

/*define for l2_pde0 field*/
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_l2_pde0_offset: u32 = 1;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_l2_pde0_mask: u32 = 0x00000001;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_l2_pde0_shift: u32 = 20;
macro_rules! SDMA_PKT_GPUVM_INV_PAYLOAD1_L2_PDE0 {
    ($x:expr) => { ((($x) & SDMA_PKT_GPUVM_INV_PAYLOAD1_l2_pde0_mask) << SDMA_PKT_GPUVM_INV_PAYLOAD1_l2_pde0_shift) };
}

/*define for l2_pde1 field*/
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_l2_pde1_offset: u32 = 1;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_l2_pde1_mask: u32 = 0x00000001;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_l2_pde1_shift: u32 = 21;
macro_rules! SDMA_PKT_GPUVM_INV_PAYLOAD1_L2_PDE1 {
    ($x:expr) => { ((($x) & SDMA_PKT_GPUVM_INV_PAYLOAD1_l2_pde1_mask) << SDMA_PKT_GPUVM_INV_PAYLOAD1_l2_pde1_shift) };
}

/*define for l2_pde2 field*/
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_l2_pde2_offset: u32 = 1;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_l2_pde2_mask: u32 = 0x00000001;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_l2_pde2_shift: u32 = 22;
macro_rules! SDMA_PKT_GPUVM_INV_PAYLOAD1_L2_PDE2 {
    ($x:expr) => { ((($x) & SDMA_PKT_GPUVM_INV_PAYLOAD1_l2_pde2_mask) << SDMA_PKT_GPUVM_INV_PAYLOAD1_l2_pde2_shift) };
}

/*define for l1_ptes field*/
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_l1_ptes_offset: u32 = 1;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_l1_ptes_mask: u32 = 0x00000001;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_l1_ptes_shift: u32 = 23;
macro_rules! SDMA_PKT_GPUVM_INV_PAYLOAD1_L1_PTES {
    ($x:expr) => { ((($x) & SDMA_PKT_GPUVM_INV_PAYLOAD1_l1_ptes_mask) << SDMA_PKT_GPUVM_INV_PAYLOAD1_l1_ptes_shift) };
}

/*define for clr_protection_fault_status_addr field*/
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_clr_protection_fault_status_addr_offset: u32 = 1;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_clr_protection_fault_status_addr_mask: u32 = 0x00000001;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_clr_protection_fault_status_addr_shift: u32 = 24;
macro_rules! SDMA_PKT_GPUVM_INV_PAYLOAD1_CLR_PROTECTION_FAULT_STATUS_ADDR {
    ($x:expr) => { ((($x) & SDMA_PKT_GPUVM_INV_PAYLOAD1_clr_protection_fault_status_addr_mask) << SDMA_PKT_GPUVM_INV_PAYLOAD1_clr_protection_fault_status_addr_shift) };
}

/*define for log_request field*/
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_log_request_offset: u32 = 1;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_log_request_mask: u32 = 0x00000001;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_log_request_shift: u32 = 25;
macro_rules! SDMA_PKT_GPUVM_INV_PAYLOAD1_LOG_REQUEST {
    ($x:expr) => { ((($x) & SDMA_PKT_GPUVM_INV_PAYLOAD1_log_request_mask) << SDMA_PKT_GPUVM_INV_PAYLOAD1_log_request_shift) };
}

/*define for four_kilobytes field*/
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_four_kilobytes_offset: u32 = 1;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_four_kilobytes_mask: u32 = 0x00000001;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD1_four_kilobytes_shift: u32 = 26;
macro_rules! SDMA_PKT_GPUVM_INV_PAYLOAD1_FOUR_KILOBYTES {
    ($x:expr) => { ((($x) & SDMA_PKT_GPUVM_INV_PAYLOAD1_four_kilobytes_mask) << SDMA_PKT_GPUVM_INV_PAYLOAD1_four_kilobytes_shift) };
}

/*define for PAYLOAD2 word*/
/*define for s field*/
pub const SDMA_PKT_GPUVM_INV_PAYLOAD2_s_offset: u32 = 2;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD2_s_mask: u32 = 0x00000001;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD2_s_shift: u32 = 0;
macro_rules! SDMA_PKT_GPUVM_INV_PAYLOAD2_S {
    ($x:expr) => { ((($x) & SDMA_PKT_GPUVM_INV_PAYLOAD2_s_mask) << SDMA_PKT_GPUVM_INV_PAYLOAD2_s_shift) };
}

/*define for page_va_42_12 field*/
pub const SDMA_PKT_GPUVM_INV_PAYLOAD2_page_va_42_12_offset: u32 = 2;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD2_page_va_42_12_mask: u32 = 0x7FFFFFFF;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD2_page_va_42_12_shift: u32 = 1;
macro_rules! SDMA_PKT_GPUVM_INV_PAYLOAD2_PAGE_VA_42_12 {
    ($x:expr) => { ((($x) & SDMA_PKT_GPUVM_INV_PAYLOAD2_page_va_42_12_mask) << SDMA_PKT_GPUVM_INV_PAYLOAD2_page_va_42_12_shift) };
}

/*define for PAYLOAD3 word*/
/*define for page_va_47_43 field*/
pub const SDMA_PKT_GPUVM_INV_PAYLOAD3_page_va_47_43_offset: u32 = 3;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD3_page_va_47_43_mask: u32 = 0x0000003F;
pub const SDMA_PKT_GPUVM_INV_PAYLOAD3_page_va_47_43_shift: u32 = 0;
macro_rules! SDMA_PKT_GPUVM_INV_PAYLOAD3_PAGE_VA_47_43 {
    ($x:expr) => { ((($x) & SDMA_PKT_GPUVM_INV_PAYLOAD3_page_va_47_43_mask) << SDMA_PKT_GPUVM_INV_PAYLOAD3_page_va_47_43_shift) };
}


/*
** Definitions for SDMA_PKT_GCR_REQ packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_GCR_REQ_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_GCR_REQ_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_GCR_REQ_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_GCR_REQ_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_GCR_REQ_HEADER_op_mask) << SDMA_PKT_GCR_REQ_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_GCR_REQ_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_GCR_REQ_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_GCR_REQ_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_GCR_REQ_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_GCR_REQ_HEADER_sub_op_mask) << SDMA_PKT_GCR_REQ_HEADER_sub_op_shift) };
}

/*define for PAYLOAD1 word*/
/*define for base_va_31_7 field*/
pub const SDMA_PKT_GCR_REQ_PAYLOAD1_base_va_31_7_offset: u32 = 1;
pub const SDMA_PKT_GCR_REQ_PAYLOAD1_base_va_31_7_mask: u32 = 0x01FFFFFF;
pub const SDMA_PKT_GCR_REQ_PAYLOAD1_base_va_31_7_shift: u32 = 7;
macro_rules! SDMA_PKT_GCR_REQ_PAYLOAD1_BASE_VA_31_7 {
    ($x:expr) => { ((($x) & SDMA_PKT_GCR_REQ_PAYLOAD1_base_va_31_7_mask) << SDMA_PKT_GCR_REQ_PAYLOAD1_base_va_31_7_shift) };
}

/*define for PAYLOAD2 word*/
/*define for base_va_56_32 field*/
pub const SDMA_PKT_GCR_REQ_PAYLOAD2_base_va_56_32_offset: u32 = 2;
pub const SDMA_PKT_GCR_REQ_PAYLOAD2_base_va_56_32_mask: u32 = 0x00FFFFFF;
pub const SDMA_PKT_GCR_REQ_PAYLOAD2_base_va_56_32_shift: u32 = 0;
macro_rules! SDMA_PKT_GCR_REQ_PAYLOAD2_BASE_VA_56_32 {
    ($x:expr) => { ((($x) & SDMA_PKT_GCR_REQ_PAYLOAD2_base_va_56_32_mask) << SDMA_PKT_GCR_REQ_PAYLOAD2_base_va_56_32_shift) };
}

/*define for PAYLOAD3 word*/
/*define for gcr_control_18_0 field*/
pub const SDMA_PKT_GCR_REQ_PAYLOAD3_gcr_control_18_0_offset: u32 = 3;
pub const SDMA_PKT_GCR_REQ_PAYLOAD3_gcr_control_18_0_mask: u32 = 0x0007FFFF;
pub const SDMA_PKT_GCR_REQ_PAYLOAD3_gcr_control_18_0_shift: u32 = 0;
macro_rules! SDMA_PKT_GCR_REQ_PAYLOAD3_GCR_CONTROL_18_0 {
    ($x:expr) => { ((($x) & SDMA_PKT_GCR_REQ_PAYLOAD3_gcr_control_18_0_mask) << SDMA_PKT_GCR_REQ_PAYLOAD3_gcr_control_18_0_shift) };
}

/*define for limit_va_15_7 field*/
pub const SDMA_PKT_GCR_REQ_PAYLOAD3_limit_va_15_7_offset: u32 = 3;
pub const SDMA_PKT_GCR_REQ_PAYLOAD3_limit_va_15_7_mask: u32 = 0x000001FF;
pub const SDMA_PKT_GCR_REQ_PAYLOAD3_limit_va_15_7_shift: u32 = 23;
macro_rules! SDMA_PKT_GCR_REQ_PAYLOAD3_LIMIT_VA_15_7 {
    ($x:expr) => { ((($x) & SDMA_PKT_GCR_REQ_PAYLOAD3_limit_va_15_7_mask) << SDMA_PKT_GCR_REQ_PAYLOAD3_limit_va_15_7_shift) };
}

/*define for PAYLOAD4 word*/
/*define for limit_va_47_16 field*/
pub const SDMA_PKT_GCR_REQ_PAYLOAD4_limit_va_47_16_offset: u32 = 4;
pub const SDMA_PKT_GCR_REQ_PAYLOAD4_limit_va_47_16_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_GCR_REQ_PAYLOAD4_limit_va_47_16_shift: u32 = 0;
macro_rules! SDMA_PKT_GCR_REQ_PAYLOAD4_LIMIT_VA_47_16 {
    ($x:expr) => { ((($x) & SDMA_PKT_GCR_REQ_PAYLOAD4_limit_va_47_16_mask) << SDMA_PKT_GCR_REQ_PAYLOAD4_limit_va_47_16_shift) };
}

/*define for PAYLOAD5 word*/
/*define for limit_va_56_48 field*/
pub const SDMA_PKT_GCR_REQ_PAYLOAD5_limit_va_56_48_offset: u32 = 5;
pub const SDMA_PKT_GCR_REQ_PAYLOAD5_limit_va_56_48_mask: u32 = 0x000001FF;
pub const SDMA_PKT_GCR_REQ_PAYLOAD5_limit_va_56_48_shift: u32 = 0;
macro_rules! SDMA_PKT_GCR_REQ_PAYLOAD5_LIMIT_VA_56_48 {
    ($x:expr) => { ((($x) & SDMA_PKT_GCR_REQ_PAYLOAD5_limit_va_56_48_mask) << SDMA_PKT_GCR_REQ_PAYLOAD5_limit_va_56_48_shift) };
}

/*define for vmid field*/
pub const SDMA_PKT_GCR_REQ_PAYLOAD5_vmid_offset: u32 = 5;
pub const SDMA_PKT_GCR_REQ_PAYLOAD5_vmid_mask: u32 = 0x0000000F;
pub const SDMA_PKT_GCR_REQ_PAYLOAD5_vmid_shift: u32 = 26;
macro_rules! SDMA_PKT_GCR_REQ_PAYLOAD5_VMID {
    ($x:expr) => { ((($x) & SDMA_PKT_GCR_REQ_PAYLOAD5_vmid_mask) << SDMA_PKT_GCR_REQ_PAYLOAD5_vmid_shift) };
}


/*
** Definitions for SDMA_PKT_NOP packet
*/

/*define for HEADER word*/
/*define for op field*/
pub const SDMA_PKT_NOP_HEADER_op_offset: u32 = 0;
pub const SDMA_PKT_NOP_HEADER_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_NOP_HEADER_op_shift: u32 = 0;
macro_rules! SDMA_PKT_NOP_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_NOP_HEADER_op_mask) << SDMA_PKT_NOP_HEADER_op_shift) };
}

/*define for sub_op field*/
pub const SDMA_PKT_NOP_HEADER_sub_op_offset: u32 = 0;
pub const SDMA_PKT_NOP_HEADER_sub_op_mask: u32 = 0x000000FF;
pub const SDMA_PKT_NOP_HEADER_sub_op_shift: u32 = 8;
macro_rules! SDMA_PKT_NOP_HEADER_SUB_OP {
    ($x:expr) => { ((($x) & SDMA_PKT_NOP_HEADER_sub_op_mask) << SDMA_PKT_NOP_HEADER_sub_op_shift) };
}

/*define for count field*/
pub const SDMA_PKT_NOP_HEADER_count_offset: u32 = 0;
pub const SDMA_PKT_NOP_HEADER_count_mask: u32 = 0x00003FFF;
pub const SDMA_PKT_NOP_HEADER_count_shift: u32 = 16;
macro_rules! SDMA_PKT_NOP_HEADER_COUNT {
    ($x:expr) => { ((($x) & SDMA_PKT_NOP_HEADER_count_mask) << SDMA_PKT_NOP_HEADER_count_shift) };
}

/*define for DATA0 word*/
/*define for data0 field*/
pub const SDMA_PKT_NOP_DATA0_data0_offset: u32 = 1;
pub const SDMA_PKT_NOP_DATA0_data0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_PKT_NOP_DATA0_data0_shift: u32 = 0;
macro_rules! SDMA_PKT_NOP_DATA0_DATA0 {
    ($x:expr) => { ((($x) & SDMA_PKT_NOP_DATA0_data0_mask) << SDMA_PKT_NOP_DATA0_data0_shift) };
}


/*
** Definitions for SDMA_AQL_PKT_HEADER packet
*/

/*define for HEADER word*/
/*define for format field*/
pub const SDMA_AQL_PKT_HEADER_HEADER_format_offset: u32 = 0;
pub const SDMA_AQL_PKT_HEADER_HEADER_format_mask: u32 = 0x000000FF;
pub const SDMA_AQL_PKT_HEADER_HEADER_format_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_HEADER_HEADER_FORMAT {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_HEADER_HEADER_format_mask) << SDMA_AQL_PKT_HEADER_HEADER_format_shift) };
}

/*define for barrier field*/
pub const SDMA_AQL_PKT_HEADER_HEADER_barrier_offset: u32 = 0;
pub const SDMA_AQL_PKT_HEADER_HEADER_barrier_mask: u32 = 0x00000001;
pub const SDMA_AQL_PKT_HEADER_HEADER_barrier_shift: u32 = 8;
macro_rules! SDMA_AQL_PKT_HEADER_HEADER_BARRIER {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_HEADER_HEADER_barrier_mask) << SDMA_AQL_PKT_HEADER_HEADER_barrier_shift) };
}

/*define for acquire_fence_scope field*/
pub const SDMA_AQL_PKT_HEADER_HEADER_acquire_fence_scope_offset: u32 = 0;
pub const SDMA_AQL_PKT_HEADER_HEADER_acquire_fence_scope_mask: u32 = 0x00000003;
pub const SDMA_AQL_PKT_HEADER_HEADER_acquire_fence_scope_shift: u32 = 9;
macro_rules! SDMA_AQL_PKT_HEADER_HEADER_ACQUIRE_FENCE_SCOPE {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_HEADER_HEADER_acquire_fence_scope_mask) << SDMA_AQL_PKT_HEADER_HEADER_acquire_fence_scope_shift) };
}

/*define for release_fence_scope field*/
pub const SDMA_AQL_PKT_HEADER_HEADER_release_fence_scope_offset: u32 = 0;
pub const SDMA_AQL_PKT_HEADER_HEADER_release_fence_scope_mask: u32 = 0x00000003;
pub const SDMA_AQL_PKT_HEADER_HEADER_release_fence_scope_shift: u32 = 11;
macro_rules! SDMA_AQL_PKT_HEADER_HEADER_RELEASE_FENCE_SCOPE {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_HEADER_HEADER_release_fence_scope_mask) << SDMA_AQL_PKT_HEADER_HEADER_release_fence_scope_shift) };
}

/*define for reserved field*/
pub const SDMA_AQL_PKT_HEADER_HEADER_reserved_offset: u32 = 0;
pub const SDMA_AQL_PKT_HEADER_HEADER_reserved_mask: u32 = 0x00000007;
pub const SDMA_AQL_PKT_HEADER_HEADER_reserved_shift: u32 = 13;
macro_rules! SDMA_AQL_PKT_HEADER_HEADER_RESERVED {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_HEADER_HEADER_reserved_mask) << SDMA_AQL_PKT_HEADER_HEADER_reserved_shift) };
}

/*define for op field*/
pub const SDMA_AQL_PKT_HEADER_HEADER_op_offset: u32 = 0;
pub const SDMA_AQL_PKT_HEADER_HEADER_op_mask: u32 = 0x0000000F;
pub const SDMA_AQL_PKT_HEADER_HEADER_op_shift: u32 = 16;
macro_rules! SDMA_AQL_PKT_HEADER_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_HEADER_HEADER_op_mask) << SDMA_AQL_PKT_HEADER_HEADER_op_shift) };
}

/*define for subop field*/
pub const SDMA_AQL_PKT_HEADER_HEADER_subop_offset: u32 = 0;
pub const SDMA_AQL_PKT_HEADER_HEADER_subop_mask: u32 = 0x00000007;
pub const SDMA_AQL_PKT_HEADER_HEADER_subop_shift: u32 = 20;
macro_rules! SDMA_AQL_PKT_HEADER_HEADER_SUBOP {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_HEADER_HEADER_subop_mask) << SDMA_AQL_PKT_HEADER_HEADER_subop_shift) };
}

/*define for cpv field*/
pub const SDMA_AQL_PKT_HEADER_HEADER_cpv_offset: u32 = 0;
pub const SDMA_AQL_PKT_HEADER_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_AQL_PKT_HEADER_HEADER_cpv_shift: u32 = 28;
macro_rules! SDMA_AQL_PKT_HEADER_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_HEADER_HEADER_cpv_mask) << SDMA_AQL_PKT_HEADER_HEADER_cpv_shift) };
}


/*
** Definitions for SDMA_AQL_PKT_COPY_LINEAR packet
*/

/*define for HEADER word*/
/*define for format field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_format_offset: u32 = 0;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_format_mask: u32 = 0x000000FF;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_format_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_HEADER_FORMAT {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_HEADER_format_mask) << SDMA_AQL_PKT_COPY_LINEAR_HEADER_format_shift) };
}

/*define for barrier field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_barrier_offset: u32 = 0;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_barrier_mask: u32 = 0x00000001;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_barrier_shift: u32 = 8;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_HEADER_BARRIER {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_HEADER_barrier_mask) << SDMA_AQL_PKT_COPY_LINEAR_HEADER_barrier_shift) };
}

/*define for acquire_fence_scope field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_acquire_fence_scope_offset: u32 = 0;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_acquire_fence_scope_mask: u32 = 0x00000003;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_acquire_fence_scope_shift: u32 = 9;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_HEADER_ACQUIRE_FENCE_SCOPE {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_HEADER_acquire_fence_scope_mask) << SDMA_AQL_PKT_COPY_LINEAR_HEADER_acquire_fence_scope_shift) };
}

/*define for release_fence_scope field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_release_fence_scope_offset: u32 = 0;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_release_fence_scope_mask: u32 = 0x00000003;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_release_fence_scope_shift: u32 = 11;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_HEADER_RELEASE_FENCE_SCOPE {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_HEADER_release_fence_scope_mask) << SDMA_AQL_PKT_COPY_LINEAR_HEADER_release_fence_scope_shift) };
}

/*define for reserved field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_reserved_offset: u32 = 0;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_reserved_mask: u32 = 0x00000007;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_reserved_shift: u32 = 13;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_HEADER_RESERVED {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_HEADER_reserved_mask) << SDMA_AQL_PKT_COPY_LINEAR_HEADER_reserved_shift) };
}

/*define for op field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_op_offset: u32 = 0;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_op_mask: u32 = 0x0000000F;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_op_shift: u32 = 16;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_HEADER_op_mask) << SDMA_AQL_PKT_COPY_LINEAR_HEADER_op_shift) };
}

/*define for subop field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_subop_offset: u32 = 0;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_subop_mask: u32 = 0x00000007;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_subop_shift: u32 = 20;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_HEADER_SUBOP {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_HEADER_subop_mask) << SDMA_AQL_PKT_COPY_LINEAR_HEADER_subop_shift) };
}

/*define for cpv field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_cpv_offset: u32 = 0;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_AQL_PKT_COPY_LINEAR_HEADER_cpv_shift: u32 = 28;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_HEADER_cpv_mask) << SDMA_AQL_PKT_COPY_LINEAR_HEADER_cpv_shift) };
}

/*define for RESERVED_DW1 word*/
/*define for reserved_dw1 field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW1_reserved_dw1_offset: u32 = 1;
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW1_reserved_dw1_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW1_reserved_dw1_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW1_RESERVED_DW1 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW1_reserved_dw1_mask) << SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW1_reserved_dw1_shift) };
}

/*define for RETURN_ADDR_LO word*/
/*define for return_addr_31_0 field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_RETURN_ADDR_LO_return_addr_31_0_offset: u32 = 2;
pub const SDMA_AQL_PKT_COPY_LINEAR_RETURN_ADDR_LO_return_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_RETURN_ADDR_LO_return_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_RETURN_ADDR_LO_RETURN_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_RETURN_ADDR_LO_return_addr_31_0_mask) << SDMA_AQL_PKT_COPY_LINEAR_RETURN_ADDR_LO_return_addr_31_0_shift) };
}

/*define for RETURN_ADDR_HI word*/
/*define for return_addr_63_32 field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_RETURN_ADDR_HI_return_addr_63_32_offset: u32 = 3;
pub const SDMA_AQL_PKT_COPY_LINEAR_RETURN_ADDR_HI_return_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_RETURN_ADDR_HI_return_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_RETURN_ADDR_HI_RETURN_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_RETURN_ADDR_HI_return_addr_63_32_mask) << SDMA_AQL_PKT_COPY_LINEAR_RETURN_ADDR_HI_return_addr_63_32_shift) };
}

/*define for COUNT word*/
/*define for count field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_COUNT_count_offset: u32 = 4;
pub const SDMA_AQL_PKT_COPY_LINEAR_COUNT_count_mask: u32 = 0x003FFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_COUNT_count_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_COUNT_COUNT {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_COUNT_count_mask) << SDMA_AQL_PKT_COPY_LINEAR_COUNT_count_shift) };
}

/*define for PARAMETER word*/
/*define for dst_sw field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_dst_sw_offset: u32 = 5;
pub const SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_dst_sw_mask: u32 = 0x00000003;
pub const SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_dst_sw_shift: u32 = 16;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_DST_SW {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_dst_sw_mask) << SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_dst_sw_shift) };
}

/*define for dst_cache_policy field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_dst_cache_policy_offset: u32 = 5;
pub const SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_dst_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_dst_cache_policy_shift: u32 = 18;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_DST_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_dst_cache_policy_mask) << SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_dst_cache_policy_shift) };
}

/*define for src_sw field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_src_sw_offset: u32 = 5;
pub const SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_src_sw_mask: u32 = 0x00000003;
pub const SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_src_sw_shift: u32 = 24;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_SRC_SW {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_src_sw_mask) << SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_src_sw_shift) };
}

/*define for src_cache_policy field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_src_cache_policy_offset: u32 = 5;
pub const SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_src_cache_policy_mask: u32 = 0x00000007;
pub const SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_src_cache_policy_shift: u32 = 26;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_SRC_CACHE_POLICY {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_src_cache_policy_mask) << SDMA_AQL_PKT_COPY_LINEAR_PARAMETER_src_cache_policy_shift) };
}

/*define for SRC_ADDR_LO word*/
/*define for src_addr_31_0 field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_SRC_ADDR_LO_src_addr_31_0_offset: u32 = 6;
pub const SDMA_AQL_PKT_COPY_LINEAR_SRC_ADDR_LO_src_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_SRC_ADDR_LO_src_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_SRC_ADDR_LO_SRC_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_SRC_ADDR_LO_src_addr_31_0_mask) << SDMA_AQL_PKT_COPY_LINEAR_SRC_ADDR_LO_src_addr_31_0_shift) };
}

/*define for SRC_ADDR_HI word*/
/*define for src_addr_63_32 field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_SRC_ADDR_HI_src_addr_63_32_offset: u32 = 7;
pub const SDMA_AQL_PKT_COPY_LINEAR_SRC_ADDR_HI_src_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_SRC_ADDR_HI_src_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_SRC_ADDR_HI_SRC_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_SRC_ADDR_HI_src_addr_63_32_mask) << SDMA_AQL_PKT_COPY_LINEAR_SRC_ADDR_HI_src_addr_63_32_shift) };
}

/*define for DST_ADDR_LO word*/
/*define for dst_addr_31_0 field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_DST_ADDR_LO_dst_addr_31_0_offset: u32 = 8;
pub const SDMA_AQL_PKT_COPY_LINEAR_DST_ADDR_LO_dst_addr_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_DST_ADDR_LO_dst_addr_31_0_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_DST_ADDR_LO_DST_ADDR_31_0 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_DST_ADDR_LO_dst_addr_31_0_mask) << SDMA_AQL_PKT_COPY_LINEAR_DST_ADDR_LO_dst_addr_31_0_shift) };
}

/*define for DST_ADDR_HI word*/
/*define for dst_addr_63_32 field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_DST_ADDR_HI_dst_addr_63_32_offset: u32 = 9;
pub const SDMA_AQL_PKT_COPY_LINEAR_DST_ADDR_HI_dst_addr_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_DST_ADDR_HI_dst_addr_63_32_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_DST_ADDR_HI_DST_ADDR_63_32 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_DST_ADDR_HI_dst_addr_63_32_mask) << SDMA_AQL_PKT_COPY_LINEAR_DST_ADDR_HI_dst_addr_63_32_shift) };
}

/*define for RESERVED_DW10 word*/
/*define for reserved_dw10 field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW10_reserved_dw10_offset: u32 = 10;
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW10_reserved_dw10_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW10_reserved_dw10_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW10_RESERVED_DW10 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW10_reserved_dw10_mask) << SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW10_reserved_dw10_shift) };
}

/*define for RESERVED_DW11 word*/
/*define for reserved_dw11 field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW11_reserved_dw11_offset: u32 = 11;
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW11_reserved_dw11_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW11_reserved_dw11_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW11_RESERVED_DW11 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW11_reserved_dw11_mask) << SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW11_reserved_dw11_shift) };
}

/*define for RESERVED_DW12 word*/
/*define for reserved_dw12 field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW12_reserved_dw12_offset: u32 = 12;
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW12_reserved_dw12_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW12_reserved_dw12_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW12_RESERVED_DW12 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW12_reserved_dw12_mask) << SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW12_reserved_dw12_shift) };
}

/*define for RESERVED_DW13 word*/
/*define for reserved_dw13 field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW13_reserved_dw13_offset: u32 = 13;
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW13_reserved_dw13_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW13_reserved_dw13_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW13_RESERVED_DW13 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW13_reserved_dw13_mask) << SDMA_AQL_PKT_COPY_LINEAR_RESERVED_DW13_reserved_dw13_shift) };
}

/*define for COMPLETION_SIGNAL_LO word*/
/*define for completion_signal_31_0 field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_COMPLETION_SIGNAL_LO_completion_signal_31_0_offset: u32 = 14;
pub const SDMA_AQL_PKT_COPY_LINEAR_COMPLETION_SIGNAL_LO_completion_signal_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_COMPLETION_SIGNAL_LO_completion_signal_31_0_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_COMPLETION_SIGNAL_LO_COMPLETION_SIGNAL_31_0 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_COMPLETION_SIGNAL_LO_completion_signal_31_0_mask) << SDMA_AQL_PKT_COPY_LINEAR_COMPLETION_SIGNAL_LO_completion_signal_31_0_shift) };
}

/*define for COMPLETION_SIGNAL_HI word*/
/*define for completion_signal_63_32 field*/
pub const SDMA_AQL_PKT_COPY_LINEAR_COMPLETION_SIGNAL_HI_completion_signal_63_32_offset: u32 = 15;
pub const SDMA_AQL_PKT_COPY_LINEAR_COMPLETION_SIGNAL_HI_completion_signal_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_COPY_LINEAR_COMPLETION_SIGNAL_HI_completion_signal_63_32_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_COPY_LINEAR_COMPLETION_SIGNAL_HI_COMPLETION_SIGNAL_63_32 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_COPY_LINEAR_COMPLETION_SIGNAL_HI_completion_signal_63_32_mask) << SDMA_AQL_PKT_COPY_LINEAR_COMPLETION_SIGNAL_HI_completion_signal_63_32_shift) };
}


/*
** Definitions for SDMA_AQL_PKT_BARRIER_OR packet
*/

/*define for HEADER word*/
/*define for format field*/
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_format_offset: u32 = 0;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_format_mask: u32 = 0x000000FF;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_format_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_HEADER_FORMAT {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_HEADER_format_mask) << SDMA_AQL_PKT_BARRIER_OR_HEADER_format_shift) };
}

/*define for barrier field*/
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_barrier_offset: u32 = 0;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_barrier_mask: u32 = 0x00000001;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_barrier_shift: u32 = 8;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_HEADER_BARRIER {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_HEADER_barrier_mask) << SDMA_AQL_PKT_BARRIER_OR_HEADER_barrier_shift) };
}

/*define for acquire_fence_scope field*/
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_acquire_fence_scope_offset: u32 = 0;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_acquire_fence_scope_mask: u32 = 0x00000003;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_acquire_fence_scope_shift: u32 = 9;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_HEADER_ACQUIRE_FENCE_SCOPE {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_HEADER_acquire_fence_scope_mask) << SDMA_AQL_PKT_BARRIER_OR_HEADER_acquire_fence_scope_shift) };
}

/*define for release_fence_scope field*/
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_release_fence_scope_offset: u32 = 0;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_release_fence_scope_mask: u32 = 0x00000003;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_release_fence_scope_shift: u32 = 11;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_HEADER_RELEASE_FENCE_SCOPE {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_HEADER_release_fence_scope_mask) << SDMA_AQL_PKT_BARRIER_OR_HEADER_release_fence_scope_shift) };
}

/*define for reserved field*/
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_reserved_offset: u32 = 0;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_reserved_mask: u32 = 0x00000007;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_reserved_shift: u32 = 13;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_HEADER_RESERVED {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_HEADER_reserved_mask) << SDMA_AQL_PKT_BARRIER_OR_HEADER_reserved_shift) };
}

/*define for op field*/
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_op_offset: u32 = 0;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_op_mask: u32 = 0x0000000F;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_op_shift: u32 = 16;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_HEADER_OP {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_HEADER_op_mask) << SDMA_AQL_PKT_BARRIER_OR_HEADER_op_shift) };
}

/*define for subop field*/
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_subop_offset: u32 = 0;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_subop_mask: u32 = 0x00000007;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_subop_shift: u32 = 20;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_HEADER_SUBOP {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_HEADER_subop_mask) << SDMA_AQL_PKT_BARRIER_OR_HEADER_subop_shift) };
}

/*define for cpv field*/
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_cpv_offset: u32 = 0;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_cpv_mask: u32 = 0x00000001;
pub const SDMA_AQL_PKT_BARRIER_OR_HEADER_cpv_shift: u32 = 28;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_HEADER_CPV {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_HEADER_cpv_mask) << SDMA_AQL_PKT_BARRIER_OR_HEADER_cpv_shift) };
}

/*define for RESERVED_DW1 word*/
/*define for reserved_dw1 field*/
pub const SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW1_reserved_dw1_offset: u32 = 1;
pub const SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW1_reserved_dw1_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW1_reserved_dw1_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW1_RESERVED_DW1 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW1_reserved_dw1_mask) << SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW1_reserved_dw1_shift) };
}

/*define for DEPENDENT_ADDR_0_LO word*/
/*define for dependent_addr_0_31_0 field*/
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_0_LO_dependent_addr_0_31_0_offset: u32 = 2;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_0_LO_dependent_addr_0_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_0_LO_dependent_addr_0_31_0_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_0_LO_DEPENDENT_ADDR_0_31_0 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_0_LO_dependent_addr_0_31_0_mask) << SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_0_LO_dependent_addr_0_31_0_shift) };
}

/*define for DEPENDENT_ADDR_0_HI word*/
/*define for dependent_addr_0_63_32 field*/
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_0_HI_dependent_addr_0_63_32_offset: u32 = 3;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_0_HI_dependent_addr_0_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_0_HI_dependent_addr_0_63_32_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_0_HI_DEPENDENT_ADDR_0_63_32 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_0_HI_dependent_addr_0_63_32_mask) << SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_0_HI_dependent_addr_0_63_32_shift) };
}

/*define for DEPENDENT_ADDR_1_LO word*/
/*define for dependent_addr_1_31_0 field*/
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_1_LO_dependent_addr_1_31_0_offset: u32 = 4;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_1_LO_dependent_addr_1_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_1_LO_dependent_addr_1_31_0_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_1_LO_DEPENDENT_ADDR_1_31_0 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_1_LO_dependent_addr_1_31_0_mask) << SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_1_LO_dependent_addr_1_31_0_shift) };
}

/*define for DEPENDENT_ADDR_1_HI word*/
/*define for dependent_addr_1_63_32 field*/
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_1_HI_dependent_addr_1_63_32_offset: u32 = 5;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_1_HI_dependent_addr_1_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_1_HI_dependent_addr_1_63_32_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_1_HI_DEPENDENT_ADDR_1_63_32 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_1_HI_dependent_addr_1_63_32_mask) << SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_1_HI_dependent_addr_1_63_32_shift) };
}

/*define for DEPENDENT_ADDR_2_LO word*/
/*define for dependent_addr_2_31_0 field*/
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_2_LO_dependent_addr_2_31_0_offset: u32 = 6;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_2_LO_dependent_addr_2_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_2_LO_dependent_addr_2_31_0_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_2_LO_DEPENDENT_ADDR_2_31_0 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_2_LO_dependent_addr_2_31_0_mask) << SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_2_LO_dependent_addr_2_31_0_shift) };
}

/*define for DEPENDENT_ADDR_2_HI word*/
/*define for dependent_addr_2_63_32 field*/
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_2_HI_dependent_addr_2_63_32_offset: u32 = 7;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_2_HI_dependent_addr_2_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_2_HI_dependent_addr_2_63_32_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_2_HI_DEPENDENT_ADDR_2_63_32 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_2_HI_dependent_addr_2_63_32_mask) << SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_2_HI_dependent_addr_2_63_32_shift) };
}

/*define for DEPENDENT_ADDR_3_LO word*/
/*define for dependent_addr_3_31_0 field*/
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_3_LO_dependent_addr_3_31_0_offset: u32 = 8;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_3_LO_dependent_addr_3_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_3_LO_dependent_addr_3_31_0_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_3_LO_DEPENDENT_ADDR_3_31_0 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_3_LO_dependent_addr_3_31_0_mask) << SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_3_LO_dependent_addr_3_31_0_shift) };
}

/*define for DEPENDENT_ADDR_3_HI word*/
/*define for dependent_addr_3_63_32 field*/
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_3_HI_dependent_addr_3_63_32_offset: u32 = 9;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_3_HI_dependent_addr_3_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_3_HI_dependent_addr_3_63_32_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_3_HI_DEPENDENT_ADDR_3_63_32 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_3_HI_dependent_addr_3_63_32_mask) << SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_3_HI_dependent_addr_3_63_32_shift) };
}

/*define for DEPENDENT_ADDR_4_LO word*/
/*define for dependent_addr_4_31_0 field*/
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_4_LO_dependent_addr_4_31_0_offset: u32 = 10;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_4_LO_dependent_addr_4_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_4_LO_dependent_addr_4_31_0_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_4_LO_DEPENDENT_ADDR_4_31_0 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_4_LO_dependent_addr_4_31_0_mask) << SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_4_LO_dependent_addr_4_31_0_shift) };
}

/*define for DEPENDENT_ADDR_4_HI word*/
/*define for dependent_addr_4_63_32 field*/
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_4_HI_dependent_addr_4_63_32_offset: u32 = 11;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_4_HI_dependent_addr_4_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_4_HI_dependent_addr_4_63_32_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_4_HI_DEPENDENT_ADDR_4_63_32 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_4_HI_dependent_addr_4_63_32_mask) << SDMA_AQL_PKT_BARRIER_OR_DEPENDENT_ADDR_4_HI_dependent_addr_4_63_32_shift) };
}

/*define for CACHE_POLICY word*/
/*define for cache_policy0 field*/
pub const SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy0_offset: u32 = 12;
pub const SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy0_mask: u32 = 0x00000007;
pub const SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy0_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_CACHE_POLICY0 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy0_mask) << SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy0_shift) };
}

/*define for cache_policy1 field*/
pub const SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy1_offset: u32 = 12;
pub const SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy1_mask: u32 = 0x00000007;
pub const SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy1_shift: u32 = 5;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_CACHE_POLICY1 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy1_mask) << SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy1_shift) };
}

/*define for cache_policy2 field*/
pub const SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy2_offset: u32 = 12;
pub const SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy2_mask: u32 = 0x00000007;
pub const SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy2_shift: u32 = 10;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_CACHE_POLICY2 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy2_mask) << SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy2_shift) };
}

/*define for cache_policy3 field*/
pub const SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy3_offset: u32 = 12;
pub const SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy3_mask: u32 = 0x00000007;
pub const SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy3_shift: u32 = 15;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_CACHE_POLICY3 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy3_mask) << SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy3_shift) };
}

/*define for cache_policy4 field*/
pub const SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy4_offset: u32 = 12;
pub const SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy4_mask: u32 = 0x00000007;
pub const SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy4_shift: u32 = 20;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_CACHE_POLICY4 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy4_mask) << SDMA_AQL_PKT_BARRIER_OR_CACHE_POLICY_cache_policy4_shift) };
}

/*define for RESERVED_DW13 word*/
/*define for reserved_dw13 field*/
pub const SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW13_reserved_dw13_offset: u32 = 13;
pub const SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW13_reserved_dw13_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW13_reserved_dw13_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW13_RESERVED_DW13 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW13_reserved_dw13_mask) << SDMA_AQL_PKT_BARRIER_OR_RESERVED_DW13_reserved_dw13_shift) };
}

/*define for COMPLETION_SIGNAL_LO word*/
/*define for completion_signal_31_0 field*/
pub const SDMA_AQL_PKT_BARRIER_OR_COMPLETION_SIGNAL_LO_completion_signal_31_0_offset: u32 = 14;
pub const SDMA_AQL_PKT_BARRIER_OR_COMPLETION_SIGNAL_LO_completion_signal_31_0_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_COMPLETION_SIGNAL_LO_completion_signal_31_0_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_COMPLETION_SIGNAL_LO_COMPLETION_SIGNAL_31_0 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_COMPLETION_SIGNAL_LO_completion_signal_31_0_mask) << SDMA_AQL_PKT_BARRIER_OR_COMPLETION_SIGNAL_LO_completion_signal_31_0_shift) };
}

/*define for COMPLETION_SIGNAL_HI word*/
/*define for completion_signal_63_32 field*/
pub const SDMA_AQL_PKT_BARRIER_OR_COMPLETION_SIGNAL_HI_completion_signal_63_32_offset: u32 = 15;
pub const SDMA_AQL_PKT_BARRIER_OR_COMPLETION_SIGNAL_HI_completion_signal_63_32_mask: u32 = 0xFFFFFFFF;
pub const SDMA_AQL_PKT_BARRIER_OR_COMPLETION_SIGNAL_HI_completion_signal_63_32_shift: u32 = 0;
macro_rules! SDMA_AQL_PKT_BARRIER_OR_COMPLETION_SIGNAL_HI_COMPLETION_SIGNAL_63_32 {
    ($x:expr) => { ((($x) & SDMA_AQL_PKT_BARRIER_OR_COMPLETION_SIGNAL_HI_completion_signal_63_32_mask) << SDMA_AQL_PKT_BARRIER_OR_COMPLETION_SIGNAL_HI_completion_signal_63_32_shift) };
}


#endif /* __SDMA_V7_1_0_PKT_OPEN_H_ */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
