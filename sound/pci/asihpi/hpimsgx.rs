// SPDX-License-Identifier: GPL-2.0-only
/******************************************************************************

    AudioScience HPI driver
    Copyright (C) 1997-2014  AudioScience Inc. <support@audioscience.com>


Extended Message Function With Response Caching

(C) Copyright AudioScience Inc. 2002
*****************************************************************************/

use core::ffi::c_void;
use core::mem::{size_of, zeroed};
use core::ptr::{copy_nonoverlapping, null_mut};

pub const SOURCEFILE_NAME: &str = "hpimsgx.c";

/*
 * C dependencies removed from executable Rust:
 * hpi_internal.h, hpi_version.h, hpimsginit.h, hpicmn.h, hpimsgx.h,
 * hpidebug.h, and the hpipcida.h initializer for asihpi_pci_tbl.
 */

type hpi_handler_func = unsafe extern "C" fn(*mut hpi_message, *mut hpi_response);

#[repr(C)]
pub struct pci_device_id {
    pub vendor: u32,
    pub device: u32,
    pub subvendor: u32,
    pub subdevice: u32,
    pub class: u32,
    pub class_mask: u32,
    pub driver_data: usize,
}

unsafe extern "C" {
    static asihpi_pci_tbl: [pci_device_id; 0];
    static mut msgx_lock: hpios_spinlock;

    fn hpios_msgxlock_init(lock: *mut hpios_spinlock);
    fn hpios_msgxlock_lock(lock: *mut hpios_spinlock);
    fn hpios_msgxlock_unlock(lock: *mut hpios_spinlock);

    fn hpi_init_response(
        phr: *mut hpi_response,
        object: u16,
        function: u16,
        error: u16,
    );
    fn hpi_init_message_response(
        phm: *mut hpi_message,
        phr: *mut hpi_response,
        object: u16,
        function: u16,
    );
    fn HPI_COMMON(phm: *mut hpi_message, phr: *mut hpi_response);
    fn hpi_debug_level_set(level: u16);
    fn HPI_DEBUG_MESSAGE(level: u16, phm: *mut hpi_message);
    fn HPI_DEBUG_RESPONSE(phr: *mut hpi_response);
    fn HPI_DEBUG_LOG(level: u16, fmt: *const u8, ...);
}

#[repr(C)]
pub struct hpios_spinlock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub vendor: u32,
    pub device: u32,
    pub subsystem_vendor: u32,
    pub subsystem_device: u32,
}

#[repr(C)]
pub struct hpi_pci {
    pub pci_dev: *mut pci_dev,
}

#[repr(C)]
pub struct hpi_response_header {
    pub size: u16,
    pub version: u8,
    pub type_: u8,
    pub object: u16,
    pub function: u16,
    pub error: u16,
    pub specific_error: u16,
}

#[repr(C)]
pub struct hpi_adapter_res {
    pub adapter_type: u16,
    pub num_outstreams: u16,
    pub num_instreams: u16,
}

