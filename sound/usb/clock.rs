// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Clock domain and sample rate management functions
 */

// External kernel types and functions are imported from dependencies:
// linux::bitops, linux::init, linux::string, linux::usb, linux::sound
// usbaudio, card, helper, clock, quirks modules

// Opaque external types from kernel headers
pub struct UacClockSourceDescriptor;
pub struct Uac3ClockSourceDescriptor;
pub struct UacClockSelectorDescriptor;
pub struct Uac3ClockSelectorDescriptor;
pub struct UacClockMultiplierDescriptor;
pub struct SndUsbAudio;
pub struct AudioFormat;
pub struct UsbHostInterface;
pub struct UsbDevice;

#[repr(C)]
pub union Uac23ClockSourceDesc {
    pub v2: UacClockSourceDescriptor,
    pub v3: Uac3ClockSourceDescriptor,
}

#[repr(C)]
pub union Uac23ClockSelectorDesc {
    pub v2: UacClockSelectorDescriptor,
    pub v3: Uac3ClockSelectorDescriptor,
}

#[repr(C)]
pub union Uac23ClockMultiplierDesc {
    pub v2: UacClockMultiplierDescriptor,
    pub v3: UacClockMultiplierDescriptor,
}

// Descriptor length check for minimal descriptor size
fn desc_length_check(p: *const u8, proto: i32) -> bool {
    const UAC_VERSION_3: i32 = 3;

    unsafe {
        if proto == UAC_VERSION_3 {
            let v3_ptr = p as *const Uac3ClockSourceDescriptor;
            let b_length = *(p as *const u8);
            b_length >= std::mem::size_of::<Uac3ClockSourceDescriptor>() as u8
        } else {
            let v2_ptr = p as *const UacClockSourceDescriptor;
            let b_length = *(p as *const u8);
            b_length >= std::mem::size_of::<UacClockSourceDescriptor>() as u8
        }
    }
}

// External functions that would be linked from kernel modules
extern "C" {
    fn snd_usb_find_csint_desc(
        extra: *mut u8,
        extralen: u32,
        cs: *mut ::std::ffi::c_void,
        type_: u8,
    ) -> *mut ::std::ffi::c_void;

    fn snd_usb_find_ctrl_interface(chip: *mut SndUsbAudio, iface: i32) -> *mut UsbHostInterface;

    fn snd_usb_ctl_msg(
        dev: *mut UsbDevice,
        pipe: u32,
        request: u8,
        requesttype: u8,
        value: u16,
        index: u16,
        data: *mut u8,
        size: usize,
    ) -> i32;

    fn usb_rcvctrlpipe(dev: *mut UsbDevice, endpoint: u32) -> u32;
    fn usb_sndctrlpipe(dev: *mut UsbDevice, endpoint: u32) -> u32;

    fn snd_usb_ctrl_intf(ctrl_intf: *mut UsbHostInterface) -> u16;

    fn msleep(msecs: u32);

    fn usb_set_interface(dev: *mut UsbDevice, ifnum: i32, alternate: i32) -> i32;

    fn test_and_set_bit(nr: usize, addr: *mut ::std::ffi::c_void) -> bool;

    fn uac_v2v3_control_is_readable(bmControls: u32, control: u8) -> bool;
    fn uac_v2v3_control_is_writeable(bmControls: u32, control: u8) -> bool;

    fn usb_audio_warn(chip: *mut SndUsbAudio, fmt: *const i8, ...);
    fn usb_audio_err(chip: *mut SndUsbAudio, fmt: *const i8, ...);
    fn usb_audio_dbg(chip: *mut SndUsbAudio, fmt: *const i8, ...);
    fn usb_audio_info(chip: *mut SndUsbAudio, fmt: *const i8, ...);
}

