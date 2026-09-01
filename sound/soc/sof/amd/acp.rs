// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021, 2023 Advanced Micro Devices, Inc. All rights reserved.
//
// Authors: Vijendar Mukunda <Vijendar.Mukunda@amd.com>
//	    Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>

/*
 * Hardware interface for generic AMD ACP processor
 */

// C dependencies: linux/acpi.h, linux/io.h, linux/module.h, linux/pci.h,
// asm/amd/node.h, ../ops.h, acp.h, acp-dsp-offset.h.

use core::ffi::c_void;
use core::mem::{offset_of, size_of};
use core::ptr;

static mut enable_fw_debug: bool = false;
// module_param(enable_fw_debug, bool, 0444);
// MODULE_PARM_DESC(enable_fw_debug, "Enable Firmware debug");

static mut quirk_valve_galileo: acp_quirk_entry = acp_quirk_entry {
    signed_fw_image: true,
    skip_iram_dram_size_mod: true,
    post_fw_run_delay: true,
};

#[no_mangle]
pub static mut acp_sof_quirk_table: [dmi_system_id; 2] = [
    dmi_system_id {
        /* Steam Deck OLED device */
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, b"Valve\0".as_ptr() as *const i8),
            DMI_MATCH(DMI_PRODUCT_NAME, b"Galileo\0".as_ptr() as *const i8),
        ],
        driver_data: unsafe { &raw mut quirk_valve_galileo as *mut c_void },
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];
// EXPORT_SYMBOL_GPL(acp_sof_quirk_table);

unsafe fn init_dma_descriptor(adata: *mut acp_dev_data) {
    let sdev: *mut snd_sof_dev = (*adata).dev;
    let desc: *const sof_amd_acp_desc = get_chip_info((*(*sdev).pdata));
    let acp_data: *mut acp_dev_data = (*(*sdev).pdata).hw_pdata as *mut acp_dev_data;
    let addr: u32;
    let acp_dma_desc_base_addr: u32;
    let acp_dma_desc_max_num_dscr: u32;

    addr = ((*desc).sram_pte_offset + (*sdev).debug_box.offset
        + offset_of!(scratch_reg_conf, dma_desc) as u32) as u32;

    match (*acp_data).pci_rev {
        ACP70_PCI_ID | ACP71_PCI_ID | ACP72_PCI_ID | ACP7B_PCI_ID | ACP7F_PCI_ID => {
            acp_dma_desc_base_addr = ACP70_DMA_DESC_BASE_ADDR;
            acp_dma_desc_max_num_dscr = ACP70_DMA_DESC_MAX_NUM_DSCR;
        }
        _ => {
            acp_dma_desc_base_addr = ACP_DMA_DESC_BASE_ADDR;
            acp_dma_desc_max_num_dscr = ACP_DMA_DESC_MAX_NUM_DSCR;
        }
    }
    snd_sof_dsp_write(sdev, ACP_DSP_BAR, acp_dma_desc_base_addr, addr);
    snd_sof_dsp_write(
        sdev,
        ACP_DSP_BAR,
        acp_dma_desc_max_num_dscr,
        ACP_MAX_DESC_CNT,
    );
}

unsafe fn configure_dma_descriptor(
    adata: *mut acp_dev_data,
    idx: u16,
    dscr_info: *mut dma_descriptor,
) {
    let sdev: *mut snd_sof_dev = (*adata).dev;
    let offset: u32;

    offset = ACP_SCRATCH_REG_0
        + (*sdev).debug_box.offset
        + offset_of!(scratch_reg_conf, dma_desc) as u32
        + (idx as u32) * size_of::<dma_descriptor>() as u32;

    snd_sof_dsp_write(sdev, ACP_DSP_BAR, offset, (*dscr_info).src_addr);
    snd_sof_dsp_write(sdev, ACP_DSP_BAR, offset + 0x4, (*dscr_info).dest_addr);
    snd_sof_dsp_write(sdev, ACP_DSP_BAR, offset + 0x8, (*dscr_info).tx_cnt.u32_all);
}

unsafe fn config_dma_channel(
    adata: *mut acp_dev_data,
    ch: u32,
    idx: u32,
    dscr_count: u32,
) -> i32 {
    let sdev: *mut snd_sof_dev = (*adata).dev;
    let acp_data: *mut acp_dev_data = (*(*sdev).pdata).hw_pdata as *mut acp_dev_data;
    let desc: *const sof_amd_acp_desc = get_chip_info((*(*sdev).pdata));
    let mut val: u32 = 0;
    let status: u32;
    let acp_dma_cntl_0: u32;
    let acp_dma_ch_rst_sts: u32;
    let acp_dma_dscr_err_sts_0: u32;
    let acp_dma_dscr_cnt_0: u32;
    let acp_dma_prio_0: u32;
    let acp_dma_dscr_strt_idx_0: u32;
    let ret: i32;

    match (*acp_data).pci_rev {
        ACP70_PCI_ID | ACP71_PCI_ID | ACP72_PCI_ID | ACP7B_PCI_ID | ACP7F_PCI_ID => {
            acp_dma_cntl_0 = ACP70_DMA_CNTL_0;
            acp_dma_ch_rst_sts = ACP70_DMA_CH_RST_STS;
            acp_dma_dscr_err_sts_0 = ACP70_DMA_ERR_STS_0;
            acp_dma_dscr_cnt_0 = ACP70_DMA_DSCR_CNT_0;
            acp_dma_prio_0 = ACP70_DMA_PRIO_0;
            acp_dma_dscr_strt_idx_0 = ACP70_DMA_DSCR_STRT_IDX_0;
        }
        _ => {
            acp_dma_cntl_0 = ACP_DMA_CNTL_0;
            acp_dma_ch_rst_sts = ACP_DMA_CH_RST_STS;
            acp_dma_dscr_err_sts_0 = ACP_DMA_ERR_STS_0;
            acp_dma_dscr_cnt_0 = ACP_DMA_DSCR_CNT_0;
            acp_dma_prio_0 = ACP_DMA_PRIO_0;
            acp_dma_dscr_strt_idx_0 = ACP_DMA_DSCR_STRT_IDX_0;
        }
    }

    snd_sof_dsp_write(
        sdev,
        ACP_DSP_BAR,
        acp_dma_cntl_0 + ch * size_of::<u32>() as u32,
        ACP_DMA_CH_RST | ACP_DMA_CH_GRACEFUL_RST_EN,
    );

    ret = snd_sof_dsp_read_poll_timeout(
        sdev,
        ACP_DSP_BAR,
        acp_dma_ch_rst_sts,
        &mut val,
        val & (1 << ch) != 0,
        ACP_REG_POLL_INTERVAL,
        ACP_REG_POLL_TIMEOUT_US,
    );
    if ret < 0 {
        status = snd_sof_dsp_read(sdev, ACP_DSP_BAR, (*desc).acp_error_stat);
        val = snd_sof_dsp_read(
            sdev,
            ACP_DSP_BAR,
            acp_dma_dscr_err_sts_0 + ch * size_of::<u32>() as u32,
        );

        dev_err(
            (*sdev).dev,
            b"ACP_DMA_ERR_STS :0x%x ACP_ERROR_STATUS :0x%x\n\0".as_ptr() as *const i8,
            val,
            status,
        );
        return ret;
    }

    snd_sof_dsp_write(
        sdev,
        ACP_DSP_BAR,
        acp_dma_cntl_0 + ch * size_of::<u32>() as u32,
        0,
    );
    snd_sof_dsp_write(
        sdev,
        ACP_DSP_BAR,
        acp_dma_dscr_cnt_0 + ch * size_of::<u32>() as u32,
        dscr_count,
    );
    snd_sof_dsp_write(
        sdev,
        ACP_DSP_BAR,
        acp_dma_dscr_strt_idx_0 + ch * size_of::<u32>() as u32,
        idx,
    );
    snd_sof_dsp_write(
        sdev,
        ACP_DSP_BAR,
        acp_dma_prio_0 + ch * size_of::<u32>() as u32,
        0,
    );
    snd_sof_dsp_write(
        sdev,
        ACP_DSP_BAR,
        acp_dma_cntl_0 + ch * size_of::<u32>() as u32,
        ACP_DMA_CH_RUN,
    );

    ret
}

