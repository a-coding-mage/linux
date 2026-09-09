/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Siemens SIMATIC IPC drivers
 *
 * Copyright (c) Siemens AG, 2018-2023
 *
 * Authors:
 *  Henning Schild <henning.schild@siemens.com>
 *  Gerd Haeussler <gerd.haeussler.ext@siemens.com>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/dmi.h and linux/platform_data/x86/simatic-ipc-base.h

pub const SIMATIC_IPC_DMI_ENTRY_OEM: u8 = 129;
/* binary type */
pub const SIMATIC_IPC_DMI_TYPE: u8 = 0xff;
pub const SIMATIC_IPC_DMI_GROUP: u8 = 0x05;
pub const SIMATIC_IPC_DMI_ENTRY: u8 = 0x02;
pub const SIMATIC_IPC_DMI_TID: u8 = 0x02;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum simatic_ipc_station_ids {
    SIMATIC_IPC_INVALID_STATION_ID = 0,
    SIMATIC_IPC_IPC227D = 0x00000501,
    SIMATIC_IPC_IPC427D = 0x00000701,
    SIMATIC_IPC_IPC227E = 0x00000901,
    SIMATIC_IPC_IPC277E = 0x00000902,
    SIMATIC_IPC_IPC427E = 0x00000A01,
    SIMATIC_IPC_IPC477E = 0x00000A02,
    SIMATIC_IPC_IPC127E = 0x00000D01,
    SIMATIC_IPC_IPC227G = 0x00000F01,
    SIMATIC_IPC_IPC277G = 0x00000F02,
    SIMATIC_IPC_IPCBX_39A = 0x00001001,
    SIMATIC_IPC_IPCPX_39A = 0x00001002,
    SIMATIC_IPC_IPCBX_21A = 0x00001101,
    SIMATIC_IPC_IPCBX_56A = 0x00001201,
    SIMATIC_IPC_IPCBX_59A = 0x00001202,
}

#[repr(C, packed)]
struct simatic_ipc_dmi_data_entry {
    type_: u8, /* type (0xff = binary) */
    len: u8, /* len of data entry */
    group: u8,
    entry: u8,
    tid: u8,
    station_id: u32, /* __le32 station id (LE) */
}

#[inline]
pub unsafe fn simatic_ipc_get_station_id(data: *mut u8, max_len: i32) -> u32 {
    let mut data_entry = data.add(core::mem::size_of::<dmi_header>())
        as *mut simatic_ipc_dmi_data_entry;

    while (data_entry as *mut u8) < data.add(max_len as usize) {
        if (*data_entry).type_ == SIMATIC_IPC_DMI_TYPE
            && (*data_entry).len as usize == core::mem::size_of::<simatic_ipc_dmi_data_entry>()
            && (*data_entry).group == SIMATIC_IPC_DMI_GROUP
            && (*data_entry).entry == SIMATIC_IPC_DMI_ENTRY
            && (*data_entry).tid == SIMATIC_IPC_DMI_TID
        {
            return le32_to_cpu((*data_entry).station_id);
        }
        data_entry = ((data_entry as *mut u8).add((*data_entry).len as usize))
            as *mut simatic_ipc_dmi_data_entry;
    }

    SIMATIC_IPC_INVALID_STATION_ID as u32
}

#[inline]
pub unsafe fn simatic_ipc_find_dmi_entry_helper(dh: *const dmi_header, data: *mut core::ffi::c_void) {
    let id = data as *mut u32;

    if (*dh).type_ != SIMATIC_IPC_DMI_ENTRY_OEM {
        return;
    }

    *id = simatic_ipc_get_station_id(dh as *mut dmi_header, (*dh).length as i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
