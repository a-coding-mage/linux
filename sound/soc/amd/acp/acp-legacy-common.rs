// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2023 Advanced Micro Devices, Inc.
//
// Authors: Syed Saba Kareem <Syed.SabaKareem@amd.com>
//

/*
 * Common file to be used by amd platforms
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{self, MaybeUninit};
use core::ptr;

type u16 = u16;
type u32 = u32;
type acpi_handle = *mut c_void;
type acpi_integer = u64;
type irqreturn_t = c_uint;

const ACP_RENOIR_PDM_ADDR: u32 = 0x02;
const ACP_REMBRANDT_PDM_ADDR: u32 = 0x03;
const ACP63_PDM_ADDR: u32 = 0x02;
const ACP70_PDM_ADDR: u32 = 0x02;

extern "C" {
    static ACP_ERROR_MASK: u32;
    static ACP_EXT_INTR_STAT_CLEAR_MASK: u32;
    static ACP_POWERED_ON: u32;
    static ACP_PGFSM_STATUS_MASK: u32;
    static ACP_POWER_ON_IN_PROGRESS: u32;
    static ACP_PGFSM_CNTL_POWER_ON_MASK: u32;
    static ACP_SOFT_RST_DONE_MASK: u32;
    static DELAY_US: u32;
    static ACP_TIMEOUT: u32;
    static DMA_SIZE: u32;
    static FIFO_SIZE: u32;
    static PDM_CLK_FREQ_MASK: u32;
    static PDM_MISC_CTRL_MASK: u32;
    static PDM_DEC_64: u32;
    static PDM_DMA_INTR_MASK: u32;
    static TDM_ENABLE: u32;
    static PLATFORM_DEVID_NONE: c_int;
    static ACPI_TYPE_INTEGER: c_int;

    static ACP_WOV_RX_RINGBUFADDR: u32;
    static ACP_WOV_RX_RINGBUFSIZE: u32;
    static ACP_WOV_RX_INTR_WATERMARK_SIZE: u32;
    static ACPAXI2AXI_ATU_CTRL: u32;
    static ACP_WOV_CLK_CTRL: u32;
    static ACP_WOV_MISC_CTRL: u32;
    static ACP_WOV_PDM_NO_OF_CHANNELS: u32;
    static ACP_WOV_PDM_DECIMATION_FACTOR: u32;
    static ACP_HS_TX_DMA_SIZE: u32;
    static ACP_HS_TX_FIFOADDR: u32;
    static ACP_HS_TX_FIFOSIZE: u32;
    static ACP_HS_TX_RINGBUFADDR: u32;
    static ACP_HS_RX_DMA_SIZE: u32;
    static ACP_HS_RX_FIFOADDR: u32;
    static ACP_HS_RX_FIFOSIZE: u32;
    static ACP_HS_RX_RINGBUFADDR: u32;
    static ACP_BTTDM_ITER: u32;
    static ACP_BTTDM_TXFRMT: u32;
    static ACP_I2STDM_ITER: u32;
    static ACP_I2STDM_TXFRMT: u32;
    static ACP_HSTDM_ITER: u32;
    static ACP_HSTDM_TXFRMT: u32;
    static ACP_BTTDM_IRER: u32;
    static ACP_BTTDM_RXFRMT: u32;
    static ACP_I2STDM_IRER: u32;
    static ACP_I2STDM_RXFRMT: u32;
    static ACP_HSTDM_IRER: u32;
    static ACP_HSTDM_RXFRMT: u32;
    static ACP_PGFSM_STATUS: u32;
    static ACP_PGFSM_CONTROL: u32;
    static ACP6X_PGFSM_STATUS: u32;
    static ACP6X_PGFSM_CONTROL: u32;
    static ACP63_PGFSM_STATUS: u32;
    static ACP63_PGFSM_CONTROL: u32;
    static ACP70_PGFSM_STATUS: u32;
    static ACP70_PGFSM_CONTROL: u32;
    static ACP_SOFT_RESET: u32;
    static ACP_CONTROL: u32;
    static ACP_ZSC_DSP_CTRL: u32;
    static ACP3X_PIN_CONFIG: u32;
    static ACP_PIN_CONFIG: u32;
    static MEM_WINDOW_START: u32;
    static SP_PB_FIFO_ADDR_OFFSET: u32;
    static SP_CAPT_FIFO_ADDR_OFFSET: u32;
    static BT_PB_FIFO_ADDR_OFFSET: u32;
    static BT_CAPT_FIFO_ADDR_OFFSET: u32;
    static HS_PB_FIFO_ADDR_OFFSET: u32;
    static HS_CAPT_FIFO_ADDR_OFFSET: u32;
    static ACP7x_I2S_SP_TX_MEM_WINDOW_START: u32;
    static ACP7x_I2S_SP_RX_MEM_WINDOW_START: u32;
    static ACP7x_I2S_BT_TX_MEM_WINDOW_START: u32;
    static ACP7x_I2S_BT_RX_MEM_WINDOW_START: u32;
    static ACP7x_I2S_HS_TX_MEM_WINDOW_START: u32;
    static ACP7x_I2S_HS_RX_MEM_WINDOW_START: u32;
    static I2S_SP_TX_MEM_WINDOW_START: u32;
    static I2S_SP_RX_MEM_WINDOW_START: u32;
    static I2S_BT_TX_MEM_WINDOW_START: u32;
    static I2S_BT_RX_MEM_WINDOW_START: u32;
    static I2S_HS_TX_MEM_WINDOW_START: u32;
    static I2S_HS_RX_MEM_WINDOW_START: u32;
    static I2S_SP_INSTANCE: u32;
    static I2S_BT_INSTANCE: u32;
    static I2S_HS_INSTANCE: u32;
    static SNDRV_PCM_STREAM_PLAYBACK: c_uint;
    static ACP_RN_PCI_ID: u32;
    static ACP_RMB_PCI_ID: u32;
    static ACP63_PCI_ID: u32;
    static ACP70_PCI_ID: u32;
    static ACP71_PCI_ID: u32;
    static ACP72_PCI_ID: u32;
    static FLAG_AMD_LEGACY_ONLY_DMIC: u32;
    static ACP_CONFIG_4: u32;
    static ACP_CONFIG_5: u32;
    static ACP_CONFIG_6: u32;
    static ACP_CONFIG_7: u32;
    static ACP_CONFIG_8: u32;
    static ACP_CONFIG_9: u32;
    static ACP_CONFIG_10: u32;
    static ACP_CONFIG_11: u32;
    static ACP_CONFIG_12: u32;
    static ACP_CONFIG_13: u32;
    static ACP_CONFIG_14: u32;
    static ACP_CONFIG_17: u32;
    static ACP_CONFIG_18: u32;
    static ACP_CONFIG_19: u32;
    static ACP_CONFIG_20: u32;
    static IRQ_HANDLED: irqreturn_t;
    static IRQ_NONE: irqreturn_t;
    static EINVAL: c_int;
}

#[repr(C)]
pub struct acp_resource {
    pub offset: u32,
    pub no_of_ctrls: u32,
    pub irqp_used: u32,
    pub soc_mclk: bool,
    pub irq_reg_offset: u32,
    pub scratch_reg_offset: u32,
    pub sram_pte_offset: u32,
}

#[repr(C)]
pub struct snd_acp_hw_ops {
    pub acp_init: Option<unsafe extern "C" fn(*mut acp_chip_info) -> c_int>,
    pub acp_deinit: Option<unsafe extern "C" fn(*mut acp_chip_info) -> c_int>,
    pub irq: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
    pub en_interrupts: Option<unsafe extern "C" fn(*mut acp_chip_info) -> c_int>,
    pub dis_interrupts: Option<unsafe extern "C" fn(*mut acp_chip_info) -> c_int>,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct acp_stream {
    pub list: list_head,
    pub irq_bit: u32,
    pub substream: *mut snd_pcm_substream,
    pub reg_offset: u32,
    pub dai_id: u32,
}

#[repr(C)]
pub struct acp_chip_info {
    pub rsrc: *mut acp_resource,
    pub acp_lock: c_void,
    pub stream_list: list_head,
    pub base: *mut u8,
    pub acp_rev: u32,
    pub ch_mask: u32,
    pub tdm_tx_fmt: [u32; 3],
    pub tdm_rx_fmt: [u32; 3],
    pub xfer_tx_resolution: [u32; 3],
    pub xfer_rx_resolution: [u32; 3],
    pub tdm_mode: u32,
    pub flag: u32,
    pub is_pdm_dev: bool,
    pub mach_dev: *mut platform_device,
    pub dev: *mut device,
    pub machines: *mut snd_soc_acpi_mach,
    pub is_i2s_config: bool,
    pub is_pdm_config: bool,
    pub acp_hw_ops: *mut snd_acp_hw_ops,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_uint,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub private_data: *mut acp_stream,
    pub period_size: usize,
    pub buffer_size: usize,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub driver: *mut snd_soc_dai_driver,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub id: u32,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub drv_name: *const c_char,
    pub mach_params: snd_soc_acpi_mach_params,
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
    pub subsystem_rev: u32,
}

#[repr(C)]
pub struct acpi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub union acpi_object_data {
    pub value: u64,
}

#[repr(C)]
pub struct acpi_object_integer {
    pub value: u64,
}

#[repr(C)]
pub union acpi_object {
    pub integer: acpi_object_integer,
}

extern "C" {
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn spin_lock(lock: *mut c_void);
    fn spin_unlock(lock: *mut c_void);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: usize) -> u32;
    fn dev_get_platdata(dev: *mut device) -> *mut acp_chip_info;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn readl_poll_timeout(addr: *mut u8, val: *mut u32, cond: c_int, delay_us: u32, timeout: u32) -> c_int;
    fn platform_device_register_data(
        parent: *mut device,
        name: *const c_char,
        id: c_int,
        data: *const c_void,
        size: usize,
    ) -> *mut platform_device;
    fn snd_soc_acpi_find_machine(machines: *mut snd_soc_acpi_mach) -> *mut snd_soc_acpi_mach;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn acpi_find_child_device(parent: *mut acpi_device, addr: u32, check_children: c_uint) -> *mut acpi_device;
    fn ACPI_COMPANION(dev: *mut device) -> *mut acpi_device;
    fn acpi_dev_get_property(
        adev: *mut acpi_device,
        name: *const c_char,
        ty: c_int,
        obj: *mut *const acpi_object,
    ) -> c_int;
    fn ACPI_HANDLE(dev: *mut device) -> acpi_handle;
    fn acpi_evaluate_integer(handle: acpi_handle, pathname: *const c_char, args: *mut c_void, data: *mut acpi_integer) -> u32;
    fn ACPI_FAILURE(status: u32) -> bool;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;

    fn ACP_EXTERNAL_INTR_STAT(chip: *mut acp_chip_info, irq: u32) -> *mut u8;
    fn ACP_EXTERNAL_INTR_ENB(chip: *mut acp_chip_info) -> *mut u8;
    fn ACP_EXTERNAL_INTR_CNTL(chip: *mut acp_chip_info, irq: u32) -> *mut u8;
    fn ACP_I2S_TX_DMA_SIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_I2S_TX_FIFOADDR(chip: *mut acp_chip_info) -> u32;
    fn ACP_I2S_TX_FIFOSIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_I2S_TX_RINGBUFADDR(chip: *mut acp_chip_info) -> u32;
    fn ACP_I2S_RX_DMA_SIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_I2S_RX_FIFOADDR(chip: *mut acp_chip_info) -> u32;
    fn ACP_I2S_RX_FIFOSIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_I2S_RX_RINGBUFADDR(chip: *mut acp_chip_info) -> u32;
    fn ACP_BT_TX_DMA_SIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_BT_TX_FIFOADDR(chip: *mut acp_chip_info) -> u32;
    fn ACP_BT_TX_FIFOSIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_BT_TX_RINGBUFADDR(chip: *mut acp_chip_info) -> u32;
    fn ACP_BT_RX_DMA_SIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_BT_RX_FIFOADDR(chip: *mut acp_chip_info) -> u32;
    fn ACP_BT_RX_FIFOSIZE(chip: *mut acp_chip_info) -> u32;
    fn ACP_BT_RX_RINGBUFADDR(chip: *mut acp_chip_info) -> u32;
    fn BIT(bit: u32) -> u32;
    fn I2S_RX_THRESHOLD(offset: u32) -> u32;
    fn BT_RX_THRESHOLD(offset: u32) -> u32;
    fn I2S_TX_THRESHOLD(offset: u32) -> u32;
    fn BT_TX_THRESHOLD(offset: u32) -> u32;
    fn HS_RX_THRESHOLD(offset: u32) -> u32;
    fn HS_TX_THRESHOLD(offset: u32) -> u32;
}

#[no_mangle]
pub static mut rn_rsrc: acp_resource = acp_resource {
    offset: 20,
    no_of_ctrls: 1,
    irqp_used: 0,
    soc_mclk: false,
    irq_reg_offset: 0x1800,
    scratch_reg_offset: 0x12800,
    sram_pte_offset: 0x02052800,
};

#[no_mangle]
pub static mut rmb_rsrc: acp_resource = acp_resource {
    offset: 0,
    no_of_ctrls: 2,
    irqp_used: 1,
    soc_mclk: true,
    irq_reg_offset: 0x1a00,
    scratch_reg_offset: 0x12800,
    sram_pte_offset: 0x03802800,
};

#[no_mangle]
pub static mut acp63_rsrc: acp_resource = acp_resource {
    offset: 0,
    no_of_ctrls: 2,
    irqp_used: 1,
    soc_mclk: true,
    irq_reg_offset: 0x1a00,
    scratch_reg_offset: 0x12800,
    sram_pte_offset: 0x03802800,
};

#[no_mangle]
pub static mut acp70_rsrc: acp_resource = acp_resource {
    offset: 0,
    no_of_ctrls: 2,
    irqp_used: 1,
    soc_mclk: true,
    irq_reg_offset: 0x1a00,
    scratch_reg_offset: 0x10000,
    sram_pte_offset: 0x03800000,
};

static acp_common_hw_ops: snd_acp_hw_ops = snd_acp_hw_ops {
    /* ACP hardware initilizations */
    acp_init: Some(acp_init),
    acp_deinit: Some(acp_deinit),

    /* ACP Interrupts*/
    irq: Some(acp_irq_handler),
    en_interrupts: Some(acp_enable_interrupts),
    dis_interrupts: Some(acp_disable_interrupts),
};

