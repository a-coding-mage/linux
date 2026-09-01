// SPDX-License-Identifier: GPL-2.0
//
// Generic AC97 sound support for SH7760
//
// (c) 2007 Manuel Lauss

// C dependencies:
// <linux/module.h>
// <linux/moduleparam.h>
// <linux/platform_device.h>
// <sound/core.h>
// <sound/pcm.h>
// <sound/soc.h>
// <asm/io.h>

const IPSEL: usize = 0xFE400034;
const ENOMEM: i32 = 12;

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const core::ffi::c_char,
    pub stream_name: *const core::ffi::c_char,
    // SND_SOC_DAILINK_REG(ac97)
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const core::ffi::c_char,
    pub owner: *mut core::ffi::c_void,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: i32,
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut core::ffi::c_void;

    fn __raw_readw(addr: usize) -> u16;
    fn __raw_writew(value: u16, addr: usize);

    fn platform_device_alloc(
        name: *const core::ffi::c_char,
        id: i32,
    ) -> *mut platform_device;
    fn platform_set_drvdata(
        pdev: *mut platform_device,
        data: *mut core::ffi::c_void,
    );
    fn platform_device_add(pdev: *mut platform_device) -> i32;
    fn platform_device_put(pdev: *mut platform_device);
    fn platform_device_unregister(pdev: *mut platform_device);
}

// SND_SOC_DAILINK_DEFS(ac97,
//      DAILINK_COMP_ARRAY(COMP_CPU("hac-dai.0")),        /* HAC0 */
//      DAILINK_COMP_ARRAY(COMP_CODEC("ac97-codec", "ac97-hifi")),
//      DAILINK_COMP_ARRAY(COMP_PLATFORM("sh7760-pcm-audio")));

static mut sh7760_ac97_dai: snd_soc_dai_link = snd_soc_dai_link {
    name: c"AC97".as_ptr(),
    stream_name: c"AC97 HiFi".as_ptr(),
};

static mut sh7760_ac97_soc_machine: snd_soc_card = snd_soc_card {
    name: c"SH7760 AC97".as_ptr(),
    owner: unsafe { THIS_MODULE },
    dai_link: core::ptr::addr_of_mut!(sh7760_ac97_dai),
    num_links: 1,
};

static mut sh7760_ac97_snd_device: *mut platform_device = core::ptr::null_mut();

unsafe fn sh7760_ac97_init() -> i32 {
    let mut ret: i32;
    let ipsel: u16;

    /* enable both AC97 controllers in pinmux reg */
    ipsel = unsafe { __raw_readw(IPSEL) };
    unsafe { __raw_writew(ipsel | ((3_u16) << 10), IPSEL) };

    ret = -ENOMEM;
    unsafe {
        sh7760_ac97_snd_device = platform_device_alloc(c"soc-audio".as_ptr(), -1);
    }
    if unsafe { sh7760_ac97_snd_device.is_null() } {
        return ret;
    }

    unsafe {
        platform_set_drvdata(
            sh7760_ac97_snd_device,
            core::ptr::addr_of_mut!(sh7760_ac97_soc_machine) as *mut core::ffi::c_void,
        );
        ret = platform_device_add(sh7760_ac97_snd_device);
    }

    if ret != 0 {
        unsafe { platform_device_put(sh7760_ac97_snd_device) };
    }

    ret
}

unsafe fn sh7760_ac97_exit() {
    unsafe { platform_device_unregister(sh7760_ac97_snd_device) };
}

// module_init(sh7760_ac97_init);
// module_exit(sh7760_ac97_exit);

// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("Generic SH7760 AC97 sound machine");
// MODULE_AUTHOR("Manuel Lauss <mano@roarinelk.homelinux.net>");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
