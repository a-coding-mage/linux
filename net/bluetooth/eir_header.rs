/* SPDX-License-Identifier: GPL-2.0 */
/*
 * BlueZ - Bluetooth protocol stack for Linux
 *
 * Copyright (C) 2021 Intel Corporation
 */

// Dependency intent: linux/unaligned.h

#[repr(C)]
pub struct hci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

extern "C" {
    pub fn eir_create(hdev: *mut hci_dev, data: *mut u8);

    pub fn eir_create_adv_data(
        hdev: *mut hci_dev,
        instance: u8,
        ptr: *mut u8,
        size: u8,
    ) -> u8;
    pub fn eir_create_scan_rsp(hdev: *mut hci_dev, instance: u8, ptr: *mut u8) -> u8;
    pub fn eir_create_per_adv_data(hdev: *mut hci_dev, instance: u8, ptr: *mut u8) -> u8;

    pub fn eir_append_local_name(hdev: *mut hci_dev, eir: *mut u8, ad_len: u8) -> u8;
    pub fn eir_append_appearance(hdev: *mut hci_dev, ptr: *mut u8, ad_len: u8) -> u8;
    pub fn eir_append_service_data(
        eir: *mut u8,
        eir_len: u16,
        uuid: u16,
        data: *mut u8,
        data_len: u8,
    ) -> u8;
}

#[inline]
pub fn eir_precalc_len(data_len: u8) -> u16 {
    2u16.wrapping_add(data_len as u16)
}

#[inline]
pub unsafe fn eir_append_data(
    eir: *mut u8,
    mut eir_len: u16,
    type_: u8,
    data: *mut u8,
    data_len: u8,
) -> u16 {
    *eir.add(eir_len as usize) = 1u8.wrapping_add(data_len);
    eir_len = eir_len.wrapping_add(1);
    *eir.add(eir_len as usize) = type_;
    eir_len = eir_len.wrapping_add(1);
    core::ptr::copy_nonoverlapping(data, eir.add(eir_len as usize), data_len as usize);
    eir_len.wrapping_add(data_len as u16)
}

#[inline]
pub unsafe fn eir_append_le16(
    eir: *mut u8,
    mut eir_len: u16,
    type_: u8,
    data: u16,
) -> u16 {
    *eir.add(eir_len as usize) = 3;
    eir_len = eir_len.wrapping_add(1);
    *eir.add(eir_len as usize) = type_;
    eir_len = eir_len.wrapping_add(1);
    let bytes = data.to_le_bytes();
    core::ptr::copy_nonoverlapping(bytes.as_ptr(), eir.add(eir_len as usize), 2);
    eir_len.wrapping_add(2)
}

extern "C" {
    pub fn skb_put(skb: *mut sk_buff, len: u16) -> *mut u8;
    pub fn WARN_ON(condition: bool) -> bool;
}

#[inline]
pub unsafe fn eir_skb_put_data(
    skb: *mut sk_buff,
    type_: u8,
    data: *mut u8,
    data_len: u8,
) -> u16 {
    let eir_len = eir_precalc_len(data_len);
    let eir = skb_put(skb, eir_len);
    WARN_ON(1u16.wrapping_add(data_len as u16) > u8::MAX as u16);
    *eir = 1u8.wrapping_add(data_len);
    *eir.add(1) = type_;
    core::ptr::copy_nonoverlapping(data, eir.add(2), data_len as usize);
    eir_len
}

#[inline]
pub unsafe fn eir_get_data(
    mut eir: *mut u8,
    eir_len: usize,
    type_: u8,
    data_len: *mut usize,
) -> *mut core::ffi::c_void {
    let mut parsed: usize = 0;

    if eir_len < 2 {
        return core::ptr::null_mut();
    }

    while parsed < eir_len - 1 {
        let field_len = *eir;

        if field_len == 0 {
            break;
        }

        parsed += field_len as usize + 1;

        if parsed > eir_len {
            break;
        }

        if *eir.add(1) != type_ {
            eir = eir.add(field_len as usize + 1);
            continue;
        }

        /* Zero length data */
        if field_len == 1 {
            return core::ptr::null_mut();
        }

        if !data_len.is_null() {
            *data_len = (field_len - 1) as usize;
        }

        return eir.add(2) as *mut core::ffi::c_void;
    }

    core::ptr::null_mut()
}

extern "C" {
    pub fn eir_get_service_data(
        eir: *mut u8,
        eir_len: usize,
        uuid: u16,
        len: *mut usize,
    ) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
