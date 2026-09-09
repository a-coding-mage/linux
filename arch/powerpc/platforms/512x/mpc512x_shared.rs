// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2007,2008 Freescale Semiconductor, Inc. All rights reserved.
 *
 * Author: John Rigby <jrigby@freescale.com>
 *
 * Description:
 * MPC512x Shared code
 */

// C kernel dependencies are supplied by the surrounding translation unit.

static mut reset_module_base: *mut mpc512x_reset_module = core::ptr::null_mut();

pub unsafe extern "C" fn mpc512x_restart(_cmd: *mut core::ffi::c_char) -> ! {
    if !reset_module_base.is_null() {
        out_be32(&mut (*reset_module_base).rpr, 0x52535445);
        out_be32(&mut (*reset_module_base).rcr, 0x2);
    } else {
        pr_err(c"Restart module not mapped.\n");
    }
    loop {}
}

#[repr(C, align(8))]
struct fsl_diu_shared_fb {
    gamma: [u8; 0x300],
    ad0: diu_ad,
    fb_phys: phys_addr_t,
    fb_len: usize,
    in_use: bool,
}

// Receives a pixel clock spec in pico seconds, adjusts the DIU clock rate.
unsafe fn mpc512x_set_pixel_clock(mut pixclock: u32) {
    let np: *mut device_node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"fsl,mpc5121-diu".as_ptr());
    if np.is_null() { pr_err(c"Could not find DIU device tree node.\n"); return; }
    let mut clk_diu = of_clk_get(np, 0);
    if is_err(clk_diu) { clk_diu = clk_get_sys((*np).name, c"ipg".as_ptr()); }
    of_node_put(np);
    if is_err(clk_diu) { pr_err(c"Could not lookup DIU clock.\n"); return; }
    if clk_prepare_enable(clk_diu) != 0 { pr_err(c"Could not enable DIU clock.\n"); return; }

    pr_debug(c"DIU pixclock in ps - %u\n", pixclock);
    pixclock = (1_000_000_000u32 / pixclock) * 1000;
    pr_debug(c"DIU pixclock freq  - %u\n", pixclock);
    let epsilon = pixclock / 20;
    let minpixclock = pixclock - epsilon;
    let maxpixclock = pixclock + epsilon;
    pr_debug(c"DIU deviation      - %lu\n", epsilon as usize);
    pr_debug(c"DIU minpixclock    - %lu\n", minpixclock as usize);
    pr_debug(c"DIU maxpixclock    - %lu\n", maxpixclock as usize);

    let mut offset = 0u32;
    let mut want = 0u32;
    let mut got = 0usize;
    let mut delta = 0usize;
    while offset <= epsilon {
        want = pixclock - offset;
        pr_debug(c"DIU checking clock - %lu\n", want as usize);
        clk_set_rate(clk_diu, want as usize);
        got = clk_get_rate(clk_diu);
        delta = (pixclock as usize).abs_diff(got);
        if delta < epsilon as usize { break; }
        if offset != 0 {
            want = pixclock + offset;
            pr_debug(c"DIU checking clock - %lu\n", want as usize);
            clk_set_rate(clk_diu, want as usize);
            got = clk_get_rate(clk_diu);
            delta = (pixclock as usize).abs_diff(got);
            if delta < epsilon as usize { break; }
        }
        offset += pixclock / 64;
    }
    if offset <= epsilon {
        pr_debug(c"DIU clock accepted - %lu\n", want as usize);
        pr_debug(c"DIU pixclock want %u, got %lu, delta %lu, eps %lu\n", pixclock, got, delta, epsilon as usize);
        return;
    }
    pr_warn(c"DIU pixclock auto search unsuccessful\n");
    pr_warn(c"DIU pixclock best effort fallback (backend's choice)\n");
    clk_set_rate(clk_diu, pixclock as usize);
    got = clk_get_rate(clk_diu);
    delta = (pixclock as usize).abs_diff(got);
    pr_debug(c"DIU pixclock want %u, got %lu, delta %lu, eps %lu\n", pixclock, got, delta, epsilon as usize);
}

unsafe fn mpc512x_valid_monitor_port(_port: fsl_diu_monitor_port) -> fsl_diu_monitor_port { FSL_DIU_PORT_DVI }

static mut diu_shared_fb: fsl_diu_shared_fb = fsl_diu_shared_fb { gamma: [0; 0x300], ad0: diu_ad::default(), fb_phys: 0, fb_len: 0, in_use: false };

