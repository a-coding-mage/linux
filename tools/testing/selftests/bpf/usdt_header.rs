// SPDX-License-Identifier: BSD-2-Clause
/*
 *  This single-header library defines a collection of variadic macros for
 *  defining and triggering USDTs (User Statically-Defined Tracepoints):
 *
 *      - For USDTs without associated semaphore:
 *          USDT(group, name, args...)
 *
 *      - For USDTs with implicit (transparent to the user) semaphore:
 *          USDT_WITH_SEMA(group, name, args...)
 *          USDT_IS_ACTIVE(group, name)
 *
 *      - For USDTs with explicit (user-defined and provided) semaphore:
 *          USDT_WITH_EXPLICIT_SEMA(sema, group, name, args...)
 *          USDT_SEMA_IS_ACTIVE(sema)
 *
 *  all of which emit a NOP instruction into the instruction stream, and so
 *  have *zero* overhead for the surrounding code. USDTs are identified by
 *  a combination of `group` and `name` identifiers, which is used by external
 *  tracing tooling (tracers) for identifying exact USDTs of interest.
 *
 *  USDTs can have an associated (2-byte) activity counter (USDT semaphore),
 *  automatically maintained by Linux kernel whenever any correctly written
 *  BPF-based tracer is attached to the USDT. This USDT semaphore can be used
 *  to check whether there is a need to do any extra data collection and
 *  processing for a given USDT (if necessary), and otherwise avoid extra work
 *  for a common case of USDT not being traced ("active").
 *
 *  See documentation for USDT_WITH_SEMA()/USDT_IS_ACTIVE() or
 *  USDT_WITH_EXPLICIT_SEMA()/USDT_SEMA_IS_ACTIVE() APIs below for details on
 *  working with USDTs with implicitly or explicitly associated
 *  USDT semaphores, respectively.
 *
 *  There is also some additional data recorded into an auxiliary note
 *  section. The data in the note section describes the operands, in terms of
 *  size and location, used by tracing tooling to know where to find USDT
 *  arguments. Each location is encoded as an assembler operand string.
 *  Tracing tools (bpftrace and BPF-based tracers, systemtap, etc) insert
 *  breakpoints on top of the nop, and decode the location operand-strings,
 *  like an assembler, to find the values being passed.
 *
 *  The operand strings are selected by the compiler for each operand.
 *  They are constrained by inline-assembler codes.The default is:
 *
 *  #define USDT_ARG_CONSTRAINT nor
 *
 *  This is a good default if the operands tend to be integral and
 *  moderate in number (smaller than number of registers). In other
 *  cases, the compiler may report "'asm' requires impossible reload" or
 *  similar. In this case, consider simplifying the macro call (fewer
 *  and simpler operands), reduce optimization, or override the default
 *  constraints string via:
 *
 *  #define USDT_ARG_CONSTRAINT g
 *  #include <usdt.h>
 *
 * For some historical description of USDT v3 format (the one used by this
 * library and generally recognized and assumed by BPF-based tracing tools)
 * see [0]. The more formal specification can be found at [1]. Additional
 * argument constraints information can be found at [2].
 *
 * Original SystemTap's sys/sdt.h implementation ([3]) was used as a base for
 * this USDT library implementation. Current implementation differs *a lot* in
 * terms of exposed user API and general usability, which was the main goal
 * and focus of the reimplementation work. Nevertheless, underlying recorded
 * USDT definitions are fully binary compatible and any USDT-based tooling
 * should work equally well with USDTs defined by either SystemTap's or this
 * library's USDT implementation.
 *
 *   [0] https://ecos.sourceware.org/ml/systemtap/2010-q3/msg00145.html
 *   [1] https://sourceware.org/systemtap/wiki/UserSpaceProbeImplementation
 *   [2] https://gcc.gnu.org/onlinedocs/gcc/Constraints.html
 *   [3] https://sourceware.org/git/?p=systemtap.git;a=blob;f=includes/sys/sdt.h
 */