#[repr(C)]
pub struct hpi_mixer_res {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hpi_stream_res {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hpi_subsys_resource {
    pub pci: *const hpi_pci,
}

#[repr(C)]
pub struct hpi_subsys_resource_wrapper {
    pub r: hpi_subsys_resource,
}

#[repr(C)]
pub struct hpi_subsys_res {
    pub version: u32,
    pub data: u32,
    pub adapter_index: u16,
    pub resource: hpi_subsys_resource_wrapper,
}

#[repr(C)]
pub struct hpi_adapterx_res {
    pub info: hpi_adapter_res,
}

#[repr(C)]
pub union hpi_response_union {
    pub s: hpi_subsys_res,
    pub ax: hpi_adapterx_res,
}

#[repr(C)]
pub struct hpi_response {
    pub size: u16,
    pub version: u8,
    pub type_: u8,
    pub object: u16,
    pub function: u16,
    pub error: u16,
    pub specific_error: u16,
    pub u: hpi_response_union,
}

#[repr(C)]
pub union hpi_message_union {
    pub s: hpi_subsys_res,
}

#[repr(C)]
pub struct hpi_message {
    pub size: u16,
    pub version: u8,
    pub type_: u8,
    pub object: u16,
    pub function: u16,
    pub adapter_index: u16,
    pub obj_index: u16,
    pub u: hpi_message_union,
}

const PCI_ANY_ID: u32 = !0u32;
const HPI_MAX_ADAPTERS: usize = 20;
const HPI_MAX_STREAMS: usize = 16;
const HPI_ADAPTER_INDEX_INVALID: u16 = 0xffff;
const HPIMSGX_ALLADAPTERS: u16 = 0xffff;
const HPI_TYPE_REQUEST: u8 = 1;
const HPI_VER: u32 = 0;

const DEBUG: u16 = 0;
const VERBOSE: u16 = 1;
const WARNING: u16 = 2;
const HPI_DEBUG_LEVEL_ERROR: u16 = 3;

const HPI_OBJ_SUBSYSTEM: u16 = 1;
const HPI_OBJ_ADAPTER: u16 = 2;
const HPI_OBJ_MIXER: u16 = 3;
const HPI_OBJ_OSTREAM: u16 = 4;
const HPI_OBJ_ISTREAM: u16 = 5;

const HPI_SUBSYS_GET_VERSION: u16 = 1;
const HPI_SUBSYS_OPEN: u16 = 2;
const HPI_SUBSYS_CLOSE: u16 = 3;
const HPI_SUBSYS_DRIVER_LOAD: u16 = 4;
const HPI_SUBSYS_DRIVER_UNLOAD: u16 = 5;
const HPI_SUBSYS_GET_NUM_ADAPTERS: u16 = 6;
const HPI_SUBSYS_GET_ADAPTER: u16 = 7;
const HPI_SUBSYS_CREATE_ADAPTER: u16 = 8;

const HPI_ADAPTER_OPEN: u16 = 1;
const HPI_ADAPTER_CLOSE: u16 = 2;
const HPI_ADAPTER_DELETE: u16 = 3;
const HPI_ADAPTER_GET_INFO: u16 = 4;

const HPI_MIXER_OPEN: u16 = 1;
const HPI_MIXER_CLOSE: u16 = 2;

const HPI_OSTREAM_OPEN: u16 = 1;
const HPI_OSTREAM_CLOSE: u16 = 2;
const HPI_OSTREAM_RESET: u16 = 3;
const HPI_OSTREAM_HOSTBUFFER_FREE: u16 = 4;
const HPI_OSTREAM_GROUP_RESET: u16 = 5;

const HPI_ISTREAM_OPEN: u16 = 1;
const HPI_ISTREAM_CLOSE: u16 = 2;
const HPI_ISTREAM_RESET: u16 = 3;
const HPI_ISTREAM_HOSTBUFFER_FREE: u16 = 4;
const HPI_ISTREAM_GROUP_RESET: u16 = 5;

const HPI_ERROR_PROCESSING_MESSAGE: u16 = 1;
const HPI_ERROR_INVALID_FUNC: u16 = 2;
const HPI_ERROR_INVALID_OBJ_INDEX: u16 = 3;
const HPI_ERROR_INVALID_TYPE: u16 = 4;
const HPI_ERROR_BAD_ADAPTER_NUMBER: u16 = 5;
const HPI_ERROR_OBJ_ALREADY_OPEN: u16 = 6;
const HPI_ERROR_OBJ_NOT_OPEN: u16 = 7;
const HPI_ERROR_BAD_ADAPTER: u16 = 8;
const HPI_ERROR_INVALID_OBJ: u16 = 9;
const HPI_ERROR_DSP_COMMUNICATION: u16 = 100;

static mut hpi_entry_points: [Option<hpi_handler_func>; HPI_MAX_ADAPTERS] =
    [None; HPI_MAX_ADAPTERS];
static mut logging_enabled: i32 = 1;

#[repr(C, packed)]
struct hpi_adapter_response {
    h: hpi_response_header,
    a: hpi_adapter_res,
}

#[repr(C, packed)]
struct hpi_mixer_response {
    h: hpi_response_header,
    m: hpi_mixer_res,
}

#[repr(C, packed)]
struct hpi_stream_response {
    h: hpi_response_header,
    d: hpi_stream_res,
}

#[repr(C, packed)]
struct adapter_info {
    type_: u16,
    num_instreams: u16,
    num_outstreams: u16,
}

#[repr(C, packed)]
struct asi_open_state {
    open_flag: i32,
    h_owner: *mut c_void,
}

static mut rESP_HPI_ADAPTER_OPEN: [hpi_adapter_response; HPI_MAX_ADAPTERS] =
    unsafe { zeroed() };
static mut rESP_HPI_OSTREAM_OPEN: [[hpi_stream_response; HPI_MAX_STREAMS]; HPI_MAX_ADAPTERS] =
    unsafe { zeroed() };
static mut rESP_HPI_ISTREAM_OPEN: [[hpi_stream_response; HPI_MAX_STREAMS]; HPI_MAX_ADAPTERS] =
    unsafe { zeroed() };
static mut rESP_HPI_MIXER_OPEN: [hpi_mixer_response; HPI_MAX_ADAPTERS] = unsafe { zeroed() };
static mut aDAPTER_INFO: [adapter_info; HPI_MAX_ADAPTERS] = unsafe { zeroed() };

/* use these to keep track of opens from user mode apps/DLLs */
static mut outstream_user_open: [[asi_open_state; HPI_MAX_STREAMS]; HPI_MAX_ADAPTERS] =
    unsafe { zeroed() };
static mut instream_user_open: [[asi_open_state; HPI_MAX_STREAMS]; HPI_MAX_ADAPTERS] =
    unsafe { zeroed() };

unsafe fn memcpy<T, U>(dst: *mut T, src: *const U, n: usize) {
    copy_nonoverlapping(src as *const u8, dst as *mut u8, n);
}

unsafe fn hpi_lookup_entry_point_function(pci_info: *const hpi_pci) -> Option<hpi_handler_func> {
    let mut i: isize = 0;

    while (*asihpi_pci_tbl.as_ptr().offset(i)).vendor != 0 {
        let tbl = asihpi_pci_tbl.as_ptr().offset(i);
        if (*tbl).vendor != PCI_ANY_ID && (*tbl).vendor != (*(*pci_info).pci_dev).vendor {
            i += 1;
            continue;
        }
        if (*tbl).device != PCI_ANY_ID && (*tbl).device != (*(*pci_info).pci_dev).device {
            i += 1;
            continue;
        }
        if (*tbl).subvendor != PCI_ANY_ID
            && (*tbl).subvendor != (*(*pci_info).pci_dev).subsystem_vendor
        {
            i += 1;
            continue;
        }
        if (*tbl).subdevice != PCI_ANY_ID
            && (*tbl).subdevice != (*(*pci_info).pci_dev).subsystem_device
        {
            i += 1;
            continue;
        }

        /* HPI_DEBUG_LOG(DEBUG, " %x,%lx\n", i,
           asihpi_pci_tbl[i].driver_data); */
        return Some(core::mem::transmute::<usize, hpi_handler_func>((*tbl).driver_data));
    }

    None
}

#[inline]
unsafe fn hw_entry_point(phm: *mut hpi_message, phr: *mut hpi_response) {
    if ((*phm).adapter_index as usize) < HPI_MAX_ADAPTERS
        && hpi_entry_points[(*phm).adapter_index as usize].is_some()
    {
        hpi_entry_points[(*phm).adapter_index as usize].unwrap()(phm, phr);
    } else {
        hpi_init_response(
            phr,
            (*phm).object,
            (*phm).function,
            HPI_ERROR_PROCESSING_MESSAGE,
        );
    }
}

unsafe fn subsys_message(phm: *mut hpi_message, phr: *mut hpi_response, h_owner: *mut c_void) {
    if (*phm).adapter_index != HPI_ADAPTER_INDEX_INVALID {
        HPI_DEBUG_LOG(
            WARNING,
            b"suspicious adapter index %d in subsys message 0x%x.\n\0".as_ptr(),
            (*phm).adapter_index as i32,
            (*phm).function as i32,
        );
    }

    match (*phm).function {
        HPI_SUBSYS_GET_VERSION => {
            hpi_init_response(phr, HPI_OBJ_SUBSYSTEM, HPI_SUBSYS_GET_VERSION, 0);
            (*phr).u.s.version = HPI_VER >> 8; /* return major.minor */
            (*phr).u.s.data = HPI_VER; /* return major.minor.release */
        }
        HPI_SUBSYS_OPEN => {
            /*do not propagate the message down the chain */
            hpi_init_response(phr, HPI_OBJ_SUBSYSTEM, HPI_SUBSYS_OPEN, 0);
        }
        HPI_SUBSYS_CLOSE => {
            /*do not propagate the message down the chain */
            hpi_init_response(phr, HPI_OBJ_SUBSYSTEM, HPI_SUBSYS_CLOSE, 0);
            HPIMSGX__cleanup(HPIMSGX_ALLADAPTERS, h_owner);
        }
        HPI_SUBSYS_DRIVER_LOAD => {
            /* Initialize this module's internal state */
            hpios_msgxlock_init(&mut msgx_lock);
            hpi_entry_points = [None; HPI_MAX_ADAPTERS];
            /* Init subsys_findadapters response to no-adapters */
            HPIMSGX__reset(HPIMSGX_ALLADAPTERS);
            hpi_init_response(phr, HPI_OBJ_SUBSYSTEM, HPI_SUBSYS_DRIVER_LOAD, 0);
            /* individual HPIs dont implement driver load */
            HPI_COMMON(phm, phr);
        }
        HPI_SUBSYS_DRIVER_UNLOAD => {
            HPI_COMMON(phm, phr);
            HPIMSGX__cleanup(HPIMSGX_ALLADAPTERS, h_owner);
            hpi_init_response(phr, HPI_OBJ_SUBSYSTEM, HPI_SUBSYS_DRIVER_UNLOAD, 0);
            return;
        }
        HPI_SUBSYS_GET_NUM_ADAPTERS | HPI_SUBSYS_GET_ADAPTER => {
            HPI_COMMON(phm, phr);
        }
        HPI_SUBSYS_CREATE_ADAPTER => {
            HPIMSGX__init(phm, phr);
        }
        _ => {
            /* Must explicitly handle every subsys message in this switch */
            hpi_init_response(
                phr,
                HPI_OBJ_SUBSYSTEM,
                (*phm).function,
                HPI_ERROR_INVALID_FUNC,
            );
        }
    }
}

unsafe fn adapter_message(phm: *mut hpi_message, phr: *mut hpi_response, h_owner: *mut c_void) {
    match (*phm).function {
        HPI_ADAPTER_OPEN => adapter_open(phm, phr),
        HPI_ADAPTER_CLOSE => adapter_close(phm, phr),
        HPI_ADAPTER_DELETE => {
            HPIMSGX__cleanup((*phm).adapter_index, h_owner);
            {
                let mut hm: hpi_message = zeroed();
                let mut hr: hpi_response = zeroed();
                hpi_init_message_response(&mut hm, &mut hr, HPI_OBJ_ADAPTER, HPI_ADAPTER_CLOSE);
                hm.adapter_index = (*phm).adapter_index;
                hw_entry_point(&mut hm, &mut hr);
            }
            hw_entry_point(phm, phr);
        }
        _ => hw_entry_point(phm, phr),
    }
}

unsafe fn mixer_message(phm: *mut hpi_message, phr: *mut hpi_response) {
    match (*phm).function {
        HPI_MIXER_OPEN => mixer_open(phm, phr),
        HPI_MIXER_CLOSE => mixer_close(phm, phr),
        _ => hw_entry_point(phm, phr),
    }
}

unsafe fn outstream_message(phm: *mut hpi_message, phr: *mut hpi_response, h_owner: *mut c_void) {
    if (*phm).obj_index >= aDAPTER_INFO[(*phm).adapter_index as usize].num_outstreams {
        hpi_init_response(
            phr,
            HPI_OBJ_OSTREAM,
            (*phm).function,
            HPI_ERROR_INVALID_OBJ_INDEX,
        );
        return;
    }

    match (*phm).function {
        HPI_OSTREAM_OPEN => outstream_open(phm, phr, h_owner),
        HPI_OSTREAM_CLOSE => outstream_close(phm, phr, h_owner),
        _ => hw_entry_point(phm, phr),
    }
}

unsafe fn instream_message(phm: *mut hpi_message, phr: *mut hpi_response, h_owner: *mut c_void) {
    if (*phm).obj_index >= aDAPTER_INFO[(*phm).adapter_index as usize].num_instreams {
        hpi_init_response(
            phr,
            HPI_OBJ_ISTREAM,
            (*phm).function,
            HPI_ERROR_INVALID_OBJ_INDEX,
        );
        return;
    }

    match (*phm).function {
        HPI_ISTREAM_OPEN => instream_open(phm, phr, h_owner),
        HPI_ISTREAM_CLOSE => instream_close(phm, phr, h_owner),
        _ => hw_entry_point(phm, phr),
    }
}

/* NOTE: HPI_Message() must be defined in the driver as a wrapper for
 * HPI_MessageEx so that functions in hpifunc.c compile.
 */
#[no_mangle]
pub unsafe extern "C" fn hpi_send_recv_ex(
    phm: *mut hpi_message,
    phr: *mut hpi_response,
    h_owner: *mut c_void,
) {
    if logging_enabled != 0 {
        HPI_DEBUG_MESSAGE(DEBUG, phm);
    }

    if (*phm).type_ != HPI_TYPE_REQUEST {
        hpi_init_response(
            phr,
            (*phm).object,
            (*phm).function,
            HPI_ERROR_INVALID_TYPE,
        );
        return;
    }

    if (*phm).adapter_index as usize >= HPI_MAX_ADAPTERS
        && (*phm).adapter_index != HPIMSGX_ALLADAPTERS
    {
        hpi_init_response(
            phr,
            (*phm).object,
            (*phm).function,
            HPI_ERROR_BAD_ADAPTER_NUMBER,
        );
        return;
    }

    match (*phm).object {
        HPI_OBJ_SUBSYSTEM => subsys_message(phm, phr, h_owner),
        HPI_OBJ_ADAPTER => adapter_message(phm, phr, h_owner),
        HPI_OBJ_MIXER => mixer_message(phm, phr),
        HPI_OBJ_OSTREAM => outstream_message(phm, phr, h_owner),
        HPI_OBJ_ISTREAM => instream_message(phm, phr, h_owner),
        _ => hw_entry_point(phm, phr),
    }

    if logging_enabled != 0 {
        HPI_DEBUG_RESPONSE(phr);
    }

    if (*phr).error >= HPI_ERROR_DSP_COMMUNICATION {
        hpi_debug_level_set(HPI_DEBUG_LEVEL_ERROR);
        logging_enabled = 0;
    }
}

unsafe fn adapter_open(phm: *mut hpi_message, phr: *mut hpi_response) {
    HPI_DEBUG_LOG(VERBOSE, b"adapter_open\n\0".as_ptr());
    memcpy(
        phr,
        &rESP_HPI_ADAPTER_OPEN[(*phm).adapter_index as usize],
        size_of::<hpi_adapter_response>(),
    );
}

unsafe fn adapter_close(_phm: *mut hpi_message, phr: *mut hpi_response) {
    HPI_DEBUG_LOG(VERBOSE, b"adapter_close\n\0".as_ptr());
    hpi_init_response(phr, HPI_OBJ_ADAPTER, HPI_ADAPTER_CLOSE, 0);
}

unsafe fn mixer_open(phm: *mut hpi_message, phr: *mut hpi_response) {
    memcpy(
        phr,
        &rESP_HPI_MIXER_OPEN[(*phm).adapter_index as usize],
        size_of::<hpi_mixer_response>(),
    );
}

unsafe fn mixer_close(_phm: *mut hpi_message, phr: *mut hpi_response) {
    hpi_init_response(phr, HPI_OBJ_MIXER, HPI_MIXER_CLOSE, 0);
}

unsafe fn instream_open(phm: *mut hpi_message, phr: *mut hpi_response, h_owner: *mut c_void) {
    let mut hm: hpi_message = zeroed();
    let mut hr: hpi_response = zeroed();

    hpi_init_response(phr, HPI_OBJ_ISTREAM, HPI_ISTREAM_OPEN, 0);

    hpios_msgxlock_lock(&mut msgx_lock);

    if instream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize].open_flag != 0 {
        (*phr).error = HPI_ERROR_OBJ_ALREADY_OPEN;
    } else if rESP_HPI_ISTREAM_OPEN[(*phm).adapter_index as usize][(*phm).obj_index as usize]
        .h
        .error
        != 0
    {
        memcpy(
            phr,
            &rESP_HPI_ISTREAM_OPEN[(*phm).adapter_index as usize][(*phm).obj_index as usize],
            size_of::<hpi_stream_response>(),
        );
    } else {
        instream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize].open_flag = 1;
        hpios_msgxlock_unlock(&mut msgx_lock);

