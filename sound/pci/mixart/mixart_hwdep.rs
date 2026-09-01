// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Digigram miXart soundcards
 *
 * DSP firmware management
 *
 * Copyright (c) 2003 by Digigram <alsa@digigram.com>
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u32 = u32;
type __be16 = u16;
type __be32 = u32;

#[repr(C)]
pub struct mixart_mgr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct firmware {
    pub size: usize,
    pub data: *const u8,
}

#[repr(C)]
pub struct mixart_uid {
    pub object_id: u32,
    pub desc: u32,
}

#[repr(C)]
pub struct mixart_msg {
    pub message_id: u32,
    pub uid: mixart_uid,
    pub data: *mut c_void,
    pub size: usize,
}

#[repr(C)]
struct snd_mixart_elf32_ehdr {
    e_ident: [u8; 16],
    e_type: __be16,
    e_machine: __be16,
    e_version: __be32,
    e_entry: __be32,
    e_phoff: __be32,
    e_shoff: __be32,
    e_flags: __be32,
    e_ehsize: __be16,
    e_phentsize: __be16,
    e_phnum: __be16,
    e_shentsize: __be16,
    e_shnum: __be16,
    e_shstrndx: __be16,
}

#[repr(C)]
struct snd_mixart_elf32_phdr {
    p_type: __be32,
    p_offset: __be32,
    p_vaddr: __be32,
    p_paddr: __be32,
    p_filesz: __be32,
    p_memsz: __be32,
    p_flags: __be32,
    p_align: __be32,
}

extern "C" {
    static mut jiffies: u64;

    fn cond_resched();
    fn readl_be(addr: *const c_void) -> u32;
    fn writel_be(value: u32, addr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcpy_toio(dst: *mut c_void, src: *const c_void, n: usize);
    fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, device: *const c_void) -> c_int;

    fn snd_mixart_send_msg(
        mgr: *mut mixart_mgr,
        request: *mut mixart_msg,
        max_resp_size: usize,
        resp: *mut c_void,
    ) -> c_int;
    fn snd_mixart_init_mailbox(mgr: *mut mixart_mgr);
    fn snd_mixart_create_pcm(chip: *mut snd_mixart) -> c_int;
    fn snd_mixart_create_mixer(mgr: *mut mixart_mgr) -> c_int;
    fn snd_card_register(card: *mut c_void) -> c_int;
    fn dev_err(dev: *const c_void, fmt: *const c_char, ...);
    fn dev_dbg(dev: *const c_void, fmt: *const c_char, ...);
}

#[repr(C)]
pub struct snd_mixart {
    pub mgr: *mut mixart_mgr,
    pub card: *mut c_void,
    _private: [u8; 0],
}

const HZ: u64 = 100;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EAGAIN: c_int = 11;
const EIO: c_int = 5;
const ENOENT: c_int = 2;
const GFP_KERNEL: u32 = 0;

extern "C" {
    fn MIXART_MEM(mgr: *mut mixart_mgr, offset: u32) -> *mut c_void;
    fn be16_to_cpu(value: __be16) -> u16;
    fn be32_to_cpu(value: __be32) -> u32;
    fn time_after_eq(a: u64, b: u64) -> bool;
}

