/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright © 1999-2010 David Woodhouse <dwmw2@infradead.org>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 *
 */

/* Header guard: __MTD_USER_H__ */

/* This file is blessed for inclusion by userspace */
/* Dependency: <mtd/mtd-abi.h> */

pub type mtd_info_t = mtd_info_user;
pub type erase_info_t = erase_info_user;
pub type region_info_t = region_info_user;
pub type nand_oobinfo_t = nand_oobinfo;
pub type nand_ecclayout_t = nand_ecclayout_user;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
