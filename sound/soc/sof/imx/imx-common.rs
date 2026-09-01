// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Copyright 2020-2025 NXP
//
// Common helpers for the audio DSP on i.MX8

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;
type size_t = usize;

const EXCEPT_MAX_HDR_SIZE: u32 = 0;
const IMX8_STACK_DUMP_SIZE: usize = 0;
const DSP_MU_CHAN_NUM: c_int = 0;
const SOF_IPC_PANIC_MAGIC_MASK: u32 = 0;
const SOF_IPC_PANIC_MAGIC: u32 = 0;
const SOF_FW_BLK_TYPE_IRAM: c_int = 0;
const SOF_FW_BLK_TYPE_DRAM: c_int = 0;
const SOF_FW_BLK_TYPE_SRAM: c_int = 0;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const EPROBE_DEFER: c_int = 517;
const IORESOURCE_MEM: c_uint = 0;
const GFP_KERNEL: c_uint = 0;
const PLATFORM_DEVID_NONE: c_int = -1;
const PD_FLAG_DEV_LINK_ON: c_uint = 0;
const SOF_DSP_PM_D0: c_uint = 0;
const SOF_DSP_PM_D3: c_uint = 3;
const KERN_ERR: *const c_char = ptr::null();
const SNDRV_PCM_INFO_MMAP: u64 = 0;
const SNDRV_PCM_INFO_MMAP_VALID: u64 = 0;
const SNDRV_PCM_INFO_INTERLEAVED: u64 = 0;
const SNDRV_PCM_INFO_PAUSE: u64 = 0;
const SNDRV_PCM_INFO_BATCH: u64 = 0;
const SNDRV_PCM_INFO_NO_PERIOD_WAKEUP: u64 = 0;

#[repr(C)]
pub struct device {
    pm_domain: *mut c_void,
    of_node: *mut c_void,
}

#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_dsp_power_state {
    state: c_uint,
}

#[repr(C)]
pub struct snd_sof_pdata {
    hw_pdata: *mut c_void,
}

#[repr(C)]
pub struct sof_box {
    offset: u32,
}

#[repr(C)]
pub struct snd_sof_dev {
    dsp_oops_offset: u32,
    dev: *mut device,
    debug_box: sof_box,
    ipc_lock: c_void,
    pdata: *mut snd_sof_pdata,
    host_box: sof_box,
    dsp_power_state: sof_dsp_power_state,
    bar: [*mut c_void; 16],
    num_cores: c_uint,
    mailbox_bar: c_int,
    dsp_box: sof_box,
}

#[repr(C)]
pub struct sof_ipc_arch_hdr {
    totalsize: u32,
}

#[repr(C)]
pub struct sof_ipc_dsp_oops_xtensa {
    arch_hdr: sof_ipc_arch_hdr,
}

#[repr(C)]
pub struct sof_ipc_panic_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_ipc_msg {
    msg_data: *mut c_void,
    msg_size: size_t,
}

#[repr(C)]
pub struct imx_dsp_ipc {
    ops: *const imx_dsp_ops,
}

#[repr(C)]
pub struct imx_dsp_ops {
    handle_reply: Option<unsafe extern "C" fn(*mut imx_dsp_ipc)>,
    handle_request: Option<unsafe extern "C" fn(*mut imx_dsp_ipc)>,
}

#[repr(C)]
pub struct imx_common_data {
    clk_num: c_int,
    clks: *mut c_void,
    ipc_dev: *mut platform_device,
    ipc_handle: *mut imx_dsp_ipc,
    pd_list: *mut c_void,
}

#[repr(C)]
pub struct imx_ipc_info {
    has_panic_code: bool,
    boot_mbox_offset: c_int,
    window_offset: c_int,
}

#[repr(C)]
pub struct imx_memory_desc {
    name: *const c_char,
    reserved: bool,
}

#[repr(C)]
pub struct imx_chip_info {
    ipc_info: imx_ipc_info,
    memory: *const imx_memory_desc,
    has_dma_reserved: bool,
}

