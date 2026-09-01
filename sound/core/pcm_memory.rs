// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Digital Audio (PCM) abstract layer
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type bool_t = bool;

const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const UINT_MAX: size_t = u32::MAX as size_t;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_DMA_TYPE_UNKNOWN: c_int = 0;

type dma_data_direction = c_int;
const DMA_TO_DEVICE: dma_data_direction = 1;
const DMA_FROM_DEVICE: dma_data_direction = 2;

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dma_device {
    pub type_: c_int,
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dma_buffer {
    pub dev: snd_dma_device,
    pub area: *mut c_void,
    pub bytes: size_t,
}

#[repr(C)]
pub struct snd_card {
    pub total_pcm_alloc_bytes: ssize_t,
    pub memory_mutex: mutex,
    pub number: c_int,
}

#[repr(C)]
pub struct snd_pcm {
    pub card: *mut snd_card,
    pub device: c_int,
    pub name: *const c_char,
    pub open_mutex: mutex,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_buffer_p: *mut snd_dma_buffer,
    pub dma_bytes: size_t,
    pub dma_area: *mut c_void,
}

#[repr(C)]
pub struct snd_info_entry_text {
    pub write: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
}

#[repr(C)]
pub struct snd_info_entry_c {
    pub text: snd_info_entry_text,
}

#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut c_void,
    pub c: snd_info_entry_c,
    pub mode: c_ulong,
}

#[repr(C)]
pub struct snd_info_buffer {
    pub error: c_int,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub dma_buffer: snd_dma_buffer,
    pub pcm: *mut snd_pcm,
    pub stream: c_int,
    pub number: c_int,
    pub runtime: *mut snd_pcm_runtime,
    pub buffer_bytes_max: size_t,
    pub dma_max: size_t,
    pub managed_buffer_alloc: c_int,
    pub proc_root: *mut c_void,
}

unsafe extern "C" {
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn WARN_ON(condition: bool_t) -> c_int;
    fn snd_BUG_ON(condition: bool_t) -> c_int;
    fn snd_dma_alloc_dir_pages(
        type_: c_int,
        dev: *mut device,
        dir: dma_data_direction,
        size: size_t,
        dmab: *mut snd_dma_buffer,
    ) -> c_int;
    fn snd_dma_free_pages(dmab: *mut snd_dma_buffer);
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn snd_info_create_card_entry(
        card: *mut snd_card,
        name: *const c_char,
        parent: *mut c_void,
    ) -> *mut snd_info_entry;
    fn snd_info_set_text_ops(
        entry: *mut snd_info_entry,
        private_data: *mut snd_pcm_substream,
        read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    );
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_info_get_line(buffer: *mut snd_info_buffer, line: *mut c_char, len: size_t) -> c_int;
    fn snd_info_get_str(str_: *mut c_char, line: *mut c_char, len: size_t);
    fn kstrtoul(s: *const c_char, base: c_uint, res: *mut c_ulong) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn kzalloc(size: size_t) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn snd_pcm_set_runtime_buffer(
        substream: *mut snd_pcm_substream,
        dmab: *mut snd_dma_buffer,
    );
    fn PCM_RUNTIME_CHECK(substream: *mut snd_pcm_substream) -> c_int;
}

type c_uint = u32;

static mut preallocate_dma: c_int = 1;
/* module_param(preallocate_dma, int, 0444); */
/* MODULE_PARM_DESC(preallocate_dma, "Preallocate DMA memory when the PCM devices are initialized."); */

static mut maximum_substreams: c_int = 4;
/* module_param(maximum_substreams, int, 0444); */
/* MODULE_PARM_DESC(maximum_substreams, "Maximum substreams with preallocated DMA memory."); */

static snd_minimum_buffer: size_t = 16384;

static mut max_alloc_per_card: c_ulong = 32u64 as c_ulong * 1024u64 as c_ulong * 1024u64 as c_ulong;
/* module_param(max_alloc_per_card, ulong, 0644); */
/* MODULE_PARM_DESC(max_alloc_per_card, "Max total allocation bytes per card."); */