unsafe fn acpbus_dma_start(
    adata: *mut acp_dev_data,
    ch: u32,
    dscr_count: u32,
    mut dscr_info: *mut dma_descriptor,
) -> i32 {
    let sdev: *mut snd_sof_dev = (*adata).dev;
    let ret: i32;
    let mut dscr: u16;

    if dscr_info.is_null() || dscr_count == 0 {
        return -EINVAL;
    }

    dscr = 0;
    while (dscr as u32) < dscr_count {
        configure_dma_descriptor(adata, dscr, dscr_info);
        dscr_info = dscr_info.add(1);
        dscr += 1;
    }

    ret = config_dma_channel(adata, ch, 0, dscr_count);
    if ret < 0 {
        dev_err((*sdev).dev, b"config dma ch failed:%d\n\0".as_ptr() as *const i8, ret);
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn configure_and_run_dma(
    adata: *mut acp_dev_data,
    src_addr: u32,
    dest_addr: u32,
    mut dsp_data_size: i32,
) -> i32 {
    let sdev: *mut snd_sof_dev = (*adata).dev;
    let mut desc_count: u32 = 0;
    let mut index: u32;
    let ret: i32;

    while desc_count < ACP_MAX_DESC && dsp_data_size >= 0 {
        (*adata).dscr_info[desc_count as usize].src_addr =
            src_addr + desc_count * ACP_PAGE_SIZE;
        (*adata).dscr_info[desc_count as usize].dest_addr =
            dest_addr + desc_count * ACP_PAGE_SIZE;
        (*adata).dscr_info[desc_count as usize].tx_cnt.bits.count = ACP_PAGE_SIZE;
        if dsp_data_size < ACP_PAGE_SIZE as i32 {
            (*adata).dscr_info[desc_count as usize].tx_cnt.bits.count = dsp_data_size as u32;
        }
        desc_count += 1;
        dsp_data_size -= ACP_PAGE_SIZE as i32;
    }

    ret = acpbus_dma_start(adata, 0, desc_count, (*adata).dscr_info.as_mut_ptr());
    if ret != 0 {
        dev_err((*sdev).dev, b"acpbus_dma_start failed\n\0".as_ptr() as *const i8);
    }

    /* Clear descriptor array */
    index = 0;
    while index < desc_count {
        ptr::write_bytes(
            &mut (*adata).dscr_info[index as usize] as *mut dma_descriptor as *mut u8,
            0x00,
            size_of::<dma_descriptor>(),
        );
        index += 1;
    }

    ret
}

/*
 * psp_mbox_ready- function to poll ready bit of psp mbox
 * @adata: acp device data
 * @ack: bool variable to check ready bit status or psp ack
 */

unsafe fn psp_mbox_ready(adata: *mut acp_dev_data, ack: bool) -> i32 {
    let sdev: *mut snd_sof_dev = (*adata).dev;
    let ret: i32;
    let mut data: i32 = 0;

    ret = read_poll_timeout(
        smn_read_register,
        &mut data,
        data > 0 && (data & MBOX_READY_MASK as i32) != 0,
        MBOX_DELAY_US,
        ACP_PSP_TIMEOUT_US,
        false,
        MP0_C2PMSG_114_REG,
    );

    if ret == 0 {
        return 0;
    }

    dev_err(
        (*sdev).dev,
        b"PSP error status %x\n\0".as_ptr() as *const i8,
        data & MBOX_STATUS_MASK as i32,
    );

    if ack {
        return -ETIMEDOUT;
    }

    -EBUSY
}

/*
 * psp_send_cmd - function to send psp command over mbox
 * @adata: acp device data
 * @cmd: non zero integer value for command type
 */

unsafe fn psp_send_cmd(adata: *mut acp_dev_data, cmd: i32) -> i32 {
    let sdev: *mut snd_sof_dev = (*adata).dev;
    let mut ret: i32;
    let mut data: i32 = 0;

    if cmd == 0 {
        return -EINVAL;
    }

    /* Get a non-zero Doorbell value from PSP */
    ret = read_poll_timeout(
        smn_read_register,
        &mut data,
        data > 0,
        MBOX_DELAY_US,
        ACP_PSP_TIMEOUT_US,
        false,
        MP0_C2PMSG_73_REG,
    );

    if ret != 0 {
        dev_err(
            (*sdev).dev,
            b"Failed to get Doorbell from MBOX %x\n\0".as_ptr() as *const i8,
            MP0_C2PMSG_73_REG,
        );
        return ret;
    }

    /* Check if PSP is ready for new command */
    ret = psp_mbox_ready(adata, false);
    if ret != 0 {
        return ret;
    }

    ret = amd_smn_write(0, MP0_C2PMSG_114_REG, cmd);
    if ret != 0 {
        return ret;
    }

    /* Ring the Doorbell for PSP */
    ret = amd_smn_write(0, MP0_C2PMSG_73_REG, data);
    if ret != 0 {
        return ret;
    }

    /* Check MBOX ready as PSP ack */
    ret = psp_mbox_ready(adata, true);

    ret
}

#[no_mangle]
pub unsafe extern "C" fn configure_and_run_sha_dma(
    adata: *mut acp_dev_data,
    image_addr: *mut c_void,
    start_addr: u32,
    dest_addr: u32,
    image_length: u32,
) -> i32 {
    let sdev: *mut snd_sof_dev = (*adata).dev;
    let mut tx_count: u32 = 0;
    let mut fw_qualifier: u32 = 0;
    let mut val: u32;
    let mut ret: i32;

    if image_addr.is_null() {
        dev_err((*sdev).dev, b"SHA DMA image address is NULL\n\0".as_ptr() as *const i8);
        return -EINVAL;
    }

    val = snd_sof_dsp_read(sdev, ACP_DSP_BAR, ACP_SHA_DMA_CMD);
    if val & ACP_SHA_RUN != 0 {
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_SHA_DMA_CMD, ACP_SHA_RESET);
        ret = snd_sof_dsp_read_poll_timeout(
            sdev,
            ACP_DSP_BAR,
            ACP_SHA_DMA_CMD_STS,
            &mut val,
            val & ACP_SHA_RESET != 0,
            ACP_REG_POLL_INTERVAL,
            ACP_REG_POLL_TIMEOUT_US,
        );
        if ret < 0 {
            dev_err((*sdev).dev, b"SHA DMA Failed to Reset\n\0".as_ptr() as *const i8);
            return ret;
        }
    }

    if (!(*adata).quirks.is_null() && (*(*adata).quirks).signed_fw_image)
        || (*adata).acp_sof_signed_firmware_image
    {
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_SHA_DMA_INCLUDE_HDR, ACP_SHA_HEADER);
    }

    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_SHA_DMA_STRT_ADDR, start_addr);
    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_SHA_DMA_DESTINATION_ADDR, dest_addr);
    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_SHA_MSG_LENGTH, image_length);

    /* psp_send_cmd only required for vangogh platform */
    if (*adata).pci_rev == ACP_VANGOGH_PCI_ID
        && !(!(*adata).quirks.is_null() && (*(*adata).quirks).skip_iram_dram_size_mod)
    {
        /* Modify IRAM and DRAM size */
        ret = psp_send_cmd(
            adata,
            (MBOX_ACP_IRAM_DRAM_FENCE_COMMAND | IRAM_DRAM_FENCE_2) as i32,
        );
        if ret != 0 {
            return ret;
        }
        ret = psp_send_cmd(
            adata,
            (MBOX_ACP_IRAM_DRAM_FENCE_COMMAND | MBOX_ISREADY_FLAG) as i32,
        );
        if ret != 0 {
            return ret;
        }
    }
    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_SHA_DMA_CMD, ACP_SHA_RUN);

    ret = snd_sof_dsp_read_poll_timeout(
        sdev,
        ACP_DSP_BAR,
        ACP_SHA_TRANSFER_BYTE_CNT,
        &mut tx_count,
        tx_count == image_length,
        ACP_REG_POLL_INTERVAL,
        ACP_DMA_COMPLETE_TIMEOUT_US,
    );
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            b"SHA DMA Failed to Transfer Length %x\n\0".as_ptr() as *const i8,
            tx_count,
        );
        return ret;
    }

    /* psp_send_cmd only required for renoir platform*/
    if (*adata).pci_rev == ACP_RN_PCI_ID {
        ret = psp_send_cmd(adata, MBOX_ACP_SHA_DMA_COMMAND as i32);
        if ret != 0 {
            return ret;
        }
    }

    ret = snd_sof_dsp_read_poll_timeout(
        sdev,
        ACP_DSP_BAR,
        ACP_SHA_DSP_FW_QUALIFIER,
        &mut fw_qualifier,
        fw_qualifier & DSP_FW_RUN_ENABLE != 0,
        ACP_REG_POLL_INTERVAL,
        ACP_DMA_COMPLETE_TIMEOUT_US,
    );
    if ret < 0 {
        val = snd_sof_dsp_read(sdev, ACP_DSP_BAR, ACP_SHA_PSP_ACK);
        dev_err(
            (*sdev).dev,
            b"PSP validation failed: fw_qualifier = %#x, ACP_SHA_PSP_ACK = %#x\n\0".as_ptr()
                as *const i8,
            fw_qualifier,
            val,
        );
        return ret;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn acp_dma_status(adata: *mut acp_dev_data, ch: u8) -> i32 {
    let sdev: *mut snd_sof_dev = (*adata).dev;
    let mut val: u32;
    let acp_dma_ch_sts: u32;
    let mut ret: i32 = 0;

    match (*adata).pci_rev {
        ACP70_PCI_ID | ACP71_PCI_ID | ACP72_PCI_ID | ACP7B_PCI_ID | ACP7F_PCI_ID => {
            acp_dma_ch_sts = ACP70_DMA_CH_STS;
        }
        _ => {
            acp_dma_ch_sts = ACP_DMA_CH_STS;
        }
    }
    val = snd_sof_dsp_read(
        sdev,
        ACP_DSP_BAR,
        ACP_DMA_CNTL_0 + (ch as u32) * size_of::<u32>() as u32,
    );
    if val & ACP_DMA_CH_RUN != 0 {
        ret = snd_sof_dsp_read_poll_timeout(
            sdev,
            ACP_DSP_BAR,
            acp_dma_ch_sts,
            &mut val,
            val == 0,
            ACP_REG_POLL_INTERVAL,
            ACP_DMA_COMPLETE_TIMEOUT_US,
        );
        if ret < 0 {
            dev_err(
                (*sdev).dev,
                b"DMA_CHANNEL %d status timeout\n\0".as_ptr() as *const i8,
                ch as i32,
            );
        }
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn memcpy_from_scratch(
    sdev: *mut snd_sof_dev,
    offset: u32,
    dst: *mut u32,
    bytes: usize,
) {
    let reg_offset: u32 = offset + ACP_SCRATCH_REG_0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;

    while (i as usize) < bytes {
        *dst.add(j as usize) = snd_sof_dsp_read(sdev, ACP_DSP_BAR, reg_offset + i as u32);
        i += 4;
        j += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn memcpy_to_scratch(
    sdev: *mut snd_sof_dev,
    offset: u32,
    src: *mut u32,
    bytes: usize,
) {
    let reg_offset: u32 = offset + ACP_SCRATCH_REG_0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;

    while (i as usize) < bytes {
        snd_sof_dsp_write(
            sdev,
            ACP_DSP_BAR,
            reg_offset + i as u32,
            *src.add(j as usize),
        );
        i += 4;
        j += 1;
    }
}

unsafe fn acp_init_scratch_mem_ipc_flags(sdev: *mut snd_sof_dev) -> i32 {
    let dsp_msg_write: u32;
    let dsp_ack_write: u32;
    let host_msg_write: u32;
    let host_ack_write: u32;

    dsp_msg_write =
        (*sdev).debug_box.offset + offset_of!(scratch_ipc_conf, sof_dsp_msg_write) as u32;
    dsp_ack_write =
        (*sdev).debug_box.offset + offset_of!(scratch_ipc_conf, sof_dsp_ack_write) as u32;
    host_msg_write =
        (*sdev).debug_box.offset + offset_of!(scratch_ipc_conf, sof_host_msg_write) as u32;
    host_ack_write =
        (*sdev).debug_box.offset + offset_of!(scratch_ipc_conf, sof_host_ack_write) as u32;
    /* Initialize host message write flag */
    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_SCRATCH_REG_0 + host_msg_write, 0);

    /* Initialize host ack write flag */
    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_SCRATCH_REG_0 + host_ack_write, 0);

    /* Initialize DSP message write flag */
    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_SCRATCH_REG_0 + dsp_msg_write, 0);

    /* Initialize DSP ack write flag */
    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_SCRATCH_REG_0 + dsp_ack_write, 0);

    0
}

unsafe fn acp_memory_init(sdev: *mut snd_sof_dev) -> i32 {
    let adata: *mut acp_dev_data = (*(*sdev).pdata).hw_pdata as *mut acp_dev_data;
    let desc: *const sof_amd_acp_desc = get_chip_info((*(*sdev).pdata));

    snd_sof_dsp_update_bits(
        sdev,
        ACP_DSP_BAR,
        (*desc).dsp_intr_base + DSP_SW_INTR_CNTL_OFFSET,
        ACP_DSP_INTR_EN_MASK,
        ACP_DSP_INTR_EN_MASK,
    );
    acp_init_scratch_mem_ipc_flags(sdev);
    init_dma_descriptor(adata);

    0
}

unsafe fn amd_sof_handle_acp70_sdw_wake_event(adata: *mut acp_dev_data) {
    let mut amd_manager: *mut amd_sdw_manager;

    if (*adata).acp70_sdw0_wake_event {
        amd_manager = dev_get_drvdata(&mut (*(*(*adata).sdw).pdev[0]).dev) as *mut amd_sdw_manager;
        if !amd_manager.is_null() {
            pm_request_resume((*amd_manager).dev);
        }
        (*adata).acp70_sdw0_wake_event = false;
    }

    if (*adata).acp70_sdw1_wake_event {
        amd_manager = dev_get_drvdata(&mut (*(*(*adata).sdw).pdev[1]).dev) as *mut amd_sdw_manager;
        if !amd_manager.is_null() {
            pm_request_resume((*amd_manager).dev);
        }
        (*adata).acp70_sdw1_wake_event = false;
    }
}

unsafe fn amd_sof_check_and_handle_acp70_sdw_wake_irq(sdev: *mut snd_sof_dev) -> i32 {
    let desc: *const sof_amd_acp_desc = get_chip_info((*(*sdev).pdata));
    let adata: *mut acp_dev_data = (*(*sdev).pdata).hw_pdata as *mut acp_dev_data;
    let mut ext_intr_stat1: u32;
    let mut irq_flag: i32 = 0;
    let mut sdw_wake_irq: bool = false;

    ext_intr_stat1 = snd_sof_dsp_read(sdev, ACP_DSP_BAR, (*desc).ext_intr_stat1);
    if ext_intr_stat1 & ACP70_SDW0_HOST_WAKE_STAT != 0 {
        snd_sof_dsp_write(
            sdev,
            ACP_DSP_BAR,
            (*desc).ext_intr_stat1,
            ACP70_SDW0_HOST_WAKE_STAT,
        );
        (*adata).acp70_sdw0_wake_event = true;
        sdw_wake_irq = true;
    }

    if ext_intr_stat1 & ACP70_SDW1_HOST_WAKE_STAT != 0 {
        snd_sof_dsp_write(
            sdev,
            ACP_DSP_BAR,
            (*desc).ext_intr_stat1,
            ACP70_SDW1_HOST_WAKE_STAT,
        );
        (*adata).acp70_sdw1_wake_event = true;
        sdw_wake_irq = true;
    }

    if ext_intr_stat1 & ACP70_SDW0_PME_STAT != 0 {
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP70_SW0_WAKE_EN, 0);
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, (*desc).ext_intr_stat1, ACP70_SDW0_PME_STAT);
        (*adata).acp70_sdw0_wake_event = true;
        sdw_wake_irq = true;
    }

    if ext_intr_stat1 & ACP70_SDW1_PME_STAT != 0 {
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP70_SW1_WAKE_EN, 0);
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, (*desc).ext_intr_stat1, ACP70_SDW1_PME_STAT);
        (*adata).acp70_sdw1_wake_event = true;
        sdw_wake_irq = true;
    }

    if sdw_wake_irq {
        amd_sof_handle_acp70_sdw_wake_event(adata);
        irq_flag = 1;
    }
    irq_flag
}

