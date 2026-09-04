// SPDX-License-Identifier: GPL-2.0+
// virtio-snd: Virtio sound device
// Copyright (C) 2022 OpenSynergy GmbH

// Includes from C:
// #include <sound/control.h>
// #include <linux/virtio_config.h>
// #include "virtio_card.h"

use core::mem;
use core::ptr;

// External types and constants from ALSA and VirtIO headers
// These would be defined in their respective crate/module imports
// snd_ctl_elem_type_t, SNDRV_CTL_ELEM_TYPE_*, VIRTIO_SND_CTL_TYPE_*,
// SNDRV_CTL_ELEM_ACCESS_*, VIRTIO_SND_CTL_ACCESS_*, etc.

// Map for converting VirtIO types to ALSA types.
static G_V2A_TYPE_MAP: &[u32] = &[
    SNDRV_CTL_ELEM_TYPE_BOOLEAN,
    SNDRV_CTL_ELEM_TYPE_INTEGER,
    SNDRV_CTL_ELEM_TYPE_INTEGER64,
    SNDRV_CTL_ELEM_TYPE_ENUMERATED,
    SNDRV_CTL_ELEM_TYPE_BYTES,
    SNDRV_CTL_ELEM_TYPE_IEC958,
];

// Map for converting VirtIO types to maximum value counts.
// Values computed from ARRAY_SIZE(((struct virtio_snd_ctl_value *)0)->value.XXX)
static G_V2A_COUNT_MAP: &[u32] = &[
    // VIRTIO_SND_CTL_TYPE_BOOLEAN: size of integer array
    128,
    // VIRTIO_SND_CTL_TYPE_INTEGER: size of integer array
    128,
    // VIRTIO_SND_CTL_TYPE_INTEGER64: size of integer64 array
    64,
    // VIRTIO_SND_CTL_TYPE_ENUMERATED: size of enumerated array
    128,
    // VIRTIO_SND_CTL_TYPE_BYTES: size of bytes array
    512,
    // VIRTIO_SND_CTL_TYPE_IEC958
    1,
];

// Map for converting VirtIO access rights to ALSA access rights.
static G_V2A_ACCESS_MAP: &[u32] = &[
    SNDRV_CTL_ELEM_ACCESS_READ,
    SNDRV_CTL_ELEM_ACCESS_WRITE,
    SNDRV_CTL_ELEM_ACCESS_VOLATILE,
    SNDRV_CTL_ELEM_ACCESS_INACTIVE,
    SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    SNDRV_CTL_ELEM_ACCESS_TLV_WRITE,
    SNDRV_CTL_ELEM_ACCESS_TLV_COMMAND,
];

// Map for converting VirtIO event masks to ALSA event masks.
static G_V2A_MASK_MAP: &[u32] = &[
    SNDRV_CTL_EVENT_MASK_VALUE,
    SNDRV_CTL_EVENT_MASK_INFO,
    SNDRV_CTL_EVENT_MASK_TLV,
];

fn virtsnd_kctl_validate_info(
    snd: *mut virtio_snd,
    cid: u32,
    kinfo: *mut virtio_snd_ctl_info,
) -> i32 {
    unsafe {
        let vdev = (*snd).vdev;
        let type_ = u32::from_le((*kinfo).type_);
        let count = u32::from_le((*kinfo).count);

        if type_ >= G_V2A_TYPE_MAP.len() as u32 {
            dev_err(
                &(*vdev).dev,
                b"control #%u: unknown type %u\n\0".as_ptr(),
                cid,
                type_,
            );
            return -22; // -EINVAL
        }

        if count > G_V2A_COUNT_MAP[type_ as usize]
            || (type_ == VIRTIO_SND_CTL_TYPE_IEC958 && count != 1)
        {
            dev_err(
                &(*vdev).dev,
                b"control #%u: invalid count %u for type %u\n\0".as_ptr(),
                cid,
                count,
                type_,
            );
            return -22; // -EINVAL
        }

        if type_ == VIRTIO_SND_CTL_TYPE_ENUMERATED
            && u32::from_le((*kinfo).value.enumerated.items) == 0
        {
            dev_err(
                &(*vdev).dev,
                b"control #%u: no items for enumerated control\n\0".as_ptr(),
                cid,
            );
            return -22; // -EINVAL
        }

        0
    }
}