unsafe fn __update_allocated_size(card: *mut snd_card, bytes: ssize_t) {
    unsafe {
        (*card).total_pcm_alloc_bytes = (*card).total_pcm_alloc_bytes.wrapping_add(bytes);
    }
}

unsafe fn update_allocated_size(card: *mut snd_card, bytes: ssize_t) {
    unsafe {
        mutex_lock(&mut (*card).memory_mutex);
        __update_allocated_size(card, bytes);
        mutex_unlock(&mut (*card).memory_mutex);
    }
}

unsafe fn decrease_allocated_size(card: *mut snd_card, bytes: size_t) {
    unsafe {
        mutex_lock(&mut (*card).memory_mutex);
        WARN_ON((*card).total_pcm_alloc_bytes < bytes as ssize_t);
        __update_allocated_size(card, -(bytes as ssize_t));
        mutex_unlock(&mut (*card).memory_mutex);
    }
}

unsafe fn do_alloc_pages(
    card: *mut snd_card,
    type_: c_int,
    dev: *mut device,
    str_: c_int,
    size: size_t,
    dmab: *mut snd_dma_buffer,
) -> c_int {
    let dir: dma_data_direction;
    let err: c_int;

    /* check and reserve the requested size */
    unsafe {
        mutex_lock(&mut (*card).memory_mutex);
        if max_alloc_per_card != 0
            && ((*card).total_pcm_alloc_bytes as size_t).wrapping_add(size)
                > max_alloc_per_card as size_t
        {
            mutex_unlock(&mut (*card).memory_mutex);
            return -ENOMEM;
        }
        __update_allocated_size(card, size as ssize_t);
        mutex_unlock(&mut (*card).memory_mutex);

        if str_ == SNDRV_PCM_STREAM_PLAYBACK {
            dir = DMA_TO_DEVICE;
        } else {
            dir = DMA_FROM_DEVICE;
        }
        err = snd_dma_alloc_dir_pages(type_, dev, dir, size, dmab);
        if err == 0 {
            /* the actual allocation size might be bigger than requested,
             * and we need to correct the account
             */
            if (*dmab).bytes != size {
                update_allocated_size(card, (*dmab).bytes.wrapping_sub(size) as ssize_t);
            }
        } else {
            /* take back on allocation failure */
            decrease_allocated_size(card, size);
        }
    }
    err
}

unsafe fn do_free_pages(card: *mut snd_card, dmab: *mut snd_dma_buffer) {
    unsafe {
        if (*dmab).area.is_null() {
            return;
        }
        decrease_allocated_size(card, (*dmab).bytes);
        snd_dma_free_pages(dmab);
        (*dmab).area = ptr::null_mut();
    }
}

/*
 * try to allocate as the large pages as possible.
 * stores the resultant memory size in *res_size.
 *
 * the minimum size is snd_minimum_buffer.  it should be power of 2.
 */
unsafe fn preallocate_pcm_pages(
    substream: *mut snd_pcm_substream,
    mut size: size_t,
    no_fallback: bool_t,
) -> c_int {
    unsafe {
        let dmab: *mut snd_dma_buffer = &mut (*substream).dma_buffer;
        let card: *mut snd_card = (*(*substream).pcm).card;
        let orig_size: size_t = size;
        let mut err: c_int;

        loop {
            err = do_alloc_pages(
                card,
                (*dmab).dev.type_,
                (*dmab).dev.dev,
                (*substream).stream,
                size,
                dmab,
            );
            if err != -ENOMEM {
                return err;
            }
            if no_fallback {
                break;
            }
            size >>= 1;
            if size < snd_minimum_buffer {
                break;
            }
        }
        (*dmab).bytes = 0; /* tell error */
        pr_warn(
            c"ALSA pcmC%dD%d%c,%d:%s: cannot preallocate for size %zu\n".as_ptr(),
            (*(*(*substream).pcm).card).number,
            (*(*substream).pcm).device,
            if (*substream).stream != 0 { 'c' as c_int } else { 'p' as c_int },
            (*substream).number,
            (*(*substream).pcm).name,
            orig_size,
        );
        -ENOMEM
    }
}

