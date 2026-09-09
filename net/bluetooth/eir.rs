// SPDX-License-Identifier: GPL-2.0
/*
 * BlueZ - Bluetooth protocol stack for Linux
 *
 * Copyright (C) 2021 Intel Corporation
 */

// Dependencies are supplied by the surrounding Bluetooth implementation.

const PNP_INFO_SVCLASS_ID: u16 = 0x1200;

pub unsafe fn eir_append_local_name(hdev: *mut hci_dev, ptr: *mut u8, ad_len: u8) -> u8 {
    let mut short_len: usize;
    let complete_len: usize;
    if (max_adv_len(hdev) - ad_len) < HCI_MAX_SHORT_NAME_LENGTH + 2 { return ad_len; }
    complete_len = strnlen((*hdev).dev_name.as_ptr(), (*hdev).dev_name.len());
    if complete_len != 0 && complete_len <= HCI_MAX_SHORT_NAME_LENGTH as usize {
        return eir_append_data(ptr, ad_len, EIR_NAME_COMPLETE, (*hdev).dev_name.as_ptr(), complete_len);
    }
    short_len = strnlen((*hdev).short_name.as_ptr(), (*hdev).short_name.len());
    if short_len != 0 {
        return eir_append_data(ptr, ad_len, EIR_NAME_SHORT, (*hdev).short_name.as_ptr(), short_len);
    }
    if complete_len != 0 {
        return eir_append_data(ptr, ad_len, EIR_NAME_SHORT, (*hdev).dev_name.as_ptr(), HCI_MAX_SHORT_NAME_LENGTH as usize);
    }
    ad_len
}

pub unsafe fn eir_append_appearance(hdev: *mut hci_dev, ptr: *mut u8, ad_len: u8) -> u8 {
    eir_append_le16(ptr, ad_len, EIR_APPEARANCE, (*hdev).appearance)
}

pub unsafe fn eir_append_service_data(eir: *mut u8, mut eir_len: u16, uuid: u16, data: *mut u8, data_len: u8) -> u8 {
    *eir.add(eir_len as usize) = (core::mem::size_of::<u8>() + core::mem::size_of::<u16>() + data_len as usize) as u8; eir_len += 1;
    *eir.add(eir_len as usize) = EIR_SERVICE_DATA; eir_len += 1;
    put_unaligned_le16(uuid, eir.add(eir_len as usize)); eir_len += 2;
    memcpy(eir.add(eir_len as usize), data, data_len as usize); eir_len += data_len as u16;
    eir_len as u8
}

unsafe fn create_uuid16_list(hdev: *mut hci_dev, data: *mut u8, len: isize) -> *mut u8 {
    let mut ptr = data; let mut uuids_start: *mut u8 = core::ptr::null_mut();
    if len < 4 { return ptr; }
    list_for_each_entry!(uuid, (*hdev).uuids, list, {
        if uuid.size != 16 { continue; }
        let uuid16 = get_unaligned_le16(uuid.uuid.as_ptr().add(12));
        if uuid16 < 0x1100 || uuid16 == PNP_INFO_SVCLASS_ID { continue; }
        if uuids_start.is_null() { uuids_start = ptr; *ptr = 1; *ptr.add(1) = EIR_UUID16_ALL; ptr = ptr.add(2); }
        if ptr.offset_from(data) + 2 > len { *uuids_start.add(1) = EIR_UUID16_SOME; break; }
        *ptr = (uuid16 & 0xff) as u8; *ptr.add(1) = (uuid16 >> 8) as u8; ptr = ptr.add(2); *uuids_start += 2;
    });
    ptr
}

unsafe fn create_uuid32_list(hdev: *mut hci_dev, data: *mut u8, len: isize) -> *mut u8 {
    let mut ptr = data; let mut uuids_start: *mut u8 = core::ptr::null_mut();
    if len < 6 { return ptr; }
    list_for_each_entry!(uuid, (*hdev).uuids, list, {
        if uuid.size != 32 { continue; }
        if uuids_start.is_null() { uuids_start = ptr; *ptr = 1; *ptr.add(1) = EIR_UUID32_ALL; ptr = ptr.add(2); }
        if ptr.offset_from(data) + 4 > len { *uuids_start.add(1) = EIR_UUID32_SOME; break; }
        memcpy(ptr, uuid.uuid.as_ptr().add(12), 4); ptr = ptr.add(4); *uuids_start += 4;
    }); ptr
}

unsafe fn create_uuid128_list(hdev: *mut hci_dev, data: *mut u8, len: isize) -> *mut u8 {
    let mut ptr = data; let mut uuids_start: *mut u8 = core::ptr::null_mut();
    if len < 18 { return ptr; }
    list_for_each_entry!(uuid, (*hdev).uuids, list, {
        if uuid.size != 128 { continue; }
        if uuids_start.is_null() { uuids_start = ptr; *ptr = 1; *ptr.add(1) = EIR_UUID128_ALL; ptr = ptr.add(2); }
        if ptr.offset_from(data) + 16 > len { *uuids_start.add(1) = EIR_UUID128_SOME; break; }
        memcpy(ptr, uuid.uuid.as_ptr(), 16); ptr = ptr.add(16); *uuids_start += 16;
    }); ptr
}