        /* issue a reset */
        hpi_init_message_response(&mut hm, &mut hr, HPI_OBJ_ISTREAM, HPI_ISTREAM_RESET);
        hm.adapter_index = (*phm).adapter_index;
        hm.obj_index = (*phm).obj_index;
        hw_entry_point(&mut hm, &mut hr);

        hpios_msgxlock_lock(&mut msgx_lock);
        if hr.error != 0 {
            instream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize]
                .open_flag = 0;
            (*phr).error = hr.error;
        } else {
            instream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize]
                .open_flag = 1;
            instream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize]
                .h_owner = h_owner;
            memcpy(
                phr,
                &rESP_HPI_ISTREAM_OPEN[(*phm).adapter_index as usize][(*phm).obj_index as usize],
                size_of::<hpi_stream_response>(),
            );
        }
    }
    hpios_msgxlock_unlock(&mut msgx_lock);
}

unsafe fn instream_close(phm: *mut hpi_message, phr: *mut hpi_response, h_owner: *mut c_void) {
    let mut hm: hpi_message = zeroed();
    let mut hr: hpi_response = zeroed();

    hpi_init_response(phr, HPI_OBJ_ISTREAM, HPI_ISTREAM_CLOSE, 0);

    hpios_msgxlock_lock(&mut msgx_lock);
    if h_owner
        == instream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize].h_owner
    {
        /* HPI_DEBUG_LOG(INFO,"closing adapter %d "
           "instream %d owned by %p\n",
           phm->wAdapterIndex, phm->wObjIndex, hOwner); */
        instream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize].h_owner =
            null_mut();
        hpios_msgxlock_unlock(&mut msgx_lock);
        /* issue a reset */
        hpi_init_message_response(&mut hm, &mut hr, HPI_OBJ_ISTREAM, HPI_ISTREAM_RESET);
        hm.adapter_index = (*phm).adapter_index;
        hm.obj_index = (*phm).obj_index;
        hw_entry_point(&mut hm, &mut hr);
        hpios_msgxlock_lock(&mut msgx_lock);
        if hr.error != 0 {
            instream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize]
                .h_owner = h_owner;
            (*phr).error = hr.error;
        } else {
            instream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize]
                .open_flag = 0;
            instream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize]
                .h_owner = null_mut();
        }
    } else {
        HPI_DEBUG_LOG(
            WARNING,
            b"%p trying to close %d instream %d owned by %p\n\0".as_ptr(),
            h_owner,
            (*phm).adapter_index as i32,
            (*phm).obj_index as i32,
            instream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize].h_owner,
        );
        (*phr).error = HPI_ERROR_OBJ_NOT_OPEN;
    }
    hpios_msgxlock_unlock(&mut msgx_lock);
}

