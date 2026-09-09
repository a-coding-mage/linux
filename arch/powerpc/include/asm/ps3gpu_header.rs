/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  PS3 GPU declarations.
 *
 *  Copyright 2009 Sony Corporation
 */

// Dependencies supplied by the surrounding kernel translation:
// <linux/mutex.h>, <asm/lv1call.h>

pub const L1GPU_CONTEXT_ATTRIBUTE_DISPLAY_SYNC: u64 = 0x101;
pub const L1GPU_CONTEXT_ATTRIBUTE_DISPLAY_FLIP: u64 = 0x102;

pub const L1GPU_CONTEXT_ATTRIBUTE_FB_SETUP: u64 = 0x600;
pub const L1GPU_CONTEXT_ATTRIBUTE_FB_BLIT: u64 = 0x601;
pub const L1GPU_CONTEXT_ATTRIBUTE_FB_BLIT_SYNC: u64 = 0x602;
pub const L1GPU_CONTEXT_ATTRIBUTE_FB_CLOSE: u64 = 0x603;

pub const L1GPU_FB_BLIT_WAIT_FOR_COMPLETION: u64 = 1u64 << 32;

pub const L1GPU_DISPLAY_SYNC_HSYNC: u64 = 1;
pub const L1GPU_DISPLAY_SYNC_VSYNC: u64 = 2;

/* mutex synchronizing GPU accesses and video mode changes */
extern "C" {
    pub static mut ps3_gpu_mutex: crate::mutex;
}

extern "C" {
    fn lv1_gpu_context_attribute(
        context_handle: u64,
        attribute: u64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
    ) -> i32;
}

pub unsafe fn lv1_gpu_display_sync(context_handle: u64, head: u64, ddr_offset: u64) -> i32 {
    lv1_gpu_context_attribute(
        context_handle,
        L1GPU_CONTEXT_ATTRIBUTE_DISPLAY_SYNC,
        head,
        ddr_offset,
        0,
        0,
    )
}

pub unsafe fn lv1_gpu_display_flip(context_handle: u64, head: u64, ddr_offset: u64) -> i32 {
    lv1_gpu_context_attribute(
        context_handle,
        L1GPU_CONTEXT_ATTRIBUTE_DISPLAY_FLIP,
        head,
        ddr_offset,
        0,
        0,
    )
}

pub unsafe fn lv1_gpu_fb_setup(
    context_handle: u64,
    xdr_lpar: u64,
    xdr_size: u64,
    ioif_offset: u64,
) -> i32 {
    lv1_gpu_context_attribute(
        context_handle,
        L1GPU_CONTEXT_ATTRIBUTE_FB_SETUP,
        xdr_lpar,
        xdr_size,
        ioif_offset,
        0,
    )
}

pub unsafe fn lv1_gpu_fb_blit(
    context_handle: u64,
    ddr_offset: u64,
    ioif_offset: u64,
    sync_width: u64,
    pitch: u64,
) -> i32 {
    lv1_gpu_context_attribute(
        context_handle,
        L1GPU_CONTEXT_ATTRIBUTE_FB_BLIT,
        ddr_offset,
        ioif_offset,
        sync_width,
        pitch,
    )
}

pub unsafe fn lv1_gpu_fb_close(context_handle: u64) -> i32 {
    lv1_gpu_context_attribute(
        context_handle,
        L1GPU_CONTEXT_ATTRIBUTE_FB_CLOSE,
        0,
        0,
        0,
        0,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
