// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock LSM - Common constants and helpers
 *
 * Copyright © 2016-2020 Mickaël Salaün <mic@digikod.net>
 * Copyright © 2018-2020 ANSSI
 */

pub const LANDLOCK_NAME: &str = "landlock";

macro_rules! pr_fmt {
    ($fmt:expr) => {
        concat!(LANDLOCK_NAME, ": ", $fmt)
    };
}

macro_rules! BIT_INDEX {
    ($bit:expr) => {
        HWEIGHT(($bit) - 1)
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
