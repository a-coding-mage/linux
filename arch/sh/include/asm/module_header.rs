/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of: #include <asm-generic/module.h>

// Equivalent of CONFIG_DWARF_UNWINDER conditional compilation.
#[cfg(CONFIG_DWARF_UNWINDER)]
#[repr(C)]
pub struct mod_arch_specific {
    pub fde_list: list_head,
    pub cie_list: list_head,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