/* Constants supplied by included miXart headers in the original C source. */
extern "C" {
    static MIXART_FLOAT_P_22_0_TO_HEX: u32;
    static MIXART_FLOAT_M_20_0_TO_HEX: u32;
    static MIXART_FLOAT____0_0_TO_HEX: u32;
    static MIXART_MAX_PHYS_CONNECTORS: u32;
    static MIXART_MAX_CARDS: u32;
    static MSG_SYSTEM_ENUM_PLAY_CONNECTOR: u32;
    static MSG_SYSTEM_ENUM_RECORD_CONNECTOR: u32;
    static MSG_CONNECTOR_GET_AUDIO_INFO: u32;
    static MSG_CONSOLE_MANAGER: u32;
    static MSG_CONSOLE_GET_CLOCK_UID: u32;
    static MSG_SYSTEM_ENUM_PHYSICAL_IO: u32;
    static MSG_SYSTEM_SEND_SYNCHRO_CMD: u32;
    static MIXART_PSEUDOREG_MXLX_STATUS_OFFSET: u32;
    static MIXART_PSEUDOREG_ELF_STATUS_OFFSET: u32;
    static MIXART_PSEUDOREG_DXLX_STATUS_OFFSET: u32;
    static MIXART_PSEUDOREG_MXLX_BASE_ADDR_OFFSET: u32;
    static MIXART_PSEUDOREG_MXLX_SIZE_OFFSET: u32;
    static MIXART_PSEUDOREG_BOARDNUMBER: u32;
    static MIXART_FLOWTABLE_PTR: u32;
    static MIXART_PSEUDOREG_DBRD_PRESENCE_OFFSET: u32;
    static MIXART_PSEUDOREG_DBRD_TYPE_OFFSET: u32;
    static DAUGHTER_TYPE_MASK: u32;
    static MIXART_DAUGHTER_TYPE_NONE: u32;
    static MIXART_DAUGHTER_TYPE_AES: u32;
    static MIXART_PSEUDOREG_DXLX_SIZE_OFFSET: u32;
    static MIXART_PSEUDOREG_DXLX_STATUS_OFFSET: u32;
    static MIXART_PSEUDOREG_DXLX_BASE_ADDR_OFFSET: u32;
}

const MIXART_FIRST_ANA_AUDIO_ID: u32 = 0;
const MIXART_FIRST_DIG_AUDIO_ID: u32 = 8;
const MIXART_MOTHERBOARD_XLX_BASE_ADDRESS: u32 = 0x00600000;
const MIXART_MOTHERBOARD_XLX_INDEX: c_int = 0;
const MIXART_MOTHERBOARD_ELF_INDEX: c_int = 1;
const MIXART_AESEBUBOARD_XLX_INDEX: c_int = 2;

#[repr(C)]
pub struct mixart_pipe {
    pub uid_left_connector: mixart_uid,
    pub uid_right_connector: mixart_uid,
}

#[repr(C)]
pub struct pci_dev {
    pub dev: c_void,
}

#[repr(C)]
pub struct mixart_flowinfo {
    pub addr: usize,
}

#[repr(C)]
pub struct mixart_mgr_fields {
    pub pci: *mut pci_dev,
    pub uid_console_manager: mixart_uid,
    pub num_cards: c_int,
    pub chip: [*mut snd_mixart_fields; 8],
    pub board_type: u32,
    pub flowinfo: mixart_flowinfo,
    pub dsp_loaded: u32,
}

#[repr(C)]
pub struct snd_mixart_fields {
    pub mgr: *mut mixart_mgr,
    pub card: *mut c_void,
    pub pipe_out_ana: mixart_pipe,
    pub pipe_out_dig: mixart_pipe,
    pub pipe_in_ana: mixart_pipe,
    pub pipe_in_dig: mixart_pipe,
    pub uid_in_analog_physio: mixart_uid,
    pub uid_out_analog_physio: mixart_uid,
}

#[repr(C)]
pub struct mixart_enum_connector_resp {
    pub error_code: u32,
    pub uid_count: u32,
    pub uid: [mixart_uid; 32],
}

#[repr(C)]
pub struct mixart_audio_info_req {
    pub line_max_level: u32,
    pub micro_max_level: u32,
    pub cd_max_level: u32,
}

#[repr(C)]
pub struct mixart_audio_info_resp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mixart_return_uid {
    pub error_code: u32,
    pub uid: mixart_uid,
}

#[repr(C)]
pub struct mixart_uid_enumeration {
    pub error_code: u32,
    pub nb_uid: u32,
    pub uid: [mixart_uid; 32],
}

unsafe fn mgr_fields<'a>(mgr: *mut mixart_mgr) -> &'a mut mixart_mgr_fields {
    &mut *(mgr as *mut mixart_mgr_fields)
}

unsafe fn chip_fields<'a>(chip: *mut snd_mixart_fields) -> &'a mut snd_mixart_fields {
    &mut *chip
}

unsafe fn kmalloc_obj<T>() -> *mut T {
    kmalloc(size_of::<T>(), GFP_KERNEL) as *mut T
}