/**
 * snd_pcm_lib_preallocate_free - release the preallocated buffer of the specified substream.
 * @substream: the pcm substream instance
 *
 * Releases the pre-allocated buffer of the given substream.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_lib_preallocate_free(substream: *mut snd_pcm_substream) {
    unsafe {
        do_free_pages((*(*substream).pcm).card, &mut (*substream).dma_buffer);
    }
}

/**
 * snd_pcm_lib_preallocate_free_for_all - release all pre-allocated buffers on the pcm
 * @pcm: the pcm instance
 *
 * Releases all the pre-allocated buffers on the given pcm.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_lib_preallocate_free_for_all(_pcm: *mut snd_pcm) {
    /*
     * C uses for_each_pcm_substream(pcm, stream, substream):
     *     snd_pcm_lib_preallocate_free(substream);
     */
}
/* EXPORT_SYMBOL(snd_pcm_lib_preallocate_free_for_all); */

/* CONFIG_SND_VERBOSE_PROCFS */
/*
 * read callback for prealloc proc file
 *
 * prints the current allocated size in kB.
 */
unsafe fn snd_pcm_lib_preallocate_proc_read(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    unsafe {
        let substream: *mut snd_pcm_substream = (*entry).private_data as *mut snd_pcm_substream;
        snd_iprintf(
            buffer,
            c"%lu\n".as_ptr(),
            ((*substream).dma_buffer.bytes as c_ulong) / 1024,
        );
    }
}

/*
 * read callback for prealloc_max proc file
 *
 * prints the maximum allowed size in kB.
 */
unsafe fn snd_pcm_lib_preallocate_max_proc_read(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    unsafe {
        let substream: *mut snd_pcm_substream = (*entry).private_data as *mut snd_pcm_substream;
        snd_iprintf(buffer, c"%lu\n".as_ptr(), ((*substream).dma_max as c_ulong) / 1024);
    }
}

/*
 * write callback for prealloc proc file
 *
 * accepts the preallocation size in kB.
 */
unsafe fn snd_pcm_lib_preallocate_proc_write(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    unsafe {
        let substream: *mut snd_pcm_substream = (*entry).private_data as *mut snd_pcm_substream;
        let card: *mut snd_card = (*(*substream).pcm).card;
        let mut line = [0 as c_char; 64];
        let mut str_ = [0 as c_char; 64];
        let mut size: c_ulong = 0;
        let mut new_dmab: snd_dma_buffer;

        mutex_lock(&mut (*(*substream).pcm).open_mutex);
        if !(*substream).runtime.is_null() {
            (*buffer).error = -EBUSY;
            mutex_unlock(&mut (*(*substream).pcm).open_mutex);
            return;
        }
        if snd_info_get_line(buffer, line.as_mut_ptr(), mem::size_of_val(&line)) == 0 {
            snd_info_get_str(str_.as_mut_ptr(), line.as_mut_ptr(), mem::size_of_val(&str_));
            (*buffer).error = kstrtoul(str_.as_ptr(), 10, &mut size);
            if (*buffer).error != 0 {
                mutex_unlock(&mut (*(*substream).pcm).open_mutex);
                return;
            }
            size = size.wrapping_mul(1024);
            if (size != 0 && size < 8192) || size as size_t > (*substream).dma_max {
                (*buffer).error = -EINVAL;
                mutex_unlock(&mut (*(*substream).pcm).open_mutex);
                return;
            }
            if (*substream).dma_buffer.bytes == size as size_t {
                mutex_unlock(&mut (*(*substream).pcm).open_mutex);
                return;
            }
            new_dmab = mem::zeroed();
            new_dmab.dev = (*substream).dma_buffer.dev;
            if size > 0 {
                if do_alloc_pages(
                    card,
                    (*substream).dma_buffer.dev.type_,
                    (*substream).dma_buffer.dev.dev,
                    (*substream).stream,
                    size as size_t,
                    &mut new_dmab,
                ) < 0
                {
                    (*buffer).error = -ENOMEM;
                    pr_debug(
                        c"ALSA pcmC%dD%d%c,%d:%s: cannot preallocate for size %lu\n".as_ptr(),
                        (*(*(*substream).pcm).card).number,
                        (*(*substream).pcm).device,
                        if (*substream).stream != 0 { 'c' as c_int } else { 'p' as c_int },
                        (*substream).number,
                        (*(*substream).pcm).name,
                        size,
                    );
                    mutex_unlock(&mut (*(*substream).pcm).open_mutex);
                    return;
                }
                (*substream).buffer_bytes_max = size as size_t;
            } else {
                (*substream).buffer_bytes_max = UINT_MAX;
            }
            if !(*substream).dma_buffer.area.is_null() {
                do_free_pages(card, &mut (*substream).dma_buffer);
            }
            (*substream).dma_buffer = new_dmab;
        } else {
            (*buffer).error = -EINVAL;
        }
        mutex_unlock(&mut (*(*substream).pcm).open_mutex);
    }
}

