/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Helper macros to be used for flag output operand handling.
 *
 * The original C header selects between compiler flag-output operands and
 * the ipm fallback according to __GCC_ASM_FLAG_OUTPUTS__ and
 * IS_ENABLED(CONFIG_CC_ASM_FLAG_OUTPUT_BROKEN).  Rust has no direct
 * equivalent for those compiler/preprocessor conditions; the two forms are
 * represented below as Rust macros.
 *
 * CC_IPM!(sym) is used at the end of inline assembly.  The fallback extracts
 * the condition code and program mask with ipm.  CC_OUT!(sym, var) describes
 * the output operand.  CC_TRANSFORM!(cc) converts an extracted condition
 * code, and CC_CLOBBER!/CC_CLOBBER_LIST! describe clobbers.
 */

/* __HAVE_ASM_FLAG_OUTPUTS__ is defined by the flag-output compiler branch. */
#[cfg(all(gcc_asm_flag_outputs, not(cc_asm_flag_output_broken)))]
pub const __HAVE_ASM_FLAG_OUTPUTS__: i32 = 1;

#[cfg(all(gcc_asm_flag_outputs, not(cc_asm_flag_output_broken)))]
macro_rules! CC_IPM {
    ($sym:ident) => {};
}

#[cfg(all(gcc_asm_flag_outputs, not(cc_asm_flag_output_broken)))]
macro_rules! CC_OUT {
    ($sym:ident, $var:expr) => { ("=@cc", $var) };
}

#[cfg(all(gcc_asm_flag_outputs, not(cc_asm_flag_output_broken)))]
macro_rules! CC_TRANSFORM {
    ($cc:expr) => { $cc };
}

#[cfg(all(gcc_asm_flag_outputs, not(cc_asm_flag_output_broken)))]
macro_rules! CC_CLOBBER {
    () => {};
}

#[cfg(all(gcc_asm_flag_outputs, not(cc_asm_flag_output_broken)))]
macro_rules! CC_CLOBBER_LIST {
    ($($arg:expr),* $(,)?) => { ($($arg),*) };
}

/* Fallback branch: equivalent to the C ipm-based operand handling. */
#[cfg(not(all(gcc_asm_flag_outputs, not(cc_asm_flag_output_broken))))]
macro_rules! CC_IPM {
    ($sym:ident) => { concat!("\tipm\t%[", stringify!($sym), "]\n") };
}

#[cfg(not(all(gcc_asm_flag_outputs, not(cc_asm_flag_output_broken))))]
macro_rules! CC_OUT {
    ($sym:ident, $var:expr) => { ($sym, "=d", $var) };
}

#[cfg(not(all(gcc_asm_flag_outputs, not(cc_asm_flag_output_broken))))]
macro_rules! CC_TRANSFORM {
    ($cc:expr) => { (($cc) >> 28) };
}

#[cfg(not(all(gcc_asm_flag_outputs, not(cc_asm_flag_output_broken))))]
macro_rules! CC_CLOBBER {
    () => { "cc" };
}

#[cfg(not(all(gcc_asm_flag_outputs, not(cc_asm_flag_output_broken))))]
macro_rules! CC_CLOBBER_LIST {
    ($($arg:expr),* $(,)?) => { ("cc", $($arg),*) };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
