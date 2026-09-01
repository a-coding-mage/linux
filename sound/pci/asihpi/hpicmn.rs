// SPDX-License-Identifier: GPL-2.0-only
/******************************************************************************

    AudioScience HPI driver
    Copyright (C) 1997-2014  AudioScience Inc. <support@audioscience.com>


\file hpicmn.c

 Common functions used by hpixxxx.c modules

(C) Copyright AudioScience Inc. 1998-2003
*******************************************************************************/

// C dependencies: hpi_internal.h, hpidebug.h, hpimsginit.h, hpicmn.h
use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

const SOURCEFILE_NAME: &[u8] = b"hpicmn.c\0";

#[repr(C)]
struct hpi_adapters_list {
    list_lock: hpios_spinlock,
    adapter: [hpi_adapter_obj; HPI_MAX_ADAPTERS as usize],
    gw_num_adapters: u16,
}

static mut adapters: hpi_adapters_list = unsafe { core::mem::zeroed() };

unsafe extern "C" {
    fn hpios_alistlock_lock(adapters: *mut hpi_adapters_list);
    fn hpios_alistlock_unlock(adapters: *mut hpi_adapters_list);
    fn hpios_alistlock_init(adapters: *mut hpi_adapters_list);
    fn hpios_dsplock_init(pao: *mut hpi_adapter_obj);
    fn hpi_init_response(
        phr: *mut hpi_response,
        object: u16,
        function: u16,
        error: u16,
    );
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
}

/**
 * hpi_validate_response - Given an HPI Message that was sent out and
 * a response that was received, validate that the response has the
 * correct fields filled in, i.e ObjectType, Function etc
 * @phm: message
 * @phr: response
 */
