/*
 * Copyright 2014 Advanced Micro Devices, Inc.
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
 */

// External kernel, amdgpu, and build-configuration dependencies are supplied by other files.

pub unsafe fn amdgpu_ih_ring_init(
    adev: *mut amdgpu_device,
    ih: *mut amdgpu_ih_ring,
    mut ring_size: libc::c_uint,
    use_bus_addr: bool,
) -> libc::c_int {
    let rb_bufsz = order_base_2(ring_size / 4);
    ring_size = (1u32 << rb_bufsz) * 4;
    (*ih).ring_size = ring_size;
    (*ih).ptr_mask = (*ih).ring_size - 1;
    (*ih).rptr = 0;
    (*ih).use_bus_addr = use_bus_addr;

    if use_bus_addr {
        let mut dma_addr: dma_addr_t = 0;
        if !(*ih).ring.is_null() {
            return 0;
        }
        (*ih).ring = dma_alloc_coherent((*adev).dev, (*ih).ring_size + 8, &mut dma_addr, GFP_KERNEL);
        if (*ih).ring.is_null() {
            return -libc::ENOMEM;
        }
        (*ih).gpu_addr = dma_addr;
        (*ih).wptr_addr = dma_addr + (*ih).ring_size as u64;
        (*ih).wptr_cpu = (*ih).ring.add((*ih).ring_size as usize / 4);
        (*ih).rptr_addr = dma_addr + (*ih).ring_size as u64 + 4;
        (*ih).rptr_cpu = (*ih).ring.add((*ih).ring_size as usize / 4 + 1);
    } else {
        let (mut wptr_offs, mut rptr_offs): (libc::c_uint, libc::c_uint) = (0, 0);
        let mut r = amdgpu_wb_get(adev, &mut wptr_offs);
        if r != 0 { return r; }
        r = amdgpu_wb_get(adev, &mut rptr_offs);
        if r != 0 {
            amdgpu_wb_free(adev, wptr_offs);
            return r;
        }
        r = amdgpu_bo_create_kernel(adev, (*ih).ring_size, PAGE_SIZE, AMDGPU_GEM_DOMAIN_GTT,
                                    &mut (*ih).ring_obj, &mut (*ih).gpu_addr,
                                    &mut (*ih).ring as *mut _ as *mut *mut u32);
        if r != 0 {
            amdgpu_wb_free(adev, rptr_offs);
            amdgpu_wb_free(adev, wptr_offs);
            return r;
        }
        (*ih).wptr_addr = (*adev).wb.gpu_addr + wptr_offs as u64 * 4;
        (*ih).wptr_cpu = (*adev).wb.wb.add(wptr_offs as usize);
        (*ih).rptr_addr = (*adev).wb.gpu_addr + rptr_offs as u64 * 4;
        (*ih).rptr_cpu = (*adev).wb.wb.add(rptr_offs as usize);
    }
    init_waitqueue_head(&mut (*ih).wait_process);
    0
}

pub unsafe fn amdgpu_ih_ring_fini(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) {
    if (*ih).ring.is_null() { return; }
    if (*ih).use_bus_addr {
        dma_free_coherent((*adev).dev, (*ih).ring_size + 8, (*ih).ring as *mut libc::c_void, (*ih).gpu_addr);
        (*ih).ring = core::ptr::null_mut();
    } else {
        amdgpu_bo_free_kernel(&mut (*ih).ring_obj, &mut (*ih).gpu_addr,
                              &mut (*ih).ring as *mut _ as *mut *mut u32);
        amdgpu_wb_free(adev, ((*ih).wptr_addr - (*ih).gpu_addr) as u32 / 4);
        amdgpu_wb_free(adev, ((*ih).rptr_addr - (*ih).gpu_addr) as u32 / 4);
    }
}

pub unsafe fn amdgpu_ih_ring_write(
    adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring, iv: *const u32, num_dw: libc::c_uint,
) {
    let mut wptr = le32_to_cpu(*(*ih).wptr_cpu) >> 2;
    for i in 0..num_dw {
        *(*ih).ring.add(wptr as usize) = cpu_to_le32(*iv.add(i as usize));
        wptr += 1;
    }
    wptr <<= 2;
    wptr &= (*ih).ptr_mask;
    if wptr != READ_ONCE((*ih).rptr) {
        wmb();
        WRITE_ONCE(*(*ih).wptr_cpu, cpu_to_le32(wptr));
    } else if (*adev).irq.retry_cam_enabled {
        dev_warn_once((*adev).dev, "IH soft ring buffer overflow 0x%X, 0x%X\n", wptr, (*ih).rptr);
    }
}

