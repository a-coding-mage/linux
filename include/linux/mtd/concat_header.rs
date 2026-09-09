/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * MTD device concatenation layer definitions
 *
 * Copyright © 2002      Robert Kaiser <rkaiser@sysgo.de>
 */

// `struct mtd_info` is supplied by the corresponding MTD dependency.
#[repr(C)]
pub struct mtd_info;

/*
 * Our storage structure:
 * Subdev points to an array of pointers to struct mtd_info objects
 * which is allocated along with this structure
 */
#[repr(C)]
pub struct mtd_concat {
    pub mtd: mtd_info,
    pub num_subdev: ::core::ffi::c_int,
    pub subdev: [*mut mtd_info; 0],
}

unsafe extern "C" {
    pub fn mtd_concat_create(
        subdev: *mut *mut mtd_info, /* subdevices to concatenate */
        num_devs: ::core::ffi::c_int, /* number of subdevices */
        name: *const ::core::ffi::c_char, /* name for the new device */
    ) -> *mut mtd_info;

    pub fn mtd_concat_destroy(mtd: *mut mtd_info);

    /**
     * mtd_virt_concat_node_create - Create a component for concatenation
     *
     * Returns a positive number representing the no. of devices found for
     * concatenation, or a negative error code.
     *
     * List all the devices for concatenations found in DT and create a
     * component for concatenation.
     */
    pub fn mtd_virt_concat_node_create() -> ::core::ffi::c_int;

    /**
     * mtd_virt_concat_add - add mtd_info object to the list of subdevices for concatenation
     * @mtd: pointer to new MTD device info structure
     *
     * Returns true if the mtd_info object is added successfully else returns false.
     *
     * The mtd_info object is added to the list of subdevices for concatenation.
     * It returns true if a match is found, and false if all subdevices have
     * already been added or if the mtd_info object does not match any of the
     * intended MTD devices.
     */
    pub fn mtd_virt_concat_add(mtd: *mut mtd_info) -> bool;

    /**
     * mtd_virt_concat_create_join - Create and register the concatenated MTD device
     *
     * Returns 0 on succes, or a negative error code.
     *
     * Creates and registers the concatenated MTD device
     */
    pub fn mtd_virt_concat_create_join() -> ::core::ffi::c_int;

    /**
     * mtd_virt_concat_destroy - Remove the concat that includes a specific mtd device
     *                           as one of its components.
     * @mtd: pointer to MTD device info structure.
     *
     * Returns 0 on succes, or a negative error code.
     *
     * If the mtd_info object is part of a concatenated device, all other MTD devices
     * within that concat are registered individually. The concatenated device is then
     * removed, along with its concatenation component.
     */
    pub fn mtd_virt_concat_destroy(mtd: *mut mtd_info) -> ::core::ffi::c_int;

    pub fn mtd_virt_concat_destroy_joins();
    pub fn mtd_virt_concat_destroy_items();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