unsafe extern "C" fn acp_irq_thread(irq: i32, context: *mut c_void) -> irqreturn_t {
    let sdev: *mut snd_sof_dev = context as *mut snd_sof_dev;
    let desc: *const sof_amd_acp_desc = get_chip_info((*(*sdev).pdata));
    let mut count: u32 = ACP_HW_SEM_RETRY_COUNT;

    spin_lock_irq(&mut (*sdev).ipc_lock);
    /* Wait until acquired HW Semaphore lock or timeout */
    while snd_sof_dsp_read(sdev, ACP_DSP_BAR, (*desc).hw_semaphore_offset) != 0 && {
        count -= 1;
        count != 0
    } {}
    spin_unlock_irq(&mut (*sdev).ipc_lock);

    if count == 0 {
        dev_err(
            (*sdev).dev,
            b"%s: Failed to acquire HW lock\n\0".as_ptr() as *const i8,
            b"acp_irq_thread\0".as_ptr() as *const i8,
        );
        return IRQ_NONE;
    }

    ((*sof_ops(sdev)).irq_thread)(irq, sdev as *mut c_void);
    /* Unlock or Release HW Semaphore */
    snd_sof_dsp_write(sdev, ACP_DSP_BAR, (*desc).hw_semaphore_offset, 0x0);

    IRQ_HANDLED
}

