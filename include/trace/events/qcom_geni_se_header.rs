/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// Translated from the C trace event header.  The Linux tracepoint machinery,
// `geni_se`, register offsets, and `readl` are supplied by other dependencies.

#[repr(C)]
pub struct geni_se {
    pub base: *mut core::ffi::c_void,
    pub dev: *mut core::ffi::c_void,
}

unsafe extern "C" {
    pub fn readl(addr: *const core::ffi::c_void) -> u32;
}

#[repr(C)]
pub struct GeniSeRegs {
    pub geni_se_name: *const core::ffi::c_char,
    pub geni_se_m_cmd0: u32,
    pub geni_se_m_irq_status: u32,
    pub geni_se_s_cmd0: u32,
    pub geni_se_s_irq_status: u32,
    pub geni_se_status: u32,
    pub geni_se_ios: u32,
    pub geni_se_m_cmd_ctrl: u32,
    pub geni_se_m_cmd_err: u32,
    pub geni_se_m_fw_err: u32,
    pub geni_se_tx_fifo_status: u32,
    pub geni_se_rx_fifo_status: u32,
    pub geni_se_tx_watermark: u32,
    pub geni_se_rx_watermark: u32,
    pub geni_se_rx_watermark_rfr: u32,
    pub geni_se_m_gp_length: u32,
    pub geni_se_s_gp_length: u32,
    pub geni_se_dma_tx_irq: u32,
    pub geni_se_dma_rx_irq: u32,
    pub geni_se_dma_tx_irq_en: u32,
    pub geni_se_dma_rx_irq_en: u32,
    pub geni_se_dma_rx_len: u32,
    pub geni_se_dma_rx_len_in: u32,
    pub geni_se_dma_tx_len: u32,
    pub geni_se_dma_tx_len_in: u32,
    pub geni_se_dma_tx_ptr_l: u32,
    pub geni_se_dma_tx_ptr_h: u32,
    pub geni_se_dma_rx_ptr_l: u32,
    pub geni_se_dma_rx_ptr_h: u32,
    pub geni_se_dma_tx_attr: u32,
    pub geni_se_dma_tx_max_burst: u32,
    pub geni_se_dma_rx_attr: u32,
    pub geni_se_dma_rx_max_burst: u32,
    pub geni_se_dma_if_en: u32,
    pub geni_se_dma_if_en_ro: u32,
    pub geni_se_dma_general_cfg: u32,
    pub geni_se_dma_qsb_trans_cfg: u32,
    pub geni_se_dma_dbg: u32,
    pub geni_se_m_irq_en: u32,
    pub geni_se_s_irq_en: u32,
    pub geni_se_gsi_event_en: u32,
    pub geni_se_irq_en: u32,
    pub geni_se_ser_m_clk_cfg: u32,
    pub geni_se_ser_s_clk_cfg: u32,
    pub geni_se_general_cfg: u32,
    pub geni_se_output_ctrl: u32,
    pub geni_se_clk_ctrl_ro: u32,
    pub geni_se_fifo_if_disable: u32,
    pub geni_se_fw_multilock_msa: u32,
    pub geni_se_clk_sel: u32,
}

// TP_printk format from the original tracepoint.
pub const GENI_SE_REGS_PRINTK: &str = "%s: m_cmd0=0x%08x m_irq_status=0x%08x s_cmd0=0x%08x s_irq_status=0x%08x geni_status=0x%08x geni_ios=0x%08x m_cmd_ctrl=0x%08x m_cmd_err=0x%08x m_fw_err=0x%08x tx_fifo_sts=0x%08x rx_fifo_sts=0x%08x tx_watermark=0x%08x rx_watermark=0x%08x rx_watermark_rfr=0x%08x m_gp_length=0x%08x s_gp_length=0x%08x dma_tx_irq=0x%08x dma_rx_irq=0x%08x dma_tx_irq_en=0x%08x dma_rx_irq_en=0x%08x dma_rx_len=0x%08x dma_rx_len_in=0x%08x dma_tx_len=0x%08x dma_tx_len_in=0x%08x dma_tx_ptr_l=0x%08x dma_tx_ptr_h=0x%08x dma_rx_ptr_l=0x%08x dma_rx_ptr_h=0x%08x dma_tx_attr=0x%08x dma_tx_max_burst=0x%08x dma_rx_attr=0x%08x dma_rx_max_burst=0x%08x dma_if_en=0x%08x dma_if_en_ro=0x%08x dma_general_cfg=0x%08x dma_qsb_trans_cfg=0x%08x dma_dbg=0x%08x m_irq_en=0x%08x s_irq_en=0x%08x gsi_event_en=0x%08x se_irq_en=0x%08x ser_m_clk_cfg=0x%08x ser_s_clk_cfg=0x%08x general_cfg=0x%08x output_ctrl=0x%08x clk_ctrl_ro=0x%08x fifo_if_dis=0x%08x fw_multilock_msa=0x%08x clk_sel=0x%08x";

// The following register names are intentionally unresolved: they are supplied
// by the translated GENI SE dependency, just as in the original header.
macro_rules! geni_se_read_regs {
    ($se:expr, $($field:ident => $reg:ident),+ $(,)?) => {{
        $( $field: unsafe { readl((*$se).base.add($reg as usize) as *const _) }, )+
    }};
}