#[repr(C)]
pub struct dev_pm_domain_attach_data {
    pd_names: *mut *const c_char,
    pd_flags: c_uint,
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    probe: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
    run: Option<unsafe extern "C" fn()>,
    reset: Option<unsafe extern "C" fn()>,
    block_read: Option<unsafe extern "C" fn()>,
    block_write: Option<unsafe extern "C" fn()>,
    mailbox_read: Option<unsafe extern "C" fn()>,
    mailbox_write: Option<unsafe extern "C" fn()>,
    send_msg: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_ipc_msg) -> c_int>,
    get_mailbox_offset: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    get_window_offset: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32) -> c_int>,
    ipc_msg_data: Option<unsafe extern "C" fn()>,
    set_stream_data_offset: Option<unsafe extern "C" fn()>,
    get_bar_index: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32) -> c_int>,
    load_firmware: Option<unsafe extern "C" fn()>,
    debugfs_add_region_item: Option<unsafe extern "C" fn()>,
    pcm_open: Option<unsafe extern "C" fn()>,
    pcm_close: Option<unsafe extern "C" fn()>,
    runtime_suspend: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    runtime_resume: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut snd_sof_dev, c_uint) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    set_power_state:
        Option<unsafe extern "C" fn(*mut snd_sof_dev, *const sof_dsp_power_state) -> c_int>,
    hw_info: u64,
}

unsafe extern "C" {
    fn sof_mailbox_read(sdev: *mut snd_sof_dev, offset: u32, dest: *mut c_void, bytes: size_t);
    fn sof_mailbox_write(sdev: *mut snd_sof_dev, offset: u32, src: *mut c_void, bytes: size_t);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn sof_print_oops_and_stack(
        sdev: *mut snd_sof_dev,
        level: *const c_char,
        panic_code: u32,
        tracep_code: u32,
        xoops: *mut sof_ipc_dsp_oops_xtensa,
        panic_info: *mut sof_ipc_panic_info,
        stack: *mut u32,
        stack_words: size_t,
    );
    fn imx_dsp_get_data(ipc: *mut imx_dsp_ipc) -> *mut snd_sof_dev;
    fn snd_sof_ipc_process_reply(sdev: *mut snd_sof_dev, msg_id: c_int);
    fn get_chip_info(sdev: *mut snd_sof_dev) -> *const imx_chip_info;
    fn snd_sof_dsp_panic(sdev: *mut snd_sof_dev, panic_code: u32, non_recoverable: bool);
    fn snd_sof_ipc_msgs_rx(sdev: *mut snd_sof_dev);
    fn imx_dsp_ring_doorbell(ipc: *mut imx_dsp_ipc, channel: c_int);
    fn clk_bulk_prepare_enable(num_clks: c_int, clks: *mut c_void) -> c_int;
    fn imx_dsp_request_channel(ipc: *mut imx_dsp_ipc, channel: c_int) -> c_int;
    fn imx_chip_core_shutdown(sdev: *mut snd_sof_dev) -> c_int;
    fn imx_dsp_free_channel(ipc: *mut imx_dsp_ipc, channel: c_int);
    fn clk_bulk_disable_unprepare(num_clks: c_int, clks: *mut c_void);
    fn snd_sof_dsp_set_power_state(
        sdev: *mut snd_sof_dev,
        target_state: *const sof_dsp_power_state,
    ) -> c_int;
    fn pm_runtime_suspended(dev: *mut device) -> bool;
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_idle(dev: *mut device);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn to_platform_device(dev: *mut device) -> *mut platform_device;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn platform_get_resource_byname(
        pdev: *mut platform_device,
        ty: c_uint,
        name: *const c_char,
    ) -> *mut resource;
    fn of_reserved_mem_region_to_resource_byname(
        node: *mut c_void,
        name: *const c_char,
        res: *mut resource,
    ) -> c_int;
    fn devm_ioremap_resource(dev: *mut device, res: *mut resource) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn of_reserved_mem_device_release(dev: *mut device);
    fn platform_device_unregister(pdev: *mut platform_device);
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn platform_device_register_data(
        parent: *mut device,
        name: *const c_char,
        id: c_int,
        data: *const c_void,
        size: size_t,
    ) -> *mut platform_device;
    fn of_reserved_mem_device_init_by_name(
        dev: *mut device,
        node: *mut c_void,
        name: *const c_char,
    ) -> c_int;
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: unsafe extern "C" fn(*mut c_void),
        data: *mut c_void,
    ) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut imx_dsp_ipc;
    fn devm_pm_domain_attach_list(
        dev: *mut device,
        data: *const dev_pm_domain_attach_data,
        list: *mut *mut c_void,
    ) -> c_int;
    fn devm_clk_bulk_get_all(dev: *mut device, clks: *mut *mut c_void) -> c_int;
    fn imx_dsp_set_data(ipc: *mut imx_dsp_ipc, data: *mut snd_sof_dev);
    fn imx_chip_probe(sdev: *mut snd_sof_dev) -> c_int;
    fn imx_chip_core_kick();
    fn imx_chip_core_reset();
    fn sof_block_read();
    fn sof_block_write();
    fn sof_ipc_msg_data();
    fn sof_set_stream_data_offset();
    fn snd_sof_load_firmware_memcpy();
    fn snd_sof_debugfs_add_region_item_iomem();
    fn sof_stream_pcm_open();
    fn sof_stream_pcm_close();
}

