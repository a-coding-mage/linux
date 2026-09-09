/* SPDX-License-Identifier: GPL-2.0-only */

// Rust translation of the linker-script header.  The original declarations
// are linker-script fragments; they are retained as string-producing macros.

#[cfg(feature = "CONFIG_MEM_ALLOC_PROFILING")]
macro_rules! IF_MEM_ALLOC_PROFILING {
    ($($items:tt)*) => { $($items)* };
}

#[cfg(not(feature = "CONFIG_MEM_ALLOC_PROFILING"))]
macro_rules! IF_MEM_ALLOC_PROFILING {
    ($($items:tt)*) => {};
}

macro_rules! SECTION_WITH_BOUNDARIES {
    ($name:ident) => {
        concat!(
            ". = ALIGN(8);\n",
            "__start_", stringify!($name), " = .;\n",
            "KEEP(*(", stringify!($name), "))\n",
            "__stop_", stringify!($name), " = .;\n",
        )
    };
}

macro_rules! CODETAG_SECTIONS {
    () => {
        IF_MEM_ALLOC_PROFILING!(SECTION_WITH_BOUNDARIES!(alloc_tags))
    };
}

macro_rules! MOD_SEPARATE_CODETAG_SECTION {
    ($name:ident) => {
        concat!(
            ".codetag.", stringify!($name), " 0 : {\n",
            SECTION_WITH_BOUNDARIES!($name),
            "}\n",
        )
    };
}

/*
 * For codetags which might be used after module unload, therefore might stay
 * longer in memory. Each such codetag type has its own section so that we can
 * unload them individually once unused.
 */
macro_rules! MOD_SEPARATE_CODETAG_SECTIONS {
    () => {
        IF_MEM_ALLOC_PROFILING!(MOD_SEPARATE_CODETAG_SECTION!(alloc_tags))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
