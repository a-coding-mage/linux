/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Defines for what uname() should return
 */

/* Corresponds to the C preprocessor fallback for UTS_SYSNAME. */
pub const UTS_SYSNAME: &str = "Linux";

/*
 * Corresponds to CONFIG_DEFAULT_HOSTNAME (set by sethostname()).
 * The build configuration supplying CONFIG_DEFAULT_HOSTNAME is external.
 */
pub const UTS_NODENAME: &str = CONFIG_DEFAULT_HOSTNAME;

/* Corresponds to the C preprocessor fallback for UTS_DOMAINNAME. */
pub const UTS_DOMAINNAME: &str = "(none)"; /* set by setdomainname() */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
