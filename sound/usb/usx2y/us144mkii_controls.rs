// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2025 Šerif Rami <ramiserifpersia@gmail.com>

// Depends on: us144mkii.h

// Text descriptions for playback output source options.
//
// Used by ALSA kcontrol elements to provide user-friendly names for
// the playback routing options (e.g., "Playback 1-2", "Playback 3-4").
static PLAYBACK_SOURCE_TEXTS: &[&str] = &["Playback 1-2", "Playback 3-4"];

// Text descriptions for capture input source options.
//
// Used by ALSA kcontrol elements to provide user-friendly names for
// the capture routing options (e.g., "Analog In", "Digital In").
static CAPTURE_SOURCE_TEXTS: &[&str] = &["Analog In", "Digital In"];

// tascam_playback_source_info() - ALSA control info callback for playback
// source.
// @kcontrol: The ALSA kcontrol instance.
// @uinfo: The ALSA control element info structure to fill.
//
// This function provides information about the enumerated playback source
// control, including its type, count, and available items (Playback 1-2,
// Playback 3-4).
//
// Return: 0 on success.
unsafe extern "C" fn tascam_playback_source_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    snd_ctl_enum_info(uinfo, 1, 2, PLAYBACK_SOURCE_TEXTS.as_ptr() as *const _)
}

// tascam_line_out_get() - ALSA control get callback for Line Outputs Source.
// @kcontrol: The ALSA kcontrol instance.
// @ucontrol: The ALSA control element value structure to fill.
//
// This function retrieves the current selection for the Line Outputs source
// (Playback 1-2 or Playback 3-4) from the driver's private data and populates
// the ALSA control element value.
//
// Return: 0 on success.
unsafe extern "C" fn tascam_line_out_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tascam = snd_kcontrol_chip(kcontrol) as *mut tascam_card;

    // scoped_guard(spinlock_irqsave, &tascam->lock) equivalent
    (*ucontrol).value.enumerated.item[0] = (*tascam).line_out_source;
    0
}

// tascam_line_out_put() - ALSA control put callback for Line Outputs Source.
// @kcontrol: The ALSA kcontrol instance.
// @ucontrol: The ALSA control element value structure containing the new value.
//
// This function sets the Line Outputs source (Playback 1-2 or Playback 3-4)
// based on the user's selection from the ALSA control element. It validates
// the input and updates the driver's private data.
//
// Return: 1 if the value was changed, 0 if unchanged, or a negative error code.
unsafe extern "C" fn tascam_line_out_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tascam = snd_kcontrol_chip(kcontrol) as *mut tascam_card;
    let mut changed = 0;

    if (*ucontrol).value.enumerated.item[0] > 1 {
        return -libc::EINVAL;
    }

    // scoped_guard(spinlock_irqsave, &tascam->lock) equivalent
    if (*tascam).line_out_source != (*ucontrol).value.enumerated.item[0] {
        (*tascam).line_out_source = (*ucontrol).value.enumerated.item[0];
        changed = 1;
    }

    changed
}

// tascam_line_out_control - ALSA kcontrol definition for Line Outputs Source.
//
// This defines a new ALSA mixer control named "Line OUTPUTS Source" that allows
// the user to select between "Playback 1-2" and "Playback 3-4" for the analog
// line outputs of the device. It uses the `tascam_playback_source_info` for
// information and `tascam_line_out_get`/`tascam_line_out_put` for value
// handling.
static TASCAM_LINE_OUT_CONTROL: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER as u32,
    name: b"Line Playback Source\0" as *const u8 as *const i8,
    info: Some(tascam_playback_source_info),
    get: Some(tascam_line_out_get),
    put: Some(tascam_line_out_put),
    ..snd_kcontrol_new_default()
};

