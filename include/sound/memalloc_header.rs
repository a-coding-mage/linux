/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *                   Takashi Iwai <tiwai@suse.de>
 *
 *  Generic memory allocators
 */

// C dependencies: linux/dma-direction.h and asm/page.h.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sg_table {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dma_data_direction {
    DMA_BIDIRECTIONAL,
}

pub type dma_addr_t = usize;

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SHIFT: usize = 12;

/* buffer device info */
#[repr(C)]
pub struct snd_dma_device {
    pub r#type: i32, // SNDRV_DMA_TYPE_XXX
    pub dir: dma_data_direction, // DMA direction
    pub need_sync: bool, // explicit sync needed?
    pub dev: *mut device, // generic device
}

/* buffer types */
pub const SNDRV_DMA_TYPE_UNKNOWN: i32 = 0; // not defined
pub const SNDRV_DMA_TYPE_CONTINUOUS: i32 = 1; // continuous no-DMA memory
pub const SNDRV_DMA_TYPE_DEV: i32 = 2; // generic device continuous
pub const SNDRV_DMA_TYPE_DEV_WC: i32 = 5; // continuous write-combined
#[cfg(feature = "CONFIG_GENERIC_ALLOCATOR")]
pub const SNDRV_DMA_TYPE_DEV_IRAM: i32 = 4; // generic device iram-buffer
#[cfg(not(feature = "CONFIG_GENERIC_ALLOCATOR"))]
pub const SNDRV_DMA_TYPE_DEV_IRAM: i32 = SNDRV_DMA_TYPE_DEV;
pub const SNDRV_DMA_TYPE_VMALLOC: i32 = 7; // vmalloc'ed buffer
pub const SNDRV_DMA_TYPE_NONCONTIG: i32 = 8; // non-coherent SG buffer
pub const SNDRV_DMA_TYPE_NONCOHERENT: i32 = 9; // non-coherent buffer

#[cfg(feature = "CONFIG_SND_DMA_SGBUF")]
pub const SNDRV_DMA_TYPE_DEV_SG: i32 = 3; // S/G pages
#[cfg(feature = "CONFIG_SND_DMA_SGBUF")]
pub const SNDRV_DMA_TYPE_DEV_WC_SG: i32 = 6; // SG write-combined
#[cfg(not(feature = "CONFIG_SND_DMA_SGBUF"))]
pub const SNDRV_DMA_TYPE_DEV_SG: i32 = SNDRV_DMA_TYPE_DEV; // no SG-buf support
#[cfg(not(feature = "CONFIG_SND_DMA_SGBUF"))]
pub const SNDRV_DMA_TYPE_DEV_WC_SG: i32 = SNDRV_DMA_TYPE_DEV_WC;

/* info for buffer allocation */
#[repr(C)]
pub struct snd_dma_buffer {
    pub dev: snd_dma_device, // device type
    pub area: *mut u8, // virtual pointer
    pub addr: dma_addr_t, // physical address
    pub bytes: usize, // buffer size in bytes
    pub private_data: *mut core::ffi::c_void, // private for allocator; don't touch
}

/* return the pages matching with the given byte size */
#[inline]
pub fn snd_sgbuf_aligned_pages(size: usize) -> u32 {
    ((size.wrapping_add(PAGE_SIZE).wrapping_sub(1)) >> PAGE_SHIFT) as u32
}

extern "C" {
    pub fn snd_dma_alloc_dir_pages(
        r#type: i32,
        dev: *mut device,
        dir: dma_data_direction,
        size: usize,
        dmab: *mut snd_dma_buffer,
    ) -> i32;

    #[inline]
    pub fn snd_dma_alloc_pages(
        r#type: i32,
        dev: *mut device,
        size: usize,
        dmab: *mut snd_dma_buffer,
    ) -> i32 {
        snd_dma_alloc_dir_pages(r#type, dev, dma_data_direction::DMA_BIDIRECTIONAL, size, dmab)
    }

    pub fn snd_dma_alloc_pages_fallback(
        r#type: i32,
        dev: *mut device,
        size: usize,
        dmab: *mut snd_dma_buffer,
    ) -> i32;
    pub fn snd_dma_free_pages(dmab: *mut snd_dma_buffer);
    pub fn snd_dma_buffer_mmap(
        dmab: *mut snd_dma_buffer,
        area: *mut vm_area_struct,
    ) -> i32;
}

#[repr(C)]
pub enum snd_dma_sync_mode {
    SNDRV_DMA_SYNC_CPU,
    SNDRV_DMA_SYNC_DEVICE,
}

#[cfg(feature = "CONFIG_HAS_DMA")]
extern "C" {
    pub fn snd_dma_buffer_sync(dmab: *mut snd_dma_buffer, mode: snd_dma_sync_mode);
}

#[cfg(not(feature = "CONFIG_HAS_DMA"))]
#[inline]
pub unsafe fn snd_dma_buffer_sync(_dmab: *mut snd_dma_buffer, _mode: snd_dma_sync_mode) {}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

extern "C" {
    pub fn snd_sgbuf_get_addr(dmab: *mut snd_dma_buffer, offset: usize) -> dma_addr_t;
    pub fn snd_sgbuf_get_page(dmab: *mut snd_dma_buffer, offset: usize) -> *mut page;
    pub fn snd_sgbuf_get_chunk_size(
        dmab: *mut snd_dma_buffer,
        ofs: u32,
        size: u32,
    ) -> u32;

    /* device-managed memory allocator */
    pub fn snd_devm_alloc_dir_pages(
        dev: *mut device,
        r#type: i32,
        dir: dma_data_direction,
        size: usize,
    ) -> *mut snd_dma_buffer;

    #[inline]
    pub fn snd_devm_alloc_pages(
        dev: *mut device,
        r#type: i32,
        size: usize,
    ) -> *mut snd_dma_buffer {
        snd_devm_alloc_dir_pages(dev, r#type, dma_data_direction::DMA_BIDIRECTIONAL, size)
    }
}

#[inline]
pub unsafe fn snd_dma_noncontig_sg_table(dmab: *mut snd_dma_buffer) -> *mut sg_table {
    (*dmab).private_data as *mut sg_table
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
