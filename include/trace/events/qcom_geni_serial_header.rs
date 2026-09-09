/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of trace/events/qcom_geni_serial.h.
// The Linux tracepoint and device definitions are supplied by external
// dependencies; this file preserves the generated event interfaces.

use core::ffi::c_char;

#[repr(C)]
pub struct Device {
    _private: [u8; 0],
}

pub type U8 = u8;
pub type U32 = u32;

#[repr(C)]
pub struct GeniSerialSetTermiosEntry {
    pub name: *mut c_char,
    pub baud: u32,
    pub bits_per_char: u32,
    pub tx_trans_cfg: U32,
    pub tx_parity_cfg: U32,
    pub rx_trans_cfg: U32,
    pub rx_parity_cfg: U32,
    pub stop_bit_len: U32,
}

#[repr(C)]
pub struct GeniSerialClkCfgEntry {
    pub name: *mut c_char,
    pub desired_rate: u32,
    pub clk_rate: usize,
    pub clk_div: u32,
    pub clk_idx: u32,
}

#[repr(C)]
pub struct GeniSerialIrqEntry {
    pub name: *mut c_char,
    pub m_irq: U32,
    pub s_irq: U32,
    pub dma_tx: U32,
    pub dma_rx: U32,
}

// __dynamic_array(u8, data, len) is represented by its allocated byte buffer.
#[repr(C)]
pub struct GeniSerialDataEntry {
    pub name: *mut c_char,
    pub data: *mut U8,
    pub data_len: usize,
}

#[repr(C)]
pub struct GeniSerialSetMctrlEntry {
    pub name: *mut c_char,
    pub mctrl: u32,
    pub uart_manual_rfr: U32,
}

#[repr(C)]
pub struct GeniSerialGetMctrlEntry {
    pub name: *mut c_char,
    pub mctrl: u32,
    pub geni_ios: U32,
}

extern "C" {
    pub fn geni_serial_set_termios(
        dev: *mut Device,
        baud: u32,
        bits_per_char: u32,
        tx_trans_cfg: U32,
        tx_parity_cfg: U32,
        rx_trans_cfg: U32,
        rx_parity_cfg: U32,
        stop_bit_len: U32,
    );

    pub fn geni_serial_clk_cfg(
        dev: *mut Device,
        desired_rate: u32,
        clk_rate: usize,
        clk_div: u32,
        clk_idx: u32,
    );

    pub fn geni_serial_irq(
        dev: *mut Device,
        m_irq: U32,
        s_irq: U32,
        dma_tx: U32,
        dma_rx: U32,
    );

    pub fn geni_serial_tx_data(dev: *mut Device, buf: *const U8, len: u32);
    pub fn geni_serial_rx_data(dev: *mut Device, buf: *const U8, len: u32);

    pub fn geni_serial_set_mctrl(
        dev: *mut Device,
        mctrl: u32,
        uart_manual_rfr: U32,
    );

    pub fn geni_serial_get_mctrl(dev: *mut Device, mctrl: u32, geni_ios: U32);
}

// TP_fast_assign for geni_serial_data copies `len` bytes from `buf` into the
// dynamic `data` array before formatting it as hexadecimal output.
// Print formats preserved from the source:
// geni_serial_set_termios: "%s: baud=%u bpc=%u tx_trans=0x%08x tx_par=0x%08x rx_trans=0x%08x rx_par=0x%08x stop=%u"
// geni_serial_clk_cfg: "%s: desired_rate=%u clk_rate=%lu clk_div=%u clk_idx=%u"
// geni_serial_irq: "%s: m_irq=0x%08x s_irq=0x%08x dma_tx=0x%08x dma_rx=0x%08x"
// geni_serial_data: "%s: len=%u data=%s"
// geni_serial_set_mctrl: "%s: mctrl=0x%04x uart_manual_rfr=0x%08x"
// geni_serial_get_mctrl: "%s: mctrl=0x%04x geni_ios=0x%08x"

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
