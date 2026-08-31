/* SPDX-License-Identifier: GPL-2.0 */

// Rust translation of bpf_misc.h.
// C header guards and include-only syntax are intentionally omitted.

#[macro_export]
macro_rules! STR {
    ($s:tt) => {
        stringify!($s)
    };
}

#[macro_export]
macro_rules! XSTR {
    ($s:tt) => {
        stringify!($s)
    };
}

/* Expand a macro and then stringize the expansion */
#[macro_export]
macro_rules! QUOTE {
    ($str:tt) => {
        stringify!($str)
    };
}

#[macro_export]
macro_rules! EXPAND_QUOTE {
    ($str:tt) => {
        stringify!($str)
    };
}

/* This set of attributes controls behavior of the
 * test_loader.c:test_loader__run_subtests().
 *
 * The test_loader sequentially loads each program in a skeleton.
 * Programs could be loaded in privileged and unprivileged modes.
 * - __success, __failure, __msg, __regex imply privileged mode;
 * - __success_unpriv, __failure_unpriv, __msg_unpriv, __regex_unpriv
 *   imply unprivileged mode.
 * If combination of privileged and unprivileged attributes is present
 * both modes are used. If none are present privileged mode is implied.
 *
 * See test_loader.c:drop_capabilities() for exact set of capabilities
 * that differ between privileged and unprivileged modes.
 *
 * For test filtering purposes the name of the program loaded in
 * unprivileged mode is derived from the usual program name by adding
 * `@unpriv' suffix.
 *
 * __msg             Message expected to be found in the verifier log.
 *                   Multiple __msg attributes could be specified.
 *                   To match a regular expression use "{{" "}}" brackets,
 *                   e.g. "foo{{[0-9]+}}"  matches strings like "foo007".
 *                   Extended POSIX regular expression syntax is allowed
 *                   inside the brackets.
 * __not_msg         Message not expected to be found in verifier log.
 *                   If __msg_not is situated between __msg tags
 *                   framework matches __msg tags first, and then
 *                   checks that __msg_not is not present in a portion of
 *                   a log between bracketing __msg tags.
 *                   Same regex syntax as for __msg is supported.
 * __msg_unpriv      Same as __msg but for unprivileged mode.
 * __not_msg_unpriv  Same as __not_msg but for unprivileged mode.
 *
 * __stderr          Message expected to be found in bpf stderr stream. The
 *                   same regex rules apply like __msg.
 * __stderr_unpriv   Same as __stderr but for unpriveleged mode.
 * __stdout          Same as __stderr but for stdout stream.
 * __stdout_unpriv   Same as __stdout but for unpriveleged mode.
 *
 * __xlated          Expect a line in a disassembly log after verifier applies rewrites.
 *                   Multiple __xlated attributes could be specified.
 *                   Regular expressions could be specified same way as in __msg.
 * __xlated_unpriv   Same as __xlated but for unprivileged mode.
 *
 * __jited           Match a line in a disassembly of the jited BPF program.
 *                   Has to be used after __arch_* macro.
 *                   For example:
 *
 *                       __arch_x86_64
 *                       __jited("   endbr64")
 *                       __jited("   nopl    (%rax,%rax)")
 *                       __jited("   xorq    %rax, %rax")
 *                       ...
 *                       __naked void some_test(void)
 *                       {
 *                           asm volatile (... ::: __clobber_all);
 *                       }
 *
 *                   Regular expressions could be included in patterns same way
 *                   as in __msg.
 *
 *                   By default assume that each pattern has to be matched on the
 *                   next consecutive line of disassembly, e.g.:
 *
 *                       __jited("   endbr64")             # matched on line N
 *                       __jited("   nopl    (%rax,%rax)") # matched on line N+1
 *
 *                   If match occurs on a wrong line an error is reported.
 *                   To override this behaviour use literal "...", e.g.:
 *
 *                       __jited("   endbr64")             # matched on line N
 *                       __jited("...")                    # not matched
 *                       __jited("   nopl    (%rax,%rax)") # matched on any line >= N
 *
 * __jited_unpriv    Same as __jited but for unprivileged mode.
 *
 *
 * __success         Expect program load success in privileged mode.
 * __success_unpriv  Expect program load success in unprivileged mode.
 *
 * __failure         Expect program load failure in privileged mode.
 * __failure_unpriv  Expect program load failure in unprivileged mode.
 *
 * __retval          Execute the program using BPF_PROG_TEST_RUN command,
 *                   expect return value to match passed parameter:
 *                   - a decimal number
 *                   - a hexadecimal number, when starts from 0x
 *                   - a macro which expands to one of the above
 *                   - literal _INT_MIN (expands to INT_MIN)
 *                   In addition, two special macros are defined below:
 *                   - POINTER_VALUE
 *                   - TEST_DATA_LEN
 * __retval_unpriv   Same, but load program in unprivileged mode.
 *
 * __description     Text to be used for display and as an additional filter
 *                   alias, while the original program name stays matchable.
 *
 * __log_level       Log level to use for the program, numeric value expected.
 *
 * __flag            Adds one flag use for the program, the following values are valid:
 *                   - BPF_F_STRICT_ALIGNMENT;
 *                   - BPF_F_TEST_RND_HI32;
 *                   - BPF_F_TEST_STATE_FREQ;
 *                   - BPF_F_SLEEPABLE;
 *                   - BPF_F_XDP_HAS_FRAGS;
 *                   - A numeric value.
 *                   Multiple __flag attributes could be specified, the final flags
 *                   value is derived by applying binary "or" to all specified values.
 *
 * __auxiliary         Annotated program is not a separate test, but used as auxiliary
 *                     for some other test cases and should always be loaded.
 * __auxiliary_unpriv  Same, but load program in unprivileged mode.
 *
 * __arch_*          Specify on which architecture the test case should be tested.
 *                   Several __arch_* annotations could be specified at once.
 *                   When test case is not run on current arch it is marked as skipped.
 * __caps_unpriv     Specify the capabilities that should be set when running the test.
 *
 * __linear_size     Specify the size of the linear area of non-linear skbs, or
 *                   0 for linear skbs.
 */

