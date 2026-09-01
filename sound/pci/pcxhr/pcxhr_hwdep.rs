// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Digigram pcxhr compatible soundcards
 *
 * hwdep device manager
 *
 * Copyright (c) 2004 by Digigram <alsa@digigram.com>
 */

/* Rust translation of pci/pcxhr/pcxhr_hwdep.c.
 * C include dependencies are expected to provide the referenced kernel,
 * ALSA, and pcxhr symbols.
 */

extern "C" {
    fn pcxhr_enable_dsp(mgr: *mut pcxhr_mgr);
    fn pcxhr_init_rmh(rmh: *mut pcxhr_rmh, cmd: ::core::ffi::c_int);
    fn pcxhr_send_msg(mgr: *mut pcxhr_mgr, rmh: *mut pcxhr_rmh) -> ::core::ffi::c_int;
    fn hr222_sub_init(mgr: *mut pcxhr_mgr) -> ::core::ffi::c_int;
    fn pcxhr_write_io_num_reg_cont(
        mgr: *mut pcxhr_mgr,
        reg: ::core::ffi::c_int,
        mask: ::core::ffi::c_int,
        value: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pcxhr_reset_dsp(mgr: *mut pcxhr_mgr);
    fn pcxhr_reset_xilinx_com(mgr: *mut pcxhr_mgr);
    fn pcxhr_set_pipe_cmd_params(
        rmh: *mut pcxhr_rmh,
        is_capture: ::core::ffi::c_int,
        pin: ::core::ffi::c_int,
        audio_count: ::core::ffi::c_int,
        stream_count: ::core::ffi::c_int,
    );
    fn pcxhr_set_pipe_state(
        mgr: *mut pcxhr_mgr,
        playback_mask: ::core::ffi::c_int,
        capture_mask: ::core::ffi::c_int,
        start: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn pcxhr_load_xilinx_binary(
        mgr: *mut pcxhr_mgr,
        dsp: *const firmware,
        second: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn pcxhr_load_eeprom_binary(mgr: *mut pcxhr_mgr, dsp: *const firmware) -> ::core::ffi::c_int;
    fn pcxhr_load_boot_binary(mgr: *mut pcxhr_mgr, dsp: *const firmware) -> ::core::ffi::c_int;
    fn pcxhr_load_dsp_binary(mgr: *mut pcxhr_mgr, dsp: *const firmware) -> ::core::ffi::c_int;
    fn pcxhr_create_pcm(chip: *mut snd_pcxhr) -> ::core::ffi::c_int;
    fn pcxhr_create_mixer(mgr: *mut pcxhr_mgr) -> ::core::ffi::c_int;
    fn snd_card_register(card: *mut snd_card) -> ::core::ffi::c_int;
    fn request_firmware(
        fw: *mut *const firmware,
        name: *const ::core::ffi::c_char,
        device: *mut device,
    ) -> ::core::ffi::c_int;
    fn release_firmware(fw: *const firmware);
}

extern "C" {
    fn dev_dbg(dev: *mut device, fmt: *const ::core::ffi::c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const ::core::ffi::c_char, ...);
    fn sprintf(s: *mut ::core::ffi::c_char, fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}

extern "C" {
    static mut CMD_SUPPORTED: ::core::ffi::c_int;
    static mut CMD_VERSION: ::core::ffi::c_int;
    static mut CMD_ACCESS_IO_READ: ::core::ffi::c_int;
    static mut CMD_ACCESS_IO_WRITE: ::core::ffi::c_int;
    static mut CMD_RES_PIPE: ::core::ffi::c_int;
    static mut CMD_FREE_PIPE: ::core::ffi::c_int;
    static mut MASK_FIRST_FIELD: u32;
    static mut FIELD_SIZE: ::core::ffi::c_int;
    static mut PCXHR_PLAYBACK_STREAMS: ::core::ffi::c_int;
    static mut IO_NUM_REG_STATUS: u32;
    static mut REG_STATUS_OPTIONS: u32;
    static mut REG_STATUS_OPT_DAUGHTER_MASK: u32;
    static mut REG_STATUS_OPT_ANALOG_BOARD: u32;
    static mut REG_CONT_UNMUTE_INPUTS: ::core::ffi::c_int;
    static mut IO_NUM_REG_MUTE_OUT: u32;
    static mut PCXHR_FIRMWARE_DSP_MAIN_INDEX: ::core::ffi::c_int;
    static mut PCXHR_FIRMWARE_DSP_EPRM_INDEX: ::core::ffi::c_int;
    static mut PCXHR_FIRMWARE_XLX_COM_INDEX: ::core::ffi::c_int;
    static mut PCXHR_FIRMWARE_XLX_INT_INDEX: ::core::ffi::c_int;
    static mut PCXHR_FIRMWARE_DSP_BOOT_INDEX: ::core::ffi::c_int;
    static mut PCXHR_PIPE_DEFINED: ::core::ffi::c_int;
    static mut PCXHR_PIPE_UNDEFINED: ::core::ffi::c_int;
    static mut EINVAL: ::core::ffi::c_int;
    static mut EFAULT: ::core::ffi::c_int;
    static mut ENOENT: ::core::ffi::c_int;
}

extern "C" {
    fn DSP_EXT_CMD_SET(mgr: *mut pcxhr_mgr) -> bool;
}

#[repr(C)]
pub struct pcxhr_mgr {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pcxhr_rmh {
    pub cmd: [u32; 16],
    pub stat: [u32; 16],
    pub cmd_len: ::core::ffi::c_int,
}
#[repr(C)]
pub struct pcxhr_pipe {
    pub is_capture: ::core::ffi::c_int,
    pub first_audio: ::core::ffi::c_int,
    pub status: ::core::ffi::c_int,
}
#[repr(C)]
pub struct snd_pcxhr {
    _private: [u8; 0],
}
#[repr(C)]
pub struct firmware {
    pub size: usize,
}
#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/* Field access below assumes the real C layout is supplied by bindings for
 * these opaque structs in the final integration environment.
 */

unsafe extern "C" fn pcxhr_sub_init(mgr: *mut pcxhr_mgr) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int;
    let mut rmh: pcxhr_rmh = ::core::mem::zeroed();

    /* get options */
    pcxhr_init_rmh(&mut rmh, CMD_ACCESS_IO_READ);
    rmh.cmd[0] |= IO_NUM_REG_STATUS;
    rmh.cmd[1] = REG_STATUS_OPTIONS;
    rmh.cmd_len = 2;
    err = pcxhr_send_msg(mgr, &mut rmh);
    if err != 0 {
        return err;
    }

    if (rmh.stat[1] & REG_STATUS_OPT_DAUGHTER_MASK) == REG_STATUS_OPT_ANALOG_BOARD {
        (*mgr).board_has_analog = 1; /* analog addon board found */
    }

    /* unmute inputs */
    err = pcxhr_write_io_num_reg_cont(
        mgr,
        REG_CONT_UNMUTE_INPUTS,
        REG_CONT_UNMUTE_INPUTS,
        ::core::ptr::null_mut(),
    );
    if err != 0 {
        return err;
    }
    /* unmute outputs (a write to IO_NUM_REG_MUTE_OUT mutes!) */
    pcxhr_init_rmh(&mut rmh, CMD_ACCESS_IO_READ);
    rmh.cmd[0] |= IO_NUM_REG_MUTE_OUT;
    if DSP_EXT_CMD_SET(mgr) {
        rmh.cmd[1] = 1; /* unmute digital plugs */
        rmh.cmd_len = 2;
    }
    err = pcxhr_send_msg(mgr, &mut rmh);
    err
}

/*
 * get basic information and init pcxhr card
 */
unsafe extern "C" fn pcxhr_init_board(mgr: *mut pcxhr_mgr) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int;
    let mut rmh: pcxhr_rmh = ::core::mem::zeroed();
    let mut card_streams: ::core::ffi::c_int;

    /* calc the number of all streams used */
    if (*mgr).mono_capture != 0 {
        card_streams = (*mgr).capture_chips * 2;
    } else {
        card_streams = (*mgr).capture_chips;
    }
    card_streams += (*mgr).playback_chips * PCXHR_PLAYBACK_STREAMS;

    /* enable interrupts */
    pcxhr_enable_dsp(mgr);

    pcxhr_init_rmh(&mut rmh, CMD_SUPPORTED);
    err = pcxhr_send_msg(mgr, &mut rmh);
    if err != 0 {
        return err;
    }
    /* test 4, 8 or 12 phys out */
    if (rmh.stat[0] & MASK_FIRST_FIELD) < ((*mgr).playback_chips * 2) as u32 {
        return -EINVAL;
    }
    /* test 4, 8 or 2 phys in */
    if ((rmh.stat[0] >> (2 * FIELD_SIZE)) & MASK_FIRST_FIELD) < ((*mgr).capture_chips * 2) as u32 {
        return -EINVAL;
    }
    /* test max nb substream per board */
    if (rmh.stat[1] & 0x5f) < card_streams as u32 {
        return -EINVAL;
    }
    /* test max nb substream per pipe */
    if ((rmh.stat[1] >> 7) & 0x5f) < PCXHR_PLAYBACK_STREAMS as u32 {
        return -EINVAL;
    }
    dev_dbg(
        &mut (*(*mgr).pci).dev,
        b"supported formats : playback=%x capture=%x\n\0".as_ptr() as *const _,
        rmh.stat[2],
        rmh.stat[3],
    );

    pcxhr_init_rmh(&mut rmh, CMD_VERSION);
    /* firmware num for DSP */
    rmh.cmd[0] |= (*mgr).firmware_num as u32;
    /* transfer granularity in samples (should be multiple of 48) */
    rmh.cmd[1] = (1 << 23) + (*mgr).granularity as u32;
    rmh.cmd_len = 2;
    err = pcxhr_send_msg(mgr, &mut rmh);
    if err != 0 {
        return err;
    }
    dev_dbg(
        &mut (*(*mgr).pci).dev,
        b"PCXHR DSP version is %d.%d.%d\n\0".as_ptr() as *const _,
        (rmh.stat[0] >> 16) & 0xff,
        (rmh.stat[0] >> 8) & 0xff,
        rmh.stat[0] & 0xff,
    );
    (*mgr).dsp_version = rmh.stat[0];

    if (*mgr).is_hr_stereo != 0 {
        err = hr222_sub_init(mgr);
    } else {
        err = pcxhr_sub_init(mgr);
    }
    err
}

pub unsafe extern "C" fn pcxhr_reset_board(mgr: *mut pcxhr_mgr) {
    let mut rmh: pcxhr_rmh = ::core::mem::zeroed();

    if ((*mgr).dsp_loaded & (1 << PCXHR_FIRMWARE_DSP_MAIN_INDEX)) != 0 {
        /* mute outputs */
        if (*mgr).is_hr_stereo == 0 {
            /* a read to IO_NUM_REG_MUTE_OUT register unmutes! */
            pcxhr_init_rmh(&mut rmh, CMD_ACCESS_IO_WRITE);
            rmh.cmd[0] |= IO_NUM_REG_MUTE_OUT;
            pcxhr_send_msg(mgr, &mut rmh);
            /* mute inputs */
            pcxhr_write_io_num_reg_cont(
                mgr,
                REG_CONT_UNMUTE_INPUTS,
                0,
                ::core::ptr::null_mut(),
            );
        }
        /* stereo cards mute with reset of dsp */
    }
    /* reset pcxhr dsp */
    if ((*mgr).dsp_loaded & (1 << PCXHR_FIRMWARE_DSP_EPRM_INDEX)) != 0 {
        pcxhr_reset_dsp(mgr);
    }
    /* reset second xilinx */
    if ((*mgr).dsp_loaded & (1 << PCXHR_FIRMWARE_XLX_COM_INDEX)) != 0 {
        pcxhr_reset_xilinx_com(mgr);
        (*mgr).dsp_loaded = 1;
    }
    return;
}

/*
 *  allocate a playback/capture pipe (pcmp0/pcmc0)
 */
unsafe extern "C" fn pcxhr_dsp_allocate_pipe(
    mgr: *mut pcxhr_mgr,
    pipe: *mut pcxhr_pipe,
    is_capture: ::core::ffi::c_int,
    pin: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let stream_count: ::core::ffi::c_int;
    let audio_count: ::core::ffi::c_int;
    let err: ::core::ffi::c_int;
    let mut rmh: pcxhr_rmh = ::core::mem::zeroed();

    if is_capture != 0 {
        stream_count = 1;
        if (*mgr).mono_capture != 0 {
            audio_count = 1;
        } else {
            audio_count = 2;
        }
    } else {
        stream_count = PCXHR_PLAYBACK_STREAMS;
        audio_count = 2; /* always stereo */
    }
    dev_dbg(
        &mut (*(*mgr).pci).dev,
        b"snd_add_ref_pipe pin(%d) pcm%c0\n\0".as_ptr() as *const _,
        pin,
        if is_capture != 0 { b'c' as ::core::ffi::c_int } else { b'p' as ::core::ffi::c_int },
    );
    (*pipe).is_capture = is_capture;
    (*pipe).first_audio = pin;
    /* define pipe (P_PCM_ONLY_MASK (0x020000) is not necessary) */
    pcxhr_init_rmh(&mut rmh, CMD_RES_PIPE);
    pcxhr_set_pipe_cmd_params(&mut rmh, is_capture, pin, audio_count, stream_count);
    rmh.cmd[1] |= 0x020000; /* add P_PCM_ONLY_MASK */
    if DSP_EXT_CMD_SET(mgr) {
        /* add channel mask to command */
        rmh.cmd[rmh.cmd_len as usize] = if audio_count == 1 { 0x01 } else { 0x03 };
        rmh.cmd_len += 1;
    }
    err = pcxhr_send_msg(mgr, &mut rmh);
    if err < 0 {
        dev_err(
            &mut (*(*mgr).pci).dev,
            b"error pipe allocation (CMD_RES_PIPE) err=%x!\n\0".as_ptr() as *const _,
            err,
        );
        return err;
    }
    (*pipe).status = PCXHR_PIPE_DEFINED;

    0
}

/*
 *  free playback/capture pipe (pcmp0/pcmc0)
 *
 * Original C source had this function inside #if 0.
 *
 * unsafe extern "C" fn pcxhr_dsp_free_pipe(mgr: *mut pcxhr_mgr, pipe: *mut pcxhr_pipe)
 *     -> ::core::ffi::c_int
 * {
 *     let mut rmh: pcxhr_rmh = ::core::mem::zeroed();
 *     let mut capture_mask = 0;
 *     let mut playback_mask = 0;
 *     let mut err = 0;
 *
 *     if (*pipe).is_capture != 0 {
 *         capture_mask = 1 << (*pipe).first_audio;
 *     } else {
 *         playback_mask = 1 << (*pipe).first_audio;
 *     }
 *
 *     /* stop one pipe */
 *     err = pcxhr_set_pipe_state(mgr, playback_mask, capture_mask, 0);
 *     if err < 0 {
 *         dev_err(&mut (*(*mgr).pci).dev, b"error stopping pipe!\n\0".as_ptr() as *const _);
 *     }
 *     /* release the pipe */
 *     pcxhr_init_rmh(&mut rmh, CMD_FREE_PIPE);
 *     pcxhr_set_pipe_cmd_params(&mut rmh, (*pipe).is_capture, (*pipe).first_audio, 0, 0);
 *     err = pcxhr_send_msg(mgr, &mut rmh);
 *     if err < 0 {
 *         dev_err(
 *             &mut (*(*mgr).pci).dev,
 *             b"error pipe release (CMD_FREE_PIPE) err(%x)\n\0".as_ptr() as *const _,
 *             err,
 *         );
 *     }
 *     (*pipe).status = PCXHR_PIPE_UNDEFINED;
 *     err
 * }
 */

unsafe extern "C" fn pcxhr_config_pipes(mgr: *mut pcxhr_mgr) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    let mut chip: *mut snd_pcxhr;
    let mut pipe: *mut pcxhr_pipe;

    /* allocate the pipes on the dsp */
    i = 0;
    while i < (*mgr).num_cards {
        chip = (*mgr).chip[i as usize];
        if (*chip).nb_streams_play != 0 {
            pipe = &mut (*chip).playback_pipe;
            err = pcxhr_dsp_allocate_pipe(mgr, pipe, 0, i * 2);
            if err != 0 {
                return err;
            }
            j = 0;
            while j < (*chip).nb_streams_play {
                (*chip).playback_stream[j as usize].pipe = pipe;
                j += 1;
            }
        }
        j = 0;
        while j < (*chip).nb_streams_capt {
            pipe = &mut (*chip).capture_pipe[j as usize];
            err = pcxhr_dsp_allocate_pipe(mgr, pipe, 1, i * 2 + j);
            if err != 0 {
                return err;
            }
            (*chip).capture_stream[j as usize].pipe = pipe;
            j += 1;
        }
        i += 1;
    }
    0
}

unsafe extern "C" fn pcxhr_start_pipes(mgr: *mut pcxhr_mgr) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int;
    let mut chip: *mut snd_pcxhr;
    let mut playback_mask: ::core::ffi::c_int = 0;
    let mut capture_mask: ::core::ffi::c_int = 0;

    /* start all the pipes on the dsp */
    i = 0;
    while i < (*mgr).num_cards {
        chip = (*mgr).chip[i as usize];
        if (*chip).nb_streams_play != 0 {
            playback_mask |= 1 << (*chip).playback_pipe.first_audio;
        }
        j = 0;
        while j < (*chip).nb_streams_capt {
            capture_mask |= 1 << (*chip).capture_pipe[j as usize].first_audio;
            j += 1;
        }
        i += 1;
    }
    pcxhr_set_pipe_state(mgr, playback_mask, capture_mask, 1)
}

