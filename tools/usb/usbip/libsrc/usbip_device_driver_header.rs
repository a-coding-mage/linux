/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2015 Karol Kosik <karo9@interia.eu>
 *		 2015 Samsung Electronics
 * Author:	 Igor Kotrasinski <i.kotrasinsk@samsung.com>
 *
 * Based on tools/usb/usbip/libsrc/usbip_host_driver.c, which is:
 * Copyright (C) 2011 matt mooney <mfm@muteddisk.com>
 *               2005-2007 Takahiro Hirofuchi
 */

// C dependencies: <stdint.h>, "usbip_common.h", "usbip_host_common.h", "list.h".
// Header guard omitted in Rust.

unsafe extern "C" {
    pub static mut device_driver: usbip_host_driver;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
