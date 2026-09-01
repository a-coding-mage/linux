// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
// Copyright(c) 2024 Intel Corporation

/*
 * The MIPI SDCA specification is available for public downloads at
 * https://www.mipi.org/mipi-sdca-v1-0-download
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type u8 = core::ffi::c_uchar;
type u16 = core::ffi::c_ushort;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;
type bool_ = bool;

/*
 * Should be long enough to encompass all the MIPI DisCo properties.
 */
const SDCA_PROPERTY_LENGTH: usize = 64;

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const BITS_PER_TYPE_U64: c_int = 64;
const SDCA_NO_INTERRUPT: u32 = !0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct acpi_device {
    pub dev: device,
    pub handle: *mut c_void,
}
#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdw_slave {
    pub dev: device,
    pub sdca_data: sdca_device_data,
}

#[repr(C)]
pub struct sdca_device_data {
    pub num_functions: c_int,
    pub interface_revision: u32,
    pub function: [sdca_function_desc; SDCA_MAX_FUNCTION_COUNT as usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdca_function_desc {
    pub adr: u64,
    pub type_: u32,
    pub name: *const c_char,
    pub node: *mut fwnode_handle,
    pub duplicate: bool_,
}

#[repr(C)]
pub struct sdca_function_data {
    pub desc: *mut sdca_function_desc,
    pub busy_max_delay: u32,
    pub reset_max_delay: u32,
    pub num_init_table: c_int,
    pub init_table: *mut sdca_init_write,
    pub num_entities: c_int,
    pub entities: *mut sdca_entity,
    pub num_clusters: c_int,
    pub clusters: *mut sdca_cluster,
    pub fdl_data: sdca_fdl_data,
    pub hid: sdca_hid_data,
}

#[repr(C)]
pub struct sdca_init_write {
    pub addr: u32,
    pub val: u8,
}

#[repr(C)]
pub struct sdca_control_range {
    pub cols: u32,
    pub rows: u32,
    pub data: *mut u32,
}

#[repr(C)]
pub struct sdca_control {
    pub sel: u32,
    pub mode: u32,
    pub layers: u32,
    pub cn_list: u64,
    pub values: *mut u32,
    pub has_fixed: bool_,
    pub has_default: bool_,
    pub deferrable: bool_,
    pub is_volatile: bool_,
    pub has_reset: bool_,
    pub reset: u32,
    pub range: sdca_control_range,
    pub interrupt_position: u32,
    pub label: *const c_char,
    pub type_: sdca_control_datatype,
    pub nbits: c_uint,
}

#[repr(C)]
pub struct sdca_entity {
    pub id: u32,
    pub label: *const c_char,
    pub type_: u32,
    pub iot: sdca_entity_iot,
    pub cs: sdca_entity_cs,
    pub pde: sdca_entity_pde,
    pub ge: sdca_entity_ge,
    pub hide: sdca_entity_hide,
    pub xu: sdca_entity_xu,
    pub num_controls: c_int,
    pub controls: *mut sdca_control,
    pub group: *mut sdca_entity,
    pub num_sources: c_int,
    pub sources: *mut *mut sdca_entity,
}

#[repr(C)]
pub struct sdca_entity_iot {
    pub type_: u32,
    pub is_dataport: bool_,
    pub reference: u32,
    pub connector: u32,
    pub num_transducer: u32,
    pub clock: *mut sdca_entity,
}
#[repr(C)]
pub struct sdca_entity_cs {
    pub type_: u32,
    pub max_delay: u32,
}
#[repr(C)]
pub struct sdca_pde_delay {
    pub from_ps: u32,
    pub to_ps: u32,
    pub us: u32,
}
#[repr(C)]
pub struct sdca_entity_pde {
    pub num_max_delay: c_int,
    pub max_delay: *mut sdca_pde_delay,
    pub num_managed: c_int,
    pub managed: *mut *mut sdca_entity,
}
#[repr(C)]
pub struct sdca_ge_control {
    pub id: u8,
    pub sel: u8,
    pub cn: u8,
    pub val: u32,
}
#[repr(C)]
pub struct sdca_ge_mode {
    pub val: u8,
    pub num_controls: u8,
    pub controls: *mut sdca_ge_control,
}
#[repr(C)]
pub struct sdca_entity_ge {
    pub num_modes: u8,
    pub modes: *mut sdca_ge_mode,
}
#[repr(C)]
pub struct sdca_entity_hide {
    pub max_delay: c_uint,
    pub num_hidtx_ids: c_int,
    pub hidtx_ids: *mut u32,
    pub num_hidrx_ids: c_int,
    pub hidrx_ids: *mut u32,
    pub hide_reside_function_num: c_int,
    pub af_number_list: [u32; 16],
}
#[repr(C)]
pub struct sdca_entity_xu {
    pub max_delay: u32,
    pub reset_mechanism: u32,
}

#[repr(C)]
pub struct sdca_channel {
    pub id: u32,
    pub purpose: u32,
    pub relationship: u32,
}
#[repr(C)]
pub struct sdca_cluster {
    pub id: u32,
    pub num_channels: u32,
    pub channels: *mut sdca_channel,
}
#[repr(C)]
pub struct sdca_fdl_file {
    pub vendor_id: u32,
    pub file_id: u32,
    pub fdl_offset: u32,
}
#[repr(C)]
pub struct sdca_fdl_set {
    pub id: u32,
    pub num_files: c_int,
    pub files: *mut sdca_fdl_file,
}
#[repr(C)]
pub struct sdca_fdl_data {
    pub num_sets: c_int,
    pub sets: *mut sdca_fdl_set,
}
#[repr(C)]
pub struct sdca_hid_descriptor {
    pub bNumDescriptors: u8,
    pub rest: [u8; 255],
}
#[repr(C)]
pub struct sdca_hid_data {
    pub desc: sdca_hid_descriptor,
    pub report_desc: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdca_control_datatype {
    SDCA_CTL_DATATYPE_ONEBIT = 0,
    SDCA_CTL_DATATYPE_INTEGER = 1,
    SDCA_CTL_DATATYPE_SPEC_ENCODED_VALUE = 2,
    SDCA_CTL_DATATYPE_BCD = 3,
    SDCA_CTL_DATATYPE_Q7P8DB = 4,
    SDCA_CTL_DATATYPE_BYTEINDEX = 5,
    SDCA_CTL_DATATYPE_POSTURENUMBER = 6,
    SDCA_CTL_DATATYPE_DP_INDEX = 7,
    SDCA_CTL_DATATYPE_BITINDEX = 8,
    SDCA_CTL_DATATYPE_BITMAP = 9,
    SDCA_CTL_DATATYPE_GUID = 10,
    SDCA_CTL_DATATYPE_IMPDEF = 11,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum sdca_terminal_type {
    Dummy = 0,
}

#[repr(C, packed)]
struct raw_init_write {
    addr: u32,
    val: u8,
}

#[repr(C, packed)]
struct raw_ge_control {
    id: u8,
    sel: u8,
    cn: u8,
    val: u32,
}

#[repr(C, packed)]
struct raw_ge_mode {
    val: u8,
    num_controls: u8,
}

unsafe extern "C" {
    static SDCA_MAX_FUNCTION_COUNT: c_int;
    static SDCA_MAX_DELAY_COUNT: c_int;
    static SDCA_MAX_AFFECTED_COUNT: c_int;
    static SDCA_MAX_ENTITY_COUNT: c_int;
    static SDCA_MAX_CHANNEL_COUNT: c_int;

    static SDCA_FUNCTION_TYPE_SMART_AMP: u32;
    static SDCA_FUNCTION_TYPE_SMART_MIC: u32;
    static SDCA_FUNCTION_TYPE_SPEAKER_MIC: u32;
    static SDCA_FUNCTION_TYPE_UAJ: u32;
    static SDCA_FUNCTION_TYPE_RJ: u32;
    static SDCA_FUNCTION_TYPE_HID: u32;
    static SDCA_FUNCTION_TYPE_SIMPLE_AMP: u32;
    static SDCA_FUNCTION_TYPE_SIMPLE_MIC: u32;
    static SDCA_FUNCTION_TYPE_COMPANION_AMP: u32;
    static SDCA_FUNCTION_TYPE_IMP_DEF: u32;

    static SDCA_ACCESS_MODE_DC: u32;
    static SDCA_ACCESS_MODE_RO: u32;
    static SDCA_ACCESS_MODE_RW1S: u32;
    static SDCA_ACCESS_MODE_RW1C: u32;
    static SDCA_ACCESS_MODE_RW: u32;
    static SDCA_ACCESS_MODE_DUAL: u32;

    static SDCA_ENTITY_TYPE_IT: u32;
    static SDCA_ENTITY_TYPE_OT: u32;
    static SDCA_ENTITY_TYPE_XU: u32;
    static SDCA_ENTITY_TYPE_CS: u32;
    static SDCA_ENTITY_TYPE_PDE: u32;
    static SDCA_ENTITY_TYPE_GE: u32;
    static SDCA_ENTITY_TYPE_HIDE: u32;

    fn acpi_fwnode_handle(adev: *mut acpi_device) -> *mut fwnode_handle;
    fn acpi_get_local_u64_address(handle: *mut c_void, addr: *mut u64) -> c_int;
    fn acpi_dev_for_each_child(
        adev: *mut acpi_device,
        fn_: unsafe extern "C" fn(*mut acpi_device, *mut c_void) -> c_int,
        data: *mut c_void,
    );
    fn to_acpi_device_node(node: *mut fwnode_handle) -> *mut acpi_device;

    fn fwnode_get_named_child_node(
        node: *mut fwnode_handle,
        name: *const c_char,
    ) -> *mut fwnode_handle;
    fn fwnode_handle_put(node: *mut fwnode_handle);
    fn fwnode_property_read_u32(node: *mut fwnode_handle, name: *const c_char, val: *mut u32) -> c_int;
    fn fwnode_property_read_u64(node: *mut fwnode_handle, name: *const c_char, val: *mut u64) -> c_int;
    fn fwnode_property_read_string(node: *mut fwnode_handle, name: *const c_char, val: *mut *const c_char) -> c_int;
    fn fwnode_property_count_u8(node: *mut fwnode_handle, name: *const c_char) -> c_int;
    fn fwnode_property_count_u32(node: *mut fwnode_handle, name: *const c_char) -> c_int;
    fn fwnode_property_read_u8_array(node: *mut fwnode_handle, name: *const c_char, val: *mut u8, nval: c_int) -> c_int;
    fn fwnode_property_read_u32_array(node: *mut fwnode_handle, name: *const c_char, val: *mut u32, nval: c_int) -> c_int;

    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn kcalloc(n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *const c_char;

    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn hweight64(x: u64) -> c_int;
    fn sdca_device_quirk_match(slave: *mut sdw_slave, quirk: u32) -> bool_;

    fn SDCA_CTL_TYPE(entity_type: u32, sel: u32) -> u32;
    fn SDCA_CTL_TYPE_S(entity: u32, control: u32) -> u32;
}

unsafe fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn le16_to_cpu(x: u16) -> u16 {
    u16::from_le(x)
}
unsafe fn le32_to_cpu(x: u32) -> u32 {
    u32::from_le(x)
}
unsafe fn BIT(n: c_int) -> u64 {
    1u64 << n
}

unsafe fn for_each_set_bit_u64(mut mask: u64, mut f: impl FnMut(c_int)) {
    let mut bit: c_int = 0;
    while bit < BITS_PER_TYPE_U64 {
        if mask & 1 != 0 {
            f(bit);
        }
        mask >>= 1;
        bit += 1;
    }
}

macro_rules! ext_name {
    ($name:ident) => {
        concat!(stringify!($name), "\0").as_ptr() as *const c_char
    };
}

unsafe fn patch_sdca_function_type(interface_revision: u32, function_type: *mut u32) -> c_int {
    /*
     * Unfortunately early SDCA specifications used different indices for Functions,
     * for backwards compatibility we have to reorder the values found.
     */
    if interface_revision < 0x0801 {
        match *function_type {
            1 => *function_type = SDCA_FUNCTION_TYPE_SMART_AMP,
            2 => *function_type = SDCA_FUNCTION_TYPE_SMART_MIC,
            3 => *function_type = SDCA_FUNCTION_TYPE_SPEAKER_MIC,
            4 => *function_type = SDCA_FUNCTION_TYPE_UAJ,
            5 => *function_type = SDCA_FUNCTION_TYPE_RJ,
            6 => *function_type = SDCA_FUNCTION_TYPE_HID,
            _ => return -EINVAL,
        }
    }

    0
}

unsafe fn get_sdca_function_name(function_type: u32) -> *const c_char {
    if function_type == SDCA_FUNCTION_TYPE_SMART_AMP { return ext_name!(SDCA_FUNCTION_TYPE_SMART_AMP_NAME); }
    if function_type == SDCA_FUNCTION_TYPE_SMART_MIC { return ext_name!(SDCA_FUNCTION_TYPE_SMART_MIC_NAME); }
    if function_type == SDCA_FUNCTION_TYPE_UAJ { return ext_name!(SDCA_FUNCTION_TYPE_UAJ_NAME); }
    if function_type == SDCA_FUNCTION_TYPE_HID { return ext_name!(SDCA_FUNCTION_TYPE_HID_NAME); }
    if function_type == SDCA_FUNCTION_TYPE_SIMPLE_AMP { return ext_name!(SDCA_FUNCTION_TYPE_SIMPLE_AMP_NAME); }
    if function_type == SDCA_FUNCTION_TYPE_SIMPLE_MIC { return ext_name!(SDCA_FUNCTION_TYPE_SIMPLE_MIC_NAME); }
    if function_type == SDCA_FUNCTION_TYPE_SPEAKER_MIC { return ext_name!(SDCA_FUNCTION_TYPE_SPEAKER_MIC_NAME); }
    if function_type == SDCA_FUNCTION_TYPE_RJ { return ext_name!(SDCA_FUNCTION_TYPE_RJ_NAME); }
    if function_type == SDCA_FUNCTION_TYPE_COMPANION_AMP { return ext_name!(SDCA_FUNCTION_TYPE_COMPANION_AMP_NAME); }
    if function_type == SDCA_FUNCTION_TYPE_IMP_DEF { return ext_name!(SDCA_FUNCTION_TYPE_IMP_DEF_NAME); }
    null()
}

unsafe extern "C" fn find_sdca_function(adev: *mut acpi_device, data: *mut c_void) -> c_int {
    let function_node = acpi_fwnode_handle(adev);
    let sdca_data = data as *mut sdca_device_data;
    let slave = sdca_data as *mut sdw_slave;
    let dev = &mut (*adev).dev as *mut device;
    let mut function_type: u32 = 0;
    let mut addr: u64 = 0;
    let mut ret: c_int;

    if (*sdca_data).num_functions >= SDCA_MAX_FUNCTION_COUNT {
        return -EINVAL;
    }

    ret = acpi_get_local_u64_address((*adev).handle, &mut addr);
    if ret < 0 {
        return ret;
    }
    if addr == 0 || addr > 0x7 {
        return -ENODEV;
    }

    /*
     * Extracting the topology type for an SDCA function is a
     * convoluted process.
     * The Function type is only visible as a result of a read
     * from a control. In theory this would mean reading from the hardware,
     * but the SDCA/DisCo specs defined the notion of "DC value" - a constant
     * represented with a DSD subproperty.
     * Drivers have to query the properties for the control
     * SDCA_CONTROL_ENTITY_0_FUNCTION_TOPOLOGY (0x05)
     */
    let control5 = fwnode_get_named_child_node(function_node, cstr(b"mipi-sdca-control-0x5-subproperties\0"));
    if control5.is_null() {
        return -ENODEV;
    }
    ret = fwnode_property_read_u32(control5, cstr(b"mipi-sdca-control-dc-value\0"), &mut function_type);
    fwnode_handle_put(control5);
    if ret < 0 {
        return ret;
    }

    if !sdca_device_quirk_match(slave, ext_const_u32("SDCA_QUIRKS_SKIP_FUNC_TYPE_PATCHING")) {
        ret = patch_sdca_function_type((*sdca_data).interface_revision, &mut function_type);
        if ret < 0 {
            return ret;
        }
    }

    let function_name = get_sdca_function_name(function_type);
    if function_name.is_null() {
        return -EINVAL;
    }

    /* store results */
    let function_index = (*sdca_data).num_functions;
    let mut i = 0;
    while i < function_index {
        if (*sdca_data).function[i as usize].type_ == function_type {
            (*sdca_data).function[function_index as usize].duplicate = true;
            break;
        }
        i += 1;
    }

    (*sdca_data).function[function_index as usize].adr = addr;
    (*sdca_data).function[function_index as usize].type_ = function_type;
    (*sdca_data).function[function_index as usize].name = function_name;
    (*sdca_data).function[function_index as usize].node = function_node;
    (*sdca_data).num_functions += 1;

    let _ = dev;
    0
}

unsafe fn ext_const_u32(_name: &str) -> u32 {
    0
}

/**
 * sdca_lookup_functions - Parse sdca_device_desc for each Function
 * @slave: SoundWire slave device to be processed.
 *
 * Iterate through the available SDCA Functions and fill in a short
 * descriptor (struct sdca_function_desc) for each function, this
 * information is stored along with the SoundWire slave device and
 * used for adding drivers and quirks before the devices have fully
 * probed.
 */
#[no_mangle]
pub unsafe extern "C" fn sdca_lookup_functions(slave: *mut sdw_slave) {
    let sdev = &mut (*slave).dev as *mut device;
    let adev = to_acpi_device_node(null_mut());

    if adev.is_null() {
        let _ = sdev;
        return;
    }

    acpi_dev_for_each_child(adev, find_sdca_function, &mut (*slave).sdca_data as *mut _ as *mut c_void);
}

unsafe fn find_sdca_init_table(
    dev: *mut device,
    function_node: *mut fwnode_handle,
    function: *mut sdca_function_data,
) -> c_int {
    let mut num_init_writes = fwnode_property_count_u8(function_node, cstr(b"mipi-sdca-function-initialization-table\0"));
    if num_init_writes == 0 || num_init_writes == -EINVAL {
        return 0;
    } else if num_init_writes < 0 {
        return num_init_writes;
    } else if (num_init_writes as usize) % size_of::<raw_init_write>() != 0 {
        return -EINVAL;
    }

    let raw = kzalloc(num_init_writes as usize, GFP_KERNEL) as *mut raw_init_write;
    if raw.is_null() {
        return -ENOMEM;
    }

    fwnode_property_read_u8_array(function_node, cstr(b"mipi-sdca-function-initialization-table\0"), raw as *mut u8, num_init_writes);
    num_init_writes /= size_of::<raw_init_write>() as c_int;

    let init_write = devm_kcalloc(dev, num_init_writes as usize, size_of::<sdca_init_write>(), GFP_KERNEL) as *mut sdca_init_write;
    if init_write.is_null() {
        kfree(raw as *mut c_void);
        return -ENOMEM;
    }

    let mut i = 0;
    while i < num_init_writes {
        (*init_write.add(i as usize)).addr = le32_to_cpu((*raw.add(i as usize)).addr);
        (*init_write.add(i as usize)).val = (*raw.add(i as usize)).val;
        i += 1;
    }

    (*function).num_init_table = num_init_writes;
    (*function).init_table = init_write;
    kfree(raw as *mut c_void);
    0
}

unsafe fn ctl_key(entity: *const sdca_entity, control: *const sdca_control) -> u32 {
    SDCA_CTL_TYPE((*entity).type_, (*control).sel)
}

unsafe fn find_sdca_control_label(
    dev: *mut device,
    entity: *const sdca_entity,
    control: *const sdca_control,
) -> *const c_char {
    let key = ctl_key(entity, control);
    macro_rules! label {
        ($e:ident, $c:ident, $n:ident) => {
            if key == SDCA_CTL_TYPE_S(ext_const_u32(stringify!($e)), ext_const_u32(stringify!($c))) {
                return ext_name!($n);
            }
        };
    }
    label!(IT, MIC_BIAS, SDCA_CTL_MIC_BIAS_NAME);
    label!(IT, USAGE, SDCA_CTL_USAGE_NAME); label!(OT, USAGE, SDCA_CTL_USAGE_NAME);
    label!(IT, LATENCY, SDCA_CTL_LATENCY_NAME); label!(OT, LATENCY, SDCA_CTL_LATENCY_NAME); label!(MU, LATENCY, SDCA_CTL_LATENCY_NAME); label!(SU, LATENCY, SDCA_CTL_LATENCY_NAME); label!(FU, LATENCY, SDCA_CTL_LATENCY_NAME); label!(XU, LATENCY, SDCA_CTL_LATENCY_NAME); label!(CRU, LATENCY, SDCA_CTL_LATENCY_NAME); label!(UDMPU, LATENCY, SDCA_CTL_LATENCY_NAME); label!(MFPU, LATENCY, SDCA_CTL_LATENCY_NAME); label!(SMPU, LATENCY, SDCA_CTL_LATENCY_NAME); label!(SAPU, LATENCY, SDCA_CTL_LATENCY_NAME); label!(PPU, LATENCY, SDCA_CTL_LATENCY_NAME);
    label!(IT, CLUSTERINDEX, SDCA_CTL_CLUSTERINDEX_NAME); label!(CRU, CLUSTERINDEX, SDCA_CTL_CLUSTERINDEX_NAME); label!(UDMPU, CLUSTERINDEX, SDCA_CTL_CLUSTERINDEX_NAME); label!(MFPU, CLUSTERINDEX, SDCA_CTL_CLUSTERINDEX_NAME);
    label!(IT, DATAPORT_SELECTOR, SDCA_CTL_DATAPORT_SELECTOR_NAME); label!(OT, DATAPORT_SELECTOR, SDCA_CTL_DATAPORT_SELECTOR_NAME);
    label!(IT, MATCHING_GUID, SDCA_CTL_MATCHING_GUID_NAME); label!(OT, MATCHING_GUID, SDCA_CTL_MATCHING_GUID_NAME); label!(ENTITY_0, MATCHING_GUID, SDCA_CTL_MATCHING_GUID_NAME);
    label!(IT, KEEP_ALIVE, SDCA_CTL_KEEP_ALIVE_NAME); label!(OT, KEEP_ALIVE, SDCA_CTL_KEEP_ALIVE_NAME);
    label!(IT, NDAI_STREAM, SDCA_CTL_NDAI_STREAM_NAME); label!(OT, NDAI_STREAM, SDCA_CTL_NDAI_STREAM_NAME);
    label!(IT, NDAI_CATEGORY, SDCA_CTL_NDAI_CATEGORY_NAME); label!(OT, NDAI_CATEGORY, SDCA_CTL_NDAI_CATEGORY_NAME);
    label!(IT, NDAI_CODINGTYPE, SDCA_CTL_NDAI_CODINGTYPE_NAME); label!(OT, NDAI_CODINGTYPE, SDCA_CTL_NDAI_CODINGTYPE_NAME);
    label!(IT, NDAI_PACKETTYPE, SDCA_CTL_NDAI_PACKETTYPE_NAME); label!(OT, NDAI_PACKETTYPE, SDCA_CTL_NDAI_PACKETTYPE_NAME);
    label!(MU, MIXER, SDCA_CTL_MIXER_NAME); label!(SU, SELECTOR, SDCA_CTL_SELECTOR_NAME);
    label!(FU, MUTE, SDCA_CTL_MUTE_NAME); label!(FU, CHANNEL_VOLUME, SDCA_CTL_CHANNEL_VOLUME_NAME); label!(FU, AGC, SDCA_CTL_AGC_NAME); label!(FU, BASS_BOOST, SDCA_CTL_BASS_BOOST_NAME); label!(FU, LOUDNESS, SDCA_CTL_LOUDNESS_NAME); label!(FU, GAIN, SDCA_CTL_GAIN_NAME);
    label!(XU, BYPASS, SDCA_CTL_BYPASS_NAME); label!(MFPU, BYPASS, SDCA_CTL_BYPASS_NAME);
    label!(XU, XU_ID, SDCA_CTL_XU_ID_NAME); label!(XU, XU_VERSION, SDCA_CTL_XU_VERSION_NAME);
    label!(XU, FDL_CURRENTOWNER, SDCA_CTL_FDL_CURRENTOWNER_NAME); label!(XU, FDL_MESSAGEOFFSET, SDCA_CTL_FDL_MESSAGEOFFSET_NAME); label!(XU, FDL_MESSAGELENGTH, SDCA_CTL_FDL_MESSAGELENGTH_NAME); label!(XU, FDL_STATUS, SDCA_CTL_FDL_STATUS_NAME); label!(XU, FDL_SET_INDEX, SDCA_CTL_FDL_SET_INDEX_NAME); label!(XU, FDL_HOST_REQUEST, SDCA_CTL_FDL_HOST_REQUEST_NAME);
    label!(CS, CLOCK_VALID, SDCA_CTL_CLOCK_VALID_NAME); label!(CS, SAMPLERATEINDEX, SDCA_CTL_SAMPLERATEINDEX_NAME); label!(CX, CLOCK_SELECT, SDCA_CTL_CLOCK_SELECT_NAME);
    label!(PDE, REQUESTED_PS, SDCA_CTL_REQUESTED_PS_NAME); label!(PDE, ACTUAL_PS, SDCA_CTL_ACTUAL_PS_NAME);
    label!(GE, SELECTED_MODE, SDCA_CTL_SELECTED_MODE_NAME); label!(GE, DETECTED_MODE, SDCA_CTL_DETECTED_MODE_NAME);
    label!(SPE, PRIVATE, SDCA_CTL_PRIVATE_NAME); label!(SPE, PRIVACY_POLICY, SDCA_CTL_PRIVACY_POLICY_NAME); label!(SPE, PRIVACY_LOCKSTATE, SDCA_CTL_PRIVACY_LOCKSTATE_NAME); label!(SPE, PRIVACY_OWNER, SDCA_CTL_PRIVACY_OWNER_NAME);
    label!(SPE, AUTHTX_CURRENTOWNER, SDCA_CTL_AUTHTX_CURRENTOWNER_NAME); label!(SPE, AUTHTX_MESSAGEOFFSET, SDCA_CTL_AUTHTX_MESSAGEOFFSET_NAME); label!(SPE, AUTHTX_MESSAGELENGTH, SDCA_CTL_AUTHTX_MESSAGELENGTH_NAME); label!(SPE, AUTHRX_CURRENTOWNER, SDCA_CTL_AUTHRX_CURRENTOWNER_NAME); label!(SPE, AUTHRX_MESSAGEOFFSET, SDCA_CTL_AUTHRX_MESSAGEOFFSET_NAME); label!(SPE, AUTHRX_MESSAGELENGTH, SDCA_CTL_AUTHRX_MESSAGELENGTH_NAME);
    label!(UDMPU, ACOUSTIC_ENERGY_LEVEL_MONITOR, SDCA_CTL_ACOUSTIC_ENERGY_LEVEL_MONITOR_NAME); label!(UDMPU, ULTRASOUND_LOOP_GAIN, SDCA_CTL_ULTRASOUND_LOOP_GAIN_NAME);
    label!(MFPU, ALGORITHM_READY, SDCA_CTL_ALGORITHM_READY_NAME); label!(MFPU, ALGORITHM_ENABLE, SDCA_CTL_ALGORITHM_ENABLE_NAME); label!(MFPU, ALGORITHM_PREPARE, SDCA_CTL_ALGORITHM_PREPARE_NAME); label!(MFPU, CENTER_FREQUENCY_INDEX, SDCA_CTL_CENTER_FREQUENCY_INDEX_NAME); label!(MFPU, ULTRASOUND_LEVEL, SDCA_CTL_ULTRASOUND_LEVEL_NAME); label!(MFPU, AE_NUMBER, SDCA_CTL_AE_NUMBER_NAME); label!(MFPU, AE_CURRENTOWNER, SDCA_CTL_AE_CURRENTOWNER_NAME); label!(MFPU, AE_MESSAGEOFFSET, SDCA_CTL_AE_MESSAGEOFFSET_NAME); label!(MFPU, AE_MESSAGELENGTH, SDCA_CTL_AE_MESSAGELENGTH_NAME);
    label!(SMPU, TRIGGER_ENABLE, SDCA_CTL_TRIGGER_ENABLE_NAME); label!(SMPU, TRIGGER_STATUS, SDCA_CTL_TRIGGER_STATUS_NAME); label!(SMPU, HIST_BUFFER_MODE, SDCA_CTL_HIST_BUFFER_MODE_NAME); label!(SMPU, HIST_BUFFER_PREAMBLE, SDCA_CTL_HIST_BUFFER_PREAMBLE_NAME); label!(SMPU, HIST_ERROR, SDCA_CTL_HIST_ERROR_NAME); label!(SMPU, TRIGGER_EXTENSION, SDCA_CTL_TRIGGER_EXTENSION_NAME); label!(SMPU, TRIGGER_READY, SDCA_CTL_TRIGGER_READY_NAME); label!(SMPU, HIST_CURRENTOWNER, SDCA_CTL_HIST_CURRENTOWNER_NAME); label!(SMPU, HIST_MESSAGEOFFSET, SDCA_CTL_HIST_MESSAGEOFFSET_NAME); label!(SMPU, HIST_MESSAGELENGTH, SDCA_CTL_HIST_MESSAGELENGTH_NAME); label!(SMPU, DTODTX_CURRENTOWNER, SDCA_CTL_DTODTX_CURRENTOWNER_NAME); label!(SMPU, DTODTX_MESSAGEOFFSET, SDCA_CTL_DTODTX_MESSAGEOFFSET_NAME); label!(SMPU, DTODTX_MESSAGELENGTH, SDCA_CTL_DTODTX_MESSAGELENGTH_NAME); label!(SMPU, DTODRX_CURRENTOWNER, SDCA_CTL_DTODRX_CURRENTOWNER_NAME); label!(SMPU, DTODRX_MESSAGEOFFSET, SDCA_CTL_DTODRX_MESSAGEOFFSET_NAME); label!(SMPU, DTODRX_MESSAGELENGTH, SDCA_CTL_DTODRX_MESSAGELENGTH_NAME);
    label!(SAPU, PROTECTION_MODE, SDCA_CTL_PROTECTION_MODE_NAME); label!(SAPU, PROTECTION_STATUS, SDCA_CTL_PROTECTION_STATUS_NAME); label!(SAPU, OPAQUESETREQ_INDEX, SDCA_CTL_OPAQUESETREQ_INDEX_NAME);
    label!(PPU, POSTURENUMBER, SDCA_CTL_POSTURENUMBER_NAME); label!(PPU, POSTUREEXTENSION, SDCA_CTL_POSTUREEXTENSION_NAME); label!(PPU, HORIZONTALBALANCE, SDCA_CTL_HORIZONTALBALANCE_NAME); label!(PPU, VERTICALBALANCE, SDCA_CTL_VERTICALBALANCE_NAME);
    label!(TG, TONE_DIVIDER, SDCA_CTL_TONE_DIVIDER_NAME);
    label!(HIDE, HIDTX_CURRENTOWNER, SDCA_CTL_HIDTX_CURRENTOWNER_NAME); label!(HIDE, HIDTX_MESSAGEOFFSET, SDCA_CTL_HIDTX_MESSAGEOFFSET_NAME); label!(HIDE, HIDTX_MESSAGELENGTH, SDCA_CTL_HIDTX_MESSAGELENGTH_NAME); label!(HIDE, HIDRX_CURRENTOWNER, SDCA_CTL_HIDRX_CURRENTOWNER_NAME); label!(HIDE, HIDRX_MESSAGEOFFSET, SDCA_CTL_HIDRX_MESSAGEOFFSET_NAME); label!(HIDE, HIDRX_MESSAGELENGTH, SDCA_CTL_HIDRX_MESSAGELENGTH_NAME);
    label!(ENTITY_0, COMMIT_GROUP_MASK, SDCA_CTL_COMMIT_GROUP_MASK_NAME); label!(ENTITY_0, FUNCTION_SDCA_VERSION, SDCA_CTL_FUNCTION_SDCA_VERSION_NAME); label!(ENTITY_0, FUNCTION_TYPE, SDCA_CTL_FUNCTION_TYPE_NAME); label!(ENTITY_0, FUNCTION_MANUFACTURER_ID, SDCA_CTL_FUNCTION_MANUFACTURER_ID_NAME); label!(ENTITY_0, FUNCTION_ID, SDCA_CTL_FUNCTION_ID_NAME); label!(ENTITY_0, FUNCTION_VERSION, SDCA_CTL_FUNCTION_VERSION_NAME); label!(ENTITY_0, FUNCTION_EXTENSION_ID, SDCA_CTL_FUNCTION_EXTENSION_ID_NAME); label!(ENTITY_0, FUNCTION_EXTENSION_VERSION, SDCA_CTL_FUNCTION_EXTENSION_VERSION_NAME); label!(ENTITY_0, FUNCTION_STATUS, SDCA_CTL_FUNCTION_STATUS_NAME); label!(ENTITY_0, FUNCTION_ACTION, SDCA_CTL_FUNCTION_ACTION_NAME); label!(ENTITY_0, DEVICE_MANUFACTURER_ID, SDCA_CTL_DEVICE_MANUFACTURER_ID_NAME); label!(ENTITY_0, DEVICE_PART_ID, SDCA_CTL_DEVICE_PART_ID_NAME); label!(ENTITY_0, DEVICE_VERSION, SDCA_CTL_DEVICE_VERSION_NAME); label!(ENTITY_0, DEVICE_SDCA_VERSION, SDCA_CTL_DEVICE_SDCA_VERSION_NAME);
    devm_kasprintf(dev, GFP_KERNEL, cstr(b"Imp-Def %#x\0"), (*control).sel)
}

unsafe fn find_sdca_control_bits(entity: *const sdca_entity, control: *const sdca_control) -> c_uint {
    let key = ctl_key(entity, control);
    macro_rules! is_ctl { ($e:ident, $c:ident) => { key == SDCA_CTL_TYPE_S(ext_const_u32(stringify!($e)), ext_const_u32(stringify!($c))) }; }
    if is_ctl!(IT, LATENCY) || is_ctl!(OT, LATENCY) || is_ctl!(MU, LATENCY) || is_ctl!(SU, LATENCY) || is_ctl!(FU, LATENCY) || is_ctl!(XU, LATENCY) || is_ctl!(XU, FDL_MESSAGEOFFSET) || is_ctl!(XU, FDL_MESSAGELENGTH) || is_ctl!(SPE, AUTHTX_MESSAGEOFFSET) || is_ctl!(SPE, AUTHTX_MESSAGELENGTH) || is_ctl!(SPE, AUTHRX_MESSAGEOFFSET) || is_ctl!(SPE, AUTHRX_MESSAGELENGTH) || is_ctl!(CRU, LATENCY) || is_ctl!(UDMPU, LATENCY) || is_ctl!(MFPU, LATENCY) || is_ctl!(MFPU, AE_MESSAGEOFFSET) || is_ctl!(MFPU, AE_MESSAGELENGTH) || is_ctl!(SMPU, LATENCY) || is_ctl!(SMPU, HIST_MESSAGEOFFSET) || is_ctl!(SMPU, HIST_MESSAGELENGTH) || is_ctl!(SMPU, DTODTX_MESSAGEOFFSET) || is_ctl!(SMPU, DTODTX_MESSAGELENGTH) || is_ctl!(SMPU, DTODRX_MESSAGEOFFSET) || is_ctl!(SMPU, DTODRX_MESSAGELENGTH) || is_ctl!(SAPU, LATENCY) || is_ctl!(SAPU, DTODTX_MESSAGEOFFSET) || is_ctl!(SAPU, DTODTX_MESSAGELENGTH) || is_ctl!(SAPU, DTODRX_MESSAGEOFFSET) || is_ctl!(SAPU, DTODRX_MESSAGELENGTH) || is_ctl!(PPU, LATENCY) || is_ctl!(HIDE, HIDTX_MESSAGEOFFSET) || is_ctl!(HIDE, HIDTX_MESSAGELENGTH) || is_ctl!(HIDE, HIDRX_MESSAGEOFFSET) || is_ctl!(HIDE, HIDRX_MESSAGELENGTH) {
        return 32;
    }
    if is_ctl!(ENTITY_0, FUNCTION_MANUFACTURER_ID) || is_ctl!(ENTITY_0, FUNCTION_ID) || is_ctl!(ENTITY_0, FUNCTION_EXTENSION_ID) || is_ctl!(ENTITY_0, DEVICE_MANUFACTURER_ID) || is_ctl!(ENTITY_0, DEVICE_PART_ID) || is_ctl!(IT, DATAPORT_SELECTOR) || is_ctl!(OT, DATAPORT_SELECTOR) || is_ctl!(MU, MIXER) || is_ctl!(FU, CHANNEL_VOLUME) || is_ctl!(FU, GAIN) || is_ctl!(XU, XU_ID) || is_ctl!(UDMPU, ACOUSTIC_ENERGY_LEVEL_MONITOR) || is_ctl!(UDMPU, ULTRASOUND_LOOP_GAIN) || is_ctl!(MFPU, ULTRASOUND_LEVEL) || is_ctl!(PPU, HORIZONTALBALANCE) || is_ctl!(PPU, VERTICALBALANCE) {
        return 16;
    }
    if is_ctl!(FU, MUTE) || is_ctl!(FU, AGC) || is_ctl!(FU, BASS_BOOST) || is_ctl!(FU, LOUDNESS) || is_ctl!(XU, BYPASS) || is_ctl!(MFPU, BYPASS) {
        return 1;
    }
    8
}

unsafe fn find_sdca_control_datatype(entity: *const sdca_entity, control: *const sdca_control) -> sdca_control_datatype {
    let key = ctl_key(entity, control);
    macro_rules! is_ctl { ($e:ident, $c:ident) => { key == SDCA_CTL_TYPE_S(ext_const_u32(stringify!($e)), ext_const_u32(stringify!($c))) }; }
    if is_ctl!(XU, BYPASS) || is_ctl!(MFPU, BYPASS) || is_ctl!(FU, MUTE) || is_ctl!(FU, AGC) || is_ctl!(FU, BASS_BOOST) || is_ctl!(FU, LOUDNESS) {
        return sdca_control_datatype::SDCA_CTL_DATATYPE_ONEBIT;
    }
    if is_ctl!(IT, LATENCY) || is_ctl!(OT, LATENCY) || is_ctl!(MU, LATENCY) || is_ctl!(SU, LATENCY) || is_ctl!(FU, LATENCY) || is_ctl!(XU, LATENCY) || is_ctl!(CRU, LATENCY) || is_ctl!(UDMPU, LATENCY) || is_ctl!(MFPU, LATENCY) || is_ctl!(SMPU, LATENCY) || is_ctl!(SAPU, LATENCY) || is_ctl!(PPU, LATENCY) || is_ctl!(SU, SELECTOR) || is_ctl!(SAPU, PROTECTION_MODE) || is_ctl!(SMPU, HIST_BUFFER_PREAMBLE) || is_ctl!(XU, FDL_HOST_REQUEST) || is_ctl!(XU, XU_ID) || is_ctl!(CX, CLOCK_SELECT) || is_ctl!(TG, TONE_DIVIDER) || is_ctl!(ENTITY_0, FUNCTION_MANUFACTURER_ID) || is_ctl!(ENTITY_0, FUNCTION_ID) || is_ctl!(ENTITY_0, FUNCTION_EXTENSION_ID) || is_ctl!(ENTITY_0, DEVICE_MANUFACTURER_ID) || is_ctl!(ENTITY_0, DEVICE_PART_ID) {
        return sdca_control_datatype::SDCA_CTL_DATATYPE_INTEGER;
    }
    if is_ctl!(IT, MIC_BIAS) || is_ctl!(SMPU, HIST_BUFFER_MODE) || is_ctl!(PDE, REQUESTED_PS) || is_ctl!(PDE, ACTUAL_PS) || is_ctl!(ENTITY_0, FUNCTION_TYPE) {
        return sdca_control_datatype::SDCA_CTL_DATATYPE_SPEC_ENCODED_VALUE;
    }
    if is_ctl!(XU, XU_VERSION) || is_ctl!(ENTITY_0, FUNCTION_SDCA_VERSION) || is_ctl!(ENTITY_0, FUNCTION_VERSION) || is_ctl!(ENTITY_0, FUNCTION_EXTENSION_VERSION) || is_ctl!(ENTITY_0, DEVICE_VERSION) || is_ctl!(ENTITY_0, DEVICE_SDCA_VERSION) {
        return sdca_control_datatype::SDCA_CTL_DATATYPE_BCD;
    }
    if is_ctl!(FU, CHANNEL_VOLUME) || is_ctl!(FU, GAIN) || is_ctl!(MU, MIXER) || is_ctl!(PPU, HORIZONTALBALANCE) || is_ctl!(PPU, VERTICALBALANCE) || is_ctl!(MFPU, ULTRASOUND_LEVEL) || is_ctl!(UDMPU, ACOUSTIC_ENERGY_LEVEL_MONITOR) || is_ctl!(UDMPU, ULTRASOUND_LOOP_GAIN) {
        return sdca_control_datatype::SDCA_CTL_DATATYPE_Q7P8DB;
    }
    if is_ctl!(IT, USAGE) || is_ctl!(OT, USAGE) || is_ctl!(IT, CLUSTERINDEX) || is_ctl!(CRU, CLUSTERINDEX) || is_ctl!(UDMPU, CLUSTERINDEX) || is_ctl!(MFPU, CLUSTERINDEX) || is_ctl!(MFPU, CENTER_FREQUENCY_INDEX) || is_ctl!(MFPU, AE_NUMBER) || is_ctl!(SAPU, OPAQUESETREQ_INDEX) || is_ctl!(XU, FDL_SET_INDEX) || is_ctl!(CS, SAMPLERATEINDEX) || is_ctl!(GE, SELECTED_MODE) || is_ctl!(GE, DETECTED_MODE) {
        return sdca_control_datatype::SDCA_CTL_DATATYPE_BYTEINDEX;
    }
    if is_ctl!(PPU, POSTURENUMBER) {
        return sdca_control_datatype::SDCA_CTL_DATATYPE_POSTURENUMBER;
    }
    if is_ctl!(IT, DATAPORT_SELECTOR) || is_ctl!(OT, DATAPORT_SELECTOR) {
        return sdca_control_datatype::SDCA_CTL_DATATYPE_DP_INDEX;
    }
    if is_ctl!(MFPU, ALGORITHM_READY) || is_ctl!(MFPU, ALGORITHM_ENABLE) || is_ctl!(MFPU, ALGORITHM_PREPARE) || is_ctl!(SAPU, PROTECTION_STATUS) || is_ctl!(SMPU, TRIGGER_ENABLE) || is_ctl!(SMPU, TRIGGER_STATUS) || is_ctl!(SMPU, TRIGGER_READY) || is_ctl!(SPE, PRIVACY_POLICY) || is_ctl!(SPE, PRIVACY_OWNER) {
        return sdca_control_datatype::SDCA_CTL_DATATYPE_BITINDEX;
    }
    if is_ctl!(IT, KEEP_ALIVE) || is_ctl!(OT, KEEP_ALIVE) || is_ctl!(IT, NDAI_STREAM) || is_ctl!(OT, NDAI_STREAM) || is_ctl!(IT, NDAI_CATEGORY) || is_ctl!(OT, NDAI_CATEGORY) || is_ctl!(IT, NDAI_CODINGTYPE) || is_ctl!(OT, NDAI_CODINGTYPE) || is_ctl!(IT, NDAI_PACKETTYPE) || is_ctl!(OT, NDAI_PACKETTYPE) || is_ctl!(SMPU, HIST_ERROR) || is_ctl!(XU, FDL_STATUS) || is_ctl!(CS, CLOCK_VALID) || is_ctl!(SPE, PRIVACY_LOCKSTATE) || is_ctl!(ENTITY_0, COMMIT_GROUP_MASK) || is_ctl!(ENTITY_0, FUNCTION_STATUS) || is_ctl!(ENTITY_0, FUNCTION_ACTION) {
        return sdca_control_datatype::SDCA_CTL_DATATYPE_BITMAP;
    }
    if is_ctl!(IT, MATCHING_GUID) || is_ctl!(OT, MATCHING_GUID) || is_ctl!(ENTITY_0, MATCHING_GUID) {
        return sdca_control_datatype::SDCA_CTL_DATATYPE_GUID;
    }
    sdca_control_datatype::SDCA_CTL_DATATYPE_IMPDEF
}

unsafe fn find_sdca_control_volatile(entity: *const sdca_entity, control: *const sdca_control) -> bool_ {
    if (*control).mode == SDCA_ACCESS_MODE_DC {
        return false;
    }
    if (*control).mode == SDCA_ACCESS_MODE_RO || (*control).mode == SDCA_ACCESS_MODE_RW1S || (*control).mode == SDCA_ACCESS_MODE_RW1C {
        return true;
    }
    let key = ctl_key(entity, control);
    macro_rules! is_ctl { ($e:ident, $c:ident) => { key == SDCA_CTL_TYPE_S(ext_const_u32(stringify!($e)), ext_const_u32(stringify!($c))) }; }
    is_ctl!(XU, FDL_CURRENTOWNER) || is_ctl!(XU, FDL_MESSAGEOFFSET) || is_ctl!(XU, FDL_MESSAGELENGTH) || is_ctl!(XU, FDL_STATUS) || is_ctl!(XU, FDL_HOST_REQUEST) || is_ctl!(SPE, AUTHTX_CURRENTOWNER) || is_ctl!(SPE, AUTHTX_MESSAGEOFFSET) || is_ctl!(SPE, AUTHTX_MESSAGELENGTH) || is_ctl!(SPE, AUTHRX_CURRENTOWNER) || is_ctl!(SPE, AUTHRX_MESSAGEOFFSET) || is_ctl!(SPE, AUTHRX_MESSAGELENGTH) || is_ctl!(MFPU, AE_CURRENTOWNER) || is_ctl!(MFPU, AE_MESSAGEOFFSET) || is_ctl!(MFPU, AE_MESSAGELENGTH) || is_ctl!(SMPU, HIST_CURRENTOWNER) || is_ctl!(SMPU, HIST_MESSAGEOFFSET) || is_ctl!(SMPU, HIST_MESSAGELENGTH) || is_ctl!(SMPU, DTODTX_CURRENTOWNER) || is_ctl!(SMPU, DTODTX_MESSAGEOFFSET) || is_ctl!(SMPU, DTODTX_MESSAGELENGTH) || is_ctl!(SMPU, DTODRX_CURRENTOWNER) || is_ctl!(SMPU, DTODRX_MESSAGEOFFSET) || is_ctl!(SMPU, DTODRX_MESSAGELENGTH) || is_ctl!(SAPU, DTODTX_CURRENTOWNER) || is_ctl!(SAPU, DTODTX_MESSAGEOFFSET) || is_ctl!(SAPU, DTODTX_MESSAGELENGTH) || is_ctl!(SAPU, DTODRX_CURRENTOWNER) || is_ctl!(SAPU, DTODRX_MESSAGEOFFSET) || is_ctl!(SAPU, DTODRX_MESSAGELENGTH) || is_ctl!(HIDE, HIDTX_CURRENTOWNER) || is_ctl!(HIDE, HIDTX_MESSAGEOFFSET) || is_ctl!(HIDE, HIDTX_MESSAGELENGTH) || is_ctl!(HIDE, HIDRX_CURRENTOWNER) || is_ctl!(HIDE, HIDRX_MESSAGEOFFSET) || is_ctl!(HIDE, HIDRX_MESSAGELENGTH)
}

unsafe fn find_sdca_control_range(dev: *mut device, control_node: *mut fwnode_handle, range: *mut sdca_control_range) -> c_int {
    let mut num_range = fwnode_property_count_u8(control_node, cstr(b"mipi-sdca-control-range\0"));
    if num_range == 0 || num_range == -EINVAL {
        return 0;
    } else if num_range < 0 {
        return num_range;
    } else if (num_range as usize) < 2 * size_of::<u16>() {
        return -EINVAL;
    }
    let range_list = devm_kcalloc(dev, num_range as usize, size_of::<u8>(), GFP_KERNEL) as *mut u8;
    if range_list.is_null() {
        return -ENOMEM;
    }
    fwnode_property_read_u8_array(control_node, cstr(b"mipi-sdca-control-range\0"), range_list, num_range);
    let limits = range_list as *mut u16;
    (*range).cols = le16_to_cpu(*limits.add(0)) as u32;
    (*range).rows = le16_to_cpu(*limits.add(1)) as u32;
    (*range).data = limits.add(2) as *mut u32;
    num_range = ((num_range as usize - (2 * size_of::<u16>())) / size_of::<u32>()) as c_int;
    if num_range as u32 != (*range).cols * (*range).rows {
        return -EINVAL;
    }
    let mut i = 0;
    while i < num_range {
        *(*range).data.add(i as usize) = le32_to_cpu(*(*range).data.add(i as usize));
        i += 1;
    }
    0
}

unsafe fn find_sdca_control_value(
    _dev: *mut device,
    _entity: *mut sdca_entity,
    control_node: *mut fwnode_handle,
    control: *mut sdca_control,
    label: *const c_char,
) -> c_int {
    let mut property = [0 as c_char; SDCA_PROPERTY_LENGTH];
    let mut global = true;
    let mut tmp: u32 = 0;
    snprintf(property.as_mut_ptr(), property.len(), cstr(b"mipi-sdca-control-%s\0"), label);
    let mut ret = fwnode_property_read_u32(control_node, property.as_mut_ptr(), &mut tmp);
    if ret == -EINVAL {
        global = false;
    } else if ret != 0 {
        return ret;
    }
    let mut i = 0usize;
    for_each_set_bit_u64((*control).cn_list, |cn| {
        if !global {
            snprintf(property.as_mut_ptr(), property.len(), cstr(b"mipi-sdca-control-cn-%d-%s\0"), cn, label);
            ret = fwnode_property_read_u32(control_node, property.as_mut_ptr(), &mut tmp);
            if ret != 0 {
                return;
            }
        }
        *(*control).values.add(i) = tmp;
        i += 1;
    });
    ret
}

unsafe fn find_sdca_control_reset(entity: *const sdca_entity, control: *mut sdca_control) -> c_int {
    let key = ctl_key(entity, control);
    macro_rules! is_ctl { ($e:ident, $c:ident) => { key == SDCA_CTL_TYPE_S(ext_const_u32(stringify!($e)), ext_const_u32(stringify!($c))) }; }
    if is_ctl!(FU, AGC) || is_ctl!(FU, BASS_BOOST) || is_ctl!(FU, LOUDNESS) || is_ctl!(SMPU, TRIGGER_ENABLE) || is_ctl!(GE, SELECTED_MODE) || is_ctl!(TG, TONE_DIVIDER) || is_ctl!(ENTITY_0, COMMIT_GROUP_MASK) {
        (*control).has_reset = true;
        (*control).reset = 0;
    } else if is_ctl!(XU, BYPASS) || is_ctl!(MFPU, BYPASS) || is_ctl!(FU, MUTE) || is_ctl!(CX, CLOCK_SELECT) {
        (*control).has_reset = true;
        (*control).reset = 1;
    } else if is_ctl!(PDE, REQUESTED_PS) {
        (*control).has_reset = true;
        (*control).reset = 3;
    }
    0
}

unsafe fn find_sdca_entity_control(dev: *mut device, entity: *mut sdca_entity, control_node: *mut fwnode_handle, control: *mut sdca_control) -> c_int {
    let mut tmp: u32 = 0;
    let mut ret = fwnode_property_read_u32(control_node, cstr(b"mipi-sdca-control-access-mode\0"), &mut tmp);
    if ret != 0 { return ret; }
    (*control).mode = tmp;
    ret = fwnode_property_read_u32(control_node, cstr(b"mipi-sdca-control-access-layer\0"), &mut tmp);
    if ret != 0 { return ret; }
    (*control).layers = tmp;
    ret = fwnode_property_read_u64(control_node, cstr(b"mipi-sdca-control-cn-list\0"), &mut (*control).cn_list);
    if ret == -EINVAL {
        (*control).cn_list = 0x1;
    } else if ret != 0 || (*control).cn_list == 0 {
        return ret;
    }
    (*control).values = devm_kcalloc(dev, hweight64((*control).cn_list) as usize, size_of::<u32>(), GFP_KERNEL) as *mut u32;
    if (*control).values.is_null() { return -ENOMEM; }

    if (*control).mode == SDCA_ACCESS_MODE_DC {
        ret = find_sdca_control_value(dev, entity, control_node, control, cstr(b"dc-value\0"));
        if ret != 0 { return ret; }
        (*control).has_fixed = true;
    } else if (*control).mode == SDCA_ACCESS_MODE_RW || (*control).mode == SDCA_ACCESS_MODE_DUAL {
        ret = find_sdca_control_value(dev, entity, control_node, control, cstr(b"default-value\0"));
        if ret == 0 { (*control).has_default = true; }
        ret = find_sdca_control_value(dev, entity, control_node, control, cstr(b"fixed-value\0"));
        if ret == 0 { (*control).has_fixed = true; }
        ret = fwnode_property_read_u32(control_node, cstr(b"mipi-sdca-control-deferrable\0"), &mut tmp);
        if ret == 0 { (*control).deferrable = tmp != 0; }
    } else if (*control).mode == SDCA_ACCESS_MODE_RO {
        ret = fwnode_property_read_u32(control_node, cstr(b"mipi-sdca-control-deferrable\0"), &mut tmp);
        if ret == 0 { (*control).deferrable = tmp != 0; }
    }

    (*control).is_volatile = find_sdca_control_volatile(entity, control);
    ret = find_sdca_control_reset(entity, control);
    if ret != 0 { return ret; }
    ret = find_sdca_control_range(dev, control_node, &mut (*control).range);
    if ret != 0 { return ret; }
    ret = fwnode_property_read_u32(control_node, cstr(b"mipi-sdca-control-interrupt-position\0"), &mut tmp);
    if ret == 0 { (*control).interrupt_position = tmp; } else { (*control).interrupt_position = SDCA_NO_INTERRUPT; }
    (*control).label = find_sdca_control_label(dev, entity, control);
    if (*control).label.is_null() { return -ENOMEM; }
    (*control).type_ = find_sdca_control_datatype(entity, control);
    (*control).nbits = find_sdca_control_bits(entity, control);
    0
}

unsafe fn find_sdca_entity_controls(dev: *mut device, entity_node: *mut fwnode_handle, entity: *mut sdca_entity) -> c_int {
    let mut control_list: u64 = 0;
    let ret0 = fwnode_property_read_u64(entity_node, cstr(b"mipi-sdca-control-list\0"), &mut control_list);
    if ret0 == -EINVAL || control_list == 0 { return 0; }
    if ret0 != 0 { return ret0; }
    let num_controls = hweight64(control_list);
    let controls = devm_kcalloc(dev, num_controls as usize, size_of::<sdca_control>(), GFP_KERNEL) as *mut sdca_control;
    if controls.is_null() { return -ENOMEM; }
    let mut i = 0usize;
    let mut ret = 0;
    for_each_set_bit_u64(control_list, |control_sel| {
        if ret != 0 { return; }
        let mut control_property = [0 as c_char; SDCA_PROPERTY_LENGTH];
        snprintf(control_property.as_mut_ptr(), control_property.len(), cstr(b"mipi-sdca-control-0x%X-subproperties\0"), control_sel);
        let control_node = fwnode_get_named_child_node(entity_node, control_property.as_mut_ptr());
        if control_node.is_null() { ret = -EINVAL; return; }
        (*controls.add(i)).sel = control_sel as u32;
        ret = find_sdca_entity_control(dev, entity, control_node, controls.add(i));
        fwnode_handle_put(control_node);
        i += 1;
    });
    if ret != 0 { return ret; }
    (*entity).num_controls = num_controls;
    (*entity).controls = controls;
    0
}

unsafe fn find_sdca_iot_dataport(terminal: *mut sdca_entity_iot) -> bool_ {
    matches!((*terminal).type_, 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14)
}

unsafe fn find_sdca_entity_iot(dev: *mut device, entity_node: *mut fwnode_handle, entity: *mut sdca_entity) -> c_int {
    let terminal = &mut (*entity).iot as *mut sdca_entity_iot;
    let mut tmp: u32 = 0;
    let mut ret = fwnode_property_read_u32(entity_node, cstr(b"mipi-sdca-terminal-type\0"), &mut tmp);
    if ret != 0 { return ret; }
    (*terminal).type_ = tmp;
    (*terminal).is_dataport = find_sdca_iot_dataport(terminal);
    if !(*terminal).is_dataport {
        let type_name = sdca_find_terminal_name(tmp as sdca_terminal_type);
        if !type_name.is_null() {
            (*entity).label = devm_kasprintf(dev, GFP_KERNEL, cstr(b"%s %s\0"), (*entity).label, type_name);
            if (*entity).label.is_null() { return -ENOMEM; }
        }
    }
    ret = fwnode_property_read_u32(entity_node, cstr(b"mipi-sdca-terminal-reference-number\0"), &mut tmp);
    if ret == 0 { (*terminal).reference = tmp; }
    ret = fwnode_property_read_u32(entity_node, cstr(b"mipi-sdca-terminal-connector-type\0"), &mut tmp);
    if ret == 0 { (*terminal).connector = tmp; }
    ret = fwnode_property_read_u32(entity_node, cstr(b"mipi-sdca-terminal-transducer-count\0"), &mut tmp);
    if ret == 0 { (*terminal).num_transducer = tmp; }
    0
}

unsafe fn find_sdca_entity_cs(_dev: *mut device, entity_node: *mut fwnode_handle, entity: *mut sdca_entity) -> c_int {
    let clock = &mut (*entity).cs as *mut sdca_entity_cs;
    let mut tmp = 0;
    let mut ret = fwnode_property_read_u32(entity_node, cstr(b"mipi-sdca-cs-type\0"), &mut tmp);
    if ret != 0 { return ret; }
    (*clock).type_ = tmp;
    ret = fwnode_property_read_u32(entity_node, cstr(b"mipi-sdca-clock-valid-max-delay\0"), &mut tmp);
    if ret == 0 { (*clock).max_delay = tmp; }
    0
}

unsafe fn find_sdca_entity_pde(dev: *mut device, entity_node: *mut fwnode_handle, entity: *mut sdca_entity) -> c_int {
    const mult_delay: c_int = 3;
    let power = &mut (*entity).pde as *mut sdca_entity_pde;
    let mut num_delays = fwnode_property_count_u32(entity_node, cstr(b"mipi-sdca-powerdomain-transition-max-delay\0"));
    if num_delays <= 0 || num_delays % mult_delay != 0 || num_delays > SDCA_MAX_DELAY_COUNT { return -EINVAL; }
    let delay_list = kcalloc(num_delays as usize, size_of::<u32>(), GFP_KERNEL) as *mut u32;
    if delay_list.is_null() { return -ENOMEM; }
    fwnode_property_read_u32_array(entity_node, cstr(b"mipi-sdca-powerdomain-transition-max-delay\0"), delay_list, num_delays);
    num_delays /= mult_delay;
    let delays = devm_kcalloc(dev, num_delays as usize, size_of::<sdca_pde_delay>(), GFP_KERNEL) as *mut sdca_pde_delay;
    if delays.is_null() { kfree(delay_list as *mut c_void); return -ENOMEM; }
    let mut i = 0;
    let mut j = 0;
    while i < num_delays {
        (*delays.add(i as usize)).from_ps = *delay_list.add(j as usize); j += 1;
        (*delays.add(i as usize)).to_ps = *delay_list.add(j as usize); j += 1;
        (*delays.add(i as usize)).us = *delay_list.add(j as usize); j += 1;
        i += 1;
    }
    (*power).num_max_delay = num_delays;
    (*power).max_delay = delays;
    kfree(delay_list as *mut c_void);
    0
}

unsafe fn find_sdca_entity_ge(dev: *mut device, entity_node: *mut fwnode_handle, entity: *mut sdca_entity) -> c_int {
    let group = &mut (*entity).ge as *mut sdca_entity_ge;
    let num_affected = fwnode_property_count_u8(entity_node, cstr(b"mipi-sdca-ge-selectedmode-controls-affected\0"));
    if num_affected == 0 { return 0; }
    if num_affected < 0 { return num_affected; }
    if num_affected > SDCA_MAX_AFFECTED_COUNT { return -EINVAL; }
    let affected_list = kcalloc(num_affected as usize, size_of::<u8>(), GFP_KERNEL) as *mut u8;
    if affected_list.is_null() { return -ENOMEM; }
    fwnode_property_read_u8_array(entity_node, cstr(b"mipi-sdca-ge-selectedmode-controls-affected\0"), affected_list, num_affected);
    (*group).num_modes = *affected_list;
    let mut affected_iter = affected_list.add(1);
    (*group).modes = devm_kcalloc(dev, (*group).num_modes as usize, size_of::<sdca_ge_mode>(), GFP_KERNEL) as *mut sdca_ge_mode;
    if (*group).modes.is_null() { kfree(affected_list as *mut c_void); return -ENOMEM; }
    let end = affected_list.add(num_affected as usize);
    let mut i = 0usize;
    while i < (*group).num_modes as usize {
        let raw = affected_iter as *mut raw_ge_mode;
        let mode = (*group).modes.add(i);
        affected_iter = affected_iter.add(size_of::<raw_ge_mode>());
        if affected_iter > end { kfree(affected_list as *mut c_void); return -EINVAL; }
        (*mode).val = (*raw).val;
        (*mode).num_controls = (*raw).num_controls;
        let raw_controls = affected_iter as *mut raw_ge_control;
        affected_iter = affected_iter.add((*mode).num_controls as usize * size_of::<raw_ge_control>());
        if affected_iter > end { kfree(affected_list as *mut c_void); return -EINVAL; }
        (*mode).controls = devm_kcalloc(dev, (*mode).num_controls as usize, size_of::<sdca_ge_control>(), GFP_KERNEL) as *mut sdca_ge_control;
        if (*mode).controls.is_null() { kfree(affected_list as *mut c_void); return -ENOMEM; }
        let mut j = 0usize;
        while j < (*mode).num_controls as usize {
            (*(*mode).controls.add(j)).id = (*raw_controls.add(j)).id;
            (*(*mode).controls.add(j)).sel = (*raw_controls.add(j)).sel;
            (*(*mode).controls.add(j)).cn = (*raw_controls.add(j)).cn;
            (*(*mode).controls.add(j)).val = le32_to_cpu((*raw_controls.add(j)).val);
            j += 1;
        }
        i += 1;
    }
    kfree(affected_list as *mut c_void);
    0
}

unsafe fn find_sdca_entity_hide(dev: *mut device, entity_node: *mut fwnode_handle, entity: *mut sdca_entity) -> c_int {
    let hide = &mut (*entity).hide as *mut sdca_entity_hide;
    let mut delay: c_uint = 0;
    let mut ret = fwnode_property_read_u32(entity_node, cstr(b"mipi-sdca-RxUMP-ownership-transition-max-delay\0"), &mut delay);
    if ret == 0 { (*hide).max_delay = delay; }
    let mut num_reports = fwnode_property_count_u32(entity_node, cstr(b"mipi-sdca-HIDTx-supported-report-ids\0"));
    if num_reports < 0 && num_reports != -EINVAL { return num_reports; } else if num_reports > 0 {
        (*hide).num_hidtx_ids = num_reports;
        (*hide).hidtx_ids = devm_kcalloc(dev, num_reports as usize, size_of::<u32>(), GFP_KERNEL) as *mut u32;
        if (*hide).hidtx_ids.is_null() { return -ENOMEM; }
        fwnode_property_read_u32_array(entity_node, cstr(b"mipi-sdca-HIDTx-supported-report-ids\0"), (*hide).hidtx_ids, num_reports);
    }
    num_reports = fwnode_property_count_u32(entity_node, cstr(b"mipi-sdca-HIDRx-supported-report-ids\0"));
    if num_reports < 0 && num_reports != -EINVAL { return num_reports; } else if num_reports > 0 {
        (*hide).num_hidrx_ids = num_reports;
        (*hide).hidrx_ids = devm_kcalloc(dev, num_reports as usize, size_of::<u32>(), GFP_KERNEL) as *mut u32;
        if (*hide).hidrx_ids.is_null() { return -ENOMEM; }
        fwnode_property_read_u32_array(entity_node, cstr(b"mipi-sdca-HIDRx-supported-report-ids\0"), (*hide).hidrx_ids, num_reports);
    }
    /*
     * FIXME: This should probably link to the actual sdca_function_data pointer,
     * but updating to do so should probably wait until we have a user.
     */
    num_reports = fwnode_property_count_u32(entity_node, cstr(b"mipi-sdca-hide-related-audio-function-list\0"));
    if num_reports <= 0 || num_reports as usize > (*hide).af_number_list.len() { return -EINVAL; }
    (*hide).hide_reside_function_num = num_reports;
    fwnode_property_read_u32_array(entity_node, cstr(b"mipi-sdca-hide-related-audio-function-list\0"), (*hide).af_number_list.as_mut_ptr(), num_reports);
    0
}

unsafe fn find_sdca_entity_xu(_dev: *mut device, entity_node: *mut fwnode_handle, entity: *mut sdca_entity) -> c_int {
    let xu = &mut (*entity).xu as *mut sdca_entity_xu;
    let mut tmp = 0;
    let mut ret = fwnode_property_read_u32(entity_node, cstr(b"mipi-sdca-RxUMP-ownership-transition-max-delay\0"), &mut tmp);
    if ret == 0 { (*xu).max_delay = tmp; }
    ret = fwnode_property_read_u32(entity_node, cstr(b"mipi-sdca-FDL-reset-mechanism\0"), &mut tmp);
    if ret == 0 { (*xu).reset_mechanism = tmp; }
    0
}

unsafe fn find_sdca_entity(dev: *mut device, function: *mut sdca_function_data, function_node: *mut fwnode_handle, entity_node: *mut fwnode_handle, entity: *mut sdca_entity) -> c_int {
    let mut tmp = 0;
    let mut ret = fwnode_property_read_string(entity_node, cstr(b"mipi-sdca-entity-label\0"), &mut (*entity).label);
    if ret != 0 { return ret; }
    if (*(*function).desc).duplicate {
        (*entity).label = devm_kasprintf(dev, GFP_KERNEL, cstr(b"%d %s\0"), (*(*function).desc).adr, (*entity).label);
        if (*entity).label.is_null() { return -ENOMEM; }
    }
    ret = fwnode_property_read_u32(entity_node, cstr(b"mipi-sdca-entity-type\0"), &mut tmp);
    if ret != 0 { return ret; }
    (*entity).type_ = tmp;
    if (*entity).type_ == SDCA_ENTITY_TYPE_IT || (*entity).type_ == SDCA_ENTITY_TYPE_OT {
        ret = find_sdca_entity_iot(dev, entity_node, entity);
    } else if (*entity).type_ == SDCA_ENTITY_TYPE_XU {
        ret = find_sdca_entity_xu(dev, entity_node, entity);
    } else if (*entity).type_ == SDCA_ENTITY_TYPE_CS {
        ret = find_sdca_entity_cs(dev, entity_node, entity);
    } else if (*entity).type_ == SDCA_ENTITY_TYPE_PDE {
        ret = find_sdca_entity_pde(dev, entity_node, entity);
    } else if (*entity).type_ == SDCA_ENTITY_TYPE_GE {
        ret = find_sdca_entity_ge(dev, entity_node, entity);
    } else if (*entity).type_ == SDCA_ENTITY_TYPE_HIDE {
        ret = find_sdca_entity_hide(dev, entity_node, entity);
    }
    if ret != 0 { return ret; }
    find_sdca_entity_controls(dev, entity_node, entity)
}

unsafe fn find_sdca_entities(dev: *mut device, function_node: *mut fwnode_handle, function: *mut sdca_function_data) -> c_int {
    let mut num_entities = fwnode_property_count_u32(function_node, cstr(b"mipi-sdca-entity-id-list\0"));
    if num_entities <= 0 || num_entities > SDCA_MAX_ENTITY_COUNT { return -EINVAL; }
    let entities = devm_kcalloc(dev, (num_entities + 1) as usize, size_of::<sdca_entity>(), GFP_KERNEL) as *mut sdca_entity;
    if entities.is_null() { return -ENOMEM; }
    let entity_list = kcalloc(num_entities as usize, size_of::<u32>(), GFP_KERNEL) as *mut u32;
    if entity_list.is_null() { return -ENOMEM; }
    fwnode_property_read_u32_array(function_node, cstr(b"mipi-sdca-entity-id-list\0"), entity_list, num_entities);
    let mut i = 0;
    while i < num_entities {
        (*entities.add(i as usize)).id = *entity_list.add(i as usize);
        i += 1;
    }
    i = 0;
    while i < num_entities {
        let mut entity_property = [0 as c_char; SDCA_PROPERTY_LENGTH];
        snprintf(entity_property.as_mut_ptr(), entity_property.len(), cstr(b"mipi-sdca-entity-id-0x%X-subproperties\0"), (*entities.add(i as usize)).id);
        let entity_node = fwnode_get_named_child_node(function_node, entity_property.as_mut_ptr());
        if entity_node.is_null() { kfree(entity_list as *mut c_void); return -EINVAL; }
        let ret = find_sdca_entity(dev, function, function_node, entity_node, entities.add(i as usize));
        fwnode_handle_put(entity_node);
        if ret != 0 { kfree(entity_list as *mut c_void); return ret; }
        i += 1;
    }
    (*entities.add(num_entities as usize)).label = cstr(b"entity0\0");
    let ret = find_sdca_entity_controls(dev, function_node, entities.add(num_entities as usize));
    if ret != 0 { kfree(entity_list as *mut c_void); return ret; }
    (*function).num_entities = num_entities + 1;
    (*function).entities = entities;
    kfree(entity_list as *mut c_void);
    0
}

#[no_mangle]
pub unsafe extern "C" fn sdca_find_entity_by_label(function: *mut sdca_function_data, mut entity_label: *const c_char) -> *mut sdca_entity {
    let mut tmp = [0 as c_char; 64];
    if (*(*function).desc).duplicate {
        snprintf(tmp.as_mut_ptr(), tmp.len(), cstr(b"%d %s\0"), (*(*function).desc).adr, entity_label);
        entity_label = tmp.as_mut_ptr();
    }
    let mut i = 0;
    while i < (*function).num_entities {
        let entity = (*function).entities.add(i as usize);
        /* check whole string first*/
        if strcmp((*entity).label, entity_label) == 0 { return entity; }
        i += 1;
    }
    i = 0;
    while i < (*function).num_entities {
        let entity = (*function).entities.add(i as usize);
        if strncmp((*entity).label, entity_label, strlen(entity_label)) == 0 { return entity; }
        i += 1;
    }
    null_mut()
}

unsafe fn find_sdca_entity_by_id(function: *mut sdca_function_data, id: c_int) -> *mut sdca_entity {
    let mut i = 0;
    while i < (*function).num_entities {
        let entity = (*function).entities.add(i as usize);
        if (*entity).id == id as u32 { return entity; }
        i += 1;
    }
    null_mut()
}

unsafe fn find_sdca_entity_connection_iot(_dev: *mut device, function: *mut sdca_function_data, entity_node: *mut fwnode_handle, entity: *mut sdca_entity) -> c_int {
    let terminal = &mut (*entity).iot as *mut sdca_entity_iot;
    let clock_node = fwnode_get_named_child_node(entity_node, cstr(b"mipi-sdca-terminal-clock-connection\0"));
    if clock_node.is_null() { return 0; }
    let mut clock_label: *const c_char = null();
    let ret = fwnode_property_read_string(clock_node, cstr(b"mipi-sdca-entity-label\0"), &mut clock_label);
    if ret != 0 { fwnode_handle_put(clock_node); return ret; }
    let clock_entity = sdca_find_entity_by_label(function, clock_label);
    if clock_entity.is_null() { fwnode_handle_put(clock_node); return -EINVAL; }
    (*terminal).clock = clock_entity;
    fwnode_handle_put(clock_node);
    0
}

unsafe fn find_sdca_entity_connection_pde(dev: *mut device, function: *mut sdca_function_data, entity_node: *mut fwnode_handle, entity: *mut sdca_entity) -> c_int {
    let power = &mut (*entity).pde as *mut sdca_entity_pde;
    let num_managed = fwnode_property_count_u32(entity_node, cstr(b"mipi-sdca-powerdomain-managed-list\0"));
    if num_managed == 0 { return 0; }
    if num_managed < 0 { return num_managed; }
    if num_managed > SDCA_MAX_ENTITY_COUNT { return -EINVAL; }
    let managed = devm_kcalloc(dev, num_managed as usize, size_of::<*mut sdca_entity>(), GFP_KERNEL) as *mut *mut sdca_entity;
    if managed.is_null() { return -ENOMEM; }
    let managed_list = kcalloc(num_managed as usize, size_of::<u32>(), GFP_KERNEL) as *mut u32;
    if managed_list.is_null() { return -ENOMEM; }
    fwnode_property_read_u32_array(entity_node, cstr(b"mipi-sdca-powerdomain-managed-list\0"), managed_list, num_managed);
    let mut i = 0;
    while i < num_managed {
        *managed.add(i as usize) = find_sdca_entity_by_id(function, *managed_list.add(i as usize) as c_int);
        if (*managed.add(i as usize)).is_null() { kfree(managed_list as *mut c_void); return -EINVAL; }
        i += 1;
    }
    (*power).num_managed = num_managed;
    (*power).managed = managed;
    kfree(managed_list as *mut c_void);
    0
}

unsafe fn find_sdca_entity_connection_ge(_dev: *mut device, function: *mut sdca_function_data, _entity_node: *mut fwnode_handle, entity: *mut sdca_entity) -> c_int {
    let mut i = 0;
    while i < (*entity).ge.num_modes as c_int {
        let mode = (*entity).ge.modes.add(i as usize);
        let mut j = 0;
        while j < (*mode).num_controls as c_int {
            let affected = (*mode).controls.add(j as usize);
            let managed = find_sdca_entity_by_id(function, (*affected).id as c_int);
            if managed.is_null() { return -EINVAL; }
            if !(*managed).group.is_null() && (*managed).group != entity { return -EINVAL; }
            (*managed).group = entity;
            j += 1;
        }
        i += 1;
    }
    0
}

unsafe fn find_sdca_entity_connection(dev: *mut device, function: *mut sdca_function_data, entity_node: *mut fwnode_handle, entity: *mut sdca_entity) -> c_int {
    let mut ret = if (*entity).type_ == SDCA_ENTITY_TYPE_IT || (*entity).type_ == SDCA_ENTITY_TYPE_OT {
        find_sdca_entity_connection_iot(dev, function, entity_node, entity)
    } else if (*entity).type_ == SDCA_ENTITY_TYPE_PDE {
        find_sdca_entity_connection_pde(dev, function, entity_node, entity)
    } else if (*entity).type_ == SDCA_ENTITY_TYPE_GE {
        find_sdca_entity_connection_ge(dev, function, entity_node, entity)
    } else { 0 };
    if ret != 0 { return ret; }
    let mut pin_list: u64 = 0;
    ret = fwnode_property_read_u64(entity_node, cstr(b"mipi-sdca-input-pin-list\0"), &mut pin_list);
    if ret == -EINVAL { return 0; }
    if ret != 0 { return ret; }
    if pin_list & BIT(0) != 0 { return -EINVAL; }
    if pin_list == 0 { return 0; }
    let num_pins = hweight64(pin_list);
    let pins = devm_kcalloc(dev, num_pins as usize, size_of::<*mut sdca_entity>(), GFP_KERNEL) as *mut *mut sdca_entity;
    if pins.is_null() { return -ENOMEM; }
    let mut i = 0usize;
    let mut out_ret = 0;
    for_each_set_bit_u64(pin_list, |pin| {
        if out_ret != 0 { return; }
        let mut pin_property = [0 as c_char; SDCA_PROPERTY_LENGTH];
        snprintf(pin_property.as_mut_ptr(), pin_property.len(), cstr(b"mipi-sdca-input-pin-%d\0"), pin);
        let connected_node = fwnode_get_named_child_node(entity_node, pin_property.as_mut_ptr());
        if connected_node.is_null() { out_ret = -EINVAL; return; }
        let mut connected_label: *const c_char = null();
        out_ret = fwnode_property_read_string(connected_node, cstr(b"mipi-sdca-entity-label\0"), &mut connected_label);
        if out_ret != 0 { fwnode_handle_put(connected_node); return; }
        let connected_entity = sdca_find_entity_by_label(function, connected_label);
        if connected_entity.is_null() { fwnode_handle_put(connected_node); out_ret = -EINVAL; return; }
        *pins.add(i) = connected_entity;
        i += 1;
        fwnode_handle_put(connected_node);
    });
    if out_ret != 0 { return out_ret; }
    (*entity).num_sources = num_pins;
    (*entity).sources = pins;
    0
}

unsafe fn find_sdca_connections(dev: *mut device, function_node: *mut fwnode_handle, function: *mut sdca_function_data) -> c_int {
    /* Entity 0 cannot have connections */
    let mut i = 0;
    while i < (*function).num_entities - 1 {
        let entity = (*function).entities.add(i as usize);
        let mut entity_property = [0 as c_char; SDCA_PROPERTY_LENGTH];
        snprintf(entity_property.as_mut_ptr(), entity_property.len(), cstr(b"mipi-sdca-entity-id-0x%X-subproperties\0"), (*entity).id);
        let entity_node = fwnode_get_named_child_node(function_node, entity_property.as_mut_ptr());
        if entity_node.is_null() { return -EINVAL; }
        let ret = find_sdca_entity_connection(dev, function, entity_node, entity);
        fwnode_handle_put(entity_node);
        if ret != 0 { return ret; }
        i += 1;
    }
    0
}

unsafe fn find_sdca_cluster_channel(_dev: *mut device, cluster: *mut sdca_cluster, channel_node: *mut fwnode_handle, channel: *mut sdca_channel) -> c_int {
    let mut tmp = 0;
    let mut ret = fwnode_property_read_u32(channel_node, cstr(b"mipi-sdca-cluster-channel-id\0"), &mut tmp);
    if ret != 0 { return ret; }
    (*channel).id = tmp;
    ret = fwnode_property_read_u32(channel_node, cstr(b"mipi-sdca-cluster-channel-purpose\0"), &mut tmp);
    if ret != 0 { return ret; }
    (*channel).purpose = tmp;
    ret = fwnode_property_read_u32(channel_node, cstr(b"mipi-sdca-cluster-channel-relationship\0"), &mut tmp);
    if ret != 0 { return ret; }
    (*channel).relationship = tmp;
    let _ = cluster;
    0
}

unsafe fn find_sdca_cluster_channels(dev: *mut device, cluster_node: *mut fwnode_handle, cluster: *mut sdca_cluster) -> c_int {
    let mut num_channels: u32 = 0;
    let ret0 = fwnode_property_read_u32(cluster_node, cstr(b"mipi-sdca-channel-count\0"), &mut num_channels);
    if ret0 < 0 { return ret0; }
    if num_channels as c_int > SDCA_MAX_CHANNEL_COUNT { return -EINVAL; }
    let channels = devm_kcalloc(dev, num_channels as usize, size_of::<sdca_channel>(), GFP_KERNEL) as *mut sdca_channel;
    if channels.is_null() { return -ENOMEM; }
    let mut i = 0u32;
    while i < num_channels {
        let mut channel_property = [0 as c_char; SDCA_PROPERTY_LENGTH];
        snprintf(channel_property.as_mut_ptr(), channel_property.len(), cstr(b"mipi-sdca-channel-%d-subproperties\0"), i + 1);
        let channel_node = fwnode_get_named_child_node(cluster_node, channel_property.as_mut_ptr());
        if channel_node.is_null() { return -EINVAL; }
        let ret = find_sdca_cluster_channel(dev, cluster, channel_node, channels.add(i as usize));
        fwnode_handle_put(channel_node);
        if ret != 0 { return ret; }
        i += 1;
    }
    (*cluster).num_channels = num_channels;
    (*cluster).channels = channels;
    0
}

unsafe fn find_sdca_clusters(dev: *mut device, function_node: *mut fwnode_handle, function: *mut sdca_function_data) -> c_int {
    let num_clusters = fwnode_property_count_u32(function_node, cstr(b"mipi-sdca-cluster-id-list\0"));
    if num_clusters == 0 || num_clusters == -EINVAL { return 0; }
    if num_clusters < 0 { return num_clusters; }
    if num_clusters > SDCA_MAX_CLUSTER_COUNT { return -EINVAL; }
    let clusters = devm_kcalloc(dev, num_clusters as usize, size_of::<sdca_cluster>(), GFP_KERNEL) as *mut sdca_cluster;
    if clusters.is_null() { return -ENOMEM; }
    let cluster_list = kcalloc(num_clusters as usize, size_of::<u32>(), GFP_KERNEL) as *mut u32;
    if cluster_list.is_null() { return -ENOMEM; }
    fwnode_property_read_u32_array(function_node, cstr(b"mipi-sdca-cluster-id-list\0"), cluster_list, num_clusters);
    let mut i = 0;
    while i < num_clusters {
        (*clusters.add(i as usize)).id = *cluster_list.add(i as usize);
        i += 1;
    }
    i = 0;
    while i < num_clusters {
        let mut cluster_property = [0 as c_char; SDCA_PROPERTY_LENGTH];
        snprintf(cluster_property.as_mut_ptr(), cluster_property.len(), cstr(b"mipi-sdca-cluster-id-0x%X-subproperties\0"), (*clusters.add(i as usize)).id);
        let cluster_node = fwnode_get_named_child_node(function_node, cluster_property.as_mut_ptr());
        if cluster_node.is_null() { kfree(cluster_list as *mut c_void); return -EINVAL; }
        let ret = find_sdca_cluster_channels(dev, cluster_node, clusters.add(i as usize));
        fwnode_handle_put(cluster_node);
        if ret != 0 { kfree(cluster_list as *mut c_void); return ret; }
        i += 1;
    }
    (*function).num_clusters = num_clusters;
    (*function).clusters = clusters;
    kfree(cluster_list as *mut c_void);
    0
}

unsafe fn find_sdca_filesets(dev: *mut device, function_node: *mut fwnode_handle, function: *mut sdca_function_data) -> c_int {
    const mult_fileset: c_int = 3;
    let mut fileset_name = [0 as c_char; SDCA_PROPERTY_LENGTH];
    let num_sets = fwnode_property_count_u32(function_node, cstr(b"mipi-sdca-file-set-id-list\0"));
    if num_sets == 0 || num_sets == -EINVAL { return 0; }
    if num_sets < 0 { return num_sets; }
    let filesets_list = kcalloc(num_sets as usize, size_of::<u32>(), GFP_KERNEL) as *mut u32;
    if filesets_list.is_null() { return -ENOMEM; }
    fwnode_property_read_u32_array(function_node, cstr(b"mipi-sdca-file-set-id-list\0"), filesets_list, num_sets);
    let sets = devm_kcalloc(dev, num_sets as usize, size_of::<sdca_fdl_set>(), GFP_KERNEL) as *mut sdca_fdl_set;
    if sets.is_null() { return -ENOMEM; }
    let mut i = 0;
    while i < num_sets {
        let set = sets.add(i as usize);
        snprintf(fileset_name.as_mut_ptr(), fileset_name.len(), cstr(b"mipi-sdca-file-set-id-0x%X\0"), *filesets_list.add(i as usize));
        let num_entries = fwnode_property_count_u32(function_node, fileset_name.as_mut_ptr());
        if num_entries <= 0 || num_entries % mult_fileset != 0 { return -EINVAL; }
        let files = devm_kcalloc(dev, (num_entries / mult_fileset) as usize, size_of::<sdca_fdl_file>(), GFP_KERNEL) as *mut sdca_fdl_file;
        if files.is_null() { return -ENOMEM; }
        let fileset_entries = kcalloc(num_entries as usize, size_of::<u32>(), GFP_KERNEL) as *mut u32;
        if fileset_entries.is_null() { return -ENOMEM; }
        fwnode_property_read_u32_array(function_node, fileset_name.as_mut_ptr(), fileset_entries, num_entries);
        let mut j = 0;
        let mut num_files = 0;
        while j < num_entries {
            let file = files.add(num_files as usize);
            (*file).vendor_id = *fileset_entries.add(j as usize); j += 1;
            (*file).file_id = *fileset_entries.add(j as usize); j += 1;
            (*file).fdl_offset = *fileset_entries.add(j as usize); j += 1;
            num_files += 1;
        }
        (*set).id = *filesets_list.add(i as usize);
        (*set).num_files = num_files;
        (*set).files = files;
        kfree(fileset_entries as *mut c_void);
        i += 1;
    }
    (*function).fdl_data.num_sets = num_sets;
    (*function).fdl_data.sets = sets;
    kfree(filesets_list as *mut c_void);
    0
}

unsafe fn find_sdca_hid(dev: *mut device, function_node: *mut fwnode_handle, function: *mut sdca_function_data) -> c_int {
    let mut num_desc = fwnode_property_count_u8(function_node, cstr(b"mipi-sdca-hid-descriptor\0"));
    if num_desc == 0 { return 0; }
    if num_desc < 0 { return num_desc; }
    if num_desc as usize > size_of_val(&(*function).hid.desc) { return -EINVAL; }
    fwnode_property_read_u8_array(function_node, cstr(b"mipi-sdca-hid-descriptor\0"), &mut (*function).hid.desc as *mut _ as *mut u8, num_desc);
    if (*function).hid.desc.bNumDescriptors == 0 { return 0; }
    num_desc = fwnode_property_count_u8(function_node, cstr(b"mipi-sdca-report-descriptor\0"));
    if num_desc <= 0 {
        if num_desc == 0 { return -EINVAL; }
        return num_desc;
    }
    (*function).hid.report_desc = devm_kzalloc(dev, num_desc as usize, GFP_KERNEL) as *mut u8;
    if (*function).hid.report_desc.is_null() { return -ENOMEM; }
    fwnode_property_read_u8_array(function_node, cstr(b"mipi-sdca-report-descriptor\0"), (*function).hid.report_desc, num_desc);
    0
}

/**
 * sdca_parse_function - parse ACPI DisCo for a Function
 * @dev: Pointer to device against which function data will be allocated.
 * @function: Pointer to the Function information, to be populated.
 *
 * Return: Returns 0 for success.
 */
#[no_mangle]
pub unsafe extern "C" fn sdca_parse_function(dev: *mut device, function: *mut sdca_function_data) -> c_int {
    let node = (*(*function).desc).node;
    let mut tmp = 0;
    let mut ret = fwnode_property_read_u32(node, cstr(b"mipi-sdca-function-busy-max-delay\0"), &mut tmp);
    if ret == 0 { (*function).busy_max_delay = tmp; }
    ret = fwnode_property_read_u32(node, cstr(b"mipi-sdca-function-reset-max-delay\0"), &mut tmp);
    if ret != 0 || tmp == 0 { (*function).reset_max_delay = 100000; } else { (*function).reset_max_delay = tmp; }
    ret = find_sdca_init_table(dev, node, function);
    if ret != 0 { return ret; }
    ret = find_sdca_entities(dev, node, function);
    if ret != 0 { return ret; }
    ret = find_sdca_connections(dev, node, function);
    if ret != 0 { return ret; }
    ret = find_sdca_clusters(dev, node, function);
    if ret < 0 { return ret; }
    ret = find_sdca_filesets(dev, node, function);
    if ret != 0 { return ret; }
    if (*(*function).desc).type_ == SDCA_FUNCTION_TYPE_HID {
        ret = find_sdca_hid(dev, node, function);
        if ret != 0 { return ret; }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn sdca_find_terminal_name(type_: sdca_terminal_type) -> *const c_char {
    match type_ as u32 {
        x if x == ext_const_u32("SDCA_TERM_TYPE_LINEIN_STEREO") => ext_name!(SDCA_TERM_TYPE_LINEIN_STEREO_NAME),
        x if x == ext_const_u32("SDCA_TERM_TYPE_LINEIN_FRONT_LR") => ext_name!(SDCA_TERM_TYPE_LINEIN_FRONT_LR_NAME),
        x if x == ext_const_u32("SDCA_TERM_TYPE_LINEIN_CENTER_LFE") => ext_name!(SDCA_TERM_TYPE_LINEIN_CENTER_LFE_NAME),
        x if x == ext_const_u32("SDCA_TERM_TYPE_LINEIN_SURROUND_LR") => ext_name!(SDCA_TERM_TYPE_LINEIN_SURROUND_LR_NAME),
        x if x == ext_const_u32("SDCA_TERM_TYPE_LINEIN_REAR_LR") => ext_name!(SDCA_TERM_TYPE_LINEIN_REAR_LR_NAME),
        x if x == ext_const_u32("SDCA_TERM_TYPE_LINEOUT_STEREO") => ext_name!(SDCA_TERM_TYPE_LINEOUT_STEREO_NAME),
        x if x == ext_const_u32("SDCA_TERM_TYPE_LINEOUT_FRONT_LR") => ext_name!(SDCA_TERM_TYPE_LINEOUT_FRONT_LR_NAME),
        x if x == ext_const_u32("SDCA_TERM_TYPE_LINEOUT_CENTER_LFE") => ext_name!(SDCA_TERM_TYPE_LINEOUT_CENTER_LFE_NAME),
        x if x == ext_const_u32("SDCA_TERM_TYPE_LINEOUT_SURROUND_LR") => ext_name!(SDCA_TERM_TYPE_LINEOUT_SURROUND_LR_NAME),
        x if x == ext_const_u32("SDCA_TERM_TYPE_LINEOUT_REAR_LR") => ext_name!(SDCA_TERM_TYPE_LINEOUT_REAR_LR_NAME),
        x if x == ext_const_u32("SDCA_TERM_TYPE_MIC_JACK") => ext_name!(SDCA_TERM_TYPE_MIC_JACK_NAME),
        x if x == ext_const_u32("SDCA_TERM_TYPE_STEREO_JACK") => ext_name!(SDCA_TERM_TYPE_STEREO_JACK_NAME),
        x if x == ext_const_u32("SDCA_TERM_TYPE_FRONT_LR_JACK") => ext_name!(SDCA_TERM_TYPE_FRONT_LR_JACK_NAME),
        x if x == ext_const_u32("SDCA_TERM_TYPE_CENTER_LFE_JACK") => ext_name!(SDCA_TERM_TYPE_CENTER_LFE_JACK_NAME),
        x if x == ext_const_u32("SDCA_TERM_TYPE_SURROUND_LR_JACK") => ext_name!(SDCA_TERM_TYPE_SURROUND_LR_JACK_NAME),
        x if x == ext_const_u32("SDCA_TERM_TYPE_REAR_LR_JACK") => ext_name!(SDCA_TERM_TYPE_REAR_LR_JACK_NAME),
        x if x == ext_const_u32("SDCA_TERM_TYPE_HEADPHONE_JACK") => ext_name!(SDCA_TERM_TYPE_HEADPHONE_JACK_NAME),
        x if x == ext_const_u32("SDCA_TERM_TYPE_HEADSET_JACK") => ext_name!(SDCA_TERM_TYPE_HEADSET_JACK_NAME),
        _ => null(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn sdca_selector_find_control(_dev: *mut device, entity: *mut sdca_entity, sel: c_int) -> *mut sdca_control {
    let mut i = 0;
    while i < (*entity).num_controls {
        let control = (*entity).controls.add(i as usize);
        if (*control).sel == sel as u32 { return control; }
        i += 1;
    }
    null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn sdca_control_find_range(_dev: *mut device, _entity: *mut sdca_entity, control: *mut sdca_control, cols: c_int, rows: c_int) -> *mut sdca_control_range {
    let range = &mut (*control).range as *mut sdca_control_range;
    if (cols != 0 && (*range).cols != cols as u32) || (rows != 0 && (*range).rows != rows as u32) || (*range).data.is_null() {
        return null_mut();
    }
    range
}

#[no_mangle]
pub unsafe extern "C" fn sdca_selector_find_range(dev: *mut device, entity: *mut sdca_entity, sel: c_int, cols: c_int, rows: c_int) -> *mut sdca_control_range {
    let control = sdca_selector_find_control(dev, entity, sel);
    if control.is_null() { return null_mut(); }
    sdca_control_find_range(dev, entity, control, cols, rows)
}

#[no_mangle]
pub unsafe extern "C" fn sdca_id_find_cluster(_dev: *mut device, function: *mut sdca_function_data, id: c_int) -> *mut sdca_cluster {
    let mut i = 0;
    while i < (*function).num_clusters {
        let cluster = (*function).clusters.add(i as usize);
        if (*cluster).id == id as u32 { return cluster; }
        i += 1;
    }
    null_mut()
}

fn size_of_val<T>(_: &T) -> usize {
    size_of::<T>()
}

/*
 * EXPORT_SYMBOL_NS(sdca_lookup_functions, "SND_SOC_SDCA");
 * EXPORT_SYMBOL_NS(sdca_find_entity_by_label, "SND_SOC_SDCA");
 * EXPORT_SYMBOL_NS(sdca_parse_function, "SND_SOC_SDCA");
 * EXPORT_SYMBOL_NS(sdca_find_terminal_name, "SND_SOC_SDCA");
 * EXPORT_SYMBOL_NS(sdca_selector_find_control, "SND_SOC_SDCA");
 * EXPORT_SYMBOL_NS(sdca_control_find_range, "SND_SOC_SDCA");
 * EXPORT_SYMBOL_NS(sdca_selector_find_range, "SND_SOC_SDCA");
 * EXPORT_SYMBOL_NS(sdca_id_find_cluster, "SND_SOC_SDCA");
 * MODULE_LICENSE("Dual BSD/GPL");
 * MODULE_DESCRIPTION("SDCA library");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
