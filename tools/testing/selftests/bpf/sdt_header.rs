/* <sys/sdt.h> - Systemtap static probe definition macros.

   This file is dedicated to the public domain, pursuant to CC0
   (https://creativecommons.org/publicdomain/zero/1.0/)
*/

/*
  This file defines a family of macros

       STAP_PROBEn(op1, ..., opn)

  that emit a nop into the instruction stream, and some data into an auxiliary
  note section.  The data in the note section describes the operands, in terms
  of size and location.  Each location is encoded as assembler operand string.
  Consumer tools such as gdb or systemtap insert breakpoints on top of
  the nop, and decode the location operand-strings, like an assembler,
  to find the values being passed.

  The operand strings are selected by the compiler for each operand.
  They are constrained by gcc inline-assembler codes.  The default is:

  #define STAP_SDT_ARG_CONSTRAINT nor

  This is a good default if the operands tend to be integral and
  moderate in number (smaller than number of registers).  In other
  cases, the compiler may report "'asm' requires impossible reload" or
  similar.  In this case, consider simplifying the macro call (fewer
  and simpler operands), reduce optimization, or override the default
  constraints string via:

  #define STAP_SDT_ARG_CONSTRAINT g
  #include <sys/sdt.h>

  See also:
  https://sourceware.org/systemtap/wiki/UserSpaceProbeImplementation
  https://gcc.gnu.org/onlinedocs/gcc/Constraints.html
 */

// C header guards and include directives are intentionally not executable Rust.
// The C __ASSEMBLER__, __cplusplus, compiler-version, architecture, and
// sdt-config.h preprocessor branches are represented below as macro behavior
// and comments where they have no file-local Rust equivalent.

#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
pub const STAP_SDT_ARG_CONSTRAINT: &str = "nZr";
#[cfg(target_arch = "arm")]
pub const STAP_SDT_ARG_CONSTRAINT: &str = "g";
#[cfg(target_arch = "loongarch64")]
pub const STAP_SDT_ARG_CONSTRAINT: &str = "nmr";
#[cfg(not(any(
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "arm",
    target_arch = "loongarch64"
)))]
pub const STAP_SDT_ARG_CONSTRAINT: &str = "nor";

pub const _SDT_NOTE_NAME: &str = "stapsdt";
pub const _SDT_NOTE_TYPE: u32 = 3;

#[cfg(target_pointer_width = "64")]
pub const _SDT_ASM_ADDR: &str = ".8byte";
#[cfg(not(target_pointer_width = "64"))]
pub const _SDT_ASM_ADDR: &str = ".4byte";

#[cfg(any(target_arch = "ia64", target_arch = "s390x"))]
pub const _SDT_NOP: &str = "nop 0";
#[cfg(not(any(target_arch = "ia64", target_arch = "s390x")))]
pub const _SDT_NOP: &str = "nop";

// sdt-config.h supplies _SDT_ASM_SECTION_AUTOGROUP_SUPPORT in C. Rust has no
// file-local equivalent here, so the non-autogroup spelling is used.
pub const _SDT_ASM_AUTOGROUP: &str = "";

unsafe extern "C" {
    pub static mut __sdt_unsp: u64;
}

#[macro_export]
macro_rules! _SDT_ASM_1 {
    ($x:expr) => {
        concat!($x, "\n")
    };
}

#[macro_export]
macro_rules! _SDT_ASM_2 {
    ($a:expr, $b:expr) => {
        concat!($a, ",", $b, "\n")
    };
}

#[macro_export]
macro_rules! _SDT_ASM_3 {
    ($a:expr, $b:expr, $c:expr) => {
        concat!($a, ",", $b, ",", $c, "\n")
    };
}

#[macro_export]
macro_rules! _SDT_ASM_5 {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
        concat!($a, ",", $b, ",", $c, ",", $d, ",", $e, "\n")
    };
}

#[macro_export]
macro_rules! _SDT_ASM_STRING {
    ($x:tt) => {
        concat!(".asciz \"", stringify!($x), "\"\n")
    };
}

