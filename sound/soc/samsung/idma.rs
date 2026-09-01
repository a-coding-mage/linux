// SPDX-License-Identifier: GPL-2.0+
//
// idma.c - I2S0 internal DMA driver
//
// Copyright (c) 2011 Samsung Electronics Co., Ltd.
//		http://www.samsung.com

// Dependencies in the original C source:
// <linux/interrupt.h>, <linux/platform_device.h>, <linux/dma-mapping.h>,
// <linux/slab.h>, <linux/module.h>, <sound/pcm.h>, <sound/pcm_params.h>,
// <sound/soc.h>, "i2s.h", "idma.h", "i2s-regs.h".

const ST_RUNNING: c_int = 1 << 0;
const ST_OPENED: c_int = 1 << 1;

static idma_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME,
    buffer_bytes_max: MAX_IDMA_BUFFER,
    period_bytes_min: 128,
    period_bytes_max: MAX_IDMA_PERIOD,
    periods_min: 1,
    periods_max: 2,
};

#[repr(C)]
struct idma_ctrl {
    lock: spinlock_t,
    state: c_int,
    start: dma_addr_t,
    pos: dma_addr_t,
    end: dma_addr_t,
    period: dma_addr_t,
    periodsz: dma_addr_t,
    token: *mut c_void,
    cb: Option<unsafe extern "C" fn(dt: *mut c_void, bytes_xfer: c_int)>,
}

#[repr(C)]
struct idma_info {
    lock: spinlock_t,
    regs: *mut c_void,
    lp_tx_addr: dma_addr_t,
}

static mut idma: idma_info = idma_info {
    lock: unsafe { core::mem::zeroed() },
    regs: core::ptr::null_mut(),
    lp_tx_addr: 0,
};

static mut idma_irq: c_int = 0;

unsafe fn idma_getpos(src: *mut dma_addr_t) {
    *src = idma.lp_tx_addr
        + ((readl(idma.regs.byte_add(I2STRNCNT)) & 0xffffff) as dma_addr_t) * 4;
}

unsafe fn idma_enqueue(substream: *mut snd_pcm_substream) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let prtd: *mut idma_ctrl = (*(*substream).runtime).private_data as *mut idma_ctrl;
    let mut val: u32;

    {
        let _guard = scoped_guard_spinlock(&mut (*prtd).lock);
        (*prtd).token = substream as *mut c_void;
    }

    /* Internal DMA Level0 Interrupt Address */
    val = (idma.lp_tx_addr + (*prtd).periodsz) as u32;
    writel(val, idma.regs.byte_add(I2SLVL0ADDR));

    /* Start address0 of I2S internal DMA operation. */
    val = idma.lp_tx_addr as u32;
    writel(val, idma.regs.byte_add(I2SSTR0));

    /*
     * Transfer block size for I2S internal DMA.
     * Should decide transfer size before start dma operation
     */
    val = readl(idma.regs.byte_add(I2SSIZE));
    val &= !(I2SSIZE_TRNMSK << I2SSIZE_SHIFT);
    val |= ((((*runtime).dma_bytes >> 2) as u32) & I2SSIZE_TRNMSK) << I2SSIZE_SHIFT;
    writel(val, idma.regs.byte_add(I2SSIZE));

    val = readl(idma.regs.byte_add(I2SAHB));
    val |= AHB_INTENLVL0;
    writel(val, idma.regs.byte_add(I2SAHB));

    0
}

unsafe fn idma_setcallbk(
    substream: *mut snd_pcm_substream,
    cb: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
) {
    let prtd: *mut idma_ctrl = (*(*substream).runtime).private_data as *mut idma_ctrl;

    let _guard = guard_spinlock(&mut (*prtd).lock);
    (*prtd).cb = cb;
}

unsafe fn idma_control(op: c_int) {
    let mut val: u32 = readl(idma.regs.byte_add(I2SAHB));

    let _guard = guard_spinlock(&mut idma.lock);

    match op {
        LPAM_DMA_START => {
            val |= AHB_INTENLVL0 | AHB_DMAEN;
        }
        LPAM_DMA_STOP => {
            val &= !(AHB_INTENLVL0 | AHB_DMAEN);
        }
        _ => {
            return;
        }
    }

    writel(val, idma.regs.byte_add(I2SAHB));
}

unsafe extern "C" fn idma_done(id: *mut c_void, bytes_xfer: c_int) {
    let substream: *mut snd_pcm_substream = id as *mut snd_pcm_substream;
    let prtd: *mut idma_ctrl = (*(*substream).runtime).private_data as *mut idma_ctrl;

    if !prtd.is_null() && ((*prtd).state & ST_RUNNING) != 0 {
        snd_pcm_period_elapsed(substream);
    }
}

