// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2002-3 Patrick Mochel
 * Copyright (c) 2002-3 Open Source Development Labs
 */

// Declarations supplied by the Linux device-model dependencies and base.h.
#[repr(C)]
pub struct backing_dev_info {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut noop_backing_dev_info: backing_dev_info;

    fn bdi_init(bdi: *mut backing_dev_info);
    fn devtmpfs_init();
    fn devices_init();
    fn buses_init();
    fn classes_init();
    fn firmware_init();
    fn hypervisor_init();
    fn faux_bus_init();
    fn of_core_init();
    fn software_node_init();
    fn platform_bus_init();
    fn auxiliary_bus_init();
    fn memory_dev_init();
    fn node_dev_init();
    fn cpu_dev_init();
    fn container_dev_init();
}

/**
 * driver_init - initialize driver model.
 *
 * Call the driver model init functions to initialize their
 * subsystems. Called early from init/main.c.
 */
pub unsafe fn driver_init() {
    /* These are the core pieces */
    bdi_init(&raw mut noop_backing_dev_info);
    devtmpfs_init();
    devices_init();
    buses_init();
    classes_init();
    firmware_init();
    hypervisor_init();

    /* These are also core pieces, but must come after the
     * core core pieces.
     */
    faux_bus_init();
    of_core_init();
    software_node_init();
    platform_bus_init();
    auxiliary_bus_init();
    memory_dev_init();
    node_dev_init();
    cpu_dev_init();
    container_dev_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
