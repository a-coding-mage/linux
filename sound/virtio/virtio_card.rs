// SPDX-License-Identifier: GPL-2.0+
/*
 * virtio-snd: Virtio sound device
 * Copyright (C) 2021 OpenSynergy GmbH
 */

// Dependencies:
// - linux/module.h
// - linux/moduleparam.h
// - linux/virtio_config.h
// - sound/initval.h
// - uapi/linux/virtio_ids.h
// - virtio_card.h (local header)

use core::ffi::{c_int, c_uint, c_void};

// External types (from included headers and virtio_card.h)
extern "C" {
    pub type virtio_device;
    pub type virtqueue;
    pub type virtio_snd;
    pub type virtio_snd_queue;
    pub type virtio_snd_event;
    pub type scatterlist;
    pub type virtqueue_info;
    pub type device;

    // Constants from kernel headers
    pub static MSEC_PER_SEC: c_uint;
    pub static VIRTIO_SND_VQ_MAX: c_uint;
    pub static VIRTIO_SND_VQ_CONTROL: c_uint;
    pub static VIRTIO_SND_VQ_EVENT: c_uint;
    pub static VIRTIO_SND_VQ_TX: c_uint;
    pub static VIRTIO_SND_VQ_RX: c_uint;
    pub static VIRTIO_F_VERSION_1: c_uint;
    pub static VIRTIO_ID_SOUND: c_uint;
    pub static VIRTIO_DEV_ANY_ID: c_uint;
    pub static VIRTIO_SND_F_CTLS: c_uint;
    pub static SNDRV_DEFAULT_IDX1: c_int;
    pub static SNDRV_DEFAULT_STR1: *const u8;
    pub static VIRTIO_SND_CARD_DRIVER: *const u8;
    pub static VIRTIO_SND_CARD_NAME: *const u8;
    pub static VIRTIO_SND_EVT_JACK_CONNECTED: c_uint;
    pub static VIRTIO_SND_EVT_JACK_DISCONNECTED: c_uint;
    pub static VIRTIO_SND_EVT_PCM_PERIOD_ELAPSED: c_uint;
    pub static VIRTIO_SND_EVT_PCM_XRUN: c_uint;
    pub static VIRTIO_SND_EVT_CTL_NOTIFY: c_uint;
    pub static GFP_KERNEL: c_int;
    pub static GFP_ATOMIC: c_int;

    // Memory allocation and utility functions
    pub fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    pub fn kmalloc_objs(objsize: usize, count: c_uint) -> *mut c_void;
    pub fn kfree(objp: *const c_void);
    pub fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_int) -> *mut c_void;

    // Scatterlist functions
    pub fn sg_init_one(sg: *mut scatterlist, buf: *const c_void, buflen: usize);

    // Virtqueue functions
    pub fn virtqueue_add_sgs(
        vq: *mut virtqueue,
        sgs: *const *mut scatterlist,
        out_sgs: c_uint,
        in_sgs: c_uint,
        data: *mut c_void,
        gfp: c_int,
    ) -> c_int;
    pub fn virtqueue_kick_prepare(vq: *mut virtqueue) -> c_int;
    pub fn virtqueue_notify(vq: *mut virtqueue);
    pub fn virtqueue_disable_cb(vq: *mut virtqueue);
    pub fn virtqueue_get_buf(vq: *mut virtqueue, len: *mut c_uint) -> *mut c_void;
    pub fn virtqueue_enable_cb(vq: *mut virtqueue) -> c_int;
    pub fn virtqueue_get_vring_size(vq: *mut virtqueue) -> c_uint;
    pub fn virtio_find_vqs(
        vdev: *mut virtio_device,
        nvqs: c_uint,
        vqs: *mut *mut virtqueue,
        vqs_info: *const virtqueue_info,
        ctx: *const c_void,
    ) -> c_int;

    // VirtIO device functions
    pub fn virtio_has_feature(vdev: *mut virtio_device, f: c_uint) -> bool;
    pub fn virtio_device_ready(vdev: *mut virtio_device);
    pub fn virtio_reset_device(vdev: *mut virtio_device);

    // Byte order conversion
    pub fn le32_to_cpu(val: u32) -> u32;

    // Sound card functions
    pub fn snd_card_new(
        parent: *mut device,
        idx: c_int,
        id: *const u8,
        module: *mut c_void,
        extra_size: c_int,
        card_ret: *mut *mut c_void,
    ) -> c_int;
    pub fn snd_card_register(card: *mut c_void) -> c_int;
    pub fn snd_card_free(card: *mut c_void);

    // String functions
    pub fn strscpy(dest: *mut u8, src: *const u8, count: usize) -> isize;
    pub fn snprintf(buf: *mut u8, size: usize, fmt: *const u8, ...) -> c_int;
    pub fn dev_name(dev: *mut device) -> *const u8;

    // Logging functions
    pub fn dev_err(dev: *mut device, fmt: *const u8, ...);

    // Spinlock functions
    pub fn spin_lock_init(lock: *mut c_void);

    // Work queue functions
    pub fn cancel_work_sync(work: *mut c_void) -> bool;

    // Local module functions
    pub fn virtsnd_ctl_notify_cb(vqueue: *mut virtqueue);
    pub fn virtsnd_pcm_tx_notify_cb(vqueue: *mut virtqueue);
    pub fn virtsnd_pcm_rx_notify_cb(vqueue: *mut virtqueue);
    pub fn virtsnd_event_queue(snd: *mut virtio_snd) -> *mut virtio_snd_queue;
    pub fn virtsnd_jack_event(snd: *mut virtio_snd, event: *mut virtio_snd_event);
    pub fn virtsnd_pcm_event(snd: *mut virtio_snd, event: *mut virtio_snd_event);
    pub fn virtsnd_kctl_event(snd: *mut virtio_snd, event: *mut virtio_snd_event);
    pub fn virtsnd_jack_parse_cfg(snd: *mut virtio_snd) -> c_int;
    pub fn virtsnd_pcm_parse_cfg(snd: *mut virtio_snd) -> c_int;
    pub fn virtsnd_chmap_parse_cfg(snd: *mut virtio_snd) -> c_int;
    pub fn virtsnd_kctl_parse_cfg(snd: *mut virtio_snd) -> c_int;
    pub fn virtsnd_jack_build_devs(snd: *mut virtio_snd) -> c_int;
    pub fn virtsnd_pcm_build_devs(snd: *mut virtio_snd) -> c_int;
    pub fn virtsnd_chmap_build_devs(snd: *mut virtio_snd) -> c_int;
    pub fn virtsnd_kctl_build_devs(snd: *mut virtio_snd) -> c_int;
    pub fn virtsnd_ctl_msg_cancel_all(snd: *mut virtio_snd);
    pub fn virtsnd_pcm_msg_free(vss: *mut c_void);
    pub fn virtsnd_pcm_validate(vdev: *mut virtio_device) -> c_int;

    pub static THIS_MODULE: *mut c_void;
    pub static KBUILD_MODNAME: *const u8;
}

