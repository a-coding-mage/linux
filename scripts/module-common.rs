// SPDX-License-Identifier: GPL-2.0

// Dependency provided by the Linux kernel module environment: <linux/module.h>
// Include build-salt.h after module.h in order to inherit the definitions.
// The C preprocessor definition is preserved as a Rust configuration marker.
#[cfg(any())]
const INCLUDE_VERMAGIC: () = ();

// Dependencies provided by the Linux kernel module environment:
// <linux/build-salt.h>, <linux/elfnote-lto.h>, and <linux/vermagic.h>

// #ifdef CONFIG_UNWINDER_ORC
// Dependency provided by the Linux kernel environment: <asm/orc_header.h>.
// ORC_HEADER;
// #endif

// BUILD_SALT;
// BUILD_LTO_INFO;

// MODULE_INFO(vermagic, VERMAGIC_STRING);

// #ifdef CONFIG_MITIGATION_RETPOLINE
#[cfg(CONFIG_MITIGATION_RETPOLINE)]
#[used]
#[link_section = ".modinfo"]
static MODULE_INFO_RETPOLINE: &[u8] = b"retpoline=Y\0";
// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
