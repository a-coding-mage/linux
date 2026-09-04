// SPDX-License-Identifier: GPL-2.0-or-later
//
// Validation of USB-audio class descriptors
//

// Note: External dependencies from <linux/init.h>, <linux/usb.h>,
// <linux/usb/audio.h>, <linux/usb/audio-v2.h>, <linux/usb/audio-v3.h>,
// <linux/usb/midi.h>, "usbaudio.h", "helper.h"

use std::ffi::c_void;

// External type declarations (defined in other modules)
#[repr(C)]
pub struct Uac1AcHeaderDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDescriptorSubtype: u8,
    pub bcdADC: u16,
    pub wTotalLength: u16,
    pub bInCollection: u8,
}

#[repr(C)]
pub struct Uac2AcHeaderDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDescriptorSubtype: u8,
    pub bcdADC: u16,
    pub bCategory: u8,
    pub wTotalLength: u16,
    pub bmControls: u8,
}

#[repr(C)]
pub struct Uac3AcHeaderDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDescriptorSubtype: u8,
    pub bcdADC: u16,
    pub bCategory: u8,
    pub wTotalLength: u16,
    pub bmControls: u32,
}

#[repr(C)]
pub struct UacAcHeaderDescriptor {
    pub bLength: u8,
}

#[repr(C)]
pub struct UacInputTerminalDescriptor {
    pub bLength: u8,
}

#[repr(C)]
pub struct Uac1OutputTerminalDescriptor {
    pub bLength: u8,
}

#[repr(C)]
pub struct Uac2InputTerminalDescriptor {
    pub bLength: u8,
}

#[repr(C)]
pub struct Uac2OutputTerminalDescriptor {
    pub bLength: u8,
}

#[repr(C)]
pub struct Uac3InputTerminalDescriptor {
    pub bLength: u8,
}

#[repr(C)]
pub struct Uac3OutputTerminalDescriptor {
    pub bLength: u8,
}

#[repr(C)]
pub struct UacMixerUnitDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDescriptorSubtype: u8,
    pub bUnitID: u8,
    pub bNrInPins: u8,
}

#[repr(C)]
pub struct UacProcessingUnitDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDescriptorSubtype: u8,
    pub bUnitID: u8,
    pub wProcessType: u16,
    pub bNrInPins: u8,
}

#[repr(C)]
pub struct UacSelectorUnitDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDescriptorSubtype: u8,
    pub bUnitID: u8,
    pub bNrInPins: u8,
}

#[repr(C)]
pub struct UacFeatureUnitDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDescriptorSubtype: u8,
    pub bUnitID: u8,
    pub bSourceID: u8,
    pub bControlSize: u8,
}

#[repr(C)]
pub struct Uac2FeatureUnitDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDescriptorSubtype: u8,
    pub bUnitID: u4,
    pub bSourceID: u8,
    pub bmaControls: [u8; 4],
}

#[repr(C)]
pub struct Uac3FeatureUnitDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDescriptorSubtype: u8,
    pub bUnitID: u8,
    pub bSourceID: u8,
    pub bmaControls: [u8; 4],
}

#[repr(C)]
pub struct Uac3PowerDomainDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDescriptorSubtype: u8,
    pub bPowerDomainID: u8,
    pub bNrEntities: u8,
}

#[repr(C)]
pub struct Uac2EffectUnitDescriptor {
    pub bLength: u8,
}

#[repr(C)]
pub struct UacClockSourceDescriptor {
    pub bLength: u8,
}

#[repr(C)]
pub struct UacClockMultiplierDescriptor {
    pub bLength: u8,
}

#[repr(C)]
pub struct Uac3ClockSourceDescriptor {
    pub bLength: u8,
}

#[repr(C)]
pub struct Uac3ClockMultiplierDescriptor {
    pub bLength: u8,
}

#[repr(C)]
pub struct UsbMidiInJackDescriptor {
    pub bLength: u8,
}

#[repr(C)]
pub struct UsbMidiOutJackDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDescriptorSubtype: u8,
    pub bJackID: u8,
    pub bJackType: u8,
    pub iJack: u8,
    pub bNrInputPins: u8,
}

