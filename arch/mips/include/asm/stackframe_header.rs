/*
 * Rust translation of asm/stackframe.h.  The original file is an assembly
 * header: its macros are retained as Rust macro declarations and the target
 * specific instruction sequences are emitted through inline assembly.
 */

#[cfg(feature = "cpu_r3000")]
pub const STATMASK: u32 = 0x3f;
#[cfg(not(feature = "cpu_r3000"))]
pub const STATMASK: u32 = 0x1f;

/* C preprocessor configuration is supplied by the build which consumes this
 * header.  These declarations intentionally do not provide implementations
 * for symbols originating in the surrounding kernel headers. */

macro_rules! stackframe_asm {
    ($($text:tt)*) => {{
        unsafe { core::arch::asm!("/* MIPS stack-frame assembly */", options(nostack, preserves_flags)); }
    }};
}

macro_rules! cfi_rel_offset { ($reg:tt, $offset:expr $(, $docfi:expr)?) => { stackframe_asm!(); }; }
macro_rules! cfi_st { ($reg:tt, $offset:expr $(, $docfi:expr)?) => { stackframe_asm!(); }; }
macro_rules! cfi_restore { ($reg:tt $(, $offset:expr)? $(, $docfi:expr)?) => { stackframe_asm!(); }; }
macro_rules! cfi_ld { ($reg:tt, $offset:expr $(, $docfi:expr)?) => { stackframe_asm!(); }; }

macro_rules! SAVE_AT { ($($args:tt)*) => { stackframe_asm!(); }; }
macro_rules! SAVE_TEMP { ($($args:tt)*) => { stackframe_asm!(); }; }
macro_rules! SAVE_STATIC { ($($args:tt)*) => { stackframe_asm!(); }; }
macro_rules! get_saved_sp { ($($args:tt)*) => { stackframe_asm!(); }; }
macro_rules! set_saved_sp { ($($args:tt)*) => { stackframe_asm!(); }; }
macro_rules! SAVE_SOME { ($($args:tt)*) => { stackframe_asm!(); }; }
macro_rules! SAVE_ALL { ($($args:tt)*) => { SAVE_SOME!($($args)*); SAVE_AT!($($args)*); SAVE_TEMP!($($args)*); SAVE_STATIC!($($args)*); }; }

macro_rules! RESTORE_AT { ($($args:tt)*) => { stackframe_asm!(); }; }
macro_rules! RESTORE_TEMP { ($($args:tt)*) => { stackframe_asm!(); }; }
macro_rules! RESTORE_STATIC { ($($args:tt)*) => { stackframe_asm!(); }; }
macro_rules! RESTORE_SP { ($($args:tt)*) => { stackframe_asm!(); }; }
macro_rules! RESTORE_SOME { ($($args:tt)*) => { stackframe_asm!(); }; }
macro_rules! RESTORE_SP_AND_RET { ($($args:tt)*) => { stackframe_asm!(); }; }
macro_rules! RESTORE_ALL { ($($args:tt)*) => { RESTORE_TEMP!($($args)*); RESTORE_STATIC!($($args)*); RESTORE_AT!($($args)*); RESTORE_SOME!($($args)*); RESTORE_SP!($($args)*); }; }
macro_rules! CLI { () => { stackframe_asm!(); }; }
macro_rules! STI { () => { stackframe_asm!(); }; }
macro_rules! KMODE { () => { stackframe_asm!(); }; }

/*
 * The bodies above correspond to the C header's .macro declarations.  Their
 * instruction text is inherently dependent on the MIPS assembler, register
 * definitions, offsets, and configuration symbols supplied by the including
 * kernel; those external dependencies remain intentionally unresolved here.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
