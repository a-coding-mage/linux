/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license. When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2021 Advanced Micro Devices, Inc. All rights reserved.
 *
 * Author: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
 */

/* C header dependencies:
 * <sound/pcm.h>, <sound/soc.h>, <sound/soc-acpi.h>, <sound/soc-dai.h>,
 * "acp_common.h", and "chip_offset_byte.h".
 */

use core::ffi::{c_char, c_int, c_void};

pub const DMIC_INSTANCE: c_int = 0x00;
pub const I2S_SP_INSTANCE: c_int = 0x01;
pub const I2S_BT_INSTANCE: c_int = 0x02;
pub const I2S_HS_INSTANCE: c_int = 0x03;

pub const MEM_WINDOW_START: u32 = 0x4080000;

pub const ACP_I2S_REG_START: u32 = 0x1242400;
pub const ACP_I2S_REG_END: u32 = 0x1242810;
pub const ACP3X_I2STDM_REG_START: u32 = 0x1242400;
pub const ACP3X_I2STDM_REG_END: u32 = 0x1242410;
pub const ACP3X_BT_TDM_REG_START: u32 = 0x1242800;
pub const ACP3X_BT_TDM_REG_END: u32 = 0x1242810;

pub const fn THRESHOLD(bit: u32, base: u32) -> u32 {
    bit + base
}
pub const fn I2S_RX_THRESHOLD(base: u32) -> u32 {
    THRESHOLD(7, base)
}
pub const fn I2S_TX_THRESHOLD(base: u32) -> u32 {
    THRESHOLD(8, base)
}
pub const fn BT_TX_THRESHOLD(base: u32) -> u32 {
    THRESHOLD(6, base)
}
pub const fn BT_RX_THRESHOLD(base: u32) -> u32 {
    THRESHOLD(5, base)
}
pub const fn HS_TX_THRESHOLD(base: u32) -> u32 {
    THRESHOLD(4, base)
}
pub const fn HS_RX_THRESHOLD(base: u32) -> u32 {
    THRESHOLD(3, base)
}

pub const ACP_SRAM_SP_PB_PTE_OFFSET: u32 = 0x0;
pub const ACP_SRAM_SP_CP_PTE_OFFSET: u32 = 0x100;
pub const ACP_SRAM_BT_PB_PTE_OFFSET: u32 = 0x200;
pub const ACP_SRAM_BT_CP_PTE_OFFSET: u32 = 0x300;
pub const ACP_SRAM_PDM_PTE_OFFSET: u32 = 0x400;
pub const ACP_SRAM_HS_PB_PTE_OFFSET: u32 = 0x500;
pub const ACP_SRAM_HS_CP_PTE_OFFSET: u32 = 0x600;
pub const PAGE_SIZE_4K_ENABLE: u32 = 0x2;

pub const I2S_SP_TX_MEM_WINDOW_START: u32 = 0x4000000;
pub const I2S_SP_RX_MEM_WINDOW_START: u32 = 0x4020000;
pub const I2S_BT_TX_MEM_WINDOW_START: u32 = 0x4040000;
pub const I2S_BT_RX_MEM_WINDOW_START: u32 = 0x4060000;
pub const I2S_HS_TX_MEM_WINDOW_START: u32 = 0x40A0000;
pub const I2S_HS_RX_MEM_WINDOW_START: u32 = 0x40C0000;

pub const ACP7X_I2S_SP_TX_MEM_WINDOW_START: u32 = 0x4000000;
pub const ACP7X_I2S_SP_RX_MEM_WINDOW_START: u32 = 0x4200000;
pub const ACP7X_I2S_BT_TX_MEM_WINDOW_START: u32 = 0x4400000;
pub const ACP7X_I2S_BT_RX_MEM_WINDOW_START: u32 = 0x4600000;
pub const ACP7X_I2S_HS_TX_MEM_WINDOW_START: u32 = 0x4800000;
pub const ACP7X_I2S_HS_RX_MEM_WINDOW_START: u32 = 0x4A00000;
pub const ACP7X_DMIC_MEM_WINDOW_START: u32 = 0x4C00000;