unsafe fn outstream_open(phm: *mut hpi_message, phr: *mut hpi_response, h_owner: *mut c_void) {
    let mut hm: hpi_message = zeroed();
    let mut hr: hpi_response = zeroed();

    hpi_init_response(phr, HPI_OBJ_OSTREAM, HPI_OSTREAM_OPEN, 0);

    hpios_msgxlock_lock(&mut msgx_lock);

    if outstream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize].open_flag != 0
    {
        (*phr).error = HPI_ERROR_OBJ_ALREADY_OPEN;
    } else if rESP_HPI_OSTREAM_OPEN[(*phm).adapter_index as usize][(*phm).obj_index as usize]
        .h
        .error
        != 0
    {
        memcpy(
            phr,
            &rESP_HPI_OSTREAM_OPEN[(*phm).adapter_index as usize][(*phm).obj_index as usize],
            size_of::<hpi_stream_response>(),
        );
    } else {
        outstream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize].open_flag =
            1;
        hpios_msgxlock_unlock(&mut msgx_lock);

        /* issue a reset */
        hpi_init_message_response(&mut hm, &mut hr, HPI_OBJ_OSTREAM, HPI_OSTREAM_RESET);
        hm.adapter_index = (*phm).adapter_index;
        hm.obj_index = (*phm).obj_index;
        hw_entry_point(&mut hm, &mut hr);

        hpios_msgxlock_lock(&mut msgx_lock);
        if hr.error != 0 {
            outstream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize]
                .open_flag = 0;
            (*phr).error = hr.error;
        } else {
            outstream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize]
                .open_flag = 1;
            outstream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize]
                .h_owner = h_owner;
            memcpy(
                phr,
                &rESP_HPI_OSTREAM_OPEN[(*phm).adapter_index as usize][(*phm).obj_index as usize],
                size_of::<hpi_stream_response>(),
            );
        }
    }
    hpios_msgxlock_unlock(&mut msgx_lock);
}