/**
 * imx8_get_registers() - This function is called in case of DSP oops
 * in order to gather information about the registers, filename and
 * linenumber and stack.
 * @sdev: SOF device
 * @xoops: Stores information about registers.
 * @panic_info: Stores information about filename and line number.
 * @stack: Stores the stack dump.
 * @stack_words: Size of the stack dump.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn imx8_get_registers(
    sdev: *mut snd_sof_dev,
    xoops: *mut sof_ipc_dsp_oops_xtensa,
    panic_info: *mut sof_ipc_panic_info,
    stack: *mut u32,
    stack_words: size_t,
) {
    let mut offset: u32 = (*sdev).dsp_oops_offset;

    /* first read registers */
    sof_mailbox_read(
        sdev,
        offset,
        xoops as *mut c_void,
        size_of::<sof_ipc_dsp_oops_xtensa>(),
    );

    /* then get panic info */
    if (*xoops).arch_hdr.totalsize > EXCEPT_MAX_HDR_SIZE {
        dev_err(
            (*sdev).dev,
            c"invalid header size 0x%x. FW oops is bogus\n".as_ptr(),
            (*xoops).arch_hdr.totalsize,
        );
        return;
    }
    offset = offset.wrapping_add((*xoops).arch_hdr.totalsize);
    sof_mailbox_read(
        sdev,
        offset,
        panic_info as *mut c_void,
        size_of::<sof_ipc_panic_info>(),
    );

    /* then get the stack */
    offset = offset.wrapping_add(size_of::<sof_ipc_panic_info>() as u32);
    sof_mailbox_read(
        sdev,
        offset,
        stack as *mut c_void,
        stack_words.wrapping_mul(size_of::<u32>()),
    );
}

/**
 * imx8_dump() - This function is called when a panic message is
 * received from the firmware.
 * @sdev: SOF device
 * @flags: parameter not used but required by ops prototype
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn imx8_dump(sdev: *mut snd_sof_dev, _flags: u32) {
    let mut xoops: sof_ipc_dsp_oops_xtensa = core::mem::zeroed();
    let mut panic_info: sof_ipc_panic_info = core::mem::zeroed();
    let mut stack: [u32; IMX8_STACK_DUMP_SIZE] = [0; IMX8_STACK_DUMP_SIZE];
    let mut status: u32 = 0;

    /* Get information about the panic status from the debug box area.
     * Compute the trace point based on the status.
     */
    sof_mailbox_read(
        sdev,
        (*sdev).debug_box.offset.wrapping_add(0x4),
        &mut status as *mut u32 as *mut c_void,
        4,
    );

    /* Get information about the registers, the filename and line
     * number and the stack.
     */
    imx8_get_registers(
        sdev,
        &mut xoops,
        &mut panic_info,
        stack.as_mut_ptr(),
        IMX8_STACK_DUMP_SIZE,
    );

    /* Print the information to the console */
    sof_print_oops_and_stack(
        sdev,
        KERN_ERR,
        status,
        status,
        &mut xoops,
        &mut panic_info,
        stack.as_mut_ptr(),
        IMX8_STACK_DUMP_SIZE,
    );
}
/* EXPORT_SYMBOL(imx8_dump); */