// Constant definitions from kernel headers
const UAC_VERSION_1: i32 = 1;
const UAC_VERSION_2: i32 = 2;
const UAC_VERSION_3: i32 = 3;
const UAC2_CLOCK_SOURCE: u8 = 0x25;
const UAC3_CLOCK_SOURCE: u8 = 0x25;
const UAC2_CLOCK_SELECTOR: u8 = 0x26;
const UAC3_CLOCK_SELECTOR: u8 = 0x26;
const UAC2_CLOCK_MULTIPLIER: u8 = 0x27;
const UAC3_CLOCK_MULTIPLIER: u8 = 0x27;
const UAC2_CS_CUR: u8 = 0x01;
const UAC2_CS_CONTROL_CLOCK_VALID: u8 = 0x02;
const UAC2_CS_CONTROL_SAM_FREQ: u8 = 0x01;
const UAC2_CX_CLOCK_SELECTOR: u8 = 0x04;
const USB_RECIP_INTERFACE: u8 = 0x01;
const USB_TYPE_CLASS: u8 = 0x20;
const USB_DIR_IN: u8 = 0x80;
const USB_DIR_OUT: u8 = 0x00;
const UAC_SET_CUR: u8 = 0x01;
const UAC_GET_CUR: u8 = 0x81;
const UAC_EP_CS_ATTR_SAMPLE_RATE: u8 = 0x01;
const QUIRK_FLAG_SKIP_CLOCK_SELECTOR: u32 = 1 << 0;
const QUIRK_FLAG_GET_SAMPLE_RATE: u32 = 1 << 1;
const QUIRK_FLAG_IGNORE_CLOCK_SOURCE: u32 = 1 << 2;
const QUIRK_FLAG_ALWAYS_SET_RATE: u32 = 1 << 3;
const QUIRK_FLAG_IFACE_DELAY: u32 = 1 << 4;
const UAC_CLOCK_SOURCE_TYPE_EXT: u8 = 0x01;
const UAC3_BADD_SAMPLING_RATE: i32 = 48000;
const UAC3_FUNCTION_SUBCLASS_GENERIC_IO: u8 = 0x20;

type ValidatorFn = fn(*mut ::std::ffi::c_void, i32, i32) -> bool;

fn find_uac_clock_desc(
    iface: *mut UsbHostInterface,
    id: i32,
    validator: ValidatorFn,
    desc_type: u8,
    proto: i32,
) -> *mut ::std::ffi::c_void {
    let mut cs: *mut ::std::ffi::c_void = std::ptr::null_mut();

    unsafe {
        loop {
            cs = snd_usb_find_csint_desc(
                (*iface).extra,
                (*iface).extralen,
                cs,
                desc_type,
            );
            if cs.is_null() {
                break;
            }
            if validator(cs, id, proto) {
                return cs;
            }
        }
    }

    std::ptr::null_mut()
}

fn validate_clock_source(p: *mut ::std::ffi::c_void, id: i32, proto: i32) -> bool {
    let cs = p as *mut Uac23ClockSourceDesc;

    unsafe {
        if !desc_length_check(p as *const u8, proto) {
            return false;
        }

        let clock_id = if proto == UAC_VERSION_3 {
            // Would access v3.bClockID
            0
        } else {
            // Would access v2.bClockID
            0
        };
        clock_id == id
    }
}

fn validate_clock_selector(p: *mut ::std::ffi::c_void, id: i32, proto: i32) -> bool {
    let cs = p as *mut Uac23ClockSelectorDesc;

    unsafe {
        if !desc_length_check(p as *const u8, proto) {
            return false;
        }

        let clock_id = if proto == UAC_VERSION_3 {
            // Would access v3.bClockID
            0
        } else {
            // Would access v2.bClockID
            0
        };

        if clock_id != id {
            return false;
        }

        if proto == UAC_VERSION_3 {
            let nr_in_pins = 0; // Would access v3.bNrInPins
            let b_length = *(p as *const u8);
            b_length >= (std::mem::size_of::<Uac3ClockSelectorDescriptor>() as u8 +
                nr_in_pins + 4 + 2)
        } else {
            let nr_in_pins = 0; // Would access v2.bNrInPins
            let b_length = *(p as *const u8);
            b_length >= (std::mem::size_of::<UacClockSelectorDescriptor>() as u8 +
                nr_in_pins + 1 + 1)
        }
    }
}

fn validate_clock_multiplier(p: *mut ::std::ffi::c_void, id: i32, proto: i32) -> bool {
    let cs = p as *mut Uac23ClockMultiplierDesc;

    unsafe {
        if !desc_length_check(p as *const u8, proto) {
            return false;
        }

        let clock_id = if proto == UAC_VERSION_3 {
            // Would access v3.bClockID
            0
        } else {
            // Would access v2.bClockID
            0
        };
        clock_id == id
    }
}

