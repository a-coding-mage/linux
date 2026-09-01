// SPDX-License-Identifier: GPL-2.0
//
// Freescale P1022DS ALSA SoC Machine driver
//
// Author: Timur Tabi <timur@freescale.com>
//
// Copyright 2010 Freescale Semiconductor, Inc.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

/* Dependencies from:
 * linux/module.h, linux/fsl/guts.h, linux/interrupt.h, linux/of.h,
 * linux/of_address.h, linux/slab.h, sound/soc.h, fsl_dma.h, fsl_ssi.h,
 * and fsl_utils.h.
 */

type phys_addr_t = usize;
type u32 = u32;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const GFP_KERNEL: c_uint = 0;
const DAI_NAME_SIZE: usize = 32;

extern "C" {
    static THIS_MODULE: *mut module;

    fn clrsetbits_be32(addr: *mut u32, clear: u32, set: u32);
    fn clrbits32(addr: *mut u32, clear: u32);
    fn ioremap(offset: phys_addr_t, size: usize) -> *mut c_void;
    fn iounmap(addr: *mut c_void);

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(
        rtd: *mut snd_soc_pcm_runtime,
        num: c_int,
    ) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_uint,
    ) -> c_int;
    fn snd_soc_register_card(card: *mut snd_soc_card) -> c_int;
    fn snd_soc_unregister_card(card: *mut snd_soc_card);

    fn to_platform_device(dev: *mut device) -> *mut platform_device;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);

    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_get_property(
        np: *mut device_node,
        name: *const c_char,
        lenp: *mut c_int,
    ) -> *const c_void;
    fn of_node_put(node: *mut device_node);
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn of_address_to_resource(
        dev: *mut device_node,
        index: c_int,
        r: *mut resource,
    ) -> c_int;
    fn be32_to_cpup(p: *const u32) -> u32;

    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn fsl_asoc_get_dma_channel(
        np: *mut device_node,
        name: *const c_char,
        dai: *mut snd_soc_dai_link,
        dma_channel_id: *mut c_uint,
        dma_id: *mut c_uint,
    ) -> c_int;
}

#[repr(C)]
struct module {
    _private: [u8; 0],
}

#[repr(C)]
struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
struct ccsr_guts {
    _reserved0: [u8; 0],
    pmuxcr: u32,
    _reserved1: [u8; 0],
    dmuxcr: u32,
}

#[repr(C)]
struct device {
    parent: *mut device,
    of_node: *mut device_node,
}

#[repr(C)]
struct platform_device {
    dev: device,
    name: *const c_char,
}

#[repr(C)]
struct resource {
    start: phys_addr_t,
}

#[repr(C)]
struct snd_soc_dai_link_component {
    name: *const c_char,
    dai_name: *const c_char,
    of_node: *mut device_node,
}

#[repr(C)]
struct snd_soc_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

#[repr(C)]
struct snd_soc_dai_link {
    name: *const c_char,
    stream_name: *const c_char,
    cpus: *mut snd_soc_dai_link_component,
    codecs: *mut snd_soc_dai_link_component,
    platforms: *mut snd_soc_dai_link_component,
    num_cpus: c_uint,
    num_codecs: c_uint,
    num_platforms: c_uint,
    ops: *const snd_soc_ops,
}

#[repr(C)]
struct snd_soc_card {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    name: *const c_char,
    owner: *mut module,
    dev: *mut device,
    num_links: c_int,
    dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
}

#[repr(C)]
struct platform_driver_inner {
    name: *const c_char,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    driver: platform_driver_inner,
}

/* P1022-specific PMUXCR and DMUXCR bit definitions */

const CCSR_GUTS_PMUXCR_UART0_I2C1_MASK: u32 = 0x0001c000;
const CCSR_GUTS_PMUXCR_UART0_I2C1_UART0_SSI: u32 = 0x00010000;
const CCSR_GUTS_PMUXCR_UART0_I2C1_SSI: u32 = 0x00018000;

const CCSR_GUTS_PMUXCR_SSI_DMA_TDM_MASK: u32 = 0x00000c00;
const CCSR_GUTS_PMUXCR_SSI_DMA_TDM_SSI: u32 = 0x00000000;

