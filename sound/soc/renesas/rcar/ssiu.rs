// SPDX-License-Identifier: GPL-2.0
//
// Renesas R-Car SSIU support
//
// Copyright (c) 2015 Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>

// Dependencies from "rsnd.h" are provided by the surrounding driver.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type u32 = core::ffi::c_uint;
type c_int = core::ffi::c_int;
type c_char = core::ffi::c_char;
type c_void = core::ffi::c_void;
type size_t = usize;

const SSIU_NAME: *const c_char = b"ssiu\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct rsnd_mod {
    pub type_: rsnd_mod_type,
    pub clk: *mut c_void,
    pub rstc: *mut reset_control,
}

#[repr(C)]
pub struct rsnd_ssiu {
    pub mod_: rsnd_mod,
    pub busif_status: [u32; 8], /* for BUSIF0 - BUSIF7 */
    pub usrcnt: core::ffi::c_uint,
    pub id: c_int,
    pub id_sub: c_int,
}

/* SSI_MODE */
const TDM_EXT: u32 = 1 << 0;
const TDM_SPLIT: u32 = 1 << 8;

unsafe fn rsnd_ssiu_nr(priv_: *mut rsnd_priv) -> c_int {
    (*priv_).ssiu_nr
}

unsafe fn rsnd_mod_to_ssiu(mod_: *mut rsnd_mod) -> *mut rsnd_ssiu {
    (mod_ as *mut u8).sub(core::mem::offset_of!(rsnd_ssiu, mod_)) as *mut rsnd_ssiu
}

/*
 *	SSI	Gen2		Gen3		Gen4		RZ/G3E
 *	0	BUSIF0-3	BUSIF0-7	BUSIF0-7	BUSIF0-3
 *	1	BUSIF0-3	BUSIF0-7			BUSIF0-3
 *	2	BUSIF0-3	BUSIF0-7			BUSIF0-3
 *	3	BUSIF0		BUSIF0-7			BUSIF0-3
 *	4	BUSIF0		BUSIF0-7			BUSIF0-3
 *	5	BUSIF0		BUSIF0				BUSIF0
 *	6	BUSIF0		BUSIF0				BUSIF0
 *	7	BUSIF0		BUSIF0				BUSIF0
 *	8	BUSIF0		BUSIF0				BUSIF0
 *	9	BUSIF0-3	BUSIF0-7			BUSIF0-3
 *	total	22		52		8		28
 */
static gen2_id: [c_int; 10] = [0, 4, 8, 12, 13, 14, 15, 16, 17, 18];
static gen3_id: [c_int; 10] = [0, 8, 16, 24, 32, 40, 41, 42, 43, 44];
static gen4_id: [c_int; 1] = [0];
static rzg3e_id: [c_int; 10] = [0, 4, 8, 12, 16, 20, 21, 22, 23, 24];

#[repr(C)]
pub struct rsnd_ssiu_ctrl {
    pub busif_status_count: core::ffi::c_uint,
}

unsafe fn rsnd_priv_to_ssiu_ctrl(priv_: *mut rsnd_priv) -> *mut rsnd_ssiu_ctrl {
    (*priv_).ssiu_ctrl as *mut rsnd_ssiu_ctrl
}

/* enable busif buffer over/under run interrupt. */
unsafe fn rsnd_ssiu_busif_err_irq_enable(mod_: *mut rsnd_mod) {
    rsnd_ssiu_busif_err_irq_ctrl(mod_, 1);
}

unsafe fn rsnd_ssiu_busif_err_irq_disable(mod_: *mut rsnd_mod) {
    rsnd_ssiu_busif_err_irq_ctrl(mod_, 0);
}

