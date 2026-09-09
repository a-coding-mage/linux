/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2017 Andes Technology Corporation */

// C build-time condition preserved: CONFIG_MODULE_SECTIONS.
// When enabled, the linker script defines zero-filled placeholder sections:
// SECTIONS {
//     .plt 0 : { BYTE(0) }
//     .got 0 : { BYTE(0) }
//     .got.plt 0 : { BYTE(0) }
// }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
