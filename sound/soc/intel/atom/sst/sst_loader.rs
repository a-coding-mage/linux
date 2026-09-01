// SPDX-License-Identifier: GPL-2.0-only
/*
 *  sst_dsp.c - Intel SST Driver for audio engine
 *
 *  Copyright (C) 2008-14	Intel Corp
 *  Authors:	Vinod Koul <vinod.koul@intel.com>
 *		Harsha Priya <priya.harsha@intel.com>
 *		Dharageswari R <dharageswari.r@intel.com>
 *		KP Jeeja <jeeja.kp@intel.com>
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 *  This file contains all dsp controlling functions like firmware download,
 * setting/resetting dsp cores, etc
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type u32 = u32;
type u64 = u64;
type bool_ = bool;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EAGAIN: c_int = 11;
const EBUSY: c_int = 16;
const GFP_KERNEL: c_int = 0;
const PM_QOS_DEFAULT_VALUE: c_int = -1;

extern "C" {
    static SST_CSR: u32;
    static SST_FW_SIGN: [c_char; 4];
    static SST_IRAM: u32;
    static SST_DRAM: u32;
    static SST_DDR: u32;
    static SST_CUSTOM_INFO: u32;
    static SST_RESET: c_int;
    static SST_FW_LOADING: c_int;
    static SST_FW_RUNNING: c_int;
    static FW_DWNL_ID: u32;
    static MRFLD_FW_DDR_BASE_OFFSET: usize;
    static MRFLD_FW_BSS_RESET_BIT: u32;
    static MRFLD_FW_FEATURE_BASE_OFFSET: usize;

    fn __iowrite32_copy(dst: *mut c_void, src: *const c_void, count: c_int);
    fn __ioread32_copy(dst: *mut c_void, src: *const c_void, count: c_int);
    fn sst_shim_read64(shim: *mut c_void, offset: u32) -> u64;
    fn sst_shim_write64(shim: *mut c_void, offset: u32, value: u64);
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn virt_to_phys(ptr: *const c_void) -> c_ulong;
    fn release_firmware(fw: *const firmware);
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut c_void) -> c_int;
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn cpu_latency_qos_update_request(qos: *mut c_void, value: c_int);
    fn sst_create_block(ctx: *mut intel_sst_drv, id: c_int, msg_id: u32) -> *mut sst_block;
    fn sst_wait_timeout(ctx: *mut intel_sst_drv, block: *mut sst_block) -> c_int;
    fn sst_free_block(ctx: *mut intel_sst_drv, block: *mut sst_block);

    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct firmware {
    pub size: usize,
    pub data: *const c_void,
}

#[repr(C)]
pub struct sst_fw_header {
    pub signature: [c_char; 4],
    pub file_size: u32,
    pub modules: u32,
    pub file_format: u32,
}

#[repr(C)]
pub struct fw_module_header {
    pub signature: [c_char; 4],
    pub mod_size: u32,
    pub blocks: u32,
    pub type_: u32,
    pub entry_point: u32,
}

#[repr(C)]
pub struct fw_block_info {
    pub type_: u32,
    pub size: u32,
    pub ram_offset: usize,
}

#[repr(C)]
pub struct sst_memcpy_list {
    pub dstn: *mut c_void,
    pub src: *const c_void,
    pub size: u32,
    pub is_io: bool_,
    pub memcpylist: list_head,
}

#[repr(C)]
pub struct intel_sst_ops {
    pub reset: Option<unsafe extern "C" fn(*mut intel_sst_drv) -> c_int>,
    pub post_download: Option<unsafe extern "C" fn(*mut intel_sst_drv)>,
    pub start: Option<unsafe extern "C" fn(*mut intel_sst_drv) -> c_int>,
    pub restore_dsp_context: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct intel_sst_drv {
    pub dev: *mut c_void,
    pub shim: *mut c_void,
    pub fw_in_mem: *mut c_void,
    pub iram: *mut c_void,
    pub dram: *mut c_void,
    pub ddr: *mut c_void,
    pub ddr_base: u32,
    pub memcpy_list: list_head,
    pub sst_lock: c_void,
    pub sst_state: c_int,
    pub firmware_name: *const c_char,
    pub qos: *mut c_void,
    pub ops: *mut intel_sst_ops,
}

#[repr(C)]
pub struct sst_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct config_status_reg_mrfld_part {
    pub xt_snoop: u64,
}

#[repr(C)]
pub union config_status_reg_mrfld {
    pub full: u64,
    pub part: config_status_reg_mrfld_part,
}

unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    let prev = (*head).prev;
    (*new).next = head;
    (*new).prev = prev;
    (*prev).next = new;
    (*head).prev = new;
}

unsafe fn list_del(entry: *mut list_head) {
    let prev = (*entry).prev;
    let next = (*entry).next;
    (*next).prev = prev;
    (*prev).next = next;
}

unsafe fn list_entry_sst_memcpy_list(ptr: *mut list_head) -> *mut sst_memcpy_list {
    (ptr as *mut u8).sub(offset_of!(sst_memcpy_list, memcpylist)) as *mut sst_memcpy_list
}

#[no_mangle]
pub unsafe extern "C" fn memcpy32_toio(dst: *mut c_void, src: *const c_void, count: c_int) {
    /*
     * __iowrite32_copy uses 32-bit count values so divide by 4 for
     * right count in words
     */
    __iowrite32_copy(dst, src, count / 4);
}

