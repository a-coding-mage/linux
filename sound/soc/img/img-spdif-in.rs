// SPDX-License-Identifier: GPL-2.0-only
/*
 * IMG SPDIF input controller driver
 *
 * Copyright (C) 2015 Imagination Technologies Ltd.
 *
 * Author: Damien Horsley <Damien.Horsley@imgtec.com>
 */

// C dependencies translated as external Rust dependencies:
// linux/clk.h, linux/init.h, linux/kernel.h, linux/module.h, linux/of.h,
// linux/platform_device.h, linux/pm_runtime.h, linux/reset.h,
// sound/core.h, sound/dmaengine_pcm.h, sound/initval.h, sound/pcm.h,
// sound/pcm_params.h, sound/soc.h.

type U32 = u32;
type SndPcmFormatT = i32;

const EINVAL: i32 = 22;
const EBUSY: i32 = 16;
const ENOMEM: i32 = 12;
const EPROBE_DEFER: i32 = 517;
const GFP_KERNEL: u32 = 0;
const LONG_MAX: i64 = i64::MAX;

const fn bit(n: u32) -> U32 {
    1u32 << n
}

const IMG_SPDIF_IN_RX_FIFO_OFFSET: U32 = 0;

const IMG_SPDIF_IN_CTL: U32 = 0x4;
const IMG_SPDIF_IN_CTL_LOCKLO_MASK: U32 = 0xff;
const IMG_SPDIF_IN_CTL_LOCKLO_SHIFT: U32 = 0;
const IMG_SPDIF_IN_CTL_LOCKHI_MASK: U32 = 0xff00;
const IMG_SPDIF_IN_CTL_LOCKHI_SHIFT: U32 = 8;
const IMG_SPDIF_IN_CTL_TRK_MASK: U32 = 0xff0000;
const IMG_SPDIF_IN_CTL_TRK_SHIFT: U32 = 16;
const IMG_SPDIF_IN_CTL_SRD_MASK: U32 = 0x70000000;
const IMG_SPDIF_IN_CTL_SRD_SHIFT: U32 = 28;
const IMG_SPDIF_IN_CTL_SRT_MASK: U32 = bit(31);

const IMG_SPDIF_IN_STATUS: U32 = 0x8;
const IMG_SPDIF_IN_STATUS_SAM_MASK: U32 = 0x7000;
const IMG_SPDIF_IN_STATUS_SAM_SHIFT: U32 = 12;
const IMG_SPDIF_IN_STATUS_LOCK_MASK: U32 = bit(15);
const IMG_SPDIF_IN_STATUS_LOCK_SHIFT: U32 = 15;

const IMG_SPDIF_IN_CLKGEN: U32 = 0x1c;
const IMG_SPDIF_IN_CLKGEN_NOM_MASK: U32 = 0x3ff;
const IMG_SPDIF_IN_CLKGEN_NOM_SHIFT: U32 = 0;
const IMG_SPDIF_IN_CLKGEN_HLD_MASK: U32 = 0x3ff0000;
const IMG_SPDIF_IN_CLKGEN_HLD_SHIFT: U32 = 16;

const IMG_SPDIF_IN_CSL: U32 = 0x20;

const IMG_SPDIF_IN_CSH: U32 = 0x24;
const IMG_SPDIF_IN_CSH_MASK: U32 = 0xff;
const IMG_SPDIF_IN_CSH_SHIFT: U32 = 0;

const IMG_SPDIF_IN_SOFT_RESET: U32 = 0x28;
const IMG_SPDIF_IN_SOFT_RESET_MASK: U32 = bit(0);

const IMG_SPDIF_IN_ACLKGEN_START: U32 = 0x2c;
const IMG_SPDIF_IN_ACLKGEN_NOM_MASK: U32 = 0x3ff;
const IMG_SPDIF_IN_ACLKGEN_NOM_SHIFT: U32 = 0;
const IMG_SPDIF_IN_ACLKGEN_HLD_MASK: U32 = 0xffc00;
const IMG_SPDIF_IN_ACLKGEN_HLD_SHIFT: U32 = 10;
const IMG_SPDIF_IN_ACLKGEN_TRK_MASK: U32 = 0xff00000;
const IMG_SPDIF_IN_ACLKGEN_TRK_SHIFT: U32 = 20;

const IMG_SPDIF_IN_NUM_ACLKGEN: usize = 4;

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: usize,
}

#[repr(C)]
pub struct reset_control {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
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
pub struct snd_dmaengine_dai_dma_data {
    pub addr: usize,
    pub addr_width: u32,
    pub maxburst: u32,
}

#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: i64,
    pub max: i64,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: u32,
    pub count: u32,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub struct snd_ctl_elem_value_iec958 {
    pub status: [u8; 24],
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub iec958: snd_ctl_elem_value_iec958,
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub access: u32,
    pub iface: u32,
    pub name: *const i8,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> i32>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> i32>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> i32>,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> i32>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, i32, *mut snd_soc_dai) -> i32>,
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> i32,
    >,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: u32,
    pub channels_max: u32,
    pub rates: u32,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const i8,
    pub legacy_dai_naming: i32,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const i8,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> i32>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const i8,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
struct img_spdif_in {
    lock: spinlock_t,
    base: *mut u8,
    clk_sys: *mut clk,
    dma_data: snd_dmaengine_dai_dma_data,
    dev: *mut device,
    trk: u32,
    multi_freq: bool,
    lock_acquire: i32,
    lock_release: i32,
    single_freq: u32,
    multi_freqs: [u32; IMG_SPDIF_IN_NUM_ACLKGEN],
    active: bool,
    suspend_clkgen: U32,
    suspend_ctl: U32,

