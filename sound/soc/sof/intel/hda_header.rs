/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2017 Intel Corporation
 *
 * Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type size_t = usize;
pub type __le32 = u32;
pub type snd_pcm_uframes_t = u64;
pub type irqreturn_t = c_uint;

/* Opaque external dependency types from included headers. */
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct snd_dma_buffer { _private: [u8; 0] }
#[repr(C)] pub struct hda_bus { _private: [u8; 0] }
#[repr(C)] pub struct hdac_bus { _private: [u8; 0] }
#[repr(C)] pub struct sof_intel_dsp_desc { _private: [u8; 0] }
#[repr(C)] pub struct hdac_ext_stream { _private: [u8; 0] }
#[repr(C)] pub struct sof_intel_stream { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct sdw_intel_acpi_info { _private: [u8; 0] }
#[repr(C)] pub struct sdw_intel_ctx { _private: [u8; 0] }
#[repr(C)] pub struct nhlt_acpi_table { _private: [u8; 0] }
#[repr(C)] pub struct snd_sof_ipc_msg { _private: [u8; 0] }
#[repr(C)] pub struct snd_sof_dev { _private: [u8; 0] }
#[repr(C)] pub struct sof_dsp_power_state { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_sof_platform_stream_params { _private: [u8; 0] }
#[repr(C)] pub struct hdac_stream { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component { _private: [u8; 0] }
#[repr(C)] pub struct snd_sof_pcm_stream { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct sof_ext_man_elem_header { _private: [u8; 0] }
#[repr(C)] pub struct sof_ipc_dma_trace_params_ext { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_driver { _private: [u8; 0] }
#[repr(C)] pub struct snd_sof_dsp_ops { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_acpi_mach { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct pci_device_id { _private: [u8; 0] }
#[repr(C)] pub struct snd_sof_dai { _private: [u8; 0] }
#[repr(C)] pub struct sof_ipc_dai_config { _private: [u8; 0] }
#[repr(C)] pub struct sdw_intel_ops { _private: [u8; 0] }
#[repr(C)] pub struct sof_ipc4_fw_library { _private: [u8; 0] }
#[repr(C)] pub struct hdac_ext_link { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { _private: [u8; 0] }
#[repr(C)] pub struct snd_sof_dai_config_data { _private: [u8; 0] }
#[repr(C)] pub struct snd_sof_widget { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }

pub const PAGE_SIZE: usize = 4096;
pub const SNDRV_PCM_STREAM_LAST: usize = 1;

pub const fn BIT(n: u32) -> u32 {
    1u32 << n
}

pub const fn GENMASK(h: u32, l: u32) -> u32 {
    if h == 31 {
        u32::MAX << l
    } else {
        ((1u32 << (h + 1)) - 1) & !((1u32 << l) - 1)
    }
}

/* PCI registers */
pub const PCI_TCSEL: u32 = 0x44;
pub const PCI_PGCTL: u32 = PCI_TCSEL;
pub const PCI_CGCTL: u32 = 0x48;

/* PCI_PGCTL bits */
pub const PCI_PGCTL_ADSPPGD: u32 = BIT(2);
pub const PCI_PGCTL_LSRMD_MASK: u32 = BIT(4);

/* PCI_CGCTL bits */
pub const PCI_CGCTL_MISCBDCGE_MASK: u32 = BIT(6);
pub const PCI_CGCTL_ADSPDCGE: u32 = BIT(1);

/* Legacy HDA registers and bits used - widths are variable */
pub const SOF_HDA_GCAP: u32 = 0x0;
pub const SOF_HDA_GCTL: u32 = 0x8;
/* accept unsol. response enable */
pub const SOF_HDA_GCTL_UNSOL: u32 = BIT(8);
pub const SOF_HDA_LLCH: u32 = 0x14;
pub const SOF_HDA_INTCTL: u32 = 0x20;
pub const SOF_HDA_INTSTS: u32 = 0x24;
pub const SOF_HDA_WAKESTS: u32 = 0x0E;
pub const SOF_HDA_WAKESTS_INT_MASK: u32 = (1 << 8) - 1;
pub const SOF_HDA_RIRBSTS: u32 = 0x5d;

/* SOF_HDA_GCTL register bist */
pub const SOF_HDA_GCTL_RESET: u32 = BIT(0);

/* SOF_HDA_INCTL regs */
pub const SOF_HDA_INT_GLOBAL_EN: u32 = BIT(31);
pub const SOF_HDA_INT_CTRL_EN: u32 = BIT(30);
pub const SOF_HDA_INT_ALL_STREAM: u32 = 0xff;

/* SOF_HDA_INTSTS regs */
pub const SOF_HDA_INTSTS_GIS: u32 = BIT(31);

pub const SOF_HDA_MAX_CAPS: u32 = 10;
pub const SOF_HDA_CAP_ID_OFF: u32 = 16;
pub const SOF_HDA_CAP_ID_MASK: u32 = GENMASK(SOF_HDA_CAP_ID_OFF + 11, SOF_HDA_CAP_ID_OFF);
pub const SOF_HDA_CAP_NEXT_MASK: u32 = 0xFFFF;

pub const SOF_HDA_GTS_CAP_ID: u32 = 0x1;
pub const SOF_HDA_ML_CAP_ID: u32 = 0x2;

pub const SOF_HDA_PP_CAP_ID: u32 = 0x3;
pub const SOF_HDA_REG_PP_PPCH: u32 = 0x10;
pub const SOF_HDA_REG_PP_PPCTL: u32 = 0x04;
pub const SOF_HDA_REG_PP_PPSTS: u32 = 0x08;
pub const SOF_HDA_PPCTL_PIE: u32 = BIT(31);
pub const SOF_HDA_PPCTL_GPROCEN: u32 = BIT(30);

/*Vendor Specific Registers*/
pub const SOF_HDA_VS_D0I3C: u32 = 0x104A;

/* D0I3C Register fields */
pub const SOF_HDA_VS_D0I3C_CIP: u32 = BIT(0); /* Command-In-Progress */
pub const SOF_HDA_VS_D0I3C_I3: u32 = BIT(2); /* D0i3 enable bit */

/* DPIB entry size: 8 Bytes = 2 DWords */
pub const SOF_HDA_DPIB_ENTRY_SIZE: u32 = 0x8;

pub const SOF_HDA_SPIB_CAP_ID: u32 = 0x4;
pub const SOF_HDA_DRSM_CAP_ID: u32 = 0x5;

pub const SOF_HDA_SPIB_BASE: u32 = 0x08;
pub const SOF_HDA_SPIB_INTERVAL: u32 = 0x08;
pub const SOF_HDA_SPIB_SPIB: u32 = 0x00;
pub const SOF_HDA_SPIB_MAXFIFO: u32 = 0x04;

pub const SOF_HDA_PPHC_BASE: u32 = 0x10;
pub const SOF_HDA_PPHC_INTERVAL: u32 = 0x10;

