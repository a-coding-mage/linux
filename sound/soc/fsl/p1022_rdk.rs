// SPDX-License-Identifier: GPL-2.0
//
// Freescale P1022RDK ALSA SoC Machine driver
//
// Author: Timur Tabi <timur@freescale.com>
//
// Copyright 2012 Freescale Semiconductor, Inc.
//
// Note: in order for audio to work correctly, the output controls need
// to be enabled, because they control the clock.  So for playback, for
// example:
//
//      amixer sset 'Left Output Mixer PCM' on
//      amixer sset 'Right Output Mixer PCM' on

// Dependencies from:
// linux/module.h, linux/fsl/guts.h, linux/interrupt.h, linux/of.h,
// linux/of_address.h, linux/slab.h, sound/soc.h, fsl_dma.h, fsl_ssi.h,
// and fsl_utils.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type PhysAddrT = usize;
type U32 = u32;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const GFP_KERNEL: c_uint = 0;
const DAI_NAME_SIZE: usize = 32;

const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const SND_SOC_CLOCK_OUT: c_uint = 0;
const SND_SOC_CLOCK_IN: c_uint = 0;
static mut THIS_MODULE: *mut c_void = ptr::null_mut();

/* P1022-specific PMUXCR and DMUXCR bit definitions */

const CCSR_GUTS_PMUXCR_UART0_I2C1_MASK: c_uint = 0x0001c000;
const CCSR_GUTS_PMUXCR_UART0_I2C1_UART0_SSI: c_uint = 0x00010000;
const CCSR_GUTS_PMUXCR_UART0_I2C1_SSI: c_uint = 0x00018000;

const CCSR_GUTS_PMUXCR_SSI_DMA_TDM_MASK: c_uint = 0x00000c00;
const CCSR_GUTS_PMUXCR_SSI_DMA_TDM_SSI: c_uint = 0x00000000;

const CCSR_GUTS_DMUXCR_PAD: c_uint = 1; /* DMA controller/channel set to pad */
const CCSR_GUTS_DMUXCR_SSI: c_uint = 2; /* DMA controller/channel set to SSI */

#[repr(C)]
pub struct ccsr_guts {
    _prefix: [u8; 0],
    pub pmuxcr: U32,
    pub dmuxcr: U32,
}

