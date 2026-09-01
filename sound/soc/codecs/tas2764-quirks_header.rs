// SPDX-License-Identifier: GPL-2.0-only

// C dependencies removed from executable Rust:
// #include <linux/regmap.h>
// #include "tas2764.h"

/* Bitmask of enabled Apple quirks */
pub const ENABLED_APPLE_QUIRKS: u32 = 0x3f;

/*
 * Disable noise gate and flip down reserved bit in NS_CFG0
 */
pub const TAS2764_NOISE_GATE_DISABLE: u32 = 1u32 << 0;

pub const tas2764_noise_gate_dis_seq: [reg_sequence; 1] = [
    reg_sequence {
        reg: TAS2764_REG(0x0, 0x35),
        def: 0xb0,
        delay_us: 0,
    },
];

/*
 * CONV_VBAT_PVDD_MODE=1
 */
pub const TAS2764_CONV_VBAT_PVDD_MODE: u32 = 1u32 << 1;

pub const tas2764_conv_vbat_pvdd_mode_seq: [reg_sequence; 1] = [
    reg_sequence {
        reg: TAS2764_REG(0x0, 0x6b),
        def: 0x41,
        delay_us: 0,
    },
];

/*
 * Reset of DAC modulator when DSP is OFF
 */
pub const TAS2764_DMOD_RST: u32 = 1u32 << 2;

pub const tas2764_dmod_rst_seq: [reg_sequence; 1] = [
    reg_sequence {
        reg: TAS2764_REG(0x0, 0x76),
        def: 0x0,
        delay_us: 0,
    },
];

/*
 * Unknown 0x133/0x137 writes (maybe TDM related)
 */
pub const TAS2764_UNK_SEQ0: u32 = 1u32 << 3;

pub const tas2764_unk_seq0: [reg_sequence; 2] = [
    reg_sequence {
        reg: TAS2764_REG(0x1, 0x33),
        def: 0x80,
        delay_us: 0,
    },
    reg_sequence {
        reg: TAS2764_REG(0x1, 0x37),
        def: 0x3a,
        delay_us: 0,
    },
];

/*
 * Unknown 0x614 - 0x61f writes
 */
pub const TAS2764_APPLE_UNK_SEQ1: u32 = 1u32 << 4;

pub const tas2764_unk_seq1: [reg_sequence; 12] = [
    reg_sequence {
        reg: TAS2764_REG(0x6, 0x14),
        def: 0x0,
        delay_us: 0,
    },
    reg_sequence {
        reg: TAS2764_REG(0x6, 0x15),
        def: 0x13,
        delay_us: 0,
    },
    reg_sequence {
        reg: TAS2764_REG(0x6, 0x16),
        def: 0x52,
        delay_us: 0,
    },
    reg_sequence {
        reg: TAS2764_REG(0x6, 0x17),
        def: 0x0,
        delay_us: 0,
    },
    reg_sequence {
        reg: TAS2764_REG(0x6, 0x18),
        def: 0xe4,
        delay_us: 0,
    },
    reg_sequence {
        reg: TAS2764_REG(0x6, 0x19),
        def: 0xc,
        delay_us: 0,
    },
    reg_sequence {
        reg: TAS2764_REG(0x6, 0x16),
        def: 0xaa,
        delay_us: 0,
    },
    reg_sequence {
        reg: TAS2764_REG(0x6, 0x1b),
        def: 0x0,
        delay_us: 0,
    },
    reg_sequence {
        reg: TAS2764_REG(0x6, 0x1c),
        def: 0x12,
        delay_us: 0,
    },
    reg_sequence {
        reg: TAS2764_REG(0x6, 0x1d),
        def: 0xa0,
        delay_us: 0,
    },
    reg_sequence {
        reg: TAS2764_REG(0x6, 0x1e),
        def: 0xd8,
        delay_us: 0,
    },
    reg_sequence {
        reg: TAS2764_REG(0x6, 0x1f),
        def: 0x0,
        delay_us: 0,
    },
];

/*
 * Unknown writes in the 0xfd page (with secondary paging inside)
 */
pub const TAS2764_APPLE_UNK_SEQ2: u32 = 1u32 << 5;

pub const tas2764_unk_seq2: [reg_sequence; 4] = [
    reg_sequence {
        reg: TAS2764_REG(0xfd, 0x0d),
        def: 0xd,
        delay_us: 0,
    },
    reg_sequence {
        reg: TAS2764_REG(0xfd, 0x6c),
        def: 0x2,
        delay_us: 0,
    },
    reg_sequence {
        reg: TAS2764_REG(0xfd, 0x6d),
        def: 0xf,
        delay_us: 0,
    },
    reg_sequence {
        reg: TAS2764_REG(0xfd, 0x0d),
        def: 0x0,
        delay_us: 0,
    },
];

/*
 * Disable 'Thermal Threshold 1'
 */
pub const TAS2764_THERMAL_TH1_DISABLE: u32 = 1u32 << 6;

pub const tas2764_thermal_th1_dis_seq: [reg_sequence; 1] = [
    reg_sequence {
        reg: TAS2764_REG(0x1, 0x47),
        def: 0x2,
        delay_us: 0,
    },
];

/*
 * Imitate Apple's shutdown dance
 */
pub const TAS2764_SHUTDOWN_DANCE: u32 = 1u32 << 7;

