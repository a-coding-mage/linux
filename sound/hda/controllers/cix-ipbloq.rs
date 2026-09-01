// SPDX-License-Identifier: GPL-2.0
// Copyright 2025 Cix Technology Group Co., Ltd.

// Rust translation of hda/controllers/cix-ipbloq.c.
// Kernel includes from the C source are dependencies supplied by the surrounding tree.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const CIX_IPBLOQ_JACKPOLL_DEFAULT_TIME_MS: c_uint = 1000;
const CIX_IPBLOQ_POWER_SAVE_DEFAULT_TIME_MS: c_uint = 100;

const CIX_IPBLOQ_SKY1_ADDR_HOST_TO_HDAC_OFFSET: u64 = 0u64.wrapping_sub(0x90000000u64);

type DmaAddrT = u64;

const GFP_KERNEL: c_uint = 0;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const SNDRV_DEV_LOWLEVEL: c_int = 0;
const SNDRV_DEFAULT_IDX1: c_int = 0;
static SNDRV_DEFAULT_STR1: *const c_char = ptr::null();
const SNDRV_CTL_POWER_D3COLD: c_int = 0;
const SNDRV_CTL_POWER_D0: c_int = 0;
const AZX_DCAPS_PM_RUNTIME: c_uint = 0;
const GCAP: c_int = 0;

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    start: c_ulong,
}

#[repr(C)]
pub struct reset_control {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_bulk_data {
    id: *const c_char,
}

#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
pub struct snd_device {
    device_data: *mut c_void,
}

#[repr(C)]
pub struct snd_device_ops {
    dev_disconnect: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
    dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}

#[repr(C)]
pub struct snd_card {
    dev: *mut device,
    sync_irq: c_int,
    driver: [c_char; 32],
    shortname: [c_char; 80],
    longname: [c_char; 80],
    private_data: *mut c_void,
}

#[repr(C)]
pub struct hdac_bus_core {
    polling_mode: c_int,
    not_use_interrupts: c_int,
    aligned_mmio: c_int,
    dma_stop_delay: c_int,
    addr_offset: DmaAddrT,
}

#[repr(C)]
pub struct hda_bus {
    bus_probing: c_int,
}

#[repr(C)]
pub struct hdac_bus {
    shutdown: c_int,
    core: hdac_bus_core,
    remap_addr: *mut c_void,
    addr: c_ulong,
    irq: c_int,
    codec_mask: c_ulong,
    chip_init: c_int,
    jackpoll_in_suspend: c_int,
}

#[repr(C)]
pub struct hda_controller_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct azx {
    bus: hdac_bus,
    card: *mut snd_card,
    ops: *const hda_controller_ops,
    driver_caps: c_uint,
    driver_type: c_uint,
    dev_index: c_int,
    single_cmd: c_int,
    codec_probe_mask: c_int,
    align_buffer_size: c_int,
    jackpoll_interval: c_ulong,
    open_mutex: mutex,
    pcm_list: list_head,
    get_position: [Option<unsafe extern "C" fn()>; 2],
    capture_streams: c_uint,
    playback_streams: c_uint,
    capture_index_offset: c_uint,
    playback_index_offset: c_uint,
    num_streams: c_uint,
    running: c_int,
}

#[repr(C)]
pub struct cix_ipbloq_hda {
    chip: azx,
    dev: *mut device,
    regs: *mut c_void,