unsafe extern "C" fn idma_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let prtd: *mut idma_ctrl = (*(*substream).runtime).private_data as *mut idma_ctrl;
    let mut mod_: u32 = readl(idma.regs.byte_add(I2SMOD));
    let mut ahb: u32 = readl(idma.regs.byte_add(I2SAHB));

    ahb |= AHB_DMARLD | AHB_INTMASK;
    mod_ |= MOD_TXS_IDMA;
    writel(ahb, idma.regs.byte_add(I2SAHB));
    writel(mod_, idma.regs.byte_add(I2SMOD));

    snd_pcm_set_runtime_buffer(substream, &mut (*substream).dma_buffer);
    (*runtime).dma_bytes = params_buffer_bytes(params);

    (*prtd).start = (*runtime).dma_addr;
    (*prtd).pos = (*prtd).start;
    (*prtd).period = params_periods(params) as dma_addr_t;
    (*prtd).periodsz = params_period_bytes(params) as dma_addr_t;
    (*prtd).end = (*runtime).dma_addr + (*runtime).dma_bytes as dma_addr_t;

    idma_setcallbk(substream, Some(idma_done));

    0
}

unsafe extern "C" fn idma_hw_free(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    snd_pcm_set_runtime_buffer(substream, core::ptr::null_mut());

    0
}

unsafe extern "C" fn idma_prepare(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let prtd: *mut idma_ctrl = (*(*substream).runtime).private_data as *mut idma_ctrl;

    (*prtd).pos = (*prtd).start;

    /* flush the DMA channel */
    idma_control(LPAM_DMA_STOP);
    idma_enqueue(substream);

    0
}

unsafe extern "C" fn idma_trigger(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let prtd: *mut idma_ctrl = (*(*substream).runtime).private_data as *mut idma_ctrl;
    let mut ret: c_int = 0;

    let _guard = guard_spinlock(&mut (*prtd).lock);

    match cmd {
        SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            (*prtd).state |= ST_RUNNING;
            idma_control(LPAM_DMA_START);
        }

        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            (*prtd).state &= !ST_RUNNING;
            idma_control(LPAM_DMA_STOP);
        }

        _ => {
            ret = -EINVAL;
        }
    }

    ret
}

unsafe extern "C" fn idma_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let prtd: *mut idma_ctrl = (*runtime).private_data as *mut idma_ctrl;
    let mut src: dma_addr_t = 0;
    let res: c_ulong;

    {
        let _guard = scoped_guard_spinlock(&mut (*prtd).lock);
        idma_getpos(&mut src);
        res = (src - (*prtd).start) as c_ulong;
    }

    bytes_to_frames((*substream).runtime, res)
}

unsafe extern "C" fn idma_mmap(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    vma: *mut vm_area_struct,
) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let size: c_ulong;
    let offset: c_ulong;

    /* From snd_pcm_lib_mmap_iomem */
    (*vma).vm_page_prot = pgprot_noncached((*vma).vm_page_prot);
    size = (*vma).vm_end - (*vma).vm_start;
    offset = (*vma).vm_pgoff << PAGE_SHIFT;
    io_remap_pfn_range(
        vma,
        (*vma).vm_start,
        (((*runtime).dma_addr + offset as dma_addr_t) >> PAGE_SHIFT) as c_ulong,
        size,
        (*vma).vm_page_prot,
    )
}

unsafe extern "C" fn iis_irq(irqno: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let prtd: *mut idma_ctrl = dev_id as *mut idma_ctrl;
    let mut iisahb: u32;
    let mut val: u32;
    let mut addr: u32;

    iisahb = readl(idma.regs.byte_add(I2SAHB));

    val = if (iisahb & AHB_LVL0INT) != 0 {
        AHB_CLRLVL0INT
    } else {
        0
    };

    if val != 0 {
        iisahb |= val;
        writel(iisahb, idma.regs.byte_add(I2SAHB));

        addr = readl(idma.regs.byte_add(I2SLVL0ADDR)) - idma.lp_tx_addr as u32;
        addr = addr.wrapping_add((*prtd).periodsz as u32);
        addr %= ((*prtd).end - (*prtd).start) as u32;
        addr = addr.wrapping_add(idma.lp_tx_addr as u32);

        writel(addr, idma.regs.byte_add(I2SLVL0ADDR));

        if let Some(cb) = (*prtd).cb {
            cb((*prtd).token, (*prtd).period as c_int);
        }
    }

    IRQ_HANDLED
}