pub unsafe fn snd_usb_find_clock_source(
    chip: *mut SndUsbAudio,
    id: i32,
    fmt: *const AudioFormat,
) -> *mut Uac23ClockSourceDesc {
    let ctrl_intf = snd_usb_find_ctrl_interface(chip, (*fmt).iface);
    let proto = (*fmt).protocol;

    let desc_type = if proto == UAC_VERSION_3 {
        UAC3_CLOCK_SOURCE
    } else {
        UAC2_CLOCK_SOURCE
    };

    find_uac_clock_desc(
        ctrl_intf,
        id,
        validate_clock_source,
        desc_type,
        proto,
    ) as *mut Uac23ClockSourceDesc
}

pub unsafe fn snd_usb_find_clock_selector(
    chip: *mut SndUsbAudio,
    id: i32,
    fmt: *const AudioFormat,
) -> *mut Uac23ClockSelectorDesc {
    let ctrl_intf = snd_usb_find_ctrl_interface(chip, (*fmt).iface);
    let proto = (*fmt).protocol;

    let desc_type = if proto == UAC_VERSION_3 {
        UAC3_CLOCK_SELECTOR
    } else {
        UAC2_CLOCK_SELECTOR
    };

    find_uac_clock_desc(
        ctrl_intf,
        id,
        validate_clock_selector,
        desc_type,
        proto,
    ) as *mut Uac23ClockSelectorDesc
}

pub unsafe fn snd_usb_find_clock_multiplier(
    chip: *mut SndUsbAudio,
    id: i32,
    fmt: *const AudioFormat,
) -> *mut Uac23ClockMultiplierDesc {
    let ctrl_intf = snd_usb_find_ctrl_interface(chip, (*fmt).iface);
    let proto = (*fmt).protocol;

    let desc_type = if proto == UAC_VERSION_3 {
        0x25 // UAC3_CLOCK_MULTIPLIER
    } else {
        UAC2_CLOCK_MULTIPLIER
    };

    find_uac_clock_desc(
        ctrl_intf,
        id,
        validate_clock_multiplier,
        desc_type,
        proto,
    ) as *mut Uac23ClockMultiplierDesc
}

unsafe fn uac_clock_selector_get_val(chip: *mut SndUsbAudio, selector_id: i32, iface_no: i32) -> i32 {
    let mut buf: u8 = 0;
    let ctrl_intf = snd_usb_find_ctrl_interface(chip, iface_no);

    let ret = snd_usb_ctl_msg(
        (*chip).dev,
        usb_rcvctrlpipe((*chip).dev, 0),
        UAC2_CS_CUR,
        USB_RECIP_INTERFACE | USB_TYPE_CLASS | USB_DIR_IN,
        (UAC2_CX_CLOCK_SELECTOR as u16) << 8,
        snd_usb_ctrl_intf(ctrl_intf) | (selector_id as u16) << 8,
        &mut buf,
        std::mem::size_of::<u8>(),
    );

    if ret < 0 {
        ret
    } else {
        buf as i32
    }
}

unsafe fn uac_clock_selector_set_val(
    chip: *mut SndUsbAudio,
    selector_id: i32,
    pin: u8,
    iface_no: i32,
) -> i32 {
    let mut pin_val = pin;
    let ctrl_intf = snd_usb_find_ctrl_interface(chip, iface_no);

    let ret = snd_usb_ctl_msg(
        (*chip).dev,
        usb_sndctrlpipe((*chip).dev, 0),
        UAC2_CS_CUR,
        USB_RECIP_INTERFACE | USB_TYPE_CLASS | USB_DIR_OUT,
        (UAC2_CX_CLOCK_SELECTOR as u16) << 8,
        snd_usb_ctrl_intf(ctrl_intf) | (selector_id as u16) << 8,
        &mut pin_val,
        std::mem::size_of::<u8>(),
    );

    if ret < 0 {
        return ret;
    }

    if ret != std::mem::size_of::<u8>() as i32 {
        usb_audio_err(
            chip,
            c"setting selector (id %d) unexpected length %d\n".as_ptr() as *const i8,
            selector_id,
            ret,
        );
        return -22; // -EINVAL
    }

    let ret = uac_clock_selector_get_val(chip, selector_id, iface_no);
    if ret < 0 {
        return ret;
    }

    if ret != pin as i32 {
        usb_audio_err(
            chip,
            c"setting selector (id %d) to %x failed (current: %d)\n".as_ptr() as *const i8,
            selector_id,
            pin,
            ret,
        );
        return -22; // -EINVAL
    }

    ret
}