pub static mut virtsnd_msg_timeout_ms: u32 = MSEC_PER_SEC as u32;

unsafe extern "C" fn virtsnd_event_send(
    vqueue: *mut virtqueue,
    event: *mut virtio_snd_event,
    notify: bool,
    gfp: c_int,
) {
    let mut sg: scatterlist = core::mem::zeroed();
    let mut psgs: [*mut scatterlist; 1] = [&mut sg];

    /* reset event content */
    memset(event as *mut c_void, 0, core::mem::size_of_val(&*event));

    sg_init_one(&mut sg, event as *const c_void, core::mem::size_of_val(&*event));

    if virtqueue_add_sgs(vqueue, psgs.as_mut_ptr(), 0, 1, event as *mut c_void, gfp) != 0
        || !notify
    {
        return;
    }

    if virtqueue_kick_prepare(vqueue) != 0 {
        virtqueue_notify(vqueue);
    }
}

unsafe extern "C" fn virtsnd_event_dispatch(
    snd: *mut virtio_snd,
    event: *mut virtio_snd_event,
) {
    let code = le32_to_cpu((*event).hdr.code);

    match code {
        VIRTIO_SND_EVT_JACK_CONNECTED | VIRTIO_SND_EVT_JACK_DISCONNECTED => {
            virtsnd_jack_event(snd, event);
        }
        VIRTIO_SND_EVT_PCM_PERIOD_ELAPSED | VIRTIO_SND_EVT_PCM_XRUN => {
            virtsnd_pcm_event(snd, event);
        }
        VIRTIO_SND_EVT_CTL_NOTIFY => {
            virtsnd_kctl_event(snd, event);
        }
        _ => {}
    }
}