const CCSR_GUTS_DMUXCR_PAD: c_uint = 1; /* DMA controller/channel set to pad */
const CCSR_GUTS_DMUXCR_SSI: c_uint = 2; /* DMA controller/channel set to SSI */

const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 0;
const SND_SOC_DAIFMT_AC97: c_uint = 0;
const SND_SOC_CLOCK_OUT: c_uint = 0;
const SND_SOC_CLOCK_IN: c_uint = 0;

/*
 * Set the DMACR register in the GUTS
 *
 * The DMACR register determines the source of initiated transfers for each
 * channel on each DMA controller.  Rather than have a bunch of repetitive
 * macros for the bit patterns, we just have a function that calculates
 * them.
 *
 * guts: Pointer to GUTS structure
 * co: The DMA controller (0 or 1)
 * ch: The channel on the DMA controller (0, 1, 2, or 3)
 * device: The device to set as the target (CCSR_GUTS_DMUXCR_xxx)
 */
unsafe fn guts_set_dmuxcr(guts: *mut ccsr_guts, co: c_uint, ch: c_uint, device: c_uint) {
    let shift: c_uint = 16 + (8 * (1u32.wrapping_sub(co)) + 2 * (3u32.wrapping_sub(ch)));

    clrsetbits_be32(
        ptr::addr_of_mut!((*guts).dmuxcr),
        3u32 << shift,
        device << shift,
    );
}

/* There's only one global utilities register */
static mut guts_phys: phys_addr_t = 0;

/**
 * machine_data: machine-specific ASoC device data
 *
 * This structure contains data for a single sound platform device on an
 * P1022 DS.  Some of the data is taken from the device tree.
 */
#[repr(C)]
struct machine_data {
    dai: [snd_soc_dai_link; 2],
    card: snd_soc_card,
    dai_format: c_uint,
    codec_clk_direction: c_uint,
    cpu_clk_direction: c_uint,
    clk_frequency: c_uint,
    ssi_id: c_uint,                 /* 0 = SSI1, 1 = SSI2, etc */
    dma_id: [c_uint; 2],            /* 0 = DMA1, 1 = DMA2, etc */
    dma_channel_id: [c_uint; 2],    /* 0 = ch 0, 1 = ch 1, etc*/
    platform_name: [[c_char; DAI_NAME_SIZE]; 2], /* One for each DMA channel */
}

unsafe fn card_to_mdata(card: *mut snd_soc_card) -> *mut machine_data {
    (card as *mut u8).sub(mem::offset_of!(machine_data, card)) as *mut machine_data
}

/**
 * p1022_ds_machine_probe: initialize the board
 *
 * This function is used to initialize the board-specific hardware.
 *
 * Here we program the DMACR and PMUXCR registers.
 */
unsafe extern "C" fn p1022_ds_machine_probe(card: *mut snd_soc_card) -> c_int {
    let mdata: *mut machine_data = card_to_mdata(card);
    let mut guts: *mut ccsr_guts;

    guts = ioremap(guts_phys, mem::size_of::<ccsr_guts>()) as *mut ccsr_guts;
    if guts.is_null() {
        dev_err((*card).dev, c"could not map global utilities\n".as_ptr());
        return -ENOMEM;
    }

    /* Enable SSI Tx signal */
    clrsetbits_be32(
        ptr::addr_of_mut!((*guts).pmuxcr),
        CCSR_GUTS_PMUXCR_UART0_I2C1_MASK,
        CCSR_GUTS_PMUXCR_UART0_I2C1_UART0_SSI,
    );

    /* Enable SSI Rx signal */
    clrsetbits_be32(
        ptr::addr_of_mut!((*guts).pmuxcr),
        CCSR_GUTS_PMUXCR_SSI_DMA_TDM_MASK,
        CCSR_GUTS_PMUXCR_SSI_DMA_TDM_SSI,
    );

    /* Enable DMA Channel for SSI */
    guts_set_dmuxcr(
        guts,
        (*mdata).dma_id[0],
        (*mdata).dma_channel_id[0],
        CCSR_GUTS_DMUXCR_SSI,
    );

    guts_set_dmuxcr(
        guts,
        (*mdata).dma_id[1],
        (*mdata).dma_channel_id[1],
        CCSR_GUTS_DMUXCR_SSI,
    );

    iounmap(guts as *mut c_void);

    0
}

