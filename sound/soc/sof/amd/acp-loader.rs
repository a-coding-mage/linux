// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021, 2023 Advanced Micro Devices, Inc.
//
// Authors: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>

/*
 * Hardware interface for ACP DSP Firmware binaries loader
 */

// C dependencies:
// linux/firmware.h, linux/module.h, linux/pci.h, linux/unaligned.h
// ../ops.h, acp-dsp-offset.h, acp.h

use core::ffi::{c_char, c_int, c_uint, c_void};

type u16 = u16;
type u32 = u32;
type size_t = usize;
type dma_addr_t = u64;

const FW_BIN: c_int = 0;
const FW_DATA_BIN: c_int = 1;
const FW_SRAM_DATA_BIN: c_int = 2;

const FW_BIN_PTE_OFFSET: u32 = 0x00;
const FW_DATA_BIN_PTE_OFFSET: u32 = 0x08;

const ACP_DSP_RUN: u32 = 0x00;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

const GFP_KERNEL: c_uint = 0;
const PAGE_SHIFT: u32 = 12;
const PAGE_SIZE: dma_addr_t = 1 << PAGE_SHIFT;
const ACP_PAGE_SIZE: u32 = 4096;
const ACP_DEFAULT_DRAM_LENGTH: u32 = 0;
const ACP_DEFAULT_SRAM_LENGTH: u32 = 0;
const ACP_DRAM_PAGE_COUNT: c_int = 0;
const ACP_SRAM_PAGE_COUNT: c_int = 0;
const ACP_IMAGE_HEADER_SIZE: c_uint = 0;
const ACP_IMAGE_HDR_SIZE_FW_SIGNED_OFF: usize = 0;
const ACP_FIRMWARE_SIGNATURE: c_uint = 0;
const ACP_SYSTEM_MEMORY_WINDOW: c_uint = 0;
const ACP_IRAM_BASE_ADDRESS: c_uint = 0;
const ACP_DRAM_BASE_ADDRESS: c_uint = 0;
const ACP_SRAM_BASE_ADDRESS: c_uint = 0;
const ACP7X_SRAM_BASE_ADDRESS: c_uint = 0;
const ACP7B_PCI_ID: c_uint = 0;
const ACP63_PCI_ID: c_uint = 0;
const ACP_RN_PCI_ID: c_uint = 0;
const ACP_DSP_BAR: c_uint = 0;
const ACPAXI2AXI_ATU_BASE_ADDR_GRP_1: c_uint = 0;
const ACPAXI2AXI_ATU_PAGE_SIZE_GRP_1: c_uint = 0;
const PAGE_SIZE_4K_ENABLE: c_uint = 0;
const ACP_SCRATCH_REG_0: c_uint = 0;
const ACPAXI2AXI_ATU_CTRL: c_uint = 0;
const ACP_ATU_CACHE_INVALID: c_uint = 0;
const ACP_DSP0_CACHE_OFFSET0: c_uint = 0;
const ACP_DSP0_CACHE_SIZE0: c_uint = 0;
const SRAM1_SIZE: c_uint = 0;
const ACP_DSP0_RUNSTALL: c_uint = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
}

#[repr(C)]
pub struct firmware {
    pub size: size_t,
    pub data: *const u8,
}

#[repr(C)]
pub struct snd_sof_fw {
    pub fw: *mut firmware,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub hw_pdata: *mut acp_dev_data,
    pub fw_filename_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub pdata: *mut snd_sof_pdata,
    pub basefw: snd_sof_fw,
}

#[repr(C)]
pub struct sof_amd_acp_desc {
    pub sram_pte_offset: u32,
    pub fusion_dsp_offset: u32,
}

#[repr(C)]
pub struct acp_quirks {
    pub signed_fw_image: bool,
}