#[repr(C)]
pub struct device {
    pub parent: *mut device,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai_link_component {
    pub name: *mut c_char,
    pub of_node: *mut device_node,
    pub dai_name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub cpus: *mut snd_soc_dai_link_component,
    pub codecs: *mut snd_soc_dai_link_component,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub num_codecs: c_uint,
    pub num_platforms: c_uint,
    pub ops: *const snd_soc_ops,
}

#[repr(C)]
pub struct snd_soc_card {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub dev: *mut device,
    pub num_links: c_uint,
    pub dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

#[repr(C)]
pub struct resource {
    pub start: PhysAddrT,
}

/*
 * machine_data: machine-specific ASoC device data
 *
 * This structure contains data for a single sound platform device on an
 * P1022 RDK.  Some of the data is taken from the device tree.
 */
#[repr(C)]
pub struct machine_data {
    pub dai: [snd_soc_dai_link; 2],
    pub card: snd_soc_card,
    pub dai_format: c_uint,
    pub codec_clk_direction: c_uint,
    pub cpu_clk_direction: c_uint,
    pub clk_frequency: c_uint,
    pub dma_id: [c_uint; 2],         /* 0 = DMA1, 1 = DMA2, etc */
    pub dma_channel_id: [c_uint; 2], /* 0 = ch 0, 1 = ch 1, etc*/
    pub platform_name: [[c_char; DAI_NAME_SIZE]; 2], /* One for each DMA channel */
}

unsafe extern "C" {
    fn clrsetbits_be32(addr: *mut U32, clear: c_uint, set: c_uint);
    fn clrbits32(addr: *mut U32, clear: c_uint);
    fn ioremap(phys: PhysAddrT, size: usize) -> *mut ccsr_guts;
    fn iounmap(addr: *mut ccsr_guts);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_uint) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn to_platform_device(dev: *mut device) -> *mut platform_device;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_get_property(
        np: *mut device_node,
        name: *const c_char,
        lenp: *mut c_void,
    ) -> *const U32;
    fn be32_to_cpup(p: *const U32) -> c_uint;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn dev_name(dev: *mut device) -> *const c_char;
    fn fsl_asoc_get_dma_channel(
        np: *mut device_node,
        name: *const c_char,
        dai: *mut snd_soc_dai_link,
        dma_channel_id: *mut c_uint,
        dma_id: *mut c_uint,
    ) -> c_int;
    fn snd_soc_register_card(card: *mut snd_soc_card) -> c_int;
    fn snd_soc_unregister_card(card: *mut snd_soc_card);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn of_node_put(np: *mut device_node);
    fn of_find_compatible_node(
        from: *mut device_node,
        ty: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn of_address_to_resource(np: *mut device_node, index: c_int, res: *mut resource) -> c_int;
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

unsafe fn kzalloc_obj<T>() -> *mut T {
    devm_kzalloc(ptr::null_mut(), size_of::<T>(), GFP_KERNEL) as *mut T
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

fn card_to_mdata(card: *mut snd_soc_card) -> *mut machine_data {
    let offset = core::mem::offset_of!(machine_data, card);
    ((card as *mut u8).wrapping_sub(offset)) as *mut machine_data
}

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

    unsafe {
        clrsetbits_be32(
            &mut (*guts).dmuxcr,
            3u32.wrapping_shl(shift),
            device.wrapping_shl(shift),
        );
    }
}

/* There's only one global utilities register */
static mut guts_phys: PhysAddrT = 0;

/**
 * p1022_rdk_machine_probe - initialize the board
 * @card: ASoC card instance
 *
 * This function is used to initialize the board-specific hardware.
 *
 * Here we program the DMACR and PMUXCR registers.
 *
 * Returns: %0 on success or negative errno value on error
 */
unsafe extern "C" fn p1022_rdk_machine_probe(card: *mut snd_soc_card) -> c_int {
    let mdata: *mut machine_data = card_to_mdata(card);
    let guts: *mut ccsr_guts;

    unsafe {
        guts = ioremap(guts_phys, size_of::<ccsr_guts>());
        if guts.is_null() {
            dev_err((*card).dev, cstr!("could not map global utilities\n"));
            return -ENOMEM;
        }

        /* Enable SSI Tx signal */
        clrsetbits_be32(
            &mut (*guts).pmuxcr,
            CCSR_GUTS_PMUXCR_UART0_I2C1_MASK,
            CCSR_GUTS_PMUXCR_UART0_I2C1_UART0_SSI,
        );

        /* Enable SSI Rx signal */
        clrsetbits_be32(
            &mut (*guts).pmuxcr,
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

        iounmap(guts);
    }

    0
}

/**
 * p1022_rdk_startup - program the board with various hardware parameters
 * @substream: ASoC substream object
 *
 * This function takes board-specific information, like clock frequencies
 * and serial data formats, and passes that information to the codec and
 * transport drivers.
 *
 * Returns: %0 on success or negative errno value on error
 */
unsafe extern "C" fn p1022_rdk_startup(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
        let mdata: *mut machine_data = card_to_mdata((*rtd).card);
        let dev: *mut device = (*(*rtd).card).dev;
        let mut ret: c_int = 0;

        /* Tell the codec driver what the serial protocol is. */
        ret = snd_soc_dai_set_fmt(snd_soc_rtd_to_codec(rtd, 0), (*mdata).dai_format);
        if ret < 0 {
            dev_err(
                dev,
                cstr!("could not set codec driver audio format (ret=%i)\n"),
                ret,
            );
            return ret;
        }

        ret = snd_soc_dai_set_pll(
            snd_soc_rtd_to_codec(rtd, 0),
            0,
            0,
            (*mdata).clk_frequency,
            (*mdata).clk_frequency,
        );
        if ret < 0 {
            dev_err(
                dev,
                cstr!("could not set codec PLL frequency (ret=%i)\n"),
                ret,
            );
            return ret;
        }

        0
    }
}

/**
 * p1022_rdk_machine_remove - Remove the sound device
 * @card: ASoC card instance
 *
 * This function is called to remove the sound device for one SSI.  We
 * de-program the DMACR and PMUXCR register.
 *
 * Returns: %0 on success or negative errno value on error
 */
unsafe extern "C" fn p1022_rdk_machine_remove(card: *mut snd_soc_card) -> c_int {
    let mdata: *mut machine_data = card_to_mdata(card);
    let guts: *mut ccsr_guts;

    unsafe {
        guts = ioremap(guts_phys, size_of::<ccsr_guts>());
        if guts.is_null() {
            dev_err((*card).dev, cstr!("could not map global utilities\n"));
            return -ENOMEM;
        }

        /* Restore the signal routing */
        clrbits32(&mut (*guts).pmuxcr, CCSR_GUTS_PMUXCR_UART0_I2C1_MASK);
        clrbits32(&mut (*guts).pmuxcr, CCSR_GUTS_PMUXCR_SSI_DMA_TDM_MASK);
        guts_set_dmuxcr(guts, (*mdata).dma_id[0], (*mdata).dma_channel_id[0], 0);
        guts_set_dmuxcr(guts, (*mdata).dma_id[1], (*mdata).dma_channel_id[1], 0);

        iounmap(guts);
    }

    0
}

/*
 * p1022_rdk_ops: ASoC machine driver operations
 */
static p1022_rdk_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(p1022_rdk_startup),
};

/**
 * p1022_rdk_probe - platform probe function for the machine driver
 * @pdev: platform device pointer
 *
 * Although this is a machine driver, the SSI node is the "master" node with
 * respect to audio hardware connections.  Therefore, we create a new ASoC
 * device for each new SSI node that has a codec attached.
 *
 * Returns: %0 on success or negative errno value on error
 */
unsafe extern "C" fn p1022_rdk_probe(pdev: *mut platform_device) -> c_int {
    unsafe {
        let dev: *mut device = (*pdev).dev.parent;
        /* ssi_pdev is the platform device for the SSI node that probed us */
        let ssi_pdev: *mut platform_device = to_platform_device(dev);
        let np: *mut device_node = (*ssi_pdev).dev.of_node;
        let mut codec_np: *mut device_node = ptr::null_mut();
        let mdata: *mut machine_data;
        let comp: *mut snd_soc_dai_link_component;
        let mut iprop: *const U32;
        let mut ret: c_int;

        /* Find the codec node for this SSI. */
        codec_np = of_parse_phandle(np, cstr!("codec-handle"), 0);
        if codec_np.is_null() {
            dev_err(dev, cstr!("could not find codec node\n"));
            return -EINVAL;
        }

        mdata = kzalloc_obj::<machine_data>();
        if mdata.is_null() {
            ret = -ENOMEM;
            goto_error_put(codec_np, ret);
            return ret;
        }

        comp = devm_kzalloc(
            &mut (*pdev).dev,
            6 * size_of::<snd_soc_dai_link_component>(),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_link_component;
        if comp.is_null() {
            ret = -ENOMEM;
            goto_error_put(codec_np, ret);
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

        (*(*mdata).dai[0].cpus).dai_name = dev_name(&mut (*ssi_pdev).dev);
        (*mdata).dai[0].ops = &p1022_rdk_ops;

        /* ASoC core can match codec with device node */
        (*(*mdata).dai[0].codecs).of_node = codec_np;

        /*
         * We register two DAIs per SSI, one for playback and the other for
         * capture.  We support codecs that have separate DAIs for both playback
         * and capture.
         */
        ptr::copy_nonoverlapping(
            &(*mdata).dai[0],
            &mut (*mdata).dai[1],
            1,
        );

        /* The DAI names from the codec (snd_soc_dai_driver.name) */
        (*(*mdata).dai[0].codecs).dai_name = cstr!("wm8960-hifi");
        (*(*mdata).dai[1].codecs).dai_name = (*(*mdata).dai[0].codecs).dai_name;

        /*
         * Configure the SSI for I2S slave mode.  Older device trees have
         * an fsl,mode property, but we ignore that since there's really
         * only one way to configure the SSI.
         */
        (*mdata).dai_format = SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBP_CFP;
        (*mdata).codec_clk_direction = SND_SOC_CLOCK_OUT;
        (*mdata).cpu_clk_direction = SND_SOC_CLOCK_IN;

        /*
         * In i2s-slave mode, the codec has its own clock source, so we
         * need to get the frequency from the device tree and pass it to
         * the codec driver.
         */
        iprop = of_get_property(codec_np, cstr!("clock-frequency"), ptr::null_mut());
        if iprop.is_null() || *iprop == 0 {
            dev_err(
                &mut (*pdev).dev,
                cstr!("codec bus-frequency property is missing or invalid\n"),
            );
            ret = -EINVAL;
            kfree(mdata as *mut c_void);
            of_node_put(codec_np);
            return ret;
        }
        (*mdata).clk_frequency = be32_to_cpup(iprop);

        if (*mdata).clk_frequency == 0 {
            dev_err(&mut (*pdev).dev, cstr!("unknown clock frequency\n"));
            ret = -EINVAL;
            kfree(mdata as *mut c_void);
            of_node_put(codec_np);
            return ret;
        }

        /* Find the playback DMA channel to use. */
        (*(*mdata).dai[0].platforms).name = (*mdata).platform_name[0].as_mut_ptr();
        ret = fsl_asoc_get_dma_channel(
            np,
            cstr!("fsl,playback-dma"),
            &mut (*mdata).dai[0],
            &mut (*mdata).dma_channel_id[0],
            &mut (*mdata).dma_id[0],
        );
        if ret != 0 {
            dev_err(
                &mut (*pdev).dev,
                cstr!("missing/invalid playback DMA phandle (ret=%i)\n"),
                ret,
            );
            kfree(mdata as *mut c_void);
            of_node_put(codec_np);
            return ret;
        }

        /* Find the capture DMA channel to use. */
        (*(*mdata).dai[1].platforms).name = (*mdata).platform_name[1].as_mut_ptr();
        ret = fsl_asoc_get_dma_channel(
            np,
            cstr!("fsl,capture-dma"),
            &mut (*mdata).dai[1],
            &mut (*mdata).dma_channel_id[1],
            &mut (*mdata).dma_id[1],
        );
        if ret != 0 {
            dev_err(
                &mut (*pdev).dev,
                cstr!("missing/invalid capture DMA phandle (ret=%i)\n"),
                ret,
            );
            kfree(mdata as *mut c_void);
            of_node_put(codec_np);
            return ret;
        }

        /* Initialize our DAI data structure.  */
        (*mdata).dai[0].stream_name = cstr!("playback");
        (*mdata).dai[1].stream_name = cstr!("capture");
        (*mdata).dai[0].name = (*mdata).dai[0].stream_name;
        (*mdata).dai[1].name = (*mdata).dai[1].stream_name;

        (*mdata).card.probe = Some(p1022_rdk_machine_probe);
        (*mdata).card.remove = Some(p1022_rdk_machine_remove);
        (*mdata).card.name = (*pdev).name; /* The platform driver name */
        (*mdata).card.owner = THIS_MODULE;
        (*mdata).card.dev = &mut (*pdev).dev;
        (*mdata).card.num_links = 2;
        (*mdata).card.dai_link = (*mdata).dai.as_mut_ptr();

        /* Register with ASoC */
        ret = snd_soc_register_card(&mut (*mdata).card);
        if ret != 0 {
            dev_err(
                &mut (*pdev).dev,
                cstr!("could not register card (ret=%i)\n"),
                ret,
            );
            kfree(mdata as *mut c_void);
            of_node_put(codec_np);
            return ret;
        }

        0
    }
}

unsafe fn goto_error_put(codec_np: *mut device_node, ret: c_int) {
    unsafe {
        of_node_put(codec_np);
    }
    let _ = ret;
}

/**
 * p1022_rdk_remove - remove the platform device
 * @pdev: platform device pointer
 *
 * This function is called when the platform device is removed.
 */
unsafe extern "C" fn p1022_rdk_remove(pdev: *mut platform_device) {
    unsafe {
        let card: *mut snd_soc_card = platform_get_drvdata(pdev) as *mut snd_soc_card;
        let mdata: *mut machine_data = card_to_mdata(card);

        snd_soc_unregister_card(card);
        kfree(mdata as *mut c_void);
    }
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

static mut p1022_rdk_driver: platform_driver = platform_driver {
    probe: Some(p1022_rdk_probe),
    remove: Some(p1022_rdk_remove),
    driver: device_driver {
        /*
         * The name must match 'compatible' property in the device tree,
         * in lowercase letters.
         */
        name: cstr!("snd-soc-p1022rdk"),
    },
};

/**
 * p1022_rdk_init - machine driver initialization.
 *
 * This function is called when this module is loaded.
 *
 * Returns: %0 on success or negative errno value on error
 */
unsafe extern "C" fn p1022_rdk_init() -> c_int {
    unsafe {
        let guts_np: *mut device_node;
        let mut res: resource = resource { start: 0 };

        /* Get the physical address of the global utilities registers */
        guts_np = of_find_compatible_node(ptr::null_mut(), ptr::null(), cstr!("fsl,p1022-guts"));
        if of_address_to_resource(guts_np, 0, &mut res) != 0 {
            pr_err(cstr!(
                "snd-soc-p1022rdk: missing/invalid global utils node\n"
            ));
            of_node_put(guts_np);
            return -EINVAL;
        }
        guts_phys = res.start;
        of_node_put(guts_np);

        platform_driver_register(&raw mut p1022_rdk_driver)
    }
}

/**
 * p1022_rdk_exit - machine driver exit
 *
 * This function is called when this driver is unloaded.
 */
unsafe extern "C" fn p1022_rdk_exit() {
    unsafe {
        platform_driver_unregister(&raw mut p1022_rdk_driver);
    }
}

// late_initcall(p1022_rdk_init);
// module_exit(p1022_rdk_exit);

// MODULE_AUTHOR("Timur Tabi <timur@freescale.com>");
// MODULE_DESCRIPTION("Freescale / iVeia P1022 RDK ALSA SoC machine driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