/**
 * mixart_wait_nice_for_register_value - wait for a value on a peudo register,
 * exit with a timeout
 *
 * @mgr: pointer to miXart manager structure
 * @offset: unsigned pseudo_register base + offset of value
 * @is_egal: wait for the equal value
 * @value: value
 * @timeout: timeout in centisenconds
 */
unsafe fn mixart_wait_nice_for_register_value(
    mgr: *mut mixart_mgr,
    offset: u32,
    is_egal: c_int,
    value: u32,
    timeout: u64,
) -> c_int {
    let end_time = jiffies.wrapping_add(timeout.wrapping_mul(HZ) / 100);
    let mut read: u32;

    loop {
        /*
         * we may take too long time in this loop.
         * so give controls back to kernel if needed.
         */
        cond_resched();

        read = readl_be(MIXART_MEM(mgr, offset));
        if is_egal != 0 {
            if read == value {
                return 0;
            }
        } else {
            /* wait for different value */
            if read != value {
                return 0;
            }
        }
        if !time_after_eq(end_time, jiffies) {
            break;
        }
    }

    -EBUSY
}

/*
  structures needed to upload elf code packets
 */
unsafe fn mixart_load_elf(mgr: *mut mixart_mgr, dsp: *const firmware) -> c_int {
    let elf32_magic_number: [c_char; 4] = [0x7f, b'E' as c_char, b'L' as c_char, b'F' as c_char];
    let elf_header: *mut snd_mixart_elf32_ehdr;
    let mut i: c_int;

    elf_header = (*dsp).data as *mut snd_mixart_elf32_ehdr;
    i = 0;
    while i < 4 {
        if elf32_magic_number[i as usize] as u8 != (*elf_header).e_ident[i as usize] {
            return -EINVAL;
        }
        i += 1;
    }

    if (*elf_header).e_phoff != 0 {
        let mut elf_programheader: snd_mixart_elf32_phdr = core::mem::zeroed();

        i = 0;
        while i < be16_to_cpu((*elf_header).e_phnum) as c_int {
            let pos = be32_to_cpu((*elf_header).e_phoff)
                .wrapping_add((i as u32).wrapping_mul(be16_to_cpu((*elf_header).e_phentsize) as u32));

            memcpy(
                &mut elf_programheader as *mut _ as *mut c_void,
                (*dsp).data.add(pos as usize) as *const c_void,
                size_of::<snd_mixart_elf32_phdr>(),
            );

            if elf_programheader.p_type != 0 {
                if elf_programheader.p_filesz != 0 {
                    memcpy_toio(
                        MIXART_MEM(mgr, be32_to_cpu(elf_programheader.p_vaddr)),
                        (*dsp).data.add(be32_to_cpu(elf_programheader.p_offset) as usize)
                            as *const c_void,
                        be32_to_cpu(elf_programheader.p_filesz) as usize,
                    );
                }
            }
            i += 1;
        }
    }
    0
}

/*
 * get basic information and init miXart
 */
