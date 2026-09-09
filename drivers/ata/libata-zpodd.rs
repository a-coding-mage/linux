// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding libata, ACPI, PM, and SCSI code.

static mut ZPODD_POWEROFF_DELAY: i32 = 30; /* 30 seconds for power off delay */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum OddMechType {
    ODD_MECH_TYPE_SLOT,
    ODD_MECH_TYPE_DRAWER,
    ODD_MECH_TYPE_UNSUPPORTED,
}

#[repr(C)]
struct Zpodd {
    mech_type: OddMechType, /* init during probe, RO afterwards */
    dev: *mut ata_device,

    /* The following fields are synchronized by PM core. */
    from_notify: bool, /* resumed as a result of acpi wake notification */
    zp_ready: bool, /* ZP ready state */
    last_ready: usize, /* last ZP ready timestamp */
    zp_sampled: bool, /* ZP ready state sampled */
    powered_off: bool, /* ODD is powered off during suspend */
}

unsafe fn eject_tray(dev: *mut ata_device) -> i32 {
    let mut tf: ata_taskfile = core::mem::zeroed();
    let cdb: [i8; ATAPI_CDB_LEN as usize] = [
        GPCMD_START_STOP_UNIT as i8, 0, 0, 0, 0x02, 0, 0, 0, 0, 0, 0, 0,
    ];

    ata_tf_init(dev, &mut tf);
    tf.flags = ATA_TFLAG_ISADDR | ATA_TFLAG_DEVICE;
    tf.command = ATA_CMD_PACKET;
    tf.protocol = ATAPI_PROT_NODATA;

    ata_exec_internal(dev, &mut tf, cdb.as_ptr(), DMA_NONE, core::ptr::null_mut(), 0, 0)
}

/* Per the spec, only slot type and drawer type ODD can be supported */
unsafe fn zpodd_get_mech_type(dev: *mut ata_device) -> OddMechType {
    let buf = kzalloc(16, GFP_KERNEL);
    if buf.is_null() {
        return OddMechType::ODD_MECH_TYPE_UNSUPPORTED;
    }
    let desc = (buf.add(8)) as *mut rm_feature_desc;
    let mut tf: ata_taskfile = core::mem::zeroed();
    let cdb: [i8; ATAPI_CDB_LEN as usize] = [
        GPCMD_GET_CONFIGURATION as i8, 2, 0, 3, 0, 0, 0, 0, 16, 0, 0, 0,
    ];

    ata_tf_init(dev, &mut tf);
    tf.flags = ATA_TFLAG_ISADDR | ATA_TFLAG_DEVICE;
    tf.command = ATA_CMD_PACKET;
    tf.protocol = ATAPI_PROT_PIO;
    tf.lbam = 16;

    let ret = ata_exec_internal(dev, &mut tf, cdb.as_ptr(), DMA_FROM_DEVICE, buf, 16, 0);
    if ret != 0 {
        kfree(buf);
        return OddMechType::ODD_MECH_TYPE_UNSUPPORTED;
    }
    if be16_to_cpu((*desc).feature_code) != 3 {
        kfree(buf);
        return OddMechType::ODD_MECH_TYPE_UNSUPPORTED;
    }

    let result = if (*desc).mech_type == 0 && (*desc).load == 0 && (*desc).eject == 1 {
        OddMechType::ODD_MECH_TYPE_SLOT
    } else if (*desc).mech_type == 1 && (*desc).load == 0 && (*desc).eject == 1 {
        OddMechType::ODD_MECH_TYPE_DRAWER
    } else {
        OddMechType::ODD_MECH_TYPE_UNSUPPORTED
    };
    kfree(buf);
    result
}

/* Test if ODD is zero power ready by sense code */
unsafe fn zpready(dev: *mut ata_device) -> bool {
    let mut sense_key: u8 = 0;
    let zpodd = (*dev).zpodd;
    let ret = atapi_eh_tur(dev, &mut sense_key);
    if ret != 0 || sense_key != NOT_READY {
        return false;
    }
    let sense_buf = (*dev).sector_buf;
    if atapi_eh_request_sense(dev, sense_buf, sense_key) != 0 {
        return false;
    }
    if ((*sense_buf.add(0) & 0x7f) != 0x70) || *sense_buf.add(7) < 6 {
        return false;
    }
    let asc = *sense_buf.add(12);
    let ascq = *sense_buf.add(13);
    if (*zpodd).mech_type == OddMechType::ODD_MECH_TYPE_SLOT {
        asc == 0x3a
    } else {
        asc == 0x3a && ascq == 0x01
    }
}

