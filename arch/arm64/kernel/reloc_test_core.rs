// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2017 Linaro, Ltd. <ard.biesheuvel@linaro.org>
 */

// C dependency: <linux/module.h>

pub static mut sym64_rel: i32 = 0;

pub const SYM64_ABS_VAL: u64 = 0xffff_8800_00cc_cccc;
pub const SYM32_ABS_VAL: u32 = 0xf800_cccc;
pub const SYM16_ABS_VAL: u16 = 0xf8cc;

// The C SET_ABS macros define absolute assembler symbols. They are supplied
// by the target build's assembler/linker and are intentionally not redefined here.
extern "C" {
    pub static sym64_abs: u64;
    pub static sym32_abs: u32;
    pub static sym16_abs: u16;
}

extern "C" {
    pub fn absolute_data64() -> u64;
    pub fn absolute_data32() -> u64;
    pub fn absolute_data16() -> u64;
    pub fn signed_movw() -> u64;
    pub fn unsigned_movw() -> u64;
    pub fn relative_adrp() -> u64;
    pub fn relative_adrp_far() -> u64;
    pub fn relative_adr() -> u64;
    pub fn relative_data64() -> u64;
    pub fn relative_data32() -> u64;
    pub fn relative_data16() -> u64;
    pub static memstart_addr: u64;
}

#[repr(C)]
struct Func {
    name: [u8; 32],
    f: unsafe extern "C" fn() -> u64,
    expect: u64,
}

const fn name(value: &str) -> [u8; 32] {
    let bytes = value.as_bytes();
    let mut result = [0u8; 32];
    let mut i = 0;
    while i < bytes.len() {
        result[i] = bytes[i];
        i += 1;
    }
    result
}

static FUNCS: [Func; 11] = [
    Func { name: name("R_AARCH64_ABS64"), f: absolute_data64, expect: SYM64_ABS_VAL },
    Func { name: name("R_AARCH64_ABS32"), f: absolute_data32, expect: SYM32_ABS_VAL as u64 },
    Func { name: name("R_AARCH64_ABS16"), f: absolute_data16, expect: SYM16_ABS_VAL as u64 },
    Func { name: name("R_AARCH64_MOVW_SABS_Gn"), f: signed_movw, expect: SYM64_ABS_VAL },
    Func { name: name("R_AARCH64_MOVW_UABS_Gn"), f: unsigned_movw, expect: SYM64_ABS_VAL },
    Func { name: name("R_AARCH64_ADR_PREL_PG_HI21"), f: relative_adrp, expect: unsafe { &sym64_rel as *const _ as u64 } },
    Func { name: name("R_AARCH64_ADR_PREL_PG_HI21"), f: relative_adrp_far, expect: unsafe { &memstart_addr as *const _ as u64 } },
    Func { name: name("R_AARCH64_ADR_PREL_LO21"), f: relative_adr, expect: unsafe { &sym64_rel as *const _ as u64 } },
    Func { name: name("R_AARCH64_PREL64"), f: relative_data64, expect: unsafe { &sym64_rel as *const _ as u64 } },
    Func { name: name("R_AARCH64_PREL32"), f: relative_data32, expect: unsafe { &sym64_rel as *const _ as u64 } },
    Func { name: name("R_AARCH64_PREL16"), f: relative_data16, expect: unsafe { &sym64_rel as *const _ as u64 } },
];

// C dependency: kernel pr_info/pr_err logging and module registration macros.
pub unsafe extern "C" fn reloc_test_init() -> i32 {
    // pr_info("Relocation test:\n");
    // pr_info("-------------------------------------------------------\n");
    for func in &FUNCS {
        let ret = (func.f)();
        // pr_info("%-31s 0x%016llx %s\n", func.name, ret,
        //     ret == func.expect ? "pass" : "fail");
        if ret != func.expect {
            // pr_err("Relocation failed, expected 0x%016llx, not 0x%016llx\n",
            //        func.expect, ret);
        }
    }
    0
}

pub unsafe extern "C" fn reloc_test_exit() {}

// C dependency: module_init(reloc_test_init); module_exit(reloc_test_exit);
// MODULE_DESCRIPTION("Relocation testing module");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
