// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the Linux EFI and EFI stub headers.

pub static mut efi_system_table: *const efi_system_table_t = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