/**
 * p1022_ds_startup: program the board with various hardware parameters
 *
 * This function takes board-specific information, like clock frequencies
 * and serial data formats, and passes that information to the codec and
 * transport drivers.
 */
unsafe extern "C" fn p1022_ds_startup(substream: *mut snd_pcm_substream) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let mdata: *mut machine_data = card_to_mdata((*rtd).card);
    let dev: *mut device = (*(*rtd).card).dev;
    let mut ret: c_int = 0;

    /* Tell the codec driver what the serial protocol is. */
    ret = snd_soc_dai_set_fmt(snd_soc_rtd_to_codec(rtd, 0), (*mdata).dai_format);
    if ret < 0 {
        dev_err(dev, c"could not set codec driver audio format\n".as_ptr());
        return ret;
    }

    /*
     * Tell the codec driver what the MCLK frequency is, and whether it's
     * a slave or master.
     */
    ret = snd_soc_dai_set_sysclk(
        snd_soc_rtd_to_codec(rtd, 0),
        0,
        (*mdata).clk_frequency,
        (*mdata).codec_clk_direction,
    );
    if ret < 0 {
        dev_err(dev, c"could not set codec driver clock params\n".as_ptr());
        return ret;
    }

    0
}

/**
 * p1022_ds_machine_remove: Remove the sound device
 *
 * This function is called to remove the sound device for one SSI.  We
 * de-program the DMACR and PMUXCR register.
 */
unsafe extern "C" fn p1022_ds_machine_remove(card: *mut snd_soc_card) -> c_int {
    let mdata: *mut machine_data = card_to_mdata(card);
    let mut guts: *mut ccsr_guts;

    guts = ioremap(guts_phys, mem::size_of::<ccsr_guts>()) as *mut ccsr_guts;
    if guts.is_null() {
        dev_err((*card).dev, c"could not map global utilities\n".as_ptr());
        return -ENOMEM;
    }

    /* Restore the signal routing */
    clrbits32(
        ptr::addr_of_mut!((*guts).pmuxcr),
        CCSR_GUTS_PMUXCR_UART0_I2C1_MASK,
    );
    clrbits32(
        ptr::addr_of_mut!((*guts).pmuxcr),
        CCSR_GUTS_PMUXCR_SSI_DMA_TDM_MASK,
    );
    guts_set_dmuxcr(guts, (*mdata).dma_id[0], (*mdata).dma_channel_id[0], 0);
    guts_set_dmuxcr(guts, (*mdata).dma_id[1], (*mdata).dma_channel_id[1], 0);

    iounmap(guts as *mut c_void);

    0
}

/**
 * p1022_ds_ops: ASoC machine driver operations
 */
static p1022_ds_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(p1022_ds_startup),
};

/**
 * p1022_ds_probe: platform probe function for the machine driver
 *
 * Although this is a machine driver, the SSI node is the "master" node with
 * respect to audio hardware connections.  Therefore, we create a new ASoC
 * device for each new SSI node that has a codec attached.
 */
