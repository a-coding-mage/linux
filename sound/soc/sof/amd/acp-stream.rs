// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021 Advanced Micro Devices, Inc.
//
// Authors: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>

/*
 * Hardware interface for generic AMD audio DSP ACP IP
 */

// C dependencies removed from executable Rust:
// #include "../ops.h"
// #include "acp-dsp-offset.h"
// #include "acp.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::offset_of;
use core::ptr;

pub type u32 = u32;
pub type dma_addr_t = u64;

pub const PTE_GRP1_OFFSET: c_uint = 0x00000000;
pub const PTE_GRP2_OFFSET: c_uint = 0x00800000;
pub const PTE_GRP3_OFFSET: c_uint = 0x01000000;
pub const PTE_GRP4_OFFSET: c_uint = 0x01800000;
pub const PTE_GRP5_OFFSET: c_uint = 0x02000000;
pub const PTE_GRP6_OFFSET: c_uint = 0x02800000;
pub const PTE_GRP7_OFFSET: c_uint = 0x03000000;
pub const PTE_GRP8_OFFSET: c_uint = 0x03800000;

unsafe extern "C" {
    static ACPAXI2AXI_ATU_BASE_ADDR_GRP_1: c_uint;
    static ACPAXI2AXI_ATU_BASE_ADDR_GRP_2: c_uint;
    static ACPAXI2AXI_ATU_BASE_ADDR_GRP_3: c_uint;
    static ACPAXI2AXI_ATU_BASE_ADDR_GRP_4: c_uint;
    static ACPAXI2AXI_ATU_BASE_ADDR_GRP_5: c_uint;
    static ACPAXI2AXI_ATU_BASE_ADDR_GRP_6: c_uint;
    static ACPAXI2AXI_ATU_BASE_ADDR_GRP_7: c_uint;
    static ACPAXI2AXI_ATU_BASE_ADDR_GRP_8: c_uint;
    static ACPAXI2AXI_ATU_PAGE_SIZE_GRP_1: c_uint;
    static ACPAXI2AXI_ATU_PAGE_SIZE_GRP_2: c_uint;
    static ACPAXI2AXI_ATU_PAGE_SIZE_GRP_3: c_uint;
    static ACPAXI2AXI_ATU_PAGE_SIZE_GRP_4: c_uint;
    static ACPAXI2AXI_ATU_PAGE_SIZE_GRP_5: c_uint;
    static ACPAXI2AXI_ATU_PAGE_SIZE_GRP_6: c_uint;
    static ACPAXI2AXI_ATU_PAGE_SIZE_GRP_7: c_uint;
    static ACPAXI2AXI_ATU_PAGE_SIZE_GRP_8: c_uint;
    static ACP_DSP_BAR: c_uint;
    static ACP_SCRATCH_REG_0: c_uint;
    static ACPAXI2AXI_ATU_CTRL: c_uint;
    static ACP_ATU_CACHE_INVALID: u32;
    static PAGE_SIZE_4K_ENABLE: u32;
    static PAGE_SIZE: c_uint;
    static EINVAL: c_int;
    static ACP_MAX_STREAM: c_int;

    fn get_chip_info(pdata: *mut snd_sof_pdata) -> *const sof_amd_acp_desc;
    fn snd_sof_dsp_write(sdev: *mut snd_sof_dev, bar: c_uint, offset: c_uint, value: u32);
    fn snd_sgbuf_get_addr(dmab: *mut snd_dma_buffer, offset: c_uint) -> dma_addr_t;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dma_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub pdata: *mut snd_sof_pdata,
    pub debug_box: debug_box,
}

#[repr(C)]
pub struct debug_box {
    pub offset: c_uint,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub hw_pdata: *mut acp_dev_data,
}

#[repr(C)]
pub struct sof_amd_acp_desc {
    pub sram_pte_offset: u32,
}

#[repr(C)]
pub struct acp_dev_data {
    pub stream_buf: *mut acp_dsp_stream,
}

#[repr(C)]
pub struct acp_dsp_stream {
    pub sdev: *mut snd_sof_dev,
    pub active: c_int,
    pub stream_tag: c_int,
    pub reg_offset: c_uint,
    pub num_pages: c_int,
    pub dmab: *mut snd_dma_buffer,
}

#[repr(C)]
pub struct scratch_reg_conf {
    pub reg_offset: [u32; 8],
    pub grp1_pte: u32,
    pub grp2_pte: u32,
    pub grp3_pte: u32,
    pub grp4_pte: u32,
    pub grp5_pte: u32,
    pub grp6_pte: u32,
    pub grp7_pte: u32,
    pub grp8_pte: u32,
}

#[inline]
fn BIT(nr: c_uint) -> u32 {
    1u32.wrapping_shl(nr)
}