unsafe fn preallocate_info_init(substream: *mut snd_pcm_substream) {
    unsafe {
        let mut entry: *mut snd_info_entry;

        entry = snd_info_create_card_entry(
            (*(*substream).pcm).card,
            c"prealloc".as_ptr(),
            (*substream).proc_root,
        );
        if !entry.is_null() {
            snd_info_set_text_ops(entry, substream, Some(snd_pcm_lib_preallocate_proc_read));
            (*entry).c.text.write = Some(snd_pcm_lib_preallocate_proc_write);
            (*entry).mode |= 0o200;
        }
        entry = snd_info_create_card_entry(
            (*(*substream).pcm).card,
            c"prealloc_max".as_ptr(),
            (*substream).proc_root,
        );
        if !entry.is_null() {
            snd_info_set_text_ops(entry, substream, Some(snd_pcm_lib_preallocate_max_proc_read));
        }
    }
}

/*
 * If CONFIG_SND_VERBOSE_PROCFS is not enabled, C uses an empty inline
 * preallocate_info_init().
 */

/*
 * pre-allocate the buffer and create a proc file for the substream
 */
unsafe fn preallocate_pages(
    substream: *mut snd_pcm_substream,
    type_: c_int,
    data: *mut device,
    size: size_t,
    max: size_t,
    managed: bool_t,
) -> c_int {
    unsafe {
        let mut err: c_int;

        if snd_BUG_ON((*substream).dma_buffer.dev.type_ != 0) != 0 {
            return -EINVAL;
        }

        (*substream).dma_buffer.dev.type_ = type_;
        (*substream).dma_buffer.dev.dev = data;

        if size > 0 {
            if max == 0 {
                /* no fallback, only also inform -ENOMEM */
                err = preallocate_pcm_pages(substream, size, true);
                if err < 0 {
                    return err;
                }
            } else if preallocate_dma != 0 && (*substream).number < maximum_substreams {
                err = preallocate_pcm_pages(substream, size, false);
                if err < 0 && err != -ENOMEM {
                    return err;
                }
            }
        }

        if (*substream).dma_buffer.bytes > 0 {
            (*substream).buffer_bytes_max = (*substream).dma_buffer.bytes;
        }
        (*substream).dma_max = max;
        if max > 0 {
            preallocate_info_init(substream);
        }
        if managed {
            (*substream).managed_buffer_alloc = 1;
        }
        0
    }
}

unsafe fn preallocate_pages_for_all(
    _pcm: *mut snd_pcm,
    _type: c_int,
    _data: *mut c_void,
    _size: size_t,
    _max: size_t,
    _managed: bool_t,
) -> c_int {
    /*
     * C uses for_each_pcm_substream(pcm, stream, substream):
     *     err = preallocate_pages(substream, type, data, size, max, managed);
     *     if (err < 0)
     *         return err;
     */
    0
}