// C btf_decl_tag attributes have no stable source-level Rust equivalent here.
// These macros preserve the annotation payload as string expressions.
#[macro_export]
macro_rules! __test_tag {
    ($tag:expr) => {
        concat!("comment:", "__COUNTER__", ":", $tag)
    };
}

#[macro_export]
macro_rules! __msg {
    ($msg:expr) => {
        $crate::__test_tag!(concat!("test_expect_msg=", $msg))
    };
}
#[macro_export]
macro_rules! __not_msg {
    ($msg:expr) => {
        $crate::__test_tag!(concat!("test_expect_not_msg=", $msg))
    };
}
#[macro_export]
macro_rules! __xlated {
    ($msg:expr) => {
        $crate::__test_tag!(concat!("test_expect_xlated=", $msg))
    };
}
#[macro_export]
macro_rules! __jited {
    ($msg:expr) => {
        $crate::__test_tag!(concat!("test_jited=", $msg))
    };
}
#[macro_export]
macro_rules! __failure {
    () => {
        $crate::__test_tag!("test_expect_failure")
    };
}
#[macro_export]
macro_rules! __success {
    () => {
        $crate::__test_tag!("test_expect_success")
    };
}
#[macro_export]
macro_rules! __description {
    ($desc:expr) => {
        $crate::__test_tag!(concat!("test_description=", $desc))
    };
}
#[macro_export]
macro_rules! __msg_unpriv {
    ($msg:expr) => {
        $crate::__test_tag!(concat!("test_expect_msg_unpriv=", $msg))
    };
}
#[macro_export]
macro_rules! __not_msg_unpriv {
    ($msg:expr) => {
        $crate::__test_tag!(concat!("test_expect_not_msg_unpriv=", $msg))
    };
}
#[macro_export]
macro_rules! __xlated_unpriv {
    ($msg:expr) => {
        $crate::__test_tag!(concat!("test_expect_xlated_unpriv=", $msg))
    };
}
#[macro_export]
macro_rules! __jited_unpriv {
    ($msg:expr) => {
        $crate::__test_tag!(concat!("test_jited_unpriv=", $msg))
    };
}
#[macro_export]
macro_rules! __failure_unpriv {
    () => {
        $crate::__test_tag!("test_expect_failure_unpriv")
    };
}
#[macro_export]
macro_rules! __success_unpriv {
    () => {
        $crate::__test_tag!("test_expect_success_unpriv")
    };
}
#[macro_export]
macro_rules! __log_level {
    ($lvl:tt) => {
        $crate::__test_tag!(concat!("test_log_level=", stringify!($lvl)))
    };
}
#[macro_export]
macro_rules! __flag {
    ($flag:tt) => {
        $crate::__test_tag!(concat!("test_prog_flags=", stringify!($flag)))
    };
}
#[macro_export]
macro_rules! __retval {
    ($val:tt) => {
        $crate::__test_tag!(concat!("test_retval=", stringify!($val)))
    };
}
#[macro_export]
macro_rules! __retval_unpriv {
    ($val:tt) => {
        $crate::__test_tag!(concat!("test_retval_unpriv=", stringify!($val)))
    };
}
#[macro_export]
macro_rules! __auxiliary {
    () => {
        $crate::__test_tag!("test_auxiliary")
    };
}
#[macro_export]
macro_rules! __auxiliary_unpriv {
    () => {
        $crate::__test_tag!("test_auxiliary_unpriv")
    };
}
#[macro_export]
macro_rules! __btf_path {
    ($path:expr) => {
        $crate::__test_tag!(concat!("test_btf_path=", $path))
    };
}
#[macro_export]
macro_rules! __btf_func_path {
    ($path:expr) => {
        $crate::__test_tag!(concat!("test_btf_func_path=", $path))
    };
}
#[macro_export]
macro_rules! __arch {
    ($arch:expr) => {
        $crate::__test_tag!(concat!("test_arch=", $arch))
    };
}
#[macro_export]
macro_rules! __arch_x86_64 {
    () => {
        $crate::__arch!("X86_64")
    };
}
#[macro_export]
macro_rules! __arch_arm64 {
    () => {
        $crate::__arch!("ARM64")
    };
}
#[macro_export]
macro_rules! __arch_riscv64 {
    () => {
        $crate::__arch!("RISCV64")
    };
}
#[macro_export]
macro_rules! __arch_s390x {
    () => {
        $crate::__arch!("s390x")
    };
}
#[macro_export]
macro_rules! __arch_loongarch {
    () => {
        $crate::__arch!("LOONGARCH")
    };
}
#[macro_export]
macro_rules! __caps_unpriv {
    ($caps:tt) => {
        $crate::__test_tag!(concat!("test_caps_unpriv=", stringify!($caps)))
    };
}
#[macro_export]
macro_rules! __load_if_JITed {
    () => {
        $crate::__test_tag!("load_mode=jited")
    };
}
#[macro_export]
macro_rules! __load_if_no_JITed {
    () => {
        $crate::__test_tag!("load_mode=no_jited")
    };
}
#[macro_export]
macro_rules! __stderr {
    ($msg:expr) => {
        $crate::__test_tag!(concat!("test_expect_stderr=", $msg))
    };
}
#[macro_export]
macro_rules! __stderr_unpriv {
    ($msg:expr) => {
        $crate::__test_tag!(concat!("test_expect_stderr_unpriv=", $msg))
    };
}
#[macro_export]
macro_rules! __stdout {
    ($msg:expr) => {
        $crate::__test_tag!(concat!("test_expect_stdout=", $msg))
    };
}
#[macro_export]
macro_rules! __stdout_unpriv {
    ($msg:expr) => {
        $crate::__test_tag!(concat!("test_expect_stdout_unpriv=", $msg))
    };
}
#[macro_export]
macro_rules! __linear_size {
    ($sz:tt) => {
        $crate::__test_tag!(concat!("test_linear_size=", stringify!($sz)))
    };
}

