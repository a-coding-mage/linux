// SPDX-License-Identifier: GPL-2.0-only
/******************************************************************************

    AudioScience HPI driver
    Copyright (C) 1997-2014  AudioScience Inc. <support@audioscience.com>


 Hardware Programming Interface (HPI) Utility functions.

 (C) Copyright AudioScience Inc. 2007
*******************************************************************************/

// C dependencies: "hpi_internal.h", "hpimsginit.h", <linux/nospec.h>

/* The actual message size for each object type */
static mut msg_size: [u16; (HPI_OBJ_MAXINDEX + 1) as usize] = HPI_MESSAGE_SIZE_BY_OBJECT;
/* The actual response size for each object type */
static mut res_size: [u16; (HPI_OBJ_MAXINDEX + 1) as usize] = HPI_RESPONSE_SIZE_BY_OBJECT;
/* Flag to enable alternate message type for SSX2 bypass. */
static mut gwSSX2_bypass: u16 = 0;

/** \internal
  * initialize the HPI message structure
  */
unsafe fn hpi_init_message(phm: *mut hpi_message, mut object: u16, function: u16) {
    let size: u16;

    if object > 0 && object <= HPI_OBJ_MAXINDEX {
        object = array_index_nospec(object, HPI_OBJ_MAXINDEX + 1);
        size = msg_size[object as usize];
    } else {
        size = core::mem::size_of::<hpi_message>() as u16;
    }

    core::ptr::write_bytes(phm as *mut u8, 0, size as usize);
    (*phm).size = size;

    if gwSSX2_bypass != 0 {
        (*phm).type_ = HPI_TYPE_SSX2BYPASS_MESSAGE;
    } else {
        (*phm).type_ = HPI_TYPE_REQUEST;
    }
    (*phm).object = object;
    (*phm).function = function;
    (*phm).version = 0;
    (*phm).adapter_index = HPI_ADAPTER_INDEX_INVALID;
    /* Expect actual adapter index to be set by caller */
}

/** \internal
  * initialize the HPI response structure
  */
pub unsafe fn hpi_init_response(
    phr: *mut hpi_response,
    mut object: u16,
    function: u16,
    error: u16,
) {
    let size: u16;

    if object > 0 && object <= HPI_OBJ_MAXINDEX {
        object = array_index_nospec(object, HPI_OBJ_MAXINDEX + 1);
        size = res_size[object as usize];
    } else {
        size = core::mem::size_of::<hpi_response>() as u16;
    }

    core::ptr::write_bytes(
        phr as *mut u8,
        0,
        core::mem::size_of::<hpi_response>(),
    );
    (*phr).size = size;
    (*phr).type_ = HPI_TYPE_RESPONSE;
    (*phr).object = object;
    (*phr).function = function;
    (*phr).error = error;
    (*phr).specific_error = 0;
    (*phr).version = 0;
}

pub unsafe fn hpi_init_message_response(
    phm: *mut hpi_message,
    phr: *mut hpi_response,
    object: u16,
    function: u16,
) {
    hpi_init_message(phm, object, function);
    /* default error return if the response is
       not filled in by the callee */
    hpi_init_response(phr, object, function, HPI_ERROR_PROCESSING_MESSAGE);
}

unsafe fn hpi_init_messageV1(
    phm: *mut hpi_message_header,
    size: u16,
    object: u16,
    function: u16,
) {
    core::ptr::write_bytes(phm as *mut u8, 0, size as usize);
    if object > 0 && object <= HPI_OBJ_MAXINDEX {
        (*phm).size = size;
        (*phm).type_ = HPI_TYPE_REQUEST;
        (*phm).object = object;
        (*phm).function = function;
        (*phm).version = 1;
        /* Expect adapter index to be set by caller */
    }
}

pub unsafe fn hpi_init_responseV1(
    phr: *mut hpi_response_header,
    size: u16,
    object: u16,
    function: u16,
) {
    let _ = object;
    let _ = function;
    core::ptr::write_bytes(phr as *mut u8, 0, size as usize);
    (*phr).size = size;
    (*phr).version = 1;
    (*phr).type_ = HPI_TYPE_RESPONSE;
    (*phr).error = HPI_ERROR_PROCESSING_MESSAGE;
}

pub unsafe fn hpi_init_message_responseV1(
    phm: *mut hpi_message_header,
    msg_size: u16,
    phr: *mut hpi_response_header,
    res_size: u16,
    object: u16,
    function: u16,
) {
    hpi_init_messageV1(phm, msg_size, object, function);
    hpi_init_responseV1(phr, res_size, object, function);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
