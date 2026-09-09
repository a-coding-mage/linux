/* SPDX-License-Identifier: GPL-2.0
 *
 * Driver for AMD network controllers and boards
 *
 * Copyright (C) 2021, Xilinx, Inc.
 * Copyright (C) 2022-2023, Advanced Micro Devices, Inc.
 */

// Dependency intent: declarations from <linux/cdx/mcdi.h> are supplied by
// other translated units.

pub const MC_CMD_EDAC_GET_DDR_CONFIG_OUT_WORD_LENGTH_LEN: u32 = 4;
/* Number of registers for the DDR controller */
pub const MC_CMD_GET_DDR_CONFIG_OFST: u32 = 4;
pub const MC_CMD_GET_DDR_CONFIG_LEN: u32 = 4;

/***********************************/
/* MC_CMD_EDAC_GET_DDR_CONFIG
 * Provides detailed configuration for the DDR controller of the given index.
 */
pub const MC_CMD_EDAC_GET_DDR_CONFIG: u32 = 0x3;

/* MC_CMD_EDAC_GET_DDR_CONFIG_IN msgrequest */
pub const MC_CMD_EDAC_GET_DDR_CONFIG_IN_CONTROLLER_INDEX_OFST: u32 = 0;
pub const MC_CMD_EDAC_GET_DDR_CONFIG_IN_CONTROLLER_INDEX_LEN: u32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