unsafe extern "C" fn virtsnd_event_notify_cb(vqueue: *mut virtqueue) {
    let snd = (*(*vqueue).vdev).priv as *mut virtio_snd;
    let queue = virtsnd_event_queue(snd);
    let mut length: c_uint = 0;

    // guard(spinlock_irqsave)(&queue->lock);
    let _guard = SpinlockIrqsaveGuard::new(&(*queue).lock);

    loop {
        virtqueue_disable_cb(vqueue);
        loop {
            let event = virtqueue_get_buf(vqueue, &mut length) as *mut virtio_snd_event;
            if event.is_null() {
                break;
            }
            virtsnd_event_dispatch(snd, event);
            virtsnd_event_send(vqueue, event, true, GFP_ATOMIC);
        }
        if virtqueue_enable_cb(vqueue) == 0 {
            break;
        }
    }
}

struct SpinlockIrqsaveGuard {
    _marker: core::marker::PhantomData<*mut c_void>,
}

impl SpinlockIrqsaveGuard {
    unsafe fn new(_lock: *mut c_void) -> Self {
        // Placeholder for actual spinlock_irqsave guard
        SpinlockIrqsaveGuard {
            _marker: core::marker::PhantomData,
        }
    }
}

unsafe extern "C" fn virtsnd_find_vqs(snd: *mut virtio_snd) -> c_int {
    let vdev = (*snd).vdev;
    let vqs_info: [virtqueue_info; 4] = [
        virtqueue_info {
            name: "virtsnd-ctl\0" as *const u8 as *const c_void,
            callback: Some(virtsnd_ctl_notify_cb),
        },
        virtqueue_info {
            name: "virtsnd-event\0" as *const u8 as *const c_void,
            callback: Some(virtsnd_event_notify_cb),
        },
        virtqueue_info {
            name: "virtsnd-tx\0" as *const u8 as *const c_void,
            callback: Some(virtsnd_pcm_tx_notify_cb),
        },
        virtqueue_info {
            name: "virtsnd-rx\0" as *const u8 as *const c_void,
            callback: Some(virtsnd_pcm_rx_notify_cb),
        },
    ];
    let mut vqs: [*mut virtqueue; 4] = [core::ptr::null_mut(); 4];
    let mut i: c_uint = 0;
    let mut n: c_uint = 0;
    let mut rc: c_int = 0;

    rc = virtio_find_vqs(vdev, VIRTIO_SND_VQ_MAX as c_uint, vqs.as_mut_ptr(), vqs_info.as_ptr(), core::ptr::null());
    if rc != 0 {
        dev_err(
            &mut (*vdev).dev,
            "failed to initialize virtqueues\n\0" as *const u8,
        );
        return rc;
    }

    i = 0;
    while i < VIRTIO_SND_VQ_MAX as c_uint {
        (*snd).queues[i as usize].vqueue = vqs[i as usize];
        i += 1;
    }

    /* Allocate events and populate the event queue */
    virtqueue_disable_cb(vqs[VIRTIO_SND_VQ_EVENT as usize]);

    n = virtqueue_get_vring_size(vqs[VIRTIO_SND_VQ_EVENT as usize]);

    (*snd).event_msgs = kmalloc_objs(core::mem::size_of::<virtio_snd_event>(), n);
    if (*snd).event_msgs.is_null() {
        return -12; // -ENOMEM
    }

    i = 0;
    while i < n {
        virtsnd_event_send(
            vqs[VIRTIO_SND_VQ_EVENT as usize],
            ((*snd).event_msgs as *mut virtio_snd_event).add(i as usize),
            false,
            GFP_KERNEL,
        );
        i += 1;
    }

    0
}

unsafe extern "C" fn virtsnd_enable_event_vq(snd: *mut virtio_snd) {
    let queue = virtsnd_event_queue(snd);

    if virtqueue_enable_cb((*queue).vqueue) == 0 {
        virtsnd_event_notify_cb((*queue).vqueue);
    }
}

unsafe extern "C" fn virtsnd_disable_event_vq(snd: *mut virtio_snd) {
    let queue = virtsnd_event_queue(snd);
    let mut length: c_uint = 0;

    if !(*queue).vqueue.is_null() {
        // guard(spinlock_irqsave)(&queue->lock);
        let _guard = SpinlockIrqsaveGuard::new(&mut (*queue).lock);

        virtqueue_disable_cb((*queue).vqueue);
        loop {
            let event = virtqueue_get_buf((*queue).vqueue, &mut length) as *mut virtio_snd_event;
            if event.is_null() {
                break;
            }
            virtsnd_event_dispatch(snd, event);
        }
    }
}

