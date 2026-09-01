// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2022 Advanced Micro Devices, Inc.
//
// Authors: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
//          V sujith kumar Reddy <Vsujithkumar.Reddy@amd.com>

/* ACP-specific Common code */

use core::ffi::{c_char, c_int, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type u32 = u32;
type size_t = usize;

const ACP_DSP_BAR: u32 = 0;
const ACP_SCRATCH_REG_0: u32 = 0;
const DSP_SW_INTR_STAT_OFFSET: u32 = 0;
const EXCEPT_MAX_HDR_SIZE: u32 = 0;
const AMD_STACK_DUMP_SIZE: usize = 0;
const KERN_ERR: *const c_char = ptr::null();

const SNDRV_PCM_INFO_MMAP: u32 = 0;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 0;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 0;
const SNDRV_PCM_INFO_PAUSE: u32 = 0;
const SNDRV_PCM_INFO_NO_PERIOD_WAKEUP: u32 = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dev {
    pub pdata: *mut snd_sof_pdata,
    pub debug_box: sof_mailbox,
    pub dsp_box: sof_mailbox,
    pub dsp_oops_offset: u32,
    pub dev: *mut device,
}

#[repr(C)]
pub struct sof_mailbox {
    pub offset: u32,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub hw_pdata: *mut c_void,
    pub desc: *const sof_dev_desc,
    pub tplg_filename: *const c_char,
    pub fw_filename: *const c_char,
}

#[repr(C)]
pub struct sof_dev_desc {
    pub machines: *mut snd_soc_acpi_mach,
    pub alt_machines: *mut snd_soc_acpi_mach,
}

#[repr(C)]
pub struct sof_amd_acp_desc {
    pub dsp_intr_base: u32,
}

#[repr(C)]
pub struct scratch_ipc_conf {
    pub sof_dsp_msg_write: u32,
    pub sof_dsp_ack_write: u32,
    pub sof_host_msg_write: u32,
    pub sof_host_ack_write: u32,
}

#[repr(C)]
pub struct sof_ipc_dsp_oops_xtensa {
    pub arch_hdr: sof_ipc_dsp_oops_arch_hdr,
}

#[repr(C)]
pub struct sof_ipc_dsp_oops_arch_hdr {
    pub totalsize: u32,
}

#[repr(C)]
pub struct sof_ipc_panic_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acp_dev_data {
    pub sdw: *mut sdw_amd_ctx,
    pub info: acp_sdw_info,
    pub pci_rev: u32,
}

#[repr(C)]
pub struct acp_sdw_info {
    pub count: c_int,
}

#[repr(C)]
pub struct sdw_amd_ctx {
    pub peripherals: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_acpi_link_adr {
    pub num_adr: u32,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub links: *const snd_soc_acpi_link_adr,
    pub link_mask: u32,
    pub machine_check: Option<unsafe extern "C" fn(*mut sdw_amd_ctx) -> bool>,
    pub mach_params: snd_soc_acpi_mach_params,
    pub sof_tplg_filename: *const c_char,
    pub fw_filename: *const c_char,
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
    pub subsystem_rev: u32,
    pub links: *const snd_soc_acpi_link_adr,
    pub link_mask: u32,
    pub platform: *const c_char,
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    pub probe: *const c_void,
    pub remove: *const c_void,
    pub write: *const c_void,
    pub read: *const c_void,
    pub block_read: *const c_void,
    pub block_write: *const c_void,
    pub load_firmware: *const c_void,
    pub pre_fw_run: *const c_void,
    pub get_bar_index: *const c_void,
    pub run: *const c_void,
    pub send_msg: *const c_void,
    pub ipc_msg_data: *const c_void,
    pub set_stream_data_offset: *const c_void,
    pub get_mailbox_offset: *const c_void,
    pub get_window_offset: *const c_void,
    pub irq_thread: *const c_void,
    pub pcm_open: *const c_void,
    pub pcm_close: *const c_void,
    pub pcm_hw_params: *const c_void,
    pub pcm_pointer: *const c_void,
    pub hw_info: u32,
    pub machine_select: unsafe extern "C" fn(*mut snd_sof_dev) -> *mut snd_soc_acpi_mach,
    pub machine_register: *const c_void,
    pub machine_unregister: *const c_void,
    pub trace_init: *const c_void,
    pub trace_release: *const c_void,
    pub suspend: *const c_void,
    pub resume: *const c_void,
    pub ipc_dump: unsafe extern "C" fn(*mut snd_sof_dev),
    pub dbg_dump: unsafe extern "C" fn(*mut snd_sof_dev, u32),
    pub debugfs_add_region_item: *const c_void,
    pub dsp_arch_ops: *const c_void,
    pub register_ipc_clients: *const c_void,
    pub unregister_ipc_clients: *const c_void,
}

unsafe extern "C" {
    fn get_chip_info(pdata: *mut snd_sof_pdata) -> *const sof_amd_acp_desc;
    fn snd_sof_dsp_read(sdev: *mut snd_sof_dev, bar: u32, offset: u32) -> u32;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn acp_mailbox_read(sdev: *mut snd_sof_dev, offset: u32, dest: *mut c_void, size: size_t);
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
    fn sdw_amd_get_slave_info(sdw: *mut sdw_amd_ctx) -> c_int;
    fn snd_soc_acpi_sdw_link_slaves_found(
        dev: *mut device,
        link: *const snd_soc_acpi_link_adr,
        peripherals: *mut c_void,
    ) -> bool;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn snd_soc_acpi_find_machine(machines: *mut snd_soc_acpi_mach) -> *mut snd_soc_acpi_mach;

    static amd_sof_acp_probe: c_void;
    static amd_sof_acp_remove: c_void;
    static sof_io_write: c_void;
    static sof_io_read: c_void;
    static acp_dsp_block_read: c_void;
    static acp_dsp_block_write: c_void;
    static snd_sof_load_firmware_memcpy: c_void;
    static acp_dsp_pre_fw_run: c_void;
    static acp_get_bar_index: c_void;
    static acp_sof_dsp_run: c_void;
    static acp_sof_ipc_send_msg: c_void;
    static acp_sof_ipc_msg_data: c_void;
    static acp_set_stream_data_offset: c_void;
    static acp_sof_ipc_get_mailbox_offset: c_void;
    static acp_sof_ipc_get_window_offset: c_void;
    static acp_sof_ipc_irq_thread: c_void;
    static acp_pcm_open: c_void;
    static acp_pcm_close: c_void;
    static acp_pcm_hw_params: c_void;
    static acp_pcm_pointer: c_void;
    static sof_machine_register: c_void;
    static sof_machine_unregister: c_void;
    static acp_sof_trace_init: c_void;
    static acp_sof_trace_release: c_void;
    static amd_sof_acp_suspend: c_void;
    static amd_sof_acp_resume: c_void;
    static snd_sof_debugfs_add_region_item_iomem: c_void;
    static sof_xtensa_arch_ops: c_void;
    static acp_probes_register: c_void;
    static acp_probes_unregister: c_void;
}

/**
 * amd_sof_ipc_dump() - This function is called when IPC tx times out.
 * @sdev: SOF device.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn amd_sof_ipc_dump(sdev: *mut snd_sof_dev) {
    let desc = unsafe { get_chip_info((*sdev).pdata) };
    let base: u32 = unsafe { (*desc).dsp_intr_base };
    let dsp_msg_write: u32 = unsafe { (*sdev).debug_box.offset }
        + offset_of!(scratch_ipc_conf, sof_dsp_msg_write) as u32;
    let dsp_ack_write: u32 = unsafe { (*sdev).debug_box.offset }
        + offset_of!(scratch_ipc_conf, sof_dsp_ack_write) as u32;
    let host_msg_write: u32 = unsafe { (*sdev).debug_box.offset }
        + offset_of!(scratch_ipc_conf, sof_host_msg_write) as u32;
    let host_ack_write: u32 = unsafe { (*sdev).debug_box.offset }
        + offset_of!(scratch_ipc_conf, sof_host_ack_write) as u32;
    let dsp_msg: u32;
    let dsp_ack: u32;
    let host_msg: u32;
    let host_ack: u32;
    let irq_stat: u32;

    dsp_msg = unsafe { snd_sof_dsp_read(sdev, ACP_DSP_BAR, ACP_SCRATCH_REG_0 + dsp_msg_write) };
    dsp_ack = unsafe { snd_sof_dsp_read(sdev, ACP_DSP_BAR, ACP_SCRATCH_REG_0 + dsp_ack_write) };
    host_msg = unsafe { snd_sof_dsp_read(sdev, ACP_DSP_BAR, ACP_SCRATCH_REG_0 + host_msg_write) };
    host_ack = unsafe { snd_sof_dsp_read(sdev, ACP_DSP_BAR, ACP_SCRATCH_REG_0 + host_ack_write) };
    irq_stat = unsafe { snd_sof_dsp_read(sdev, ACP_DSP_BAR, base + DSP_SW_INTR_STAT_OFFSET) };

    unsafe {
        dev_err(
            (*sdev).dev,
            c"dsp_msg = %#x dsp_ack = %#x host_msg = %#x host_ack = %#x irq_stat = %#x\n".as_ptr(),
            dsp_msg,
            dsp_ack,
            host_msg,
            host_ack,
            irq_stat,
        );
    }
}

/**
 * amd_get_registers() - This function is called in case of DSP oops
 * in order to gather information about the registers, filename and
 * linenumber and stack.
 * @sdev: SOF device.
 * @xoops: Stores information about registers.
 * @panic_info: Stores information about filename and line number.
 * @stack: Stores the stack dump.
 * @stack_words: Size of the stack dump.
 */
unsafe fn amd_get_registers(
    sdev: *mut snd_sof_dev,
    xoops: *mut sof_ipc_dsp_oops_xtensa,
    panic_info: *mut sof_ipc_panic_info,
    stack: *mut u32,
    stack_words: size_t,
) {
    let mut offset: u32 = unsafe { (*sdev).dsp_oops_offset };

    /* first read registers */
    unsafe {
        acp_mailbox_read(
            sdev,
            offset,
            xoops as *mut c_void,
            size_of::<sof_ipc_dsp_oops_xtensa>(),
        );
    }

    /* then get panic info */
    if unsafe { (*xoops).arch_hdr.totalsize } > EXCEPT_MAX_HDR_SIZE {
        unsafe {
            dev_err(
                (*sdev).dev,
                c"invalid header size 0x%x. FW oops is bogus\n".as_ptr(),
                (*xoops).arch_hdr.totalsize,
            );
        }
        return;
    }

    offset += unsafe { (*xoops).arch_hdr.totalsize };
    unsafe {
        acp_mailbox_read(
            sdev,
            offset,
            panic_info as *mut c_void,
            size_of::<sof_ipc_panic_info>(),
        );
    }

    /* then get the stack */
    offset += size_of::<sof_ipc_panic_info>() as u32;
    unsafe {
        acp_mailbox_read(
            sdev,
            offset,
            stack as *mut c_void,
            stack_words * size_of::<u32>(),
        );
    }
}

/**
 * amd_sof_dump() - This function is called when a panic message is
 * received from the firmware.
 * @sdev: SOF device.
 * @flags: parameter not used but required by ops prototype
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn amd_sof_dump(sdev: *mut snd_sof_dev, _flags: u32) {
    let mut xoops: sof_ipc_dsp_oops_xtensa = unsafe { core::mem::zeroed() };
    let mut panic_info: sof_ipc_panic_info = sof_ipc_panic_info { _private: [] };
    let mut stack: [u32; AMD_STACK_DUMP_SIZE] = [0; AMD_STACK_DUMP_SIZE];
    let mut status: u32 = 0;

    /* Get information about the panic status from the debug box area.
     * Compute the trace point based on the status.
     */
    if unsafe { (*sdev).dsp_oops_offset > (*sdev).debug_box.offset } {
        unsafe {
            acp_mailbox_read(
                sdev,
                (*sdev).debug_box.offset,
                &mut status as *mut u32 as *mut c_void,
                size_of::<u32>(),
            );
        }
    } else {
        /* Read DSP Panic status from dsp_box.
         * As window information for exception box offset and size is not available
         * before FW_READY
         */
        unsafe {
            acp_mailbox_read(
                sdev,
                (*sdev).dsp_box.offset,
                &mut status as *mut u32 as *mut c_void,
                size_of::<u32>(),
            );
            (*sdev).dsp_oops_offset = (*sdev).dsp_box.offset + size_of::<u32>() as u32;
        }
    }

    /* Get information about the registers, the filename and line
     * number and the stack.
     */
    unsafe {
        amd_get_registers(
            sdev,
            &mut xoops,
            &mut panic_info,
            stack.as_mut_ptr(),
            AMD_STACK_DUMP_SIZE,
        );
    }

    /* Print the information to the console */
    unsafe {
        sof_print_oops_and_stack(
            sdev,
            KERN_ERR,
            status,
            status,
            &mut xoops,
            &mut panic_info,
            stack.as_mut_ptr(),
            AMD_STACK_DUMP_SIZE,
        );
    }
}

// Original C condition: #if IS_ENABLED(CONFIG_SND_SOC_SOF_AMD_SOUNDWIRE)
unsafe fn amd_sof_sdw_get_slave_info(sdev: *mut snd_sof_dev) -> c_int {
    let acp_data = unsafe { (*(*sdev).pdata).hw_pdata as *mut acp_dev_data };

    unsafe { sdw_amd_get_slave_info((*acp_data).sdw) }
}

// Original C condition: #if IS_ENABLED(CONFIG_SND_SOC_SOF_AMD_SOUNDWIRE)
unsafe fn amd_sof_sdw_machine_select(sdev: *mut snd_sof_dev) -> *mut snd_soc_acpi_mach {
    let mut mach: *mut snd_soc_acpi_mach;
    let mut link: *const snd_soc_acpi_link_adr;
    let acp_data = unsafe { (*(*sdev).pdata).hw_pdata as *mut acp_dev_data };
    let mut ret: c_int;
    let mut i: c_int;

    if unsafe { (*acp_data).info.count } != 0 {
        ret = unsafe { amd_sof_sdw_get_slave_info(sdev) };
        if ret != 0 {
            unsafe {
                dev_info((*sdev).dev, c"failed to read slave information\n".as_ptr());
            }
            return ptr::null_mut();
        }
        mach = unsafe { (*(*(*sdev).pdata).desc).alt_machines };
        while !mach.is_null() {
            if unsafe { (*mach).links.is_null() } {
                break;
            }
            link = unsafe { (*mach).links };
            i = 0;
            while i < unsafe { (*acp_data).info.count } && unsafe { (*link).num_adr } != 0 {
                if !unsafe {
                    snd_soc_acpi_sdw_link_slaves_found(
                        (*sdev).dev,
                        link,
                        (*(*acp_data).sdw).peripherals,
                    )
                } {
                    break;
                }
                link = unsafe { link.add(1) };
                i += 1;
            }
            if i == unsafe { (*acp_data).info.count } || unsafe { (*link).num_adr } == 0 {
                if unsafe { (*mach).machine_check.is_none() }
                    || unsafe { ((*mach).machine_check.unwrap())((*acp_data).sdw) }
                {
                    break;
                }
            }
            mach = unsafe { mach.add(1) };
        }
        if !mach.is_null() && unsafe { (*mach).link_mask } != 0 {
            unsafe {
                (*mach).mach_params.subsystem_rev = (*acp_data).pci_rev;
                (*mach).mach_params.links = (*mach).links;
                (*mach).mach_params.link_mask = (*mach).link_mask;
                (*mach).mach_params.platform = dev_name((*sdev).dev);
            }
            return mach;
        }
    }
    unsafe {
        dev_info((*sdev).dev, c"No SoundWire machine driver found\n".as_ptr());
    }
    ptr::null_mut()
}

// Original C #else for !IS_ENABLED(CONFIG_SND_SOC_SOF_AMD_SOUNDWIRE):
// static struct snd_soc_acpi_mach *amd_sof_sdw_machine_select(struct snd_sof_dev *sdev)
// {
//      return NULL;
// }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn amd_sof_machine_select(
    sdev: *mut snd_sof_dev,
) -> *mut snd_soc_acpi_mach {
    let sof_pdata = unsafe { (*sdev).pdata };
    let acp_data = unsafe { (*(*sdev).pdata).hw_pdata as *mut acp_dev_data };
    let desc = unsafe { (*sof_pdata).desc };
    let mut mach: *mut snd_soc_acpi_mach = ptr::null_mut();

    if !unsafe { (*desc).machines }.is_null() {
        mach = unsafe { snd_soc_acpi_find_machine((*desc).machines) };
    }
    if mach.is_null() {
        mach = unsafe { amd_sof_sdw_machine_select(sdev) };
        if mach.is_null() {
            unsafe {
                dev_warn(
                    (*sdev).dev,
                    c"No matching ASoC machine driver found\n".as_ptr(),
                );
            }
            return ptr::null_mut();
        }
    }

    unsafe {
        (*mach).mach_params.subsystem_rev = (*acp_data).pci_rev;
        (*sof_pdata).tplg_filename = (*mach).sof_tplg_filename;
        (*sof_pdata).fw_filename = (*mach).fw_filename;
    }

    mach
}

