/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

/*
 * Note that bpf programs need to include either
 * vmlinux.h (auto-generated from BTF) or linux/types.h
 * in advance since bpf_helper_defs.h uses such types
 * as __u64.
 *
 * C dependency: bpf_helper_defs.h
 */

/*
 * C map-definition helper macros:
 *   #define __uint(name, val) int (*name)[val]
 *   #define __type(name, val) typeof(val) *name
 *   #define __array(name, val) typeof(val) *name[]
 *   #define __ulong(name, val) enum { ___bpf_concat(__unique_value, __COUNTER__) = val } name
 *
 * These expand to C declaration fragments and have no direct Rust item-level
 * equivalent without the surrounding struct declaration.
 */

#[macro_export]
macro_rules! likely {
    ($x:expr) => {
        !!$x
    };
}

#[macro_export]
macro_rules! unlikely {
    ($x:expr) => {
        !!$x
    };
}

/*
 * Helper macro to place programs, maps, license in
 * different sections in elf_bpf file. Section names
 * are interpreted by libbpf depending on the context (BPF programs, BPF maps,
 * extern variables, etc).
 * To allow use of SEC() with externs (e.g., for extern .maps declarations),
 * make sure __attribute__((unused)) doesn't trigger compilation warning.
 *
 * Rust equivalent for concrete items is #[link_section = name] plus #[used]
 * where applicable; a declaration-fragment SEC(name) macro cannot be represented
 * as a stable Rust macro attribute from this header alone.
 */

/* C attributes:
 *   __always_inline: inline __attribute__((always_inline))
 *   __noinline: __attribute__((noinline))
 *   __weak: __attribute__((weak))
 *   __hidden: __attribute__((visibility("hidden")))
 *
 * Rust equivalents are item attributes such as #[inline(always)],
 * #[inline(never)], weak linkage where available for the target/toolchain, and
 * symbol visibility/linkage attributes on concrete items.
 */

pub const NULL: *mut core::ffi::c_void = core::ptr::null_mut();

#[macro_export]
macro_rules! KERNEL_VERSION {
    ($a:expr, $b:expr, $c:expr) => {
        (($a << 16) + ($b << 8) + if $c > 255 { 255 } else { $c })
    };
}

/*
 * Helper macros to manipulate data structures
 */

/* offsetof() definition that uses __builtin_offset() might not preserve field
 * offset CO-RE relocation properly, so force-redefine offsetof() using
 * old-school approach which works with CO-RE correctly
 */
#[macro_export]
macro_rules! offsetof {
    ($type:ty, $member:tt) => {{
        let base = core::ptr::null::<$type>();
        unsafe { core::ptr::addr_of!((*base).$member) as usize }
    }};
}

/* redefined container_of() to ensure we use the above offsetof() macro */
#[macro_export]
macro_rules! container_of {
    ($ptr:expr, $type:ty, $member:tt) => {{
        let __mptr = $ptr as *mut core::ffi::c_void as *mut u8;
        unsafe { __mptr.sub($crate::offsetof!($type, $member)) as *mut $type }
    }};
}

/*
 * Compiler (optimization) barrier.
 */
#[macro_export]
macro_rules! barrier {
    () => {
        unsafe {
            core::arch::asm!("", options(nostack, preserves_flags), clobber_abi("C"));
        }
    };
}

/* Variable-specific compiler (optimization) barrier. It's a no-op which makes
 * compiler believe that there is some black box modification of a given
 * variable and thus prevents compiler from making extra assumption about its
 * value and potential simplifications and optimizations on this variable.
 *
 * E.g., compiler might often delay or even omit 32-bit to 64-bit casting of
 * a variable, making some code patterns unverifiable. Putting barrier_var()
 * in place will ensure that cast is performed before the barrier_var()
 * invocation, because compiler has to pessimistically assume that embedded
 * asm section might perform some extra operations on that variable.
 *
 * This is a variable-specific variant of more global barrier().
 */
#[macro_export]
macro_rules! barrier_var {
    ($var:expr) => {
        unsafe {
            core::arch::asm!("", inout(reg) $var, options(nostack, preserves_flags));
        }
    };
}

/*
 * Helper macro to throw a compilation error if __bpf_unreachable() gets
 * built into the resulting code. This works given BPF back end does not
 * implement __builtin_trap(). This is useful to assert that certain paths
 * of the program code are never used and hence eliminated by the compiler.
 *
 * For example, consider a switch statement that covers known cases used by
 * the program. __bpf_unreachable() can then reside in the default case. If
 * the program gets extended such that a case is not covered in the switch
 * statement, then it will throw a build error due to the default case not
 * being compiled out.
 */