unsafe fn mixart_enum_connectors(mgr: *mut mixart_mgr) -> c_int {
    let mut k: u32;
    let mut err: c_int;
    let mut request: mixart_msg = core::mem::zeroed();
    let connector = kmalloc_obj::<mixart_enum_connector_resp>();
    let audio_info_req = kmalloc_obj::<mixart_audio_info_req>();
    let audio_info = kmalloc_obj::<mixart_audio_info_resp>();

    if connector.is_null() || audio_info_req.is_null() || audio_info.is_null() {
        err = -ENOMEM;
        goto_error(connector, audio_info_req, audio_info, err)
    } else {
        (*audio_info_req).line_max_level = MIXART_FLOAT_P_22_0_TO_HEX;
        (*audio_info_req).micro_max_level = MIXART_FLOAT_M_20_0_TO_HEX;
        (*audio_info_req).cd_max_level = MIXART_FLOAT____0_0_TO_HEX;

        request.message_id = MSG_SYSTEM_ENUM_PLAY_CONNECTOR;
        request.uid = mixart_uid { object_id: 0, desc: 0 }; /* board num = 0 */
        request.data = ptr::null_mut();
        request.size = 0;

        err = snd_mixart_send_msg(
            mgr,
            &mut request,
            size_of::<mixart_enum_connector_resp>(),
            connector as *mut c_void,
        );
        if err < 0 || (*connector).error_code != 0 || (*connector).uid_count > MIXART_MAX_PHYS_CONNECTORS {
            dev_err(
                &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
                b"error MSG_SYSTEM_ENUM_PLAY_CONNECTOR\n\0".as_ptr() as *const c_char,
            );
            err = -EINVAL;
            return goto_error(connector, audio_info_req, audio_info, err);
        }

        k = 0;
        while k < (*connector).uid_count {
            let pipe: *mut mixart_pipe;

            if k < MIXART_FIRST_DIG_AUDIO_ID {
                pipe = &mut (*mgr_fields(mgr).chip[(k / 2) as usize]).pipe_out_ana;
            } else {
                pipe = &mut (*mgr_fields(mgr).chip[((k - MIXART_FIRST_DIG_AUDIO_ID) / 2) as usize])
                    .pipe_out_dig;
            }
            if (k & 1) != 0 {
                (*pipe).uid_right_connector = (*connector).uid[k as usize]; /* odd */
            } else {
                (*pipe).uid_left_connector = (*connector).uid[k as usize]; /* even */
            }

            /* dev_dbg(&mgr->pci->dev, "playback connector[%d].object_id = %x\n", k, connector->uid[k].object_id); */

            /* TODO: really need send_msg MSG_CONNECTOR_GET_AUDIO_INFO for each connector ? perhaps for analog level caps ? */
            request.message_id = MSG_CONNECTOR_GET_AUDIO_INFO;
            request.uid = (*connector).uid[k as usize];
            request.data = audio_info_req as *mut c_void;
            request.size = size_of::<mixart_audio_info_req>();

            err = snd_mixart_send_msg(
                mgr,
                &mut request,
                size_of::<mixart_audio_info_resp>(),
                audio_info as *mut c_void,
            );
            if err < 0 {
                dev_err(
                    &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
                    b"error MSG_CONNECTOR_GET_AUDIO_INFO\n\0".as_ptr() as *const c_char,
                );
                return goto_error(connector, audio_info_req, audio_info, err);
            }
            /*dev_dbg(&mgr->pci->dev, "play  analog_info.analog_level_present = %x\n", audio_info->info.analog_info.analog_level_present);*/
            k += 1;
        }

        request.message_id = MSG_SYSTEM_ENUM_RECORD_CONNECTOR;
        request.uid = mixart_uid { object_id: 0, desc: 0 }; /* board num = 0 */
        request.data = ptr::null_mut();
        request.size = 0;

        err = snd_mixart_send_msg(
            mgr,
            &mut request,
            size_of::<mixart_enum_connector_resp>(),
            connector as *mut c_void,
        );
        if err < 0 || (*connector).error_code != 0 || (*connector).uid_count > MIXART_MAX_PHYS_CONNECTORS {
            dev_err(
                &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
                b"error MSG_SYSTEM_ENUM_RECORD_CONNECTOR\n\0".as_ptr() as *const c_char,
            );
            err = -EINVAL;
            return goto_error(connector, audio_info_req, audio_info, err);
        }

        k = 0;
        while k < (*connector).uid_count {
            let pipe: *mut mixart_pipe;

            if k < MIXART_FIRST_DIG_AUDIO_ID {
                pipe = &mut (*mgr_fields(mgr).chip[(k / 2) as usize]).pipe_in_ana;
            } else {
                pipe = &mut (*mgr_fields(mgr).chip[((k - MIXART_FIRST_DIG_AUDIO_ID) / 2) as usize])
                    .pipe_in_dig;
            }
            if (k & 1) != 0 {
                (*pipe).uid_right_connector = (*connector).uid[k as usize]; /* odd */
            } else {
                (*pipe).uid_left_connector = (*connector).uid[k as usize]; /* even */
            }

            /* dev_dbg(&mgr->pci->dev, "capture connector[%d].object_id = %x\n", k, connector->uid[k].object_id); */

            /* TODO: really need send_msg MSG_CONNECTOR_GET_AUDIO_INFO for each connector ? perhaps for analog level caps ? */
            request.message_id = MSG_CONNECTOR_GET_AUDIO_INFO;
            request.uid = (*connector).uid[k as usize];
            request.data = audio_info_req as *mut c_void;
            request.size = size_of::<mixart_audio_info_req>();

            err = snd_mixart_send_msg(
                mgr,
                &mut request,
                size_of::<mixart_audio_info_resp>(),
                audio_info as *mut c_void,
            );
            if err < 0 {
                dev_err(
                    &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
                    b"error MSG_CONNECTOR_GET_AUDIO_INFO\n\0".as_ptr() as *const c_char,
                );
                return goto_error(connector, audio_info_req, audio_info, err);
            }
            /*dev_dbg(&mgr->pci->dev, "rec  analog_info.analog_level_present = %x\n", audio_info->info.analog_info.analog_level_present);*/
            k += 1;
        }
        err = 0;
        goto_error(connector, audio_info_req, audio_info, err)
    }
}

