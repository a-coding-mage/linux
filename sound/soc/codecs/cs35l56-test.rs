// SPDX-License-Identifier: GPL-2.0-only
//
// KUnit test for the Cirrus Logic cs35l56 driver.
//
// Copyright (C) 2026 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u32 = u32;
type s32 = i32;
type efi_status_t = u64;
type efi_char16_t = u16;
type efi_guid_t = c_void;

const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const EFI_SUCCESS: efi_status_t = 0;
const EFI_BUFFER_TOO_SMALL: efi_status_t = 5;
const PCI_VENDOR_ID_DELL: c_int = 0x1028;
const KUNIT_PARAM_DESC_SIZE: usize = 128;

type c_uint = u32;

#[repr(C)]
pub struct kunit {
    pub priv_: *mut c_void,
    pub param_value: *const c_void,
}

#[repr(C)]
pub struct faux_device {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct property_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct software_node {
    pub name: *const c_char,
    pub properties: *const property_entry,
    pub parent: *const software_node,
}

#[repr(C)]
pub struct snd_soc_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct cs_dsp {
    pub system_name: *const c_char,
    pub fwf_suffix: *const c_char,
}

#[repr(C)]
pub struct cs35l56_base {
    pub dev: *mut device,
    pub type_: u8,
    pub rev: u8,
    pub onchip_spkid_gpios: [s32; 4],
    pub onchip_spkid_pulls: [s32; 4],
    pub num_onchip_spkid_gpios: c_int,
    pub num_onchip_spkid_pulls: c_int,
}

#[repr(C)]
pub struct cs35l56_private {
    pub base: cs35l56_base,
    pub component: *mut snd_soc_component,
    pub dsp: cs_dsp,
    pub fallback_fw_suffix: *const c_char,
    pub speaker_id: c_int,
    pub sdw_link_num: c_int,
    pub sdw_unique_id: c_int,
    pub sdw_peripheral: *mut c_void,
}

#[repr(C)]
pub struct cs_amp_test_hooks_type {
    pub get_efi_variable: Option<
        unsafe extern "C" fn(
            *mut efi_char16_t,
            *mut efi_guid_t,
            *mut u32,
            *mut c_ulong,
            *mut c_void,
        ) -> efi_status_t,
    >,
}

#[repr(C)]
struct cs35l56_test_priv {
    amp_dev: *mut faux_device,
    cs35l56_priv: *mut cs35l56_private,

    ssidexv2: *const c_char,

    read_onchip_spkid_called: bool,
    configure_onchip_spkid_pads_called: bool,
}

#[repr(C)]
struct cs35l56_test_param {
    type_: u8,
    rev: u8,

    spkid_gpios: [s32; 4],
    spkid_pulls: [s32; 4],
}

#[repr(C)]
pub struct kunit_case {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kunit_suite {
    pub name: *const c_char,
    pub init: Option<unsafe extern "C" fn(*mut kunit) -> c_int>,
    pub test_cases: *mut kunit_case,
}

unsafe extern "C" {
    static mut cs_amp_test_hooks: *mut cs_amp_test_hooks_type;

    fn ERR_PTR(error: c_long) -> *const c_char;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;

    fn kunit_get_current_test() -> *mut kunit;
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: c_uint) -> *mut c_void;
    fn kunit_add_action_or_reset(
        test: *mut kunit,
        action: unsafe extern "C" fn(*mut c_void),
        ctx: *mut c_void,
    ) -> c_int;
    fn kunit_activate_static_stub(test: *mut kunit, replacement: *const c_void, target: *const c_void);

    fn faux_device_create(name: *const c_char, a: *mut c_void, b: *mut c_void) -> *mut faux_device;
    fn faux_device_destroy(faux_dev: *mut faux_device);
    fn software_node_register_node_group(group: *const *const software_node) -> c_int;
    fn software_node_unregister_node_group(group: *const *const software_node);
    fn software_node_register(node: *const software_node) -> c_int;
    fn software_node_unregister(node: *const software_node);
    fn device_add_software_node(dev: *mut device, node: *const software_node) -> c_int;
    fn device_remove_software_node(dev: *mut device);

    fn snd_soc_card_set_pci_ssid(card: *mut snd_soc_card, ssid_vendor: c_int, ssid_device: c_int);
    fn snd_soc_component_set_drvdata(component: *mut snd_soc_component, data: *mut c_void);
    fn cs35l56_get_firmware_uid(cs35l56: *mut cs35l56_private) -> c_int;
    fn cs35l56_set_fw_name(component: *mut snd_soc_component) -> c_int;
    fn cs35l56_set_fw_suffix(cs35l56: *mut cs35l56_private) -> c_int;
    fn cs35l56_process_xu_properties(cs35l56: *mut cs35l56_private) -> c_int;
    fn cs35l56_configure_onchip_spkid_pads(cs35l56_base: *mut cs35l56_base) -> c_int;
    fn cs35l56_read_onchip_spkid(cs35l56_base: *mut cs35l56_base) -> c_int;
    fn cs_amp_devm_get_vendor_specific_variant_id(
        dev: *mut device,
        ssid_vendor: c_int,
        ssid_device: c_int,
    ) -> *const c_char;

