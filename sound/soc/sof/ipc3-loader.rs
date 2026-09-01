// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2022 Intel Corporation

// C dependencies: <linux/firmware.h>, "sof-priv.h", "sof-audio.h",
// "ipc3-priv.h", "ops.h"

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type ssize_t = isize;
type size_t = usize;
type u8 = u8;
type u32 = u32;
type uintptr_t = usize;

const EINVAL: c_int = 22;

#[repr(C)]
pub struct firmware {
    pub size: size_t,
    pub data: *const u8,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut c_void,
    pub fw_ready: sof_ipc_fw_ready,
    pub first_boot: bool,
    pub basefw: snd_sof_basefw,
}

#[repr(C)]
pub struct sof_ipc_fw_ready {
    pub version: sof_ipc_fw_version,
    pub flags: u32,
}

#[repr(C)]
pub struct sof_ipc_fw_version {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_basefw {
    pub fw: *const firmware,
    pub payload_offset: u32,
}

#[repr(C)]
pub struct sof_ext_man_elem_header {
    pub type_: u32,
    pub size: u32,
}

#[repr(C)]
pub struct sof_ext_man_header {
    pub magic: u32,
    pub full_size: u32,
    pub header_size: u32,
    pub header_version: u32,
}

#[repr(C)]
pub struct sof_ext_man_fw_version {
    pub hdr: sof_ext_man_elem_header,
    pub version: sof_ipc_fw_version,
    pub flags: u32,
}

#[repr(C)]
pub struct sof_ext_man_window {
    pub hdr: sof_ext_man_elem_header,
    pub ipc_window: sof_ipc_window,
}

#[repr(C)]
pub struct sof_ipc_window {
    pub ext_hdr: sof_ipc_ext_data_hdr,
}

#[repr(C)]
pub struct sof_ext_man_cc_version {
    pub hdr: sof_ext_man_elem_header,
    pub cc_version: sof_ipc_cc_version,
}

#[repr(C)]
pub struct sof_ipc_cc_version {
    pub ext_hdr: sof_ipc_ext_data_hdr,
}

#[repr(C)]
pub struct sof_ipc_ext_data_hdr {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct ext_man_dbg_abi {
    pub hdr: sof_ext_man_elem_header,
    pub dbg_abi: sof_ipc_dbg_abi,
}

#[repr(C)]
pub struct sof_ipc_dbg_abi {
    pub abi_dbg_version: u32,
}

#[repr(C)]
pub struct sof_ext_man_config_data {
    pub hdr: sof_ext_man_elem_header,
    pub elems: [sof_config_elem; 0],
}

#[repr(C)]
pub struct sof_config_elem {
    pub token: u32,
    pub value: u32,
}

#[repr(C)]
pub struct snd_sof_blk_hdr {
    pub type_: u32,
    pub size: u32,
    pub offset: u32,
}

#[repr(C)]
pub struct snd_sof_mod_hdr {
    pub size: u32,
    pub num_blocks: u32,
    pub type_: u32,
}

#[repr(C)]
pub struct snd_sof_fw_header {
    pub sig: *const c_char,
    pub file_size: size_t,
    pub num_modules: u32,
    pub abi: u32,
}

#[repr(C)]
pub struct sof_ops_table {
    pub load_module: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_mod_hdr) -> c_int>,
}

#[repr(C)]
pub struct sof_ipc_fw_loader_ops {
    pub validate: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
    pub parse_ext_manifest: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> size_t>,
    pub load_fw_to_dsp: Option<unsafe extern "C" fn(*mut snd_sof_dev) -> c_int>,
}

extern "C" {
    static SND_SOF_FW_SIG: *const c_char;

    fn sof_ipc3_validate_fw_version(sdev: *mut snd_sof_dev) -> c_int;
    fn sof_ipc3_get_ext_windows(
        sdev: *mut snd_sof_dev,
        ext_hdr: *const sof_ipc_ext_data_hdr,
    ) -> c_int;
    fn sof_ipc3_get_cc_info(
        sdev: *mut snd_sof_dev,
        ext_hdr: *const sof_ipc_ext_data_hdr,
    ) -> c_int;
    fn snd_sof_dbg_memory_info_init(sdev: *mut snd_sof_dev) -> c_int;
    fn snd_sof_dsp_parse_platform_ext_manifest(
        sdev: *mut snd_sof_dev,
        elem_hdr: *const sof_ext_man_elem_header,
    ) -> c_int;
    fn snd_sof_dsp_block_write(
        sdev: *mut snd_sof_dev,
        block_type: u32,
        offset: u32,
        src: *const c_void,
        size: u32,
    ) -> c_int;
    fn sof_ops(sdev: *mut snd_sof_dev) -> *mut sof_ops_table;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
}

unsafe extern "C" {
    static SOF_EXT_MAN_MAGIC_NUMBER: u32;
    static SOF_EXT_MAN_VERSION: u32;
    static SOF_EXT_MAN_ELEM_FW_VERSION: u32;
    static SOF_EXT_MAN_ELEM_WINDOW: u32;
    static SOF_EXT_MAN_ELEM_CC_VERSION: u32;
    static SOF_EXT_MAN_ELEM_PROBE_INFO: u32;
    static SOF_EXT_MAN_ELEM_DBG_ABI: u32;
    static SOF_EXT_MAN_ELEM_CONFIG_DATA: u32;
    static SOF_EXT_MAN_ELEM_PLATFORM_CONFIG_DATA: u32;
    static SOF_EXT_MAN_CONFIG_EMPTY: u32;
    static SOF_EXT_MAN_CONFIG_IPC_MSG_SIZE: u32;
    static SOF_EXT_MAN_CONFIG_MEMORY_USAGE_SCAN: u32;
    static SOF_FW_BLK_TYPE_RSRVD0: u32;
    static SOF_FW_BLK_TYPE_ROM: u32;
    static SOF_FW_BLK_TYPE_RSRVD14: u32;
    static SOF_FW_BLK_TYPE_IRAM: u32;
    static SOF_FW_BLK_TYPE_DRAM: u32;
    static SOF_FW_BLK_TYPE_SRAM: u32;
    static SND_SOF_FW_SIG_SIZE: size_t;
}

extern "Rust" {
    fn SOF_ABI_VERSION_MAJOR(version: u32) -> u32;
    fn SOF_ABI_VERSION_MINOR(version: u32) -> u32;
    fn SOF_ABI_VERSION_PATCH(version: u32) -> u32;
    fn SOF_EXT_MAN_VERSION_INCOMPATIBLE(used: u32, found: u32) -> bool;
}

unsafe extern "C" fn ipc3_fw_ext_man_get_version(
    sdev: *mut snd_sof_dev,
    hdr: *const sof_ext_man_elem_header,
) -> c_int {
    let v = hdr as *const sof_ext_man_fw_version;

    ptr::copy_nonoverlapping(
        &(*v).version as *const sof_ipc_fw_version as *const u8,
        &mut (*sdev).fw_ready.version as *mut sof_ipc_fw_version as *mut u8,
        size_of::<sof_ipc_fw_version>(),
    );
    (*sdev).fw_ready.flags = (*v).flags;

    /* log ABI versions and check FW compatibility */
    sof_ipc3_validate_fw_version(sdev)
}

unsafe extern "C" fn ipc3_fw_ext_man_get_windows(
    sdev: *mut snd_sof_dev,
    hdr: *const sof_ext_man_elem_header,
) -> c_int {
    let w: *const sof_ext_man_window;

    w = hdr as *const sof_ext_man_window;

    sof_ipc3_get_ext_windows(sdev, &(*w).ipc_window.ext_hdr)
}

unsafe extern "C" fn ipc3_fw_ext_man_get_cc_info(
    sdev: *mut snd_sof_dev,
    hdr: *const sof_ext_man_elem_header,
) -> c_int {
    let cc: *const sof_ext_man_cc_version;

    cc = hdr as *const sof_ext_man_cc_version;

    sof_ipc3_get_cc_info(sdev, &(*cc).cc_version.ext_hdr)
}

unsafe extern "C" fn ipc3_fw_ext_man_get_dbg_abi_info(
    sdev: *mut snd_sof_dev,
    hdr: *const sof_ext_man_elem_header,
) -> c_int {
    let dbg_abi = hdr as *const ext_man_dbg_abi;

    if (*sdev).first_boot {
        dev_dbg!(
            (*sdev).dev,
            "Firmware: DBG_ABI %d:%d:%d\n",
            SOF_ABI_VERSION_MAJOR((*dbg_abi).dbg_abi.abi_dbg_version),
            SOF_ABI_VERSION_MINOR((*dbg_abi).dbg_abi.abi_dbg_version),
            SOF_ABI_VERSION_PATCH((*dbg_abi).dbg_abi.abi_dbg_version)
        );
    }

    0
}

unsafe extern "C" fn ipc3_fw_ext_man_get_config_data(
    sdev: *mut snd_sof_dev,
    hdr: *const sof_ext_man_elem_header,
) -> c_int {
    let config = hdr as *const sof_ext_man_config_data;
    let mut elem: *const sof_config_elem;
    let elems_counter: c_int;
    let elems_size: c_int;
    let mut ret: c_int = 0;
    let mut i: c_int;

    /* calculate elements counter */
    elems_size = (*config).hdr.size as c_int - size_of::<sof_ext_man_elem_header>() as c_int;
    elems_counter = elems_size / size_of::<sof_config_elem>() as c_int;

    dev_dbg!(
        (*sdev).dev,
        "manifest can hold up to %d config elements\n",
        elems_counter
    );

    i = 0;
    while i < elems_counter {
        elem = (*config).elems.as_ptr().offset(i as isize);
        dev_dbg!(
            (*sdev).dev,
            "get index %d token %d val %d\n",
            i,
            (*elem).token,
            (*elem).value
        );
        if (*elem).token == SOF_EXT_MAN_CONFIG_EMPTY {
            /* unused memory space is zero filled - mapped to EMPTY elements */
        } else if (*elem).token == SOF_EXT_MAN_CONFIG_IPC_MSG_SIZE {
            /* TODO: use ipc msg size from config data */
        } else if (*elem).token == SOF_EXT_MAN_CONFIG_MEMORY_USAGE_SCAN {
            if (*sdev).first_boot && (*elem).value != 0 {
                ret = snd_sof_dbg_memory_info_init(sdev);
            }
        } else {
            dev_info!(
                (*sdev).dev,
                "Unknown firmware configuration token %d value %d",
                (*elem).token,
                (*elem).value
            );
        }
        if ret < 0 {
            dev_err!(
                (*sdev).dev,
                "%s: processing failed for token %d value %#x, %d\n",
                "ipc3_fw_ext_man_get_config_data\0".as_ptr() as *const c_char,
                (*elem).token,
                (*elem).value,
                ret
            );
            return ret;
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn ipc3_fw_ext_man_size(
    sdev: *mut snd_sof_dev,
    fw: *const firmware,
) -> ssize_t {
    let head: *const sof_ext_man_header;

    head = (*fw).data as *const sof_ext_man_header;

    /*
     * assert fw size is big enough to contain extended manifest header,
     * it prevents from reading unallocated memory from `head` in following
     * step.
     */
    if (*fw).size < size_of::<sof_ext_man_header>() {
        return -EINVAL as ssize_t;
    }

    /*
     * When fw points to extended manifest,
     * then first u32 must be equal SOF_EXT_MAN_MAGIC_NUMBER.
     */
    if (*head).magic == SOF_EXT_MAN_MAGIC_NUMBER {
        return (*head).full_size as ssize_t;
    }

    /* otherwise given fw don't have an extended manifest */
    dev_dbg!(
        (*sdev).dev,
        "Unexpected extended manifest magic number: %#x\n",
        (*head).magic
    );
    0
}

unsafe extern "C" fn sof_ipc3_fw_parse_ext_man(sdev: *mut snd_sof_dev) -> size_t {
    let fw = (*sdev).basefw.fw;
    let mut elem_hdr: *const sof_ext_man_elem_header;
    let head: *const sof_ext_man_header;
    let ext_man_size: ssize_t;
    let mut remaining: ssize_t;
    let mut iptr: uintptr_t;
    let mut ret: c_int = 0;

    head = (*fw).data as *const sof_ext_man_header;
    remaining = (*head).full_size as ssize_t - (*head).header_size as ssize_t;
    if remaining < 0 || remaining > (*(*sdev).basefw.fw).size as ssize_t {
        return (-EINVAL as ssize_t) as size_t;
    }
    ext_man_size = ipc3_fw_ext_man_size(sdev, fw);

    /* Assert firmware starts with extended manifest */
    if ext_man_size <= 0 {
        return ext_man_size as size_t;
    }

    /* incompatible version */
    if SOF_EXT_MAN_VERSION_INCOMPATIBLE(SOF_EXT_MAN_VERSION, (*head).header_version) {
        dev_err!(
            (*sdev).dev,
            "extended manifest version %#x differ from used %#x\n",
            (*head).header_version,
            SOF_EXT_MAN_VERSION
        );
        return (-EINVAL as ssize_t) as size_t;
    }

    /* get first extended manifest element header */
    iptr = (*fw).data as uintptr_t + (*head).header_size as uintptr_t;

    while remaining > size_of::<sof_ext_man_elem_header>() as ssize_t {
        elem_hdr = iptr as *const sof_ext_man_elem_header;

        dev_dbg!(
            (*sdev).dev,
            "found sof_ext_man header type %d size %#x\n",
            (*elem_hdr).type_,
            (*elem_hdr).size
        );

        if (*elem_hdr).size < size_of::<sof_ext_man_elem_header>() as u32
            || (*elem_hdr).size as ssize_t > remaining
        {
            dev_err!(
                (*sdev).dev,
                "invalid sof_ext_man header size, type %d size %#x\n",
                (*elem_hdr).type_,
                (*elem_hdr).size
            );
            return (-EINVAL as ssize_t) as size_t;
        }

        /* process structure data */
        if (*elem_hdr).type_ == SOF_EXT_MAN_ELEM_FW_VERSION {
            ret = ipc3_fw_ext_man_get_version(sdev, elem_hdr);
        } else if (*elem_hdr).type_ == SOF_EXT_MAN_ELEM_WINDOW {
            ret = ipc3_fw_ext_man_get_windows(sdev, elem_hdr);
        } else if (*elem_hdr).type_ == SOF_EXT_MAN_ELEM_CC_VERSION {
            ret = ipc3_fw_ext_man_get_cc_info(sdev, elem_hdr);
        } else if (*elem_hdr).type_ == SOF_EXT_MAN_ELEM_PROBE_INFO {
            dev_dbg!((*sdev).dev, "Probe info (not parsed)\n");
        } else if (*elem_hdr).type_ == SOF_EXT_MAN_ELEM_DBG_ABI {
            ret = ipc3_fw_ext_man_get_dbg_abi_info(sdev, elem_hdr);
        } else if (*elem_hdr).type_ == SOF_EXT_MAN_ELEM_CONFIG_DATA {
            ret = ipc3_fw_ext_man_get_config_data(sdev, elem_hdr);
        } else if (*elem_hdr).type_ == SOF_EXT_MAN_ELEM_PLATFORM_CONFIG_DATA {
            ret = snd_sof_dsp_parse_platform_ext_manifest(sdev, elem_hdr);
        } else {
            dev_info!(
                (*sdev).dev,
                "unknown sof_ext_man header type %d size %#x\n",
                (*elem_hdr).type_,
                (*elem_hdr).size
            );
        }

        if ret < 0 {
            dev_err!(
                (*sdev).dev,
                "failed to parse sof_ext_man header type %d size %#x\n",
                (*elem_hdr).type_,
                (*elem_hdr).size
            );
            return ret as size_t;
        }

        remaining -= (*elem_hdr).size as ssize_t;
        iptr += (*elem_hdr).size as uintptr_t;
    }

    if remaining != 0 {
        dev_err!((*sdev).dev, "error: sof_ext_man header is inconsistent\n");
        return (-EINVAL as ssize_t) as size_t;
    }

    ext_man_size as size_t
}

/* generic module parser for mmaped DSPs */
unsafe extern "C" fn sof_ipc3_parse_module_memcpy(
    sdev: *mut snd_sof_dev,
    module: *mut snd_sof_mod_hdr,
) -> c_int {
    let mut block: *mut snd_sof_blk_hdr;
    let mut count: c_int;
    let ret: c_int;
    let mut offset: u32 = 0;
    let mut remaining: size_t;

    dev_dbg!(
        (*sdev).dev,
        "new module size %#x blocks %#x type %#x\n",
        (*module).size,
        (*module).num_blocks,
        (*module).type_
    );

    block = (module as *mut u8).add(size_of::<snd_sof_mod_hdr>()) as *mut snd_sof_blk_hdr;

    /* module->size doesn't include header size */
    remaining = (*module).size as size_t;
    count = 0;
    while count < (*module).num_blocks as c_int {
        /* check for wrap */
        if remaining < size_of::<snd_sof_blk_hdr>() {
            dev_err!((*sdev).dev, "not enough data remaining\n");
            return -EINVAL;
        }

        /* minus header size of block */
        remaining -= size_of::<snd_sof_blk_hdr>();

        if (*block).size == 0 {
            dev_warn!((*sdev).dev, "warning: block %d size zero\n", count);
            dev_warn!(
                (*sdev).dev,
                " type %#x offset %#x\n",
                (*block).type_,
                (*block).offset
            );
            count += 1;
            continue;
        }

        if (*block).type_ == SOF_FW_BLK_TYPE_RSRVD0
            || ((*block).type_ >= SOF_FW_BLK_TYPE_ROM && (*block).type_ <= SOF_FW_BLK_TYPE_RSRVD14)
        {
            count += 1;
            continue; /* not handled atm */
        } else if (*block).type_ == SOF_FW_BLK_TYPE_IRAM
            || (*block).type_ == SOF_FW_BLK_TYPE_DRAM
            || (*block).type_ == SOF_FW_BLK_TYPE_SRAM
        {
            offset = (*block).offset;
        } else {
            dev_err!(
                (*sdev).dev,
                "%s: bad type %#x for block %#x\n",
                "sof_ipc3_parse_module_memcpy\0".as_ptr() as *const c_char,
                (*block).type_,
                count
            );
            return -EINVAL;
        }

        dev_dbg!(
            (*sdev).dev,
            "block %d type %#x size %#x ==>  offset %#x\n",
            count,
            (*block).type_,
            (*block).size,
            offset
        );

        /* checking block->size to avoid unaligned access */
        if ((*block).size as size_t) % size_of::<u32>() != 0 {
            dev_err!(
                (*sdev).dev,
                "%s: invalid block size %#x\n",
                "sof_ipc3_parse_module_memcpy\0".as_ptr() as *const c_char,
                (*block).size
            );
            return -EINVAL;
        }
        let ret = snd_sof_dsp_block_write(
            sdev,
            (*block).type_,
            offset,
            block.add(1) as *const c_void,
            (*block).size,
        );
        if ret < 0 {
            dev_err!(
                (*sdev).dev,
                "%s: write to block type %#x failed\n",
                "sof_ipc3_parse_module_memcpy\0".as_ptr() as *const c_char,
                (*block).type_
            );
            return ret;
        }

        if remaining < (*block).size as size_t {
            dev_err!(
                (*sdev).dev,
                "%s: not enough data remaining\n",
                "sof_ipc3_parse_module_memcpy\0".as_ptr() as *const c_char
            );
            return -EINVAL;
        }

        /* minus body size of block */
        remaining -= (*block).size as size_t;
        /* next block */
        block = (block as *mut u8)
            .add(size_of::<snd_sof_blk_hdr>() + (*block).size as size_t)
            as *mut snd_sof_blk_hdr;
        count += 1;
    }

    0
}

unsafe extern "C" fn sof_ipc3_load_fw_to_dsp(sdev: *mut snd_sof_dev) -> c_int {
    let payload_offset: u32 = (*sdev).basefw.payload_offset;
    let fw = (*sdev).basefw.fw;
    let header: *mut snd_sof_fw_header;
    let mut module: *mut snd_sof_mod_hdr;
    let mut load_module: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_mod_hdr) -> c_int>;
    let mut remaining: size_t;
    let mut ret: c_int;
    let mut count: c_int;

    if fw.is_null() {
        return -EINVAL;
    }

    header = (*fw).data.add(payload_offset as size_t) as *mut snd_sof_fw_header;
    load_module = (*sof_ops(sdev)).load_module;
    if load_module.is_none() {
        dev_dbg!((*sdev).dev, "Using generic module loading\n");
        load_module = Some(sof_ipc3_parse_module_memcpy);
    } else {
        dev_dbg!((*sdev).dev, "Using custom module loading\n");
    }

    /* parse each module */
    module = (*fw)
        .data
        .add(payload_offset as size_t + size_of::<snd_sof_fw_header>())
        as *mut snd_sof_mod_hdr;
    remaining = (*fw).size - size_of::<snd_sof_fw_header>() - payload_offset as size_t;
    /* check for wrap */
    if remaining > (*fw).size {
        dev_err!(
            (*sdev).dev,
            "%s: fw size smaller than header size\n",
            "sof_ipc3_load_fw_to_dsp\0".as_ptr() as *const c_char
        );
        return -EINVAL;
    }

    count = 0;
    while count < (*header).num_modules as c_int {
        /* check for wrap */
        if remaining < size_of::<snd_sof_mod_hdr>() {
            dev_err!(
                (*sdev).dev,
                "%s: not enough data for a module\n",
                "sof_ipc3_load_fw_to_dsp\0".as_ptr() as *const c_char
            );
            return -EINVAL;
        }

        /* minus header size of module */
        remaining -= size_of::<snd_sof_mod_hdr>();

        /* module */
        ret = load_module.unwrap()(sdev, module);
        if ret < 0 {
            dev_err!(
                (*sdev).dev,
                "%s: invalid module %d\n",
                "sof_ipc3_load_fw_to_dsp\0".as_ptr() as *const c_char,
                count
            );
            return ret;
        }

        if remaining < (*module).size as size_t {
            dev_err!(
                (*sdev).dev,
                "%s: not enough data remaining\n",
                "sof_ipc3_load_fw_to_dsp\0".as_ptr() as *const c_char
            );
            return -EINVAL;
        }

        /* minus body size of module */
        remaining -= (*module).size as size_t;
        module = (module as *mut u8)
            .add(size_of::<snd_sof_mod_hdr>() + (*module).size as size_t)
            as *mut snd_sof_mod_hdr;
        count += 1;
    }

    0
}

unsafe extern "C" fn sof_ipc3_validate_firmware(sdev: *mut snd_sof_dev) -> c_int {
    let payload_offset: u32 = (*sdev).basefw.payload_offset;
    let fw = (*sdev).basefw.fw;
    let header: *mut snd_sof_fw_header;
    let fw_size: size_t = (*fw).size - payload_offset as size_t;

    if (*fw).size <= payload_offset as size_t {
        dev_err!(
            (*sdev).dev,
            "firmware size must be greater than firmware offset\n"
        );
        return -EINVAL;
    }

    /* Read the header information from the data pointer */
    header = (*fw).data.add(payload_offset as size_t) as *mut snd_sof_fw_header;

    /* verify FW sig */
    if strncmp((*header).sig, SND_SOF_FW_SIG, SND_SOF_FW_SIG_SIZE) != 0 {
        dev_err!((*sdev).dev, "invalid firmware signature\n");
        return -EINVAL;
    }

    /* check size is valid */
    if fw_size != (*header).file_size + size_of::<snd_sof_fw_header>() {
        dev_err!(
            (*sdev).dev,
            "invalid filesize mismatch got 0x%zx expected 0x%zx\n",
            fw_size,
            (*header).file_size + size_of::<snd_sof_fw_header>()
        );
        return -EINVAL;
    }

    dev_dbg!(
        (*sdev).dev,
        "header size=0x%x modules=0x%x abi=0x%x size=%zu\n",
        (*header).file_size,
        (*header).num_modules,
        (*header).abi,
        size_of::<snd_sof_fw_header>()
    );

    0
}

#[no_mangle]
pub static ipc3_loader_ops: sof_ipc_fw_loader_ops = sof_ipc_fw_loader_ops {
    validate: Some(sof_ipc3_validate_firmware),
    parse_ext_manifest: Some(sof_ipc3_fw_parse_ext_man),
    load_fw_to_dsp: Some(sof_ipc3_load_fw_to_dsp),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
