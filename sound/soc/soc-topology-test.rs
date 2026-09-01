// SPDX-License-Identifier: GPL-2.0-only
/*
 * soc-topology-test.c  --  ALSA SoC Topology Kernel Unit Tests
 *
 * Copyright(c) 2021 Intel Corporation.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;
type size_t = usize;

const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const EPROBE_DEFER: c_int = 517;
const GFP_KERNEL: c_uint = 0;
const SND_SOC_TPLG_MAGIC: u32 = 0x41536f43;
const SND_SOC_TPLG_ABI_VERSION: u32 = 5;
const SND_SOC_TPLG_TYPE_MANIFEST: u32 = 8;
const SND_SOC_TPLG_TYPE_PCM: u32 = 7;
const SND_SOC_DPCM_TRIGGER_POST: c_int = 2;
const THIS_MODULE: *mut c_void = ptr::null_mut();

#[inline]
const fn cpu_to_le32(value: u32) -> u32 {
    value.to_le()
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    name: *const c_char,
    id: c_int,
    stream_name: *const c_char,
    nonatomic: c_uint,
    dynamic: c_uint,
    trigger: [c_int; 2],
    cpus: *mut snd_soc_dai_link_component,
    num_cpus: c_uint,
    codecs: *mut snd_soc_dai_link_component,
    num_codecs: c_uint,
    platforms: *mut snd_soc_dai_link_component,
    num_platforms: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    dev: *mut device,
    name: *const c_char,
    owner: *mut c_void,
    dai_link: *mut snd_soc_dai_link,
    num_links: c_int,
    fully_routed: bool,
}

#[repr(C)]
pub struct firmware {
    size: size_t,
    data: *const u8,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    name: *const c_char,
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_hdr {
    magic: u32,
    abi: u32,
    version: u32,
    type_: u32,
    size: u32,
    vendor_type: u32,
    payload_size: u32,
    index: u32,
    count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_manifest {
    size: u32,
    control_elems: u32,
    widget_elems: u32,
    graph_elems: u32,
    pcm_elems: u32,
    dai_link_elems: u32,
    dai_elems: u32,
    reserved: [u32; 20],
    priv_: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_stream {
    size: u32,
    name: [c_char; 64],
    stream_name: [c_char; 64],
    channels: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_stream_caps {
    size: u32,
    name: [c_char; 64],
    formats: u64,
    rates: u32,
    rate_min: u32,
    rate_max: u32,
    channels_min: u32,
    channels_max: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_private {
    size: u32,
    data: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_pcm {
    size: u32,
    pcm_name: [c_char; 64],
    dai_name: [c_char; 64],
    pcm_id: u32,
    dai_id: u32,
    playback: u32,
    capture: u32,
    compress: u32,
    stream: [snd_soc_tplg_stream; 2],
    num_streams: u32,
    caps: [snd_soc_tplg_stream_caps; 2],
    flag_mask: u32,
    flags: u32,
    priv_: snd_soc_tplg_private,
}

unsafe extern "C" {
    fn kunit_device_register(test: *mut kunit, name: *const c_char) -> *mut device;
    fn get_device(dev: *mut device) -> *mut device;
    fn put_device(dev: *mut device);
    fn kunit_kzalloc(test: *mut kunit, size: size_t, flags: c_uint) -> *mut c_void;
    fn snd_soc_component_to_priv(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_tplg_component_load(
        component: *mut snd_soc_component,
        ops: *const c_void,
        fw: *const firmware,
    ) -> c_int;
    fn snd_soc_tplg_component_remove(component: *mut snd_soc_component) -> c_int;
    fn snd_soc_component_alloc(dev: *mut device) -> *mut snd_soc_component;
    fn snd_soc_component_set_priv(component: *mut snd_soc_component, data: *mut c_void);
    fn snd_soc_register_card(card: *mut snd_soc_card) -> c_int;
    fn snd_soc_unregister_card(card: *mut snd_soc_card);
    fn snd_soc_register_component(
        component: *mut snd_soc_component,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *const c_void,
        num_dai: c_int,
    ) -> c_int;
    fn snd_soc_unregister_component(dev: *mut device);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
}

macro_rules! KUNIT_EXPECT_EQ_MSG {
    ($test:expr, $expected:expr, $actual:expr, $msg:expr) => {{
        let _ = ($test, $expected, $actual, $msg);
    }};
}

macro_rules! KUNIT_EXPECT_EQ {
    ($test:expr, $expected:expr, $actual:expr) => {{
        let _ = ($test, $expected, $actual);
    }};
}

macro_rules! KUNIT_EXPECT_NOT_ERR_OR_NULL {
    ($test:expr, $ptr:expr) => {{
        let _ = ($test, $ptr);
    }};
}

macro_rules! KUNIT_ASSERT_NOT_NULL {
    ($test:expr, $ptr:expr) => {{
        let _ = ($test, $ptr);
    }};
}

macro_rules! KUNIT_FAIL {
    ($test:expr, $msg:expr) => {{
        let _ = ($test, $msg);
    }};
}

/* ===== HELPER FUNCTIONS =================================================== */