    /* Write-only registers */
    aclkgen_regs: [u32; IMG_SPDIF_IN_NUM_ACLKGEN],
}

unsafe extern "C" {
    static SNDRV_CTL_ELEM_TYPE_IEC958: u32;
    static SNDRV_CTL_ELEM_TYPE_INTEGER: u32;
    static SNDRV_CTL_ELEM_ACCESS_READ: u32;
    static SNDRV_CTL_ELEM_ACCESS_VOLATILE: u32;
    static SNDRV_CTL_ELEM_IFACE_PCM: u32;
    static SNDRV_PCM_TRIGGER_START: i32;
    static SNDRV_PCM_TRIGGER_RESUME: i32;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: i32;
    static SNDRV_PCM_TRIGGER_STOP: i32;
    static SNDRV_PCM_TRIGGER_SUSPEND: i32;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: i32;
    static SNDRV_PCM_FORMAT_S32_LE: SndPcmFormatT;
    static SNDRV_PCM_RATE_8000_192000: u32;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;

    fn dev_get_drvdata(dev: *mut device) -> *mut img_spdif_in;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_get_rate(clk: *mut clk) -> usize;
    fn dev_err(dev: *mut device, fmt: *const i8, ...) -> i32;
    fn writel(val: U32, addr: *mut u8);
    fn readl(addr: *mut u8) -> U32;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut img_spdif_in;
    fn params_rate(params: *mut snd_pcm_hw_params) -> u32;
    fn params_channels(params: *mut snd_pcm_hw_params) -> u32;
    fn params_format(params: *mut snd_pcm_hw_params) -> SndPcmFormatT;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn snd_soc_add_dai_controls(
        dai: *mut snd_soc_dai,
        controls: *mut snd_kcontrol_new,
        num_controls: usize,
    ) -> i32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut img_spdif_in);
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: u32,
        res: *mut *mut resource,
    ) -> *mut u8;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> i32;
    fn devm_clk_get(dev: *mut device, id: *const i8) -> *mut clk;
    fn dev_err_probe(dev: *mut device, err: i32, fmt: *const i8, ...) -> i32;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> bool;
    fn pm_runtime_resume_and_get(dev: *mut device) -> i32;
    fn devm_reset_control_get_exclusive(dev: *mut device, id: *const i8) -> *mut reset_control;
    fn dev_dbg(dev: *mut device, fmt: *const i8, ...) -> i32;
    fn reset_control_assert(rst: *mut reset_control) -> i32;
    fn reset_control_deassert(rst: *mut reset_control) -> i32;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn pm_runtime_put(dev: *mut device) -> i32;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: i32,
    ) -> i32;
    fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *mut core::ffi::c_void,
        flags: u32,
    ) -> i32;
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_status_suspended(dev: *mut device) -> bool;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
}

struct SpinlockIrqsaveGuard {
    lock: *mut spinlock_t,
    flags: usize,
}

impl SpinlockIrqsaveGuard {
    unsafe fn new(lock: *mut spinlock_t) -> Self {
        let mut flags = 0usize;
        unsafe { spin_lock_irqsave(lock, &mut flags) };
        Self { lock, flags }
    }
}

impl Drop for SpinlockIrqsaveGuard {
    fn drop(&mut self) {
        unsafe { spin_unlock_irqrestore(self.lock, self.flags) };
    }
}

unsafe extern "C" fn img_spdif_in_runtime_suspend(dev: *mut device) -> i32 {
    let spdif = unsafe { dev_get_drvdata(dev) };

    unsafe { clk_disable_unprepare((*spdif).clk_sys) };

    0
}

unsafe extern "C" fn img_spdif_in_runtime_resume(dev: *mut device) -> i32 {
    let spdif = unsafe { dev_get_drvdata(dev) };
    let ret: i32;

    ret = unsafe { clk_prepare_enable((*spdif).clk_sys) };
    if ret != 0 {
        unsafe { dev_err(dev, c"Unable to enable sys clock\n".as_ptr()) };
        return ret;
    }

    0
}

unsafe fn img_spdif_in_writel(spdif: *mut img_spdif_in, val: U32, reg: U32) {
    unsafe { writel(val, (*spdif).base.add(reg as usize)) };
}

unsafe fn img_spdif_in_readl(spdif: *mut img_spdif_in, reg: U32) -> U32 {
    unsafe { readl((*spdif).base.add(reg as usize)) }
}

unsafe fn img_spdif_in_aclkgen_writel(spdif: *mut img_spdif_in, index: U32) {
    unsafe {
        img_spdif_in_writel(
            spdif,
            (*spdif).aclkgen_regs[index as usize],
            IMG_SPDIF_IN_ACLKGEN_START + (index * 0x4),
        )
    };
}

