/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <uapi/linux/personality.h>

/*
 * Return the base personality without flags.
 */
macro_rules! personality {
    ($pers:expr) => {
        ($pers & PER_MASK)
    };
}

/*
 * Change personality of the currently running process.
 *
 * `current` and its `personality` field are supplied by the surrounding
 * kernel translation.
 */
macro_rules! set_personality {
    ($pers:expr) => {
        (current.personality = $pers)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
