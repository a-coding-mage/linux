/* SPDX-License-Identifier: GPL-2.0 */

// Translation of TRACE_SYSTEM qcom_geni_i2c.
// The Linux tracepoint include and header guard are C preprocessor constructs;
// their dependency/conditional intent is retained here as comments.

use core::ffi::c_char;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GeniI2cBusSetupEntry {
    pub name: *const c_char,
    pub clk_freq: u32,
    pub clk_div: u8,
    pub t_high_cnt: u8,
    pub t_low_cnt: u8,
    pub t_cycle_cnt: u8,
}

#[repr(C)]
pub struct GeniI2cIrqEntry {
    pub name: *const c_char,
    pub m_stat: u32,
    pub rx_st: u32,
    pub dm_tx_st: u32,
    pub dm_rx_st: u32,
}

#[repr(C)]
pub struct GeniI2cErrEntry {
    pub name: *const c_char,
    pub err: core::ffi::c_int,
    pub msg: *const c_char,
}

extern "C" {
    // TRACE_EVENT(geni_i2c_bus_setup)
    // TP_printk: "%s: clk_freq=%u clk_div=%u t_high=%u t_low=%u t_cycle=%u"
    pub fn geni_i2c_bus_setup(
        dev: *mut device,
        clk_freq: u32,
        clk_div: u8,
        t_high_cnt: u8,
        t_low_cnt: u8,
        t_cycle_cnt: u8,
    );

    // TRACE_EVENT(geni_i2c_irq)
    // TP_printk: "%s: m_stat=0x%08x rx_st=0x%08x dm_tx=0x%08x dm_rx=0x%08x"
    pub fn geni_i2c_irq(
        dev: *mut device,
        m_stat: u32,
        rx_st: u32,
        dm_tx_st: u32,
        dm_rx_st: u32,
    );

    // TRACE_EVENT(geni_i2c_err)
    // TP_printk: "%s: err=%d msg=%s"
    pub fn geni_i2c_err(
        dev: *mut device,
        err: core::ffi::c_int,
        msg: *const c_char,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
