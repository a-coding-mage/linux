// SPDX-License-Identifier: GPL-2.0-only
// External Linux/DRM declarations supplied by the surrounding kernel Rust bindings.

const ADP_INT_STATUS: usize = 0x34;
const ADP_INT_STATUS_INT_MASK: u32 = 0x7;
const ADP_INT_STATUS_VBLANK: u32 = 0x1;
const ADP_CTRL: usize = 0x100;
const ADP_CTRL_VBLANK_ON: u32 = 0x12;
const ADP_CTRL_FIFO_ON: u32 = 0x601;
const ADP_SCREEN_SIZE: usize = 0x0c;
const ADP_SCREEN_HSIZE: u32 = 0xffff;
const ADP_SCREEN_VSIZE: u32 = 0xffff0000;
const ADBE_FIFO: usize = 0x10c0;
const ADBE_FIFO_SYNC: u32 = 0xc0000000;
const ADBE_BLEND_BYPASS: usize = 0x2020;
const ADBE_BLEND_EN1: usize = 0x2028;
const ADBE_BLEND_EN2: usize = 0x2074;
const ADBE_BLEND_EN3: usize = 0x202c;
const ADBE_BLEND_EN4: usize = 0x2034;
const ADBE_MASK_BUF: usize = 0x2200;
const ADBE_SRC_START: usize = 0x4040;
const ADBE_SRC_SIZE: usize = 0x4048;
const ADBE_DST_START: usize = 0x4050;
const ADBE_DST_SIZE: usize = 0x4054;
const ADBE_STRIDE: usize = 0x4038;
const ADBE_FB_BASE: usize = 0x4030;
const ADBE_LAYER_EN1: usize = 0x4020;
const ADBE_LAYER_EN2: usize = 0x4068;
const ADBE_LAYER_EN3: usize = 0x40b4;
const ADBE_LAYER_EN4: usize = 0x40f4;
const ADBE_SCALE_CTL: usize = 0x40ac;
const ADBE_SCALE_CTL_BYPASS: u32 = 0x100000;
const ADBE_LAYER_CTL: usize = 0x1038;
const ADBE_LAYER_CTL_ENABLE: u32 = 0x10000;
const ADBE_PIX_FMT: usize = 0x402c;
const ADBE_PIX_FMT_XRGB32: u32 = 0x53e4001;
const ALL_CRTCS: u32 = 1;

#[repr(C)]
struct AdpDrvPrivate {
    drm: drm_device,
    crtc: drm_crtc,
    encoder: *mut drm_encoder,
    connector: *mut drm_connector,
    next_bridge: *mut drm_bridge,
    be: *mut core::ffi::c_void,
    fe: *mut core::ffi::c_void,
    mask_buf: *mut u32,
    mask_buf_size: u64,
    mask_iova: dma_addr_t,
    be_irq: i32,
    fe_irq: i32,
    event: *mut drm_pending_vblank_event,
}

unsafe fn adp_open(inode: *mut inode, filp: *mut file) -> i32 {
    /* The modesetting driver does not check the non-desktop connector property. */
    if (*current).comm[0] == b'X' as _ { return -EBUSY; }
    drm_open(inode, filp)
}

unsafe fn adp_drm_gem_dumb_create(file_priv: *mut drm_file, drm: *mut drm_device,
                                  args: *mut drm_mode_create_dumb) -> i32 {
    (*args).height = ALIGN((*args).height, 64);
    (*args).size = (*args).pitch * (*args).height;
    drm_gem_dma_dumb_create_internal(file_priv, drm, args)
}

unsafe fn adp_plane_atomic_check(plane: *mut drm_plane, state: *mut drm_atomic_commit) -> i32 {
    let new_plane_state = drm_atomic_get_new_plane_state(state, plane);
    if (*new_plane_state).crtc.is_null() { return 0; }
    let crtc_state = drm_atomic_get_crtc_state(state, (*new_plane_state).crtc);
    if IS_ERR(crtc_state) { return PTR_ERR(crtc_state); }
    drm_atomic_helper_check_plane_state(new_plane_state, crtc_state,
        DRM_PLANE_NO_SCALING, DRM_PLANE_NO_SCALING, true, true)
}