/**
 * snd_pcm_lib_preallocate_pages - pre-allocation for the given DMA type
 * @substream: the pcm substream instance
 * @type: DMA type (SNDRV_DMA_TYPE_*)
 * @data: DMA type dependent data
 * @size: the requested pre-allocation size in bytes
 * @max: the max. allowed pre-allocation size
 *
 * Do pre-allocation for the given DMA buffer type.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_lib_preallocate_pages(
    substream: *mut snd_pcm_substream,
    type_: c_int,
    data: *mut device,
    size: size_t,
    max: size_t,
) {
    unsafe {
        preallocate_pages(substream, type_, data, size, max, false);
    }
}
/* EXPORT_SYMBOL(snd_pcm_lib_preallocate_pages); */

/**
 * snd_pcm_lib_preallocate_pages_for_all - pre-allocation for continuous memory type (all substreams)
 * @pcm: the pcm instance
 * @type: DMA type (SNDRV_DMA_TYPE_*)
 * @data: DMA type dependent data
 * @size: the requested pre-allocation size in bytes
 * @max: the max. allowed pre-allocation size
 *
 * Do pre-allocation to all substreams of the given pcm for the
 * specified DMA type.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_lib_preallocate_pages_for_all(
    pcm: *mut snd_pcm,
    type_: c_int,
    data: *mut c_void,
    size: size_t,
    max: size_t,
) {
    unsafe {
        preallocate_pages_for_all(pcm, type_, data, size, max, false);
    }
}
/* EXPORT_SYMBOL(snd_pcm_lib_preallocate_pages_for_all); */

/**
 * snd_pcm_set_managed_buffer - set up buffer management for a substream
 * @substream: the pcm substream instance
 * @type: DMA type (SNDRV_DMA_TYPE_*)
 * @data: DMA type dependent data
 * @size: the requested pre-allocation size in bytes
 * @max: the max. allowed pre-allocation size
 *
 * Do pre-allocation for the given DMA buffer type, and set the managed
 * buffer allocation mode to the given substream.
 * In this mode, PCM core will allocate a buffer automatically before PCM
 * hw_params ops call, and release the buffer after PCM hw_free ops call
 * as well, so that the driver doesn't need to invoke the allocation and
 * the release explicitly in its callback.
 * When a buffer is actually allocated before the PCM hw_params call, it
 * turns on the runtime buffer_changed flag for drivers changing their h/w
 * parameters accordingly.
 *
 * When @size is non-zero and @max is zero, this tries to allocate for only
 * the exact buffer size without fallback, and may return -ENOMEM.
 * Otherwise, the function tries to allocate smaller chunks if the allocation
 * fails.  This is the behavior of snd_pcm_set_fixed_buffer().
 *
 * When both @size and @max are zero, the function only sets up the buffer
 * for later dynamic allocations. It's used typically for buffers with
 * SNDRV_DMA_TYPE_VMALLOC type.
 *
 * Upon successful buffer allocation and setup, the function returns 0.
 *
 * Return: zero if successful, or a negative error code
 */
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_set_managed_buffer(
    substream: *mut snd_pcm_substream,
    type_: c_int,
    data: *mut device,
    size: size_t,
    max: size_t,
) -> c_int {
    unsafe { preallocate_pages(substream, type_, data, size, max, true) }
}
/* EXPORT_SYMBOL(snd_pcm_set_managed_buffer); */

/**
 * snd_pcm_set_managed_buffer_all - set up buffer management for all substreams
 *	for all substreams
 * @pcm: the pcm instance
 * @type: DMA type (SNDRV_DMA_TYPE_*)
 * @data: DMA type dependent data
 * @size: the requested pre-allocation size in bytes
 * @max: the max. allowed pre-allocation size
 *
 * Do pre-allocation to all substreams of the given pcm for the specified DMA
 * type and size, and set the managed_buffer_alloc flag to each substream.
 *
 * Return: zero if successful, or a negative error code
 */
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_set_managed_buffer_all(
    pcm: *mut snd_pcm,
    type_: c_int,
    data: *mut device,
    size: size_t,
    max: size_t,
) -> c_int {
    unsafe { preallocate_pages_for_all(pcm, type_, data as *mut c_void, size, max, true) }
}
/* EXPORT_SYMBOL(snd_pcm_set_managed_buffer_all); */