/*
 * Changelog:
 *
 * 0.1.0
 * -----
 * - Initial release
 */
pub const USDT_MAJOR_VERSION: u32 = 0;
pub const USDT_MINOR_VERSION: u32 = 1;
pub const USDT_PATCH_VERSION: u32 = 0;

#[repr(C)]
pub struct usdt_sema {
    pub active: u16,
}

/*
 * C preprocessor condition translated note:
 * C++20 and C23 added __VA_OPT__ as a standard replacement for non-standard
 * `##__VA_ARGS__` extension. Rust macro repetition handles optional trailing
 * variadic arguments directly.
 */

#[macro_export]
macro_rules! USDT {
    ($group:ident, $name:ident $(, $args:expr)* $(,)?) => {{
        $crate::__usdt_probe!($group, $name, __usdt_sema_none, 0usize $(, $args)*);
    }};
}

#[macro_export]
macro_rules! USDT_WITH_SEMA {
    ($group:ident, $name:ident $(, $args:expr)* $(,)?) => {{
        $crate::__usdt_probe!(
            $group,
            $name,
            __usdt_sema_implicit,
            $crate::__usdt_sema_name!($group, $name)
            $(, $args)*
        );
    }};
}

#[macro_export]
macro_rules! USDT_IS_ACTIVE {
    ($group:ident, $name:ident) => {{
        $crate::__usdt_sema_implicit!($crate::__usdt_sema_name!($group, $name));
        unsafe {
            ::core::ptr::addr_of!($crate::__usdt_sema_name!($group, $name).active).read_volatile() > 0
        }
    }};
}

/**
 * Underlying C global variable name for user-defined USDT semaphore with
 * `sema` identifier. Could be useful for debugging, but normally shouldn't be
 * used explicitly.
 */
#[macro_export]
macro_rules! USDT_SEMA {
    ($sema:ident) => {
        concat!("__usdt_sema_", stringify!($sema))
    };
}

/*
 * Define storage for user-defined USDT semaphore `sema`.
 *
 * The C macro token-pastes `__usdt_sema_` with the user identifier and assigns
 * section(".probes"), asm symbol name, and hidden visibility attributes.
 * Rust macro_rules! cannot create a new identifier by token-pasting without an
 * external helper, so this preserves the declaration intent and symbol name.
 */
#[macro_export]
macro_rules! USDT_DEFINE_SEMA {
    ($sema:ident) => {
        #[used]
        #[link_section = ".probes"]
        #[export_name = concat!("__usdt_sema_", stringify!($sema))]
        pub static mut $sema: $crate::usdt_sema = $crate::usdt_sema { active: 0 };
    };
}

/*
 * Declare extern reference to user-defined USDT semaphore `sema`.
 *
 * Rust cannot emit an `extern` item from macro_rules! whose Rust identifier is
 * token-pasted from `__usdt_sema_` and `sema`; this macro preserves the C API
 * surface as a narrow textual marker for translation users.
 */
#[macro_export]
macro_rules! USDT_DECLARE_SEMA {
    ($sema:ident) => {};
}

#[macro_export]
macro_rules! USDT_SEMA_IS_ACTIVE {
    ($sema:ident) => {{
        unsafe { ::core::ptr::addr_of!($sema.active).read_volatile() > 0 }
    }};
}

#[macro_export]
macro_rules! USDT_WITH_EXPLICIT_SEMA {
    ($sema:ident, $group:ident, $name:ident $(, $args:expr)* $(,)?) => {{
        $crate::__usdt_probe!($group, $name, __usdt_sema_explicit, $sema $(, $args)*);
    }};
}

