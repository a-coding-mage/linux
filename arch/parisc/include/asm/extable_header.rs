/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// #include <asm/ptrace.h>
// #include <linux/compiler.h>

/*
 * The exception table consists of three addresses:
 *
 * - A relative address to the instruction that is allowed to fault.
 * - A relative address at which the program should continue (fixup routine)
 * - An asm statement which specifies which CPU register will
 *   receive -EFAULT when an exception happens if the lowest bit in the
 *   fixup address is set.
 *
 * Note: The register specified in the err_opcode instruction will be
 * modified at runtime if a fault happens. Register %r0 will be ignored.
 *
 * Since relative addresses are used, 32bit values are sufficient even on
 * 64bit kernel.
 */

#[allow(non_camel_case_types)]
pub struct pt_regs;

unsafe extern "C" {
    pub fn fixup_exception(regs: *mut pt_regs) -> ::core::ffi::c_int;
}

pub const ARCH_HAS_RELATIVE_EXTABLE: bool = true;

#[repr(C)]
pub struct exception_table_entry {
    pub insn: ::core::ffi::c_int,       /* relative address of insn that is allowed to fault. */
    pub fixup: ::core::ffi::c_int,      /* relative address of fixup routine */
    pub err_opcode: ::core::ffi::c_int, /* sample opcode with register which holds error code */
}

// ASM_EXCEPTIONTABLE_ENTRY emits architecture-specific assembler text in C.
// The string form preserves the original section, alignment, relative-address,
// opcode, and section-restoration operations for downstream assembly handling.
#[macro_export]
macro_rules! ASM_EXCEPTIONTABLE_ENTRY {
    ($fault_addr:expr, $except_addr:expr, $opcode:expr) => {
        concat!(
            ".section __ex_table,\"aw\"\n",
            ".align 4\n",
            ".word (", stringify!($fault_addr), " - .), (",
            stringify!($except_addr), " - .)\n",
            $opcode,
            "\n.previous\n"
        )
    };
}

/*
 * ASM_EXCEPTIONTABLE_ENTRY_EFAULT() creates a special exception table entry
 * (with lowest bit set) for which the fault handler in fixup_exception() will
 * load -EFAULT on fault into the register specified by the err_opcode instruction,
 * and zeroes the target register in case of a read fault in get_user().
 */
#[macro_export]
macro_rules! ASM_EXCEPTIONTABLE_VAR {
    ($err_var:ident) => {
        let $err_var = 0;
    };
}

#[macro_export]
macro_rules! ASM_EXCEPTIONTABLE_ENTRY_EFAULT {
    ($fault_addr:expr, $except_addr:expr, $register:expr) => {
        $crate::ASM_EXCEPTIONTABLE_ENTRY!(
            $fault_addr,
            $except_addr + 1,
            concat!("or %%r0,%%r0,", $register)
        )
    };
}

#[inline]
pub unsafe fn swap_ex_entry_fixup(
    a: *mut exception_table_entry,
    b: *mut exception_table_entry,
    tmp: exception_table_entry,
    delta: ::core::ffi::c_int,
) {
    (*a).fixup = (*b).fixup + delta;
    (*b).fixup = tmp.fixup - delta;
    (*a).err_opcode = (*b).err_opcode;
    (*b).err_opcode = tmp.err_opcode;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