pub const tas2764_shutdown_dance_init_seq: [reg_sequence; 1] = [
    /*
     * SDZ_MODE=01 (immediate)
     *
     * We want the shutdown to happen under the influence of
     * the magic writes in the 0xfdXX region, so make sure
     * the shutdown is immediate and there's no grace period
     * followed by the codec part.
     */
    reg_sequence {
        reg: TAS2764_REG(0x0, 0x7),
        def: 0x60,
        delay_us: 0,
    },
];

pub const tas2764_pre_shutdown_seq: [reg_sequence; 3] = [
    reg_sequence {
        reg: TAS2764_REG(0xfd, 0x0d),
        def: 0xd,
        delay_us: 0,
    }, /* switch hidden page */
    reg_sequence {
        reg: TAS2764_REG(0xfd, 0x64),
        def: 0x4,
        delay_us: 0,
    }, /* do write (unknown semantics) */
    reg_sequence {
        reg: TAS2764_REG(0xfd, 0x0d),
        def: 0x0,
        delay_us: 0,
    }, /* switch hidden page back */
];

pub const tas2764_post_shutdown_seq: [reg_sequence; 3] = [
    reg_sequence {
        reg: TAS2764_REG(0xfd, 0x0d),
        def: 0xd,
        delay_us: 0,
    },
    reg_sequence {
        reg: TAS2764_REG(0xfd, 0x64),
        def: 0x0,
        delay_us: 0,
    }, /* revert write from pre sequence */
    reg_sequence {
        reg: TAS2764_REG(0xfd, 0x0d),
        def: 0x0,
        delay_us: 0,
    },
];

pub unsafe fn tas2764_do_quirky_pwr_ctrl_change(
    tas2764: *mut tas2764_priv,
    target: core::ffi::c_uint,
) -> core::ffi::c_int {
    let curr: core::ffi::c_uint;
    let mut ret: core::ffi::c_int;

    curr = snd_soc_component_read_field(
        (*tas2764).component,
        TAS2764_PWR_CTRL,
        TAS2764_PWR_CTRL_MASK,
    );

    if target == curr {
        return 0;
    }

    /* Handle power state transition to shutdown */
    if target == TAS2764_PWR_CTRL_SHUTDOWN
        && (curr == TAS2764_PWR_CTRL_MUTE || curr == TAS2764_PWR_CTRL_ACTIVE)
    {
        ret = regmap_multi_reg_write(
            (*tas2764).regmap,
            tas2764_pre_shutdown_seq.as_ptr(),
            tas2764_pre_shutdown_seq.len() as core::ffi::c_int,
        );
        if ret == 0 {
            ret = snd_soc_component_update_bits(
                (*tas2764).component,
                TAS2764_PWR_CTRL,
                TAS2764_PWR_CTRL_MASK,
                TAS2764_PWR_CTRL_SHUTDOWN,
            );
        }
        if ret == 0 {
            ret = regmap_multi_reg_write(
                (*tas2764).regmap,
                tas2764_post_shutdown_seq.as_ptr(),
                tas2764_post_shutdown_seq.len() as core::ffi::c_int,
            );
        }
    }

    ret = snd_soc_component_update_bits(
        (*tas2764).component,
        TAS2764_PWR_CTRL,
        TAS2764_PWR_CTRL_MASK,
        target,
    );

    ret
}

/*
 * Via devicetree (TODO):
 *  - switch from spread spectrum to class-D switching
 *  - disable edge control
 *  - set BOP settings (the BOP config bits *and* BOP_SRC)
 */

/*
 * Other setup TODOs:
 *  - DVC ramp rate
 */

#[repr(C)]
pub struct tas2764_quirk_init_sequence {
    pub seq: *const reg_sequence,
    pub len: core::ffi::c_int,
}

pub const tas2764_quirk_init_sequences: [tas2764_quirk_init_sequence; 8] = [
    tas2764_quirk_init_sequence {
        seq: tas2764_noise_gate_dis_seq.as_ptr(),
        len: tas2764_noise_gate_dis_seq.len() as core::ffi::c_int,
    },
    tas2764_quirk_init_sequence {
        seq: tas2764_dmod_rst_seq.as_ptr(),
        len: tas2764_dmod_rst_seq.len() as core::ffi::c_int,
    },
    tas2764_quirk_init_sequence {
        seq: tas2764_conv_vbat_pvdd_mode_seq.as_ptr(),
        len: tas2764_conv_vbat_pvdd_mode_seq.len() as core::ffi::c_int,
    },
    tas2764_quirk_init_sequence {
        seq: tas2764_unk_seq0.as_ptr(),
        len: tas2764_unk_seq0.len() as core::ffi::c_int,
    },
    tas2764_quirk_init_sequence {
        seq: tas2764_unk_seq1.as_ptr(),
        len: tas2764_unk_seq1.len() as core::ffi::c_int,
    },
    tas2764_quirk_init_sequence {
        seq: tas2764_unk_seq2.as_ptr(),
        len: tas2764_unk_seq2.len() as core::ffi::c_int,
    },
    tas2764_quirk_init_sequence {
        seq: tas2764_thermal_th1_dis_seq.as_ptr(),
        len: tas2764_thermal_th1_dis_seq.len() as core::ffi::c_int,
    },
    tas2764_quirk_init_sequence {
        seq: tas2764_shutdown_dance_init_seq.as_ptr(),
        len: tas2764_shutdown_dance_init_seq.len() as core::ffi::c_int,
    },
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
