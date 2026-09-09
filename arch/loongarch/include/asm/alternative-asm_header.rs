/* SPDX-License-Identifier: GPL-2.0 */

//! Rust representation of the assembler-only alternative instruction helpers.
//!
//! The source header is enabled only for the assembler and uses GNU assembler
//! section directives, numeric local labels, and macro arguments containing
//! assembler instructions.  Those constructs have no direct Rust item-level
//! equivalent; the macros below retain their names and arguments while the
//! original assembler operations are documented at their point of use.

/// Descriptor emitted into `.altinstructions` for one alternative.
///
/// Equivalent assembler operation:
///
/// ```text
/// .long orig - .
/// .long alt - .
/// .short feature
/// .byte orig_len
/// .byte alt_len
/// ```
#[macro_export]
macro_rules! altinstruction_entry {
    ($orig:tt, $alt:tt, $feature:expr, $orig_len:expr, $alt_len:expr $(,)?) => {{
        /* Requires assembler section/relocation support; no Rust expression
         * can emit this descriptor in isolation. */
        let _ = ($orig, $alt, $feature, $orig_len, $alt_len);
    }};
}

/// Define an alternative between two assembler instruction sequences.
///
/// The original macro emits the old instruction, pads it with `0x03400000`
/// when necessary, and places an `altinstruction_entry` in `.altinstructions`
/// before emitting the replacement in subsection 1.
#[macro_export]
macro_rules! ALTERNATIVE {
    ($oldinstr:tt, $newinstr:tt, $feature:expr $(,)?) => {{
        /* TODO: emit assembler labels 140--144, `.fill`, `.pushsection`,
         * `.subsection`, and `.previous` when used from an assembler-aware
         * Rust build. */
        let _ = ($oldinstr, $newinstr, $feature);
    }};
}

/// Define an alternative between two replacement instruction sequences.
#[macro_export]
macro_rules! ALTERNATIVE_2 {
    ($oldinstr:tt, $newinstr1:tt, $feature1:expr, $newinstr2:tt, $feature2:expr $(,)?) => {{
        /* TODO: emit the two `.altinstructions` descriptors, assembler labels
         * 140--145, maximum-length padding, and subsection directives. */
        let _ = ($oldinstr, $newinstr1, $feature1, $newinstr2, $feature2);
    }};
}

/// Maximum of two signed assembler lengths without a branch.
#[inline]
pub const fn alt_max_short(a: isize, b: isize) -> isize {
    a ^ ((a ^ b) & -(-((a < b) as isize)))
}

/* The following assembler local-length names are intentionally retained as
 * comments: their values are determined by labels inside each macro
 * expansion, and therefore cannot be represented as file-local Rust constants.
 *
 * old_len  = 141b - 140b
 * new_len1 = 144f - 143f
 * new_len2 = 145f - 144f
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
