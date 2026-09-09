/* SPDX-License-Identifier: GPL-2.0 */

/* POSIX mandated with _POSIX_ARG_MAX that we can rely on 4096 chars in the
 * command line, so this choice is ok.
 */
pub const COMMAND_LINE_SIZE: usize = 4096;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