unsafe fn reg(base: *mut u8, offset: u32) -> *mut u8 {
    base.add(offset as usize)
}

unsafe fn list_entry_acp_stream(pos: *mut list_head) -> *mut acp_stream {
    (pos as *mut u8).sub(mem::offset_of!(acp_stream, list)) as *mut acp_stream
}

#[no_mangle]
pub unsafe extern "C" fn acp_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    let chip = data as *mut acp_chip_info;
    let rsrc = (*chip).rsrc;
    let mut i2s_flag: u16 = 0;
    let mut ext_intr_stat1: u32 = 0;

    if (*rsrc).no_of_ctrls == 2 {
        ext_intr_stat1 = readl(ACP_EXTERNAL_INTR_STAT(chip, (*rsrc).irqp_used.wrapping_sub(1)));
    }

    let ext_intr_stat = readl(ACP_EXTERNAL_INTR_STAT(chip, (*rsrc).irqp_used));

    spin_lock(&mut (*chip).acp_lock);
    let mut pos = (*chip).stream_list.next;
    while pos != &mut (*chip).stream_list {
        let stream = list_entry_acp_stream(pos);
        if (ext_intr_stat & (*stream).irq_bit) != 0 {
            writel((*stream).irq_bit, ACP_EXTERNAL_INTR_STAT(chip, (*rsrc).irqp_used));
            snd_pcm_period_elapsed((*stream).substream);
            i2s_flag = 1;
        }
        if (*(*chip).rsrc).no_of_ctrls == 2 {
            if (ext_intr_stat1 & (*stream).irq_bit) != 0 {
                writel(
                    (*stream).irq_bit,
                    ACP_EXTERNAL_INTR_STAT(chip, (*rsrc).irqp_used.wrapping_sub(1)),
                );
                snd_pcm_period_elapsed((*stream).substream);
                i2s_flag = 1;
            }
        }
        pos = (*pos).next;
    }
    spin_unlock(&mut (*chip).acp_lock);
    if i2s_flag != 0 {
        return IRQ_HANDLED;
    }

    IRQ_NONE
}

