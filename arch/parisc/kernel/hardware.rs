// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Hardware descriptions for HP 9000 based hardware.
 * The constants and shared kernel types referenced here are supplied by the
 * surrounding PARISC translation unit.
 */

#[repr(C)]
pub struct HpHardware {
    pub hw_type: u32,
    pub hversion: u32,
    pub sversion: u32,
    pub rev: u32,
    pub name: &'static str,
}

#[repr(C)]
pub struct PariscDeviceId {
    pub hw_type: u32,
    pub hversion: u32,
    pub sversion: u32,
}

#[repr(C)]
pub struct HpCpuTypeMask<CpuType> {
    pub model: u16,
    pub mask: u16,
    pub cpu: CpuType,
}

/* The complete hardware table is generated from the corresponding C
 * database; entries retain their original ordering and spelling. */
extern "C" {
    static hp_hardware_list: HpHardware;
    static hp_cpu_type_mask_list: HpCpuTypeMask<u32>;
}

extern "C" {
    static cpu_name_version: [[&'static str; 2]; 13];
    fn panic(message: *const u8) -> !;
}

pub unsafe fn parisc_hardware_description(id: *const PariscDeviceId) -> &'static str {
    let mut listptr = &hp_hardware_list as *const HpHardware;
    loop {
        if (*listptr).hw_type == HPHW_FAULTY {
            break;
        }
        if (*listptr).hw_type == (*id).hw_type
            && (*listptr).hversion == (*id).hversion
            && (*listptr).sversion == (*id).sversion
        {
            return (*listptr).name;
        }
        listptr = listptr.add(1);
    }

    match (*id).hw_type {
        HPHW_NPROC => "Unknown machine",
        HPHW_A_DIRECT => match (*id).sversion {
            0x0D => "MUX port",
            0x0E => "RS-232 port",
            _ => "unknown device",
        },
        HPHW_MEMORY => "Memory",
        _ => "unknown device",
    }
}

pub unsafe fn parisc_get_cpu_type(hversion: u64) -> u32 {
    let model = (hversion as u16) >> 4;
    let mut ptr = &hp_cpu_type_mask_list as *const HpCpuTypeMask<u32>;
    loop {
        if (*ptr).mask == 0 {
            panic(b"could not identify CPU type\n\0".as_ptr());
        }
        if (*ptr).model == (model & (*ptr).mask) {
            return (*ptr).cpu;
        }
        ptr = ptr.add(1);
    }
}

extern "C" {
    static HPHW_FAULTY: u32;
    static HPHW_NPROC: u32;
    static HPHW_A_DIRECT: u32;
    static HPHW_MEMORY: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
