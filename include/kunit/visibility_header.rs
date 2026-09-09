/* SPDX-License-Identifier: GPL-2.0 */
/*
 * KUnit API to allow symbols to be conditionally visible during KUnit
 * testing
 *
 * Copyright (C) 2022, Google LLC.
 * Author: Rae Moar <rmoar@google.com>
 */

/*
 * The original header conditionally changes symbol visibility when
 * CONFIG_KUNIT is enabled.  Rust has no direct equivalent of a declaration
 * modifier macro, so preserve the intent with cfg-selected local macros.
 */
#[cfg(feature = "kunit")]
macro_rules! VISIBLE_IF_KUNIT {
    () => {};
}

#[cfg(not(feature = "kunit"))]
macro_rules! VISIBLE_IF_KUNIT {
    () => { static };
}

/*
 * EXPORT_SYMBOL_IF_KUNIT(symbol) exports into the
 * EXPORTED_FOR_KUNIT_TESTING namespace only when KUnit is enabled.  The
 * actual export mechanism is supplied by the surrounding kernel bindings.
 */
#[cfg(feature = "kunit")]
macro_rules! EXPORT_SYMBOL_IF_KUNIT {
    ($symbol:ident) => {
        $crate::EXPORT_SYMBOL_NS!($symbol, "EXPORTED_FOR_KUNIT_TESTING");
    };
}

#[cfg(not(feature = "kunit"))]
macro_rules! EXPORT_SYMBOL_IF_KUNIT {
    ($symbol:ident) => {};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