#[repr(C)]
pub struct UsbMsHeaderDescriptor {
    pub bLength: u8,
}

type UsbDescValidatorFunc = fn(*const c_void, *const UsbDescValidator) -> bool;

#[repr(C)]
pub struct UsbDescValidator {
    pub protocol: u8,
    pub r#type: u8,
    pub func: Option<UsbDescValidatorFunc>,
    pub size: usize,
}

const UAC_VERSION_ALL: u8 = u8::MAX;
const UAC_VERSION_1: u8 = 0x01;
const UAC_VERSION_2: u8 = 0x20;
const UAC_VERSION_3: u8 = 0x30;

const USB_DT_CS_INTERFACE: u8 = 0x24;

const UAC_HEADER: u8 = 0x01;
const UAC_INPUT_TERMINAL: u8 = 0x02;
const UAC_OUTPUT_TERMINAL: u8 = 0x03;
const UAC_MIXER_UNIT: u8 = 0x04;
const UAC_SELECTOR_UNIT: u8 = 0x05;
const UAC_FEATURE_UNIT: u8 = 0x06;
const UAC1_PROCESSING_UNIT: u8 = 0x07;
const UAC1_EXTENSION_UNIT: u8 = 0x08;

const UAC2_EFFECT_UNIT: u8 = 0x07;
const UAC2_PROCESSING_UNIT_V2: u8 = 0x08;
const UAC2_EXTENSION_UNIT_V2: u8 = 0x09;
const UAC2_CLOCK_SOURCE: u8 = 0x0a;
const UAC2_CLOCK_SELECTOR: u8 = 0x0b;
const UAC2_CLOCK_MULTIPLIER: u8 = 0x0c;

const UAC3_MIXER_UNIT: u8 = 0x04;
const UAC3_SELECTOR_UNIT: u8 = 0x05;
const UAC3_FEATURE_UNIT: u8 = 0x06;
const UAC3_EFFECT_UNIT: u8 = 0x07;
const UAC3_PROCESSING_UNIT: u8 = 0x08;
const UAC3_EXTENSION_UNIT: u8 = 0x09;
const UAC3_CLOCK_SOURCE: u8 = 0x0a;
const UAC3_CLOCK_SELECTOR: u8 = 0x0b;
const UAC3_CLOCK_MULTIPLIER: u8 = 0x0c;
const UAC3_POWER_DOMAIN: u8 = 0x0d;

const USB_MS_HEADER: u8 = 0x01;
const USB_MS_MIDI_IN_JACK: u8 = 0x02;
const USB_MS_MIDI_OUT_JACK: u8 = 0x03;

const UAC_PROCESS_UP_DOWNMIX: u16 = 0x01;
const UAC_PROCESS_DOLBY_PROLOGIC: u16 = 0x02;

const UAC2_PROCESS_UP_DOWNMIX: u16 = 0x01;
const UAC2_PROCESS_DOLBY_PROLOCIC: u16 = 0x02;

const UAC3_PROCESS_UP_DOWNMIX: u16 = 0x01;
const UAC3_PROCESS_MULTI_FUNCTION: u16 = 0x02;

const KERN_ERR: &str = "";
const DUMP_PREFIX_NONE: u32 = 0;

// External function declarations
extern "C" {
    fn print_hex_dump(
        log_level: *const u8,
        prefix: *const u8,
        prefix_type: u32,
        rowsize: u32,
        groupsize: u32,
        buf: *const u8,
        len: u8,
        ascii: bool,
    );
    static snd_usb_skip_validation: bool;
}

fn le16_to_cpu(x: u16) -> u16 {
    u16::from_le(x)
}

// UAC1 only
fn validate_uac1_header(p: *const c_void, _v: *const UsbDescValidator) -> bool {
    unsafe {
        let d = p as *const Uac1AcHeaderDescriptor;
        let size = std::mem::size_of::<Uac1AcHeaderDescriptor>();
        (*d).bLength >= size as u8 &&
            (*d).bLength >= (size as u8 + (*d).bInCollection)
    }
}