unsafe fn img_spdif_in_check_max_rate(
    spdif: *mut img_spdif_in,
    sample_rate: u32,
    actual_freq: *mut usize,
) -> i32 {
    let min_freq: usize;
    let freq_t: usize;

    /* Clock rate must be at least 24x the bit rate */
    min_freq = (sample_rate as usize).wrapping_mul(2).wrapping_mul(32).wrapping_mul(24);

    freq_t = unsafe { clk_get_rate((*spdif).clk_sys) };

    if freq_t < min_freq {
        return -EINVAL;
    }

    unsafe { *actual_freq = freq_t };

    0
}

unsafe fn img_spdif_in_do_clkgen_calc(
    rate: u32,
    pnom: *mut u32,
    phld: *mut u32,
    clk_rate: usize,
) -> i32 {
    let ori: u32;
    let mut nom: u32;
    let mut hld: u32;

    /*
     * Calculate oversampling ratio, nominal phase increment and hold
     * increment for the given rate / frequency
     */

    if rate == 0 {
        return -EINVAL;
    }

    ori = (clk_rate / ((rate as usize).wrapping_mul(64))) as u32;

    if ori == 0 {
        return -EINVAL;
    }

    nom = (4096 / ori) + 1;
    loop {
        nom = nom.wrapping_sub(1);
        hld = 4096u32.wrapping_sub(nom.wrapping_mul(ori.wrapping_sub(1)));
        if hld >= 120 {
            break;
        }
    }

    unsafe {
        *pnom = nom;
        *phld = hld;
    }

    0
}

unsafe fn img_spdif_in_do_clkgen_single(spdif: *mut img_spdif_in, rate: u32) -> i32 {
    let mut nom: u32 = 0;
    let mut hld: u32 = 0;
    let mut clk_rate: usize = 0;
    let mut ret: i32 = 0;
    let mut reg: U32;

    ret = unsafe { img_spdif_in_check_max_rate(spdif, rate, &mut clk_rate) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { img_spdif_in_do_clkgen_calc(rate, &mut nom, &mut hld, clk_rate) };
    if ret != 0 {
        return ret;
    }

    reg = (nom << IMG_SPDIF_IN_CLKGEN_NOM_SHIFT) & IMG_SPDIF_IN_CLKGEN_NOM_MASK;
    reg |= (hld << IMG_SPDIF_IN_CLKGEN_HLD_SHIFT) & IMG_SPDIF_IN_CLKGEN_HLD_MASK;

    let _guard = unsafe { SpinlockIrqsaveGuard::new(&mut (*spdif).lock) };

    if unsafe { (*spdif).active } {
        return -EBUSY;
    }

    unsafe { img_spdif_in_writel(spdif, reg, IMG_SPDIF_IN_CLKGEN) };

    unsafe { (*spdif).single_freq = rate };

    0
}

unsafe fn img_spdif_in_do_clkgen_multi(spdif: *mut img_spdif_in, multi_freqs: *mut u32) -> i32 {
    let mut nom: u32 = 0;
    let mut hld: u32 = 0;
    let mut rate: u32;
    let mut max_rate: u32 = 0;
    let mut clk_rate: usize = 0;
    let mut i: i32;
    let mut ret: i32 = 0;
    let mut reg: U32;
    let trk_reg: U32;
    let mut temp_regs: [U32; IMG_SPDIF_IN_NUM_ACLKGEN] = [0; IMG_SPDIF_IN_NUM_ACLKGEN];

    i = 0;
    while i < IMG_SPDIF_IN_NUM_ACLKGEN as i32 {
        if unsafe { *multi_freqs.add(i as usize) } > max_rate {
            max_rate = unsafe { *multi_freqs.add(i as usize) };
        }
        i += 1;
    }

    ret = unsafe { img_spdif_in_check_max_rate(spdif, max_rate, &mut clk_rate) };
    if ret != 0 {
        return ret;
    }

    i = 0;
    while i < IMG_SPDIF_IN_NUM_ACLKGEN as i32 {
        rate = unsafe { *multi_freqs.add(i as usize) };

        ret = unsafe { img_spdif_in_do_clkgen_calc(rate, &mut nom, &mut hld, clk_rate) };
        if ret != 0 {
            return ret;
        }

        reg = (nom << IMG_SPDIF_IN_ACLKGEN_NOM_SHIFT) & IMG_SPDIF_IN_ACLKGEN_NOM_MASK;
        reg |= (hld << IMG_SPDIF_IN_ACLKGEN_HLD_SHIFT) & IMG_SPDIF_IN_ACLKGEN_HLD_MASK;
        temp_regs[i as usize] = reg;
        i += 1;
    }

    let _guard = unsafe { SpinlockIrqsaveGuard::new(&mut (*spdif).lock) };

    if unsafe { (*spdif).active } {
        return -EBUSY;
    }

    trk_reg = unsafe { (*spdif).trk } << IMG_SPDIF_IN_ACLKGEN_TRK_SHIFT;

    i = 0;
    while i < IMG_SPDIF_IN_NUM_ACLKGEN as i32 {
        unsafe {
            (*spdif).aclkgen_regs[i as usize] = temp_regs[i as usize] | trk_reg;
            img_spdif_in_aclkgen_writel(spdif, i as U32);
        }
        i += 1;
    }

    unsafe {
        (*spdif).multi_freq = true;
        (*spdif).multi_freqs[0] = *multi_freqs.add(0);
        (*spdif).multi_freqs[1] = *multi_freqs.add(1);
        (*spdif).multi_freqs[2] = *multi_freqs.add(2);
        (*spdif).multi_freqs[3] = *multi_freqs.add(3);
    }

    0
}

unsafe extern "C" fn img_spdif_in_iec958_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
        (*uinfo).count = 1;
    }

    0
}

