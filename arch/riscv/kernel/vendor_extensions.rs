// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2024 Rivos, Inc
 */

// The declarations below are supplied by the corresponding kernel headers.
// Configuration symbols retain the intent of the C preprocessor conditionals.

extern "C" {
    pub static mut riscv_isa_vendor_ext_list_andes: riscv_isa_vendor_ext_data_list;
    pub static mut riscv_isa_vendor_ext_list_mips: riscv_isa_vendor_ext_data_list;
    pub static mut riscv_isa_vendor_ext_list_sifive: riscv_isa_vendor_ext_data_list;
    pub static mut riscv_isa_vendor_ext_list_thead: riscv_isa_vendor_ext_data_list;

    fn test_bit(bit: usize, addr: *const usize) -> bool;
}

pub static mut riscv_isa_vendor_ext_list: [*mut riscv_isa_vendor_ext_data_list; 4] = [
    #[cfg(CONFIG_RISCV_ISA_VENDOR_EXT_ANDES)]
    unsafe { &raw mut riscv_isa_vendor_ext_list_andes },
    #[cfg(CONFIG_RISCV_ISA_VENDOR_EXT_MIPS)]
    unsafe { &raw mut riscv_isa_vendor_ext_list_mips },
    #[cfg(CONFIG_RISCV_ISA_VENDOR_EXT_SIFIVE)]
    unsafe { &raw mut riscv_isa_vendor_ext_list_sifive },
    #[cfg(CONFIG_RISCV_ISA_VENDOR_EXT_THEAD)]
    unsafe { &raw mut riscv_isa_vendor_ext_list_thead },
];

pub const riscv_isa_vendor_ext_list_size: usize =
    riscv_isa_vendor_ext_list.len();

/**
 * __riscv_isa_vendor_extension_available() - Check whether given vendor
 * extension is available or not.
 *
 * @cpu: check if extension is available on this cpu
 * @vendor: vendor that the extension is a member of
 * @bit: bit position of the desired extension
 * Return: true or false
 *
 * NOTE: When cpu is -1, will check if extension is available on all cpus
 */
#[no_mangle]
pub unsafe extern "C" fn __riscv_isa_vendor_extension_available(
    cpu: i32,
    vendor: usize,
    bit: u32,
) -> bool {
    let mut bmap: *mut riscv_isavendorinfo;
    let mut cpu_bmap: *mut riscv_isavendorinfo;

    match vendor {
        #[cfg(CONFIG_RISCV_ISA_VENDOR_EXT_ANDES)]
        ANDES_VENDOR_ID => {
            bmap = &raw mut riscv_isa_vendor_ext_list_andes.all_harts_isa_bitmap;
            cpu_bmap = riscv_isa_vendor_ext_list_andes.per_hart_isa_bitmap;
        }
        #[cfg(CONFIG_RISCV_ISA_VENDOR_EXT_MIPS)]
        MIPS_VENDOR_ID => {
            bmap = &raw mut riscv_isa_vendor_ext_list_mips.all_harts_isa_bitmap;
            cpu_bmap = riscv_isa_vendor_ext_list_mips.per_hart_isa_bitmap;
        }
        #[cfg(CONFIG_RISCV_ISA_VENDOR_EXT_SIFIVE)]
        SIFIVE_VENDOR_ID => {
            bmap = &raw mut riscv_isa_vendor_ext_list_sifive.all_harts_isa_bitmap;
            cpu_bmap = riscv_isa_vendor_ext_list_sifive.per_hart_isa_bitmap;
        }
        #[cfg(CONFIG_RISCV_ISA_VENDOR_EXT_THEAD)]
        THEAD_VENDOR_ID => {
            bmap = &raw mut riscv_isa_vendor_ext_list_thead.all_harts_isa_bitmap;
            cpu_bmap = riscv_isa_vendor_ext_list_thead.per_hart_isa_bitmap;
        }
        _ => return false,
    }

    if cpu != -1 {
        bmap = cpu_bmap.add(cpu as usize);
    }

    if bit >= RISCV_ISA_VENDOR_EXT_MAX {
        return false;
    }

    test_bit(bit as usize, (*bmap).isa.as_ptr())
}

// EXPORT_SYMBOL_GPL(__riscv_isa_vendor_extension_available);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