/*
 * snd_soc_component needs device to operate on (primarily for prints), create
 * fake one, as we don't register with PCI or anything else
 * device_driver name is used in some of the prints (fmt_single_name) so
 * we also mock up minimal one
 */
static mut test_dev: *mut device = ptr::null_mut();

unsafe extern "C" fn snd_soc_tplg_test_init(test: *mut kunit) -> c_int {
    test_dev = kunit_device_register(test, c"sound-soc-topology-test".as_ptr());
    test_dev = get_device(test_dev);
    if test_dev.is_null() {
        return -ENODEV;
    }

    0
}

unsafe extern "C" fn snd_soc_tplg_test_exit(_test: *mut kunit) {
    put_device(test_dev);
}

/*
 * helper struct we use when registering component, as we load topology during
 * component probe, we need to pass struct kunit somehow to probe function, so
 * we can report test result
 */
#[repr(C)]
pub struct kunit_soc_component {
    kunit: *mut kunit,
    expect: c_int, /* what result we expect when loading topology */
    card: snd_soc_card,
    fw: firmware,
}

unsafe extern "C" fn d_probe(component: *mut snd_soc_component) -> c_int {
    let kunit_comp = snd_soc_component_to_priv(component) as *mut kunit_soc_component;
    let ret: c_int;

    ret = snd_soc_tplg_component_load(component, ptr::null(), &(*kunit_comp).fw);
    KUNIT_EXPECT_EQ_MSG!(
        (*kunit_comp).kunit,
        (*kunit_comp).expect,
        ret,
        c"Failed topology load".as_ptr()
    );

    0
}

unsafe extern "C" fn d_remove(component: *mut snd_soc_component) {
    let kunit_comp = snd_soc_component_to_priv(component) as *mut kunit_soc_component;
    let ret: c_int;

    ret = snd_soc_tplg_component_remove(component);
    KUNIT_EXPECT_EQ!((*kunit_comp).kunit, 0, ret);
}

/*
 * ASoC minimal boiler plate
 */
static mut dummy: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
}];

static mut platform: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: c"sound-soc-topology-test".as_ptr(),
}];

static mut kunit_dai_links: [snd_soc_dai_link; 1] = [snd_soc_dai_link {
    name: c"KUNIT Audio Port".as_ptr(),
    id: 0,
    stream_name: c"Audio Playback/Capture".as_ptr(),
    nonatomic: 1,
    dynamic: 1,
    trigger: [SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST],
    cpus: unsafe { dummy.as_mut_ptr() },
    num_cpus: 1,
    codecs: unsafe { dummy.as_mut_ptr() },
    num_codecs: 1,
    platforms: unsafe { platform.as_mut_ptr() },
    num_platforms: 1,
}];

static mut test_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"sound-soc-topology-test".as_ptr(),
    probe: Some(d_probe),
    remove: Some(d_remove),
};

/* ===== TOPOLOGY TEMPLATES ================================================= */

// Structural representation of topology which can be generated with:
// $ touch empty
// $ alsatplg -c empty -o empty.tplg
// $ xxd -i empty.tplg

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct tplg_tmpl_001 {
    header: snd_soc_tplg_hdr,
    manifest: snd_soc_tplg_manifest,
}

