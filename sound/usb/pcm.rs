// SPDX-License-Identifier: GPL-2.0-or-later
/*
 */

// Requires: linux/init.h, linux/slab.h, linux/bitrev.h, linux/ratelimit.h,
// linux/usb.h, linux/usb/audio.h, linux/usb/audio-v2.h, sound/core.h,
// sound/pcm.h, sound/pcm_params.h
// Requires: usbaudio.h, card.h, quirks.h, endpoint.h, helper.h, pcm.h,
// clock.h, power.h, media.h, implicit.h

const SUBSTREAM_FLAG_DATA_EP_STARTED: i32 = 0;
const SUBSTREAM_FLAG_SYNC_EP_STARTED: i32 = 1;

// return the estimated delay based on USB frame counters
fn snd_usb_pcm_delay(
    subs: *mut snd_usb_substream,
    runtime: *mut snd_pcm_runtime,
) -> snd_pcm_uframes_t {
    let mut current_frame_number: u32;
    let mut frame_diff: u32;
    let mut est_delay: i32;
    let mut queued: i32;

    unsafe {
        if (*subs).direction == SNDRV_PCM_STREAM_PLAYBACK {
            queued = bytes_to_frames(runtime, (*subs).inflight_bytes);
            if queued == 0 {
                return 0;
            }
        } else if !(*subs).running {
            return 0;
        }

        current_frame_number = usb_get_current_frame_number((*subs).dev);
        // HCD implementations use different widths, use lower 8 bits.
        // The delay will be managed up to 256ms, which is more than enough
        frame_diff = (current_frame_number.wrapping_sub((*subs).last_frame_number)) & 0xff;

        // Approximation based on number of samples per USB frame (ms),
        // some truncation for 44.1 but the estimate is good enough
        est_delay = (frame_diff as i32) * ((*runtime).rate as i32) / 1000;

        if (*subs).direction == SNDRV_PCM_STREAM_PLAYBACK {
            est_delay = queued - est_delay;
            if est_delay < 0 {
                est_delay = 0;
            }
        }

        est_delay as snd_pcm_uframes_t
    }
}

// return the current pcm pointer. just based on the hwptr_done value.
fn snd_usb_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let mut hwptr_done: u32;

    unsafe {
        let runtime = (*substream).runtime;
        let subs = (*runtime).private_data as *mut snd_usb_substream;

        if atomic_read(&(*(*subs).stream).chip.shutdown) != 0 {
            return SNDRV_PCM_POS_XRUN;
        }

        // scoped_guard equivalent: lock scope
        {
            let _guard = spinlock_guard(&(*subs).lock);
            hwptr_done = (*subs).hwptr_done;
            (*runtime).delay = snd_usb_pcm_delay(subs, runtime);
        }

        bytes_to_frames(runtime, hwptr_done)
    }
}

// find a matching audio format
fn find_format(
    fmt_list_head: *mut list_head,
    format: snd_pcm_format_t,
    rate: u32,
    channels: u32,
    strict_match: bool,
    subs: *mut snd_usb_substream,
) -> *const audioformat {
    let mut found: *const audioformat = std::ptr::null();
    let mut cur_attr: i32 = 0;

    unsafe {
        let mut fp: *const audioformat;
        let mut list_entry = (*fmt_list_head).next;

        while list_entry != fmt_list_head {
            fp = list_entry as *const audioformat;

            if strict_match {
                if ((*fp).formats & pcm_format_to_bits(format)) == 0 {
                    list_entry = (*list_entry).next;
                    continue;
                }
                if (*fp).channels != channels {
                    list_entry = (*list_entry).next;
                    continue;
                }
            }

            if rate < (*fp).rate_min || rate > (*fp).rate_max {
                list_entry = (*list_entry).next;
                continue;
            }

            if ((*fp).rates & SNDRV_PCM_RATE_CONTINUOUS) == 0 {
                let mut i = 0;
                while i < (*fp).nr_rates {
                    if *(*fp).rate_table.add(i) == rate {
                        break;
                    }
                    i += 1;
                }
                if i >= (*fp).nr_rates {
                    list_entry = (*list_entry).next;
                    continue;
                }
            }

            let attr = (*fp).ep_attr & USB_ENDPOINT_SYNCTYPE;

            if found.is_null() {
                found = fp;
                cur_attr = attr;
                list_entry = (*list_entry).next;
                continue;
            }

            // avoid async out and adaptive in if the other method supports the same format.
            // this is a workaround for the case like M-audio audiophile USB.
            if !subs.is_null() && attr != cur_attr {
                if (attr == USB_ENDPOINT_SYNC_ASYNC
                    && (*subs).direction == SNDRV_PCM_STREAM_PLAYBACK)
                    || (attr == USB_ENDPOINT_SYNC_ADAPTIVE
                        && (*subs).direction == SNDRV_PCM_STREAM_CAPTURE)
                {
                    list_entry = (*list_entry).next;
                    continue;
                }
                if (cur_attr == USB_ENDPOINT_SYNC_ASYNC
                    && (*subs).direction == SNDRV_PCM_STREAM_PLAYBACK)
                    || (cur_attr == USB_ENDPOINT_SYNC_ADAPTIVE
                        && (*subs).direction == SNDRV_PCM_STREAM_CAPTURE)
                {
                    found = fp;
                    cur_attr = attr;
                    list_entry = (*list_entry).next;
                    continue;
                }
            }

            // find the format with the largest max. packet size
            if (*fp).maxpacksize > (*found).maxpacksize {
                found = fp;
                cur_attr = attr;
            }

            list_entry = (*list_entry).next;
        }
    }

    found
}

#[no_mangle]
pub extern "C" fn snd_usb_find_format(
    fmt_list_head: *mut list_head,
    format: snd_pcm_format_t,
    rate: u32,
    channels: u32,
    strict_match: bool,
    subs: *mut snd_usb_substream,
) -> *const audioformat {
    find_format(fmt_list_head, format, rate, channels, strict_match, subs)
}

fn find_substream_format(
    subs: *mut snd_usb_substream,
    params: *const snd_pcm_hw_params,
) -> *const audioformat {
    unsafe {
        find_format(
            &mut (*subs).fmt_list as *mut _,
            params_format(params),
            params_rate(params),
            params_channels(params),
            true,
            subs,
        )
    }
}

#[no_mangle]
pub extern "C" fn snd_usb_find_substream_format(
    subs: *mut snd_usb_substream,
    params: *const snd_pcm_hw_params,
) -> *const audioformat {
    find_substream_format(subs, params)
}

#[no_mangle]
pub extern "C" fn snd_usb_pcm_has_fixed_rate(subs: *mut snd_usb_substream) -> bool {
    unsafe {
        if subs.is_null() {
            return false;
        }

        let chip = (*(*subs).stream).chip;
        if ((*chip).quirk_flags & QUIRK_FLAG_FIXED_RATE) == 0 {
            return false;
        }

        let mut rate: i32 = -1;
        let mut fp: *const audioformat;
        let mut list_entry = (*(*subs).fmt_list).next;
        let fmt_list_head = &mut (*subs).fmt_list as *mut _;

        while list_entry != fmt_list_head {
            fp = list_entry as *const audioformat;

            if ((*fp).rates & SNDRV_PCM_RATE_CONTINUOUS) != 0 {
                return false;
            }
            if (*fp).nr_rates < 1 {
                list_entry = (*list_entry).next;
                continue;
            }
            if (*fp).nr_rates > 1 {
                return false;
            }
            if rate < 0 {
                rate = *(*fp).rate_table;
                list_entry = (*list_entry).next;
                continue;
            }
            if rate != *(*fp).rate_table {
                return false;
            }

            list_entry = (*list_entry).next;
        }

        true
    }
}

fn init_pitch_v1(chip: *mut snd_usb_audio, ep: i32) -> i32 {
    unsafe {
        let dev = (*chip).dev;
        let mut data: [u8; 1] = [1];

        snd_usb_ctl_msg(
            dev,
            usb_sndctrlpipe(dev, 0),
            UAC_SET_CUR,
            USB_TYPE_CLASS | USB_RECIP_ENDPOINT | USB_DIR_OUT,
            (UAC_EP_CS_ATTR_PITCH_CONTROL as u32) << 8,
            ep as u32,
            data.as_mut_ptr() as *mut u8,
            std::mem::size_of_val(&data) as u32,
        )
    }
}

fn init_pitch_v2(chip: *mut snd_usb_audio, ep: i32) -> i32 {
    unsafe {
        let dev = (*chip).dev;
        let mut data: [u8; 1] = [1];

        snd_usb_ctl_msg(
            dev,
            usb_sndctrlpipe(dev, 0),
            UAC2_CS_CUR,
            USB_TYPE_CLASS | USB_RECIP_ENDPOINT | USB_DIR_OUT,
            (UAC2_EP_CS_PITCH as u32) << 8,
            0,
            data.as_mut_ptr() as *mut u8,
            std::mem::size_of_val(&data) as u32,
        )
    }
}

// initialize the pitch control and sample rate
#[no_mangle]
pub extern "C" fn snd_usb_init_pitch(chip: *mut snd_usb_audio, fmt: *const audioformat) -> i32 {
    unsafe {
        // if endpoint doesn't have pitch control, bail out
        if ((*fmt).attributes & UAC_EP_CS_ATTR_PITCH_CONTROL) == 0 {
            return 0;
        }

        usb_audio_dbg(chip, "enable PITCH for EP 0x%x\n".as_ptr(), (*fmt).endpoint);

        let err = match (*fmt).protocol {
            UAC_VERSION_1 => init_pitch_v1(chip, (*fmt).endpoint as i32),
            UAC_VERSION_2 => init_pitch_v2(chip, (*fmt).endpoint as i32),
            _ => return 0,
        };

        if err < 0 {
            usb_audio_err(
                chip,
                "failed to enable PITCH for EP 0x%x\n".as_ptr(),
                (*fmt).endpoint,
            );
            return err;
        }

        0
    }
}

fn stop_endpoints(subs: *mut snd_usb_substream, keep_pending: bool) -> bool {
    unsafe {
        let mut stopped = false;

        if test_and_clear_bit(SUBSTREAM_FLAG_SYNC_EP_STARTED, &mut (*subs).flags) {
            snd_usb_endpoint_stop((*subs).sync_endpoint, keep_pending);
            stopped = true;
        }
        if test_and_clear_bit(SUBSTREAM_FLAG_DATA_EP_STARTED, &mut (*subs).flags) {
            snd_usb_endpoint_stop((*subs).data_endpoint, keep_pending);
            stopped = true;
        }
        stopped
    }
}

fn start_endpoints(subs: *mut snd_usb_substream) -> i32 {
    unsafe {
        if (*subs).data_endpoint.is_null() {
            return -EINVAL;
        }

        if !test_and_set_bit(SUBSTREAM_FLAG_DATA_EP_STARTED, &mut (*subs).flags) {
            let err = snd_usb_endpoint_start((*subs).data_endpoint);
            if err < 0 {
                clear_bit(SUBSTREAM_FLAG_DATA_EP_STARTED, &mut (*subs).flags);
                return error(subs);
            }
        }

        if !(*subs).sync_endpoint.is_null()
            && !test_and_set_bit(SUBSTREAM_FLAG_SYNC_EP_STARTED, &mut (*subs).flags)
        {
            let err = snd_usb_endpoint_start((*subs).sync_endpoint);
            if err < 0 {
                clear_bit(SUBSTREAM_FLAG_SYNC_EP_STARTED, &mut (*subs).flags);
                return error(subs);
            }
        }

        0
    }

    #[cold]
    unsafe fn error(subs: *mut snd_usb_substream) -> i32 {
        stop_endpoints(subs, false);
        -1 // placeholder for actual err value
    }
}

