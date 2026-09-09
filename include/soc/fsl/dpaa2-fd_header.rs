/* SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause) */
/*
 * Copyright 2014-2016 Freescale Semiconductor Inc.
 * Copyright 2016 NXP
 */

//! DPAA2 FD - Frame Descriptor APIs for DPAA2.
//! Frame descriptors describe frame data and may be single, scatter-gather,
//! or frame-list descriptors.

#[repr(C)]
pub union dpaa2_fd {
    pub words: [u32; 8],
    pub simple: dpaa2_fd_simple,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_fd_simple {
    pub addr: __le64,
    pub len: __le32,
    pub bpid: __le16,
    pub format_offset: __le16,
    pub frc: __le32,
    pub ctrl: __le32,
    pub flc: __le64,
}

pub const FD_SHORT_LEN_FLAG_MASK: u32 = 0x1;
pub const FD_SHORT_LEN_FLAG_SHIFT: u32 = 14;
pub const FD_SHORT_LEN_MASK: u32 = 0x3FFFF;
pub const FD_OFFSET_MASK: u16 = 0x0FFF;
pub const FD_FORMAT_MASK: u16 = 0x3;
pub const FD_FORMAT_SHIFT: u32 = 12;
pub const FD_BPID_MASK: u16 = 0x3FFF;
pub const SG_SHORT_LEN_FLAG_MASK: u32 = 0x1;
pub const SG_SHORT_LEN_FLAG_SHIFT: u32 = 14;
pub const SG_SHORT_LEN_MASK: u32 = 0x1FFFF;
pub const SG_OFFSET_MASK: u16 = 0x0FFF;
pub const SG_FORMAT_MASK: u16 = 0x3;
pub const SG_FORMAT_SHIFT: u32 = 12;
pub const SG_BPID_MASK: u16 = 0x3FFF;
pub const SG_FINAL_FLAG_MASK: u16 = 0x1;
pub const SG_FINAL_FLAG_SHIFT: u32 = 15;
pub const FL_SHORT_LEN_FLAG_MASK: u32 = 0x1;
pub const FL_SHORT_LEN_FLAG_SHIFT: u32 = 14;
pub const FL_SHORT_LEN_MASK: u32 = 0x3FFFF;
pub const FL_OFFSET_MASK: u16 = 0x0FFF;
pub const FL_FORMAT_MASK: u16 = 0x3;
pub const FL_FORMAT_SHIFT: u32 = 12;
pub const FL_BPID_MASK: u16 = 0x3FFF;
pub const FL_FINAL_FLAG_MASK: u16 = 0x1;
pub const FL_FINAL_FLAG_SHIFT: u32 = 15;

pub const FD_CTRL_ERR_MASK: u32 = 0x000000FF;
pub const FD_CTRL_UFD: u32 = 0x00000004;
pub const FD_CTRL_SBE: u32 = 0x00000008;
pub const FD_CTRL_FLC: u32 = 0x00000010;
pub const FD_CTRL_FSE: u32 = 0x00000020;
pub const FD_CTRL_FAERR: u32 = 0x00000040;
pub const FD_CTRL_PTA: u32 = 0x00800000;
pub const FD_CTRL_PTV1: u32 = 0x00400000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpaa2_fd_format { dpaa2_fd_single = 0, dpaa2_fd_list, dpaa2_fd_sg }

pub unsafe fn dpaa2_fd_get_addr(fd: *const dpaa2_fd) -> dma_addr_t { le64_to_cpu((*fd).simple.addr) as dma_addr_t }
pub unsafe fn dpaa2_fd_set_addr(fd: *mut dpaa2_fd, addr: dma_addr_t) { (*fd).simple.addr = cpu_to_le64(addr); }
pub unsafe fn dpaa2_fd_get_frc(fd: *const dpaa2_fd) -> u32 { le32_to_cpu((*fd).simple.frc) }
pub unsafe fn dpaa2_fd_set_frc(fd: *mut dpaa2_fd, frc: u32) { (*fd).simple.frc = cpu_to_le32(frc); }
pub unsafe fn dpaa2_fd_get_ctrl(fd: *const dpaa2_fd) -> u32 { le32_to_cpu((*fd).simple.ctrl) }
pub unsafe fn dpaa2_fd_set_ctrl(fd: *mut dpaa2_fd, ctrl: u32) { (*fd).simple.ctrl = cpu_to_le32(ctrl); }
pub unsafe fn dpaa2_fd_get_flc(fd: *const dpaa2_fd) -> dma_addr_t { le64_to_cpu((*fd).simple.flc) as dma_addr_t }
pub unsafe fn dpaa2_fd_set_flc(fd: *mut dpaa2_fd, flc_addr: dma_addr_t) { (*fd).simple.flc = cpu_to_le64(flc_addr); }
pub unsafe fn dpaa2_fd_short_len(fd: *const dpaa2_fd) -> bool { ((le16_to_cpu((*fd).simple.format_offset) as u32 >> FD_SHORT_LEN_FLAG_SHIFT) & FD_SHORT_LEN_FLAG_MASK) != 0 }
pub unsafe fn dpaa2_fd_get_len(fd: *const dpaa2_fd) -> u32 { let len = le32_to_cpu((*fd).simple.len); if dpaa2_fd_short_len(fd) { len & FD_SHORT_LEN_MASK } else { len } }
pub unsafe fn dpaa2_fd_set_len(fd: *mut dpaa2_fd, len: u32) { (*fd).simple.len = cpu_to_le32(len); }
pub unsafe fn dpaa2_fd_get_offset(fd: *const dpaa2_fd) -> u16 { le16_to_cpu((*fd).simple.format_offset) & FD_OFFSET_MASK }
pub unsafe fn dpaa2_fd_set_offset(fd: *mut dpaa2_fd, offset: u16) { (*fd).simple.format_offset &= cpu_to_le16(!FD_OFFSET_MASK); (*fd).simple.format_offset |= cpu_to_le16(offset); }
pub unsafe fn dpaa2_fd_get_format(fd: *const dpaa2_fd) -> dpaa2_fd_format { core::mem::transmute(((le16_to_cpu((*fd).simple.format_offset) as u32 >> FD_FORMAT_SHIFT) & FD_FORMAT_MASK as u32) as i32) }
pub unsafe fn dpaa2_fd_set_format(fd: *mut dpaa2_fd, format: dpaa2_fd_format) { (*fd).simple.format_offset &= cpu_to_le16(!(FD_FORMAT_MASK << FD_FORMAT_SHIFT)); (*fd).simple.format_offset |= cpu_to_le16((format as u16) << FD_FORMAT_SHIFT); }
pub unsafe fn dpaa2_fd_get_bpid(fd: *const dpaa2_fd) -> u16 { le16_to_cpu((*fd).simple.bpid) & FD_BPID_MASK }
pub unsafe fn dpaa2_fd_set_bpid(fd: *mut dpaa2_fd, bpid: u16) { (*fd).simple.bpid &= cpu_to_le16(!FD_BPID_MASK); (*fd).simple.bpid |= cpu_to_le16(bpid); }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_sg_entry { pub addr: __le64, pub len: __le32, pub bpid: __le16, pub format_offset: __le16 }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpaa2_sg_format { dpaa2_sg_single = 0, dpaa2_sg_frame_data, dpaa2_sg_sgt_ext }

pub unsafe fn dpaa2_sg_get_addr(sg: *const dpaa2_sg_entry) -> dma_addr_t { le64_to_cpu((*sg).addr) as dma_addr_t }
pub unsafe fn dpaa2_sg_set_addr(sg: *mut dpaa2_sg_entry, addr: dma_addr_t) { (*sg).addr = cpu_to_le64(addr); }
pub unsafe fn dpaa2_sg_short_len(sg: *const dpaa2_sg_entry) -> bool { ((le16_to_cpu((*sg).format_offset) as u32 >> SG_SHORT_LEN_FLAG_SHIFT) & SG_SHORT_LEN_FLAG_MASK) != 0 }
pub unsafe fn dpaa2_sg_get_len(sg: *const dpaa2_sg_entry) -> u32 { let len = le32_to_cpu((*sg).len); if dpaa2_sg_short_len(sg) { len & SG_SHORT_LEN_MASK } else { len } }
pub unsafe fn dpaa2_sg_set_len(sg: *mut dpaa2_sg_entry, len: u32) { (*sg).len = cpu_to_le32(len); }
pub unsafe fn dpaa2_sg_get_offset(sg: *const dpaa2_sg_entry) -> u16 { le16_to_cpu((*sg).format_offset) & SG_OFFSET_MASK }
pub unsafe fn dpaa2_sg_set_offset(sg: *mut dpaa2_sg_entry, offset: u16) { (*sg).format_offset &= cpu_to_le16(!SG_OFFSET_MASK); (*sg).format_offset |= cpu_to_le16(offset); }
pub unsafe fn dpaa2_sg_get_format(sg: *const dpaa2_sg_entry) -> dpaa2_sg_format { core::mem::transmute(((le16_to_cpu((*sg).format_offset) as u32 >> SG_FORMAT_SHIFT) & SG_FORMAT_MASK as u32) as i32) }
pub unsafe fn dpaa2_sg_set_format(sg: *mut dpaa2_sg_entry, format: dpaa2_sg_format) { (*sg).format_offset &= cpu_to_le16(!(SG_FORMAT_MASK << SG_FORMAT_SHIFT)); (*sg).format_offset |= cpu_to_le16((format as u16) << SG_FORMAT_SHIFT); }
pub unsafe fn dpaa2_sg_get_bpid(sg: *const dpaa2_sg_entry) -> u16 { le16_to_cpu((*sg).bpid) & SG_BPID_MASK }
pub unsafe fn dpaa2_sg_set_bpid(sg: *mut dpaa2_sg_entry, bpid: u16) { (*sg).bpid &= cpu_to_le16(!SG_BPID_MASK); (*sg).bpid |= cpu_to_le16(bpid); }
pub unsafe fn dpaa2_sg_is_final(sg: *const dpaa2_sg_entry) -> bool { (le16_to_cpu((*sg).format_offset) as u32 >> SG_FINAL_FLAG_SHIFT) != 0 }
pub unsafe fn dpaa2_sg_set_final(sg: *mut dpaa2_sg_entry, final_: bool) { (*sg).format_offset &= cpu_to_le16(((!(SG_FINAL_FLAG_MASK << SG_FINAL_FLAG_SHIFT)) & 0xFFFF) as u16); (*sg).format_offset |= cpu_to_le16((final_ as u16) << SG_FINAL_FLAG_SHIFT); }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_fl_entry { pub addr: __le64, pub len: __le32, pub bpid: __le16, pub format_offset: __le16, pub frc: __le32, pub ctrl: __le32, pub flc: __le64 }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpaa2_fl_format { dpaa2_fl_single = 0, dpaa2_fl_res, dpaa2_fl_sg }

pub unsafe fn dpaa2_fl_get_addr(fle: *const dpaa2_fl_entry) -> dma_addr_t { le64_to_cpu((*fle).addr) as dma_addr_t }
pub unsafe fn dpaa2_fl_set_addr(fle: *mut dpaa2_fl_entry, addr: dma_addr_t) { (*fle).addr = cpu_to_le64(addr); }
pub unsafe fn dpaa2_fl_get_frc(fle: *const dpaa2_fl_entry) -> u32 { le32_to_cpu((*fle).frc) }
pub unsafe fn dpaa2_fl_set_frc(fle: *mut dpaa2_fl_entry, frc: u32) { (*fle).frc = cpu_to_le32(frc); }
pub unsafe fn dpaa2_fl_get_ctrl(fle: *const dpaa2_fl_entry) -> u32 { le32_to_cpu((*fle).ctrl) }
pub unsafe fn dpaa2_fl_set_ctrl(fle: *mut dpaa2_fl_entry, ctrl: u32) { (*fle).ctrl = cpu_to_le32(ctrl); }
pub unsafe fn dpaa2_fl_get_flc(fle: *const dpaa2_fl_entry) -> dma_addr_t { le64_to_cpu((*fle).flc) as dma_addr_t }
pub unsafe fn dpaa2_fl_set_flc(fle: *mut dpaa2_fl_entry, flc_addr: dma_addr_t) { (*fle).flc = cpu_to_le64(flc_addr); }
pub unsafe fn dpaa2_fl_short_len(fle: *const dpaa2_fl_entry) -> bool { ((le16_to_cpu((*fle).format_offset) as u32 >> FL_SHORT_LEN_FLAG_SHIFT) & FL_SHORT_LEN_FLAG_MASK) != 0 }
pub unsafe fn dpaa2_fl_get_len(fle: *const dpaa2_fl_entry) -> u32 { let len = le32_to_cpu((*fle).len); if dpaa2_fl_short_len(fle) { len & FL_SHORT_LEN_MASK } else { len } }
pub unsafe fn dpaa2_fl_set_len(fle: *mut dpaa2_fl_entry, len: u32) { (*fle).len = cpu_to_le32(len); }
pub unsafe fn dpaa2_fl_get_offset(fle: *const dpaa2_fl_entry) -> u16 { le16_to_cpu((*fle).format_offset) & FL_OFFSET_MASK }
pub unsafe fn dpaa2_fl_set_offset(fle: *mut dpaa2_fl_entry, offset: u16) { (*fle).format_offset &= cpu_to_le16(!FL_OFFSET_MASK); (*fle).format_offset |= cpu_to_le16(offset); }
pub unsafe fn dpaa2_fl_get_format(fle: *const dpaa2_fl_entry) -> dpaa2_fl_format { core::mem::transmute(((le16_to_cpu((*fle).format_offset) as u32 >> FL_FORMAT_SHIFT) & FL_FORMAT_MASK as u32) as i32) }
pub unsafe fn dpaa2_fl_set_format(fle: *mut dpaa2_fl_entry, format: dpaa2_fl_format) { (*fle).format_offset &= cpu_to_le16(!(FL_FORMAT_MASK << FL_FORMAT_SHIFT)); (*fle).format_offset |= cpu_to_le16((format as u16) << FL_FORMAT_SHIFT); }
pub unsafe fn dpaa2_fl_get_bpid(fle: *const dpaa2_fl_entry) -> u16 { le16_to_cpu((*fle).bpid) & FL_BPID_MASK }
pub unsafe fn dpaa2_fl_set_bpid(fle: *mut dpaa2_fl_entry, bpid: u16) { (*fle).bpid &= cpu_to_le16(!FL_BPID_MASK); (*fle).bpid |= cpu_to_le16(bpid); }
pub unsafe fn dpaa2_fl_is_final(fle: *const dpaa2_fl_entry) -> bool { (le16_to_cpu((*fle).format_offset) as u32 >> FL_FINAL_FLAG_SHIFT) != 0 }
pub unsafe fn dpaa2_fl_set_final(fle: *mut dpaa2_fl_entry, final_: bool) { (*fle).format_offset &= cpu_to_le16(((!(FL_FINAL_FLAG_MASK << FL_FINAL_FLAG_SHIFT)) & 0xFFFF) as u16); (*fle).format_offset |= cpu_to_le16((final_ as u16) << FL_FINAL_FLAG_SHIFT); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
