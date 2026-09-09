// SPDX-License-Identifier: GPL-2.0
/*
 * Clang Control Flow Integrity (CFI) support.
 *
 * Copyright (C) 2022 Google LLC
 */

// Dependencies supplied by the kernel headers and other translation units:
// linux/string.h, linux/cfi.h, asm/insn.h, and asm/insn-eval.h.

unsafe fn decode_cfi_insn(
    regs: *mut pt_regs,
    target: *mut c_ulong,
    type_: *mut u32,
) -> bool {
    let mut buffer: [c_char; MAX_INSN_SIZE] = [0; MAX_INSN_SIZE];
    let mut insn: insn = core::mem::zeroed();
    let mut offset: i32 = 0;

    *target = 0;
    *type_ = 0;

    /*
     * The compiler generates the following instruction sequence
     * for indirect call checks:
     *
     *   movl    -<id>, %r10d       ; 6 bytes
     *   addl    -<pos>(%reg), %r10d; 4 bytes
     *   je      .Ltmp1             ; 2 bytes
     *   ud2                        ; <- regs->ip
     *   .Ltmp1:
     *
     * We can decode the expected type and the target address from the
     * movl/addl instructions.
     */
    if copy_from_kernel_nofault(
        buffer.as_mut_ptr().cast(),
        (regs as *mut u8).offset((*regs).ip as isize).offset(-12).cast(),
        MAX_INSN_SIZE,
    ) != 0
    {
        return false;
    }
    if insn_decode_kernel(&mut insn, buffer.as_mut_ptr().offset(offset as isize).cast()) != 0 {
        return false;
    }
    if insn.opcode.value != 0xBA {
        return false;
    }

    *type_ = (-(insn.immediate.value as u32)) as u32;

    if copy_from_kernel_nofault(
        buffer.as_mut_ptr().cast(),
        (regs as *mut u8).offset((*regs).ip as isize).offset(-6).cast(),
        MAX_INSN_SIZE,
    ) != 0
    {
        return false;
    }
    if insn_decode_kernel(&mut insn, buffer.as_mut_ptr().offset(offset as isize).cast()) != 0 {
        return false;
    }
    if insn.opcode.value != 0x3 {
        return false;
    }

    /* Read the target address from the register. */
    offset = insn_get_modrm_rm_off(&insn, regs);
    if offset < 0 {
        return false;
    }

    *target = *((regs as *mut u8).offset(offset as isize) as *mut c_ulong);
    true
}

/*
 * Checks if a ud2 trap is because of a CFI failure, and handles the trap
 * if needed. Returns a bug_trap_type value similarly to report_bug.
 */
unsafe fn handle_cfi_failure(regs: *mut pt_regs) -> bug_trap_type {
    let mut target: c_ulong = 0;
    let mut addr: c_ulong = (*regs).ip;
    let mut type_: u32 = 0;

    match cfi_mode {
        CFI_KCFI => {
            if !is_cfi_trap(addr) {
                /*
                 * The updated kCFI sequence has "test $0xd6, %al" instead of
                 * "ud2", adjust the offset.
                 */
                addr = addr.wrapping_sub(1);
                if !is_cfi_trap(addr) {
                    return BUG_TRAP_TYPE_NONE;
                }
            }

            if !decode_cfi_insn(regs, &mut target, &mut type_) {
                return report_cfi_failure_noaddr(regs, addr);
            }
        }

        CFI_FINEIBT => {
            if !decode_fineibt_insn(regs, &mut target, &mut type_) {
                return BUG_TRAP_TYPE_NONE;
            }
        }

        _ => return BUG_TRAP_TYPE_NONE,
    }

    report_cfi_failure(regs, addr, &target, type_)
}

/* Ensure that __kcfi_typeid_ symbols are emitted for functions that may not
 * be indirectly called with all configurations. */
// __ADDRESSABLE(__memcpy)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
