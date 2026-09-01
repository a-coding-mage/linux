// SPDX-License-Identifier: GPL-2.0-only
/*
 * bebob_focusrite.c - a part of driver for BeBoB based devices
 *
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

// C dependency: ./bebob.h

use core::ffi::c_void;

type u32 = u32;
type u64 = u64;
type __be32 = u32;

const ANA_IN: &str = "Analog In";
const DIG_IN: &str = "Digital In";
const ANA_OUT: &str = "Analog Out";
const DIG_OUT: &str = "Digital Out";
const STM_IN: &str = "Stream In";

const SAFFIRE_ADDRESS_BASE: u64 = 0x000100000000u64;

const SAFFIRE_OFFSET_CLOCK_SOURCE: u64 = 0x00f8;
const SAFFIREPRO_OFFSET_CLOCK_SOURCE: u64 = 0x0174;

/* whether sync to external device or not */
const SAFFIRE_OFFSET_CLOCK_SYNC_EXT: u64 = 0x013c;
const SAFFIRE_LE_OFFSET_CLOCK_SYNC_EXT: u64 = 0x0432;
const SAFFIREPRO_OFFSET_CLOCK_SYNC_EXT: u64 = 0x0164;

const SAFFIRE_CLOCK_SOURCE_INTERNAL: u32 = 0;
const SAFFIRE_CLOCK_SOURCE_SPDIF: u32 = 1;

/* clock sources as returned from register of Saffire Pro 10 and 26 */
const SAFFIREPRO_CLOCK_SOURCE_SELECT_MASK: u32 = 0x000000ff;
const SAFFIREPRO_CLOCK_SOURCE_DETECT_MASK: u32 = 0x0000ff00;
const SAFFIREPRO_CLOCK_SOURCE_INTERNAL: usize = 0;
const SAFFIREPRO_CLOCK_SOURCE_SKIP: usize = 1; /* never used on hardware */
const SAFFIREPRO_CLOCK_SOURCE_SPDIF: usize = 2;
const SAFFIREPRO_CLOCK_SOURCE_ADAT1: usize = 3; /* not used on s.pro. 10 */
const SAFFIREPRO_CLOCK_SOURCE_ADAT2: usize = 4; /* not used on s.pro. 10 */
const SAFFIREPRO_CLOCK_SOURCE_WORDCLOCK: usize = 5;
const SAFFIREPRO_CLOCK_SOURCE_COUNT: usize = 6;

/* S/PDIF, ADAT1, ADAT2 is enabled or not. three quadlets */
const SAFFIREPRO_ENABLE_DIG_IFACES: u64 = 0x01a4;

/* saffirepro has its own parameter for sampling frequency */
const SAFFIREPRO_RATE_NOREBOOT: u64 = 0x01cc;
/* index is the value for this register */
static rates: [u32; 7] = [0, 44100, 48000, 88200, 96000, 176400, 192000];

/* saffire(no label)/saffire LE has metering */
const SAFFIRE_OFFSET_METER: u64 = 0x0100;
const SAFFIRE_LE_OFFSET_METER: u64 = 0x0168;

const TCODE_READ_BLOCK_REQUEST: i32 = 0; // external constant from dependency
const TCODE_READ_QUADLET_REQUEST: i32 = 0; // external constant from dependency
const TCODE_WRITE_QUADLET_REQUEST: i32 = 0; // external constant from dependency
const EIO: i32 = 5;
const EINVAL: i32 = 22;

#[repr(C)]
pub struct snd_bebob {
    pub unit: *mut c_void,
    pub spec: *const snd_bebob_spec,
}

#[repr(C)]
pub struct snd_bebob_spec {
    pub clock: *const snd_bebob_clock_spec,
    pub rate: *const snd_bebob_rate_spec,
    pub meter: *const snd_bebob_meter_spec,
}

#[repr(C)]
pub struct snd_bebob_rate_spec {
    pub get: Option<unsafe extern "C" fn(*mut snd_bebob, *mut u32) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut snd_bebob, u32) -> i32>,
}

#[repr(C)]
pub struct snd_bebob_clock_spec {
    pub num: usize,
    pub types: *const snd_bebob_clock_type,
    pub get: Option<unsafe extern "C" fn(*mut snd_bebob, *mut u32) -> i32>,
}