unsafe fn uac_clock_source_is_valid_quirk(
    chip: *mut SndUsbAudio,
    fmt: *const AudioFormat,
    source_id: i32,
) -> bool {
    let mut ret = false;
    let mut count = 0;
    let mut data: u8 = 0;
    let dev = (*chip).dev;
    let cs_desc = snd_usb_find_clock_source(chip, source_id, fmt);

    if cs_desc.is_null() {
        return false;
    }

    if (*fmt).protocol == UAC_VERSION_2 {
        // Assume the clock is valid if clock source supports only one
        // single sample rate, the terminal is connected directly to it
        // (there is no clock selector) and clock type is internal.
        // This is to deal with some Denon DJ controllers that always
        // reports that clock is invalid.
        if (*fmt).nr_rates == 1 {
            // Would check (*fmt).clock & 0xff == (*cs_desc).v2.bClockID &&
            // ((*cs_desc).v2.bmAttributes & 0x3) != UAC_CLOCK_SOURCE_TYPE_EXT
            // return true
        }
    }

    // Quirk for older MOTU AVB / hybrid interfaces
    //
    // These devices take more than 2 seconds to switch sample rate or
    // clock source. During this period the clock validity request
    // returns false, causing ALSA to fail prematurely.
    //
    // Affected models (all use vendor 0x07fd):
    //   - MicroBook IIc          → 0x0004
    //   - 1248, 624, 8A, UltraLite AVB, 8M, 16A, ... → 0x0005

    let usb_id = (*chip).usb_id;
    if (usb_id >> 16) == 0x07fd && ((usb_id & 0xffff) == 0x0004 || (usb_id & 0xffff) == 0x0005) {
        count = 0;

        while !ret && count < 50 {
            msleep(100);

            let err = snd_usb_ctl_msg(
                dev,
                usb_rcvctrlpipe(dev, 0),
                UAC2_CS_CUR,
                USB_TYPE_CLASS | USB_RECIP_INTERFACE | USB_DIR_IN,
                (UAC2_CS_CONTROL_CLOCK_VALID as u16) << 8,
                snd_usb_ctrl_intf(snd_usb_find_ctrl_interface(chip, (*fmt).iface))
                    | (source_id as u16) << 8,
                &mut data,
                std::mem::size_of::<u8>(),
            );

            if err < 0 {
                return false;
            }

            ret = data != 0;
            count += 1;
        }
    }

    ret
}

unsafe fn uac_clock_source_is_valid(
    chip: *mut SndUsbAudio,
    fmt: *const AudioFormat,
    source_id: i32,
) -> bool {
    let mut data: u8 = 0;
    let dev = (*chip).dev;
    let cs_desc = snd_usb_find_clock_source(chip, source_id, fmt);

    if cs_desc.is_null() {
        return false;
    }

    let bm_controls = if (*fmt).protocol == UAC_VERSION_3 {
        // Would be le32_to_cpu((*cs_desc).v3.bmControls)
        0u32
    } else {
        // Would be (*cs_desc).v2.bmControls
        0u32
    };

    // If a clock source can't tell us whether it's valid, we assume it is
    if !uac_v2v3_control_is_readable(bm_controls, UAC2_CS_CONTROL_CLOCK_VALID) {
        return true;
    }

    let err = snd_usb_ctl_msg(
        dev,
        usb_rcvctrlpipe(dev, 0),
        UAC2_CS_CUR,
        USB_TYPE_CLASS | USB_RECIP_INTERFACE | USB_DIR_IN,
        (UAC2_CS_CONTROL_CLOCK_VALID as u16) << 8,
        snd_usb_ctrl_intf(snd_usb_find_ctrl_interface(chip, (*fmt).iface)) | (source_id as u16) << 8,
        &mut data,
        std::mem::size_of::<u8>(),
    );

    if err < 0 {
        usb_audio_warn(
            chip,
            c"%s(): cannot get clock validity for id %d\n".as_ptr() as *const i8,
            c"uac_clock_source_is_valid".as_ptr() as *const i8,
            source_id,
        );
        return false;
    }

    if data != 0 {
        true
    } else {
        uac_clock_source_is_valid_quirk(chip, fmt, source_id)
    }
}