#[inline]
fn lower_32_bits(addr: dma_addr_t) -> u32 {
    addr as u32
}

#[inline]
fn upper_32_bits(addr: dma_addr_t) -> u32 {
    (addr >> 32) as u32
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acp_dsp_stream_config(
    sdev: *mut snd_sof_dev,
    stream: *mut acp_dsp_stream,
) -> c_int {
    let desc: *const sof_amd_acp_desc = unsafe { get_chip_info((*sdev).pdata) };
    let mut pte_reg: c_uint;
    let mut pte_size: c_uint;
    let mut phy_addr_offset: c_uint;
    let index: c_uint;
    let mut low: u32;
    let mut high: u32;
    let mut offset: u32;
    let reg_val: u32;
    let mut addr: dma_addr_t;
    let mut page_idx: c_int;
    let stream_tag: c_int = unsafe { (*stream).stream_tag };

    match stream_tag {
        1 => {
            pte_reg = unsafe { ACPAXI2AXI_ATU_BASE_ADDR_GRP_1 };
            pte_size = unsafe { ACPAXI2AXI_ATU_PAGE_SIZE_GRP_1 };
            offset = offset_of!(scratch_reg_conf, grp1_pte) as u32;
            unsafe { (*stream).reg_offset = PTE_GRP1_OFFSET };
        }
        2 => {
            pte_reg = unsafe { ACPAXI2AXI_ATU_BASE_ADDR_GRP_2 };
            pte_size = unsafe { ACPAXI2AXI_ATU_PAGE_SIZE_GRP_2 };
            offset = offset_of!(scratch_reg_conf, grp2_pte) as u32;
            unsafe { (*stream).reg_offset = PTE_GRP2_OFFSET };
        }
        3 => {
            pte_reg = unsafe { ACPAXI2AXI_ATU_BASE_ADDR_GRP_3 };
            pte_size = unsafe { ACPAXI2AXI_ATU_PAGE_SIZE_GRP_3 };
            offset = offset_of!(scratch_reg_conf, grp3_pte) as u32;
            unsafe { (*stream).reg_offset = PTE_GRP3_OFFSET };
        }
        4 => {
            pte_reg = unsafe { ACPAXI2AXI_ATU_BASE_ADDR_GRP_4 };
            pte_size = unsafe { ACPAXI2AXI_ATU_PAGE_SIZE_GRP_4 };
            offset = offset_of!(scratch_reg_conf, grp4_pte) as u32;
            unsafe { (*stream).reg_offset = PTE_GRP4_OFFSET };
        }
        5 => {
            pte_reg = unsafe { ACPAXI2AXI_ATU_BASE_ADDR_GRP_5 };
            pte_size = unsafe { ACPAXI2AXI_ATU_PAGE_SIZE_GRP_5 };
            offset = offset_of!(scratch_reg_conf, grp5_pte) as u32;
            unsafe { (*stream).reg_offset = PTE_GRP5_OFFSET };
        }
        6 => {
            pte_reg = unsafe { ACPAXI2AXI_ATU_BASE_ADDR_GRP_6 };
            pte_size = unsafe { ACPAXI2AXI_ATU_PAGE_SIZE_GRP_6 };
            offset = offset_of!(scratch_reg_conf, grp6_pte) as u32;
            unsafe { (*stream).reg_offset = PTE_GRP6_OFFSET };
        }
        7 => {
            pte_reg = unsafe { ACPAXI2AXI_ATU_BASE_ADDR_GRP_7 };
            pte_size = unsafe { ACPAXI2AXI_ATU_PAGE_SIZE_GRP_7 };
            offset = offset_of!(scratch_reg_conf, grp7_pte) as u32;
            unsafe { (*stream).reg_offset = PTE_GRP7_OFFSET };
        }
        8 => {
            pte_reg = unsafe { ACPAXI2AXI_ATU_BASE_ADDR_GRP_8 };
            pte_size = unsafe { ACPAXI2AXI_ATU_PAGE_SIZE_GRP_8 };
            offset = offset_of!(scratch_reg_conf, grp8_pte) as u32;
            unsafe { (*stream).reg_offset = PTE_GRP8_OFFSET };
        }
        _ => {
            unsafe {
                dev_err(
                    (*sdev).dev,
                    c"Invalid stream tag %d\n".as_ptr(),
                    stream_tag,
                );
            }
            return unsafe { -EINVAL };
        }
    }

    /* write phy_addr in scratch memory */

    phy_addr_offset = unsafe { (*sdev).debug_box.offset }
        .wrapping_add(offset_of!(scratch_reg_conf, reg_offset) as c_uint);
    index = (stream_tag - 1) as c_uint;
    phy_addr_offset = phy_addr_offset.wrapping_add(index.wrapping_mul(4));

    unsafe {
        snd_sof_dsp_write(
            sdev,
            ACP_DSP_BAR,
            ACP_SCRATCH_REG_0.wrapping_add(phy_addr_offset),
            (*stream).reg_offset,
        );
    }

    /* Group Enable */
    offset = offset.wrapping_add(unsafe { (*sdev).debug_box.offset });
    reg_val = unsafe { (*desc).sram_pte_offset }.wrapping_add(offset);
    unsafe {
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, pte_reg, reg_val | BIT(31));
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, pte_size, PAGE_SIZE_4K_ENABLE);
    }

    page_idx = 0;
    while page_idx < unsafe { (*stream).num_pages } {
        addr = unsafe {
            snd_sgbuf_get_addr(
                (*stream).dmab,
                (page_idx as c_uint).wrapping_mul(PAGE_SIZE),
            )
        };

        /* Load the low address of page int ACP SRAM through SRBM */
        low = lower_32_bits(addr);
        high = upper_32_bits(addr);

        unsafe {
            snd_sof_dsp_write(
                sdev,
                ACP_DSP_BAR,
                ACP_SCRATCH_REG_0.wrapping_add(offset),
                low,
            );
        }

        high |= BIT(31);
        unsafe {
            snd_sof_dsp_write(
                sdev,
                ACP_DSP_BAR,
                ACP_SCRATCH_REG_0.wrapping_add(offset).wrapping_add(4),
                high,
            );
        }
        /* Move to next physically contiguous page */
        offset = offset.wrapping_add(8);
        page_idx += 1;
    }

    /* Flush ATU Cache after PTE Update */
    unsafe {
        snd_sof_dsp_write(
            sdev,
            ACP_DSP_BAR,
            ACPAXI2AXI_ATU_CTRL,
            ACP_ATU_CACHE_INVALID,
        );
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acp_dsp_stream_get(
    sdev: *mut snd_sof_dev,
    tag: c_int,
) -> *mut acp_dsp_stream {
    let adata: *mut acp_dev_data = unsafe { (*(*sdev).pdata).hw_pdata };
    let mut stream: *mut acp_dsp_stream = unsafe { (*adata).stream_buf };
    let mut i: c_int;

    i = 0;
    while i < unsafe { ACP_MAX_STREAM } {
        if unsafe { (*stream).active } != 0 {
            i += 1;
            stream = unsafe { stream.add(1) };
            continue;
        }

        /* return stream if tag not specified*/
        if tag == 0 {
            unsafe { (*stream).active = 1 };
            return stream;
        }

        /* check if this is the requested stream tag */
        if unsafe { (*stream).stream_tag } == tag {
            unsafe { (*stream).active = 1 };
            return stream;
        }

        i += 1;
        stream = unsafe { stream.add(1) };
    }

    unsafe {
        dev_err(
            (*sdev).dev,
            c"stream %d active or no inactive stream\n".as_ptr(),
            tag,
        );
    }
    ptr::null_mut()
}
// EXPORT_SYMBOL_NS(acp_dsp_stream_get, "SND_SOC_SOF_AMD_COMMON");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acp_dsp_stream_put(
    sdev: *mut snd_sof_dev,
    acp_stream: *mut acp_dsp_stream,
) -> c_int {
    let adata: *mut acp_dev_data = unsafe { (*(*sdev).pdata).hw_pdata };
    let mut stream: *mut acp_dsp_stream = unsafe { (*adata).stream_buf };
    let mut i: c_int;

    /* Free an active stream */
    i = 0;
    while i < unsafe { ACP_MAX_STREAM } {
        if stream == acp_stream {
            unsafe { (*stream).active = 0 };
            return 0;
        }

        i += 1;
        stream = unsafe { stream.add(1) };
    }

    unsafe {
        dev_err(
            (*sdev).dev,
            c"Cannot find active stream tag %d\n".as_ptr(),
            (*acp_stream).stream_tag,
        );
    }
    unsafe { -EINVAL }
}
// EXPORT_SYMBOL_NS(acp_dsp_stream_put, "SND_SOC_SOF_AMD_COMMON");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acp_dsp_stream_init(sdev: *mut snd_sof_dev) -> c_int {
    let adata: *mut acp_dev_data = unsafe { (*(*sdev).pdata).hw_pdata };
    let mut i: c_int;

    i = 0;
    while i < unsafe { ACP_MAX_STREAM } {
        unsafe {
            (*(*adata).stream_buf.add(i as usize)).sdev = sdev;
            (*(*adata).stream_buf.add(i as usize)).active = 0;
            (*(*adata).stream_buf.add(i as usize)).stream_tag = i + 1;
        }
        i += 1;
    }
    0
}
// EXPORT_SYMBOL_NS(acp_dsp_stream_init, "SND_SOC_SOF_AMD_COMMON");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