unsafe extern "C" fn acp_irq_handler(_irq: i32, dev_id: *mut c_void) -> irqreturn_t {
    let mut amd_manager: *mut amd_sdw_manager;
    let sdev: *mut snd_sof_dev = dev_id as *mut snd_sof_dev;
    let desc: *const sof_amd_acp_desc = get_chip_info((*(*sdev).pdata));
    let adata: *mut acp_dev_data = (*(*sdev).pdata).hw_pdata as *mut acp_dev_data;
    let base: u32 = (*desc).dsp_intr_base;
    let mut val: u32;
    let mut irq_flag: i32 = 0;
    let mut wake_irq_flag: i32 = 0;

    val = snd_sof_dsp_read(sdev, ACP_DSP_BAR, base + DSP_SW_INTR_STAT_OFFSET);
    if val & ACP_DSP_TO_HOST_IRQ != 0 {
        snd_sof_dsp_write(
            sdev,
            ACP_DSP_BAR,
            base + DSP_SW_INTR_STAT_OFFSET,
            ACP_DSP_TO_HOST_IRQ,
        );
        return IRQ_WAKE_THREAD;
    }

    val = snd_sof_dsp_read(sdev, ACP_DSP_BAR, (*desc).ext_intr_stat);
    if val & ACP_SDW0_IRQ_MASK != 0 {
        amd_manager = dev_get_drvdata(&mut (*(*(*adata).sdw).pdev[0]).dev) as *mut amd_sdw_manager;
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, (*desc).ext_intr_stat, ACP_SDW0_IRQ_MASK);
        if !amd_manager.is_null() {
            schedule_work(&mut (*amd_manager).amd_sdw_irq_thread);
        }
        irq_flag = 1;
    }

    if val & ACP_ERROR_IRQ_MASK != 0 {
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, (*desc).ext_intr_stat, ACP_ERROR_IRQ_MASK);
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, (*desc).acp_sw0_i2s_err_reason, 0);
        /* ACP_SW1_I2S_ERROR_REASON is newly added register from rmb platform onwards */
        match (*adata).pci_rev {
            ACP_RMB_PCI_ID | ACP63_PCI_ID | ACP70_PCI_ID | ACP71_PCI_ID | ACP72_PCI_ID => {
                snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_SW1_I2S_ERROR_REASON, 0);
            }
            _ => {}
        }
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, (*desc).acp_error_stat, 0);
        irq_flag = 1;
    }

    if (*desc).ext_intr_stat1 != 0 {
        val = snd_sof_dsp_read(sdev, ACP_DSP_BAR, (*desc).ext_intr_stat1);
        if val & ACP_SDW1_IRQ_MASK != 0 {
            amd_manager =
                dev_get_drvdata(&mut (*(*(*adata).sdw).pdev[1]).dev) as *mut amd_sdw_manager;
            snd_sof_dsp_write(sdev, ACP_DSP_BAR, (*desc).ext_intr_stat1, ACP_SDW1_IRQ_MASK);
            if !amd_manager.is_null() {
                schedule_work(&mut (*amd_manager).amd_sdw_irq_thread);
            }
            irq_flag = 1;
        }
        match (*adata).pci_rev {
            ACP70_PCI_ID | ACP71_PCI_ID | ACP72_PCI_ID => {
                wake_irq_flag = amd_sof_check_and_handle_acp70_sdw_wake_irq(sdev);
            }
            _ => {}
        }
    }
    if irq_flag != 0 || wake_irq_flag != 0 {
        IRQ_HANDLED
    } else {
        IRQ_NONE
    }
}