#[repr(C)]
pub struct acp_dev_data {
    pub dev: *mut snd_sof_dev,
    pub bin_buf: *mut c_void,
    pub sha_dma_addr: dma_addr_t,
    pub fw_bin_size: c_uint,
    pub data_buf: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub fw_data_bin_size: c_uint,
    pub is_dram_in_use: bool,
    pub sram_data_buf: *mut c_void,
    pub sram_dma_addr: dma_addr_t,
    pub fw_sram_data_bin_size: c_uint,
    pub is_sram_in_use: bool,
    pub fw_bin_page_count: u32,
    pub pci_rev: c_uint,
    pub acp_sof_signed_firmware_image: bool,
    pub quirks: *mut acp_quirks,
    pub enable_fw_debug: bool,
    pub fw_code_bin: *const c_char,
    pub fw_data_bin: *const c_char,
    pub fw_dbin: *mut firmware,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_sof_fw_blk_type {
    SOF_FW_BLK_TYPE_IRAM,
    SOF_FW_BLK_TYPE_DRAM,
    SOF_FW_BLK_TYPE_SRAM,
}

unsafe extern "C" {
    fn get_chip_info(pdata: *mut snd_sof_pdata) -> *const sof_amd_acp_desc;
    fn memcpy_from_scratch(sdev: *mut snd_sof_dev, offset: u32, dest: *mut c_void, size: size_t);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn dma_alloc_coherent(
        dev: *mut device,
        size: size_t,
        dma_handle: *mut dma_addr_t,
        flag: c_uint,
    ) -> *mut c_void;
    fn dma_free_coherent(dev: *mut device, size: size_t, cpu_addr: *mut c_void, dma_handle: dma_addr_t);
    fn memcpy(dest: *mut c_void, src: *const c_void, size: size_t) -> *mut c_void;
    fn snd_sof_dsp_write(sdev: *mut snd_sof_dev, bar: c_uint, offset: c_uint, value: c_uint);
    fn snd_sof_dsp_read(sdev: *mut snd_sof_dev, bar: c_uint, offset: c_uint) -> c_int;
    fn configure_and_run_sha_dma(
        adata: *mut acp_dev_data,
        src: *mut c_void,
        src_addr: c_uint,
        dest_addr: c_uint,
        size: c_uint,
    ) -> c_int;
    fn configure_and_run_dma(
        adata: *mut acp_dev_data,
        src_addr: c_uint,
        dest_addr: c_uint,
        size: c_uint,
    ) -> c_int;
    fn acp_dma_status(adata: *mut acp_dev_data, ch: c_int) -> c_int;
    fn kasprintf(gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn kfree(ptr: *const c_void);
    fn request_firmware(fw: *mut *mut firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn snd_sof_dsp_block_write(
        sdev: *mut snd_sof_dev,
        blk_type: snd_sof_fw_blk_type,
        offset: u32,
        src: *mut c_void,
        size: size_t,
    ) -> c_int;
}

const fn bit(n: u32) -> u32 {
    1u32 << n
}

fn page_align(size: c_uint) -> c_uint {
    (size.wrapping_add((PAGE_SIZE as c_uint).wrapping_sub(1))) & !((PAGE_SIZE as c_uint).wrapping_sub(1))
}

fn lower_32_bits(addr: dma_addr_t) -> c_uint {
    addr as c_uint
}

fn upper_32_bits(addr: dma_addr_t) -> c_uint {
    (addr >> 32) as c_uint
}

unsafe fn get_unaligned_le32(ptr: *const c_void) -> c_uint {
    u32::from_le(core::ptr::read_unaligned(ptr as *const u32))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acp_dsp_block_read(
    sdev: *mut snd_sof_dev,
    blk_type: snd_sof_fw_blk_type,
    mut offset: u32,
    dest: *mut c_void,
    size: size_t,
) -> c_int {
    let desc = get_chip_info((*sdev).pdata);

    match blk_type {
        snd_sof_fw_blk_type::SOF_FW_BLK_TYPE_SRAM => {
            offset = offset.wrapping_sub((*desc).sram_pte_offset);
            memcpy_from_scratch(sdev, offset, dest, size);
        }
        _ => {
            dev_err((*sdev).dev, c"bad blk type 0x%x\n".as_ptr(), blk_type as c_uint);
            return -EINVAL;
        }
    }

    0
}
// EXPORT_SYMBOL_NS(acp_dsp_block_read, "SND_SOC_SOF_AMD_COMMON");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acp_dsp_block_write(
    sdev: *mut snd_sof_dev,
    blk_type: snd_sof_fw_blk_type,
    offset: u32,
    src: *mut c_void,
    size: size_t,
) -> c_int {
    let pci = to_pci_dev((*sdev).dev);
    let adata: *mut acp_dev_data;
    let dest: *mut c_void;
    let dma_size: u32;
    let page_count: u32;
    let size_fw: c_uint;

    adata = (*(*sdev).pdata).hw_pdata;

    match blk_type {
        snd_sof_fw_blk_type::SOF_FW_BLK_TYPE_IRAM => {
            if (*adata).bin_buf.is_null() {
                size_fw = (*(*sdev).basefw.fw).size as c_uint;
                page_count = page_align(size_fw) >> PAGE_SHIFT;
                dma_size = page_count.wrapping_mul(ACP_PAGE_SIZE);
                (*adata).bin_buf = dma_alloc_coherent(
                    &mut (*pci).dev,
                    dma_size as size_t,
                    &mut (*adata).sha_dma_addr,
                    GFP_KERNEL,
                );
                if (*adata).bin_buf.is_null() {
                    return -ENOMEM;
                }
            }
            (*adata).fw_bin_size = (size as c_uint).wrapping_add(offset);
            dest = ((*adata).bin_buf as *mut u8).add(offset as usize) as *mut c_void;
        }
        snd_sof_fw_blk_type::SOF_FW_BLK_TYPE_DRAM => {
            if (*adata).data_buf.is_null() {
                (*adata).data_buf = dma_alloc_coherent(
                    &mut (*pci).dev,
                    ACP_DEFAULT_DRAM_LENGTH as size_t,
                    &mut (*adata).dma_addr,
                    GFP_KERNEL,
                );
                if (*adata).data_buf.is_null() {
                    return -ENOMEM;
                }
            }
            dest = ((*adata).data_buf as *mut u8).add(offset as usize) as *mut c_void;
            (*adata).fw_data_bin_size = (size as c_uint).wrapping_add(offset);
            (*adata).is_dram_in_use = true;
        }
        snd_sof_fw_blk_type::SOF_FW_BLK_TYPE_SRAM => {
            if (*adata).sram_data_buf.is_null() {
                (*adata).sram_data_buf = dma_alloc_coherent(
                    &mut (*pci).dev,
                    ACP_DEFAULT_SRAM_LENGTH as size_t,
                    &mut (*adata).sram_dma_addr,
                    GFP_KERNEL,
                );
                if (*adata).sram_data_buf.is_null() {
                    return -ENOMEM;
                }
            }
            (*adata).fw_sram_data_bin_size = (size as c_uint).wrapping_add(offset);
            dest = ((*adata).sram_data_buf as *mut u8).add(offset as usize) as *mut c_void;
            (*adata).is_sram_in_use = true;
        }
        _ => {
            dev_err((*sdev).dev, c"bad blk type 0x%x\n".as_ptr(), blk_type as c_uint);
            return -EINVAL;
        }
    }

    memcpy(dest, src, size);
    0
}
// EXPORT_SYMBOL_NS(acp_dsp_block_write, "SND_SOC_SOF_AMD_COMMON");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acp_get_bar_index(_sdev: *mut snd_sof_dev, type_: u32) -> c_int {
    type_ as c_int
}
// EXPORT_SYMBOL_NS(acp_get_bar_index, "SND_SOC_SOF_AMD_COMMON");

unsafe fn configure_pte_for_fw_loading(type_: c_int, num_pages: c_int, adata: *mut acp_dev_data) {
    let sdev = (*adata).dev;
    let desc = get_chip_info((*sdev).pdata);
    let mut low: c_uint;
    let mut high: c_uint;
    let mut addr: dma_addr_t;
    let mut page_idx: u16;
    let mut offset: u32;

    match type_ {
        FW_BIN => {
            offset = FW_BIN_PTE_OFFSET;
            addr = (*adata).sha_dma_addr;
        }
        FW_DATA_BIN => {
            offset = (*adata).fw_bin_page_count.wrapping_mul(8);
            addr = (*adata).dma_addr;
        }
        FW_SRAM_DATA_BIN => {
            offset = ((*adata).fw_bin_page_count.wrapping_add(ACP_DRAM_PAGE_COUNT as u32)).wrapping_mul(8);
            addr = (*adata).sram_dma_addr;
        }
        _ => {
            dev_err((*sdev).dev, c"Invalid data type %x\n".as_ptr(), type_);
            return;
        }
    }

    /* Group Enable */
    snd_sof_dsp_write(
        sdev,
        ACP_DSP_BAR,
        ACPAXI2AXI_ATU_BASE_ADDR_GRP_1,
        (*desc).sram_pte_offset | bit(31),
    );
    snd_sof_dsp_write(
        sdev,
        ACP_DSP_BAR,
        ACPAXI2AXI_ATU_PAGE_SIZE_GRP_1,
        PAGE_SIZE_4K_ENABLE,
    );

    page_idx = 0;
    while (page_idx as c_int) < num_pages {
        low = lower_32_bits(addr);
        high = upper_32_bits(addr);
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_SCRATCH_REG_0.wrapping_add(offset), low);
        high |= bit(31);
        snd_sof_dsp_write(
            sdev,
            ACP_DSP_BAR,
            ACP_SCRATCH_REG_0.wrapping_add(offset).wrapping_add(4),
            high,
        );
        offset = offset.wrapping_add(8);
        addr = addr.wrapping_add(PAGE_SIZE);
        page_idx = page_idx.wrapping_add(1);
    }

    /* Flush ATU Cache after PTE Update */
    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACPAXI2AXI_ATU_CTRL, ACP_ATU_CACHE_INVALID);
}

