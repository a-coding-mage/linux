// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2020 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//

// Dependencies from the original C file:
// <linux/dma-mapping.h>, <linux/firmware.h>, <linux/ioport.h>,
// <linux/slab.h>, "core.h", "registers.h"

pub const FW_READY_TIMEOUT_MS: u32 = 250;

pub const FW_SIGNATURE: &[u8; 4] = b"$SST";
pub const FW_SIGNATURE_SIZE: usize = 4;

#[repr(C, packed)]
pub struct catpt_fw_hdr {
    pub signature: [::core::ffi::c_char; FW_SIGNATURE_SIZE],
    pub file_size: u32,
    pub modules: u32,
    pub file_format: u32,
    pub reserved: [u32; 4],
}

#[repr(C, packed)]
pub struct catpt_fw_module_hdr {
    pub signature: [::core::ffi::c_char; FW_SIGNATURE_SIZE],
    pub mod_size: u32,
    pub blocks: u32,
    pub slot: u16,
    pub module_id: u16,
    pub entry_point: u32,
    pub persistent_size: u32,
    pub scratch_size: u32,
}

#[repr(C)]
pub enum catpt_ram_type {
    CATPT_RAM_TYPE_IRAM = 1,
    CATPT_RAM_TYPE_DRAM = 2,
    /* DRAM with module's initial state */
    CATPT_RAM_TYPE_INSTANCE = 3,
}

#[repr(C, packed)]
pub struct catpt_fw_block_hdr {
    pub ram_type: u32,
    pub size: u32,
    pub ram_offset: u32,
    pub rsvd: u32,
}

extern "C" {
    static CATPT_MODID_LAST: u16;
    static CATPT_MODID_BASE_FW: u16;
    static CATPT_DX_TYPE_MEMORY_DUMP: u32;
    static CATPT_DX_TYPE_FW_IMAGE: u32;
    static DUMP_PREFIX_OFFSET: i32;
    static GFP_KERNEL: gfp_t;

    fn release_resource(res: *mut resource) -> i32;
    fn kfree(ptr: *const ::core::ffi::c_void);
    fn __request_region(
        parent: *mut resource,
        start: resource_size_t,
        n: resource_size_t,
        name: *const ::core::ffi::c_char,
        flags: i32,
    ) -> *mut resource;
    fn __release_region(parent: *mut resource, start: resource_size_t, n: resource_size_t);
    fn resource_size(res: *const resource) -> resource_size_t;
    fn resource_set_range(res: *mut resource, start: resource_size_t, size: resource_size_t);
    fn resource_contains(r1: *const resource, r2: *const resource) -> bool;
    fn resource_intersection(
        r1: *const resource,
        r2: *const resource,
        result: *mut resource,
    ) -> bool;

    fn catpt_dma_request_config_chan(cdev: *mut catpt_dev) -> *mut dma_chan;
    fn dma_release_channel(chan: *mut dma_chan);
    fn catpt_dma_memcpy_fromdsp(
        cdev: *mut catpt_dev,
        chan: *mut dma_chan,
        dst_addr: dma_addr_t,
        src_addr: dma_addr_t,
        size: usize,
    ) -> i32;
    fn catpt_dma_memcpy_todsp(
        cdev: *mut catpt_dev,
        chan: *mut dma_chan,
        dst_addr: dma_addr_t,
        src_addr: dma_addr_t,
        size: usize,
    ) -> i32;

    fn catpt_dsp_stall(cdev: *mut catpt_dev, stall: bool) -> i32;
    fn catpt_to_host_offset(offset: u32) -> u32;
    fn catpt_dsp_update_srampge(cdev: *mut catpt_dev, sram: *mut resource, mask: u32);
    fn catpt_dsp_update_lpclock(cdev: *mut catpt_dev) -> i32;
    fn catpt_ipc_get_mixer_stream_info(cdev: *mut catpt_dev, mixer: *mut catpt_mixer) -> i32;
    fn catpt_arm_stream_templates(cdev: *mut catpt_dev) -> i32;

    fn dev_dbg(dev: *mut device, fmt: *const ::core::ffi::c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const ::core::ffi::c_char, ...);
    fn print_hex_dump_debug(
        level: *const ::core::ffi::c_char,
        prefix_type: i32,
        rowsize: i32,
        groupsize: i32,
        buf: *const ::core::ffi::c_void,
        len: usize,
        ascii: bool,
    );
    fn strncmp(
        cs: *const ::core::ffi::c_char,
        ct: *const ::core::ffi::c_char,
        count: usize,
    ) -> i32;
    fn request_firmware(
        firmware_p: *mut *const firmware,
        name: *const ::core::ffi::c_char,
        device: *mut device,
    ) -> i32;
    fn release_firmware(fw: *const firmware);
    fn dma_alloc_coherent(
        dev: *mut device,
        size: usize,
        dma_handle: *mut dma_addr_t,
        flag: gfp_t,
    ) -> *mut ::core::ffi::c_void;
    fn dma_free_coherent(
        dev: *mut device,
        size: usize,
        cpu_addr: *mut ::core::ffi::c_void,
        dma_handle: dma_addr_t,
    );
    fn memcpy(
        dest: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        n: usize,
    ) -> *mut ::core::ffi::c_void;
    fn reinit_completion(x: *mut completion);
    fn wait_for_completion_timeout(x: *mut completion, timeout: u64) -> u64;
    fn msecs_to_jiffies(m: u32) -> u64;
    fn IS_ERR(ptr: *const ::core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const ::core::ffi::c_void) -> i32;
    fn CATPT_IPC_RET(ret: i32) -> i32;
}

