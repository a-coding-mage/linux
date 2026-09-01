/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Rust translation of arch/x86/include/asm/asm.h.
 *
 * C preprocessor branches in the original distinguish assembler input,
 * C inline-asm string input, 32-bit x86, 64-bit x86, __KERNEL__, and
 * CONFIG_KPROBES.  Rust has no direct equivalent for the assembler-vs-C
 * preprocessor mode in this header, so these macros preserve the C-side
 * inline-asm string behavior and keep the assembler-side intent in comments.
 */

macro_rules! __ASM_FORM {
    ($x:literal $(, $arg:tt)*) => {
        concat!(" ", stringify!($x $(, $arg)*), " ")
    };
    ($x:ident $(, $arg:tt)*) => {
        concat!(" ", stringify!($x $(, $arg)*), " ")
    };
    ($($x:tt)+) => {
        concat!(" ", stringify!($($x)+), " ")
    };
}

macro_rules! __ASM_FORM_RAW {
    ($($x:tt)+) => {
        stringify!($($x)+)
    };
}

macro_rules! __ASM_FORM_COMMA {
    ($($x:tt)+) => {
        concat!(" ", stringify!($($x)+), ",")
    };
}

macro_rules! _ASM_BYTES {
    ($x:tt $(, $arg:tt)*) => {
        __ASM_FORM!(.byte $x $(, $arg)* ;)
    };
}

/* 32-bit/64-bit selector macros. */
#[cfg(not(target_arch = "x86_64"))]
macro_rules! __ASM_SEL {
    ($a:tt, $b:tt) => {
        __ASM_FORM!($a)
    };
}

#[cfg(target_arch = "x86_64")]
macro_rules! __ASM_SEL {
    ($a:tt, $b:tt) => {
        __ASM_FORM!($b)
    };
}

#[cfg(not(target_arch = "x86_64"))]
macro_rules! __ASM_SEL_RAW {
    ($a:tt, $b:tt) => {
        __ASM_FORM_RAW!($a)
    };
}

#[cfg(target_arch = "x86_64")]
macro_rules! __ASM_SEL_RAW {
    ($a:tt, $b:tt) => {
        __ASM_FORM_RAW!($b)
    };
}

macro_rules! __ASM_SIZE {
    (mov) => {
        __ASM_SEL!(movl, movq)
    };
    (inc) => {
        __ASM_SEL!(incl, incq)
    };
    (dec) => {
        __ASM_SEL!(decl, decq)
    };
    (add) => {
        __ASM_SEL!(addl, addq)
    };
    (sub) => {
        __ASM_SEL!(subl, subq)
    };
    (xadd) => {
        __ASM_SEL!(xaddl, xaddq)
    };
    (mul) => {
        __ASM_SEL!(mull, mulq)
    };
    ($inst:ident, $suffix:ident) => {
        __ASM_SEL!(
            concat!(stringify!($inst), "l", stringify!($suffix)),
            concat!(stringify!($inst), "q", stringify!($suffix))
        )
    };
}

macro_rules! __ASM_REG {
    (ax) => {
        __ASM_SEL_RAW!(eax, rax)
    };
    (bx) => {
        __ASM_SEL_RAW!(ebx, rbx)
    };
    (cx) => {
        __ASM_SEL_RAW!(ecx, rcx)
    };
    (dx) => {
        __ASM_SEL_RAW!(edx, rdx)
    };
    (sp) => {
        __ASM_SEL_RAW!(esp, rsp)
    };
    (bp) => {
        __ASM_SEL_RAW!(ebp, rbp)
    };
    (si) => {
        __ASM_SEL_RAW!(esi, rsi)
    };
    (di) => {
        __ASM_SEL_RAW!(edi, rdi)
    };
}

macro_rules! _ASM_PTR {
    () => {
        __ASM_SEL!(.long, .quad)
    };
}

macro_rules! _ASM_ALIGN {
    () => {
        __ASM_SEL!(.balign 4, .balign 8)
    };
}

macro_rules! _ASM_MOV {
    () => {
        __ASM_SIZE!(mov)
    };
}

macro_rules! _ASM_INC {
    () => {
        __ASM_SIZE!(inc)
    };
}

macro_rules! _ASM_DEC {
    () => {
        __ASM_SIZE!(dec)
    };
}

macro_rules! _ASM_ADD {
    () => {
        __ASM_SIZE!(add)
    };
}

macro_rules! _ASM_SUB {
    () => {
        __ASM_SIZE!(sub)
    };
}

macro_rules! _ASM_XADD {
    () => {
        __ASM_SIZE!(xadd)
    };
}

macro_rules! _ASM_MUL {
    () => {
        __ASM_SIZE!(mul)
    };
}

macro_rules! _ASM_AX {
    () => {
        __ASM_REG!(ax)
    };
}

macro_rules! _ASM_BX {
    () => {
        __ASM_REG!(bx)
    };
}

macro_rules! _ASM_CX {
    () => {
        __ASM_REG!(cx)
    };
}

macro_rules! _ASM_DX {
    () => {
        __ASM_REG!(dx)
    };
}

macro_rules! _ASM_SP {
    () => {
        __ASM_REG!(sp)
    };
}

macro_rules! _ASM_BP {
    () => {
        __ASM_REG!(bp)
    };
}

macro_rules! _ASM_SI {
    () => {
        __ASM_REG!(si)
    };
}

macro_rules! _ASM_DI {
    () => {
        __ASM_REG!(di)
    };
}

/* 32 bit */
#[cfg(not(target_arch = "x86_64"))]
macro_rules! _ASM_ARG1 {
    () => {
        _ASM_AX!()
    };
}