/* pre fw run operations */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acp_dsp_pre_fw_run(sdev: *mut snd_sof_dev) -> c_int {
    let pci = to_pci_dev((*sdev).dev);
    let desc = get_chip_info((*sdev).pdata);
    let adata: *mut acp_dev_data;
    let mut src_addr: c_uint;
    let mut size_fw: c_uint;
    let mut dest_addr: c_uint;
    let page_count: u32;
    let dma_size: u32;
    let mut ret: c_int;

    adata = (*(*sdev).pdata).hw_pdata;

    if (*adata).pci_rev >= ACP7B_PCI_ID {
        if (*adata).acp_sof_signed_firmware_image {
            if (*adata).fw_bin_size <= ACP_IMAGE_HEADER_SIZE {
                dev_err(
                    (*sdev).dev,
                    c"Invalid signed firmware size %u\n".as_ptr(),
                    (*adata).fw_bin_size,
                );
                return -EINVAL;
            }
            size_fw = get_unaligned_le32(
                ((*adata).bin_buf as *const u8).add(ACP_IMAGE_HDR_SIZE_FW_SIGNED_OFF) as *const c_void,
            );
            if size_fw == 0
                || size_fw > (*adata).fw_bin_size.wrapping_sub(ACP_IMAGE_HEADER_SIZE)
            {
                dev_err(
                    (*sdev).dev,
                    c"Invalid signed firmware payload size %u (max %u)\n".as_ptr(),
                    size_fw,
                    (*adata).fw_bin_size.wrapping_sub(ACP_IMAGE_HEADER_SIZE),
                );
                return -EINVAL;
            }
            size_fw = size_fw.wrapping_add(ACP_IMAGE_HEADER_SIZE);
        } else {
            size_fw = (*adata).fw_bin_size;
        }
    } else if !(*adata).quirks.is_null() && (*(*adata).quirks).signed_fw_image {
        size_fw = (*adata).fw_bin_size.wrapping_sub(ACP_FIRMWARE_SIGNATURE);
    } else {
        size_fw = (*adata).fw_bin_size;
    }

    page_count = page_align(size_fw) >> PAGE_SHIFT;
    (*adata).fw_bin_page_count = page_count;

    configure_pte_for_fw_loading(FW_BIN, page_count as c_int, adata);
    ret = configure_and_run_sha_dma(
        adata,
        (*adata).bin_buf,
        ACP_SYSTEM_MEMORY_WINDOW,
        ACP_IRAM_BASE_ADDRESS,
        size_fw,
    );
    if ret < 0 {
        dev_err((*sdev).dev, c"SHA DMA transfer failed status: %d\n".as_ptr(), ret);
        return ret;
    }
    if (*adata).is_dram_in_use {
        configure_pte_for_fw_loading(FW_DATA_BIN, ACP_DRAM_PAGE_COUNT, adata);
        src_addr = ACP_SYSTEM_MEMORY_WINDOW.wrapping_add(page_count.wrapping_mul(ACP_PAGE_SIZE));
        dest_addr = ACP_DRAM_BASE_ADDRESS;

        ret = configure_and_run_dma(adata, src_addr, dest_addr, (*adata).fw_data_bin_size);
        if ret < 0 {
            dev_err((*sdev).dev, c"acp dma configuration failed: %d\n".as_ptr(), ret);
            return ret;
        }
        ret = acp_dma_status(adata, 0);
        if ret < 0 {
            dev_err((*sdev).dev, c"acp dma transfer status: %d\n".as_ptr(), ret);
        }
    }
    if (*adata).is_sram_in_use {
        configure_pte_for_fw_loading(FW_SRAM_DATA_BIN, ACP_SRAM_PAGE_COUNT, adata);
        src_addr = ACP_SYSTEM_MEMORY_WINDOW
            .wrapping_add(ACP_DEFAULT_SRAM_LENGTH)
            .wrapping_add(page_count.wrapping_mul(ACP_PAGE_SIZE));
        if (*adata).pci_rev > ACP63_PCI_ID {
            dest_addr = ACP7X_SRAM_BASE_ADDRESS;
        } else {
            dest_addr = ACP_SRAM_BASE_ADDRESS;
        }

        ret = configure_and_run_dma(adata, src_addr, dest_addr, (*adata).fw_sram_data_bin_size);
        if ret < 0 {
            dev_err((*sdev).dev, c"acp dma configuration failed: %d\n".as_ptr(), ret);
            return ret;
        }
        ret = acp_dma_status(adata, 0);
        if ret < 0 {
            dev_err((*sdev).dev, c"acp dma transfer status: %d\n".as_ptr(), ret);
        }
    }

    if (*adata).pci_rev > ACP_RN_PCI_ID {
        /* Cache Window enable */
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_DSP0_CACHE_OFFSET0, (*desc).sram_pte_offset);
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_DSP0_CACHE_SIZE0, SRAM1_SIZE | bit(31));
    }

    /* Free memory once DMA is complete */
    dma_size = (page_align((*(*sdev).basefw.fw).size as c_uint) >> PAGE_SHIFT).wrapping_mul(ACP_PAGE_SIZE);
    dma_free_coherent(&mut (*pci).dev, dma_size as size_t, (*adata).bin_buf, (*adata).sha_dma_addr);
    (*adata).bin_buf = core::ptr::null_mut();
    if (*adata).is_dram_in_use {
        dma_free_coherent(
            &mut (*pci).dev,
            ACP_DEFAULT_DRAM_LENGTH as size_t,
            (*adata).data_buf,
            (*adata).dma_addr,
        );
        (*adata).data_buf = core::ptr::null_mut();
    }
    if (*adata).is_sram_in_use {
        dma_free_coherent(
            &mut (*pci).dev,
            ACP_DEFAULT_SRAM_LENGTH as size_t,
            (*adata).sram_data_buf,
            (*adata).sram_dma_addr,
        );
        (*adata).sram_data_buf = core::ptr::null_mut();
    }
    ret
}
// EXPORT_SYMBOL_NS(acp_dsp_pre_fw_run, "SND_SOC_SOF_AMD_COMMON");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acp_sof_dsp_run(sdev: *mut snd_sof_dev) -> c_int {
    let adata = (*(*sdev).pdata).hw_pdata;
    let desc = get_chip_info((*sdev).pdata);
    let mut val: c_int;

    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_DSP0_RUNSTALL, ACP_DSP_RUN);
    val = snd_sof_dsp_read(sdev, ACP_DSP_BAR, ACP_DSP0_RUNSTALL);
    dev_dbg((*sdev).dev, c"ACP_DSP0_RUNSTALL : 0x%0x\n".as_ptr(), val);

    /* Some platforms won't support fusion DSP,keep offset zero for no support */
    if (*desc).fusion_dsp_offset != 0 && (*adata).enable_fw_debug {
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, (*desc).fusion_dsp_offset, ACP_DSP_RUN);
        val = snd_sof_dsp_read(sdev, ACP_DSP_BAR, (*desc).fusion_dsp_offset);
        dev_dbg((*sdev).dev, c"ACP_DSP0_FUSION_RUNSTALL : 0x%0x\n".as_ptr(), val);
    }
    0
}
// EXPORT_SYMBOL_NS(acp_sof_dsp_run, "SND_SOC_SOF_AMD_COMMON");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acp_sof_load_signed_firmware(sdev: *mut snd_sof_dev) -> c_int {
    let plat_data = (*sdev).pdata;
    let adata = (*plat_data).hw_pdata;
    let mut fw_filename: *mut c_char;
    let mut ret: c_int;

    fw_filename = kasprintf(
        GFP_KERNEL,
        c"%s/%s".as_ptr(),
        (*plat_data).fw_filename_prefix,
        (*adata).fw_code_bin,
    );
    if fw_filename.is_null() {
        return -ENOMEM;
    }

    ret = request_firmware(&mut (*sdev).basefw.fw, fw_filename, (*sdev).dev);
    if ret < 0 {
        kfree(fw_filename as *const c_void);
        dev_err((*sdev).dev, c"sof signed firmware code bin is missing\n".as_ptr());
        return ret;
    } else {
        dev_dbg((*sdev).dev, c"request_firmware %s successful\n".as_ptr(), fw_filename);
    }
    kfree(fw_filename as *const c_void);

    ret = snd_sof_dsp_block_write(
        sdev,
        snd_sof_fw_blk_type::SOF_FW_BLK_TYPE_IRAM,
        0,
        (*(*sdev).basefw.fw).data as *mut c_void,
        (*(*sdev).basefw.fw).size,
    );
    if ret < 0 {
        return ret;
    }

    fw_filename = kasprintf(
        GFP_KERNEL,
        c"%s/%s".as_ptr(),
        (*plat_data).fw_filename_prefix,
        (*adata).fw_data_bin,
    );
    if fw_filename.is_null() {
        return -ENOMEM;
    }

    ret = request_firmware(&mut (*adata).fw_dbin, fw_filename, (*sdev).dev);
    if ret < 0 {
        kfree(fw_filename as *const c_void);
        dev_err((*sdev).dev, c"sof signed firmware data bin is missing\n".as_ptr());
        return ret;
    } else {
        dev_dbg((*sdev).dev, c"request_firmware %s successful\n".as_ptr(), fw_filename);
    }
    kfree(fw_filename as *const c_void);

    if (*adata).pci_rev >= ACP7B_PCI_ID {
        ret = snd_sof_dsp_block_write(
            sdev,
            snd_sof_fw_blk_type::SOF_FW_BLK_TYPE_SRAM,
            0,
            (*(*adata).fw_dbin).data as *mut c_void,
            (*(*adata).fw_dbin).size,
        );
    } else {
        ret = snd_sof_dsp_block_write(
            sdev,
            snd_sof_fw_blk_type::SOF_FW_BLK_TYPE_DRAM,
            0,
            (*(*adata).fw_dbin).data as *mut c_void,
            (*(*adata).fw_dbin).size,
        );
    }
    ret
}
// EXPORT_SYMBOL_NS(acp_sof_load_signed_firmware, "SND_SOC_SOF_AMD_COMMON");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
