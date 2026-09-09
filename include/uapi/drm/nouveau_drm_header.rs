/*
 * Copyright 2005 Stephane Marchesin.
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

pub const DRM_NOUVEAU_EVENT_NVIF: u32 = 0x80000000;

pub const NOUVEAU_GETPARAM_PCI_VENDOR: u32 = 3;
pub const NOUVEAU_GETPARAM_PCI_DEVICE: u32 = 4;
pub const NOUVEAU_GETPARAM_BUS_TYPE: u32 = 5;
pub const NOUVEAU_GETPARAM_FB_SIZE: u32 = 8;
pub const NOUVEAU_GETPARAM_AGP_SIZE: u32 = 9;
pub const NOUVEAU_GETPARAM_CHIPSET_ID: u32 = 11;
pub const NOUVEAU_GETPARAM_VM_VRAM_BASE: u32 = 12;
pub const NOUVEAU_GETPARAM_GRAPH_UNITS: u32 = 13;
pub const NOUVEAU_GETPARAM_PTIMER_TIME: u32 = 14;
pub const NOUVEAU_GETPARAM_HAS_BO_USAGE: u32 = 15;
pub const NOUVEAU_GETPARAM_HAS_PAGEFLIP: u32 = 16;
pub const NOUVEAU_GETPARAM_EXEC_PUSH_MAX: u32 = 17;
pub const NOUVEAU_GETPARAM_VRAM_BAR_SIZE: u32 = 18;
pub const NOUVEAU_GETPARAM_VRAM_USED: u32 = 19;
pub const NOUVEAU_GETPARAM_HAS_VMA_TILEMODE: u32 = 20;

#[repr(C)]
pub struct drm_nouveau_getparam { pub param: u64, pub value: u64 }

pub const NOUVEAU_FIFO_ENGINE_GR: u32 = 0x01;
pub const NOUVEAU_FIFO_ENGINE_VP: u32 = 0x02;
pub const NOUVEAU_FIFO_ENGINE_PPP: u32 = 0x04;
pub const NOUVEAU_FIFO_ENGINE_BSP: u32 = 0x08;
pub const NOUVEAU_FIFO_ENGINE_CE: u32 = 0x30;
pub const NOUVEAU_FIFO_ENGINE_NVDEC: u32 = 0x300;

#[repr(C)]
pub struct drm_nouveau_channel_alloc {
    pub fb_ctxdma_handle: u32, pub tt_ctxdma_handle: u32, pub channel: i32,
    pub pushbuf_domains: u32, pub notifier_handle: u32,
    pub subchan: [drm_nouveau_channel_alloc_subchan; 8], pub nr_subchan: u32,
}
#[repr(C)] pub struct drm_nouveau_channel_alloc_subchan { pub handle: u32, pub grclass: u32 }
#[repr(C)] pub struct drm_nouveau_channel_free { pub channel: i32 }
#[repr(C)] pub struct drm_nouveau_notifierobj_alloc { pub channel: u32, pub handle: u32, pub size: u32, pub offset: u32 }
#[repr(C)] pub struct drm_nouveau_gpuobj_free { pub channel: i32, pub handle: u32 }

pub const NOUVEAU_GEM_DOMAIN_CPU: u32 = 1 << 0;
pub const NOUVEAU_GEM_DOMAIN_VRAM: u32 = 1 << 1;
pub const NOUVEAU_GEM_DOMAIN_GART: u32 = 1 << 2;
pub const NOUVEAU_GEM_DOMAIN_MAPPABLE: u32 = 1 << 3;
pub const NOUVEAU_GEM_DOMAIN_COHERENT: u32 = 1 << 4;
pub const NOUVEAU_GEM_DOMAIN_NO_SHARE: u32 = 1 << 5;
pub const NOUVEAU_GEM_TILE_COMP: u32 = 0x00030000;
pub const NOUVEAU_GEM_TILE_LAYOUT_MASK: u32 = 0x0000ff00;
pub const NOUVEAU_GEM_TILE_16BPP: u32 = 1;
pub const NOUVEAU_GEM_TILE_32BPP: u32 = 2;
pub const NOUVEAU_GEM_TILE_ZETA: u32 = 4;
pub const NOUVEAU_GEM_TILE_NONCONTIG: u32 = 8;

#[repr(C)] pub struct drm_nouveau_gem_info { pub handle: u32, pub domain: u32, pub size: u64, pub offset: u64, pub map_handle: u64, pub tile_mode: u32, pub tile_flags: u32 }
#[repr(C)] pub struct drm_nouveau_gem_new { pub info: drm_nouveau_gem_info, pub channel_hint: u32, pub align: u32 }
pub const NOUVEAU_GEM_MAX_BUFFERS: usize = 1024;
#[repr(C)] pub struct drm_nouveau_gem_pushbuf_bo_presumed { pub valid: u32, pub domain: u32, pub offset: u64 }
#[repr(C)] pub struct drm_nouveau_gem_pushbuf_bo { pub user_priv: u64, pub handle: u32, pub read_domains: u32, pub write_domains: u32, pub valid_domains: u32, pub presumed: drm_nouveau_gem_pushbuf_bo_presumed }
pub const NOUVEAU_GEM_RELOC_LOW: u32 = 1; pub const NOUVEAU_GEM_RELOC_HIGH: u32 = 2; pub const NOUVEAU_GEM_RELOC_OR: u32 = 4; pub const NOUVEAU_GEM_MAX_RELOCS: usize = 1024;
#[repr(C)] pub struct drm_nouveau_gem_pushbuf_reloc { pub reloc_bo_index: u32, pub reloc_bo_offset: u32, pub bo_index: u32, pub flags: u32, pub data: u32, pub vor: u32, pub tor: u32 }
pub const NOUVEAU_GEM_MAX_PUSH: usize = 512;
#[repr(C)] pub struct drm_nouveau_gem_pushbuf_push { pub bo_index: u32, pub pad: u32, pub offset: u64, pub length: u64 }
pub const NOUVEAU_GEM_PUSHBUF_NO_PREFETCH: u32 = 1 << 23;
#[repr(C)] pub struct drm_nouveau_gem_pushbuf { pub channel: u32, pub nr_buffers: u32, pub buffers: u64, pub nr_relocs: u32, pub nr_push: u32, pub relocs: u64, pub push: u64, pub suffix0: u32, pub suffix1: u32, pub vram_available: u64, pub gart_available: u64 }
pub const NOUVEAU_GEM_PUSHBUF_SYNC: u64 = 1;
pub const NOUVEAU_GEM_CPU_PREP_NOWAIT: u32 = 1; pub const NOUVEAU_GEM_CPU_PREP_WRITE: u32 = 4;
#[repr(C)] pub struct drm_nouveau_gem_cpu_prep { pub handle: u32, pub flags: u32 }
#[repr(C)] pub struct drm_nouveau_gem_cpu_fini { pub handle: u32 }

#[repr(C)] pub struct drm_nouveau_sync { pub flags: u32, pub handle: u32, pub timeline_value: u64 }
pub const DRM_NOUVEAU_SYNC_SYNCOBJ: u32 = 0; pub const DRM_NOUVEAU_SYNC_TIMELINE_SYNCOBJ: u32 = 1; pub const DRM_NOUVEAU_SYNC_TYPE_MASK: u32 = 0xf;
#[repr(C)] pub struct drm_nouveau_vm_init { pub kernel_managed_addr: u64, pub kernel_managed_size: u64 }
#[repr(C)] pub struct drm_nouveau_vm_bind_op { pub op: u32, pub flags: u32, pub handle: u32, pub pad: u32, pub addr: u64, pub bo_offset: u64, pub range: u64 }
pub const DRM_NOUVEAU_VM_BIND_OP_MAP: u32 = 0; pub const DRM_NOUVEAU_VM_BIND_OP_UNMAP: u32 = 1; pub const DRM_NOUVEAU_VM_BIND_SPARSE: u32 = 1 << 8;
#[repr(C)] pub struct drm_nouveau_vm_bind { pub op_count: u32, pub flags: u32, pub wait_count: u32, pub sig_count: u32, pub wait_ptr: u64, pub sig_ptr: u64, pub op_ptr: u64 }
pub const DRM_NOUVEAU_VM_BIND_RUN_ASYNC: u32 = 1;
#[repr(C)] pub struct drm_nouveau_exec_push { pub va: u64, pub va_len: u32, pub flags: u32 }
pub const DRM_NOUVEAU_EXEC_PUSH_NO_PREFETCH: u32 = 1;
#[repr(C)] pub struct drm_nouveau_exec { pub channel: u32, pub push_count: u32, pub wait_count: u32, pub sig_count: u32, pub wait_ptr: u64, pub sig_ptr: u64, pub push_ptr: u64 }
#[repr(C)] pub struct drm_nouveau_get_zcull_info { pub width_align_pixels: u32, pub height_align_pixels: u32, pub pixel_squares_by_aliquots: u32, pub aliquot_total: u32, pub zcull_region_byte_multiplier: u32, pub zcull_region_header_size: u32, pub zcull_subregion_header_size: u32, pub subregion_count: u32, pub subregion_width_align_pixels: u32, pub subregion_height_align_pixels: u32, pub ctxsw_size: u32, pub ctxsw_align: u32 }

pub const DRM_NOUVEAU_GETPARAM: u32 = 0x00; pub const DRM_NOUVEAU_SETPARAM: u32 = 0x01; pub const DRM_NOUVEAU_CHANNEL_ALLOC: u32 = 0x02; pub const DRM_NOUVEAU_CHANNEL_FREE: u32 = 0x03; pub const DRM_NOUVEAU_GROBJ_ALLOC: u32 = 0x04; pub const DRM_NOUVEAU_NOTIFIEROBJ_ALLOC: u32 = 0x05; pub const DRM_NOUVEAU_GPUOBJ_FREE: u32 = 0x06; pub const DRM_NOUVEAU_NVIF: u32 = 0x07; pub const DRM_NOUVEAU_SVM_INIT: u32 = 0x08; pub const DRM_NOUVEAU_SVM_BIND: u32 = 0x09; pub const DRM_NOUVEAU_VM_INIT: u32 = 0x10; pub const DRM_NOUVEAU_VM_BIND: u32 = 0x11; pub const DRM_NOUVEAU_EXEC: u32 = 0x12; pub const DRM_NOUVEAU_GET_ZCULL_INFO: u32 = 0x13; pub const DRM_NOUVEAU_GEM_NEW: u32 = 0x40; pub const DRM_NOUVEAU_GEM_PUSHBUF: u32 = 0x41; pub const DRM_NOUVEAU_GEM_CPU_PREP: u32 = 0x42; pub const DRM_NOUVEAU_GEM_CPU_FINI: u32 = 0x43; pub const DRM_NOUVEAU_GEM_INFO: u32 = 0x44;
#[repr(C)] pub struct drm_nouveau_svm_init { pub unmanaged_addr: u64, pub unmanaged_size: u64 }
#[repr(C)] pub struct drm_nouveau_svm_bind { pub header: u64, pub va_start: u64, pub va_end: u64, pub npages: u64, pub stride: u64, pub result: u64, pub reserved0: u64, pub reserved1: u64 }
pub const NOUVEAU_SVM_BIND_COMMAND_SHIFT: u32 = 0; pub const NOUVEAU_SVM_BIND_COMMAND_BITS: u32 = 8; pub const NOUVEAU_SVM_BIND_COMMAND_MASK: u32 = (1 << 8) - 1; pub const NOUVEAU_SVM_BIND_PRIORITY_SHIFT: u32 = 8; pub const NOUVEAU_SVM_BIND_PRIORITY_BITS: u32 = 8; pub const NOUVEAU_SVM_BIND_PRIORITY_MASK: u32 = (1 << 8) - 1; pub const NOUVEAU_SVM_BIND_TARGET_SHIFT: u32 = 16; pub const NOUVEAU_SVM_BIND_TARGET_BITS: u32 = 32; pub const NOUVEAU_SVM_BIND_TARGET_MASK: u32 = 0xffffffff;
pub const NOUVEAU_SVM_BIND_VALID_BITS: u32 = 48; pub const NOUVEAU_SVM_BIND_VALID_MASK: u64 = (1u64 << NOUVEAU_SVM_BIND_VALID_BITS) - 1; pub const NOUVEAU_SVM_BIND_COMMAND__MIGRATE: u32 = 0; pub const NOUVEAU_SVM_BIND_TARGET__GPU_VRAM: u64 = 1u64 << 31;

// ioctl encodings depend on DRM_IOWR/DRM_IOW/DRM_IOR and DRM_COMMAND_BASE supplied by drm.h.
pub const DRM_IOCTL_NOUVEAU_GETPARAM: u64 = DRM_IOWR!(DRM_COMMAND_BASE + DRM_NOUVEAU_GETPARAM, drm_nouveau_getparam);
pub const DRM_IOCTL_NOUVEAU_CHANNEL_ALLOC: u64 = DRM_IOWR!(DRM_COMMAND_BASE + DRM_NOUVEAU_CHANNEL_ALLOC, drm_nouveau_channel_alloc);
pub const DRM_IOCTL_NOUVEAU_CHANNEL_FREE: u64 = DRM_IOW!(DRM_COMMAND_BASE + DRM_NOUVEAU_CHANNEL_FREE, drm_nouveau_channel_free);
pub const DRM_IOCTL_NOUVEAU_SVM_INIT: u64 = DRM_IOWR!(DRM_COMMAND_BASE + DRM_NOUVEAU_SVM_INIT, drm_nouveau_svm_init);
pub const DRM_IOCTL_NOUVEAU_SVM_BIND: u64 = DRM_IOWR!(DRM_COMMAND_BASE + DRM_NOUVEAU_SVM_BIND, drm_nouveau_svm_bind);
pub const DRM_IOCTL_NOUVEAU_GEM_NEW: u64 = DRM_IOWR!(DRM_COMMAND_BASE + DRM_NOUVEAU_GEM_NEW, drm_nouveau_gem_new);
pub const DRM_IOCTL_NOUVEAU_GEM_PUSHBUF: u64 = DRM_IOWR!(DRM_COMMAND_BASE + DRM_NOUVEAU_GEM_PUSHBUF, drm_nouveau_gem_pushbuf);
pub const DRM_IOCTL_NOUVEAU_GEM_CPU_PREP: u64 = DRM_IOW!(DRM_COMMAND_BASE + DRM_NOUVEAU_GEM_CPU_PREP, drm_nouveau_gem_cpu_prep);
pub const DRM_IOCTL_NOUVEAU_GEM_CPU_FINI: u64 = DRM_IOW!(DRM_COMMAND_BASE + DRM_NOUVEAU_GEM_CPU_FINI, drm_nouveau_gem_cpu_fini);
pub const DRM_IOCTL_NOUVEAU_GEM_INFO: u64 = DRM_IOWR!(DRM_COMMAND_BASE + DRM_NOUVEAU_GEM_INFO, drm_nouveau_gem_info);
pub const DRM_IOCTL_NOUVEAU_VM_INIT: u64 = DRM_IOWR!(DRM_COMMAND_BASE + DRM_NOUVEAU_VM_INIT, drm_nouveau_vm_init);
pub const DRM_IOCTL_NOUVEAU_VM_BIND: u64 = DRM_IOWR!(DRM_COMMAND_BASE + DRM_NOUVEAU_VM_BIND, drm_nouveau_vm_bind);
pub const DRM_IOCTL_NOUVEAU_EXEC: u64 = DRM_IOWR!(DRM_COMMAND_BASE + DRM_NOUVEAU_EXEC, drm_nouveau_exec);
pub const DRM_IOCTL_NOUVEAU_GET_ZCULL_INFO: u64 = DRM_IOR!(DRM_COMMAND_BASE + DRM_NOUVEAU_GET_ZCULL_INFO, drm_nouveau_get_zcull_info);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