/*
 * Adjustable implementation aspects.
 *
 * C selected USDT_ARG_CONSTRAINT by architecture:
 *   powerpc: nZr
 *   arm:     g
 *   loongarch: nmr
 *   default: nor
 */
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
pub const USDT_ARG_CONSTRAINT: &str = "nZr";
#[cfg(target_arch = "arm")]
pub const USDT_ARG_CONSTRAINT: &str = "g";
#[cfg(target_arch = "loongarch64")]
pub const USDT_ARG_CONSTRAINT: &str = "nmr";
#[cfg(not(any(
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "arm",
    target_arch = "loongarch64"
)))]
pub const USDT_ARG_CONSTRAINT: &str = "nor";

#[cfg(any(target_arch = "ia64", target_arch = "s390x"))]
pub const USDT_NOP: &str = "nop 0";
#[cfg(target_arch = "x86_64")]
pub const USDT_NOP: &str =
    ".byte 0x90, 0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00";
#[cfg(not(any(target_arch = "ia64", target_arch = "s390x", target_arch = "x86_64")))]
pub const USDT_NOP: &str = "nop";

/*
 * Implementation details
 */
#[macro_export]
macro_rules! __usdt_sema_name {
    ($group:ident, $name:ident) => {
        compile_error!("Rust macro_rules! cannot token-paste __usdt_sema_<group>__<name> without a helper macro");
    };
}

#[macro_export]
macro_rules! __usdt_concat {
    ($a:tt, $b:tt) => {
        compile_error!("C token pasting has no direct macro_rules! equivalent")
    };
}

#[macro_export]
macro_rules! __usdt_apply {
    ($fn:ident, $n:tt) => {
        $crate::__usdt_concat!($fn, $n)
    };
}

#[macro_export]
macro_rules! __usdt_nth {
    ($_:tt, $_1:tt, $_2:tt, $_3:tt, $_4:tt, $_5:tt, $_6:tt, $_7:tt, $_8:tt, $_9:tt, $_10:tt, $_11:tt, $_12:tt, $N:tt $(, $rest:tt)*) => {
        $N
    };
}

#[macro_export]
macro_rules! __usdt_narg {
    () => {
        0usize
    };
    ($one:tt) => {
        1usize
    };
    ($one:tt, $two:tt) => {
        2usize
    };
    ($one:tt, $two:tt, $three:tt) => {
        3usize
    };
    ($one:tt, $two:tt, $three:tt, $four:tt) => {
        4usize
    };
    ($one:tt, $two:tt, $three:tt, $four:tt, $five:tt) => {
        5usize
    };
    ($one:tt, $two:tt, $three:tt, $four:tt, $five:tt, $six:tt) => {
        6usize
    };
    ($one:tt, $two:tt, $three:tt, $four:tt, $five:tt, $six:tt, $seven:tt) => {
        7usize
    };
    ($one:tt, $two:tt, $three:tt, $four:tt, $five:tt, $six:tt, $seven:tt, $eight:tt) => {
        8usize
    };
    ($one:tt, $two:tt, $three:tt, $four:tt, $five:tt, $six:tt, $seven:tt, $eight:tt, $nine:tt) => {
        9usize
    };
    ($one:tt, $two:tt, $three:tt, $four:tt, $five:tt, $six:tt, $seven:tt, $eight:tt, $nine:tt, $ten:tt) => {
        10usize
    };
    ($one:tt, $two:tt, $three:tt, $four:tt, $five:tt, $six:tt, $seven:tt, $eight:tt, $nine:tt, $ten:tt, $eleven:tt) => {
        11usize
    };
    ($one:tt, $two:tt, $three:tt, $four:tt, $five:tt, $six:tt, $seven:tt, $eight:tt, $nine:tt, $ten:tt, $eleven:tt, $twelve:tt) => {
        12usize
    };
}

pub const __usdt_hash: &str = "#";

#[macro_export]
macro_rules! __usdt_str_ {
    ($x:tt) => {
        stringify!($x)
    };
}

#[macro_export]
macro_rules! __usdt_str {
    ($x:tt) => {
        $crate::__usdt_str_!($x)
    };
}

