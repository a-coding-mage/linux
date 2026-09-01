// SPDX-License-Identifier: GPL-2.0
//
// Renesas R-Car Audio DMAC support
//
// Copyright (C) 2015 Renesas Electronics Corp.
// Copyright (c) 2015 Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>

/*
 * Dependencies from the original C includes:
 * <linux/delay.h>
 * <linux/of_dma.h>
 * <sound/dmaengine_pcm.h>
 * "rsnd.h"
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::ManuallyDrop;

/*
 * Audio DMAC peri peri register
 */
const PDMASAR: u32 = 0x00;
const PDMADAR: u32 = 0x04;
const PDMACHCR: u32 = 0x0c;

/* PDMACHCR */
const PDMACHCR_DE: u32 = 1 << 0;

#[repr(C)]
pub struct rsnd_dmaen {
    chan: *mut dma_chan,
}

#[repr(C)]
pub struct rsnd_dmapp {
    dmapp_id: c_int,
    chcr: u32,
}

#[repr(C)]
pub union rsnd_dma_union {
    en: ManuallyDrop<rsnd_dmaen>,
    pp: ManuallyDrop<rsnd_dmapp>,
}

#[repr(C)]
pub struct rsnd_dma {
    mod_: rsnd_mod,
    mod_from: *mut rsnd_mod,
    mod_to: *mut rsnd_mod,
    src_addr: dma_addr_t,
    dst_addr: dma_addr_t,
    dma: rsnd_dma_union,
}

#[repr(C)]
pub struct rsnd_dma_ctrl {
    ppbase: *mut c_void,
    ppres: phys_addr_t,
    dmaen_num: c_int,
    dmapp_num: c_int,
    /* RZ/G3E: Audio DMAC peri-peri clock and reset */
    audmapp_clk: *mut clk,
    audmapp_rstc: *mut reset_control,
}

unsafe fn rsnd_priv_to_dmac(p: *mut rsnd_priv) -> *mut rsnd_dma_ctrl {
    unsafe { (*p).dma as *mut rsnd_dma_ctrl }
}

unsafe fn rsnd_mod_to_dma(_mod: *mut rsnd_mod) -> *mut rsnd_dma {
    container_of!(_mod, rsnd_dma, mod_)
}

unsafe fn rsnd_dma_to_dmaen(dma: *mut rsnd_dma) -> *mut rsnd_dmaen {
    unsafe { &raw mut (*dma).dma.en as *mut rsnd_dmaen }
}

unsafe fn rsnd_dma_to_dmapp(dma: *mut rsnd_dma) -> *mut rsnd_dmapp {
    unsafe { &raw mut (*dma).dma.pp as *mut rsnd_dmapp }
}

