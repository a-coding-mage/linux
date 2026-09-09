/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Board-specific data used to set up AT73c213 audio DAC driver.
 */

/**
 * at73c213_board_info - how the external DAC is wired to the device.
 *
 * @ssc_id: SSC platform_driver id the DAC shall use to stream the audio.
 * @dac_clk: the external clock used to provide master clock to the DAC.
 * @shortname: a short discription for the DAC, seen by userspace tools.
 *
 * This struct contains the configuration of the hardware connection to the
 * external DAC. The DAC needs a master clock and a I2S audio stream. It also
 * provides a name which is used to identify it in userspace tools.
 */
#[repr(C)]
pub struct at73c213_board_info {
    pub ssc_id: i32,
    // `struct clk` is supplied by an external dependency.
    pub dac_clk: *mut clk,
    pub shortname: [i8; 32],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