// TRACE_EVENT(geni_se_regs, ...): the event's field layout and fast assignment
// are represented by GeniSeRegs and this declaration-only hook.
pub unsafe fn geni_se_regs(se: *mut geni_se, name: *const core::ffi::c_char) -> GeniSeRegs {
    GeniSeRegs {
        geni_se_name: name,
        geni_se_m_cmd0: readl((*se).base.add(SE_GENI_M_CMD0 as usize)),
        geni_se_m_irq_status: readl((*se).base.add(SE_GENI_M_IRQ_STATUS as usize)),
        geni_se_s_cmd0: readl((*se).base.add(SE_GENI_S_CMD0 as usize)),
        geni_se_s_irq_status: readl((*se).base.add(SE_GENI_S_IRQ_STATUS as usize)),
        geni_se_status: readl((*se).base.add(SE_GENI_STATUS as usize)),
        geni_se_ios: readl((*se).base.add(SE_GENI_IOS as usize)),
        geni_se_m_cmd_ctrl: readl((*se).base.add(SE_GENI_M_CMD_CTRL_REG as usize)),
        geni_se_m_cmd_err: readl((*se).base.add(M_CMD_ERR_STATUS as usize)),
        geni_se_m_fw_err: readl((*se).base.add(M_FW_ERR_STATUS as usize)),
        geni_se_tx_fifo_status: readl((*se).base.add(SE_GENI_TX_FIFO_STATUS as usize)),
        geni_se_rx_fifo_status: readl((*se).base.add(SE_GENI_RX_FIFO_STATUS as usize)),
        geni_se_tx_watermark: readl((*se).base.add(SE_GENI_TX_WATERMARK_REG as usize)),
        geni_se_rx_watermark: readl((*se).base.add(SE_GENI_RX_WATERMARK_REG as usize)),
        geni_se_rx_watermark_rfr: readl((*se).base.add(SE_GENI_RX_RFR_WATERMARK_REG as usize)),
        geni_se_m_gp_length: readl((*se).base.add(SE_GENI_M_GP_LENGTH as usize)),
        geni_se_s_gp_length: readl((*se).base.add(SE_GENI_S_GP_LENGTH as usize)),
        geni_se_dma_tx_irq: readl((*se).base.add(SE_DMA_TX_IRQ_STAT as usize)),
        geni_se_dma_rx_irq: readl((*se).base.add(SE_DMA_RX_IRQ_STAT as usize)),
        geni_se_dma_tx_irq_en: readl((*se).base.add(SE_DMA_TX_IRQ_EN as usize)),
        geni_se_dma_rx_irq_en: readl((*se).base.add(SE_DMA_RX_IRQ_EN as usize)),
        geni_se_dma_rx_len: readl((*se).base.add(SE_DMA_RX_LEN as usize)),
        geni_se_dma_rx_len_in: readl((*se).base.add(SE_DMA_RX_LEN_IN as usize)),
        geni_se_dma_tx_len: readl((*se).base.add(SE_DMA_TX_LEN as usize)),
        geni_se_dma_tx_len_in: readl((*se).base.add(SE_DMA_TX_LEN_IN as usize)),
        geni_se_dma_tx_ptr_l: readl((*se).base.add(SE_DMA_TX_PTR_L as usize)),
        geni_se_dma_tx_ptr_h: readl((*se).base.add(SE_DMA_TX_PTR_H as usize)),
        geni_se_dma_rx_ptr_l: readl((*se).base.add(SE_DMA_RX_PTR_L as usize)),
        geni_se_dma_rx_ptr_h: readl((*se).base.add(SE_DMA_RX_PTR_H as usize)),
        geni_se_dma_tx_attr: readl((*se).base.add(SE_DMA_TX_ATTR as usize)),
        geni_se_dma_tx_max_burst: readl((*se).base.add(SE_DMA_TX_MAX_BURST as usize)),
        geni_se_dma_rx_attr: readl((*se).base.add(SE_DMA_RX_ATTR as usize)),
        geni_se_dma_rx_max_burst: readl((*se).base.add(SE_DMA_RX_MAX_BURST as usize)),
        geni_se_dma_if_en: readl((*se).base.add(SE_DMA_IF_EN as usize)),
        geni_se_dma_if_en_ro: readl((*se).base.add(DMA_IF_EN_RO as usize)),
        geni_se_dma_general_cfg: readl((*se).base.add(DMA_GENERAL_CFG as usize)),
        geni_se_dma_qsb_trans_cfg: readl((*se).base.add(SE_DMA_QSB_TRANS_CFG as usize)),
        geni_se_dma_dbg: readl((*se).base.add(SE_DMA_DEBUG_REG0 as usize)),
        geni_se_m_irq_en: readl((*se).base.add(SE_GENI_M_IRQ_EN as usize)),
        geni_se_s_irq_en: readl((*se).base.add(SE_GENI_S_IRQ_EN as usize)),
        geni_se_gsi_event_en: readl((*se).base.add(SE_GSI_EVENT_EN as usize)),
        geni_se_irq_en: readl((*se).base.add(SE_IRQ_EN as usize)),
        geni_se_ser_m_clk_cfg: readl((*se).base.add(GENI_SER_M_CLK_CFG as usize)),
        geni_se_ser_s_clk_cfg: readl((*se).base.add(GENI_SER_S_CLK_CFG as usize)),
        geni_se_general_cfg: readl((*se).base.add(GENI_GENERAL_CFG as usize)),
        geni_se_output_ctrl: readl((*se).base.add(GENI_OUTPUT_CTRL as usize)),
        geni_se_clk_ctrl_ro: readl((*se).base.add(GENI_CLK_CTRL_RO as usize)),
        geni_se_fifo_if_disable: readl((*se).base.add(GENI_IF_DISABLE_RO as usize)),
        geni_se_fw_multilock_msa: readl((*se).base.add(GENI_FW_MULTILOCK_MSA_RO as usize)),
        geni_se_clk_sel: readl((*se).base.add(SE_GENI_CLK_SEL as usize)),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