fn sync_pending_stops(subs: *mut snd_usb_substream) {
    unsafe {
        snd_usb_endpoint_sync_pending_stop((*subs).sync_endpoint);
        snd_usb_endpoint_sync_pending_stop((*subs).data_endpoint);
    }
}

// PCM sync_stop callback
#[no_mangle]
pub extern "C" fn snd_usb_pcm_sync_stop(substream: *mut snd_pcm_substream) -> i32 {
    unsafe {
        let subs = (*(*substream).runtime).private_data as *mut snd_usb_substream;
        sync_pending_stops(subs);
        0
    }
}

// Set up sync endpoint
#[no_mangle]
pub extern "C" fn snd_usb_audioformat_set_sync_ep(
    chip: *mut snd_usb_audio,
    fmt: *mut audioformat,
) -> i32 {
    unsafe {
        if (*fmt).sync_ep != 0 {
            return 0; // already set up
        }

        let dev = (*chip).dev;
        let alts = snd_usb_get_host_interface(chip, (*fmt).iface, (*fmt).altsetting);
        if alts.is_null() {
            return 0;
        }

        let altsd = get_iface_desc(alts);

        let err = snd_usb_parse_implicit_fb_quirk(chip, fmt, alts);
        if err > 0 {
            return 0; // matched
        }

        // Generic sync EP handling
        if (*fmt).ep_idx > 0 || (*altsd).bNumEndpoints < 2 {
            return 0;
        }

        let is_playback =
            ((*get_endpoint(alts, 0)).bEndpointAddress & USB_DIR_IN as u8) == 0;
        let attr = (*fmt).ep_attr & USB_ENDPOINT_SYNCTYPE;

        if (is_playback
            && (attr == USB_ENDPOINT_SYNC_SYNC || attr == USB_ENDPOINT_SYNC_ADAPTIVE))
            || (!is_playback && attr != USB_ENDPOINT_SYNC_ADAPTIVE)
        {
            return 0;
        }

        let sync_attr = (*get_endpoint(alts, 1)).bmAttributes;

        // In case of illegal SYNC_NONE for OUT endpoint, we keep going to see
        // if we don't find a sync endpoint, as on M-Audio Transit. In case of
        // error fall back to SYNC mode and don't create sync endpoint

        // check sync-pipe endpoint
        if (sync_attr & USB_ENDPOINT_XFERTYPE_MASK) != USB_ENDPOINT_XFER_ISOC
            || ((*get_endpoint(alts, 1)).bLength >= USB_DT_ENDPOINT_AUDIO_SIZE
                && (*get_endpoint(alts, 1)).bSynchAddress != 0)
        {
            dev_err(
                &(*dev).dev as *const _,
                "%d:%d : invalid sync pipe. bmAttributes %02x, bLength %d, bSynchAddress %02x\n"
                    .as_ptr(),
                (*fmt).iface,
                (*fmt).altsetting,
                (*get_endpoint(alts, 1)).bmAttributes,
                (*get_endpoint(alts, 1)).bLength,
                (*get_endpoint(alts, 1)).bSynchAddress,
            );
            if is_playback && attr == USB_ENDPOINT_SYNC_NONE {
                return 0;
            }
            return -EINVAL;
        }

        let ep = (*get_endpoint(alts, 1)).bEndpointAddress;

        if (*get_endpoint(alts, 0)).bLength >= USB_DT_ENDPOINT_AUDIO_SIZE
            && (*get_endpoint(alts, 0)).bSynchAddress != 0
            && ((is_playback
                && ep != (((*get_endpoint(alts, 0)).bSynchAddress as u32) | USB_DIR_IN as u32)
                    as u8)
                || (!is_playback
                    && ep != (((*get_endpoint(alts, 0)).bSynchAddress as u32) & !(USB_DIR_IN as u32))
                        as u8))
        {
            dev_err(
                &(*dev).dev as *const _,
                "%d:%d : invalid sync pipe. is_playback %d, ep %02x, bSynchAddress %02x\n"
                    .as_ptr(),
                (*fmt).iface,
                (*fmt).altsetting,
                if is_playback { 1 } else { 0 },
                ep,
                (*get_endpoint(alts, 0)).bSynchAddress,
            );
            if is_playback && attr == USB_ENDPOINT_SYNC_NONE {
                return 0;
            }
            return -EINVAL;
        }

        (*fmt).sync_ep = ep;
        (*fmt).sync_iface = (*altsd).bInterfaceNumber;
        (*fmt).sync_altsetting = (*altsd).bAlternateSetting;
        (*fmt).sync_ep_idx = 1;
        if (sync_attr & USB_ENDPOINT_USAGE_MASK) == USB_ENDPOINT_USAGE_IMPLICIT_FB {
            (*fmt).implicit_fb = 1;
        }

        dev_dbg(
            &(*dev).dev as *const _,
            "%d:%d: found sync_ep=0x%x, iface=%d, alt=%d, implicit_fb=%d\n".as_ptr(),
            (*fmt).iface,
            (*fmt).altsetting,
            (*fmt).sync_ep,
            (*fmt).sync_iface,
            (*fmt).sync_altsetting,
            (*fmt).implicit_fb,
        );

        0
    }
}

fn snd_usb_pcm_change_state(subs: *mut snd_usb_substream, state: i32) -> i32 {
    unsafe {
        if (*subs).str_pd.is_null() {
            return 0;
        }

        let ret =
            snd_usb_power_domain_set((*(*subs).stream).chip, (*subs).str_pd, state);
        if ret < 0 {
            dev_err(
                &(*(*subs).dev).dev as *const _,
                "Cannot change Power Domain ID: %d to state: %d. Err: %d\n".as_ptr(),
                (*(*subs).str_pd).pd_id,
                state,
                ret,
            );
            return ret;
        }

        0
    }
}

#[no_mangle]
pub extern "C" fn snd_usb_pcm_suspend(as_: *mut snd_usb_stream) -> i32 {
    unsafe {
        let ret = snd_usb_pcm_change_state(&mut (*as_).substream[0], UAC3_PD_STATE_D2);
        if ret < 0 {
            return ret;
        }

        let ret = snd_usb_pcm_change_state(&mut (*as_).substream[1], UAC3_PD_STATE_D2);
        if ret < 0 {
            return ret;
        }

        0
    }
}

#[no_mangle]
pub extern "C" fn snd_usb_pcm_resume(as_: *mut snd_usb_stream) -> i32 {
    unsafe {
        let ret = snd_usb_pcm_change_state(&mut (*as_).substream[0], UAC3_PD_STATE_D1);
        if ret < 0 {
            return ret;
        }

        let ret = snd_usb_pcm_change_state(&mut (*as_).substream[1], UAC3_PD_STATE_D1);
        if ret < 0 {
            return ret;
        }

        0
    }
}

fn close_endpoints(chip: *mut snd_usb_audio, subs: *mut snd_usb_substream) {
    unsafe {
        if !(*subs).data_endpoint.is_null() {
            snd_usb_endpoint_set_sync(chip, (*subs).data_endpoint, std::ptr::null_mut());
            snd_usb_endpoint_close(chip, (*subs).data_endpoint);
            (*subs).data_endpoint = std::ptr::null_mut();
        }

        if !(*subs).sync_endpoint.is_null() {
            snd_usb_endpoint_close(chip, (*subs).sync_endpoint);
            (*subs).sync_endpoint = std::ptr::null_mut();
        }
    }
}