unsafe extern "C" fn virtsnd_build_devs(snd: *mut virtio_snd) -> c_int {
    let vdev = (*snd).vdev;
    let dev = &mut (*vdev).dev;
    let mut rc: c_int = 0;

    rc = snd_card_new(
        dev,
        SNDRV_DEFAULT_IDX1,
        SNDRV_DEFAULT_STR1,
        THIS_MODULE,
        0,
        &mut (*snd).card as *mut *mut c_void,
    );
    if rc < 0 {
        return rc;
    }

    (*(*snd).card).private_data = snd as *mut c_void;

    strscpy(
        (*(*snd).card).driver.as_mut_ptr(),
        VIRTIO_SND_CARD_DRIVER,
        core::mem::size_of_val(&(*(*snd).card).driver),
    );
    strscpy(
        (*(*snd).card).shortname.as_mut_ptr(),
        VIRTIO_SND_CARD_NAME,
        core::mem::size_of_val(&(*(*snd).card).shortname),
    );
    if !(*(*dev).parent).bus.is_null() {
        snprintf(
            (*(*snd).card).longname.as_mut_ptr(),
            core::mem::size_of_val(&(*(*snd).card).longname),
            "Virtio sound device at %s/%s/%s\0" as *const u8,
            (*(*(*dev).parent).bus).name,
            dev_name((*dev).parent),
            dev_name(dev),
        );
    } else {
        snprintf(
            (*(*snd).card).longname.as_mut_ptr(),
            core::mem::size_of_val(&(*(*snd).card).longname),
            "Virtio sound device at %s/%s\0" as *const u8,
            dev_name((*dev).parent),
            dev_name(dev),
        );
    }

    rc = virtsnd_jack_parse_cfg(snd);
    if rc != 0 {
        return rc;
    }

    rc = virtsnd_pcm_parse_cfg(snd);
    if rc != 0 {
        return rc;
    }

    rc = virtsnd_chmap_parse_cfg(snd);
    if rc != 0 {
        return rc;
    }

    if virtio_has_feature(vdev, VIRTIO_SND_F_CTLS) {
        rc = virtsnd_kctl_parse_cfg(snd);
        if rc != 0 {
            return rc;
        }
    }

    if (*snd).njacks != 0 {
        rc = virtsnd_jack_build_devs(snd);
        if rc != 0 {
            return rc;
        }
    }

    if (*snd).nsubstreams != 0 {
        rc = virtsnd_pcm_build_devs(snd);
        if rc != 0 {
            return rc;
        }
    }

    if (*snd).nchmaps != 0 {
        rc = virtsnd_chmap_build_devs(snd);
        if rc != 0 {
            return rc;
        }
    }

    if (*snd).nkctls != 0 {
        rc = virtsnd_kctl_build_devs(snd);
        if rc != 0 {
            return rc;
        }
    }

    snd_card_register((*snd).card)
}

unsafe extern "C" fn virtsnd_validate(vdev: *mut virtio_device) -> c_int {
    if (*(*vdev).config).get.is_none() {
        dev_err(
            &mut (*vdev).dev,
            "configuration access disabled\n\0" as *const u8,
        );
        return -22; // -EINVAL
    }

    if !virtio_has_feature(vdev, VIRTIO_F_VERSION_1) {
        dev_err(
            &mut (*vdev).dev,
            "device does not comply with spec version 1.x\n\0" as *const u8,
        );
        return -22; // -EINVAL
    }

    if virtsnd_msg_timeout_ms == 0 {
        dev_err(
            &mut (*vdev).dev,
            "msg_timeout_ms value cannot be zero\n\0" as *const u8,
        );
        return -22; // -EINVAL
    }

    if virtsnd_pcm_validate(vdev) != 0 {
        return -22; // -EINVAL
    }

    0
}

unsafe extern "C" fn virtsnd_probe(vdev: *mut virtio_device) -> c_int {
    let mut snd: *mut virtio_snd;
    let mut i: c_uint = 0;
    let mut rc: c_int = 0;

    snd = devm_kzalloc(&mut (*vdev).dev, core::mem::size_of::<virtio_snd>(), GFP_KERNEL)
        as *mut virtio_snd;
    if snd.is_null() {
        return -12; // -ENOMEM
    }

    (*snd).vdev = vdev;
    // INIT_LIST_HEAD(&snd->ctl_msgs);
    // INIT_LIST_HEAD(&snd->pcm_list);

    (*vdev).priv = snd as *mut c_void;

    i = 0;
    while i < VIRTIO_SND_VQ_MAX as c_uint {
        spin_lock_init(&mut (*snd).queues[i as usize].lock as *mut c_void);
        i += 1;
    }

    rc = virtsnd_find_vqs(snd);
    if rc != 0 {
        virtsnd_remove(vdev);
        return rc;
    }

    virtio_device_ready(vdev);

    rc = virtsnd_build_devs(snd);
    if rc != 0 {
        virtsnd_remove(vdev);
        return rc;
    }

    virtsnd_enable_event_vq(snd);

    rc
}