#[no_mangle]
pub unsafe extern "C" fn acp_enable_interrupts(chip: *mut acp_chip_info) -> c_int {
    let rsrc = (*chip).rsrc;
    writel(0x01, ACP_EXTERNAL_INTR_ENB(chip));
    let mut ext_intr_ctrl = readl(ACP_EXTERNAL_INTR_CNTL(chip, (*rsrc).irqp_used));
    ext_intr_ctrl |= ACP_ERROR_MASK;
    writel(ext_intr_ctrl, ACP_EXTERNAL_INTR_CNTL(chip, (*rsrc).irqp_used));

    0
}

#[no_mangle]
pub unsafe extern "C" fn acp_disable_interrupts(chip: *mut acp_chip_info) -> c_int {
    let rsrc = (*chip).rsrc;

    writel(ACP_EXT_INTR_STAT_CLEAR_MASK, ACP_EXTERNAL_INTR_STAT(chip, (*rsrc).irqp_used));
    writel(0x00, ACP_EXTERNAL_INTR_ENB(chip));

    0
}

unsafe fn set_acp_pdm_ring_buffer(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let runtime = (*substream).runtime;
    let stream = (*runtime).private_data;
    let dev = (*(*dai).component).dev;
    let chip = dev_get_platdata(dev);

    let period_bytes = frames_to_bytes(runtime, (*runtime).period_size);
    let pdm_size = frames_to_bytes(runtime, (*runtime).buffer_size);
    let physical_addr = (*stream).reg_offset.wrapping_add(MEM_WINDOW_START);

    /* Init ACP PDM Ring buffer */
    writel(physical_addr, reg((*chip).base, ACP_WOV_RX_RINGBUFADDR));
    writel(pdm_size, reg((*chip).base, ACP_WOV_RX_RINGBUFSIZE));
    writel(period_bytes, reg((*chip).base, ACP_WOV_RX_INTR_WATERMARK_SIZE));
    writel(0x01, reg((*chip).base, ACPAXI2AXI_ATU_CTRL));
}

