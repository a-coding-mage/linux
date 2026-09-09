/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This is used to ensure the compiler did actually allocate the register we
 * asked it for some inline assembly sequences. Apparently we can't trust
 * the compiler from one version to another so a bit of paranoia won't hurt.
 * This string is meant to be concatenated with the inline asm string and
 * will cause compilation to stop on mismatch.
 * (for details, see gcc PR 15089)
 * For compatibility with clang, we have to specifically take the equivalence
 * of 'r11' <-> 'fp' and 'r12' <-> 'ip' into account as well.
 */
#[macro_export]
macro_rules! __asmeq {
    ($x:expr, $y:expr) => {
        concat!(
            ".ifnc ", $x, ",", $y, "; ",
            ".ifnc ", $x, $y, ",fpr11; ",
            ".ifnc ", $x, $y, ",r11fp; ",
            ".ifnc ", $x, $y, ",ipr12; ",
            ".ifnc ", $x, $y, ",r12ip; ",
            ".err; ",
            ".endif; ",
            ".endif; ",
            ".endif; ",
            ".endif; ",
            ".endif\n\t"
        )
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
