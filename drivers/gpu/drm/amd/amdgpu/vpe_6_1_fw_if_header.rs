/* Copyright 2023 Advanced Micro Devices, Inc.
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
 * Authors: AMD
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VPE_CMD_OPCODE {
    VPE_CMD_OPCODE_NOP = 0x0,
    VPE_CMD_OPCODE_VPE_DESC = 0x1,
    VPE_CMD_OPCODE_PLANE_CFG = 0x2,
    VPE_CMD_OPCODE_VPEP_CFG = 0x3,
    VPE_CMD_OPCODE_INDIRECT = 0x4,
    VPE_CMD_OPCODE_FENCE = 0x5,
    VPE_CMD_OPCODE_TRAP = 0x6,
    VPE_CMD_OPCODE_REG_WRITE = 0x7,
    VPE_CMD_OPCODE_POLL_REGMEM = 0x8,
    VPE_CMD_OPCODE_COND_EXE = 0x9,
    VPE_CMD_OPCODE_ATOMIC = 0xA,
    VPE_CMD_OPCODE_PRED_EXE = 0xB,
    VPE_CMD_OPCODE_COLLAB_SYNC = 0xC,
    VPE_CMD_OPCODE_TIMESTAMP = 0xD,
}

pub const VPE_HEADER_SUB_OPCODE__SHIFT: u32 = 8;
pub const VPE_HEADER_SUB_OPCODE_MASK: u32 = 0x0000FF00;
pub const VPE_HEADER_OPCODE__SHIFT: u32 = 0;
pub const VPE_HEADER_OPCODE_MASK: u32 = 0x000000FF;

#[inline]
pub const fn VPE_CMD_HEADER(op: u32, subop: u32) -> u32 {
    ((subop << VPE_HEADER_SUB_OPCODE__SHIFT) & VPE_HEADER_SUB_OPCODE_MASK)
        | ((op << VPE_HEADER_OPCODE__SHIFT) & VPE_HEADER_OPCODE_MASK)
}

pub const VPE_CMD_NOP_HEADER_COUNT__SHIFT: u32 = 16;
pub const VPE_CMD_NOP_HEADER_COUNT_MASK: u32 = 0x00003FFF;
#[inline]
pub const fn VPE_CMD_NOP_HEADER_COUNT(count: u32) -> u32 {
    (count & VPE_CMD_NOP_HEADER_COUNT_MASK) << VPE_CMD_NOP_HEADER_COUNT__SHIFT
}

pub const VPE_DESC_CD__SHIFT: u32 = 16;
pub const VPE_DESC_CD_MASK: u32 = 0x000F0000;
#[inline]
pub const fn VPE_DESC_CMD_HEADER(cd: u32) -> u32 {
    VPE_CMD_HEADER(VPE_CMD_OPCODE::VPE_CMD_OPCODE_VPE_DESC as u32, 0)
        | ((cd << VPE_DESC_CD__SHIFT) & VPE_DESC_CD_MASK)
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VPE_PLANE_CFG_SUBOP {
    VPE_PLANE_CFG_SUBOP_1_TO_1 = 0x0,
    VPE_PLANE_CFG_SUBOP_2_TO_1 = 0x1,
    VPE_PLANE_CFG_SUBOP_2_TO_2 = 0x2,
}

pub const VPE_PLANE_CFG_ONE_PLANE: u32 = 0;
pub const VPE_PLANE_CFG_TWO_PLANES: u32 = 1;
pub const VPE_PLANE_CFG_NPS0__SHIFT: u32 = 16;
pub const VPE_PLANE_CFG_NPS0_MASK: u32 = 0x00030000;
pub const VPE_PLANE_CFG_NPD0__SHIFT: u32 = 18;
pub const VPE_PLANE_CFG_NPD0_MASK: u32 = 0x000C0000;
pub const VPE_PLANE_CFG_NPS1__SHIFT: u32 = 20;
pub const VPE_PLANE_CFG_NPS1_MASK: u32 = 0x00300000;
pub const VPE_PLANE_CFG_NPD1__SHIFT: u32 = 22;
pub const VPE_PLANE_CFG_NPD1_MASK: u32 = 0x00C00000;
pub const VPE_PLANE_CFG_TMZ__SHIFT: u32 = 16;
pub const VPE_PLANE_CFG_TMZ_MASK: u32 = 0x00010000;
pub const VPE_PLANE_CFG_SWIZZLE_MODE__SHIFT: u32 = 3;
pub const VPE_PLANE_CFG_SWIZZLE_MODE_MASK: u32 = 0x000000F8;
pub const VPE_PLANE_CFG_ROTATION__SHIFT: u32 = 0;
pub const VPE_PLANE_CFG_ROTATION_MASK: u32 = 0x00000003;
pub const VPE_PLANE_ADDR_LO__SHIFT: u32 = 0;
pub const VPE_PLANE_ADDR_LO_MASK: u32 = 0xFFFFFF00;
pub const VPE_PLANE_CFG_PITCH__SHIFT: u32 = 0;
pub const VPE_PLANE_CFG_PITCH_MASK: u32 = 0x00003FFF;
pub const VPE_PLANE_CFG_VIEWPORT_Y__SHIFT: u32 = 16;
pub const VPE_PLANE_CFG_VIEWPORT_Y_MASK: u32 = 0x3FFF0000;
pub const VPE_PLANE_CFG_VIEWPORT_X__SHIFT: u32 = 0;
pub const VPE_PLANE_CFG_VIEWPORT_X_MASK: u32 = 0x00003FFF;
pub const VPE_PLANE_CFG_VIEWPORT_HEIGHT__SHIFT: u32 = 16;
pub const VPE_PLANE_CFG_VIEWPORT_HEIGHT_MASK: u32 = 0x1FFF0000;
pub const VPE_PLANE_CFG_VIEWPORT_ELEMENT_SIZE__SHIFT: u32 = 13;
pub const VPE_PLANE_CFG_VIEWPORT_ELEMENT_SIZE_MASK: u32 = 0x0000E000;
pub const VPE_PLANE_CFG_VIEWPORT_WIDTH__SHIFT: u32 = 0;
pub const VPE_PLANE_CFG_VIEWPORT_WIDTH_MASK: u32 = 0x00001FFF;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VPE_PLANE_CFG_ELEMENT_SIZE {
    VPE_PLANE_CFG_ELEMENT_SIZE_8BPE = 0,
    VPE_PLANE_CFG_ELEMENT_SIZE_16BPE = 1,
    VPE_PLANE_CFG_ELEMENT_SIZE_32BPE = 2,
    VPE_PLANE_CFG_ELEMENT_SIZE_64BPE = 3,
}

#[inline]
pub const fn VPE_PLANE_CFG_CMD_HEADER(subop: u32, nps0: u32, npd0: u32, nps1: u32, npd1: u32) -> u32 {
    VPE_CMD_HEADER(VPE_CMD_OPCODE::VPE_CMD_OPCODE_PLANE_CFG as u32, subop)
        | ((nps0 << 16) & 0x00030000)
        | ((npd0 << 18) & 0x000C0000)
        | ((nps1 << 20) & 0x00300000)
        | ((npd0 << 22) & 0x00C00000)
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VPE_VPEP_CFG_SUBOP {
    VPE_VPEP_CFG_SUBOP_DIR_CFG = 0x0,
    VPE_VPEP_CFG_SUBOP_IND_CFG = 0x1,
}

pub const VPE_DIR_CFG_HEADER_ARRAY_SIZE__SHIFT: u32 = 16;
pub const VPE_DIR_CFG_HEADER_ARRAY_SIZE_MASK: u32 = 0xFFFF0000;
#[inline]
pub const fn VPE_DIR_CFG_CMD_HEADER(subop: u32, arr_sz: u32) -> u32 {
    VPE_CMD_HEADER(VPE_CMD_OPCODE::VPE_CMD_OPCODE_VPEP_CFG as u32, subop)
        | ((arr_sz << 16) & 0xFFFF0000)
}
pub const VPE_DIR_CFG_PKT_REGISTER_OFFSET__SHIFT: u32 = 2;
pub const VPE_DIR_CFG_PKT_REGISTER_OFFSET_MASK: u32 = 0x000FFFFC;
pub const VPE_DIR_CFG_PKT_DATA_SIZE__SHIFT: u32 = 20;
pub const VPE_DIR_CFG_PKT_DATA_SIZE_MASK: u32 = 0xFFF00000;
pub const VPE_IND_CFG_HEADER_NUM_DST__SHIFT: u32 = 28;
pub const VPE_IND_CFG_HEADER_NUM_DST_MASK: u32 = 0xF0000000;
#[inline]
pub const fn VPE_IND_CFG_CMD_HEADER(subop: u32, num_dst: u32) -> u32 {
    VPE_CMD_HEADER(VPE_CMD_OPCODE::VPE_CMD_OPCODE_VPEP_CFG as u32, subop)
        | ((num_dst << 28) & 0xF0000000)
}
pub const VPE_CMD_INDIRECT_HEADER_VMID__SHIFT: u32 = 16;
pub const VPE_CMD_INDIRECT_HEADER_VMID_MASK: u32 = 0x0000000F;
#[inline]
pub const fn VPE_CMD_INDIRECT_HEADER_VMID(vmid: u32) -> u32 {
    (vmid & VPE_CMD_INDIRECT_HEADER_VMID_MASK) << VPE_CMD_INDIRECT_HEADER_VMID__SHIFT
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VPE_POLL_REGMEM_SUBOP {
    VPE_POLL_REGMEM_SUBOP_REGMEM = 0x0,
    VPE_POLL_REGMEM_SUBOP_REGMEM_WRITE = 0x1,
}

pub const VPE_CMD_POLL_REGMEM_HEADER_FUNC__SHIFT: u32 = 28;
pub const VPE_CMD_POLL_REGMEM_HEADER_FUNC_MASK: u32 = 0x00000007;
#[inline]
pub const fn VPE_CMD_POLL_REGMEM_HEADER_FUNC(func: u32) -> u32 {
    (func & VPE_CMD_POLL_REGMEM_HEADER_FUNC_MASK) << VPE_CMD_POLL_REGMEM_HEADER_FUNC__SHIFT
}
pub const VPE_CMD_POLL_REGMEM_HEADER_MEM__SHIFT: u32 = 31;
pub const VPE_CMD_POLL_REGMEM_HEADER_MEM_MASK: u32 = 0x00000001;
#[inline]
pub const fn VPE_CMD_POLL_REGMEM_HEADER_MEM(mem: u32) -> u32 {
    (mem & VPE_CMD_POLL_REGMEM_HEADER_MEM_MASK) << VPE_CMD_POLL_REGMEM_HEADER_MEM__SHIFT
}
pub const VPE_CMD_POLL_REGMEM_DW5_INTERVAL__SHIFT: u32 = 0;
pub const VPE_CMD_POLL_REGMEM_DW5_INTERVAL_MASK: u32 = 0x0000FFFF;
#[inline]
pub const fn VPE_CMD_POLL_REGMEM_DW5_INTERVAL(interval: u32) -> u32 {
    (interval & VPE_CMD_POLL_REGMEM_DW5_INTERVAL_MASK) << VPE_CMD_POLL_REGMEM_DW5_INTERVAL__SHIFT
}
pub const VPE_CMD_POLL_REGMEM_DW5_RETRY_COUNT__SHIFT: u32 = 16;
pub const VPE_CMD_POLL_REGMEM_DW5_RETRY_COUNT_MASK: u32 = 0x00000FFF;
#[inline]
pub const fn VPE_CMD_POLL_REGMEM_DW5_RETRY_COUNT(count: u32) -> u32 {
    (count & VPE_CMD_POLL_REGMEM_DW5_RETRY_COUNT_MASK) << VPE_CMD_POLL_REGMEM_DW5_RETRY_COUNT__SHIFT
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
