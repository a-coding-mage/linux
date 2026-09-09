// SPDX-License-Identifier: GPL-2.0
/*
 * linux/kernel/dma.c: A DMA channel allocator. Inspired by linux/kernel/irq.c.
 *
 * Written by Hennus Bergman, 1992.
 *
 * 1994/12/26: Changes by Alex Nash to fix a minor bug in /proc/dma.
 *   In the previous version the reported device could end up being wrong,
 *   if a device requested a DMA channel that was already in use.
 *   [It also happened to remove the sizeof(char *) == sizeof(int)
 *   assumption introduced because of those /proc/dma patches. -- Hennus]
 */

// A note on resource allocation:
//
// All drivers needing DMA channels, should allocate and release them
// through the public routines `request_dma()` and `free_dma()`.
//
// In order to avoid problems, all processes should allocate resources in
// the same sequence and release them in the reverse order.
//
// So, when allocating DMAs and IRQs, first allocate the IRQ, then the DMA.
// When releasing them, first release the DMA, then release the IRQ.
// If you don't, you may cause allocation requests to fail unnecessarily.
// This doesn't really matter now, but it will once we get real semaphores
// in the kernel.

// Dependencies supplied by other translation units:
// DEFINE_SPINLOCK(dma_spin_lock);

#[cfg(max_dma_channels)]
struct DmaChan {
    lock: i32,
    device_id: *const core::ffi::c_char,
}

#[cfg(max_dma_channels)]
static mut DMA_CHAN_BUSY: [DmaChan; MAX_DMA_CHANNELS] = {
    let mut channels = [DmaChan {
        lock: 0,
        device_id: core::ptr::null(),
    }; MAX_DMA_CHANNELS];
    channels[4] = DmaChan {
        lock: 1,
        device_id: b"cascade\0".as_ptr() as *const core::ffi::c_char,
    };
    channels
};

/**
 * request_dma - request and reserve a system DMA channel
 * @dmanr: DMA channel number
 * @device_id: reserving device ID string, used in /proc/dma
 */
#[cfg(max_dma_channels)]
pub unsafe fn request_dma(
    dmanr: u32,
    device_id: *const core::ffi::c_char,
) -> i32 {
    if dmanr >= MAX_DMA_CHANNELS as u32 {
        return -EINVAL;
    }

    // C xchg(&dma_chan_busy[dmanr].lock, 1).
    let old = core::ptr::replace(&mut DMA_CHAN_BUSY[dmanr as usize].lock, 1);
    if old != 0 {
        return -EBUSY;
    }

    DMA_CHAN_BUSY[dmanr as usize].device_id = device_id;

    /* old flag was 0, now contains 1 to indicate busy */
    0
} // request_dma

/**
 * free_dma - free a reserved system DMA channel
 * @dmanr: DMA channel number
 */
#[cfg(max_dma_channels)]
pub unsafe fn free_dma(dmanr: u32) {
    if dmanr >= MAX_DMA_CHANNELS as u32 {
        printk!(KERN_WARNING, "Trying to free DMA%d\n", dmanr);
        return;
    }

    if core::ptr::replace(&mut DMA_CHAN_BUSY[dmanr as usize].lock, 0) == 0 {
        printk!(KERN_WARNING, "Trying to free free DMA%d\n", dmanr);
        return;
    }
} // free_dma

#[cfg(not(max_dma_channels))]
pub fn request_dma(_dmanr: u32, _device_id: *const core::ffi::c_char) -> i32 {
    -EINVAL
}

#[cfg(not(max_dma_channels))]
pub fn free_dma(_dmanr: u32) {}

#[cfg(all(config_proc_fs, max_dma_channels))]
unsafe fn proc_dma_show(m: *mut SeqFile, _v: *mut core::ffi::c_void) -> i32 {
    let mut i = 0;
    while i < MAX_DMA_CHANNELS {
        if DMA_CHAN_BUSY[i].lock != 0 {
            seq_printf!(m, "%2d: %s\n", i, DMA_CHAN_BUSY[i].device_id);
        }
        i += 1;
    }
    0
}

#[cfg(all(config_proc_fs, not(max_dma_channels)))]
unsafe fn proc_dma_show(m: *mut SeqFile, _v: *mut core::ffi::c_void) -> i32 {
    seq_puts!(m, "No DMA\n");
    0
}

#[cfg(config_proc_fs)]
unsafe fn proc_dma_init() -> i32 {
    proc_create_single!("dma", 0, core::ptr::null_mut(), proc_dma_show);
    0
}

// __initcall(proc_dma_init);

// EXPORT_SYMBOL(request_dma);
// EXPORT_SYMBOL(free_dma);
// EXPORT_SYMBOL(dma_spin_lock);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