/* Define common capabilities tested using __caps_unpriv */
pub const CAP_NET_ADMIN: i32 = 12;
pub const CAP_SYS_ADMIN: i32 = 21;
pub const CAP_PERFMON: i32 = 38;
pub const CAP_BPF: i32 = 39;

/* Convenience macro for use with 'asm volatile' blocks */
// __naked maps to a C naked function attribute.
// __clobber_all, __clobber_common, and __imm* are inline-assembly operand helpers.
pub const __CLOBBER_ALL: &[&str] = &[
    "r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8", "r9", "memory",
];
pub const __CLOBBER_COMMON: &[&str] = &["r0", "r1", "r2", "r3", "r4", "r5", "memory"];

#[macro_export]
macro_rules! sizeof_field {
    ($TYPE:ty, $MEMBER:tt) => {
        ::core::mem::size_of_val(&unsafe { (*(0 as *const $TYPE)).$MEMBER })
    };
}

#[macro_export]
macro_rules! offsetofend {
    ($TYPE:ty, $MEMBER:tt) => {
        ::core::mem::offset_of!($TYPE, $MEMBER) + $crate::sizeof_field!($TYPE, $MEMBER)
    };
}

/* Magic constants used with __retval() */
pub const POINTER_VALUE: i32 = 0xbadcafe;
pub const TEST_DATA_LEN: i32 = 64;