unsafe extern "C" fn imx_handle_reply(ipc: *mut imx_dsp_ipc) {
    let sdev: *mut snd_sof_dev = imx_dsp_get_data(ipc);

    /* guard(spinlock_irqsave)(&sdev->ipc_lock); */
    snd_sof_ipc_process_reply(sdev, 0);
}

unsafe extern "C" fn imx_handle_request(ipc: *mut imx_dsp_ipc) {
    let sdev: *mut snd_sof_dev;
    let mut panic_code: u32 = 0;

    sdev = imx_dsp_get_data(ipc);

    if (*get_chip_info(sdev)).ipc_info.has_panic_code {
        sof_mailbox_read(
            sdev,
            (*sdev).debug_box.offset.wrapping_add(0x4),
            &mut panic_code as *mut u32 as *mut c_void,
            size_of::<u32>(),
        );

        if (panic_code & SOF_IPC_PANIC_MAGIC_MASK) == SOF_IPC_PANIC_MAGIC {
            snd_sof_dsp_panic(sdev, panic_code, true);
            return;
        }
    }

    snd_sof_ipc_msgs_rx(sdev);
}

static imx_ipc_ops: imx_dsp_ops = imx_dsp_ops {
    handle_reply: Some(imx_handle_reply),
    handle_request: Some(imx_handle_request),
};

unsafe extern "C" fn imx_send_msg(
    sdev: *mut snd_sof_dev,
    msg: *mut snd_sof_ipc_msg,
) -> c_int {
    let common: *mut imx_common_data = (*(*sdev).pdata).hw_pdata as *mut imx_common_data;

    sof_mailbox_write(
        sdev,
        (*sdev).host_box.offset,
        (*msg).msg_data,
        (*msg).msg_size,
    );
    imx_dsp_ring_doorbell((*common).ipc_handle, 0x0);

    0
}

unsafe extern "C" fn imx_get_bar_index(_sdev: *mut snd_sof_dev, ty: u32) -> c_int {
    match ty as c_int {
        SOF_FW_BLK_TYPE_IRAM | SOF_FW_BLK_TYPE_SRAM => ty as c_int,
        _ => -EINVAL,
    }
}

unsafe extern "C" fn imx_get_mailbox_offset(sdev: *mut snd_sof_dev) -> c_int {
    (*get_chip_info(sdev)).ipc_info.boot_mbox_offset
}

unsafe extern "C" fn imx_get_window_offset(sdev: *mut snd_sof_dev, _id: u32) -> c_int {
    (*get_chip_info(sdev)).ipc_info.window_offset
}

unsafe extern "C" fn imx_set_power_state(
    sdev: *mut snd_sof_dev,
    target: *const sof_dsp_power_state,
) -> c_int {
    (*sdev).dsp_power_state = *target;

    0
}

unsafe extern "C" fn imx_common_resume(sdev: *mut snd_sof_dev) -> c_int {
    let common: *mut imx_common_data;
    let mut ret: c_int;
    let mut i: c_int;

    common = (*(*sdev).pdata).hw_pdata as *mut imx_common_data;

    ret = clk_bulk_prepare_enable((*common).clk_num, (*common).clks);
    if ret != 0 {
        dev_err(
            (*sdev).dev,
            c"failed to enable clocks: %d\n".as_ptr(),
            ret,
        );
    }

    i = 0;
    while i < DSP_MU_CHAN_NUM {
        imx_dsp_request_channel((*common).ipc_handle, i);
        i += 1;
    }

    /* done. If need be, core will be started by SOF core immediately after */
    0
}