unsafe extern "C" fn p1022_ds_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = (*pdev).dev.parent;
    /* ssi_pdev is the platform device for the SSI node that probed us */
    let ssi_pdev: *mut platform_device = to_platform_device(dev);
    let np: *mut device_node = (*ssi_pdev).dev.of_node;
    let mut codec_np: *mut device_node = ptr::null_mut();
    let mut mdata: *mut machine_data;
    let comp: *mut snd_soc_dai_link_component;
    let mut ret: c_int;
    let mut sprop: *const c_char;
    let mut iprop: *const u32;

    /* Find the codec node for this SSI. */
    codec_np = of_parse_phandle(np, c"codec-handle".as_ptr(), 0);
    if codec_np.is_null() {
        dev_err(dev, c"could not find codec node\n".as_ptr());
        return -EINVAL;
    }

    mdata = kzalloc(mem::size_of::<machine_data>(), GFP_KERNEL) as *mut machine_data;
    if mdata.is_null() {
        ret = -ENOMEM;
        goto_error_put(mdata, codec_np, ret);
        return ret;
    }

    comp = devm_kzalloc(
        ptr::addr_of_mut!((*pdev).dev),
        6 * mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if comp.is_null() {
        ret = -ENOMEM;
        goto_error_put(mdata, codec_np, ret);
        return ret;
    }

    (*mdata).dai[0].cpus = comp.add(0);
    (*mdata).dai[0].codecs = comp.add(1);
    (*mdata).dai[0].platforms = comp.add(2);

    (*mdata).dai[0].num_cpus = 1;
    (*mdata).dai[0].num_codecs = 1;
    (*mdata).dai[0].num_platforms = 1;

    (*mdata).dai[1].cpus = comp.add(3);
    (*mdata).dai[1].codecs = comp.add(4);
    (*mdata).dai[1].platforms = comp.add(5);

    (*mdata).dai[1].num_cpus = 1;
    (*mdata).dai[1].num_codecs = 1;
    (*mdata).dai[1].num_platforms = 1;

    (*(*mdata).dai[0].cpus).dai_name = dev_name(ptr::addr_of_mut!((*ssi_pdev).dev));
    (*mdata).dai[0].ops = ptr::addr_of!(p1022_ds_ops);

    /* ASoC core can match codec with device node */
    (*(*mdata).dai[0].codecs).of_node = codec_np;

    /* We register two DAIs per SSI, one for playback and the other for
     * capture.  We support codecs that have separate DAIs for both playback
     * and capture.
     */
    memcpy(
        ptr::addr_of_mut!((*mdata).dai[1]) as *mut c_void,
        ptr::addr_of!((*mdata).dai[0]) as *const c_void,
        mem::size_of::<snd_soc_dai_link>(),
    );

    /* The DAI names from the codec (snd_soc_dai_driver.name) */
    (*(*mdata).dai[0].codecs).dai_name = c"wm8776-hifi-playback".as_ptr();
    (*(*mdata).dai[1].codecs).dai_name = c"wm8776-hifi-capture".as_ptr();

    /* Get the device ID */
    iprop = of_get_property(np, c"cell-index".as_ptr(), ptr::null_mut()) as *const u32;
    if iprop.is_null() {
        dev_err(
            ptr::addr_of_mut!((*pdev).dev),
            c"cell-index property not found\n".as_ptr(),
        );
        ret = -EINVAL;
        goto_error(mdata, codec_np, ret);
        return ret;
    }
    (*mdata).ssi_id = be32_to_cpup(iprop);

    /* Get the serial format and clock direction. */
    sprop = of_get_property(np, c"fsl,mode".as_ptr(), ptr::null_mut()) as *const c_char;
    if sprop.is_null() {
        dev_err(
            ptr::addr_of_mut!((*pdev).dev),
            c"fsl,mode property not found\n".as_ptr(),
        );
        ret = -EINVAL;
        goto_error(mdata, codec_np, ret);
        return ret;
    }

    if strcasecmp(sprop, c"i2s-slave".as_ptr()) == 0 {
        (*mdata).dai_format = SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBP_CFP;
        (*mdata).codec_clk_direction = SND_SOC_CLOCK_OUT;
        (*mdata).cpu_clk_direction = SND_SOC_CLOCK_IN;

        /* In i2s-slave mode, the codec has its own clock source, so we
         * need to get the frequency from the device tree and pass it to
         * the codec driver.
         */
        iprop = of_get_property(codec_np, c"clock-frequency".as_ptr(), ptr::null_mut()) as *const u32;
        if iprop.is_null() || *iprop == 0 {
            dev_err(
                ptr::addr_of_mut!((*pdev).dev),
                c"codec bus-frequency property is missing or invalid\n".as_ptr(),
            );
            ret = -EINVAL;
            goto_error(mdata, codec_np, ret);
            return ret;
        }
        (*mdata).clk_frequency = be32_to_cpup(iprop);
    } else if strcasecmp(sprop, c"i2s-master".as_ptr()) == 0 {
        (*mdata).dai_format = SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC;
        (*mdata).codec_clk_direction = SND_SOC_CLOCK_IN;
        (*mdata).cpu_clk_direction = SND_SOC_CLOCK_OUT;
    } else if strcasecmp(sprop, c"lj-slave".as_ptr()) == 0 {
        (*mdata).dai_format = SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_CBP_CFP;
        (*mdata).codec_clk_direction = SND_SOC_CLOCK_OUT;
        (*mdata).cpu_clk_direction = SND_SOC_CLOCK_IN;
    } else if strcasecmp(sprop, c"lj-master".as_ptr()) == 0 {
        (*mdata).dai_format = SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_CBC_CFC;
        (*mdata).codec_clk_direction = SND_SOC_CLOCK_IN;
        (*mdata).cpu_clk_direction = SND_SOC_CLOCK_OUT;
    } else if strcasecmp(sprop, c"rj-slave".as_ptr()) == 0 {
        (*mdata).dai_format = SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_RIGHT_J | SND_SOC_DAIFMT_CBP_CFP;
        (*mdata).codec_clk_direction = SND_SOC_CLOCK_OUT;
        (*mdata).cpu_clk_direction = SND_SOC_CLOCK_IN;
    } else if strcasecmp(sprop, c"rj-master".as_ptr()) == 0 {
        (*mdata).dai_format = SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_RIGHT_J | SND_SOC_DAIFMT_CBC_CFC;
        (*mdata).codec_clk_direction = SND_SOC_CLOCK_IN;
        (*mdata).cpu_clk_direction = SND_SOC_CLOCK_OUT;
    } else if strcasecmp(sprop, c"ac97-slave".as_ptr()) == 0 {
        (*mdata).dai_format = SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_AC97 | SND_SOC_DAIFMT_CBP_CFP;
        (*mdata).codec_clk_direction = SND_SOC_CLOCK_OUT;
        (*mdata).cpu_clk_direction = SND_SOC_CLOCK_IN;
    } else if strcasecmp(sprop, c"ac97-master".as_ptr()) == 0 {
        (*mdata).dai_format = SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_AC97 | SND_SOC_DAIFMT_CBC_CFC;
        (*mdata).codec_clk_direction = SND_SOC_CLOCK_IN;
        (*mdata).cpu_clk_direction = SND_SOC_CLOCK_OUT;
    } else {
        dev_err(
            ptr::addr_of_mut!((*pdev).dev),
            c"unrecognized fsl,mode property '%s'\n".as_ptr(),
            sprop,
        );
        ret = -EINVAL;
        goto_error(mdata, codec_np, ret);
        return ret;
    }

    if (*mdata).clk_frequency == 0 {
        dev_err(
            ptr::addr_of_mut!((*pdev).dev),
            c"unknown clock frequency\n".as_ptr(),
        );
        ret = -EINVAL;
        goto_error(mdata, codec_np, ret);
        return ret;
    }

    /* Find the playback DMA channel to use. */
    (*(*mdata).dai[0].platforms).name = (*mdata).platform_name[0].as_mut_ptr();
    ret = fsl_asoc_get_dma_channel(
        np,
        c"fsl,playback-dma".as_ptr(),
        ptr::addr_of_mut!((*mdata).dai[0]),
        ptr::addr_of_mut!((*mdata).dma_channel_id[0]),
        ptr::addr_of_mut!((*mdata).dma_id[0]),
    );
    if ret != 0 {
        dev_err(
            ptr::addr_of_mut!((*pdev).dev),
            c"missing/invalid playback DMA phandle\n".as_ptr(),
        );
        goto_error(mdata, codec_np, ret);
        return ret;
    }

    /* Find the capture DMA channel to use. */
    (*(*mdata).dai[1].platforms).name = (*mdata).platform_name[1].as_mut_ptr();
    ret = fsl_asoc_get_dma_channel(
        np,
        c"fsl,capture-dma".as_ptr(),
        ptr::addr_of_mut!((*mdata).dai[1]),
        ptr::addr_of_mut!((*mdata).dma_channel_id[1]),
        ptr::addr_of_mut!((*mdata).dma_id[1]),
    );
    if ret != 0 {
        dev_err(
            ptr::addr_of_mut!((*pdev).dev),
            c"missing/invalid capture DMA phandle\n".as_ptr(),
        );
        goto_error(mdata, codec_np, ret);
        return ret;
    }

    /* Initialize our DAI data structure.  */
    (*mdata).dai[0].stream_name = c"playback".as_ptr();
    (*mdata).dai[1].stream_name = c"capture".as_ptr();
    (*mdata).dai[0].name = (*mdata).dai[0].stream_name;
    (*mdata).dai[1].name = (*mdata).dai[1].stream_name;

    (*mdata).card.probe = Some(p1022_ds_machine_probe);
    (*mdata).card.remove = Some(p1022_ds_machine_remove);
    (*mdata).card.name = (*pdev).name; /* The platform driver name */
    (*mdata).card.owner = THIS_MODULE;
    (*mdata).card.dev = ptr::addr_of_mut!((*pdev).dev);
    (*mdata).card.num_links = 2;
    (*mdata).card.dai_link = (*mdata).dai.as_mut_ptr();

    /* Register with ASoC */
    ret = snd_soc_register_card(ptr::addr_of_mut!((*mdata).card));
    if ret != 0 {
        dev_err(
            ptr::addr_of_mut!((*pdev).dev),
            c"could not register card\n".as_ptr(),
        );
        goto_error(mdata, codec_np, ret);
        return ret;
    }

    of_node_put(codec_np);

    0
}