pub const SOF_HDA_PPLC_BASE: u32 = 0x10;
pub const SOF_HDA_PPLC_MULTI: u32 = 0x10;
pub const SOF_HDA_PPLC_INTERVAL: u32 = 0x10;

pub const SOF_HDA_DRSM_BASE: u32 = 0x08;
pub const SOF_HDA_DRSM_INTERVAL: u32 = 0x08;

/* Descriptor error interrupt */
pub const SOF_HDA_CL_DMA_SD_INT_DESC_ERR: u32 = 0x10;

/* FIFO error interrupt */
pub const SOF_HDA_CL_DMA_SD_INT_FIFO_ERR: u32 = 0x08;

/* Buffer completion interrupt */
pub const SOF_HDA_CL_DMA_SD_INT_COMPLETE: u32 = 0x04;

pub const SOF_HDA_CL_DMA_SD_INT_MASK: u32 =
    SOF_HDA_CL_DMA_SD_INT_DESC_ERR | SOF_HDA_CL_DMA_SD_INT_FIFO_ERR | SOF_HDA_CL_DMA_SD_INT_COMPLETE;
pub const SOF_HDA_SD_CTL_DMA_START: u32 = 0x02; /* Stream DMA start bit */

/* Intel HD Audio Code Loader DMA Registers */
pub const SOF_HDA_ADSP_LOADER_BASE: u32 = 0x80;
pub const SOF_HDA_ADSP_DPLBASE: u32 = 0x70;
pub const SOF_HDA_ADSP_DPUBASE: u32 = 0x74;
pub const SOF_HDA_ADSP_DPLBASE_ENABLE: u32 = 0x01;

/* Stream Registers */
pub const SOF_HDA_ADSP_REG_SD_CTL: u32 = 0x00;
pub const SOF_HDA_ADSP_REG_SD_STS: u32 = 0x03;
pub const SOF_HDA_ADSP_REG_SD_LPIB: u32 = 0x04;
pub const SOF_HDA_ADSP_REG_SD_CBL: u32 = 0x08;
pub const SOF_HDA_ADSP_REG_SD_LVI: u32 = 0x0C;
pub const SOF_HDA_ADSP_REG_SD_FIFOW: u32 = 0x0E;
pub const SOF_HDA_ADSP_REG_SD_FIFOSIZE: u32 = 0x10;
pub const SOF_HDA_ADSP_REG_SD_FORMAT: u32 = 0x12;
pub const SOF_HDA_ADSP_REG_SD_FIFOL: u32 = 0x14;
pub const SOF_HDA_ADSP_REG_SD_BDLPL: u32 = 0x18;
pub const SOF_HDA_ADSP_REG_SD_BDLPU: u32 = 0x1C;
pub const SOF_HDA_ADSP_SD_ENTRY_SIZE: u32 = 0x20;

/* SDxFIFOS FIFOS */
pub const SOF_HDA_SD_FIFOSIZE_FIFOS_MASK: u32 = GENMASK(15, 0);

/* CL: Software Position Based FIFO Capability Registers */
pub const SOF_DSP_REG_CL_SPBFIFO: u32 = SOF_HDA_ADSP_LOADER_BASE + 0x20;
pub const SOF_HDA_ADSP_REG_CL_SPBFIFO_SPBFCH: u32 = 0x0;
pub const SOF_HDA_ADSP_REG_CL_SPBFIFO_SPBFCCTL: u32 = 0x4;
pub const SOF_HDA_ADSP_REG_CL_SPBFIFO_SPIB: u32 = 0x8;
pub const SOF_HDA_ADSP_REG_CL_SPBFIFO_MAXFIFOS: u32 = 0xc;

/* Stream Number */
pub const SOF_HDA_CL_SD_CTL_STREAM_TAG_SHIFT: u32 = 20;
pub const SOF_HDA_CL_SD_CTL_STREAM_TAG_MASK: u32 =
    GENMASK(SOF_HDA_CL_SD_CTL_STREAM_TAG_SHIFT + 3, SOF_HDA_CL_SD_CTL_STREAM_TAG_SHIFT);

pub const HDA_DSP_HDA_BAR: u32 = 0;
pub const HDA_DSP_PP_BAR: u32 = 1;
pub const HDA_DSP_SPIB_BAR: u32 = 2;
pub const HDA_DSP_DRSM_BAR: u32 = 3;
pub const HDA_DSP_BAR: u32 = 4;

pub const fn SRAM_WINDOW_OFFSET(x: u32) -> u32 {
    0x80000 + x * 0x20000
}

pub const HDA_DSP_MBOX_OFFSET: u32 = SRAM_WINDOW_OFFSET(0);

pub const fn HDA_DSP_PANIC_OFFSET(x: u32) -> u32 {
    (x & 0xFFFFFF) + HDA_DSP_MBOX_OFFSET
}

/* SRAM window 0 FW "registers" */
pub const HDA_DSP_SRAM_REG_ROM_STATUS: u32 = HDA_DSP_MBOX_OFFSET + 0x0;
pub const HDA_DSP_SRAM_REG_ROM_ERROR: u32 = HDA_DSP_MBOX_OFFSET + 0x4;
/* FW and ROM share offset 4 */
pub const HDA_DSP_SRAM_REG_FW_STATUS: u32 = HDA_DSP_MBOX_OFFSET + 0x4;
pub const HDA_DSP_SRAM_REG_FW_TRACEP: u32 = HDA_DSP_MBOX_OFFSET + 0x8;
pub const HDA_DSP_SRAM_REG_FW_END: u32 = HDA_DSP_MBOX_OFFSET + 0xc;

pub const HDA_DSP_MBOX_UPLINK_OFFSET: u32 = 0x81000;

pub const HDA_DSP_STREAM_RESET_TIMEOUT: u32 = 300;
/*
 * Timeout in us, for setting the stream RUN bit, during
 * start/stop the stream. The timeout expires if new RUN bit
 * value cannot be read back within the specified time.
 */
pub const HDA_DSP_STREAM_RUN_TIMEOUT: u32 = 300;

pub const HDA_DSP_SPIB_ENABLE: u32 = 1;
pub const HDA_DSP_SPIB_DISABLE: u32 = 0;

pub const SOF_HDA_MAX_BUFFER_SIZE: usize = 32 * PAGE_SIZE;

pub const HDA_DSP_STACK_DUMP_SIZE: u32 = 32;

/* ROM/FW status register */
pub const FSR_STATE_MASK: u32 = GENMASK(23, 0);
pub const FSR_WAIT_STATE_MASK: u32 = GENMASK(27, 24);
pub const FSR_MODULE_MASK: u32 = GENMASK(30, 28);
pub const FSR_HALTED: u32 = BIT(31);
pub const fn FSR_TO_STATE_CODE(x: u32) -> u32 { x & FSR_STATE_MASK }
pub const fn FSR_TO_WAIT_STATE_CODE(x: u32) -> u32 { (x & FSR_WAIT_STATE_MASK) >> 24 }
pub const fn FSR_TO_MODULE_CODE(x: u32) -> u32 { (x & FSR_MODULE_MASK) >> 28 }