unsafe extern "C" fn imx_common_suspend(sdev: *mut snd_sof_dev) -> c_int {
    let common: *mut imx_common_data;
    let mut i: c_int;
    let ret: c_int;

    common = (*(*sdev).pdata).hw_pdata as *mut imx_common_data;

    ret = imx_chip_core_shutdown(sdev);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            c"failed to shutdown core: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    i = 0;
    while i < DSP_MU_CHAN_NUM {
        imx_dsp_free_channel((*common).ipc_handle, i);
        i += 1;
    }

    clk_bulk_disable_unprepare((*common).clk_num, (*common).clks);

    0
}

unsafe extern "C" fn imx_runtime_resume(sdev: *mut snd_sof_dev) -> c_int {
    let target_state: sof_dsp_power_state = sof_dsp_power_state {
        state: SOF_DSP_PM_D0,
    };
    let ret: c_int;

    ret = imx_common_resume(sdev);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            c"failed to runtime common resume: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    snd_sof_dsp_set_power_state(sdev, &target_state)
}

unsafe extern "C" fn imx_resume(sdev: *mut snd_sof_dev) -> c_int {
    let target_state: sof_dsp_power_state = sof_dsp_power_state {
        state: SOF_DSP_PM_D0,
    };
    let ret: c_int;

    ret = imx_common_resume(sdev);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            c"failed to common resume: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    if pm_runtime_suspended((*sdev).dev) {
        pm_runtime_disable((*sdev).dev);
        pm_runtime_set_active((*sdev).dev);
        pm_runtime_mark_last_busy((*sdev).dev);
        pm_runtime_enable((*sdev).dev);
        pm_runtime_idle((*sdev).dev);
    }

    snd_sof_dsp_set_power_state(sdev, &target_state)
}

unsafe extern "C" fn imx_runtime_suspend(sdev: *mut snd_sof_dev) -> c_int {
    let target_state: sof_dsp_power_state = sof_dsp_power_state {
        state: SOF_DSP_PM_D3,
    };
    let ret: c_int;

    ret = imx_common_suspend(sdev);
    if ret < 0 {
        dev_err(
            (*sdev).dev,
            c"failed to runtime common suspend: %d\n".as_ptr(),
            ret,
        );
    }

    snd_sof_dsp_set_power_state(sdev, &target_state)
}

unsafe extern "C" fn imx_suspend(sdev: *mut snd_sof_dev, target_state: c_uint) -> c_int {
    let target_power_state: sof_dsp_power_state = sof_dsp_power_state {
        state: target_state,
    };
    let ret: c_int;

    if !pm_runtime_suspended((*sdev).dev) {
        ret = imx_common_suspend(sdev);
        if ret < 0 {
            dev_err(
                (*sdev).dev,
                c"failed to common suspend: %d\n".as_ptr(),
                ret,
            );
            return ret;
        }
    }

    snd_sof_dsp_set_power_state(sdev, &target_power_state)
}

unsafe extern "C" fn imx_region_name_to_blk_type(region_name: *const c_char) -> c_int {
    if strcmp(region_name, c"iram".as_ptr()) == 0 {
        SOF_FW_BLK_TYPE_IRAM
    } else if strcmp(region_name, c"dram".as_ptr()) == 0 {
        SOF_FW_BLK_TYPE_DRAM
    } else if strcmp(region_name, c"sram".as_ptr()) == 0 {
        SOF_FW_BLK_TYPE_SRAM
    } else {
        -EINVAL
    }
}

