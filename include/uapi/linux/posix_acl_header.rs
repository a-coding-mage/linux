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

pub const ACL_UNDEFINED_ID: i32 = -1;

/* a_type field in acl_user_posix_entry_t */
pub const ACL_TYPE_ACCESS: u16 = 0x8000;
pub const ACL_TYPE_DEFAULT: u16 = 0x4000;

/* e_tag entry in struct posix_acl_entry */
pub const ACL_USER_OBJ: u8 = 0x01;
pub const ACL_USER: u8 = 0x02;
pub const ACL_GROUP_OBJ: u8 = 0x04;
pub const ACL_GROUP: u8 = 0x08;
pub const ACL_MASK: u8 = 0x10;
pub const ACL_OTHER: u8 = 0x20;

/* permissions in the e_perm field */
pub const ACL_READ: u8 = 0x04;
pub const ACL_WRITE: u8 = 0x02;
pub const ACL_EXECUTE: u8 = 0x01;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