unsafe extern "C" fn pcxhr_dsp_load(
    mgr: *mut pcxhr_mgr,
    index: ::core::ffi::c_int,
    dsp: *const firmware,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int;
    let mut card_index: ::core::ffi::c_int;

    dev_dbg(
        &mut (*(*mgr).pci).dev,
        b"loading dsp [%d] size = %zd\n\0".as_ptr() as *const _,
        index,
        (*dsp).size,
    );

    if index == PCXHR_FIRMWARE_XLX_INT_INDEX {
        pcxhr_reset_xilinx_com(mgr);
        return pcxhr_load_xilinx_binary(mgr, dsp, 0);
    } else if index == PCXHR_FIRMWARE_XLX_COM_INDEX {
        pcxhr_reset_xilinx_com(mgr);
        return pcxhr_load_xilinx_binary(mgr, dsp, 1);
    } else if index == PCXHR_FIRMWARE_DSP_EPRM_INDEX {
        pcxhr_reset_dsp(mgr);
        return pcxhr_load_eeprom_binary(mgr, dsp);
    } else if index == PCXHR_FIRMWARE_DSP_BOOT_INDEX {
        return pcxhr_load_boot_binary(mgr, dsp);
    } else if index == PCXHR_FIRMWARE_DSP_MAIN_INDEX {
        err = pcxhr_load_dsp_binary(mgr, dsp);
        if err != 0 {
            return err;
        }
    } else {
        dev_err(&mut (*(*mgr).pci).dev, b"wrong file index\n\0".as_ptr() as *const _);
        return -EFAULT;
    } /* end of switch file index*/

    /* first communication with embedded */
    err = pcxhr_init_board(mgr);
    if err < 0 {
        dev_err(&mut (*(*mgr).pci).dev, b"pcxhr could not be set up\n\0".as_ptr() as *const _);
        return err;
    }
    err = pcxhr_config_pipes(mgr);
    if err < 0 {
        dev_err(&mut (*(*mgr).pci).dev, b"pcxhr pipes could not be set up\n\0".as_ptr() as *const _);
        return err;
    }
    /* create devices and mixer in accordance with HW options*/
    card_index = 0;
    while card_index < (*mgr).num_cards {
        let chip: *mut snd_pcxhr = (*mgr).chip[card_index as usize];

        err = pcxhr_create_pcm(chip);
        if err < 0 {
            return err;
        }

        if card_index == 0 {
            err = pcxhr_create_mixer((*chip).mgr);
            if err < 0 {
                return err;
            }
        }
        err = snd_card_register((*chip).card);
        if err < 0 {
            return err;
        }
        card_index += 1;
    }
    err = pcxhr_start_pipes(mgr);
    if err < 0 {
        dev_err(
            &mut (*(*mgr).pci).dev,
            b"pcxhr pipes could not be started\n\0".as_ptr() as *const _,
        );
        return err;
    }
    dev_dbg(
        &mut (*(*mgr).pci).dev,
        b"pcxhr firmware downloaded and successfully set up\n\0".as_ptr() as *const _,
    );

    0
}

