/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  S390 version
 *    Copyright IBM Corp. 1999
 *    Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com),
 *               Christian Borntraeger (cborntra@de.ibm.com),
 */

/*
 * the lowlevel function for cpcmd
 */
extern "C" {
    pub fn __cpcmd(
        cmd: *const ::core::ffi::c_char,
        response: *mut ::core::ffi::c_char,
        rlen: ::core::ffi::c_int,
        response_code: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

/*
 * cpcmd is the in-kernel interface for issuing CP commands
 *
 * cmd:        null-terminated command string, max 240 characters
 * response:   response buffer for VM's textual response
 * rlen:       size of the response buffer, cpcmd will not exceed this size
 *             but will cap the output, if its too large. Everything that
 *             did not fit into the buffer will be silently dropped
 * response_code: return pointer for VM's error code
 * return value: the size of the response. The caller can check if the buffer
 *             was large enough by comparing the return value and rlen
 * NOTE: If the response buffer is not in real storage, cpcmd can sleep
 */
extern "C" {
    pub fn cpcmd(
        cmd: *const ::core::ffi::c_char,
        response: *mut ::core::ffi::c_char,
        rlen: ::core::ffi::c_int,
        response_code: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
