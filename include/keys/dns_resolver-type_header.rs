/* SPDX-License-Identifier: GPL-2.0-or-later */
/* DNS resolver key type
 *
 * Copyright (C) 2010 Wang Lei. All Rights Reserved.
 * Written by Wang Lei (wang840925@gmail.com)
 */

// Dependency: <linux/key-type.h>

extern "C" {
    pub static mut key_type_dns_resolver: key_type;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