/* AMD Common DSP ops */
#[unsafe(no_mangle)]
pub static sof_acp_common_ops: snd_sof_dsp_ops = snd_sof_dsp_ops {
    /* probe and remove */
    probe: unsafe { &amd_sof_acp_probe as *const c_void },
    remove: unsafe { &amd_sof_acp_remove as *const c_void },

    /* Register IO */
    write: unsafe { &sof_io_write as *const c_void },
    read: unsafe { &sof_io_read as *const c_void },

    /* Block IO */
    block_read: unsafe { &acp_dsp_block_read as *const c_void },
    block_write: unsafe { &acp_dsp_block_write as *const c_void },

    /*Firmware loading */
    load_firmware: unsafe { &snd_sof_load_firmware_memcpy as *const c_void },
    pre_fw_run: unsafe { &acp_dsp_pre_fw_run as *const c_void },
    get_bar_index: unsafe { &acp_get_bar_index as *const c_void },

    /* DSP core boot */
    run: unsafe { &acp_sof_dsp_run as *const c_void },

    /*IPC */
    send_msg: unsafe { &acp_sof_ipc_send_msg as *const c_void },
    ipc_msg_data: unsafe { &acp_sof_ipc_msg_data as *const c_void },
    set_stream_data_offset: unsafe { &acp_set_stream_data_offset as *const c_void },
    get_mailbox_offset: unsafe { &acp_sof_ipc_get_mailbox_offset as *const c_void },
    get_window_offset: unsafe { &acp_sof_ipc_get_window_offset as *const c_void },
    irq_thread: unsafe { &acp_sof_ipc_irq_thread as *const c_void },

    /* stream callbacks */
    pcm_open: unsafe { &acp_pcm_open as *const c_void },
    pcm_close: unsafe { &acp_pcm_close as *const c_void },
    pcm_hw_params: unsafe { &acp_pcm_hw_params as *const c_void },
    pcm_pointer: unsafe { &acp_pcm_pointer as *const c_void },

    hw_info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_NO_PERIOD_WAKEUP,

    /* Machine driver callbacks */
    machine_select: amd_sof_machine_select,
    machine_register: unsafe { &sof_machine_register as *const c_void },
    machine_unregister: unsafe { &sof_machine_unregister as *const c_void },

    /* Trace Logger */
    trace_init: unsafe { &acp_sof_trace_init as *const c_void },
    trace_release: unsafe { &acp_sof_trace_release as *const c_void },

    /* PM */
    suspend: unsafe { &amd_sof_acp_suspend as *const c_void },
    resume: unsafe { &amd_sof_acp_resume as *const c_void },

    ipc_dump: amd_sof_ipc_dump,
    dbg_dump: amd_sof_dump,
    debugfs_add_region_item: unsafe { &snd_sof_debugfs_add_region_item_iomem as *const c_void },
    dsp_arch_ops: unsafe { &sof_xtensa_arch_ops as *const c_void },

    /* probe client device registation */
    register_ipc_clients: unsafe { &acp_probes_register as *const c_void },
    unregister_ipc_clients: unsafe { &acp_probes_unregister as *const c_void },
};

// EXPORT_SYMBOL_NS(sof_acp_common_ops, "SND_SOC_SOF_AMD_COMMON");
// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("ACP SOF COMMON Driver");
// MODULE_IMPORT_NS("SND_SOC_SOF_AMD_COMMON");
// MODULE_IMPORT_NS("SND_SOC_SOF_XTENSA");
// MODULE_IMPORT_NS("SOUNDWIRE_AMD_INIT");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