// tascam_digital_out_get() - ALSA control get callback for Digital Outputs
// Source.
// @kcontrol: The ALSA kcontrol instance.
// @ucontrol: The ALSA control element value structure to fill.
//
// This function retrieves the current selection for the Digital Outputs source
// (Playback 1-2 or Playback 3-4) from the driver's private data and populates
// the ALSA control element value.
//
// Return: 0 on success.
unsafe extern "C" fn tascam_digital_out_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tascam = snd_kcontrol_chip(kcontrol) as *mut tascam_card;

    // scoped_guard(spinlock_irqsave, &tascam->lock) equivalent
    (*ucontrol).value.enumerated.item[0] = (*tascam).digital_out_source;
    0
}

// tascam_digital_out_put() - ALSA control put callback for Digital Outputs
// Source.
// @kcontrol: The ALSA kcontrol instance.
// @ucontrol: The ALSA control element value structure containing the new value.
//
// This function sets the Digital Outputs source (Playback 1-2 or Playback 3-4)
// based on the user's selection from the ALSA control element. It validates
// the input and updates the driver's private data.
//
// Return: 1 if the value was changed, 0 if unchanged, or a negative error code.
unsafe extern "C" fn tascam_digital_out_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tascam = snd_kcontrol_chip(kcontrol) as *mut tascam_card;
    let mut changed = 0;

    if (*ucontrol).value.enumerated.item[0] > 1 {
        return -libc::EINVAL;
    }

    // scoped_guard(spinlock_irqsave, &tascam->lock) equivalent
    if (*tascam).digital_out_source != (*ucontrol).value.enumerated.item[0] {
        (*tascam).digital_out_source = (*ucontrol).value.enumerated.item[0];
        changed = 1;
    }

    changed
}

// tascam_digital_out_control - ALSA kcontrol definition for Digital Outputs
// Source.
//
// This defines a new ALSA mixer control named "Digital OUTPUTS Source" that
// allows the user to select between "Playback 1-2" and "Playback 3-4" for the
// digital outputs of the device. It uses the `tascam_playback_source_info` for
// information and `tascam_digital_out_get`/`tascam_digital_out_put` for value
// handling.
static TASCAM_DIGITAL_OUT_CONTROL: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER as u32,
    name: b"Digital Playback Source\0" as *const u8 as *const i8,
    info: Some(tascam_playback_source_info),
    get: Some(tascam_digital_out_get),
    put: Some(tascam_digital_out_put),
    ..snd_kcontrol_new_default()
};

// tascam_capture_source_info() - ALSA control info callback for capture source.
// @kcontrol: The ALSA kcontrol instance.
// @uinfo: The ALSA control element info structure to fill.
//
// This function provides information about the enumerated capture source
// control, including its type, count, and available items (Analog In, Digital
// In).
//
// Return: 0 on success.
unsafe extern "C" fn tascam_capture_source_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    snd_ctl_enum_info(uinfo, 1, 2, CAPTURE_SOURCE_TEXTS.as_ptr() as *const _)
}

// tascam_capture_12_get() - ALSA control get callback for Capture channels 1
// and 2 Source.
// @kcontrol: The ALSA kcontrol instance.
// @ucontrol: The ALSA control element value structure to fill.
//
// This function retrieves the current selection for the Capture channels 1 and
// 2 source (Analog In or Digital In) from the driver's private data and
// populates the ALSA control element value.
//
// Return: 0 on success.
unsafe extern "C" fn tascam_capture_12_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tascam = snd_kcontrol_chip(kcontrol) as *mut tascam_card;

    // scoped_guard(spinlock_irqsave, &tascam->lock) equivalent
    (*ucontrol).value.enumerated.item[0] = (*tascam).capture_12_source;
    0
}