unsafe fn outstream_close(phm: *mut hpi_message, phr: *mut hpi_response, h_owner: *mut c_void) {
    let mut hm: hpi_message = zeroed();
    let mut hr: hpi_response = zeroed();

    hpi_init_response(phr, HPI_OBJ_OSTREAM, HPI_OSTREAM_CLOSE, 0);

    hpios_msgxlock_lock(&mut msgx_lock);

    if h_owner
        == outstream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize].h_owner
    {
        /* HPI_DEBUG_LOG(INFO,"closing adapter %d "
           "outstream %d owned by %p\n",
           phm->wAdapterIndex, phm->wObjIndex, hOwner); */
        outstream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize].h_owner =
            null_mut();
        hpios_msgxlock_unlock(&mut msgx_lock);
        /* issue a reset */
        hpi_init_message_response(&mut hm, &mut hr, HPI_OBJ_OSTREAM, HPI_OSTREAM_RESET);
        hm.adapter_index = (*phm).adapter_index;
        hm.obj_index = (*phm).obj_index;
        hw_entry_point(&mut hm, &mut hr);
        hpios_msgxlock_lock(&mut msgx_lock);
        if hr.error != 0 {
            outstream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize]
                .h_owner = h_owner;
            (*phr).error = hr.error;
        } else {
            outstream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize]
                .open_flag = 0;
            outstream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize]
                .h_owner = null_mut();
        }
    } else {
        HPI_DEBUG_LOG(
            WARNING,
            b"%p trying to close %d outstream %d owned by %p\n\0".as_ptr(),
            h_owner,
            (*phm).adapter_index as i32,
            (*phm).obj_index as i32,
            outstream_user_open[(*phm).adapter_index as usize][(*phm).obj_index as usize].h_owner,
        );
        (*phr).error = HPI_ERROR_OBJ_NOT_OPEN;
    }
    hpios_msgxlock_unlock(&mut msgx_lock);
}

