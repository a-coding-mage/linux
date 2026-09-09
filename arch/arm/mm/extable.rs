// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/arm/mm/extable.c
 */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    fn search_exception_tables(addr: usize) -> *const exception_table_entry;
    fn instruction_pointer(regs: *const pt_regs) -> usize;
}

#[repr(C)]
pub struct pt_regs {
    pub ARM_pc: usize,
    pub ARM_cpsr: usize,
}

#[repr(C)]
pub struct exception_table_entry {
    pub fixup: usize,
}

#[cfg(CONFIG_THUMB2_KERNEL)]
const PSR_IT_MASK: usize = 0x0600_fc00;

pub unsafe fn fixup_exception(regs: *mut pt_regs) -> i32 {
    let fixup: *const exception_table_entry;

    fixup = search_exception_tables(instruction_pointer(regs));
    if !fixup.is_null() {
        (*regs).ARM_pc = (*fixup).fixup;
        #[cfg(CONFIG_THUMB2_KERNEL)]
        {
            /* Clear the IT state to avoid nasty surprises in the fixup */
            (*regs).ARM_cpsr &= !PSR_IT_MASK;
        }
    }

    if !fixup.is_null() { 1 } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
