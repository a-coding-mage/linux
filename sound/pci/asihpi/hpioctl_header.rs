/* SPDX-License-Identifier: GPL-2.0-only */
/*******************************************************************************

    AudioScience HPI driver
    Copyright (C) 1997-2011  AudioScience Inc. <support@audioscience.com>


Linux HPI ioctl, and shared module init functions
*******************************************************************************/

// External dependency types from the original C includes/kernel headers.
// __init and __exit are Linux section annotations on the C declarations.

extern "C" {
    pub fn asihpi_adapter_probe(
        pci_dev: *mut pci_dev,
        pci_id: *const pci_device_id,
    ) -> ::std::os::raw::c_int;
    pub fn asihpi_adapter_remove(pci_dev: *mut pci_dev);
    pub fn asihpi_init();
    pub fn asihpi_exit();

    pub fn asihpi_hpi_release(file: *mut file) -> ::std::os::raw::c_int;

    pub fn asihpi_hpi_ioctl(
        file: *mut file,
        cmd: ::std::os::raw::c_uint,
        arg: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_long;

    /* This is called from hpifunc.c functions, called by ALSA
     * (or other kernel process) In this case there is no file descriptor
     * available for the message cache code
     */
    pub fn hpi_send_recv(phm: *mut hpi_message, phr: *mut hpi_response);
}

pub const HOWNER_KERNEL: *mut ::std::os::raw::c_void =
    (-1isize) as *mut ::std::os::raw::c_void;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
