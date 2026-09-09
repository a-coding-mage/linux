/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of linux/linkage.h.
// The original header is primarily preprocessor and assembler syntax.  The
// following macros preserve its names and expansion intent for consumers that
// provide the corresponding architecture/build definitions.

/* Some toolchains use other characters (e.g. '`') to mark new line in macro. */
#[allow(unused_macros)]
macro_rules! ASM_NL { () => { ";" }; }

#[allow(unused_macros)]
macro_rules! CPP_ASMLINKAGE { () => {}; }

#[allow(unused_macros)]
macro_rules! asmlinkage { ($($tt:tt)*) => { $($tt)* }; }

#[allow(unused_macros)]
macro_rules! cond_syscall {
    ($x:ident) => {
        core::arch::global_asm!(concat!(".weak ", stringify!($x), "\n\t",
                                        ".set  ", stringify!($x), ",sys_ni_syscall"));
    };
}

#[allow(unused_macros)]
macro_rules! SYSCALL_ALIAS {
    ($alias:ident, $name:ident) => {
        core::arch::global_asm!(concat!(".globl ", stringify!($alias), "\n\t",
                                        ".set   ", stringify!($alias), ",", stringify!($name)));
    };
}

/* These retain the source section/alignment annotations for declarations. */
#[allow(unused_macros)]
macro_rules! __page_aligned_data { ($item:item) => { $item }; }
#[allow(unused_macros)]
macro_rules! __page_aligned_bss { ($item:item) => { $item }; }
#[allow(unused_macros)]
macro_rules! __bss_pgtbl { ($item:item) => { $item }; }

/* Assembly directives. */
#[allow(unused_macros)]
macro_rules! __PAGE_ALIGNED_DATA { () => { ".section \\".data..page_aligned\\", \\"aw\\"" }; }
#[allow(unused_macros)]
macro_rules! __PAGE_ALIGNED_BSS { () => { ".section \\".bss..page_aligned\\", \\"aw\\"" }; }

#[allow(unused_macros)]
macro_rules! asmlinkage_protect { ($n:expr, $ret:expr $(, $args:expr)*) => {{ let _ = ($n, $ret); }}; }

#[allow(unused_macros)]
macro_rules! __ALIGN { () => { ".balign CONFIG_FUNCTION_ALIGNMENT" }; }
#[allow(unused_macros)]
macro_rules! __ALIGN_STR { () => { ".balign CONFIG_FUNCTION_ALIGNMENT" }; }

/* The following symbols are assembler-only annotations from the C header. */
#[allow(unused_macros)]
macro_rules! SYM_T_FUNC { () => { "STT_FUNC" }; }
#[allow(unused_macros)]
macro_rules! SYM_T_OBJECT { () => { "STT_OBJECT" }; }
#[allow(unused_macros)]
macro_rules! SYM_T_NONE { () => { "STT_NOTYPE" }; }
#[allow(unused_macros)]
macro_rules! SYM_A_ALIGN { () => { "ALIGN" }; }
#[allow(unused_macros)]
macro_rules! SYM_A_NONE { () => {}; }

#[allow(unused_macros)]
macro_rules! SYM_L_GLOBAL { ($name:ident) => { concat!(".globl ", stringify!($name)) }; }
#[allow(unused_macros)]
macro_rules! SYM_L_WEAK { ($name:ident) => { concat!(".weak ", stringify!($name)) }; }
#[allow(unused_macros)]
macro_rules! SYM_L_LOCAL { ($name:ident) => {}; }

#[allow(unused_macros)]
macro_rules! SYM_ENTRY { ($name:ident, $linkage:ident $(, $align:ident)*) => { concat!(stringify!($name), ":") }; }
#[allow(unused_macros)]
macro_rules! SYM_START { ($name:ident, $linkage:ident $(, $align:ident)*) => { SYM_ENTRY!($name, $linkage $(, $align)*) }; }
#[allow(unused_macros)]
macro_rules! SYM_END { ($name:ident, $sym_type:ident) => { concat!(".type ", stringify!($name), " ", stringify!($sym_type), "\n.size ", stringify!($name), ", .-", stringify!($name)) }; }
#[allow(unused_macros)]
macro_rules! SYM_ALIAS { ($alias:ident, $name:ident, $linkage:ident) => { concat!(".set ", stringify!($alias), ", ", stringify!($name)) }; }