unsafe fn goto_error(
    connector: *mut mixart_enum_connector_resp,
    audio_info_req: *mut mixart_audio_info_req,
    audio_info: *mut mixart_audio_info_resp,
    err: c_int,
) -> c_int {
    kfree(connector as *const c_void);
    kfree(audio_info_req as *const c_void);
    kfree(audio_info as *const c_void);

    err
}

unsafe fn mixart_enum_physio(mgr: *mut mixart_mgr) -> c_int {
    let mut k: u32;
    let mut err: c_int;
    let mut request: mixart_msg = core::mem::zeroed();
    let mut get_console_mgr: mixart_uid = core::mem::zeroed();
    let mut console_mgr: mixart_return_uid = core::mem::zeroed();
    let mut phys_io: mixart_uid_enumeration = core::mem::zeroed();

    /* get the uid for the console manager */
    get_console_mgr.object_id = 0;
    get_console_mgr.desc = MSG_CONSOLE_MANAGER | 0; /* cardindex = 0 */

    request.message_id = MSG_CONSOLE_GET_CLOCK_UID;
    request.uid = get_console_mgr;
    request.data = &mut get_console_mgr as *mut _ as *mut c_void;
    request.size = size_of::<mixart_uid>();

    err = snd_mixart_send_msg(
        mgr,
        &mut request,
        size_of::<mixart_return_uid>(),
        &mut console_mgr as *mut _ as *mut c_void,
    );

    if err < 0 || console_mgr.error_code != 0 {
        dev_dbg(
            &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
            b"error MSG_CONSOLE_GET_CLOCK_UID : err=%x\n\0".as_ptr() as *const c_char,
            console_mgr.error_code,
        );
        return -EINVAL;
    }

    /* used later for clock issues ! */
    mgr_fields(mgr).uid_console_manager = console_mgr.uid;

    request.message_id = MSG_SYSTEM_ENUM_PHYSICAL_IO;
    request.uid = mixart_uid { object_id: 0, desc: 0 };
    request.data = &mut console_mgr.uid as *mut _ as *mut c_void;
    request.size = size_of::<mixart_uid>();

    err = snd_mixart_send_msg(
        mgr,
        &mut request,
        size_of::<mixart_uid_enumeration>(),
        &mut phys_io as *mut _ as *mut c_void,
    );
    if err < 0 || phys_io.error_code != 0 {
        dev_err(
            &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
            b"error MSG_SYSTEM_ENUM_PHYSICAL_IO err(%x) error_code(%x)\n\0".as_ptr()
                as *const c_char,
            err,
            phys_io.error_code,
        );
        return -EINVAL;
    }

    /* min 2 phys io per card (analog in + analog out) */
    if phys_io.nb_uid < MIXART_MAX_CARDS * 2 {
        return -EINVAL;
    }

    k = 0;
    while k < mgr_fields(mgr).num_cards as u32 {
        chip_fields(mgr_fields(mgr).chip[k as usize]).uid_in_analog_physio = phys_io.uid[k as usize];
        chip_fields(mgr_fields(mgr).chip[k as usize]).uid_out_analog_physio =
            phys_io.uid[(phys_io.nb_uid / 2 + k) as usize];
        k += 1;
    }

    0
}