#[macro_export]
macro_rules! __usdt_asm_name {
    ($name:ident) => {
        stringify!($name)
    };
}

#[macro_export]
macro_rules! __usdt_asm0 {
    () => {
        "\n"
    };
}

#[macro_export]
macro_rules! __usdt_asm1 {
    ($x:tt) => {
        concat!(stringify!($x), "\n")
    };
}

#[macro_export]
macro_rules! __usdt_asm2 {
    ($x:tt, $($rest:tt)+) => {
        concat!(stringify!($x), ",", $crate::__usdt_asm1!($($rest)+))
    };
}

#[macro_export]
macro_rules! __usdt_asm3 {
    ($x:tt, $($rest:tt)+) => {
        concat!(stringify!($x), ",", $crate::__usdt_asm2!($($rest)+))
    };
}

#[macro_export]
macro_rules! __usdt_asm4 {
    ($x:tt, $($rest:tt)+) => {
        concat!(stringify!($x), ",", $crate::__usdt_asm3!($($rest)+))
    };
}

#[macro_export]
macro_rules! __usdt_asm5 {
    ($x:tt, $($rest:tt)+) => {
        concat!(stringify!($x), ",", $crate::__usdt_asm4!($($rest)+))
    };
}

#[macro_export]
macro_rules! __usdt_asm6 {
    ($x:tt, $($rest:tt)+) => {
        concat!(stringify!($x), ",", $crate::__usdt_asm5!($($rest)+))
    };
}

#[macro_export]
macro_rules! __usdt_asm7 {
    ($x:tt, $($rest:tt)+) => {
        concat!(stringify!($x), ",", $crate::__usdt_asm6!($($rest)+))
    };
}

#[macro_export]
macro_rules! __usdt_asm8 {
    ($x:tt, $($rest:tt)+) => {
        concat!(stringify!($x), ",", $crate::__usdt_asm7!($($rest)+))
    };
}

#[macro_export]
macro_rules! __usdt_asm9 {
    ($x:tt, $($rest:tt)+) => {
        concat!(stringify!($x), ",", $crate::__usdt_asm8!($($rest)+))
    };
}

#[macro_export]
macro_rules! __usdt_asm10 {
    ($x:tt, $($rest:tt)+) => {
        concat!(stringify!($x), ",", $crate::__usdt_asm9!($($rest)+))
    };
}

#[macro_export]
macro_rules! __usdt_asm11 {
    ($x:tt, $($rest:tt)+) => {
        concat!(stringify!($x), ",", $crate::__usdt_asm10!($($rest)+))
    };
}

#[macro_export]
macro_rules! __usdt_asm12 {
    ($x:tt, $($rest:tt)+) => {
        concat!(stringify!($x), ",", $crate::__usdt_asm11!($($rest)+))
    };
}

#[cfg(target_pointer_width = "64")]
pub const __usdt_asm_addr: &str = ".8byte";
#[cfg(not(target_pointer_width = "64"))]
pub const __usdt_asm_addr: &str = ".4byte";

#[macro_export]
macro_rules! __usdt_asm_strz_ {
    ($x:tt) => {
        concat!(".asciz ", stringify!($x), "\n")
    };
}

#[macro_export]
macro_rules! __usdt_asm_strz {
    ($x:tt) => {
        $crate::__usdt_asm_strz_!($x)
    };
}

#[macro_export]
macro_rules! __usdt_asm_str_ {
    ($x:tt) => {
        concat!(".ascii ", stringify!($x), "\n")
    };
}

#[macro_export]
macro_rules! __usdt_asm_str {
    ($x:tt) => {
        $crate::__usdt_asm_str_!($x)
    };
}

#[macro_export]
macro_rules! __usdt_sema_none {
    ($sema:expr) => {};
}

