// SPDX-License-Identifier: GPL-2.0-only
/*
 * DMA Router driver for STM32 DMA MUX
 *
 * Based on TI DMA Crossbar driver
 */

// Linux kernel dependencies supplied by other translation units.

const STM32_DMAMUX_MAX_DMA_REQUESTS: usize = 32;
const STM32_DMAMUX_MAX_REQUESTS: u32 = 255;

#[inline]
const fn stm32_dmamux_ccr(x: u32) -> u32 { 0x4u32.wrapping_mul(x) }

#[repr(C)]
pub struct stm32_dmamux {
    pub master: u32,
    pub request: u32,
    pub chan_id: u32,
}

#[repr(C)]
pub struct stm32_dmamux_data {
    pub dmarouter: dma_router,
    pub clk: *mut clk,
    pub iomem: *mut core::ffi::c_void,
    pub dma_requests: u32,
    pub dmamux_requests: u32,
    pub lock: spinlock_t,
    pub dma_inuse: [u8; STM32_DMAMUX_MAX_DMA_REQUESTS],
    pub ccr: [u32; STM32_DMAMUX_MAX_DMA_REQUESTS],
    // Flexible array member: number of DMA requests per DMA master.
    pub dma_reqs: [u32; 0],
}

#[inline]
unsafe fn stm32_dmamux_read(iomem: *mut core::ffi::c_void, reg: u32) -> u32 {
    readl_relaxed(iomem.add(reg as usize))
}

#[inline]
unsafe fn stm32_dmamux_write(iomem: *mut core::ffi::c_void, reg: u32, val: u32) {
    writel_relaxed(val, iomem.add(reg as usize));
}

unsafe fn stm32_dmamux_free(dev: *mut device, route_data: *mut core::ffi::c_void) {
    let dmamux = dev_get_drvdata(dev) as *mut stm32_dmamux_data;
    let mux = route_data as *mut stm32_dmamux;
    let mut flags: ulong = 0;
    spin_lock_irqsave(&mut (*dmamux).lock, &mut flags);
    stm32_dmamux_write((*dmamux).iomem, stm32_dmamux_ccr((*mux).chan_id), 0);
    clear_bit((*mux).chan_id as usize, (*dmamux).dma_inuse.as_mut_ptr());
    pm_runtime_put_sync(dev);
    spin_unlock_irqrestore(&mut (*dmamux).lock, flags);
    dev_dbg(dev, "Unmapping DMAMUX(%u) to DMA%u(%u)\n", (*mux).request, (*mux).master, (*mux).chan_id);
    kfree(mux as *mut core::ffi::c_void);
}

unsafe fn stm32_dmamux_route_allocate(dma_spec: *mut of_phandle_args, ofdma: *mut of_dma) -> *mut core::ffi::c_void {
    let pdev = of_find_device_by_node((*ofdma).of_node);
    let dmamux = platform_get_drvdata(pdev) as *mut stm32_dmamux_data;
    let mut mux: *mut stm32_dmamux = core::ptr::null_mut();
    let mut ret: i32 = -22;
    let mut flags: ulong = 0;
    if (*dma_spec).args_count != 3 { dev_err(&mut (*pdev).dev, "invalid number of dma mux args\n"); put_device(&mut (*pdev).dev); return err_ptr(ret); }
    if (*dma_spec).args[0] > (*dmamux).dmamux_requests { dev_err(&mut (*pdev).dev, "invalid mux request number: %d\n", (*dma_spec).args[0]); put_device(&mut (*pdev).dev); return err_ptr(ret); }
    mux = kzalloc(core::mem::size_of::<stm32_dmamux>()) as *mut stm32_dmamux;
    if mux.is_null() { ret = -12; put_device(&mut (*pdev).dev); return err_ptr(ret); }
    spin_lock_irqsave(&mut (*dmamux).lock, &mut flags);
    (*mux).chan_id = find_first_zero_bit((*dmamux).dma_inuse.as_mut_ptr(), (*dmamux).dma_requests as usize) as u32;
    if (*mux).chan_id == (*dmamux).dma_requests { spin_unlock_irqrestore(&mut (*dmamux).lock, flags); dev_err(&mut (*pdev).dev, "Run out of free DMA requests\n"); kfree(mux as *mut core::ffi::c_void); put_device(&mut (*pdev).dev); return err_ptr(-12); }
    set_bit((*mux).chan_id as usize, (*dmamux).dma_inuse.as_mut_ptr());
    spin_unlock_irqrestore(&mut (*dmamux).lock, flags);
    let mut i = 1u32; let mut min = 0u32; let mut max = dma_req_at(dmamux, i);
    while i <= dma_req_at(dmamux, 0) { if (*mux).chan_id < max { break; } min += dma_req_at(dmamux, i); i += 1; max += dma_req_at(dmamux, i); }
    (*mux).master = i - 1;
    (*dma_spec).np = of_parse_phandle((*ofdma).of_node, "dma-masters", (i - 1) as i32);
    if (*dma_spec).np.is_null() { dev_err(&mut (*pdev).dev, "can't get dma master\n"); clear_bit((*mux).chan_id as usize, (*dmamux).dma_inuse.as_mut_ptr()); kfree(mux as *mut core::ffi::c_void); put_device(&mut (*pdev).dev); return err_ptr(-22); }
    spin_lock_irqsave(&mut (*dmamux).lock, &mut flags); ret = pm_runtime_resume_and_get(&mut (*pdev).dev); if ret < 0 { spin_unlock_irqrestore(&mut (*dmamux).lock, flags); of_node_put((*dma_spec).np); clear_bit((*mux).chan_id as usize, (*dmamux).dma_inuse.as_mut_ptr()); kfree(mux as *mut core::ffi::c_void); put_device(&mut (*pdev).dev); return err_ptr(ret); } spin_unlock_irqrestore(&mut (*dmamux).lock, flags);
    (*mux).request = (*dma_spec).args[0];
    (*dma_spec).args[3] = (*dma_spec).args[2] | ((*mux).chan_id << 16); (*dma_spec).args[2] = (*dma_spec).args[1]; (*dma_spec).args[1] = 0; (*dma_spec).args[0] = (*mux).chan_id - min; (*dma_spec).args_count = 4;
    stm32_dmamux_write((*dmamux).iomem, stm32_dmamux_ccr((*mux).chan_id), (*mux).request);
    dev_dbg(&mut (*pdev).dev, "Mapping DMAMUX(%u) to DMA%u(%u)\n", (*mux).request, (*mux).master, (*mux).chan_id); put_device(&mut (*pdev).dev); mux as *mut core::ffi::c_void
}

#[repr(C)]
struct of_device_id { compatible: *const u8 }

#[repr(C)]
struct dev_pm_ops {
    suspend: Option<unsafe extern "C" fn(*mut device) -> i32>,
    resume: Option<unsafe extern "C" fn(*mut device) -> i32>,
    runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> i32>,
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> i32>,
}

static STM32_STM32DMA_MASTER_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"st,stm32-dma\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

static STM32_DMAMUX_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"st,stm32h7-dmamux\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

// CONFIG_PM and CONFIG_PM_SLEEP conditionally include the following callbacks.
extern "C" {
    fn stm32_dmamux_probe(pdev: *mut platform_device) -> i32;
    fn stm32_dmamux_runtime_suspend(dev: *mut device) -> i32;
    fn stm32_dmamux_runtime_resume(dev: *mut device) -> i32;
    fn stm32_dmamux_suspend(dev: *mut device) -> i32;
    fn stm32_dmamux_resume(dev: *mut device) -> i32;
    fn stm32_dmamux_init() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