/// virtsnd_kctl_info() - Returns information about the control.
/// @kcontrol: ALSA control element.
/// @uinfo: Element information.
///
/// Context: Process context.
/// Return: 0 on success, -errno on failure.
fn virtsnd_kctl_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    unsafe {
        let snd = snd_kcontrol_chip(kcontrol);
        let private_value = (*kcontrol).private_value;
        let kctl = &mut (*snd).kctls[private_value as usize];
        let kinfo = &(*snd).kctl_infos[private_value as usize];

        (*uinfo).type_ = G_V2A_TYPE_MAP[u32::from_le(kinfo.type_) as usize];
        (*uinfo).count = u32::from_le(kinfo.count);

        match (*uinfo).type_ {
            SNDRV_CTL_ELEM_TYPE_INTEGER => {
                (*uinfo).value.integer.min = i32::from_le(kinfo.value.integer.min);
                (*uinfo).value.integer.max = i32::from_le(kinfo.value.integer.max);
                (*uinfo).value.integer.step = u32::from_le(kinfo.value.integer.step);
            }
            SNDRV_CTL_ELEM_TYPE_INTEGER64 => {
                (*uinfo).value.integer64.min = i64::from_le(kinfo.value.integer64.min);
                (*uinfo).value.integer64.max = i64::from_le(kinfo.value.integer64.max);
                (*uinfo).value.integer64.step = u64::from_le(kinfo.value.integer64.step);
            }
            SNDRV_CTL_ELEM_TYPE_ENUMERATED => {
                (*uinfo).value.enumerated.items = u32::from_le(kinfo.value.enumerated.items);
                let i = (*uinfo).value.enumerated.item;
                if i >= (*uinfo).value.enumerated.items {
                    return -22; // -EINVAL
                }

                strscpy(
                    (*uinfo).value.enumerated.name.as_mut_ptr(),
                    kctl.items[i as usize].item.as_ptr(),
                    mem::size_of_val(&(*uinfo).value.enumerated.name),
                );
            }
            _ => {}
        }

        0
    }
}

/// virtsnd_kctl_get() - Read the value from the control.
/// @kcontrol: ALSA control element.
/// @uvalue: Element value.
///
/// Context: Process context.
/// Return: 0 on success, -errno on failure.
fn virtsnd_kctl_get(
    kcontrol: *mut snd_kcontrol,
    uvalue: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let snd = snd_kcontrol_chip(kcontrol);
        let private_value = (*kcontrol).private_value;
        let kinfo = &(*snd).kctl_infos[private_value as usize];
        let type_ = u32::from_le(kinfo.type_);
        let count = u32::from_le(kinfo.count);

        let request_size = mem::size_of::<virtio_snd_ctl_hdr>();
        let response_size = mem::size_of::<virtio_snd_hdr>() + mem::size_of::<virtio_snd_ctl_value>();

        let msg = virtsnd_ctl_msg_alloc(request_size, response_size, GFP_KERNEL);
        if msg.is_null() {
            return -12; // -ENOMEM
        }

        virtsnd_ctl_msg_ref(msg);

        let hdr = virtsnd_ctl_msg_request(msg) as *mut virtio_snd_ctl_hdr;
        (*hdr).hdr.code = u32::to_le(VIRTIO_SND_R_CTL_READ);
        (*hdr).control_id = u32::to_le(private_value as u32);

        let mut rc = virtsnd_ctl_msg_send_sync(snd, msg);
        if rc != 0 {
            virtsnd_ctl_msg_unref(msg);
            return rc;
        }

        let kvalue = (virtsnd_ctl_msg_response(msg) as *mut u8)
            .add(mem::size_of::<virtio_snd_hdr>()) as *mut virtio_snd_ctl_value;

        match type_ {
            VIRTIO_SND_CTL_TYPE_BOOLEAN | VIRTIO_SND_CTL_TYPE_INTEGER => {
                for i in 0..count {
                    (*uvalue).value.integer.value[i as usize] =
                        i32::from_le((*kvalue).value.integer[i as usize]);
                }
            }
            VIRTIO_SND_CTL_TYPE_INTEGER64 => {
                for i in 0..count {
                    (*uvalue).value.integer64.value[i as usize] =
                        i64::from_le((*kvalue).value.integer64[i as usize]);
                }
            }
            VIRTIO_SND_CTL_TYPE_ENUMERATED => {
                for i in 0..count {
                    (*uvalue).value.enumerated.item[i as usize] =
                        u32::from_le((*kvalue).value.enumerated[i as usize]);
                }
            }
            VIRTIO_SND_CTL_TYPE_BYTES => {
                ptr::copy_nonoverlapping(
                    (*kvalue).value.bytes.as_ptr(),
                    (*uvalue).value.bytes.data.as_mut_ptr(),
                    count as usize,
                );
            }
            VIRTIO_SND_CTL_TYPE_IEC958 => {
                ptr::copy_nonoverlapping(
                    &(*kvalue).value.iec958 as *const _ as *const u8,
                    &mut (*uvalue).value.iec958 as *mut _ as *mut u8,
                    mem::size_of_val(&(*uvalue).value.iec958),
                );
            }
            _ => {}
        }

        virtsnd_ctl_msg_unref(msg);
        rc
    }
}