#[macro_export]
macro_rules! __bpf_unreachable {
    () => {
        core::intrinsics::abort()
    };
}

/*
 * Helper function to perform a tail call with a constant/immediate map slot.
 *
 * Original C condition:
 *   #if (defined(__clang__) && __clang_major__ >= 8) || (!defined(__clang__) && __GNUC__ > 12)
 *   #if defined(__bpf__)
 */
#[cfg(target_arch = "bpf")]
#[inline(always)]
pub unsafe fn bpf_tail_call_static(ctx: *mut core::ffi::c_void, map: *const core::ffi::c_void, slot: __u32) {
    /*
     * C requires slot to be a compile-time constant and calls
     * __bpf_unreachable() otherwise. Rust cannot test that file-locally.
     */

    /*
     * Provide a hard guarantee that LLVM won't optimize setting r2 (map
     * pointer) and r3 (constant map index) from _different paths_ ending
     * up at the _same_ call insn as otherwise we won't be able to use the
     * jmpq/nopl retpoline-free patching by the x86-64 JIT in the kernel
     * given they mismatch. See also d2e4c1e6c294 ("bpf: Constant map key
     * tracking for prog array pokes") for details on verifier tracking.
     *
     * Note on clobber list: we need to stay in-line with BPF calling
     * convention, so even if we don't end up using r0, r4, r5, we need
     * to mark them as clobber so that LLVM doesn't end up using them
     * before / after the call.
     */
    core::arch::asm!(
        "r1 = {ctx}",
        "r2 = {map}",
        "r3 = {slot}",
        "call 12",
        ctx = in(reg) ctx,
        map = in(reg) map,
        slot = const slot,
        out("r0") _,
        out("r1") _,
        out("r2") _,
        out("r3") _,
        out("r4") _,
        out("r5") _,
    );
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum libbpf_pin_type {
    LIBBPF_PIN_NONE = 0,
    /* PIN_BY_NAME: pin maps by name (in /sys/fs/bpf by default) */
    LIBBPF_PIN_BY_NAME = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum libbpf_tristate {
    TRI_NO = 0,
    TRI_YES = 1,
    TRI_MODULE = 2,
}

/*
 * C attributes:
 *   __kconfig: __attribute__((section(".kconfig")))
 *   __ksym: __attribute__((section(".ksyms")))
 *   __kptr_untrusted: __attribute__((btf_type_tag("kptr_untrusted")))
 *   __kptr: __attribute__((btf_type_tag("kptr")))
 *   __percpu_kptr: __attribute__((btf_type_tag("percpu_kptr")))
 *   __uptr: __attribute__((btf_type_tag("uptr")))
 */

#[macro_export]
macro_rules! bpf_ksym_exists {
    ($sym:expr) => {
        !!$sym
    };
}

/*
 * C BTF declaration tags:
 *   __arg_ctx: __attribute__((btf_decl_tag("arg:ctx")))
 *   __arg_nonnull: __attribute((btf_decl_tag("arg:nonnull")))
 *   __arg_nullable: __attribute((btf_decl_tag("arg:nullable")))
 *   __arg_trusted: __attribute((btf_decl_tag("arg:trusted")))
 *   __arg_untrusted: __attribute((btf_decl_tag("arg:untrusted")))
 *   __arg_arena: __attribute((btf_decl_tag("arg:arena")))
 */

#[macro_export]
macro_rules! ___bpf_concat {
    ($a:ident, $b:ident) => {
        compile_error!("C token concatenation has no direct Rust expression equivalent")
    };
}

#[macro_export]
macro_rules! ___bpf_apply {
    ($fn:ident, $n:ident) => {
        $crate::___bpf_concat!($fn, $n)
    };
}

#[macro_export]
macro_rules! ___bpf_nth {
    ($_:expr, $_1:expr, $_2:expr, $_3:expr, $_4:expr, $_5:expr, $_6:expr, $_7:expr, $_8:expr, $_9:expr, $_a:expr, $_b:expr, $_c:expr, $N:expr $(, $rest:expr)*) => {
        $N
    };
}

#[macro_export]
macro_rules! ___bpf_narg {
    () => {
        0usize
    };
    ($($args:expr),+ $(,)?) => {
        <[()]>::len(&[$($crate::___bpf_narg!(@sub $args)),+])
    };
    (@sub $arg:expr) => {
        ()
    };
}

#[macro_export]
macro_rules! ___bpf_fill0 {
    ($arr:expr, $p:expr, $x:expr) => {};
}

#[macro_export]
macro_rules! ___bpf_fill1 {
    ($arr:expr, $p:expr, $x:expr) => {
        $arr[$p] = $x as _;
    };
}

#[macro_export]
macro_rules! ___bpf_fill2 {
    ($arr:expr, $p:expr, $x:expr, $($args:expr),+ $(,)?) => {
        $arr[$p] = $x as _;
        $crate::___bpf_fill1!($arr, $p + 1, $($args),+);
    };
}

#[macro_export]
macro_rules! ___bpf_fill3 {
    ($arr:expr, $p:expr, $x:expr, $($args:expr),+ $(,)?) => {
        $arr[$p] = $x as _;
        $crate::___bpf_fill2!($arr, $p + 1, $($args),+);
    };
}

#[macro_export]
macro_rules! ___bpf_fill4 {
    ($arr:expr, $p:expr, $x:expr, $($args:expr),+ $(,)?) => {
        $arr[$p] = $x as _;
        $crate::___bpf_fill3!($arr, $p + 1, $($args),+);
    };
}

#[macro_export]
macro_rules! ___bpf_fill5 {
    ($arr:expr, $p:expr, $x:expr, $($args:expr),+ $(,)?) => {
        $arr[$p] = $x as _;
        $crate::___bpf_fill4!($arr, $p + 1, $($args),+);
    };
}

#[macro_export]
macro_rules! ___bpf_fill6 {
    ($arr:expr, $p:expr, $x:expr, $($args:expr),+ $(,)?) => {
        $arr[$p] = $x as _;
        $crate::___bpf_fill5!($arr, $p + 1, $($args),+);
    };
}

#[macro_export]
macro_rules! ___bpf_fill7 {
    ($arr:expr, $p:expr, $x:expr, $($args:expr),+ $(,)?) => {
        $arr[$p] = $x as _;
        $crate::___bpf_fill6!($arr, $p + 1, $($args),+);
    };
}

#[macro_export]
macro_rules! ___bpf_fill8 {
    ($arr:expr, $p:expr, $x:expr, $($args:expr),+ $(,)?) => {
        $arr[$p] = $x as _;
        $crate::___bpf_fill7!($arr, $p + 1, $($args),+);
    };
}

#[macro_export]
macro_rules! ___bpf_fill9 {
    ($arr:expr, $p:expr, $x:expr, $($args:expr),+ $(,)?) => {
        $arr[$p] = $x as _;
        $crate::___bpf_fill8!($arr, $p + 1, $($args),+);
    };
}

#[macro_export]
macro_rules! ___bpf_fill10 {
    ($arr:expr, $p:expr, $x:expr, $($args:expr),+ $(,)?) => {
        $arr[$p] = $x as _;
        $crate::___bpf_fill9!($arr, $p + 1, $($args),+);
    };
}

#[macro_export]
macro_rules! ___bpf_fill11 {
    ($arr:expr, $p:expr, $x:expr, $($args:expr),+ $(,)?) => {
        $arr[$p] = $x as _;
        $crate::___bpf_fill10!($arr, $p + 1, $($args),+);
    };
}

#[macro_export]
macro_rules! ___bpf_fill12 {
    ($arr:expr, $p:expr, $x:expr, $($args:expr),+ $(,)?) => {
        $arr[$p] = $x as _;
        $crate::___bpf_fill11!($arr, $p + 1, $($args),+);
    };
}

#[macro_export]
macro_rules! ___bpf_fill {
    ($arr:expr $(,)?) => {};
    ($arr:expr, $($args:expr),+ $(,)?) => {{
        let mut ___i = 0usize;
        $(
            $arr[___i] = $args as _;
            ___i += 1;
        )+
        let _ = ___i;
    }};
}

/*
 * BPF_SEQ_PRINTF to wrap bpf_seq_printf to-be-printed values
 * in a structure.
 */
#[macro_export]
macro_rules! BPF_SEQ_PRINTF {
    ($seq:expr, $fmt:expr $(, $args:expr)* $(,)?) => {{
        static ___FMT: &[u8] = concat!($fmt, "\0").as_bytes();
        let mut ___param: [u64; $crate::___bpf_narg!($($args),*)] = [0; $crate::___bpf_narg!($($args),*)];
        $crate::___bpf_fill!(___param $(, $args)*);
        unsafe {
            bpf_seq_printf(
                $seq,
                ___FMT.as_ptr() as *const core::ffi::c_char,
                ___FMT.len() as _,
                ___param.as_mut_ptr() as *mut core::ffi::c_void,
                core::mem::size_of_val(&___param) as _,
            )
        }
    }};
}

/*
 * BPF_SNPRINTF wraps the bpf_snprintf helper with variadic arguments instead of
 * an array of u64.
 */
#[macro_export]
macro_rules! BPF_SNPRINTF {
    ($out:expr, $out_size:expr, $fmt:expr $(, $args:expr)* $(,)?) => {{
        static ___FMT: &[u8] = concat!($fmt, "\0").as_bytes();
        let mut ___param: [u64; $crate::___bpf_narg!($($args),*)] = [0; $crate::___bpf_narg!($($args),*)];
        $crate::___bpf_fill!(___param $(, $args)*);
        unsafe {
            bpf_snprintf(
                $out,
                $out_size,
                ___FMT.as_ptr() as *const core::ffi::c_char,
                ___param.as_mut_ptr() as *mut core::ffi::c_void,
                core::mem::size_of_val(&___param) as _,
            )
        }
    }};
}

/* Original C conditional:
 *   #ifdef BPF_NO_GLOBAL_DATA
 *   #define BPF_PRINTK_FMT_MOD
 *   #else
 *   #define BPF_PRINTK_FMT_MOD static const
 *   #endif
 */

#[macro_export]
macro_rules! __bpf_printk {
    ($fmt:expr $(, $args:expr)* $(,)?) => {{
        static ____FMT: &[u8] = concat!($fmt, "\0").as_bytes();
        unsafe {
            bpf_trace_printk(
                ____FMT.as_ptr() as *const core::ffi::c_char,
                ____FMT.len() as _,
                $($args),*
            )
        }
    }};
}

/*
 * __bpf_vprintk wraps the bpf_trace_vprintk helper with variadic arguments
 * instead of an array of u64.
 */
#[macro_export]
macro_rules! __bpf_vprintk {
    ($fmt:expr $(, $args:expr)* $(,)?) => {{
        static ___FMT: &[u8] = concat!($fmt, "\0").as_bytes();
        let mut ___param: [u64; $crate::___bpf_narg!($($args),*)] = [0; $crate::___bpf_narg!($($args),*)];
        $crate::___bpf_fill!(___param $(, $args)*);
        unsafe {
            bpf_trace_vprintk(
                ___FMT.as_ptr() as *const core::ffi::c_char,
                ___FMT.len() as _,
                ___param.as_mut_ptr() as *mut core::ffi::c_void,
                core::mem::size_of_val(&___param) as _,
            )
        }
    }};
}

#[macro_export]
macro_rules! bpf_stream_printk {
    ($stream_id:expr, $fmt:expr $(, $args:expr)* $(,)?) => {{
        static ___FMT: &[u8] = concat!($fmt, "\0").as_bytes();
        let mut ___param: [u64; $crate::___bpf_narg!($($args),*)] = [0; $crate::___bpf_narg!($($args),*)];
        $crate::___bpf_fill!(___param $(, $args)*);
        unsafe {
            bpf_stream_vprintk(
                $stream_id,
                ___FMT.as_ptr() as *const core::ffi::c_char,
                ___param.as_mut_ptr() as *mut core::ffi::c_void,
                core::mem::size_of_val(&___param) as _,
            )
        }
    }};
}

/* Use __bpf_printk when bpf_printk call has 3 or fewer fmt args
 * Otherwise use __bpf_vprintk
 */
#[macro_export]
macro_rules! ___bpf_pick_printk {
    () => {
        $crate::__bpf_printk
    };
    ($_1:expr $(,)?) => {
        $crate::__bpf_printk
    };
    ($_1:expr, $_2:expr $(,)?) => {
        $crate::__bpf_printk
    };
    ($_1:expr, $_2:expr, $_3:expr $(,)?) => {
        $crate::__bpf_printk
    };
    ($($args:expr),+ $(,)?) => {
        $crate::__bpf_vprintk
    };
}

/* Helper macro to print out debug messages */
#[macro_export]
macro_rules! bpf_printk {
    ($fmt:expr $(,)?) => {
        $crate::__bpf_printk!($fmt)
    };
    ($fmt:expr, $a1:expr $(,)?) => {
        $crate::__bpf_printk!($fmt, $a1)
    };
    ($fmt:expr, $a1:expr, $a2:expr $(,)?) => {
        $crate::__bpf_printk!($fmt, $a1, $a2)
    };
    ($fmt:expr, $a1:expr, $a2:expr, $a3:expr $(,)?) => {
        $crate::__bpf_printk!($fmt, $a1, $a2, $a3)
    };
    ($fmt:expr, $($args:expr),+ $(,)?) => {
        $crate::__bpf_vprintk!($fmt, $($args),+)
    };
}

#[repr(C)]
pub struct bpf_iter_num {
    _private: [u8; 0],
}

extern "C" {
    pub fn bpf_iter_num_new(it: *mut bpf_iter_num, start: core::ffi::c_int, end: core::ffi::c_int) -> core::ffi::c_int;
    pub fn bpf_iter_num_next(it: *mut bpf_iter_num) -> *mut core::ffi::c_int;
    pub fn bpf_iter_num_destroy(it: *mut bpf_iter_num);
}

/*
 * bpf_for_each(iter_type, cur_elem, args...) provides generic construct for
 * using BPF open-coded iterators without having to write mundane explicit
 * low-level loop logic. Instead, it provides for()-like generic construct
 * that can be used pretty naturally. E.g., for some hypothetical cgroup
 * iterator, you'd write:
 *
 * struct cgroup *cg, *parent_cg = <...>;
 *
 * bpf_for_each(cgroup, cg, parent_cg, CG_ITER_CHILDREN) {
 *     bpf_printk("Child cgroup id = %d", cg->cgroup_id);
 *     if (cg->cgroup_id == 123)
 *         break;
 * }
 *
 * I.e., it looks almost like high-level for each loop in other languages,
 * supports continue/break, and is verifiable by BPF verifier.
 *
 * For iterating integers, the difference between bpf_for_each(num, i, N, M)
 * and bpf_for(i, N, M) is in that bpf_for() provides additional proof to
 * verifier that i is in [N, M) range, and in bpf_for_each() case i is `int
 * *`, not just `int`. So for integers bpf_for() is more convenient.
 *
 * Note: this macro relies on C99 feature of allowing to declare variables
 * inside for() loop, bound to for() loop lifetime. It also utilizes GCC
 * extension: __attribute__((cleanup(<func>))), supported by both GCC and
 * Clang.
 *
 * This C for-loop declaration fragment cannot be represented as a direct Rust
 * macro with arbitrary user loop body and break/continue semantics from this
 * header alone.
 */

/*
 * bpf_for(i, start, end) implements a for()-like looping construct that sets
 * provided integer variable *i* to values starting from *start* through,
 * but not including, *end*. It also proves to BPF verifier that *i* belongs
 * to range [start, end), so this can be used for accessing arrays without
 * extra checks.
 *
 * Note: *start* and *end* are assumed to be expressions with no side effects
 * and whose values do not change throughout bpf_for() loop execution. They do
 * not have to be statically known or constant, though.
 *
 * Note: similarly to bpf_for_each(), it relies on C99 feature of declaring for()
 * loop bound variables and cleanup attribute, supported by GCC and Clang.
 */
#[macro_export]
macro_rules! bpf_for {
    ($i:ident, $start:expr, $end:expr, $body:block) => {{
        let mut ___it = core::mem::MaybeUninit::<$crate::bpf_iter_num>::uninit();
        unsafe {
            bpf_iter_num_new(___it.as_mut_ptr(), $start, $end);
            let ___it = ___it.as_mut_ptr();
            loop {
                let ___t = bpf_iter_num_next(___it);
                if ___t.is_null() {
                    break;
                }
                $i = *___t;
                if !(($i) >= ($start) && ($i) < ($end)) {
                    break;
                }
                $body
            }
            bpf_iter_num_destroy(___it);
        }
    }};
}

/*
 * bpf_repeat(N) performs N iterations without exposing iteration number
 *
 * Note: similarly to bpf_for_each(), it relies on C99 feature of declaring for()
 * loop bound variables and cleanup attribute, supported by GCC and Clang.
 */
#[macro_export]
macro_rules! bpf_repeat {
    ($N:expr, $body:block) => {{
        let mut ___it = core::mem::MaybeUninit::<$crate::bpf_iter_num>::uninit();
        unsafe {
            bpf_iter_num_new(___it.as_mut_ptr(), 0, $N);
            let ___it = ___it.as_mut_ptr();
            while !bpf_iter_num_next(___it).is_null() {
                $body
            }
            bpf_iter_num_destroy(___it);
        }
    }};
}