unsafe fn set_acp_pdm_clk(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let dev = (*(*dai).component).dev;
    let chip = dev_get_platdata(dev);

    /* Enable default ACP PDM clk */
    writel(PDM_CLK_FREQ_MASK, reg((*chip).base, ACP_WOV_CLK_CTRL));
    let mut pdm_ctrl = readl(reg((*chip).base, ACP_WOV_MISC_CTRL));
    pdm_ctrl |= PDM_MISC_CTRL_MASK;
    writel(pdm_ctrl, reg((*chip).base, ACP_WOV_MISC_CTRL));
    set_acp_pdm_ring_buffer(substream, dai);
}

#[no_mangle]
pub unsafe extern "C" fn restore_acp_pdm_params(
    substream: *mut snd_pcm_substream,
    chip: *mut acp_chip_info,
) {
    let soc_runtime = snd_soc_substream_to_rtd(substream);
    let dai = snd_soc_rtd_to_cpu(soc_runtime, 0);

    /* Programming channel mask and sampling rate */
    writel((*chip).ch_mask, reg((*chip).base, ACP_WOV_PDM_NO_OF_CHANNELS));
    writel(PDM_DEC_64, reg((*chip).base, ACP_WOV_PDM_DECIMATION_FACTOR));

    /* Enabling ACP Pdm interuppts */
    let mut ext_int_ctrl = readl(ACP_EXTERNAL_INTR_CNTL(chip, 0));
    ext_int_ctrl |= PDM_DMA_INTR_MASK;
    writel(ext_int_ctrl, ACP_EXTERNAL_INTR_CNTL(chip, 0));
    set_acp_pdm_clk(substream, dai);
}