unsafe fn goto_error(mdata: *mut machine_data, codec_np: *mut device_node, ret: c_int) {
    kfree(mdata as *mut c_void);
    goto_error_put(mdata, codec_np, ret);
}

unsafe fn goto_error_put(_mdata: *mut machine_data, codec_np: *mut device_node, _ret: c_int) {
    of_node_put(codec_np);
}

/**
 * p1022_ds_remove: remove the platform device
 *
 * This function is called when the platform device is removed.
 */
unsafe extern "C" fn p1022_ds_remove(pdev: *mut platform_device) {
    let card: *mut snd_soc_card = platform_get_drvdata(pdev) as *mut snd_soc_card;
    let mdata: *mut machine_data = card_to_mdata(card);

    snd_soc_unregister_card(card);
    kfree(mdata as *mut c_void);
}

static mut p1022_ds_driver: platform_driver = platform_driver {
    probe: Some(p1022_ds_probe),
    remove: Some(p1022_ds_remove),
    driver: platform_driver_inner {
        /*
         * The name must match 'compatible' property in the device tree,
         * in lowercase letters.
         */
        name: c"snd-soc-p1022ds".as_ptr(),
    },
};

/**
 * p1022_ds_init: machine driver initialization.
 *
 * This function is called when this module is loaded.
 */