unsafe extern "C" fn imx_parse_ioremap_memory(sdev: *mut snd_sof_dev) -> c_int {
    let chip_info: *const imx_chip_info;
    let pdev: *mut platform_device;
    let mut res: *mut resource;
    let mut _res: resource = core::mem::zeroed();
    let mut i: isize;
    let mut blk_type: c_int;
    let mut ret: c_int;

    pdev = to_platform_device((*sdev).dev);
    chip_info = get_chip_info(sdev);

    i = 0;
    while !(*(*chip_info).memory.offset(i)).name.is_null() {
        blk_type = imx_region_name_to_blk_type((*(*chip_info).memory.offset(i)).name);
        if blk_type < 0 {
            return dev_err_probe(
                (*sdev).dev,
                blk_type,
                c"no blk type for region %s\n".as_ptr(),
                (*(*chip_info).memory.offset(i)).name,
            );
        }

        if !(*(*chip_info).memory.offset(i)).reserved {
            res = platform_get_resource_byname(
                pdev,
                IORESOURCE_MEM,
                (*(*chip_info).memory.offset(i)).name,
            );
            if res.is_null() {
                return dev_err_probe(
                    (*sdev).dev,
                    -ENODEV,
                    c"failed to fetch %s resource\n".as_ptr(),
                    (*(*chip_info).memory.offset(i)).name,
                );
            }
        } else {
            ret = of_reserved_mem_region_to_resource_byname(
                (*pdev).dev.of_node,
                (*(*chip_info).memory.offset(i)).name,
                &mut _res,
            );
            if ret < 0 {
                return dev_err_probe(
                    (*sdev).dev,
                    ret,
                    c"no valid entry for %s\n".as_ptr(),
                    (*(*chip_info).memory.offset(i)).name,
                );
            }
            res = &mut _res;
        }

        (*sdev).bar[blk_type as usize] = devm_ioremap_resource((*sdev).dev, res);
        if IS_ERR((*sdev).bar[blk_type as usize]) {
            return dev_err_probe(
                (*sdev).dev,
                PTR_ERR((*sdev).bar[blk_type as usize]),
                c"failed to ioremap %s region\n".as_ptr(),
                (*(*chip_info).memory.offset(i)).name,
            );
        }

        i += 1;
    }

    0
}

unsafe extern "C" fn imx_unregister_action(data: *mut c_void) {
    let common: *mut imx_common_data;
    let sdev: *mut snd_sof_dev;

    sdev = data as *mut snd_sof_dev;
    common = (*(*sdev).pdata).hw_pdata as *mut imx_common_data;

    if (*get_chip_info(sdev)).has_dma_reserved {
        of_reserved_mem_device_release((*sdev).dev);
    }

    platform_device_unregister((*common).ipc_dev);
}

unsafe extern "C" fn imx_probe(sdev: *mut snd_sof_dev) -> c_int {
    let domain_data: dev_pm_domain_attach_data = dev_pm_domain_attach_data {
        pd_names: ptr::null_mut(), /* no filtering */
        pd_flags: PD_FLAG_DEV_LINK_ON,
    };
    let common: *mut imx_common_data;
    let pdev: *mut platform_device;
    let mut ret: c_int;

    pdev = to_platform_device((*sdev).dev);

    common = devm_kzalloc(
        (*sdev).dev,
        size_of::<imx_common_data>(),
        GFP_KERNEL,
    ) as *mut imx_common_data;
    if common.is_null() {
        return -ENOMEM;
    }

    (*(*sdev).pdata).hw_pdata = common as *mut c_void;

    (*common).ipc_dev = platform_device_register_data(
        (*sdev).dev,
        c"imx-dsp".as_ptr(),
        PLATFORM_DEVID_NONE,
        pdev as *const c_void,
        size_of::<platform_device>(),
    );
    if IS_ERR((*common).ipc_dev as *const c_void) {
        return dev_err_probe(
            (*sdev).dev,
            PTR_ERR((*common).ipc_dev as *const c_void),
            c"failed to create IPC device\n".as_ptr(),
        );
    }

    if (*get_chip_info(sdev)).has_dma_reserved {
        ret = of_reserved_mem_device_init_by_name(
            (*sdev).dev,
            (*pdev).dev.of_node,
            c"dma".as_ptr(),
        );
        if ret != 0 {
            platform_device_unregister((*common).ipc_dev);

            return dev_err_probe(
                (*sdev).dev,
                ret,
                c"failed to bind DMA region\n".as_ptr(),
            );
        }
    }

    /* let the devres API take care of the cleanup */
    ret = devm_add_action_or_reset(
        (*sdev).dev,
        imx_unregister_action,
        sdev as *mut c_void,
    );
    if ret != 0 {
        return ret;
    }

    (*common).ipc_handle = dev_get_drvdata(&mut (*(*common).ipc_dev).dev);
    if (*common).ipc_handle.is_null() {
        return dev_err_probe(
            (*sdev).dev,
            -EPROBE_DEFER,
            c"failed to fetch IPC handle\n".as_ptr(),
        );
    }

    ret = imx_parse_ioremap_memory(sdev);
    if ret < 0 {
        return dev_err_probe(
            (*sdev).dev,
            ret,
            c"failed to parse/ioremap memory regions\n".as_ptr(),
        );
    }

    if (*(*sdev).dev).pm_domain.is_null() {
        ret = devm_pm_domain_attach_list((*sdev).dev, &domain_data, &mut (*common).pd_list);
        if ret < 0 {
            return dev_err_probe(
                (*sdev).dev,
                ret,
                c"failed to attach PDs\n".as_ptr(),
            );
        }
    }

    ret = devm_clk_bulk_get_all((*sdev).dev, &mut (*common).clks);
    if ret < 0 {
        return dev_err_probe(
            (*sdev).dev,
            ret,
            c"failed to fetch clocks\n".as_ptr(),
        );
    }
    (*common).clk_num = ret;

    ret = clk_bulk_prepare_enable((*common).clk_num, (*common).clks);
    if ret < 0 {
        return dev_err_probe(
            (*sdev).dev,
            ret,
            c"failed to enable clocks\n".as_ptr(),
        );
    }

    (*(*common).ipc_handle).ops = &imx_ipc_ops;
    imx_dsp_set_data((*common).ipc_handle, sdev);

    (*sdev).num_cores = 1;
    (*sdev).mailbox_bar = SOF_FW_BLK_TYPE_SRAM;
    (*sdev).dsp_box.offset = (*get_chip_info(sdev)).ipc_info.boot_mbox_offset as u32;

    imx_chip_probe(sdev)
}