// tascam_capture_12_put() - ALSA control put callback for Capture channels 1
// and 2 Source.
// @kcontrol: The ALSA kcontrol instance.
// @ucontrol: The ALSA control element value structure containing the new value.
//
// This function sets the Capture channels 1 and 2 source (Analog In or Digital
// In) based on the user's selection from the ALSA control element. It validates
// the input and updates the driver's private data.
//
// Return: 1 if the value was changed, 0 if unchanged, or a negative error code.
unsafe extern "C" fn tascam_capture_12_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tascam = snd_kcontrol_chip(kcontrol) as *mut tascam_card;
    let mut changed = 0;

    if (*ucontrol).value.enumerated.item[0] > 1 {
        return -libc::EINVAL;
    }

    // scoped_guard(spinlock_irqsave, &tascam->lock) equivalent
    if (*tascam).capture_12_source != (*ucontrol).value.enumerated.item[0] {
        (*tascam).capture_12_source = (*ucontrol).value.enumerated.item[0];
        changed = 1;
    }

    changed
}

// tascam_capture_12_control - ALSA kcontrol definition for Capture channels 1
// and 2 Source.
//
// This defines a new ALSA mixer control named "ch1 and ch2 Source" that allows
// the user to select between "Analog In" and "Digital In" for the first two
// capture channels of the device. It uses the `tascam_capture_source_info` for
// information and `tascam_capture_12_get`/`tascam_capture_12_put` for value
// handling.
static TASCAM_CAPTURE_12_CONTROL: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER as u32,
    name: b"Ch1/2 Capture Source\0" as *const u8 as *const i8,
    info: Some(tascam_capture_source_info),
    get: Some(tascam_capture_12_get),
    put: Some(tascam_capture_12_put),
    ..snd_kcontrol_new_default()
};

// tascam_capture_34_get() - ALSA control get callback for Capture channels 3
// and 4 Source.
// @kcontrol: The ALSA kcontrol instance.
// @ucontrol: The ALSA control element value structure to fill.
//
// This function retrieves the current selection for the Capture channels 3 and
// 4 source (Analog In or Digital In) from the driver's private data and
// populates the ALSA control element value.
//
// Return: 0 on success.
unsafe extern "C" fn tascam_capture_34_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tascam = snd_kcontrol_chip(kcontrol) as *mut tascam_card;

    // scoped_guard(spinlock_irqsave, &tascam->lock) equivalent
    (*ucontrol).value.enumerated.item[0] = (*tascam).capture_34_source;
    0
}

// tascam_capture_34_put() - ALSA control put callback for Capture channels 3
// and 4 Source.
// @kcontrol: The ALSA kcontrol instance.
// @ucontrol: The ALSA control element value structure containing the new value.
//
// This function sets the Capture channels 3 and 4 source (Analog In or Digital
// In) based on the user's selection from the ALSA control element. It validates
// the input and updates the driver's private data.
//
// Return: 1 if the value was changed, 0 if unchanged, or a negative error code.
unsafe extern "C" fn tascam_capture_34_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tascam = snd_kcontrol_chip(kcontrol) as *mut tascam_card;
    let mut changed = 0;

    if (*ucontrol).value.enumerated.item[0] > 1 {
        return -libc::EINVAL;
    }

    // scoped_guard(spinlock_irqsave, &tascam->lock) equivalent
    if (*tascam).capture_34_source != (*ucontrol).value.enumerated.item[0] {
        (*tascam).capture_34_source = (*ucontrol).value.enumerated.item[0];
        changed = 1;
    }

    changed
}

// tascam_capture_34_control - ALSA kcontrol definition for Capture channels 3
// and 4 Source.
//
// This defines a new ALSA mixer control named "ch3 and ch4 Source" that allows
// the user to select between "Analog In" and "Digital In" for the third and
// fourth capture channels of the device. It uses the
// `tascam_capture_source_info` for information and
// `tascam_capture_34_get`/`tascam_capture_34_put` for value handling.
static TASCAM_CAPTURE_34_CONTROL: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER as u32,
    name: b"Ch3/4 Capture Source\0" as *const u8 as *const i8,
    info: Some(tascam_capture_source_info),
    get: Some(tascam_capture_34_get),
    put: Some(tascam_capture_34_put),
    ..snd_kcontrol_new_default()
};