/// virtsnd_kctl_put() - Write the value to the control.
/// @kcontrol: ALSA control element.
/// @uvalue: Element value.
///
/// Context: Process context.
/// Return: 0 on success, -errno on failure.
fn virtsnd_kctl_put(
    kcontrol: *mut snd_kcontrol,
    uvalue: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let snd = snd_kcontrol_chip(kcontrol);
        let private_value = (*kcontrol).private_value;
        let kinfo = &(*snd).kctl_infos[private_value as usize];
        let type_ = u32::from_le(kinfo.type_);
        let count = u32::from_le(kinfo.count);

        let request_size = mem::size_of::<virtio_snd_ctl_hdr>() + mem::size_of::<virtio_snd_ctl_value>();
        let response_size = mem::size_of::<virtio_snd_hdr>();

        let msg = virtsnd_ctl_msg_alloc(request_size, response_size, GFP_KERNEL);
        if msg.is_null() {
            return -12; // -ENOMEM
        }

        let hdr = virtsnd_ctl_msg_request(msg) as *mut virtio_snd_ctl_hdr;
        (*hdr).hdr.code = u32::to_le(VIRTIO_SND_R_CTL_WRITE);
        (*hdr).control_id = u32::to_le(private_value as u32);

        let kvalue = (hdr as *mut u8).add(mem::size_of::<virtio_snd_ctl_hdr>())
            as *mut virtio_snd_ctl_value;

        match type_ {
            VIRTIO_SND_CTL_TYPE_BOOLEAN | VIRTIO_SND_CTL_TYPE_INTEGER => {
                for i in 0..count {
                    (*kvalue).value.integer[i as usize] =
                        i32::to_le((*uvalue).value.integer.value[i as usize]);
                }
            }
            VIRTIO_SND_CTL_TYPE_INTEGER64 => {
                for i in 0..count {
                    (*kvalue).value.integer64[i as usize] =
                        i64::to_le((*uvalue).value.integer64.value[i as usize]);
                }
            }
            VIRTIO_SND_CTL_TYPE_ENUMERATED => {
                for i in 0..count {
                    (*kvalue).value.enumerated[i as usize] =
                        u32::to_le((*uvalue).value.enumerated.item[i as usize]);
                }
            }
            VIRTIO_SND_CTL_TYPE_BYTES => {
                ptr::copy_nonoverlapping(
                    (*uvalue).value.bytes.data.as_ptr(),
                    (*kvalue).value.bytes.as_mut_ptr(),
                    count as usize,
                );
            }
            VIRTIO_SND_CTL_TYPE_IEC958 => {
                ptr::copy_nonoverlapping(
                    &(*uvalue).value.iec958 as *const _ as *const u8,
                    &mut (*kvalue).value.iec958 as *mut _ as *mut u8,
                    mem::size_of_val(&(*kvalue).value.iec958),
                );
            }
            _ => {}
        }

        virtsnd_ctl_msg_send_sync(snd, msg)
    }
}

