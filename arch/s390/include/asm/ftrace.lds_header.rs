/* SPDX-License-Identifier: GPL-2.0 */

// C linker-script macro translated as a const function.
pub const fn div_round_up(n: usize, d: usize) -> usize {
    (n + d - 1) / d
}

pub const SIZEOF_MCOUNT_LOC_ENTRY: usize = 8;
pub const SIZEOF_FTRACE_HOTPATCH_TRAMPOLINE: usize = 24;

pub const fn ftrace_hotpatch_trampolines_size(n: usize) -> usize {
    div_round_up(
        SIZEOF_FTRACE_HOTPATCH_TRAMPOLINE * n,
        SIZEOF_MCOUNT_LOC_ENTRY,
    )
}

// The original definition is linker-script text and has no direct executable
// Rust equivalent. Preserve its conditional intent and emitted symbols here.
#[cfg(feature = "function_tracer")]
pub const FTRACE_HOTPATCH_TRAMPOLINES_TEXT: &str =
    ". = ALIGN(8); __ftrace_hotpatch_trampolines_start = .; \\\n+. = . + FTRACE_HOTPATCH_TRAMPOLINES_SIZE(__stop_mcount_loc - __start_mcount_loc); \\\n+__ftrace_hotpatch_trampolines_end = .;";

#[cfg(not(feature = "function_tracer"))]
pub const FTRACE_HOTPATCH_TRAMPOLINES_TEXT: &str = "";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
