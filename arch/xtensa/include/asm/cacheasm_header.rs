/*
 * Rust translation of include/asm-xtensa/cacheasm.h.
 *
 * The source file defines Xtensa assembler macros.  Rust has no direct
 * equivalent for assembler .macro/.if/.rep directives, so the macro bodies
 * are retained as assembler-oriented token macros and preserve the original
 * conditional intent and ordering.
 */

// Dependencies supplied by the original header environment:
// <asm/cache.h>, <asm/asmmacro.h>, and <linux/stringify.h>.

macro_rules! __loop_cache_unroll {
    ($ar:tt, $at:tt, $insn:tt, $size:tt, $line_width:tt, $max_immed:tt) => {{
        // Original assembler logic:
        // if (1 << line_width) > max_immed: reps = 1;
        // else if (2 << line_width) > max_immed: reps = 2;
        // else: reps = 4;
        // __loopi ar, at, size, (reps << line_width);
        // for index = 0 .. reps: insn ar, index << line_width;
        // __endla ar, at, reps << line_width;
    }};
}

macro_rules! __loop_cache_all {
    ($ar:tt, $at:tt, $insn:tt, $size:tt, $line_width:tt, $max_immed:tt) => {{
        // movi ar, 0;
        __loop_cache_unroll!($ar, $at, $insn, $size, $line_width, $max_immed);
    }};
}

macro_rules! __loop_cache_range {
    ($ar:tt, $as:tt, $at:tt, $insn:tt, $line_width:tt) => {{
        // extui at, ar, 0, line_width;
        // add as, as, at;
        // __loops ar, as, at, line_width;
        // insn ar, 0;
        // __endla ar, at, (1 << line_width);
    }};
}

macro_rules! __loop_cache_page {
    ($ar:tt, $at:tt, $insn:tt, $line_width:tt, $max_immed:tt) => {{
        __loop_cache_unroll!($ar, $at, $insn, PAGE_SIZE, $line_width, $max_immed);
    }};
}

// The following feature conditions preserve the original #if expressions.

macro_rules! ___unlock_dcache_all {
    ($ar:tt, $at:tt) => {{
        // #if XCHAL_DCACHE_LINE_LOCKABLE && XCHAL_DCACHE_SIZE
        __loop_cache_all!($ar, $at, diu, XCHAL_DCACHE_SIZE, XCHAL_DCACHE_LINEWIDTH, 240);
        // #endif
    }};
}

macro_rules! ___unlock_icache_all {
    ($ar:tt, $at:tt) => {{
        // #if XCHAL_ICACHE_LINE_LOCKABLE && XCHAL_ICACHE_SIZE
        __loop_cache_all!($ar, $at, iiu, XCHAL_ICACHE_SIZE, XCHAL_ICACHE_LINEWIDTH, 240);
        // #endif
    }};
}

macro_rules! ___flush_invalidate_dcache_all {
    ($ar:tt, $at:tt) => {{
        // #if XCHAL_DCACHE_SIZE
        __loop_cache_all!($ar, $at, diwbi, XCHAL_DCACHE_SIZE, XCHAL_DCACHE_LINEWIDTH, 240);
        // #endif
    }};
}

macro_rules! ___flush_dcache_all {
    ($ar:tt, $at:tt) => {{
        // #if XCHAL_DCACHE_SIZE
        __loop_cache_all!($ar, $at, diwb, XCHAL_DCACHE_SIZE, XCHAL_DCACHE_LINEWIDTH, 240);
        // #endif
    }};
}

macro_rules! ___invalidate_dcache_all {
    ($ar:tt, $at:tt) => {{
        // #if XCHAL_DCACHE_SIZE
        __loop_cache_all!($ar, $at, dii, XCHAL_DCACHE_SIZE, XCHAL_DCACHE_LINEWIDTH, 1020);
        // #endif
    }};
}

macro_rules! ___invalidate_icache_all {
    ($ar:tt, $at:tt) => {{
        // #if XCHAL_ICACHE_SIZE
        __loop_cache_all!($ar, $at, iii, XCHAL_ICACHE_SIZE, XCHAL_ICACHE_LINEWIDTH, 1020);
        // #endif
    }};
}

macro_rules! ___flush_invalidate_dcache_range { ($ar:tt, $as:tt, $at:tt) => {{ __loop_cache_range!($ar, $as, $at, dhwbi, XCHAL_DCACHE_LINEWIDTH); }}; }
macro_rules! ___flush_dcache_range { ($ar:tt, $as:tt, $at:tt) => {{ __loop_cache_range!($ar, $as, $at, dhwb, XCHAL_DCACHE_LINEWIDTH); }}; }
macro_rules! ___invalidate_dcache_range { ($ar:tt, $as:tt, $at:tt) => {{ __loop_cache_range!($ar, $as, $at, dhi, XCHAL_DCACHE_LINEWIDTH); }}; }
macro_rules! ___invalidate_icache_range { ($ar:tt, $as:tt, $at:tt) => {{ __loop_cache_range!($ar, $as, $at, ihi, XCHAL_ICACHE_LINEWIDTH); }}; }

macro_rules! ___flush_invalidate_dcache_page { ($ar:tt, $as:tt) => {{ __loop_cache_page!($ar, $as, dhwbi, XCHAL_DCACHE_LINEWIDTH, 1020); }}; }
macro_rules! ___flush_dcache_page { ($ar:tt, $as:tt) => {{ __loop_cache_page!($ar, $as, dhwb, XCHAL_DCACHE_LINEWIDTH, 1020); }}; }
macro_rules! ___invalidate_dcache_page { ($ar:tt, $as:tt) => {{ __loop_cache_page!($ar, $as, dhi, XCHAL_DCACHE_LINEWIDTH, 1020); }}; }
macro_rules! ___invalidate_icache_page { ($ar:tt, $as:tt) => {{ __loop_cache_page!($ar, $as, ihi, XCHAL_ICACHE_LINEWIDTH, 1020); }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