unsafe extern "C" fn img_spdif_in_get_status_mask(
    _kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        (*ucontrol).value.iec958.status[0] = 0xff;
        (*ucontrol).value.iec958.status[1] = 0xff;
        (*ucontrol).value.iec958.status[2] = 0xff;
        (*ucontrol).value.iec958.status[3] = 0xff;
        (*ucontrol).value.iec958.status[4] = 0xff;
    }

    0
}

unsafe extern "C" fn img_spdif_in_get_status(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let cpu_dai = unsafe { snd_kcontrol_chip(kcontrol) };
    let spdif = unsafe { snd_soc_dai_get_drvdata(cpu_dai) };
    let mut reg: U32;

    reg = unsafe { img_spdif_in_readl(spdif, IMG_SPDIF_IN_CSL) };
    unsafe {
        (*ucontrol).value.iec958.status[0] = (reg & 0xff) as u8;
        (*ucontrol).value.iec958.status[1] = ((reg >> 8) & 0xff) as u8;
        (*ucontrol).value.iec958.status[2] = ((reg >> 16) & 0xff) as u8;
        (*ucontrol).value.iec958.status[3] = ((reg >> 24) & 0xff) as u8;
    }
    reg = unsafe { img_spdif_in_readl(spdif, IMG_SPDIF_IN_CSH) };
    unsafe {
        (*ucontrol).value.iec958.status[4] =
            ((reg & IMG_SPDIF_IN_CSH_MASK) >> IMG_SPDIF_IN_CSH_SHIFT) as u8;
    }

    0
}

unsafe extern "C" fn img_spdif_in_info_multi_freq(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = IMG_SPDIF_IN_NUM_ACLKGEN as u32;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = LONG_MAX;
    }

    0
}

unsafe extern "C" fn img_spdif_in_get_multi_freq(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let cpu_dai = unsafe { snd_kcontrol_chip(kcontrol) };
    let spdif = unsafe { snd_soc_dai_get_drvdata(cpu_dai) };

    let _guard = unsafe { SpinlockIrqsaveGuard::new(&mut (*spdif).lock) };
    unsafe {
        if (*spdif).multi_freq {
            (*ucontrol).value.integer.value[0] = (*spdif).multi_freqs[0] as i64;
            (*ucontrol).value.integer.value[1] = (*spdif).multi_freqs[1] as i64;
            (*ucontrol).value.integer.value[2] = (*spdif).multi_freqs[2] as i64;
            (*ucontrol).value.integer.value[3] = (*spdif).multi_freqs[3] as i64;
        } else {
            (*ucontrol).value.integer.value[0] = 0;
            (*ucontrol).value.integer.value[1] = 0;
            (*ucontrol).value.integer.value[2] = 0;
            (*ucontrol).value.integer.value[3] = 0;
        }
    }

    0
}

unsafe extern "C" fn img_spdif_in_set_multi_freq(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let cpu_dai = unsafe { snd_kcontrol_chip(kcontrol) };
    let spdif = unsafe { snd_soc_dai_get_drvdata(cpu_dai) };
    let mut multi_freqs: [u32; IMG_SPDIF_IN_NUM_ACLKGEN] = [0; IMG_SPDIF_IN_NUM_ACLKGEN];
    let multi_freq: bool;

    unsafe {
        if ((*ucontrol).value.integer.value[0] == 0)
            && ((*ucontrol).value.integer.value[1] == 0)
            && ((*ucontrol).value.integer.value[2] == 0)
            && ((*ucontrol).value.integer.value[3] == 0)
        {
            multi_freq = false;
        } else {
            multi_freqs[0] = (*ucontrol).value.integer.value[0] as u32;
            multi_freqs[1] = (*ucontrol).value.integer.value[1] as u32;
            multi_freqs[2] = (*ucontrol).value.integer.value[2] as u32;
            multi_freqs[3] = (*ucontrol).value.integer.value[3] as u32;
            multi_freq = true;
        }
    }

    if multi_freq {
        return unsafe { img_spdif_in_do_clkgen_multi(spdif, multi_freqs.as_mut_ptr()) };
    }

    let _guard = unsafe { SpinlockIrqsaveGuard::new(&mut (*spdif).lock) };

    if unsafe { (*spdif).active } {
        return -EBUSY;
    }

    unsafe { (*spdif).multi_freq = false };

    0
}

unsafe extern "C" fn img_spdif_in_info_lock_freq(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 1;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = LONG_MAX;
    }

    0
}