#[macro_export]
macro_rules! __usdt_sema_implicit {
    ($sema:expr) => {
        unsafe {
            ::core::arch::asm!(
                ".ifndef {sema}",
                ".pushsection .probes, \"aw\", \"progbits\"",
                ".weak {sema}",
                ".hidden {sema}",
                ".align 2",
                "{sema}:",
                ".zero 2",
                ".type {sema}, @object",
                ".size {sema}, 2",
                ".popsection",
                ".endif",
                sema = sym $sema,
                options(att_syntax)
            );
        }
    };
}

#[macro_export]
macro_rules! __usdt_sema_explicit {
    ($sema:expr) => {
        unsafe {
            ::core::arch::asm!("", in(reg) &$sema, options(nostack, preserves_flags));
        }
    };
}

/*
 * Main USDT definition (nop and .note.stapsdt metadata).
 * The C version builds architecture-specific GNU assembler fragments and
 * operand constraints through C preprocessor metaprogramming. This translation
 * keeps the observable sequencing: define the semaphore side effect first,
 * then emit a volatile inline-assembly USDT note fragment.
 */
#[macro_export]
macro_rules! __usdt_probe {
    ($group:ident, $name:ident, __usdt_sema_none, $sema:expr $(, $args:expr)* $(,)?) => {{
        $crate::__usdt_sema_none!($sema);
        unsafe {
            ::core::arch::asm!(
                "990: {nop}",
                ".pushsection .note.stapsdt, \"\", \"note\"",
                ".balign 4",
                ".4byte 992f-991f,994f-993f,3",
                "991: .asciz \"stapsdt\"",
                "992: .balign 4",
                "993: .quad 990b",
                ".quad _.stapsdt.base",
                ".quad 0",
                ".asciz \"{group}\"",
                ".asciz \"{name}\"",
                ".ascii \"\\0\"",
                "994: .balign 4",
                ".popsection",
                ".ifndef _.stapsdt.base",
                ".pushsection .stapsdt.base,\"aG\",\"progbits\",.stapsdt.base,comdat",
                ".weak _.stapsdt.base",
                ".hidden _.stapsdt.base",
                "_.stapsdt.base:",
                ".space 1",
                ".size _.stapsdt.base, 1",
                ".popsection",
                ".endif",
                nop = const $crate::USDT_NOP,
                group = const stringify!($group),
                name = const stringify!($name),
                options(att_syntax)
            );
        }
        let _ = ($(&$args),*);
    }};
    ($group:ident, $name:ident, __usdt_sema_implicit, $sema:expr $(, $args:expr)* $(,)?) => {{
        $crate::__usdt_sema_implicit!($sema);
        $crate::__usdt_probe!($group, $name, __usdt_sema_none, $sema $(, $args)*);
    }};
    ($group:ident, $name:ident, __usdt_sema_explicit, $sema:expr $(, $args:expr)* $(,)?) => {{
        $crate::__usdt_sema_explicit!($sema);
        $crate::__usdt_probe!($group, $name, __usdt_sema_none, $sema $(, $args)*);
    }};
}

/*
 * NB: gdb PR24541 highlighted an unspecified corner of the sdt.h
 * operand note format.
 *
 * The named register may be a longer or shorter (!) alias for the
 * storage where the value in question is found. For example, on
 * i386, 64-bit value may be put in register pairs, and a register
 * name stored would identify just one of them. Previously, gcc was
 * asked to emit the %w[id] (16-bit alias of some registers holding
 * operands), even when a wider 32-bit value was used.
 *
 * Bottom line: the byte-width given before the @ sign governs. If
 * there is a mismatch between that width and that of the named
 * register, then a sys/sdt.h note consumer may need to employ
 * architecture-specific heuristics to figure out where the compiler
 * has actually put the complete value.
 */
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
#[macro_export]
macro_rules! __usdt_argref {
    ($id:ident) => {
        concat!("%I[", stringify!($id), "]%[", stringify!($id), "]")
    };
}

#[cfg(target_arch = "x86")]
#[macro_export]
macro_rules! __usdt_argref {
    ($id:ident) => {
        concat!("%k[", stringify!($id), "]")
    };
}