/*
 * Update the zpodd->zp_ready field. This field will only be set
 * if the ODD has stayed in ZP ready state for zpodd_poweroff_delay
 * time, and will be used to decide if power off is allowed. If it
 * is set, it will be cleared during resume from powered off state.
 */
pub unsafe fn zpodd_on_suspend(dev: *mut ata_device) {
    let zpodd = (*dev).zpodd;
    if !zpready(dev) {
        (*zpodd).zp_sampled = false;
        (*zpodd).zp_ready = false;
        return;
    }
    if !(*zpodd).zp_sampled {
        (*zpodd).zp_sampled = true;
        (*zpodd).last_ready = jiffies;
        return;
    }
    let expires = (*zpodd).last_ready.wrapping_add(secs_to_jiffies(ZPODD_POWEROFF_DELAY));
    if time_before(jiffies, expires) {
        return;
    }
    (*zpodd).zp_ready = true;
}

pub unsafe fn zpodd_zpready(dev: *mut ata_device) -> bool {
    (*(*dev).zpodd).zp_ready
}

/* Enable runtime wake capability through ACPI and set the powered_off flag. */
pub unsafe fn zpodd_enable_run_wake(dev: *mut ata_device) {
    let zpodd = (*dev).zpodd;
    sdev_disable_disk_events((*dev).sdev);
    (*zpodd).powered_off = true;
    acpi_pm_set_device_wakeup(&mut (*dev).tdev, true);
}

/* Disable runtime wake capability if it is enabled */
pub unsafe fn zpodd_disable_run_wake(dev: *mut ata_device) {
    let zpodd = (*dev).zpodd;
    if (*zpodd).powered_off {
        acpi_pm_set_device_wakeup(&mut (*dev).tdev, false);
    }
}

/* Post power on processing after the ODD has been recovered. */
pub unsafe fn zpodd_post_poweron(dev: *mut ata_device) {
    let zpodd = (*dev).zpodd;
    if !(*zpodd).powered_off { return; }
    (*zpodd).powered_off = false;
    if (*zpodd).from_notify {
        (*zpodd).from_notify = false;
        if (*zpodd).mech_type == OddMechType::ODD_MECH_TYPE_DRAWER { eject_tray(dev); }
    }
    (*zpodd).zp_sampled = false;
    (*zpodd).zp_ready = false;
    sdev_enable_disk_events((*dev).sdev);
}

unsafe extern "C" fn zpodd_wake_dev(_handle: acpi_handle, event: u32, context: *mut core::ffi::c_void) {
    let ata_dev = context as *mut ata_device;
    let zpodd = (*ata_dev).zpodd;
    let dev = &mut (*(*ata_dev).sdev).sdev_gendev;
    if event == ACPI_NOTIFY_DEVICE_WAKE && pm_runtime_suspended(dev) {
        (*zpodd).from_notify = true;
        pm_runtime_resume(dev);
    }
}

unsafe fn ata_acpi_add_pm_notifier(dev: *mut ata_device) {
    let handle = ata_dev_acpi_handle(dev);
    acpi_install_notify_handler(handle, ACPI_SYSTEM_NOTIFY, zpodd_wake_dev, dev as *mut _);
}

unsafe fn ata_acpi_remove_pm_notifier(dev: *mut ata_device) {
    let handle = ata_dev_acpi_handle(dev);
    acpi_remove_notify_handler(handle, ACPI_SYSTEM_NOTIFY, zpodd_wake_dev);
}

pub unsafe fn zpodd_init(dev: *mut ata_device) {
    let adev = ACPI_COMPANION(&mut (*dev).tdev);
    if !(*dev).zpodd.is_null() || adev.is_null() || !acpi_device_can_poweroff(adev) { return; }
    let mech_type = zpodd_get_mech_type(dev);
    if mech_type == OddMechType::ODD_MECH_TYPE_UNSUPPORTED { return; }
    let zpodd = kzalloc_obj::<Zpodd>();
    if zpodd.is_null() { return; }
    (*zpodd).mech_type = mech_type;
    ata_acpi_add_pm_notifier(dev);
    (*zpodd).dev = dev;
    (*dev).zpodd = zpodd;
    dev_pm_qos_expose_flags(&mut (*dev).tdev, 0);
}

pub unsafe fn zpodd_exit(dev: *mut ata_device) {
    ata_acpi_remove_pm_notifier(dev);
    kfree((*dev).zpodd as *mut _);
    (*dev).zpodd = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