unsafe extern "C" fn img_spdif_in_get_lock_freq(
    kcontrol: *mut snd_kcontrol,
    uc: *mut snd_ctl_elem_value,
) -> i32 {
    let cpu_dai = unsafe { snd_kcontrol_chip(kcontrol) };
    let spdif = unsafe { snd_soc_dai_get_drvdata(cpu_dai) };
    let reg: U32;
    let i: i32;

    let _guard = unsafe { SpinlockIrqsaveGuard::new(&mut (*spdif).lock) };

    reg = unsafe { img_spdif_in_readl(spdif, IMG_SPDIF_IN_STATUS) };
    unsafe {
        if (reg & IMG_SPDIF_IN_STATUS_LOCK_MASK) != 0 {
            if (*spdif).multi_freq {
                i = (((reg & IMG_SPDIF_IN_STATUS_SAM_MASK) >> IMG_SPDIF_IN_STATUS_SAM_SHIFT)
                    as i32)
                    - 1;
                (*uc).value.integer.value[0] = (*spdif).multi_freqs[i as usize] as i64;
            } else {
                (*uc).value.integer.value[0] = (*spdif).single_freq as i64;
            }
        } else {
            (*uc).value.integer.value[0] = 0;
        }
    }

    0
}

unsafe extern "C" fn img_spdif_in_info_trk(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 1;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = 255;
    }

    0
}

unsafe extern "C" fn img_spdif_in_get_trk(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let cpu_dai = unsafe { snd_kcontrol_chip(kcontrol) };
    let spdif = unsafe { snd_soc_dai_get_drvdata(cpu_dai) };

    unsafe { (*ucontrol).value.integer.value[0] = (*spdif).trk as i64 };

    0
}

