/* SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note */
/*
 * Copyright (C) 2002 Andreas Gruenbacher <a.gruenbacher@computer.org>
 * Copyright (C) 2016 Red Hat, Inc.
 *
 * This file is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This file is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 */

// Dependency equivalent of the C header: <linux/types.h>.

/* Supported ACL a_version fields */
pub const POSIX_ACL_XATTR_VERSION: u32 = 0x0002;

/* An undefined entry e_id value */
pub const ACL_UNDEFINED_ID: i32 = -1;

#[repr(C)]
pub struct posix_acl_xattr_entry {
    pub e_tag: __le16,
    pub e_perm: __le16,
    pub e_id: __le32,
}

#[repr(C)]
pub struct posix_acl_xattr_header {
    pub a_version: __le32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