unsafe fn adp_plane_atomic_update(plane: *mut drm_plane, state: *mut drm_atomic_commit) {
    let new_state = drm_atomic_get_new_plane_state(state, plane);
    if plane.is_null() || new_state.is_null() || (*new_state).fb.is_null() { return; }
    let adp = container_of((*plane).dev, AdpDrvPrivate, drm);
    let mut src_rect = core::mem::zeroed::<drm_rect>();
    drm_rect_fp_to_int(&mut src_rect, &(*new_state).src);
    let src_pos = ((*src_rect).x1 << 16) | (*src_rect).y1;
    let dst_pos = ((*new_state).dst.x1 << 16) | (*new_state).dst.y1;
    let src_size = (drm_rect_width(&src_rect) << 16) | drm_rect_height(&src_rect);
    let dst_size = (drm_rect_width(&(*new_state).dst) << 16) | drm_rect_height(&(*new_state).dst);
    writel(src_pos as _, (*adp).be.add(ADBE_SRC_START));
    writel(src_size as _, (*adp).be.add(ADBE_SRC_SIZE));
    writel(dst_pos as _, (*adp).be.add(ADBE_DST_START));
    writel(dst_size as _, (*adp).be.add(ADBE_DST_SIZE));
    writel((*(*new_state).fb).pitches[0], (*adp).be.add(ADBE_STRIDE));
    let obj = drm_fb_dma_get_gem_obj((*new_state).fb, 0);
    if !obj.is_null() { writel((*obj).dma_addr + (*(*new_state).fb).offsets[0] as _, (*adp).be.add(ADBE_FB_BASE)); }
    writel(BIT(0), (*adp).be.add(ADBE_LAYER_EN1)); writel(BIT(0), (*adp).be.add(ADBE_LAYER_EN2));
    writel(BIT(0), (*adp).be.add(ADBE_LAYER_EN3)); writel(BIT(0), (*adp).be.add(ADBE_LAYER_EN4));
    writel(ADBE_SCALE_CTL_BYPASS, (*adp).be.add(ADBE_SCALE_CTL));
    writel(ADBE_LAYER_CTL_ENABLE | BIT(0), (*adp).be.add(ADBE_LAYER_CTL));
    writel(ADBE_PIX_FMT_XRGB32, (*adp).be.add(ADBE_PIX_FMT));
}

unsafe fn adp_plane_atomic_disable(plane: *mut drm_plane, _state: *mut drm_atomic_commit) {
    let adp = container_of((*plane).dev, AdpDrvPrivate, drm);
    writel(0, (*adp).be.add(ADBE_LAYER_EN1)); writel(0, (*adp).be.add(ADBE_LAYER_EN2));
    writel(0, (*adp).be.add(ADBE_LAYER_EN3)); writel(0, (*adp).be.add(ADBE_LAYER_EN4));
    writel(ADBE_LAYER_CTL_ENABLE, (*adp).be.add(ADBE_LAYER_CTL));
}

unsafe fn adp_enable_vblank(adp: *mut AdpDrvPrivate) {
    writel(ADP_INT_STATUS_INT_MASK, (*adp).fe.add(ADP_INT_STATUS));
    let cur_ctrl = readl((*adp).fe.add(ADP_CTRL));
    writel(cur_ctrl | ADP_CTRL_VBLANK_ON, (*adp).fe.add(ADP_CTRL));
}
unsafe fn adp_crtc_enable_vblank(crtc: *mut drm_crtc) -> i32 { adp_enable_vblank(container_of((*crtc).dev, AdpDrvPrivate, drm)); 0 }
unsafe fn adp_disable_vblank(adp: *mut AdpDrvPrivate) {
    let cur_ctrl = readl((*adp).fe.add(ADP_CTRL));
    writel(cur_ctrl & !ADP_CTRL_VBLANK_ON, (*adp).fe.add(ADP_CTRL));
    writel(ADP_INT_STATUS_INT_MASK, (*adp).fe.add(ADP_INT_STATUS));
}
unsafe fn adp_crtc_disable_vblank(crtc: *mut drm_crtc) { adp_disable_vblank(container_of((*crtc).dev, AdpDrvPrivate, drm)); }

unsafe fn adp_crtc_atomic_enable(crtc: *mut drm_crtc, _state: *mut drm_atomic_commit) {
    let adp = container_of(crtc, AdpDrvPrivate, crtc);
    writel(BIT(0), (*adp).be.add(ADBE_BLEND_EN2)); writel(BIT(4), (*adp).be.add(ADBE_BLEND_EN1));
    writel(BIT(0), (*adp).be.add(ADBE_BLEND_EN3)); writel(BIT(0), (*adp).be.add(ADBE_BLEND_BYPASS));
    writel(BIT(0), (*adp).be.add(ADBE_BLEND_EN4)); drm_crtc_vblank_on(crtc);
}
unsafe fn adp_crtc_atomic_disable(crtc: *mut drm_crtc, state: *mut drm_atomic_commit) {
    let adp = container_of(crtc, AdpDrvPrivate, crtc);
    let old_state = drm_atomic_get_old_crtc_state(state, crtc);
    drm_atomic_helper_disable_planes_on_crtc(old_state, false);
    writel(0, (*adp).be.add(ADBE_BLEND_EN2)); writel(0, (*adp).be.add(ADBE_BLEND_EN1));
    writel(0, (*adp).be.add(ADBE_BLEND_EN3)); writel(0, (*adp).be.add(ADBE_BLEND_BYPASS));
    writel(0, (*adp).be.add(ADBE_BLEND_EN4)); drm_crtc_vblank_off(crtc);
}