#[no_mangle]
pub unsafe extern "C" fn memcpy32_fromio(dst: *mut c_void, src: *const c_void, count: c_int) {
    /*
     * __ioread32_copy uses 32-bit count values so divide by 4 for
     * right count in words
     */
    __ioread32_copy(dst, src, count / 4);
}

/**
 * intel_sst_reset_dsp_mrfld - Resetting SST DSP
 * @sst_drv_ctx: intel_sst_drv context pointer
 *
 * This resets DSP in case of MRFLD platfroms
 */
#[no_mangle]
pub unsafe extern "C" fn intel_sst_reset_dsp_mrfld(sst_drv_ctx: *mut intel_sst_drv) -> c_int {
    let mut csr: config_status_reg_mrfld = config_status_reg_mrfld { full: 0 };

    dev_dbg((*sst_drv_ctx).dev, c"sst: Resetting the DSP in mrfld\n".as_ptr());
    csr.full = sst_shim_read64((*sst_drv_ctx).shim, SST_CSR);

    dev_dbg((*sst_drv_ctx).dev, c"value:0x%llx\n".as_ptr(), csr.full);

    csr.full |= 0x7;
    sst_shim_write64((*sst_drv_ctx).shim, SST_CSR, csr.full);
    csr.full = sst_shim_read64((*sst_drv_ctx).shim, SST_CSR);

    dev_dbg((*sst_drv_ctx).dev, c"value:0x%llx\n".as_ptr(), csr.full);

    csr.full &= !0x1;
    sst_shim_write64((*sst_drv_ctx).shim, SST_CSR, csr.full);

    csr.full = sst_shim_read64((*sst_drv_ctx).shim, SST_CSR);
    dev_dbg((*sst_drv_ctx).dev, c"value:0x%llx\n".as_ptr(), csr.full);
    0
}

/**
 * sst_start_mrfld - Start the SST DSP processor
 * @sst_drv_ctx: intel_sst_drv context pointer
 *
 * This starts the DSP in MERRIFIELD platfroms
 */