#[cfg(not(target_arch = "x86_64"))]
macro_rules! _ASM_ARG2 {
    () => {
        _ASM_DX!()
    };
}

#[cfg(not(target_arch = "x86_64"))]
macro_rules! _ASM_ARG3 {
    () => {
        _ASM_CX!()
    };
}

#[cfg(not(target_arch = "x86_64"))]
pub const _ASM_ARG1L: &str = "eax";
#[cfg(not(target_arch = "x86_64"))]
pub const _ASM_ARG2L: &str = "edx";
#[cfg(not(target_arch = "x86_64"))]
pub const _ASM_ARG3L: &str = "ecx";

#[cfg(not(target_arch = "x86_64"))]
pub const _ASM_ARG1W: &str = "ax";
#[cfg(not(target_arch = "x86_64"))]
pub const _ASM_ARG2W: &str = "dx";
#[cfg(not(target_arch = "x86_64"))]
pub const _ASM_ARG3W: &str = "cx";

#[cfg(not(target_arch = "x86_64"))]
pub const _ASM_ARG1B: &str = "al";
#[cfg(not(target_arch = "x86_64"))]
pub const _ASM_ARG2B: &str = "dl";
#[cfg(not(target_arch = "x86_64"))]
pub const _ASM_ARG3B: &str = "cl";

/* 64 bit */
#[cfg(target_arch = "x86_64")]
macro_rules! _ASM_ARG1 {
    () => {
        _ASM_DI!()
    };
}

#[cfg(target_arch = "x86_64")]
macro_rules! _ASM_ARG2 {
    () => {
        _ASM_SI!()
    };
}

#[cfg(target_arch = "x86_64")]
macro_rules! _ASM_ARG3 {
    () => {
        _ASM_DX!()
    };
}

#[cfg(target_arch = "x86_64")]
macro_rules! _ASM_ARG4 {
    () => {
        _ASM_CX!()
    };
}

#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG5: &str = "r8";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG6: &str = "r9";

#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG1Q: &str = "rdi";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG2Q: &str = "rsi";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG3Q: &str = "rdx";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG4Q: &str = "rcx";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG5Q: &str = "r8";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG6Q: &str = "r9";

#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG1L: &str = "edi";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG2L: &str = "esi";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG3L: &str = "edx";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG4L: &str = "ecx";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG5L: &str = "r8d";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG6L: &str = "r9d";

#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG1W: &str = "di";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG2W: &str = "si";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG3W: &str = "dx";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG4W: &str = "cx";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG5W: &str = "r8w";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG6W: &str = "r9w";

#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG1B: &str = "dil";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG2B: &str = "sil";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG3B: &str = "dl";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG4B: &str = "cl";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG5B: &str = "r8b";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ARG6B: &str = "r9b";

/* Exception table entry */
/* Original code is guarded by __KERNEL__. */

/*
 * Assembler-side original:
 * _ASM_EXTABLE_HANDLE(from, to, handler) emits:
 *   .pushsection "__ex_table","a";
 *   .balign 4;
 *   .long (from) - .;
 *   .long (to) - .;
 *   .long (handler) - .;
 *   .popsection
 *
 * CONFIG_KPROBES assembler-side _ASM_NOKPROBE(entry) emits an entry in
 * "_kprobe_blacklist"; otherwise it expands to nothing.
 */

macro_rules! _EXPAND_EXTABLE_HANDLE {
    ($x:tt) => {
        stringify!($x)
    };
}

macro_rules! _ASM_EXTABLE_HANDLE {
    ($from:tt, $to:tt, $handler:tt) => {
        concat!(
            " .pushsection \"__ex_table\",\"a\"\n",
            " .balign 4\n",
            " .long (",
            stringify!($from),
            ") - .\n",
            " .long (",
            stringify!($to),
            ") - .\n",
            " .long (",
            _EXPAND_EXTABLE_HANDLE!($handler),
            ") - .\n",
            " .popsection\n"
        )
    };
}

macro_rules! _ASM_EXTABLE {
    ($from:tt, $to:tt) => {
        _ASM_EXTABLE_HANDLE!($from, $to, ex_handler_default)
    };
}

macro_rules! _ASM_EXTABLE_UA {
    ($from:tt, $to:tt) => {
        _ASM_EXTABLE_HANDLE!($from, $to, ex_handler_uaccess)
    };
}

macro_rules! _ASM_EXTABLE_CPY {
    ($from:tt, $to:tt) => {
        _ASM_EXTABLE_HANDLE!($from, $to, ex_handler_copy)
    };
}

macro_rules! _ASM_EXTABLE_FAULT {
    ($from:tt, $to:tt) => {
        _ASM_EXTABLE_HANDLE!($from, $to, ex_handler_fault)
    };
}

/* For C file, we already have NOKPROBE_SYMBOL macro */

/*
 * This output constraint should be used for any inline asm which has a "call"
 * instruction.  Otherwise the asm may be inserted before the frame pointer
 * gets set up by the containing function.  If you forget to do this, objtool
 * may print a "call without frame pointer save/setup" warning.
 *
 * Original C declaration:
 * register unsigned long current_stack_pointer asm(_ASM_SP);
 */
#[cfg(target_pointer_width = "64")]
pub type current_stack_pointer_word = u64;

#[cfg(target_pointer_width = "32")]
pub type current_stack_pointer_word = u32;

unsafe extern "C" {
    pub static mut current_stack_pointer: current_stack_pointer_word;
}

macro_rules! ASM_CALL_CONSTRAINT {
    () => {
        concat!("+r", " (current_stack_pointer)")
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