/// virtsnd_kctl_tlv_op() - Perform an operation on the control's metadata.
/// @kcontrol: ALSA control element.
/// @op_flag: Operation code (SNDRV_CTL_TLV_OP_XXX).
/// @size: Size of the TLV data in bytes.
/// @utlv: TLV data.
///
/// Context: Process context.
/// Return: 0 on success, -errno on failure.
fn virtsnd_kctl_tlv_op(
    kcontrol: *mut snd_kcontrol,
    op_flag: i32,
    size: u32,
    utlv: *mut u32,
) -> i32 {
    unsafe {
        let snd = snd_kcontrol_chip(kcontrol);
        let msg = virtsnd_ctl_msg_alloc(
            mem::size_of::<virtio_snd_ctl_hdr>(),
            mem::size_of::<virtio_snd_hdr>(),
            GFP_KERNEL,
        );
        if msg.is_null() {
            return -12; // -ENOMEM
        }

        let tlv = kzalloc(size as usize, GFP_KERNEL) as *mut u32;
        if tlv.is_null() {
            virtsnd_ctl_msg_unref(msg);
            return -12; // -ENOMEM
        }

        let mut sg: scatterlist = mem::zeroed();
        sg_init_one(&mut sg, tlv as *const u8, size as usize);

        let hdr = virtsnd_ctl_msg_request(msg) as *mut virtio_snd_ctl_hdr;
        (*hdr).control_id = u32::to_le((*kcontrol).private_value as u32);

        let rc = match op_flag {
            SNDRV_CTL_TLV_OP_READ => {
                (*hdr).hdr.code = u32::to_le(VIRTIO_SND_R_CTL_TLV_READ);

                let result = virtsnd_ctl_msg_send(snd, msg, ptr::null_mut(), &mut sg, false);
                if result == 0 {
                    if copy_to_user(utlv as *mut u8, tlv as *const u8, size as usize) != 0 {
                        -14 // -EFAULT
                    } else {
                        0
                    }
                } else {
                    result
                }
            }
            SNDRV_CTL_TLV_OP_WRITE | SNDRV_CTL_TLV_OP_CMD => {
                if op_flag == SNDRV_CTL_TLV_OP_WRITE {
                    (*hdr).hdr.code = u32::to_le(VIRTIO_SND_R_CTL_TLV_WRITE);
                } else {
                    (*hdr).hdr.code = u32::to_le(VIRTIO_SND_R_CTL_TLV_COMMAND);
                }

                if copy_from_user(tlv as *mut u8, utlv as *const u8, size as usize) != 0 {
                    -14 // -EFAULT
                } else {
                    virtsnd_ctl_msg_send(snd, msg, &mut sg, ptr::null_mut(), false)
                }
            }
            _ => {
                -22 // -EINVAL
            }
        };

        kfree(tlv as *mut u8);
        if rc != 0 {
            virtsnd_ctl_msg_unref(msg);
        }

        rc
    }
}