unsafe fn adapter_prepare(adapter: u16) -> u16 {
    let mut hm: hpi_message = zeroed();
    let mut hr: hpi_response = zeroed();

    /* Open the adapter and streams */
    let mut i: u16;

    /* call to HPI_ADAPTER_OPEN */
    hpi_init_message_response(&mut hm, &mut hr, HPI_OBJ_ADAPTER, HPI_ADAPTER_OPEN);
    hm.adapter_index = adapter;
    hw_entry_point(&mut hm, &mut hr);
    memcpy(
        &mut rESP_HPI_ADAPTER_OPEN[adapter as usize].h,
        &hr,
        size_of::<hpi_response_header>(),
    );
    memcpy(
        &mut rESP_HPI_ADAPTER_OPEN[adapter as usize].a,
        &hr.u.ax.info,
        size_of::<hpi_adapter_res>(),
    );
    if hr.error != 0 {
        return hr.error;
    }

    /* call to HPI_ADAPTER_GET_INFO */
    hpi_init_message_response(&mut hm, &mut hr, HPI_OBJ_ADAPTER, HPI_ADAPTER_GET_INFO);
    hm.adapter_index = adapter;
    hw_entry_point(&mut hm, &mut hr);
    if hr.error != 0 {
        return hr.error;
    }

    aDAPTER_INFO[adapter as usize].num_outstreams = hr.u.ax.info.num_outstreams;
    aDAPTER_INFO[adapter as usize].num_instreams = hr.u.ax.info.num_instreams;
    aDAPTER_INFO[adapter as usize].type_ = hr.u.ax.info.adapter_type;

    /* call to HPI_OSTREAM_OPEN */
    i = 0;
    while i < aDAPTER_INFO[adapter as usize].num_outstreams {
        hpi_init_message_response(&mut hm, &mut hr, HPI_OBJ_OSTREAM, HPI_OSTREAM_OPEN);
        hm.adapter_index = adapter;
        hm.obj_index = i;
        hw_entry_point(&mut hm, &mut hr);
        memcpy(
            &mut rESP_HPI_OSTREAM_OPEN[adapter as usize][i as usize],
            &hr,
            size_of::<hpi_stream_response>(),
        );
        outstream_user_open[adapter as usize][i as usize].open_flag = 0;
        outstream_user_open[adapter as usize][i as usize].h_owner = null_mut();
        i = i.wrapping_add(1);
    }

    /* call to HPI_ISTREAM_OPEN */
    i = 0;
    while i < aDAPTER_INFO[adapter as usize].num_instreams {
        hpi_init_message_response(&mut hm, &mut hr, HPI_OBJ_ISTREAM, HPI_ISTREAM_OPEN);
        hm.adapter_index = adapter;
        hm.obj_index = i;
        hw_entry_point(&mut hm, &mut hr);
        memcpy(
            &mut rESP_HPI_ISTREAM_OPEN[adapter as usize][i as usize],
            &hr,
            size_of::<hpi_stream_response>(),
        );
        instream_user_open[adapter as usize][i as usize].open_flag = 0;
        instream_user_open[adapter as usize][i as usize].h_owner = null_mut();
        i = i.wrapping_add(1);
    }

    /* call to HPI_MIXER_OPEN */
    hpi_init_message_response(&mut hm, &mut hr, HPI_OBJ_MIXER, HPI_MIXER_OPEN);
    hm.adapter_index = adapter;
    hw_entry_point(&mut hm, &mut hr);
    memcpy(
        &mut rESP_HPI_MIXER_OPEN[adapter as usize],
        &hr,
        size_of::<hpi_mixer_response>(),
    );

    0
}