unsafe extern "C" fn acp7x_irq_handler(_irq: i32, dev_id: *mut c_void) -> irqreturn_t {
    let sdev: *mut snd_sof_dev = dev_id as *mut snd_sof_dev;
    let desc: *const sof_amd_acp_desc = get_chip_info((*(*sdev).pdata));
    let base: u32 = (*desc).dsp_intr_base;
    let mut val: u32;
    let ext_intr_stat: u32;
    let mut irq_flag: i32 = 0;

    val = snd_sof_dsp_read(sdev, ACP_DSP_BAR, base + DSP_SW_INTR_STAT_OFFSET);
    if val & ACP_DSP_TO_HOST_IRQ != 0 {
        snd_sof_dsp_write(
            sdev,
            ACP_DSP_BAR,
            base + DSP_SW_INTR_STAT_OFFSET,
            ACP_DSP_TO_HOST_IRQ,
        );
        return IRQ_WAKE_THREAD;
    }

    ext_intr_stat = snd_sof_dsp_read(sdev, ACP_DSP_BAR, (*desc).ext_intr_stat);
    if ext_intr_stat & ACP_ERROR_IRQ_MASK != 0 {
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, (*desc).ext_intr_stat, ACP_ERROR_IRQ_MASK);
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, (*desc).acp_error_stat, 0);
        irq_flag = 1;
    }

    if irq_flag != 0 {
        return IRQ_HANDLED;
    }

    IRQ_NONE
}

unsafe fn acp_power_on(sdev: *mut snd_sof_dev) -> i32 {
    let desc: *const sof_amd_acp_desc = get_chip_info((*(*sdev).pdata));
    let adata: *mut acp_dev_data = (*(*sdev).pdata).hw_pdata as *mut acp_dev_data;
    let base: u32 = (*desc).pgfsm_base;
    let mut val: u32;
    let acp_pgfsm_status_mask: u32;
    let acp_pgfsm_cntl_mask: u32;
    let mut use_masked_status: bool = false;
    let ret: i32;

    val = snd_sof_dsp_read(sdev, ACP_DSP_BAR, base + PGFSM_STATUS_OFFSET);

    if val == ACP_POWERED_ON {
        return 0;
    }

    match (*adata).pci_rev {
        ACP_RN_PCI_ID | ACP_VANGOGH_PCI_ID => {
            acp_pgfsm_status_mask = ACP3X_PGFSM_STATUS_MASK;
            acp_pgfsm_cntl_mask = ACP3X_PGFSM_CNTL_POWER_ON_MASK;
        }
        ACP_RMB_PCI_ID | ACP63_PCI_ID => {
            acp_pgfsm_status_mask = ACP6X_PGFSM_STATUS_MASK;
            acp_pgfsm_cntl_mask = ACP6X_PGFSM_CNTL_POWER_ON_MASK;
        }
        ACP70_PCI_ID | ACP71_PCI_ID | ACP72_PCI_ID => {
            acp_pgfsm_status_mask = ACP70_PGFSM_STATUS_MASK;
            acp_pgfsm_cntl_mask = ACP70_PGFSM_CNTL_POWER_ON_MASK;
        }
        ACP7B_PCI_ID | ACP7F_PCI_ID => {
            acp_pgfsm_status_mask = ACP7X_PGFSM_STATUS_MASK;
            acp_pgfsm_cntl_mask = ACP7X_PGFSM_CNTL_POWER_ON_MASK;
            use_masked_status = true;
        }
        _ => {
            return -EINVAL;
        }
    }

    if val & acp_pgfsm_status_mask != 0 {
        snd_sof_dsp_write(
            sdev,
            ACP_DSP_BAR,
            base + PGFSM_CONTROL_OFFSET,
            acp_pgfsm_cntl_mask,
        );
    }

    if use_masked_status {
        ret = snd_sof_dsp_read_poll_timeout(
            sdev,
            ACP_DSP_BAR,
            base + PGFSM_STATUS_OFFSET,
            &mut val,
            val & acp_pgfsm_status_mask == 0,
            ACP_REG_POLL_INTERVAL,
            ACP_REG_POLL_TIMEOUT_US,
        );
    } else {
        ret = snd_sof_dsp_read_poll_timeout(
            sdev,
            ACP_DSP_BAR,
            base + PGFSM_STATUS_OFFSET,
            &mut val,
            val == 0,
            ACP_REG_POLL_INTERVAL,
            ACP_REG_POLL_TIMEOUT_US,
        );
    }
    if ret < 0 {
        dev_err((*sdev).dev, b"timeout in ACP_PGFSM_STATUS read\n\0".as_ptr() as *const i8);
    }

    ret
}

unsafe fn acp_reset(sdev: *mut snd_sof_dev) -> i32 {
    let mut val: u32 = 0;
    let mut ret: i32;

    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_SOFT_RESET, ACP_ASSERT_RESET);

    ret = snd_sof_dsp_read_poll_timeout(
        sdev,
        ACP_DSP_BAR,
        ACP_SOFT_RESET,
        &mut val,
        val & ACP_SOFT_RESET_DONE_MASK != 0,
        ACP_REG_POLL_INTERVAL,
        ACP_REG_POLL_TIMEOUT_US,
    );
    if ret < 0 {
        dev_err((*sdev).dev, b"timeout asserting reset\n\0".as_ptr() as *const i8);
        return ret;
    }

    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_SOFT_RESET, ACP_RELEASE_RESET);

    ret = snd_sof_dsp_read_poll_timeout(
        sdev,
        ACP_DSP_BAR,
        ACP_SOFT_RESET,
        &mut val,
        val == 0,
        ACP_REG_POLL_INTERVAL,
        ACP_REG_POLL_TIMEOUT_US,
    );
    if ret < 0 {
        dev_err((*sdev).dev, b"timeout in releasing reset\n\0".as_ptr() as *const i8);
    }

    ret
}

unsafe fn acp_dsp_reset(sdev: *mut snd_sof_dev) -> i32 {
    let mut val: u32 = 0;
    let mut ret: i32;

    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_SOFT_RESET, ACP_DSP_ASSERT_RESET);

    ret = snd_sof_dsp_read_poll_timeout(
        sdev,
        ACP_DSP_BAR,
        ACP_SOFT_RESET,
        &mut val,
        val & ACP_DSP_SOFT_RESET_DONE_MASK != 0,
        ACP_REG_POLL_INTERVAL,
        ACP_REG_POLL_TIMEOUT_US,
    );
    if ret < 0 {
        dev_err((*sdev).dev, b"timeout asserting reset\n\0".as_ptr() as *const i8);
        return ret;
    }

    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_SOFT_RESET, ACP_DSP_RELEASE_RESET);

    ret = snd_sof_dsp_read_poll_timeout(
        sdev,
        ACP_DSP_BAR,
        ACP_SOFT_RESET,
        &mut val,
        val == 0,
        ACP_REG_POLL_INTERVAL,
        ACP_REG_POLL_TIMEOUT_US,
    );
    if ret < 0 {
        dev_err((*sdev).dev, b"timeout in releasing reset\n\0".as_ptr() as *const i8);
    }

    ret
}

