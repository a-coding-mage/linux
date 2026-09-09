/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Definitions common to probes files
 *
 * Copyright IBM Corporation, 2012
 *
 * The original declarations are available only when building the kernel.
 */

/* #define BREAKPOINT_INSTRUCTION PPC_RAW_TRAP() */
macro_rules! BREAKPOINT_INSTRUCTION {
    () => { PPC_RAW_TRAP!() };
}

/* Trap definitions per ISA */
macro_rules! IS_TW {
    ($instr:expr) => { (($instr & 0xfc0007fe) == 0x7c000008) };
}
macro_rules! IS_TD {
    ($instr:expr) => { (($instr & 0xfc0007fe) == 0x7c000088) };
}
macro_rules! IS_TDI {
    ($instr:expr) => { (($instr & 0xfc000000) == 0x08000000) };
}
macro_rules! IS_TWI {
    ($instr:expr) => { (($instr & 0xfc000000) == 0x0c000000) };
}

/* CONFIG_PPC64 selects the ISA-specific definition. */
#[cfg(target_pointer_width = "64")]
macro_rules! is_trap {
    ($instr:expr) => {
        IS_TW!($instr) || IS_TD!($instr) || IS_TWI!($instr) || IS_TDI!($instr)
    };
}
#[cfg(not(target_pointer_width = "64"))]
macro_rules! is_trap {
    ($instr:expr) => { IS_TW!($instr) || IS_TWI!($instr) };
}

/* CONFIG_PPC_ADV_DEBUG_REGS selects the single-step status bit. */
#[cfg(feature = "CONFIG_PPC_ADV_DEBUG_REGS")]
const MSR_SINGLESTEP: u64 = MSR_DE;
#[cfg(not(feature = "CONFIG_PPC_ADV_DEBUG_REGS"))]
const MSR_SINGLESTEP: u64 = MSR_SE;

unsafe fn can_single_step(inst: u32) -> bool {
    match get_op(inst) {
        OP_TRAP_64 => false,
        OP_TRAP => false,
        OP_SC => false,
        OP_19 => {
            match get_xop(inst) {
                OP_19_XOP_RFID => false,
                OP_19_XOP_RFMCI => false,
                OP_19_XOP_RFDI => false,
                OP_19_XOP_RFI => false,
                OP_19_XOP_RFCI => false,
                OP_19_XOP_RFSCV => false,
                OP_19_XOP_HRFID => false,
                OP_19_XOP_URFID => false,
                OP_19_XOP_STOP => false,
                OP_19_XOP_DOZE => false,
                OP_19_XOP_NAP => false,
                OP_19_XOP_SLEEP => false,
                OP_19_XOP_RVWINKLE => false,
                _ => true,
            }
        }
        OP_31 => {
            match get_xop(inst) {
                OP_31_XOP_TRAP => false,
                OP_31_XOP_TRAP_64 => false,
                OP_31_XOP_MTMSR => false,
                OP_31_XOP_MTMSRD => false,
                _ => true,
            }
        }
        _ => true,
    }
}

/* Enable single stepping for the current task */
unsafe fn enable_single_step(regs: *mut pt_regs) {
    regs_set_return_msr(regs, (*regs).msr | MSR_SINGLESTEP);
    #[cfg(feature = "CONFIG_PPC_ADV_DEBUG_REGS")]
    {
        /*
         * We turn off Critical Input Exception(CE) to ensure that the single
         * step will be for the instruction we have the probe on; if we don't,
         * it is possible we'd get the single step reported for CE.
         */
        regs_set_return_msr(regs, (*regs).msr & !MSR_CE);
        mtspr(SPRN_DBCR0, mfspr(SPRN_DBCR0) | DBCR0_IC | DBCR0_IDM);
        #[cfg(feature = "CONFIG_PPC_47x")]
        isync();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
