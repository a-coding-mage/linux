/* SPDX-License-Identifier: GPL-2.0 */
/* C header guard: _ASM_M68K_PCI_H */

#[inline]
pub const fn pcibios_assign_all_busses() -> i32 {
    1
}

pub const PCIBIOS_MIN_IO: u32 = 0x0000_0100;
pub const PCIBIOS_MIN_MEM: u32 = 0x0200_0000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