/*
 * fw loader entry
 */
pub unsafe extern "C" fn pcxhr_setup_firmware(mgr: *mut pcxhr_mgr) -> ::core::ffi::c_int {
    static FW_0_0: &[u8] = b"xlxint.dat\0";
    static FW_0_1: &[u8] = b"xlxc882hr.dat\0";
    static FW_0_2: &[u8] = b"dspe882.e56\0";
    static FW_0_3: &[u8] = b"dspb882hr.b56\0";
    static FW_0_4: &[u8] = b"dspd882.d56\0";
    static FW_1_1: &[u8] = b"xlxc882e.dat\0";
    static FW_1_3: &[u8] = b"dspb882e.b56\0";
    static FW_2_1: &[u8] = b"xlxc1222hr.dat\0";
    static FW_2_3: &[u8] = b"dspb1222hr.b56\0";
    static FW_2_4: &[u8] = b"dspd1222.d56\0";
    static FW_3_1: &[u8] = b"xlxc1222e.dat\0";
    static FW_3_3: &[u8] = b"dspb1222e.b56\0";
    static FW_4_1: &[u8] = b"xlxc222.dat\0";
    static FW_4_2: &[u8] = b"dspe924.e56\0";
    static FW_4_3: &[u8] = b"dspb924.b56\0";
    static FW_4_4: &[u8] = b"dspd222.d56\0";
    static FW_5_1: &[u8] = b"xlxc924.dat\0";

    static FW_FILES: [[*const ::core::ffi::c_char; 5]; 6] = [
        [
            FW_0_0.as_ptr() as *const _,
            FW_0_1.as_ptr() as *const _,
            FW_0_2.as_ptr() as *const _,
            FW_0_3.as_ptr() as *const _,
            FW_0_4.as_ptr() as *const _,
        ],
        [
            FW_0_0.as_ptr() as *const _,
            FW_1_1.as_ptr() as *const _,
            FW_0_2.as_ptr() as *const _,
            FW_1_3.as_ptr() as *const _,
            FW_0_4.as_ptr() as *const _,
        ],
        [
            FW_0_0.as_ptr() as *const _,
            FW_2_1.as_ptr() as *const _,
            FW_0_2.as_ptr() as *const _,
            FW_2_3.as_ptr() as *const _,
            FW_2_4.as_ptr() as *const _,
        ],
        [
            FW_0_0.as_ptr() as *const _,
            FW_3_1.as_ptr() as *const _,
            FW_0_2.as_ptr() as *const _,
            FW_3_3.as_ptr() as *const _,
            FW_2_4.as_ptr() as *const _,
        ],
        [
            ::core::ptr::null(),
            FW_4_1.as_ptr() as *const _,
            FW_4_2.as_ptr() as *const _,
            FW_4_3.as_ptr() as *const _,
            FW_4_4.as_ptr() as *const _,
        ],
        [
            ::core::ptr::null(),
            FW_5_1.as_ptr() as *const _,
            FW_4_2.as_ptr() as *const _,
            FW_4_3.as_ptr() as *const _,
            FW_4_4.as_ptr() as *const _,
        ],
    ];
    let mut path: [::core::ffi::c_char; 32] = [0; 32];

    let mut i: ::core::ffi::c_int;
    let mut err: ::core::ffi::c_int;
    let fw_set: ::core::ffi::c_int = (*mgr).fw_file_set;

    i = 0;
    while i < 5 {
        if FW_FILES[fw_set as usize][i as usize].is_null() {
            i += 1;
            continue;
        }
        sprintf(
            path.as_mut_ptr(),
            b"pcxhr/%s\0".as_ptr() as *const _,
            FW_FILES[fw_set as usize][i as usize],
        );
        let mut fw_entry: *const firmware = ::core::ptr::null();
        if request_firmware(&mut fw_entry, path.as_ptr(), &mut (*(*mgr).pci).dev) != 0 {
            dev_err(
                &mut (*(*mgr).pci).dev,
                b"pcxhr: can't load firmware %s\n\0".as_ptr() as *const _,
                path.as_ptr(),
            );
            return -ENOENT;
        }
        /* fake hwdep dsp record */
        err = pcxhr_dsp_load(mgr, i, fw_entry);
        release_firmware(fw_entry);
        if err < 0 {
            return err;
        }
        (*mgr).dsp_loaded |= 1 << i;
        i += 1;
    }
    0
}

/* MODULE_FIRMWARE declarations preserved from the C source:
 * pcxhr/xlxint.dat
 * pcxhr/xlxc882hr.dat
 * pcxhr/xlxc882e.dat
 * pcxhr/dspe882.e56
 * pcxhr/dspb882hr.b56
 * pcxhr/dspb882e.b56
 * pcxhr/dspd882.d56
 *
 * pcxhr/xlxc1222hr.dat
 * pcxhr/xlxc1222e.dat
 * pcxhr/dspb1222hr.b56
 * pcxhr/dspb1222e.b56
 * pcxhr/dspd1222.d56
 *
 * pcxhr/xlxc222.dat
 * pcxhr/xlxc924.dat
 * pcxhr/dspe924.e56
 * pcxhr/dspb924.b56
 * pcxhr/dspd222.d56
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