/// virtsnd_kctl_get_enum_items() - Query items for the ENUMERATED element type.
/// @snd: VirtIO sound device.
/// @cid: Control element ID.
///
/// This function is called during initial device initialization.
///
/// Context: Any context that permits to sleep.
/// Return: 0 on success, -errno on failure.
fn virtsnd_kctl_get_enum_items(snd: *mut virtio_snd, cid: u32) -> i32 {
    unsafe {
        let vdev = (*snd).vdev;
        let kinfo = &(*snd).kctl_infos[cid as usize];
        let kctl = &mut (*snd).kctls[cid as usize];
        let n = u32::from_le(kinfo.value.enumerated.items) as usize;

        let msg = virtsnd_ctl_msg_alloc(
            mem::size_of::<virtio_snd_ctl_hdr>(),
            mem::size_of::<virtio_snd_hdr>(),
            GFP_KERNEL,
        );
        if msg.is_null() {
            return -12; // -ENOMEM
        }

        let items_ptr = devm_kcalloc(&(*vdev).dev, n, mem::size_of::<virtio_snd_ctl_enum_item>(), GFP_KERNEL);
        if items_ptr.is_null() {
            virtsnd_ctl_msg_unref(msg);
            return -12; // -ENOMEM
        }

        kctl.items = items_ptr as *mut virtio_snd_ctl_enum_item;

        let mut sg: scatterlist = mem::zeroed();
        sg_init_one(&mut sg, kctl.items as *const u8, n * mem::size_of::<virtio_snd_ctl_enum_item>());

        let hdr = virtsnd_ctl_msg_request(msg) as *mut virtio_snd_ctl_hdr;
        (*hdr).hdr.code = u32::to_le(VIRTIO_SND_R_CTL_ENUM_ITEMS);
        (*hdr).control_id = u32::to_le(cid);

        virtsnd_ctl_msg_send(snd, msg, ptr::null_mut(), &mut sg, false)
    }
}

/// virtsnd_kctl_parse_cfg() - Parse the control element configuration.
/// @snd: VirtIO sound device.
///
/// This function is called during initial device initialization.
///
/// Context: Any context that permits to sleep.
/// Return: 0 on success, -errno on failure.
pub fn virtsnd_kctl_parse_cfg(snd: *mut virtio_snd) -> i32 {
    unsafe {
        let vdev = (*snd).vdev;

        virtio_cread_le(
            vdev,
            &mut (*snd).nkctls,
            mem::offset_of!(virtio_snd_config, controls),
        );

        if (*snd).nkctls == 0 {
            return 0;
        }

        let kctl_infos_ptr = devm_kcalloc(
            &(*vdev).dev,
            (*snd).nkctls as usize,
            mem::size_of::<virtio_snd_ctl_info>(),
            GFP_KERNEL,
        );
        if kctl_infos_ptr.is_null() {
            return -12; // -ENOMEM
        }
        (*snd).kctl_infos = kctl_infos_ptr as *mut virtio_snd_ctl_info;

        let kctls_ptr = devm_kcalloc(
            &(*vdev).dev,
            (*snd).nkctls as usize,
            mem::size_of::<virtio_kctl>(),
            GFP_KERNEL,
        );
        if kctls_ptr.is_null() {
            return -12; // -ENOMEM
        }
        (*snd).kctls = kctls_ptr as *mut virtio_kctl;

        let rc = virtsnd_ctl_query_info(
            snd,
            VIRTIO_SND_R_CTL_INFO,
            0,
            (*snd).nkctls,
            mem::size_of::<virtio_snd_ctl_info>(),
            (*snd).kctl_infos as *mut u8,
        );
        if rc != 0 {
            return rc;
        }

        for i in 0..(*snd).nkctls {
            let kinfo = &(*snd).kctl_infos[i as usize];
            let type_ = u32::from_le(kinfo.type_);

            let rc = virtsnd_kctl_validate_info(snd, i, kinfo as *mut virtio_snd_ctl_info);
            if rc != 0 {
                return rc;
            }

            if type_ == VIRTIO_SND_CTL_TYPE_ENUMERATED {
                let rc = virtsnd_kctl_get_enum_items(snd, i);
                if rc != 0 {
                    return rc;
                }
            }
        }

        0
    }
}

