// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation.

/* All Andes vendor extensions supported in Linux */
static RISCV_ISA_VENDOR_EXT_ANDES: [riscv_isa_ext_data; 1] = [
    riscv_isa_ext_data {
        name: xandespmu,
        id: RISCV_ISA_VENDOR_EXT_XANDESPMU,
    },
];

static mut riscv_isa_vendor_ext_list_andes: riscv_isa_vendor_ext_data_list =
    riscv_isa_vendor_ext_data_list {
        ext_data_count: RISCV_ISA_VENDOR_EXT_ANDES.len(),
        ext_data: RISCV_ISA_VENDOR_EXT_ANDES.as_ptr(),
    };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