// for mixer unit; covering all UACs
fn validate_mixer_unit(p: *const c_void, v: *const UsbDescValidator) -> bool {
    unsafe {
        let d = p as *const UacMixerUnitDescriptor;
        let mut len: usize;

        if (*d).bLength < std::mem::size_of::<UacMixerUnitDescriptor>() as u8 || (*d).bNrInPins == 0
        {
            return false;
        }
        len = std::mem::size_of::<UacMixerUnitDescriptor>() + (*d).bNrInPins as usize;
        match (*v).protocol {
            UAC_VERSION_1 | _ => {
                len += 2 + 1;
                len += 1;
            }
            UAC_VERSION_2 => {
                len += 4 + 1;
                len += 1 + 1;
            }
            UAC_VERSION_3 => {
                len += 2;
            }
        }
        (*d).bLength as usize >= len
    }
}

// both for processing and extension units; covering all UACs
fn validate_processing_unit(p: *const c_void, v: *const UsbDescValidator) -> bool {
    unsafe {
        let d = p as *const UacProcessingUnitDescriptor;
        let hdr = p as *const u8;
        let mut len: usize;
        let m: usize;

        if (*d).bLength < std::mem::size_of::<UacProcessingUnitDescriptor>() as u8 {
            return false;
        }
        len = std::mem::size_of::<UacProcessingUnitDescriptor>() + (*d).bNrInPins as usize;
        if (*d).bLength < len as u8 {
            return false;
        }
        match (*v).protocol {
            UAC_VERSION_1 | _ => {
                len += 1 + 2 + 1;
                if (*d).bLength < (len + 1) as u8 {
                    return false;
                }
                let m_val = *hdr.add(len);
                len += 1 + m_val as usize + 1;
            }
            UAC_VERSION_2 => {
                len += 1 + 4 + 1;
                if (*v).r#type == UAC2_PROCESSING_UNIT_V2 {
                    len += 2;
                } else {
                    len += 1;
                }
                len += 1;
            }
            UAC_VERSION_3 => {
                len += 2 + 4;
            }
        }
        if (*d).bLength < len as u8 {
            return false;
        }

        match (*v).protocol {
            UAC_VERSION_1 | _ => {
                if (*v).r#type == UAC1_EXTENSION_UNIT {
                    return true;
                }
                match le16_to_cpu((*d).wProcessType) {
                    UAC_PROCESS_UP_DOWNMIX | UAC_PROCESS_DOLBY_PROLOGIC => {
                        if (*d).bLength < (len + 1) as u8 {
                            return false;
                        }
                        let m_val = *hdr.add(len);
                        len += 1 + (m_val as usize) * 2;
                    }
                    _ => {}
                }
            }
            UAC_VERSION_2 => {
                if (*v).r#type == UAC2_EXTENSION_UNIT_V2 {
                    return true;
                }
                match le16_to_cpu((*d).wProcessType) {
                    UAC2_PROCESS_UP_DOWNMIX | UAC2_PROCESS_DOLBY_PROLOCIC => {
                        if (*d).bLength < (len + 1) as u8 {
                            return false;
                        }
                        let m_val = *hdr.add(len);
                        len += 1 + (m_val as usize) * 4;
                    }
                    _ => {}
                }
            }
            UAC_VERSION_3 => {
                if (*v).r#type == UAC3_EXTENSION_UNIT {
                    len += 2;
                } else {
                    match le16_to_cpu((*d).wProcessType) {
                        UAC3_PROCESS_UP_DOWNMIX => {
                            if (*d).bLength < (len + 1) as u8 {
                                return false;
                            }
                            let m_val = *hdr.add(len);
                            len += 1 + (m_val as usize) * 2;
                        }
                        UAC3_PROCESS_MULTI_FUNCTION => {
                            len += 2 + 4;
                        }
                        _ => {}
                    }
                }
            }
        }
        if (*d).bLength < len as u8 {
            return false;
        }

        true
    }
}

// both for selector and clock selector units; covering all UACs
fn validate_selector_unit(p: *const c_void, v: *const UsbDescValidator) -> bool {
    unsafe {
        let d = p as *const UacSelectorUnitDescriptor;
        let mut len: usize;

        if (*d).bLength < std::mem::size_of::<UacSelectorUnitDescriptor>() as u8 {
            return false;
        }
        len = std::mem::size_of::<UacSelectorUnitDescriptor>() + (*d).bNrInPins as usize;
        match (*v).protocol {
            UAC_VERSION_1 | _ => {
                len += 1;
            }
            UAC_VERSION_2 => {
                len += 1 + 1;
            }
            UAC_VERSION_3 => {
                len += 4 + 2;
            }
        }
        (*d).bLength as usize >= len
    }
}