#[no_mangle]
pub extern "C" fn snd_usb_hw_params(
    subs: *mut snd_usb_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> i32 {
    unsafe {
        let chip = (*(*subs).stream).chip;

        let ret = snd_media_start_pipeline(subs);
        if ret != 0 {
            return ret;
        }

        let fixed_rate = snd_usb_pcm_has_fixed_rate(subs);
        let fmt = find_substream_format(subs, hw_params as *const _);
        if fmt.is_null() {
            usb_audio_dbg(
                chip,
                "cannot find format: format=%s, rate=%d, channels=%d\n".as_ptr(),
                snd_pcm_format_name(params_format(hw_params as *const _)),
                params_rate(hw_params as *const _),
                params_channels(hw_params as *const _),
            );
            let ret = -EINVAL;
            snd_media_stop_pipeline(subs);
            return ret;
        }

        let (sync_fmt, sync_fixed_rate) = if (*fmt).implicit_fb != 0 {
            let mut sync_fixed_rate = false;
            let sync_fmt = snd_usb_find_implicit_fb_sync_format(
                chip,
                fmt,
                hw_params as *const _,
                (*subs).direction == 0,
                &mut sync_fixed_rate as *mut bool,
            );
            if sync_fmt.is_null() {
                usb_audio_dbg(
                    chip,
                    "cannot find sync format: ep=0x%x, iface=%d:%d, format=%s, rate=%d, channels=%d\n"
                        .as_ptr(),
                    (*fmt).sync_ep,
                    (*fmt).sync_iface,
                    (*fmt).sync_altsetting,
                    snd_pcm_format_name(params_format(hw_params as *const _)),
                    params_rate(hw_params as *const _),
                    params_channels(hw_params as *const _),
                );
                snd_media_stop_pipeline(subs);
                return -EINVAL;
            }
            (sync_fmt, sync_fixed_rate)
        } else {
            (fmt, fixed_rate)
        };

        let ret = snd_usb_lock_shutdown(chip);
        if ret < 0 {
            snd_media_stop_pipeline(subs);
            return ret;
        }

        let mut ret = snd_usb_pcm_change_state(subs, UAC3_PD_STATE_D0);
        if ret < 0 {
            goto_unlock(chip, subs, ret);
            return ret;
        }

        if !(*subs).data_endpoint.is_null() {
            if snd_usb_endpoint_compatible(
                chip,
                (*subs).data_endpoint,
                fmt,
                hw_params as *const _,
            ) {
                goto_unlock(chip, subs, 0);
                return 0;
            }
            if stop_endpoints(subs, false) {
                sync_pending_stops(subs);
            }
            close_endpoints(chip, subs);
        }

        (*subs).data_endpoint =
            snd_usb_endpoint_open(chip, fmt, hw_params as *const _, false, fixed_rate);
        if (*subs).data_endpoint.is_null() {
            ret = -EINVAL;
            goto_unlock(chip, subs, ret);
            return ret;
        }

        if (*fmt).sync_ep != 0 {
            (*subs).sync_endpoint = snd_usb_endpoint_open(
                chip,
                sync_fmt,
                hw_params as *const _,
                fmt == sync_fmt,
                sync_fixed_rate,
            );
            if (*subs).sync_endpoint.is_null() {
                ret = -EINVAL;
                goto_unlock(chip, subs, ret);
                return ret;
            }

            snd_usb_endpoint_set_sync(chip, (*subs).data_endpoint, (*subs).sync_endpoint);
        }

        // scoped_guard equivalent
        {
            let _guard = mutex_guard(&(*chip).mutex);
            (*subs).cur_audiofmt = fmt as *mut _;
        }

        if (*(*subs).data_endpoint).need_setup == 0 {
            goto_unlock(chip, subs, 0);
            return 0;
        }

        if !(*subs).sync_endpoint.is_null() {
            ret = snd_usb_endpoint_set_params(chip, (*subs).sync_endpoint);
            if ret < 0 {
                goto_unlock(chip, subs, ret);
                return ret;
            }
        }

        ret = snd_usb_endpoint_set_params(chip, (*subs).data_endpoint);

        if ret < 0 {
            close_endpoints(chip, subs);
        }

        snd_usb_unlock_shutdown(chip);
        if ret < 0 {
            snd_media_stop_pipeline(subs);
        }

        ret
    }

    #[inline]
    unsafe fn goto_unlock(chip: *mut snd_usb_audio, subs: *mut snd_usb_substream, ret: i32) -> i32 {
        if ret < 0 {
            close_endpoints(chip, subs);
        }
        snd_usb_unlock_shutdown(chip);
        ret
    }
}

// hw_params callback
// allocate a buffer and set the given audio format.
// so far we use a physically linear buffer although packetize transfer
// doesn't need a continuous area.
// if sg buffer is supported on the later version of alsa, we'll follow that.
#[no_mangle]
pub extern "C" fn snd_usb_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> i32 {
    unsafe {
        let subs = (*(*substream).runtime).private_data as *mut snd_usb_substream;
        snd_usb_hw_params(subs, hw_params)
    }
}

#[no_mangle]
pub extern "C" fn snd_usb_hw_free(subs: *mut snd_usb_substream) -> i32 {
    unsafe {
        let chip = (*(*subs).stream).chip;

        snd_media_stop_pipeline(subs);

        // scoped_guard equivalent
        {
            let _guard = mutex_guard(&(*chip).mutex);
            (*subs).cur_audiofmt = std::ptr::null_mut();
        }

        // CLASS macro equivalent
        let pm_err = snd_usb_lock_shutdown(chip);
        if pm_err == 0 {
            if stop_endpoints(subs, false) {
                sync_pending_stops(subs);
            }
            close_endpoints(chip, subs);
        }

        0
    }
}

// hw_free callback
// reset the audio format and release the buffer
#[no_mangle]
pub extern "C" fn snd_usb_pcm_hw_free(substream: *mut snd_pcm_substream) -> i32 {
    unsafe {
        let subs = (*(*substream).runtime).private_data as *mut snd_usb_substream;
        snd_usb_hw_free(subs)
    }
}

// free-wheeling mode? (e.g. dmix)
fn in_free_wheeling_mode(runtime: *const snd_pcm_runtime) -> bool {
    unsafe { (*runtime).stop_threshold > (*runtime).buffer_size }
}

// check whether early start is needed for playback stream
fn lowlatency_playback_available(
    runtime: *const snd_pcm_runtime,
    subs: *const snd_usb_substream,
) -> bool {
    unsafe {
        let chip = (*(*subs).stream).chip;

        if (*subs).direction != SNDRV_PCM_STREAM_PLAYBACK {
            return false;
        }
        // disabled via module option?
        if (*chip).lowlatency == 0 {
            return false;
        }
        if in_free_wheeling_mode(runtime) {
            return false;
        }
        // implicit feedback mode has own operation mode
        if snd_usb_endpoint_implicit_feedback_sink((*subs).data_endpoint) {
            return false;
        }
        true
    }
}

// prepare callback
// only a few subtle things...
#[no_mangle]
pub extern "C" fn snd_usb_pcm_prepare(substream: *mut snd_pcm_substream) -> i32 {
    unsafe {
        let runtime = (*substream).runtime;
        let subs = (*runtime).private_data as *mut snd_usb_substream;
        let chip = (*(*subs).stream).chip;
        let mut retry = 0;

        let pm_err = snd_usb_lock_shutdown(chip);
        if pm_err < 0 {
            return pm_err;
        }

        if (*subs).data_endpoint.is_null() {
            snd_usb_unlock_shutdown(chip);
            return -EIO;
        }

        let mut ret = snd_usb_pcm_change_state(subs, UAC3_PD_STATE_D0);
        if ret < 0 {
            snd_usb_unlock_shutdown(chip);
            return ret;
        }

        loop {
            if !(*subs).sync_endpoint.is_null() {
                ret = snd_usb_endpoint_prepare(chip, (*subs).sync_endpoint);
                if ret < 0 {
                    snd_usb_unlock_shutdown(chip);
                    return ret;
                }
            }

            ret = snd_usb_endpoint_prepare(chip, (*subs).data_endpoint);
            if ret < 0 {
                snd_usb_unlock_shutdown(chip);
                return ret;
            } else if ret > 0 {
                snd_usb_set_format_quirk(subs, (*subs).cur_audiofmt);
            }
            ret = 0;

            // reset the pointer
            (*subs).buffer_bytes = frames_to_bytes(runtime, (*runtime).buffer_size);
            (*subs).inflight_bytes = 0;
            (*subs).hwptr_done = 0;
            (*subs).transfer_done = 0;
            (*subs).last_frame_number = 0;
            (*subs).period_elapsed_pending = 0;
            (*runtime).delay = 0;

            (*subs).lowlatency_playback = lowlatency_playback_available(runtime, subs);
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK && (*subs).lowlatency_playback == 0 {
                ret = start_endpoints(subs);
                // if XRUN happens at starting streams (possibly with implicit fb case),
                // restart again, but only try once.
                if ret == -EPIPE && retry == 0 {
                    retry = 1;
                    sync_pending_stops(subs);
                    continue;
                }
            }

            break;
        }

        snd_usb_unlock_shutdown(chip);
        ret
    }
}

// h/w constraints

// hwc_debug macro equivalent
#[allow(dead_code)]
macro_rules! hwc_debug {
    ($($arg:tt)*) => {
        #[cfg(feature = "HW_CONST_DEBUG")]
        {
            // pr_debug implementation would go here
        }
    };
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_pcm_hardware {
    info: u32,
    channels_min: u32,
    channels_max: u32,
    buffer_bytes_max: u32,
    period_bytes_min: u32,
    period_bytes_max: u32,
    periods_min: u32,
    periods_max: u32,
}

const SND_USB_HARDWARE: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_BATCH
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_PAUSE,
    channels_min: 1,
    channels_max: 256,
    buffer_bytes_max: i32::MAX as u32,
    period_bytes_min: 64,
    period_bytes_max: i32::MAX as u32,
    periods_min: 2,
    periods_max: 1024,
};

fn hw_check_valid_format(
    subs: *mut snd_usb_substream,
    params: *mut snd_pcm_hw_params,
    fp: *const audioformat,
) -> i32 {
    unsafe {
        let it = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
        let ct = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
        let fmts = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
        let pt = hw_param_interval(params, SNDRV_PCM_HW_PARAM_PERIOD_TIME);

        // check the format
        let mut check_fmts: snd_mask = std::mem::zeroed();
        check_fmts.bits[0] = (*fp).formats as u32;
        check_fmts.bits[1] = ((*fp).formats >> 32) as u32;
        snd_mask_intersect(&mut check_fmts, fmts);
        if snd_mask_empty(&check_fmts) != 0 {
            hwc_debug!("   > check: no supported format 0x%llx\n", (*fp).formats);
            return 0;
        }

        // check the channels
        if (*fp).channels < (*it).min || (*fp).channels > (*it).max {
            hwc_debug!("   > check: no valid channels %d (%d/%d)\n", (*fp).channels, (*it).min, (*it).max);
            return 0;
        }

        // check the rate is within the range
        if (*fp).rate_min > (*it).max || ((*fp).rate_min == (*it).max && (*it).openmax != 0) {
            hwc_debug!("   > check: rate_min %d > max %d\n", (*fp).rate_min, (*it).max);
            return 0;
        }
        if (*fp).rate_max < (*it).min || ((*fp).rate_max == (*it).min && (*it).openmin != 0) {
            hwc_debug!("   > check: rate_max %d < min %d\n", (*fp).rate_max, (*it).min);
            return 0;
        }

        // check whether the period time is >= the data packet interval
        if (*subs).speed != USB_SPEED_FULL {
            let ptime = 125 * (1 << (*fp).datainterval);
            if ptime > (*pt).max || (ptime == (*pt).max && (*pt).openmax != 0) {
                hwc_debug!("   > check: ptime %u > max %u\n", ptime, (*pt).max);
                return 0;
            }
        }

        1
    }
}

fn apply_hw_params_minmax(it: *mut snd_interval, rmin: u32, rmax: u32) -> i32 {
    unsafe {
        if rmin > rmax {
            hwc_debug!("  --> get empty\n");
            (*it).empty = 1;
            return -EINVAL;
        }

        let mut changed = 0;
        if (*it).min < rmin {
            (*it).min = rmin;
            (*it).openmin = 0;
            changed = 1;
        }
        if (*it).max > rmax {
            (*it).max = rmax;
            (*it).openmax = 0;
            changed = 1;
        }
        if snd_interval_checkempty(it) != 0 {
            (*it).empty = 1;
            return -EINVAL;
        }

        hwc_debug!("  --> (%d, %d) (changed = %d)\n", (*it).min, (*it).max, changed);
        changed
    }
}

// get the specified endpoint object that is being used by other streams
// (i.e. the parameter is locked)
fn get_endpoint_in_use(
    chip: *mut snd_usb_audio,
    endpoint: i32,
    ref_ep: *const snd_usb_endpoint,
) -> *const snd_usb_endpoint {
    unsafe {
        let ep = snd_usb_get_endpoint(chip, endpoint);
        if !ep.is_null()
            && !(*ep).cur_audiofmt.is_null()
            && (ep != ref_ep || (*ep).opened > 1)
        {
            ep
        } else {
            std::ptr::null()
        }
    }
}

fn hw_rule_rate(params: *mut snd_pcm_hw_params, rule: *mut snd_pcm_hw_rule) -> i32 {
    unsafe {
        let subs = (*rule).private as *mut snd_usb_substream;
        let chip = (*(*subs).stream).chip;
        let it = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
        let mut rmin = u32::MAX;
        let mut rmax: u32 = 0;

        hwc_debug!("hw_rule_rate: (%d,%d)\n", (*it).min, (*it).max);

        let mut fp: *const audioformat;
        let mut list_entry = (*(*subs).fmt_list).next;
        let fmt_list_head = &mut (*subs).fmt_list as *mut _;

        while list_entry != fmt_list_head {
            fp = list_entry as *const audioformat;

            if hw_check_valid_format(subs, params, fp) == 0 {
                list_entry = (*list_entry).next;
                continue;
            }

            let ep = get_endpoint_in_use(chip, (*fp).endpoint, (*subs).data_endpoint);
            if !ep.is_null() {
                hwc_debug!("rate limit %d for ep#%x\n", (*ep).cur_rate, (*fp).endpoint);
                rmin = rmin.min((*ep).cur_rate);
                rmax = rmax.max((*ep).cur_rate);
                list_entry = (*list_entry).next;
                continue;
            }

            if (*fp).implicit_fb != 0 {
                let ep = get_endpoint_in_use(chip, (*fp).sync_ep, (*subs).sync_endpoint);
                if !ep.is_null() {
                    hwc_debug!("rate limit %d for sync_ep#%x\n", (*ep).cur_rate, (*fp).sync_ep);
                    rmin = rmin.min((*ep).cur_rate);
                    rmax = rmax.max((*ep).cur_rate);
                    list_entry = (*list_entry).next;
                    continue;
                }
            }

            let r = snd_usb_endpoint_get_clock_rate(chip, (*fp).clock);
            if r > 0 {
                if snd_interval_test(it, r) == 0 {
                    list_entry = (*list_entry).next;
                    continue;
                }
                rmin = rmin.min(r as u32);
                rmax = rmax.max(r as u32);
                list_entry = (*list_entry).next;
                continue;
            }

            if !(*fp).rate_table.is_null() && (*fp).nr_rates > 0 {
                let mut i = 0;
                while i < (*fp).nr_rates {
                    let r = *(*fp).rate_table.add(i);
                    if snd_interval_test(it, r) == 0 {
                        i += 1;
                        continue;
                    }
                    rmin = rmin.min(r);
                    rmax = rmax.max(r);
                    i += 1;
                }
            } else {
                rmin = rmin.min((*fp).rate_min);
                rmax = rmax.max((*fp).rate_max);
            }

            list_entry = (*list_entry).next;
        }

        apply_hw_params_minmax(it, rmin, rmax)
    }
}

fn hw_rule_channels(params: *mut snd_pcm_hw_params, rule: *mut snd_pcm_hw_rule) -> i32 {
    unsafe {
        let subs = (*rule).private as *mut snd_usb_substream;
        let it = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
        let mut rmin = u32::MAX;
        let mut rmax: u32 = 0;

        hwc_debug!("hw_rule_channels: (%d,%d)\n", (*it).min, (*it).max);

        let mut fp: *const audioformat;
        let mut list_entry = (*(*subs).fmt_list).next;
        let fmt_list_head = &mut (*subs).fmt_list as *mut _;

        while list_entry != fmt_list_head {
            fp = list_entry as *const audioformat;

            if hw_check_valid_format(subs, params, fp) == 0 {
                list_entry = (*list_entry).next;
                continue;
            }

            rmin = rmin.min((*fp).channels);
            rmax = rmax.max((*fp).channels);

            list_entry = (*list_entry).next;
        }

        apply_hw_params_minmax(it, rmin, rmax)
    }
}

fn apply_hw_params_format_bits(fmt: *mut snd_mask, fbits: u64) -> i32 {
    unsafe {
        let oldbits = [(*fmt).bits[0], (*fmt).bits[1]];
        (*fmt).bits[0] &= fbits as u32;
        (*fmt).bits[1] &= (fbits >> 32) as u32;

        if (*fmt).bits[0] == 0 && (*fmt).bits[1] == 0 {
            hwc_debug!("  --> get empty\n");
            return -EINVAL;
        }

        let changed = (oldbits[0] != (*fmt).bits[0]) || (oldbits[1] != (*fmt).bits[1]);
        hwc_debug!("  --> %x:%x (changed = %d)\n", (*fmt).bits[0], (*fmt).bits[1], if changed { 1 } else { 0 });

        if changed { 1 } else { 0 }
    }
}

fn hw_rule_format(params: *mut snd_pcm_hw_params, rule: *mut snd_pcm_hw_rule) -> i32 {
    unsafe {
        let subs = (*rule).private as *mut snd_usb_substream;
        let chip = (*(*subs).stream).chip;
        let fmt = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
        let mut fbits: u64 = 0;

        hwc_debug!("hw_rule_format: %x:%x\n", (*fmt).bits[0], (*fmt).bits[1]);

        let mut fp: *const audioformat;
        let mut list_entry = (*(*subs).fmt_list).next;
        let fmt_list_head = &mut (*subs).fmt_list as *mut _;

        while list_entry != fmt_list_head {
            fp = list_entry as *const audioformat;

            if hw_check_valid_format(subs, params, fp) == 0 {
                list_entry = (*list_entry).next;
                continue;
            }

            let ep = get_endpoint_in_use(chip, (*fp).endpoint, (*subs).data_endpoint);
            if !ep.is_null() {
                hwc_debug!("format limit %d for ep#%x\n", (*ep).cur_format, (*fp).endpoint);
                fbits |= pcm_format_to_bits((*ep).cur_format);
                list_entry = (*list_entry).next;
                continue;
            }

            if (*fp).implicit_fb != 0 {
                let ep = get_endpoint_in_use(chip, (*fp).sync_ep, (*subs).sync_endpoint);
                if !ep.is_null() {
                    hwc_debug!("format limit %d for sync_ep#%x\n", (*ep).cur_format, (*fp).sync_ep);
                    fbits |= pcm_format_to_bits((*ep).cur_format);
                    list_entry = (*list_entry).next;
                    continue;
                }
            }

            fbits |= (*fp).formats;

            list_entry = (*list_entry).next;
        }

        apply_hw_params_format_bits(fmt, fbits)
    }
}

fn hw_rule_period_time(params: *mut snd_pcm_hw_params, rule: *mut snd_pcm_hw_rule) -> i32 {
    unsafe {
        let subs = (*rule).private as *mut snd_usb_substream;
        let it = hw_param_interval(params, SNDRV_PCM_HW_PARAM_PERIOD_TIME);
        let mut min_datainterval: u8 = 0xff;

        hwc_debug!("hw_rule_period_time: (%u,%u)\n", (*it).min, (*it).max);

        let mut fp: *const audioformat;
        let mut list_entry = (*(*subs).fmt_list).next;
        let fmt_list_head = &mut (*subs).fmt_list as *mut _;

        while list_entry != fmt_list_head {
            fp = list_entry as *const audioformat;

            if hw_check_valid_format(subs, params, fp) == 0 {
                list_entry = (*list_entry).next;
                continue;
            }

            min_datainterval = min_datainterval.min((*fp).datainterval);

            list_entry = (*list_entry).next;
        }

        if min_datainterval == 0xff {
            hwc_debug!("  --> get empty\n");
            (*it).empty = 1;
            return -EINVAL;
        }

        let pmin = 125 * (1 << min_datainterval);
        apply_hw_params_minmax(it, pmin as u32, u32::MAX)
    }
}

// additional hw constraints for implicit feedback mode
fn hw_rule_period_size_implicit_fb(params: *mut snd_pcm_hw_params, rule: *mut snd_pcm_hw_rule) -> i32 {
    unsafe {
        let subs = (*rule).private as *mut snd_usb_substream;
        let chip = (*(*subs).stream).chip;
        let it = hw_param_interval(params, SNDRV_PCM_HW_PARAM_PERIOD_SIZE);
        let mut rmin = u32::MAX;
        let mut rmax: u32 = 0;

        hwc_debug!("hw_rule_period_size: (%u,%u)\n", (*it).min, (*it).max);

        let mut fp: *const audioformat;
        let mut list_entry = (*(*subs).fmt_list).next;
        let fmt_list_head = &mut (*subs).fmt_list as *mut _;

        while list_entry != fmt_list_head {
            fp = list_entry as *const audioformat;

            if hw_check_valid_format(subs, params, fp) == 0 {
                list_entry = (*list_entry).next;
                continue;
            }

            let ep = get_endpoint_in_use(chip, (*fp).endpoint, (*subs).data_endpoint);
            if !ep.is_null() {
                hwc_debug!("period size limit %d for ep#%x\n", (*ep).cur_period_frames, (*fp).endpoint);
                rmin = rmin.min((*ep).cur_period_frames);
                rmax = rmax.max((*ep).cur_period_frames);
                list_entry = (*list_entry).next;
                continue;
            }

            if (*fp).implicit_fb != 0 {
                let ep = get_endpoint_in_use(chip, (*fp).sync_ep, (*subs).sync_endpoint);
                if !ep.is_null() {
                    hwc_debug!("period size limit %d for sync_ep#%x\n", (*ep).cur_period_frames, (*fp).sync_ep);
                    rmin = rmin.min((*ep).cur_period_frames);
                    rmax = rmax.max((*ep).cur_period_frames);
                    list_entry = (*list_entry).next;
                    continue;
                }
            }

            list_entry = (*list_entry).next;
        }

        if rmax == 0 {
            return 0; // no limit by implicit fb
        }

        apply_hw_params_minmax(it, rmin, rmax)
    }
}

fn hw_rule_periods_implicit_fb(params: *mut snd_pcm_hw_params, rule: *mut snd_pcm_hw_rule) -> i32 {
    unsafe {
        let subs = (*rule).private as *mut snd_usb_substream;
        let chip = (*(*subs).stream).chip;
        let it = hw_param_interval(params, SNDRV_PCM_HW_PARAM_PERIODS);
        let mut rmin = u32::MAX;
        let mut rmax: u32 = 0;

        hwc_debug!("hw_rule_periods: (%u,%u)\n", (*it).min, (*it).max);

        let mut fp: *const audioformat;
        let mut list_entry = (*(*subs).fmt_list).next;
        let fmt_list_head = &mut (*subs).fmt_list as *mut _;

        while list_entry != fmt_list_head {
            fp = list_entry as *const audioformat;

            if hw_check_valid_format(subs, params, fp) == 0 {
                list_entry = (*list_entry).next;
                continue;
            }

            let ep = get_endpoint_in_use(chip, (*fp).endpoint, (*subs).data_endpoint);
            if !ep.is_null() {
                hwc_debug!("periods limit %d for ep#%x\n", (*ep).cur_buffer_periods, (*fp).endpoint);
                rmin = rmin.min((*ep).cur_buffer_periods);
                rmax = rmax.max((*ep).cur_buffer_periods);
                list_entry = (*list_entry).next;
                continue;
            }

            if (*fp).implicit_fb != 0 {
                let ep = get_endpoint_in_use(chip, (*fp).sync_ep, (*subs).sync_endpoint);
                if !ep.is_null() {
                    hwc_debug!("periods limit %d for sync_ep#%x\n", (*ep).cur_buffer_periods, (*fp).sync_ep);
                    rmin = rmin.min((*ep).cur_buffer_periods);
                    rmax = rmax.max((*ep).cur_buffer_periods);
                    list_entry = (*list_entry).next;
                    continue;
                }
            }

            list_entry = (*list_entry).next;
        }

        if rmax == 0 {
            return 0; // no limit by implicit fb
        }

        apply_hw_params_minmax(it, rmin, rmax)
    }
}

// set up the runtime hardware information.
fn setup_hw_info(runtime: *mut snd_pcm_runtime, subs: *mut snd_usb_substream) -> i32 {
    unsafe {
        (*runtime).hw.formats = (*subs).formats;

        (*runtime).hw.rate_min = 0x7fffffff;
        (*runtime).hw.rate_max = 0;
        (*runtime).hw.channels_min = 256;
        (*runtime).hw.channels_max = 0;
        (*runtime).hw.rates = 0;
        let mut ptmin = u32::MAX;

        // check min/max rates and channels
        let mut fp: *const audioformat;
        let mut list_entry = (*(*subs).fmt_list).next;
        let fmt_list_head = &mut (*subs).fmt_list as *mut _;

        while list_entry != fmt_list_head {
            fp = list_entry as *const audioformat;

            (*runtime).hw.rates |= (*fp).rates;
            if (*runtime).hw.rate_min > (*fp).rate_min {
                (*runtime).hw.rate_min = (*fp).rate_min;
            }
            if (*runtime).hw.rate_max < (*fp).rate_max {
                (*runtime).hw.rate_max = (*fp).rate_max;
            }
            if (*runtime).hw.channels_min > (*fp).channels {
                (*runtime).hw.channels_min = (*fp).channels;
            }
            if (*runtime).hw.channels_max < (*fp).channels {
                (*runtime).hw.channels_max = (*fp).channels;
            }
            if (*fp).fmt_type == UAC_FORMAT_TYPE_II && (*fp).frame_size > 0 {
                // FIXME: there might be more than one audio formats...
                (*runtime).hw.period_bytes_min = (*fp).frame_size;
                (*runtime).hw.period_bytes_max = (*fp).frame_size;
            }
            let pt = 125 * (1 << (*fp).datainterval);
            ptmin = ptmin.min(pt as u32);

            list_entry = (*list_entry).next;
        }

        let mut param_period_time_if_needed = SNDRV_PCM_HW_PARAM_PERIOD_TIME as i32;
        if (*subs).speed == USB_SPEED_FULL {
            // full speed devices have fixed data packet interval
            ptmin = 1000;
        }
        if ptmin == 1000 {
            // if period time doesn't go below 1 ms, no rules needed
            param_period_time_if_needed = -1;
        }

        let mut err = snd_pcm_hw_constraint_minmax(
            runtime,
            SNDRV_PCM_HW_PARAM_PERIOD_TIME,
            ptmin,
            u32::MAX,
        );
        if err < 0 {
            return err;
        }

        err = snd_pcm_hw_rule_add(
            runtime,
            0,
            SNDRV_PCM_HW_PARAM_RATE,
            Some(hw_rule_rate),
            subs as *mut _,
            SNDRV_PCM_HW_PARAM_RATE,
            SNDRV_PCM_HW_PARAM_FORMAT,
            SNDRV_PCM_HW_PARAM_CHANNELS,
            param_period_time_if_needed,
            -1,
        );
        if err < 0 {
            return err;
        }

        err = snd_pcm_hw_rule_add(
            runtime,
            0,
            SNDRV_PCM_HW_PARAM_CHANNELS,
            Some(hw_rule_channels),
            subs as *mut _,
            SNDRV_PCM_HW_PARAM_CHANNELS,
            SNDRV_PCM_HW_PARAM_FORMAT,
            SNDRV_PCM_HW_PARAM_RATE,
            param_period_time_if_needed,
            -1,
        );
        if err < 0 {
            return err;
        }

        err = snd_pcm_hw_rule_add(
            runtime,
            0,
            SNDRV_PCM_HW_PARAM_FORMAT,
            Some(hw_rule_format),
            subs as *mut _,
            SNDRV_PCM_HW_PARAM_FORMAT,
            SNDRV_PCM_HW_PARAM_RATE,
            SNDRV_PCM_HW_PARAM_CHANNELS,
            param_period_time_if_needed,
            -1,
        );
        if err < 0 {
            return err;
        }

        if param_period_time_if_needed >= 0 {
            err = snd_pcm_hw_rule_add(
                runtime,
                0,
                SNDRV_PCM_HW_PARAM_PERIOD_TIME,
                Some(hw_rule_period_time),
                subs as *mut _,
                SNDRV_PCM_HW_PARAM_FORMAT,
                SNDRV_PCM_HW_PARAM_CHANNELS,
                SNDRV_PCM_HW_PARAM_RATE,
                -1,
            );
            if err < 0 {
                return err;
            }
        }

        // set max period and buffer sizes for 1 and 2 seconds, respectively
        err = snd_pcm_hw_constraint_minmax(
            runtime,
            SNDRV_PCM_HW_PARAM_PERIOD_TIME,
            0,
            1000000,
        );
        if err < 0 {
            return err;
        }

        err = snd_pcm_hw_constraint_minmax(
            runtime,
            SNDRV_PCM_HW_PARAM_BUFFER_TIME,
            0,
            2000000,
        );
        if err < 0 {
            return err;
        }

        // additional hw constraints for implicit fb
        err = snd_pcm_hw_rule_add(
            runtime,
            0,
            SNDRV_PCM_HW_PARAM_PERIOD_SIZE,
            Some(hw_rule_period_size_implicit_fb),
            subs as *mut _,
            SNDRV_PCM_HW_PARAM_PERIOD_SIZE,
            -1,
        );
        if err < 0 {
            return err;
        }

        err = snd_pcm_hw_rule_add(
            runtime,
            0,
            SNDRV_PCM_HW_PARAM_PERIODS,
            Some(hw_rule_periods_implicit_fb),
            subs as *mut _,
            SNDRV_PCM_HW_PARAM_PERIODS,
            -1,
        );
        if err < 0 {
            return err;
        }

        let mut list_entry = (*(*subs).fmt_list).next;
        let fmt_list_head = &mut (*subs).fmt_list as *mut _;

        while list_entry != fmt_list_head {
            let fp = list_entry as *const audioformat;
            if (*fp).implicit_fb != 0 {
                (*runtime).hw.info |= SNDRV_PCM_INFO_JOINT_DUPLEX;
                break;
            }
            list_entry = (*list_entry).next;
        }

        0
    }
}

#[no_mangle]
pub extern "C" fn snd_usb_pcm_open(substream: *mut snd_pcm_substream) -> i32 {
    unsafe {
        let direction = (*substream).stream;
        let as_ = snd_pcm_substream_chip(substream) as *mut snd_usb_stream;
        let runtime = (*substream).runtime;
        let subs = &mut (*as_).substream[direction as usize];
        let chip = (*(*subs).stream).chip;

        // scoped_guard equivalent
        {
            let _guard = mutex_guard(&(*chip).mutex);
            if (*subs).opened != 0 {
                return -EBUSY;
            }
            (*subs).opened = 1;
        }

        (*runtime).hw = SND_USB_HARDWARE;

        // need an explicit sync to catch applptr update in low-latency mode
        if direction == SNDRV_PCM_STREAM_PLAYBACK && (*(*as_).chip).lowlatency != 0 {
            (*runtime).hw.info |= SNDRV_PCM_INFO_SYNC_APPLPTR;
        }

        (*runtime).private_data = subs as *mut _ as *mut libc::c_void;
        (*subs).pcm_substream = substream;

        // initialize DSD/DOP context
        (*subs).dsd_dop.byte_idx = 0;
        (*subs).dsd_dop.channel = 0;
        (*subs).dsd_dop.marker = 1;

        let mut ret = setup_hw_info(runtime, subs);
        if ret < 0 {
            goto_err_resume(subs, chip, ret);
            return ret;
        }

        ret = snd_usb_autoresume((*subs).stream.chip);
        if ret < 0 {
            goto_err_open(subs, chip);
            return ret;
        }

        ret = snd_media_stream_init(subs, (*as_).pcm, direction);
        if ret < 0 {
            snd_usb_autosuspend((*subs).stream.chip);
            goto_err_open(subs, chip);
            return ret;
        }

        0
    }

    #[inline]
    unsafe fn goto_err_open(subs: *mut snd_usb_substream, chip: *mut snd_usb_audio) {
        let _guard = mutex_guard(&(*chip).mutex);
        (*subs).opened = 0;
    }

    #[inline]
    unsafe fn goto_err_resume(
        subs: *mut snd_usb_substream,
        chip: *mut snd_usb_audio,
        _ret: i32,
    ) {
        snd_usb_autosuspend((*subs).stream.chip);
        goto_err_open(subs, chip);
    }
}

#[no_mangle]
pub extern "C" fn snd_usb_pcm_close(substream: *mut snd_pcm_substream) -> i32 {
    unsafe {
        let direction = (*substream).stream;
        let as_ = snd_pcm_substream_chip(substream) as *mut snd_usb_stream;
        let subs = &mut (*as_).substream[direction as usize];
        let chip = (*(*subs).stream).chip;

        snd_media_stop_pipeline(subs);

        {
            let pm_err = snd_usb_lock_shutdown(chip);
            if pm_err != 0 {
                return pm_err;
            }
            let ret = snd_usb_pcm_change_state(subs, UAC3_PD_STATE_D1);
            snd_usb_unlock_shutdown(chip);
            if ret < 0 {
                return ret;
            }
        }

        (*subs).pcm_substream = std::ptr::null_mut();
        snd_usb_autosuspend((*subs).stream.chip);

        {
            let _guard = mutex_guard(&(*chip).mutex);
            (*subs).opened = 0;
        }

        0
    }
}

// Since a URB can handle only a single linear buffer, we must use double
// buffering when the data to be transferred overflows the buffer boundary.
// To avoid inconsistencies when updating hwptr_done, we use double buffering
// for all URBs.
fn retire_capture_urb(subs: *mut snd_usb_substream, urb: *mut urb) {
    unsafe {
        let runtime = (*(*subs).pcm_substream).runtime;
        let stride = ((*runtime).frame_bits >> 3) as usize;
        let mut period_elapsed = 0;

        // read frame number here, update pointer in critical section
        let current_frame_number = usb_get_current_frame_number((*subs).dev);

        for i in 0..(*urb).number_of_packets {
            let cp = ((*urb).transfer_buffer as *mut u8)
                .add((*urb).iso_frame_desc[i].offset as usize)
                .add((*subs).pkt_offset_adj as usize);

            if (*urb).iso_frame_desc[i].status != 0 {
                dev_dbg_ratelimited(
                    &(*subs).dev.dev as *const _,
                    "frame %d active: %d\n".as_ptr(),
                    i as i32,
                    (*urb).iso_frame_desc[i].status,
                );
            }

            let mut bytes = (*urb).iso_frame_desc[i].actual_length as usize;

            if (*subs).stream_offset_adj > 0 {
                let adj = ((*subs).stream_offset_adj as usize).min(bytes);
                bytes -= adj;
                (*subs).stream_offset_adj -= adj as u32;
            }

            let mut frames = bytes / stride;
            if (*subs).txfr_quirk == 0 {
                bytes = frames * stride;
            }

            if bytes % (((*runtime).sample_bits >> 3) as usize) != 0 {
                let oldbytes = bytes;
                bytes = frames * stride;
                dev_warn_ratelimited(
                    &(*subs).dev.dev as *const _,
                    "Corrected urb data len. %d->%d\n".as_ptr(),
                    oldbytes as i32,
                    bytes as i32,
                );
            }

            // update the current pointer
            {
                let _guard = spinlock_irqsave_guard(&(*subs).lock);
                let oldptr = (*subs).hwptr_done as usize;
                (*subs).hwptr_done = (oldptr + bytes) as u32;
                if (*subs).hwptr_done as usize >= (*subs).buffer_bytes as usize {
                    (*subs).hwptr_done =
                        ((*subs).hwptr_done as usize - (*subs).buffer_bytes as usize) as u32;
                }
                frames = (bytes + (oldptr % stride)) / stride;
                (*subs).transfer_done += frames as u32;
                if (*subs).transfer_done >= (*runtime).period_size {
                    (*subs).transfer_done -= (*runtime).period_size;
                    period_elapsed = 1;
                }

                // realign last_frame_number
                (*subs).last_frame_number = current_frame_number;

                // copy a data chunk
                if oldptr + bytes > (*subs).buffer_bytes as usize {
                    let bytes1 = (*subs).buffer_bytes as usize - oldptr;
                    std::ptr::copy_nonoverlapping(
                        cp,
                        ((*runtime).dma_area as *mut u8).add(oldptr),
                        bytes1,
                    );
                    std::ptr::copy_nonoverlapping(
                        cp.add(bytes1),
                        (*runtime).dma_area as *mut u8,
                        bytes - bytes1,
                    );
                } else {
                    std::ptr::copy_nonoverlapping(
                        cp,
                        ((*runtime).dma_area as *mut u8).add(oldptr),
                        bytes,
                    );
                }
            }
        }

        if period_elapsed != 0 {
            snd_pcm_period_elapsed((*subs).pcm_substream);
        }
    }
}

fn urb_ctx_queue_advance(subs: *mut snd_usb_substream, urb: *mut urb, bytes: u32) {
    unsafe {
        let ctx = (*urb).context as *mut snd_urb_ctx;
        (*ctx).queued += bytes;
        (*subs).inflight_bytes += bytes;
        (*subs).hwptr_done += bytes;
        if (*subs).hwptr_done >= (*subs).buffer_bytes {
            (*subs).hwptr_done -= (*subs).buffer_bytes;
        }
    }
}

fn fill_playback_urb_dsd_dop(subs: *mut snd_usb_substream, urb: *mut urb, bytes: u32) {
    unsafe {
        let runtime = (*(*subs).pcm_substream).runtime;
        let mut dst_idx: u32 = 0;
        let mut src_idx = (*subs).hwptr_done;
        let wrap = (*subs).buffer_bytes;
        let dst = (*urb).transfer_buffer as *mut u8;
        let src = (*runtime).dma_area as *mut u8;
        let marker = [0x05u8, 0xfau8];
        let mut queued: u32 = 0;

        // The DSP DOP format defines a way to transport DSD samples over
        // normal PCM data endpoints. It requires stuffing of marker bytes
        // (0x05 and 0xfa, alternating per sample frame), and then expects
        // 2 additional bytes of actual payload. The whole frame is stored LSB.

        let mut b = bytes;
        while b > 0 {
            (*subs).dsd_dop.byte_idx += 1;
            if (*subs).dsd_dop.byte_idx == 3 {
                // frame boundary?
                *dst.add(dst_idx as usize) = marker[(*subs).dsd_dop.marker as usize];
                dst_idx += 1;
                src_idx += 2;
                (*subs).dsd_dop.byte_idx = 0;

                (*subs).dsd_dop.channel += 1;
                if (*subs).dsd_dop.channel % (*runtime).channels == 0 {
                    // alternate the marker
                    (*subs).dsd_dop.marker += 1;
                    (*subs).dsd_dop.marker %= marker.len() as u32;
                    (*subs).dsd_dop.channel = 0;
                }
            } else {
                // stuff the DSD payload
                let idx = ((src_idx + (*subs).dsd_dop.byte_idx as u32 - 1) % wrap) as usize;

                if (*(*subs).cur_audiofmt).dsd_bitrev != 0 {
                    *dst.add(dst_idx as usize) = bitrev8(*src.add(idx));
                } else {
                    *dst.add(dst_idx as usize) = *src.add(idx);
                }
                dst_idx += 1;
                queued += 1;
            }
            b -= 1;
        }

        urb_ctx_queue_advance(subs, urb, queued);
    }
}

// copy bit-reversed bytes onto transfer buffer
fn fill_playback_urb_dsd_bitrev(subs: *mut snd_usb_substream, urb: *mut urb, bytes: u32) {
    unsafe {
        let runtime = (*(*subs).pcm_substream).runtime;
        let src = (*runtime).dma_area as *mut u8;
        let buf = (*urb).transfer_buffer as *mut u8;
        let mut ofs = (*subs).hwptr_done as i32;

        for i in 0..bytes {
            *buf.add(i as usize) = bitrev8(*src.add(ofs as usize));
            ofs += 1;
            if ofs as u32 >= (*subs).buffer_bytes {
                ofs = 0;
            }
        }

        urb_ctx_queue_advance(subs, urb, bytes);
    }
}

fn copy_to_urb(subs: *mut snd_usb_substream, urb: *mut urb, offset: i32, stride: i32, bytes: u32) {
    unsafe {
        let runtime = (*(*subs).pcm_substream).runtime;

        if (*subs).hwptr_done + bytes > (*subs).buffer_bytes {
            // err, the transferred area goes over buffer boundary.
            let bytes1 = (*subs).buffer_bytes - (*subs).hwptr_done;

            std::ptr::copy_nonoverlapping(
                ((*runtime).dma_area as *mut u8).add((*subs).hwptr_done as usize),
                ((*urb).transfer_buffer as *mut u8).add(offset as usize),
                bytes1 as usize,
            );
            std::ptr::copy_nonoverlapping(
                (*runtime).dma_area as *mut u8,
                ((*urb).transfer_buffer as *mut u8)
                    .add((offset as u32 + bytes1) as usize),
                (bytes - bytes1) as usize,
            );
        } else {
            std::ptr::copy_nonoverlapping(
                ((*runtime).dma_area as *mut u8).add((*subs).hwptr_done as usize),
                ((*urb).transfer_buffer as *mut u8).add(offset as usize),
                bytes as usize,
            );
        }

        urb_ctx_queue_advance(subs, urb, bytes);
    }
}

fn copy_to_urb_quirk(subs: *mut snd_usb_substream, urb: *mut urb, stride: i32, bytes: u32) -> u32 {
    unsafe {
        let mut packet_length: u32;

        // Put __le32 length descriptor at start of each packet.
        for i in 0..(*urb).number_of_packets {
            let length = (*urb).iso_frame_desc[i].length;
            let mut offset = (*urb).iso_frame_desc[i].offset;

            packet_length = cpu_to_le32(length);
            offset += (i as u32) * std::mem::size_of::<u32>() as u32;
            (*urb).iso_frame_desc[i].offset = offset;
            (*urb).iso_frame_desc[i].length += std::mem::size_of::<u32>() as u32;

            std::ptr::copy_nonoverlapping(
                &packet_length as *const _ as *const u8,
                ((*urb).transfer_buffer as *mut u8).add(offset as usize),
                std::mem::size_of::<u32>(),
            );
            copy_to_urb(
                subs,
                urb,
                (offset + std::mem::size_of::<u32>() as u32) as i32,
                stride,
                length,
            );
        }

        // Adjust transfer size accordingly.
        bytes + ((*urb).number_of_packets as u32) * std::mem::size_of::<u32>() as u32
    }
}

fn prepare_playback_urb(
    subs: *mut snd_usb_substream,
    urb: *mut urb,
    in_stream_lock: bool,
) -> i32 {
    unsafe {
        let runtime = (*(*subs).pcm_substream).runtime;
        let ep = (*subs).data_endpoint;
        let ctx = (*urb).context as *mut snd_urb_ctx;
        let mut frames: u32 = 0;
        let mut period_elapsed = 0;

        let stride = (*ep).stride;

        (*ctx).queued = 0;
        (*urb).number_of_packets = 0;

        {
            let _guard = spinlock_irqsave_guard(&(*subs).lock);

            let frame_limit = (*subs).frame_limit + (*ep).max_urb_frames;
            let mut transfer_done = (*subs).transfer_done;
            let mut avail: u32 = 0;

            if (*subs).lowlatency_playback != 0 && (*runtime).state != SNDRV_PCM_STATE_DRAINING {
                let hwptr = (*subs).hwptr_done / stride as u32;

                // calculate the byte offset-in-buffer of the appl_ptr
                avail = (((*(*runtime).control).appl_ptr - (*runtime).hw_ptr_base)
                    % (*runtime).buffer_size) as u32;
                if avail <= hwptr {
                    avail += (*runtime).buffer_size;
                }
                avail -= hwptr;
            }

            for i in 0..(*ctx).packets {
                let counts = snd_usb_endpoint_next_packet_size(ep, ctx, i as i32, avail);
                if counts < 0 || (frames as i32 + counts) * stride > (*ctx).buffer_size as i32 {
                    break;
                }

                // set up descriptor
                (*urb).iso_frame_desc[i as usize].offset = frames * stride as u32;
                (*urb).iso_frame_desc[i as usize].length = (counts * stride) as u32;
                frames += counts as u32;
                avail = avail.saturating_sub(counts as u32);
                (*urb).number_of_packets += 1;
                transfer_done += counts as u32;

                if transfer_done >= (*runtime).period_size {
                    transfer_done -= (*runtime).period_size;
                    period_elapsed = 1;

                    if (*subs).fmt_type == UAC_FORMAT_TYPE_II {
                        if transfer_done > 0 {
                            // FIXME: fill-max mode is not supported yet
                            frames -= transfer_done;
                            (*urb).iso_frame_desc[i as usize].length =
                                ((counts as u32 - transfer_done) * stride as u32);
                            transfer_done = 0;
                        }
                        if (i + 1) < (*ctx).packets {
                            // add a transfer delimiter
                            let next_i = i + 1;
                            (*urb).iso_frame_desc[next_i as usize].offset = frames * stride as u32;
                            (*urb).iso_frame_desc[next_i as usize].length = 0;
                            (*urb).number_of_packets += 1;
                        }
                        break;
                    }
                }

                // finish at the period boundary or after enough frames
                if (period_elapsed != 0
                    || transfer_done >= frame_limit)
                    && snd_usb_endpoint_implicit_feedback_sink(ep) == 0
                {
                    break;
                }
            }

            if frames == 0 {
                return -EAGAIN;
            }

            let bytes = frames * stride as u32;
            (*subs).transfer_done = transfer_done;
            (*subs).frame_limit = frame_limit;

            if unlikely(
                (*ep).cur_format == SNDRV_PCM_FORMAT_DSD_U16_LE
                    && (*(*subs).cur_audiofmt).dsd_dop != 0,
            ) {
                fill_playback_urb_dsd_dop(subs, urb, bytes);
            } else if unlikely(
                (*ep).cur_format == SNDRV_PCM_FORMAT_DSD_U8
                    && (*(*subs).cur_audiofmt).dsd_bitrev != 0,
            ) {
                fill_playback_urb_dsd_bitrev(subs, urb, bytes);
            } else {
                // usual PCM
                if (*subs).tx_length_quirk == 0 {
                    copy_to_urb(subs, urb, 0, stride, bytes);
                } else {
                    let adjusted_bytes = copy_to_urb_quirk(subs, urb, stride, bytes);
                    (*urb).transfer_buffer_length = adjusted_bytes;
                }
            }

            (*subs).last_frame_number = usb_get_current_frame_number((*subs).dev);

            if (*subs).trigger_tstamp_pending_update != 0 {
                // this is the first actual URB submitted,
                // update trigger timestamp to reflect actual start time
                snd_pcm_gettime(runtime, &mut (*runtime).trigger_tstamp as *mut _);
                (*subs).trigger_tstamp_pending_update = 0;
            }

            if period_elapsed != 0 && (*subs).running == 0 && (*subs).lowlatency_playback != 0 {
                (*subs).period_elapsed_pending = 1;
                period_elapsed = 0;
            }

            (*urb).transfer_buffer_length = frames * stride as u32;
        }

        if period_elapsed != 0 {
            if in_stream_lock {
                snd_pcm_period_elapsed_under_stream_lock((*subs).pcm_substream);
            } else {
                snd_pcm_period_elapsed((*subs).pcm_substream);
            }
        }

        0
    }

    #[inline]
    fn unlikely(cond: bool) -> bool {
        cond
    }
}

// process after playback data complete
// - decrease the delay count again
fn retire_playback_urb(subs: *mut snd_usb_substream, urb: *mut urb) {
    unsafe {
        let ctx = (*urb).context as *mut snd_urb_ctx;
        let mut period_elapsed = false;

        {
            let _guard = spinlock_irqsave_guard(&(*subs).lock);
            if (*ctx).queued > 0 {
                if (*subs).inflight_bytes >= (*ctx).queued {
                    (*subs).inflight_bytes -= (*ctx).queued;
                } else {
                    (*subs).inflight_bytes = 0;
                }
            }

            (*subs).last_frame_number = usb_get_current_frame_number((*subs).dev);
            if (*subs).running != 0 {
                period_elapsed = (*subs).period_elapsed_pending != 0;
                (*subs).period_elapsed_pending = 0;
            }
        }

        if period_elapsed {
            snd_pcm_period_elapsed((*subs).pcm_substream);
        }
    }
}

// PCM ack callback for the playback stream;
// this plays a role only when the stream is running in low-latency mode.
#[no_mangle]
pub extern "C" fn snd_usb_pcm_playback_ack(substream: *mut snd_pcm_substream) -> i32 {
    unsafe {
        let subs = (*(*substream).runtime).private_data as *mut snd_usb_substream;

        if (*subs).lowlatency_playback == 0 || (*subs).running == 0 {
            return 0;
        }

        let ep = (*subs).data_endpoint;
        if ep.is_null() {
            return 0;
        }

        // When no more in-flight URBs available, try to process the pending
        // outputs here
        if (*ep).active_mask == 0 {
            snd_usb_queue_pending_output_urbs(ep, true)
        } else {
            0
        }
    }
}

fn snd_usb_substream_playback_trigger(substream: *mut snd_pcm_substream, cmd: i32) -> i32 {
    unsafe {
        let subs = (*(*substream).runtime).private_data as *mut snd_usb_substream;

        match cmd {
            SNDRV_PCM_TRIGGER_START => {
                (*subs).trigger_tstamp_pending_update = 1;
                // fallthrough
                snd_usb_endpoint_set_callback(
                    (*subs).data_endpoint,
                    Some(prepare_playback_urb),
                    Some(retire_playback_urb),
                    subs as *mut _,
                );

                if (*subs).lowlatency_playback != 0 && cmd == SNDRV_PCM_TRIGGER_START {
                    if in_free_wheeling_mode((*substream).runtime) {
                        (*subs).lowlatency_playback = 0;
                    }
                    let err = start_endpoints(subs);
                    if err < 0 {
                        snd_usb_endpoint_set_callback(
                            (*subs).data_endpoint,
                            None,
                            None,
                            std::ptr::null_mut(),
                        );
                        return err;
                    }
                }

                (*subs).running = 1;
                dev_dbg(
                    &(*subs).dev.dev as *const _,
                    "%d:%d Start Playback PCM\n".as_ptr(),
                    (*(*subs).cur_audiofmt).iface,
                    (*(*subs).cur_audiofmt).altsetting,
                );
                0
            }
            SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
                snd_usb_endpoint_set_callback(
                    (*subs).data_endpoint,
                    Some(prepare_playback_urb),
                    Some(retire_playback_urb),
                    subs as *mut _,
                );

                if (*subs).lowlatency_playback != 0 {
                    if in_free_wheeling_mode((*substream).runtime) {
                        (*subs).lowlatency_playback = 0;
                    }
                    let err = start_endpoints(subs);
                    if err < 0 {
                        snd_usb_endpoint_set_callback(
                            (*subs).data_endpoint,
                            None,
                            None,
                            std::ptr::null_mut(),
                        );
                        return err;
                    }
                }

                (*subs).running = 1;
                dev_dbg(
                    &(*subs).dev.dev as *const _,
                    "%d:%d Start Playback PCM\n".as_ptr(),
                    (*(*subs).cur_audiofmt).iface,
                    (*(*subs).cur_audiofmt).altsetting,
                );
                0
            }
            SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP => {
                let draining = (*(*substream).runtime).state == SNDRV_PCM_STATE_DRAINING;
                stop_endpoints(subs, draining);
                snd_usb_endpoint_set_callback(
                    (*subs).data_endpoint,
                    None,
                    None,
                    std::ptr::null_mut(),
                );
                (*subs).running = 0;
                dev_dbg(
                    &(*subs).dev.dev as *const _,
                    "%d:%d Stop Playback PCM\n".as_ptr(),
                    (*(*subs).cur_audiofmt).iface,
                    (*(*subs).cur_audiofmt).altsetting,
                );
                0
            }
            SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
                // keep retire_data_urb for delay calculation
                snd_usb_endpoint_set_callback(
                    (*subs).data_endpoint,
                    None,
                    Some(retire_playback_urb),
                    subs as *mut _,
                );
                (*subs).running = 0;
                dev_dbg(
                    &(*subs).dev.dev as *const _,
                    "%d:%d Pause Playback PCM\n".as_ptr(),
                    (*(*subs).cur_audiofmt).iface,
                    (*(*subs).cur_audiofmt).altsetting,
                );
                0
            }
            _ => -EINVAL,
        }
    }
}