    reset: *mut reset_control,
    clocks: [clk_bulk_data; 2],
    nclocks: c_uint,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver_driver {
    name: *const c_char,
    pm: *const dev_pm_ops,
    of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    driver: platform_driver_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    shutdown: Option<unsafe extern "C" fn(*mut platform_device)>,
}

unsafe extern "C" {
    static cix_ipbloq_hda_ops: hda_controller_ops;
    static THIS_MODULE: *mut c_void;
    static azx_get_pos_lpib: Option<unsafe extern "C" fn()>;

    fn azx_bus(chip: *mut azx) -> *mut hdac_bus;
    fn to_hda_bus(bus: *mut hdac_bus) -> *mut hda_bus;
    fn azx_stop_all_streams(chip: *mut azx);
    fn azx_stop_chip(chip: *mut azx);
    fn azx_free_stream_pages(chip: *mut azx);
    fn azx_free_streams(chip: *mut azx);
    fn snd_hdac_bus_exit(bus: *mut hdac_bus);
    fn azx_probe_codecs(chip: *mut azx, max_slots: c_int) -> c_int;
    fn azx_codec_configure(chip: *mut azx) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_hda_set_power_save(bus: *mut hdac_bus, delay: c_uint);
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: unsafe extern "C" fn(),
        flags: c_uint,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn azx_interrupt();
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn azx_readw(chip: *mut azx, reg: c_int) -> u16;
    fn azx_init_streams(chip: *mut azx) -> c_int;
    fn azx_alloc_stream_pages(chip: *mut azx) -> c_int;
    fn azx_init_chip(chip: *mut azx, full_reset: c_int);
    fn strscpy(dst: *mut c_char, src: *const c_char, size: usize) -> isize;
    fn of_get_property(
        np: *mut device_node,
        name: *const c_char,
        lenp: *mut c_int,
    ) -> *const c_char;
    fn strlen(s: *const c_char) -> usize;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn mutex_init(lock: *mut mutex);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn azx_bus_init(chip: *mut azx, model: *const c_void) -> c_int;
    fn snd_device_new(
        card: *mut snd_card,
        ty: c_int,
        device_data: *mut c_void,
        ops: *const snd_device_ops,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_reset_control_get(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn devm_clk_bulk_get(
        dev: *mut device,
        num_clks: c_uint,
        clks: *mut clk_bulk_data,
    ) -> c_int;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn of_reserved_mem_device_init(dev: *mut device) -> c_int;
    fn snd_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *const c_char,
        module: *mut c_void,
        extra_size: c_int,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn pm_runtime_enable(dev: *mut device);
    fn azx_has_pm_runtime(chip: *mut azx) -> bool;
    fn pm_runtime_forbid(dev: *mut device);
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put(dev: *mut device);
    fn snd_device_free(card: *mut snd_card, device_data: *mut c_void);
    fn snd_card_free(card: *mut snd_card);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn azx_enter_link_reset(chip: *mut azx);
    fn clk_bulk_disable_unprepare(num_clks: c_uint, clks: *mut clk_bulk_data);
    fn clk_bulk_prepare_enable(num_clks: c_uint, clks: *mut clk_bulk_data) -> c_int;
    fn reset_control_assert(rstc: *mut reset_control) -> c_int;
    fn reset_control_deassert(rstc: *mut reset_control) -> c_int;
}

const fn DMA_BIT_MASK(nr: u32) -> u64 {
    if nr == 64 {
        !0u64
    } else {
        (1u64 << nr).wrapping_sub(1)
    }
}

unsafe extern "C" fn cix_ipbloq_hda_dev_disconnect(device: *mut snd_device) -> c_int {
    let chip = unsafe { (*device).device_data as *mut azx };

    unsafe {
        (*chip).bus.shutdown = 1;
    }

    0
}

unsafe extern "C" fn cix_ipbloq_hda_dev_free(device: *mut snd_device) -> c_int {
    let chip = unsafe { (*device).device_data as *mut azx };

    unsafe {
        if (*azx_bus(chip)).chip_init != 0 {
            azx_stop_all_streams(chip);
            azx_stop_chip(chip);
        }

        azx_free_stream_pages(chip);
        azx_free_streams(chip);
        snd_hdac_bus_exit(azx_bus(chip));
    }

    0
}

unsafe fn cix_ipbloq_hda_probe_codec(hda: *mut cix_ipbloq_hda) -> c_int {
    let chip = unsafe { &mut (*hda).chip as *mut azx };
    let bus = unsafe { azx_bus(chip) };
    let mut err: c_int;

    unsafe {
        (*to_hda_bus(bus)).bus_probing = 1;
    }

    /* create codec instances */
    err = unsafe { azx_probe_codecs(chip, 8) };
    if err < 0 {
        unsafe {
            dev_err((*hda).dev, c"probe codecs failed: %d\n".as_ptr(), err);
        }
        return err;
    }

    err = unsafe { azx_codec_configure(chip) };
    if err < 0 {
        unsafe {
            dev_err((*hda).dev, c"codec configure failed: %d\n".as_ptr(), err);
        }
        return err;
    }

    err = unsafe { snd_card_register((*chip).card) };
    if err < 0 {
        unsafe {
            dev_err((*hda).dev, c"card register failed: %d\n".as_ptr(), err);
        }
        return err;
    }

    unsafe {
        (*chip).running = 1;

        (*to_hda_bus(bus)).bus_probing = 0;

        snd_hda_set_power_save(&mut (*chip).bus, CIX_IPBLOQ_POWER_SAVE_DEFAULT_TIME_MS);
    }

    0
}

unsafe fn cix_ipbloq_hda_init(
    hda: *mut cix_ipbloq_hda,
    chip: *mut azx,
    pdev: *mut platform_device,
) -> c_int {
    let mut sname: *const c_char = ptr::null();
    let drv_name: *const c_char = c"cix-ipbloq-hda".as_ptr();
    let bus = unsafe { azx_bus(chip) };
    let card = unsafe { (*chip).card };
    let mut res: *mut resource = ptr::null_mut();
    let mut gcap: u16;
    let mut irq_id: c_int;
    let mut err: c_int;

    unsafe {
        (*hda).regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
        if IS_ERR((*hda).regs) {
            dev_err((*hda).dev, c"failed to get and ioremap resource\n".as_ptr());
            return PTR_ERR((*hda).regs);
        }
        (*bus).remap_addr = (*hda).regs;
        (*bus).addr = (*res).start;

        irq_id = platform_get_irq(pdev, 0);
        if irq_id < 0 {
            return irq_id;
        }

        err = devm_request_irq(
            (*hda).dev,
            irq_id,
            azx_interrupt,
            0,
            c"cix-ipbloq-hda".as_ptr(),
            chip as *mut c_void,
        );
        if err < 0 {
            return dev_err_probe(
                (*hda).dev,
                err,
                c"unable to request IRQ %d : err = %d\n".as_ptr(),
                irq_id,
                err,
            );
        }
        (*bus).irq = irq_id;
        (*card).sync_irq = (*bus).irq;

        gcap = azx_readw(chip, GCAP);
        (*chip).capture_streams = ((gcap >> 8) & 0x0f) as c_uint;
        (*chip).playback_streams = ((gcap >> 12) & 0x0f) as c_uint;
        (*chip).capture_index_offset = 0;
        (*chip).playback_index_offset = (*chip).capture_streams;
        (*chip).num_streams = (*chip).playback_streams + (*chip).capture_streams;

        /* initialize streams */
        err = azx_init_streams(chip);
        if err < 0 {
            dev_err((*hda).dev, c"failed to initialize streams: %d\n".as_ptr(), err);
            return err;
        }

        err = azx_alloc_stream_pages(chip);
        if err < 0 {
            dev_err((*hda).dev, c"failed to allocate stream pages: %d\n".as_ptr(), err);
            return err;
        }

        /* initialize chip */
        azx_init_chip(chip, 1);

        /* codec detection */
        if (*bus).codec_mask == 0 {
            dev_err((*hda).dev, c"no codecs found\n".as_ptr());
            return -ENODEV;
        }
        dev_dbg(
            (*card).dev,
            c"codec detection mask = 0x%lx\n".as_ptr(),
            (*bus).codec_mask,
        );

        /* driver name */
        strscpy(
            (*card).driver.as_mut_ptr(),
            drv_name,
            size_of_val(&(*card).driver),
        );

        /* shortname for card */
        sname = of_get_property((*pdev).dev.of_node, c"model".as_ptr(), ptr::null_mut());
        if sname.is_null() {
            sname = drv_name;
        }
        if strlen(sname) > size_of_val(&(*card).shortname) {
            dev_dbg((*card).dev, c"truncating shortname for card\n".as_ptr());
        }
        strscpy(
            (*card).shortname.as_mut_ptr(),
            sname,
            size_of_val(&(*card).shortname),
        );

        /* longname for card */
        snprintf(
            (*card).longname.as_mut_ptr(),
            size_of_val(&(*card).longname),
            c"%s at 0x%lx irq %i".as_ptr(),
            (*card).shortname.as_ptr(),
            (*bus).addr,
            (*bus).irq,
        );
    }

    0
}

unsafe fn cix_ipbloq_hda_create(
    hda: *mut cix_ipbloq_hda,
    card: *mut snd_card,
    driver_caps: c_uint,
) -> c_int {
    static OPS: snd_device_ops = snd_device_ops {
        dev_disconnect: Some(cix_ipbloq_hda_dev_disconnect),
        dev_free: Some(cix_ipbloq_hda_dev_free),
    };
    let chip: *mut azx;
    let mut err: c_int;

    unsafe {
        chip = &mut (*hda).chip;
        (*chip).card = card;
        (*chip).ops = &cix_ipbloq_hda_ops;
        (*chip).driver_caps = driver_caps;
        (*chip).driver_type = driver_caps & 0xff;
        (*chip).dev_index = 0;
        (*chip).single_cmd = 0;
        (*chip).codec_probe_mask = -1;
        (*chip).align_buffer_size = 1;
        (*chip).jackpoll_interval = msecs_to_jiffies(CIX_IPBLOQ_JACKPOLL_DEFAULT_TIME_MS);
        mutex_init(&mut (*chip).open_mutex);
        INIT_LIST_HEAD(&mut (*chip).pcm_list);

        /*
         * HD-audio controllers appear pretty inaccurate about the update-IRQ timing.
         * The IRQ is issued before actually the data is processed. So use stream
         * link position by default instead of dma position buffer.
         */
        (*chip).get_position[0] = azx_get_pos_lpib;
        (*chip).get_position[1] = azx_get_pos_lpib;

        err = azx_bus_init(chip, ptr::null());
        if err < 0 {
            dev_err((*hda).dev, c"failed to init bus, err = %d\n".as_ptr(), err);
            return err;
        }

        /* RIRBSTS.RINTFL cannot be cleared, cause interrupt storm */
        (*chip).bus.core.polling_mode = 1;
        (*chip).bus.core.not_use_interrupts = 1;

        (*chip).bus.core.aligned_mmio = 1;
        (*chip).bus.core.dma_stop_delay = 100;
        (*chip).bus.core.addr_offset = CIX_IPBLOQ_SKY1_ADDR_HOST_TO_HDAC_OFFSET as DmaAddrT;

        (*chip).bus.jackpoll_in_suspend = 1;

        err = snd_device_new(card, SNDRV_DEV_LOWLEVEL, chip as *mut c_void, &OPS);
        if err < 0 {
            dev_err((*card).dev, c"failed to create device, err = %d\n".as_ptr(), err);
            return err;
        }
    }

    0
}

unsafe extern "C" fn cix_ipbloq_hda_probe(pdev: *mut platform_device) -> c_int {
    let driver_flags: c_uint = AZX_DCAPS_PM_RUNTIME;
    let hda: *mut cix_ipbloq_hda;
    let mut card: *mut snd_card = ptr::null_mut();
    let chip: *mut azx;
    let mut err: c_int;

    unsafe {
        hda = devm_kzalloc(
            &mut (*pdev).dev,
            size_of::<cix_ipbloq_hda>(),
            GFP_KERNEL,
        ) as *mut cix_ipbloq_hda;
        if hda.is_null() {
            return -ENOMEM;
        }
        (*hda).dev = &mut (*pdev).dev;

        (*hda).reset = devm_reset_control_get((*hda).dev, ptr::null());
        if IS_ERR((*hda).reset as *const c_void) {
            return dev_err_probe(
                (*hda).dev,
                PTR_ERR((*hda).reset as *const c_void),
                c"failed to get reset, err = %ld\n".as_ptr(),
                PTR_ERR((*hda).reset as *const c_void) as c_long,
            );
        }

        (*hda).clocks[(*hda).nclocks as usize].id = c"ipg".as_ptr();
        (*hda).nclocks += 1;
        (*hda).clocks[(*hda).nclocks as usize].id = c"per".as_ptr();
        (*hda).nclocks += 1;
        err = devm_clk_bulk_get((*hda).dev, (*hda).nclocks, (*hda).clocks.as_mut_ptr());
        if err < 0 {
            return dev_err_probe((*hda).dev, err, c"failed to get clk, err = %d\n".as_ptr(), err);
        }

        dma_set_mask_and_coherent((*hda).dev, DMA_BIT_MASK(32));

        err = of_reserved_mem_device_init((*hda).dev);
        if err < 0 && err != -ENODEV {
            dev_err(
                (*hda).dev,
                c"failed to init reserved mem for DMA, err = %d\n".as_ptr(),
                err,
            );
            return err;
        }

        err = snd_card_new(
            (*hda).dev,
            SNDRV_DEFAULT_IDX1,
            SNDRV_DEFAULT_STR1,
            THIS_MODULE,
            0,
            &mut card,
        );
        if err < 0 {
            return dev_err_probe(
                (*hda).dev,
                err,
                c"failed to crate card, err = %d\n".as_ptr(),
                err,
            );
        }

        err = cix_ipbloq_hda_create(hda, card, driver_flags);
        if err < 0 {
            snd_card_free(card);
            return err;
        }

        chip = &mut (*hda).chip;
        (*card).private_data = chip as *mut c_void;
        dev_set_drvdata((*hda).dev, card as *mut c_void);

        pm_runtime_enable((*hda).dev);
        if !azx_has_pm_runtime(chip) {
            pm_runtime_forbid((*hda).dev);
        }

        err = pm_runtime_resume_and_get((*hda).dev);
        if err < 0 {
            dev_err(
                (*hda).dev,
                c"runtime resume and get failed, err = %d\n".as_ptr(),
                err,
            );
            snd_device_free(card, chip as *mut c_void);
            snd_card_free(card);
            return err;
        }

        err = cix_ipbloq_hda_init(hda, chip, pdev);
        if err < 0 {
            snd_device_free(card, chip as *mut c_void);
            snd_card_free(card);
            return err;
        }

        err = cix_ipbloq_hda_probe_codec(hda);
        if err < 0 {
            snd_device_free(card, chip as *mut c_void);
            snd_card_free(card);
            return err;
        }

        pm_runtime_put((*hda).dev);
    }

    0
}

unsafe extern "C" fn cix_ipbloq_hda_remove(pdev: *mut platform_device) {
    let card = unsafe { dev_get_drvdata(&mut (*pdev).dev) as *mut snd_card };
    let chip = unsafe { (*card).private_data as *mut azx };

    unsafe {
        snd_device_free(card, chip as *mut c_void);
        snd_card_free(card);

        pm_runtime_disable(&mut (*pdev).dev);
    }
}

unsafe extern "C" fn cix_ipbloq_hda_shutdown(pdev: *mut platform_device) {
    let card = unsafe { dev_get_drvdata(&mut (*pdev).dev) as *mut snd_card };
    let chip: *mut azx;

    if card.is_null() {
        return;
    }

    unsafe {
        chip = (*card).private_data as *mut azx;
        if !chip.is_null() && (*chip).running != 0 {
            azx_stop_chip(chip);
        }
    }
}

unsafe extern "C" fn cix_ipbloq_hda_suspend(dev: *mut device) -> c_int {
    let card = unsafe { dev_get_drvdata(dev) as *mut snd_card };
    let mut rc: c_int;

    unsafe {
        rc = pm_runtime_force_suspend(dev);
        if rc < 0 {
            return rc;
        }
        snd_power_change_state(card, SNDRV_CTL_POWER_D3COLD);
    }

    0
}

unsafe extern "C" fn cix_ipbloq_hda_resume(dev: *mut device) -> c_int {
    let card = unsafe { dev_get_drvdata(dev) as *mut snd_card };
    let mut rc: c_int;

    unsafe {
        rc = pm_runtime_force_resume(dev);
        if rc < 0 {
            return rc;
        }
        snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    }

    0
}

unsafe extern "C" fn cix_ipbloq_hda_runtime_suspend(dev: *mut device) -> c_int {
    let card = unsafe { dev_get_drvdata(dev) as *mut snd_card };
    let chip = unsafe { (*card).private_data as *mut azx };
    let hda = chip as *mut cix_ipbloq_hda;

    unsafe {
        if !chip.is_null() && (*chip).running != 0 {
            azx_stop_chip(chip);
            azx_enter_link_reset(chip);
        }

        clk_bulk_disable_unprepare((*hda).nclocks, (*hda).clocks.as_mut_ptr());
    }

    0
}

unsafe extern "C" fn cix_ipbloq_hda_runtime_resume(dev: *mut device) -> c_int {
    let card = unsafe { dev_get_drvdata(dev) as *mut snd_card };
    let chip = unsafe { (*card).private_data as *mut azx };
    let hda = chip as *mut cix_ipbloq_hda;
    let mut rc: c_int;

    unsafe {
        rc = clk_bulk_prepare_enable((*hda).nclocks, (*hda).clocks.as_mut_ptr());
        if rc != 0 {
            dev_err(dev, c"failed to enable clk bulk, rc: %d\n".as_ptr(), rc);
            return rc;
        }

        rc = reset_control_assert((*hda).reset);
        if rc != 0 {
            dev_err(dev, c"failed to assert reset, rc: %d\n".as_ptr(), rc);
            return rc;
        }

        rc = reset_control_deassert((*hda).reset);
        if rc != 0 {
            dev_err(dev, c"failed to deassert reset, rc: %d\n".as_ptr(), rc);
            return rc;
        }

        if !chip.is_null() && (*chip).running != 0 {
            azx_init_chip(chip, 1);
        }
    }

    0
}

// C source:
// static const struct dev_pm_ops cix_ipbloq_hda_pm = {
//      SYSTEM_SLEEP_PM_OPS(cix_ipbloq_hda_suspend, cix_ipbloq_hda_resume)
//      RUNTIME_PM_OPS(cix_ipbloq_hda_runtime_suspend,
//                     cix_ipbloq_hda_runtime_resume, NULL)
// };
static CIX_IPBLOQ_HDA_PM: dev_pm_ops = dev_pm_ops { _private: [] };

static CIX_IPBLOQ_HDA_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: c"cix,sky1-ipbloq-hda".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, cix_ipbloq_hda_match);

static mut CIX_IPBLOQ_HDA_DRIVER: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: c"cix-ipbloq-hda".as_ptr(),
        pm: &CIX_IPBLOQ_HDA_PM,
        of_match_table: CIX_IPBLOQ_HDA_MATCH.as_ptr(),
    },
    probe: Some(cix_ipbloq_hda_probe),
    remove: Some(cix_ipbloq_hda_remove),
    shutdown: Some(cix_ipbloq_hda_shutdown),
};
// module_platform_driver(cix_ipbloq_hda_driver);

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("CIX IPBLOQ HDA bus driver");
// MODULE_AUTHOR("Joakim Zhang <joakim.zhang@cixtech.com>");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