#[no_mangle]
pub unsafe extern "C" fn sst_start_mrfld(sst_drv_ctx: *mut intel_sst_drv) -> c_int {
    let mut csr: config_status_reg_mrfld = config_status_reg_mrfld { full: 0 };

    dev_dbg((*sst_drv_ctx).dev, c"sst: Starting the DSP in mrfld LALALALA\n".as_ptr());
    csr.full = sst_shim_read64((*sst_drv_ctx).shim, SST_CSR);
    dev_dbg((*sst_drv_ctx).dev, c"value:0x%llx\n".as_ptr(), csr.full);

    csr.full |= 0x7;
    sst_shim_write64((*sst_drv_ctx).shim, SST_CSR, csr.full);

    csr.full = sst_shim_read64((*sst_drv_ctx).shim, SST_CSR);
    dev_dbg((*sst_drv_ctx).dev, c"value:0x%llx\n".as_ptr(), csr.full);

    csr.part.xt_snoop = 1;
    csr.full &= !0x5;
    sst_shim_write64((*sst_drv_ctx).shim, SST_CSR, csr.full);

    csr.full = sst_shim_read64((*sst_drv_ctx).shim, SST_CSR);
    dev_dbg(
        (*sst_drv_ctx).dev,
        c"sst: Starting the DSP_merrifield:%llx\n".as_ptr(),
        csr.full,
    );
    0
}

unsafe fn sst_validate_fw_image(
    ctx: *mut intel_sst_drv,
    size: c_ulong,
    module: *mut *mut fw_module_header,
    num_modules: *mut u32,
) -> c_int {
    let header: *mut sst_fw_header;
    let sst_fw_in_mem: *const c_void = (*ctx).fw_in_mem;

    dev_dbg((*ctx).dev, c"Enter\n".as_ptr());

    /* Read the header information from the data pointer */
    header = sst_fw_in_mem as *mut sst_fw_header;
    dev_dbg(
        (*ctx).dev,
        c"header sign=%s size=%x modules=%x fmt=%x size=%zx\n".as_ptr(),
        (*header).signature.as_ptr(),
        (*header).file_size,
        (*header).modules,
        (*header).file_format,
        size_of::<sst_fw_header>(),
    );

    /* verify FW */
    if strncmp((*header).signature.as_ptr(), SST_FW_SIGN.as_ptr(), 4) != 0
        || size != ((*header).file_size as c_ulong + size_of::<sst_fw_header>() as c_ulong)
    {
        /* Invalid FW signature */
        dev_err((*ctx).dev, c"InvalidFW sign/filesize mismatch\n".as_ptr());
        return -EINVAL;
    }
    *num_modules = (*header).modules;
    *module = (sst_fw_in_mem as *mut u8).add(size_of::<sst_fw_header>()) as *mut fw_module_header;

    0
}

/*
 * sst_fill_memcpy_list - Fill the memcpy list
 *
 * @memcpy_list: List to be filled
 * @destn: Destination addr to be filled in the list
 * @src: Source addr to be filled in the list
 * @size: Size to be filled in the list
 *
 * Adds the node to the list after required fields
 * are populated in the node
 */
unsafe fn sst_fill_memcpy_list(
    memcpy_list: *mut list_head,
    destn: *mut c_void,
    src: *const c_void,
    size: u32,
    is_io: bool_,
) -> c_int {
    let listnode: *mut sst_memcpy_list;

    listnode = kzalloc(size_of::<sst_memcpy_list>(), GFP_KERNEL) as *mut sst_memcpy_list;
    if listnode.is_null() {
        return -ENOMEM;
    }
    (*listnode).dstn = destn;
    (*listnode).src = src;
    (*listnode).size = size;
    (*listnode).is_io = is_io;
    list_add_tail(&mut (*listnode).memcpylist, memcpy_list);

    0
}

/**
 * sst_parse_module_memcpy - Parse audio FW modules and populate the memcpy list
 *
 * @sst_drv_ctx		: driver context
 * @module		: FW module header
 * @memcpy_list	: Pointer to the list to be populated
 * Create the memcpy list as the number of block to be copied
 * returns error or 0 if module sizes are proper
 */
