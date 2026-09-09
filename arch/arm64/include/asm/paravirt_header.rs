/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_PARAVIRT */
#[cfg(feature = "CONFIG_PARAVIRT")]
extern "C" {
    pub fn pv_time_init();
}

/* Equivalent of the empty pv_time_init() macro when CONFIG_PARAVIRT is unset. */
#[cfg(not(feature = "CONFIG_PARAVIRT"))]
macro_rules! pv_time_init {
    () => {{}};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