unsafe fn rsnd_ssiu_busif_err_irq_ctrl(mod_: *mut rsnd_mod, enable: c_int) {
    let priv_ = rsnd_mod_to_priv(mod_);
    let id = rsnd_mod_id(mod_);
    let shift: c_int;
    let offset: c_int;

    match id {
        0 | 1 | 2 | 3 | 4 => {
            shift = id;
            offset = 0;
        }
        9 => {
            shift = 1;
            offset = 1;
        }
        _ => return,
    }

    let mut i: core::ffi::c_uint = 0;
    while i < (*rsnd_priv_to_ssiu_ctrl(priv_)).busif_status_count {
        let reg = SSI_SYS_INT_ENABLE((i * 2).wrapping_add(offset as u32));
        let val: u32 = 0xfu32 << (shift * 4);
        let mut sys_int_enable = rsnd_mod_read(mod_, reg);

        if enable != 0 {
            sys_int_enable |= val;
        } else {
            sys_int_enable &= !val;
        }
        rsnd_mod_write(mod_, reg, sys_int_enable);
        i = i.wrapping_add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_ssiu_busif_err_status_clear(mod_: *mut rsnd_mod) -> bool {
    let priv_ = rsnd_mod_to_priv(mod_);
    let mut error = false;
    let id = rsnd_mod_id(mod_);
    let shift: c_int;
    let offset: c_int;

    match id {
        0 | 1 | 2 | 3 | 4 => {
            shift = id;
            offset = 0;
        }
        9 => {
            shift = 1;
            offset = 1;
        }
        _ => return error,
    }

    let mut i: core::ffi::c_uint = 0;
    while i < (*rsnd_priv_to_ssiu_ctrl(priv_)).busif_status_count {
        let reg: u32 = SSI_SYS_STATUS(i * 2).wrapping_add(offset as u32);
        let mut status = rsnd_mod_read(mod_, reg);
        let val: u32 = 0xfu32 << (shift * 4);

        status &= val;
        if status != 0 {
            let dev = rsnd_priv_to_dev(priv_);

            rsnd_print_irq_status(
                dev,
                b"%s err status : 0x%08x\n\0".as_ptr() as *const c_char,
                rsnd_mod_name(mod_),
                status,
            );
            error = true;
        }
        rsnd_mod_write(mod_, reg, val);
        i = i.wrapping_add(1);
    }

    error
}

unsafe extern "C" fn rsnd_ssiu_get_status(
    mod_: *mut rsnd_mod,
    _io: *mut rsnd_dai_stream,
    _type: rsnd_mod_type,
) -> *mut u32 {
    let ssiu = rsnd_mod_to_ssiu(mod_);
    let busif = rsnd_mod_id_sub(mod_);

    &mut (*ssiu).busif_status[busif as usize] as *mut u32
}

unsafe extern "C" fn rsnd_ssiu_init(
    mod_: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    priv_: *mut rsnd_priv,
) -> c_int {
    let rdai = rsnd_io_to_rdai(io);
    let mut ssis = rsnd_ssi_multi_secondaries_runtime(io);
    let use_busif = rsnd_ssi_use_busif(io);
    let id = rsnd_mod_id(mod_);
    let is_clk_master = rsnd_rdai_is_clk_master(rdai);
    let mut val1: u32;
    let mut val2: u32;

    /* clear status */
    rsnd_ssiu_busif_err_status_clear(mod_);

    /* Gen4 doesn't have SSI_MODE */
    if rsnd_is_gen4(priv_) != 0 {
        rsnd_ssiu_busif_err_irq_enable(mod_);
        return 0;
    }

    /*
     * SSI_MODE0
     */
    if rsnd_is_rzg3e(priv_) == 0 {
        rsnd_mod_bset(mod_, SSI_MODE0, 1u32 << id, ((!use_busif) as u32) << id);
    }

    /*
     * SSI_MODE1 / SSI_MODE2
     *
     * FIXME
     * sharing/multi with SSI0 are mainly supported
     */
    val1 = rsnd_mod_read(mod_, SSI_MODE1);
    val2 = rsnd_mod_read(mod_, SSI_MODE2);
    if rsnd_ssi_is_pin_sharing(io) != 0 {
        ssis |= 1u32 << id;
    } else if ssis != 0 {
        /*
         * Multi SSI
         *
         * set synchronized bit here
         */

        /* SSI4 is synchronized with SSI3 */
        if (ssis & (1 << 4)) != 0 {
            val1 |= 1 << 20;
        }
        /* SSI012 are synchronized */
        if ssis == 0x0006 {
            val1 |= 1 << 4;
        }
        /* SSI0129 are synchronized */
        if ssis == 0x0206 {
            val2 |= 1 << 4;
        }
    }

    /* SSI1 is sharing pin with SSI0 */
    if (ssis & (1 << 1)) != 0 {
        val1 |= if is_clk_master != 0 { 0x2 } else { 0x1 };
    }

    /* SSI2 is sharing pin with SSI0 */
    if (ssis & (1 << 2)) != 0 {
        val1 |= if is_clk_master != 0 { 0x2 << 2 } else { 0x1 << 2 };
    }
    /* SSI4 is sharing pin with SSI3 */
    if (ssis & (1 << 4)) != 0 {
        val1 |= if is_clk_master != 0 { 0x2 << 16 } else { 0x1 << 16 };
    }
    /* SSI9 is sharing pin with SSI0 */
    if (ssis & (1 << 9)) != 0 {
        val2 |= if is_clk_master != 0 { 0x2 } else { 0x1 };
    }

    rsnd_mod_bset(mod_, SSI_MODE1, 0x0013001f, val1);
    rsnd_mod_bset(mod_, SSI_MODE2, 0x00000017, val2);

    /*
     * Enable busif buffer over/under run interrupt.
     * It will be handled from ssi.c
     * see
     *	__rsnd_ssi_interrupt()
     */
    rsnd_ssiu_busif_err_irq_enable(mod_);

    0
}

unsafe extern "C" fn rsnd_ssiu_quit(
    mod_: *mut rsnd_mod,
    _io: *mut rsnd_dai_stream,
    _priv: *mut rsnd_priv,
) -> c_int {
    /* disable busif buffer over/under run interrupt. */
    rsnd_ssiu_busif_err_irq_disable(mod_);

    0
}

static mut rsnd_ssiu_ops_gen1: rsnd_mod_ops = rsnd_mod_ops {
    name: SSIU_NAME,
    dma_req: None,
    init: Some(rsnd_ssiu_init),
    quit: Some(rsnd_ssiu_quit),
    start: None,
    stop: None,
    get_status: Some(rsnd_ssiu_get_status),
    debug_info: None,
    id: None,
    id_sub: None,
};

unsafe extern "C" fn rsnd_ssiu_init_gen2(
    mod_: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    priv_: *mut rsnd_priv,
) -> c_int {
    let ssiu = rsnd_mod_to_ssiu(mod_);
    let has_hdmi0 = rsnd_flags_has(io as *mut c_void, RSND_STREAM_HDMI0);
    let has_hdmi1 = rsnd_flags_has(io as *mut c_void, RSND_STREAM_HDMI1);
    let ret: c_int;
    let mut mode: u32 = 0;

    ret = rsnd_ssiu_init(mod_, io, priv_);
    if ret < 0 {
        return ret;
    }

    (*ssiu).usrcnt = (*ssiu).usrcnt.wrapping_add(1);

    /*
     * TDM Extend/Split Mode
     * see
     *	rsnd_ssi_config_init()
     */
    if rsnd_runtime_is_tdm(io) != 0 {
        mode = TDM_EXT;
    } else if rsnd_runtime_is_tdm_split(io) != 0 {
        mode = TDM_SPLIT;
    }

    rsnd_mod_write(mod_, SSI_MODE, mode);

    if rsnd_ssi_use_busif(io) != 0 {
        let id = rsnd_mod_id(mod_);
        let busif = rsnd_mod_id_sub(mod_);
        let adinr_reg: rsnd_reg;
        let mode_reg: rsnd_reg;
        let dalign_reg: rsnd_reg;

        if (id == 9) && (busif >= 4) {
            adinr_reg = SSI9_BUSIF_ADINR(busif);
            mode_reg = SSI9_BUSIF_MODE(busif);
            dalign_reg = SSI9_BUSIF_DALIGN(busif);
        } else {
            adinr_reg = SSI_BUSIF_ADINR(busif);
            mode_reg = SSI_BUSIF_MODE(busif);
            dalign_reg = SSI_BUSIF_DALIGN(busif);
        }

        rsnd_mod_write(
            mod_,
            adinr_reg,
            rsnd_get_adinr_bit(mod_, io)
                | if rsnd_io_is_play(io) != 0 {
                    rsnd_runtime_channel_after_ctu(io)
                } else {
                    rsnd_runtime_channel_original(io)
                },
        );
        rsnd_mod_write(mod_, mode_reg, rsnd_get_busif_shift(io, mod_) | 1);
        rsnd_mod_write(mod_, dalign_reg, rsnd_get_dalign(mod_, io));
    }

    if has_hdmi0 != 0 || has_hdmi1 != 0 {
        let rsnd_ssi_array: [rsnd_mod_type; 3] = [
            RSND_MOD_SSIM1,
            RSND_MOD_SSIM2,
            RSND_MOD_SSIM3,
        ];
        let ssi_mod = rsnd_io_to_mod_ssi(io);
        let mut pos: *mut rsnd_mod;
        let mut val: u32;
        let mut i: c_int;

        i = rsnd_mod_id(ssi_mod);

        /* output all same SSI as default */
        val = ((i as u32) << 16)
            | ((i as u32) << 20)
            | ((i as u32) << 24)
            | ((i as u32) << 28)
            | (i as u32);

        let mut idx: usize = 0;
        while idx < rsnd_ssi_array.len() {
            pos = rsnd_io_to_mod_by_type(io, rsnd_ssi_array[idx]);
            if !pos.is_null() {
                i = idx as c_int;
                let shift = (i * 4) + 20;

                val = (val & !(0xFu32 << shift)) | ((rsnd_mod_id(pos) as u32) << shift);
            }
            idx += 1;
        }

        if has_hdmi0 != 0 {
            rsnd_mod_write(mod_, HDMI0_SEL, val);
        }
        if has_hdmi1 != 0 {
            rsnd_mod_write(mod_, HDMI1_SEL, val);
        }
    }

    0
}

unsafe extern "C" fn rsnd_ssiu_start_gen2(
    mod_: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    _priv: *mut rsnd_priv,
) -> c_int {
    let busif = rsnd_mod_id_sub(mod_);

    if rsnd_ssi_use_busif(io) == 0 {
        return 0;
    }

    rsnd_mod_bset(mod_, SSI_CTRL, 1u32 << (busif * 4), 1u32 << (busif * 4));

    if rsnd_ssi_multi_secondaries_runtime(io) != 0 {
        rsnd_mod_write(mod_, SSI_CONTROL, 0x1);
    }

    0
}

unsafe extern "C" fn rsnd_ssiu_stop_gen2(
    mod_: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    _priv: *mut rsnd_priv,
) -> c_int {
    let ssiu = rsnd_mod_to_ssiu(mod_);
    let busif = rsnd_mod_id_sub(mod_);

    if rsnd_ssi_use_busif(io) == 0 {
        return 0;
    }

    rsnd_mod_bset(mod_, SSI_CTRL, 1u32 << (busif * 4), 0);

    (*ssiu).usrcnt = (*ssiu).usrcnt.wrapping_sub(1);
    if (*ssiu).usrcnt != 0 {
        return 0;
    }

    if rsnd_ssi_multi_secondaries_runtime(io) != 0 {
        rsnd_mod_write(mod_, SSI_CONTROL, 0);
    }

    0
}

unsafe extern "C" fn rsnd_ssiu_id(mod_: *mut rsnd_mod) -> c_int {
    let ssiu = rsnd_mod_to_ssiu(mod_);

    /* see rsnd_ssiu_probe() */
    (*ssiu).id
}

unsafe extern "C" fn rsnd_ssiu_id_sub(mod_: *mut rsnd_mod) -> c_int {
    let ssiu = rsnd_mod_to_ssiu(mod_);

    /* see rsnd_ssiu_probe() */
    (*ssiu).id_sub
}

unsafe extern "C" fn rsnd_ssiu_dma_req(
    io: *mut rsnd_dai_stream,
    mod_: *mut rsnd_mod,
) -> *mut dma_chan {
    let priv_ = rsnd_mod_to_priv(mod_);
    let is_play = rsnd_io_is_play(io);
    let name: *const c_char;

    /*
     * It should use "rcar_sound,ssiu" (R-Car) or "ssiu" (RZ/G3E) on DT.
     * We need to keep compatibility for old versions.
     *
     * If it has "rcar_sound.ssiu" or "ssiu", it will be used.
     * If not, "rcar_sound.ssi" or "ssi" will be used.
     * see
     *	rsnd_ssi_dma_req()
     *	rsnd_dma_of_path()
     */

    name = if is_play != 0 {
        b"rx\0".as_ptr() as *const c_char
    } else {
        b"tx\0".as_ptr() as *const c_char
    };

    rsnd_dma_request_channel(rsnd_ssiu_of_node(priv_), SSIU_NAME, mod_, name)
}

// CONFIG_DEBUG_FS conditional debugfs hook.
#[cfg(CONFIG_DEBUG_FS)]
unsafe extern "C" fn rsnd_ssiu_debug_info(
    m: *mut seq_file,
    _io: *mut rsnd_dai_stream,
    mod_: *mut rsnd_mod,
) {
    rsnd_debugfs_mod_reg_show(m, mod_, RSND_BASE_SSIU, rsnd_mod_id(mod_) as u32 * 0x80, 0x80);
}

static mut rsnd_ssiu_ops_gen2: rsnd_mod_ops = rsnd_mod_ops {
    name: SSIU_NAME,
    dma_req: Some(rsnd_ssiu_dma_req),
    init: Some(rsnd_ssiu_init_gen2),
    quit: Some(rsnd_ssiu_quit),
    start: Some(rsnd_ssiu_start_gen2),
    stop: Some(rsnd_ssiu_stop_gen2),
    get_status: Some(rsnd_ssiu_get_status),
    #[cfg(CONFIG_DEBUG_FS)]
    debug_info: Some(rsnd_ssiu_debug_info),
    #[cfg(not(CONFIG_DEBUG_FS))]
    debug_info: None,
    id: None,
    id_sub: None,
};

unsafe fn rsnd_ssiu_mod_get(priv_: *mut rsnd_priv, mut id: c_int) -> *mut rsnd_mod {
    if WARN_ON((id < 0 || id >= rsnd_ssiu_nr(priv_)) as c_int) != 0 {
        id = 0;
    }

    rsnd_mod_get(((*priv_).ssiu as *mut rsnd_ssiu).add(id as usize))
}

unsafe fn rsnd_parse_connect_ssiu_compatible(
    priv_: *mut rsnd_priv,
    io: *mut rsnd_dai_stream,
) {
    let ssi_mod = rsnd_io_to_mod_ssi(io);
    let mut ssiu: *mut rsnd_ssiu;
    let is_dma_mode: c_int;
    let mut i: c_int;

    if ssi_mod.is_null() {
        return;
    }

    is_dma_mode = rsnd_ssi_is_dma_mode(ssi_mod);

    /* select BUSIF0 */
    i = 0;
    while i < rsnd_ssiu_nr(priv_) {
        ssiu = ((*priv_).ssiu as *mut rsnd_ssiu).add(i as usize);
        let mod_ = rsnd_mod_get(ssiu);

        if is_dma_mode != 0
            && (rsnd_mod_id(ssi_mod) == rsnd_mod_id(mod_))
            && (rsnd_mod_id_sub(mod_) == 0)
        {
            rsnd_dai_connect(mod_, io, (*mod_).type_);
            return;
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_parse_connect_ssiu(
    rdai: *mut rsnd_dai,
    playback: *mut device_node,
    capture: *mut device_node,
) {
    let priv_ = rsnd_rdai_to_priv(rdai);
    let dev = rsnd_priv_to_dev(priv_);
    let node = rsnd_ssiu_of_node(priv_);
    let io_p = &mut (*rdai).playback as *mut rsnd_dai_stream;
    let io_c = &mut (*rdai).capture as *mut rsnd_dai_stream;

    /* use rcar_sound,ssiu if exist */
    if !node.is_null() {
        let mut i: c_int = 0;
        let mut np: *mut device_node = core::ptr::null_mut();

        while {
            np = for_each_child_of_node_scoped_next(node, np);
            !np.is_null()
        } {
            let mod_: *mut rsnd_mod;

            i = rsnd_node_fixed_index(dev, np, SSIU_NAME, i);
            if i < 0 {
                break;
            }

            mod_ = rsnd_ssiu_mod_get(priv_, i);

            if np == playback {
                rsnd_dai_connect(mod_, io_p, (*mod_).type_);
            }
            if np == capture {
                rsnd_dai_connect(mod_, io_c, (*mod_).type_);
            }
            i += 1;
        }

        of_node_put(node);
    }

    /* Keep DT compatibility */
    if rsnd_io_to_mod_ssiu(io_p).is_null() {
        rsnd_parse_connect_ssiu_compatible(priv_, io_p);
    }
    if rsnd_io_to_mod_ssiu(io_c).is_null() {
        rsnd_parse_connect_ssiu_compatible(priv_, io_c);
    }
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_ssiu_probe(priv_: *mut rsnd_priv) -> c_int {
    let dev = rsnd_priv_to_dev(priv_);
    let node = rsnd_ssiu_of_node(priv_);
    let rstc: *mut reset_control;
    let ctrl: *mut rsnd_ssiu_ctrl;
    let mut ssiu: *mut rsnd_ssiu;
    let ops: *mut rsnd_mod_ops;
    let mut list: *const c_int = core::ptr::null();
    let mut i: c_int;
    let mut nr: c_int;

    /*
     * Keep DT compatibility.
     * if it has "rcar_sound,ssiu", use it.
     * if not, use "rcar_sound,ssi"
     * see
     *	rsnd_ssiu_bufsif_to_id()
     */
    if !node.is_null() {
        nr = rsnd_node_count(priv_, node, SSIU_NAME);
    } else {
        nr = (*priv_).ssi_nr;
    }

    if nr == 0 {
        return -EINVAL;
    }

    ssiu = devm_kcalloc(dev, nr as size_t, core::mem::size_of::<rsnd_ssiu>(), GFP_KERNEL)
        as *mut rsnd_ssiu;
    if ssiu.is_null() {
        return -ENOMEM;
    }

    ctrl = devm_kzalloc(dev, core::mem::size_of::<rsnd_ssiu_ctrl>(), GFP_KERNEL)
        as *mut rsnd_ssiu_ctrl;
    if ctrl.is_null() {
        return -ENOMEM;
    }

    (*ctrl).busif_status_count = if rsnd_flags_has(
        priv_ as *mut c_void,
        RSND_SSIU_BUSIF_STATUS_COUNT_2,
    ) != 0
    {
        2
    } else {
        4
    };

    (*priv_).ssiu = ssiu as *mut c_void;
    (*priv_).ssiu_nr = nr;
    (*priv_).ssiu_ctrl = ctrl as *mut c_void;

    if rsnd_is_gen1(priv_) != 0 {
        ops = &mut rsnd_ssiu_ops_gen1;
    } else {
        ops = &mut rsnd_ssiu_ops_gen2;
    }

    /* Keep compatibility */
    nr = 0;
    if !node.is_null() && core::ptr::eq(ops, &raw mut rsnd_ssiu_ops_gen2) {
        (*ops).id = Some(rsnd_ssiu_id);
        (*ops).id_sub = Some(rsnd_ssiu_id_sub);

        if rsnd_is_gen2(priv_) != 0 {
            list = gen2_id.as_ptr();
            nr = gen2_id.len() as c_int;
        } else if rsnd_is_gen3(priv_) != 0 {
            list = gen3_id.as_ptr();
            nr = gen3_id.len() as c_int;
        } else if rsnd_is_gen4(priv_) != 0 {
            list = gen4_id.as_ptr();
            nr = gen4_id.len() as c_int;
        } else if rsnd_is_rzg3e(priv_) != 0 {
            list = rzg3e_id.as_ptr();
            nr = rzg3e_id.len() as c_int;
        } else {
            dev_err(dev, b"unknown SSIU\n\0".as_ptr() as *const c_char);
            return -ENODEV;
        }
    }

    /* Acquire shared reset once for all SSIU modules */
    rstc = devm_reset_control_get_optional_shared(dev, b"ssi-all\0".as_ptr() as *const c_char);
    if IS_ERR(rstc as *const c_void) != 0 {
        return dev_err_probe(
            dev,
            PTR_ERR(rstc as *const c_void) as c_int,
            b"failed to get ssi-all reset\n\0".as_ptr() as *const c_char,
        );
    }

    i = 0;
    while i < rsnd_ssiu_nr(priv_) {
        let ret: c_int;

        ssiu = ((*priv_).ssiu as *mut rsnd_ssiu).add(i as usize);
        if !node.is_null() {
            let mut j: c_int;

            /*
             * see
             *	rsnd_ssiu_get_id()
             *	rsnd_ssiu_get_id_sub()
             */
            j = 0;
            while j < nr {
                if *list.add(j as usize) > i {
                    break;
                }
                (*ssiu).id = j;
                (*ssiu).id_sub = i - *list.add((*ssiu).id as usize);
                j += 1;
            }
        } else {
            (*ssiu).id = i;
        }

        ret = rsnd_mod_init(
            priv_,
            rsnd_mod_get(ssiu),
            ops,
            core::ptr::null_mut(),
            rstc,
            RSND_MOD_SSIU,
            i,
        );
        if ret != 0 {
            return ret;
        }
        i += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_ssiu_remove(priv_: *mut rsnd_priv) {
    let mut ssiu: *mut rsnd_ssiu;
    let mut i: c_int;

    i = 0;
    while i < rsnd_ssiu_nr(priv_) {
        ssiu = ((*priv_).ssiu as *mut rsnd_ssiu).add(i as usize);
        rsnd_mod_quit(rsnd_mod_get(ssiu));
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_ssiu_suspend(priv_: *mut rsnd_priv) {
    let mut ssiu: *mut rsnd_ssiu;
    let mut i: c_int;

    i = 0;
    while i < rsnd_ssiu_nr(priv_) {
        ssiu = ((*priv_).ssiu as *mut rsnd_ssiu).add(i as usize);
        rsnd_suspend_clk_reset((*rsnd_mod_get(ssiu)).clk, (*rsnd_mod_get(ssiu)).rstc);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_ssiu_resume(priv_: *mut rsnd_priv) {
    let mut ssiu: *mut rsnd_ssiu;
    let mut i: c_int;

    i = 0;
    while i < rsnd_ssiu_nr(priv_) {
        ssiu = ((*priv_).ssiu as *mut rsnd_ssiu).add(i as usize);
        rsnd_resume_clk_reset((*rsnd_mod_get(ssiu)).clk, (*rsnd_mod_get(ssiu)).rstc);
        i += 1;
    }
}

#[repr(C)]
pub struct rsnd_priv {
    pub ssiu_nr: c_int,
    pub ssiu: *mut c_void,
    pub ssiu_ctrl: *mut c_void,
    pub ssi_nr: c_int,
}

#[repr(C)]
pub struct rsnd_dai {
    pub playback: rsnd_dai_stream,
    pub capture: rsnd_dai_stream,
}

#[repr(C)]
pub struct rsnd_dai_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rsnd_mod_ops {
    pub name: *const c_char,
    pub dma_req: Option<unsafe extern "C" fn(*mut rsnd_dai_stream, *mut rsnd_mod) -> *mut dma_chan>,
    pub init: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub quit: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub start: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub stop: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub get_status: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, rsnd_mod_type) -> *mut u32>,
    pub debug_info: Option<unsafe extern "C" fn(*mut seq_file, *mut rsnd_dai_stream, *mut rsnd_mod)>,
    pub id: Option<unsafe extern "C" fn(*mut rsnd_mod) -> c_int>,
    pub id_sub: Option<unsafe extern "C" fn(*mut rsnd_mod) -> c_int>,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct reset_control {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dma_chan {
    _private: [u8; 0],
}
#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

type rsnd_reg = u32;
type rsnd_mod_type = c_int;

extern "C" {
    static SSI_MODE0: rsnd_reg;
    static SSI_MODE1: rsnd_reg;
    static SSI_MODE2: rsnd_reg;
    static SSI_MODE: rsnd_reg;
    static SSI_CTRL: rsnd_reg;
    static SSI_CONTROL: rsnd_reg;
    static HDMI0_SEL: rsnd_reg;
    static HDMI1_SEL: rsnd_reg;
    static RSND_STREAM_HDMI0: c_int;
    static RSND_STREAM_HDMI1: c_int;
    static RSND_SSIU_BUSIF_STATUS_COUNT_2: c_int;
    static GFP_KERNEL: c_int;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static ENODEV: c_int;
    static RSND_MOD_SSIU: rsnd_mod_type;
    static RSND_MOD_SSIM1: rsnd_mod_type;
    static RSND_MOD_SSIM2: rsnd_mod_type;
    static RSND_MOD_SSIM3: rsnd_mod_type;
    static RSND_BASE_SSIU: c_int;

    fn SSI_SYS_INT_ENABLE(id: u32) -> rsnd_reg;
    fn SSI_SYS_STATUS(id: u32) -> rsnd_reg;
    fn SSI9_BUSIF_ADINR(busif: c_int) -> rsnd_reg;
    fn SSI9_BUSIF_MODE(busif: c_int) -> rsnd_reg;
    fn SSI9_BUSIF_DALIGN(busif: c_int) -> rsnd_reg;
    fn SSI_BUSIF_ADINR(busif: c_int) -> rsnd_reg;
    fn SSI_BUSIF_MODE(busif: c_int) -> rsnd_reg;
    fn SSI_BUSIF_DALIGN(busif: c_int) -> rsnd_reg;

    fn rsnd_mod_to_priv(mod_: *mut rsnd_mod) -> *mut rsnd_priv;
    fn rsnd_mod_id(mod_: *mut rsnd_mod) -> c_int;
    fn rsnd_mod_id_sub(mod_: *mut rsnd_mod) -> c_int;
    fn rsnd_mod_read(mod_: *mut rsnd_mod, reg: rsnd_reg) -> u32;
    fn rsnd_mod_write(mod_: *mut rsnd_mod, reg: rsnd_reg, data: u32);
    fn rsnd_mod_bset(mod_: *mut rsnd_mod, reg: rsnd_reg, mask: u32, data: u32);
    fn rsnd_priv_to_dev(priv_: *mut rsnd_priv) -> *mut device;
    fn rsnd_print_irq_status(dev: *mut device, fmt: *const c_char, ...);
    fn rsnd_mod_name(mod_: *mut rsnd_mod) -> *const c_char;
    fn rsnd_io_to_rdai(io: *mut rsnd_dai_stream) -> *mut rsnd_dai;
    fn rsnd_ssi_multi_secondaries_runtime(io: *mut rsnd_dai_stream) -> u32;
    fn rsnd_ssi_use_busif(io: *mut rsnd_dai_stream) -> c_int;
    fn rsnd_rdai_is_clk_master(rdai: *mut rsnd_dai) -> c_int;
    fn rsnd_is_gen4(priv_: *mut rsnd_priv) -> c_int;
    fn rsnd_is_rzg3e(priv_: *mut rsnd_priv) -> c_int;
    fn rsnd_ssi_is_pin_sharing(io: *mut rsnd_dai_stream) -> c_int;
    fn rsnd_runtime_is_tdm(io: *mut rsnd_dai_stream) -> c_int;
    fn rsnd_runtime_is_tdm_split(io: *mut rsnd_dai_stream) -> c_int;
    fn rsnd_flags_has(ptr: *mut c_void, flag: c_int) -> u32;
    fn rsnd_get_adinr_bit(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream) -> u32;
    fn rsnd_io_is_play(io: *mut rsnd_dai_stream) -> c_int;
    fn rsnd_runtime_channel_after_ctu(io: *mut rsnd_dai_stream) -> u32;
    fn rsnd_runtime_channel_original(io: *mut rsnd_dai_stream) -> u32;
    fn rsnd_get_busif_shift(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) -> u32;
    fn rsnd_get_dalign(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream) -> u32;
    fn rsnd_io_to_mod_ssi(io: *mut rsnd_dai_stream) -> *mut rsnd_mod;
    fn rsnd_io_to_mod_by_type(io: *mut rsnd_dai_stream, type_: rsnd_mod_type) -> *mut rsnd_mod;
    fn rsnd_ssiu_of_node(priv_: *mut rsnd_priv) -> *mut device_node;
    fn rsnd_dma_request_channel(
        node: *mut device_node,
        name: *const c_char,
        mod_: *mut rsnd_mod,
        dir: *const c_char,
    ) -> *mut dma_chan;
    fn rsnd_debugfs_mod_reg_show(
        m: *mut seq_file,
        mod_: *mut rsnd_mod,
        base: c_int,
        offset: u32,
        size: u32,
    );
    fn WARN_ON(condition: c_int) -> c_int;
    fn rsnd_mod_get(ssiu: *mut rsnd_ssiu) -> *mut rsnd_mod;
    fn rsnd_ssi_is_dma_mode(mod_: *mut rsnd_mod) -> c_int;
    fn rsnd_dai_connect(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, type_: rsnd_mod_type);
    fn rsnd_rdai_to_priv(rdai: *mut rsnd_dai) -> *mut rsnd_priv;
    fn rsnd_node_fixed_index(
        dev: *mut device,
        np: *mut device_node,
        name: *const c_char,
        i: c_int,
    ) -> c_int;
    fn for_each_child_of_node_scoped_next(
        node: *mut device_node,
        previous: *mut device_node,
    ) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn rsnd_io_to_mod_ssiu(io: *mut rsnd_dai_stream) -> *mut rsnd_mod;
    fn rsnd_node_count(priv_: *mut rsnd_priv, node: *mut device_node, name: *const c_char) -> c_int;
    fn devm_kcalloc(dev: *mut device, n: size_t, size: size_t, flags: c_int) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_int) -> *mut c_void;
    fn rsnd_is_gen1(priv_: *mut rsnd_priv) -> c_int;
    fn rsnd_is_gen2(priv_: *mut rsnd_priv) -> c_int;
    fn rsnd_is_gen3(priv_: *mut rsnd_priv) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_reset_control_get_optional_shared(
        dev: *mut device,
        id: *const c_char,
    ) -> *mut reset_control;
    fn IS_ERR(ptr: *const c_void) -> c_int;
    fn PTR_ERR(ptr: *const c_void) -> isize;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn rsnd_mod_init(
        priv_: *mut rsnd_priv,
        mod_: *mut rsnd_mod,
        ops: *mut rsnd_mod_ops,
        clk: *mut c_void,
        rstc: *mut reset_control,
        type_: rsnd_mod_type,
        id: c_int,
    ) -> c_int;
    fn rsnd_mod_quit(mod_: *mut rsnd_mod);
    fn rsnd_suspend_clk_reset(clk: *mut c_void, rstc: *mut reset_control);
    fn rsnd_resume_clk_reset(clk: *mut c_void, rstc: *mut reset_control);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