#[no_mangle]
pub unsafe extern "C" fn hpi_validate_response(
    phm: *mut hpi_message,
    phr: *mut hpi_response,
) -> u16 {
    if (*phr).type_ != HPI_TYPE_RESPONSE {
        HPI_DEBUG_LOG!(ERROR, "header type %d invalid\n", (*phr).type_);
        return HPI_ERROR_INVALID_RESPONSE;
    }

    if (*phr).object != (*phm).object {
        HPI_DEBUG_LOG!(ERROR, "header object %d invalid\n", (*phr).object);
        return HPI_ERROR_INVALID_RESPONSE;
    }

    if (*phr).function != (*phm).function {
        HPI_DEBUG_LOG!(ERROR, "header function %d invalid\n", (*phr).function);
        return HPI_ERROR_INVALID_RESPONSE;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn hpi_add_adapter(pao: *mut hpi_adapter_obj) -> u16 {
    let mut retval: u16 = 0;
    /*HPI_ASSERT(pao->type); */

    hpios_alistlock_lock(ptr::addr_of_mut!(adapters));

    if (*pao).index >= HPI_MAX_ADAPTERS as u16 {
        retval = HPI_ERROR_BAD_ADAPTER_NUMBER;
        goto_unlock(retval)
    } else {
        if adapters.adapter[(*pao).index as usize].type_ != 0 {
            let mut a: c_int = HPI_MAX_ADAPTERS as c_int - 1;
            while a >= 0 {
                if adapters.adapter[a as usize].type_ == 0 {
                    HPI_DEBUG_LOG!(
                        WARNING,
                        "ASI%X duplicate index %d moved to %d\n",
                        (*pao).type_,
                        (*pao).index,
                        a
                    );
                    (*pao).index = a as u16;
                    break;
                }
                a -= 1;
            }
            if a < 0 {
                retval = HPI_ERROR_DUPLICATE_ADAPTER_NUMBER;
                return goto_unlock(retval);
            }
        }
        adapters.adapter[(*pao).index as usize] = *pao;
        hpios_dsplock_init(&mut adapters.adapter[(*pao).index as usize]);
        adapters.gw_num_adapters = adapters.gw_num_adapters.wrapping_add(1);
        goto_unlock(retval)
    }
}

unsafe fn goto_unlock(retval: u16) -> u16 {
    hpios_alistlock_unlock(ptr::addr_of_mut!(adapters));
    retval
}

#[no_mangle]
pub unsafe extern "C" fn hpi_delete_adapter(pao: *mut hpi_adapter_obj) {
    if (*pao).type_ == 0 {
        HPI_DEBUG_LOG!(ERROR, "removing null adapter?\n");
        return;
    }

    hpios_alistlock_lock(ptr::addr_of_mut!(adapters));
    if adapters.adapter[(*pao).index as usize].type_ != 0 {
        adapters.gw_num_adapters = adapters.gw_num_adapters.wrapping_sub(1);
    }
    memset(
        &mut adapters.adapter[(*pao).index as usize] as *mut hpi_adapter_obj as *mut c_void,
        0,
        size_of::<hpi_adapter_obj>(),
    );
    hpios_alistlock_unlock(ptr::addr_of_mut!(adapters));
}

/**
 * hpi_find_adapter - FindAdapter returns a pointer to the struct
 * hpi_adapter_obj with index wAdapterIndex in an HPI_ADAPTERS_LIST
 * structure.
 * @adapter_index: value in [0, HPI_MAX_ADAPTERS[
 */
#[no_mangle]
pub unsafe extern "C" fn hpi_find_adapter(adapter_index: u16) -> *mut hpi_adapter_obj {
    let pao: *mut hpi_adapter_obj;

    if adapter_index >= HPI_MAX_ADAPTERS as u16 {
        HPI_DEBUG_LOG!(VERBOSE, "find_adapter invalid index %d\n", adapter_index);
        return ptr::null_mut();
    }

    pao = &mut adapters.adapter[adapter_index as usize];
    if (*pao).type_ != 0 {
        /*
           HPI_DEBUG_LOG(VERBOSE, "Found adapter index %d\n",
           wAdapterIndex);
         */
        pao
    } else {
        /*
           HPI_DEBUG_LOG(VERBOSE, "No adapter index %d\n",
           wAdapterIndex);
         */
        ptr::null_mut()
    }
}

/**
 * wipe_adapter_list - wipe an HPI_ADAPTERS_LIST structure.
 *
 */
unsafe fn wipe_adapter_list() {
    memset(
        ptr::addr_of_mut!(adapters) as *mut c_void,
        0,
        size_of::<hpi_adapters_list>(),
    );
}

unsafe fn subsys_get_adapter(phm: *mut hpi_message, phr: *mut hpi_response) {
    let mut count: c_int = (*phm).obj_index as c_int;
    let mut index: u16 = 0;

    /* find the nCount'th nonzero adapter in array */
    while index < HPI_MAX_ADAPTERS as u16 {
        if adapters.adapter[index as usize].type_ != 0 {
            if count == 0 {
                break;
            }
            count -= 1;
        }
        index = index.wrapping_add(1);
    }

    if index < HPI_MAX_ADAPTERS as u16 {
        (*phr).u.s.adapter_index = adapters.adapter[index as usize].index;
        (*phr).u.s.adapter_type = adapters.adapter[index as usize].type_;
    } else {
        (*phr).u.s.adapter_index = 0;
        (*phr).u.s.adapter_type = 0;
        (*phr).error = HPI_ERROR_INVALID_OBJ_INDEX;
    }
}

unsafe fn control_cache_alloc_check(pC: *mut hpi_control_cache) -> c_uint {
    let mut i: c_uint;
    let mut cached: c_int = 0;
    if pC.is_null() {
        return 0;
    }

    if (*pC).init != 0 {
        return (*pC).init as c_uint;
    }

    if (*pC).p_cache.is_null() {
        return 0;
    }

    if (*pC).control_count != 0 && (*pC).cache_size_in_bytes != 0 {
        let p_master_cache: *mut c_char;
        let mut byte_count: c_uint = 0;

        p_master_cache = (*pC).p_cache as *mut c_char;
        HPI_DEBUG_LOG!(DEBUG, "check %d controls\n", (*pC).control_count);
        i = 0;
        while i < (*pC).control_count {
            let info: *mut hpi_control_cache_info =
                p_master_cache.add(byte_count as usize) as *mut hpi_control_cache_info;
            let control_index: u16 = (*info).control_index;

            if control_index as u32 >= (*pC).control_count {
                HPI_DEBUG_LOG!(
                    INFO,
                    "adap %d control index %d out of range, cache not ready?\n",
                    (*pC).adap_idx,
                    control_index
                );
                return 0;
            }

            if (*info).size_in32bit_words == 0 {
                if i == 0 {
                    HPI_DEBUG_LOG!(INFO, "adap %d cache not ready?\n", (*pC).adap_idx);
                    return 0;
                }
                /* The cache is invalid.
                 * Minimum valid entry size is
                 * sizeof(struct hpi_control_cache_info)
                 */
                HPI_DEBUG_LOG!(
                    ERROR,
                    "adap %d zero size cache entry %d\n",
                    (*pC).adap_idx,
                    i
                );
                break;
            }

            if (*info).control_type != 0 {
                *(*pC).p_info.add(control_index as usize) = info;
                cached += 1;
            } else {
                /* dummy cache entry */
                *(*pC).p_info.add(control_index as usize) = ptr::null_mut();
            }

            byte_count = byte_count.wrapping_add((*info).size_in32bit_words as c_uint * 4);

            HPI_DEBUG_LOG!(
                VERBOSE,
                "cached %d, pinfo %p index %d type %d size %d\n",
                cached,
                *(*pC).p_info.add((*info).control_index as usize),
                (*info).control_index,
                (*info).control_type,
                (*info).size_in32bit_words
            );

            /* quit loop early if whole cache has been scanned.
             * dwControlCount is the maximum possible entries
             * but some may be absent from the cache
             */
            if byte_count >= (*pC).cache_size_in_bytes {
                break;
            }
            /* have seen last control index */
            if (*info).control_index as u32 == (*pC).control_count - 1 {
                break;
            }

            i = i.wrapping_add(1);
        }

        if byte_count != (*pC).cache_size_in_bytes {
            HPI_DEBUG_LOG!(
                WARNING,
                "adap %d bytecount %d != cache size %d\n",
                (*pC).adap_idx,
                byte_count,
                (*pC).cache_size_in_bytes
            );
        } else {
            HPI_DEBUG_LOG!(
                DEBUG,
                "adap %d cache good, bytecount == cache size = %d\n",
                (*pC).adap_idx,
                byte_count
            );
        }

        (*pC).init = cached as u16;
    }
    (*pC).init as c_uint
}

/** Find a control.
*/
unsafe fn find_control(
    control_index: u16,
    p_cache: *mut hpi_control_cache,
    pI: *mut *mut hpi_control_cache_info,
) -> i16 {
    if control_cache_alloc_check(p_cache) == 0 {
        HPI_DEBUG_LOG!(
            VERBOSE,
            "control_cache_alloc_check() failed %d\n",
            control_index
        );
        return 0;
    }

    if control_index as u32 >= (*p_cache).control_count {
        HPI_DEBUG_LOG!(VERBOSE, "control_index out of bounce %d\n", control_index);
        return 0;
    }

    *pI = *(*p_cache).p_info.add(control_index as usize);
    if (*pI).is_null() {
        HPI_DEBUG_LOG!(VERBOSE, "Uncached Control %d\n", control_index);
        return 0;
    } else {
        HPI_DEBUG_LOG!(VERBOSE, "find_control() type %d\n", (**pI).control_type);
    }
    1
}

/* allow unified treatment of several string fields within struct */
#[repr(C)]
struct pad_ofs_size {
    offset: c_uint,
    field_size: c_uint,
}

unsafe fn hpicmn_pad_ofs_and_size(index: usize) -> pad_ofs_size {
    let p: *const hpi_control_cache_pad = ptr::null();

    match index {
        0 => pad_ofs_size {
            offset: offset_of!(hpi_control_cache_pad, c_channel) as c_uint,
            field_size: size_of_val_raw(ptr::addr_of!((*p).c_channel)) as c_uint,
        }, /* HPI_PAD_CHANNEL_NAME */
        1 => pad_ofs_size {
            offset: offset_of!(hpi_control_cache_pad, c_artist) as c_uint,
            field_size: size_of_val_raw(ptr::addr_of!((*p).c_artist)) as c_uint,
        }, /* HPI_PAD_ARTIST */
        2 => pad_ofs_size {
            offset: offset_of!(hpi_control_cache_pad, c_title) as c_uint,
            field_size: size_of_val_raw(ptr::addr_of!((*p).c_title)) as c_uint,
        }, /* HPI_PAD_TITLE */
        _ => pad_ofs_size {
            offset: offset_of!(hpi_control_cache_pad, c_comment) as c_uint,
            field_size: size_of_val_raw(ptr::addr_of!((*p).c_comment)) as c_uint,
        }, /* HPI_PAD_COMMENT */
    }
}

unsafe fn size_of_val_raw<T>(_: *const T) -> usize {
    size_of::<T>()
}

/** CheckControlCache checks the cache and fills the struct hpi_response
 * accordingly. It returns one if a cache hit occurred, zero otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn hpi_check_control_cache_single(
    pC: *mut hpi_control_cache_single,
    phm: *mut hpi_message,
    phr: *mut hpi_response,
) -> i16 {
    let response_size: usize;
    let mut found: i16 = 1;

    /* set the default response size */
    response_size = size_of::<hpi_response_header>() + size_of::<hpi_control_res>();

    match (*pC).u.i.control_type {
        HPI_CONTROL_METER => {
            if (*phm).u.c.attribute == HPI_METER_PEAK {
                (*phr).u.c.an_log_value[0] = (*pC).u.meter.an_log_peak[0];
                (*phr).u.c.an_log_value[1] = (*pC).u.meter.an_log_peak[1];
            } else if (*phm).u.c.attribute == HPI_METER_RMS {
                if (*pC).u.meter.an_logRMS[0] == HPI_CACHE_INVALID_SHORT {
                    (*phr).error = HPI_ERROR_INVALID_CONTROL_ATTRIBUTE;
                    (*phr).u.c.an_log_value[0] = HPI_METER_MINIMUM;
                    (*phr).u.c.an_log_value[1] = HPI_METER_MINIMUM;
                } else {
                    (*phr).u.c.an_log_value[0] = (*pC).u.meter.an_logRMS[0];
                    (*phr).u.c.an_log_value[1] = (*pC).u.meter.an_logRMS[1];
                }
            } else {
                found = 0;
            }
        }
        HPI_CONTROL_VOLUME => {
            if (*phm).u.c.attribute == HPI_VOLUME_GAIN {
                (*phr).u.c.an_log_value[0] = (*pC).u.vol.an_log[0];
                (*phr).u.c.an_log_value[1] = (*pC).u.vol.an_log[1];
            } else if (*phm).u.c.attribute == HPI_VOLUME_MUTE {
                if (*pC).u.vol.flags & HPI_VOLUME_FLAG_HAS_MUTE != 0 {
                    if (*pC).u.vol.flags & HPI_VOLUME_FLAG_MUTED != 0 {
                        (*phr).u.c.param1 = HPI_BITMASK_ALL_CHANNELS;
                    } else {
                        (*phr).u.c.param1 = 0;
                    }
                } else {
                    (*phr).error = HPI_ERROR_INVALID_CONTROL_ATTRIBUTE;
                    (*phr).u.c.param1 = 0;
                }
            } else {
                found = 0;
            }
        }
        HPI_CONTROL_MULTIPLEXER => {
            if (*phm).u.c.attribute == HPI_MULTIPLEXER_SOURCE {
                (*phr).u.c.param1 = (*pC).u.mux.source_node_type;
                (*phr).u.c.param2 = (*pC).u.mux.source_node_index;
            } else {
                found = 0;
            }
        }
        HPI_CONTROL_CHANNEL_MODE => {
            if (*phm).u.c.attribute == HPI_CHANNEL_MODE_MODE {
                (*phr).u.c.param1 = (*pC).u.mode.mode;
            } else {
                found = 0;
            }
        }
        HPI_CONTROL_LEVEL => {
            if (*phm).u.c.attribute == HPI_LEVEL_GAIN {
                (*phr).u.c.an_log_value[0] = (*pC).u.level.an_log[0];
                (*phr).u.c.an_log_value[1] = (*pC).u.level.an_log[1];
            } else {
                found = 0;
            }
        }
        HPI_CONTROL_TUNER => {
            if (*phm).u.c.attribute == HPI_TUNER_FREQ {
                (*phr).u.c.param1 = (*pC).u.tuner.freq_ink_hz;
            } else if (*phm).u.c.attribute == HPI_TUNER_BAND {
                (*phr).u.c.param1 = (*pC).u.tuner.band;
            } else if (*phm).u.c.attribute == HPI_TUNER_LEVEL_AVG {
                if (*pC).u.tuner.s_level_avg == HPI_CACHE_INVALID_SHORT {
                    (*phr).u.cu.tuner.s_level = 0;
                    (*phr).error = HPI_ERROR_INVALID_CONTROL_ATTRIBUTE;
                } else {
                    (*phr).u.cu.tuner.s_level = (*pC).u.tuner.s_level_avg;
                }
            } else {
                found = 0;
            }
        }
        HPI_CONTROL_AESEBU_RECEIVER => {
            if (*phm).u.c.attribute == HPI_AESEBURX_ERRORSTATUS {
                (*phr).u.c.param1 = (*pC).u.aes3rx.error_status;
            } else if (*phm).u.c.attribute == HPI_AESEBURX_FORMAT {
                (*phr).u.c.param1 = (*pC).u.aes3rx.format;
            } else {
                found = 0;
            }
        }
        HPI_CONTROL_AESEBU_TRANSMITTER => {
            if (*phm).u.c.attribute == HPI_AESEBUTX_FORMAT {
                (*phr).u.c.param1 = (*pC).u.aes3tx.format;
            } else {
                found = 0;
            }
        }
        HPI_CONTROL_TONEDETECTOR => {
            if (*phm).u.c.attribute == HPI_TONEDETECTOR_STATE {
                (*phr).u.c.param1 = (*pC).u.tone.state;
            } else {
                found = 0;
            }
        }
        HPI_CONTROL_SILENCEDETECTOR => {
            if (*phm).u.c.attribute == HPI_SILENCEDETECTOR_STATE {
                (*phr).u.c.param1 = (*pC).u.silence.state;
            } else {
                found = 0;
            }
        }
        HPI_CONTROL_MICROPHONE => {
            if (*phm).u.c.attribute == HPI_MICROPHONE_PHANTOM_POWER {
                (*phr).u.c.param1 = (*pC).u.microphone.phantom_state;
            } else {
                found = 0;
            }
        }
        HPI_CONTROL_SAMPLECLOCK => {
            if (*phm).u.c.attribute == HPI_SAMPLECLOCK_SOURCE {
                (*phr).u.c.param1 = (*pC).u.clk.source;
            } else if (*phm).u.c.attribute == HPI_SAMPLECLOCK_SOURCE_INDEX {
                if (*pC).u.clk.source_index == HPI_CACHE_INVALID_UINT16 {
                    (*phr).u.c.param1 = 0;
                    (*phr).error = HPI_ERROR_INVALID_CONTROL_ATTRIBUTE;
                } else {
                    (*phr).u.c.param1 = (*pC).u.clk.source_index;
                }
            } else if (*phm).u.c.attribute == HPI_SAMPLECLOCK_SAMPLERATE {
                (*phr).u.c.param1 = (*pC).u.clk.sample_rate;
            } else {
                found = 0;
            }
        }
        HPI_CONTROL_PAD => {
            let p_pad: *mut hpi_control_cache_pad = pC as *mut hpi_control_cache_pad;

            if (*p_pad).field_valid_flags
                & (1 << HPI_CTL_ATTR_INDEX((*phm).u.c.attribute)) == 0
            {
                (*phr).error = HPI_ERROR_INVALID_CONTROL_ATTRIBUTE;
            } else if (*phm).u.c.attribute == HPI_PAD_PROGRAM_ID {
                (*phr).u.c.param1 = (*p_pad).pI;
            } else if (*phm).u.c.attribute == HPI_PAD_PROGRAM_TYPE {
                (*phr).u.c.param1 = (*p_pad).pTY;
            } else {
                let index: c_uint = HPI_CTL_ATTR_INDEX((*phm).u.c.attribute) - 1;
                let offset: c_uint = (*phm).u.c.param1;
                let pad_string_len: c_uint;
                let field_size: c_uint;
                let pad_string: *mut c_char;
                let mut tocopy: c_uint;

                if index > 4 - 1 {
                    (*phr).error = HPI_ERROR_INVALID_CONTROL_ATTRIBUTE;
                } else {
                    let desc = hpicmn_pad_ofs_and_size(index as usize);
                    pad_string = (p_pad as *mut c_char).add(desc.offset as usize);
                    field_size = desc.field_size;
                    /* Ensure null terminator */
                    *pad_string.add(field_size as usize - 1) = 0;

                    pad_string_len = strlen(pad_string) as c_uint + 1;

                    if offset > pad_string_len {
                        (*phr).error = HPI_ERROR_INVALID_CONTROL_VALUE;
                    } else {
                        tocopy = pad_string_len - offset;
                        if tocopy
                            > size_of_val(&(*phr).u.cu.chars8.sz_data) as c_uint
                        {
                            tocopy = size_of_val(&(*phr).u.cu.chars8.sz_data) as c_uint;
                        }

                        memcpy(
                            (*phr).u.cu.chars8.sz_data.as_mut_ptr() as *mut c_void,
                            pad_string.add(offset as usize) as *const c_void,
                            tocopy as usize,
                        );

                        (*phr).u.cu.chars8.remaining_chars =
                            pad_string_len - offset - tocopy;
                    }
                }
            }
        }
        _ => {
            found = 0;
        }
    }

    HPI_DEBUG_LOG!(
        VERBOSE,
        "%s Adap %d, Ctl %d, Type %d, Attr %d\n",
        if found != 0 { "Cached" } else { "Uncached" },
        (*phm).adapter_index,
        (*pC).u.i.control_index,
        (*pC).u.i.control_type,
        (*phm).u.c.attribute
    );

    if found != 0 {
        (*phr).size = response_size as u16;
        (*phr).type_ = HPI_TYPE_RESPONSE;
        (*phr).object = (*phm).object;
        (*phr).function = (*phm).function;
    }

    found
}

#[no_mangle]
pub unsafe extern "C" fn hpi_check_control_cache(
    p_cache: *mut hpi_control_cache,
    phm: *mut hpi_message,
    phr: *mut hpi_response,
) -> i16 {
    let mut pI: *mut hpi_control_cache_info = ptr::null_mut();

    if find_control((*phm).obj_index, p_cache, &mut pI) == 0 {
        HPI_DEBUG_LOG!(
            VERBOSE,
            "HPICMN find_control() failed for adap %d\n",
            (*phm).adapter_index
        );
        return 0;
    }

    (*phr).error = 0;
    (*phr).specific_error = 0;
    (*phr).version = 0;

    hpi_check_control_cache_single(pI as *mut hpi_control_cache_single, phm, phr)
}

/** Updates the cache with Set values.

Only update if no error.
Volume and Level return the limited values in the response, so use these
Multiplexer does so use sent values
*/
#[no_mangle]
pub unsafe extern "C" fn hpi_cmn_control_cache_sync_to_msg_single(
    pC: *mut hpi_control_cache_single,
    phm: *mut hpi_message,
    phr: *mut hpi_response,
) {
    match (*pC).u.i.control_type {
        HPI_CONTROL_VOLUME => {
            if (*phm).u.c.attribute == HPI_VOLUME_GAIN {
                (*pC).u.vol.an_log[0] = (*phr).u.c.an_log_value[0];
                (*pC).u.vol.an_log[1] = (*phr).u.c.an_log_value[1];
            } else if (*phm).u.c.attribute == HPI_VOLUME_MUTE {
                if (*phm).u.c.param1 != 0 {
                    (*pC).u.vol.flags |= HPI_VOLUME_FLAG_MUTED;
                } else {
                    (*pC).u.vol.flags &= !HPI_VOLUME_FLAG_MUTED;
                }
            }
        }
        HPI_CONTROL_MULTIPLEXER => {
            /* mux does not return its setting on Set command. */
            if (*phm).u.c.attribute == HPI_MULTIPLEXER_SOURCE {
                (*pC).u.mux.source_node_type = (*phm).u.c.param1 as u16;
                (*pC).u.mux.source_node_index = (*phm).u.c.param2 as u16;
            }
        }
        HPI_CONTROL_CHANNEL_MODE => {
            /* mode does not return its setting on Set command. */
            if (*phm).u.c.attribute == HPI_CHANNEL_MODE_MODE {
                (*pC).u.mode.mode = (*phm).u.c.param1 as u16;
            }
        }
        HPI_CONTROL_LEVEL => {
            if (*phm).u.c.attribute == HPI_LEVEL_GAIN {
                (*pC).u.vol.an_log[0] = (*phr).u.c.an_log_value[0];
                (*pC).u.vol.an_log[1] = (*phr).u.c.an_log_value[1];
            }
        }
        HPI_CONTROL_MICROPHONE => {
            if (*phm).u.c.attribute == HPI_MICROPHONE_PHANTOM_POWER {
                (*pC).u.microphone.phantom_state = (*phm).u.c.param1 as u16;
            }
        }
        HPI_CONTROL_AESEBU_TRANSMITTER => {
            if (*phm).u.c.attribute == HPI_AESEBUTX_FORMAT {
                (*pC).u.aes3tx.format = (*phm).u.c.param1;
            }
        }
        HPI_CONTROL_AESEBU_RECEIVER => {
            if (*phm).u.c.attribute == HPI_AESEBURX_FORMAT {
                (*pC).u.aes3rx.format = (*phm).u.c.param1;
            }
        }
        HPI_CONTROL_SAMPLECLOCK => {
            if (*phm).u.c.attribute == HPI_SAMPLECLOCK_SOURCE {
                (*pC).u.clk.source = (*phm).u.c.param1 as u16;
            } else if (*phm).u.c.attribute == HPI_SAMPLECLOCK_SOURCE_INDEX {
                (*pC).u.clk.source_index = (*phm).u.c.param1 as u16;
            } else if (*phm).u.c.attribute == HPI_SAMPLECLOCK_SAMPLERATE {
                (*pC).u.clk.sample_rate = (*phm).u.c.param1;
            }
        }
        _ => {}
    }
}

#[no_mangle]
pub unsafe extern "C" fn hpi_cmn_control_cache_sync_to_msg(
    p_cache: *mut hpi_control_cache,
    phm: *mut hpi_message,
    phr: *mut hpi_response,
) {
    let pC: *mut hpi_control_cache_single;
    let mut pI: *mut hpi_control_cache_info = ptr::null_mut();

    if (*phr).error != 0 {
        return;
    }

    if find_control((*phm).obj_index, p_cache, &mut pI) == 0 {
        HPI_DEBUG_LOG!(
            VERBOSE,
            "HPICMN find_control() failed for adap %d\n",
            (*phm).adapter_index
        );
        return;
    }

    /* pC is the default cached control strucure.
       May be cast to something else in the following switch statement.
     */
    pC = pI as *mut hpi_control_cache_single;

    hpi_cmn_control_cache_sync_to_msg_single(pC, phm, phr);
}

/** Allocate control cache.

\return Cache pointer, or NULL if allocation fails.
*/
#[no_mangle]
pub unsafe extern "C" fn hpi_alloc_control_cache(
    control_count: u32,
    size_in_bytes: u32,
    p_dsp_control_buffer: *mut u8,
) -> *mut hpi_control_cache {
    let p_cache: *mut hpi_control_cache;
    let alloc_size =
        size_of::<hpi_control_cache>() + control_count as usize * size_of::<*mut hpi_control_cache_info>();

    p_cache = kzalloc(alloc_size, GFP_KERNEL) as *mut hpi_control_cache;
    if p_cache.is_null() {
        return ptr::null_mut();
    }

    (*p_cache).cache_size_in_bytes = size_in_bytes;
    (*p_cache).control_count = control_count;
    (*p_cache).p_cache = p_dsp_control_buffer;
    (*p_cache).init = 0;
    p_cache
}

#[no_mangle]
pub unsafe extern "C" fn hpi_free_control_cache(p_cache: *mut hpi_control_cache) {
    kfree(p_cache as *const c_void);
}

unsafe fn subsys_message(phm: *mut hpi_message, phr: *mut hpi_response) {
    hpi_init_response(phr, HPI_OBJ_SUBSYSTEM, (*phm).function, 0);

    match (*phm).function {
        HPI_SUBSYS_OPEN | HPI_SUBSYS_CLOSE | HPI_SUBSYS_DRIVER_UNLOAD => {}
        HPI_SUBSYS_DRIVER_LOAD => {
            wipe_adapter_list();
            hpios_alistlock_init(ptr::addr_of_mut!(adapters));
        }
        HPI_SUBSYS_GET_ADAPTER => {
            subsys_get_adapter(phm, phr);
        }
        HPI_SUBSYS_GET_NUM_ADAPTERS => {
            (*phr).u.s.num_adapters = adapters.gw_num_adapters;
        }
        HPI_SUBSYS_CREATE_ADAPTER => {}
        _ => {
            (*phr).error = HPI_ERROR_INVALID_FUNC;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn HPI_COMMON(phm: *mut hpi_message, phr: *mut hpi_response) {
    match (*phm).type_ {
        HPI_TYPE_REQUEST => match (*phm).object {
            HPI_OBJ_SUBSYSTEM => {
                subsys_message(phm, phr);
            }
            _ => {}
        },
        _ => {
            (*phr).error = HPI_ERROR_INVALID_TYPE;
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