#[macro_export]
macro_rules! _SDT_ASM_SUBSTR {
    ($x:expr) => {
        concat!(".ascii \"", $x, "\"\n")
    };
}

#[macro_export]
macro_rules! _SDT_ARGTMPL {
    ($id:tt) => {
        concat!("%[", stringify!($id), "]")
    };
}

#[cfg(any(target_arch = "x86"))]
#[macro_export]
macro_rules! _SDT_ARGTMPL {
    ($id:tt) => {
        concat!("%k[", stringify!($id), "]")
    };
}

#[macro_export]
macro_rules! _SDT_ARGFMT {
    ($no:tt) => {
        concat!("/* _SDT_ARGFMT(", stringify!($no), "): GCC operand size/sign/type encoding is not directly expressible in Rust macro_rules. */\n")
    };
}

#[macro_export]
macro_rules! _SDT_ASM_TEMPLATE {
    (0) => {
        ""
    };
    (1) => {
        $crate::_SDT_ARGFMT!(1)
    };
    (2) => {
        concat!($crate::_SDT_ASM_TEMPLATE!(1), " ", $crate::_SDT_ARGFMT!(2))
    };
    (3) => {
        concat!($crate::_SDT_ASM_TEMPLATE!(2), " ", $crate::_SDT_ARGFMT!(3))
    };
    (4) => {
        concat!($crate::_SDT_ASM_TEMPLATE!(3), " ", $crate::_SDT_ARGFMT!(4))
    };
    (5) => {
        concat!($crate::_SDT_ASM_TEMPLATE!(4), " ", $crate::_SDT_ARGFMT!(5))
    };
    (6) => {
        concat!($crate::_SDT_ASM_TEMPLATE!(5), " ", $crate::_SDT_ARGFMT!(6))
    };
    (7) => {
        concat!($crate::_SDT_ASM_TEMPLATE!(6), " ", $crate::_SDT_ARGFMT!(7))
    };
    (8) => {
        concat!($crate::_SDT_ASM_TEMPLATE!(7), " ", $crate::_SDT_ARGFMT!(8))
    };
    (9) => {
        concat!($crate::_SDT_ASM_TEMPLATE!(8), " ", $crate::_SDT_ARGFMT!(9))
    };
    (10) => {
        concat!($crate::_SDT_ASM_TEMPLATE!(9), " ", $crate::_SDT_ARGFMT!(10))
    };
    (11) => {
        concat!($crate::_SDT_ASM_TEMPLATE!(10), " ", $crate::_SDT_ARGFMT!(11))
    };
    (12) => {
        concat!($crate::_SDT_ASM_TEMPLATE!(11), " ", $crate::_SDT_ARGFMT!(12))
    };
}

#[macro_export]
macro_rules! _SDT_DEF_MACROS {
    () => {
        concat!(
            ".altmacro\n",
            ".macro _SDT_SIGN x\n",
            ".pushsection .note.stapsdt,\"\",\"note\"\n",
            ".iflt \\x\n",
            ".ascii \"-\"\n",
            ".endif\n",
            ".popsection\n",
            ".endm\n",
            ".macro _SDT_SIZE_ x\n",
            ".pushsection .note.stapsdt,\"\",\"note\"\n",
            ".ascii \"\\x\"\n",
            ".popsection\n",
            ".endm\n",
            ".macro _SDT_SIZE x\n",
            "_SDT_SIZE_ %((-(-\\x*((-\\x>0)-(-\\x<0))))>>8)\n",
            ".endm\n",
            ".macro _SDT_TYPE_ x\n",
            ".pushsection .note.stapsdt,\"\",\"note\"\n",
            ".ifc 8,\\x\n",
            ".ascii \"f\"\n",
            ".endif\n",
            ".ascii \"@\"\n",
            ".popsection\n",
            ".endm\n",
            ".macro _SDT_TYPE x\n",
            "_SDT_TYPE_ %((\\x)&(0xff))\n",
            ".endm\n",
        )
    };
}

