/* SPDX-License-Identifier: GPL-2.0 */

// Translation of the C header guard: _ARCH_HALTPOLL_H.

unsafe extern "C" {
    pub fn arch_haltpoll_enable(cpu: u32);
    pub fn arch_haltpoll_disable(cpu: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