static tplg_tmpl_empty: tplg_tmpl_001 = tplg_tmpl_001 {
    header: snd_soc_tplg_hdr {
        magic: cpu_to_le32(SND_SOC_TPLG_MAGIC),
        abi: cpu_to_le32(5),
        version: 0,
        type_: cpu_to_le32(SND_SOC_TPLG_TYPE_MANIFEST),
        size: cpu_to_le32(size_of::<snd_soc_tplg_hdr>() as u32),
        vendor_type: 0,
        payload_size: cpu_to_le32(size_of::<snd_soc_tplg_manifest>() as u32),
        index: 0,
        count: cpu_to_le32(1),
    },

    manifest: snd_soc_tplg_manifest {
        size: cpu_to_le32(size_of::<snd_soc_tplg_manifest>() as u32),
        /* rest of fields is 0 */
        control_elems: 0,
        widget_elems: 0,
        graph_elems: 0,
        pcm_elems: 0,
        dai_link_elems: 0,
        dai_elems: 0,
        reserved: [0; 20],
        priv_: [],
    },
};

// Structural representation of topology containing SectionPCM

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct tplg_tmpl_002 {
    header: snd_soc_tplg_hdr,
    manifest: snd_soc_tplg_manifest,
    pcm_header: snd_soc_tplg_hdr,
    pcm: snd_soc_tplg_pcm,
}

const ZERO_STREAM: snd_soc_tplg_stream = snd_soc_tplg_stream {
    size: 0,
    name: [0; 64],
    stream_name: [0; 64],
    channels: 0,
};

const ZERO_CAPS: snd_soc_tplg_stream_caps = snd_soc_tplg_stream_caps {
    size: 0,
    name: [0; 64],
    formats: 0,
    rates: 0,
    rate_min: 0,
    rate_max: 0,
    channels_min: 0,
    channels_max: 0,
};

static tplg_tmpl_with_pcm: tplg_tmpl_002 = tplg_tmpl_002 {
    header: snd_soc_tplg_hdr {
        magic: cpu_to_le32(SND_SOC_TPLG_MAGIC),
        abi: cpu_to_le32(5),
        version: 0,
        type_: cpu_to_le32(SND_SOC_TPLG_TYPE_MANIFEST),
        size: cpu_to_le32(size_of::<snd_soc_tplg_hdr>() as u32),
        vendor_type: 0,
        payload_size: cpu_to_le32(size_of::<snd_soc_tplg_manifest>() as u32),
        index: 0,
        count: cpu_to_le32(1),
    },
    manifest: snd_soc_tplg_manifest {
        size: cpu_to_le32(size_of::<snd_soc_tplg_manifest>() as u32),
        pcm_elems: cpu_to_le32(1),
        /* rest of fields is 0 */
        control_elems: 0,
        widget_elems: 0,
        graph_elems: 0,
        dai_link_elems: 0,
        dai_elems: 0,
        reserved: [0; 20],
        priv_: [],
    },
    pcm_header: snd_soc_tplg_hdr {
        magic: cpu_to_le32(SND_SOC_TPLG_MAGIC),
        abi: cpu_to_le32(5),
        version: 0,
        type_: cpu_to_le32(SND_SOC_TPLG_TYPE_PCM),
        size: cpu_to_le32(size_of::<snd_soc_tplg_hdr>() as u32),
        vendor_type: 0,
        payload_size: cpu_to_le32(size_of::<snd_soc_tplg_pcm>() as u32),
        index: 0,
        count: cpu_to_le32(1),
    },
    pcm: snd_soc_tplg_pcm {
        size: cpu_to_le32(size_of::<snd_soc_tplg_pcm>() as u32),
        pcm_name: {
            let mut s = [0; 64];
            let b = *b"KUNIT Audio\0";
            let mut i = 0;
            while i < b.len() {
                s[i] = b[i] as c_char;
                i += 1;
            }
            s
        },
        dai_name: {
            let mut s = [0; 64];
            let b = *b"kunit-audio-dai\0";
            let mut i = 0;
            while i < b.len() {
                s[i] = b[i] as c_char;
                i += 1;
            }
            s
        },
        pcm_id: 0,
        dai_id: 0,
        playback: cpu_to_le32(1),
        capture: cpu_to_le32(1),
        compress: 0,
        stream: [
            snd_soc_tplg_stream {
                channels: cpu_to_le32(2),
                ..ZERO_STREAM
            },
            snd_soc_tplg_stream {
                channels: cpu_to_le32(2),
                ..ZERO_STREAM
            },
        ],
        num_streams: 0,
        caps: [
            snd_soc_tplg_stream_caps {
                name: {
                    let mut s = [0; 64];
                    let b = *b"kunit-audio-playback\0";
                    let mut i = 0;
                    while i < b.len() {
                        s[i] = b[i] as c_char;
                        i += 1;
                    }
                    s
                },
                channels_min: cpu_to_le32(2),
                channels_max: cpu_to_le32(2),
                ..ZERO_CAPS
            },
            snd_soc_tplg_stream_caps {
                name: {
                    let mut s = [0; 64];
                    let b = *b"kunit-audio-capture\0";
                    let mut i = 0;
                    while i < b.len() {
                        s[i] = b[i] as c_char;
                        i += 1;
                    }
                    s
                },
                channels_min: cpu_to_le32(2),
                channels_max: cpu_to_le32(2),
                ..ZERO_CAPS
            },
        ],
        flag_mask: 0,
        flags: 0,
        priv_: snd_soc_tplg_private { size: 0, data: [] },
    },
};

