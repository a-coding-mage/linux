/* SPDX-License-Identifier: GPL-2.0 */

// The original C header undefines any prior definitions of these macros.
// Rust macro definitions below provide the local aliases.

macro_rules! IRQ_DISABLE {
    () => { SOFTIRQ_DISABLE };
}

macro_rules! IRQ_ENABLE {
    () => { SOFTIRQ_ENABLE };
}

macro_rules! IRQ_ENTER {
    () => { SOFTIRQ_ENTER };
}

macro_rules! IRQ_EXIT {
    () => { SOFTIRQ_EXIT };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