fn validate_uac1_feature_unit(p: *const c_void, _v: *const UsbDescValidator) -> bool {
    unsafe {
        let d = p as *const UacFeatureUnitDescriptor;

        if (*d).bLength < std::mem::size_of::<UacFeatureUnitDescriptor>() as u8 || (*d).bControlSize == 0
        {
            return false;
        }
        (*d).bLength as usize >= std::mem::size_of::<UacFeatureUnitDescriptor>() + (*d).bControlSize as usize + 1
    }
}

fn validate_uac2_feature_unit(p: *const c_void, _v: *const UsbDescValidator) -> bool {
    unsafe {
        let d = p as *const Uac2FeatureUnitDescriptor;

        if (*d).bLength < std::mem::size_of::<Uac2FeatureUnitDescriptor>() as u8 {
            return false;
        }
        (*d).bLength as usize >= std::mem::size_of::<Uac2FeatureUnitDescriptor>() + 4 + 1
    }
}

fn validate_uac3_feature_unit(p: *const c_void, _v: *const UsbDescValidator) -> bool {
    unsafe {
        let d = p as *const Uac3FeatureUnitDescriptor;

        if (*d).bLength < std::mem::size_of::<Uac3FeatureUnitDescriptor>() as u8 {
            return false;
        }
        (*d).bLength as usize >= std::mem::size_of::<Uac3FeatureUnitDescriptor>() + 4 + 2
    }
}

fn validate_uac3_power_domain_unit(p: *const c_void, _v: *const UsbDescValidator) -> bool {
    unsafe {
        let d = p as *const Uac3PowerDomainDescriptor;

        if (*d).bLength < std::mem::size_of::<Uac3PowerDomainDescriptor>() as u8 {
            return false;
        }
        (*d).bLength as usize >= std::mem::size_of::<Uac3PowerDomainDescriptor>() + (*d).bNrEntities as usize + 2
    }
}

fn validate_midi_out_jack(p: *const c_void, _v: *const UsbDescValidator) -> bool {
    unsafe {
        let d = p as *const UsbMidiOutJackDescriptor;

        (*d).bLength >= std::mem::size_of::<UsbMidiOutJackDescriptor>() as u8 &&
            (*d).bLength >= (std::mem::size_of::<UsbMidiOutJackDescriptor>() as u8 + (*d).bNrInputPins * 2)
    }
}