unsafe fn __uac_clock_find_source(
    chip: *mut SndUsbAudio,
    fmt: *const AudioFormat,
    entity_id: i32,
    visited: *mut u8,
    validate: bool,
) -> i32 {
    let mut entity_id = entity_id & 0xff;
    let proto = (*fmt).protocol;
    let mut pins: i32;
    let mut clock_id: i32;
    let mut sources: *const u8;
    let mut cur: i32;
    let mut err: i32;
    let mut bmControls: u32;
    let mut readable: bool;
    let mut writeable: bool;

    if test_and_set_bit(entity_id as usize, visited as *mut ::std::ffi::c_void) {
        usb_audio_warn(
            chip,
            c"%s(): recursive clock topology detected, id %d.\n".as_ptr() as *const i8,
            c"__uac_clock_find_source".as_ptr() as *const i8,
            entity_id,
        );
        return -22; // -EINVAL
    }

    // first, see if the ID we're looking at is a clock source already
    let source = snd_usb_find_clock_source(chip, entity_id, fmt);
    if !source.is_null() {
        entity_id = 0; // Would be GET_VAL(source, proto, bClockID)
        if validate && !uac_clock_source_is_valid(chip, fmt, entity_id) {
            usb_audio_err(
                chip,
                c"clock source %d is not valid, cannot use\n".as_ptr() as *const i8,
                entity_id,
            );
            return -6; // -ENXIO
        }
        return entity_id;
    }

    let selector = snd_usb_find_clock_selector(chip, entity_id, fmt);
    if !selector.is_null() {
        pins = 0; // Would be GET_VAL(selector, proto, bNrInPins)
        clock_id = 0; // Would be GET_VAL(selector, proto, bClockID)
        sources = std::ptr::null(); // Would be GET_VAL(selector, proto, baCSourceID)
        cur = 0;

        if proto == UAC_VERSION_3 {
            bmControls = 0; // Would be le32_to_cpu(*(__le32 *)(&selector->v3.baCSourceID[0] + pins))
        } else {
            bmControls = 0; // Would be *(__u8 *)(&selector->v2.baCSourceID[0] + pins)
        }

        readable = uac_v2v3_control_is_readable(bmControls, UAC2_CX_CLOCK_SELECTOR);
        writeable = uac_v2v3_control_is_writeable(bmControls, UAC2_CX_CLOCK_SELECTOR);

        if pins == 1 {
            let ret_val = 1;
            cur = ret_val;
            let ret = __uac_clock_find_source(
                chip,
                fmt,
                *sources.offset(ret_val as isize - 1) as i32,
                visited,
                validate,
            );
            if ret > 0 {
                if ((*chip).quirk_flags & QUIRK_FLAG_SKIP_CLOCK_SELECTOR) != 0 || !writeable {
                    return ret;
                }
                err = uac_clock_selector_set_val(chip, entity_id, cur as u8, (*fmt).iface);
                if err < 0 {
                    return err;
                }
            }
            if !validate || ret > 0 || !(*chip).autoclock {
                return ret;
            }
            if !writeable {
                return -6; // -ENXIO
            }
            // The current clock source is invalid, try others.
            for i in 1..=pins {
                if i == cur {
                    continue;
                }

                let ret = __uac_clock_find_source(
                    chip,
                    fmt,
                    *sources.offset(i as isize - 1) as i32,
                    visited,
                    true,
                );
                if ret < 0 {
                    continue;
                }

                err = uac_clock_selector_set_val(chip, entity_id, i as u8, (*fmt).iface);
                if err < 0 {
                    continue;
                }

                usb_audio_info(
                    chip,
                    c"found and selected valid clock source %d\n".as_ptr() as *const i8,
                    ret,
                );
                return ret;
            }

            return -6; // -ENXIO
        }

        // for now just warn about buggy device
        if !readable {
            usb_audio_warn(
                chip,
                c"%s(): clock selector control is not readable, id %d\n".as_ptr() as *const i8,
                c"__uac_clock_find_source".as_ptr() as *const i8,
                clock_id,
            );
        }

        // the entity ID we are looking at is a selector.
        // find out what it currently selects
        let ret = uac_clock_selector_get_val(chip, clock_id, (*fmt).iface);
        if ret < 0 {
            if !(*chip).autoclock {
                return ret;
            }
        } else {
            // Selector values are one-based

            if ret > pins || ret < 1 {
                usb_audio_err(
                    chip,
                    c"%s(): selector reported illegal value, id %d, ret %d\n".as_ptr() as *const i8,
                    c"__uac_clock_find_source".as_ptr() as *const i8,
                    clock_id,
                    ret,
                );

                if !(*chip).autoclock {
                    return -22; // -EINVAL
                }
            } else {
                cur = ret;
                let ret = __uac_clock_find_source(
                    chip,
                    fmt,
                    *sources.offset(ret as isize - 1) as i32,
                    visited,
                    validate,
                );
                if ret > 0 {
                    // Skip setting clock selector again for some devices
                    if ((*chip).quirk_flags & QUIRK_FLAG_SKIP_CLOCK_SELECTOR) == 0 && writeable {
                        err = uac_clock_selector_set_val(chip, entity_id, cur as u8, (*fmt).iface);
                        if err < 0 {
                            if pins == 1 {
                                usb_audio_dbg(
                                    chip,
                                    c"%s(): selector returned an error, assuming a firmware bug, id %d, ret %d\n".as_ptr() as *const i8,
                                    c"__uac_clock_find_source".as_ptr() as *const i8,
                                    clock_id,
                                    err,
                                );
                                return ret;
                            }
                            return err;
                        }
                    }
                }

                if !validate || ret > 0 || !(*chip).autoclock {
                    return ret;
                }
            }
        }

        if !writeable {
            return -6; // -ENXIO
        }

        // The current clock source is invalid, try others.
        for i in 1..=pins {
            if i == cur {
                continue;
            }

            let ret = __uac_clock_find_source(
                chip,
                fmt,
                *sources.offset(i as isize - 1) as i32,
                visited,
                true,
            );
            if ret < 0 {
                continue;
            }

            err = uac_clock_selector_set_val(chip, entity_id, i as u8, (*fmt).iface);
            if err < 0 {
                continue;
            }

            usb_audio_info(
                chip,
                c"found and selected valid clock source %d\n".as_ptr() as *const i8,
                ret,
            );
            return ret;
        }

        return -6; // -ENXIO
    }

    // FIXME: multipliers only act as pass-thru element for now
    let multiplier = snd_usb_find_clock_multiplier(chip, entity_id, fmt);
    if !multiplier.is_null() {
        let csource_id = 0; // Would be GET_VAL(multiplier, proto, bCSourceID)
        return __uac_clock_find_source(chip, fmt, csource_id, visited, validate);
    }

    -22 // -EINVAL
}