pub unsafe fn eir_create(hdev: *mut hci_dev, data: *mut u8) {
    let mut ptr = data; let mut name_len = strnlen((*hdev).dev_name.as_ptr(), (*hdev).dev_name.len());
    if name_len > 0 { if name_len > 48 { name_len = 48; *ptr.add(1) = EIR_NAME_SHORT; } else { *ptr.add(1) = EIR_NAME_COMPLETE; } *ptr = (name_len + 1) as u8; memcpy(ptr.add(2), (*hdev).dev_name.as_ptr(), name_len); ptr = ptr.add(name_len + 2); }
    if (*hdev).inq_tx_power != HCI_TX_POWER_INVALID { *ptr = 2; *ptr.add(1) = EIR_TX_POWER; *ptr.add(2) = (*hdev).inq_tx_power as u8; ptr = ptr.add(3); }
    if (*hdev).devid_source > 0 { *ptr = 9; *ptr.add(1) = EIR_DEVICE_ID; put_unaligned_le16((*hdev).devid_source, ptr.add(2)); put_unaligned_le16((*hdev).devid_vendor, ptr.add(4)); put_unaligned_le16((*hdev).devid_product, ptr.add(6)); put_unaligned_le16((*hdev).devid_version, ptr.add(8)); ptr = ptr.add(10); }
    ptr = create_uuid16_list(hdev, ptr, HCI_MAX_EIR_LENGTH as isize - ptr.offset_from(data)); ptr = create_uuid32_list(hdev, ptr, HCI_MAX_EIR_LENGTH as isize - ptr.offset_from(data)); let _ = create_uuid128_list(hdev, ptr, HCI_MAX_EIR_LENGTH as isize - ptr.offset_from(data));
}

pub unsafe fn eir_create_per_adv_data(hdev: *mut hci_dev, instance: u8, ptr: *mut u8) -> u8 {
    let mut adv: *mut adv_info = core::ptr::null_mut(); let mut ad_len = 0;
    if instance != 0 { adv = hci_find_adv_instance(hdev, instance); if adv.is_null() { return 0; } }
    if !adv.is_null() { memcpy(ptr, (*adv).per_adv_data.as_ptr(), (*adv).per_adv_data_len as usize); ad_len += (*adv).per_adv_data_len; }
    ad_len
}

pub unsafe fn eir_create_adv_data(hdev: *mut hci_dev, instance: u8, ptr: *mut u8, size: u8) -> u8 {
    let mut adv: *mut adv_info = core::ptr::null_mut(); let mut ad_len = 0; let mut flags = 0;
    if instance != 0 { adv = hci_find_adv_instance(hdev, instance); if adv.is_null() { return 0; } }
    let instance_flags = hci_adv_instance_flags(hdev, instance);
    if !adv.is_null() && eir_get_data((*adv).adv_data.as_ptr(), (*adv).adv_data_len, EIR_FLAGS, core::ptr::null_mut()) { goto_skip_flags(); }
    if instance_flags & MGMT_ADV_FLAG_DISCOV != 0 { flags |= LE_AD_GENERAL; }
    if instance_flags & MGMT_ADV_FLAG_LIMITED_DISCOV != 0 { flags |= LE_AD_LIMITED; }
    if !hci_dev_test_flag(hdev, HCI_BREDR_ENABLED) { flags |= LE_AD_NO_BREDR; }
    if flags != 0 || instance_flags & MGMT_ADV_FLAG_MANAGED_FLAGS != 0 {
        if flags == 0 { flags |= mgmt_get_adv_discov_flags(hdev); }
        if flags != 0 && ad_len + eir_precalc_len(1) + if adv.is_null() { 0 } else { (*adv).adv_data_len } <= size { *ptr=2; *ptr.add(1)=EIR_FLAGS; *ptr.add(2)=flags; ad_len+=3; }
    }
    'skip: {
        if !adv.is_null() { ptr = ptr.add(ad_len as usize); memcpy(ptr, (*adv).adv_data.as_ptr(), (*adv).adv_data_len as usize); ad_len += (*adv).adv_data_len; ptr = ptr.add((*adv).adv_data_len as usize); }
        if instance_flags & MGMT_ADV_FLAG_TX_POWER != 0 { let p = if ext_adv_capable(hdev) && !adv.is_null() { (*adv).tx_power } else { (*hdev).adv_tx_power }; if p != HCI_TX_POWER_INVALID && ad_len + eir_precalc_len(1) <= size { *ptr=2; *ptr.add(1)=EIR_TX_POWER; *ptr.add(2)=p as u8; ad_len+=3; } }
    }
    ad_len
}

unsafe fn create_default_scan_rsp(hdev: *mut hci_dev, ptr: *mut u8) -> u8 { let mut n=0; if (*hdev).appearance != 0 { n=eir_append_appearance(hdev,ptr,n); } eir_append_local_name(hdev,ptr,n) }
pub unsafe fn eir_create_scan_rsp(hdev:*mut hci_dev, instance:u8, ptr:*mut u8)->u8 { if instance==0{return create_default_scan_rsp(hdev,ptr)} let adv=hci_find_adv_instance(hdev,instance); if adv.is_null(){return 0} let mut n=0; if (*adv).flags & MGMT_ADV_FLAG_APPEARANCE != 0 && (*hdev).appearance != 0 {n=eir_append_appearance(hdev,ptr,n)} memcpy(ptr.add(n as usize),(*adv).scan_rsp_data.as_ptr(),(*adv).scan_rsp_len as usize); n+=(*adv).scan_rsp_len; if (*adv).flags & MGMT_ADV_FLAG_LOCAL_NAME != 0 {n=eir_append_local_name(hdev,ptr,n)} n }
pub unsafe fn eir_get_service_data(mut eir:*mut u8, mut eir_len:usize, uuid:u16, len:*mut usize)->*mut core::ffi::c_void { let end=eir.add(eir_len); let mut d=0; while {eir=eir_get_data(eir,eir_len,EIR_SERVICE_DATA,&mut d)} != core::ptr::null_mut() {if get_unaligned_le16(eir)==uuid {if !len.is_null(){*len=d-2} return eir.add(2) as *mut _} eir=eir.add(d); eir_len=end.offset_from(eir) as usize} core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
