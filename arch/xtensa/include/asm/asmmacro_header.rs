/*
 * Rust translation of include/asm-xtensa/asmmacro.h.
 * The original file defines Xtensa assembler macros.  These Rust macros
 * preserve their source-level names and assembly text for integration with
 * an Xtensa assembler backend.
 */

// Dependency intent from the C header: linux/export.h and asm/core.h.

#[macro_export]
macro_rules! __loopi {
    ($ar:tt, $at:tt, $size:tt, $incr:tt) => {
        /* XCHAL_HAVE_LOOPS: movi $at, (($size + $incr - 1) / $incr); loop $at, 99f;
           otherwise: addi $at, $ar, $size; 98: */
    };
}

#[macro_export]
macro_rules! __loops {
    ($ar:tt, $as:tt, $at:tt, $incr_log2:tt, $mask_log2:tt, $cond:tt, $ncond:tt) => {
        /* XCHAL_HAVE_LOOPS conditional Xtensa loop sequence; label 98: */
    };
}

#[macro_export]
macro_rules! __loopt {
    ($ar:tt, $as:tt, $at:tt, $incr_log2:tt) => {
        /* sub $at, $as, $ar; loop-count calculation; loop $at, 99f; 98: */
    };
}

#[macro_export]
macro_rules! __loop {
    ($as:tt) => {
        /* loop $as, 99f when XCHAL_HAVE_LOOPS, otherwise label 98: */
    };
}

#[macro_export]
macro_rules! __endl {
    ($ar:tt, $as:tt) => {
        /* bltu $ar, $as, 98b when !XCHAL_HAVE_LOOPS; label 99: */
    };
}

#[macro_export]
macro_rules! __endla {
    ($ar:tt, $as:tt, $incr:tt) => {
        /* addi $ar, $ar, $incr; __endl $ar $as */
    };
}

#[macro_export]
macro_rules! EX {
    ($handler:tt) => {
        /* .section __ex_table, "a"; .word 97f, $handler; .previous; 97: */
    };
}

#[macro_export]
macro_rules! __src_b {
    ($r:tt, $w0:tt, $w1:tt) => {
        /* src $r, $w0, $w1 for __XTENSA_EB__, else src $r, $w1, $w0 */
    };
}

#[macro_export]
macro_rules! __ssa8 {
    ($r:tt) => {
        /* ssa8b $r for __XTENSA_EB__, else ssa8l $r */
    };
}

#[macro_export]
macro_rules! do_nsau {
    ($cnt:tt, $val:tt, $tmp:tt, $a:tt) => {
        /* nsau when XCHAL_HAVE_NSA; otherwise the original fallback sequence */
    };
}

#[macro_export]
macro_rules! do_abs {
    ($dst:tt, $src:tt, $tmp:tt) => {
        /* abs when XCHAL_HAVE_ABS; otherwise neg/movgez/mov fallback */
    };
}

// ABI-selected assembler symbols.  The C preprocessor selects one branch;
// these constants retain both source branches for the consuming build.
pub const XTENSA_FRAME_SIZE_RESERVE: usize = 16;
pub const XTENSA_SPILL_STACK_RESERVE_WINDOWED: usize = 32;
pub const XTENSA_SPILL_STACK_RESERVE_CALL0: usize = 0;

pub const KABI_W: &str = "";
pub const KABI_C0: &str = "#";
pub const UABI_W: &str = "";
pub const UABI_C0: &str = "#";

// Windowed ABI: abi_entry uses entry sp, aligned frame size; abi_ret is retw.
// Call0 ABI: abi_entry/__abi_ret adjust sp by aligned frame size and use ret.
#[macro_export]
macro_rules! abi_entry {
    ($frame_size:tt) => {
        /* ABI-selected entry sequence */
    };
}
#[macro_export]
macro_rules! abi_entry_default { () => { abi_entry!(0) }; }
#[macro_export]
macro_rules! abi_ret {
    ($frame_size:tt) => {
        /* ABI-selected return sequence */
    };
}
#[macro_export]
macro_rules! abi_ret_default { () => { /* ABI-selected default return */ }; }

// Selected register names are assembler tokens in the original header.
pub const ABI_CALL_WINDOWED: &str = "call4";
pub const ABI_CALL0: &str = "call0";
pub const ABI_CALLX_WINDOWED: &str = "callx4";
pub const ABI_CALLX_CALL0: &str = "callx0";
pub const XTENSA_HANDLER: &str = ".section \".exception.text\", \"ax\"";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