    fn KUNIT_EXPECT_EQ(test: *mut kunit, left: c_int, right: c_int);
    fn KUNIT_ASSERT_EQ(test: *mut kunit, left: c_int, right: c_int);
    fn KUNIT_ASSERT_LE(test: *mut kunit, left: c_int, right: usize);
    fn KUNIT_EXPECT_STREQ(test: *mut kunit, left: *const c_char, right: *const c_char);
    fn KUNIT_EXPECT_NULL(test: *mut kunit, ptr: *const c_void);
    fn KUNIT_EXPECT_TRUE(test: *mut kunit, condition: bool);
    fn KUNIT_ASSERT_NOT_NULL(test: *mut kunit, ptr: *const c_void);
    fn KUNIT_EXPECT_EQ_MSG(test: *mut kunit, left: c_int, right: c_int, fmt: *const c_char, ...);
}

type c_long = i64;

const fn software_node_const(
    name: *const c_char,
    properties: *const property_entry,
    parent: *const software_node,
) -> software_node {
    software_node { name, properties, parent }
}

static cs35l56_test_dev_sw_node: software_node =
    software_node_const(c"SWD1".as_ptr(), ptr::null(), ptr::null());

static cs35l56_test_af01_sw_node: software_node =
    software_node_const(c"AF01".as_ptr(), ptr::null(), &cs35l56_test_dev_sw_node);

static cs35l56_test_dev_and_af01_node_group: [*const software_node; 3] = [
    &cs35l56_test_dev_sw_node,
    &cs35l56_test_af01_sw_node,
    ptr::null(),
];

unsafe extern "C" fn faux_device_destroy_wrapper(ctx: *mut c_void) {
    unsafe { faux_device_destroy(ctx as *mut faux_device) };
}

unsafe extern "C" fn software_node_unregister_node_group_wrapper(ctx: *mut c_void) {
    unsafe { software_node_unregister_node_group(ctx as *const *const software_node) };
}

unsafe extern "C" fn software_node_unregister_wrapper(ctx: *mut c_void) {
    unsafe { software_node_unregister(ctx as *const software_node) };
}

unsafe extern "C" fn device_remove_software_node_wrapper(ctx: *mut c_void) {
    unsafe { device_remove_software_node(ctx as *mut device) };
}

unsafe extern "C" fn cs35l56_test_devm_get_vendor_specific_variant_id_none(
    _dev: *mut device,
    _ssid_vendor: c_int,
    _ssid_device: c_int,
) -> *const c_char {
    unsafe { ERR_PTR(-(ENOENT as c_long)) }
}

unsafe extern "C" fn cs35l56_test_system_name_from_ssid(test: *mut kunit) {
    let priv_ = unsafe { (*test).priv_ as *mut cs35l56_test_priv };
    let cs35l56 = unsafe { (*priv_).cs35l56_priv };

    unsafe {
        (*cs35l56).speaker_id = -1;
        snd_soc_card_set_pci_ssid((*(*cs35l56).component).card, 0x12b4, 0xa7c8);

        KUNIT_EXPECT_EQ(test, cs35l56_get_firmware_uid(cs35l56), 0);
        KUNIT_EXPECT_EQ(test, cs35l56_set_fw_name((*cs35l56).component), 0);
        KUNIT_EXPECT_STREQ(test, (*cs35l56).dsp.system_name, c"12b4a7c8".as_ptr());
    }
}

unsafe extern "C" fn cs35l56_test_system_name_from_ssid_and_spkid(test: *mut kunit) {
    let priv_ = unsafe { (*test).priv_ as *mut cs35l56_test_priv };
    let cs35l56 = unsafe { (*priv_).cs35l56_priv };

    unsafe {
        (*cs35l56).speaker_id = 1;
        snd_soc_card_set_pci_ssid((*(*cs35l56).component).card, 0x12b4, 0xa7c8);

        KUNIT_EXPECT_EQ(test, cs35l56_get_firmware_uid(cs35l56), 0);
        KUNIT_EXPECT_EQ(test, cs35l56_set_fw_name((*cs35l56).component), 0);
        KUNIT_EXPECT_STREQ(test, (*cs35l56).dsp.system_name, c"12b4a7c8-spkid1".as_ptr());
    }
}

unsafe extern "C" fn cs35l56_test_system_name_from_property(test: *mut kunit) {
    let priv_ = unsafe { (*test).priv_ as *mut cs35l56_test_priv };
    let cs35l56 = unsafe { (*priv_).cs35l56_priv };
    /* PROPERTY_ENTRY_STRING("cirrus,firmware-uid", "acme"), { } */
    let dev_props: [property_entry; 2] = unsafe { core::mem::zeroed() };
    let dev_node = software_node_const(c"SPK1".as_ptr(), dev_props.as_ptr(), ptr::null());

    unsafe {
        (*cs35l56).speaker_id = -1;

        KUNIT_ASSERT_EQ(test, device_add_software_node((*cs35l56).base.dev, &dev_node), 0);
        KUNIT_ASSERT_EQ(
            test,
            0,
            kunit_add_action_or_reset(
                test,
                device_remove_software_node_wrapper,
                (*cs35l56).base.dev as *mut c_void,
            ),
        );

        KUNIT_EXPECT_EQ(test, cs35l56_get_firmware_uid(cs35l56), 0);
        KUNIT_EXPECT_EQ(test, cs35l56_set_fw_name((*cs35l56).component), 0);
        KUNIT_EXPECT_STREQ(test, (*cs35l56).dsp.system_name, c"acme".as_ptr());
    }
}

unsafe extern "C" fn cs35l56_test_system_name_from_property_and_spkid(test: *mut kunit) {
    let priv_ = unsafe { (*test).priv_ as *mut cs35l56_test_priv };
    let cs35l56 = unsafe { (*priv_).cs35l56_priv };
    /* PROPERTY_ENTRY_STRING("cirrus,firmware-uid", "acme"), { } */
    let dev_props: [property_entry; 2] = unsafe { core::mem::zeroed() };
    let dev_node = software_node_const(c"SPK1".as_ptr(), dev_props.as_ptr(), ptr::null());

    unsafe {
        (*cs35l56).speaker_id = 1;

        KUNIT_ASSERT_EQ(test, device_add_software_node((*cs35l56).base.dev, &dev_node), 0);
        KUNIT_ASSERT_EQ(
            test,
            0,
            kunit_add_action_or_reset(
                test,
                device_remove_software_node_wrapper,
                (*cs35l56).base.dev as *mut c_void,
            ),
        );

        KUNIT_EXPECT_EQ(test, cs35l56_get_firmware_uid(cs35l56), 0);
        KUNIT_EXPECT_EQ(test, cs35l56_set_fw_name((*cs35l56).component), 0);
        KUNIT_EXPECT_STREQ(test, (*cs35l56).dsp.system_name, c"acme-spkid1".as_ptr());
    }
}

unsafe extern "C" fn cs35l56_test_l56_b0_suffix_sdw(test: *mut kunit) {
    let priv_ = unsafe { (*test).priv_ as *mut cs35l56_test_priv };
    let cs35l56 = unsafe { (*priv_).cs35l56_priv };

    unsafe {
        /* Set device type info */
        (*cs35l56).base.type_ = 0x56;
        (*cs35l56).base.rev = 0xb0;

        /* Set the ALSA name prefix */
        (*(*cs35l56).component).name_prefix = c"AMP1".as_ptr();

        /* Set SoundWire link and UID number */
        (*cs35l56).sdw_link_num = 1;
        (*cs35l56).sdw_unique_id = 5;

        kunit_activate_static_stub(
            test,
            cs35l56_test_devm_get_vendor_specific_variant_id_none as *const c_void,
            cs_amp_devm_get_vendor_specific_variant_id as *const c_void,
        );

        KUNIT_EXPECT_EQ(test, 0, cs35l56_set_fw_suffix(cs35l56));

        /* Priority suffix should be the legacy ALSA prefix */
        KUNIT_EXPECT_STREQ(test, (*cs35l56).dsp.fwf_suffix, c"AMP1".as_ptr());

        /* Fallback suffix should be the new SoundWire ID */
        KUNIT_EXPECT_STREQ(test, (*cs35l56).fallback_fw_suffix, c"l1u5".as_ptr());
    }
}

unsafe extern "C" fn cs35l56_test_suffix_sdw(test: *mut kunit) {
    let priv_ = unsafe { (*test).priv_ as *mut cs35l56_test_priv };
    let cs35l56 = unsafe { (*priv_).cs35l56_priv };

    unsafe {
        /* Set the ALSA name prefix */
        (*(*cs35l56).component).name_prefix = c"AMP1".as_ptr();

        /* Set SoundWire link and UID number */
        (*cs35l56).sdw_link_num = 1;
        (*cs35l56).sdw_unique_id = 5;

        kunit_activate_static_stub(
            test,
            cs35l56_test_devm_get_vendor_specific_variant_id_none as *const c_void,
            cs_amp_devm_get_vendor_specific_variant_id as *const c_void,
        );

        KUNIT_EXPECT_EQ(test, 0, cs35l56_set_fw_suffix(cs35l56));

        /* Suffix should be the SoundWire ID without a fallback */
        KUNIT_EXPECT_STREQ(test, (*cs35l56).dsp.fwf_suffix, c"l1u5".as_ptr());
        KUNIT_EXPECT_NULL(test, (*cs35l56).fallback_fw_suffix as *const c_void);
    }
}

unsafe extern "C" fn cs35l56_test_suffix_i2cspi(test: *mut kunit) {
    let priv_ = unsafe { (*test).priv_ as *mut cs35l56_test_priv };
    let cs35l56 = unsafe { (*priv_).cs35l56_priv };

    unsafe {
        /* Set the ALSA name prefix */
        (*(*cs35l56).component).name_prefix = c"AMP1".as_ptr();

        kunit_activate_static_stub(
            test,
            cs35l56_test_devm_get_vendor_specific_variant_id_none as *const c_void,
            cs_amp_devm_get_vendor_specific_variant_id as *const c_void,
        );

        KUNIT_EXPECT_EQ(test, 0, cs35l56_set_fw_suffix(cs35l56));

        /* Suffix strings should not be set: use default wm_adsp suffixing */
        KUNIT_EXPECT_NULL(test, (*cs35l56).dsp.fwf_suffix as *const c_void);
        KUNIT_EXPECT_NULL(test, (*cs35l56).fallback_fw_suffix as *const c_void);
    }
}

unsafe extern "C" fn cs35l56_test_get_efi_ssidexv2(
    _name: *mut efi_char16_t,
    _guid: *mut efi_guid_t,
    _returned_attr: *mut u32,
    size: *mut c_ulong,
    buf: *mut c_void,
) -> efi_status_t {
    unsafe {
        let test = kunit_get_current_test();
        let priv_ = (*test).priv_ as *mut cs35l56_test_priv;
        let len: c_uint;

        KUNIT_ASSERT_NOT_NULL(test, (*priv_).ssidexv2 as *const c_void);
        len = strlen((*priv_).ssidexv2) as c_uint;

        if *size < len as c_ulong {
            *size = len as c_ulong;
            return EFI_BUFFER_TOO_SMALL;
        }

        KUNIT_ASSERT_NOT_NULL(test, buf);
        memcpy(buf, (*priv_).ssidexv2 as *const c_void, len as usize);

        EFI_SUCCESS
    }
}

unsafe extern "C" fn cs35l56_test_ssidexv2_suffix_sdw(test: *mut kunit) {
    let priv_ = unsafe { (*test).priv_ as *mut cs35l56_test_priv };
    let cs35l56 = unsafe { (*priv_).cs35l56_priv };

    unsafe {
        /* Set the ALSA name prefix */
        (*(*cs35l56).component).name_prefix = c"AMP1".as_ptr();

        /* Set SoundWire link and UID number */
        (*cs35l56).sdw_link_num = 1;
        (*cs35l56).sdw_unique_id = 5;

        /* Set a SSID to enable lookup of SSIDExV2 */
        snd_soc_card_set_pci_ssid((*(*cs35l56).component).card, PCI_VENDOR_ID_DELL, 0x1234);

        (*priv_).ssidexv2 = c"10281234_01_BB_CC".as_ptr();

        kunit_activate_static_stub(
            test,
            (*cs_amp_test_hooks).get_efi_variable.unwrap() as *const c_void,
            cs35l56_test_get_efi_ssidexv2 as *const c_void,
        );

        KUNIT_EXPECT_EQ(test, 0, cs35l56_set_fw_suffix(cs35l56));

        /* Priority suffix should be the SSIDExV2 string with SoundWire ID */
        KUNIT_EXPECT_STREQ(test, (*cs35l56).dsp.fwf_suffix, c"01-l1u5".as_ptr());

        /* Fallback suffix should be the SoundWireID */
        KUNIT_EXPECT_STREQ(test, (*cs35l56).fallback_fw_suffix, c"l1u5".as_ptr());
    }
}

unsafe extern "C" fn cs35l56_test_ssidexv2_suffix_i2cspi(test: *mut kunit) {
    let priv_ = unsafe { (*test).priv_ as *mut cs35l56_test_priv };
    let cs35l56 = unsafe { (*priv_).cs35l56_priv };

    unsafe {
        /* Set the ALSA name prefix */
        (*(*cs35l56).component).name_prefix = c"AMP1".as_ptr();

        /* Set a SSID to enable lookup of SSIDExV2 */
        snd_soc_card_set_pci_ssid((*(*cs35l56).component).card, PCI_VENDOR_ID_DELL, 0x1234);

        (*priv_).ssidexv2 = c"10281234_01_BB_CC".as_ptr();

        kunit_activate_static_stub(
            test,
            (*cs_amp_test_hooks).get_efi_variable.unwrap() as *const c_void,
            cs35l56_test_get_efi_ssidexv2 as *const c_void,
        );

        KUNIT_EXPECT_EQ(test, 0, cs35l56_set_fw_suffix(cs35l56));

        /* Priority suffix should be the SSIDExV2 string with ALSA name prefix */
        KUNIT_EXPECT_STREQ(test, (*cs35l56).dsp.fwf_suffix, c"01-AMP1".as_ptr());

        /* Fallback suffix should be the ALSA name prefix */
        KUNIT_EXPECT_STREQ(test, (*cs35l56).fallback_fw_suffix, c"AMP1".as_ptr());
    }
}

/*
 * CS35L56 B0 SoundWire should ignore any SSIDExV2 suffix. It isn't needed
 * on any products with B0 silicon and would interfere with the fallback
 * to legacy naming convention for early B0-based laptops.
 */
unsafe extern "C" fn cs35l56_test_l56_b0_ssidexv2_ignored_suffix_sdw(test: *mut kunit) {
    let priv_ = unsafe { (*test).priv_ as *mut cs35l56_test_priv };
    let cs35l56 = unsafe { (*priv_).cs35l56_priv };

    unsafe {
        /* Set device type info */
        (*cs35l56).base.type_ = 0x56;
        (*cs35l56).base.rev = 0xb0;

        /* Set the ALSA name prefix */
        (*(*cs35l56).component).name_prefix = c"AMP1".as_ptr();

        /* Set SoundWire link and UID number */
        (*cs35l56).sdw_link_num = 1;
        (*cs35l56).sdw_unique_id = 5;

        /* Set a SSID to enable lookup of SSIDExV2 */
        snd_soc_card_set_pci_ssid((*(*cs35l56).component).card, PCI_VENDOR_ID_DELL, 0x1234);

        (*priv_).ssidexv2 = c"10281234_01_BB_CC".as_ptr();

        kunit_activate_static_stub(
            test,
            (*cs_amp_test_hooks).get_efi_variable.unwrap() as *const c_void,
            cs35l56_test_get_efi_ssidexv2 as *const c_void,
        );

        KUNIT_EXPECT_EQ(test, 0, cs35l56_set_fw_suffix(cs35l56));

        /* Priority suffix should be the legacy ALSA prefix */
        KUNIT_EXPECT_STREQ(test, (*cs35l56).dsp.fwf_suffix, c"AMP1".as_ptr());

        /* Fallback suffix should be the new SoundWire ID */
        KUNIT_EXPECT_STREQ(test, (*cs35l56).fallback_fw_suffix, c"l1u5".as_ptr());
    }
}

/*
 * Test that cs35l56_process_xu_properties() correctly parses the GPIO and
 * pull values from properties into the arrays in struct cs35l56_base.
 *
 * This test creates the node tree:
 *
 * Node("SWD1") { // top-level device node
 *	Node("AF01") {
 *		Node("mipi-sdca-function-expansion-subproperties") {
 *			property: "01fa-spk-id-gpios-onchip"
 *			property: 01fa-spk-id-gpios-onchip-pull
 *		}
 *	}
 * }
 *
 * Note that in ACPI "mipi-sdca-function-expansion-subproperties" is
 * a special _DSD property that points to a Device(EXT0) node but behaves
 * as an alias of the EXT0 node. The equivalent in software nodes is to
 * create a Node named "mipi-sdca-function-expansion-subproperties" with
 * the properties.
 *
 */
unsafe extern "C" fn cs35l56_test_parse_xu_onchip_spkid(test: *mut kunit) {
    let param = unsafe { (*test).param_value as *const cs35l56_test_param };
    let priv_ = unsafe { (*test).priv_ as *mut cs35l56_test_priv };
    let cs35l56 = unsafe { (*priv_).cs35l56_priv };
    let mut ext0_node: *mut software_node;
    let mut num_gpios: c_int;
    let mut num_pulls: c_int;
    let mut i: c_int;

    unsafe {
        num_gpios = 0;
        while num_gpios < (*param).spkid_gpios.len() as c_int {
            if (*param).spkid_gpios[num_gpios as usize] < 0 {
                break;
            }
            num_gpios += 1;
        }
        KUNIT_ASSERT_LE(test, num_gpios, (*cs35l56).base.onchip_spkid_gpios.len());

        num_pulls = 0;
        while num_pulls < (*param).spkid_pulls.len() as c_int {
            if (*param).spkid_pulls[num_pulls as usize] < 0 {
                break;
            }
            num_pulls += 1;
        }
        KUNIT_ASSERT_LE(test, num_pulls, (*cs35l56).base.onchip_spkid_pulls.len());

        /* PROPERTY_ENTRY_U32_ARRAY_LEN entries using param arrays and counted lengths, { } */
        let ext0_props: [property_entry; 3] = core::mem::zeroed();

        KUNIT_ASSERT_EQ(
            test,
            software_node_register_node_group(cs35l56_test_dev_and_af01_node_group.as_ptr()),
            0,
        );
        KUNIT_ASSERT_EQ(
            test,
            kunit_add_action_or_reset(
                test,
                software_node_unregister_node_group_wrapper,
                cs35l56_test_dev_and_af01_node_group.as_ptr() as *mut c_void,
            ),
            0,
        );

        ext0_node = kunit_kzalloc(test, size_of::<software_node>(), GFP_KERNEL) as *mut software_node;
        KUNIT_ASSERT_NOT_NULL(test, ext0_node as *const c_void);
        *ext0_node = software_node_const(
            c"mipi-sdca-function-expansion-subproperties".as_ptr(),
            ext0_props.as_ptr(),
            &cs35l56_test_af01_sw_node,
        );

        KUNIT_ASSERT_EQ(test, software_node_register(ext0_node), 0);
        KUNIT_ASSERT_EQ(
            test,
            kunit_add_action_or_reset(test, software_node_unregister_wrapper, ext0_node as *mut c_void),
            0,
        );

        KUNIT_ASSERT_EQ(
            test,
            device_add_software_node((*cs35l56).base.dev, &cs35l56_test_dev_sw_node),
            0,
        );
        KUNIT_ASSERT_EQ(
            test,
            0,
            kunit_add_action_or_reset(
                test,
                device_remove_software_node_wrapper,
                (*cs35l56).base.dev as *mut c_void,
            ),
        );

        KUNIT_EXPECT_EQ(test, cs35l56_process_xu_properties(cs35l56), 0);

        KUNIT_EXPECT_EQ(test, (*cs35l56).base.num_onchip_spkid_gpios, num_gpios);
        KUNIT_EXPECT_EQ(test, (*cs35l56).base.num_onchip_spkid_pulls, num_pulls);

        i = 0;
        while i < (*param).spkid_gpios.len() as c_int {
            if (*param).spkid_gpios[i as usize] < 0 {
                break;
            }

            /*
             * cs35l56_process_xu_properties() stores the GPIO numbers
             * zero-based, which is one less than the value in the property.
             */
            KUNIT_EXPECT_EQ_MSG(
                test,
                (*cs35l56).base.onchip_spkid_gpios[i as usize],
                (*param).spkid_gpios[i as usize] - 1,
                c"i=%d".as_ptr(),
                i,
            );
            i += 1;
        }

        i = 0;
        while i < (*param).spkid_pulls.len() as c_int {
            if (*param).spkid_pulls[i as usize] < 0 {
                break;
            }

            KUNIT_EXPECT_EQ_MSG(
                test,
                (*cs35l56).base.onchip_spkid_pulls[i as usize],
                (*param).spkid_pulls[i as usize],
                c"i=%d".as_ptr(),
                i,
            );
            i += 1;
        }
    }
}

unsafe extern "C" fn cs35l56_test_dummy_read_onchip_spkid(_cs35l56_base: *mut cs35l56_base) -> c_int {
    unsafe {
        let test = kunit_get_current_test();
        let priv_ = (*test).priv_ as *mut cs35l56_test_priv;

        (*priv_).read_onchip_spkid_called = true;

        4
    }
}

unsafe extern "C" fn cs35l56_test_dummy_configure_onchip_spkid_pads(
    _cs35l56_base: *mut cs35l56_base,
) -> c_int {
    unsafe {
        let test = kunit_get_current_test();
        let priv_ = (*test).priv_ as *mut cs35l56_test_priv;

        (*priv_).configure_onchip_spkid_pads_called = true;

        0
    }
}

unsafe extern "C" fn cs35l56_test_set_fw_name_reads_onchip_spkid(test: *mut kunit) {
    let priv_ = unsafe { (*test).priv_ as *mut cs35l56_test_priv };
    let cs35l56 = unsafe { (*priv_).cs35l56_priv };

    unsafe {
        /* Provide some on-chip GPIOs for spkid */
        (*cs35l56).base.onchip_spkid_gpios[0] = 1;
        (*cs35l56).base.num_onchip_spkid_gpios = 1;

        (*cs35l56).speaker_id = -ENOENT;

        kunit_activate_static_stub(
            test,
            cs35l56_configure_onchip_spkid_pads as *const c_void,
            cs35l56_test_dummy_configure_onchip_spkid_pads as *const c_void,
        );
        kunit_activate_static_stub(
            test,
            cs35l56_read_onchip_spkid as *const c_void,
            cs35l56_test_dummy_read_onchip_spkid as *const c_void,
        );

        (*priv_).configure_onchip_spkid_pads_called = false;
        (*priv_).read_onchip_spkid_called = false;
        KUNIT_EXPECT_EQ(test, cs35l56_set_fw_name((*cs35l56).component), 0);
        KUNIT_EXPECT_TRUE(test, (*priv_).configure_onchip_spkid_pads_called);
        KUNIT_EXPECT_TRUE(test, (*priv_).read_onchip_spkid_called);
        KUNIT_EXPECT_EQ(
            test,
            (*cs35l56).speaker_id,
            cs35l56_test_dummy_read_onchip_spkid(&mut (*cs35l56).base),
        );
    }
}

unsafe extern "C" fn cs35l56_test_set_fw_name_preserves_spkid_with_onchip_gpios(test: *mut kunit) {
    let priv_ = unsafe { (*test).priv_ as *mut cs35l56_test_priv };
    let cs35l56 = unsafe { (*priv_).cs35l56_priv };

    unsafe {
        /* Provide some on-chip GPIOs for spkid */
        (*cs35l56).base.onchip_spkid_gpios[0] = 1;
        (*cs35l56).base.num_onchip_spkid_gpios = 1;

        /* Simulate that the driver already got a spkid from somewhere */
        (*cs35l56).speaker_id = 15;

        KUNIT_EXPECT_EQ(test, cs35l56_set_fw_name((*cs35l56).component), 0);
        KUNIT_EXPECT_EQ(test, (*cs35l56).speaker_id, 15);
    }
}

unsafe extern "C" fn cs35l56_test_set_fw_name_preserves_spkid_without_onchip_gpios(test: *mut kunit) {
    let priv_ = unsafe { (*test).priv_ as *mut cs35l56_test_priv };
    let cs35l56 = unsafe { (*priv_).cs35l56_priv };

    unsafe {
        (*cs35l56).base.num_onchip_spkid_gpios = 0;

        /* Simulate that the driver already got a spkid from somewhere */
        (*cs35l56).speaker_id = 15;

        KUNIT_EXPECT_EQ(test, cs35l56_set_fw_name((*cs35l56).component), 0);
        KUNIT_EXPECT_EQ(test, (*cs35l56).speaker_id, 15);
    }
}

unsafe extern "C" fn cs35l56_test_case_init_common(test: *mut kunit) -> c_int {
    unsafe {
        let mut priv_: *mut cs35l56_test_priv;
        let param = (*test).param_value as *const cs35l56_test_param;
        let cs35l56: *mut cs35l56_private;

        KUNIT_ASSERT_NOT_NULL(test, cs_amp_test_hooks as *const c_void);

        priv_ = kunit_kzalloc(test, size_of::<cs35l56_test_priv>(), GFP_KERNEL) as *mut cs35l56_test_priv;
        if priv_.is_null() {
            return -ENOMEM;
        }

        (*test).priv_ = priv_ as *mut c_void;

        /* Create dummy amp driver dev */
        (*priv_).amp_dev = faux_device_create(c"cs35l56_test_drv".as_ptr(), ptr::null_mut(), ptr::null_mut());
        KUNIT_ASSERT_NOT_NULL(test, (*priv_).amp_dev as *const c_void);
        KUNIT_ASSERT_EQ(
            test,
            0,
            kunit_add_action_or_reset(test, faux_device_destroy_wrapper, (*priv_).amp_dev as *mut c_void),
        );

        /* Construct minimal set of driver structs */
        (*priv_).cs35l56_priv =
            kunit_kzalloc(test, size_of::<cs35l56_private>(), GFP_KERNEL) as *mut cs35l56_private;
        KUNIT_ASSERT_NOT_NULL(test, (*priv_).cs35l56_priv as *const c_void);
        cs35l56 = (*priv_).cs35l56_priv;
        (*cs35l56).base.dev = &mut (*(*priv_).amp_dev).dev;

        (*cs35l56).component =
            kunit_kzalloc(test, size_of::<snd_soc_component>(), GFP_KERNEL) as *mut snd_soc_component;
        KUNIT_ASSERT_NOT_NULL(test, (*cs35l56).component as *const c_void);
        (*(*cs35l56).component).dev = (*cs35l56).base.dev;
        snd_soc_component_set_drvdata((*cs35l56).component, cs35l56 as *mut c_void);

        (*(*cs35l56).component).card =
            kunit_kzalloc(test, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
        KUNIT_ASSERT_NOT_NULL(test, (*(*cs35l56).component).card as *const c_void);

        if !param.is_null() {
            (*cs35l56).base.type_ = (*param).type_;
            (*cs35l56).base.rev = (*param).rev;
        }

        0
    }
}

unsafe extern "C" fn cs35l56_test_case_init_soundwire(test: *mut kunit) -> c_int {
    unsafe {
        let priv_: *mut cs35l56_test_priv;
        let cs35l56: *mut cs35l56_private;
        let ret: c_int;

        ret = cs35l56_test_case_init_common(test);
        if ret != 0 {
            return ret;
        }

        priv_ = (*test).priv_ as *mut cs35l56_test_priv;
        cs35l56 = (*priv_).cs35l56_priv;

        /* Dummy to indicate this is Soundwire */
        (*cs35l56).sdw_peripheral = kunit_kzalloc(test, 1, GFP_KERNEL);
        if (*cs35l56).sdw_peripheral.is_null() {
            return -ENOMEM;
        }

        0
    }
}

unsafe extern "C" fn cs35l56_test_gpio_param_desc(param: *const cs35l56_test_param, desc: *mut c_char) {
    unsafe {
        let mut gpios = [0 as c_char; 1 + (2 * 4)];
        let mut pulls = [0 as c_char; 1 + (2 * 4)];
        let mut i: c_int;
        let mut gpios_len: usize = 0;
        let mut pulls_len: usize = 0;

        i = 0;
        while i < (*param).spkid_gpios.len() as c_int {
            if (*param).spkid_gpios[i as usize] < 0 {
                break;
            }

            gpios_len += snprintf(
                gpios.as_mut_ptr().add(gpios_len),
                gpios.len() - gpios_len,
                c"%s%d".as_ptr(),
                if i == 0 { c"".as_ptr() } else { c",".as_ptr() },
                (*param).spkid_gpios[i as usize],
            ) as usize;
            i += 1;
        }

        i = 0;
        while i < (*param).spkid_pulls.len() as c_int {
            if (*param).spkid_pulls[i as usize] < 0 {
                break;
            }

            pulls_len += snprintf(
                pulls.as_mut_ptr().add(pulls_len),
                pulls.len() - pulls_len,
                c"%s%d".as_ptr(),
                if i == 0 { c"".as_ptr() } else { c",".as_ptr() },
                (*param).spkid_pulls[i as usize],
            ) as usize;
            i += 1;
        }

        snprintf(
            desc,
            KUNIT_PARAM_DESC_SIZE,
            c"gpios:{%s} pulls:{%s}".as_ptr(),
            gpios.as_ptr(),
            pulls.as_ptr(),
        );
    }
}

static cs35l56_test_onchip_spkid_cases: [cs35l56_test_param; 20] = [
    cs35l56_test_param { type_: 0, rev: 0, spkid_gpios: [1, -1, 0, 0], spkid_pulls: [-1, -1, 0, 0] },
    cs35l56_test_param { type_: 0, rev: 0, spkid_gpios: [1, -1, 0, 0], spkid_pulls: [-1, -1, 0, 0] },
    cs35l56_test_param { type_: 0, rev: 0, spkid_gpios: [1, -1, 0, 0], spkid_pulls: [1, -1, 0, 0] },
    cs35l56_test_param { type_: 0, rev: 0, spkid_gpios: [1, -1, 0, 0], spkid_pulls: [2, -1, 0, 0] },

    cs35l56_test_param { type_: 0, rev: 0, spkid_gpios: [7, -1, 0, 0], spkid_pulls: [-1, -1, 0, 0] },
    cs35l56_test_param { type_: 0, rev: 0, spkid_gpios: [7, -1, 0, 0], spkid_pulls: [-1, -1, 0, 0] },
    cs35l56_test_param { type_: 0, rev: 0, spkid_gpios: [7, -1, 0, 0], spkid_pulls: [1, -1, 0, 0] },
    cs35l56_test_param { type_: 0, rev: 0, spkid_gpios: [7, -1, 0, 0], spkid_pulls: [2, -1, 0, 0] },

    cs35l56_test_param { type_: 0, rev: 0, spkid_gpios: [1, 7, -1, 0], spkid_pulls: [-1, -1, -1, 0] },
    cs35l56_test_param { type_: 0, rev: 0, spkid_gpios: [1, 7, -1, 0], spkid_pulls: [-1, -1, -1, 0] },
    cs35l56_test_param { type_: 0, rev: 0, spkid_gpios: [1, 7, -1, 0], spkid_pulls: [1, 1, -1, 0] },
    cs35l56_test_param { type_: 0, rev: 0, spkid_gpios: [1, 7, -1, 0], spkid_pulls: [2, 2, -1, 0] },

    cs35l56_test_param { type_: 0, rev: 0, spkid_gpios: [7, 1, -1, 0], spkid_pulls: [-1, -1, -1, 0] },
    cs35l56_test_param { type_: 0, rev: 0, spkid_gpios: [7, 1, -1, 0], spkid_pulls: [-1, -1, -1, 0] },
    cs35l56_test_param { type_: 0, rev: 0, spkid_gpios: [7, 1, -1, 0], spkid_pulls: [1, 1, -1, 0] },
    cs35l56_test_param { type_: 0, rev: 0, spkid_gpios: [7, 1, -1, 0], spkid_pulls: [2, 2, -1, 0] },

    cs35l56_test_param { type_: 0, rev: 0, spkid_gpios: [3, 7, 1, -1], spkid_pulls: [-1, -1, -1, -1] },
    cs35l56_test_param { type_: 0, rev: 0, spkid_gpios: [3, 7, 1, -1], spkid_pulls: [-1, -1, -1, -1] },
    cs35l56_test_param { type_: 0, rev: 0, spkid_gpios: [3, 7, 1, -1], spkid_pulls: [1, 1, 1, -1] },
    cs35l56_test_param { type_: 0, rev: 0, spkid_gpios: [3, 7, 1, -1], spkid_pulls: [2, 2, 2, -1] },
];
/* KUNIT_ARRAY_PARAM(cs35l56_test_onchip_spkid, cs35l56_test_onchip_spkid_cases, cs35l56_test_gpio_param_desc); */

unsafe extern "C" fn cs35l56_test_type_rev_param_desc(param: *const cs35l56_test_param, desc: *mut c_char) {
    unsafe {
        snprintf(
            desc,
            KUNIT_PARAM_DESC_SIZE,
            c"type: %02x rev: %02x".as_ptr(),
            (*param).type_ as c_int,
            (*param).rev as c_int,
        );
    }
}

static cs35l56_test_type_rev_ex_b0_param_cases: [cs35l56_test_param; 3] = [
    cs35l56_test_param { type_: 0x56, rev: 0xb2, spkid_gpios: [0; 4], spkid_pulls: [0; 4] },
    cs35l56_test_param { type_: 0x57, rev: 0xb2, spkid_gpios: [0; 4], spkid_pulls: [0; 4] },
    cs35l56_test_param { type_: 0x63, rev: 0xa1, spkid_gpios: [0; 4], spkid_pulls: [0; 4] },
];
/* KUNIT_ARRAY_PARAM(cs35l56_test_type_rev_ex_b0, cs35l56_test_type_rev_ex_b0_param_cases, cs35l56_test_type_rev_param_desc); */

static cs35l56_test_type_rev_all_param_cases: [cs35l56_test_param; 4] = [
    cs35l56_test_param { type_: 0x56, rev: 0xb0, spkid_gpios: [0; 4], spkid_pulls: [0; 4] },
    cs35l56_test_param { type_: 0x56, rev: 0xb2, spkid_gpios: [0; 4], spkid_pulls: [0; 4] },
    cs35l56_test_param { type_: 0x57, rev: 0xb2, spkid_gpios: [0; 4], spkid_pulls: [0; 4] },
    cs35l56_test_param { type_: 0x63, rev: 0xa1, spkid_gpios: [0; 4], spkid_pulls: [0; 4] },
];
/* KUNIT_ARRAY_PARAM(cs35l56_test_type_rev_all, cs35l56_test_type_rev_all_param_cases, cs35l56_test_type_rev_param_desc); */

/* KUNIT_CASE and KUNIT_CASE_PARAM encode test function pointers and parameter generators in kernel KUnit metadata. */
static mut cs35l56_test_cases_soundwire: [kunit_case; 13] = unsafe { core::mem::zeroed() };

static mut cs35l56_test_cases_not_soundwire: [kunit_case; 10] = unsafe { core::mem::zeroed() };

static mut cs35l56_test_suite_soundwire: kunit_suite = kunit_suite {
    name: c"snd-soc-cs35l56-test-soundwire".as_ptr(),
    init: Some(cs35l56_test_case_init_soundwire),
    test_cases: unsafe { cs35l56_test_cases_soundwire.as_mut_ptr() },
};

static mut cs35l56_test_suite_not_soundwire: kunit_suite = kunit_suite {
    name: c"snd-soc-cs35l56-test-not-soundwire".as_ptr(),
    init: Some(cs35l56_test_case_init_common),
    test_cases: unsafe { cs35l56_test_cases_not_soundwire.as_mut_ptr() },
};

/* kunit_test_suites(&cs35l56_test_suite_soundwire, &cs35l56_test_suite_not_soundwire); */

/* MODULE_IMPORT_NS("SND_SOC_CS_AMP_LIB"); */
/* MODULE_IMPORT_NS("SND_SOC_CS35L56_SHARED"); */
/* MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING"); */
/* MODULE_DESCRIPTION("KUnit test for Cirrus Logic cs35l56 codec driver"); */
/* MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