/* Wait states */
pub const FSR_WAIT_FOR_IPC_BUSY: u32 = 0x1;
pub const FSR_WAIT_FOR_IPC_DONE: u32 = 0x2;
pub const FSR_WAIT_FOR_CACHE_INVALIDATION: u32 = 0x3;
pub const FSR_WAIT_FOR_LP_SRAM_OFF: u32 = 0x4;
pub const FSR_WAIT_FOR_DMA_BUFFER_FULL: u32 = 0x5;
pub const FSR_WAIT_FOR_CSE_CSR: u32 = 0x6;

/* Module codes */
pub const FSR_MOD_ROM: u32 = 0x0;
pub const FSR_MOD_ROM_BYP: u32 = 0x1;
pub const FSR_MOD_BASE_FW: u32 = 0x2;
pub const FSR_MOD_LP_BOOT: u32 = 0x3;
pub const FSR_MOD_BRNGUP: u32 = 0x4;
pub const FSR_MOD_ROM_EXT: u32 = 0x5;

/* State codes (module dependent) */
/* Module independent states */
pub const FSR_STATE_INIT: u32 = 0x0;
pub const FSR_STATE_INIT_DONE: u32 = 0x1;
pub const FSR_STATE_FW_ENTERED: u32 = 0x5;

/* ROM states */
pub const FSR_STATE_ROM_INIT: u32 = FSR_STATE_INIT;
pub const FSR_STATE_ROM_INIT_DONE: u32 = FSR_STATE_INIT_DONE;
pub const FSR_STATE_ROM_CSE_MANIFEST_LOADED: u32 = 0x2;
pub const FSR_STATE_ROM_FW_MANIFEST_LOADED: u32 = 0x3;
pub const FSR_STATE_ROM_FW_FW_LOADED: u32 = 0x4;
pub const FSR_STATE_ROM_FW_ENTERED: u32 = FSR_STATE_FW_ENTERED;
pub const FSR_STATE_ROM_VERIFY_FEATURE_MASK: u32 = 0x6;
pub const FSR_STATE_ROM_GET_LOAD_OFFSET: u32 = 0x7;
pub const FSR_STATE_ROM_FETCH_ROM_EXT: u32 = 0x8;
pub const FSR_STATE_ROM_FETCH_ROM_EXT_DONE: u32 = 0x9;
pub const FSR_STATE_ROM_BASEFW_ENTERED: u32 = 0xf; /* SKL */

/* (ROM) CSE states */
pub const FSR_STATE_ROM_CSE_IMR_REQUEST: u32 = 0x10;
pub const FSR_STATE_ROM_CSE_IMR_GRANTED: u32 = 0x11;
pub const FSR_STATE_ROM_CSE_VALIDATE_IMAGE_REQUEST: u32 = 0x12;
pub const FSR_STATE_ROM_CSE_IMAGE_VALIDATED: u32 = 0x13;

pub const FSR_STATE_ROM_CSE_IPC_IFACE_INIT: u32 = 0x20;
pub const FSR_STATE_ROM_CSE_IPC_RESET_PHASE_1: u32 = 0x21;
pub const FSR_STATE_ROM_CSE_IPC_OPERATIONAL_ENTRY: u32 = 0x22;
pub const FSR_STATE_ROM_CSE_IPC_OPERATIONAL: u32 = 0x23;
pub const FSR_STATE_ROM_CSE_IPC_DOWN: u32 = 0x24;

/* BRINGUP (or BRNGUP) states */
pub const FSR_STATE_BRINGUP_INIT: u32 = FSR_STATE_INIT;
pub const FSR_STATE_BRINGUP_INIT_DONE: u32 = FSR_STATE_INIT_DONE;
pub const FSR_STATE_BRINGUP_HPSRAM_LOAD: u32 = 0x2;
pub const FSR_STATE_BRINGUP_UNPACK_START: u32 = 0X3;
pub const FSR_STATE_BRINGUP_IMR_RESTORE: u32 = 0x4;
pub const FSR_STATE_BRINGUP_FW_ENTERED: u32 = FSR_STATE_FW_ENTERED;

/* ROM  status/error values */
pub const HDA_DSP_ROM_CSE_ERROR: u32 = 40;
pub const HDA_DSP_ROM_CSE_WRONG_RESPONSE: u32 = 41;
pub const HDA_DSP_ROM_IMR_TO_SMALL: u32 = 42;
pub const HDA_DSP_ROM_BASE_FW_NOT_FOUND: u32 = 43;
pub const HDA_DSP_ROM_CSE_VALIDATION_FAILED: u32 = 44;
pub const HDA_DSP_ROM_IPC_FATAL_ERROR: u32 = 45;
pub const HDA_DSP_ROM_L2_CACHE_ERROR: u32 = 46;
pub const HDA_DSP_ROM_LOAD_OFFSET_TO_SMALL: u32 = 47;
pub const HDA_DSP_ROM_API_PTR_INVALID: u32 = 50;
pub const HDA_DSP_ROM_BASEFW_INCOMPAT: u32 = 51;
pub const HDA_DSP_ROM_UNHANDLED_INTERRUPT: u32 = 0xBEE00000;
pub const HDA_DSP_ROM_MEMORY_HOLE_ECC: u32 = 0xECC00000;
pub const HDA_DSP_ROM_KERNEL_EXCEPTION: u32 = 0xCAFE0000;
pub const HDA_DSP_ROM_USER_EXCEPTION: u32 = 0xBEEF0000;
pub const HDA_DSP_ROM_UNEXPECTED_RESET: u32 = 0xDECAF000;
pub const HDA_DSP_ROM_NULL_FW_ENTRY: u32 = 0x4c4c4e55;

pub const HDA_DSP_ROM_IPC_CONTROL: u32 = 0x01000000;
pub const HDA_DSP_ROM_IPC_PURGE_FW: u32 = 0x00004000;

/* various timeout values */
pub const HDA_DSP_PU_TIMEOUT: u32 = 50;
pub const HDA_DSP_PD_TIMEOUT: u32 = 50;
pub const HDA_DSP_RESET_TIMEOUT_US: u32 = 50000;
pub const HDA_DSP_BASEFW_TIMEOUT_US: u32 = 3000000;
pub const HDA_DSP_INIT_TIMEOUT_US: u32 = 500000;
pub const HDA_DSP_CTRL_RESET_TIMEOUT: u32 = 100;
pub const HDA_DSP_WAIT_TIMEOUT: u32 = 500; /* 500 msec */
pub const HDA_DSP_REG_POLL_INTERVAL_US: u32 = 500; /* 0.5 msec */
pub const HDA_DSP_REG_POLL_RETRY_COUNT: u32 = 50;

pub const HDA_DSP_ADSPIC_IPC: u32 = BIT(0);
pub const HDA_DSP_ADSPIS_IPC: u32 = BIT(0);