static AUDIO_VALIDATORS: &[UsbDescValidator] = &[
    // UAC1
    UsbDescValidator {
        protocol: UAC_VERSION_1,
        r#type: UAC_HEADER,
        func: Some(validate_uac1_header),
        size: 0,
    },
    UsbDescValidator {
        protocol: UAC_VERSION_1,
        r#type: UAC_INPUT_TERMINAL,
        func: None,
        size: std::mem::size_of::<UacInputTerminalDescriptor>(),
    },
    UsbDescValidator {
        protocol: UAC_VERSION_1,
        r#type: UAC_OUTPUT_TERMINAL,
        func: None,
        size: std::mem::size_of::<Uac1OutputTerminalDescriptor>(),
    },
    UsbDescValidator {
        protocol: UAC_VERSION_1,
        r#type: UAC_MIXER_UNIT,
        func: Some(validate_mixer_unit),
        size: 0,
    },
    UsbDescValidator {
        protocol: UAC_VERSION_1,
        r#type: UAC_SELECTOR_UNIT,
        func: Some(validate_selector_unit),
        size: 0,
    },
    UsbDescValidator {
        protocol: UAC_VERSION_1,
        r#type: UAC_FEATURE_UNIT,
        func: Some(validate_uac1_feature_unit),
        size: 0,
    },
    UsbDescValidator {
        protocol: UAC_VERSION_1,
        r#type: UAC1_PROCESSING_UNIT,
        func: Some(validate_processing_unit),
        size: 0,
    },
    UsbDescValidator {
        protocol: UAC_VERSION_1,
        r#type: UAC1_EXTENSION_UNIT,
        func: Some(validate_processing_unit),
        size: 0,
    },
    // UAC2
    UsbDescValidator {
        protocol: UAC_VERSION_2,
        r#type: UAC_HEADER,
        func: None,
        size: std::mem::size_of::<Uac2AcHeaderDescriptor>(),
    },
    UsbDescValidator {
        protocol: UAC_VERSION_2,
        r#type: UAC_INPUT_TERMINAL,
        func: None,
        size: std::mem::size_of::<Uac2InputTerminalDescriptor>(),
    },
    UsbDescValidator {
        protocol: UAC_VERSION_2,
        r#type: UAC_OUTPUT_TERMINAL,
        func: None,
        size: std::mem::size_of::<Uac2OutputTerminalDescriptor>(),
    },
    UsbDescValidator {
        protocol: UAC_VERSION_2,
        r#type: UAC_MIXER_UNIT,
        func: Some(validate_mixer_unit),
        size: 0,
    },
    UsbDescValidator {
        protocol: UAC_VERSION_2,
        r#type: UAC_SELECTOR_UNIT,
        func: Some(validate_selector_unit),
        size: 0,
    },
    UsbDescValidator {
        protocol: UAC_VERSION_2,
        r#type: UAC_FEATURE_UNIT,
        func: Some(validate_uac2_feature_unit),
        size: 0,
    },
    UsbDescValidator {
        protocol: UAC_VERSION_2,
        r#type: UAC2_EFFECT_UNIT,
        func: None,
        size: std::mem::size_of::<Uac2EffectUnitDescriptor>(),
    },
    UsbDescValidator {
        protocol: UAC_VERSION_2,
        r#type: UAC2_PROCESSING_UNIT_V2,
        func: Some(validate_processing_unit),
        size: 0,
    },
    UsbDescValidator {
        protocol: UAC_VERSION_2,
        r#type: UAC2_EXTENSION_UNIT_V2,
        func: Some(validate_processing_unit),
        size: 0,
    },
    UsbDescValidator {
        protocol: UAC_VERSION_2,
        r#type: UAC2_CLOCK_SOURCE,
        func: None,
        size: std::mem::size_of::<UacClockSourceDescriptor>(),
    },
    UsbDescValidator {
        protocol: UAC_VERSION_2,
        r#type: UAC2_CLOCK_SELECTOR,
        func: Some(validate_selector_unit),
        size: 0,
    },
    UsbDescValidator {
        protocol: UAC_VERSION_2,
        r#type: UAC2_CLOCK_MULTIPLIER,
        func: None,
        size: std::mem::size_of::<UacClockMultiplierDescriptor>(),
    },
    // UAC3
    UsbDescValidator {
        protocol: UAC_VERSION_3,
        r#type: UAC_HEADER,
        func: None,
        size: std::mem::size_of::<Uac3AcHeaderDescriptor>(),
    },
    UsbDescValidator {
        protocol: UAC_VERSION_3,
        r#type: UAC_INPUT_TERMINAL,
        func: None,
        size: std::mem::size_of::<Uac3InputTerminalDescriptor>(),
    },
    UsbDescValidator {
        protocol: UAC_VERSION_3,
        r#type: UAC_OUTPUT_TERMINAL,
        func: None,
        size: std::mem::size_of::<Uac3OutputTerminalDescriptor>(),
    },
    UsbDescValidator {
        protocol: UAC_VERSION_3,
        r#type: UAC3_MIXER_UNIT,
        func: Some(validate_mixer_unit),
        size: 0,
    },
    UsbDescValidator {
        protocol: UAC_VERSION_3,
        r#type: UAC3_SELECTOR_UNIT,
        func: Some(validate_selector_unit),
        size: 0,
    },
    UsbDescValidator {
        protocol: UAC_VERSION_3,
        r#type: UAC3_FEATURE_UNIT,
        func: Some(validate_uac3_feature_unit),
        size: 0,
    },
    UsbDescValidator {
        protocol: UAC_VERSION_3,
        r#type: UAC3_EFFECT_UNIT,
        func: None,
        size: std::mem::size_of::<Uac2EffectUnitDescriptor>(),
    },
    UsbDescValidator {
        protocol: UAC_VERSION_3,
        r#type: UAC3_PROCESSING_UNIT,
        func: Some(validate_processing_unit),
        size: 0,
    },
    UsbDescValidator {
        protocol: UAC_VERSION_3,
        r#type: UAC3_EXTENSION_UNIT,
        func: Some(validate_processing_unit),
        size: 0,
    },
    UsbDescValidator {
        protocol: UAC_VERSION_3,
        r#type: UAC3_CLOCK_SOURCE,
        func: None,
        size: std::mem::size_of::<Uac3ClockSourceDescriptor>(),
    },
    UsbDescValidator {
        protocol: UAC_VERSION_3,
        r#type: UAC3_CLOCK_SELECTOR,
        func: Some(validate_selector_unit),
        size: 0,
    },
    UsbDescValidator {
        protocol: UAC_VERSION_3,
        r#type: UAC3_CLOCK_MULTIPLIER,
        func: None,
        size: std::mem::size_of::<Uac3ClockMultiplierDescriptor>(),
    },
    UsbDescValidator {
        protocol: UAC_VERSION_3,
        r#type: UAC3_POWER_DOMAIN,
        func: Some(validate_uac3_power_domain_unit),
        size: 0,
    },
    UsbDescValidator {
        protocol: 0,
        r#type: 0,
        func: None,
        size: 0,
    },
];

