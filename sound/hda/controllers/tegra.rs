// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Implementation of primary ALSA driver code base for NVIDIA Tegra HDA.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::offset_of;
use core::ptr::{addr_of_mut, null, null_mut};

/* Defines for Nvidia Tegra HDA support */
const HDA_BAR0: usize = 0x8000;

const HDA_CFG_CMD: usize = 0x1004;
const HDA_CFG_BAR0: usize = 0x1010;

const HDA_ENABLE_IO_SPACE: u32 = 1 << 0;
const HDA_ENABLE_MEM_SPACE: u32 = 1 << 1;
const HDA_ENABLE_BUS_MASTER: u32 = 1 << 2;
const HDA_ENABLE_SERR: u32 = 1 << 8;
const HDA_DISABLE_INTR: u32 = 1 << 10;
const HDA_BAR0_INIT_PROGRAM: u32 = 0xFFFFFFFF;
const HDA_BAR0_FINAL_PROGRAM: u32 = 1 << 14;

/* IPFS */
const HDA_IPFS_CONFIG: usize = 0x180;
const HDA_IPFS_EN_FPCI: u32 = 0x1;

const HDA_IPFS_FPCI_BAR0: usize = 0x80;
const HDA_FPCI_BAR0_START: u32 = 0x40;

const HDA_IPFS_INTR_MASK: usize = 0x188;
const HDA_IPFS_EN_INTR: u32 = 1 << 16;

/* FPCI */
const FPCI_DBG_CFG_2: usize = 0x10F4;
const FPCI_GCAP_NSDO_SHIFT: u32 = 18;
const FPCI_GCAP_NSDO_MASK: u32 = 0x3 << FPCI_GCAP_NSDO_SHIFT;

/* max number of SDs */
const NUM_CAPTURE_SD: c_uint = 1;
const NUM_PLAYBACK_SD: c_uint = 1;

/*
 * Tegra194 does not reflect correct number of SDO lines. Below macro
 * is used to update the GCAP register to workaround the issue.
 */
const TEGRA194_NUM_SDO_LINES: u32 = 4;

const IRQF_SHARED: c_ulong = 0;
const GFP_KERNEL: c_uint = 0;
const SNDRV_CTL_POWER_D3HOT: c_int = 0;
const SNDRV_CTL_POWER_D0: c_int = 0;
const SNDRV_DEV_LOWLEVEL: c_int = 0;
const SNDRV_DEFAULT_IDX1: c_int = 0;
static SNDRV_DEFAULT_STR1: *const c_char = null();
const THIS_MODULE: *mut c_void = null_mut();
const KBUILD_MODNAME: *const c_char = b"tegra-hda\0".as_ptr() as *const c_char;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const WAKEEN: c_uint = 0;
const GCAP: c_uint = 0;
const STATESTS_INT_MASK: u16 = 0;
const AZX_DCAPS_CORBRP_SELF_CLEAR: c_uint = 0;
const AZX_DCAPS_PM_RUNTIME: c_uint = 0;
const AZX_DCAPS_4K_BDLE_BOUNDARY: c_uint = 0;

#[repr(C)]
pub struct hda_tegra_soc {
    pub has_hda2codec_2x_reset: bool,
    pub has_hda2hdmi: bool,
    pub has_hda2codec_2x: bool,
    pub input_stream: bool,
    pub always_on: bool,
    pub requires_init: bool,
}

#[repr(C)]
pub struct hda_tegra {
    pub chip: azx,
    pub dev: *mut device,
    pub resets: [reset_control_bulk_data; 3],
    pub clocks: [clk_bulk_data; 3],
    pub nresets: c_uint,
    pub nclocks: c_uint,
    pub regs: *mut c_void,
    pub probe_work: work_struct,
    pub soc: *const hda_tegra_soc,
}

#[repr(C)]
pub struct azx {
    pub bus: hdac_bus,
    pub card: *mut snd_card,
    pub ops: *const hda_controller_ops,
    pub driver_caps: c_uint,
    pub driver_type: c_uint,
    pub dev_index: c_int,
    pub pcm_list: list_head,
    pub codec_probe_mask: c_int,
    pub single_cmd: bool,
    pub snoop: bool,
    pub open_mutex: mutex,
    pub align_buffer_size: c_int,
    pub capture_streams: c_uint,
    pub playback_streams: c_uint,
    pub capture_index_offset: c_uint,
    pub playback_index_offset: c_uint,
    pub num_streams: c_uint,
    pub jackpoll_interval: c_ulong,
    pub running: c_int,
}