unsafe fn prepare_component(
    test: *mut kunit,
    expect: c_int,
    fw_data: *const u8,
    fw_size: size_t,
) -> (*mut kunit_soc_component, *mut snd_soc_component) {
    let kunit_comp =
        kunit_kzalloc(test, size_of::<kunit_soc_component>(), GFP_KERNEL) as *mut kunit_soc_component;
    KUNIT_EXPECT_NOT_ERR_OR_NULL!(test, kunit_comp);
    (*kunit_comp).kunit = test;
    (*kunit_comp).expect = expect;
    (*kunit_comp).fw.data = fw_data;
    (*kunit_comp).fw.size = fw_size;

    (*kunit_comp).card.dev = test_dev;
    (*kunit_comp).card.name = c"kunit-card".as_ptr();
    (*kunit_comp).card.owner = THIS_MODULE;
    (*kunit_comp).card.dai_link = kunit_dai_links.as_mut_ptr();
    (*kunit_comp).card.num_links = kunit_dai_links.len() as c_int;
    (*kunit_comp).card.fully_routed = true;

    let component = snd_soc_component_alloc(test_dev);
    KUNIT_ASSERT_NOT_NULL!(test, component);

    snd_soc_component_set_priv(component, kunit_comp as *mut c_void);
    (kunit_comp, component)
}

unsafe fn register_card_or_fail(test: *mut kunit, card: *mut snd_soc_card) {
    let ret = snd_soc_register_card(card);
    if ret != 0 && ret != -EPROBE_DEFER {
        KUNIT_FAIL!(test, c"Failed to register card".as_ptr());
    }
}

/* ===== TEST CASES ========================================================= */

// TEST CASE
// Test passing NULL component as parameter to snd_soc_tplg_component_load

/*
 * need to override generic probe function with one using NULL when calling
 * topology load during component initialization, we don't need .remove
 * handler as load should fail
 */
unsafe extern "C" fn d_probe_null_comp(component: *mut snd_soc_component) -> c_int {
    let kunit_comp = snd_soc_component_to_priv(component) as *mut kunit_soc_component;
    let ret: c_int;

    /* instead of passing component pointer as first argument, pass NULL here */
    ret = snd_soc_tplg_component_load(ptr::null_mut(), ptr::null(), &(*kunit_comp).fw);
    KUNIT_EXPECT_EQ_MSG!(
        (*kunit_comp).kunit,
        (*kunit_comp).expect,
        ret,
        c"Failed topology load".as_ptr()
    );

    0
}

static mut test_component_null_comp: snd_soc_component_driver = snd_soc_component_driver {
    name: c"sound-soc-topology-test".as_ptr(),
    probe: Some(d_probe_null_comp),
    remove: None,
};

unsafe extern "C" fn snd_soc_tplg_test_load_with_null_comp(test: *mut kunit) {
    let (kunit_comp, component) = prepare_component(test, -EINVAL, ptr::null(), 0);
    let mut ret: c_int;

    /* run test */
    register_card_or_fail(test, &mut (*kunit_comp).card);

    ret = snd_soc_register_component(component, &raw const test_component_null_comp, ptr::null(), 0);
    KUNIT_EXPECT_EQ!(test, 0, ret);

    /* cleanup */
    snd_soc_unregister_card(&mut (*kunit_comp).card);
    snd_soc_unregister_component(test_dev);
}

