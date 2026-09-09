/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Translation of linux/compiler_types.h.
 *
 * This header is chiefly a compiler/preprocessor interface.  Its C
 * preprocessor macros and compiler attributes have no direct, file-local Rust
 * equivalent; their conditional intent is retained below.  The C data layout
 * declarations are translated literally.
 */

/* __has_builtin(x) is defined as 0 when the compiler does not provide it. */
/* Indirect argument-pasting macros: ___PASTE(a, b), __PASTE(a, b). */

/*
 * C23's `auto` versus the GNU __auto_type extension:
 * before C23, the source defines `auto` as `__auto_type`.
 */

/* BTF_TYPE_TAG(value) expands to the btf_type_tag attribute when supported. */

/* linux/compiler-context-analysis.h is an external dependency. */

/*
 * sparse checker address-space qualifiers and checker-only attributes:
 * __kernel, __user, __iomem, __percpu, __rcu, __force, __nocast, __safe,
 * __private, ACCESS_PRIVATE, __chk_user_ptr, and __chk_io_ptr.
 * In non-checker builds these are empty (apart from BTF_TYPE_TAG annotations),
 * and __builtin_warning(x, y...) expands to 1.
 */

/* __KERNEL__ compiler attributes and linux/compiler-*.h are external. */
/* __function_aligned, __cold, __preserve_most, and __retain retain their
 * configuration-dependent compiler-attribute intent in the source comments. */

#[repr(C)]
pub struct ftrace_branch_data {
    pub func: *const core::ffi::c_char,
    pub file: *const core::ffi::c_char,
    pub line: core::ffi::c_uint,
    pub data: ftrace_branch_data_union,
}

#[repr(C)]
pub union ftrace_branch_data_union {
    pub correct_incorrect: ftrace_branch_data_correct_incorrect,
    pub miss_hit_named: ftrace_branch_data_miss_hit,
    pub miss_hit: [core::ffi::c_ulong; 2],
}

#[repr(C)]
pub struct ftrace_branch_data_correct_incorrect {
    pub correct: core::ffi::c_ulong,
    pub incorrect: core::ffi::c_ulong,
}

#[repr(C)]
pub struct ftrace_branch_data_miss_hit {
    pub miss: core::ffi::c_ulong,
    pub hit: core::ffi::c_ulong,
}

#[repr(C)]
pub struct ftrace_likely_data {
    pub data: ftrace_branch_data,
    pub constant: core::ffi::c_ulong,
}

/*
 * notrace selects hotpatch(0, 0), patchable_function_entry(0, 0), or
 * __no_instrument_function__ according to compiler configuration.
 * __naked, inline/__inline__, __inline_maybe_unused, noinline_for_stack,
 * noinline_for_tracing, sanitizer helper attributes, __assume, __counted_by,
 * __counted_by_ptr, __nonstring_array, and endian-specific counted_by macros
 * are compiler/preprocessor attributes and are intentionally retained as
 * configuration intent rather than represented by misleading Rust attributes.
 */

/*
 * at_least is `static` outside sparse checker builds and empty for checker
 * builds.  __noinstr_section(section), noinstr, and __cpuidle select the
 * corresponding uninstrumented sections and attributes.
 */

/* __latent_entropy defaults to empty when not supplied by a compiler plugin. */
/*
 * Under RANDSTRUCT (and outside __CHECKER__), __randomize_layout and
 * __no_randomize_layout select the corresponding attributes and randomized
 * struct field delimiters. Otherwise __randomize_layout is __designated_init
 * and the delimiters are empty.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