// C __aligned(x) and __used attributes are represented by Rust attributes at use sites.

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub const SYSCALL_WRAPPER: i32 = 1;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub const SYS_PREFIX: &str = "__x64_";

#[cfg(target_arch = "s390x")]
pub const SYSCALL_WRAPPER: i32 = 1;
#[cfg(target_arch = "s390x")]
pub const SYS_PREFIX: &str = "__s390x_";

#[cfg(target_arch = "aarch64")]
pub const SYSCALL_WRAPPER: i32 = 1;
#[cfg(target_arch = "aarch64")]
pub const SYS_PREFIX: &str = "__arm64_";

#[cfg(target_arch = "riscv64")]
pub const SYSCALL_WRAPPER: i32 = 1;
#[cfg(target_arch = "riscv64")]
pub const SYS_PREFIX: &str = "__riscv_";

#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
pub const SYSCALL_WRAPPER: i32 = 1;
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
pub const SYS_PREFIX: &str = "";

#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "s390x",
    target_arch = "aarch64",
    target_arch = "riscv64",
    target_arch = "powerpc",
    target_arch = "powerpc64"
)))]
pub const SYSCALL_WRAPPER: i32 = 0;
#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "s390x",
    target_arch = "aarch64",
    target_arch = "riscv64",
    target_arch = "powerpc",
    target_arch = "powerpc64"
)))]
pub const SYS_PREFIX: &str = "__se_";

/* How many arguments are passed to function in register */
#[cfg(any(target_arch = "x86_64"))]
pub const FUNC_REG_ARG_CNT: i32 = 6;
#[cfg(target_arch = "x86")]
pub const FUNC_REG_ARG_CNT: i32 = 3;
#[cfg(target_arch = "s390x")]
pub const FUNC_REG_ARG_CNT: i32 = 5;
#[cfg(target_arch = "arm")]
pub const FUNC_REG_ARG_CNT: i32 = 4;
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "riscv64"
))]
pub const FUNC_REG_ARG_CNT: i32 = 8;
#[cfg(any(target_arch = "sparc", target_arch = "sparc64"))]
pub const FUNC_REG_ARG_CNT: i32 = 6;
#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "s390x",
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "sparc",
    target_arch = "sparc64",
    target_arch = "riscv64"
)))]
pub const FUNC_REG_ARG_CNT: i32 = 5;

/* make it look to compiler like value is read and written */
#[macro_export]
macro_rules! __sink {
    ($expr:expr) => {
        unsafe {
            ::core::arch::asm!("", inout(reg) $expr);
        }
    };
}

#[macro_export]
macro_rules! ARRAY_SIZE {
    ($x:expr) => {
        ::core::mem::size_of_val(&$x) / ::core::mem::size_of_val(&$x[0])
    };
}

// CAN_USE_GOTOL is defined by the C header for selected BPF target architectures
// when __clang_major__ >= 18.
// CAN_USE_BPF_ST is defined by the C header when __clang_major__ >= 18.
// CAN_USE_LOAD_ACQ_STORE_REL is defined by the C header when __clang_major__ >= 18,
// ENABLE_ATOMICS_TESTS is defined, and the BPF target architecture supports it.

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub const SPEC_V1: bool = true;

#[cfg(target_arch = "x86_64")]
pub const SPEC_V4: bool = true;
