/* SPDX-License-Identifier: GPL-2.0 */

/*
 * About the exception table:
 *
 * - insn is a 32-bit pc-relative offset from the faulting insn.
 * - nextinsn is a 16-bit offset off of the faulting instruction
 *   (not off of the *next* instruction as branches are).
 * - errreg is the register in which to place -EFAULT.
 * - valreg is the final target register for the load sequence
 *   and will be zeroed.
 *
 * Either errreg or valreg may be $31, in which case nothing happens.
 *
 * The exception fixup information "just so happens" to be arranged
 * as in a MEM format instruction.  This lets us emit our three
 * values like so:
 *
 *      lda valreg, nextinsn(errreg)
 *
 */

#[repr(C)]
pub struct exception_table_entry {
    pub insn: i32,
    pub fixup: exception_fixup,
}

#[repr(C)]
pub union exception_fixup {
    pub unit: u32,
    pub bits: exception_fixup_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exception_fixup_bits {
    /* C bit-fields are represented by the packed instruction word. */
    pub raw: u32,
}

impl exception_fixup_bits {
    #[inline]
    pub fn nextinsn(&self) -> i32 {
        ((self.raw as u16) as i16) as i32
    }

    #[inline]
    pub fn errreg(&self) -> u32 {
        (self.raw >> 16) & 0x1f
    }

    #[inline]
    pub fn valreg(&self) -> u32 {
        (self.raw >> 21) & 0x1f
    }
}

/* Returns the new pc */
#[macro_export]
macro_rules! fixup_exception {
    ($map_reg:expr, $fixup:expr, $pc:expr) => {{
        let __fixup = $fixup;
        let __bits = unsafe { &(*__fixup).fixup.bits };
        if __bits.valreg() != 31 {
            $map_reg(__bits.valreg()) = 0;
        }
        if __bits.errreg() != 31 {
            $map_reg(__bits.errreg()) = -EFAULT;
        }
        ($pc) + __bits.nextinsn()
    }};
}

/* ARCH_HAS_RELATIVE_EXTABLE */

#[macro_export]
macro_rules! swap_ex_entry_fixup {
    ($a:expr, $b:expr, $tmp:expr, $delta:expr) => {{
        unsafe {
            (*$a).fixup.unit = (*$b).fixup.unit;
            (*$b).fixup.unit = (*$tmp).fixup.unit;
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