pub unsafe fn amdgpu_ih_wait_on_checkpoint_process_ts(
    adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring,
) -> libc::c_int {
    let timeout: libc::c_long = HZ;
    if !(*ih).enabled || (*adev).shutdown { return -libc::ENODEV; }
    let checkpoint_wptr = amdgpu_ih_get_wptr(adev, ih);
    rmb();
    let checkpoint_ts = amdgpu_ih_decode_iv_ts(adev, ih, checkpoint_wptr, -1);
    wait_event_interruptible_timeout(&mut (*ih).wait_process,
        amdgpu_ih_ts_after(checkpoint_ts, (*ih).processed_timestamp)
            || (*ih).rptr == amdgpu_ih_get_wptr(adev, ih), timeout)
}

pub unsafe fn amdgpu_ih_process(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) -> libc::c_int {
    if !(*ih).enabled || (*adev).shutdown { return IRQ_NONE; }
    let mut wptr = amdgpu_ih_get_wptr(adev, ih);
    'restart_ih: loop {
        let mut count = AMDGPU_IH_MAX_NUM_IVS;
        dev_dbg((*adev).dev, "%s: rptr %d, wptr %d\n", __func__, (*ih).rptr, wptr);
        rmb();
        while (*ih).rptr != wptr && { count -= 1; count != 0 } {
            amdgpu_irq_dispatch(adev, ih);
            (*ih).rptr &= (*ih).ptr_mask;
        }
        if !(*ih).overflow { amdgpu_ih_set_rptr(adev, ih); }
        wake_up_all(&mut (*ih).wait_process);
        wptr = amdgpu_ih_get_wptr(adev, ih);
        if wptr != (*ih).rptr && !(*ih).overflow { continue 'restart_ih; }
        break;
    }
    if (*ih).overflow && amdgpu_sriov_runtime(adev) {
        WARN_ONCE(!amdgpu_reset_domain_schedule((*adev).reset_domain, &mut (*adev).virt.flr_work),
                  "Failed to queue work! at %s", __func__);
    }
    IRQ_HANDLED
}

pub unsafe fn amdgpu_ih_decode_iv_helper(
    _adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring, entry: *mut amdgpu_iv_entry,
) {
    let ring_index = (*ih).rptr >> 2;
    let mut dw = [0u32; 8];
    for i in 0..8 { dw[i] = le32_to_cpu(*(*ih).ring.add((ring_index + i as u32) as usize)); }
    (*entry).client_id = dw[0] & 0xff;
    (*entry).src_id = (dw[0] >> 8) & 0xff;
    (*entry).ring_id = (dw[0] >> 16) & 0xff;
    (*entry).vmid = (dw[0] >> 24) & 0xf;
    (*entry).vmid_src = dw[0] >> 31;
    (*entry).timestamp = dw[1] as u64 | (((dw[2] & 0xffff) as u64) << 32);
    (*entry).timestamp_src = dw[2] >> 31;
    (*entry).pasid = dw[3] & 0xffff;
    (*entry).node_id = (dw[3] >> 16) & 0xff;
    (*entry).src_data[0] = dw[4]; (*entry).src_data[1] = dw[5];
    (*entry).src_data[2] = dw[6]; (*entry).src_data[3] = dw[7];
    (*ih).rptr += 32;
}

pub unsafe fn amdgpu_ih_decode_iv_ts_helper(ih: *mut amdgpu_ih_ring, mut rptr: u32, offset: i32) -> u64 {
    rptr = rptr.wrapping_add(32u32.wrapping_mul(offset as u32));
    let ring_index = (rptr & (*ih).ptr_mask) >> 2;
    let dw1 = le32_to_cpu(*(*ih).ring.add((ring_index + 1) as usize));
    let dw2 = le32_to_cpu(*(*ih).ring.add((ring_index + 2) as usize));
    dw1 as u64 | (((dw2 & 0xffff) as u64) << 32)
}

pub unsafe fn amdgpu_ih_ring_name(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) -> *const libc::c_char {
    if ih == &mut (*adev).irq.ih { b"ih\0".as_ptr() as *const _ }
    else if ih == &mut (*adev).irq.ih_soft { b"sw ih\0".as_ptr() as *const _ }
    else if ih == &mut (*adev).irq.ih1 { b"ih1\0".as_ptr() as *const _ }
    else if ih == &mut (*adev).irq.ih2 { b"ih2\0".as_ptr() as *const _ }
    else { b"unknown\0".as_ptr() as *const _ }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
