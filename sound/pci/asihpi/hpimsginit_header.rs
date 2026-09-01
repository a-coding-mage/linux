// SPDX-License-Identifier: GPL-2.0-only
/******************************************************************************

    AudioScience HPI driver
    Copyright (C) 1997-2011  AudioScience Inc. <support@audioscience.com>


 Hardware Programming Interface (HPI) Utility functions

 (C) Copyright AudioScience Inc. 2007
*******************************************************************************/
/* Initialise response headers, or msg/response pairs.
Note that it is valid to just init a response e.g. when a lower level is
preparing a response to a message.
However, when sending a message, a matching response buffer must always be
prepared.
*/

use crate::{
    hpi_message, hpi_message_header, hpi_response, hpi_response_header,
};

unsafe extern "C" {
    pub fn hpi_init_response(
        phr: *mut hpi_response,
        object: u16,
        function: u16,
        error: u16,
    );

    pub fn hpi_init_message_response(
        phm: *mut hpi_message,
        phr: *mut hpi_response,
        object: u16,
        function: u16,
    );

    pub fn hpi_init_responseV1(
        phr: *mut hpi_response_header,
        size: u16,
        object: u16,
        function: u16,
    );

    pub fn hpi_init_message_responseV1(
        phm: *mut hpi_message_header,
        msg_size: u16,
        phr: *mut hpi_response_header,
        res_size: u16,
        object: u16,
        function: u16,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