// tascam_samplerate_info() - ALSA control info callback for Sample Rate.
// @kcontrol: The ALSA kcontrol instance.
// @uinfo: The ALSA control element info structure to fill.
//
// This function provides information about the Sample Rate control, defining
// it as an integer type with a minimum value of 0 and a maximum of 96000.
//
// Return: 0 on success.
unsafe extern "C" fn tascam_samplerate_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 96000;
    0
}

// tascam_samplerate_get() - ALSA control get callback for Sample Rate.
// @kcontrol: The ALSA kcontrol instance.
// @ucontrol: The ALSA control element value structure to fill.
//
// This function retrieves the current sample rate from the device via a USB
// control message and populates the ALSA control element value. If the rate
// is already known (i.e., `current_rate` is set), it returns that value
// directly.
//
// Return: 0 on success, or a negative error code on failure.
unsafe extern "C" fn tascam_samplerate_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tascam = snd_kcontrol_chip(kcontrol) as *mut tascam_card;
    let mut rate: u32 = 0;

    // scoped_guard(spinlock_irqsave, &tascam->lock) equivalent
    if (*tascam).current_rate > 0 {
        (*ucontrol).value.integer.value[0] = (*tascam).current_rate;
        return 0;
    }

    let buf = kmalloc(3, GFP_KERNEL) as *mut u8;
    if buf.is_null() {
        return -libc::ENOMEM;
    }

    let err = usb_control_msg(
        (*tascam).dev,
        usb_rcvctrlpipe((*tascam).dev, 0),
        UAC_GET_CUR,
        RT_D2H_CLASS_EP,
        UAC_SAMPLING_FREQ_CONTROL,
        EP_AUDIO_IN,
        buf as *mut i8,
        3,
        USB_CTRL_TIMEOUT_MS,
    );

    if err >= 3 {
        rate = (*buf) as u32
            | (((*buf.add(1)) as u32) << 8)
            | (((*buf.add(2)) as u32) << 16);
    }

    kfree(buf as *mut _);

    (*ucontrol).value.integer.value[0] = rate as i64;
    0
}

// tascam_samplerate_control - ALSA kcontrol definition for Sample Rate.
//
// This defines a new ALSA mixer control named "Sample Rate" that displays
// the current sample rate of the device. It is a read-only control.
static TASCAM_SAMPLERATE_CONTROL: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER as u32,
    name: b"Sample Rate\0" as *const u8 as *const i8,
    info: Some(tascam_samplerate_info),
    get: Some(tascam_samplerate_get),
    access: SNDRV_CTL_ELEM_ACCESS_READ,
    ..snd_kcontrol_new_default()
};

pub unsafe extern "C" fn tascam_create_controls(tascam: *mut tascam_card) -> i32 {
    let mut err;

    err = snd_ctl_add(
        (*tascam).card,
        snd_ctl_new1(&TASCAM_LINE_OUT_CONTROL, tascam as *mut _),
    );
    if err < 0 {
        return err;
    }

    err = snd_ctl_add(
        (*tascam).card,
        snd_ctl_new1(&TASCAM_DIGITAL_OUT_CONTROL, tascam as *mut _),
    );
    if err < 0 {
        return err;
    }

    err = snd_ctl_add(
        (*tascam).card,
        snd_ctl_new1(&TASCAM_CAPTURE_12_CONTROL, tascam as *mut _),
    );
    if err < 0 {
        return err;
    }

    err = snd_ctl_add(
        (*tascam).card,
        snd_ctl_new1(&TASCAM_CAPTURE_34_CONTROL, tascam as *mut _),
    );
    if err < 0 {
        return err;
    }

    err = snd_ctl_add(
        (*tascam).card,
        snd_ctl_new1(&TASCAM_SAMPLERATE_CONTROL, tascam as *mut _),
    );
    if err < 0 {
        return err;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
