// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/drivers/dma/dma-api.c
 *
 * SuperH-specific DMA management API
 *
 * Copyright (C) 2003, 2004, 2005  Paul Mundt
 */

// Linux kernel dependencies and build-time annotations are supplied externally.

static mut DMA_SPIN_LOCK: Spinlock = DEFINE_SPINLOCK!();
static mut REGISTERED_DMAC_LIST: ListHead = LIST_HEAD_INIT!();

pub unsafe fn get_dma_info(chan: u32) -> *mut DmaInfo {
    let mut info: *mut DmaInfo;

    /*
     * Look for each DMAC's range to determine who the owner of
     * the channel is.
     */
    list_for_each_entry!(info, &mut REGISTERED_DMAC_LIST, list, {
        if chan < (*info).first_vchannel_nr
            || chan >= (*info).first_vchannel_nr + (*info).nr_channels
        {
            continue;
        }

        return info;
    });

    core::ptr::null_mut()
}

// EXPORT_SYMBOL(get_dma_info);

unsafe fn get_nr_channels() -> u32 {
    let mut info: *mut DmaInfo;
    let mut nr: u32 = 0;

    if unlikely!(list_empty!(&REGISTERED_DMAC_LIST)) {
        return nr;
    }

    list_for_each_entry!(info, &REGISTERED_DMAC_LIST, list, {
        nr += (*info).nr_channels;
    });

    nr
}

pub unsafe fn get_dma_channel(chan: u32) -> *mut DmaChannel {
    let info: *mut DmaInfo = get_dma_info(chan);
    let mut channel: *mut DmaChannel;
    let mut i: i32;

    if unlikely!(info.is_null()) {
        return ERR_PTR!(-EINVAL);
    }

    i = 0;
    while i < (*info).nr_channels as i32 {
        channel = (*info).channels.add(i as usize);
        if (*channel).vchan == chan {
            return channel;
        }
        i += 1;
    }

    core::ptr::null_mut()
}

// EXPORT_SYMBOL(get_dma_channel);

pub unsafe fn get_dma_residue(chan: u32) -> i32 {
    let info: *mut DmaInfo = get_dma_info(chan);
    let channel: *mut DmaChannel = get_dma_channel(chan);

    if let Some(get_residue) = (*(*info).ops).get_residue {
        return get_residue(channel);
    }

    0
}

// EXPORT_SYMBOL(get_dma_residue);

pub unsafe fn request_dma(chan: u32, dev_id: *const i8) -> i32 {
    let mut channel: *mut DmaChannel = core::ptr::null_mut();
    let info: *mut DmaInfo = get_dma_info(chan);
    let result: i32;

    channel = get_dma_channel(chan);
    if atomic_xchg!(&mut (*channel).busy, 1) != 0 {
        return -EBUSY;
    }

    strscpy!((*channel).dev_id.as_mut_ptr(), dev_id, core::mem::size_of_val(&(*channel).dev_id));

    if let Some(request) = (*(*info).ops).request {
        result = request(channel);
        if result != 0 {
            atomic_set!(&mut (*channel).busy, 0);
        }

        return result;
    }

    0
}

// EXPORT_SYMBOL(request_dma);

pub unsafe fn free_dma(chan: u32) {
    let info: *mut DmaInfo = get_dma_info(chan);
    let channel: *mut DmaChannel = get_dma_channel(chan);

    if let Some(free) = (*(*info).ops).free {
        free(channel);
    }

    atomic_set!(&mut (*channel).busy, 0);
}

// EXPORT_SYMBOL(free_dma);

pub unsafe fn dma_wait_for_completion(chan: u32) {
    let info: *mut DmaInfo = get_dma_info(chan);
    let channel: *mut DmaChannel = get_dma_channel(chan);

    if (*channel).flags & DMA_TEI_CAPABLE != 0 {
        wait_event!((*channel).wait_queue, ((*(*info).ops).get_residue.unwrap()(channel) == 0));
        return;
    }

    while (*(*info).ops).get_residue.unwrap()(channel) != 0 {
        cpu_relax!();
    }
}

// EXPORT_SYMBOL(dma_wait_for_completion);

pub unsafe fn dma_configure_channel(chan: u32, flags: usize) {
    let info: *mut DmaInfo = get_dma_info(chan);
    let channel: *mut DmaChannel = get_dma_channel(chan);

    if let Some(configure) = (*(*info).ops).configure {
        configure(channel, flags);
    }
}

