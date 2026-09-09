/*
 *  DIO Driver Services
 *
 *  Copyright (C) 2004 Jochen Friedrich
 *
 *  Loosely based on drivers/pci/pci-driver.c and drivers/zorro/zorro-driver.c
 *
 *  This file is subject to the terms and conditions of the GNU General Public
 *  License.  See the file COPYING in the main directory of this archive
 *  for more details.
 */

// Dependencies supplied by the kernel headers: linux/init.h, linux/module.h,
// and linux/dio.h.

/**
 *  dio_match_device - Tell if a DIO device structure has a matching DIO device id structure
 *  @ids: array of DIO device id structures to search in
 *  @d: the DIO device structure to match against
 *
 *  Used by a driver to check whether a DIO device present in the
 *  system is in its list of supported devices. Returns the matching
 *  dio_device_id structure or %NULL if there is no match.
 */
unsafe fn dio_match_device(
    mut ids: *const dio_device_id,
    d: *const dio_dev,
) -> *const dio_device_id {
    while (*ids).id != 0 {
        if (*ids).id == DIO_WILDCARD {
            return ids;
        }
        if DIO_NEEDSSECID((*ids).id & 0xff) {
            if (*ids).id == (*d).id {
                return ids;
            }
        } else if ((*ids).id & 0xff) == ((*d).id & 0xff) {
            return ids;
        }
        ids = ids.add(1);
    }
    core::ptr::null()
}

unsafe fn dio_device_probe(dev: *mut device) -> i32 {
    let mut error: i32 = 0;
    let drv: *mut dio_driver = to_dio_driver((*dev).driver);
    let d: *mut dio_dev = to_dio_dev(dev);

    if (*d).driver.is_null() && !(*drv).probe.is_none() {
        let id: *const dio_device_id;

        id = dio_match_device((*drv).id_table, d);
        if !id.is_null() {
            error = ((*drv).probe.unwrap())(d, id);
        }
        if error >= 0 {
            (*d).driver = drv;
            error = 0;
        }
    }
    error
}

/**
 *  dio_register_driver - register a new DIO driver
 *  @drv: the driver structure to register
 *
 *  Adds the driver structure to the list of registered drivers
 *  Returns zero or a negative error value.
 */
unsafe fn dio_register_driver(drv: *mut dio_driver) -> i32 {
    /* initialize common driver fields */
    (*drv).driver.name = (*drv).name;
    (*drv).driver.bus = &dio_bus_type;

    /* register with core */
    driver_register(&mut (*drv).driver)
}

/**
 *  dio_unregister_driver - unregister a DIO driver
 *  @drv: the driver structure to unregister
 *
 *  Deletes the driver structure from the list of registered DIO drivers,
 *  gives it a chance to clean up by calling its remove() function for
 *  each device it was responsible for, and marks those devices as
 *  driverless.
 */

unsafe fn dio_unregister_driver(drv: *mut dio_driver) {
    driver_unregister(&mut (*drv).driver);
}

/**
 *  dio_bus_match - Tell if a DIO device structure has a matching DIO device id structure
 *  @dev: the DIO device structure to match against
 *  @drv: the &device_driver that points to the array of DIO device id structures to search
 *
 *  Used by the driver core to check whether a DIO device present in the
 *  system is in a driver's list of supported devices. Returns 1 if supported,
 *  and 0 if there is no match.
 */

unsafe fn dio_bus_match(dev: *mut device, drv: *const device_driver) -> i32 {
    let d: *mut dio_dev = to_dio_dev(dev);
    let dio_drv: *const dio_driver = to_dio_driver(drv);
    let ids: *const dio_device_id = (*dio_drv).id_table;

    if ids.is_null() {
        return 0;
    }

    if !dio_match_device(ids, d).is_null() { 1 } else { 0 }
}

const dio_bus_type: bus_type = bus_type {
    name: c"dio".as_ptr(),
    match_: Some(dio_bus_match),
    probe: Some(dio_device_probe),
};

unsafe fn dio_driver_init() -> i32 {
    bus_register(&dio_bus_type)
}

// Equivalent to postcore_initcall(dio_driver_init).
postcore_initcall!(dio_driver_init);

// Equivalent to EXPORT_SYMBOL(dio_register_driver), EXPORT_SYMBOL(dio_unregister_driver),
// and EXPORT_SYMBOL(dio_bus_type).
export_symbol!(dio_register_driver);
export_symbol!(dio_unregister_driver);
export_symbol!(dio_bus_type);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