/* Intel HD Audio General DSP Registers */
pub const HDA_DSP_GEN_BASE: u32 = 0x0;
pub const HDA_DSP_REG_ADSPCS: u32 = HDA_DSP_GEN_BASE + 0x04;
pub const HDA_DSP_REG_ADSPIC: u32 = HDA_DSP_GEN_BASE + 0x08;
pub const HDA_DSP_REG_ADSPIS: u32 = HDA_DSP_GEN_BASE + 0x0C;
pub const HDA_DSP_REG_ADSPIC2: u32 = HDA_DSP_GEN_BASE + 0x10;
pub const HDA_DSP_REG_ADSPIS2: u32 = HDA_DSP_GEN_BASE + 0x14;

pub const HDA_DSP_REG_ADSPIC2_SNDW: u32 = BIT(5);
pub const HDA_DSP_REG_ADSPIS2_SNDW: u32 = BIT(5);

/* Intel HD Audio Inter-Processor Communication Registers */
pub const HDA_DSP_IPC_BASE: u32 = 0x40;
pub const HDA_DSP_REG_HIPCT: u32 = HDA_DSP_IPC_BASE + 0x00;
pub const HDA_DSP_REG_HIPCTE: u32 = HDA_DSP_IPC_BASE + 0x04;
pub const HDA_DSP_REG_HIPCI: u32 = HDA_DSP_IPC_BASE + 0x08;
pub const HDA_DSP_REG_HIPCIE: u32 = HDA_DSP_IPC_BASE + 0x0C;
pub const HDA_DSP_REG_HIPCCTL: u32 = HDA_DSP_IPC_BASE + 0x10;

/* Intel Vendor Specific Registers */
pub const HDA_VS_INTEL_EM2: u32 = 0x1030;
pub const HDA_VS_INTEL_EM2_L1SEN: u32 = BIT(13);
pub const HDA_VS_INTEL_LTRP: u32 = 0x1048;
pub const HDA_VS_INTEL_LTRP_GB_MASK: u32 = 0x3F;

/*  HIPCI */
pub const HDA_DSP_REG_HIPCI_BUSY: u32 = BIT(31);
pub const HDA_DSP_REG_HIPCI_MSG_MASK: u32 = 0x7FFFFFFF;

/* HIPCIE */
pub const HDA_DSP_REG_HIPCIE_DONE: u32 = BIT(30);
pub const HDA_DSP_REG_HIPCIE_MSG_MASK: u32 = 0x3FFFFFFF;

/* HIPCCTL */
pub const HDA_DSP_REG_HIPCCTL_DONE: u32 = BIT(1);
pub const HDA_DSP_REG_HIPCCTL_BUSY: u32 = BIT(0);

/* HIPCT */
pub const HDA_DSP_REG_HIPCT_BUSY: u32 = BIT(31);
pub const HDA_DSP_REG_HIPCT_MSG_MASK: u32 = 0x7FFFFFFF;

/* HIPCTE */
pub const HDA_DSP_REG_HIPCTE_MSG_MASK: u32 = 0x3FFFFFFF;

pub const HDA_DSP_ADSPIC_CL_DMA: u32 = BIT(1);
pub const HDA_DSP_ADSPIS_CL_DMA: u32 = BIT(1);

/* Delay before scheduling D0i3 entry */
pub const BXT_D0I3_DELAY: u32 = 5000;

pub const FW_CL_STREAM_NUMBER: u32 = 0x1;
pub const HDA_FW_BOOT_ATTEMPTS: u32 = 3;

/* ADSPCS - Audio DSP Control & Status */
pub const HDA_DSP_ADSPCS_CRST_SHIFT: u32 = 0;
pub const fn HDA_DSP_ADSPCS_CRST_MASK(cm: u32) -> u32 { cm << HDA_DSP_ADSPCS_CRST_SHIFT }
pub const HDA_DSP_ADSPCS_CSTALL_SHIFT: u32 = 8;
pub const fn HDA_DSP_ADSPCS_CSTALL_MASK(cm: u32) -> u32 { cm << HDA_DSP_ADSPCS_CSTALL_SHIFT }
pub const HDA_DSP_ADSPCS_SPA_SHIFT: u32 = 16;
pub const fn HDA_DSP_ADSPCS_SPA_MASK(cm: u32) -> u32 { cm << HDA_DSP_ADSPCS_SPA_SHIFT }
pub const HDA_DSP_ADSPCS_CPA_SHIFT: u32 = 24;
pub const fn HDA_DSP_ADSPCS_CPA_MASK(cm: u32) -> u32 { cm << HDA_DSP_ADSPCS_CPA_SHIFT }
pub const fn SOF_DSP_CORES_MASK(nc: u32) -> u32 { GENMASK(nc - 1, 0) }

/* Intel HD Audio Inter-Processor Communication Registers for Cannonlake*/
pub const CNL_DSP_IPC_BASE: u32 = 0xc0;
pub const CNL_DSP_REG_HIPCTDR: u32 = CNL_DSP_IPC_BASE + 0x00;
pub const CNL_DSP_REG_HIPCTDA: u32 = CNL_DSP_IPC_BASE + 0x04;
pub const CNL_DSP_REG_HIPCTDD: u32 = CNL_DSP_IPC_BASE + 0x08;
pub const CNL_DSP_REG_HIPCIDR: u32 = CNL_DSP_IPC_BASE + 0x10;
pub const CNL_DSP_REG_HIPCIDA: u32 = CNL_DSP_IPC_BASE + 0x14;
pub const CNL_DSP_REG_HIPCIDD: u32 = CNL_DSP_IPC_BASE + 0x18;
pub const CNL_DSP_REG_HIPCCTL: u32 = CNL_DSP_IPC_BASE + 0x28;

pub const CNL_DSP_REG_HIPCIDR_BUSY: u32 = BIT(31);
pub const CNL_DSP_REG_HIPCIDR_MSG_MASK: u32 = 0x7FFFFFFF;
pub const CNL_DSP_REG_HIPCIDA_DONE: u32 = BIT(31);
pub const CNL_DSP_REG_HIPCIDA_MSG_MASK: u32 = 0x7FFFFFFF;
pub const CNL_DSP_REG_HIPCCTL_DONE: u32 = BIT(1);
pub const CNL_DSP_REG_HIPCCTL_BUSY: u32 = BIT(0);
pub const CNL_DSP_REG_HIPCTDR_BUSY: u32 = BIT(31);
pub const CNL_DSP_REG_HIPCTDR_MSG_MASK: u32 = 0x7FFFFFFF;
pub const CNL_DSP_REG_HIPCTDA_DONE: u32 = BIT(31);
pub const CNL_DSP_REG_HIPCTDA_MSG_MASK: u32 = 0x7FFFFFFF;
pub const CNL_DSP_REG_HIPCTDD_MSG_MASK: u32 = 0x7FFFFFFF;

/* BDL */
pub const HDA_DSP_BDL_SIZE: usize = 4096;

#[repr(C, packed)]
pub struct sof_intel_dsp_bdl {
    pub addr_l: __le32,
    pub addr_h: __le32,
    pub size: __le32,
    pub ioc: __le32,
}