#[macro_export]
macro_rules! _SDT_UNDEF_MACROS {
    () => {
        concat!(
            ".purgem _SDT_SIGN\n",
            ".purgem _SDT_SIZE_\n",
            ".purgem _SDT_SIZE\n",
            ".purgem _SDT_TYPE_\n",
            ".purgem _SDT_TYPE\n",
        )
    };
}

#[macro_export]
macro_rules! _SDT_SEMAPHORE {
    ($provider:tt, $name:tt) => {
        concat!(".8byte 0\n")
    };
}

#[macro_export]
macro_rules! _SDT_ASM_BODY {
    ($provider:tt, $name:tt, $n:tt) => {
        concat!(
            $crate::_SDT_DEF_MACROS!(),
            "990:\tnop\n",
            ".pushsection .note.stapsdt,\"\",\"note\"\n",
            ".balign 4\n",
            ".4byte 992f-991f, 994f-993f, 3\n",
            "991:\t.asciz \"stapsdt\"\n",
            "992:\t.balign 4\n",
            "993:\t",
            ".8byte",
            " 990b\n",
            "\t",
            ".8byte",
            " _.stapsdt.base\n",
            $crate::_SDT_SEMAPHORE!($provider, $name),
            ".asciz \"",
            stringify!($provider),
            "\"\n",
            ".asciz \"",
            stringify!($name),
            "\"\n",
            $crate::_SDT_ASM_TEMPLATE!($n),
            ".ascii \"\\x00\"\n",
            $crate::_SDT_UNDEF_MACROS!(),
            "994:\t.balign 4\n",
            ".popsection\n",
        )
    };
}

#[macro_export]
macro_rules! _SDT_ASM_BASE {
    () => {
        concat!(
            ".ifndef _.stapsdt.base\n",
            ".pushsection .stapsdt.base,\"aG\",\"progbits\",.stapsdt.base,comdat\n",
            ".weak _.stapsdt.base\n",
            ".hidden _.stapsdt.base\n",
            "_.stapsdt.base: .space 1\n",
            ".size _.stapsdt.base, 1\n",
            ".popsection\n",
            ".endif\n",
        )
    };
}

#[macro_export]
macro_rules! _SDT_PROBE {
    ($provider:tt, $name:tt, $n:tt $(, $arg:expr)*) => {{
        $(let _ = &$arg;)*
        unsafe {
            core::arch::asm!(
                concat!($crate::_SDT_ASM_BODY!($provider, $name, $n), $crate::_SDT_ASM_BASE!()),
                options(nostack, preserves_flags)
            );
        }
    }};
}

#[macro_export]
macro_rules! STAP_PROBE {
    ($provider:tt, $name:tt) => {
        $crate::_SDT_PROBE!($provider, $name, 0)
    };
}

#[macro_export]
macro_rules! STAP_PROBE1 {
    ($provider:tt, $name:tt, $arg1:expr) => {
        $crate::_SDT_PROBE!($provider, $name, 1, $arg1)
    };
}

#[macro_export]
macro_rules! STAP_PROBE2 {
    ($provider:tt, $name:tt, $arg1:expr, $arg2:expr) => {
        $crate::_SDT_PROBE!($provider, $name, 2, $arg1, $arg2)
    };
}

#[macro_export]
macro_rules! STAP_PROBE3 {
    ($provider:tt, $name:tt, $arg1:expr, $arg2:expr, $arg3:expr) => {
        $crate::_SDT_PROBE!($provider, $name, 3, $arg1, $arg2, $arg3)
    };
}

#[macro_export]
macro_rules! STAP_PROBE4 {
    ($provider:tt, $name:tt, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
        $crate::_SDT_PROBE!($provider, $name, 4, $arg1, $arg2, $arg3, $arg4)
    };
}

