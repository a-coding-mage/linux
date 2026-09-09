/* SPDX-License-Identifier: GPL-2.0 */

// Translated dependencies: <linux/notifier.h>, <linux/types.h>

extern "C" {
    pub static mut panic_notifier_list: atomic_notifier_head;

    pub static mut crash_kexec_post_notifiers: bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
