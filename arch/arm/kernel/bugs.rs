// SPDX-License-Identifier: GPL-2.0

// External declarations supplied by the surrounding kernel code.
unsafe extern "C" {
    fn check_writebuffer_bugs();
    static mut cpu_check_bugs: Option<unsafe extern "C" fn()>;
}

pub unsafe extern "C" fn check_other_bugs() {
    // Preserved from the C build-time MULTI_CPU condition.
    #[cfg(MULTI_CPU)]
    {
        if let Some(check) = cpu_check_bugs {
            check();
        }
    }
}

// C __init annotation preserved as intent; initialization linkage is supplied
// by the surrounding kernel build.
pub unsafe extern "C" fn arch_cpu_finalize_init() {
    check_writebuffer_bugs();
    check_other_bugs();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