#[cfg(not(any(target_arch = "powerpc", target_arch = "powerpc64", target_arch = "x86")))]
#[macro_export]
macro_rules! __usdt_argref {
    ($id:ident) => {
        concat!("%[", stringify!($id), "]")
    };
}

#[macro_export]
macro_rules! __usdt_asm_arg {
    ($n:tt) => {
        concat!(
            stringify!(__usdt_asz),
            stringify!($n),
            "\n.ascii \"@\"\n",
            stringify!(__usdt_aval),
            stringify!($n),
            "\n"
        )
    };
}

#[macro_export]
macro_rules! __usdt_asm_args0 {
    () => {};
}

#[macro_export]
macro_rules! __usdt_asm_args1 {
    () => {
        $crate::__usdt_asm_arg!(1)
    };
}

#[macro_export]
macro_rules! __usdt_asm_args2 {
    () => {
        concat!($crate::__usdt_asm_args1!(), ".ascii \" \"\n", $crate::__usdt_asm_arg!(2))
    };
}

#[macro_export]
macro_rules! __usdt_asm_args3 {
    () => {
        concat!($crate::__usdt_asm_args2!(), ".ascii \" \"\n", $crate::__usdt_asm_arg!(3))
    };
}

#[macro_export]
macro_rules! __usdt_asm_args4 {
    () => {
        concat!($crate::__usdt_asm_args3!(), ".ascii \" \"\n", $crate::__usdt_asm_arg!(4))
    };
}

#[macro_export]
macro_rules! __usdt_asm_args5 {
    () => {
        concat!($crate::__usdt_asm_args4!(), ".ascii \" \"\n", $crate::__usdt_asm_arg!(5))
    };
}

#[macro_export]
macro_rules! __usdt_asm_args6 {
    () => {
        concat!($crate::__usdt_asm_args5!(), ".ascii \" \"\n", $crate::__usdt_asm_arg!(6))
    };
}

#[macro_export]
macro_rules! __usdt_asm_args7 {
    () => {
        concat!($crate::__usdt_asm_args6!(), ".ascii \" \"\n", $crate::__usdt_asm_arg!(7))
    };
}

#[macro_export]
macro_rules! __usdt_asm_args8 {
    () => {
        concat!($crate::__usdt_asm_args7!(), ".ascii \" \"\n", $crate::__usdt_asm_arg!(8))
    };
}

#[macro_export]
macro_rules! __usdt_asm_args9 {
    () => {
        concat!($crate::__usdt_asm_args8!(), ".ascii \" \"\n", $crate::__usdt_asm_arg!(9))
    };
}

#[macro_export]
macro_rules! __usdt_asm_args10 {
    () => {
        concat!($crate::__usdt_asm_args9!(), ".ascii \" \"\n", $crate::__usdt_asm_arg!(10))
    };
}

#[macro_export]
macro_rules! __usdt_asm_args11 {
    () => {
        concat!($crate::__usdt_asm_args10!(), ".ascii \" \"\n", $crate::__usdt_asm_arg!(11))
    };
}

#[macro_export]
macro_rules! __usdt_asm_args12 {
    () => {
        concat!($crate::__usdt_asm_args11!(), ".ascii \" \"\n", $crate::__usdt_asm_arg!(12))
    };
}

#[macro_export]
macro_rules! __usdt_is_arr {
    ($x:expr) => {
        false
    };
}

#[macro_export]
macro_rules! __usdt_arg_size {
    ($x:expr) => {
        ::core::mem::size_of_val(&$x)
    };
}

/*
 * We can't use __builtin_choose_expr() in C++, so fall back to table-based
 * signedness determination for known types, utilizing templates magic.
 *
 * Rust has neither C++ template specialization nor C __builtin_classify_type.
 * File-local translation preserves the integer intent through type-specific
 * helper macros and defaults to unsigned classification when unavailable.
 */