/// virtsnd_kctl_build_devs() - Build ALSA control elements.
/// @snd: VirtIO sound device.
///
/// Context: Any context that permits to sleep.
/// Return: 0 on success, -errno on failure.
pub fn virtsnd_kctl_build_devs(snd: *mut virtio_snd) -> i32 {
    unsafe {
        for cid in 0..(*snd).nkctls {
            let kinfo = &(*snd).kctl_infos[cid as usize];
            let kctl = &mut (*snd).kctls[cid as usize];
            let mut kctl_new: snd_kcontrol_new = mem::zeroed();

            kctl_new.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
            kctl_new.name = kinfo.name.as_ptr();
            kctl_new.index = u32::from_le(kinfo.index);

            for i in 0..G_V2A_ACCESS_MAP.len() {
                if (u32::from_le(kinfo.access) & (1 << i)) != 0 {
                    kctl_new.access |= G_V2A_ACCESS_MAP[i];
                }
            }

            if (kctl_new.access
                & (SNDRV_CTL_ELEM_ACCESS_TLV_READ
                    | SNDRV_CTL_ELEM_ACCESS_TLV_WRITE
                    | SNDRV_CTL_ELEM_ACCESS_TLV_COMMAND))
                != 0
            {
                kctl_new.access |= SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK;
                kctl_new.tlv.c = virtsnd_kctl_tlv_op as *const ();
            }

            kctl_new.info = virtsnd_kctl_info as *const ();
            kctl_new.get = virtsnd_kctl_get as *const ();
            kctl_new.put = virtsnd_kctl_put as *const ();
            kctl_new.private_value = cid as usize;

            kctl.kctl = snd_ctl_new1(&kctl_new, snd as *mut u8);
            if kctl.kctl.is_null() {
                return -12; // -ENOMEM
            }

            let rc = snd_ctl_add((*snd).card, kctl.kctl);
            if rc != 0 {
                return rc;
            }
        }

        0
    }
}

/// virtsnd_kctl_event() - Handle the control element event notification.
/// @snd: VirtIO sound device.
/// @event: VirtIO sound event.
///
/// Context: Interrupt context.
pub fn virtsnd_kctl_event(snd: *mut virtio_snd, event: *mut virtio_snd_event) {
    unsafe {
        let kevent = event as *mut virtio_snd_ctl_event;
        let cid = u16::from_le((*kevent).control_id) as u32;

        if cid >= (*snd).nkctls {
            return;
        }

        let mut mask = 0u32;
        for i in 0..G_V2A_MASK_MAP.len() {
            if (u16::from_le((*kevent).mask) as u32 & (1 << i)) != 0 {
                mask |= G_V2A_MASK_MAP[i];
            }
        }

        let kctl = &(*snd).kctls[cid as usize];
        snd_ctl_notify((*snd).card, mask, &(*kctl.kctl).id);
    }
}

// External declarations for functions from other modules/crates
extern "C" {
    fn dev_err(dev: *const u8, fmt: *const u8, ...);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut virtio_snd;
    fn strscpy(dest: *mut u8, src: *const u8, size: usize) -> usize;
    fn virtsnd_ctl_msg_alloc(request_size: usize, response_size: usize, gfp: u32) -> *mut virtio_snd_msg;
    fn virtsnd_ctl_msg_ref(msg: *mut virtio_snd_msg);
    fn virtsnd_ctl_msg_unref(msg: *mut virtio_snd_msg);
    fn virtsnd_ctl_msg_request(msg: *mut virtio_snd_msg) -> *mut u8;
    fn virtsnd_ctl_msg_response(msg: *mut virtio_snd_msg) -> *mut u8;
    fn virtsnd_ctl_msg_send_sync(snd: *mut virtio_snd, msg: *mut virtio_snd_msg) -> i32;
    fn virtsnd_ctl_msg_send(
        snd: *mut virtio_snd,
        msg: *mut virtio_snd_msg,
        request_sg: *mut scatterlist,
        response_sg: *mut scatterlist,
        nowait: bool,
    ) -> i32;
    fn sg_init_one(sg: *mut scatterlist, buf: *const u8, buflen: usize);
    fn kzalloc(size: usize, gfp: u32) -> *mut u8;
    fn kfree(ptr: *mut u8);
    fn copy_to_user(to: *mut u8, from: *const u8, n: usize) -> usize;
    fn copy_from_user(to: *mut u8, from: *const u8, n: usize) -> usize;
    fn devm_kcalloc(dev: *const u8, n: usize, size: usize, gfp: u32) -> *mut u8;
    fn virtio_cread_le(vdev: *const virtio_device, offset: usize, dest: *mut u8);
    fn virtsnd_ctl_query_info(
        snd: *mut virtio_snd,
        code: u32,
        start_id: u32,
        count: u32,
        size: usize,
        info: *mut u8,
    ) -> i32;
    fn snd_ctl_new1(kctl_new: *const snd_kcontrol_new, private_data: *mut u8) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> i32;
    fn snd_ctl_notify(card: *mut snd_card, mask: u32, id: *const snd_ctl_elem_id);
}