unsafe fn HPIMSGX__reset(adapter_index: u16) {
    let mut i: i32;
    let mut adapter: u16;
    let mut hr: hpi_response = zeroed();

    if adapter_index == HPIMSGX_ALLADAPTERS {
        adapter = 0;
        while (adapter as usize) < HPI_MAX_ADAPTERS {
            hpi_init_response(
                &mut hr,
                HPI_OBJ_ADAPTER,
                HPI_ADAPTER_OPEN,
                HPI_ERROR_BAD_ADAPTER,
            );
            memcpy(
                &mut rESP_HPI_ADAPTER_OPEN[adapter as usize],
                &hr,
                size_of::<hpi_adapter_response>(),
            );

            hpi_init_response(
                &mut hr,
                HPI_OBJ_MIXER,
                HPI_MIXER_OPEN,
                HPI_ERROR_INVALID_OBJ,
            );
            memcpy(
                &mut rESP_HPI_MIXER_OPEN[adapter as usize],
                &hr,
                size_of::<hpi_mixer_response>(),
            );

            i = 0;
            while (i as usize) < HPI_MAX_STREAMS {
                hpi_init_response(
                    &mut hr,
                    HPI_OBJ_OSTREAM,
                    HPI_OSTREAM_OPEN,
                    HPI_ERROR_INVALID_OBJ,
                );
                memcpy(
                    &mut rESP_HPI_OSTREAM_OPEN[adapter as usize][i as usize],
                    &hr,
                    size_of::<hpi_stream_response>(),
                );
                hpi_init_response(
                    &mut hr,
                    HPI_OBJ_ISTREAM,
                    HPI_ISTREAM_OPEN,
                    HPI_ERROR_INVALID_OBJ,
                );
                memcpy(
                    &mut rESP_HPI_ISTREAM_OPEN[adapter as usize][i as usize],
                    &hr,
                    size_of::<hpi_stream_response>(),
                );
                i += 1;
            }
            adapter = adapter.wrapping_add(1);
        }
    } else if (adapter_index as usize) < HPI_MAX_ADAPTERS {
        rESP_HPI_ADAPTER_OPEN[adapter_index as usize].h.error = HPI_ERROR_BAD_ADAPTER;
        rESP_HPI_MIXER_OPEN[adapter_index as usize].h.error = HPI_ERROR_INVALID_OBJ;
        i = 0;
        while (i as usize) < HPI_MAX_STREAMS {
            rESP_HPI_OSTREAM_OPEN[adapter_index as usize][i as usize]
                .h
                .error = HPI_ERROR_INVALID_OBJ;
            rESP_HPI_ISTREAM_OPEN[adapter_index as usize][i as usize]
                .h
                .error = HPI_ERROR_INVALID_OBJ;
            i += 1;
        }
    }
}