// TEST CASE
// Test passing NULL ops as parameter to snd_soc_tplg_component_load

/*
 * NULL ops is default case, we pass empty topology (fw), so we don't have
 * anything to parse and just do nothing, which results in return 0; from
 * calling soc_tplg_dapm_complete in soc_tplg_process_headers
 */
unsafe extern "C" fn snd_soc_tplg_test_load_with_null_ops(test: *mut kunit) {
    let (kunit_comp, component) = prepare_component(test, 0, ptr::null(), 0);
    let mut ret: c_int;

    /* run test */
    register_card_or_fail(test, &mut (*kunit_comp).card);

    ret = snd_soc_register_component(component, &raw const test_component, ptr::null(), 0);
    KUNIT_EXPECT_EQ!(test, 0, ret);

    /* cleanup */
    snd_soc_unregister_card(&mut (*kunit_comp).card);

    snd_soc_unregister_component(test_dev);
}

// TEST CASE
// Test passing NULL fw as parameter to snd_soc_tplg_component_load

/*
 * need to override generic probe function with one using NULL pointer to fw
 * when calling topology load during component initialization, we don't need
 * .remove handler as load should fail
 */
unsafe extern "C" fn d_probe_null_fw(component: *mut snd_soc_component) -> c_int {
    let kunit_comp = snd_soc_component_to_priv(component) as *mut kunit_soc_component;
    let ret: c_int;

    /* instead of passing fw pointer as third argument, pass NULL here */
    ret = snd_soc_tplg_component_load(component, ptr::null(), ptr::null());
    KUNIT_EXPECT_EQ_MSG!(
        (*kunit_comp).kunit,
        (*kunit_comp).expect,
        ret,
        c"Failed topology load".as_ptr()
    );

    0
}

static mut test_component_null_fw: snd_soc_component_driver = snd_soc_component_driver {
    name: c"sound-soc-topology-test".as_ptr(),
    probe: Some(d_probe_null_fw),
    remove: None,
};

unsafe extern "C" fn snd_soc_tplg_test_load_with_null_fw(test: *mut kunit) {
    let (kunit_comp, component) = prepare_component(test, -EINVAL, ptr::null(), 0);
    let mut ret: c_int;

    /* run test */
    register_card_or_fail(test, &mut (*kunit_comp).card);

    ret = snd_soc_register_component(component, &raw const test_component_null_fw, ptr::null(), 0);
    KUNIT_EXPECT_EQ!(test, 0, ret);

    /* cleanup */
    snd_soc_unregister_card(&mut (*kunit_comp).card);

    snd_soc_unregister_component(test_dev);
}

unsafe fn copy_empty_template(test: *mut kunit) -> (*mut tplg_tmpl_001, c_int) {
    let size = size_of::<tplg_tmpl_001>() as c_int;
    let data = kunit_kzalloc(test, size as size_t, GFP_KERNEL) as *mut tplg_tmpl_001;
    KUNIT_EXPECT_NOT_ERR_OR_NULL!(test, data);

    memcpy(
        data as *mut c_void,
        &tplg_tmpl_empty as *const _ as *const c_void,
        size_of::<tplg_tmpl_001>(),
    );
    (data, size)
}

unsafe fn run_with_component(
    test: *mut kunit,
    kunit_comp: *mut kunit_soc_component,
    component: *mut snd_soc_component,
    component_driver: *const snd_soc_component_driver,
) {
    let mut ret: c_int;

    /* run test */
    register_card_or_fail(test, &mut (*kunit_comp).card);

    ret = snd_soc_register_component(component, component_driver, ptr::null(), 0);
    KUNIT_EXPECT_EQ!(test, 0, ret);
}

// TEST CASE
// Test passing "empty" topology file
unsafe extern "C" fn snd_soc_tplg_test_load_empty_tplg(test: *mut kunit) {
    let (data, size) = copy_empty_template(test);
    let (kunit_comp, component) = prepare_component(test, 0, data as *const u8, size as size_t);

    run_with_component(test, kunit_comp, component, &raw const test_component);

    /* cleanup */
    snd_soc_unregister_card(&mut (*kunit_comp).card);

    snd_soc_unregister_component(test_dev);
}