unsafe extern "C" fn idma_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let prtd: *mut idma_ctrl;
    let ret: c_int;

    snd_soc_set_runtime_hwparams(substream, &idma_hardware);

    prtd = kzalloc_obj_idma_ctrl();
    if prtd.is_null() {
        return -ENOMEM;
    }

    ret = request_irq(idma_irq, Some(iis_irq), 0, c_str!("i2s"), prtd as *mut c_void);
    if ret < 0 {
        pr_err(c_str!("fail to claim i2s irq , ret = %d\n"), ret);
        kfree(prtd as *mut c_void);
        return ret;
    }

    spin_lock_init(&mut (*prtd).lock);

    (*runtime).private_data = prtd as *mut c_void;

    0
}

unsafe extern "C" fn idma_close(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let prtd: *mut idma_ctrl = (*runtime).private_data as *mut idma_ctrl;

    free_irq(idma_irq, prtd as *mut c_void);

    if prtd.is_null() {
        pr_err(c_str!("idma_close called with prtd == NULL\n"));
    }

    kfree(prtd as *mut c_void);

    0
}

unsafe extern "C" fn idma_free(component: *mut snd_soc_component, pcm: *mut snd_pcm) {
    let substream: *mut snd_pcm_substream;
    let buf: *mut snd_dma_buffer;

    substream = (*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream;
    if substream.is_null() {
        return;
    }

    buf = &mut (*substream).dma_buffer;
    if (*buf).area.is_null() {
        return;
    }

    iounmap((*buf).area as *mut c_void);

    (*buf).area = core::ptr::null_mut();
    (*buf).addr = 0;
}

unsafe fn preallocate_idma_buffer(pcm: *mut snd_pcm, stream: c_int) -> c_int {
    let substream: *mut snd_pcm_substream = (*pcm).streams[stream as usize].substream;
    let buf: *mut snd_dma_buffer = &mut (*substream).dma_buffer;

    (*buf).dev.dev = (*(*(*pcm).card).dev).dev;
    (*buf).private_data = core::ptr::null_mut();

    /* Assign PCM buffer pointers */
    (*buf).dev.type_ = SNDRV_DMA_TYPE_CONTINUOUS;
    (*buf).addr = idma.lp_tx_addr;
    (*buf).bytes = idma_hardware.buffer_bytes_max;
    (*buf).area = ioremap((*buf).addr, (*buf).bytes) as *mut c_uchar;
    if (*buf).area.is_null() {
        return -ENOMEM;
    }

    0
}

unsafe extern "C" fn idma_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let card: *mut snd_card = (*(*rtd).card).snd_card;
    let pcm: *mut snd_pcm = (*rtd).pcm;
    let mut ret: c_int;

    ret = dma_coerce_mask_and_coherent((*card).dev, DMA_BIT_MASK(32));
    if ret != 0 {
        return ret;
    }

    if !(*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize]
        .substream
        .is_null()
    {
        ret = preallocate_idma_buffer(pcm, SNDRV_PCM_STREAM_PLAYBACK);
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn idma_reg_addr_init(regs: *mut c_void, addr: dma_addr_t) {
    spin_lock_init(&mut idma.lock);
    idma.regs = regs;
    idma.lp_tx_addr = addr;
}
// EXPORT_SYMBOL_GPL(idma_reg_addr_init);

static asoc_idma_platform: snd_soc_component_driver = snd_soc_component_driver {
    open: Some(idma_open),
    close: Some(idma_close),
    trigger: Some(idma_trigger),
    pointer: Some(idma_pointer),
    mmap: Some(idma_mmap),
    hw_params: Some(idma_hw_params),
    hw_free: Some(idma_hw_free),
    prepare: Some(idma_prepare),
    pcm_new: Some(idma_new),
    pcm_free: Some(idma_free),
};

unsafe extern "C" fn asoc_idma_platform_probe(pdev: *mut platform_device) -> c_int {
    idma_irq = platform_get_irq(pdev, 0);
    if idma_irq < 0 {
        return idma_irq;
    }

    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &asoc_idma_platform,
        core::ptr::null(),
        0,
    )
}

static mut asoc_idma_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c_str!("samsung-idma"),
    },

    probe: Some(asoc_idma_platform_probe),
};

module_platform_driver!(asoc_idma_driver);

MODULE_AUTHOR!("Jaswinder Singh, <jassisinghbrar@gmail.com>");
MODULE_DESCRIPTION!("Samsung ASoC IDMA Driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
