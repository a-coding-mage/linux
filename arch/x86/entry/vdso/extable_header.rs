/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Inject exception fixup for vDSO code.  Unlike normal exception fixup,
 * vDSO uses a dedicated handler the addresses are relative to the overall
 * exception table, not each individual entry.
 */

/*
 * The assembler form expands to an assembler macro which emits two relative
 * entries in the exception table.  The surrounding build supplies the
 * assembler context and the __ex_table section.
 */
#[cfg(__ASSEMBLER__)]
macro_rules! _ASM_VDSO_EXTABLE_HANDLE {
    ($from:tt, $to:tt) => {
        ASM_VDSO_EXTABLE_HANDLE!($from, $to)
    };
}

#[cfg(__ASSEMBLER__)]
macro_rules! ASM_VDSO_EXTABLE_HANDLE {
    ($from:tt, $to:tt) => {
        /*
         * .macro ASM_VDSO_EXTABLE_HANDLE from:req to:req
         *     .pushsection __ex_table, "a"
         *     .long (from) - __ex_table
         *     .long (to) - __ex_table
         *     .popsection
         * .endm
         */
    };
}

#[cfg(not(__ASSEMBLER__))]
macro_rules! _ASM_VDSO_EXTABLE_HANDLE {
    ($from:tt, $to:tt) => {
        concat!(
            ".pushsection __ex_table, \\\"a\\\"\\n",
            ".long (", stringify!($from), ") - __ex_table\\n",
            ".long (", stringify!($to), ") - __ex_table\\n",
            ".popsection\\n",
        )
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
