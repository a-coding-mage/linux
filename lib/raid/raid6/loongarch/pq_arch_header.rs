/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependency supplied by <asm/cpu-features.h> and other translation units.

extern "C" {
    pub static raid6_lsx: raid6_calls;
    pub static raid6_lasx: raid6_calls;

    pub static raid6_recov_lsx: raid6_recov_calls;
    pub static raid6_recov_lasx: raid6_recov_calls;
}

extern "C" {
    fn raid6_algo_add_default();
    fn raid6_algo_add(algo: *const raid6_calls);
    fn raid6_recov_algo_add(algo: *const raid6_recov_calls);

    static cpu_has_lsx: bool;
    static cpu_has_lasx: bool;
}

/// Architecture-specific RAID6 initialization.
///
/// `IS_ENABLED(CONFIG_CPU_HAS_LSX)` and `IS_ENABLED(CONFIG_CPU_HAS_LASX)` are
/// build-time kernel conditions preserved here as runtime configuration
/// placeholders; their concrete mapping is supplied by the build environment.
#[inline(always)]
pub unsafe fn arch_raid6_init() {
    raid6_algo_add_default();

    // IS_ENABLED(CONFIG_CPU_HAS_LSX) && cpu_has_lsx
    if cpu_has_lsx {
        raid6_algo_add(&raid6_lsx as *const raid6_calls);
    }

    // IS_ENABLED(CONFIG_CPU_HAS_LASX) && cpu_has_lasx
    if cpu_has_lasx {
        raid6_algo_add(&raid6_lasx as *const raid6_calls);
    }

    // IS_ENABLED(CONFIG_CPU_HAS_LASX) && cpu_has_lasx
    if cpu_has_lasx {
        raid6_recov_algo_add(&raid6_recov_lasx as *const raid6_recov_calls);
    // else if IS_ENABLED(CONFIG_CPU_HAS_LSX) && cpu_has_lsx
    } else if cpu_has_lsx {
        raid6_recov_algo_add(&raid6_recov_lsx as *const raid6_recov_calls);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