pub const HDA_DSP_MAX_BDL_ENTRIES: usize = HDA_DSP_BDL_SIZE / core::mem::size_of::<sof_intel_dsp_bdl>();

/* Number of DAIs */
pub const SOF_SKL_NUM_DAIS_NOCODEC: u32 = 9;

/* CONFIG_SND_SOC_SOF_HDA_AUDIO_CODEC: 16 when enabled, SOF_SKL_NUM_DAIS_NOCODEC otherwise. */
pub const SOF_SKL_NUM_DAIS: u32 = SOF_SKL_NUM_DAIS_NOCODEC;

/* Intel HD Audio SRAM Window 0*/
pub const HDA_DSP_SRAM_REG_ROM_STATUS_SKL: u32 = 0x8000;
pub const HDA_ADSP_SRAM0_BASE_SKL: u32 = 0x8000;

/* Firmware status window */
pub const HDA_ADSP_FW_STATUS_SKL: u32 = HDA_ADSP_SRAM0_BASE_SKL;
pub const HDA_ADSP_ERROR_CODE_SKL: u32 = HDA_ADSP_FW_STATUS_SKL + 0x4;

/* Host Device Memory Space */
pub const APL_SSP_BASE_OFFSET: u32 = 0x2000;
pub const CNL_SSP_BASE_OFFSET: u32 = 0x10000;

/* Host Device Memory Size of a Single SSP */
pub const SSP_DEV_MEM_SIZE: u32 = 0x1000;

/* SSP Count of the Platform */
pub const APL_SSP_COUNT: u32 = 6;
pub const CNL_SSP_COUNT: u32 = 3;
pub const ICL_SSP_COUNT: u32 = 6;
pub const TGL_SSP_COUNT: u32 = 3;
pub const MTL_SSP_COUNT: u32 = 3;

/* SSP Registers */
pub const SSP_SSC1_OFFSET: u32 = 0x4;
pub const SSP_SET_SCLK_CONSUMER: u32 = BIT(25);
pub const SSP_SET_SFRM_CONSUMER: u32 = BIT(24);
pub const SSP_SET_CBP_CFP: u32 = SSP_SET_SCLK_CONSUMER | SSP_SET_SFRM_CONSUMER;

pub const HDA_EXT_ADDR: u32 = 0;
pub const fn HDA_EXT_CODEC(x: u32) -> u32 { x & BIT(HDA_EXT_ADDR) }
pub const HDA_IDISP_ADDR: u32 = 2;
pub const fn HDA_IDISP_CODEC(x: u32) -> u32 { x & BIT(HDA_IDISP_ADDR) }

pub const SOF_HDA_PLAYBACK_STREAMS: u32 = 16;
pub const SOF_HDA_CAPTURE_STREAMS: u32 = 16;
pub const SOF_HDA_PLAYBACK: u32 = 0;
pub const SOF_HDA_CAPTURE: u32 = 1;

/* stream flags */
pub const SOF_HDA_STREAM_DMI_L1_COMPATIBLE: u32 = 1;

/*
 * Time in ms for opportunistic D0I3 entry delay.
 * This has been deliberately chosen to be long to avoid race conditions.
 * Could be optimized in future.
 */
pub const SOF_HDA_D0I3_WORK_DELAY_MS: u32 = 5000;

/* HDA DSP D0 substate */
#[repr(C)]
pub enum sof_hda_D0_substate {
    SOF_HDA_DSP_PM_D0I0,
    SOF_HDA_DSP_PM_D0I3,
}

#[repr(C)]
pub struct sof_ace3_mic_privacy {
    pub active: bool,
    pub work: work_struct,
}

/* represents DSP HDA controller frontend - i.e. host facing control */
#[repr(C)]
pub struct sof_intel_hda_dev {
    pub imrboot_supported: bool,
    pub skip_imr_boot: bool,
    pub booted_from_imr: bool,
    pub boot_iteration: c_int,
    pub cl_dmab: snd_dma_buffer,
    pub cl_dmab_contains_basefw: bool,
    pub iccmax_dmab: snd_dma_buffer,
    pub hbus: hda_bus,
    pub desc: *const sof_intel_dsp_desc,
    pub dtrace_stream: *mut hdac_ext_stream,
    pub no_ipc_position: u32,
    pub stream_max: u32,
    pub link_dma_active_sdw_mask: [u32; SNDRV_PCM_STREAM_LAST + 1],
    pub link_dma_active_multi_mask: [u32; SNDRV_PCM_STREAM_LAST + 1],
    pub link_dma_out_hda_used_mask: u32,
    pub l1_disabled: bool,
    pub dmic_dev: *mut platform_device,
    pub d0i3_work: delayed_work,
    pub info: sdw_intel_acpi_info,
    pub sdw: *mut sdw_intel_ctx,
    pub clk_config_lpro: bool,
    pub waitq: wait_queue_head_t,
    pub code_loading: bool,
    pub nhlt: *mut nhlt_acpi_table,
    pub mic_privacy: sof_ace3_mic_privacy,
    pub delayed_ipc_tx_msg: *mut snd_sof_ipc_msg,
}

pub unsafe fn sof_to_bus(_s: *mut snd_sof_dev) -> *mut hdac_bus {
    /* Requires snd_sof_dev::pdata, platform pdata layout, and hda_bus::core from external headers. */
    core::ptr::null_mut()
}

pub unsafe fn sof_to_hbus(_s: *mut snd_sof_dev) -> *mut hda_bus {
    /* Requires snd_sof_dev::pdata and platform pdata layout from external headers. */
    core::ptr::null_mut()
}

#[repr(C)]
pub struct sof_intel_hda_stream {
    pub sdev: *mut snd_sof_dev,
    pub hext_stream: hdac_ext_stream,
    pub sof_intel_stream: sof_intel_stream,
    pub host_reserved: c_int, /* reserve host DMA channel */
    pub flags: u32,
    pub ioc: completion,
}

/* hstream_to_sof_hda_stream(hstream): container_of(hstream, struct sof_intel_hda_stream, hext_stream) */
/* bus_to_sof_hda(bus): container_of(bus, struct sof_intel_hda_dev, hbus.core) */
pub unsafe fn SOF_STREAM_SD_OFFSET(s: *const hdac_stream_index) -> u32 {
    SOF_HDA_ADSP_SD_ENTRY_SIZE * ((*s).index as u32) + SOF_HDA_ADSP_LOADER_BASE
}
#[repr(C)]
pub struct hdac_stream_index {
    pub index: c_int,
}
pub const SOF_STREAM_SD_OFFSET_CRST: u32 = 0x1;

