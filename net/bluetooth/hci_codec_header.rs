/* SPDX-License-Identifier: GPL-2.0 */

/* Copyright (C) 2014 Intel Corporation */

#[repr(C)]
pub struct hci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

extern "C" {
    pub fn hci_read_supported_codecs(hdev: *mut hci_dev);
    pub fn hci_read_supported_codecs_v2(hdev: *mut hci_dev);
    pub fn hci_codec_list_clear(codec_list: *mut list_head);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