unsafe extern "C" fn img_spdif_in_set_trk(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let cpu_dai = unsafe { snd_kcontrol_chip(kcontrol) };
    let spdif = unsafe { snd_soc_dai_get_drvdata(cpu_dai) };
    let mut i: i32;
    let mut reg: U32;

    let _guard = unsafe { SpinlockIrqsaveGuard::new(&mut (*spdif).lock) };

    if unsafe { (*spdif).active } {
        return -EBUSY;
    }

    unsafe { (*spdif).trk = (*ucontrol).value.integer.value[0] as u32 };

    reg = unsafe { img_spdif_in_readl(spdif, IMG_SPDIF_IN_CTL) };
    reg &= !IMG_SPDIF_IN_CTL_TRK_MASK;
    reg |= unsafe { (*spdif).trk } << IMG_SPDIF_IN_CTL_TRK_SHIFT;
    unsafe { img_spdif_in_writel(spdif, reg, IMG_SPDIF_IN_CTL) };

    i = 0;
    while i < IMG_SPDIF_IN_NUM_ACLKGEN as i32 {
        unsafe {
            (*spdif).aclkgen_regs[i as usize] = ((*spdif).aclkgen_regs[i as usize]
                & !IMG_SPDIF_IN_ACLKGEN_TRK_MASK)
                | ((*spdif).trk << IMG_SPDIF_IN_ACLKGEN_TRK_SHIFT);

            img_spdif_in_aclkgen_writel(spdif, i as U32);
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn img_spdif_in_info_lock(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 1;
        (*uinfo).value.integer.min = -128;
        (*uinfo).value.integer.max = 127;
    }

    0
}

unsafe extern "C" fn img_spdif_in_get_lock_acquire(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let cpu_dai = unsafe { snd_kcontrol_chip(kcontrol) };
    let spdif = unsafe { snd_soc_dai_get_drvdata(cpu_dai) };

    unsafe { (*ucontrol).value.integer.value[0] = (*spdif).lock_acquire as i64 };

    0
}

unsafe extern "C" fn img_spdif_in_set_lock_acquire(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let cpu_dai = unsafe { snd_kcontrol_chip(kcontrol) };
    let spdif = unsafe { snd_soc_dai_get_drvdata(cpu_dai) };
    let mut reg: U32;

    let _guard = unsafe { SpinlockIrqsaveGuard::new(&mut (*spdif).lock) };

    if unsafe { (*spdif).active } {
        return -EBUSY;
    }

    unsafe { (*spdif).lock_acquire = (*ucontrol).value.integer.value[0] as i32 };

    reg = unsafe { img_spdif_in_readl(spdif, IMG_SPDIF_IN_CTL) };
    reg &= !IMG_SPDIF_IN_CTL_LOCKHI_MASK;
    reg |= ((unsafe { (*spdif).lock_acquire } as U32) << IMG_SPDIF_IN_CTL_LOCKHI_SHIFT)
        & IMG_SPDIF_IN_CTL_LOCKHI_MASK;
    unsafe { img_spdif_in_writel(spdif, reg, IMG_SPDIF_IN_CTL) };

    0
}

unsafe extern "C" fn img_spdif_in_get_lock_release(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let cpu_dai = unsafe { snd_kcontrol_chip(kcontrol) };
    let spdif = unsafe { snd_soc_dai_get_drvdata(cpu_dai) };

    unsafe { (*ucontrol).value.integer.value[0] = (*spdif).lock_release as i64 };

    0
}

unsafe extern "C" fn img_spdif_in_set_lock_release(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let cpu_dai = unsafe { snd_kcontrol_chip(kcontrol) };
    let spdif = unsafe { snd_soc_dai_get_drvdata(cpu_dai) };
    let mut reg: U32;

    let _guard = unsafe { SpinlockIrqsaveGuard::new(&mut (*spdif).lock) };

    if unsafe { (*spdif).active } {
        return -EBUSY;
    }

    unsafe { (*spdif).lock_release = (*ucontrol).value.integer.value[0] as i32 };

    reg = unsafe { img_spdif_in_readl(spdif, IMG_SPDIF_IN_CTL) };
    reg &= !IMG_SPDIF_IN_CTL_LOCKLO_MASK;
    reg |= ((unsafe { (*spdif).lock_release } as U32) << IMG_SPDIF_IN_CTL_LOCKLO_SHIFT)
        & IMG_SPDIF_IN_CTL_LOCKLO_MASK;
    unsafe { img_spdif_in_writel(spdif, reg, IMG_SPDIF_IN_CTL) };

    0
}

static mut IMG_SPDIF_IN_CONTROLS: [snd_kcontrol_new; 7] = [
    snd_kcontrol_new {
        access: unsafe { SNDRV_CTL_ELEM_ACCESS_READ },
        iface: unsafe { SNDRV_CTL_ELEM_IFACE_PCM },
        name: c"IEC958 Capture Mask".as_ptr(),
        info: Some(img_spdif_in_iec958_info),
        get: Some(img_spdif_in_get_status_mask),
        put: None,
    },
    snd_kcontrol_new {
        access: unsafe { SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE },
        iface: unsafe { SNDRV_CTL_ELEM_IFACE_PCM },
        name: c"IEC958 Capture Default".as_ptr(),
        info: Some(img_spdif_in_iec958_info),
        get: Some(img_spdif_in_get_status),
        put: None,
    },
    snd_kcontrol_new {
        access: 0,
        iface: unsafe { SNDRV_CTL_ELEM_IFACE_PCM },
        name: c"SPDIF In Multi Frequency Acquire".as_ptr(),
        info: Some(img_spdif_in_info_multi_freq),
        get: Some(img_spdif_in_get_multi_freq),
        put: Some(img_spdif_in_set_multi_freq),
    },
    snd_kcontrol_new {
        access: unsafe { SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE },
        iface: unsafe { SNDRV_CTL_ELEM_IFACE_PCM },
        name: c"SPDIF In Lock Frequency".as_ptr(),
        info: Some(img_spdif_in_info_lock_freq),
        get: Some(img_spdif_in_get_lock_freq),
        put: None,
    },
    snd_kcontrol_new {
        access: 0,
        iface: unsafe { SNDRV_CTL_ELEM_IFACE_PCM },
        name: c"SPDIF In Lock TRK".as_ptr(),
        info: Some(img_spdif_in_info_trk),
        get: Some(img_spdif_in_get_trk),
        put: Some(img_spdif_in_set_trk),
    },
    snd_kcontrol_new {
        access: 0,
        iface: unsafe { SNDRV_CTL_ELEM_IFACE_PCM },
        name: c"SPDIF In Lock Acquire Threshold".as_ptr(),
        info: Some(img_spdif_in_info_lock),
        get: Some(img_spdif_in_get_lock_acquire),
        put: Some(img_spdif_in_set_lock_acquire),
    },
    snd_kcontrol_new {
        access: 0,
        iface: unsafe { SNDRV_CTL_ELEM_IFACE_PCM },
        name: c"SPDIF In Lock Release Threshold".as_ptr(),
        info: Some(img_spdif_in_info_lock),
        get: Some(img_spdif_in_get_lock_release),
        put: Some(img_spdif_in_set_lock_release),
    },
];

unsafe extern "C" fn img_spdif_in_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: i32,
    dai: *mut snd_soc_dai,
) -> i32 {
    let spdif = unsafe { snd_soc_dai_get_drvdata(dai) };
    let mut ret: i32 = 0;
    let mut reg: U32;

    let _guard = unsafe { SpinlockIrqsaveGuard::new(&mut (*spdif).lock) };

    unsafe {
        if cmd == SNDRV_PCM_TRIGGER_START
            || cmd == SNDRV_PCM_TRIGGER_RESUME
            || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE
        {
            reg = img_spdif_in_readl(spdif, IMG_SPDIF_IN_CTL);
            if (*spdif).multi_freq {
                reg &= !IMG_SPDIF_IN_CTL_SRD_MASK;
            } else {
                reg |= 1u32 << IMG_SPDIF_IN_CTL_SRD_SHIFT;
            }
            reg |= IMG_SPDIF_IN_CTL_SRT_MASK;
            img_spdif_in_writel(spdif, reg, IMG_SPDIF_IN_CTL);
            (*spdif).active = true;
        } else if cmd == SNDRV_PCM_TRIGGER_STOP
            || cmd == SNDRV_PCM_TRIGGER_SUSPEND
            || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH
        {
            reg = img_spdif_in_readl(spdif, IMG_SPDIF_IN_CTL);
            reg &= !IMG_SPDIF_IN_CTL_SRT_MASK;
            img_spdif_in_writel(spdif, reg, IMG_SPDIF_IN_CTL);
            (*spdif).active = false;
        } else {
            ret = -EINVAL;
        }
    }

    ret
}

unsafe extern "C" fn img_spdif_in_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let spdif = unsafe { snd_soc_dai_get_drvdata(dai) };
    let rate: u32;
    let channels: u32;
    let format: SndPcmFormatT;

    rate = unsafe { params_rate(params) };
    channels = unsafe { params_channels(params) };
    format = unsafe { params_format(params) };

    if format != unsafe { SNDRV_PCM_FORMAT_S32_LE } {
        return -EINVAL;
    }

    if channels != 2 {
        return -EINVAL;
    }

    unsafe { img_spdif_in_do_clkgen_single(spdif, rate) }
}