// EXPORT_SYMBOL(dma_configure_channel);

pub unsafe fn dma_xfer(chan: u32, from: usize, to: usize, size: usize, mode: u32) -> i32 {
    let info: *mut DmaInfo = get_dma_info(chan);
    let channel: *mut DmaChannel = get_dma_channel(chan);

    (*channel).sar = from;
    (*channel).dar = to;
    (*channel).count = size;
    (*channel).mode = mode;

    (*(*info).ops).xfer.unwrap()(channel)
}

// EXPORT_SYMBOL(dma_xfer);

unsafe fn dma_proc_show(m: *mut SeqFile, v: *mut core::ffi::c_void) -> i32 {
    let mut info: *mut DmaInfo = v.cast();

    if list_empty!(&REGISTERED_DMAC_LIST) {
        return 0;
    }

    /* Iterate over each registered DMAC */
    list_for_each_entry!(info, &REGISTERED_DMAC_LIST, list, {
        /* Iterate over each channel */
        let mut i: i32 = 0;
        while i < (*info).nr_channels as i32 {
            let channel: *mut DmaChannel = (*info).channels.add(i as usize);

            if (*channel).flags & DMA_CONFIGURED == 0 {
                i += 1;
                continue;
            }

            seq_printf!(m, "%2d: %14s    %s\n", i, (*info).name, (*channel).dev_id);
            i += 1;
        }
    });

    0
}

pub unsafe fn register_dmac(info: *mut DmaInfo) -> i32 {
    let mut total_channels: u32;
    let mut i: u32;

    INIT_LIST_HEAD!(&mut (*info).list);

    printk!(KERN_INFO, "DMA: Registering %s handler (%d channel%s).\n",
        (*info).name, (*info).nr_channels, if (*info).nr_channels > 1 { "s" } else { "" });

    BUG_ON!(((*info).flags & DMAC_CHANNELS_CONFIGURED != 0) && (*info).channels.is_null());

    (*info).pdev = platform_device_register_simple!((*info).name, -1, core::ptr::null_mut(), 0);
    if IS_ERR!((*info).pdev) {
        return PTR_ERR!((*info).pdev);
    }

    /* Don't touch pre-configured channels */
    if (*info).flags & DMAC_CHANNELS_CONFIGURED == 0 {
        let size = core::mem::size_of::<DmaChannel>() * (*info).nr_channels as usize;
        (*info).channels = kzalloc!(size, GFP_KERNEL).cast();
        if (*info).channels.is_null() {
            return -ENOMEM;
        }
    }

    total_channels = get_nr_channels();
    (*info).first_vchannel_nr = total_channels;
    i = 0;
    while i < (*info).nr_channels {
        let chan = (*info).channels.add(i as usize);

        atomic_set!(&mut (*chan).busy, 0);
        (*chan).chan = (*info).first_channel_nr + i;
        (*chan).vchan = (*info).first_channel_nr + i + total_channels;
        memcpy!((*chan).dev_id.as_mut_ptr(), b"Unused\0".as_ptr(), 7);

        if (*info).flags & DMAC_CHANNELS_TEI_CAPABLE != 0 {
            (*chan).flags |= DMA_TEI_CAPABLE;
        }

        init_waitqueue_head!(&mut (*chan).wait_queue);
        dma_create_sysfs_files!(chan, info);
        i += 1;
    }

    list_add!(&mut (*info).list, &mut REGISTERED_DMAC_LIST);
    0
}

// EXPORT_SYMBOL(register_dmac);

pub unsafe fn unregister_dmac(info: *mut DmaInfo) {
    let mut i: u32 = 0;
    while i < (*info).nr_channels {
        dma_remove_sysfs_files!((*info).channels.add(i as usize), info);
        i += 1;
    }

    if (*info).flags & DMAC_CHANNELS_CONFIGURED == 0 {
        kfree!((*info).channels.cast());
    }

    list_del!(&mut (*info).list);
    platform_device_unregister!((*info).pdev);
}

// EXPORT_SYMBOL(unregister_dmac);

#[init]
unsafe fn dma_api_init() -> i32 {
    printk!(KERN_NOTICE, "DMA: Registering DMA API.\n");
    if proc_create_single!("dma", 0, core::ptr::null_mut(), dma_proc_show).is_null() {
        -ENOMEM
    } else {
        0
    }
}

// MODULE_AUTHOR("Paul Mundt <lethal@linux-sh.org>");
// MODULE_DESCRIPTION("DMA API for SuperH");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
