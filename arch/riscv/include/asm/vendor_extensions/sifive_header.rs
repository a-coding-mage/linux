/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// #include <asm/vendor_extensions.h>
// #include <linux/types.h>

pub const RISCV_ISA_VENDOR_EXT_XSFVQMACCDOD: u32 = 0;
pub const RISCV_ISA_VENDOR_EXT_XSFVQMACCQOQ: u32 = 1;
pub const RISCV_ISA_VENDOR_EXT_XSFVFNRCLIPXFQF: u32 = 2;
pub const RISCV_ISA_VENDOR_EXT_XSFVFWMACCQQQ: u32 = 3;

extern "C" {
    pub static mut riscv_isa_vendor_ext_list_sifive: riscv_isa_vendor_ext_data_list;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