unsafe fn mixart_first_init(mgr: *mut mixart_mgr) -> c_int {
    let mut k: u32 = 0;
    let mut err: c_int;
    let mut request: mixart_msg = core::mem::zeroed();

    err = mixart_enum_connectors(mgr);
    if err < 0 {
        return err;
    }

    err = mixart_enum_physio(mgr);
    if err < 0 {
        return err;
    }

    /* send a synchro command to card (necessary to do this before first MSG_STREAM_START_STREAM_GRP_PACKET) */
    /* though why not here */
    request.message_id = MSG_SYSTEM_SEND_SYNCHRO_CMD;
    request.uid = mixart_uid { object_id: 0, desc: 0 };
    request.data = ptr::null_mut();
    request.size = 0;
    /* this command has no data. response is a 32 bit status */
    err = snd_mixart_send_msg(
        mgr,
        &mut request,
        size_of::<u32>(),
        &mut k as *mut _ as *mut c_void,
    );
    if err < 0 || k != 0 {
        dev_err(
            &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
            b"error MSG_SYSTEM_SEND_SYNCHRO_CMD\n\0".as_ptr() as *const c_char,
        );
        return if err == 0 { -EINVAL } else { err };
    }

    0
}

unsafe fn mixart_dsp_load(mgr: *mut mixart_mgr, index: c_int, dsp: *const firmware) -> c_int {
    let mut err: c_int;
    let mut card_index: c_int;
    let status_xilinx: u32;
    let status_elf: u32;
    let status_daught: u32;
    let mut val: u32;

    /* read motherboard xilinx status */
    status_xilinx = readl_be(MIXART_MEM(mgr, MIXART_PSEUDOREG_MXLX_STATUS_OFFSET));
    /* read elf status */
    status_elf = readl_be(MIXART_MEM(mgr, MIXART_PSEUDOREG_ELF_STATUS_OFFSET));
    /* read daughterboard xilinx status */
    status_daught = readl_be(MIXART_MEM(mgr, MIXART_PSEUDOREG_DXLX_STATUS_OFFSET));

    /* motherboard xilinx status 5 will say that the board is performing a reset */
    if status_xilinx == 5 {
        dev_err(
            &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
            b"miXart is resetting !\n\0".as_ptr() as *const c_char,
        );
        return -EAGAIN; /* try again later */
    }

    match index {
        MIXART_MOTHERBOARD_XLX_INDEX => {
            /* xilinx already loaded ? */
            if status_xilinx == 4 {
                dev_dbg(
                    &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
                    b"xilinx is already loaded !\n\0".as_ptr() as *const c_char,
                );
                return 0;
            }
            /* the status should be 0 == "idle" */
            if status_xilinx != 0 {
                dev_err(
                    &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
                    b"xilinx load error ! status = %d\n\0".as_ptr() as *const c_char,
                    status_xilinx,
                );
                return -EIO; /* modprob -r may help ? */
            }

            /* check xilinx validity */
            if *((*dsp).data as *const u32) == 0xffffffff {
                return -EINVAL;
            }
            if (*dsp).size % 4 != 0 {
                return -EINVAL;
            }

            /* set xilinx status to copying */
            writel_be(1, MIXART_MEM(mgr, MIXART_PSEUDOREG_MXLX_STATUS_OFFSET));

            /* setup xilinx base address */
            writel_be(
                MIXART_MOTHERBOARD_XLX_BASE_ADDRESS,
                MIXART_MEM(mgr, MIXART_PSEUDOREG_MXLX_BASE_ADDR_OFFSET),
            );
            /* setup code size for xilinx file */
            writel_be((*dsp).size as u32, MIXART_MEM(mgr, MIXART_PSEUDOREG_MXLX_SIZE_OFFSET));

            /* copy xilinx code */
            memcpy_toio(
                MIXART_MEM(mgr, MIXART_MOTHERBOARD_XLX_BASE_ADDRESS),
                (*dsp).data as *const c_void,
                (*dsp).size,
            );

            /* set xilinx status to copy finished */
            writel_be(2, MIXART_MEM(mgr, MIXART_PSEUDOREG_MXLX_STATUS_OFFSET));

            /* return, because no further processing needed */
            return 0;
        }
        MIXART_MOTHERBOARD_ELF_INDEX => {
            if status_elf == 4 {
                dev_dbg(
                    &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
                    b"elf file already loaded !\n\0".as_ptr() as *const c_char,
                );
                return 0;
            }

            /* the status should be 0 == "idle" */
            if status_elf != 0 {
                dev_err(
                    &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
                    b"elf load error ! status = %d\n\0".as_ptr() as *const c_char,
                    status_elf,
                );
                return -EIO; /* modprob -r may help ? */
            }

            /* wait for xilinx status == 4 */
            err = mixart_wait_nice_for_register_value(
                mgr,
                MIXART_PSEUDOREG_MXLX_STATUS_OFFSET,
                1,
                4,
                500,
            ); /* 5sec */
            if err < 0 {
                dev_err(
                    &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
                    b"xilinx was not loaded or could not be started\n\0".as_ptr() as *const c_char,
                );
                return err;
            }

            /* init some data on the card */
            writel_be(0, MIXART_MEM(mgr, MIXART_PSEUDOREG_BOARDNUMBER)); /* set miXart boardnumber to 0 */
            writel_be(0, MIXART_MEM(mgr, MIXART_FLOWTABLE_PTR)); /* reset pointer to flow table on miXart */

            /* set elf status to copying */
            writel_be(1, MIXART_MEM(mgr, MIXART_PSEUDOREG_ELF_STATUS_OFFSET));

            /* process the copying of the elf packets */
            err = mixart_load_elf(mgr, dsp);
            if err < 0 {
                return err;
            }

            /* set elf status to copy finished */
            writel_be(2, MIXART_MEM(mgr, MIXART_PSEUDOREG_ELF_STATUS_OFFSET));

            /* wait for elf status == 4 */
            err = mixart_wait_nice_for_register_value(
                mgr,
                MIXART_PSEUDOREG_ELF_STATUS_OFFSET,
                1,
                4,
                300,
            ); /* 3sec */
            if err < 0 {
                dev_err(
                    &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
                    b"elf could not be started\n\0".as_ptr() as *const c_char,
                );
                return err;
            }

            /* miXart waits at this point on the pointer to the flow table */
            writel_be(
                mgr_fields(mgr).flowinfo.addr as u32,
                MIXART_MEM(mgr, MIXART_FLOWTABLE_PTR),
            ); /* give pointer of flow table to miXart */

            return 0; /* return, another xilinx file has to be loaded before */
        }
        _ => {
            /* elf and xilinx should be loaded */
            if status_elf != 4 || status_xilinx != 4 {
                dev_err(
                    &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
                    b"xilinx or elf not successfully loaded\n\0".as_ptr() as *const c_char,
                );
                return -EIO; /* modprob -r may help ? */
            }

            /* wait for daughter detection != 0 */
            err = mixart_wait_nice_for_register_value(
                mgr,
                MIXART_PSEUDOREG_DBRD_PRESENCE_OFFSET,
                0,
                0,
                30,
            ); /* 300msec */
            if err < 0 {
                dev_err(
                    &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
                    b"error starting elf file\n\0".as_ptr() as *const c_char,
                );
                return err;
            }

            /* the board type can now be retrieved */
            mgr_fields(mgr).board_type =
                DAUGHTER_TYPE_MASK & readl_be(MIXART_MEM(mgr, MIXART_PSEUDOREG_DBRD_TYPE_OFFSET));

            if mgr_fields(mgr).board_type == MIXART_DAUGHTER_TYPE_NONE {
                /* no daughter board; the file does not have to be loaded, continue after the switch */
            } else {
                /* only if aesebu daughter board presence (elf code must run)  */
                if mgr_fields(mgr).board_type != MIXART_DAUGHTER_TYPE_AES {
                    return -EINVAL;
                }

                /* daughter should be idle */
                if status_daught != 0 {
                    dev_err(
                        &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
                        b"daughter load error ! status = %d\n\0".as_ptr() as *const c_char,
                        status_daught,
                    );
                    return -EIO; /* modprob -r may help ? */
                }

                /* check daughterboard xilinx validity */
                if *((*dsp).data as *const u32) == 0xffffffff {
                    return -EINVAL;
                }
                if (*dsp).size % 4 != 0 {
                    return -EINVAL;
                }

                /* inform mixart about the size of the file */
                writel_be((*dsp).size as u32, MIXART_MEM(mgr, MIXART_PSEUDOREG_DXLX_SIZE_OFFSET));

                /* set daughterboard status to 1 */
                writel_be(1, MIXART_MEM(mgr, MIXART_PSEUDOREG_DXLX_STATUS_OFFSET));

                /* wait for status == 2 */
                err = mixart_wait_nice_for_register_value(
                    mgr,
                    MIXART_PSEUDOREG_DXLX_STATUS_OFFSET,
                    1,
                    2,
                    30,
                ); /* 300msec */
                if err < 0 {
                    dev_err(
                        &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
                        b"daughter board load error\n\0".as_ptr() as *const c_char,
                    );
                    return err;
                }

                /* get the address where to write the file */
                val = readl_be(MIXART_MEM(mgr, MIXART_PSEUDOREG_DXLX_BASE_ADDR_OFFSET));
                if val == 0 {
                    return -EINVAL;
                }

                /* copy daughterboard xilinx code */
                memcpy_toio(MIXART_MEM(mgr, val), (*dsp).data as *const c_void, (*dsp).size);

                /* set daughterboard status to 4 */
                writel_be(4, MIXART_MEM(mgr, MIXART_PSEUDOREG_DXLX_STATUS_OFFSET));

                /* continue with init */
            }
        } /* end of switch file index*/
    }

    /* wait for daughter status == 3 */
    err = mixart_wait_nice_for_register_value(
        mgr,
        MIXART_PSEUDOREG_DXLX_STATUS_OFFSET,
        1,
        3,
        300,
    ); /* 3sec */
    if err < 0 {
        dev_err(
            &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
            b"daughter board could not be initialised\n\0".as_ptr() as *const c_char,
        );
        return err;
    }

    /* init mailbox (communication with embedded) */
    snd_mixart_init_mailbox(mgr);

    /* first communication with embedded */
    err = mixart_first_init(mgr);
    if err < 0 {
        dev_err(
            &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
            b"miXart could not be set up\n\0".as_ptr() as *const c_char,
        );
        return err;
    }

    /* create devices and mixer in accordance with HW options*/
    card_index = 0;
    while card_index < mgr_fields(mgr).num_cards {
        let chip = mgr_fields(mgr).chip[card_index as usize];

        err = snd_mixart_create_pcm(chip as *mut snd_mixart);
        if err < 0 {
            return err;
        }

        if card_index == 0 {
            err = snd_mixart_create_mixer(chip_fields(chip).mgr);
            if err < 0 {
                return err;
            }
        }

        err = snd_card_register(chip_fields(chip).card);
        if err < 0 {
            return err;
        }
        card_index += 1;
    }

    dev_dbg(
        &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
        b"miXart firmware downloaded and successfully set up\n\0".as_ptr() as *const c_char,
    );

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_mixart_setup_firmware(mgr: *mut mixart_mgr) -> c_int {
    static FW_FILES: [&[u8]; 3] = [b"miXart8.xlx\0", b"miXart8.elf\0", b"miXart8AES.xlx\0"];
    let mut path: [c_char; 32] = [0; 32];
    let mut i: c_int;
    let mut err: c_int;

    i = 0;
    while i < 3 {
        sprintf(
            path.as_mut_ptr(),
            b"mixart/%s\0".as_ptr() as *const c_char,
            FW_FILES[i as usize].as_ptr() as *const c_char,
        );
        let mut fw_entry: *const firmware = ptr::null();
        if request_firmware(
            &mut fw_entry,
            path.as_ptr(),
            &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
        ) != 0
        {
            dev_err(
                &mut (*mgr_fields(mgr).pci).dev as *mut _ as *const c_void,
                b"miXart: can't load firmware %s\n\0".as_ptr() as *const c_char,
                path.as_ptr(),
            );
            return -ENOENT;
        }
        /* fake hwdep dsp record */
        err = mixart_dsp_load(mgr, i, fw_entry);
        if err < 0 {
            return err;
        }
        mgr_fields(mgr).dsp_loaded |= 1 << i;
        i += 1;
    }
    0
}

/* MODULE_FIRMWARE("mixart/miXart8.xlx"); */
/* MODULE_FIRMWARE("mixart/miXart8.elf"); */
/* MODULE_FIRMWARE("mixart/miXart8AES.xlx"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