fn snd_usb_substream_capture_trigger(substream: *mut snd_pcm_substream, cmd: i32) -> i32 {
    unsafe {
        let subs = (*(*substream).runtime).private_data as *mut snd_usb_substream;

        match cmd {
            SNDRV_PCM_TRIGGER_START => {
                let err = start_endpoints(subs);
                if err < 0 {
                    return err;
                }
                // fallthrough
                snd_usb_endpoint_set_callback(
                    (*subs).data_endpoint,
                    None,
                    Some(retire_capture_urb),
                    subs as *mut _,
                );
                (*subs).last_frame_number = usb_get_current_frame_number((*subs).dev);
                (*subs).running = 1;
                dev_dbg(
                    &(*subs).dev.dev as *const _,
                    "%d:%d Start Capture PCM\n".as_ptr(),
                    (*(*subs).cur_audiofmt).iface,
                    (*(*subs).cur_audiofmt).altsetting,
                );
                0
            }
            SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
                snd_usb_endpoint_set_callback(
                    (*subs).data_endpoint,
                    None,
                    Some(retire_capture_urb),
                    subs as *mut _,
                );
                (*subs).last_frame_number = usb_get_current_frame_number((*subs).dev);
                (*subs).running = 1;
                dev_dbg(
                    &(*subs).dev.dev as *const _,
                    "%d:%d Start Capture PCM\n".as_ptr(),
                    (*(*subs).cur_audiofmt).iface,
                    (*(*subs).cur_audiofmt).altsetting,
                );
                0
            }
            SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP => {
                stop_endpoints(subs, false);
                // fallthrough
                snd_usb_endpoint_set_callback(
                    (*subs).data_endpoint,
                    None,
                    None,
                    std::ptr::null_mut(),
                );
                (*subs).running = 0;
                dev_dbg(
                    &(*subs).dev.dev as *const _,
                    "%d:%d Stop Capture PCM\n".as_ptr(),
                    (*(*subs).cur_audiofmt).iface,
                    (*(*subs).cur_audiofmt).altsetting,
                );
                0
            }
            SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
                snd_usb_endpoint_set_callback(
                    (*subs).data_endpoint,
                    None,
                    None,
                    std::ptr::null_mut(),
                );
                (*subs).running = 0;
                dev_dbg(
                    &(*subs).dev.dev as *const _,
                    "%d:%d Stop Capture PCM\n".as_ptr(),
                    (*(*subs).cur_audiofmt).iface,
                    (*(*subs).cur_audiofmt).altsetting,
                );
                0
            }
            _ => -EINVAL,
        }
    }
}

