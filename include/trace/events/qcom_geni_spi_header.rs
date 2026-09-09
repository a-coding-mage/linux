/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the Linux tracepoint header qcom_geni_spi.h.
// The C tracepoint machinery and its build-time multi-read condition are
// supplied by the surrounding kernel translation.

use core::ffi::c_char;

pub type u8 = core::ffi::c_uchar;
pub type u32 = core::ffi::c_uint;
pub type c_ulong = core::ffi::c_ulong;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn dev_name(dev: *const device) -> *const c_char;
}

#[repr(C)]
pub struct geni_spi_setup_params_entry {
    pub name: *const c_char,
    pub cs: u8,
    pub mode: u32,
    pub mode_changed: u32,
    pub cs_changed: bool,
}

#[repr(C)]
pub struct geni_spi_clk_cfg_entry {
    pub name: *const c_char,
    pub req_hz: c_ulong,
    pub sclk_hz: c_ulong,
    pub clk_idx: core::ffi::c_uint,
    pub clk_div: core::ffi::c_uint,
    pub bpw: core::ffi::c_uint,
}

#[repr(C)]
pub struct geni_spi_transfer_entry {
    pub name: *const c_char,
    pub len: core::ffi::c_uint,
    pub m_cmd: u32,
}

#[repr(C)]
pub struct geni_spi_irq_entry {
    pub name: *const c_char,
    pub m_irq: u32,
    pub dma_tx: u32,
    pub dma_rx: u32,
}

#[inline]
pub unsafe fn geni_spi_setup_params_assign(
    entry: *mut geni_spi_setup_params_entry,
    dev: *mut device,
    cs: u8,
    mode: u32,
    mode_changed: u32,
    cs_changed: bool,
) {
    (*entry).name = dev_name(dev);
    (*entry).cs = cs;
    (*entry).mode = mode;
    (*entry).mode_changed = mode_changed;
    (*entry).cs_changed = cs_changed;
}

#[inline]
pub unsafe fn geni_spi_clk_cfg_assign(
    entry: *mut geni_spi_clk_cfg_entry,
    dev: *mut device,
    req_hz: c_ulong,
    sclk_hz: c_ulong,
    clk_idx: core::ffi::c_uint,
    clk_div: core::ffi::c_uint,
    bpw: core::ffi::c_uint,
) {
    (*entry).name = dev_name(dev);
    (*entry).req_hz = req_hz;
    (*entry).sclk_hz = sclk_hz;
    (*entry).clk_idx = clk_idx;
    (*entry).clk_div = clk_div;
    (*entry).bpw = bpw;
}

#[inline]
pub unsafe fn geni_spi_transfer_assign(
    entry: *mut geni_spi_transfer_entry,
    dev: *mut device,
    len: core::ffi::c_uint,
    m_cmd: u32,
) {
    (*entry).name = dev_name(dev);
    (*entry).len = len;
    (*entry).m_cmd = m_cmd;
}

#[inline]
pub unsafe fn geni_spi_irq_assign(
    entry: *mut geni_spi_irq_entry,
    dev: *mut device,
    m_irq: u32,
    dma_tx: u32,
    dma_rx: u32,
) {
    (*entry).name = dev_name(dev);
    (*entry).m_irq = m_irq;
    (*entry).dma_tx = dma_tx;
    (*entry).dma_rx = dma_rx;
}

// TP_printk formats:
// geni_spi_setup_params: "%s: cs=%u mode=0x%08x mode_changed=0x%08x cs_changed=%d"
// geni_spi_clk_cfg: "%s: req_hz=%lu sclk_hz=%lu clk_idx=%u clk_div=%u bpw=%u"
// geni_spi_transfer: "%s: len=%u m_cmd=0x%08x"
// geni_spi_irq: "%s: m_irq=0x%08x dma_tx=0x%08x dma_rx=0x%08x"

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