unsafe fn HPIMSGX__init(phm: *mut hpi_message, phr: *mut hpi_response) -> u16 {
    let mut hr: hpi_response = zeroed();

    /* Init response here so we can pass in previous adapter list */
    hpi_init_response(&mut hr, (*phm).object, (*phm).function, HPI_ERROR_INVALID_OBJ);

    let entry_point_func = hpi_lookup_entry_point_function((*phm).u.s.resource.r.pci);

    if let Some(entry_point_func) = entry_point_func {
        HPI_DEBUG_MESSAGE(DEBUG, phm);
        entry_point_func(phm, &mut hr);
    } else {
        (*phr).error = HPI_ERROR_PROCESSING_MESSAGE;
        return (*phr).error;
    }
    if hr.error == 0 && hr.u.s.adapter_index as usize <= HPI_MAX_ADAPTERS - 1 {
        /* the adapter was created successfully
           save the mapping for future use */
        hpi_entry_points[hr.u.s.adapter_index as usize] = entry_point_func.into();
        /* prepare adapter (pre-open streams etc.) */
        HPI_DEBUG_LOG(
            DEBUG,
            b"HPI_SUBSYS_CREATE_ADAPTER successful, preparing adapter\n\0".as_ptr(),
        );
        adapter_prepare(hr.u.s.adapter_index);
    }
    memcpy(phr, &hr, hr.size as usize);
    (*phr).error
}

unsafe fn HPIMSGX__cleanup(adapter_index: u16, h_owner: *mut c_void) {
    let mut i: i32;
    let mut adapter: i32;
    let adapter_limit: i32;

    if h_owner.is_null() {
        return;
    }

    if adapter_index == HPIMSGX_ALLADAPTERS {
        adapter = 0;
        adapter_limit = HPI_MAX_ADAPTERS as i32;
    } else {
        adapter = adapter_index as i32;
        adapter_limit = adapter + 1;
    }

    while adapter < adapter_limit {
        /*      printk(KERN_INFO "Cleanup adapter #%d\n",wAdapter); */
        i = 0;
        while (i as usize) < HPI_MAX_STREAMS {
            if h_owner == outstream_user_open[adapter as usize][i as usize].h_owner {
                let mut hm: hpi_message = zeroed();
                let mut hr: hpi_response = zeroed();

                HPI_DEBUG_LOG(
                    DEBUG,
                    b"Close adapter %d ostream %d\n\0".as_ptr(),
                    adapter,
                    i,
                );

                hpi_init_message_response(
                    &mut hm,
                    &mut hr,
                    HPI_OBJ_OSTREAM,
                    HPI_OSTREAM_RESET,
                );
                hm.adapter_index = adapter as u16;
                hm.obj_index = i as u16;
                hw_entry_point(&mut hm, &mut hr);

                hm.function = HPI_OSTREAM_HOSTBUFFER_FREE;
                hw_entry_point(&mut hm, &mut hr);

                hm.function = HPI_OSTREAM_GROUP_RESET;
                hw_entry_point(&mut hm, &mut hr);

                outstream_user_open[adapter as usize][i as usize].open_flag = 0;
                outstream_user_open[adapter as usize][i as usize].h_owner = null_mut();
            }
            if h_owner == instream_user_open[adapter as usize][i as usize].h_owner {
                let mut hm: hpi_message = zeroed();
                let mut hr: hpi_response = zeroed();

                HPI_DEBUG_LOG(
                    DEBUG,
                    b"Close adapter %d istream %d\n\0".as_ptr(),
                    adapter,
                    i,
                );

                hpi_init_message_response(
                    &mut hm,
                    &mut hr,
                    HPI_OBJ_ISTREAM,
                    HPI_ISTREAM_RESET,
                );
                hm.adapter_index = adapter as u16;
                hm.obj_index = i as u16;
                hw_entry_point(&mut hm, &mut hr);

                hm.function = HPI_ISTREAM_HOSTBUFFER_FREE;
                hw_entry_point(&mut hm, &mut hr);

                hm.function = HPI_ISTREAM_GROUP_RESET;
                hw_entry_point(&mut hm, &mut hr);

                instream_user_open[adapter as usize][i as usize].open_flag = 0;
                instream_user_open[adapter as usize][i as usize].h_owner = null_mut();
            }
            i += 1;
        }
        adapter += 1;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
