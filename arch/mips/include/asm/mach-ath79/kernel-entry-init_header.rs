/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Atheros AR71XX/AR724X/AR913X specific kernel entry setup
 *
 *  Copyright (C) 2009 Gabor Juhos <juhosg@openwrt.org>
 */

/*
 * Some bootloaders set the 'Kseg0 coherency algorithm' to
 * 'Cacheable, noncoherent, write-through, no write allocate'
 * and this cause performance issues. Let's go and change it to
 * 'Cacheable, noncoherent, write-back, write allocate'
 */
#[macro_export]
macro_rules! kernel_entry_setup {
    () => {{
        unsafe {
            core::arch::asm!(
                "mfc0 {t0}, {cp0_config}",
                "li {t1}, ~{conf_cm_cmask}",
                "and {t0}, {t1}",
                "ori {t0}, {conf_cm_cachable_noncoherent}",
                "mtc0 {t0}, {cp0_config}",
                "nop",
                t0 = out(reg) _,
                t1 = out(reg) _,
                cp0_config = const CP0_CONFIG,
                conf_cm_cmask = const CONF_CM_CMASK,
                conf_cm_cachable_noncoherent = const CONF_CM_CACHABLE_NONCOHERENT,
            );
        }
    }};
}

#[macro_export]
macro_rules! smp_slave_setup {
    () => {{}};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