#[allow(unused_macros)]
macro_rules! SYM_INNER_LABEL_ALIGN { ($name:ident, $linkage:ident) => { SYM_ENTRY!($name, $linkage, SYM_A_ALIGN) }; }
#[allow(unused_macros)]
macro_rules! SYM_INNER_LABEL { ($name:ident, $linkage:ident) => { SYM_ENTRY!($name, $linkage, SYM_A_NONE) }; }

#[allow(unused_macros)]
macro_rules! SYM_FUNC_START { ($name:ident) => { SYM_START!($name, SYM_L_GLOBAL, SYM_A_ALIGN) }; }
#[allow(unused_macros)]
macro_rules! SYM_FUNC_START_NOALIGN { ($name:ident) => { SYM_START!($name, SYM_L_GLOBAL, SYM_A_NONE) }; }
#[allow(unused_macros)]
macro_rules! SYM_FUNC_START_LOCAL { ($name:ident) => { SYM_START!($name, SYM_L_LOCAL, SYM_A_ALIGN) }; }
#[allow(unused_macros)]
macro_rules! SYM_FUNC_START_LOCAL_NOALIGN { ($name:ident) => { SYM_START!($name, SYM_L_LOCAL, SYM_A_NONE) }; }
#[allow(unused_macros)]
macro_rules! SYM_FUNC_START_WEAK { ($name:ident) => { SYM_START!($name, SYM_L_WEAK, SYM_A_ALIGN) }; }
#[allow(unused_macros)]
macro_rules! SYM_FUNC_START_WEAK_NOALIGN { ($name:ident) => { SYM_START!($name, SYM_L_WEAK, SYM_A_NONE) }; }
#[allow(unused_macros)]
macro_rules! SYM_FUNC_END { ($name:ident) => { SYM_END!($name, SYM_T_FUNC) }; }
#[allow(unused_macros)]
macro_rules! SYM_FUNC_ALIAS { ($alias:ident, $name:ident) => { SYM_ALIAS!($alias, $name, SYM_L_GLOBAL) }; }
#[allow(unused_macros)]
macro_rules! SYM_FUNC_ALIAS_LOCAL { ($alias:ident, $name:ident) => { SYM_ALIAS!($alias, $name, SYM_L_LOCAL) }; }
#[allow(unused_macros)]
macro_rules! SYM_FUNC_ALIAS_WEAK { ($alias:ident, $name:ident) => { SYM_ALIAS!($alias, $name, SYM_L_WEAK) }; }

#[allow(unused_macros)]
macro_rules! SYM_CODE_START { ($name:ident) => { SYM_START!($name, SYM_L_GLOBAL, SYM_A_ALIGN) }; }
#[allow(unused_macros)]
macro_rules! SYM_CODE_START_NOALIGN { ($name:ident) => { SYM_START!($name, SYM_L_GLOBAL, SYM_A_NONE) }; }
#[allow(unused_macros)]
macro_rules! SYM_CODE_START_LOCAL { ($name:ident) => { SYM_START!($name, SYM_L_LOCAL, SYM_A_ALIGN) }; }
#[allow(unused_macros)]
macro_rules! SYM_CODE_START_LOCAL_NOALIGN { ($name:ident) => { SYM_START!($name, SYM_L_LOCAL, SYM_A_NONE) }; }
#[allow(unused_macros)]
macro_rules! SYM_CODE_END { ($name:ident) => { SYM_END!($name, SYM_T_NONE) }; }

#[allow(unused_macros)]
macro_rules! SYM_DATA_START { ($name:ident) => { SYM_START!($name, SYM_L_GLOBAL, SYM_A_NONE) }; }
#[allow(unused_macros)]
macro_rules! SYM_DATA_START_LOCAL { ($name:ident) => { SYM_START!($name, SYM_L_LOCAL, SYM_A_NONE) }; }
#[allow(unused_macros)]
macro_rules! SYM_DATA_END { ($name:ident) => { SYM_END!($name, SYM_T_OBJECT) }; }
#[allow(unused_macros)]
macro_rules! SYM_DATA_END_LABEL { ($name:ident, $linkage:ident, $label:ident) => { SYM_END!($name, SYM_T_OBJECT) }; }
#[allow(unused_macros)]
macro_rules! SYM_DATA { ($name:ident $(, $data:tt)*) => { SYM_DATA_START!($name); SYM_DATA_END!($name); }; }
#[allow(unused_macros)]
macro_rules! SYM_DATA_LOCAL { ($name:ident $(, $data:tt)*) => { SYM_DATA_START_LOCAL!($name); SYM_DATA_END!($name); }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
