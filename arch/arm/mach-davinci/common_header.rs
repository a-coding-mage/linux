/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Header for code common to all DaVinci machines.
 *
 * Author: Kevin Hilman, MontaVista Software, Inc. <source@mvista.com>
 *
 * 2007 (c) MontaVista Software, Inc.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external, as in the original C header.

pub const DAVINCI_INTC_START: usize = NR_IRQS;
#[inline]
pub const fn DAVINCI_INTC_IRQ(irqnum: usize) -> usize {
    DAVINCI_INTC_START + irqnum
}

pub struct davinci_gpio_controller;

/*
 * SoC info passed into common davinci modules.
 *
 * Base addresses in this structure should be physical and not virtual.
 * Modules that take such base addresses, should internally ioremap() them to
 * use.
 */
#[repr(C)]
pub struct davinci_soc_info {
    pub io_desc: *mut map_desc,
    pub io_desc_num: c_ulong,
    pub cpu_id: u32,
    pub jtag_id: u32,
    pub jtag_id_reg: u32,
    pub ids: *mut davinci_id,
    pub ids_num: c_ulong,
    pub pinmux_base: u32,
    pub pinmux_pins: *const mux_config,
    pub pinmux_pins_num: c_ulong,
    pub gpio_type: c_int,
    pub gpio_base: u32,
    pub gpio_num: c_uint,
    pub gpio_irq: c_uint,
    pub gpio_unbanked: c_uint,
    pub sram_dma: dma_addr_t,
    pub sram_len: c_uint,
}

extern "C" {
    pub static mut davinci_soc_info: davinci_soc_info;

    pub fn davinci_common_init(soc_info: *const davinci_soc_info);
    pub fn davinci_init_late();

    // CONFIG_SUSPEND selects the external implementation in the original
    // build; otherwise the header supplies the inline zero-return stub.
    #[cfg(feature = "CONFIG_SUSPEND")]
    pub fn davinci_pm_init() -> c_int;

    pub fn pdata_quirks_init();
}

#[cfg(not(feature = "CONFIG_SUSPEND"))]
#[inline]
pub fn davinci_pm_init() -> c_int {
    0
}

pub const SRAM_SIZE: usize = SZ_128K;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