// External type declarations (placeholders for types from ALSA and VirtIO)
#[repr(C)]
pub struct virtio_snd {
    vdev: *mut virtio_device,
    card: *mut snd_card,
    nkctls: u32,
    kctl_infos: *mut virtio_snd_ctl_info,
    kctls: *mut virtio_kctl,
}

#[repr(C)]
pub struct virtio_device {
    dev: u8, // Placeholder
}

#[repr(C)]
pub struct snd_card;

#[repr(C)]
pub struct virtio_snd_ctl_info {
    type_: u32,
    count: u32,
    access: u32,
    index: u32,
    name: [u8; 64],
    value: virtio_snd_ctl_value_union,
}

#[repr(C)]
pub union virtio_snd_ctl_value_union {
    integer: virtio_snd_ctl_value_integer,
    integer64: virtio_snd_ctl_value_integer64,
    enumerated: virtio_snd_ctl_value_enumerated,
    bytes: [u8; 512],
    iec958: [u8; 128],
}

#[repr(C)]
pub struct virtio_snd_ctl_value_integer {
    min: i32,
    max: i32,
    step: u32,
}

#[repr(C)]
pub struct virtio_snd_ctl_value_integer64 {
    min: i64,
    max: i64,
    step: u64,
}

#[repr(C)]
pub struct virtio_snd_ctl_value_enumerated {
    items: u32,
}

#[repr(C)]
pub struct virtio_kctl {
    kctl: *mut snd_kcontrol,
    items: *mut virtio_snd_ctl_enum_item,
}

#[repr(C)]
pub struct virtio_snd_ctl_enum_item {
    item: [u8; 64],
}

#[repr(C)]
pub struct snd_kcontrol {
    private_value: usize,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    type_: u32,
    count: u32,
    value: snd_ctl_elem_value_union,
}

#[repr(C)]
pub union snd_ctl_elem_value_union {
    integer: snd_ctl_elem_integer,
    integer64: snd_ctl_elem_integer64,
    enumerated: snd_ctl_elem_enumerated,
    bytes: snd_ctl_elem_bytes,
    iec958: [u8; 128],
}

#[repr(C)]
pub struct snd_ctl_elem_integer {
    min: i32,
    max: i32,
    step: u32,
}

#[repr(C)]
pub struct snd_ctl_elem_integer64 {
    min: i64,
    max: i64,
    step: u64,
}

#[repr(C)]
pub struct snd_ctl_elem_enumerated {
    items: u32,
    item: u32,
    name: [u8; 64],
}

#[repr(C)]
pub struct snd_ctl_elem_bytes {
    data: [u8; 512],
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_union,
}

#[repr(C)]
pub struct virtio_snd_msg;

#[repr(C)]
pub struct virtio_snd_hdr {
    code: u32,
}

#[repr(C)]
pub struct virtio_snd_ctl_hdr {
    hdr: virtio_snd_hdr,
    control_id: u32,
}

#[repr(C)]
pub struct virtio_snd_ctl_value {
    value: virtio_snd_ctl_value_union,
}

#[repr(C)]
pub struct scatterlist;

#[repr(C)]
pub struct virtio_snd_event;

#[repr(C)]
pub struct virtio_snd_ctl_event {
    control_id: u16,
    mask: u16,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    iface: u32,
    name: *const u8,
    index: u32,
    access: u32,
    tlv: snd_kcontrol_new_tlv,
    info: *const (),
    get: *const (),
    put: *const (),
    private_value: usize,
}

#[repr(C)]
pub union snd_kcontrol_new_tlv {
    c: *const (),
}

#[repr(C)]
pub struct snd_ctl_elem_id;

#[repr(C)]
pub struct virtio_snd_config;

// Constants (from ALSA and VirtIO headers)
const GFP_KERNEL: u32 = 0xd0;