unsafe fn acp_init(sdev: *mut snd_sof_dev) -> i32 {
    let desc: *const sof_amd_acp_desc = get_chip_info((*(*sdev).pdata));
    let acp_data: *mut acp_dev_data;
    let sdw0_wake_en: u32;
    let sdw1_wake_en: u32;
    let ret: i32;

    /* power on */
    acp_data = (*(*sdev).pdata).hw_pdata as *mut acp_dev_data;
    ret = acp_power_on(sdev);
    if ret != 0 {
        dev_err((*sdev).dev, b"ACP power on failed\n\0".as_ptr() as *const i8);
        return ret;
    }

    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_CONTROL, 0x01);
    /* Reset */
    let ret = acp_reset(sdev);
    if ret != 0 {
        return ret;
    }

    if (*desc).acp_clkmux_sel != 0 {
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, (*desc).acp_clkmux_sel, ACP_CLOCK_ACLK);
    }

    if (*desc).ext_intr_enb != 0 {
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, (*desc).ext_intr_enb, 0x01);
    }

    if (*desc).ext_intr_cntl != 0 {
        snd_sof_dsp_write(sdev, ACP_DSP_BAR, (*desc).ext_intr_cntl, ACP_ERROR_IRQ_MASK);
    }

    match (*acp_data).pci_rev {
        ACP70_PCI_ID | ACP71_PCI_ID | ACP72_PCI_ID => {
            sdw0_wake_en = snd_sof_dsp_read(sdev, ACP_DSP_BAR, ACP70_SW0_WAKE_EN);
            sdw1_wake_en = snd_sof_dsp_read(sdev, ACP_DSP_BAR, ACP70_SW1_WAKE_EN);
            if sdw0_wake_en != 0 || sdw1_wake_en != 0 {
                snd_sof_dsp_update_bits(
                    sdev,
                    ACP_DSP_BAR,
                    ACP70_EXTERNAL_INTR_CNTL1,
                    ACP70_SDW_HOST_WAKE_MASK,
                    ACP70_SDW_HOST_WAKE_MASK,
                );
            }

            snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP70_PME_EN, 1);
        }
        ACP7B_PCI_ID | ACP7F_PCI_ID => {
            snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP7X_ZSC_DSP_CTRL, 0);
            snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP7X_PME_EN, 1);
            snd_sof_dsp_write(
                sdev,
                ACP_DSP_BAR,
                ACP7X_DSP0_IDMA_ERROR_MASK,
                ACP7X_IDMA_ERROR_MASK,
            );
        }
        _ => {}
    }
    0
}

unsafe fn check_acp_sdw_enable_status(sdev: *mut snd_sof_dev) -> bool {
    let acp_data: *mut acp_dev_data;
    let sdw0_en: u32;
    let sdw1_en: u32;

    acp_data = (*(*sdev).pdata).hw_pdata as *mut acp_dev_data;
    if (*acp_data).sdw.is_null() {
        return false;
    }

    sdw0_en = snd_sof_dsp_read(sdev, ACP_DSP_BAR, ACP_SW0_EN);
    sdw1_en = snd_sof_dsp_read(sdev, ACP_DSP_BAR, ACP_SW1_EN);
    (*acp_data).sdw_en_stat = sdw0_en != 0 || sdw1_en != 0;
    (*acp_data).sdw_en_stat
}

#[no_mangle]
pub unsafe extern "C" fn amd_sof_acp_suspend(
    sdev: *mut snd_sof_dev,
    _target_state: u32,
) -> i32 {
    let acp_data: *mut acp_dev_data;
    let ret: i32;
    let mut enable: bool = false;

    acp_data = (*(*sdev).pdata).hw_pdata as *mut acp_dev_data;
    /* When acp_reset() function is invoked, it will apply ACP SOFT reset and
     * DSP reset. ACP Soft reset sequence will cause all ACP IP registers will
     * be reset to default values which will break the ClockStop Mode functionality.
     * Add a condition check to apply DSP reset when SoundWire ClockStop mode
     * is selected. For the rest of the scenarios, apply acp reset sequence.
     */
    if check_acp_sdw_enable_status(sdev) {
        return acp_dsp_reset(sdev);
    }

    ret = acp_reset(sdev);
    if ret != 0 {
        dev_err((*sdev).dev, b"ACP Reset failed\n\0".as_ptr() as *const i8);
        return ret;
    }
    match (*acp_data).pci_rev {
        ACP70_PCI_ID | ACP71_PCI_ID | ACP72_PCI_ID => {
            enable = true;
        }
        _ => {}
    }
    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_CONTROL, enable as u32);

    0
}
// EXPORT_SYMBOL_NS(amd_sof_acp_suspend, "SND_SOC_SOF_AMD_COMMON");

#[no_mangle]
pub unsafe extern "C" fn amd_sof_acp_resume(sdev: *mut snd_sof_dev) -> i32 {
    let mut ret: i32;
    let acp_data: *mut acp_dev_data;

    acp_data = (*(*sdev).pdata).hw_pdata as *mut acp_dev_data;
    if !(*acp_data).sdw_en_stat {
        ret = acp_init(sdev);
        if ret != 0 {
            dev_err((*sdev).dev, b"ACP Init failed\n\0".as_ptr() as *const i8);
            return ret;
        }
        return acp_memory_init(sdev);
    }
    match (*acp_data).pci_rev {
        ACP70_PCI_ID | ACP71_PCI_ID | ACP72_PCI_ID => {
            snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP70_PME_EN, 1);
        }
        _ => {}
    }

    acp_dsp_reset(sdev)
}
// EXPORT_SYMBOL_NS(amd_sof_acp_resume, "SND_SOC_SOF_AMD_COMMON");

// #if IS_ENABLED(CONFIG_SND_SOC_SOF_AMD_SOUNDWIRE)
unsafe fn acp_sof_scan_sdw_devices(sdev: *mut snd_sof_dev, addr: u64) -> i32 {
    let sdw_dev: *mut acpi_device;
    let acp_data: *mut acp_dev_data;
    let desc: *const sof_amd_acp_desc = get_chip_info((*(*sdev).pdata));

    if addr == 0 {
        return -ENODEV;
    }

    acp_data = (*(*sdev).pdata).hw_pdata as *mut acp_dev_data;
    sdw_dev = acpi_find_child_device(ACPI_COMPANION((*sdev).dev), addr, 0);
    if sdw_dev.is_null() {
        return -ENODEV;
    }

    (*acp_data).info.handle = (*sdw_dev).handle;
    (*acp_data).info.count = (*desc).sdw_max_link_count;

    amd_sdw_scan_controller(&mut (*acp_data).info)
}

unsafe fn amd_sof_sdw_probe(sdev: *mut snd_sof_dev) -> i32 {
    let acp_data: *mut acp_dev_data;
    let mut sdw_res: sdw_amd_res = core::mem::zeroed();
    let ret: i32;

    acp_data = (*(*sdev).pdata).hw_pdata as *mut acp_dev_data;

    sdw_res.addr = (*acp_data).addr;
    sdw_res.reg_range = (*acp_data).reg_range;
    sdw_res.handle = (*acp_data).info.handle;
    sdw_res.parent = (*sdev).dev;
    sdw_res.dev = (*sdev).dev;
    sdw_res.acp_lock = &mut (*acp_data).acp_lock;
    sdw_res.count = (*acp_data).info.count;
    sdw_res.link_mask = (*acp_data).info.link_mask;
    sdw_res.mmio_base = (*sdev).bar[ACP_DSP_BAR as usize];
    sdw_res.acp_rev = (*acp_data).pci_rev;

    ret = sdw_amd_probe(&mut sdw_res, &mut (*acp_data).sdw);
    if ret != 0 {
        dev_err((*sdev).dev, b"SoundWire probe failed\n\0".as_ptr() as *const i8);
    }
    ret
}

unsafe fn amd_sof_sdw_exit(sdev: *mut snd_sof_dev) -> i32 {
    let acp_data: *mut acp_dev_data;

    acp_data = (*(*sdev).pdata).hw_pdata as *mut acp_dev_data;
    if !(*acp_data).sdw.is_null() {
        sdw_amd_exit((*acp_data).sdw);
    }
    (*acp_data).sdw = ptr::null_mut();

    0
}

// #else
// static int acp_sof_scan_sdw_devices(struct snd_sof_dev *sdev, u64 addr) { return 0; }
// static int amd_sof_sdw_probe(struct snd_sof_dev *sdev) { return 0; }
// static int amd_sof_sdw_exit(struct snd_sof_dev *sdev) { return 0; }
// #endif

