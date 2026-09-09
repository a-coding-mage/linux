/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants for most input bindings.
 *
 * Most input bindings include key code, matrix key code format.
 * In most cases, key code and matrix key code format uses
 * the standard values/macro defined in this header.
 */

// Dependency intent: symbols from "linux-event-codes.h" are supplied externally.

macro_rules! MATRIX_KEY {
    ($row:expr, $col:expr, $code:expr) => {
        ((($row & 0xFF) << 24) | (($col & 0xFF) << 16) | ($code & 0xFFFF))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
