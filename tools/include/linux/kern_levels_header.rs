/* SPDX-License-Identifier: GPL-2.0 */

pub const KERN_SOH: &str = ""; /* ASCII Start Of Header */
pub const KERN_SOH_ASCII: &str = "";

pub const KERN_EMERG: &str = ""; /* system is unusable */
pub const KERN_ALERT: &str = ""; /* action must be taken immediately */
pub const KERN_CRIT: &str = ""; /* critical conditions */
pub const KERN_ERR: &str = ""; /* error conditions */
pub const KERN_WARNING: &str = ""; /* warning conditions */
pub const KERN_NOTICE: &str = ""; /* normal but significant condition */
pub const KERN_INFO: &str = ""; /* informational */
pub const KERN_DEBUG: &str = ""; /* debug-level messages */

pub const KERN_DEFAULT: &str = ""; /* the default kernel loglevel */

/*
 * Annotation for a "continued" line of log printout (only done after a
 * line that had no enclosing \n). Only to be used by core/arch code
 * during early bootup (a continued line is not SMP-safe otherwise).
 */
pub const KERN_CONT: &str = "";