pub const SP_PB_FIFO_ADDR_OFFSET: u32 = 0x500;
pub const SP_CAPT_FIFO_ADDR_OFFSET: u32 = 0x700;
pub const BT_PB_FIFO_ADDR_OFFSET: u32 = 0x900;
pub const BT_CAPT_FIFO_ADDR_OFFSET: u32 = 0xB00;
pub const HS_PB_FIFO_ADDR_OFFSET: u32 = 0xD00;
pub const HS_CAPT_FIFO_ADDR_OFFSET: u32 = 0xF00;
pub const PLAYBACK_MIN_NUM_PERIODS: u32 = 2;
pub const PLAYBACK_MAX_NUM_PERIODS: u32 = 8;
pub const PLAYBACK_MAX_PERIOD_SIZE: u32 = 8192;
pub const PLAYBACK_MIN_PERIOD_SIZE: u32 = 1024;
pub const CAPTURE_MIN_NUM_PERIODS: u32 = 2;
pub const CAPTURE_MAX_NUM_PERIODS: u32 = 8;
pub const CAPTURE_MAX_PERIOD_SIZE: u32 = 8192;
pub const CAPTURE_MIN_PERIOD_SIZE: u32 = 1024;

pub const MAX_BUFFER: u32 = 65536;
pub const MIN_BUFFER: u32 = MAX_BUFFER;
pub const FIFO_SIZE: u32 = 0x100;
pub const DMA_SIZE: u32 = 0x40;
pub const FRM_LEN: u32 = 0x100;

pub const ACP3X_ITER_IRER_SAMP_LEN_MASK: u32 = 0x38;

pub const ACP_MAX_STREAM: u32 = 8;

pub const TDM_ENABLE: u32 = 1;
pub const TDM_DISABLE: u32 = 0;

pub const SLOT_WIDTH_8: u32 = 0x8;
pub const SLOT_WIDTH_16: u32 = 0x10;
pub const SLOT_WIDTH_24: u32 = 0x18;
pub const SLOT_WIDTH_32: u32 = 0x20;

pub const ACP6X_PGFSM_CONTROL: u32 = 0x1024;
pub const ACP6X_PGFSM_STATUS: u32 = 0x1028;

pub const ACP63_PGFSM_CONTROL: u32 = ACP6X_PGFSM_CONTROL;
pub const ACP63_PGFSM_STATUS: u32 = ACP6X_PGFSM_STATUS;

pub const ACP70_PGFSM_CONTROL: u32 = ACP6X_PGFSM_CONTROL;
pub const ACP70_PGFSM_STATUS: u32 = ACP6X_PGFSM_STATUS;

pub const ACP_ZSC_DSP_CTRL: u32 = 0x0001014;
pub const ACP_ZSC_STS: u32 = 0x0001018;
pub const ACP_SOFT_RST_DONE_MASK: u32 = 0x00010001;

pub const ACP_PGFSM_CNTL_POWER_ON_MASK: u32 = 0xffffffff;
pub const ACP_PGFSM_CNTL_POWER_OFF_MASK: u32 = 0x00;
pub const ACP_PGFSM_STATUS_MASK: u32 = 0x03;
pub const ACP_POWERED_ON: u32 = 0x00;
pub const ACP_POWER_ON_IN_PROGRESS: u32 = 0x01;
pub const ACP_POWERED_OFF: u32 = 0x02;
pub const ACP_POWER_OFF_IN_PROGRESS: u32 = 0x03;

pub const ACP_ERROR_MASK: u32 = 0x20000000;
pub const ACP_EXT_INTR_STAT_CLEAR_MASK: u32 = 0xffffffff;