#[macro_export]
macro_rules! STAP_PROBE5 {
    ($provider:tt, $name:tt, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {
        $crate::_SDT_PROBE!($provider, $name, 5, $arg1, $arg2, $arg3, $arg4, $arg5)
    };
}

#[macro_export]
macro_rules! STAP_PROBE6 {
    ($provider:tt, $name:tt, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {
        $crate::_SDT_PROBE!($provider, $name, 6, $arg1, $arg2, $arg3, $arg4, $arg5, $arg6)
    };
}

#[macro_export]
macro_rules! STAP_PROBE7 {
    ($provider:tt, $name:tt, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr, $arg7:expr) => {
        $crate::_SDT_PROBE!($provider, $name, 7, $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7)
    };
}

#[macro_export]
macro_rules! STAP_PROBE8 {
    ($provider:tt, $name:tt, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr, $arg7:expr, $arg8:expr) => {
        $crate::_SDT_PROBE!($provider, $name, 8, $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8)
    };
}

#[macro_export]
macro_rules! STAP_PROBE9 {
    ($provider:tt, $name:tt, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr, $arg7:expr, $arg8:expr, $arg9:expr) => {
        $crate::_SDT_PROBE!($provider, $name, 9, $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8, $arg9)
    };
}

#[macro_export]
macro_rules! STAP_PROBE10 {
    ($provider:tt, $name:tt, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr, $arg7:expr, $arg8:expr, $arg9:expr, $arg10:expr) => {
        $crate::_SDT_PROBE!($provider, $name, 10, $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8, $arg9, $arg10)
    };
}

#[macro_export]
macro_rules! STAP_PROBE11 {
    ($provider:tt, $name:tt, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr, $arg7:expr, $arg8:expr, $arg9:expr, $arg10:expr, $arg11:expr) => {
        $crate::_SDT_PROBE!($provider, $name, 11, $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8, $arg9, $arg10, $arg11)
    };
}

#[macro_export]
macro_rules! STAP_PROBE12 {
    ($provider:tt, $name:tt, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr, $arg7:expr, $arg8:expr, $arg9:expr, $arg10:expr, $arg11:expr, $arg12:expr) => {
        $crate::_SDT_PROBE!($provider, $name, 12, $arg1, $arg2, $arg3, $arg4, $arg5, $arg6, $arg7, $arg8, $arg9, $arg10, $arg11, $arg12)
    };
}

#[macro_export]
macro_rules! STAP_PROBEV {
    ($provider:tt, $name:tt $(, $arg:expr)*) => {
        $crate::_SDT_PROBE!($provider, $name, 0 $(, $arg)*)
    };
}

#[macro_export]
macro_rules! STAP_PROBE_ASM {
    ($provider:tt, $name:tt $(, $args:tt)*) => {
        concat!($crate::_SDT_ASM_BODY!($provider, $name, 0), $crate::_SDT_ASM_BASE!())
    };
}

#[macro_export]
macro_rules! STAP_PROBE_ASM_OPERANDS {
    ($n:tt $(, $arg:expr)*) => {
        ()
    };
}

#[macro_export]
macro_rules! STAP_PROBE_ASM_TEMPLATE {
    ($n:tt) => {
        concat!($crate::_SDT_ASM_TEMPLATE!($n), ",\"use _SDT_ASM_TEMPLATE_\"")
    };
}

#[macro_export]
macro_rules! DTRACE_PROBE {
    ($provider:tt, $probe:tt) => {
        $crate::STAP_PROBE!($provider, $probe)
    };
}

#[macro_export]
macro_rules! DTRACE_PROBE1 {
    ($provider:tt, $probe:tt, $parm1:expr) => {
        $crate::STAP_PROBE1!($provider, $probe, $parm1)
    };
}

#[macro_export]
macro_rules! DTRACE_PROBE2 {
    ($provider:tt, $probe:tt, $parm1:expr, $parm2:expr) => {
        $crate::STAP_PROBE2!($provider, $probe, $parm1, $parm2)
    };
}

#[macro_export]
macro_rules! DTRACE_PROBE3 {
    ($provider:tt, $probe:tt, $parm1:expr, $parm2:expr, $parm3:expr) => {
        $crate::STAP_PROBE3!($provider, $probe, $parm1, $parm2, $parm3)
    };
}

#[macro_export]
macro_rules! DTRACE_PROBE4 {
    ($provider:tt, $probe:tt, $parm1:expr, $parm2:expr, $parm3:expr, $parm4:expr) => {
        $crate::STAP_PROBE4!($provider, $probe, $parm1, $parm2, $parm3, $parm4)
    };
}

#[macro_export]
macro_rules! DTRACE_PROBE5 {
    ($provider:tt, $probe:tt, $parm1:expr, $parm2:expr, $parm3:expr, $parm4:expr, $parm5:expr) => {
        $crate::STAP_PROBE5!($provider, $probe, $parm1, $parm2, $parm3, $parm4, $parm5)
    };
}

#[macro_export]
macro_rules! DTRACE_PROBE6 {
    ($provider:tt, $probe:tt, $parm1:expr, $parm2:expr, $parm3:expr, $parm4:expr, $parm5:expr, $parm6:expr) => {
        $crate::STAP_PROBE6!($provider, $probe, $parm1, $parm2, $parm3, $parm4, $parm5, $parm6)
    };
}

#[macro_export]
macro_rules! DTRACE_PROBE7 {
    ($provider:tt, $probe:tt, $parm1:expr, $parm2:expr, $parm3:expr, $parm4:expr, $parm5:expr, $parm6:expr, $parm7:expr) => {
        $crate::STAP_PROBE7!($provider, $probe, $parm1, $parm2, $parm3, $parm4, $parm5, $parm6, $parm7)
    };
}

#[macro_export]
macro_rules! DTRACE_PROBE8 {
    ($provider:tt, $probe:tt, $parm1:expr, $parm2:expr, $parm3:expr, $parm4:expr, $parm5:expr, $parm6:expr, $parm7:expr, $parm8:expr) => {
        $crate::STAP_PROBE8!($provider, $probe, $parm1, $parm2, $parm3, $parm4, $parm5, $parm6, $parm7, $parm8)
    };
}

#[macro_export]
macro_rules! DTRACE_PROBE9 {
    ($provider:tt, $probe:tt, $parm1:expr, $parm2:expr, $parm3:expr, $parm4:expr, $parm5:expr, $parm6:expr, $parm7:expr, $parm8:expr, $parm9:expr) => {
        $crate::STAP_PROBE9!($provider, $probe, $parm1, $parm2, $parm3, $parm4, $parm5, $parm6, $parm7, $parm8, $parm9)
    };
}

#[macro_export]
macro_rules! DTRACE_PROBE10 {
    ($provider:tt, $probe:tt, $parm1:expr, $parm2:expr, $parm3:expr, $parm4:expr, $parm5:expr, $parm6:expr, $parm7:expr, $parm8:expr, $parm9:expr, $parm10:expr) => {
        $crate::STAP_PROBE10!($provider, $probe, $parm1, $parm2, $parm3, $parm4, $parm5, $parm6, $parm7, $parm8, $parm9, $parm10)
    };
}

#[macro_export]
macro_rules! DTRACE_PROBE11 {
    ($provider:tt, $probe:tt, $parm1:expr, $parm2:expr, $parm3:expr, $parm4:expr, $parm5:expr, $parm6:expr, $parm7:expr, $parm8:expr, $parm9:expr, $parm10:expr, $parm11:expr) => {
        $crate::STAP_PROBE11!($provider, $probe, $parm1, $parm2, $parm3, $parm4, $parm5, $parm6, $parm7, $parm8, $parm9, $parm10, $parm11)
    };
}

#[macro_export]
macro_rules! DTRACE_PROBE12 {
    ($provider:tt, $probe:tt, $parm1:expr, $parm2:expr, $parm3:expr, $parm4:expr, $parm5:expr, $parm6:expr, $parm7:expr, $parm8:expr, $parm9:expr, $parm10:expr, $parm11:expr, $parm12:expr) => {
        $crate::STAP_PROBE12!($provider, $probe, $parm1, $parm2, $parm3, $parm4, $parm5, $parm6, $parm7, $parm8, $parm9, $parm10, $parm11, $parm12)
    };
}