unsafe fn mpc512x_free_bootmem(page: *mut page) {
    BUG_ON(PageTail(page));
    BUG_ON(page_ref_count(page) > 1);
    free_reserved_page(page);
}

unsafe fn mpc512x_release_bootmem() {
    let addr = diu_shared_fb.fb_phys & PAGE_MASK;
    let size = diu_shared_fb.fb_len;
    if diu_shared_fb.in_use {
        let mut start = PFN_UP(addr);
        let end = PFN_DOWN(addr + size);
        while start < end { mpc512x_free_bootmem(pfn_to_page(start)); start += 1; }
        diu_shared_fb.in_use = false;
    }
    (*diu_ops).release_bootmem = None;
}

unsafe fn mpc512x_init_diu() {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"fsl,mpc5121-diu".as_ptr());
    if np.is_null() { pr_err(c"No DIU node\n"); return; }
    let diu_reg = of_iomap(np, 0) as *mut diu;
    of_node_put(np);
    if diu_reg.is_null() { pr_err(c"Can't map DIU\n"); return; }
    let mode = in_be32(&(*diu_reg).diu_mode);
    if mode == MFB_MODE0 { pr_info(c"%s: DIU OFF\n", c"mpc512x_init_diu".as_ptr()); iounmap(diu_reg as *mut _); return; }
    let mut desc = in_be32(&(*diu_reg).desc[0]) as phys_addr_t;
    let mut vaddr = ioremap(desc, core::mem::size_of::<diu_ad>());
    if vaddr.is_null() { pr_err(c"Can't map DIU area desc.\n"); iounmap(diu_reg as *mut _); return; }
    core::ptr::copy_nonoverlapping(vaddr as *const diu_ad, &mut diu_shared_fb.ad0, 1);
    flush_dcache_range((&diu_shared_fb.ad0 as *const _ as usize), (&diu_shared_fb.ad0 as *const _ as usize) + core::mem::size_of::<diu_ad>() - 1);
    let res = in_be32(&(*diu_reg).disp_size);
    let pix_fmt = in_le32(vaddr as *const u32);
    let bpp = ((pix_fmt >> 16) & 3) + 1;
    diu_shared_fb.fb_phys = in_le32(vaddr.add(4) as *const u32) as phys_addr_t;
    diu_shared_fb.fb_len = (((res & 0xfff0000) >> 16) as usize) * ((res & 0xfff) as usize) * (bpp as usize);
    diu_shared_fb.in_use = true;
    iounmap(vaddr);
    desc = in_be32(&(*diu_reg).gamma) as phys_addr_t;
    vaddr = ioremap(desc, diu_shared_fb.gamma.len());
    if vaddr.is_null() { pr_err(c"Can't map DIU area desc.\n"); diu_shared_fb.in_use = false; iounmap(diu_reg as *mut _); return; }
    core::ptr::copy_nonoverlapping(vaddr, diu_shared_fb.gamma.as_mut_ptr(), diu_shared_fb.gamma.len());
    flush_dcache_range(diu_shared_fb.gamma.as_ptr() as usize, diu_shared_fb.gamma.as_ptr() as usize + diu_shared_fb.gamma.len() - 1);
    iounmap(vaddr);
    out_be32(&mut (*diu_reg).gamma, virt_to_phys(diu_shared_fb.gamma.as_ptr()));
    out_be32(&mut (*diu_reg).desc[1], 0); out_be32(&mut (*diu_reg).desc[2], 0);
    out_be32(&mut (*diu_reg).desc[0], virt_to_phys(&diu_shared_fb.ad0));
    iounmap(diu_reg as *mut _);
}

unsafe fn mpc512x_setup_diu() {
    if diu_shared_fb.in_use && memblock_reserve(diu_shared_fb.fb_phys, diu_shared_fb.fb_len) != 0 { pr_err(c"%s: reserve bootmem failed\n", c"mpc512x_setup_diu".as_ptr()); diu_shared_fb.in_use = false; }
    (*diu_ops).set_pixel_clock = Some(mpc512x_set_pixel_clock);
    (*diu_ops).valid_monitor_port = Some(mpc512x_valid_monitor_port);
    (*diu_ops).release_bootmem = Some(mpc512x_release_bootmem);
}

pub unsafe extern "C" fn mpc512x_init_IRQ() {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"fsl,mpc5121-ipic".as_ptr());
    if !np.is_null() { ipic_init(np, 0); of_node_put(np); ipic_set_default_priority(); }
}

const DEFAULT_FIFO_SIZE: u32 = 16;

