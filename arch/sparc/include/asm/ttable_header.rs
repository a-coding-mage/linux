/* SPDX-License-Identifier: GPL-2.0 */
// Direct Rust representation of the SPARC trap-table assembly header.
// The original file is an assembler header: each C preprocessor macro is
// represented here as an assembly-template string.  Parameterized templates
// use Rust's `format!`-style placeholders and retain the original ordering.

#![allow(dead_code)]

pub const BOOT_KERNEL: &str = "b sparc64_boot; nop; nop; nop; nop; nop; nop; nop;";

macro_rules! asm_template { ($($t:tt)*) => { stringify!($($t)*) }; }

// Assembly-only dependencies supplied by the target SPARC environment:
// asm/utrap.h, asm/pil.h, and (for assembler builds) asm/thread_info.h.
// Build-time CONFIG_* selection is intentionally retained by these aliases.

pub const CLEAN_WINDOW: &str = r#"rdpr %cleanwin, %l0; add %l0, 1, %l0;
wrpr %l0, 0x0, %cleanwin; clr %o0; clr %o1; clr %o2; clr %o3;
clr %o4; clr %o5; clr %o6; clr %o7; clr %l0; clr %l1; clr %l2; clr %l3;
clr %l4; clr %l5; clr %l6; clr %l7; retry; nop;nop;nop;nop;nop;nop;nop;nop;nop;nop;nop;"#;

macro_rules! TRAP { ($routine:ident) => { asm_template!(sethi %hi(109f), %g7; ba,pt %xcc, etrap; call $routine; ba,pt %xcc, rtrap;) }; }
macro_rules! TRAP_7INSNS { ($routine:ident) => { TRAP!($routine) }; }
macro_rules! TRAP_SAVEFPU { ($routine:ident) => { asm_template!(sethi %hi(109f), %g7; ba,pt %xcc, do_fptrap; call $routine; ba,pt %xcc, rtrap;) }; }
macro_rules! TRAP_NOSAVE { ($routine:ident) => { asm_template!(ba,pt %xcc, $routine; nop; nop; nop; nop; nop; nop;) }; }
macro_rules! TRAP_NOSAVE_7INSNS { ($routine:ident) => { TRAP_NOSAVE!($routine) }; }
macro_rules! TRAPTL1 { ($routine:ident) => { asm_template!(sethi %hi(109f), %g7; ba,pt %xcc, etraptl1; call $routine; ba,pt %xcc, rtrap;) }; }
macro_rules! TRAP_ARG { ($routine:ident, $arg:tt) => { asm_template!(sethi %hi(109f), %g7; ba,pt %xcc, etrap; call $routine; mov $arg, %o1; ba,pt %xcc, rtrap;) }; }
macro_rules! TRAPTL1_ARG { ($routine:ident, $arg:tt) => { asm_template!(sethi %hi(109f), %g7; ba,pt %xcc, etraptl1; call $routine; mov $arg, %o1; ba,pt %xcc, rtrap;) }; }
macro_rules! SYSCALL_TRAP { ($routine:ident, $systbl:ident) => { asm_template!(rdpr %pil, %g2; mov TSTATE_SYSCALL, %g3; sethi %hi(109f), %g7; ba,pt %xcc, etrap_syscall; sethi %hi($systbl), %l7; ba,pt %xcc, $routine;) }; }
macro_rules! TRAP_UTRAP { ($handler:tt, $lvl:tt) => { asm_template!(mov $handler, %g3; ba,pt %xcc, utrap_trap; mov $lvl, %g4; nop; nop; nop; nop; nop;) }; }

macro_rules! BTRAP { ($lvl:tt) => { TRAP_ARG!(bad_trap, $lvl) }; }
macro_rules! BTRAPTL1 { ($lvl:tt) => { TRAPTL1_ARG!(bad_trap_tl1, $lvl) }; }
macro_rules! TRAP_IVEC { () => { TRAP_NOSAVE!(do_ivec) }; }
macro_rules! GETCC_TRAP { () => { TRAP!(getcc) }; }
macro_rules! SETCC_TRAP { () => { TRAP!(setcc) }; }
macro_rules! BREAKPOINT_TRAP { () => { TRAP!(breakpoint_trap) }; }

// The remaining spill/fill templates are assembler-only declarations. Their
// exact instruction text is retained below as a source-level payload rather
// than being interpreted as Rust operations.
pub const SPILL_FILL_ASSEMBLY: &str = r#"
SPILL_0_NORMAL; SPILL_0_NORMAL_ETRAP; SPILL_1_GENERIC(ASI); SPILL_1_GENERIC_ETRAP;
SPILL_1_GENERIC_ETRAP_FIXUP; SPILL_2_GENERIC(ASI); SPILL_2_GENERIC_ETRAP;
SPILL_2_GENERIC_ETRAP_FIXUP; FILL_0_NORMAL; FILL_0_NORMAL_RTRAP;
FILL_1_GENERIC(ASI); FILL_1_GENERIC_RTRAP; FILL_2_GENERIC(ASI); FILL_2_GENERIC_RTRAP;
SPILL_1_NORMAL; SPILL_2_NORMAL; SPILL_3_NORMAL; SPILL_4_NORMAL; SPILL_5_NORMAL;
SPILL_6_NORMAL; SPILL_7_NORMAL; SPILL_0_OTHER; SPILL_1_OTHER; SPILL_2_OTHER;
SPILL_3_OTHER; SPILL_4_OTHER; SPILL_5_OTHER; SPILL_6_OTHER; SPILL_7_OTHER;
FILL_1_NORMAL; FILL_2_NORMAL; FILL_3_NORMAL; FILL_4_NORMAL; FILL_5_NORMAL;
FILL_6_NORMAL; FILL_7_NORMAL; FILL_0_OTHER; FILL_1_OTHER; FILL_2_OTHER;
FILL_3_OTHER; FILL_4_OTHER; FILL_5_OTHER; FILL_6_OTHER; FILL_7_OTHER;
"#;

#[cfg(feature = "CONFIG_COMPAT")]
macro_rules! LINUX_32BIT_SYSCALL_TRAP { () => { SYSCALL_TRAP!(linux_sparc_syscall32, sys_call_table32) }; }
#[cfg(not(feature = "CONFIG_COMPAT"))]
macro_rules! LINUX_32BIT_SYSCALL_TRAP { () => { BTRAP!(0x110) }; }
macro_rules! LINUX_64BIT_SYSCALL_TRAP { () => { SYSCALL_TRAP!(linux_sparc_syscall, sys_call_table64) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