unsafe fn set_acp_i2s_dma_fifo(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let dev = (*(*dai).component).dev;
    let chip = dev_get_platdata(dev);
    let rsrc = (*chip).rsrc;
    let stream = (*(*substream).runtime).private_data;
    let dir = (*substream).stream;
    let (reg_dma_size, reg_fifo_size, reg_fifo_addr, acp_fifo_addr, phy_addr);

    match (*(*dai).driver).id {
        id if id == I2S_SP_INSTANCE => {
            if dir == SNDRV_PCM_STREAM_PLAYBACK {
                reg_dma_size = ACP_I2S_TX_DMA_SIZE(chip);
                acp_fifo_addr = (*rsrc).sram_pte_offset.wrapping_add(SP_PB_FIFO_ADDR_OFFSET);
                reg_fifo_addr = ACP_I2S_TX_FIFOADDR(chip);
                reg_fifo_size = ACP_I2S_TX_FIFOSIZE(chip);
                phy_addr = if (*chip).acp_rev >= ACP70_PCI_ID {
                    ACP7x_I2S_SP_TX_MEM_WINDOW_START
                } else {
                    I2S_SP_TX_MEM_WINDOW_START.wrapping_add((*stream).reg_offset)
                };
                writel(phy_addr, reg((*chip).base, ACP_I2S_TX_RINGBUFADDR(chip)));
            } else {
                reg_dma_size = ACP_I2S_RX_DMA_SIZE(chip);
                acp_fifo_addr = (*rsrc).sram_pte_offset.wrapping_add(SP_CAPT_FIFO_ADDR_OFFSET);
                reg_fifo_addr = ACP_I2S_RX_FIFOADDR(chip);
                reg_fifo_size = ACP_I2S_RX_FIFOSIZE(chip);
                phy_addr = if (*chip).acp_rev >= ACP70_PCI_ID {
                    ACP7x_I2S_SP_RX_MEM_WINDOW_START
                } else {
                    I2S_SP_RX_MEM_WINDOW_START.wrapping_add((*stream).reg_offset)
                };
                writel(phy_addr, reg((*chip).base, ACP_I2S_RX_RINGBUFADDR(chip)));
            }
        }
        id if id == I2S_BT_INSTANCE => {
            if dir == SNDRV_PCM_STREAM_PLAYBACK {
                reg_dma_size = ACP_BT_TX_DMA_SIZE(chip);
                acp_fifo_addr = (*rsrc).sram_pte_offset.wrapping_add(BT_PB_FIFO_ADDR_OFFSET);
                reg_fifo_addr = ACP_BT_TX_FIFOADDR(chip);
                reg_fifo_size = ACP_BT_TX_FIFOSIZE(chip);
                phy_addr = if (*chip).acp_rev >= ACP70_PCI_ID {
                    ACP7x_I2S_BT_TX_MEM_WINDOW_START
                } else {
                    I2S_BT_TX_MEM_WINDOW_START.wrapping_add((*stream).reg_offset)
                };
                writel(phy_addr, reg((*chip).base, ACP_BT_TX_RINGBUFADDR(chip)));
            } else {
                reg_dma_size = ACP_BT_RX_DMA_SIZE(chip);
                acp_fifo_addr = (*rsrc).sram_pte_offset.wrapping_add(BT_CAPT_FIFO_ADDR_OFFSET);
                reg_fifo_addr = ACP_BT_RX_FIFOADDR(chip);
                reg_fifo_size = ACP_BT_RX_FIFOSIZE(chip);
                phy_addr = if (*chip).acp_rev >= ACP70_PCI_ID {
                    ACP7x_I2S_BT_RX_MEM_WINDOW_START
                } else {
                    I2S_BT_RX_MEM_WINDOW_START.wrapping_add((*stream).reg_offset)
                };
                writel(phy_addr, reg((*chip).base, ACP_BT_RX_RINGBUFADDR(chip)));
            }
        }
        id if id == I2S_HS_INSTANCE => {
            if dir == SNDRV_PCM_STREAM_PLAYBACK {
                reg_dma_size = ACP_HS_TX_DMA_SIZE;
                acp_fifo_addr = (*rsrc).sram_pte_offset.wrapping_add(HS_PB_FIFO_ADDR_OFFSET);
                reg_fifo_addr = ACP_HS_TX_FIFOADDR;
                reg_fifo_size = ACP_HS_TX_FIFOSIZE;
                phy_addr = if (*chip).acp_rev >= ACP70_PCI_ID {
                    ACP7x_I2S_HS_TX_MEM_WINDOW_START
                } else {
                    I2S_HS_TX_MEM_WINDOW_START.wrapping_add((*stream).reg_offset)
                };
                writel(phy_addr, reg((*chip).base, ACP_HS_TX_RINGBUFADDR));
            } else {
                reg_dma_size = ACP_HS_RX_DMA_SIZE;
                acp_fifo_addr = (*rsrc).sram_pte_offset.wrapping_add(HS_CAPT_FIFO_ADDR_OFFSET);
                reg_fifo_addr = ACP_HS_RX_FIFOADDR;
                reg_fifo_size = ACP_HS_RX_FIFOSIZE;
                phy_addr = if (*chip).acp_rev >= ACP70_PCI_ID {
                    ACP7x_I2S_HS_RX_MEM_WINDOW_START
                } else {
                    I2S_HS_RX_MEM_WINDOW_START.wrapping_add((*stream).reg_offset)
                };
                writel(phy_addr, reg((*chip).base, ACP_HS_RX_RINGBUFADDR));
            }
        }
        _ => {
            dev_err(dev, b"Invalid dai id %x\n\0".as_ptr() as *const c_char, (*(*dai).driver).id);
            return -EINVAL;
        }
    }

    writel(DMA_SIZE, reg((*chip).base, reg_dma_size));
    writel(acp_fifo_addr, reg((*chip).base, reg_fifo_addr));
    writel(FIFO_SIZE, reg((*chip).base, reg_fifo_size));

    let mut ext_int_ctrl = readl(ACP_EXTERNAL_INTR_CNTL(chip, (*rsrc).irqp_used));
    ext_int_ctrl |= BIT(I2S_RX_THRESHOLD((*rsrc).offset))
        | BIT(BT_RX_THRESHOLD((*rsrc).offset))
        | BIT(I2S_TX_THRESHOLD((*rsrc).offset))
        | BIT(BT_TX_THRESHOLD((*rsrc).offset))
        | BIT(HS_RX_THRESHOLD((*rsrc).offset))
        | BIT(HS_TX_THRESHOLD((*rsrc).offset));

    writel(ext_int_ctrl, ACP_EXTERNAL_INTR_CNTL(chip, (*rsrc).irqp_used));
    0
}