unsafe extern "C" fn p1022_ds_init() -> c_int {
    let mut guts_np: *mut device_node;
    let mut res: resource = mem::zeroed();

    /* Get the physical address of the global utilities registers */
    guts_np = of_find_compatible_node(ptr::null_mut(), ptr::null(), c"fsl,p1022-guts".as_ptr());
    if of_address_to_resource(guts_np, 0, ptr::addr_of_mut!(res)) != 0 {
        pr_err(c"snd-soc-p1022ds: missing/invalid global utils node\n".as_ptr());
        of_node_put(guts_np);
        return -EINVAL;
    }
    guts_phys = res.start;
    of_node_put(guts_np);

    platform_driver_register(ptr::addr_of_mut!(p1022_ds_driver))
}

/**
 * p1022_ds_exit: machine driver exit
 *
 * This function is called when this driver is unloaded.
 */
unsafe extern "C" fn p1022_ds_exit() {
    platform_driver_unregister(ptr::addr_of_mut!(p1022_ds_driver));
}

/* module_init(p1022_ds_init); */
/* module_exit(p1022_ds_exit); */

/* MODULE_AUTHOR("Timur Tabi <timur@freescale.com>"); */
/* MODULE_DESCRIPTION("Freescale P1022 DS ALSA SoC machine driver"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