#[repr(C)]
struct snd_pcm_ops {
    open: Option<extern "C" fn(*mut snd_pcm_substream) -> i32>,
    close: Option<extern "C" fn(*mut snd_pcm_substream) -> i32>,
    hw_params: Option<extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> i32>,
    hw_free: Option<extern "C" fn(*mut snd_pcm_substream) -> i32>,
    prepare: Option<extern "C" fn(*mut snd_pcm_substream) -> i32>,
    trigger: Option<extern "C" fn(*mut snd_pcm_substream, i32) -> i32>,
    sync_stop: Option<extern "C" fn(*mut snd_pcm_substream) -> i32>,
    pointer: Option<extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    ack: Option<extern "C" fn(*mut snd_pcm_substream) -> i32>,
}

const SND_USB_PLAYBACK_OPS: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_usb_pcm_open),
    close: Some(snd_usb_pcm_close),
    hw_params: Some(snd_usb_pcm_hw_params),
    hw_free: Some(snd_usb_pcm_hw_free),
    prepare: Some(snd_usb_pcm_prepare),
    trigger: Some(snd_usb_substream_playback_trigger),
    sync_stop: Some(snd_usb_pcm_sync_stop),
    pointer: Some(snd_usb_pcm_pointer),
    ack: Some(snd_usb_pcm_playback_ack),
};