unsafe extern "C" fn imx_remove(sdev: *mut snd_sof_dev) {
    let common: *mut imx_common_data;
    let ret: c_int;

    common = (*(*sdev).pdata).hw_pdata as *mut imx_common_data;

    if !pm_runtime_suspended((*sdev).dev) {
        ret = imx_chip_core_shutdown(sdev);
        if ret < 0 {
            dev_err(
                (*sdev).dev,
                c"failed to shutdown core: %d\n".as_ptr(),
                ret,
            );
        }

        clk_bulk_disable_unprepare((*common).clk_num, (*common).clks);
    }
}

#[unsafe(no_mangle)]
pub static sof_imx_ops: snd_sof_dsp_ops = snd_sof_dsp_ops {
    probe: Some(imx_probe),
    remove: Some(imx_remove),

    run: Some(imx_chip_core_kick),
    reset: Some(imx_chip_core_reset),

    block_read: Some(sof_block_read),
    block_write: Some(sof_block_write),

    mailbox_read: Some(sof_mailbox_read as unsafe extern "C" fn()),
    mailbox_write: Some(sof_mailbox_write as unsafe extern "C" fn()),

    send_msg: Some(imx_send_msg),
    get_mailbox_offset: Some(imx_get_mailbox_offset),
    get_window_offset: Some(imx_get_window_offset),

    ipc_msg_data: Some(sof_ipc_msg_data),
    set_stream_data_offset: Some(sof_set_stream_data_offset),

    get_bar_index: Some(imx_get_bar_index),
    load_firmware: Some(snd_sof_load_firmware_memcpy),

    debugfs_add_region_item: Some(snd_sof_debugfs_add_region_item_iomem),

    pcm_open: Some(sof_stream_pcm_open),
    pcm_close: Some(sof_stream_pcm_close),

    runtime_suspend: Some(imx_runtime_suspend),
    runtime_resume: Some(imx_runtime_resume),
    suspend: Some(imx_suspend),
    resume: Some(imx_resume),

    set_power_state: Some(imx_set_power_state),

    hw_info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_BATCH
        | SNDRV_PCM_INFO_NO_PERIOD_WAKEUP,
};
/* EXPORT_SYMBOL(sof_imx_ops); */

/* MODULE_LICENSE("Dual BSD/GPL"); */
/* MODULE_DESCRIPTION("SOF helpers for IMX platforms"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