/**
 * snd_pcm_lib_malloc_pages - allocate the DMA buffer
 * @substream: the substream to allocate the DMA buffer to
 * @size: the requested buffer size in bytes
 *
 * Allocates the DMA buffer on the BUS type given earlier to
 * snd_pcm_lib_preallocate_xxx_pages().
 *
 * Return: 1 if the buffer is changed, 0 if not changed, or a negative
 * code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_lib_malloc_pages(
    substream: *mut snd_pcm_substream,
    size: size_t,
) -> c_int {
    unsafe {
        let card: *mut snd_card;
        let runtime: *mut snd_pcm_runtime;
        let mut dmab: *mut snd_dma_buffer = ptr::null_mut();

        if PCM_RUNTIME_CHECK(substream) != 0 {
            return -EINVAL;
        }
        if snd_BUG_ON((*substream).dma_buffer.dev.type_ == SNDRV_DMA_TYPE_UNKNOWN) != 0 {
            return -EINVAL;
        }
        runtime = (*substream).runtime;
        card = (*(*substream).pcm).card;

        if !(*runtime).dma_buffer_p.is_null() {
            /* perphaps, we might free the large DMA memory region
               to save some space here, but the actual solution
               costs us less time */
            if (*(*runtime).dma_buffer_p).bytes >= size {
                (*runtime).dma_bytes = size;
                return 0; /* ok, do not change */
            }
            snd_pcm_lib_free_pages(substream);
        }
        if !(*substream).dma_buffer.area.is_null() && (*substream).dma_buffer.bytes >= size {
            dmab = &mut (*substream).dma_buffer; /* use the pre-allocated buffer */
        } else {
            /* dma_max=0 means the fixed size preallocation */
            if !(*substream).dma_buffer.area.is_null() && (*substream).dma_max == 0 {
                return -ENOMEM;
            }
            dmab = kzalloc(mem::size_of::<snd_dma_buffer>()) as *mut snd_dma_buffer;
            if dmab.is_null() {
                return -ENOMEM;
            }
            (*dmab).dev = (*substream).dma_buffer.dev;
            if do_alloc_pages(
                card,
                (*substream).dma_buffer.dev.type_,
                (*substream).dma_buffer.dev.dev,
                (*substream).stream,
                size,
                dmab,
            ) < 0
            {
                kfree(dmab as *mut c_void);
                pr_debug(
                    c"ALSA pcmC%dD%d%c,%d:%s: cannot allocate for size %zu\n".as_ptr(),
                    (*(*(*substream).pcm).card).number,
                    (*(*substream).pcm).device,
                    if (*substream).stream != 0 { 'c' as c_int } else { 'p' as c_int },
                    (*substream).number,
                    (*(*substream).pcm).name,
                    size,
                );
                return -ENOMEM;
            }
        }
        snd_pcm_set_runtime_buffer(substream, dmab);
        (*runtime).dma_bytes = size;
        1 /* area was changed */
    }
}
/* EXPORT_SYMBOL(snd_pcm_lib_malloc_pages); */

/**
 * snd_pcm_lib_free_pages - release the allocated DMA buffer.
 * @substream: the substream to release the DMA buffer
 *
 * Releases the DMA buffer allocated via snd_pcm_lib_malloc_pages().
 *
 * Return: Zero if successful, or a negative error code on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_pcm_lib_free_pages(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let runtime: *mut snd_pcm_runtime;

        if PCM_RUNTIME_CHECK(substream) != 0 {
            return -EINVAL;
        }
        runtime = (*substream).runtime;
        if (*runtime).dma_area.is_null() {
            return 0;
        }
        if (*runtime).dma_buffer_p != &mut (*substream).dma_buffer {
            let card: *mut snd_card = (*(*substream).pcm).card;

            /* it's a newly allocated buffer.  release it now. */
            do_free_pages(card, (*runtime).dma_buffer_p);
            kfree((*runtime).dma_buffer_p as *mut c_void);
        }
        snd_pcm_set_runtime_buffer(substream, ptr::null_mut());
        0
    }
}
/* EXPORT_SYMBOL(snd_pcm_lib_free_pages); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