pub unsafe extern "C" fn mpc512x_select_psc_compat() -> *const core::ffi::c_char {
    if of_machine_is_compatible(c"fsl,mpc5121".as_ptr()) { return c"fsl,mpc5121-psc".as_ptr(); }
    if of_machine_is_compatible(c"fsl,mpc5125".as_ptr()) { return c"fsl,mpc5125-psc".as_ptr(); }
    core::ptr::null()
}

unsafe fn mpc512x_select_reset_compat() -> *const core::ffi::c_char {
    if of_machine_is_compatible(c"fsl,mpc5121".as_ptr()) { return c"fsl,mpc5121-reset".as_ptr(); }
    if of_machine_is_compatible(c"fsl,mpc5125".as_ptr()) { return c"fsl,mpc5125-reset".as_ptr(); }
    core::ptr::null()
}

unsafe fn get_fifo_size(np: *mut device_node, prop_name: *const core::ffi::c_char) -> u32 {
    let fp = of_get_property(np, prop_name, core::ptr::null_mut());
    if !fp.is_null() { return *fp; }
    pr_warn(c"no fifo property in node, defaulting to %d\n", DEFAULT_FIFO_SIZE);
    DEFAULT_FIFO_SIZE
}

unsafe fn mpc512x_psc_fifo_init() {
    let psc_compat = mpc512x_select_psc_compat();
    if psc_compat.is_null() { pr_err(c"%s: no compatible devices found\n", c"mpc512x_psc_fifo_init".as_ptr()); return; }
    let mut fifobase = 0u32;
    for_each_compatible_node!(np, psc_compat) {
        let mut tx = get_fifo_size(np, c"fsl,tx-fifo-size".as_ptr()) / 4;
        let mut rx = get_fifo_size(np, c"fsl,rx-fifo-size".as_ptr()) / 4;
        if tx == 0 { tx = 1; } if rx == 0 { rx = 1; }
        let psc = of_iomap(np, 0);
        if psc.is_null() { pr_err(c"%s: Can't map device\n", c"mpc512x_psc_fifo_init".as_ptr()); continue; }
        if fifobase + tx + rx > 0x1000 { pr_err(c"%s: no fifo space available\n", c"mpc512x_psc_fifo_init".as_ptr()); iounmap(psc); continue; }
        let fifo = (psc as usize + core::mem::size_of::<mpc52xx_psc>()) as *mut mpc512x_psc_fifo;
        out_be32(&mut (*fifo).txsz, (fifobase << 16) | tx); fifobase += tx;
        out_be32(&mut (*fifo).rxsz, (fifobase << 16) | rx); fifobase += rx;
        out_be32(&mut (*fifo).txcmd, 0x80); out_be32(&mut (*fifo).txcmd, 1);
        out_be32(&mut (*fifo).rxcmd, 0x80); out_be32(&mut (*fifo).rxcmd, 1); iounmap(psc);
    }
}

unsafe fn mpc512x_declare_of_platform_devices() { if of_platform_bus_probe(core::ptr::null_mut(), core::ptr::null(), core::ptr::null_mut()) != 0 { printk(c"Error while probing of_platform bus\n".as_ptr()); } }
unsafe fn mpc512x_restart_init() { let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), mpc512x_select_reset_compat()); if !np.is_null() { reset_module_base = of_iomap(np, 0) as *mut _; of_node_put(np); } }

pub unsafe extern "C" fn mpc512x_init_early() { mpc512x_restart_init(); if IS_ENABLED_CONFIG_FB_FSL_DIU { mpc512x_init_diu(); } }
pub unsafe extern "C" fn mpc512x_init() { mpc5121_clk_init(); mpc512x_declare_of_platform_devices(); mpc512x_psc_fifo_init(); }
pub unsafe extern "C" fn mpc512x_setup_arch() { if IS_ENABLED_CONFIG_FB_FSL_DIU { mpc512x_setup_diu(); } }

pub unsafe extern "C" fn mpc512x_cs_config(cs: u32, val: u32) -> i32 {
    static mut lpc: *mut mpc512x_lpc = core::ptr::null_mut();
    if cs > 7 { return -EINVAL; }
    if lpc.is_null() { let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"fsl,mpc5121-lpc".as_ptr()); lpc = of_iomap(np, 0) as *mut mpc512x_lpc; of_node_put(np); if lpc.is_null() { return -ENOMEM; } }
    out_be32(&mut (*lpc).cs_cfg[cs as usize], val); 0
}

// External kernel types, constants, helpers, and the PSC/device-tree routines are supplied by dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