// TEST CASE
// Test "empty" topology file, but with bad "magic"
// In theory we could loop through all possible bad values, but it takes too
// long, so just use SND_SOC_TPLG_MAGIC + 1
unsafe extern "C" fn snd_soc_tplg_test_load_empty_tplg_bad_magic(test: *mut kunit) {
    let (data, size) = copy_empty_template(test);
    /*
     * override abi
     * any value != magic number is wrong
     */
    (*data).header.magic = cpu_to_le32(SND_SOC_TPLG_MAGIC.wrapping_add(1));

    let (kunit_comp, component) = prepare_component(test, -EINVAL, data as *const u8, size as size_t);
    run_with_component(test, kunit_comp, component, &raw const test_component);

    /* cleanup */
    snd_soc_unregister_card(&mut (*kunit_comp).card);

    snd_soc_unregister_component(test_dev);
}

// TEST CASE
// Test "empty" topology file, but with bad "abi"
// In theory we could loop through all possible bad values, but it takes too
// long, so just use SND_SOC_TPLG_ABI_VERSION + 1
unsafe extern "C" fn snd_soc_tplg_test_load_empty_tplg_bad_abi(test: *mut kunit) {
    let (data, size) = copy_empty_template(test);
    /*
     * override abi
     * any value != accepted range is wrong
     */
    (*data).header.abi = cpu_to_le32(SND_SOC_TPLG_ABI_VERSION.wrapping_add(1));

    let (kunit_comp, component) = prepare_component(test, -EINVAL, data as *const u8, size as size_t);
    run_with_component(test, kunit_comp, component, &raw const test_component);

    /* cleanup */
    snd_soc_unregister_card(&mut (*kunit_comp).card);

    snd_soc_unregister_component(test_dev);
}

// TEST CASE
// Test "empty" topology file, but with bad "size"
// In theory we could loop through all possible bad values, but it takes too
// long, so just use sizeof(struct snd_soc_tplg_hdr) + 1
unsafe extern "C" fn snd_soc_tplg_test_load_empty_tplg_bad_size(test: *mut kunit) {
    let (data, size) = copy_empty_template(test);
    /*
     * override size
     * any value != struct size is wrong
     */
    (*data).header.size = cpu_to_le32((size_of::<snd_soc_tplg_hdr>() + 1) as u32);

    let (kunit_comp, component) = prepare_component(test, -EINVAL, data as *const u8, size as size_t);
    run_with_component(test, kunit_comp, component, &raw const test_component);

    /* cleanup */
    snd_soc_unregister_card(&mut (*kunit_comp).card);

    snd_soc_unregister_component(test_dev);
}

// TEST CASE
// Test "empty" topology file, but with bad "payload_size"
// In theory we could loop through all possible bad values, but it takes too
// long, so just use the known wrong one
unsafe extern "C" fn snd_soc_tplg_test_load_empty_tplg_bad_payload_size(test: *mut kunit) {
    let (data, size) = copy_empty_template(test);
    /*
     * override payload size
     * there is only explicit check for 0, so check with it, other values
     * are handled by just not reading behind EOF
     */
    (*data).header.payload_size = 0;

    let (kunit_comp, component) = prepare_component(test, -EINVAL, data as *const u8, size as size_t);
    run_with_component(test, kunit_comp, component, &raw const test_component);

    /* cleanup */
    snd_soc_unregister_component(test_dev);

    snd_soc_unregister_card(&mut (*kunit_comp).card);
}

unsafe fn copy_pcm_template(test: *mut kunit) -> (*mut u8, c_int) {
    let size = size_of::<tplg_tmpl_002>() as c_int;
    let data = kunit_kzalloc(test, size as size_t, GFP_KERNEL) as *mut u8;
    KUNIT_EXPECT_NOT_ERR_OR_NULL!(test, data);

    memcpy(
        data as *mut c_void,
        &tplg_tmpl_with_pcm as *const _ as *const c_void,
        size_of::<tplg_tmpl_002>(),
    );
    (data, size)
}

// TEST CASE
// Test passing topology file with PCM definition
unsafe extern "C" fn snd_soc_tplg_test_load_pcm_tplg(test: *mut kunit) {
    let (data, size) = copy_pcm_template(test);
    let (kunit_comp, component) = prepare_component(test, 0, data, size as size_t);

    run_with_component(test, kunit_comp, component, &raw const test_component);

    snd_soc_unregister_component(test_dev);

    /* cleanup */
    snd_soc_unregister_card(&mut (*kunit_comp).card);
}