const SND_USB_CAPTURE_OPS: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_usb_pcm_open),
    close: Some(snd_usb_pcm_close),
    hw_params: Some(snd_usb_pcm_hw_params),
    hw_free: Some(snd_usb_pcm_hw_free),
    prepare: Some(snd_usb_pcm_prepare),
    trigger: Some(snd_usb_substream_capture_trigger),
    sync_stop: Some(snd_usb_pcm_sync_stop),
    pointer: Some(snd_usb_pcm_pointer),
    ack: None,
};

#[no_mangle]
pub extern "C" fn snd_usb_set_pcm_ops(pcm: *mut snd_pcm, stream: i32) {
    unsafe {
        let ops = if stream == SNDRV_PCM_STREAM_PLAYBACK {
            &SND_USB_PLAYBACK_OPS
        } else {
            &SND_USB_CAPTURE_OPS
        };
        snd_pcm_set_ops(pcm, stream, ops as *const _);
    }
}

#[no_mangle]
pub extern "C" fn snd_usb_preallocate_buffer(subs: *mut snd_usb_substream) {
    unsafe {
        let pcm = (*(*subs).stream).pcm;
        let s = (*(*pcm).streams[(*subs).direction as usize]).substream;
        let dev = (*(*(*subs).dev).bus).sysdev;

        if snd_usb_use_vmalloc != 0 {
            snd_pcm_set_managed_buffer(
                s,
                SNDRV_DMA_TYPE_VMALLOC,
                std::ptr::null_mut(),
                0,
                0,
            );
        } else {
            snd_pcm_set_managed_buffer(
                s,
                SNDRV_DMA_TYPE_DEV_SG,
                dev,
                64 * 1024,
                512 * 1024,
            );
        }
    }
}