unsafe extern "C" fn img_spdif_in_dai_probe(dai: *mut snd_soc_dai) -> i32 {
    let spdif = unsafe { snd_soc_dai_get_drvdata(dai) };

    unsafe { snd_soc_dai_init_dma_data(dai, core::ptr::null_mut(), &mut (*spdif).dma_data) };

    unsafe {
        snd_soc_add_dai_controls(
            dai,
            core::ptr::addr_of_mut!(IMG_SPDIF_IN_CONTROLS) as *mut snd_kcontrol_new,
            IMG_SPDIF_IN_CONTROLS.len(),
        )
    };

    0
}

static IMG_SPDIF_IN_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(img_spdif_in_dai_probe),
    trigger: Some(img_spdif_in_trigger),
    hw_params: Some(img_spdif_in_hw_params),
};

static mut IMG_SPDIF_IN_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    capture: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        rates: unsafe { SNDRV_PCM_RATE_8000_192000 },
        formats: unsafe { SNDRV_PCM_FMTBIT_S32_LE },
    },
    ops: &IMG_SPDIF_IN_DAI_OPS,
};

static IMG_SPDIF_IN_COMPONENT: snd_soc_component_driver = snd_soc_component_driver {
    name: c"img-spdif-in".as_ptr(),
    legacy_dai_naming: 1,
};

unsafe extern "C" fn img_spdif_in_probe(pdev: *mut platform_device) -> i32 {
    let spdif: *mut img_spdif_in;
    let mut res: *mut resource = core::ptr::null_mut();
    let base: *mut u8;
    let mut ret: i32;
    let rst: *mut reset_control;
    let mut reg: U32;
    let dev = unsafe { &mut (*pdev).dev as *mut device };

    spdif = unsafe { devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<img_spdif_in>(), GFP_KERNEL) }
        as *mut img_spdif_in;
    if spdif.is_null() {
        return -ENOMEM;
    }

    unsafe { platform_set_drvdata(pdev, spdif) };

    unsafe { (*spdif).dev = &mut (*pdev).dev };

    base = unsafe { devm_platform_get_and_ioremap_resource(pdev, 0, &mut res) };
    if unsafe { IS_ERR(base as *const core::ffi::c_void) } {
        return unsafe { PTR_ERR(base as *const core::ffi::c_void) };
    }

    unsafe { (*spdif).base = base };

    unsafe { (*spdif).clk_sys = devm_clk_get(dev, c"sys".as_ptr()) };
    if unsafe { IS_ERR((*spdif).clk_sys as *const core::ffi::c_void) } {
        return unsafe {
            dev_err_probe(
                dev,
                PTR_ERR((*spdif).clk_sys as *const core::ffi::c_void),
                c"Failed to acquire clock 'sys'\n".as_ptr(),
            )
        };
    }

    unsafe { pm_runtime_enable(&mut (*pdev).dev) };
    if !unsafe { pm_runtime_enabled(&mut (*pdev).dev) } {
        ret = unsafe { img_spdif_in_runtime_resume(&mut (*pdev).dev) };
        if ret != 0 {
            goto_err_pm_disable(&mut ret, pdev);
            return ret;
        }
    }
    ret = unsafe { pm_runtime_resume_and_get(&mut (*pdev).dev) };
    if ret < 0 {
        goto_err_suspend(&mut ret, pdev);
        return ret;
    }

    rst = unsafe { devm_reset_control_get_exclusive(&mut (*pdev).dev, c"rst".as_ptr()) };
    if unsafe { IS_ERR(rst as *const core::ffi::c_void) } {
        if unsafe { PTR_ERR(rst as *const core::ffi::c_void) } == -EPROBE_DEFER {
            ret = -EPROBE_DEFER;
            goto_err_pm_put(&mut ret, pdev);
            return ret;
        }
        unsafe {
            dev_dbg(dev, c"No top level reset found\n".as_ptr());
            img_spdif_in_writel(spdif, IMG_SPDIF_IN_SOFT_RESET_MASK, IMG_SPDIF_IN_SOFT_RESET);
            img_spdif_in_writel(spdif, 0, IMG_SPDIF_IN_SOFT_RESET);
        }
    } else {
        unsafe {
            reset_control_assert(rst);
            reset_control_deassert(rst);
        }
    }

    unsafe { spin_lock_init(&mut (*spdif).lock) };

    unsafe {
        (*spdif).dma_data.addr = (*res).start + IMG_SPDIF_IN_RX_FIFO_OFFSET as usize;
        (*spdif).dma_data.addr_width = 4;
        (*spdif).dma_data.maxburst = 4;
        (*spdif).trk = 0x80;
        (*spdif).lock_acquire = 4;
        (*spdif).lock_release = -128;
    }

    reg = ((unsafe { (*spdif).lock_acquire } as U32) << IMG_SPDIF_IN_CTL_LOCKHI_SHIFT)
        & IMG_SPDIF_IN_CTL_LOCKHI_MASK;
    reg |= ((unsafe { (*spdif).lock_release } as U32) << IMG_SPDIF_IN_CTL_LOCKLO_SHIFT)
        & IMG_SPDIF_IN_CTL_LOCKLO_MASK;
    reg |= (unsafe { (*spdif).trk } << IMG_SPDIF_IN_CTL_TRK_SHIFT) & IMG_SPDIF_IN_CTL_TRK_MASK;
    unsafe { img_spdif_in_writel(spdif, reg, IMG_SPDIF_IN_CTL) };

    unsafe { pm_runtime_put(&mut (*pdev).dev) };

    ret = unsafe {
        devm_snd_soc_register_component(
            &mut (*pdev).dev,
            &IMG_SPDIF_IN_COMPONENT,
            core::ptr::addr_of_mut!(IMG_SPDIF_IN_DAI),
            1,
        )
    };
    if ret != 0 {
        goto_err_suspend(&mut ret, pdev);
        return ret;
    }

    ret = unsafe { devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, core::ptr::null_mut(), 0) };
    if ret != 0 {
        goto_err_suspend(&mut ret, pdev);
        return ret;
    }

    0
}

