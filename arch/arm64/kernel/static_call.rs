// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel headers:
// linux/static_call.h, linux/memory.h, asm/text-patching.h, asm/insn.h

use core::ffi::c_void;

extern "C" {
    static __static_call_return0: c_void;

    fn aarch64_insn_adrp_get_offset(insn: u32) -> i64;
    fn aarch64_insn_decode_immediate(imm_type: i32, insn: u32) -> u32;
    fn aarch64_insn_write_literal_u64(literal: *mut c_void, value: u64) -> i32;
    fn le32_to_cpup(ptr: *const u8) -> u32;
    fn WARN_ON_ONCE(condition: bool) -> bool;
}

const SZ_4K: u64 = 4096;
const AARCH64_INSN_IMM_12: i32 = 12;

#[inline]
unsafe fn align_down(value: u64, alignment: u64) -> u64 {
    value & !(alignment - 1)
}

pub unsafe extern "C" fn arch_static_call_transform(
    site: *mut c_void,
    mut tramp: *mut c_void,
    mut func: *mut c_void,
    tail: bool,
) {
    let literal: u64;
    let ret: i32;

    let _ = site;
    let _ = tail;

    if func.is_null() {
        func = core::ptr::addr_of!(__static_call_return0) as *mut c_void;
    }

    /* decode the instructions to discover the literal address */
    literal = align_down(tramp as u64 + 4, SZ_4K)
        .wrapping_add(aarch64_insn_adrp_get_offset(le32_to_cpup(
            (tramp as *mut u8).add(4),
        )))
        .wrapping_add(
            8u64.wrapping_mul(aarch64_insn_decode_immediate(
                AARCH64_INSN_IMM_12,
                le32_to_cpup((tramp as *mut u8).add(8)),
            ) as u64),
        );

    ret = aarch64_insn_write_literal_u64(literal as *mut c_void, func as u64);
    WARN_ON_ONCE(ret != 0);
}

// EXPORT_SYMBOL_GPL(arch_static_call_transform);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