pub const ACP_TIMEOUT: u32 = 500;
pub const DELAY_US: u32 = 5;
pub const ACP_SUSPEND_DELAY_MS: u32 = 2000;

pub const PDM_DMA_STAT: u32 = 0x10;
pub const PDM_DMA_INTR_MASK: u32 = 0x10000;
pub const PDM_DEC_64: u32 = 0x2;
pub const PDM_CLK_FREQ_MASK: u32 = 0x07;
pub const PDM_MISC_CTRL_MASK: u32 = 0x18;
pub const PDM_ENABLE: u32 = 0x01;
pub const PDM_DISABLE: u32 = 0x00;
pub const DMA_EN_MASK: u32 = 0x02;
/* DELAY_US is defined a second time in the C header with the same value. */
pub const PDM_TIMEOUT: u32 = 1000;
pub const ACP_REGION2_OFFSET: u32 = 0x02000000;

#[repr(C)]
pub struct acp_chip_info {
    pub name: *mut c_char, /* Platform name */
    pub res: *mut resource,
    pub dev: *mut device,
    pub dai_driver: *mut snd_soc_dai_driver,

    pub acp_rev: u32,      /* ACP Revision id */
    pub base: *mut c_void, /* ACP memory PCI base */
    pub acp_hw_ops: *mut snd_acp_hw_ops,
    pub acp_hw_ops_init: Option<unsafe extern "C" fn(chip: *mut acp_chip_info) -> c_int>,
    pub chip_pdev: *mut platform_device,
    pub rsrc: *mut acp_resource, /* Platform specific resources*/
    pub stream_list: list_head,
    pub acp_lock: spinlock_t, /* Used to protect stream_list */
    pub dmic_codec_dev: *mut platform_device,
    pub acp_plat_dev: *mut platform_device,
    pub mach_dev: *mut platform_device,
    pub machines: *mut snd_soc_acpi_mach,
    pub num_dai: c_int,
    pub addr: u32,
    pub bclk_div: u32,
    pub lrclk_div: u32,
    pub ch_mask: u32,
    pub tdm_tx_fmt: [u32; 3],
    pub tdm_rx_fmt: [u32; 3],
    pub xfer_tx_resolution: [u32; 3],
    pub xfer_rx_resolution: [u32; 3],
    pub flag: u32,        /* Distinguish b/w Legacy or Only PDM */
    pub is_pdm_dev: bool, /* flag set to true when ACP PDM controller exists */
    pub is_pdm_config: bool, /* flag set to true when PDM configuration is selected from BIOS */
    pub is_i2s_config: bool, /* flag set to true when I2S configuration is selected from BIOS */
    pub tdm_mode: bool,
}

#[repr(C)]
pub struct acp_stream {
    pub list: list_head,
    pub substream: *mut snd_pcm_substream,
    pub irq_bit: c_int,
    pub dai_id: c_int,
    pub id: c_int,
    pub dir: c_int,
    pub bytescount: u64,
    pub reg_offset: u32,
    pub pte_offset: u32,
    pub fifo_offset: u32,
}

#[repr(C)]
pub struct acp_resource {
    pub offset: c_int,
    pub no_of_ctrls: c_int,
    pub irqp_used: c_int,
    pub soc_mclk: bool,
    pub irq_reg_offset: u32,
    pub scratch_reg_offset: u64,
    pub sram_pte_offset: u64,
}

/**
 * struct snd_acp_hw_ops - ACP PCI driver platform specific ops
 * @acp_init: ACP initialization
 * @acp_deinit: ACP de-initialization
 * @irq: ACP irq handler
 * @en_interrupts: ACP enable interrupts
 * @dis_interrupts: ACP disable interrupts
 */
#[repr(C)]
pub struct snd_acp_hw_ops {
    /* ACP hardware initilizations */
    pub acp_init: Option<unsafe extern "C" fn(chip: *mut acp_chip_info) -> c_int>,
    pub acp_deinit: Option<unsafe extern "C" fn(chip: *mut acp_chip_info) -> c_int>,