#[no_mangle]
pub unsafe extern "C" fn restore_acp_i2s_params(
    substream: *mut snd_pcm_substream,
    chip: *mut acp_chip_info,
    stream: *mut acp_stream,
) -> c_int {
    let soc_runtime = snd_soc_substream_to_rtd(substream);
    let dai = snd_soc_rtd_to_cpu(soc_runtime, 0);
    let tdm_fmt;
    let reg_val;
    let fmt_reg;
    let mut val;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        tdm_fmt = (*chip).tdm_tx_fmt[((*stream).dai_id - 1) as usize];
        match (*stream).dai_id {
            id if id == I2S_BT_INSTANCE => {
                reg_val = ACP_BTTDM_ITER;
                fmt_reg = ACP_BTTDM_TXFRMT;
            }
            id if id == I2S_SP_INSTANCE => {
                reg_val = ACP_I2STDM_ITER;
                fmt_reg = ACP_I2STDM_TXFRMT;
            }
            id if id == I2S_HS_INSTANCE => {
                reg_val = ACP_HSTDM_ITER;
                fmt_reg = ACP_HSTDM_TXFRMT;
            }
            _ => {
                pr_err(b"Invalid dai id %x\n\0".as_ptr() as *const c_char, (*stream).dai_id);
                return -EINVAL;
            }
        }
        val = (*chip).xfer_tx_resolution[((*stream).dai_id - 1) as usize] << 3;
    } else {
        tdm_fmt = (*chip).tdm_rx_fmt[((*stream).dai_id - 1) as usize];
        match (*stream).dai_id {
            id if id == I2S_BT_INSTANCE => {
                reg_val = ACP_BTTDM_IRER;
                fmt_reg = ACP_BTTDM_RXFRMT;
            }
            id if id == I2S_SP_INSTANCE => {
                reg_val = ACP_I2STDM_IRER;
                fmt_reg = ACP_I2STDM_RXFRMT;
            }
            id if id == I2S_HS_INSTANCE => {
                reg_val = ACP_HSTDM_IRER;
                fmt_reg = ACP_HSTDM_RXFRMT;
            }
            _ => {
                pr_err(b"Invalid dai id %x\n\0".as_ptr() as *const c_char, (*stream).dai_id);
                return -EINVAL;
            }
        }
        val = (*chip).xfer_rx_resolution[((*stream).dai_id - 1) as usize] << 3;
    }
    writel(val, reg((*chip).base, reg_val));
    if (*chip).tdm_mode == TDM_ENABLE {
        writel(tdm_fmt, reg((*chip).base, fmt_reg));
        val = readl(reg((*chip).base, reg_val));
        writel(val | 0x2, reg((*chip).base, reg_val));
    }
    set_acp_i2s_dma_fifo(substream, dai)
}

unsafe fn acp_power_on(chip: *mut acp_chip_info) -> c_int {
    let acp_pgfsm_stat_reg;
    let acp_pgfsm_ctrl_reg;
    let base = (*chip).base;

    match (*chip).acp_rev {
        rev if rev == ACP_RN_PCI_ID => {
            acp_pgfsm_stat_reg = ACP_PGFSM_STATUS;
            acp_pgfsm_ctrl_reg = ACP_PGFSM_CONTROL;
        }
        rev if rev == ACP_RMB_PCI_ID => {
            acp_pgfsm_stat_reg = ACP6X_PGFSM_STATUS;
            acp_pgfsm_ctrl_reg = ACP6X_PGFSM_CONTROL;
        }
        rev if rev == ACP63_PCI_ID => {
            acp_pgfsm_stat_reg = ACP63_PGFSM_STATUS;
            acp_pgfsm_ctrl_reg = ACP63_PGFSM_CONTROL;
        }
        rev if rev == ACP70_PCI_ID || rev == ACP71_PCI_ID || rev == ACP72_PCI_ID => {
            acp_pgfsm_stat_reg = ACP70_PGFSM_STATUS;
            acp_pgfsm_ctrl_reg = ACP70_PGFSM_CONTROL;
        }
        _ => return -EINVAL,
    }

    let mut val = readl(reg(base, acp_pgfsm_stat_reg));
    if val == ACP_POWERED_ON {
        return 0;
    }

    if (val & ACP_PGFSM_STATUS_MASK) != ACP_POWER_ON_IN_PROGRESS {
        writel(ACP_PGFSM_CNTL_POWER_ON_MASK, reg(base, acp_pgfsm_ctrl_reg));
    }

    readl_poll_timeout(
        reg(base, acp_pgfsm_stat_reg),
        &mut val,
        (val == 0) as c_int,
        DELAY_US,
        ACP_TIMEOUT,
    )
}