unsafe fn goto_err_pm_put(ret: &mut i32, pdev: *mut platform_device) {
    unsafe { pm_runtime_put(&mut (*pdev).dev) };
    unsafe { goto_err_suspend(ret, pdev) };
}

unsafe fn goto_err_suspend(_ret: &mut i32, pdev: *mut platform_device) {
    unsafe {
        if !pm_runtime_enabled(&mut (*pdev).dev) {
            img_spdif_in_runtime_suspend(&mut (*pdev).dev);
        }
        goto_err_pm_disable(_ret, pdev);
    }
}

unsafe fn goto_err_pm_disable(_ret: &mut i32, pdev: *mut platform_device) {
    unsafe { pm_runtime_disable(&mut (*pdev).dev) };
}

unsafe extern "C" fn img_spdif_in_dev_remove(pdev: *mut platform_device) {
    unsafe {
        pm_runtime_disable(&mut (*pdev).dev);
        if !pm_runtime_status_suspended(&mut (*pdev).dev) {
            img_spdif_in_runtime_suspend(&mut (*pdev).dev);
        }
    }
}

unsafe extern "C" fn img_spdif_in_suspend(dev: *mut device) -> i32 {
    let spdif = unsafe { dev_get_drvdata(dev) };
    let ret: i32;

    if unsafe { pm_runtime_status_suspended(dev) } {
        ret = unsafe { img_spdif_in_runtime_resume(dev) };
        if ret != 0 {
            return ret;
        }
    }

    unsafe {
        (*spdif).suspend_clkgen = img_spdif_in_readl(spdif, IMG_SPDIF_IN_CLKGEN);
        (*spdif).suspend_ctl = img_spdif_in_readl(spdif, IMG_SPDIF_IN_CTL);

        img_spdif_in_runtime_suspend(dev);
    }

    0
}

unsafe extern "C" fn img_spdif_in_resume(dev: *mut device) -> i32 {
    let spdif = unsafe { dev_get_drvdata(dev) };
    let mut i: i32;
    let ret: i32;

    ret = unsafe { img_spdif_in_runtime_resume(dev) };
    if ret != 0 {
        return ret;
    }

    i = 0;
    while i < IMG_SPDIF_IN_NUM_ACLKGEN as i32 {
        unsafe { img_spdif_in_aclkgen_writel(spdif, i as U32) };
        i += 1;
    }

    unsafe {
        img_spdif_in_writel(spdif, (*spdif).suspend_clkgen, IMG_SPDIF_IN_CLKGEN);
        img_spdif_in_writel(spdif, (*spdif).suspend_ctl, IMG_SPDIF_IN_CTL);

        if pm_runtime_status_suspended(dev) {
            img_spdif_in_runtime_suspend(dev);
        }
    }

    0
}

static IMG_SPDIF_IN_OF_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: c"img,spdif-in".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, img_spdif_in_of_match);

static IMG_SPDIF_IN_PM_OPS: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(img_spdif_in_runtime_suspend),
    runtime_resume: Some(img_spdif_in_runtime_resume),
    suspend: Some(img_spdif_in_suspend),
    resume: Some(img_spdif_in_resume),
};

static mut IMG_SPDIF_IN_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: c"img-spdif-in".as_ptr(),
        of_match_table: IMG_SPDIF_IN_OF_MATCH.as_ptr(),
        pm: &IMG_SPDIF_IN_PM_OPS,
    },
    probe: Some(img_spdif_in_probe),
    remove: Some(img_spdif_in_dev_remove),
};
// module_platform_driver(img_spdif_in_driver);

// MODULE_AUTHOR("Damien Horsley <Damien.Horsley@imgtec.com>");
// MODULE_DESCRIPTION("IMG SPDIF Input driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
