/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <asm-generic/fprobe.h>

// CONFIG_64BIT
// The C header undefines the generic FPROBE_HEADER_MSB_PATTERN definition and
// replaces it with the platform-specific value below.
#[cfg(target_pointer_width = "64")]
pub const FPROBE_HEADER_MSB_PATTERN: usize = PAGE_OFFSET & !FPROBE_HEADER_MSB_MASK;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
