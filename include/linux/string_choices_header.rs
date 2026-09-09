/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Here provide a series of helpers in the str_$TRUE_$FALSE format (you can
 * also expand some helpers as needed), where $TRUE and $FALSE are their
 * corresponding literal strings. These helpers can be used in the printing
 * and also in other places where constant strings are required. Using these
 * helpers offers the following benefits:
 *  1) Reducing the hardcoding of strings, which makes the code more elegant
 *     through these simple literal-meaning helpers.
 *  2) Unifying the output, which prevents the same string from being printed
 *     in various forms, such as enable/disable, enabled/disabled, en/dis.
 *  3) Deduping by the linker, which results in a smaller binary file.
 */

#[inline]
fn str_assert_deassert(v: bool) -> &'static str {
    if v { "assert" } else { "deassert" }
}
macro_rules! str_deassert_assert {
    ($v:expr) => { str_assert_deassert(!$v) };
}

#[inline]
fn str_enable_disable(v: bool) -> &'static str {
    if v { "enable" } else { "disable" }
}
macro_rules! str_disable_enable {
    ($v:expr) => { str_enable_disable(!$v) };
}

#[inline]
fn str_enabled_disabled(v: bool) -> &'static str {
    if v { "enabled" } else { "disabled" }
}
macro_rules! str_disabled_enabled {
    ($v:expr) => { str_enabled_disabled(!$v) };
}

#[inline]
fn str_hi_lo(v: bool) -> &'static str {
    if v { "hi" } else { "lo" }
}
macro_rules! str_lo_hi {
    ($v:expr) => { str_hi_lo(!$v) };
}

#[inline]
fn str_high_low(v: bool) -> &'static str {
    if v { "high" } else { "low" }
}
macro_rules! str_low_high {
    ($v:expr) => { str_high_low(!$v) };
}

#[inline]
fn str_input_output(v: bool) -> &'static str {
    if v { "input" } else { "output" }
}
macro_rules! str_output_input {
    ($v:expr) => { str_input_output(!$v) };
}

#[inline]
fn str_on_off(v: bool) -> &'static str {
    if v { "on" } else { "off" }
}
macro_rules! str_off_on {
    ($v:expr) => { str_on_off(!$v) };
}

#[inline]
fn str_read_write(v: bool) -> &'static str {
    if v { "read" } else { "write" }
}
macro_rules! str_write_read {
    ($v:expr) => { str_read_write(!$v) };
}

#[inline]
fn str_true_false(v: bool) -> &'static str {
    if v { "true" } else { "false" }
}
macro_rules! str_false_true {
    ($v:expr) => { str_true_false(!$v) };
}

#[inline]
fn str_up_down(v: bool) -> &'static str {
    if v { "up" } else { "down" }
}
macro_rules! str_down_up {
    ($v:expr) => { str_up_down(!$v) };
}

#[inline]
fn str_yes_no(v: bool) -> &'static str {
    if v { "yes" } else { "no" }
}
macro_rules! str_no_yes {
    ($v:expr) => { str_yes_no(!$v) };
}

/**
 * str_plural - Return the simple pluralization based on English counts
 * @num: Number used for deciding pluralization
 *
 * If @num is 1, returns empty string, otherwise returns "s".
 */
#[inline]
fn str_plural(num: usize) -> &'static str {
    if num == 1 { "" } else { "s" }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