unsafe fn sst_parse_module_memcpy(
    sst_drv_ctx: *mut intel_sst_drv,
    module: *mut fw_module_header,
    memcpy_list: *mut list_head,
) -> c_int {
    let mut block: *mut fw_block_info;
    let mut count: u32;
    let mut ret_val: c_int = 0;
    let mut ram_iomem: *mut c_void = ptr::null_mut();

    dev_dbg(
        (*sst_drv_ctx).dev,
        c"module sign %s size %x blocks %x type %x\n".as_ptr(),
        (*module).signature.as_ptr(),
        (*module).mod_size,
        (*module).blocks,
        (*module).type_,
    );
    dev_dbg(
        (*sst_drv_ctx).dev,
        c"module entrypoint 0x%x\n".as_ptr(),
        (*module).entry_point,
    );

    block = (module as *mut u8).add(size_of::<fw_module_header>()) as *mut fw_block_info;

    count = 0;
    while count < (*module).blocks {
        if (*block).size <= 0 {
            dev_err((*sst_drv_ctx).dev, c"block size invalid\n".as_ptr());
            return -EINVAL;
        }
        if (*block).type_ == SST_IRAM {
            ram_iomem = (*sst_drv_ctx).iram;
        } else if (*block).type_ == SST_DRAM {
            ram_iomem = (*sst_drv_ctx).dram;
        } else if (*block).type_ == SST_DDR {
            ram_iomem = (*sst_drv_ctx).ddr;
        } else if (*block).type_ == SST_CUSTOM_INFO {
            block = (block as *mut u8)
                .add(size_of::<fw_block_info>())
                .add((*block).size as usize) as *mut fw_block_info;
            count += 1;
            continue;
        } else {
            dev_err(
                (*sst_drv_ctx).dev,
                c"wrong ram type0x%x in block0x%x\n".as_ptr(),
                (*block).type_,
                count,
            );
            return -EINVAL;
        }

        ret_val = sst_fill_memcpy_list(
            memcpy_list,
            (ram_iomem as *mut u8).add((*block).ram_offset) as *mut c_void,
            (block as *mut u8).add(size_of::<fw_block_info>()) as *const c_void,
            (*block).size,
            true,
        );
        if ret_val != 0 {
            return ret_val;
        }

        block = (block as *mut u8)
            .add(size_of::<fw_block_info>())
            .add((*block).size as usize) as *mut fw_block_info;
        count += 1;
    }
    0
}

/**
 * sst_parse_fw_memcpy - parse the firmware image & populate the list for memcpy
 *
 * @ctx			: pointer to drv context
 * @size		: size of the firmware
 * @fw_list		: pointer to list_head to be populated
 * This function parses the FW image and saves the parsed image in the list
 * for memcpy
 */
unsafe fn sst_parse_fw_memcpy(
    ctx: *mut intel_sst_drv,
    size: c_ulong,
    fw_list: *mut list_head,
) -> c_int {
    let mut module: *mut fw_module_header = ptr::null_mut();
    let mut count: u32;
    let mut num_modules: u32 = 0;
    let mut ret_val: c_int;

    ret_val = sst_validate_fw_image(ctx, size, &mut module, &mut num_modules);
    if ret_val != 0 {
        return ret_val;
    }

    count = 0;
    while count < num_modules {
        ret_val = sst_parse_module_memcpy(ctx, module, fw_list);
        if ret_val != 0 {
            return ret_val;
        }
        module = (module as *mut u8)
            .add(size_of::<fw_module_header>())
            .add((*module).mod_size as usize) as *mut fw_module_header;
        count += 1;
    }

    0
}

/**
 * sst_do_memcpy - function initiates the memcpy
 *
 * @memcpy_list: Pter to memcpy list on which the memcpy needs to be initiated
 *
 * Triggers the memcpy
 */
unsafe fn sst_do_memcpy(memcpy_list: *mut list_head) {
    let mut pos = (*memcpy_list).next;

    while pos != memcpy_list {
        let listnode = list_entry_sst_memcpy_list(pos);
        if (*listnode).is_io {
            memcpy32_toio((*listnode).dstn, (*listnode).src, (*listnode).size as c_int);
        } else {
            memcpy(
                (*listnode).dstn,
                (*listnode).src,
                (*listnode).size as usize,
            );
        }
        pos = (*pos).next;
    }
}