static MIDI_VALIDATORS: &[UsbDescValidator] = &[
    UsbDescValidator {
        protocol: UAC_VERSION_ALL,
        r#type: USB_MS_HEADER,
        func: None,
        size: std::mem::size_of::<UsbMsHeaderDescriptor>(),
    },
    UsbDescValidator {
        protocol: UAC_VERSION_ALL,
        r#type: USB_MS_MIDI_IN_JACK,
        func: None,
        size: std::mem::size_of::<UsbMidiInJackDescriptor>(),
    },
    UsbDescValidator {
        protocol: UAC_VERSION_ALL,
        r#type: USB_MS_MIDI_OUT_JACK,
        func: Some(validate_midi_out_jack),
        size: 0,
    },
    UsbDescValidator {
        protocol: 0,
        r#type: 0,
        func: None,
        size: 0,
    },
];

fn validate_desc(hdr: *const u8, protocol: i32, mut v: *const UsbDescValidator) -> bool {
    unsafe {
        if *hdr.add(1) != USB_DT_CS_INTERFACE {
            return true;
        }

        loop {
            if (*v).r#type == 0 {
                break;
            }
            if (*v).r#type == *hdr.add(2)
                && ((*v).protocol == UAC_VERSION_ALL || (*v).protocol == protocol as u8)
            {
                if let Some(func) = (*v).func {
                    return func(hdr as *const c_void, v);
                }
                return *hdr >= (*v).size as u8;
            }
            v = v.add(1);
        }

        true
    }
}

pub fn snd_usb_validate_audio_desc(p: *mut c_void, protocol: i32) -> bool {
    unsafe {
        let c = p as *const u8;
        let mut valid = validate_desc(p as *const u8, protocol, AUDIO_VALIDATORS.as_ptr());

        if !valid && snd_usb_skip_validation {
            print_hex_dump(
                KERN_ERR.as_ptr(),
                b"USB-audio: buggy audio desc: \0".as_ptr(),
                DUMP_PREFIX_NONE,
                16,
                1,
                c,
                *c,
                true,
            );
            valid = true;
        }
        valid
    }
}

pub fn snd_usb_validate_midi_desc(p: *mut c_void) -> bool {
    unsafe {
        let c = p as *const u8;
        let mut valid = validate_desc(p as *const u8, UAC_VERSION_1 as i32, MIDI_VALIDATORS.as_ptr());

        if !valid && snd_usb_skip_validation {
            print_hex_dump(
                KERN_ERR.as_ptr(),
                b"USB-audio: buggy midi desc: \0".as_ptr(),
                DUMP_PREFIX_NONE,
                16,
                1,
                c,
                *c,
                true,
            );
            valid = true;
        }
        valid
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
