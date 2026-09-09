/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the C header guard __ASM_SUSPEND_H.

unsafe extern "C" {
    pub fn loongarch_common_suspend();
    pub fn loongarch_common_resume();
    pub fn loongarch_suspend_enter();
    pub fn loongarch_wakeup_start();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