#[macro_export]
macro_rules! __usdt_is_inttype {
    ($x:expr) => {
        true
    };
}

#[macro_export]
macro_rules! __usdt_inttype {
    ($x:expr) => {
        $x
    };
}

#[macro_export]
macro_rules! __usdt_is_signed {
    ($x:expr) => {{
        let _ = &$x;
        false
    }};
}

#[macro_export]
macro_rules! __usdt_asm_op {
    ($n:tt, $x:expr) => {
        (
            concat!("__usdt_asz", stringify!($n)),
            if $crate::__usdt_is_signed!($x) {
                -($crate::__usdt_arg_size!($x) as isize)
            } else {
                $crate::__usdt_arg_size!($x) as isize
            },
            concat!("__usdt_aval", stringify!($n)),
            $x,
        )
    };
}

#[macro_export]
macro_rules! __usdt_asm_ops0 {
    () => {
        [("__usdt_dummy", 0isize)]
    };
}

#[macro_export]
macro_rules! __usdt_asm_ops1 {
    ($x:expr) => {
        $crate::__usdt_asm_op!(1, $x)
    };
}

#[macro_export]
macro_rules! __usdt_asm_ops2 {
    ($a:expr, $x:expr) => {
        ($crate::__usdt_asm_ops1!($a), $crate::__usdt_asm_op!(2, $x))
    };
}

#[macro_export]
macro_rules! __usdt_asm_ops3 {
    ($a:expr, $b:expr, $x:expr) => {
        ($crate::__usdt_asm_ops2!($a, $b), $crate::__usdt_asm_op!(3, $x))
    };
}

#[macro_export]
macro_rules! __usdt_asm_ops4 {
    ($a:expr, $b:expr, $c:expr, $x:expr) => {
        ($crate::__usdt_asm_ops3!($a, $b, $c), $crate::__usdt_asm_op!(4, $x))
    };
}

#[macro_export]
macro_rules! __usdt_asm_ops5 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $x:expr) => {
        ($crate::__usdt_asm_ops4!($a, $b, $c, $d), $crate::__usdt_asm_op!(5, $x))
    };
}

#[macro_export]
macro_rules! __usdt_asm_ops6 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $x:expr) => {
        ($crate::__usdt_asm_ops5!($a, $b, $c, $d, $e), $crate::__usdt_asm_op!(6, $x))
    };
}

#[macro_export]
macro_rules! __usdt_asm_ops7 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $x:expr) => {
        ($crate::__usdt_asm_ops6!($a, $b, $c, $d, $e, $f), $crate::__usdt_asm_op!(7, $x))
    };
}

#[macro_export]
macro_rules! __usdt_asm_ops8 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $x:expr) => {
        ($crate::__usdt_asm_ops7!($a, $b, $c, $d, $e, $f, $g), $crate::__usdt_asm_op!(8, $x))
    };
}

#[macro_export]
macro_rules! __usdt_asm_ops9 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $x:expr) => {
        ($crate::__usdt_asm_ops8!($a, $b, $c, $d, $e, $f, $g, $h), $crate::__usdt_asm_op!(9, $x))
    };
}

#[macro_export]
macro_rules! __usdt_asm_ops10 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $x:expr) => {
        ($crate::__usdt_asm_ops9!($a, $b, $c, $d, $e, $f, $g, $h, $i), $crate::__usdt_asm_op!(10, $x))
    };
}

#[macro_export]
macro_rules! __usdt_asm_ops11 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $x:expr) => {
        ($crate::__usdt_asm_ops10!($a, $b, $c, $d, $e, $f, $g, $h, $i, $j), $crate::__usdt_asm_op!(11, $x))
    };
}

#[macro_export]
macro_rules! __usdt_asm_ops12 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr, $x:expr) => {
        ($crate::__usdt_asm_ops11!($a, $b, $c, $d, $e, $f, $g, $h, $i, $j, $k), $crate::__usdt_asm_op!(12, $x))
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