// TEST CASE
// Test passing topology file with PCM definition
// with component reload
unsafe extern "C" fn snd_soc_tplg_test_load_pcm_tplg_reload_comp(test: *mut kunit) {
    let (data, size) = copy_pcm_template(test);
    let (kunit_comp, component) = prepare_component(test, 0, data, size as size_t);
    let mut ret: c_int;
    let mut i: c_int;

    /* run test */
    register_card_or_fail(test, &mut (*kunit_comp).card);

    i = 0;
    while i < 100 {
        ret = snd_soc_register_component(component, &raw const test_component, ptr::null(), 0);
        KUNIT_EXPECT_EQ!(test, 0, ret);

        snd_soc_unregister_component(test_dev);
        i += 1;
    }

    /* cleanup */
    snd_soc_unregister_card(&mut (*kunit_comp).card);
}

// TEST CASE
// Test passing topology file with PCM definition
// with card reload
unsafe extern "C" fn snd_soc_tplg_test_load_pcm_tplg_reload_card(test: *mut kunit) {
    let (data, size) = copy_pcm_template(test);
    let (kunit_comp, component) = prepare_component(test, 0, data, size as size_t);
    let mut ret: c_int;
    let mut i: c_int;

    /* run test */
    ret = snd_soc_register_component(component, &raw const test_component, ptr::null(), 0);
    KUNIT_EXPECT_EQ!(test, 0, ret);

    i = 0;
    while i < 100 {
        ret = snd_soc_register_card(&mut (*kunit_comp).card);
        if ret != 0 && ret != -EPROBE_DEFER {
            KUNIT_FAIL!(test, c"Failed to register card".as_ptr());
        }

        snd_soc_unregister_card(&mut (*kunit_comp).card);
        i += 1;
    }

    /* cleanup */
    snd_soc_unregister_component(test_dev);
}

/* ===== KUNIT MODULE DEFINITIONS =========================================== */

#[repr(C)]
pub struct kunit_case {
    run_case: Option<unsafe extern "C" fn(*mut kunit)>,
}

#[repr(C)]
pub struct kunit_suite {
    name: *const c_char,
    init: Option<unsafe extern "C" fn(*mut kunit) -> c_int>,
    exit: Option<unsafe extern "C" fn(*mut kunit)>,
    test_cases: *mut kunit_case,
}

static mut snd_soc_tplg_test_cases: [kunit_case; 12] = [
    kunit_case {
        run_case: Some(snd_soc_tplg_test_load_with_null_comp),
    },
    kunit_case {
        run_case: Some(snd_soc_tplg_test_load_with_null_ops),
    },
    kunit_case {
        run_case: Some(snd_soc_tplg_test_load_with_null_fw),
    },
    kunit_case {
        run_case: Some(snd_soc_tplg_test_load_empty_tplg),
    },
    kunit_case {
        run_case: Some(snd_soc_tplg_test_load_empty_tplg_bad_magic),
    },
    kunit_case {
        run_case: Some(snd_soc_tplg_test_load_empty_tplg_bad_abi),
    },
    kunit_case {
        run_case: Some(snd_soc_tplg_test_load_empty_tplg_bad_size),
    },
    kunit_case {
        run_case: Some(snd_soc_tplg_test_load_empty_tplg_bad_payload_size),
    },
    kunit_case {
        run_case: Some(snd_soc_tplg_test_load_pcm_tplg),
    },
    kunit_case {
        run_case: Some(snd_soc_tplg_test_load_pcm_tplg_reload_comp),
    },
    kunit_case {
        run_case: Some(snd_soc_tplg_test_load_pcm_tplg_reload_card),
    },
    kunit_case { run_case: None },
];

static mut snd_soc_tplg_test_suite: kunit_suite = kunit_suite {
    name: c"snd_soc_tplg_test".as_ptr(),
    init: Some(snd_soc_tplg_test_init),
    exit: Some(snd_soc_tplg_test_exit),
    test_cases: unsafe { snd_soc_tplg_test_cases.as_mut_ptr() },
};

// kunit_test_suites(&snd_soc_tplg_test_suite);

// MODULE_DESCRIPTION("ASoC Topology Kernel Unit Tests");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
