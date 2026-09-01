// SPDX-License-Identifier: GPL-2.0+
//
// eukrea-tlv320.c  --  SoC audio for eukrea_cpuimxXX in I2S mode
//
// Copyright 2010 Eric Bénard, Eukréa Electromatique <eric@eukrea.com>
//
// based on sound/soc/s3c24xx/s3c24xx_simtec_tlv320aic23.c
// which is Copyright 2009 Simtec Electronics
// and on sound/soc/imx/phycore-ac97.c which is
// Copyright 2009 Sascha Hauer, Pengutronix <s.hauer@pengutronix.de>

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const CODEC_CLOCK: c_uint = 12000000;

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;

extern "C" {
    static THIS_MODULE: *mut module;

    static IMX_SSP_SYS_CLK: c_int;
    static SND_SOC_CLOCK_OUT: c_int;
    static SND_SOC_CLOCK_IN: c_int;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;

    static MX27_AUDMUX_HPCR1_SSI0: c_int;
    static MX27_AUDMUX_HPCR3_SSI_PINS_4: c_int;
    static IMX_AUDMUX_V1_PCR_SYN: c_uint;
    static IMX_AUDMUX_V1_PCR_TFSDIR: c_uint;
    static IMX_AUDMUX_V1_PCR_TCLKDIR: c_uint;
    static IMX_AUDMUX_V1_PCR_RFSDIR: c_uint;
    static IMX_AUDMUX_V1_PCR_RCLKDIR: c_uint;
    static IMX_AUDMUX_V2_PTCR_SYN: c_uint;
    static IMX_AUDMUX_V2_PTCR_TFSDIR: c_uint;
    static IMX_AUDMUX_V2_PTCR_TCLKDIR: c_uint;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_property_read_u32(
        np: *mut device_node,
        propname: *const c_char,
        out_value: *mut c_int,
    ) -> c_int;
    fn of_machine_is_compatible(compat: *const c_char) -> bool;
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn imx_audmux_v1_configure_port(port: c_int, pcr: c_uint);
    fn imx_audmux_v2_configure_port(port: c_int, ptcr: c_uint, pdcr: c_uint);
    fn IMX_AUDMUX_V1_PCR_TFCSEL(port: c_int) -> c_uint;
    fn IMX_AUDMUX_V1_PCR_RFCSEL(port: c_int) -> c_uint;
    fn IMX_AUDMUX_V1_PCR_RXDSEL(port: c_int) -> c_uint;
    fn IMX_AUDMUX_V2_PTCR_TFSEL(port: c_int) -> c_uint;
    fn IMX_AUDMUX_V2_PTCR_TCSEL(port: c_int) -> c_uint;
    fn IMX_AUDMUX_V2_PDCR_RXDSEL(port: c_int) -> c_uint;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub dai_fmt: c_uint,
    pub ops: *const snd_soc_ops,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub dev: *mut device,
    pub owner: *mut module,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

unsafe extern "C" fn eukrea_tlv320_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut ret: c_int;

    ret = snd_soc_dai_set_sysclk(codec_dai, 0, CODEC_CLOCK, SND_SOC_CLOCK_OUT);
    if ret != 0 {
        dev_err(
            (*cpu_dai).dev,
            b"Failed to set the codec sysclk.\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    snd_soc_dai_set_tdm_slot(cpu_dai, 0x3, 0x3, 2, 0);

    ret = snd_soc_dai_set_sysclk(cpu_dai, IMX_SSP_SYS_CLK, 0, SND_SOC_CLOCK_IN);
    /* fsl_ssi lacks the set_sysclk ops */
    if ret != 0 && ret != -EINVAL {
        dev_err(
            (*cpu_dai).dev,
            b"Can't set the IMX_SSP_SYS_CLK CPU system clock.\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    0
}

static eukrea_tlv320_snd_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(eukrea_tlv320_hw_params),
};

static mut hifi_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut hifi_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: b"tlv320aic23-hifi\0".as_ptr() as *const c_char,
    of_node: ptr::null_mut(),
}];

static mut hifi_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut eukrea_tlv320_dai: snd_soc_dai_link = snd_soc_dai_link {
    name: b"tlv320aic23\0".as_ptr() as *const c_char,
    stream_name: b"TLV320AIC23\0".as_ptr() as *const c_char,
    dai_fmt: 0,
    ops: &eukrea_tlv320_snd_ops,
    cpus: ptr::null_mut(),
    num_cpus: 1,
    codecs: ptr::null_mut(),
    num_codecs: 1,
    platforms: ptr::null_mut(),
    num_platforms: 1,
};

static mut eukrea_tlv320: snd_soc_card = snd_soc_card {
    name: ptr::null(),
    dev: ptr::null_mut(),
    owner: ptr::null_mut(),
    dai_link: ptr::null_mut(),
    num_links: 1,
};

unsafe fn init_static_links() {
    eukrea_tlv320_dai.dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP;
    eukrea_tlv320_dai.cpus = hifi_cpus.as_mut_ptr();
    eukrea_tlv320_dai.codecs = hifi_codecs.as_mut_ptr();
    eukrea_tlv320_dai.platforms = hifi_platforms.as_mut_ptr();
    eukrea_tlv320.owner = THIS_MODULE;
    eukrea_tlv320.dai_link = &mut eukrea_tlv320_dai;
}

unsafe extern "C" fn eukrea_tlv320_probe(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int;
    let mut int_port: c_int = 0;
    let mut ext_port: c_int = 0;
    let np = (*pdev).dev.of_node;
    let mut ssi_np: *mut device_node = ptr::null_mut();
    let mut codec_np: *mut device_node = ptr::null_mut();
    let mut tmp_np: *mut device_node = ptr::null_mut();

    init_static_links();
    eukrea_tlv320.dev = &mut (*pdev).dev;
    if !np.is_null() {
        ret = snd_soc_of_parse_card_name(
            &mut eukrea_tlv320,
            b"eukrea,model\0".as_ptr() as *const c_char,
        );
        if ret != 0 {
            dev_err(
                &mut (*pdev).dev,
                b"eukrea,model node missing or invalid.\n\0".as_ptr() as *const c_char,
            );
            goto_err(ret, ssi_np, pdev);
            return ret;
        }

        ssi_np = of_parse_phandle(
            (*pdev).dev.of_node,
            b"ssi-controller\0".as_ptr() as *const c_char,
            0,
        );
        if ssi_np.is_null() {
            dev_err(
                &mut (*pdev).dev,
                b"ssi-controller missing or invalid.\n\0".as_ptr() as *const c_char,
            );
            ret = -ENODEV;
            goto_err(ret, ssi_np, pdev);
            return ret;
        }

        codec_np = of_parse_phandle(ssi_np, b"codec-handle\0".as_ptr() as *const c_char, 0);
        if !codec_np.is_null() {
            (*eukrea_tlv320_dai.codecs).of_node = codec_np;
        } else {
            dev_err(
                &mut (*pdev).dev,
                b"codec-handle node missing or invalid.\n\0".as_ptr() as *const c_char,
            );
        }

        ret = of_property_read_u32(np, b"fsl,mux-int-port\0".as_ptr() as *const c_char, &mut int_port);
        if ret != 0 {
            dev_err(
                &mut (*pdev).dev,
                b"fsl,mux-int-port node missing or invalid.\n\0".as_ptr() as *const c_char,
            );
            goto_err(ret, ssi_np, pdev);
            return ret;
        }
        ret = of_property_read_u32(np, b"fsl,mux-ext-port\0".as_ptr() as *const c_char, &mut ext_port);
        if ret != 0 {
            dev_err(
                &mut (*pdev).dev,
                b"fsl,mux-ext-port node missing or invalid.\n\0".as_ptr() as *const c_char,
            );
            goto_err(ret, ssi_np, pdev);
            return ret;
        }

        /*
         * The port numbering in the hardware manual starts at 1, while
         * the audmux API expects it starts at 0.
         */
        int_port -= 1;
        ext_port -= 1;

        (*eukrea_tlv320_dai.cpus).of_node = ssi_np;
        (*eukrea_tlv320_dai.platforms).of_node = ssi_np;
    } else {
        (*eukrea_tlv320_dai.cpus).dai_name = b"imx-ssi.0\0".as_ptr() as *const c_char;
        (*eukrea_tlv320_dai.platforms).name = b"imx-ssi.0\0".as_ptr() as *const c_char;
        (*eukrea_tlv320_dai.codecs).name = b"tlv320aic23-codec.0-001a\0".as_ptr() as *const c_char;
        eukrea_tlv320.name = b"cpuimx-audio\0".as_ptr() as *const c_char;
    }

    tmp_np = of_find_compatible_node(
        ptr::null_mut(),
        ptr::null(),
        b"fsl,imx21-audmux\0".as_ptr() as *const c_char,
    );
    if of_machine_is_compatible(b"eukrea,cpuimx27\0".as_ptr() as *const c_char) || !tmp_np.is_null() {
        imx_audmux_v1_configure_port(
            MX27_AUDMUX_HPCR1_SSI0,
            IMX_AUDMUX_V1_PCR_SYN
                | IMX_AUDMUX_V1_PCR_TFSDIR
                | IMX_AUDMUX_V1_PCR_TCLKDIR
                | IMX_AUDMUX_V1_PCR_RFSDIR
                | IMX_AUDMUX_V1_PCR_RCLKDIR
                | IMX_AUDMUX_V1_PCR_TFCSEL(MX27_AUDMUX_HPCR3_SSI_PINS_4)
                | IMX_AUDMUX_V1_PCR_RFCSEL(MX27_AUDMUX_HPCR3_SSI_PINS_4)
                | IMX_AUDMUX_V1_PCR_RXDSEL(MX27_AUDMUX_HPCR3_SSI_PINS_4),
        );
        imx_audmux_v1_configure_port(
            MX27_AUDMUX_HPCR3_SSI_PINS_4,
            IMX_AUDMUX_V1_PCR_SYN | IMX_AUDMUX_V1_PCR_RXDSEL(MX27_AUDMUX_HPCR1_SSI0),
        );
        of_node_put(tmp_np);
    } else {
        tmp_np = of_find_compatible_node(
            ptr::null_mut(),
            ptr::null(),
            b"fsl,imx31-audmux\0".as_ptr() as *const c_char,
        );
        if of_machine_is_compatible(b"eukrea,cpuimx25\0".as_ptr() as *const c_char)
            || of_machine_is_compatible(b"eukrea,cpuimx35\0".as_ptr() as *const c_char)
            || of_machine_is_compatible(b"eukrea,cpuimx51\0".as_ptr() as *const c_char)
            || !tmp_np.is_null()
        {
            if np.is_null() {
                ext_port = if of_machine_is_compatible(b"eukrea,cpuimx25\0".as_ptr() as *const c_char) {
                    4
                } else {
                    3
                };
            }

            imx_audmux_v2_configure_port(
                int_port,
                IMX_AUDMUX_V2_PTCR_SYN
                    | IMX_AUDMUX_V2_PTCR_TFSDIR
                    | IMX_AUDMUX_V2_PTCR_TFSEL(ext_port)
                    | IMX_AUDMUX_V2_PTCR_TCLKDIR
                    | IMX_AUDMUX_V2_PTCR_TCSEL(ext_port),
                IMX_AUDMUX_V2_PDCR_RXDSEL(ext_port),
            );
            imx_audmux_v2_configure_port(
                ext_port,
                IMX_AUDMUX_V2_PTCR_SYN,
                IMX_AUDMUX_V2_PDCR_RXDSEL(int_port),
            );
            of_node_put(tmp_np);
        } else if !np.is_null() {
            /* The eukrea,asoc-tlv320 driver was explicitly
             * requested (through the device tree).
             */
            dev_err(
                &mut (*pdev).dev,
                b"Missing or invalid audmux DT node.\n\0".as_ptr() as *const c_char,
            );
            return -ENODEV;
        } else {
            /* Return happy.
             * We might run on a totally different machine.
             */
            return 0;
        }
    }

    ret = devm_snd_soc_register_card(&mut (*pdev).dev, &mut eukrea_tlv320);
    goto_err(ret, ssi_np, pdev);
    ret
}

unsafe fn goto_err(ret: c_int, ssi_np: *mut device_node, pdev: *mut platform_device) {
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"snd_soc_register_card failed (%d)\n\0".as_ptr() as *const c_char,
            ret,
        );
    }
    of_node_put(ssi_np);
}

static imx_tlv320_dt_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: b"eukrea,asoc-tlv320\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

/* MODULE_DEVICE_TABLE(of, imx_tlv320_dt_ids); */

static mut eukrea_tlv320_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"eukrea_tlv320\0".as_ptr() as *const c_char,
        of_match_table: imx_tlv320_dt_ids.as_ptr(),
    },
    probe: Some(eukrea_tlv320_probe),
};

/* module_platform_driver(eukrea_tlv320_driver); */

/* MODULE_AUTHOR("Eric Bénard <eric@eukrea.com>"); */
/* MODULE_DESCRIPTION("CPUIMX ALSA SoC driver"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:eukrea_tlv320"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
