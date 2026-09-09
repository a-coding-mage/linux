// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2005 Dmitry Torokhov
 */

// Dependencies supplied by the corresponding USB, input, and byte-order
// headers are intentionally referenced but not defined here.

#[inline]
pub unsafe fn usb_to_input_id(dev: *const usb_device, id: *mut input_id) {
    (*id).bustype = BUS_USB;
    (*id).vendor = le16_to_cpu((*dev).descriptor.idVendor);
    (*id).product = le16_to_cpu((*dev).descriptor.idProduct);
    (*id).version = le16_to_cpu((*dev).descriptor.bcdDevice);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
