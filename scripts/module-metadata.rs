// SPDX-License-Identifier: GPL-2.0-only
//! Typed module-loader metadata; the actual module owner remains authoritative.
#![no_std]

use kernel::bindings;

#[path = "../include/linux/export-internal_header.rs"]
mod export_internal;

const fn module_value(name: &[u8]) -> bindings::module {
    let mut value: bindings::module = pin_init::zeroed();
    let count = name.len() - 1;
    assert!(count <= value.name.len());
    let mut at = 0;
    while at < count {
        value.name[at] = name[at] as _;
        at += 1;
    }
    value
}

// Configurations without basic versions do not instantiate this helper.
#[allow(dead_code)]
const fn version_value(crc: u32, name: &[u8]) -> bindings::modversion_info {
    let mut value: bindings::modversion_info = pin_init::zeroed();
    let count = name.len() - 1;
    assert!(count <= value.name.len());
    value.crc = crc as _;
    let mut at = 0;
    while at < count {
        value.name[at] = name[at] as _;
        at += 1;
    }
    value
}

// Original asm/module.h creates these mandatory empty linker sections even
// when the real owner itself is Rust. They contain no translated algorithms.
#[cfg(all(CONFIG_PPC, target_pointer_width = "64"))]
core::arch::global_asm!(".pushsection .stubs,\"ax\",@nobits\n.balign 8\n.popsection");
#[cfg(all(CONFIG_PPC, target_pointer_width = "64", CONFIG_PPC_KERNEL_PCREL))]
core::arch::global_asm!(".pushsection .mygot,\"a\",@nobits\n.balign 8\n.popsection");
#[cfg(all(CONFIG_PPC, target_pointer_width = "32"))]
core::arch::global_asm!(".pushsection .plt,\"ax\",@nobits\n.balign 8\n.popsection\n.pushsection .init.plt,\"ax\",@nobits\n.balign 8\n.popsection");

include!(env!("MODULE_METADATA_DATA"));