pub type resource_size_t = u64;
pub type dma_addr_t = u64;
pub type gfp_t = u32;

#[repr(C)]
pub struct resource {
    pub start: resource_size_t,
    pub end: resource_size_t,
    pub child: *mut resource,
    pub sibling: *mut resource,
}

#[repr(C)]
pub struct firmware {
    pub size: usize,
    pub data: *const u8,
}

#[repr(C)]
pub struct dma_chan {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct catpt_stream_info {
    pub stream_hw_id: i32,
}

#[repr(C)]
pub struct catpt_stream_runtime {
    pub node: list_head,
    pub persistent: *mut resource,
    pub info: catpt_stream_info,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct catpt_module_type {
    pub loaded: bool,
    pub state_size: u32,
    pub state_offset: u32,
    pub entry_point: u32,
    pub persistent_size: u32,
    pub scratch_size: u32,
}

#[repr(C)]
pub struct catpt_save_meminfo {
    pub source: u32,
    pub offset: u32,
    pub size: u32,
}

#[repr(C)]
pub struct catpt_dx_context {
    pub num_meminfo: i32,
    pub meminfo: *mut catpt_save_meminfo,
}

#[repr(C)]
pub struct catpt_spec {
    pub fw_name: *const ::core::ffi::c_char,
    pub dram_mask: u32,
    pub iram_mask: u32,
}

#[repr(C)]
pub struct catpt_ipc {
    pub ready: bool,
}

#[repr(C)]
pub struct catpt_mixer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct catpt_dev {
    pub dev: *mut device,
    pub stream_list: list_head,
    pub modules: *mut catpt_module_type,
    pub dxbuf_paddr: dma_addr_t,
    pub lpe_base: dma_addr_t,
    pub dx_ctx: catpt_dx_context,
    pub dram: resource,
    pub iram: resource,
    pub spec: *mut catpt_spec,
    pub fw_ready: completion,
    pub ipc: catpt_ipc,
    pub mixer: catpt_mixer,
}

const EBUSY: i32 = 16;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const ETIMEDOUT: i32 = 110;
const EREMOTEIO: i32 = 121;

#[inline]
const fn ALIGN(x: u32, a: u32) -> usize {
    (((x).wrapping_add(a).wrapping_sub(1)) & !(a.wrapping_sub(1))) as usize
}

unsafe fn list_entry_stream_runtime(ptr: *mut list_head) -> *mut catpt_stream_runtime {
    (ptr as *mut u8).sub(::core::mem::offset_of!(catpt_stream_runtime, node))
        as *mut catpt_stream_runtime
}

#[no_mangle]
pub unsafe extern "C" fn catpt_sram_free(sram: *mut resource) {
    let mut res: *mut resource = (*sram).child;
    let mut save: *mut resource;

    while !res.is_null() {
        save = (*res).sibling;
        release_resource(res);
        kfree(res as *const ::core::ffi::c_void);
        res = save;
    }
}

#[no_mangle]
pub unsafe extern "C" fn catpt_request_region(
    root: *mut resource,
    size: resource_size_t,
) -> *mut resource {
    let mut res: *mut resource = (*root).child;
    let mut addr: resource_size_t = (*root).start;

    loop {
        if (*res).start.wrapping_sub(addr) >= size {
            break;
        }
        addr = (*res).end.wrapping_add(1);
        res = (*res).sibling;
        if res.is_null() {
            return ::core::ptr::null_mut();
        }
    }

    __request_region(root, addr, size, ::core::ptr::null(), 0)
}

unsafe fn catpt_store_streams_context(cdev: *mut catpt_dev, chan: *mut dma_chan) -> i32 {
    let head = &mut (*cdev).stream_list as *mut list_head;
    let mut pos = (*head).next;

    /* Lockless as no streams can be added or removed during D3 -> D0 transition. */
    while pos != head {
        let stream = list_entry_stream_runtime(pos);
        let off: u32;
        let size: u32;
        let ret: i32;

        off = (*(*stream).persistent).start as u32;
        size = resource_size((*stream).persistent) as u32;
        dev_dbg(
            (*cdev).dev,
            c"storing stream %d ctx: off 0x%08x size %d\n".as_ptr(),
            (*stream).info.stream_hw_id,
            off,
            size,
        );

        ret = catpt_dma_memcpy_fromdsp(
            cdev,
            chan,
            (*cdev).dxbuf_paddr.wrapping_add(off as dma_addr_t),
            (*cdev).lpe_base.wrapping_add(off as dma_addr_t),
            ALIGN(size, 4),
        );
        if ret != 0 {
            dev_err((*cdev).dev, c"memcpy fromdsp failed: %d\n".as_ptr(), ret);
            return ret;
        }
        pos = (*pos).next;
    }

    0
}

unsafe fn catpt_store_module_states(cdev: *mut catpt_dev, chan: *mut dma_chan) -> i32 {
    let mut i: i32;

    i = 0;
    while i < CATPT_MODID_LAST as i32 + 1 {
        let type_: *mut catpt_module_type;
        let off: u32;
        let ret: i32;

        type_ = (*cdev).modules.add(i as usize);
        if !(*type_).loaded || (*type_).state_size == 0 {
            i += 1;
            continue;
        }

        off = (*type_).state_offset;
        dev_dbg(
            (*cdev).dev,
            c"storing mod %d state: off 0x%08x size %d\n".as_ptr(),
            i,
            off,
            (*type_).state_size,
        );

        ret = catpt_dma_memcpy_fromdsp(
            cdev,
            chan,
            (*cdev).dxbuf_paddr.wrapping_add(off as dma_addr_t),
            (*cdev).lpe_base.wrapping_add(off as dma_addr_t),
            ALIGN((*type_).state_size, 4),
        );
        if ret != 0 {
            dev_err((*cdev).dev, c"memcpy fromdsp failed: %d\n".as_ptr(), ret);
            return ret;
        }
        i += 1;
    }

    0
}

unsafe fn catpt_store_dram_data(cdev: *mut catpt_dev, chan: *mut dma_chan) -> i32 {
    let mut i: i32;

    i = 0;
    while i < (*cdev).dx_ctx.num_meminfo {
        let info: *mut catpt_save_meminfo;
        let off: u32;
        let ret: i32;

        info = (*cdev).dx_ctx.meminfo.add(i as usize);
        if (*info).source != CATPT_DX_TYPE_MEMORY_DUMP {
            i += 1;
            continue;
        }

        off = catpt_to_host_offset((*info).offset);
        if (off as resource_size_t) < (*cdev).dram.start || (off as resource_size_t) > (*cdev).dram.end {
            i += 1;
            continue;
        }

        dev_dbg(
            (*cdev).dev,
            c"storing memdump: off 0x%08x size %d\n".as_ptr(),
            off,
            (*info).size,
        );

        ret = catpt_dma_memcpy_fromdsp(
            cdev,
            chan,
            (*cdev).dxbuf_paddr.wrapping_add(off as dma_addr_t),
            (*cdev).lpe_base.wrapping_add(off as dma_addr_t),
            ALIGN((*info).size, 4),
        );
        if ret != 0 {
            dev_err((*cdev).dev, c"memcpy fromdsp failed: %d\n".as_ptr(), ret);
            return ret;
        }
        i += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn catpt_store_firmware_context(cdev: *mut catpt_dev) -> i32 {
    let chan: *mut dma_chan;
    let mut ret: i32;

    chan = catpt_dma_request_config_chan(cdev);
    if IS_ERR(chan as *const ::core::ffi::c_void) {
        return PTR_ERR(chan as *const ::core::ffi::c_void);
    }

    ret = catpt_dsp_stall(cdev, true);
    if ret != 0 {
        dma_release_channel(chan);
        return ret;
    }

    ret = catpt_store_dram_data(cdev, chan);
    if ret != 0 {
        dev_err((*cdev).dev, c"store memdumps failed: %d\n".as_ptr(), ret);
        dma_release_channel(chan);
        return ret;
    }

    ret = catpt_store_module_states(cdev, chan);
    if ret != 0 {
        dev_err((*cdev).dev, c"store module states failed: %d\n".as_ptr(), ret);
        dma_release_channel(chan);
        return ret;
    }

    ret = catpt_store_streams_context(cdev, chan);
    if ret != 0 {
        dev_err((*cdev).dev, c"store streams ctx failed: %d\n".as_ptr(), ret);
    }
    dma_release_channel(chan);
    ret
}

unsafe fn catpt_restore_streams_context(cdev: *mut catpt_dev, chan: *mut dma_chan) -> i32 {
    let head = &mut (*cdev).stream_list as *mut list_head;
    let mut pos = (*head).next;

    /* Lockless as no streams can be added or removed during D3 -> D0 transition. */
    while pos != head {
        let stream = list_entry_stream_runtime(pos);
        let off: u32;
        let size: u32;
        let ret: i32;

        off = (*(*stream).persistent).start as u32;
        size = resource_size((*stream).persistent) as u32;
        dev_dbg(
            (*cdev).dev,
            c"restoring stream %d ctx: off 0x%08x size %d\n".as_ptr(),
            (*stream).info.stream_hw_id,
            off,
            size,
        );

        ret = catpt_dma_memcpy_todsp(
            cdev,
            chan,
            (*cdev).lpe_base.wrapping_add(off as dma_addr_t),
            (*cdev).dxbuf_paddr.wrapping_add(off as dma_addr_t),
            ALIGN(size, 4),
        );
        if ret != 0 {
            dev_err((*cdev).dev, c"memcpy fromdsp failed: %d\n".as_ptr(), ret);
            return ret;
        }
        pos = (*pos).next;
    }

    0
}

unsafe fn catpt_restore_dram_data(cdev: *mut catpt_dev, chan: *mut dma_chan) -> i32 {
    let mut i: i32;

    i = 0;
    while i < (*cdev).dx_ctx.num_meminfo {
        let info: *mut catpt_save_meminfo;
        let mut r: resource = ::core::mem::zeroed();
        let off: u32;
        let ret: i32;

        info = (*cdev).dx_ctx.meminfo.add(i as usize);
        if (*info).source != CATPT_DX_TYPE_MEMORY_DUMP {
            i += 1;
            continue;
        }

        off = catpt_to_host_offset((*info).offset);
        resource_set_range(&mut r, off as resource_size_t, (*info).size as resource_size_t);
        if !resource_contains(&(*cdev).dram, &r) {
            i += 1;
            continue;
        }

        dev_dbg(
            (*cdev).dev,
            c"restoring memdump: off 0x%08x size %d\n".as_ptr(),
            off,
            (*info).size,
        );

        ret = catpt_dma_memcpy_todsp(
            cdev,
            chan,
            (*cdev).lpe_base.wrapping_add(off as dma_addr_t),
            (*cdev).dxbuf_paddr.wrapping_add(off as dma_addr_t),
            ALIGN((*info).size, 4),
        );
        if ret != 0 {
            dev_err((*cdev).dev, c"restore block failed: %d\n".as_ptr(), ret);
            return ret;
        }
        i += 1;
    }

    0
}

unsafe fn catpt_restore_dram_rodata(
    cdev: *mut catpt_dev,
    chan: *mut dma_chan,
    mut paddr: dma_addr_t,
    blk: *mut catpt_fw_block_hdr,
) -> i32 {
    let mut r1: resource = ::core::mem::zeroed();
    let mut i: i32;

    print_hex_dump_debug(
        c"catpt_restore_dram_rodata".as_ptr(),
        DUMP_PREFIX_OFFSET,
        8,
        4,
        blk as *const ::core::ffi::c_void,
        ::core::mem::size_of_val(&*blk),
        false,
    );

    resource_set_range(
        &mut r1,
        (*cdev).dram.start.wrapping_add((*blk).ram_offset as resource_size_t),
        (*blk).size as resource_size_t,
    );
    /* advance to data area */
    paddr = paddr.wrapping_add(::core::mem::size_of_val(&*blk) as dma_addr_t);

    i = 0;
    while i < (*cdev).dx_ctx.num_meminfo {
        let info: *mut catpt_save_meminfo;
        let mut common: resource = ::core::mem::zeroed();
        let mut r2: resource = ::core::mem::zeroed();
        let mut off: u32;
        let ret: i32;

        info = (*cdev).dx_ctx.meminfo.add(i as usize);
        if (*info).source != CATPT_DX_TYPE_FW_IMAGE {
            i += 1;
            continue;
        }

        off = catpt_to_host_offset((*info).offset);
        resource_set_range(&mut r2, off as resource_size_t, (*info).size as resource_size_t);
        if !resource_contains(&(*cdev).dram, &r2) {
            i += 1;
            continue;
        }

        if !resource_intersection(&r2, &r1, &mut common) {
            i += 1;
            continue;
        }
        /* calculate start offset of common data area */
        off = common.start.wrapping_sub(r1.start) as u32;

        dev_dbg((*cdev).dev, c"restoring fwimage: %pr\n".as_ptr(), &mut common);

        ret = catpt_dma_memcpy_todsp(
            cdev,
            chan,
            common.start as dma_addr_t,
            paddr.wrapping_add(off as dma_addr_t),
            resource_size(&common) as usize,
        );
        if ret != 0 {
            dev_err((*cdev).dev, c"memcpy todsp failed: %d\n".as_ptr(), ret);
            return ret;
        }
        i += 1;
    }

    0
}

unsafe fn catpt_load_block(
    cdev: *mut catpt_dev,
    chan: *mut dma_chan,
    mut paddr: dma_addr_t,
    blk: *mut catpt_fw_block_hdr,
    alloc: bool,
) -> i32 {
    let sram: *mut resource;
    let res: *mut resource;
    let dst_addr: dma_addr_t;
    let ret: i32;

    print_hex_dump_debug(
        c"catpt_load_block".as_ptr(),
        DUMP_PREFIX_OFFSET,
        8,
        4,
        blk as *const ::core::ffi::c_void,
        ::core::mem::size_of_val(&*blk),
        false,
    );

    match (*blk).ram_type {
        x if x == catpt_ram_type::CATPT_RAM_TYPE_IRAM as u32 => {
            sram = &mut (*cdev).iram;
        }
        _ => {
            sram = &mut (*cdev).dram;
        }
    }

    dst_addr = (*sram).start.wrapping_add((*blk).ram_offset as resource_size_t) as dma_addr_t;
    if alloc {
        res = __request_region(sram, dst_addr as resource_size_t, (*blk).size as resource_size_t, ::core::ptr::null(), 0);
        if res.is_null() {
            return -EBUSY;
        }
    }

    /* advance to data area */
    paddr = paddr.wrapping_add(::core::mem::size_of_val(&*blk) as dma_addr_t);

    ret = catpt_dma_memcpy_todsp(cdev, chan, dst_addr, paddr, (*blk).size as usize);
    if ret != 0 {
        dev_err((*cdev).dev, c"memcpy error: %d\n".as_ptr(), ret);
        __release_region(sram, dst_addr as resource_size_t, (*blk).size as resource_size_t);
    }

    ret
}

unsafe fn catpt_restore_basefw(
    cdev: *mut catpt_dev,
    chan: *mut dma_chan,
    paddr: dma_addr_t,
    basefw: *mut catpt_fw_module_hdr,
) -> i32 {
    let mut off: u32 = ::core::mem::size_of_val(&*basefw) as u32;
    let mut ret: i32;
    let mut i: i32;

    print_hex_dump_debug(
        c"catpt_restore_basefw".as_ptr(),
        DUMP_PREFIX_OFFSET,
        8,
        4,
        basefw as *const ::core::ffi::c_void,
        ::core::mem::size_of_val(&*basefw),
        false,
    );

    /* Restore IRAM and .rodata for DRAM based on the firmware image. */
    i = 0;
    while i < (*basefw).blocks as i32 {
        let blk: *mut catpt_fw_block_hdr;

        blk = (basefw as *mut u8).add(off as usize) as *mut catpt_fw_block_hdr;

        match (*blk).ram_type {
            x if x == catpt_ram_type::CATPT_RAM_TYPE_IRAM as u32 => {
                ret = catpt_load_block(cdev, chan, paddr.wrapping_add(off as dma_addr_t), blk, false);
            }
            _ => {
                ret = catpt_restore_dram_rodata(cdev, chan, paddr.wrapping_add(off as dma_addr_t), blk);
            }
        }

        if ret != 0 {
            dev_err((*cdev).dev, c"restore block failed: %d\n".as_ptr(), ret);
            return ret;
        }

        off = off
            .wrapping_add(::core::mem::size_of_val(&*blk) as u32)
            .wrapping_add((*blk).size);
        i += 1;
    }

    /* Then proceed with DRAM .data saved before D3. */
    ret = catpt_restore_dram_data(cdev, chan);
    if ret != 0 {
        dev_err((*cdev).dev, c"restore memdumps failed: %d\n".as_ptr(), ret);
    }

    ret
}

unsafe fn catpt_restore_module(
    cdev: *mut catpt_dev,
    chan: *mut dma_chan,
    paddr: dma_addr_t,
    mod_: *mut catpt_fw_module_hdr,
) -> i32 {
    let mut off: u32 = ::core::mem::size_of_val(&*mod_) as u32;
    let mut i: i32;

    print_hex_dump_debug(
        c"catpt_restore_module".as_ptr(),
        DUMP_PREFIX_OFFSET,
        8,
        4,
        mod_ as *const ::core::ffi::c_void,
        ::core::mem::size_of_val(&*mod_),
        false,
    );

    i = 0;
    while i < (*mod_).blocks as i32 {
        let blk: *mut catpt_fw_block_hdr;
        let ret: i32;

        blk = (mod_ as *mut u8).add(off as usize) as *mut catpt_fw_block_hdr;

        match (*blk).ram_type {
            x if x == catpt_ram_type::CATPT_RAM_TYPE_INSTANCE as u32 => {
                /* restore module state */
                ret = catpt_dma_memcpy_todsp(
                    cdev,
                    chan,
                    (*cdev).lpe_base.wrapping_add((*blk).ram_offset as dma_addr_t),
                    (*cdev).dxbuf_paddr.wrapping_add((*blk).ram_offset as dma_addr_t),
                    ALIGN((*blk).size, 4),
                );
            }
            _ => {
                ret = catpt_load_block(cdev, chan, paddr.wrapping_add(off as dma_addr_t), blk, false);
            }
        }

        if ret != 0 {
            dev_err((*cdev).dev, c"restore block failed: %d\n".as_ptr(), ret);
            return ret;
        }

        off = off
            .wrapping_add(::core::mem::size_of_val(&*blk) as u32)
            .wrapping_add((*blk).size);
        i += 1;
    }

    0
}

unsafe fn catpt_load_module(
    cdev: *mut catpt_dev,
    chan: *mut dma_chan,
    paddr: dma_addr_t,
    mod_: *mut catpt_fw_module_hdr,
) -> i32 {
    let type_: *mut catpt_module_type;
    let mut off: u32 = ::core::mem::size_of_val(&*mod_) as u32;
    let mut i: i32;

    print_hex_dump_debug(
        c"catpt_load_module".as_ptr(),
        DUMP_PREFIX_OFFSET,
        8,
        4,
        mod_ as *const ::core::ffi::c_void,
        ::core::mem::size_of_val(&*mod_),
        false,
    );

    type_ = (*cdev).modules.add((*mod_).module_id as usize);

    i = 0;
    while i < (*mod_).blocks as i32 {
        let blk: *mut catpt_fw_block_hdr;
        let ret: i32;

        blk = (mod_ as *mut u8).add(off as usize) as *mut catpt_fw_block_hdr;

        ret = catpt_load_block(cdev, chan, paddr.wrapping_add(off as dma_addr_t), blk, true);
        if ret != 0 {
            dev_err((*cdev).dev, c"load block failed: %d\n".as_ptr(), ret);
            return ret;
        }

        /*
         * Save state window coordinates - these will be
         * used to capture module state on D0 exit.
         */
        if (*blk).ram_type == catpt_ram_type::CATPT_RAM_TYPE_INSTANCE as u32 {
            (*type_).state_offset = (*blk).ram_offset;
            (*type_).state_size = (*blk).size;
        }

        off = off
            .wrapping_add(::core::mem::size_of_val(&*blk) as u32)
            .wrapping_add((*blk).size);
        i += 1;
    }

    /* init module type static info */
    (*type_).loaded = true;
    /* DSP expects address from module header substracted by 4 */
    (*type_).entry_point = (*mod_).entry_point.wrapping_sub(4);
    (*type_).persistent_size = (*mod_).persistent_size;
    (*type_).scratch_size = (*mod_).scratch_size;

    0
}

unsafe fn catpt_restore_firmware(
    cdev: *mut catpt_dev,
    chan: *mut dma_chan,
    paddr: dma_addr_t,
    fw: *mut catpt_fw_hdr,
) -> i32 {
    let mut off: u32 = ::core::mem::size_of_val(&*fw) as u32;
    let mut i: i32;

    print_hex_dump_debug(
        c"catpt_restore_firmware".as_ptr(),
        DUMP_PREFIX_OFFSET,
        8,
        4,
        fw as *const ::core::ffi::c_void,
        ::core::mem::size_of_val(&*fw),
        false,
    );

    i = 0;
    while i < (*fw).modules as i32 {
        let mod_: *mut catpt_fw_module_hdr;
        let ret: i32;

        mod_ = (fw as *mut u8).add(off as usize) as *mut catpt_fw_module_hdr;
        if strncmp(
            (*fw).signature.as_ptr(),
            (*mod_).signature.as_ptr(),
            FW_SIGNATURE_SIZE,
        ) != 0
        {
            dev_err((*cdev).dev, c"module signature mismatch\n".as_ptr());
            return -EINVAL;
        }

        if (*mod_).module_id > CATPT_MODID_LAST {
            return -EINVAL;
        }

        match (*mod_).module_id {
            x if x == CATPT_MODID_BASE_FW => {
                ret = catpt_restore_basefw(cdev, chan, paddr.wrapping_add(off as dma_addr_t), mod_);
            }
            _ => {
                ret = catpt_restore_module(cdev, chan, paddr.wrapping_add(off as dma_addr_t), mod_);
            }
        }

        if ret != 0 {
            dev_err((*cdev).dev, c"restore module failed: %d\n".as_ptr(), ret);
            return ret;
        }

        off = off
            .wrapping_add(::core::mem::size_of_val(&*mod_) as u32)
            .wrapping_add((*mod_).mod_size);
        i += 1;
    }

    0
}

unsafe fn catpt_load_firmware(
    cdev: *mut catpt_dev,
    chan: *mut dma_chan,
    paddr: dma_addr_t,
    fw: *mut catpt_fw_hdr,
) -> i32 {
    let mut off: u32 = ::core::mem::size_of_val(&*fw) as u32;
    let mut i: i32;

    print_hex_dump_debug(
        c"catpt_load_firmware".as_ptr(),
        DUMP_PREFIX_OFFSET,
        8,
        4,
        fw as *const ::core::ffi::c_void,
        ::core::mem::size_of_val(&*fw),
        false,
    );

    i = 0;
    while i < (*fw).modules as i32 {
        let mod_: *mut catpt_fw_module_hdr;
        let ret: i32;

        mod_ = (fw as *mut u8).add(off as usize) as *mut catpt_fw_module_hdr;
        if strncmp(
            (*fw).signature.as_ptr(),
            (*mod_).signature.as_ptr(),
            FW_SIGNATURE_SIZE,
        ) != 0
        {
            dev_err((*cdev).dev, c"module signature mismatch\n".as_ptr());
            return -EINVAL;
        }

        if (*mod_).module_id > CATPT_MODID_LAST {
            return -EINVAL;
        }

        ret = catpt_load_module(cdev, chan, paddr.wrapping_add(off as dma_addr_t), mod_);
        if ret != 0 {
            dev_err((*cdev).dev, c"load module failed: %d\n".as_ptr(), ret);
            return ret;
        }

        off = off
            .wrapping_add(::core::mem::size_of_val(&*mod_) as u32)
            .wrapping_add((*mod_).mod_size);
        i += 1;
    }

    0
}

unsafe fn catpt_request_load_firmware(
    cdev: *mut catpt_dev,
    chan: *mut dma_chan,
    name: *const ::core::ffi::c_char,
    restore: bool,
) -> i32 {
    let mut fw: *mut catpt_fw_hdr;
    let mut paddr: dma_addr_t = 0;
    let vaddr: *mut ::core::ffi::c_void;
    let mut ret: i32;

    let mut img: *const firmware = ::core::ptr::null();
    ret = request_firmware(&mut img, name, (*cdev).dev);
    if ret != 0 {
        return ret;
    }

    fw = (*img).data as *mut catpt_fw_hdr;
    if strncmp(
        (*fw).signature.as_ptr(),
        FW_SIGNATURE.as_ptr() as *const ::core::ffi::c_char,
        FW_SIGNATURE_SIZE,
    ) != 0
    {
        dev_err((*cdev).dev, c"firmware signature mismatch\n".as_ptr());
        release_firmware(img);
        return -EINVAL;
    }

    vaddr = dma_alloc_coherent((*cdev).dev, (*img).size, &mut paddr, GFP_KERNEL);
    if vaddr.is_null() {
        release_firmware(img);
        return -ENOMEM;
    }

    memcpy(vaddr, (*img).data as *const ::core::ffi::c_void, (*img).size);
    fw = vaddr as *mut catpt_fw_hdr;
    if restore {
        ret = catpt_restore_firmware(cdev, chan, paddr, fw);
    } else {
        ret = catpt_load_firmware(cdev, chan, paddr, fw);
    }

    dma_free_coherent((*cdev).dev, (*img).size, vaddr, paddr);
    release_firmware(img);
    ret
}

unsafe fn catpt_request_dma_load_firmware(cdev: *mut catpt_dev, restore: bool) -> i32 {
    let chan: *mut dma_chan;
    let mut ret: i32;

    chan = catpt_dma_request_config_chan(cdev);
    if IS_ERR(chan as *const ::core::ffi::c_void) {
        return PTR_ERR(chan as *const ::core::ffi::c_void);
    }

    ret = catpt_request_load_firmware(cdev, chan, (*(*cdev).spec).fw_name, restore);
    if ret != 0 {
        dma_release_channel(chan);
        return ret;
    }

    if !restore {
        dma_release_channel(chan);
        return ret;
    }
    ret = catpt_restore_streams_context(cdev, chan);
    if ret != 0 {
        dev_err((*cdev).dev, c"restore streams ctx failed: %d\n".as_ptr(), ret);
    }
    dma_release_channel(chan);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn catpt_boot_firmware(cdev: *mut catpt_dev, restore: bool) -> i32 {
    let mut ret: i32;

    catpt_dsp_stall(cdev, true);

    ret = catpt_request_dma_load_firmware(cdev, restore);
    if ret != 0 {
        dev_err((*cdev).dev, c"load binaries failed: %d\n".as_ptr(), ret);
        return ret;
    }

    reinit_completion(&mut (*cdev).fw_ready);
    catpt_dsp_stall(cdev, false);

    ret = wait_for_completion_timeout(
        &mut (*cdev).fw_ready,
        msecs_to_jiffies(FW_READY_TIMEOUT_MS),
    ) as i32;
    if ret == 0 {
        dev_err((*cdev).dev, c"firmware ready timeout\n".as_ptr());
        return -ETIMEDOUT;
    }
    /* Wake up does not mean FW is ready, an exception could occur. */
    if !(*cdev).ipc.ready {
        return -EREMOTEIO;
    }

    /* update sram pg & clock once done booting */
    catpt_dsp_update_srampge(cdev, &mut (*cdev).dram, (*(*cdev).spec).dram_mask);
    catpt_dsp_update_srampge(cdev, &mut (*cdev).iram, (*(*cdev).spec).iram_mask);

    catpt_dsp_update_lpclock(cdev)
}

#[no_mangle]
pub unsafe extern "C" fn catpt_first_boot_firmware(cdev: *mut catpt_dev) -> i32 {
    let mut res: *mut resource;
    let mut ret: i32;

    ret = catpt_boot_firmware(cdev, false);
    if ret != 0 {
        dev_err((*cdev).dev, c"basefw boot failed: %d\n".as_ptr(), ret);
        return ret;
    }

    /* restrict FW Core dump area */
    __request_region(&mut (*cdev).dram, 0, 0x200, ::core::ptr::null(), 0);
    /* restrict entire area following BASE_FW - highest offset in DRAM */
    res = (*cdev).dram.child;
    while !(*res).sibling.is_null() {
        res = (*res).sibling;
    }
    __request_region(
        &mut (*cdev).dram,
        (*res).end.wrapping_add(1),
        (*cdev).dram.end.wrapping_sub((*res).end),
        ::core::ptr::null(),
        0,
    );

    ret = catpt_ipc_get_mixer_stream_info(cdev, &mut (*cdev).mixer);
    if ret != 0 {
        return CATPT_IPC_RET(ret);
    }

    ret = catpt_arm_stream_templates(cdev);
    if ret != 0 {
        dev_err((*cdev).dev, c"arm templates failed: %d\n".as_ptr(), ret);
        return ret;
    }

    /* update dram pg for scratch and restricted regions */
    catpt_dsp_update_srampge(cdev, &mut (*cdev).dram, (*(*cdev).spec).dram_mask);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