#[no_mangle]
pub unsafe extern "C" fn amd_sof_acp_probe(sdev: *mut snd_sof_dev) -> i32 {
    let pci: *mut pci_dev = to_pci_dev((*sdev).dev);
    let adata: *mut acp_dev_data;
    let chip: *const sof_amd_acp_desc;
    let mut dmi_id: *const dmi_system_id;
    let addr: u32;
    let mut ret: i32;

    chip = get_chip_info((*(*sdev).pdata));
    if chip.is_null() {
        dev_err(
            (*sdev).dev,
            b"no such device supported, chip id:%x\n\0".as_ptr() as *const i8,
            (*pci).device,
        );
        return -EIO;
    }
    adata = devm_kzalloc(
        (*sdev).dev,
        size_of::<acp_dev_data>(),
        GFP_KERNEL,
    ) as *mut acp_dev_data;
    if adata.is_null() {
        return -ENOMEM;
    }

    (*adata).dev = sdev;
    (*adata).dmic_dev = platform_device_register_data(
        (*sdev).dev,
        b"dmic-codec\0".as_ptr() as *const i8,
        PLATFORM_DEVID_NONE,
        ptr::null(),
        0,
    );
    if IS_ERR((*adata).dmic_dev as *const c_void) {
        dev_err(
            (*sdev).dev,
            b"failed to register platform for dmic codec\n\0".as_ptr() as *const i8,
        );
        return PTR_ERR((*adata).dmic_dev as *const c_void);
    }
    addr = pci_resource_start(pci, ACP_DSP_BAR) as u32;
    (*sdev).bar[ACP_DSP_BAR as usize] =
        devm_ioremap((*sdev).dev, addr, pci_resource_len(pci, ACP_DSP_BAR));
    if (*sdev).bar[ACP_DSP_BAR as usize].is_null() {
        dev_err((*sdev).dev, b"ioremap error\n\0".as_ptr() as *const i8);
        ret = -ENXIO;
        platform_device_unregister((*adata).dmic_dev);
        return ret;
    }

    pci_set_master(pci);
    (*adata).addr = addr;
    (*adata).reg_range = (*chip).reg_end_addr - (*chip).reg_start_addr;
    (*adata).pci_rev = (*pci).revision;
    mutex_init(&mut (*adata).acp_lock);
    (*(*sdev).pdata).hw_pdata = adata as *mut c_void;

    ret = acp_init(sdev);
    if ret < 0 {
        platform_device_unregister((*adata).dmic_dev);
        return ret;
    }

    (*sdev).ipc_irq = (*pci).irq;
    ret = request_threaded_irq(
        (*sdev).ipc_irq,
        Some(acp_irq_handler),
        Some(acp_irq_thread),
        IRQF_SHARED,
        b"AudioDSP\0".as_ptr() as *const i8,
        sdev as *mut c_void,
    );
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            b"failed to register IRQ %d\n\0".as_ptr() as *const i8,
            (*sdev).ipc_irq,
        );
        platform_device_unregister((*adata).dmic_dev);
        return ret;
    }

    /* scan SoundWire capabilities exposed by DSDT */
    ret = acp_sof_scan_sdw_devices(sdev, (*chip).sdw_acpi_dev_addr);
    if ret < 0 {
        dev_dbg(
            (*sdev).dev,
            b"skipping SoundWire, not detected with ACPI scan\n\0".as_ptr() as *const i8,
        );
    } else {
        ret = amd_sof_sdw_probe(sdev);
        if ret < 0 {
            dev_err(
                (*sdev).dev,
                b"error: SoundWire probe error\n\0".as_ptr() as *const i8,
            );
            free_irq((*sdev).ipc_irq, sdev as *mut c_void);
            return ret;
        }
    }

    (*sdev).dsp_box.offset = 0;
    (*sdev).dsp_box.size = BOX_SIZE_512;

    (*sdev).host_box.offset = (*sdev).dsp_box.offset + (*sdev).dsp_box.size;
    (*sdev).host_box.size = BOX_SIZE_512;

    (*sdev).debug_box.offset = (*sdev).host_box.offset + (*sdev).host_box.size;
    (*sdev).debug_box.size = BOX_SIZE_1024;

    dmi_id = dmi_first_match(acp_sof_quirk_table.as_ptr());
    if !dmi_id.is_null() {
        (*adata).quirks = (*dmi_id).driver_data as *mut acp_quirk_entry;

        if (*(*adata).quirks).signed_fw_image {
            (*adata).fw_code_bin = devm_kasprintf(
                (*sdev).dev,
                GFP_KERNEL,
                b"sof-%s-code.bin\0".as_ptr() as *const i8,
                (*chip).name,
            );
            if (*adata).fw_code_bin.is_null() {
                ret = -ENOMEM;
                free_irq((*sdev).ipc_irq, sdev as *mut c_void);
                platform_device_unregister((*adata).dmic_dev);
                return ret;
            }

            (*adata).fw_data_bin = devm_kasprintf(
                (*sdev).dev,
                GFP_KERNEL,
                b"sof-%s-data.bin\0".as_ptr() as *const i8,
                (*chip).name,
            );
            if (*adata).fw_data_bin.is_null() {
                ret = -ENOMEM;
                free_irq((*sdev).ipc_irq, sdev as *mut c_void);
                platform_device_unregister((*adata).dmic_dev);
                return ret;
            }
        }
    }

    (*adata).enable_fw_debug = enable_fw_debug;
    acp_memory_init(sdev);

    acp_dsp_stream_init(sdev);

    0
}
// EXPORT_SYMBOL_NS(amd_sof_acp_probe, "SND_SOC_SOF_AMD_COMMON");

#[no_mangle]
pub unsafe extern "C" fn amd_sof_acp_remove(sdev: *mut snd_sof_dev) {
    let adata: *mut acp_dev_data = (*(*sdev).pdata).hw_pdata as *mut acp_dev_data;

    if !(*adata).sdw.is_null() {
        amd_sof_sdw_exit(sdev);
    }

    if (*sdev).ipc_irq != 0 {
        free_irq((*sdev).ipc_irq, sdev as *mut c_void);
    }

    if !(*adata).dmic_dev.is_null() {
        platform_device_unregister((*adata).dmic_dev);
    }

    acp_reset(sdev);
}
// EXPORT_SYMBOL_NS(amd_sof_acp_remove, "SND_SOC_SOF_AMD_COMMON");