unsafe fn adp_fe_irq(_irq: i32, arg: *mut core::ffi::c_void) -> irqreturn_t {
    let adp = arg as *mut AdpDrvPrivate; let int_status = readl((*adp).fe.add(ADP_INT_STATUS));
    if int_status & ADP_INT_STATUS_VBLANK != 0 { drm_crtc_handle_vblank(&mut (*adp).crtc); }
    writel(int_status, (*adp).fe.add(ADP_INT_STATUS)); IRQ_HANDLED
}

unsafe fn adp_crtc_atomic_flush(crtc: *mut drm_crtc, state: *mut drm_atomic_commit) {
    let adp = container_of(crtc, AdpDrvPrivate, crtc);
    let new_state = drm_atomic_get_new_crtc_state(state, crtc);
    let new_size = ALIGN(((*new_state).mode.hdisplay * (*new_state).mode.vdisplay * 4) as u64, PAGE_SIZE);
    if new_size != (*adp).mask_buf_size {
        if !(*adp).mask_buf.is_null() { dma_free_coherent((*crtc).dev.dev, (*adp).mask_buf_size, (*adp).mask_buf as _, (*adp).mask_iova); }
        (*adp).mask_buf = core::ptr::null_mut();
        if new_size != 0 {
            (*adp).mask_buf = dma_alloc_coherent((*crtc).dev.dev, new_size, &mut (*adp).mask_iova, GFP_KERNEL) as *mut u32;
            memset((*adp).mask_buf as _, 0xff, new_size);
            writel((*adp).mask_iova as _, (*adp).be.add(ADBE_MASK_BUF));
        }
        (*adp).mask_buf_size = new_size;
    }
    writel(ADBE_FIFO_SYNC | 1, (*adp).be.add(ADBE_FIFO));
    // FIXME: use adbe flush interrupt
    if !(*crtc).state.event.is_null() {
        let event = (*crtc).state.event; (*crtc).state.event = core::ptr::null_mut();
        let mut flags = 0; spin_lock_irqsave(&mut (*(*crtc).dev).event_lock, &mut flags);
        if drm_crtc_vblank_get(crtc) != 0 { drm_crtc_send_vblank_event(crtc, event); } else { (*adp).event = event; }
        spin_unlock_irqrestore(&mut (*(*crtc).dev).event_lock, flags);
    }
}

