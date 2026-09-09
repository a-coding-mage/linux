// SPDX-License-Identifier: GPL-2.0
//
// The C initializers include architecture-generic audit class definitions.
// Those header expansions are supplied by the surrounding kernel build.

// Contents from <asm-generic/audit_dir_write.h>, followed by ~0U.
pub static mut parisc32_dir_class: [u32; 1] = [u32::MAX];

// Contents from <asm-generic/audit_change_attr.h>, followed by ~0U.
pub static mut parisc32_chattr_class: [u32; 1] = [u32::MAX];

// Contents from <asm-generic/audit_write.h>, followed by ~0U.
pub static mut parisc32_write_class: [u32; 1] = [u32::MAX];

// Contents from <asm-generic/audit_read.h>, followed by ~0U.
pub static mut parisc32_read_class: [u32; 1] = [u32::MAX];

// Contents from <asm-generic/audit_signal.h>, followed by ~0U.
pub static mut parisc32_signal_class: [u32; 1] = [u32::MAX];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
