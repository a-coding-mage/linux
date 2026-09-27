/* SPDX-License-Identifier: GPL-2.0 */

pub const KERN_SOH: &str = "\u{1}"; /* ASCII Start Of Header */
pub const KERN_SOH_ASCII: u8 = 1;

pub const KERN_EMERG: &str = concat!("\u{1}", "0"); /* system is unusable */
pub const KERN_ALERT: &str = concat!("\u{1}", "1"); /* action must be taken immediately */
pub const KERN_CRIT: &str = concat!("\u{1}", "2"); /* critical conditions */
pub const KERN_ERR: &str = concat!("\u{1}", "3"); /* error conditions */
pub const KERN_WARNING: &str = concat!("\u{1}", "4"); /* warning conditions */
pub const KERN_NOTICE: &str = concat!("\u{1}", "5"); /* normal but significant condition */
pub const KERN_INFO: &str = concat!("\u{1}", "6"); /* informational */
pub const KERN_DEBUG: &str = concat!("\u{1}", "7"); /* debug-level messages */

pub const KERN_DEFAULT: &str = ""; /* the default kernel loglevel */

/*
 * Annotation for a "continued" line of log printout (only done after a
 * line that had no enclosing \\n+). Only to be used by core/arch code
 * during early bootup (a continued line is not SMP-safe otherwise).
 */
pub const KERN_CONT: &str = concat!("\u{1}", "c");

/*
 * C builds messages by string-literal concatenation (KERN_ERR "text");
 * these literal forms let Rust do the same with concat!().
 */
#[macro_export]
macro_rules! KERN_SOH {
    () => {
        "\x01"
    };
}
#[macro_export]
macro_rules! KERN_EMERG {
    () => {
        "\x010"
    };
}
#[macro_export]
macro_rules! KERN_ALERT {
    () => {
        "\x011"
    };
}
#[macro_export]
macro_rules! KERN_CRIT {
    () => {
        "\x012"
    };
}
#[macro_export]
macro_rules! KERN_ERR {
    () => {
        "\x013"
    };
}
#[macro_export]
macro_rules! KERN_WARNING {
    () => {
        "\x014"
    };
}
#[macro_export]
macro_rules! KERN_NOTICE {
    () => {
        "\x015"
    };
}
#[macro_export]
macro_rules! KERN_INFO {
    () => {
        "\x016"
    };
}
#[macro_export]
macro_rules! KERN_DEBUG {
    () => {
        "\x017"
    };
}
#[macro_export]
macro_rules! KERN_DEFAULT {
    () => {
        ""
    };
}
#[macro_export]
macro_rules! KERN_CONT {
    () => {
        "\x01c"
    };
}

/* integer equivalents of KERN_<LEVEL> */
pub const LOGLEVEL_SCHED: i32 = -2; /* Deferred messages from sched code
                                      * are set to this special level */
pub const LOGLEVEL_DEFAULT: i32 = -1; /* default (or last) loglevel */
pub const LOGLEVEL_EMERG: i32 = 0; /* system is unusable */
pub const LOGLEVEL_ALERT: i32 = 1; /* action must be taken immediately */
pub const LOGLEVEL_CRIT: i32 = 2; /* critical conditions */
pub const LOGLEVEL_ERR: i32 = 3; /* error conditions */
pub const LOGLEVEL_WARNING: i32 = 4; /* warning conditions */
pub const LOGLEVEL_NOTICE: i32 = 5; /* normal but significant condition */
pub const LOGLEVEL_INFO: i32 = 6; /* informational */
pub const LOGLEVEL_DEBUG: i32 = 7; /* debug-level messages */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