#[no_mangle]
pub unsafe extern "C" fn amd_sof_acp7x_probe(sdev: *mut snd_sof_dev) -> i32 {
    let pci: *mut pci_dev = to_pci_dev((*sdev).dev);
    let adata: *mut acp_dev_data;
    let chip: *const sof_amd_acp_desc;
    let obj: *const acpi_object;
    let adev: *mut acpi_device;
    let addr: u32;
    let irqflags: u32;
    let mut ret: i32;

    chip = get_chip_info((*(*sdev).pdata));
    if chip.is_null() {
        dev_err(
            (*sdev).dev,
            b"no such device supported, chip id:%x\n\0".as_ptr() as *const i8,
            (*pci).device,
        );
        return -EIO;
    }
    adata = devm_kzalloc((*sdev).dev, size_of::<acp_dev_data>(), GFP_KERNEL) as *mut acp_dev_data;
    if adata.is_null() {
        return -ENOMEM;
    }

    (*adata).dev = sdev;
    (*adata).dmic_dev = platform_device_register_data(
        (*sdev).dev,
        b"dmic-codec\0".as_ptr() as *const i8,
        PLATFORM_DEVID_NONE,
        ptr::null(),
        0,
    );
    if IS_ERR((*adata).dmic_dev as *const c_void) {
        dev_err(
            (*sdev).dev,
            b"failed to register platform for dmic codec\n\0".as_ptr() as *const i8,
        );
        return PTR_ERR((*adata).dmic_dev as *const c_void);
    }

    addr = pci_resource_start(pci, ACP_DSP_BAR) as u32;
    (*sdev).bar[ACP_DSP_BAR as usize] =
        devm_ioremap((*sdev).dev, addr, pci_resource_len(pci, ACP_DSP_BAR));
    if (*sdev).bar[ACP_DSP_BAR as usize].is_null() {
        dev_err((*sdev).dev, b"ioremap error\n\0".as_ptr() as *const i8);
        ret = -ENXIO;
        platform_device_unregister((*adata).dmic_dev);
        return ret;
    }

    pci_set_master(pci);
    (*adata).addr = addr;
    (*adata).reg_range = (*chip).reg_end_addr - (*chip).reg_start_addr;
    (*adata).pci_rev = (*pci).revision;
    mutex_init(&mut (*adata).acp_lock);
    (*(*sdev).pdata).hw_pdata = adata as *mut c_void;

    ret = acp_init(sdev);
    if ret < 0 {
        platform_device_unregister((*adata).dmic_dev);
        return ret;
    }

    adev = ACPI_COMPANION(&mut (*pci).dev);

    (*sdev).ipc_irq = (*pci).irq;
    irqflags = IRQF_SHARED;

    ret = request_threaded_irq(
        (*pci).irq,
        Some(acp7x_irq_handler),
        Some(acp_irq_thread),
        irqflags,
        b"AudioDSP\0".as_ptr() as *const i8,
        sdev as *mut c_void,
    );
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            b"failed to register IRQ %d\n\0".as_ptr() as *const i8,
            (*sdev).ipc_irq,
        );
        platform_device_unregister((*adata).dmic_dev);
        return ret;
    }

    if !adev.is_null() {
        if acpi_dev_get_property(
            adev,
            b"acp-sof-signed-firmware-image\0".as_ptr() as *const i8,
            ACPI_TYPE_INTEGER,
            &obj,
        ) == 0
        {
            (*adata).acp_sof_signed_firmware_image = (*obj).integer.value != 0;
        }
    }

    (*sdev).dsp_box.offset = 0;
    (*sdev).dsp_box.size = BOX_SIZE_512;

    (*sdev).host_box.offset = (*sdev).dsp_box.offset + (*sdev).dsp_box.size;
    (*sdev).host_box.size = BOX_SIZE_512;

    (*sdev).debug_box.offset = (*sdev).host_box.offset + (*sdev).host_box.size;
    (*sdev).debug_box.size = BOX_SIZE_1024;

    if (*adata).acp_sof_signed_firmware_image {
        (*adata).fw_code_bin = devm_kasprintf(
            (*sdev).dev,
            GFP_KERNEL,
            b"sof-%s-code.bin\0".as_ptr() as *const i8,
            (*chip).name,
        );
        if (*adata).fw_code_bin.is_null() {
            ret = -ENOMEM;
            free_irq((*sdev).ipc_irq, sdev as *mut c_void);
            platform_device_unregister((*adata).dmic_dev);
            return ret;
        }
        (*adata).fw_data_bin = devm_kasprintf(
            (*sdev).dev,
            GFP_KERNEL,
            b"sof-%s-data.bin\0".as_ptr() as *const i8,
            (*chip).name,
        );
        if (*adata).fw_data_bin.is_null() {
            ret = -ENOMEM;
            free_irq((*sdev).ipc_irq, sdev as *mut c_void);
            platform_device_unregister((*adata).dmic_dev);
            return ret;
        }
    }

    (*adata).enable_fw_debug = enable_fw_debug;
    acp_memory_init(sdev);
    acp_dsp_stream_init(sdev);

    0
}
// EXPORT_SYMBOL_NS(amd_sof_acp7x_probe, "SND_SOC_SOF_AMD_COMMON");

#[no_mangle]
pub unsafe extern "C" fn amd_sof_acp7x_remove(sdev: *mut snd_sof_dev) {
    let adata: *mut acp_dev_data = (*(*sdev).pdata).hw_pdata as *mut acp_dev_data;

    if (*sdev).ipc_irq != 0 {
        free_irq((*sdev).ipc_irq, sdev as *mut c_void);
    }

    if !(*adata).dmic_dev.is_null() {
        platform_device_unregister((*adata).dmic_dev);
    }

    acp_reset(sdev);
}
// EXPORT_SYMBOL_NS(amd_sof_acp7x_remove, "SND_SOC_SOF_AMD_COMMON");

#[no_mangle]
pub unsafe extern "C" fn amd_sof_acp7x_suspend(
    sdev: *mut snd_sof_dev,
    _target_state: u32,
) -> i32 {
    let acp_data: *mut acp_dev_data;
    let ret: i32;
    let mut enable: bool = false;

    acp_data = (*(*sdev).pdata).hw_pdata as *mut acp_dev_data;

    ret = acp_reset(sdev);
    if ret != 0 {
        dev_err((*sdev).dev, b"ACP Reset failed\n\0".as_ptr() as *const i8);
        return ret;
    }
    match (*acp_data).pci_rev {
        ACP7B_PCI_ID | ACP7F_PCI_ID => {
            enable = true;
        }
        _ => {}
    }
    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP_CONTROL, enable as u32);
    snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP7X_ZSC_DSP_CTRL, 1);

    0
}
// EXPORT_SYMBOL_NS(amd_sof_acp7x_suspend, "SND_SOC_SOF_AMD_COMMON");

#[no_mangle]
pub unsafe extern "C" fn amd_sof_acp7x_resume(sdev: *mut snd_sof_dev) -> i32 {
    let acp_data: *mut acp_dev_data;
    let mut ret: i32;

    acp_data = (*(*sdev).pdata).hw_pdata as *mut acp_dev_data;

    ret = acp_init(sdev);
    if ret != 0 {
        dev_err((*sdev).dev, b"ACP Init failed\n\0".as_ptr() as *const i8);
        return ret;
    }
    ret = acp_memory_init(sdev);
    if ret != 0 {
        dev_err((*sdev).dev, b"ACP Memory init failed\n\0".as_ptr() as *const i8);
        return ret;
    }

    match (*acp_data).pci_rev {
        ACP7B_PCI_ID | ACP7F_PCI_ID => {
            snd_sof_dsp_write(sdev, ACP_DSP_BAR, ACP7X_PME_EN, 1);
        }
        _ => {}
    }

    0
}
// EXPORT_SYMBOL_NS(amd_sof_acp7x_resume, "SND_SOC_SOF_AMD_COMMON");

#[no_mangle]
pub unsafe extern "C" fn amd_sof_acp7x_suspend_runtime(sdev: *mut snd_sof_dev) -> i32 {
    amd_sof_acp7x_suspend(sdev, 0)
}
// EXPORT_SYMBOL_NS(amd_sof_acp7x_suspend_runtime, "SND_SOC_SOF_AMD_COMMON");

#[no_mangle]
pub unsafe extern "C" fn amd_sof_acp7x_resume_runtime(sdev: *mut snd_sof_dev) -> i32 {
    amd_sof_acp7x_resume(sdev)
}
// EXPORT_SYMBOL_NS(amd_sof_acp7x_resume_runtime, "SND_SOC_SOF_AMD_COMMON");

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("AMD ACP sof driver");
// MODULE_IMPORT_NS("SOUNDWIRE_AMD_INIT");
// MODULE_IMPORT_NS("SND_AMD_SOUNDWIRE_ACPI");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
