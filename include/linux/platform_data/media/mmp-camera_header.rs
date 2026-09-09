/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Information for the Marvell Armada MMP camera
 */

// Dependency supplied by the media/v4l2-mediabus interface.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dphy3_algo {
    DPHY3_ALGO_DEFAULT = 0,
    DPHY3_ALGO_PXA910,
    DPHY3_ALGO_PXA2128,
}

#[repr(C)]
pub struct mmp_camera_platform_data {
    pub bus_type: v4l2_mbus_type,
    pub mclk_src: i32, /* which clock source the MCLK derives from */
    pub mclk_div: i32, /* Clock Divider Value for MCLK */
    /*
     * MIPI support
     */
    pub dphy: [i32; 3], /* DPHY: CSI2_DPHY3, CSI2_DPHY5, CSI2_DPHY6 */
    pub dphy3_algo: dphy3_algo, /* algos for calculate CSI2_DPHY3 */
    pub lane: i32, /* ccic used lane number; 0 means DVP mode */
    pub lane_clk: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