// External dependencies and declarations below - placeholders for linking
// These would be defined in other source files

// Helper trait/function stubs for scoped guards
#[inline]
unsafe fn spinlock_guard(_lock: *mut spinlock_t) -> SpinlockGuard {
    SpinlockGuard
}

#[inline]
unsafe fn spinlock_irqsave_guard(_lock: *mut spinlock_t) -> SpinlockIrqsaveGuard {
    SpinlockIrqsaveGuard
}

#[inline]
unsafe fn mutex_guard(_mutex: *mut mutex) -> MutexGuard {
    MutexGuard
}

struct SpinlockGuard;
impl Drop for SpinlockGuard {
    fn drop(&mut self) {}
}

struct SpinlockIrqsaveGuard;
impl Drop for SpinlockIrqsaveGuard {
    fn drop(&mut self) {}
}

struct MutexGuard;
impl Drop for MutexGuard {
    fn drop(&mut self) {}
}

// External C function declarations
extern "C" {
    fn usb_get_current_frame_number(dev: *mut usb_device) -> u32;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: u32) -> snd_pcm_uframes_t;
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: snd_pcm_uframes_t) -> u32;
    fn pcm_format_to_bits(format: snd_pcm_format_t) -> u64;
    fn snd_usb_ctl_msg(
        dev: *mut usb_device,
        pipe: u32,
        request: u8,
        requesttype: u8,
        value: u32,
        index: u32,
        data: *mut u8,
        size: u32,
    ) -> i32;
    fn usb_sndctrlpipe(dev: *mut usb_device, endpoint: u32) -> u32;
    fn usb_audio_dbg(chip: *mut snd_usb_audio, fmt: *const u8, ...);
    fn usb_audio_err(chip: *mut snd_usb_audio, fmt: *const u8, ...);
    fn atomic_read(v: *const atomic_t) -> i32;
    fn test_and_clear_bit(nr: i32, addr: *mut i32) -> i32;
    fn test_and_set_bit(nr: i32, addr: *mut i32) -> i32;
    fn clear_bit(nr: i32, addr: *mut i32);
    fn snd_usb_endpoint_stop(ep: *mut snd_usb_endpoint, keep_pending: bool);
    fn snd_usb_endpoint_start(ep: *mut snd_usb_endpoint) -> i32;
    fn snd_usb_endpoint_sync_pending_stop(ep: *mut snd_usb_endpoint);
    fn snd_usb_get_host_interface(
        chip: *mut snd_usb_audio,
        ifnum: i32,
        altsetting: i32,
    ) -> *mut usb_host_interface;
    fn get_iface_desc(alts: *mut usb_host_interface) -> *mut usb_interface_descriptor;
    fn get_endpoint(alts: *mut usb_host_interface, ep: u32) -> *mut usb_endpoint_descriptor;
    fn snd_usb_parse_implicit_fb_quirk(
        chip: *mut snd_usb_audio,
        fmt: *mut audioformat,
        alts: *mut usb_host_interface,
    ) -> i32;
    fn snd_usb_power_domain_set(
        chip: *mut snd_usb_audio,
        pd: *mut snd_usb_power_domain,
        state: i32,
    ) -> i32;
    fn dev_err(dev: *const device, fmt: *const u8, ...);
    fn dev_dbg(dev: *const device, fmt: *const u8, ...);
    fn dev_dbg_ratelimited(dev: *const device, fmt: *const u8, ...);
    fn dev_warn_ratelimited(dev: *const device, fmt: *const u8, ...);
    fn snd_usb_lock_shutdown(chip: *mut snd_usb_audio) -> i32;
    fn snd_usb_unlock_shutdown(chip: *mut snd_usb_audio);
    fn snd_usb_endpoint_compatible(
        chip: *mut snd_usb_audio,
        ep: *mut snd_usb_endpoint,
        fmt: *const audioformat,
        params: *const snd_pcm_hw_params,
    ) -> i32;
    fn snd_usb_endpoint_open(
        chip: *mut snd_usb_audio,
        fmt: *const audioformat,
        params: *const snd_pcm_hw_params,
        is_sync: bool,
        fixed_rate: bool,
    ) -> *mut snd_usb_endpoint;
    fn snd_usb_endpoint_close(chip: *mut snd_usb_audio, ep: *mut snd_usb_endpoint);
    fn snd_usb_endpoint_set_sync(
        chip: *mut snd_usb_audio,
        data_ep: *mut snd_usb_endpoint,
        sync_ep: *mut snd_usb_endpoint,
    );
    fn snd_usb_endpoint_set_params(chip: *mut snd_usb_audio, ep: *mut snd_usb_endpoint) -> i32;
    fn snd_usb_endpoint_prepare(chip: *mut snd_usb_audio, ep: *mut snd_usb_endpoint) -> i32;
    fn snd_usb_set_format_quirk(subs: *mut snd_usb_substream, fmt: *mut audioformat);
    fn snd_usb_endpoint_implicit_feedback_sink(ep: *mut snd_usb_endpoint) -> i32;
    fn snd_usb_find_implicit_fb_sync_format(
        chip: *mut snd_usb_audio,
        fmt: *const audioformat,
        params: *const snd_pcm_hw_params,
        datapipe: bool,
        sync_fixed_rate: *mut bool,
    ) -> *const audioformat;
    fn snd_media_start_pipeline(subs: *mut snd_usb_substream) -> i32;
    fn snd_media_stop_pipeline(subs: *mut snd_usb_substream);
    fn snd_media_stream_init(
        subs: *mut snd_usb_substream,
        pcm: *mut snd_pcm,
        direction: i32,
    ) -> i32;
    fn snd_usb_autoresume(chip: *mut snd_usb_audio) -> i32;
    fn snd_usb_autosuspend(chip: *mut snd_usb_audio);
    fn snd_pcm_format_name(format: snd_pcm_format_t) -> *const u8;
    fn params_format(params: *const snd_pcm_hw_params) -> snd_pcm_format_t;
    fn params_rate(params: *const snd_pcm_hw_params) -> u32;
    fn params_channels(params: *const snd_pcm_hw_params) -> u32;
    fn hw_param_interval(
        params: *mut snd_pcm_hw_params,
        var: i32,
    ) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: i32) -> *mut snd_mask;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_intersect(dst: *mut snd_mask, src: *const snd_mask);
    fn snd_mask_empty(mask: *const snd_mask) -> i32;
    fn snd_interval_checkempty(i: *mut snd_interval) -> i32;
    fn snd_interval_test(i: *mut snd_interval, val: u32) -> i32;
    fn snd_usb_get_endpoint(chip: *mut snd_usb_audio, endpoint: i32) -> *mut snd_usb_endpoint;
    fn snd_usb_endpoint_get_clock_rate(chip: *mut snd_usb_audio, clock: i32) -> i32;
    fn snd_usb_endpoint_next_packet_size(
        ep: *mut snd_usb_endpoint,
        ctx: *mut snd_urb_ctx,
        idx: i32,
        avail: u32,
    ) -> i32;
    fn snd_pcm_hw_constraint_minmax(
        runtime: *mut snd_pcm_runtime,
        var: u32,
        min: u32,
        max: u32,
    ) -> i32;
    fn snd_pcm_hw_rule_add(
        runtime: *mut snd_pcm_runtime,
        cond: i32,
        var: u32,
        func: Option<fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> i32>,
        private: *mut libc::c_void,
        dep1: i32,
        dep2: i32,
        dep3: i32,
        dep4: i32,
        dep5: i32,
    ) -> i32;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut libc::c_void;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: i32, ops: *const snd_pcm_ops);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_pcm_period_elapsed_under_stream_lock(substream: *mut snd_pcm_substream);
    fn snd_pcm_gettime(runtime: *mut snd_pcm_runtime, tv: *mut timespec);
    fn snd_pcm_set_managed_buffer(
        substream: *mut snd_pcm_substream,
        type_: i32,
        dev: *mut device,
        prealloc: u64,
        max: u64,
    );
    fn snd_usb_endpoint_set_callback(
        ep: *mut snd_usb_endpoint,
        prepare: Option<fn(*mut snd_usb_substream, *mut urb, bool) -> i32>,
        retire: Option<fn(*mut snd_usb_substream, *mut urb)>,
        private: *mut libc::c_void,
    );
    fn snd_usb_queue_pending_output_urbs(ep: *mut snd_usb_endpoint, in_stream_lock: bool) -> i32;
    fn bitrev8(byte: u8) -> u8;
    fn cpu_to_le32(val: u32) -> u32;

    static snd_usb_use_vmalloc: i32;
}