/* for DEBUG */
static mut mem_ops: rsnd_mod_ops = rsnd_mod_ops {
    name: c_str!("mem").as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

static mut mem: rsnd_mod = unsafe { core::mem::zeroed() };

/*
 *		Audio DMAC
 */
unsafe fn rsnd_dmaen_request_channel(
    io: *mut rsnd_dai_stream,
    mod_from: *mut rsnd_mod,
    mod_to: *mut rsnd_mod,
) -> *mut dma_chan {
    unsafe {
        if (mod_from.is_null() && mod_to.is_null()) || (!mod_from.is_null() && !mod_to.is_null()) {
            return core::ptr::null_mut();
        }

        if !mod_from.is_null() {
            rsnd_mod_dma_req(io, mod_from)
        } else {
            rsnd_mod_dma_req(io, mod_to)
        }
    }
}

unsafe fn rsnd_dmaen_stop(
    _mod: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    _priv: *mut rsnd_priv,
) -> c_int {
    unsafe { snd_dmaengine_pcm_trigger((*io).substream, SNDRV_PCM_TRIGGER_STOP) }
}

unsafe fn rsnd_dmaen_cleanup(
    _mod: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    _priv: *mut rsnd_priv,
) -> c_int {
    unsafe {
        let dma = rsnd_mod_to_dma(_mod);
        let dmaen = rsnd_dma_to_dmaen(dma);

        /*
         * DMAEngine release uses mutex lock.
         * Thus, it shouldn't be called under spinlock.
         * Let's call it under prepare
         */
        if !(*dmaen).chan.is_null() {
            snd_dmaengine_pcm_close_release_chan((*io).substream);
        }

        (*dmaen).chan = core::ptr::null_mut();

        0
    }
}

unsafe fn rsnd_dmaen_prepare(
    _mod: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    priv_: *mut rsnd_priv,
) -> c_int {
    unsafe {
        let dma = rsnd_mod_to_dma(_mod);
        let dmaen = rsnd_dma_to_dmaen(dma);
        let dev = rsnd_priv_to_dev(priv_);

        /* maybe suspended */
        if !(*dmaen).chan.is_null() {
            return 0;
        }

        /*
         * DMAEngine request uses mutex lock.
         * Thus, it shouldn't be called under spinlock.
         * Let's call it under prepare
         */
        (*dmaen).chan = rsnd_dmaen_request_channel(io, (*dma).mod_from, (*dma).mod_to);
        if IS_ERR_OR_NULL((*dmaen).chan as *const c_void) {
            (*dmaen).chan = core::ptr::null_mut();
            dev_err(dev, c_str!("can't get dma channel\n").as_ptr());
            return -EIO;
        }

        snd_dmaengine_pcm_open((*io).substream, (*dmaen).chan)
    }
}

unsafe fn rsnd_dmaen_start(
    _mod: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    priv_: *mut rsnd_priv,
) -> c_int {
    unsafe {
        let dma = rsnd_mod_to_dma(_mod);
        let dmaen = rsnd_dma_to_dmaen(dma);
        let dev = rsnd_priv_to_dev(priv_);
        let mut cfg: dma_slave_config = core::mem::zeroed();
        let mut buswidth: dma_slave_buswidth = DMA_SLAVE_BUSWIDTH_4_BYTES;
        let ret: c_int;

        /*
         * in case of monaural data writing or reading through Audio-DMAC
         * data is always in Left Justified format, so both src and dst
         * DMA Bus width need to be set equal to physical data width.
         */
        if rsnd_runtime_channel_original(io) == 1 {
            let runtime = rsnd_io_to_runtime(io);
            let bits = snd_pcm_format_physical_width((*runtime).format);

            match bits {
                8 => buswidth = DMA_SLAVE_BUSWIDTH_1_BYTE,
                16 => buswidth = DMA_SLAVE_BUSWIDTH_2_BYTES,
                32 => buswidth = DMA_SLAVE_BUSWIDTH_4_BYTES,
                _ => {
                    dev_err(dev, c_str!("invalid format width %d\n").as_ptr(), bits);
                    return -EINVAL;
                }
            }
        }

        cfg.direction = snd_pcm_substream_to_dma_direction((*io).substream);
        cfg.src_addr = (*dma).src_addr;
        cfg.dst_addr = (*dma).dst_addr;
        cfg.src_addr_width = buswidth;
        cfg.dst_addr_width = buswidth;

        dev_dbg(
            dev,
            c_str!("%s %pad -> %pad\n").as_ptr(),
            rsnd_mod_name(_mod),
            &raw const cfg.src_addr,
            &raw const cfg.dst_addr,
        );

        ret = dmaengine_slave_config((*dmaen).chan, &raw const cfg);
        if ret < 0 {
            return ret;
        }

        snd_dmaengine_pcm_trigger((*io).substream, SNDRV_PCM_TRIGGER_START)
    }
}

pub unsafe fn rsnd_dma_request_channel(
    of_node: *mut device_node,
    name: *mut c_char,
    _mod: *mut rsnd_mod,
    x: *mut c_char,
) -> *mut dma_chan {
    unsafe {
        let priv_ = rsnd_mod_to_priv(_mod);
        let dev = rsnd_priv_to_dev(priv_);
        let mut chan: *mut dma_chan = core::ptr::null_mut();
        let mut i: c_int = 0;

        for_each_child_of_node_scoped!(of_node, np, {
            i = rsnd_node_fixed_index(dev, np, name, i);
            if i < 0 {
                chan = core::ptr::null_mut();
                break;
            }

            if i == rsnd_mod_id_raw(_mod) && chan.is_null() {
                chan = of_dma_request_slave_channel(np, x);
            }
            i += 1;
        });

        /* It should call of_node_put(), since, it is rsnd_xxx_of_node() */
        of_node_put(of_node);

        chan
    }
}

unsafe fn rsnd_dmaen_attach(
    io: *mut rsnd_dai_stream,
    _dma: *mut rsnd_dma,
    mod_from: *mut rsnd_mod,
    mod_to: *mut rsnd_mod,
) -> c_int {
    unsafe {
        let priv_ = rsnd_io_to_priv(io);
        let dmac = rsnd_priv_to_dmac(priv_);
        let mut chan: *mut dma_chan;

        /* try to get DMAEngine channel */
        chan = rsnd_dmaen_request_channel(io, mod_from, mod_to);
        if IS_ERR_OR_NULL(chan as *const c_void) {
            /* Let's follow when -EPROBE_DEFER case */
            if PTR_ERR(chan as *const c_void) == -EPROBE_DEFER as isize {
                return PTR_ERR(chan as *const c_void) as c_int;
            }

            /*
             * DMA failed. try to PIO mode
             * see
             *	rsnd_ssi_fallback()
             *	rsnd_rdai_continuance_probe()
             */
            return -EAGAIN;
        }

        /*
         * use it for IPMMU if needed
         * see
         *	rsnd_preallocate_pages()
         */
        (*io).dmac_dev = (*(*chan).device).dev;

        dma_release_channel(chan);

        (*dmac).dmaen_num += 1;

        0
    }
}

unsafe fn rsnd_dmaen_pointer(
    _mod: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    pointer: *mut snd_pcm_uframes_t,
) -> c_int {
    unsafe {
        *pointer = snd_dmaengine_pcm_pointer((*io).substream);
        0
    }
}

static mut rsnd_dmaen_ops: rsnd_mod_ops = rsnd_mod_ops {
    name: c_str!("audmac").as_ptr(),
    prepare: Some(rsnd_dmaen_prepare),
    cleanup: Some(rsnd_dmaen_cleanup),
    start: Some(rsnd_dmaen_start),
    stop: Some(rsnd_dmaen_stop),
    pointer: Some(rsnd_dmaen_pointer),
    get_status: Some(rsnd_mod_get_status),
    ..unsafe { core::mem::zeroed() }
};

/*
 *		Audio DMAC peri peri
 */
static gen2_id_table_ssiu: [u8; 80] = [
    /* SSI00 ~ SSI07 */
    0x00, 0x01, 0x02, 0x03, 0x39, 0x3a, 0x3b, 0x3c,
    /* SSI10 ~ SSI17 */
    0x04, 0x05, 0x06, 0x07, 0x3d, 0x3e, 0x3f, 0x40,
    /* SSI20 ~ SSI27 */
    0x08, 0x09, 0x0a, 0x0b, 0x41, 0x42, 0x43, 0x44,
    /* SSI30 ~ SSI37 */
    0x0c, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b,
    /* SSI40 ~ SSI47 */
    0x0d, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52,
    /* SSI5 */
    0x0e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    /* SSI6 */
    0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    /* SSI7 */
    0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    /* SSI8 */
    0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    /* SSI90 ~ SSI97 */
    0x12, 0x13, 0x14, 0x15, 0x53, 0x54, 0x55, 0x56,
];

static gen2_id_table_scu: [u8; 10] = [
    0x2d, /* SCU_SRCI0 */
    0x2e, /* SCU_SRCI1 */
    0x2f, /* SCU_SRCI2 */
    0x30, /* SCU_SRCI3 */
    0x31, /* SCU_SRCI4 */
    0x32, /* SCU_SRCI5 */
    0x33, /* SCU_SRCI6 */
    0x34, /* SCU_SRCI7 */
    0x35, /* SCU_SRCI8 */
    0x36, /* SCU_SRCI9 */
];

static gen2_id_table_cmd: [u8; 2] = [
    0x37, /* SCU_CMD0 */
    0x38, /* SCU_CMD1 */
];

unsafe fn rsnd_dmapp_get_id(io: *mut rsnd_dai_stream, _mod: *mut rsnd_mod) -> u32 {
    unsafe {
        let ssi = rsnd_io_to_mod_ssi(io);
        let ssiu = rsnd_io_to_mod_ssiu(io);
        let src = rsnd_io_to_mod_src(io);
        let dvc = rsnd_io_to_mod_dvc(io);
        let mut entry: *const u8 = core::ptr::null();
        let mut id: c_int = 255;
        let mut size: c_int = 0;

        if _mod == ssi || _mod == ssiu {
            let busif = rsnd_mod_id_sub(ssiu);

            entry = gen2_id_table_ssiu.as_ptr();
            size = gen2_id_table_ssiu.len() as c_int;
            id = (rsnd_mod_id(_mod) * 8) + busif;
        } else if _mod == src {
            entry = gen2_id_table_scu.as_ptr();
            size = gen2_id_table_scu.len() as c_int;
            id = rsnd_mod_id(_mod);
        } else if _mod == dvc {
            entry = gen2_id_table_cmd.as_ptr();
            size = gen2_id_table_cmd.len() as c_int;
            id = rsnd_mod_id(_mod);
        }

        if entry.is_null() || size <= id {
            let dev = rsnd_priv_to_dev(rsnd_io_to_priv(io));

            dev_err(dev, c_str!("unknown connection (%s)\n").as_ptr(), rsnd_mod_name(_mod));

            /* use non-prohibited SRS number as error */
            return 0x00; /* SSI00 */
        }

        *entry.add(id as usize) as u32
    }
}

unsafe fn rsnd_dmapp_get_chcr(
    io: *mut rsnd_dai_stream,
    mod_from: *mut rsnd_mod,
    mod_to: *mut rsnd_mod,
) -> u32 {
    unsafe { (rsnd_dmapp_get_id(io, mod_from) << 24) + (rsnd_dmapp_get_id(io, mod_to) << 16) }
}

unsafe fn rsnd_dmapp_addr(dmac: *mut rsnd_dma_ctrl, dma: *mut rsnd_dma, reg: u32) -> *mut c_void {
    unsafe {
        ((*dmac).ppbase as *mut u8)
            .add((0x20 + reg + (0x10 * (*rsnd_dma_to_dmapp(dma)).dmapp_id as u32)) as usize)
            as *mut c_void
    }
}

unsafe fn rsnd_dmapp_write(dma: *mut rsnd_dma, data: u32, reg: u32) {
    unsafe {
        let mod_ = rsnd_mod_get(dma as *mut c_void);
        let priv_ = rsnd_mod_to_priv(mod_);
        let dmac = rsnd_priv_to_dmac(priv_);
        let dev = rsnd_priv_to_dev(priv_);

        dev_dbg(
            dev,
            c_str!("w 0x%px : %08x\n").as_ptr(),
            rsnd_dmapp_addr(dmac, dma, reg),
            data,
        );

        iowrite32(data, rsnd_dmapp_addr(dmac, dma, reg));
    }
}

unsafe fn rsnd_dmapp_read(dma: *mut rsnd_dma, reg: u32) -> u32 {
    unsafe {
        let mod_ = rsnd_mod_get(dma as *mut c_void);
        let priv_ = rsnd_mod_to_priv(mod_);
        let dmac = rsnd_priv_to_dmac(priv_);

        ioread32(rsnd_dmapp_addr(dmac, dma, reg))
    }
}

unsafe fn rsnd_dmapp_bset(dma: *mut rsnd_dma, data: u32, mask: u32, reg: u32) {
    unsafe {
        let mod_ = rsnd_mod_get(dma as *mut c_void);
        let priv_ = rsnd_mod_to_priv(mod_);
        let dmac = rsnd_priv_to_dmac(priv_);
        let addr = rsnd_dmapp_addr(dmac, dma, reg);
        let mut val = ioread32(addr);

        val &= !mask;
        val |= data & mask;

        iowrite32(val, addr);
    }
}

unsafe fn rsnd_dmapp_stop(
    _mod: *mut rsnd_mod,
    _io: *mut rsnd_dai_stream,
    _priv: *mut rsnd_priv,
) -> c_int {
    unsafe {
        let dma = rsnd_mod_to_dma(_mod);
        let mut i: c_int;

        rsnd_dmapp_bset(dma, 0, PDMACHCR_DE, PDMACHCR);

        i = 0;
        while i < 1024 {
            if 0 == (rsnd_dmapp_read(dma, PDMACHCR) & PDMACHCR_DE) {
                return 0;
            }
            udelay(1);
            i += 1;
        }

        -EIO
    }
}

unsafe fn rsnd_dmapp_start(
    _mod: *mut rsnd_mod,
    _io: *mut rsnd_dai_stream,
    _priv: *mut rsnd_priv,
) -> c_int {
    unsafe {
        let dma = rsnd_mod_to_dma(_mod);
        let dmapp = rsnd_dma_to_dmapp(dma);

        rsnd_dmapp_write(dma, (*dma).src_addr as u32, PDMASAR);
        rsnd_dmapp_write(dma, (*dma).dst_addr as u32, PDMADAR);
        rsnd_dmapp_write(dma, (*dmapp).chcr, PDMACHCR);

        0
    }
}

unsafe fn rsnd_dmapp_attach(
    io: *mut rsnd_dai_stream,
    dma: *mut rsnd_dma,
    mod_from: *mut rsnd_mod,
    mod_to: *mut rsnd_mod,
) -> c_int {
    unsafe {
        let dmapp = rsnd_dma_to_dmapp(dma);
        let priv_ = rsnd_io_to_priv(io);
        let dmac = rsnd_priv_to_dmac(priv_);
        let dev = rsnd_priv_to_dev(priv_);

        (*dmapp).dmapp_id = (*dmac).dmapp_num;
        (*dmapp).chcr = rsnd_dmapp_get_chcr(io, mod_from, mod_to) | PDMACHCR_DE;

        (*dmac).dmapp_num += 1;

        dev_dbg(
            dev,
            c_str!("id/src/dst/chcr = %d/%pad/%pad/%08x\n").as_ptr(),
            (*dmapp).dmapp_id,
            &raw const (*dma).src_addr,
            &raw const (*dma).dst_addr,
            (*dmapp).chcr,
        );

        0
    }
}

// CONFIG_DEBUG_FS conditional code from C is preserved here for builds that
// provide an equivalent Rust cfg.
#[cfg(CONFIG_DEBUG_FS)]
unsafe fn rsnd_dmapp_debug_info(
    m: *mut seq_file,
    io: *mut rsnd_dai_stream,
    _mod: *mut rsnd_mod,
) {
    unsafe {
        let priv_ = rsnd_mod_to_priv(_mod);
        let dmac = rsnd_priv_to_dmac(priv_);
        let dma = rsnd_mod_to_dma(_mod);
        let dmapp = rsnd_dma_to_dmapp(dma);

        rsnd_debugfs_reg_show(
            m,
            (*dmac).ppres,
            (*dmac).ppbase,
            0x20 + 0x10 * (*dmapp).dmapp_id as u32,
            0x10,
        );
    }
}

static mut rsnd_dmapp_ops: rsnd_mod_ops = rsnd_mod_ops {
    name: c_str!("audmac-pp").as_ptr(),
    start: Some(rsnd_dmapp_start),
    stop: Some(rsnd_dmapp_stop),
    quit: Some(rsnd_dmapp_stop),
    get_status: Some(rsnd_mod_get_status),
    #[cfg(CONFIG_DEBUG_FS)]
    debug_info: Some(rsnd_dmapp_debug_info),
    ..unsafe { core::mem::zeroed() }
};

#[repr(C)]
pub struct rsnd_dma_addr {
    out_addr: dma_addr_t,
    in_addr: dma_addr_t,
}

#[repr(C)]
pub struct rsnd_dma_addr_dir {
    capture: [rsnd_dma_addr; 3],
    playback: [rsnd_dma_addr; 3],
}

#[repr(C)]
pub struct rsnd_dma_addr_map {
    src: rsnd_dma_addr_dir,
    ssi: rsnd_dma_addr_dir,
    ssiu: rsnd_dma_addr_dir,
}

unsafe fn rsnd_dma_addr_lookup(
    io: *mut rsnd_dai_stream,
    _mod: *mut rsnd_mod,
    priv_: *mut rsnd_priv,
    map: *const rsnd_dma_addr_map,
    is_play: c_int,
    is_from: c_int,
) -> dma_addr_t {
    unsafe {
        let dev = rsnd_priv_to_dev(priv_);
        let mut is_ssi = ((rsnd_io_to_mod_ssi(io) == _mod) as c_int)
            || ((rsnd_io_to_mod_ssiu(io) == _mod) as c_int);
        let use_src = (!rsnd_io_to_mod_src(io).is_null()) as c_int;
        let use_cmd = (!rsnd_io_to_mod_dvc(io).is_null()
            || !rsnd_io_to_mod_mix(io).is_null()
            || !rsnd_io_to_mod_ctu(io).is_null()) as c_int;
        let id = rsnd_mod_id(_mod);
        let dir: *const rsnd_dma_addr_dir;
        let addr: *const rsnd_dma_addr;

        /* it shouldn't happen */
        if use_cmd != 0 && use_src == 0 {
            dev_err(dev, c_str!("DVC is selected without SRC\n").as_ptr());
        }

        /* use SSIU or SSI? */
        if is_ssi != 0 && rsnd_ssi_use_busif(io) != 0 {
            is_ssi += 1;
        }

        dev_dbg(
            dev,
            c_str!("dma%d addr : is_ssi=%d use_src=%d use_cmd=%d\n").as_ptr(),
            id,
            is_ssi,
            use_src,
            use_cmd,
        );

        match is_ssi {
            2 => dir = &raw const (*map).ssiu,
            1 => dir = &raw const (*map).ssi,
            _ => dir = &raw const (*map).src,
        }

        addr = if is_play != 0 {
            &raw const (*dir).playback[(use_src + use_cmd) as usize]
        } else {
            &raw const (*dir).capture[(use_src + use_cmd) as usize]
        };

        if is_from != 0 {
            (*addr).out_addr
        } else {
            (*addr).in_addr
        }
    }
}

/*
 *		Common DMAC Interface
 */

/*
 *	DMA read/write register offset
 *
 *	RSND_xxx_I_N	for Audio DMAC input
 *	RSND_xxx_O_N	for Audio DMAC output
 *	RSND_xxx_I_P	for Audio DMAC peri peri input
 *	RSND_xxx_O_P	for Audio DMAC peri peri output
 *
 *	ex) R-Car H2 case
 *	      mod        / DMAC in    / DMAC out   / DMAC PP in / DMAC pp out
 *	SSI : 0xec541000 / 0xec241008 / 0xec24100c
 *	SSIU: 0xec541000 / 0xec100000 / 0xec100000 / 0xec400000 / 0xec400000
 *	SCU : 0xec500000 / 0xec000000 / 0xec004000 / 0xec300000 / 0xec304000
 *	CMD : 0xec500000 /            / 0xec008000                0xec308000
 */
fn RDMA_SSI_I_N(addr_reg: phys_addr_t, i: c_int) -> dma_addr_t {
    (addr_reg - 0x00300000 + (0x40 * i as phys_addr_t) + 0x8) as dma_addr_t
}
fn RDMA_SSI_O_N(addr_reg: phys_addr_t, i: c_int) -> dma_addr_t {
    (addr_reg - 0x00300000 + (0x40 * i as phys_addr_t) + 0xc) as dma_addr_t
}
fn RDMA_SSIU_I_N(addr_reg: phys_addr_t, i: c_int, j: c_int) -> dma_addr_t {
    (addr_reg - 0x00441000
        + (0x1000 * i as phys_addr_t)
        + (((j / 4) as phys_addr_t) * 0xA000)
        + (((j % 4) as phys_addr_t) * 0x400)
        - (0x4000 * ((i / 9) as phys_addr_t) * ((j / 4) as phys_addr_t))) as dma_addr_t
}
fn RDMA_SSIU_O_N(addr_reg: phys_addr_t, i: c_int, j: c_int) -> dma_addr_t {
    RDMA_SSIU_I_N(addr_reg, i, j)
}
fn RDMA_SSIU_I_P(addr_reg: phys_addr_t, i: c_int, j: c_int) -> dma_addr_t {
    (addr_reg - 0x00141000
        + (0x1000 * i as phys_addr_t)
        + (((j / 4) as phys_addr_t) * 0xA000)
        + (((j % 4) as phys_addr_t) * 0x400)
        - (0x4000 * ((i / 9) as phys_addr_t) * ((j / 4) as phys_addr_t))) as dma_addr_t
}
fn RDMA_SSIU_O_P(addr_reg: phys_addr_t, i: c_int, j: c_int) -> dma_addr_t {
    RDMA_SSIU_I_P(addr_reg, i, j)
}
fn RDMA_SRC_I_N(addr_reg: phys_addr_t, i: c_int) -> dma_addr_t {
    (addr_reg - 0x00500000 + (0x400 * i as phys_addr_t)) as dma_addr_t
}
fn RDMA_SRC_O_N(addr_reg: phys_addr_t, i: c_int) -> dma_addr_t {
    (addr_reg - 0x004fc000 + (0x400 * i as phys_addr_t)) as dma_addr_t
}
fn RDMA_SRC_I_P(addr_reg: phys_addr_t, i: c_int) -> dma_addr_t {
    (addr_reg - 0x00200000 + (0x400 * i as phys_addr_t)) as dma_addr_t
}
fn RDMA_SRC_O_P(addr_reg: phys_addr_t, i: c_int) -> dma_addr_t {
    (addr_reg - 0x001fc000 + (0x400 * i as phys_addr_t)) as dma_addr_t
}
fn RDMA_CMD_O_N(addr_reg: phys_addr_t, i: c_int) -> dma_addr_t {
    (addr_reg - 0x004f8000 + (0x400 * i as phys_addr_t)) as dma_addr_t
}
fn RDMA_CMD_O_P(addr_reg: phys_addr_t, i: c_int) -> dma_addr_t {
    (addr_reg - 0x001f8000 + (0x400 * i as phys_addr_t)) as dma_addr_t
}

unsafe fn rsnd_gen2_dma_addr(
    io: *mut rsnd_dai_stream,
    _mod: *mut rsnd_mod,
    is_play: c_int,
    is_from: c_int,
) -> dma_addr_t {
    unsafe {
        let priv_ = rsnd_io_to_priv(io);
        let dev = rsnd_priv_to_dev(priv_);
        let ssi_reg = rsnd_gen_get_phy_addr(priv_, RSND_BASE_SSI);
        let src_reg = rsnd_gen_get_phy_addr(priv_, RSND_BASE_SCU);
        let id = rsnd_mod_id(_mod);
        let busif = rsnd_mod_id_sub(rsnd_io_to_mod_ssiu(io));
        let map = rsnd_dma_addr_map {
            src: rsnd_dma_addr_dir {
                capture: [
                    rsnd_dma_addr { out_addr: 0, in_addr: 0 },
                    rsnd_dma_addr { out_addr: RDMA_SRC_O_N(src_reg, id), in_addr: RDMA_SRC_I_P(src_reg, id) },
                    rsnd_dma_addr { out_addr: RDMA_CMD_O_N(src_reg, id), in_addr: RDMA_SRC_I_P(src_reg, id) },
                ],
                playback: [
                    rsnd_dma_addr { out_addr: 0, in_addr: 0 },
                    rsnd_dma_addr { out_addr: RDMA_SRC_O_P(src_reg, id), in_addr: RDMA_SRC_I_N(src_reg, id) },
                    rsnd_dma_addr { out_addr: RDMA_CMD_O_P(src_reg, id), in_addr: RDMA_SRC_I_N(src_reg, id) },
                ],
            },
            ssi: rsnd_dma_addr_dir {
                capture: [
                    rsnd_dma_addr { out_addr: RDMA_SSI_O_N(ssi_reg, id), in_addr: 0 },
                    rsnd_dma_addr { out_addr: RDMA_SSIU_O_P(ssi_reg, id, busif), in_addr: 0 },
                    rsnd_dma_addr { out_addr: RDMA_SSIU_O_P(ssi_reg, id, busif), in_addr: 0 },
                ],
                playback: [
                    rsnd_dma_addr { out_addr: 0, in_addr: RDMA_SSI_I_N(ssi_reg, id) },
                    rsnd_dma_addr { out_addr: 0, in_addr: RDMA_SSIU_I_P(ssi_reg, id, busif) },
                    rsnd_dma_addr { out_addr: 0, in_addr: RDMA_SSIU_I_P(ssi_reg, id, busif) },
                ],
            },
            ssiu: rsnd_dma_addr_dir {
                capture: [
                    rsnd_dma_addr { out_addr: RDMA_SSIU_O_N(ssi_reg, id, busif), in_addr: 0 },
                    rsnd_dma_addr { out_addr: RDMA_SSIU_O_P(ssi_reg, id, busif), in_addr: 0 },
                    rsnd_dma_addr { out_addr: RDMA_SSIU_O_P(ssi_reg, id, busif), in_addr: 0 },
                ],
                playback: [
                    rsnd_dma_addr { out_addr: 0, in_addr: RDMA_SSIU_I_N(ssi_reg, id, busif) },
                    rsnd_dma_addr { out_addr: 0, in_addr: RDMA_SSIU_I_P(ssi_reg, id, busif) },
                    rsnd_dma_addr { out_addr: 0, in_addr: RDMA_SSIU_I_P(ssi_reg, id, busif) },
                ],
            },
        };

        /*
         * FIXME
         *
         * We can't support SSI9-4/5/6/7, because its address is
         * out of calculation rule
         */
        if id == 9 && busif >= 4 {
            dev_err(
                dev,
                c_str!("This driver doesn't support SSI%d-%d, so far").as_ptr(),
                id,
                busif,
            );
        }

        rsnd_dma_addr_lookup(io, _mod, priv_, &raw const map, is_play, is_from)
    }
}

/*
 *	ex) G3E case
 *	      mod        / DMAC in    / DMAC out   / DMAC PP in / DMAC pp out
 *	SSI : 0x13C31000 / 0x13C40000 / 0x13C40000
 *	SSIU: 0x13C31000 / 0x13C40000 / 0x13C40000 / 0xEC400000 / 0xEC400000
 *	SCU : 0x13C00000 / 0x13C10000 / 0x13C14000 / 0xEC300000 / 0xEC304000
 *	CMD : 0x13C00000 /            / 0x13C18000                0xEC308000
 */

/* RZ/G3E DMA address macros */
fn RDMA_SSI_I_N_G3E(addr_reg: phys_addr_t, i: c_int) -> dma_addr_t {
    (addr_reg + 0x0000F000 + (0x1000 * i as phys_addr_t)) as dma_addr_t
}
fn RDMA_SSI_O_N_G3E(addr_reg: phys_addr_t, i: c_int) -> dma_addr_t {
    (addr_reg + 0x0000F000 + (0x1000 * i as phys_addr_t)) as dma_addr_t
}
fn RDMA_SSIU_I_N_G3E(addr_reg: phys_addr_t, i: c_int, j: c_int) -> dma_addr_t {
    (addr_reg + 0x0000F000
        + (0x1000 * i as phys_addr_t)
        + (((j / 4) as phys_addr_t) * 0xA000)
        + (((j % 4) as phys_addr_t) * 0x400)
        - (0x4000 * ((i / 9) as phys_addr_t) * ((j / 4) as phys_addr_t))) as dma_addr_t
}
fn RDMA_SSIU_O_N_G3E(addr_reg: phys_addr_t, i: c_int, j: c_int) -> dma_addr_t {
    RDMA_SSIU_I_N_G3E(addr_reg, i, j)
}
fn RDMA_SSIU_I_P_G3E(addr_reg: phys_addr_t, i: c_int, j: c_int) -> dma_addr_t {
    (addr_reg + 0xD87CF000
        + (0x1000 * i as phys_addr_t)
        + (((j / 4) as phys_addr_t) * 0xA000)
        + (((j % 4) as phys_addr_t) * 0x400)
        - (0x4000 * ((i / 9) as phys_addr_t) * ((j / 4) as phys_addr_t))) as dma_addr_t
}
fn RDMA_SSIU_O_P_G3E(addr_reg: phys_addr_t, i: c_int, j: c_int) -> dma_addr_t {
    RDMA_SSIU_I_P_G3E(addr_reg, i, j)
}
fn RDMA_SRC_I_N_G3E(addr_reg: phys_addr_t, i: c_int) -> dma_addr_t {
    (addr_reg + 0x00010000 + (0x400 * i as phys_addr_t)) as dma_addr_t
}
fn RDMA_SRC_O_N_G3E(addr_reg: phys_addr_t, i: c_int) -> dma_addr_t {
    (addr_reg + 0x00014000 + (0x400 * i as phys_addr_t)) as dma_addr_t
}
fn RDMA_SRC_I_P_G3E(addr_reg: phys_addr_t, i: c_int) -> dma_addr_t {
    (addr_reg + 0xD8700000 + (0x400 * i as phys_addr_t)) as dma_addr_t
}
fn RDMA_SRC_O_P_G3E(addr_reg: phys_addr_t, i: c_int) -> dma_addr_t {
    (addr_reg + 0xD8704000 + (0x400 * i as phys_addr_t)) as dma_addr_t
}
fn RDMA_CMD_O_N_G3E(addr_reg: phys_addr_t, i: c_int) -> dma_addr_t {
    (addr_reg + 0x00018000 + (0x400 * i as phys_addr_t)) as dma_addr_t
}
fn RDMA_CMD_O_P_G3E(addr_reg: phys_addr_t, i: c_int) -> dma_addr_t {
    (addr_reg + 0xD8708000 + (0x400 * i as phys_addr_t)) as dma_addr_t
}

unsafe fn rsnd_rzg3e_dma_addr(
    io: *mut rsnd_dai_stream,
    _mod: *mut rsnd_mod,
    is_play: c_int,
    is_from: c_int,
) -> dma_addr_t {
    unsafe {
        let priv_ = rsnd_io_to_priv(io);
        let ssi_reg = rsnd_gen_get_phy_addr(priv_, RSND_BASE_SSI);
        let src_reg = rsnd_gen_get_phy_addr(priv_, RSND_BASE_SCU);
        let id = rsnd_mod_id(_mod);
        let busif = rsnd_mod_id_sub(rsnd_io_to_mod_ssiu(io));
        let map = rsnd_dma_addr_map {
            src: rsnd_dma_addr_dir {
                capture: [
                    rsnd_dma_addr { out_addr: 0, in_addr: 0 },
                    rsnd_dma_addr { out_addr: RDMA_SRC_O_N_G3E(src_reg, id), in_addr: RDMA_SRC_I_P_G3E(src_reg, id) },
                    rsnd_dma_addr { out_addr: RDMA_CMD_O_N_G3E(src_reg, id), in_addr: RDMA_SRC_I_P_G3E(src_reg, id) },
                ],
                playback: [
                    rsnd_dma_addr { out_addr: 0, in_addr: 0 },
                    rsnd_dma_addr { out_addr: RDMA_SRC_O_P_G3E(src_reg, id), in_addr: RDMA_SRC_I_N_G3E(src_reg, id) },
                    rsnd_dma_addr { out_addr: RDMA_CMD_O_P_G3E(src_reg, id), in_addr: RDMA_SRC_I_N_G3E(src_reg, id) },
                ],
            },
            ssi: rsnd_dma_addr_dir {
                capture: [
                    rsnd_dma_addr { out_addr: RDMA_SSI_O_N_G3E(ssi_reg, id), in_addr: 0 },
                    rsnd_dma_addr { out_addr: RDMA_SSIU_O_P_G3E(ssi_reg, id, busif), in_addr: 0 },
                    rsnd_dma_addr { out_addr: RDMA_SSIU_O_P_G3E(ssi_reg, id, busif), in_addr: 0 },
                ],
                playback: [
                    rsnd_dma_addr { out_addr: 0, in_addr: RDMA_SSI_I_N_G3E(ssi_reg, id) },
                    rsnd_dma_addr { out_addr: 0, in_addr: RDMA_SSIU_I_P_G3E(ssi_reg, id, busif) },
                    rsnd_dma_addr { out_addr: 0, in_addr: RDMA_SSIU_I_P_G3E(ssi_reg, id, busif) },
                ],
            },
            ssiu: rsnd_dma_addr_dir {
                capture: [
                    rsnd_dma_addr { out_addr: RDMA_SSIU_O_N_G3E(ssi_reg, id, busif), in_addr: 0 },
                    rsnd_dma_addr { out_addr: RDMA_SSIU_O_P_G3E(ssi_reg, id, busif), in_addr: 0 },
                    rsnd_dma_addr { out_addr: RDMA_SSIU_O_P_G3E(ssi_reg, id, busif), in_addr: 0 },
                ],
                playback: [
                    rsnd_dma_addr { out_addr: 0, in_addr: RDMA_SSIU_I_N_G3E(ssi_reg, id, busif) },
                    rsnd_dma_addr { out_addr: 0, in_addr: RDMA_SSIU_I_P_G3E(ssi_reg, id, busif) },
                    rsnd_dma_addr { out_addr: 0, in_addr: RDMA_SSIU_I_P_G3E(ssi_reg, id, busif) },
                ],
            },
        };

        rsnd_dma_addr_lookup(io, _mod, priv_, &raw const map, is_play, is_from)
    }
}

/*
 *	Gen4 DMA read/write register offset
 *
 *	ex) R-Car V4H case
 *		  mod		/ SYS-DMAC in	/ SYS-DMAC out
 *	SSI_SDMC: 0xec400000	/ 0xec400000	/ 0xec400000
 */
fn RDMA_SSI_SDMC(addr: phys_addr_t, i: c_int) -> dma_addr_t {
    (addr + (0x8000 * i as phys_addr_t)) as dma_addr_t
}

unsafe fn rsnd_gen4_dma_addr(
    io: *mut rsnd_dai_stream,
    _mod: *mut rsnd_mod,
    _is_play: c_int,
    _is_from: c_int,
) -> dma_addr_t {
    unsafe {
        let priv_ = rsnd_io_to_priv(io);
        let addr = rsnd_gen_get_phy_addr(priv_, RSND_BASE_SDMC);
        let id = rsnd_mod_id(_mod);
        let busif = rsnd_mod_id_sub(_mod);

        /*
         * SSI0 only is supported
         */
        if id != 0 {
            let dev = rsnd_priv_to_dev(priv_);

            dev_err(dev, c_str!("This driver doesn't support non SSI0").as_ptr());
            return -EINVAL as dma_addr_t;
        }

        RDMA_SSI_SDMC(addr, busif)
    }
}

unsafe fn rsnd_dma_addr(
    io: *mut rsnd_dai_stream,
    _mod: *mut rsnd_mod,
    is_play: c_int,
    is_from: c_int,
) -> dma_addr_t {
    unsafe {
        let priv_ = rsnd_io_to_priv(io);

        if _mod.is_null() {
            return 0;
        }

        /*
         * gen1 uses default DMA addr
         */
        if rsnd_is_gen1(priv_) != 0 {
            0
        } else if rsnd_is_gen4(priv_) != 0 {
            rsnd_gen4_dma_addr(io, _mod, is_play, is_from)
        } else if rsnd_is_rzg3e(priv_) != 0 {
            rsnd_rzg3e_dma_addr(io, _mod, is_play, is_from)
        } else {
            rsnd_gen2_dma_addr(io, _mod, is_play, is_from)
        }
    }
}

const MOD_MAX: usize = (RSND_MOD_MAX + 1) as usize; /* +Memory */

unsafe fn rsnd_dma_of_path(
    mut this: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    is_play: c_int,
    mod_from: *mut *mut rsnd_mod,
    mod_to: *mut *mut rsnd_mod,
) {
    unsafe {
        let mut ssi: *mut rsnd_mod;
        let mut src = rsnd_io_to_mod_src(io);
        let mut ctu = rsnd_io_to_mod_ctu(io);
        let mut mix = rsnd_io_to_mod_mix(io);
        let mut dvc = rsnd_io_to_mod_dvc(io);
        let mut mod_: [*mut rsnd_mod; MOD_MAX] = [core::ptr::null_mut(); MOD_MAX];
        let mod_start: *mut rsnd_mod;
        let mod_end: *mut rsnd_mod;
        let priv_ = rsnd_mod_to_priv(this);
        let dev = rsnd_priv_to_dev(priv_);
        let mut nr: c_int;
        let mut i: c_int;
        let mut idx: c_int;

        /*
         * It should use "rcar_sound,ssiu" (R-Car) or "ssiu" (RZ/G3E) on DT.
         * We need to keep compatibility for old version.
         *
         * If it has "rcar_sound.ssiu" or "ssiu", it will be used.
         * If not, "rcar_sound.ssi" or "ssi" will be used.
         * see
         *	rsnd_ssiu_dma_req()
         *	rsnd_ssi_dma_req()
         */
        if !rsnd_ssiu_of_node(priv_).is_null() {
            let ssiu = rsnd_io_to_mod_ssiu(io);

            /* use SSIU */
            ssi = ssiu;
            if this == rsnd_io_to_mod_ssi(io) {
                this = ssiu;
            }
        } else {
            /* keep compatible, use SSI */
            ssi = rsnd_io_to_mod_ssi(io);
        }

        if ssi.is_null() {
            return;
        }

        nr = 0;
        i = 0;
        while i < MOD_MAX as c_int {
            mod_[i as usize] = core::ptr::null_mut();
            nr += (!rsnd_io_to_mod(io, i).is_null()) as c_int;
            i += 1;
        }

        /*
         * [S] -*-> [E]
         * [S] -*-> SRC -o-> [E]
         * [S] -*-> SRC -> DVC -o-> [E]
         * [S] -*-> SRC -> CTU -> MIX -> DVC -o-> [E]
         *
         * playback	[S] = mem
         *		[E] = SSI
         *
         * capture	[S] = SSI
         *		[E] = mem
         *
         * -*->		Audio DMAC
         * -o->		Audio DMAC peri peri
         */
        mod_start = if is_play != 0 { core::ptr::null_mut() } else { ssi };
        mod_end = if is_play != 0 { ssi } else { core::ptr::null_mut() };

        idx = 0;
        mod_[idx as usize] = mod_start;
        idx += 1;
        i = 1;
        while i < nr {
            if !src.is_null() {
                mod_[idx as usize] = src;
                idx += 1;
                src = core::ptr::null_mut();
            } else if !ctu.is_null() {
                mod_[idx as usize] = ctu;
                idx += 1;
                ctu = core::ptr::null_mut();
            } else if !mix.is_null() {
                mod_[idx as usize] = mix;
                idx += 1;
                mix = core::ptr::null_mut();
            } else if !dvc.is_null() {
                mod_[idx as usize] = dvc;
                idx += 1;
                dvc = core::ptr::null_mut();
            }
            i += 1;
        }
        mod_[idx as usize] = mod_end;

        /*
         *		| SSI | SRC |
         * -------------+-----+-----+
         *  is_play	|  o  |  *  |
         * !is_play	|  *  |  o  |
         */
        if ((this == ssi) as c_int) == is_play {
            *mod_from = mod_[(idx - 1) as usize];
            *mod_to = mod_[idx as usize];
        } else {
            *mod_from = mod_[0];
            *mod_to = mod_[1];
        }

        dev_dbg(dev, c_str!("module connection (this is %s)\n").as_ptr(), rsnd_mod_name(this));
        i = 0;
        while i <= idx {
            dev_dbg(
                dev,
                c_str!("  %s%s\n").as_ptr(),
                rsnd_mod_name(if !mod_[i as usize].is_null() { mod_[i as usize] } else { &raw mut mem }),
                if mod_[i as usize] == *mod_from {
                    c_str!(" from").as_ptr()
                } else if mod_[i as usize] == *mod_to {
                    c_str!(" to").as_ptr()
                } else {
                    c_str!("").as_ptr()
                },
            );
            i += 1;
        }
    }
}

unsafe fn rsnd_dma_alloc(
    io: *mut rsnd_dai_stream,
    _mod: *mut rsnd_mod,
    dma_mod: *mut *mut rsnd_mod,
) -> c_int {
    unsafe {
        let mut mod_from: *mut rsnd_mod = core::ptr::null_mut();
        let mut mod_to: *mut rsnd_mod = core::ptr::null_mut();
        let priv_ = rsnd_io_to_priv(io);
        let dmac = rsnd_priv_to_dmac(priv_);
        let dev = rsnd_priv_to_dev(priv_);
        let dma: *mut rsnd_dma;
        let mut ops: *mut rsnd_mod_ops;
        let mut type_: rsnd_mod_type;
        let mut attach: unsafe fn(*mut rsnd_dai_stream, *mut rsnd_dma, *mut rsnd_mod, *mut rsnd_mod) -> c_int;
        let is_play = rsnd_io_is_play(io);
        let mut ret: c_int;
        let mut dma_id: c_int;

        /*
         * DMA failed. try to PIO mode
         * see
         *	rsnd_ssi_fallback()
         *	rsnd_rdai_continuance_probe()
         */
        if dmac.is_null() {
            return -EAGAIN;
        }

        rsnd_dma_of_path(_mod, io, is_play, &raw mut mod_from, &raw mut mod_to);

        /* for Gen2 or later */
        if !mod_from.is_null() && !mod_to.is_null() {
            ops = &raw mut rsnd_dmapp_ops;
            attach = rsnd_dmapp_attach;
            dma_id = (*dmac).dmapp_num;
            type_ = RSND_MOD_AUDMAPP;
        } else {
            ops = &raw mut rsnd_dmaen_ops;
            attach = rsnd_dmaen_attach;
            dma_id = (*dmac).dmaen_num;
            type_ = RSND_MOD_AUDMA;
        }

        /* for Gen1, overwrite */
        if rsnd_is_gen1(priv_) != 0 {
            ops = &raw mut rsnd_dmaen_ops;
            attach = rsnd_dmaen_attach;
            dma_id = (*dmac).dmaen_num;
            type_ = RSND_MOD_AUDMA;
        }

        dma = devm_kzalloc(dev, core::mem::size_of::<rsnd_dma>(), GFP_KERNEL) as *mut rsnd_dma;
        if dma.is_null() {
            return -ENOMEM;
        }

        *dma_mod = rsnd_mod_get(dma as *mut c_void);

        ret = rsnd_mod_init(priv_, *dma_mod, ops, core::ptr::null_mut(), core::ptr::null_mut(), type_, dma_id);
        if ret < 0 {
            return ret;
        }

        dev_dbg(
            dev,
            c_str!("%s %s -> %s\n").as_ptr(),
            rsnd_mod_name(*dma_mod),
            rsnd_mod_name(if !mod_from.is_null() { mod_from } else { &raw mut mem }),
            rsnd_mod_name(if !mod_to.is_null() { mod_to } else { &raw mut mem }),
        );

        ret = attach(io, dma, mod_from, mod_to);
        if ret < 0 {
            return ret;
        }

        (*dma).src_addr = rsnd_dma_addr(io, mod_from, is_play, 1);
        (*dma).dst_addr = rsnd_dma_addr(io, mod_to, is_play, 0);
        (*dma).mod_from = mod_from;
        (*dma).mod_to = mod_to;

        0
    }
}

pub unsafe fn rsnd_dma_attach(
    io: *mut rsnd_dai_stream,
    _mod: *mut rsnd_mod,
    dma_mod: *mut *mut rsnd_mod,
) -> c_int {
    unsafe {
        if (*dma_mod).is_null() {
            let ret = rsnd_dma_alloc(io, _mod, dma_mod);

            if ret < 0 {
                return ret;
            }
        }

        rsnd_dai_connect(*dma_mod, io, (**dma_mod).type_)
    }
}

pub unsafe fn rsnd_dma_probe(priv_: *mut rsnd_priv) -> c_int {
    unsafe {
        let pdev = rsnd_priv_to_pdev(priv_);
        let dev = rsnd_priv_to_dev(priv_);
        let dmac: *mut rsnd_dma_ctrl;
        let res: *mut resource;

        /*
         * for Gen1
         */
        if rsnd_is_gen1(priv_) != 0 {
            return 0;
        }

        /*
         * for Gen2 or later
         */
        dmac = devm_kzalloc(dev, core::mem::size_of::<rsnd_dma_ctrl>(), GFP_KERNEL) as *mut rsnd_dma_ctrl;
        if dmac.is_null() {
            dev_err(dev, c_str!("dma allocate failed\n").as_ptr());
            return 0; /* it will be PIO mode */
        }

        /* for Gen4 doesn't have DMA-pp */
        if rsnd_is_gen4(priv_) == 0 {
            res = platform_get_resource_byname(pdev, IORESOURCE_MEM, c_str!("audmapp").as_ptr());
            if res.is_null() {
                dev_err(dev, c_str!("lack of audmapp in DT\n").as_ptr());
                return 0; /* it will be PIO mode */
            }

            /*
             * Audio DMAC peri-peri clock and reset for RZ/G3E.
             * These use optional APIs, so they gracefully return NULL
             * (no error) on platforms whose DT does not provide them.
             *
             * Enable the clock first so the block sees a stable clock on
             * the way out of reset, then deassert the reset line.
             */
            (*dmac).audmapp_clk = devm_clk_get_optional_enabled(dev, c_str!("audmapp").as_ptr());
            if IS_ERR((*dmac).audmapp_clk as *const c_void) {
                return dev_err_probe(
                    dev,
                    PTR_ERR((*dmac).audmapp_clk as *const c_void) as c_int,
                    c_str!("failed to get audmapp clock\n").as_ptr(),
                );
            }

            (*dmac).audmapp_rstc =
                devm_reset_control_get_optional_exclusive_deasserted(dev, c_str!("audmapp").as_ptr());
            if IS_ERR((*dmac).audmapp_rstc as *const c_void) {
                return dev_err_probe(
                    dev,
                    PTR_ERR((*dmac).audmapp_rstc as *const c_void) as c_int,
                    c_str!("failed to get audmapp reset\n").as_ptr(),
                );
            }

            (*dmac).dmapp_num = 0;
            (*dmac).ppres = (*res).start;
            (*dmac).ppbase = devm_ioremap_resource(dev, res);
            if IS_ERR((*dmac).ppbase as *const c_void) {
                return PTR_ERR((*dmac).ppbase as *const c_void) as c_int;
            }
        }

        (*priv_).dma = dmac as *mut c_void;

        /* dummy mem mod for debug */
        rsnd_mod_init(
            core::ptr::null_mut(),
            &raw mut mem,
            &raw mut mem_ops,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            0,
            0,
        )
    }
}

pub unsafe fn rsnd_dma_suspend(priv_: *mut rsnd_priv) {
    unsafe {
        let dmac = rsnd_priv_to_dmac(priv_);

        if !dmac.is_null() {
            /* Mirror probe (which enables clk before deasserting reset) */
            rsnd_suspend_clk_reset(core::ptr::null_mut(), (*dmac).audmapp_rstc);
            clk_disable_unprepare((*dmac).audmapp_clk);
        }
    }
}

pub unsafe fn rsnd_dma_resume(priv_: *mut rsnd_priv) {
    unsafe {
        let dmac = rsnd_priv_to_dmac(priv_);

        if !dmac.is_null() {
            /* Clock must be stable before reset is deasserted */
            clk_prepare_enable((*dmac).audmapp_clk);
            rsnd_resume_clk_reset(core::ptr::null_mut(), (*dmac).audmapp_rstc);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