#[no_mangle]
pub unsafe extern "C" fn sst_memcpy_free_resources(sst_drv_ctx: *mut intel_sst_drv) {
    let head = &mut (*sst_drv_ctx).memcpy_list as *mut list_head;
    let mut pos = (*head).next;

    /* Free the list */
    while pos != head {
        let listnode = list_entry_sst_memcpy_list(pos);
        let next = (*pos).next;
        list_del(&mut (*listnode).memcpylist);
        kfree(listnode as *mut c_void);
        pos = next;
    }
}

unsafe fn sst_cache_and_parse_fw(sst: *mut intel_sst_drv, fw: *const firmware) -> c_int {
    let mut retval: c_int = 0;

    (*sst).fw_in_mem = kzalloc((*fw).size, GFP_KERNEL);
    if (*sst).fw_in_mem.is_null() {
        retval = -ENOMEM;
        release_firmware(fw);
        return retval;
    }
    dev_dbg((*sst).dev, c"copied fw to %p".as_ptr(), (*sst).fw_in_mem);
    dev_dbg(
        (*sst).dev,
        c"phys: %lx".as_ptr(),
        virt_to_phys((*sst).fw_in_mem),
    );
    memcpy((*sst).fw_in_mem, (*fw).data, (*fw).size);
    retval = sst_parse_fw_memcpy(sst, (*fw).size as c_ulong, &mut (*sst).memcpy_list);
    if retval != 0 {
        dev_err((*sst).dev, c"Failed to parse fw\n".as_ptr());
        kfree((*sst).fw_in_mem);
        (*sst).fw_in_mem = ptr::null_mut();
    }

    release_firmware(fw);
    retval
}

#[no_mangle]
pub unsafe extern "C" fn sst_firmware_load_cb(fw: *const firmware, context: *mut c_void) {
    let ctx = context as *mut intel_sst_drv;

    dev_dbg((*ctx).dev, c"Enter\n".as_ptr());

    if fw.is_null() {
        dev_err((*ctx).dev, c"request fw failed\n".as_ptr());
        return;
    }

    mutex_lock(&mut (*ctx).sst_lock as *mut c_void);

    if (*ctx).sst_state != SST_RESET || !(*ctx).fw_in_mem.is_null() {
        release_firmware(fw);
        mutex_unlock(&mut (*ctx).sst_lock as *mut c_void);
        return;
    }

    dev_dbg((*ctx).dev, c"Request Fw completed\n".as_ptr());
    sst_cache_and_parse_fw(ctx, fw);
    mutex_unlock(&mut (*ctx).sst_lock as *mut c_void);
}

/*
 * sst_request_fw - requests audio fw from kernel and saves a copy
 *
 * This function requests the SST FW from the kernel, parses it and
 * saves a copy in the driver context
 */
unsafe fn sst_request_fw(sst: *mut intel_sst_drv) -> c_int {
    let mut retval: c_int = 0;
    let mut fw: *const firmware = ptr::null();

    retval = request_firmware(&mut fw, (*sst).firmware_name, (*sst).dev);
    if retval != 0 {
        dev_err((*sst).dev, c"request fw failed %d\n".as_ptr(), retval);
        return retval;
    }
    if fw.is_null() {
        dev_err((*sst).dev, c"fw is returning as null\n".as_ptr());
        return -EINVAL;
    }
    mutex_lock(&mut (*sst).sst_lock as *mut c_void);
    retval = sst_cache_and_parse_fw(sst, fw);
    mutex_unlock(&mut (*sst).sst_lock as *mut c_void);

    retval
}

/*
 * Writing the DDR physical base to DCCM offset
 * so that FW can use it to setup TLB
 */