// Type stubs
type snd_pcm_uframes_t = u32;
type snd_pcm_format_t = i32;
type atomic_t = i32;
type spinlock_t = libc::c_void;
type mutex = libc::c_void;
type device = libc::c_void;
type usb_device = libc::c_void;
type usb_host_interface = libc::c_void;
type usb_interface_descriptor = libc::c_void;
type usb_endpoint_descriptor = libc::c_void;
type snd_pcm = libc::c_void;
type snd_pcm_substream = libc::c_void;
type snd_pcm_runtime = libc::c_void;
type snd_pcm_hw_params = libc::c_void;
type snd_pcm_hw_rule = libc::c_void;
type snd_usb_audio = libc::c_void;
type snd_usb_stream = libc::c_void;
type snd_usb_substream = libc::c_void;
type snd_usb_endpoint = libc::c_void;
type snd_usb_power_domain = libc::c_void;
type audioformat = libc::c_void;
type list_head = libc::c_void;
type urb = libc::c_void;
type snd_urb_ctx = libc::c_void;
type timespec = libc::c_void;

#[repr(C)]
struct snd_interval {
    min: u32,
    max: u32,
    openmin: i32,
    openmax: i32,
    empty: i32,
}

#[repr(C)]
struct snd_mask {
    bits: [u32; 2],
}

// Constants
const SNDRV_PCM_STREAM_PLAYBACK: i32 = 0;
const SNDRV_PCM_STREAM_CAPTURE: i32 = 1;
const SNDRV_PCM_POS_XRUN: u32 = 0xffffffff;
const SNDRV_PCM_STATE_DRAINING: i32 = 4;
const SNDRV_PCM_INFO_MMAP: u32 = 0x00000001;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 0x00000002;
const SNDRV_PCM_INFO_BATCH: u32 = 0x00000010;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 0x00000100;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: u32 = 0x00000200;
const SNDRV_PCM_INFO_PAUSE: u32 = 0x00080000;
const SNDRV_PCM_INFO_SYNC_APPLPTR: u32 = 0x00100000;
const SNDRV_PCM_INFO_JOINT_DUPLEX: u32 = 0x00200000;
const SNDRV_PCM_HW_PARAM_RATE: i32 = 11;
const SNDRV_PCM_HW_PARAM_CHANNELS: i32 = 10;
const SNDRV_PCM_HW_PARAM_FORMAT: i32 = 9;
const SNDRV_PCM_HW_PARAM_PERIOD_TIME: i32 = 8;
const SNDRV_PCM_HW_PARAM_PERIOD_SIZE: i32 = 12;
const SNDRV_PCM_HW_PARAM_PERIODS: i32 = 13;
const SNDRV_PCM_HW_PARAM_BUFFER_TIME: i32 = 7;
const SNDRV_PCM_RATE_CONTINUOUS: u32 = 0x00000040;
const USB_ENDPOINT_SYNCTYPE: i32 = 0x0c;
const USB_ENDPOINT_SYNC_NONE: i32 = 0x00;
const USB_ENDPOINT_SYNC_ASYNC: i32 = 0x04;
const USB_ENDPOINT_SYNC_ADAPTIVE: i32 = 0x08;
const USB_ENDPOINT_SYNC_SYNC: i32 = 0x0c;
const USB_ENDPOINT_XFERTYPE_MASK: i32 = 0x03;
const USB_ENDPOINT_XFER_ISOC: i32 = 0x01;
const USB_ENDPOINT_USAGE_MASK: i32 = 0x30;
const USB_ENDPOINT_USAGE_IMPLICIT_FB: i32 = 0x20;
const USB_DIR_IN: i32 = 0x80;
const USB_TYPE_CLASS: i32 = 0x20;
const USB_RECIP_ENDPOINT: i32 = 0x02;
const USB_DIR_OUT: i32 = 0x00;
const USB_DT_ENDPOINT_AUDIO_SIZE: i32 = 9;
const USB_SPEED_FULL: i32 = 1;
const UAC_VERSION_1: i32 = 0x0100;
const UAC_VERSION_2: i32 = 0x0200;
const UAC_FORMAT_TYPE_II: i32 = 2;
const UAC_EP_CS_ATTR_PITCH_CONTROL: i32 = 0x01;
const UAC_SET_CUR: u8 = 0x01;
const UAC2_CS_CUR: u8 = 0x01;
const UAC2_EP_CS_PITCH: i32 = 0x01;
const UAC3_PD_STATE_D0: i32 = 0;
const UAC3_PD_STATE_D1: i32 = 1;
const UAC3_PD_STATE_D2: i32 = 2;
const QUIRK_FLAG_FIXED_RATE: u32 = 0x00001000;
const SNDRV_PCM_FORMAT_DSD_U8: i32 = 48;
const SNDRV_PCM_FORMAT_DSD_U16_LE: i32 = 49;
const SNDRV_PCM_TRIGGER_START: i32 = 0;
const SNDRV_PCM_TRIGGER_STOP: i32 = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: i32 = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: i32 = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: i32 = 5;
const SNDRV_DMA_TYPE_VMALLOC: i32 = 2;
const SNDRV_DMA_TYPE_DEV_SG: i32 = 3;
const EINVAL: i32 = 22;
const EAGAIN: i32 = 11;
const EIO: i32 = 5;
const EPIPE: i32 = 32;
const EBUSY: i32 = 16;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
