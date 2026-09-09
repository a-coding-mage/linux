/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Jailhouse paravirt detection
 *
 * Copyright (c) Siemens AG, 2015-2017
 *
 * Authors:
 *  Jan Kiszka <jan.kiszka@siemens.com>
 */

// Corresponds to CONFIG_JAILHOUSE_GUEST.
#[cfg(CONFIG_JAILHOUSE_GUEST)]
unsafe extern "C" {
    pub fn jailhouse_paravirt() -> bool;
}

#[cfg(not(CONFIG_JAILHOUSE_GUEST))]
#[inline]
pub fn jailhouse_paravirt() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
