/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Intel Keem Bay OCS AES Crypto Driver.
 *
 * Copyright (C) 2018-2020 Intel Corporation
 */

// Dependency declarations supplied by the surrounding kernel translation.
use crate::{completion, crypto_engine, device, dma_addr_t, irqreturn_t, list_head,
    scatterlist};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocs_cipher {
    OCS_AES = 0,
    OCS_SM4 = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocs_mode {
    OCS_MODE_ECB = 0,
    OCS_MODE_CBC = 1,
    OCS_MODE_CTR = 2,
    OCS_MODE_CCM = 6,
    OCS_MODE_GCM = 7,
    OCS_MODE_CTS = 9,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocs_instruction {
    OCS_ENCRYPT = 0,
    OCS_DECRYPT = 1,
    OCS_EXPAND = 2,
    OCS_BYPASS = 3,
}

/**
 * struct ocs_aes_dev - AES device context.
 * @list: List head for insertion into device list hold by driver.
 * @dev: OCS AES device.
 * @irq: IRQ number.
 * @base_reg: IO base address of OCS AES.
 * @irq_completion: Completion to indicate IRQ has been triggered.
 * @dma_err_mask: Error reported by OCS DMA interrupts.
 * @engine: Crypto engine for the device.
 */
#[repr(C)]
pub struct ocs_aes_dev {
    pub list: list_head,
    pub dev: *mut device,
    pub irq: core::ffi::c_int,
    pub base_reg: *mut core::ffi::c_void,
    pub irq_completion: completion,
    pub dma_err_mask: u32,
    pub engine: *mut crypto_engine,
}

/**
 * struct ocs_dll_desc - Descriptor of an OCS DMA Linked List.
 * @vaddr: Virtual address of the linked list head.
 * @dma_addr: DMA address of the linked list head.
 * @size: Size (in bytes) of the linked list.
 */
#[repr(C)]
pub struct ocs_dll_desc {
    pub vaddr: *mut core::ffi::c_void,
    pub dma_addr: dma_addr_t,
    pub size: usize,
}

extern "C" {
    pub fn ocs_aes_set_key(
        aes_dev: *mut ocs_aes_dev,
        key_size: u32,
        key: *const u8,
        cipher: ocs_cipher,
    ) -> core::ffi::c_int;

    pub fn ocs_aes_op(
        aes_dev: *mut ocs_aes_dev,
        mode: ocs_mode,
        cipher: ocs_cipher,
        instruction: ocs_instruction,
        dst_dma_list: dma_addr_t,
        src_dma_list: dma_addr_t,
        src_size: u32,
        iv: *mut u8,
        iv_size: u32,
    ) -> core::ffi::c_int;

    /** Use OCS DMA to copy data. */
    pub fn ocs_aes_gcm_op(
        aes_dev: *mut ocs_aes_dev,
        cipher: ocs_cipher,
        instruction: ocs_instruction,
        dst_dma_list: dma_addr_t,
        src_dma_list: dma_addr_t,
        src_size: u32,
        iv: *const u8,
        aad_dma_list: dma_addr_t,
        aad_size: u32,
        out_tag: *mut u8,
        tag_size: u32,
    ) -> core::ffi::c_int;

    pub fn ocs_aes_ccm_op(
        aes_dev: *mut ocs_aes_dev,
        cipher: ocs_cipher,
        instruction: ocs_instruction,
        dst_dma_list: dma_addr_t,
        src_dma_list: dma_addr_t,
        src_size: u32,
        iv: *mut u8,
        adata_dma_list: dma_addr_t,
        adata_size: u32,
        in_tag: *mut u8,
        tag_size: u32,
    ) -> core::ffi::c_int;

    pub fn ocs_create_linked_list_from_sg(
        aes_dev: *const ocs_aes_dev,
        sg: *mut scatterlist,
        sg_dma_count: core::ffi::c_int,
        dll_desc: *mut ocs_dll_desc,
        data_size: usize,
        data_offset: usize,
    ) -> core::ffi::c_int;

    pub fn ocs_aes_irq_handler(irq: core::ffi::c_int, dev_id: *mut core::ffi::c_void)
        -> irqreturn_t;
}

#[inline]
pub unsafe fn ocs_aes_bypass_op(
    aes_dev: *mut ocs_aes_dev,
    dst_dma_list: dma_addr_t,
    src_dma_list: dma_addr_t,
    src_size: u32,
) -> core::ffi::c_int {
    ocs_aes_op(
        aes_dev,
        ocs_mode::OCS_MODE_ECB,
        ocs_cipher::OCS_AES,
        ocs_instruction::OCS_BYPASS,
        dst_dma_list,
        src_dma_list,
        src_size,
        core::ptr::null_mut(),
        0,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
