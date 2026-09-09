/* SPDX-License-Identifier: GPL-2.0 */

// `CONFIG_PAGE_SHIFT` is supplied by the surrounding build configuration.

/*
 * PAGE_SHIFT determines the page size.
 *
 * Note: This definition is required because PAGE_SHIFT is used
 * in several places throughout the codebase.
 */
pub const PAGE_SHIFT: usize = CONFIG_PAGE_SHIFT;

pub const PAGE_SIZE: usize = 1usize << CONFIG_PAGE_SHIFT;

/*
 * On 32-bit architectures, the C expression uses an int before extension;
 * on 64-bit architectures it masks using PAGE_SIZE.  `usize` preserves the
 * resulting machine-word mask for the target architecture.
 */
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
