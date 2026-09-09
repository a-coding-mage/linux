/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_uint;

/*
 * When CONFIG_ARCH_CPUIDLE_HALTPOLL is enabled, the architecture-specific
 * declarations are supplied by <asm/cpuidle_haltpoll.h>.
 */
#[cfg(CONFIG_ARCH_CPUIDLE_HALTPOLL)]
// Dependency supplied by the architecture-specific header.

#[cfg(not(CONFIG_ARCH_CPUIDLE_HALTPOLL))]
#[inline]
pub unsafe fn arch_haltpoll_enable(_cpu: c_uint) {}

#[cfg(not(CONFIG_ARCH_CPUIDLE_HALTPOLL))]
#[inline]
pub unsafe fn arch_haltpoll_disable(_cpu: c_uint) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
