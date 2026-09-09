// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright 2020 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: Christian König
 */

// Dependencies supplied by the surrounding DRM/TTM translation.

/* state back for walking over vram_mgr and gtt_mgr allocations */
#[repr(C)]
pub struct amdgpu_res_cursor {
    pub start: u64,
    pub size: u64,
    pub remaining: u64,
    pub node: *mut core::ffi::c_void,
    pub mem_type: u32,
}

/**
 * amdgpu_res_first - initialize a amdgpu_res_cursor
 *
 * @res: TTM resource object to walk
 * @start: Start of the range
 * @size: Size of the range
 * @cur: cursor object to initialize
 *
 * Start walking over the range of allocations between @start and @size.
 */
#[inline]
pub unsafe fn amdgpu_res_first(
    res: *mut ttm_resource,
    mut start: u64,
    size: u64,
    cur: *mut amdgpu_res_cursor,
) {
    let mut block: *mut gpu_buddy_block;
    let mut head: *mut list_head;
    let mut next: *mut list_head;
    let mut node: *mut drm_mm_node;

    if res.is_null() {
        (*cur).start = start;
        (*cur).size = size;
        (*cur).remaining = size;
        (*cur).node = core::ptr::null_mut();
        return;
    }

    BUG_ON(start + size > (*res).size);

    (*cur).mem_type = (*res).mem_type;

    match (*cur).mem_type {
        TTM_PL_VRAM => {
            head = &mut to_amdgpu_vram_mgr_resource(res).blocks;
            block = list_first_entry_or_null(head);
            if block.is_null() {
                (*cur).start = start;
                (*cur).size = size;
                (*cur).remaining = size;
                (*cur).node = core::ptr::null_mut();
                return;
            }

            while start >= amdgpu_vram_mgr_block_size(block) {
                start -= amdgpu_vram_mgr_block_size(block);
                next = (*block).link.next;
                if next != head {
                    block = list_entry(next);
                }
            }

            (*cur).start = amdgpu_vram_mgr_block_start(block) + start;
            (*cur).size = core::cmp::min(amdgpu_vram_mgr_block_size(block) - start, size);
            (*cur).remaining = size;
            (*cur).node = block.cast();
        }
        TTM_PL_TT | AMDGPU_PL_DOORBELL | AMDGPU_PL_MMIO_REMAP => {
            node = to_ttm_range_mgr_node(res).mm_nodes;
            while start >= (*node).size << PAGE_SHIFT {
                start -= (*node).size << PAGE_SHIFT;
                node = node.add(1);
            }

            (*cur).start = ((*node).start << PAGE_SHIFT) + start;
            (*cur).size = core::cmp::min(((*node).size << PAGE_SHIFT) - start, size);
            (*cur).remaining = size;
            (*cur).node = node.cast();
        }
        _ => {
            (*cur).start = start;
            (*cur).size = size;
            (*cur).remaining = size;
            (*cur).node = core::ptr::null_mut();
            WARN_ON(start + size > (*res).size);
        }
    }
}

/**
 * amdgpu_res_next - advance the cursor
 *
 * @cur: the cursor to advance
 * @size: number of bytes to move forward
 *
 * Move the cursor @size bytes forwrad, walking to the next node if necessary.
 */
#[inline]
pub unsafe fn amdgpu_res_next(cur: *mut amdgpu_res_cursor, size: u64) {
    let mut block: *mut gpu_buddy_block;
    let mut node: *mut drm_mm_node;
    let mut next: *mut list_head;

    BUG_ON(size > (*cur).remaining);
    (*cur).remaining -= size;
    if (*cur).remaining == 0 { return; }
    (*cur).size -= size;
    if (*cur).size != 0 {
        (*cur).start += size;
        return;
    }

    match (*cur).mem_type {
        TTM_PL_VRAM => {
            block = (*cur).node.cast();
            next = (*block).link.next;
            block = list_entry(next);
            (*cur).node = block.cast();
            (*cur).start = amdgpu_vram_mgr_block_start(block);
            (*cur).size = core::cmp::min(amdgpu_vram_mgr_block_size(block), (*cur).remaining);
        }
        TTM_PL_TT | AMDGPU_PL_DOORBELL | AMDGPU_PL_MMIO_REMAP => {
            node = (*cur).node.cast();
            node = node.add(1);
            (*cur).node = node.cast();
            (*cur).start = (*node).start << PAGE_SHIFT;
            (*cur).size = core::cmp::min((*node).size << PAGE_SHIFT, (*cur).remaining);
        }
        _ => {}
    }
}

/**
 * amdgpu_res_cleared - check if blocks are cleared
 *
 * @cur: the cursor to extract the block
 *
 * Check if the @cur block is cleared
 */
#[inline]
pub unsafe fn amdgpu_res_cleared(cur: *mut amdgpu_res_cursor) -> bool {
    match (*cur).mem_type {
        TTM_PL_VRAM => amdgpu_vram_mgr_is_cleared((*cur).node.cast()),
        _ => false,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