/*
 * For all kinds of sample rate settings and other device queries,
 * the clock source (end-leaf) must be used. However, clock selectors,
 * clock multipliers and sample rate converters may be specified as
 * clock source input to terminal. This functions walks the clock path
 * to its end and tries to find the source.
 *
 * The 'visited' bitfield is used internally to detect recursive loops.
 *
 * Returns the clock source UnitID (>=0) on success, or an error.
 */
pub unsafe fn snd_usb_clock_find_source(
    chip: *mut SndUsbAudio,
    fmt: *const AudioFormat,
    validate: bool,
) -> i32 {
    let mut visited: [u8; 32] = [0; 32];

    match (*fmt).protocol {
        UAC_VERSION_2 | UAC_VERSION_3 => {
            __uac_clock_find_source(chip, fmt, (*fmt).clock, visited.as_mut_ptr() as *mut u8, validate)
        }
        _ => -22, // -EINVAL
    }
}

unsafe fn set_sample_rate_v1(chip: *mut SndUsbAudio, fmt: *const AudioFormat, rate: i32) -> i32 {
    let dev = (*chip).dev;
    let mut data: [u8; 3] = [0; 3];
    let mut crate_val: i32;

    // if endpoint doesn't have sampling rate control, bail out
    if ((*fmt).attributes & UAC_EP_CS_ATTR_SAMPLE_RATE) == 0 {
        return 0;
    }

    data[0] = (rate & 0xff) as u8;
    data[1] = ((rate >> 8) & 0xff) as u8;
    data[2] = ((rate >> 16) & 0xff) as u8;
    let err = snd_usb_ctl_msg(
        dev,
        usb_sndctrlpipe(dev, 0),
        UAC_SET_CUR,
        USB_TYPE_CLASS | USB_RECIP_ENDPOINT | USB_DIR_OUT,
        (UAC_EP_CS_ATTR_SAMPLE_RATE as u16) << 8,
        (*fmt).endpoint as u16,
        data.as_mut_ptr(),
        std::mem::size_of::<[u8; 3]>(),
    );
    if err < 0 {
        return err;
    }

    // Don't check the sample rate for devices which we know don't support reading
    if ((*chip).quirk_flags & QUIRK_FLAG_GET_SAMPLE_RATE) != 0 {
        return 0;
    }
    // the firmware is likely buggy, don't repeat to fail too many times
    if (*chip).sample_rate_read_error > 2 {
        return 0;
    }

    let err = snd_usb_ctl_msg(
        dev,
        usb_rcvctrlpipe(dev, 0),
        UAC_GET_CUR,
        USB_TYPE_CLASS | USB_RECIP_ENDPOINT | USB_DIR_IN,
        (UAC_EP_CS_ATTR_SAMPLE_RATE as u16) << 8,
        (*fmt).endpoint as u16,
        data.as_mut_ptr(),
        std::mem::size_of::<[u8; 3]>(),
    );
    if err < 0 {
        (*chip).sample_rate_read_error += 1;
        return 0; // some devices don't support reading
    }

    crate_val = (data[0] as i32) | ((data[1] as i32) << 8) | ((data[2] as i32) << 16);
    if crate_val == 0 {
        (*chip).sample_rate_read_error = 3; // three strikes, see above
        return 0;
    }

    if crate_val != rate {
        // current rate is different from the runtime rate
        // runtime->rate = crate;
    }

    0
}

