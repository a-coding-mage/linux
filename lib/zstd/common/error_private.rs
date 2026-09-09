// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */

/* The purpose of this file is to have a single list of error strings embedded in binary */

use core::ffi::c_char;

// The error type and PREFIX! macro are supplied by the translated dependencies.
pub unsafe extern "C" fn ERR_getErrorString(code: ERR_enum) -> *const c_char {
    #[cfg(feature = "ZSTD_STRIP_ERROR_STRINGS")]
    {
        let _ = code;
        return b"Error strings stripped\0".as_ptr() as *const c_char;
    }

    #[cfg(not(feature = "ZSTD_STRIP_ERROR_STRINGS"))]
    {
        static NOT_ERROR_CODE: &[u8] = b"Unspecified error code\0";
        match code {
            PREFIX!(no_error) => b"No error detected\0".as_ptr() as *const c_char,
            PREFIX!(GENERIC) => b"Error (generic)\0".as_ptr() as *const c_char,
            PREFIX!(prefix_unknown) => b"Unknown frame descriptor\0".as_ptr() as *const c_char,
            PREFIX!(version_unsupported) => b"Version not supported\0".as_ptr() as *const c_char,
            PREFIX!(frameParameter_unsupported) => b"Unsupported frame parameter\0".as_ptr() as *const c_char,
            PREFIX!(frameParameter_windowTooLarge) => b"Frame requires too much memory for decoding\0".as_ptr() as *const c_char,
            PREFIX!(corruption_detected) => b"Data corruption detected\0".as_ptr() as *const c_char,
            PREFIX!(checksum_wrong) => b"Restored data doesn't match checksum\0".as_ptr() as *const c_char,
            PREFIX!(literals_headerWrong) => b"Header of Literals' block doesn't respect format specification\0".as_ptr() as *const c_char,
            PREFIX!(parameter_unsupported) => b"Unsupported parameter\0".as_ptr() as *const c_char,
            PREFIX!(parameter_combination_unsupported) => b"Unsupported combination of parameters\0".as_ptr() as *const c_char,
            PREFIX!(parameter_outOfBound) => b"Parameter is out of bound\0".as_ptr() as *const c_char,
            PREFIX!(init_missing) => b"Context should be init first\0".as_ptr() as *const c_char,
            PREFIX!(memory_allocation) => b"Allocation error : not enough memory\0".as_ptr() as *const c_char,
            PREFIX!(workSpace_tooSmall) => b"workSpace buffer is not large enough\0".as_ptr() as *const c_char,
            PREFIX!(stage_wrong) => b"Operation not authorized at current processing stage\0".as_ptr() as *const c_char,
            PREFIX!(tableLog_tooLarge) => b"tableLog requires too much memory : unsupported\0".as_ptr() as *const c_char,
            PREFIX!(maxSymbolValue_tooLarge) => b"Unsupported max Symbol Value : too large\0".as_ptr() as *const c_char,
            PREFIX!(maxSymbolValue_tooSmall) => b"Specified maxSymbolValue is too small\0".as_ptr() as *const c_char,
            PREFIX!(cannotProduce_uncompressedBlock) => b"This mode cannot generate an uncompressed block\0".as_ptr() as *const c_char,
            PREFIX!(stabilityCondition_notRespected) => b"pledged buffer stability condition is not respected\0".as_ptr() as *const c_char,
            PREFIX!(dictionary_corrupted) => b"Dictionary is corrupted\0".as_ptr() as *const c_char,
            PREFIX!(dictionary_wrong) => b"Dictionary mismatch\0".as_ptr() as *const c_char,
            PREFIX!(dictionaryCreation_failed) => b"Cannot create Dictionary from provided samples\0".as_ptr() as *const c_char,
            PREFIX!(dstSize_tooSmall) => b"Destination buffer is too small\0".as_ptr() as *const c_char,
            PREFIX!(srcSize_wrong) => b"Src size is incorrect\0".as_ptr() as *const c_char,
            PREFIX!(dstBuffer_null) => b"Operation on NULL destination buffer\0".as_ptr() as *const c_char,
            PREFIX!(noForwardProgress_destFull) => b"Operation made no progress over multiple calls, due to output buffer being full\0".as_ptr() as *const c_char,
            PREFIX!(noForwardProgress_inputEmpty) => b"Operation made no progress over multiple calls, due to input being empty\0".as_ptr() as *const c_char,
            /* following error codes are not stable and may be removed or changed in a future version */
            PREFIX!(frameIndex_tooLarge) => b"Frame index is too large\0".as_ptr() as *const c_char,
            PREFIX!(seekableIO) => b"An I/O error occurred when reading/seeking\0".as_ptr() as *const c_char,
            PREFIX!(dstBuffer_wrong) => b"Destination buffer is wrong\0".as_ptr() as *const c_char,
            PREFIX!(srcBuffer_wrong) => b"Source buffer is wrong\0".as_ptr() as *const c_char,
            PREFIX!(sequenceProducer_failed) => b"Block-level external sequence producer returned an error code\0".as_ptr() as *const c_char,
            PREFIX!(externalSequences_invalid) => b"External sequences are not valid\0".as_ptr() as *const c_char,
            PREFIX!(maxCode) => NOT_ERROR_CODE.as_ptr() as *const c_char,
            _ => NOT_ERROR_CODE.as_ptr() as *const c_char,
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