unsafe fn sst_dccm_config_write(dram_base: *mut c_void, ddr_base: c_uint) {
    let mut addr: *mut c_void;
    let mut bss_reset: u32 = 0;

    addr = (dram_base as *mut u8).add(MRFLD_FW_DDR_BASE_OFFSET) as *mut c_void;
    memcpy32_toio(
        addr,
        &ddr_base as *const c_uint as *const c_void,
        size_of::<u32>() as c_int,
    );
    bss_reset |= 1 << MRFLD_FW_BSS_RESET_BIT;
    addr = (dram_base as *mut u8).add(MRFLD_FW_FEATURE_BASE_OFFSET) as *mut c_void;
    memcpy32_toio(
        addr,
        &bss_reset as *const u32 as *const c_void,
        size_of::<u32>() as c_int,
    );
}

type c_uint = u32;

#[no_mangle]
pub unsafe extern "C" fn sst_post_download_mrfld(ctx: *mut intel_sst_drv) {
    sst_dccm_config_write((*ctx).dram, (*ctx).ddr_base);
    dev_dbg((*ctx).dev, c"config written to DCCM\n".as_ptr());
}

/**
 * sst_load_fw - function to load FW into DSP
 * @sst_drv_ctx: intel_sst_drv context pointer
 *
 * Transfers the FW to DSP using dma/memcpy
 */
#[no_mangle]
pub unsafe extern "C" fn sst_load_fw(sst_drv_ctx: *mut intel_sst_drv) -> c_int {
    let mut ret_val: c_int = 0;
    let block: *mut sst_block;

    dev_dbg((*sst_drv_ctx).dev, c"sst_load_fw\n".as_ptr());

    if (*sst_drv_ctx).sst_state != SST_RESET {
        return -EAGAIN;
    }

    if (*sst_drv_ctx).fw_in_mem.is_null() {
        dev_dbg(
            (*sst_drv_ctx).dev,
            c"sst: FW not in memory retry to download\n".as_ptr(),
        );
        ret_val = sst_request_fw(sst_drv_ctx);
        if ret_val != 0 {
            return ret_val;
        }
    }

    block = sst_create_block(sst_drv_ctx, 0, FW_DWNL_ID);
    if block.is_null() {
        return -ENOMEM;
    }

    /* Prevent C-states beyond C6 */
    cpu_latency_qos_update_request((*sst_drv_ctx).qos, 0);

    (*sst_drv_ctx).sst_state = SST_FW_LOADING;

    ret_val = ((*(*sst_drv_ctx).ops).reset.unwrap())(sst_drv_ctx);
    if ret_val == 0 {
        sst_do_memcpy(&mut (*sst_drv_ctx).memcpy_list);

        /* Write the DRAM/DCCM config before enabling FW */
        if let Some(post_download) = (*(*sst_drv_ctx).ops).post_download {
            post_download(sst_drv_ctx);
        }

        /* bring sst out of reset */
        ret_val = ((*(*sst_drv_ctx).ops).start.unwrap())(sst_drv_ctx);
        if ret_val == 0 {
            ret_val = sst_wait_timeout(sst_drv_ctx, block);
            if ret_val != 0 {
                dev_err(
                    (*sst_drv_ctx).dev,
                    c"fw download failed %d\n".as_ptr(),
                    ret_val,
                );
                /* FW download failed due to timeout */
                ret_val = -EBUSY;
            }
        }
    }

    /* Re-enable Deeper C-states beyond C6 */
    cpu_latency_qos_update_request((*sst_drv_ctx).qos, PM_QOS_DEFAULT_VALUE);
    sst_free_block(sst_drv_ctx, block);
    dev_dbg((*sst_drv_ctx).dev, c"fw load successful!!!\n".as_ptr());

    if let Some(restore_dsp_context) = (*(*sst_drv_ctx).ops).restore_dsp_context {
        restore_dsp_context();
    }
    (*sst_drv_ctx).sst_state = SST_FW_RUNNING;
    ret_val
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