unsafe extern "C" {
    pub fn hda_is_chain_dma_supported(sdev: *mut snd_sof_dev, dai_type: u32) -> bool;
    pub fn hda_dsp_probe_early(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_dsp_probe(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_dsp_remove(sdev: *mut snd_sof_dev);
    pub fn hda_dsp_remove_late(sdev: *mut snd_sof_dev);
    pub fn hda_dsp_core_power_up(sdev: *mut snd_sof_dev, core_mask: c_uint) -> c_int;
    pub fn hda_dsp_core_run(sdev: *mut snd_sof_dev, core_mask: c_uint) -> c_int;
    pub fn hda_dsp_enable_core(sdev: *mut snd_sof_dev, core_mask: c_uint) -> c_int;
    pub fn hda_dsp_core_reset_power_down(sdev: *mut snd_sof_dev, core_mask: c_uint) -> c_int;
    pub fn hda_power_down_dsp(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_dsp_core_get(sdev: *mut snd_sof_dev, core: c_int) -> c_int;
    pub fn hda_dsp_ipc_int_enable(sdev: *mut snd_sof_dev);
    pub fn hda_dsp_ipc_int_disable(sdev: *mut snd_sof_dev);
    pub fn hda_dsp_core_is_enabled(sdev: *mut snd_sof_dev, core_mask: c_uint) -> bool;
    pub fn hda_dsp_set_power_state_ipc3(sdev: *mut snd_sof_dev, target_state: *const sof_dsp_power_state) -> c_int;
    pub fn hda_dsp_set_power_state_ipc4(sdev: *mut snd_sof_dev, target_state: *const sof_dsp_power_state) -> c_int;
    pub fn hda_dsp_suspend(sdev: *mut snd_sof_dev, target_state: u32) -> c_int;
    pub fn hda_dsp_resume(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_dsp_runtime_suspend(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_dsp_runtime_resume(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_dsp_runtime_idle(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_dsp_shutdown_dma_flush(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_dsp_shutdown(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_dsp_set_hw_params_upon_resume(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_dsp_dump(sdev: *mut snd_sof_dev, flags: u32);
    pub fn hda_ipc4_dsp_dump(sdev: *mut snd_sof_dev, flags: u32);
    pub fn hda_ipc_dump(sdev: *mut snd_sof_dev);
    pub fn hda_ipc_irq_dump(sdev: *mut snd_sof_dev);
    pub fn hda_dsp_d0i3_work(work: *mut work_struct);
    pub fn hda_dsp_disable_interrupts(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_check_ipc_irq(sdev: *mut snd_sof_dev) -> bool;
    pub fn hda_get_interface_mask(sdev: *mut snd_sof_dev) -> u32;
    pub fn hda_dsp_get_mult_div(sdev: *mut snd_sof_dev, rate: c_int) -> u32;
    pub fn hda_dsp_get_bits(sdev: *mut snd_sof_dev, sample_bits: c_int) -> u32;
    pub fn hda_dsp_pcm_open(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int;
    pub fn hda_dsp_pcm_close(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int;
    pub fn hda_dsp_pcm_hw_params(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, platform_params: *mut snd_sof_platform_stream_params) -> c_int;
    pub fn hda_dsp_stream_hw_free(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int;
    pub fn hda_dsp_pcm_trigger(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream, cmd: c_int) -> c_int;
    pub fn hda_dsp_pcm_pointer(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t;
    pub fn hda_dsp_pcm_ack(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int;
    pub fn hda_dsp_stream_init(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_dsp_stream_free(sdev: *mut snd_sof_dev);
    pub fn hda_dsp_stream_hw_params(sdev: *mut snd_sof_dev, hext_stream: *mut hdac_ext_stream, dmab: *mut snd_dma_buffer, params: *mut snd_pcm_hw_params) -> c_int;
    pub fn hda_dsp_iccmax_stream_hw_params(sdev: *mut snd_sof_dev, hext_stream: *mut hdac_ext_stream, dmab: *mut snd_dma_buffer, params: *mut snd_pcm_hw_params) -> c_int;
    pub fn hda_dsp_stream_trigger(sdev: *mut snd_sof_dev, hext_stream: *mut hdac_ext_stream, cmd: c_int) -> c_int;
    pub fn hda_dsp_stream_threaded_handler(irq: c_int, context: *mut c_void) -> irqreturn_t;
    pub fn hda_dsp_stream_setup_bdl(sdev: *mut snd_sof_dev, dmab: *mut snd_dma_buffer, hstream: *mut hdac_stream) -> c_int;
    pub fn hda_dsp_check_ipc_irq(sdev: *mut snd_sof_dev) -> bool;
    pub fn hda_dsp_check_stream_irq(sdev: *mut snd_sof_dev) -> bool;
    pub fn hda_dsp_stream_get_position(hstream: *mut hdac_stream, direction: c_int, can_sleep: bool) -> snd_pcm_uframes_t;
    pub fn hda_dsp_get_stream_llp(sdev: *mut snd_sof_dev, component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> u64;
    pub fn hda_dsp_get_stream_ldp(sdev: *mut snd_sof_dev, component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> u64;
    pub fn hda_dsp_stream_get(sdev: *mut snd_sof_dev, direction: c_int, flags: u32) -> *mut hdac_ext_stream;
    pub fn hda_dsp_stream_pair_get(sdev: *mut snd_sof_dev, direction: c_int, flags: u32) -> *mut hdac_ext_stream;
    pub fn hda_dsp_stream_put(sdev: *mut snd_sof_dev, direction: c_int, stream_tag: c_int) -> c_int;
    pub fn hda_dsp_stream_pair_put(sdev: *mut snd_sof_dev, direction: c_int, stream_tag: c_int) -> c_int;
    pub fn hda_dsp_stream_spib_config(sdev: *mut snd_sof_dev, hext_stream: *mut hdac_ext_stream, enable: c_int, size: u32) -> c_int;
    pub fn hda_ipc_msg_data(sdev: *mut snd_sof_dev, sps: *mut snd_sof_pcm_stream, p: *mut c_void, sz: size_t) -> c_int;
    pub fn hda_set_stream_data_offset(sdev: *mut snd_sof_dev, sps: *mut snd_sof_pcm_stream, posn_offset: size_t) -> c_int;
    pub fn hda_dsp_ipc_send_msg(sdev: *mut snd_sof_dev, msg: *mut snd_sof_ipc_msg) -> c_int;
    pub fn hda_dsp_ipc_get_reply(sdev: *mut snd_sof_dev);
    pub fn hda_dsp_ipc_get_mailbox_offset(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_dsp_ipc_get_window_offset(sdev: *mut snd_sof_dev, id: u32) -> c_int;
    pub fn hda_dsp_ipc_irq_thread(irq: c_int, context: *mut c_void) -> irqreturn_t;
    pub fn hda_dsp_ipc_cmd_done(sdev: *mut snd_sof_dev, dir: c_int) -> c_int;
    pub fn hda_dsp_get_state(sdev: *mut snd_sof_dev, level: *const c_char);
    pub fn hda_dsp_dump_ext_rom_status(sdev: *mut snd_sof_dev, level: *const c_char, flags: u32);
    pub fn hda_dsp_cl_boot_firmware(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_dsp_cl_boot_firmware_iccmax(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_cl_copy_fw(sdev: *mut snd_sof_dev, hext_stream: *mut hdac_ext_stream) -> c_int;
    pub fn hda_cl_prepare(dev: *mut device, format: c_uint, size: c_uint, dmab: *mut snd_dma_buffer, persistent_buffer: bool, direction: c_int, is_iccmax: bool) -> *mut hdac_ext_stream;
    pub fn hda_cl_trigger(dev: *mut device, hext_stream: *mut hdac_ext_stream, cmd: c_int) -> c_int;
    pub fn hda_cl_cleanup(dev: *mut device, dmab: *mut snd_dma_buffer, persistent_buffer: bool, hext_stream: *mut hdac_ext_stream, is_iccmax: bool) -> c_int;
    pub fn cl_dsp_init(sdev: *mut snd_sof_dev, stream_tag: c_int, imr_boot: bool) -> c_int;
}

pub const HDA_CL_STREAM_FORMAT: u32 = 0x40;

unsafe extern "C" {
    pub fn hda_dsp_pre_fw_run(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_dsp_post_fw_run(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_dsp_ext_man_get_cavs_config_data(sdev: *mut snd_sof_dev, hdr: *const sof_ext_man_elem_header) -> c_int;
    pub fn hda_dsp_ctrl_get_caps(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_dsp_ctrl_ppcap_enable(sdev: *mut snd_sof_dev, enable: bool);
    pub fn hda_dsp_ctrl_ppcap_int_enable(sdev: *mut snd_sof_dev, enable: bool);
    pub fn hda_dsp_ctrl_link_reset(sdev: *mut snd_sof_dev, reset: bool) -> c_int;
    pub fn hda_dsp_ctrl_misc_clock_gating(sdev: *mut snd_sof_dev, enable: bool);
    pub fn hda_dsp_ctrl_clock_power_gating(sdev: *mut snd_sof_dev, enable: bool) -> c_int;
    pub fn hda_dsp_ctrl_init_chip(sdev: *mut snd_sof_dev, detect_codec: bool) -> c_int;
    pub fn hda_dsp_ctrl_stop_chip(sdev: *mut snd_sof_dev);
    pub fn sof_hda_bus_init(sdev: *mut snd_sof_dev, dev: *mut device);
    pub fn sof_hda_bus_exit(sdev: *mut snd_sof_dev);
}

/* CONFIG_SND_SOC_SOF_HDA_AUDIO_CODEC disabled inline fallbacks. */
pub unsafe fn hda_codec_probe_bus(_sdev: *mut snd_sof_dev) {}
pub unsafe fn hda_codec_jack_wake_enable(_sdev: *mut snd_sof_dev, _enable: bool) {}
pub unsafe fn hda_codec_jack_check(_sdev: *mut snd_sof_dev) {}
pub unsafe fn hda_codec_check_for_state_change(_sdev: *mut snd_sof_dev) {}
pub unsafe fn hda_codec_init_cmd_io(_sdev: *mut snd_sof_dev) {}
pub unsafe fn hda_codec_resume_cmd_io(_sdev: *mut snd_sof_dev) {}
pub unsafe fn hda_codec_stop_cmd_io(_sdev: *mut snd_sof_dev) {}
pub unsafe fn hda_codec_suspend_cmd_io(_sdev: *mut snd_sof_dev) {}
pub unsafe fn hda_codec_detect_mask(_sdev: *mut snd_sof_dev) {}
pub unsafe fn hda_codec_rirb_status_clear(_sdev: *mut snd_sof_dev) {}
pub unsafe fn hda_codec_check_rirb_status(_sdev: *mut snd_sof_dev) -> bool { false }
pub unsafe fn hda_codec_set_codec_wakeup(_sdev: *mut snd_sof_dev, _status: bool) {}
pub unsafe fn hda_codec_device_remove(_sdev: *mut snd_sof_dev) {}

/* CONFIG_SND_SOC_SOF_HDA_AUDIO_CODEC && CONFIG_SND_HDA_CODEC_HDMI disabled inline fallbacks. */
pub unsafe fn hda_codec_i915_display_power(_sdev: *mut snd_sof_dev, _enable: bool) {}
pub unsafe fn hda_codec_i915_init(_sdev: *mut snd_sof_dev) -> c_int { 0 }
pub unsafe fn hda_codec_i915_exit(_sdev: *mut snd_sof_dev) -> c_int { 0 }

unsafe extern "C" {
    pub fn hda_dsp_trace_init(sdev: *mut snd_sof_dev, dmab: *mut snd_dma_buffer, dtrace_params: *mut sof_ipc_dma_trace_params_ext) -> c_int;
    pub fn hda_dsp_trace_release(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_dsp_trace_trigger(sdev: *mut snd_sof_dev, cmd: c_int) -> c_int;
}

/* CONFIG_SND_SOC_SOF_INTEL_SOUNDWIRE disabled inline fallbacks. */
pub unsafe fn hda_sdw_check_lcount_common(_sdev: *mut snd_sof_dev) -> c_int { 0 }
pub unsafe fn hda_sdw_check_lcount_ext(_sdev: *mut snd_sof_dev) -> c_int { 0 }
pub unsafe fn hda_sdw_check_lcount(_sdev: *mut snd_sof_dev) -> c_int { 0 }
pub unsafe fn hda_sdw_startup(_sdev: *mut snd_sof_dev) -> c_int { 0 }
pub unsafe fn hda_common_enable_sdw_irq(_sdev: *mut snd_sof_dev, _enable: bool) {}
pub unsafe fn hda_sdw_int_enable(_sdev: *mut snd_sof_dev, _enable: bool) {}
pub unsafe fn hda_sdw_check_wakeen_irq_common(_sdev: *mut snd_sof_dev) -> bool { false }
pub unsafe fn hda_sdw_process_wakeen_common(_sdev: *mut snd_sof_dev) {}
pub unsafe fn hda_sdw_process_wakeen(_sdev: *mut snd_sof_dev) {}
pub unsafe fn hda_common_check_sdw_irq(_sdev: *mut snd_sof_dev) -> bool { false }

unsafe extern "C" {
    pub fn sdw_hda_dai_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, cpu_dai: *mut snd_soc_dai, link_id: c_int, intel_alh_id: c_int) -> c_int;
    pub fn sdw_hda_dai_hw_free(substream: *mut snd_pcm_substream, cpu_dai: *mut snd_soc_dai, link_id: c_int) -> c_int;
    pub fn sdw_hda_dai_trigger(substream: *mut snd_pcm_substream, cmd: c_int, cpu_dai: *mut snd_soc_dai) -> c_int;
    pub fn hda_data_stream_prepare(dev: *mut device, format: c_uint, size: c_uint, dmab: *mut snd_dma_buffer, persistent_buffer: bool, direction: c_int, is_iccmax: bool, pair: bool) -> *mut hdac_ext_stream;
    pub fn hda_data_stream_cleanup(dev: *mut device, dmab: *mut snd_dma_buffer, persistent_buffer: bool, hext_stream: *mut hdac_ext_stream, is_iccmax: bool, pair: bool) -> c_int;
    pub static mut skl_dai: [snd_soc_dai_driver; 0];
    pub fn hda_dsp_dais_suspend(sdev: *mut snd_sof_dev) -> c_int;
    pub static sof_hda_common_ops: snd_sof_dsp_ops;
    pub static mut sof_skl_ops: snd_sof_dsp_ops;
    pub fn sof_skl_ops_init(sdev: *mut snd_sof_dev) -> c_int;
    pub static mut sof_apl_ops: snd_sof_dsp_ops;
    pub fn sof_apl_ops_init(sdev: *mut snd_sof_dev) -> c_int;
    pub static mut sof_cnl_ops: snd_sof_dsp_ops;
    pub fn sof_cnl_ops_init(sdev: *mut snd_sof_dev) -> c_int;
    pub static mut sof_tgl_ops: snd_sof_dsp_ops;
    pub fn sof_tgl_ops_init(sdev: *mut snd_sof_dev) -> c_int;
    pub static mut sof_icl_ops: snd_sof_dsp_ops;
    pub fn sof_icl_ops_init(sdev: *mut snd_sof_dev) -> c_int;
    pub static skl_chip_info: sof_intel_dsp_desc;
    pub static apl_chip_info: sof_intel_dsp_desc;
    pub static cnl_chip_info: sof_intel_dsp_desc;
    pub static icl_chip_info: sof_intel_dsp_desc;
    pub static tgl_chip_info: sof_intel_dsp_desc;
    pub static tglh_chip_info: sof_intel_dsp_desc;
    pub static ehl_chip_info: sof_intel_dsp_desc;
    pub static jsl_chip_info: sof_intel_dsp_desc;
    pub static adls_chip_info: sof_intel_dsp_desc;
    pub static mtl_chip_info: sof_intel_dsp_desc;
    pub static arl_s_chip_info: sof_intel_dsp_desc;
    pub static lnl_chip_info: sof_intel_dsp_desc;
    pub static ptl_chip_info: sof_intel_dsp_desc;
    pub static wcl_chip_info: sof_intel_dsp_desc;
    pub static nvl_chip_info: sof_intel_dsp_desc;
    pub static nvl_s_chip_info: sof_intel_dsp_desc;
}

/* CONFIG_SND_SOC_SOF_HDA_PROBES disabled inline fallbacks. */
pub unsafe fn hda_probes_register(_sdev: *mut snd_sof_dev) -> c_int { 0 }
pub unsafe fn hda_probes_unregister(_sdev: *mut snd_sof_dev) {}

unsafe extern "C" {
    pub fn hda_register_clients(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_unregister_clients(sdev: *mut snd_sof_dev);
    pub fn hda_machine_select(sdev: *mut snd_sof_dev) -> *mut snd_soc_acpi_mach;
    pub fn hda_set_mach_params(mach: *mut snd_soc_acpi_mach, sdev: *mut snd_sof_dev);
    pub fn hda_pci_intel_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int;
}

pub const SOF_HDA_POSITION_QUIRK_USE_SKYLAKE_LEGACY: c_int = 0; /* previous implementation */
pub const SOF_HDA_POSITION_QUIRK_USE_DPIB_REGISTERS: c_int = 1; /* recommended if VC0 only */
pub const SOF_HDA_POSITION_QUIRK_USE_DPIB_DDR_UPDATE: c_int = 2; /* recommended with VC0 or VC1 */

unsafe extern "C" {
    pub static mut sof_hda_position_quirk: c_int;
    pub fn hda_set_dai_drv_ops(sdev: *mut snd_sof_dev, ops: *mut snd_sof_dsp_ops);
    pub fn hda_ops_free(sdev: *mut snd_sof_dev);
    pub fn hda_dsp_cl_boot_firmware_skl(sdev: *mut snd_sof_dev) -> c_int;
    pub fn hda_dsp_core_stall_reset(sdev: *mut snd_sof_dev, core_mask: c_uint) -> c_int;
    pub fn cnl_ipc4_irq_thread(irq: c_int, context: *mut c_void) -> irqreturn_t;
    pub fn cnl_ipc4_send_msg(sdev: *mut snd_sof_dev, msg: *mut snd_sof_ipc_msg) -> c_int;
    pub fn hda_dsp_ipc4_irq_thread(irq: c_int, context: *mut c_void) -> irqreturn_t;
    pub fn hda_ipc4_tx_is_busy(sdev: *mut snd_sof_dev) -> bool;
    pub fn hda_dsp_ipc4_schedule_d0i3_work(hdev: *mut sof_intel_hda_dev, msg: *mut snd_sof_ipc_msg);
    pub fn hda_dsp_ipc4_send_msg(sdev: *mut snd_sof_dev, msg: *mut snd_sof_ipc_msg) -> c_int;
    pub fn hda_ipc4_dump(sdev: *mut snd_sof_dev);
    pub static mut sdw_callback: sdw_intel_ops;
    pub fn hda_dsp_ipc4_load_library(sdev: *mut snd_sof_dev, fw_lib: *mut sof_ipc4_fw_library, reload: bool) -> c_int;
}

/**
 * struct hda_dai_widget_dma_ops - DAI DMA ops optional by default unless specified otherwise
 */
#[repr(C)]
pub struct hda_dai_widget_dma_ops {
    pub get_hext_stream: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_soc_dai, *mut snd_pcm_substream) -> *mut hdac_ext_stream>,
    pub assign_hext_stream: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_soc_dai, *mut snd_pcm_substream, *mut hdac_ext_link) -> *mut hdac_ext_stream>,
    pub release_hext_stream: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_soc_dai, *mut snd_pcm_substream)>,
    pub setup_hext_stream: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut hdac_ext_stream, c_uint)>,
    pub reset_hext_stream: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut hdac_ext_stream)>,
    pub pre_trigger: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_soc_dai, *mut snd_pcm_substream, c_int) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_soc_dai, *mut snd_pcm_substream, c_int) -> c_int>,
    pub post_trigger: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_soc_dai, *mut snd_pcm_substream, c_int) -> c_int>,
    pub codec_dai_set_stream: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_pcm_substream, *mut hdac_stream)>,
    pub calc_stream_format: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_uint>,
    pub get_hlink: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_pcm_substream) -> *mut hdac_ext_link>,
}

unsafe extern "C" {
    pub fn hda_select_dai_widget_ops(sdev: *mut snd_sof_dev, swidget: *mut snd_sof_widget) -> *const hda_dai_widget_dma_ops;
    pub fn hda_dai_config(w: *mut snd_soc_dapm_widget, flags: c_uint, data: *mut snd_sof_dai_config_data) -> c_int;
}

pub unsafe fn widget_to_sdev(_w: *mut snd_soc_dapm_widget) -> *mut snd_sof_dev {
    /* Requires snd_soc_dapm_widget::dobj, snd_sof_widget::scomp, and snd_soc_component_get_drvdata(). */
    core::ptr::null_mut()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