#[repr(C)]
pub struct hdac_bus {
    pub core: hdac_bus_core,
    pub shutdown: c_int,
    pub remap_addr: *mut c_void,
    pub addr: c_ulong,
    pub irq: c_int,
    pub dma_stop_delay: c_int,
    pub codec_mask: c_uint,
    pub jackpoll_in_suspend: c_int,
}

#[repr(C)]
pub struct hdac_bus_core {
    pub sync_write: c_int,
    pub needs_damn_long_delay: c_int,
    pub aligned_mmio: c_int,
    pub sdo_limit: c_int,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub private_data: *mut c_void,
    pub sync_irq: c_int,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct snd_device {
    pub device_data: *mut c_void,
}

#[repr(C)]
pub struct snd_device_ops {
    pub dev_disconnect: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
    pub dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct resource {
    pub start: c_ulong,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_idle: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct reset_control_bulk_data {
    pub id: *const c_char,
}

#[repr(C)]
pub struct clk_bulk_data {
    pub id: *const c_char,
}

#[repr(C)]
pub struct work_struct {
    _private: [usize; 0],
}

#[repr(C)]
pub struct hda_controller_ops {
    _private: [usize; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [usize; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [usize; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [usize; 0],
}

#[cfg(CONFIG_PM)]
static mut power_save: c_int = CONFIG_SND_HDA_POWER_SAVE_DEFAULT;
#[cfg(not(CONFIG_PM))]
const power_save: c_int = 0;
/* module_param(power_save, bint, 0644); */
/* MODULE_PARM_DESC(power_save,
 *		 "Automatic power-saving timeout (in seconds, 0 = disable).");
 */

static hda_tegra_ops: hda_controller_ops = hda_controller_ops { _private: [] }; /* nothing special */

unsafe extern "C" {
    fn readl(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_forbid(dev: *mut device);
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn azx_readw(chip: *mut azx, reg: c_uint) -> u16;
    fn azx_writew(chip: *mut azx, reg: c_uint, value: u16);
    fn azx_stop_chip(chip: *mut azx);
    fn azx_enter_link_reset(chip: *mut azx);
    fn clk_bulk_disable_unprepare(nclocks: c_uint, clocks: *mut clk_bulk_data);
    fn reset_control_bulk_assert(nresets: c_uint, resets: *mut reset_control_bulk_data) -> c_int;
    fn clk_bulk_prepare_enable(nclocks: c_uint, clocks: *mut clk_bulk_data) -> c_int;
    fn reset_control_bulk_deassert(nresets: c_uint, resets: *mut reset_control_bulk_data) -> c_int;
    fn azx_init_chip(chip: *mut azx, full_reset: c_int);
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn azx_stop_all_streams(chip: *mut azx);
    fn azx_free_stream_pages(chip: *mut azx);
    fn azx_free_streams(chip: *mut azx);
    fn snd_hdac_bus_exit(bus: *mut hdac_bus);
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> c_int,
        flags: c_ulong,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn azx_interrupt(irq: c_int, dev_id: *mut c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn of_device_is_compatible(np: *mut device_node, compat: *const c_char) -> c_int;
    fn azx_init_streams(chip: *mut azx) -> c_int;
    fn azx_alloc_stream_pages(chip: *mut azx) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn of_get_property(np: *mut device_node, name: *const c_char, lenp: *mut c_int) -> *const c_char;
    fn strlen(s: *const c_char) -> usize;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn azx_bus_init(chip: *mut azx, model: *const c_char) -> c_int;
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn snd_device_new(
        card: *mut snd_card,
        ty: c_int,
        device_data: *mut c_void,
        ops: *const snd_device_ops,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn snd_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *const c_char,
        module: *mut c_void,
        extra_size: c_int,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn devm_reset_control_bulk_get_exclusive(
        dev: *mut device,
        num_rstcs: c_uint,
        rstcs: *mut reset_control_bulk_data,
    ) -> c_int;
    fn devm_clk_bulk_get(dev: *mut device, num_clks: c_uint, clks: *mut clk_bulk_data) -> c_int;
    fn snd_card_free(card: *mut snd_card);
    fn azx_has_pm_runtime(chip: *mut azx) -> bool;
    fn schedule_work(work: *mut work_struct) -> bool;
    fn to_platform_device(dev: *mut device) -> *mut platform_device;
    fn pm_runtime_active(dev: *mut device);
    fn azx_probe_codecs(chip: *mut azx, max_slots: c_uint) -> c_int;
    fn azx_codec_configure(chip: *mut azx) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_hda_set_power_save(bus: *mut hdac_bus, delay: c_int);
}

#[inline]
unsafe fn azx_bus(chip: *mut azx) -> *mut hdac_bus {
    addr_of_mut!((*chip).bus)
}

#[inline]
unsafe fn hda_from_chip(chip: *mut azx) -> *mut hda_tegra {
    (chip as *mut u8).sub(offset_of!(hda_tegra, chip)) as *mut hda_tegra
}

#[inline]
unsafe fn hda_from_work(work: *mut work_struct) -> *mut hda_tegra {
    (work as *mut u8).sub(offset_of!(hda_tegra, probe_work)) as *mut hda_tegra
}

unsafe extern "C" fn hda_tegra_init(hda: *mut hda_tegra) {
    let mut v: u32;

    /* Enable PCI access */
    v = readl((*hda).regs.add(HDA_IPFS_CONFIG) as *const c_void);
    v |= HDA_IPFS_EN_FPCI;
    writel(v, (*hda).regs.add(HDA_IPFS_CONFIG));

    /* Enable MEM/IO space and bus master */
    v = readl((*hda).regs.add(HDA_CFG_CMD) as *const c_void);
    v &= !HDA_DISABLE_INTR;
    v |= HDA_ENABLE_MEM_SPACE | HDA_ENABLE_IO_SPACE | HDA_ENABLE_BUS_MASTER | HDA_ENABLE_SERR;
    writel(v, (*hda).regs.add(HDA_CFG_CMD));

    writel(HDA_BAR0_INIT_PROGRAM, (*hda).regs.add(HDA_CFG_BAR0));
    writel(HDA_BAR0_FINAL_PROGRAM, (*hda).regs.add(HDA_CFG_BAR0));
    writel(HDA_FPCI_BAR0_START, (*hda).regs.add(HDA_IPFS_FPCI_BAR0));

    v = readl((*hda).regs.add(HDA_IPFS_INTR_MASK) as *const c_void);
    v |= HDA_IPFS_EN_INTR;
    writel(v, (*hda).regs.add(HDA_IPFS_INTR_MASK));
}

/*
 * power management
 */
unsafe extern "C" fn hda_tegra_suspend(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let mut rc: c_int;

    rc = pm_runtime_force_suspend(dev);
    if rc < 0 {
        return rc;
    }
    snd_power_change_state(card, SNDRV_CTL_POWER_D3HOT);

    0
}

unsafe extern "C" fn hda_tegra_resume(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let mut rc: c_int;

    rc = pm_runtime_force_resume(dev);
    if rc < 0 {
        return rc;
    }
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);

    0
}

unsafe extern "C" fn hda_tegra_runtime_suspend(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let chip = (*card).private_data as *mut azx;
    let hda = hda_from_chip(chip);

    if !chip.is_null() && (*chip).running != 0 {
        /* enable controller wake up event */
        azx_writew(chip, WAKEEN, azx_readw(chip, WAKEEN) | STATESTS_INT_MASK);

        azx_stop_chip(chip);
        azx_enter_link_reset(chip);
    }
    clk_bulk_disable_unprepare((*hda).nclocks, (*hda).clocks.as_mut_ptr());

    0
}

unsafe extern "C" fn hda_tegra_runtime_resume(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let chip = (*card).private_data as *mut azx;
    let hda = hda_from_chip(chip);
    let mut rc: c_int;

    if (*chip).running == 0 {
        rc = reset_control_bulk_assert((*hda).nresets, (*hda).resets.as_mut_ptr());
        if rc != 0 {
            return rc;
        }
    }

    rc = clk_bulk_prepare_enable((*hda).nclocks, (*hda).clocks.as_mut_ptr());
    if rc != 0 {
        return rc;
    }
    if (*chip).running != 0 {
        if (*(*hda).soc).requires_init {
            hda_tegra_init(hda);
        }

        azx_init_chip(chip, 1);
        /* disable controller wake up event*/
        azx_writew(chip, WAKEEN, azx_readw(chip, WAKEEN) & !STATESTS_INT_MASK);
    } else {
        usleep_range(10, 100);

        rc = reset_control_bulk_deassert((*hda).nresets, (*hda).resets.as_mut_ptr());
        if rc != 0 {
            return rc;
        }
    }

    0
}

static hda_tegra_pm: dev_pm_ops = dev_pm_ops {
    suspend: Some(hda_tegra_suspend),
    resume: Some(hda_tegra_resume),
    runtime_suspend: Some(hda_tegra_runtime_suspend),
    runtime_resume: Some(hda_tegra_runtime_resume),
    runtime_idle: None,
};

unsafe extern "C" fn hda_tegra_dev_disconnect(device: *mut snd_device) -> c_int {
    let chip = (*device).device_data as *mut azx;

    (*chip).bus.shutdown = 1;
    0
}

/*
 * destructor
 */
unsafe extern "C" fn hda_tegra_dev_free(device: *mut snd_device) -> c_int {
    let chip = (*device).device_data as *mut azx;
    let hda = hda_from_chip(chip);

    cancel_work_sync(addr_of_mut!((*hda).probe_work));
    if (*azx_bus(chip)).core.sync_write != 0 {
        azx_stop_all_streams(chip);
        azx_stop_chip(chip);
    }

    azx_free_stream_pages(chip);
    azx_free_streams(chip);
    snd_hdac_bus_exit(azx_bus(chip));

    0
}

unsafe extern "C" fn hda_tegra_init_chip(chip: *mut azx, pdev: *mut platform_device) -> c_int {
    let hda = hda_from_chip(chip);
    let bus = azx_bus(chip);
    let mut res: *mut resource = null_mut();

    (*hda).regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR((*hda).regs) {
        return PTR_ERR((*hda).regs);
    }

    (*bus).remap_addr = (*hda).regs.add(HDA_BAR0);
    (*bus).addr = (*res).start + HDA_BAR0 as c_ulong;

    if (*(*hda).soc).requires_init {
        hda_tegra_init(hda);
    }

    0
}

unsafe extern "C" fn hda_tegra_first_init(chip: *mut azx, pdev: *mut platform_device) -> c_int {
    let hda = hda_from_chip(chip);
    let bus = azx_bus(chip);
    let card = (*chip).card;
    let mut err: c_int;
    let mut gcap: u16;
    let irq_id = platform_get_irq(pdev, 0);
    let mut sname: *const c_char;
    let drv_name: *const c_char = b"tegra-hda\0".as_ptr() as *const c_char;
    let np = (*pdev).dev.of_node;

    if irq_id < 0 {
        return irq_id;
    }

    err = hda_tegra_init_chip(chip, pdev);
    if err != 0 {
        return err;
    }

    err = devm_request_irq(
        (*(*chip).card).dev,
        irq_id,
        azx_interrupt,
        IRQF_SHARED,
        KBUILD_MODNAME,
        chip as *mut c_void,
    );
    if err != 0 {
        dev_err(
            (*(*chip).card).dev,
            b"unable to request IRQ %d, disabling device\n\0".as_ptr() as *const c_char,
            irq_id,
        );
        return err;
    }
    (*bus).irq = irq_id;
    (*bus).dma_stop_delay = 100;
    (*card).sync_irq = (*bus).irq;

    /*
     * Tegra194 has 4 SDO lines and the STRIPE can be used to
     * indicate how many of the SDO lines the stream should be
     * striped. But GCAP register does not reflect the true
     * capability of HW. Below workaround helps to fix this.
     *
     * GCAP_NSDO is bits 19:18 in T_AZA_DBG_CFG_2,
     * 0 for 1 SDO, 1 for 2 SDO, 2 for 4 SDO lines.
     */
    if of_device_is_compatible(np, b"nvidia,tegra194-hda\0".as_ptr() as *const c_char) != 0 {
        let mut val: u32;

        dev_info(
            (*card).dev,
            b"Override SDO lines to %u\n\0".as_ptr() as *const c_char,
            TEGRA194_NUM_SDO_LINES,
        );

        val = readl((*hda).regs.add(FPCI_DBG_CFG_2) as *const c_void) & !FPCI_GCAP_NSDO_MASK;
        val |= (TEGRA194_NUM_SDO_LINES >> 1) << FPCI_GCAP_NSDO_SHIFT;
        writel(val, (*hda).regs.add(FPCI_DBG_CFG_2));
    }

    gcap = azx_readw(chip, GCAP);
    dev_dbg(
        (*card).dev,
        b"chipset global capabilities = 0x%x\n\0".as_ptr() as *const c_char,
        gcap as c_uint,
    );

    (*chip).align_buffer_size = 1;

    /* read number of streams from GCAP register instead of using
     * hardcoded value
     */
    (*chip).capture_streams = ((gcap >> 8) & 0x0f) as c_uint;

    /* The GCAP register on Tegra234 implies no Input Streams(ISS) support,
     * but the HW output stream descriptor programming should start with
     * offset 0x20*4 from base stream descriptor address. This will be a
     * problem while calculating the offset for output stream descriptor
     * which will be considering input stream also. So here output stream
     * starts with offset 0 which is wrong as HW register for output stream
     * offset starts with 4.
     */
    if !(*(*hda).soc).input_stream {
        (*chip).capture_streams = 4;
    }

    (*chip).playback_streams = ((gcap >> 12) & 0x0f) as c_uint;
    if (*chip).playback_streams == 0 && (*chip).capture_streams == 0 {
        /* gcap didn't give any info, switching to old method */
        (*chip).playback_streams = NUM_PLAYBACK_SD;
        (*chip).capture_streams = NUM_CAPTURE_SD;
    }
    (*chip).capture_index_offset = 0;
    (*chip).playback_index_offset = (*chip).capture_streams;
    (*chip).num_streams = (*chip).playback_streams + (*chip).capture_streams;

    /* initialize streams */
    err = azx_init_streams(chip);
    if err < 0 {
        dev_err(
            (*card).dev,
            b"failed to initialize streams: %d\n\0".as_ptr() as *const c_char,
            err,
        );
        return err;
    }

    err = azx_alloc_stream_pages(chip);
    if err < 0 {
        dev_err(
            (*card).dev,
            b"failed to allocate stream pages: %d\n\0".as_ptr() as *const c_char,
            err,
        );
        return err;
    }

    /* initialize chip */
    azx_init_chip(chip, 1);

    /*
     * Playback (for 44.1K/48K, 2-channel, 16-bps) fails with
     * 4 SDO lines due to legacy design limitation. Following
     * is, from HD Audio Specification (Revision 1.0a), used to
     * control striping of the stream across multiple SDO lines
     * for sample rates <= 48K.
     *
     * { ((num_channels * bits_per_sample) / number of SDOs) >= 8 }
     *
     * Due to legacy design issue it is recommended that above
     * ratio must be greater than 8. Since number of SDO lines is
     * in powers of 2, next available ratio is 16 which can be
     * used as a limiting factor here.
     */
    if of_device_is_compatible(np, b"nvidia,tegra30-hda\0".as_ptr() as *const c_char) != 0 {
        (*chip).bus.core.sdo_limit = 16;
    }

    /* codec detection */
    if (*bus).codec_mask == 0 {
        dev_err((*card).dev, b"no codecs found!\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    /* driver name */
    strscpy((*card).driver.as_mut_ptr(), drv_name);
    /* shortname for card */
    sname = of_get_property(np, b"nvidia,model\0".as_ptr() as *const c_char, null_mut());
    if sname.is_null() {
        sname = drv_name;
    }
    if strlen(sname) > (*card).shortname.len() {
        dev_info((*card).dev, b"truncating shortname for card\n\0".as_ptr() as *const c_char);
    }
    strscpy((*card).shortname.as_mut_ptr(), sname);

    /* longname for card */
    snprintf(
        (*card).longname.as_mut_ptr(),
        (*card).longname.len(),
        b"%s at 0x%lx irq %i\0".as_ptr() as *const c_char,
        (*card).shortname.as_ptr(),
        (*bus).addr,
        (*bus).irq,
    );

    0
}

/*
 * constructor
 */

unsafe extern "C" fn hda_tegra_probe_work(work: *mut work_struct) {
    let hda = hda_from_work(work);
    let chip = addr_of_mut!((*hda).chip);
    let pdev = to_platform_device((*hda).dev);
    let mut err: c_int;

    pm_runtime_active((*hda).dev);
    err = hda_tegra_first_init(chip, pdev);
    if err < 0 {
        return;
    }

    /* create codec instances */
    err = azx_probe_codecs(chip, 8);
    if err < 0 {
        return;
    }

    err = azx_codec_configure(chip);
    if err < 0 {
        return;
    }

    err = snd_card_register((*chip).card);
    if err < 0 {
        return;
    }

    (*chip).running = 1;
    snd_hda_set_power_save(addr_of_mut!((*chip).bus), power_save * 1000);
}

unsafe extern "C" fn hda_tegra_create(
    card: *mut snd_card,
    driver_caps: c_uint,
    hda: *mut hda_tegra,
) -> c_int {
    static ops: snd_device_ops = snd_device_ops {
        dev_disconnect: Some(hda_tegra_dev_disconnect),
        dev_free: Some(hda_tegra_dev_free),
    };
    let chip: *mut azx;
    let mut err: c_int;

    chip = addr_of_mut!((*hda).chip);

    mutex_init(addr_of_mut!((*chip).open_mutex));
    (*chip).card = card;
    (*chip).ops = &hda_tegra_ops;
    (*chip).driver_caps = driver_caps;
    (*chip).driver_type = driver_caps & 0xff;
    (*chip).dev_index = 0;
    INIT_LIST_HEAD(addr_of_mut!((*chip).pcm_list));

    (*chip).codec_probe_mask = -1;

    (*chip).single_cmd = false;
    (*chip).snoop = true;

    INIT_WORK(addr_of_mut!((*hda).probe_work), hda_tegra_probe_work);

    err = azx_bus_init(chip, null());
    if err < 0 {
        return err;
    }

    (*chip).bus.core.sync_write = 0;
    (*chip).bus.core.needs_damn_long_delay = 1;
    (*chip).bus.core.aligned_mmio = 1;

    /*
     * HDA power domain and clocks are always on for Tegra264 and
     * the jack detection logic would work always, so no need of
     * jack polling mechanism running.
     */
    if !(*(*hda).soc).always_on {
        (*chip).jackpoll_interval = msecs_to_jiffies(5000);
        (*chip).bus.jackpoll_in_suspend = 1;
    }

    err = snd_device_new(card, SNDRV_DEV_LOWLEVEL, chip as *mut c_void, &ops);
    if err < 0 {
        dev_err((*card).dev, b"Error creating device\n\0".as_ptr() as *const c_char);
        return err;
    }

    0
}

static tegra30_data: hda_tegra_soc = hda_tegra_soc {
    has_hda2codec_2x_reset: true,
    has_hda2hdmi: true,
    has_hda2codec_2x: true,
    input_stream: true,
    always_on: false,
    requires_init: true,
};

static tegra194_data: hda_tegra_soc = hda_tegra_soc {
    has_hda2codec_2x_reset: false,
    has_hda2hdmi: true,
    has_hda2codec_2x: true,
    input_stream: true,
    always_on: false,
    requires_init: true,
};

static tegra234_data: hda_tegra_soc = hda_tegra_soc {
    has_hda2codec_2x_reset: true,
    has_hda2hdmi: false,
    has_hda2codec_2x: true,
    input_stream: false,
    always_on: false,
    requires_init: true,
};

static tegra264_data: hda_tegra_soc = hda_tegra_soc {
    has_hda2codec_2x_reset: true,
    has_hda2hdmi: false,
    has_hda2codec_2x: false,
    input_stream: false,
    always_on: true,
    requires_init: false,
};

static hda_tegra_match: [of_device_id; 5] = [
    of_device_id {
        compatible: b"nvidia,tegra30-hda\0".as_ptr() as *const c_char,
        data: &tegra30_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"nvidia,tegra194-hda\0".as_ptr() as *const c_char,
        data: &tegra194_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"nvidia,tegra234-hda\0".as_ptr() as *const c_char,
        data: &tegra234_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"nvidia,tegra264-hda\0".as_ptr() as *const c_char,
        data: &tegra264_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: null(),
        data: null(),
    },
];
/* MODULE_DEVICE_TABLE(of, hda_tegra_match); */

unsafe extern "C" fn hda_tegra_probe(pdev: *mut platform_device) -> c_int {
    let driver_flags: c_uint =
        AZX_DCAPS_CORBRP_SELF_CLEAR | AZX_DCAPS_PM_RUNTIME | AZX_DCAPS_4K_BDLE_BOUNDARY;
    let mut card: *mut snd_card = null_mut();
    let chip: *mut azx;
    let hda: *mut hda_tegra;
    let mut err: c_int;

    hda = devm_kzalloc(
        addr_of_mut!((*pdev).dev),
        core::mem::size_of::<hda_tegra>(),
        GFP_KERNEL,
    ) as *mut hda_tegra;
    if hda.is_null() {
        return -ENOMEM;
    }
    (*hda).dev = addr_of_mut!((*pdev).dev);
    chip = addr_of_mut!((*hda).chip);

    (*hda).soc = of_device_get_match_data(addr_of_mut!((*pdev).dev)) as *const hda_tegra_soc;

    err = snd_card_new(
        addr_of_mut!((*pdev).dev),
        SNDRV_DEFAULT_IDX1,
        SNDRV_DEFAULT_STR1,
        THIS_MODULE,
        0,
        &mut card,
    );
    if err < 0 {
        dev_err(addr_of_mut!((*pdev).dev), b"Error creating card!\n\0".as_ptr() as *const c_char);
        return err;
    }

    (*hda).resets[(*hda).nresets as usize].id = b"hda\0".as_ptr() as *const c_char;
    (*hda).nresets += 1;

    /*
     * "hda2hdmi" is not applicable for Tegra234. This is because the
     * codec is separate IP and not under display SOR partition now.
     */
    if (*(*hda).soc).has_hda2hdmi {
        (*hda).resets[(*hda).nresets as usize].id = b"hda2hdmi\0".as_ptr() as *const c_char;
        (*hda).nresets += 1;
    }

    /*
     * "hda2codec_2x" reset is not present on Tegra194. Though DT would
     * be updated to reflect this, but to have backward compatibility
     * below is necessary.
     */
    if (*(*hda).soc).has_hda2codec_2x_reset {
        (*hda).resets[(*hda).nresets as usize].id = b"hda2codec_2x\0".as_ptr() as *const c_char;
        (*hda).nresets += 1;
    }

    err = devm_reset_control_bulk_get_exclusive(
        addr_of_mut!((*pdev).dev),
        (*hda).nresets,
        (*hda).resets.as_mut_ptr(),
    );
    if err != 0 {
        snd_card_free(card);
        return err;
    }

    (*hda).clocks[(*hda).nclocks as usize].id = b"hda\0".as_ptr() as *const c_char;
    (*hda).nclocks += 1;
    if (*(*hda).soc).has_hda2hdmi {
        (*hda).clocks[(*hda).nclocks as usize].id = b"hda2hdmi\0".as_ptr() as *const c_char;
        (*hda).nclocks += 1;
    }

    if (*(*hda).soc).has_hda2codec_2x {
        (*hda).clocks[(*hda).nclocks as usize].id = b"hda2codec_2x\0".as_ptr() as *const c_char;
        (*hda).nclocks += 1;
    }

    err = devm_clk_bulk_get(
        addr_of_mut!((*pdev).dev),
        (*hda).nclocks,
        (*hda).clocks.as_mut_ptr(),
    );
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    err = hda_tegra_create(card, driver_flags, hda);
    if err < 0 {
        snd_card_free(card);
        return err;
    }
    (*card).private_data = chip as *mut c_void;

    dev_set_drvdata(addr_of_mut!((*pdev).dev), card as *mut c_void);

    pm_runtime_enable((*hda).dev);
    if !azx_has_pm_runtime(chip) {
        pm_runtime_forbid((*hda).dev);
    }

    schedule_work(addr_of_mut!((*hda).probe_work));

    0
}

unsafe extern "C" fn hda_tegra_remove(pdev: *mut platform_device) {
    snd_card_free(dev_get_drvdata(addr_of_mut!((*pdev).dev)) as *mut snd_card);
    pm_runtime_disable(addr_of_mut!((*pdev).dev));
}

unsafe extern "C" fn hda_tegra_shutdown(pdev: *mut platform_device) {
    let card = dev_get_drvdata(addr_of_mut!((*pdev).dev)) as *mut snd_card;
    let chip: *mut azx;

    if card.is_null() {
        return;
    }
    chip = (*card).private_data as *mut azx;
    if !chip.is_null() && (*chip).running != 0 {
        azx_stop_chip(chip);
    }
}

static mut tegra_platform_hda: platform_driver = platform_driver {
    driver: device_driver {
        name: b"tegra-hda\0".as_ptr() as *const c_char,
        pm: &hda_tegra_pm,
        of_match_table: hda_tegra_match.as_ptr(),
    },
    probe: Some(hda_tegra_probe),
    remove: Some(hda_tegra_remove),
    shutdown: Some(hda_tegra_shutdown),
};
/* module_platform_driver(tegra_platform_hda); */

/* MODULE_DESCRIPTION("Tegra HDA bus driver"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
