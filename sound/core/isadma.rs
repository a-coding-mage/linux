// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  ISA DMA support functions
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

/*
 * Defining following add some delay. Maybe this helps for some broken
 * ISA DMA controllers.
 */

/* #undef HAVE_REALLY_SLOW_DMA_CONTROLLER */

/* Dependencies from:
 * #include <linux/export.h>
 * #include <linux/isa-dma.h>
 * #include <sound/core.h>
 */

use core::ffi::{c_char, c_int, c_ulong, c_uint, c_ushort, c_void};

const DMA_MODE_NO_ENABLE: c_ushort = 0x10;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn claim_dma_lock() -> c_ulong;
    fn release_dma_lock(flags: c_ulong);
    fn disable_dma(dma: c_ulong);
    fn clear_dma_ff(dma: c_ulong);
    fn set_dma_mode(dma: c_ulong, mode: c_ushort);
    fn set_dma_addr(dma: c_ulong, addr: c_ulong);
    fn set_dma_count(dma: c_ulong, size: c_uint);
    fn enable_dma(dma: c_ulong);
    fn get_dma_residue(dma: c_ulong) -> c_uint;
    fn request_dma(dma: c_uint, name: *const c_char) -> c_int;
    fn free_dma(dma: c_uint);
    fn devres_alloc(
        release: unsafe extern "C" fn(dev: *mut device, data: *mut c_void),
        size: usize,
        gfp: c_uint,
    ) -> *mut c_void;
    fn devres_add(dev: *mut device, res: *mut c_void);

    static isa_dma_bridge_buggy: c_int;
}

/*
 * snd_dma_program - program an ISA DMA transfer
 * @dma: the dma number
 * @addr: the physical address of the buffer
 * @size: the DMA transfer size
 * @mode: the DMA transfer mode, DMA_MODE_XXX
 *
 * Programs an ISA DMA transfer for the given buffer.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_dma_program(
    dma: c_ulong,
    addr: c_ulong,
    size: c_uint,
    mode: c_ushort,
) {
    let flags: c_ulong;

    unsafe {
        flags = claim_dma_lock();
        disable_dma(dma);
        clear_dma_ff(dma);
        set_dma_mode(dma, mode);
        set_dma_addr(dma, addr);
        set_dma_count(dma, size);
        if mode & DMA_MODE_NO_ENABLE == 0 {
            enable_dma(dma);
        }
        release_dma_lock(flags);
    }
}
/* EXPORT_SYMBOL(snd_dma_program); */

/*
 * snd_dma_disable - stop the ISA DMA transfer
 * @dma: the dma number
 *
 * Stops the ISA DMA transfer.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_dma_disable(dma: c_ulong) {
    let flags: c_ulong;

    unsafe {
        flags = claim_dma_lock();
        clear_dma_ff(dma);
        disable_dma(dma);
        release_dma_lock(flags);
    }
}
/* EXPORT_SYMBOL(snd_dma_disable); */

/*
 * snd_dma_pointer - return the current pointer to DMA transfer buffer in bytes
 * @dma: the dma number
 * @size: the dma transfer size
 *
 * Return: The current pointer in DMA transfer buffer in bytes.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_dma_pointer(dma: c_ulong, size: c_uint) -> c_uint {
    let flags: c_ulong;
    let mut result: c_uint;
    let result1: c_uint;

    unsafe {
        flags = claim_dma_lock();
        clear_dma_ff(dma);
        if isa_dma_bridge_buggy == 0 {
            disable_dma(dma);
        }
        result = get_dma_residue(dma);
        /*
         * HACK - read the counter again and choose higher value in order to
         * avoid reading during counter lower byte roll over if the
         * isa_dma_bridge_buggy is set.
         */
        result1 = get_dma_residue(dma);
        if isa_dma_bridge_buggy == 0 {
            enable_dma(dma);
        }
        release_dma_lock(flags);
    }
    if result < result1 {
        result = result1;
    }
    /*
     * CONFIG_SND_DEBUG:
     * if (result > size)
     *     pr_err("ALSA: pointer (0x%x) for DMA #%ld is greater than transfer size (0x%x)\n",
     *            result, dma, size);
     */
    if result >= size || result == 0 {
        0
    } else {
        size - result
    }
}
/* EXPORT_SYMBOL(snd_dma_pointer); */

#[repr(C)]
struct snd_dma_data {
    dma: c_int,
}

unsafe extern "C" fn __snd_release_dma(_dev: *mut device, data: *mut c_void) {
    let p: *mut snd_dma_data = data as *mut snd_dma_data;

    unsafe {
        snd_dma_disable((*p).dma as c_ulong);
        free_dma((*p).dma as c_uint);
    }
}

/*
 * snd_devm_request_dma - the managed version of request_dma()
 * @dev: the device pointer
 * @dma: the dma number
 * @name: the name string of the requester
 *
 * The requested DMA will be automatically released at unbinding via devres.
 *
 * Return: zero on success, or a negative error code
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_devm_request_dma(
    dev: *mut device,
    dma: c_int,
    name: *const c_char,
) -> c_int {
    let p: *mut snd_dma_data;

    unsafe {
        if request_dma(dma as c_uint, name) != 0 {
            return -EBUSY;
        }
        p = devres_alloc(
            __snd_release_dma,
            core::mem::size_of::<snd_dma_data>(),
            GFP_KERNEL,
        ) as *mut snd_dma_data;
        if p.is_null() {
            free_dma(dma as c_uint);
            return -ENOMEM;
        }
        (*p).dma = dma;
        devres_add(dev, p as *mut c_void);
    }
    0
}
/* EXPORT_SYMBOL_GPL(snd_devm_request_dma); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