unsafe fn acp_reset(base: *mut u8) -> c_int {
    let mut val: u32 = 0;

    writel(1, reg(base, ACP_SOFT_RESET));
    let ret = readl_poll_timeout(
        reg(base, ACP_SOFT_RESET),
        &mut val,
        ((val & ACP_SOFT_RST_DONE_MASK) != 0) as c_int,
        DELAY_US,
        ACP_TIMEOUT,
    );
    if ret != 0 {
        return ret;
    }

    writel(0, reg(base, ACP_SOFT_RESET));
    readl_poll_timeout(
        reg(base, ACP_SOFT_RESET),
        &mut val,
        (val == 0) as c_int,
        DELAY_US,
        ACP_TIMEOUT,
    )
}

#[no_mangle]
pub unsafe extern "C" fn acp_init(chip: *mut acp_chip_info) -> c_int {
    /* power on */
    let mut ret = acp_power_on(chip);
    if ret != 0 {
        pr_err(b"ACP power on failed\n\0".as_ptr() as *const c_char);
        return ret;
    }
    writel(0x01, reg((*chip).base, ACP_CONTROL));

    /* Reset */
    ret = acp_reset((*chip).base);
    if ret != 0 {
        pr_err(b"ACP reset failed\n\0".as_ptr() as *const c_char);
        return ret;
    }
    if (*chip).acp_rev >= ACP70_PCI_ID {
        writel(0, reg((*chip).base, ACP_ZSC_DSP_CTRL));
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn acp_deinit(chip: *mut acp_chip_info) -> c_int {
    /* Reset */
    let ret = acp_reset((*chip).base);
    if ret != 0 {
        return ret;
    }

    if (*chip).acp_rev < ACP70_PCI_ID {
        writel(0, reg((*chip).base, ACP_CONTROL));
    } else {
        writel(0x01, reg((*chip).base, ACP_ZSC_DSP_CTRL));
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn acp_machine_select(chip: *mut acp_chip_info) -> c_int {
    let mut mach: *mut snd_soc_acpi_mach;
    let size: c_int;
    let mut platform: c_int;

    if (*chip).flag == FLAG_AMD_LEGACY_ONLY_DMIC && (*chip).is_pdm_dev {
        platform = (*chip).acp_rev as c_int;
        (*chip).mach_dev = platform_device_register_data(
            (*chip).dev,
            b"acp-pdm-mach\0".as_ptr() as *const c_char,
            PLATFORM_DEVID_NONE,
            &platform as *const _ as *const c_void,
            mem::size_of_val(&platform),
        );
    } else {
        size = mem::size_of::<snd_soc_acpi_mach>() as c_int;
        mach = snd_soc_acpi_find_machine((*chip).machines);
        if mach.is_null() {
            dev_err(
                (*chip).dev,
                b"warning: No matching ASoC machine driver found\n\0".as_ptr() as *const c_char,
            );
            return -EINVAL;
        }
        (*mach).mach_params.subsystem_rev = (*chip).acp_rev;
        (*chip).mach_dev = platform_device_register_data(
            (*chip).dev,
            (*mach).drv_name,
            PLATFORM_DEVID_NONE,
            mach as *const c_void,
            size as usize,
        );
    }
    if IS_ERR((*chip).mach_dev as *const c_void) {
        dev_warn((*chip).dev, b"Unable to register Machine device\n\0".as_ptr() as *const c_char);
    }
    0
}

unsafe fn check_acp3x_config(chip: *mut acp_chip_info) {
    let val = readl(reg((*chip).base, ACP3X_PIN_CONFIG));
    match val {
        v if v == ACP_CONFIG_4 => {
            (*chip).is_i2s_config = true;
            (*chip).is_pdm_config = true;
        }
        _ => {
            (*chip).is_pdm_config = true;
        }
    }
}

unsafe fn check_acp6x_config(chip: *mut acp_chip_info) {
    let val = readl(reg((*chip).base, ACP_PIN_CONFIG));
    match val {
        v if v == ACP_CONFIG_4
            || v == ACP_CONFIG_5
            || v == ACP_CONFIG_6
            || v == ACP_CONFIG_7
            || v == ACP_CONFIG_8
            || v == ACP_CONFIG_11
            || v == ACP_CONFIG_14 =>
        {
            (*chip).is_pdm_config = true;
        }
        v if v == ACP_CONFIG_9 => {
            (*chip).is_i2s_config = true;
        }
        v if v == ACP_CONFIG_10 || v == ACP_CONFIG_12 || v == ACP_CONFIG_13 => {
            (*chip).is_i2s_config = true;
            (*chip).is_pdm_config = true;
        }
        _ => {}
    }
}

unsafe fn check_acp70_config(chip: *mut acp_chip_info) {
    let val = readl(reg((*chip).base, ACP_PIN_CONFIG));
    match val {
        v if v == ACP_CONFIG_4
            || v == ACP_CONFIG_5
            || v == ACP_CONFIG_6
            || v == ACP_CONFIG_7
            || v == ACP_CONFIG_8
            || v == ACP_CONFIG_11
            || v == ACP_CONFIG_14
            || v == ACP_CONFIG_17
            || v == ACP_CONFIG_18 =>
        {
            (*chip).is_pdm_config = true;
        }
        v if v == ACP_CONFIG_9 => {
            (*chip).is_i2s_config = true;
        }
        v if v == ACP_CONFIG_10
            || v == ACP_CONFIG_12
            || v == ACP_CONFIG_13
            || v == ACP_CONFIG_19
            || v == ACP_CONFIG_20 =>
        {
            (*chip).is_i2s_config = true;
            (*chip).is_pdm_config = true;
        }
        _ => {}
    }
}

#[no_mangle]
pub unsafe extern "C" fn check_acp_config(pci: *mut pci_dev, chip: *mut acp_chip_info) {
    let mut obj: *const acpi_object = ptr::null();
    let handle: acpi_handle;
    let mut dmic_status: acpi_integer = 0;
    let mut pdm_addr: u32 = 0;

    match (*chip).acp_rev {
        rev if rev == ACP_RN_PCI_ID => {
            pdm_addr = ACP_RENOIR_PDM_ADDR;
            check_acp3x_config(chip);
        }
        rev if rev == ACP_RMB_PCI_ID => {
            pdm_addr = ACP_REMBRANDT_PDM_ADDR;
            check_acp6x_config(chip);
        }
        rev if rev == ACP63_PCI_ID => {
            pdm_addr = ACP63_PDM_ADDR;
            check_acp6x_config(chip);
        }
        rev if rev == ACP70_PCI_ID || rev == ACP71_PCI_ID || rev == ACP72_PCI_ID => {
            pdm_addr = ACP70_PDM_ADDR;
            check_acp70_config(chip);
        }
        _ => {}
    }

    if (*chip).is_pdm_config {
        let pdm_dev = acpi_find_child_device(ACPI_COMPANION(&mut (*pci).dev), pdm_addr, 0);
        if !pdm_dev.is_null() {
            if acpi_dev_get_property(
                pdm_dev,
                b"acp-audio-device-type\0".as_ptr() as *const c_char,
                ACPI_TYPE_INTEGER,
                &mut obj,
            ) == 0
                && (*obj).integer.value == pdm_addr as u64
            {
                (*chip).is_pdm_dev = true;
            }
        }

        handle = ACPI_HANDLE(&mut (*pci).dev);
        let ret = acpi_evaluate_integer(
            handle,
            b"_WOV\0".as_ptr() as *const c_char,
            ptr::null_mut(),
            &mut dmic_status,
        );
        if !ACPI_FAILURE(ret) {
            (*chip).is_pdm_dev = dmic_status != 0;
        }
    }
}

#[no_mangle]
pub static mut acp31_common_hw_ops: MaybeUninit<snd_acp_hw_ops> = MaybeUninit::uninit();

#[no_mangle]
pub unsafe extern "C" fn acp31_hw_ops_init(chip: *mut acp_chip_info) -> c_int {
    memcpy(
        acp31_common_hw_ops.as_mut_ptr() as *mut c_void,
        &acp_common_hw_ops as *const _ as *const c_void,
        mem::size_of_val(&acp_common_hw_ops),
    );
    (*chip).acp_hw_ops = acp31_common_hw_ops.as_mut_ptr();

    0
}

#[no_mangle]
pub static mut acp6x_common_hw_ops: MaybeUninit<snd_acp_hw_ops> = MaybeUninit::uninit();

#[no_mangle]
pub unsafe extern "C" fn acp6x_hw_ops_init(chip: *mut acp_chip_info) -> c_int {
    memcpy(
        acp6x_common_hw_ops.as_mut_ptr() as *mut c_void,
        &acp_common_hw_ops as *const _ as *const c_void,
        mem::size_of_val(&acp_common_hw_ops),
    );
    (*chip).acp_hw_ops = acp6x_common_hw_ops.as_mut_ptr();

    0
}

#[no_mangle]
pub static mut acp63_common_hw_ops: MaybeUninit<snd_acp_hw_ops> = MaybeUninit::uninit();

#[no_mangle]
pub unsafe extern "C" fn acp63_hw_ops_init(chip: *mut acp_chip_info) -> c_int {
    memcpy(
        acp63_common_hw_ops.as_mut_ptr() as *mut c_void,
        &acp_common_hw_ops as *const _ as *const c_void,
        mem::size_of_val(&acp_common_hw_ops),
    );
    (*chip).acp_hw_ops = acp63_common_hw_ops.as_mut_ptr();

    0
}

#[no_mangle]
pub static mut acp70_common_hw_ops: MaybeUninit<snd_acp_hw_ops> = MaybeUninit::uninit();

#[no_mangle]
pub unsafe extern "C" fn acp70_hw_ops_init(chip: *mut acp_chip_info) -> c_int {
    memcpy(
        acp70_common_hw_ops.as_mut_ptr() as *mut c_void,
        &acp_common_hw_ops as *const _ as *const c_void,
        mem::size_of_val(&acp_common_hw_ops),
    );
    (*chip).acp_hw_ops = acp70_common_hw_ops.as_mut_ptr();

    0
}

// MODULE_DESCRIPTION("AMD ACP legacy common features");
// MODULE_LICENSE("Dual BSD/GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
