/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Generic HDLC support routines for Linux
 *
 * Copyright (C) 1999-2005 Krzysztof Halasa <khc@pm.waw.pl>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of version 2 of the GNU General Public License
 * as published by the Free Software Foundation.
 */

pub const HDLC_MAX_MTU: u32 = 1500; /* Ethernet 1500 bytes */

/*
 * The alternative MRU definition is disabled by the source conditional:
 * HDLC_MAX_MRU (HDLC_MAX_MTU + 10 + 14 + 4), for ETH+VLAN over FR.
 */
pub const HDLC_MAX_MRU: u32 = 1600; /* as required for FR network */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
