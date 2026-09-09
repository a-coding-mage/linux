/*
 * Copyright © 2012 Red Hat
 * Copyright 1999 Precision Insight, Inc., Cedar Park, Texas.
 * Copyright 2000 VA Linux Systems, Inc., Sunnyvale, California.
 * Copyright (c) 2009-2010, Code Aurora Forum.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 *
 * Authors:
 *      Dave Airlie <airlied@redhat.com>
 *      Rob Clark <rob.clark@linaro.org>
 */

use core::ffi::{c_int, c_uint, c_ulong, c_void};

// Types supplied by the included Linux kernel headers.
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct rb_root { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct dma_buf_export_info { _private: [u8; 0] }
#[repr(C)] pub struct dma_buf { _private: [u8; 0] }
#[repr(C)] pub struct dma_buf_attachment { _private: [u8; 0] }
#[repr(C)] pub struct iosys_map { _private: [u8; 0] }
#[repr(C)] pub struct drm_device { _private: [u8; 0] }
#[repr(C)] pub struct drm_gem_object { _private: [u8; 0] }
#[repr(C)] pub struct drm_file { _private: [u8; 0] }
#[repr(C)] pub struct sg_table { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }

pub type dma_data_direction = c_int;
pub type dma_addr_t = u64;

/**
 * struct drm_prime_file_private - per-file tracking for PRIME
 *
 * This just contains the internal &struct dma_buf and handle caches for each
 * &struct drm_file used by the PRIME core code.
 */
#[repr(C)]
pub struct drm_prime_file_private {
    /* private: */
    pub lock: mutex,
    pub dmabufs: rb_root,
    pub handles: rb_root,
}

extern "C" {
    pub fn drm_gem_dmabuf_export(dev: *mut drm_device,
                                 exp_info: *mut dma_buf_export_info) -> *mut dma_buf;
    pub fn drm_gem_dmabuf_release(dma_buf: *mut dma_buf);

    pub fn drm_gem_prime_fd_to_handle(dev: *mut drm_device,
                                      file_priv: *mut drm_file,
                                      prime_fd: c_int,
                                      handle: *mut u32) -> c_int;
    pub fn drm_gem_prime_handle_to_dmabuf(dev: *mut drm_device,
                                          file_priv: *mut drm_file,
                                          handle: u32,
                                          flags: u32) -> *mut dma_buf;
    pub fn drm_gem_prime_handle_to_fd(dev: *mut drm_device,
                                      file_priv: *mut drm_file,
                                      handle: u32,
                                      flags: u32,
                                      prime_fd: *mut c_int) -> c_int;

    pub fn drm_gem_map_attach(dma_buf: *mut dma_buf,
                              attach: *mut dma_buf_attachment) -> c_int;
    pub fn drm_gem_map_detach(dma_buf: *mut dma_buf,
                              attach: *mut dma_buf_attachment);
    pub fn drm_gem_map_dma_buf(attach: *mut dma_buf_attachment,
                               dir: dma_data_direction) -> *mut sg_table;
    pub fn drm_gem_unmap_dma_buf(attach: *mut dma_buf_attachment,
                                 sgt: *mut sg_table,
                                 dir: dma_data_direction);
    pub fn drm_gem_dmabuf_vmap(dma_buf: *mut dma_buf,
                               map: *mut iosys_map) -> c_int;
    pub fn drm_gem_dmabuf_vunmap(dma_buf: *mut dma_buf,
                                 map: *mut iosys_map);

    pub fn drm_gem_prime_mmap(obj: *mut drm_gem_object,
                              vma: *mut vm_area_struct) -> c_int;
    pub fn drm_gem_dmabuf_mmap(dma_buf: *mut dma_buf,
                               vma: *mut vm_area_struct) -> c_int;

    pub fn drm_prime_pages_to_sg(dev: *mut drm_device,
                                 pages: *mut *mut page,
                                 nr_pages: c_uint) -> *mut sg_table;
    pub fn drm_gem_prime_export(obj: *mut drm_gem_object,
                                flags: c_int) -> *mut dma_buf;
    pub fn drm_prime_get_contiguous_size(sgt: *mut sg_table) -> c_ulong;

    pub fn drm_gem_is_prime_exported_dma_buf(dev: *mut drm_device,
                                             dma_buf: *mut dma_buf) -> bool;
    pub fn drm_gem_prime_import_dev(dev: *mut drm_device,
                                    dma_buf: *mut dma_buf,
                                    attach_dev: *mut device) -> *mut drm_gem_object;
    pub fn drm_gem_prime_import(dev: *mut drm_device,
                                dma_buf: *mut dma_buf) -> *mut drm_gem_object;
    pub fn drm_prime_gem_destroy(obj: *mut drm_gem_object,
                                 sg: *mut sg_table);
    pub fn drm_prime_sg_to_page_array(sgt: *mut sg_table,
                                      pages: *mut *mut page,
                                      max_pages: c_int) -> c_int;
    pub fn drm_prime_sg_to_dma_addr_array(sgt: *mut sg_table,
                                          addrs: *mut dma_addr_t,
                                          max_pages: c_int) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
