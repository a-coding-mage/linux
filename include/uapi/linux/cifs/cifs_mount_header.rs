/* SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note */
/*
 *
 *   Author(s): Scott Lovenberg (scott.lovenberg@gmail.com)
 *
 *   This library is free software; you can redistribute it and/or modify
 *   it under the terms of the GNU Lesser General Public License as published
 *   by the Free Software Foundation; either version 2.1 of the License, or
 *   (at your option) any later version.
 *
 *   This library is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See
 *   the GNU Lesser General Public License for more details.
 */

/* Max string lengths for cifs mounting options. */
pub const CIFS_MAX_DOMAINNAME_LEN: usize = 256; /* max fully qualified domain name */
pub const CIFS_MAX_USERNAME_LEN: usize = 256; /* reasonable max for current servers */
pub const CIFS_MAX_PASSWORD_LEN: usize = 512; /* Windows max seems to be 256 wide chars */
pub const CIFS_MAX_SHARE_LEN: usize = 256; /* reasonable max share name length */
pub const CIFS_NI_MAXHOST: usize = 1024; /* max host name length (256 * 4 bytes) */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