unsafe extern "C" fn virtsnd_remove(vdev: *mut virtio_device) {
    let snd = (*vdev).priv as *mut virtio_snd;
    let mut i: c_uint = 0;

    virtsnd_disable_event_vq(snd);
    virtsnd_ctl_msg_cancel_all(snd);

    if !(*snd).card.is_null() {
        snd_card_free((*snd).card);
    }

    (*(*vdev).config).del_vqs.unwrap()(vdev);
    virtio_reset_device(vdev);

    if !(*snd).substreams.is_null() && i < (*snd).nsubstreams {
        while i < (*snd).nsubstreams {
            let vss = &mut *(*snd).substreams.add(i as usize);

            cancel_work_sync(&mut vss.elapsed_period as *mut c_void);
            virtsnd_pcm_msg_free(vss as *mut c_void);
            i += 1;
        }
    }

    kfree((*snd).event_msgs);
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
unsafe extern "C" fn virtsnd_freeze(vdev: *mut virtio_device) -> c_int {
    let snd = (*vdev).priv as *mut virtio_snd;
    let mut i: c_uint = 0;

    virtsnd_disable_event_vq(snd);
    virtsnd_ctl_msg_cancel_all(snd);

    (*(*vdev).config).del_vqs.unwrap()(vdev);
    virtio_reset_device(vdev);

    while i < (*snd).nsubstreams {
        cancel_work_sync(
            &mut (*snd).substreams.add(i as usize) as *mut c_void,
        );
        i += 1;
    }

    kfree((*snd).event_msgs);
    (*snd).event_msgs = core::ptr::null_mut();

    0
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
unsafe extern "C" fn virtsnd_restore(vdev: *mut virtio_device) -> c_int {
    let snd = (*vdev).priv as *mut virtio_snd;
    let mut rc: c_int = 0;

    rc = virtsnd_find_vqs(snd);
    if rc != 0 {
        return rc;
    }

    virtio_device_ready(vdev);

    virtsnd_enable_event_vq(snd);

    0
}

#[repr(C)]
pub struct VirtioDeviceId {
    pub device: u32,
    pub vendor: u32,
}

pub static ID_TABLE: [VirtioDeviceId; 2] = [
    VirtioDeviceId {
        device: VIRTIO_ID_SOUND as u32,
        vendor: VIRTIO_DEV_ANY_ID as u32,
    },
    VirtioDeviceId {
        device: 0,
        vendor: 0,
    },
];

pub static FEATURES: [c_uint; 1] = [VIRTIO_SND_F_CTLS];

#[repr(C)]
pub struct VirtioDriver {
    pub driver_name: *const u8,
    pub id_table: *const VirtioDeviceId,
    pub feature_table: *const c_uint,
    pub feature_table_size: c_uint,
    pub validate: Option<unsafe extern "C" fn(*mut virtio_device) -> c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut virtio_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut virtio_device)>,
    #[cfg(feature = "CONFIG_PM_SLEEP")]
    pub freeze: Option<unsafe extern "C" fn(*mut virtio_device) -> c_int>,
    #[cfg(feature = "CONFIG_PM_SLEEP")]
    pub restore: Option<unsafe extern "C" fn(*mut virtio_device) -> c_int>,
}

pub static VIRTSND_DRIVER: VirtioDriver = VirtioDriver {
    driver_name: KBUILD_MODNAME,
    id_table: ID_TABLE.as_ptr(),
    feature_table: FEATURES.as_ptr(),
    feature_table_size: 1,
    validate: Some(virtsnd_validate),
    probe: Some(virtsnd_probe),
    remove: Some(virtsnd_remove),
    #[cfg(feature = "CONFIG_PM_SLEEP")]
    freeze: Some(virtsnd_freeze),
    #[cfg(feature = "CONFIG_PM_SLEEP")]
    restore: Some(virtsnd_restore),
};

// module_virtio_driver(virtsnd_driver);
// MODULE_DEVICE_TABLE(virtio, id_table);
// MODULE_DESCRIPTION("Virtio sound card driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
