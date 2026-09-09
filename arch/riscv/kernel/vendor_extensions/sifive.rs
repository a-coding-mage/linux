// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation:
// asm/cpufeature.h, asm/vendor_extensions.h, asm/vendor_extensions/sifive.h,
// linux/array_size.h, and linux/types.h.

/* All SiFive vendor extensions supported in Linux */
static riscv_isa_vendor_ext_sifive: [riscv_isa_ext_data; 4] = [
    __RISCV_ISA_EXT_DATA!(xsfvfnrclipxfqf, RISCV_ISA_VENDOR_EXT_XSFVFNRCLIPXFQF),
    __RISCV_ISA_EXT_DATA!(xsfvfwmaccqqq, RISCV_ISA_VENDOR_EXT_XSFVFWMACCQQQ),
    __RISCV_ISA_EXT_DATA!(xsfvqmaccdod, RISCV_ISA_VENDOR_EXT_XSFVQMACCDOD),
    __RISCV_ISA_EXT_DATA!(xsfvqmaccqoq, RISCV_ISA_VENDOR_EXT_XSFVQMACCQOQ),
];

static mut riscv_isa_vendor_ext_list_sifive: riscv_isa_vendor_ext_data_list =
    riscv_isa_vendor_ext_data_list {
        ext_data_count: riscv_isa_vendor_ext_sifive.len(),
        ext_data: riscv_isa_vendor_ext_sifive.as_ptr(),
    };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