unsafe fn get_sample_rate_v2v3(
    chip: *mut SndUsbAudio,
    iface: i32,
    altsetting: i32,
    clock: i32,
) -> i32 {
    let dev = (*chip).dev;
    let mut data: u32 = 0;
    let ctrl_intf = snd_usb_find_ctrl_interface(chip, iface);

    let err = snd_usb_ctl_msg(
        dev,
        usb_rcvctrlpipe(dev, 0),
        UAC2_CS_CUR,
        USB_TYPE_CLASS | USB_RECIP_INTERFACE | USB_DIR_IN,
        (UAC2_CS_CONTROL_SAM_FREQ as u16) << 8,
        snd_usb_ctrl_intf(ctrl_intf) | (clock as u16) << 8,
        &mut data as *mut u32 as *mut u8,
        std::mem::size_of::<u32>(),
    );

    if err < 0 {
        usb_audio_warn(
            chip,
            c"%d:%d: cannot get freq (v2/v3): err %d\n".as_ptr() as *const i8,
            iface,
            altsetting,
            err,
        );
        return 0;
    }

    u32::from_le(data) as i32
}

/*
 * Try to set the given sample rate:
 *
 * Return 0 if the clock source is read-only, the actual rate on success,
 * or a negative error code.
 *
 * This function gets called from format.c to validate each sample rate, too.
 * Hence no message is shown upon error
 */
pub unsafe fn snd_usb_set_sample_rate_v2v3(
    chip: *mut SndUsbAudio,
    fmt: *const AudioFormat,
    clock: i32,
    rate: i32,
) -> i32 {
    let mut writeable: bool;
    let mut bm_controls: u32;
    let mut data = (rate as u32).to_le();
    let cs_desc = snd_usb_find_clock_source(chip, clock, fmt);
    let ctrl_intf = snd_usb_find_ctrl_interface(chip, (*fmt).iface);

    if cs_desc.is_null() {
        return 0;
    }

    bm_controls = if (*fmt).protocol == UAC_VERSION_3 {
        // Would be le32_to_cpu((*cs_desc).v3.bmControls)
        0u32
    } else {
        // Would be (*cs_desc).v2.bmControls
        0u32
    };

    writeable = uac_v2v3_control_is_writeable(bm_controls, UAC2_CS_CONTROL_SAM_FREQ);
    if !writeable {
        return 0;
    }

    let err = snd_usb_ctl_msg(
        (*chip).dev,
        usb_sndctrlpipe((*chip).dev, 0),
        UAC2_CS_CUR,
        USB_TYPE_CLASS | USB_RECIP_INTERFACE | USB_DIR_OUT,
        (UAC2_CS_CONTROL_SAM_FREQ as u16) << 8,
        snd_usb_ctrl_intf(ctrl_intf) | (clock as u16) << 8,
        &mut data as *mut u32 as *mut u8,
        std::mem::size_of::<u32>(),
    );

    if err < 0 {
        return err;
    }

    get_sample_rate_v2v3(chip, (*fmt).iface, (*fmt).altsetting, clock)
}