// VIRTIO control types
const VIRTIO_SND_CTL_TYPE_BOOLEAN: u32 = 0;
const VIRTIO_SND_CTL_TYPE_INTEGER: u32 = 1;
const VIRTIO_SND_CTL_TYPE_INTEGER64: u32 = 2;
const VIRTIO_SND_CTL_TYPE_ENUMERATED: u32 = 3;
const VIRTIO_SND_CTL_TYPE_BYTES: u32 = 4;
const VIRTIO_SND_CTL_TYPE_IEC958: u32 = 5;

// VIRTIO control access types
const VIRTIO_SND_CTL_ACCESS_READ: u32 = 0;
const VIRTIO_SND_CTL_ACCESS_WRITE: u32 = 1;
const VIRTIO_SND_CTL_ACCESS_VOLATILE: u32 = 2;
const VIRTIO_SND_CTL_ACCESS_INACTIVE: u32 = 3;
const VIRTIO_SND_CTL_ACCESS_TLV_READ: u32 = 4;
const VIRTIO_SND_CTL_ACCESS_TLV_WRITE: u32 = 5;
const VIRTIO_SND_CTL_ACCESS_TLV_COMMAND: u32 = 6;

// VIRTIO event mask types
const VIRTIO_SND_CTL_EVT_MASK_VALUE: u32 = 0;
const VIRTIO_SND_CTL_EVT_MASK_INFO: u32 = 1;
const VIRTIO_SND_CTL_EVT_MASK_TLV: u32 = 2;

// VIRTIO control request/response codes
const VIRTIO_SND_R_CTL_READ: u32 = 256;
const VIRTIO_SND_R_CTL_WRITE: u32 = 257;
const VIRTIO_SND_R_CTL_TLV_READ: u32 = 258;
const VIRTIO_SND_R_CTL_TLV_WRITE: u32 = 259;
const VIRTIO_SND_R_CTL_TLV_COMMAND: u32 = 260;
const VIRTIO_SND_R_CTL_INFO: u32 = 261;
const VIRTIO_SND_R_CTL_ENUM_ITEMS: u32 = 262;

// ALSA control element types
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: u32 = 0;
const SNDRV_CTL_ELEM_TYPE_INTEGER: u32 = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER64: u32 = 2;
const SNDRV_CTL_ELEM_TYPE_ENUMERATED: u32 = 3;
const SNDRV_CTL_ELEM_TYPE_BYTES: u32 = 4;
const SNDRV_CTL_ELEM_TYPE_IEC958: u32 = 5;

// ALSA control element access flags
const SNDRV_CTL_ELEM_ACCESS_READ: u32 = 0x0001;
const SNDRV_CTL_ELEM_ACCESS_WRITE: u32 = 0x0002;
const SNDRV_CTL_ELEM_ACCESS_VOLATILE: u32 = 0x0010;
const SNDRV_CTL_ELEM_ACCESS_INACTIVE: u32 = 0x0020;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: u32 = 0x0100;
const SNDRV_CTL_ELEM_ACCESS_TLV_WRITE: u32 = 0x0200;
const SNDRV_CTL_ELEM_ACCESS_TLV_COMMAND: u32 = 0x0400;
const SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK: u32 = 0x1000;

// ALSA control event masks
const SNDRV_CTL_EVENT_MASK_VALUE: u32 = 1 << 0;
const SNDRV_CTL_EVENT_MASK_INFO: u32 = 1 << 1;
const SNDRV_CTL_EVENT_MASK_TLV: u32 = 1 << 2;

// ALSA control interface type
const SNDRV_CTL_ELEM_IFACE_MIXER: u32 = 0;

// TLV operation flags
const SNDRV_CTL_TLV_OP_READ: i32 = 0;
const SNDRV_CTL_TLV_OP_WRITE: i32 = 1;
const SNDRV_CTL_TLV_OP_CMD: i32 = 2;

// Helper macro/function to get offset in a struct (equivalent to C's offsetof)
const fn offset_of<T: ?Sized>(ty: std::marker::PhantomData<T>, field_offset: usize) -> usize {
    field_offset
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
