/* SPDX-License-Identifier: GPL-2.0 */

// _ASM_POWERPC_CONTEXT_TRACKING_H

#[cfg(CONFIG_CONTEXT_TRACKING_USER)]
macro_rules! SCHEDULE_USER {
    () => {
        bl schedule_user
    };
}

#[cfg(not(CONFIG_CONTEXT_TRACKING_USER))]
macro_rules! SCHEDULE_USER {
    () => {
        bl schedule
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