    /* ACP Interrupts*/
    pub irq: Option<unsafe extern "C" fn(irq: c_int, data: *mut c_void) -> irqreturn_t>,
    pub en_interrupts: Option<unsafe extern "C" fn(chip: *mut acp_chip_info) -> c_int>,
    pub dis_interrupts: Option<unsafe extern "C" fn(chip: *mut acp_chip_info) -> c_int>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum acp_config {
    ACP_CONFIG_0 = 0,
    ACP_CONFIG_1,
    ACP_CONFIG_2,
    ACP_CONFIG_3,
    ACP_CONFIG_4,
    ACP_CONFIG_5,
    ACP_CONFIG_6,
    ACP_CONFIG_7,
    ACP_CONFIG_8,
    ACP_CONFIG_9,
    ACP_CONFIG_10,
    ACP_CONFIG_11,
    ACP_CONFIG_12,
    ACP_CONFIG_13,
    ACP_CONFIG_14,
    ACP_CONFIG_15,
    ACP_CONFIG_16,
    ACP_CONFIG_17,
    ACP_CONFIG_18,
    ACP_CONFIG_19,
    ACP_CONFIG_20,
}

unsafe extern "C" {
    pub static mut rn_rsrc: acp_resource;
    pub static mut rmb_rsrc: acp_resource;
    pub static mut acp63_rsrc: acp_resource;
    pub static mut acp70_rsrc: acp_resource;

    pub static mut snd_soc_acpi_amd_acp_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_amd_rmb_acp_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_amd_acp63_acp_machines: [snd_soc_acpi_mach; 0];
    pub static mut snd_soc_acpi_amd_acp70_acp_machines: [snd_soc_acpi_mach; 0];

    pub static asoc_acp_cpu_dai_ops: snd_soc_dai_ops;
    pub static acp_dmic_dai_ops: snd_soc_dai_ops;

    pub fn acp_platform_register(dev: *mut device) -> c_int;
    pub fn acp_platform_unregister(dev: *mut device) -> c_int;

    pub fn acp_machine_select(chip: *mut acp_chip_info) -> c_int;

    pub fn acp_init(chip: *mut acp_chip_info) -> c_int;
    pub fn acp_deinit(chip: *mut acp_chip_info) -> c_int;
    pub fn acp_enable_interrupts(chip: *mut acp_chip_info) -> c_int;
    pub fn acp_disable_interrupts(chip: *mut acp_chip_info) -> c_int;
    pub fn acp_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t;

    pub static mut acp31_common_hw_ops: snd_acp_hw_ops;
    pub static mut acp6x_common_hw_ops: snd_acp_hw_ops;
    pub static mut acp63_common_hw_ops: snd_acp_hw_ops;
    pub static mut acp70_common_hw_ops: snd_acp_hw_ops;
    pub fn acp31_hw_ops_init(chip: *mut acp_chip_info) -> c_int;
    pub fn acp6x_hw_ops_init(chip: *mut acp_chip_info) -> c_int;
    pub fn acp63_hw_ops_init(chip: *mut acp_chip_info) -> c_int;
    pub fn acp70_hw_ops_init(chip: *mut acp_chip_info) -> c_int;
    /* Machine configuration */
    pub fn snd_amd_acp_find_config(pci: *mut pci_dev) -> c_int;

    pub fn config_pte_for_stream(chip: *mut acp_chip_info, stream: *mut acp_stream);
    pub fn config_acp_dma(chip: *mut acp_chip_info, stream: *mut acp_stream, size: c_int);
    pub fn restore_acp_pdm_params(substream: *mut snd_pcm_substream, chip: *mut acp_chip_info);

    pub fn restore_acp_i2s_params(
        substream: *mut snd_pcm_substream,
        chip: *mut acp_chip_info,
        stream: *mut acp_stream,
    ) -> c_int;

    pub fn check_acp_config(pci: *mut pci_dev, chip: *mut acp_chip_info);

    pub static EOPNOTSUPP: c_int;
    pub static SNDRV_PCM_STREAM_PLAYBACK: c_int;

    pub fn readl(addr: *const c_void) -> u32;
    pub fn dev_err(dev: *mut device, fmt: *const c_char, ...);

    pub fn ACP_BT_TX_LINEARPOSITIONCNTR_HIGH(chip: *mut acp_chip_info) -> usize;
    pub fn ACP_BT_TX_LINEARPOSITIONCNTR_LOW(chip: *mut acp_chip_info) -> usize;
    pub fn ACP_I2S_TX_LINEARPOSITIONCNTR_HIGH(chip: *mut acp_chip_info) -> usize;
    pub fn ACP_I2S_TX_LINEARPOSITIONCNTR_LOW(chip: *mut acp_chip_info) -> usize;
    pub fn ACP_BT_RX_LINEARPOSITIONCNTR_HIGH(chip: *mut acp_chip_info) -> usize;
    pub fn ACP_BT_RX_LINEARPOSITIONCNTR_LOW(chip: *mut acp_chip_info) -> usize;
    pub fn ACP_I2S_RX_LINEARPOSITIONCNTR_HIGH(chip: *mut acp_chip_info) -> usize;
    pub fn ACP_I2S_RX_LINEARPOSITIONCNTR_LOW(chip: *mut acp_chip_info) -> usize;

    pub static ACP_HS_TX_LINEARPOSITIONCNTR_HIGH: usize;
    pub static ACP_HS_TX_LINEARPOSITIONCNTR_LOW: usize;
    pub static ACP_HS_RX_LINEARPOSITIONCNTR_HIGH: usize;
    pub static ACP_HS_RX_LINEARPOSITIONCNTR_LOW: usize;
    pub static ACP_WOV_RX_LINEARPOSITIONCNTR_HIGH: usize;
    pub static ACP_WOV_RX_LINEARPOSITIONCNTR_LOW: usize;
}

#[inline]
unsafe fn acp_reg_ptr(chip: *mut acp_chip_info, offset: usize) -> *const c_void {
    unsafe { ((*chip).base as *const u8).add(offset) as *const c_void }
}

#[inline]
pub unsafe fn acp_hw_init(chip: *mut acp_chip_info) -> c_int {
    if !chip.is_null()
        && !unsafe { (*chip).acp_hw_ops }.is_null()
        && unsafe { (*(*chip).acp_hw_ops).acp_init }.is_some()
    {
        return unsafe { ((*(*chip).acp_hw_ops).acp_init.unwrap())(chip) };
    }
    -unsafe { EOPNOTSUPP }
}

#[inline]
pub unsafe fn acp_hw_deinit(chip: *mut acp_chip_info) -> c_int {
    if !chip.is_null()
        && !unsafe { (*chip).acp_hw_ops }.is_null()
        && unsafe { (*(*chip).acp_hw_ops).acp_deinit }.is_some()
    {
        return unsafe { ((*(*chip).acp_hw_ops).acp_deinit.unwrap())(chip) };
    }
    -unsafe { EOPNOTSUPP }
}

#[inline]
pub unsafe fn acp_hw_en_interrupts(chip: *mut acp_chip_info) -> c_int {
    if !chip.is_null()
        && !unsafe { (*chip).acp_hw_ops }.is_null()
        && unsafe { (*(*chip).acp_hw_ops).en_interrupts }.is_some()
    {
        return unsafe { ((*(*chip).acp_hw_ops).en_interrupts.unwrap())(chip) };
    }
    -unsafe { EOPNOTSUPP }
}

#[inline]
pub unsafe fn acp_hw_dis_interrupts(chip: *mut acp_chip_info) -> c_int {
    if !chip.is_null()
        && !unsafe { (*chip).acp_hw_ops }.is_null()
        && unsafe { (*(*chip).acp_hw_ops).dis_interrupts }.is_some()
    {
        unsafe { ((*(*chip).acp_hw_ops).dis_interrupts.unwrap())(chip) };
    }
    -unsafe { EOPNOTSUPP }
}

#[inline]
pub unsafe fn acp_get_byte_count(
    chip: *mut acp_chip_info,
    dai_id: c_int,
    direction: c_int,
) -> u64 {
    let mut byte_count: u64 = 0;
    let mut low: u64 = 0;
    let mut high: u64 = 0;

    if direction == unsafe { SNDRV_PCM_STREAM_PLAYBACK } {
        match dai_id {
            I2S_BT_INSTANCE => {
                high = unsafe { readl(acp_reg_ptr(chip, ACP_BT_TX_LINEARPOSITIONCNTR_HIGH(chip))) }
                    as u64;
                low =
                    unsafe { readl(acp_reg_ptr(chip, ACP_BT_TX_LINEARPOSITIONCNTR_LOW(chip))) }
                        as u64;
            }
            I2S_SP_INSTANCE => {
                high = unsafe { readl(acp_reg_ptr(chip, ACP_I2S_TX_LINEARPOSITIONCNTR_HIGH(chip))) }
                    as u64;
                low =
                    unsafe { readl(acp_reg_ptr(chip, ACP_I2S_TX_LINEARPOSITIONCNTR_LOW(chip))) }
                        as u64;
            }
            I2S_HS_INSTANCE => {
                high = unsafe { readl(acp_reg_ptr(chip, ACP_HS_TX_LINEARPOSITIONCNTR_HIGH)) } as u64;
                low = unsafe { readl(acp_reg_ptr(chip, ACP_HS_TX_LINEARPOSITIONCNTR_LOW)) } as u64;
            }
            _ => {
                unsafe { dev_err((*chip).dev, c"Invalid dai id %x\n".as_ptr(), dai_id) };
                return byte_count;
            }
        }
    } else {
        match dai_id {
            I2S_BT_INSTANCE => {
                high = unsafe { readl(acp_reg_ptr(chip, ACP_BT_RX_LINEARPOSITIONCNTR_HIGH(chip))) }
                    as u64;
                low =
                    unsafe { readl(acp_reg_ptr(chip, ACP_BT_RX_LINEARPOSITIONCNTR_LOW(chip))) }
                        as u64;
            }
            I2S_SP_INSTANCE => {
                high = unsafe { readl(acp_reg_ptr(chip, ACP_I2S_RX_LINEARPOSITIONCNTR_HIGH(chip))) }
                    as u64;
                low =
                    unsafe { readl(acp_reg_ptr(chip, ACP_I2S_RX_LINEARPOSITIONCNTR_LOW(chip))) }
                        as u64;
            }
            I2S_HS_INSTANCE => {
                high = unsafe { readl(acp_reg_ptr(chip, ACP_HS_RX_LINEARPOSITIONCNTR_HIGH)) } as u64;
                low = unsafe { readl(acp_reg_ptr(chip, ACP_HS_RX_LINEARPOSITIONCNTR_LOW)) } as u64;
            }
            DMIC_INSTANCE => {
                high = unsafe { readl(acp_reg_ptr(chip, ACP_WOV_RX_LINEARPOSITIONCNTR_HIGH)) }
                    as u64;
                low =
                    unsafe { readl(acp_reg_ptr(chip, ACP_WOV_RX_LINEARPOSITIONCNTR_LOW)) } as u64;
            }
            _ => {
                unsafe { dev_err((*chip).dev, c"Invalid dai id %x\n".as_ptr(), dai_id) };
                return byte_count;
            }
        }
    }
    /* Get 64 bit value from two 32 bit registers */
    byte_count = (high << 32) | low;

    byte_count
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