#[repr(C)]
pub struct snd_bebob_meter_spec {
    pub num: usize,
    pub labels: *const *const u8,
    pub get: Option<unsafe extern "C" fn(*mut snd_bebob, *mut u32, u32) -> i32>,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_bebob_clock_type {
    SND_BEBOB_CLOCK_TYPE_INTERNAL,
    SND_BEBOB_CLOCK_TYPE_EXTERNAL,
}

extern "C" {
    fn snd_fw_transaction(
        unit: *mut c_void,
        tcode: i32,
        offset: u64,
        buffer: *mut c_void,
        length: usize,
        flags: i32,
    ) -> i32;
    fn snd_bebob_stream_get_rate(bebob: *mut snd_bebob, rate: *mut u32) -> i32;
    fn snd_bebob_stream_set_rate(bebob: *mut snd_bebob, rate: u32) -> i32;
}

#[inline]
fn be32_to_cpu(value: __be32) -> u32 {
    u32::from_be(value)
}

#[inline]
fn cpu_to_be32(value: u32) -> __be32 {
    value.to_be()
}

#[inline]
unsafe fn saffire_read_block(
    bebob: *mut snd_bebob,
    offset: u64,
    buf: *mut u32,
    size: u32,
) -> i32 {
    let mut i: u32;
    let mut err: i32;
    let tmp: *mut __be32 = buf as *mut __be32;

    err = snd_fw_transaction(
        (*bebob).unit,
        TCODE_READ_BLOCK_REQUEST,
        SAFFIRE_ADDRESS_BASE + offset,
        tmp as *mut c_void,
        size as usize,
        0,
    );
    if err < 0 {
        return err;
    }

    i = 0;
    while i < size / core::mem::size_of::<u32>() as u32 {
        *buf.add(i as usize) = be32_to_cpu(*tmp.add(i as usize));
        i += 1;
    }

    err
}

#[inline]
unsafe fn saffire_read_quad(bebob: *mut snd_bebob, offset: u64, value: *mut u32) -> i32 {
    let mut err: i32;
    let mut tmp: __be32 = 0;

    err = snd_fw_transaction(
        (*bebob).unit,
        TCODE_READ_QUADLET_REQUEST,
        SAFFIRE_ADDRESS_BASE + offset,
        &mut tmp as *mut __be32 as *mut c_void,
        core::mem::size_of::<__be32>(),
        0,
    );
    if err < 0 {
        return err;
    }

    *value = be32_to_cpu(tmp);
    err
}

#[inline]
unsafe fn saffire_write_quad(bebob: *mut snd_bebob, offset: u64, value: u32) -> i32 {
    let mut data: __be32 = cpu_to_be32(value);

    snd_fw_transaction(
        (*bebob).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        SAFFIRE_ADDRESS_BASE + offset,
        &mut data as *mut __be32 as *mut c_void,
        core::mem::size_of::<__be32>(),
        0,
    )
}

static saffirepro_10_clk_src_types: [snd_bebob_clock_type; 3] = [
    snd_bebob_clock_type::SND_BEBOB_CLOCK_TYPE_INTERNAL,
    snd_bebob_clock_type::SND_BEBOB_CLOCK_TYPE_EXTERNAL, /* S/PDIF */
    snd_bebob_clock_type::SND_BEBOB_CLOCK_TYPE_EXTERNAL, /* Word Clock */
];
static saffirepro_26_clk_src_types: [snd_bebob_clock_type; 5] = [
    snd_bebob_clock_type::SND_BEBOB_CLOCK_TYPE_INTERNAL,
    snd_bebob_clock_type::SND_BEBOB_CLOCK_TYPE_EXTERNAL, /* S/PDIF */
    snd_bebob_clock_type::SND_BEBOB_CLOCK_TYPE_EXTERNAL, /* ADAT1 */
    snd_bebob_clock_type::SND_BEBOB_CLOCK_TYPE_EXTERNAL, /* ADAT2 */
    snd_bebob_clock_type::SND_BEBOB_CLOCK_TYPE_EXTERNAL, /* Word Clock */
];
/* Value maps between registers and labels for SaffirePro 10/26. */
static saffirepro_clk_maps: [[i8; SAFFIREPRO_CLOCK_SOURCE_COUNT]; 2] = [
    /* SaffirePro 10 */
    {
        let mut map = [-1i8; SAFFIREPRO_CLOCK_SOURCE_COUNT];
        map[SAFFIREPRO_CLOCK_SOURCE_INTERNAL] = 0;
        map[SAFFIREPRO_CLOCK_SOURCE_SKIP] = -1; /* not supported */
        map[SAFFIREPRO_CLOCK_SOURCE_SPDIF] = 1;
        map[SAFFIREPRO_CLOCK_SOURCE_ADAT1] = -1; /* not supported */
        map[SAFFIREPRO_CLOCK_SOURCE_ADAT2] = -1; /* not supported */
        map[SAFFIREPRO_CLOCK_SOURCE_WORDCLOCK] = 2;
        map
    },
    /* SaffirePro 26 */
    {
        let mut map = [-1i8; SAFFIREPRO_CLOCK_SOURCE_COUNT];
        map[SAFFIREPRO_CLOCK_SOURCE_INTERNAL] = 0;
        map[SAFFIREPRO_CLOCK_SOURCE_SKIP] = -1; /* not supported */
        map[SAFFIREPRO_CLOCK_SOURCE_SPDIF] = 1;
        map[SAFFIREPRO_CLOCK_SOURCE_ADAT1] = 2;
        map[SAFFIREPRO_CLOCK_SOURCE_ADAT2] = 3;
        map[SAFFIREPRO_CLOCK_SOURCE_WORDCLOCK] = 4;
        map
    },
];

unsafe extern "C" fn saffirepro_both_clk_freq_get(
    bebob: *mut snd_bebob,
    rate: *mut u32,
) -> i32 {
    let mut id: u32 = 0;
    let mut err: i32;

    err = saffire_read_quad(bebob, SAFFIREPRO_RATE_NOREBOOT, &mut id);
    if err < 0 {
        return err;
    }
    if id as usize >= rates.len() {
        err = -EIO;
    } else {
        *rate = rates[id as usize];
    }
    err
}

unsafe extern "C" fn saffirepro_both_clk_freq_set(bebob: *mut snd_bebob, rate: u32) -> i32 {
    let mut id: u32;

    id = 0;
    while (id as usize) < rates.len() {
        if rates[id as usize] == rate {
            break;
        }
        id += 1;
    }
    if id as usize == rates.len() {
        return -EINVAL;
    }

    saffire_write_quad(bebob, SAFFIREPRO_RATE_NOREBOOT, id)
}

/*
 * query hardware for current clock source, return our internally
 * used clock index in *id, depending on hardware.
 */
unsafe extern "C" fn saffirepro_both_clk_src_get(
    bebob: *mut snd_bebob,
    id: *mut u32,
) -> i32 {
    let mut err: i32;
    let mut value: u32 = 0; /* clock source read from hw register */
    let map: *const i8;

    err = saffire_read_quad(bebob, SAFFIREPRO_OFFSET_CLOCK_SOURCE, &mut value);
    if err < 0 {
        return err;
    }

    /* depending on hardware, use a different mapping */
    if (*(*(*bebob).spec).clock).types == saffirepro_10_clk_src_types.as_ptr() {
        map = saffirepro_clk_maps[0].as_ptr();
    } else {
        map = saffirepro_clk_maps[1].as_ptr();
    }

    /* In a case that this driver cannot handle the value of register. */
    value &= SAFFIREPRO_CLOCK_SOURCE_SELECT_MASK;
    if value as usize >= SAFFIREPRO_CLOCK_SOURCE_COUNT || *map.add(value as usize) < 0 {
        err = -EIO;
        return err;
    }

    *id = *map.add(value as usize) as u32;
    err
}

pub static saffire_le_spec: snd_bebob_spec = snd_bebob_spec {
    clock: &saffire_both_clk_spec,
    rate: &saffire_both_rate_spec,
    meter: &saffire_le_meter_spec,
};
static saffire_both_clk_src_types: [snd_bebob_clock_type; 2] = [
    snd_bebob_clock_type::SND_BEBOB_CLOCK_TYPE_INTERNAL,
    snd_bebob_clock_type::SND_BEBOB_CLOCK_TYPE_EXTERNAL,
];

unsafe extern "C" fn saffire_both_clk_src_get(bebob: *mut snd_bebob, id: *mut u32) -> i32 {
    let mut err: i32;
    let mut value: u32 = 0;

    err = saffire_read_quad(bebob, SAFFIRE_OFFSET_CLOCK_SOURCE, &mut value);
    if err >= 0 {
        *id = 0xff & value;
    }

    err
}

static saffire_le_meter_labels: [*const u8; 9] = [
    ANA_IN.as_ptr(),
    ANA_IN.as_ptr(),
    DIG_IN.as_ptr(),
    ANA_OUT.as_ptr(),
    ANA_OUT.as_ptr(),
    ANA_OUT.as_ptr(),
    ANA_OUT.as_ptr(),
    STM_IN.as_ptr(),
    STM_IN.as_ptr(),
];
static saffire_meter_labels: [*const u8; 7] = [
    ANA_IN.as_ptr(),
    ANA_IN.as_ptr(),
    STM_IN.as_ptr(),
    STM_IN.as_ptr(),
    STM_IN.as_ptr(),
    STM_IN.as_ptr(),
    STM_IN.as_ptr(),
];

unsafe extern "C" fn saffire_meter_get(
    bebob: *mut snd_bebob,
    buf: *mut u32,
    size: u32,
) -> i32 {
    let spec: *const snd_bebob_meter_spec = (*(*bebob).spec).meter;
    let channels: u32;
    let offset: u64;
    let mut err: i32;

    if (*spec).labels == saffire_le_meter_labels.as_ptr() {
        offset = SAFFIRE_LE_OFFSET_METER;
    } else {
        offset = SAFFIRE_OFFSET_METER;
    }

    channels = ((*spec).num as u32) * 2;
    if size < channels * core::mem::size_of::<u32>() as u32 {
        return -EIO;
    }

    err = saffire_read_block(bebob, offset, buf, size);
    if err >= 0 && (*spec).labels == saffire_le_meter_labels.as_ptr() {
        core::ptr::swap(buf.add(1), buf.add(3));
        core::ptr::swap(buf.add(2), buf.add(3));
        core::ptr::swap(buf.add(3), buf.add(4));

        core::ptr::swap(buf.add(7), buf.add(10));
        core::ptr::swap(buf.add(8), buf.add(10));
        core::ptr::swap(buf.add(9), buf.add(11));
        core::ptr::swap(buf.add(11), buf.add(12));

        core::ptr::swap(buf.add(15), buf.add(16));
    }

    err
}

static saffirepro_both_rate_spec: snd_bebob_rate_spec = snd_bebob_rate_spec {
    get: Some(saffirepro_both_clk_freq_get),
    set: Some(saffirepro_both_clk_freq_set),
};
/* Saffire Pro 26 I/O  */
static saffirepro_26_clk_spec: snd_bebob_clock_spec = snd_bebob_clock_spec {
    num: saffirepro_26_clk_src_types.len(),
    types: saffirepro_26_clk_src_types.as_ptr(),
    get: Some(saffirepro_both_clk_src_get),
};
pub static saffirepro_26_spec: snd_bebob_spec = snd_bebob_spec {
    clock: &saffirepro_26_clk_spec,
    rate: &saffirepro_both_rate_spec,
    meter: core::ptr::null(),
};
/* Saffire Pro 10 I/O */
static saffirepro_10_clk_spec: snd_bebob_clock_spec = snd_bebob_clock_spec {
    num: saffirepro_10_clk_src_types.len(),
    types: saffirepro_10_clk_src_types.as_ptr(),
    get: Some(saffirepro_both_clk_src_get),
};
pub static saffirepro_10_spec: snd_bebob_spec = snd_bebob_spec {
    clock: &saffirepro_10_clk_spec,
    rate: &saffirepro_both_rate_spec,
    meter: core::ptr::null(),
};

static saffire_both_rate_spec: snd_bebob_rate_spec = snd_bebob_rate_spec {
    get: Some(snd_bebob_stream_get_rate),
    set: Some(snd_bebob_stream_set_rate),
};
static saffire_both_clk_spec: snd_bebob_clock_spec = snd_bebob_clock_spec {
    num: saffire_both_clk_src_types.len(),
    types: saffire_both_clk_src_types.as_ptr(),
    get: Some(saffire_both_clk_src_get),
};
/* Saffire LE */
static saffire_le_meter_spec: snd_bebob_meter_spec = snd_bebob_meter_spec {
    num: saffire_le_meter_labels.len(),
    labels: saffire_le_meter_labels.as_ptr(),
    get: Some(saffire_meter_get),
};
/* Saffire */
static saffire_meter_spec: snd_bebob_meter_spec = snd_bebob_meter_spec {
    num: saffire_meter_labels.len(),
    labels: saffire_meter_labels.as_ptr(),
    get: Some(saffire_meter_get),
};
pub static saffire_spec: snd_bebob_spec = snd_bebob_spec {
    clock: &saffire_both_clk_spec,
    rate: &saffire_both_rate_spec,
    meter: &saffire_meter_spec,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
