/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ilsel_source_t {
    ILSEL_NONE = 0,
    ILSEL_LAN,
    ILSEL_USBH_I,
    ILSEL_USBH_S,
    ILSEL_USBH_V,
    ILSEL_RTC,
    ILSEL_USBP_I,
    ILSEL_USBP_S,
    ILSEL_USBP_V,
    ILSEL_KEY,

    /*
     * ILSEL Aliases - corner cases for interleaved level tables.
     *
     * Someone thought this was a good idea and less hassle than
     * demuxing a shared vector, really.
     */

    /* ILSEL0 and 2 */
    ILSEL_FPGA0,
    ILSEL_FPGA1,
    ILSEL_EX1,
    ILSEL_EX2,
    ILSEL_EX3,
    ILSEL_EX4,

    /* ILSEL1 and 3 */
    ILSEL_FPGA2 = ILSEL_FPGA0,
    ILSEL_FPGA3 = ILSEL_FPGA1,
    ILSEL_EX5 = ILSEL_EX1,
    ILSEL_EX6 = ILSEL_EX2,
    ILSEL_EX7 = ILSEL_EX3,
    ILSEL_EX8 = ILSEL_EX4,
}

/* arch/sh/boards/renesas/x3proto/ilsel.c */
extern "C" {
    pub fn ilsel_enable(set: ilsel_source_t) -> i32;
    pub fn ilsel_enable_fixed(set: ilsel_source_t, level: u32) -> i32;
    pub fn ilsel_disable(irq: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