unsafe fn adp_setup_crtc(adp: *mut AdpDrvPrivate) -> i32 {
    let primary = adp_plane_new(adp); if IS_ERR(primary) { return PTR_ERR(primary); }
    let ret = drm_crtc_init_with_planes(&mut (*adp).drm, &mut (*adp).crtc, primary, core::ptr::null_mut(), &adp_crtc_funcs, core::ptr::null_mut());
    if ret != 0 { return ret; } drm_crtc_helper_add(&mut (*adp).crtc, &adp_crtc_helper_funcs); 0
}
unsafe fn adp_setup_mode_config(adp: *mut AdpDrvPrivate) -> i32 {
    let drm = &mut (*adp).drm; let ret = drmm_mode_config_init(drm); if ret != 0 { return ret; }
    let size = readl((*adp).fe.add(ADP_SCREEN_SIZE));
    drm.mode_config.min_width = 32; drm.mode_config.min_height = 32;
    drm.mode_config.max_width = ALIGN(FIELD_GET(ADP_SCREEN_HSIZE, size), 64);
    drm.mode_config.max_height = ALIGN(FIELD_GET(ADP_SCREEN_VSIZE, size), 64);
    drm.mode_config.preferred_depth = 24; drm.mode_config.prefer_shadow = 0; drm.mode_config.funcs = &adp_mode_config_funcs;
    let ret = adp_setup_crtc(adp); if ret != 0 { return ret; }
    (*adp).encoder = drmm_plain_encoder_alloc(drm, core::ptr::null_mut(), DRM_MODE_ENCODER_DSI, core::ptr::null_mut());
    if IS_ERR((*adp).encoder) { return PTR_ERR((*adp).encoder); } (*(*adp).encoder).possible_crtcs = ALL_CRTCS;
    let ret = drm_bridge_attach((*adp).encoder, (*adp).next_bridge, core::ptr::null_mut(), DRM_BRIDGE_ATTACH_NO_CONNECTOR); if ret != 0 { return ret; }
    (*adp).connector = drm_bridge_connector_init(drm, (*adp).encoder); if IS_ERR((*adp).connector) { return PTR_ERR((*adp).connector); }
    let ret = drm_vblank_init(drm, drm.mode_config.num_crtc); if ret < 0 { return ret; } drm_mode_config_reset(drm); 0
}
unsafe fn adp_drm_bind(dev: *mut device) -> i32 {
    let drm = dev_get_drvdata(dev) as *mut drm_device; let adp = container_of(drm, AdpDrvPrivate, drm);
    writel(ADP_CTRL_FIFO_ON, (*adp).fe.add(ADP_CTRL));
    (*adp).next_bridge = drmm_of_get_bridge(&mut (*adp).drm, (*dev).of_node, 0, 0); if IS_ERR((*adp).next_bridge) { return PTR_ERR((*adp).next_bridge); }
    let err = adp_setup_mode_config(adp); if err < 0 { return err; }
    let err = request_irq((*adp).fe_irq, adp_fe_irq, 0, b"adp-fe\0".as_ptr() as _, adp as _); if err != 0 { return err; }
    drm_dev_register(&mut (*adp).drm, 0)
}
unsafe fn adp_drm_unbind(dev: *mut device) { let drm = dev_get_drvdata(dev) as *mut drm_device; let adp = container_of(drm, AdpDrvPrivate, drm); drm_dev_unregister(drm); drm_atomic_helper_shutdown(drm); free_irq((*adp).fe_irq, adp as _); }
unsafe fn adp_plane_new(adp: *mut AdpDrvPrivate) -> *mut drm_plane {
    let plane = __drmm_universal_plane_alloc(&mut (*adp).drm, size_of::<drm_plane>(), 0, ALL_CRTCS, &adp_plane_funcs, plane_formats.as_ptr(), 1, core::ptr::null_mut(), DRM_PLANE_TYPE_PRIMARY, b"plane\0".as_ptr() as _);
    if IS_ERR(plane) { drm_err(&mut (*adp).drm, b"failed to allocate plane\0".as_ptr() as _); return plane; }
    drm_plane_helper_add(plane, &adp_plane_helper_funcs); plane
}
unsafe fn adp_parse_of(pdev: *mut platform_device, adp: *mut AdpDrvPrivate) -> i32 {
    let dev = &mut (*pdev).dev; (*adp).be = devm_platform_ioremap_resource_byname(pdev, b"be\0".as_ptr() as _); if IS_ERR((*adp).be) { return PTR_ERR((*adp).be); }
    (*adp).fe = devm_platform_ioremap_resource_byname(pdev, b"fe\0".as_ptr() as _); if IS_ERR((*adp).fe) { return PTR_ERR((*adp).fe); }
    (*adp).be_irq = platform_get_irq_byname(pdev, b"be\0".as_ptr() as _); if (*adp).be_irq < 0 { return (*adp).be_irq; }
    (*adp).fe_irq = platform_get_irq_byname(pdev, b"fe\0".as_ptr() as _); if (*adp).fe_irq < 0 { return (*adp).fe_irq; } 0
}
unsafe fn compare_dev(dev: *mut device, data: *mut core::ffi::c_void) -> i32 { ((*dev).of_node != data) as i32 }
unsafe fn adp_probe(pdev: *mut platform_device) -> i32 {
    let adp = devm_drm_dev_alloc(&mut (*pdev).dev, &adp_driver, size_of::<AdpDrvPrivate>(), 0) as *mut AdpDrvPrivate; if IS_ERR(adp) { return PTR_ERR(adp); }
    dev_set_drvdata(&mut (*pdev).dev, &mut (*adp).drm as _); let err = adp_parse_of(pdev, adp); if err < 0 { return err; }
    let port = of_graph_get_remote_node((*pdev).dev.of_node, 0, 0); if port.is_null() { return -ENODEV; }
    let mut mat: *mut component_match = core::ptr::null_mut(); drm_of_component_match_add(&mut (*pdev).dev, &mut mat, compare_dev, port); of_node_put(port);
    component_master_add_with_match(&mut (*pdev).dev, &adp_master_ops, mat)
}
unsafe fn adp_remove(pdev: *mut platform_device) { component_master_del(&mut (*pdev).dev, &adp_master_ops); dev_set_drvdata(&mut (*pdev).dev, core::ptr::null_mut()); }

extern "C" {
    static plane_formats: [u32; 1];
    static adp_plane_funcs: drm_plane_funcs;
    static adp_plane_helper_funcs: drm_plane_helper_funcs;
    static adp_crtc_funcs: drm_crtc_funcs;
    static adp_crtc_helper_funcs: drm_crtc_helper_funcs;
    static adp_mode_config_funcs: drm_mode_config_funcs;
    static adp_master_ops: component_master_ops;
    static adp_fops: file_operations;
    static adp_driver: drm_driver;
    static adp_platform_driver: platform_driver;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
