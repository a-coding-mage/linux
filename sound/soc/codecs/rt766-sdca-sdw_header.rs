// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt766-sdca-sdw.h -- RT766 SDCA ALSA SoC audio driver header
 *
 * Copyright(c) 2026 Realtek Semiconductor Corp.
 */

// C header dependencies:
// #include <linux/regmap.h>
// #include <linux/soundwire/sdw_registers.h>

pub const rt766_sdca_defaults: [reg_default; 32] = [
    /* 0x40400289 - 0x4040028a */
    reg_default { reg: RT766_MUTE_REG!(UAJ, USER_FU41, 1), def: 0x01 },
    reg_default { reg: RT766_MUTE_REG!(UAJ, USER_FU41, 2), def: 0x01 },
    /* 0x40400291 - 0x40400292 */
    reg_default { reg: RT766_VOLUME_REG!(UAJ, USER_FU41, 1), def: 0x0000 },
    reg_default { reg: RT766_VOLUME_REG!(UAJ, USER_FU41, 2), def: 0x0000 },
    /* 0x40400789 - 0x4040078a */
    reg_default { reg: RT766_MUTE_REG!(UAJ, USER_FU36, 1), def: 0x01 },
    reg_default { reg: RT766_MUTE_REG!(UAJ, USER_FU36, 2), def: 0x01 },
    /* 0x40400791 - 0x40400792 */
    reg_default { reg: RT766_VOLUME_REG!(UAJ, USER_FU36, 1), def: 0x0000 },
    reg_default { reg: RT766_VOLUME_REG!(UAJ, USER_FU36, 2), def: 0x0000 },
    reg_default { reg: RT766_PDE_REQ_REG!(UAJ, PDE47), def: 0x03 }, /* 0x40401408 */
    reg_default { reg: RT766_PDE_REQ_REG!(UAJ, PDE34), def: 0x03 }, /* 0x40401488 */
    reg_default { reg: RT766_SDCA_CTL!(UAJ, CS41, SDCA_CTL_CS_SAMPLERATEINDEX), def: 0x09 }, /* 0x40480080 */
    reg_default { reg: RT766_SDCA_CTL!(UAJ, CS36, SDCA_CTL_CS_SAMPLERATEINDEX), def: 0x09 }, /* 0x40480880 */
    /* 0x40600259 - 0x4060025a */
    reg_default { reg: RT766_GAIN_REG!(UAJ, PLATFORM_FU33, 1), def: 0xfe00 },
    reg_default { reg: RT766_GAIN_REG!(UAJ, PLATFORM_FU33, 2), def: 0xfe00 },
    reg_default { reg: RT766_SDCA_CTL!(UAJ, GE49, SDCA_CTL_GE_SELECTED_MODE), def: 0x00 }, /* 0x40600488 */

    reg_default { reg: RT766_PDE_REQ_REG!(MIC, PDE11), def: 0x03 }, /* 0x40801508 */
    /* 0x40801809 - 0x4080180c */
    reg_default { reg: RT766_MUTE_REG!(MIC, USER_FU113, 1), def: 0x01 },
    reg_default { reg: RT766_MUTE_REG!(MIC, USER_FU113, 2), def: 0x01 },
    reg_default { reg: RT766_MUTE_REG!(MIC, USER_FU113, 3), def: 0x01 },
    reg_default { reg: RT766_MUTE_REG!(MIC, USER_FU113, 4), def: 0x01 },
    /* 0x40801811 - 0x40801814 */
    reg_default { reg: RT766_VOLUME_REG!(MIC, USER_FU113, 1), def: 0x0000 },
    reg_default { reg: RT766_VOLUME_REG!(MIC, USER_FU113, 2), def: 0x0000 },
    reg_default { reg: RT766_VOLUME_REG!(MIC, USER_FU113, 3), def: 0x0000 },
    reg_default { reg: RT766_VOLUME_REG!(MIC, USER_FU113, 4), def: 0x0000 },
    reg_default { reg: RT766_SDCA_CTL!(MIC, CS113, SDCA_CTL_CS_SAMPLERATEINDEX), def: 0x09 }, /* 0x40880900 */

    /* 0x41000189 - 0x4100018a */
    reg_default { reg: RT766_MUTE_REG!(AMP, USER_FU21, 1), def: 0x01 },
    reg_default { reg: RT766_MUTE_REG!(AMP, USER_FU21, 2), def: 0x01 },
    /* 0x41000191 - 0x41000192 */
    reg_default { reg: RT766_VOLUME_REG!(AMP, USER_FU21, 1), def: 0x0000 },
    reg_default { reg: RT766_VOLUME_REG!(AMP, USER_FU21, 2), def: 0x0000 },
    reg_default { reg: RT766_PDE_REQ_REG!(AMP, PDE23), def: 0x03 }, /* 0x41001988 */
    reg_default { reg: RT766_SDCA_CTL!(AMP, PPU21, SDCA_CTL_PPU_POSTURENUMBER), def: 0x00 }, /* 0x41080200 */
    reg_default { reg: RT766_SDCA_CTL!(AMP, CS21, SDCA_CTL_CS_SAMPLERATEINDEX), def: 0x09 }, /* 0x41081080 */
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
