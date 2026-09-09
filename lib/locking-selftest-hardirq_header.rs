/* SPDX-License-Identifier: GPL-2.0 */

// The C preprocessor undefinitions reset any prior definitions before the
// aliases below are established.

macro_rules! IRQ_ENABLE {
    () => { HARDIRQ_ENABLE };
}

macro_rules! IRQ_DISABLE {
    () => { HARDIRQ_DISABLE };
}

macro_rules! IRQ_ENTER {
    () => { HARDIRQ_ENTER };
}

macro_rules! IRQ_EXIT {
    () => { HARDIRQ_EXIT };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