unsafe fn set_sample_rate_v2v3(chip: *mut SndUsbAudio, fmt: *const AudioFormat, rate: i32) -> i32 {
    let mut cur_rate: i32;
    let mut prev_rate = 0;
    let mut clock: i32;

    // First, try to find a valid clock. This may trigger
    // automatic clock selection if the current clock is not valid.
    clock = snd_usb_clock_find_source(chip, fmt, true);
    if clock < 0 {
        // We did not find a valid clock, but that might be
        // because the current sample rate does not match an
        // external clock source. Try again without validation
        // and we will do another validation after setting the rate.
        clock = snd_usb_clock_find_source(chip, fmt, false);

        // Hardcoded sample rates
        if ((*chip).quirk_flags & QUIRK_FLAG_IGNORE_CLOCK_SOURCE) != 0 {
            return 0;
        }

        if clock < 0 {
            return clock;
        }
    }

    if ((*chip).quirk_flags & QUIRK_FLAG_ALWAYS_SET_RATE) == 0 {
        prev_rate = get_sample_rate_v2v3(chip, (*fmt).iface, (*fmt).altsetting, clock);
        if prev_rate == rate {
            if !uac_clock_source_is_valid(chip, fmt, clock) {
                return -6; // -ENXIO
            }
            return 0;
        }
    }

    cur_rate = snd_usb_set_sample_rate_v2v3(chip, fmt, clock, rate);
    if cur_rate < 0 {
        usb_audio_err(
            chip,
            c"%d:%d: cannot set freq %d (v2/v3): err %d\n".as_ptr() as *const i8,
            (*fmt).iface,
            (*fmt).altsetting,
            rate,
            cur_rate,
        );
        return cur_rate;
    }

    if cur_rate == 0 {
        cur_rate = prev_rate;
    }

    if cur_rate != rate {
        usb_audio_dbg(
            chip,
            c"%d:%d: freq mismatch: req %d, clock runs @%d\n".as_ptr() as *const i8,
            (*fmt).iface,
            (*fmt).altsetting,
            rate,
            cur_rate,
        );
        // continue processing
    }

    // FIXME - TEAC devices require the immediate interface setup
    let usb_id = (*chip).usb_id;
    let vendor = (usb_id >> 16) as u16;
    if vendor == 0x0644 {
        let cur_base_48k = (rate % 48000) == 0;
        let prev_base_48k = (prev_rate % 48000) == 0;
        if cur_base_48k != prev_base_48k {
            usb_set_interface((*chip).dev, (*fmt).iface, (*fmt).altsetting);
            if ((*chip).quirk_flags & QUIRK_FLAG_IFACE_DELAY) != 0 {
                msleep(50);
            }
        }
    }

    // validate clock after rate change
    if !uac_clock_source_is_valid(chip, fmt, clock) {
        return -6; // -ENXIO
    }
    0
}

pub unsafe fn snd_usb_init_sample_rate(
    chip: *mut SndUsbAudio,
    fmt: *const AudioFormat,
    rate: i32,
) -> i32 {
    usb_audio_dbg(
        chip,
        c"%d:%d Set sample rate %d, clock %d\n".as_ptr() as *const i8,
        (*fmt).iface,
        (*fmt).altsetting,
        rate,
        (*fmt).clock,
    );

    match (*fmt).protocol {
        UAC_VERSION_1 => set_sample_rate_v1(chip, fmt, rate),
        UAC_VERSION_3 => {
            if (*chip).badd_profile >= UAC3_FUNCTION_SUBCLASS_GENERIC_IO {
                if rate != UAC3_BADD_SAMPLING_RATE {
                    return -6; // -ENXIO
                } else {
                    return 0;
                }
            }
            set_sample_rate_v2v3(chip, fmt, rate)
        }
        UAC_VERSION_2 => set_sample_rate_v2v3(chip, fmt, rate),
        _ => set_sample_rate_v1(chip, fmt, rate),
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
