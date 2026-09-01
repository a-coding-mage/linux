/* SPDX-License-Identifier: GPL-2.0-only */
/***********************************************************************/
/*

    AudioScience HPI driver
    Copyright (C) 1997-2011  AudioScience Inc. <support@audioscience.com>


\file
Functions for reading DSP code to load into DSP

*/
/***********************************************************************/

// Depends on declarations from hpi_internal.h, including u32-compatible types.

/** Header structure for dsp firmware file
 This structure must match that used in s2bin.c for generation of asidsp.bin
 */
/* Original C header had disabled pack pragmas around this structure. */
#[repr(C)]
pub struct code_header {
    /** Size in bytes including header */
    pub size: u32,
    /** File type tag "CODE" == 0x45444F43 */
    pub type_: u32,
    /** Adapter model number */
    pub adapter: u32,
    /** Firmware version*/
    pub version: u32,
    /** Data checksum */
    pub checksum: u32,
}

/*? Don't need the pragmas? */
const _: [(); 20] = [(); core::mem::size_of::<code_header>()];

#[repr(C)]
pub struct dsp_code_private {
    _unused: [u8; 0],
}

/** Descriptor for dspcode from firmware loader */
#[repr(C)]
pub struct dsp_code {
    /** copy of  file header */
    pub header: code_header,
    /** Expected number of words in the whole dsp code,INCL header */
    pub block_length: u32,
    /** Number of words read so far */
    pub word_count: u32,

    /** internal state of DSP code reader */
    pub pvt: *mut dsp_code_private,
}

unsafe extern "C" {
    /** Prepare *psDspCode to refer to the requested adapter's firmware.
    Code file name is obtained from HpiOs_GetDspCodePath

    \return 0 for success, or error code if requested code is not available
    */
    pub fn hpi_dsp_code_open(
        /** Code identifier, usually adapter family */
        adapter: u32,
        pci_dev: *mut core::ffi::c_void,
        /** Pointer to DSP code control structure */
        ps_dsp_code: *mut dsp_code,
        /** Pointer to dword to receive OS specific error code */
        pos_error_code: *mut u32,
    ) -> i16;

    /** Close the DSP code file */
    pub fn hpi_dsp_code_close(ps_dsp_code: *mut dsp_code);

    /** Rewind to the beginning of the DSP code file (for verify) */
    pub fn hpi_dsp_code_rewind(ps_dsp_code: *mut dsp_code);

    /** Read one word from the dsp code file
        \return 0 for success, or error code if eof, or block length exceeded
    */
    pub fn hpi_dsp_code_read_word(
        ps_dsp_code: *mut dsp_code,
        /**< DSP code descriptor */
        pword: *mut u32, /**< Where to store the read word */
    ) -> i16;

    /** Get a block of dsp code into an internal buffer, and provide a pointer to
    that buffer. (If dsp code is already an array in memory, it is referenced,
    not copied.)

    \return Error if requested number of words are not available
    */
    pub fn hpi_dsp_code_read_block(
        words_requested: usize,
        ps_dsp_code: *mut dsp_code,
        /* Pointer to store (Pointer to code buffer) */
        ppblock: *mut *mut u32,
    ) -> i16;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
