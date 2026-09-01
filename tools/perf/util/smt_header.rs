/* SPDX-License-Identifier: GPL-2.0 */

use std::os::raw::c_char;

extern "C" {
    /*
     * Returns true if SMT (aka hyperthreading) is enabled. Determined via sysfs or
     * the online topology.
     */
    pub fn smt_on() -> bool;

    /*
     * Returns true when system wide and all SMT threads for a core are in the
     * user_requested_cpus map.
     */
    pub fn core_wide(system_wide: bool, user_requested_cpu_list: *const c_char) -> bool;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
