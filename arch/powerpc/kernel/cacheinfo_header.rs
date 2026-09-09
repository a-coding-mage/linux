/* SPDX-License-Identifier: GPL-2.0 */

/* These are just hooks for sysfs.c to use. */
extern "C" {
    pub fn cacheinfo_cpu_online(cpu_id: ::core::ffi::c_uint);
    pub fn cacheinfo_cpu_offline(cpu_id: ::core::ffi::c_uint);
}

/* Allow migration/suspend to tear down and rebuild the hierarchy. */
extern "C" {
    pub fn cacheinfo_teardown();
    pub fn cacheinfo_rebuild();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
