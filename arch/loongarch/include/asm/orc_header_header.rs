/* SPDX-License-Identifier: GPL-2.0-or-later */

// Translated from the C header. `ORC_HASH` is supplied by the corresponding
// ORC hash dependency/build-time definition.

#[used]
#[link_section = ".orc_header"]
static orc_header: OrcHeader = OrcHeader([ORC_HASH]);

#[repr(align(4))]
struct OrcHeader([u8; 20]);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
