// SPDX-License-Identifier: GPL-2.0
/*
 * transport_class.c - implementation of generic transport classes
 *                     using attribute_containers
 *
 * Copyright (c) 2005 - James Bottomley <James.Bottomley@steeleye.com>
 *
 * The basic idea here is to allow any "device controller" (which
 * would most often be a Host Bus Adapter to use the services of one
 * or more tranport classes for performing transport specific
 * services.  Transport specific services are things that the generic
 * command layer doesn't want to know about (speed settings, line
 * condidtioning, etc), but which the user might be interested in.
 * Thus, the HBA's use the routines exported by the transport classes
 * to perform these functions.  The transport classes export certain
 * values to the user via sysfs using attribute containers.
 *
 * Note: because not every HBA will care about every transport
 * attribute, there's a many to one relationship that goes like this:
 *
 * transport class<-----attribute container<----class device
 *
 * Usually the attribute container is per-HBA, but the design doesn't
 * mandate that.  Although most of the services will be specific to
 * the actual external storage connection used by the HBA, the generic
 * transport class is framed entirely in terms of generic devices to
 * allow it to be used by any physical HBA in the system.
 */

// External kernel types and functions are supplied by the corresponding Rust dependencies.

pub unsafe fn transport_class_register(tclass: *mut transport_class) -> i32 {
    class_register(&mut (*tclass).class)
}

pub unsafe fn transport_class_unregister(tclass: *mut transport_class) {
    class_unregister(&mut (*tclass).class);
}

unsafe fn anon_transport_dummy_function(
    _tc: *mut transport_container,
    _dev: *mut device,
    _cdev: *mut device,
) -> i32 {
    // do nothing
    0
}

pub unsafe fn anon_transport_class_register(atc: *mut anon_transport_class) {
    (*atc).container.class = &mut (*atc).tclass.class;
    attribute_container_set_no_classdevs(&mut (*atc).container);
    attribute_container_register(&mut (*atc).container);
    (*atc).tclass.setup = Some(anon_transport_dummy_function);
    (*atc).tclass.remove = Some(anon_transport_dummy_function);
}

pub unsafe fn anon_transport_class_unregister(atc: *mut anon_transport_class) {
    if unlikely(attribute_container_unregister(&mut (*atc).container) != 0) {
        BUG();
    }
}

unsafe fn transport_setup_classdev(
    cont: *mut attribute_container,
    dev: *mut device,
    classdev: *mut device,
) -> i32 {
    let tclass = class_to_transport_class((*cont).class);
    let tcont = attribute_container_to_transport_container(cont);

    if let Some(setup) = (*tclass).setup {
        setup(tcont, dev, classdev);
    }

    0
}

pub unsafe fn transport_setup_device(dev: *mut device) {
    attribute_container_add_device(dev, transport_setup_classdev);
}

unsafe fn transport_add_class_device(
    cont: *mut attribute_container,
    dev: *mut device,
    classdev: *mut device,
) -> i32 {
    let tclass = class_to_transport_class((*cont).class);
    let mut error = attribute_container_add_class_device(classdev);
    let tcont = attribute_container_to_transport_container(cont);

    if error != 0 {
        if let Some(remove) = (*tclass).remove {
            remove(tcont, dev, classdev);
        }
        return error;
    }

    if !(*tcont).statistics.is_null() {
        error = sysfs_create_group(&mut (*classdev).kobj, (*tcont).statistics);
        if error != 0 {
            attribute_container_class_device_del(classdev);
            if let Some(remove) = (*tclass).remove {
                remove(tcont, dev, classdev);
            }
            return error;
        }
    }

    if !(*tcont).encryption.is_null() {
        error = sysfs_create_group(&mut (*classdev).kobj, (*tcont).encryption);
        if error != 0 {
            if !(*tcont).statistics.is_null() {
                sysfs_remove_group(&mut (*classdev).kobj, (*tcont).statistics);
            }
            attribute_container_class_device_del(classdev);
            if let Some(remove) = (*tclass).remove {
                remove(tcont, dev, classdev);
            }
            return error;
        }
    }

    0
}

pub unsafe fn transport_add_device(dev: *mut device) -> i32 {
    attribute_container_device_trigger_safe(dev, transport_add_class_device, transport_remove_classdev)
}

unsafe fn transport_configure(
    cont: *mut attribute_container,
    dev: *mut device,
    cdev: *mut device,
) -> i32 {
    let tclass = class_to_transport_class((*cont).class);
    let tcont = attribute_container_to_transport_container(cont);

    if let Some(configure) = (*tclass).configure {
        configure(tcont, dev, cdev);
    }

    0
}

pub unsafe fn transport_configure_device(dev: *mut device) {
    attribute_container_device_trigger(dev, transport_configure);
}

unsafe fn transport_remove_classdev(
    cont: *mut attribute_container,
    dev: *mut device,
    classdev: *mut device,
) -> i32 {
    let tcont = attribute_container_to_transport_container(cont);
    let tclass = class_to_transport_class((*cont).class);

    if let Some(remove) = (*tclass).remove {
        remove(tcont, dev, classdev);
    }

    if (*tclass).remove != Some(anon_transport_dummy_function) {
        if !(*tcont).statistics.is_null() {
            sysfs_remove_group(&mut (*classdev).kobj, (*tcont).statistics);
        }
        if !(*tcont).encryption.is_null() {
            sysfs_remove_group(&mut (*classdev).kobj, (*tcont).encryption);
        }
        attribute_container_class_device_del(classdev);
    }

    0
}

pub unsafe fn transport_remove_device(dev: *mut device) {
    attribute_container_device_trigger(dev, transport_remove_classdev);
}

unsafe fn transport_destroy_classdev(
    cont: *mut attribute_container,
    _dev: *mut device,
    classdev: *mut device,
) {
    let tclass = class_to_transport_class((*cont).class);

    if (*tclass).remove != Some(anon_transport_dummy_function) {
        put_device(classdev);
    }
}

pub unsafe fn transport_destroy_device(dev: *mut device) {
    attribute_container_remove_device(dev, transport_destroy_classdev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
