/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by linux/ratelimit.h.
use crate::ratelimit_state;

extern "C" {
    pub static mut net_ratelimit_state: ratelimit_state;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
