/*
 * arch/arm/plat-orion/include/plat/mpp.h
 *
 * Marvell Orion SoC MPP handling.
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2.  This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

/* C header guard: __PLAT_MPP_H */

macro_rules! MPP_NUM {
    ($x:expr) => {
        (($x) & 0xff)
    };
}

macro_rules! MPP_SEL {
    ($x:expr) => {
        ((($x) >> 8) & 0xf)
    };
}

/* This is the generic MPP macro, without any variant information.
   Each machine architecture is expected to extend this with further
   bit fields indicating which MPP configurations are valid for a
   specific variant. */

macro_rules! GENERIC_MPP {
    ($num:expr, $sel:expr, $input:expr, $output:expr) => {
        /* MPP number */ (($num) & 0xff)
            /* MPP select value */ | ((($sel) & 0xf) << 8)
            /* may be input signal */ | ((($input != 0) as u32) << 12)
            /* may be output signal */ | ((($output != 0) as u32) << 13)
    };
}

const MPP_INPUT_MASK: u32 = GENERIC_MPP!(0, 0x0, 1, 0);
const MPP_OUTPUT_MASK: u32 = GENERIC_MPP!(0, 0x0, 0, 1);

/* C __init annotation retained as declaration intent. */
unsafe extern "C" {
    pub fn orion_mpp_conf(
        mpp_list: *mut u32,
        variant_mask: u32,
        mpp_max: u32,
        dev_bus: *mut core::ffi::c_void,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
