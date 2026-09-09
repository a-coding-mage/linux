// SPDX-License-Identifier: GPL-2.0
/*
 * Driver for FPGA Management Engine (FME) Partial Reconfiguration
 *
 * Copyright (C) 2017-2018 Intel Corporation, Inc.
 *
 * Authors:
 *   Kang Luwei <luwei.kang@intel.com>
 *   Xiao Guangrong <guangrong.xiao@linux.intel.com>
 *   Wu Hao <hao.wu@intel.com>
 *   Joseph Grecco <joe.grecco@intel.com>
 *   Enno Luebbers <enno.luebbers@intel.com>
 *   Tim Whisonant <tim.whisonant@intel.com>
 *   Ananda Ravuri <ananda.ravuri@intel.com>
 *   Christopher Rauer <christopher.rauer@intel.com>
 *   Henry Mitchel <henry.mitchel@intel.com>
 */

// Dependencies supplied by the Linux kernel and the surrounding driver.

unsafe fn dfl_fme_region_find_by_port_id(fme: *mut dfl_fme, port_id: i32) -> *mut dfl_fme_region {
    let mut fme_region: *mut dfl_fme_region;
    // list_for_each_entry(fme_region, &fme->region_list, node)
    for fme_region in unsafe { list_entries(fme, 0) } {
        if unsafe { (*fme_region).port_id } == port_id {
            return fme_region;
        }
    }
    std::ptr::null_mut()
}

unsafe fn dfl_fme_region_match(dev: *mut device, data: *const core::ffi::c_void) -> i32 {
    (unsafe { (*dev).parent } == data) as i32
}

unsafe fn dfl_fme_region_find(fme: *mut dfl_fme, port_id: i32) -> *mut fpga_region {
    let fme_region = dfl_fme_region_find_by_port_id(fme, port_id);
    if fme_region.is_null() { return std::ptr::null_mut(); }
    let region = fpga_region_class_find(
        std::ptr::null_mut(),
        unsafe { &mut (*(*fme_region).region).dev },
        dfl_fme_region_match,
    );
    if region.is_null() { return std::ptr::null_mut(); }
    region
}

unsafe fn fme_pr(pdev: *mut platform_device, arg: usize) -> i32 {
    let fdata = to_dfl_feature_dev_data(unsafe { &mut (*pdev).dev });
    let argp = arg as *mut core::ffi::c_void;
    let mut port_pr: dfl_fpga_fme_port_pr = core::mem::zeroed();
    let mut info: *mut fpga_image_info;
    let mut region: *mut fpga_region;
    let fme_hdr: *mut u8;
    let mut fme: *mut dfl_fme;
    let minsz = core::mem::size_of::<dfl_fpga_fme_port_pr>();
    let mut buf: *mut core::ffi::c_void = std::ptr::null_mut();
    let length: usize;
    let mut ret = 0i32;

    if copy_from_user(&mut port_pr as *mut _ as *mut _, argp, minsz) != 0 { return -EFAULT; }
    if port_pr.argsz < minsz || port_pr.flags != 0 { return -EINVAL; }
    fme_hdr = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_HEADER);
    let v = readq(fme_hdr.add(FME_HDR_CAP));
    if port_pr.port_id >= FIELD_GET(FME_CAP_NUM_PORTS, v) {
        dev_dbg(&mut (*pdev).dev, "port number more than maximum\n");
        return -EINVAL;
    }
    length = ALIGN(port_pr.buffer_size, 4);
    buf = vmalloc(length);
    if buf.is_null() { return -ENOMEM; }
    if copy_from_user(buf, port_pr.buffer_address as usize as *mut _, port_pr.buffer_size) != 0 {
        ret = -EFAULT; goto free_exit;
    }
    info = fpga_image_info_alloc(&mut (*pdev).dev);
    if info.is_null() { ret = -ENOMEM; goto free_exit; }
    (*info).flags |= FPGA_MGR_PARTIAL_RECONFIG;
    mutex_lock(&mut (*fdata).lock);
    fme = dfl_fpga_fdata_get_private(fdata);
    if fme.is_null() { ret = -EINVAL; goto unlock_exit; }
    region = dfl_fme_region_find(fme, port_pr.port_id);
    if region.is_null() { ret = -EINVAL; goto unlock_exit; }
    fpga_image_info_free((*region).info);
    (*info).buf = buf;
    (*info).count = length;
    (*info).region_id = port_pr.port_id;
    (*region).info = info;
    ret = fpga_region_program_fpga(region);
    if !(*region).get_bridges.is_none() { fpga_bridges_put(&mut (*region).bridge_list); }
    put_device(&mut (*region).dev);
unlock_exit:
    mutex_unlock(&mut (*fdata).lock);
free_exit:
    vfree(buf);
    ret
}

/* dfl_fme_create_mgr - create fpga mgr platform device as child device */
unsafe fn dfl_fme_create_mgr(fdata: *mut dfl_feature_dev_data, feature: *mut dfl_feature) -> *mut platform_device {
    let fme = (*fdata).dev;
    let mut mgr_pdata: dfl_fme_mgr_pdata = core::mem::zeroed();
    let mut ret = -ENOMEM;
    if (*feature).ioaddr.is_null() { return ERR_PTR(-ENODEV); }
    mgr_pdata.ioaddr = (*feature).ioaddr;
    let mgr = platform_device_alloc(DFL_FPGA_FME_MGR, (*fme).id);
    if mgr.is_null() { return ERR_PTR(ret); }
    (*mgr).dev.parent = &mut (*fme).dev;
    ret = platform_device_add_data(mgr, &mgr_pdata as *const _ as *const _, core::mem::size_of_val(&mgr_pdata));
    if ret != 0 { platform_device_put(mgr); return ERR_PTR(ret); }
    ret = platform_device_add(mgr);
    if ret != 0 { platform_device_put(mgr); return ERR_PTR(ret); }
    mgr
}

unsafe fn dfl_fme_destroy_mgr(fdata: *mut dfl_feature_dev_data) {
    let priv_ = dfl_fpga_fdata_get_private(fdata);
    platform_device_unregister((*priv_).mgr);
}

/* dfl_fme_create_bridge - create fme fpga bridge platform device as child */
unsafe fn dfl_fme_create_bridge(fdata: *mut dfl_feature_dev_data, port_id: i32) -> *mut dfl_fme_bridge {
    let dev = &mut (*(*fdata).dev).dev;
    let mut br_pdata: dfl_fme_br_pdata = core::mem::zeroed();
    let fme_br = devm_kzalloc(dev, core::mem::size_of::<dfl_fme_bridge>(), GFP_KERNEL) as *mut dfl_fme_bridge;
    if fme_br.is_null() { return ERR_PTR(-ENOMEM); }
    br_pdata.cdev = (*fdata).dfl_cdev; br_pdata.port_id = port_id;
    (*fme_br).br = platform_device_alloc(DFL_FPGA_FME_BRIDGE, PLATFORM_DEVID_AUTO);
    if (*fme_br).br.is_null() { return ERR_PTR(-ENOMEM); }
    (*(*fme_br).br).dev.parent = dev;
    let mut ret = platform_device_add_data((*fme_br).br, &br_pdata as *const _ as *const _, core::mem::size_of_val(&br_pdata));
    if ret == 0 { ret = platform_device_add((*fme_br).br); }
    if ret != 0 { platform_device_put((*fme_br).br); return ERR_PTR(ret); }
    fme_br
}

unsafe fn dfl_fme_destroy_bridge(fme_br: *mut dfl_fme_bridge) { platform_device_unregister((*fme_br).br); }

unsafe fn dfl_fme_destroy_bridges(fdata: *mut dfl_feature_dev_data) {
    let priv_ = dfl_fpga_fdata_get_private(fdata);
    // list_for_each_entry_safe(fbridge, tmp, &priv->bridge_list, node)
    for (fbridge, tmp) in list_entries_safe(priv_, 1) { list_del(&mut (*fbridge).node); dfl_fme_destroy_bridge(fbridge); let _ = tmp; }
}

/* Region creation/destruction and management lifecycle retain the kernel list and device operations. */
unsafe fn dfl_fme_destroy_region(r: *mut dfl_fme_region) { platform_device_unregister((*r).region); }
unsafe fn dfl_fme_destroy_regions(fdata: *mut dfl_feature_dev_data) { let p=dfl_fpga_fdata_get_private(fdata); for (r,t) in list_entries_safe(p,0) { list_del(&mut (*r).node); dfl_fme_destroy_region(r); let _=t; } }

unsafe fn dfl_fme_create_region(fdata: *mut dfl_feature_dev_data, mgr: *mut platform_device, br: *mut platform_device, port_id: i32) -> *mut dfl_fme_region {
    let dev=&mut (*(*fdata).dev).dev; let r=devm_kzalloc(dev,core::mem::size_of::<dfl_fme_region>(),GFP_KERNEL) as *mut dfl_fme_region; if r.is_null(){return ERR_PTR(-ENOMEM);}
    let mut pdata:dfl_fme_region_pdata=core::mem::zeroed(); pdata.mgr=mgr; pdata.br=br; (*r).region=platform_device_alloc(DFL_FPGA_FME_REGION,(*br).id); if (*r).region.is_null(){return ERR_PTR(-ENOMEM);} (*(*r).region).dev.parent=dev;
    let mut ret=platform_device_add_data((*r).region,&pdata as *const _ as *const _,core::mem::size_of_val(&pdata)); if ret==0{ret=platform_device_add((*r).region);} if ret!=0{platform_device_put((*r).region);return ERR_PTR(ret);} (*r).port_id=port_id; r
}

unsafe fn pr_mgmt_init(pdev: *mut platform_device, feature: *mut dfl_feature) -> i32 {
    let f=to_dfl_feature_dev_data(&mut (*pdev).dev); let p=dfl_fpga_fdata_get_private(f); let hdr=dfl_get_feature_ioaddr_by_id(f,FME_FEATURE_ID_HEADER); let mut ret=-ENODEV; mutex_lock(&mut (*f).lock); INIT_LIST_HEAD(&mut (*p).region_list); INIT_LIST_HEAD(&mut (*p).bridge_list); let m=dfl_fme_create_mgr(f,feature); if IS_ERR(m){dev_err(&mut (*pdev).dev,"fail to create fpga mgr pdev\n");mutex_unlock(&mut (*f).lock);return ret;} (*p).mgr=m;
    let cap=readq(hdr.add(FME_HDR_CAP)); let n=FIELD_GET(FME_CAP_NUM_PORTS,cap); for i in 0..n { let off=readq(hdr.add(FME_HDR_PORT_OFST(i))); if off&FME_PORT_OFST_IMP==0{continue;} let b=dfl_fme_create_bridge(f,i as i32); if IS_ERR(b){ret=PTR_ERR(b);break;} list_add(&mut (*b).node,&mut (*p).bridge_list); let r=dfl_fme_create_region(f,m,(*b).br,i as i32); if IS_ERR(r){ret=PTR_ERR(r);break;} list_add(&mut (*r).node,&mut (*p).region_list); ret=0; } if ret!=0 { dfl_fme_destroy_regions(f); dfl_fme_destroy_bridges(f); dfl_fme_destroy_mgr(f); } mutex_unlock(&mut (*f).lock); ret
}
unsafe fn pr_mgmt_uinit(pdev: *mut platform_device, _: *mut dfl_feature) { let f=to_dfl_feature_dev_data(&mut (*pdev).dev); mutex_lock(&mut (*f).lock); dfl_fme_destroy_regions(f); dfl_fme_destroy_bridges(f); dfl_fme_destroy_mgr(f); mutex_unlock(&mut (*f).lock); }
unsafe fn fme_pr_ioctl(pdev: *mut platform_device, _: *mut dfl_feature, cmd: u32, arg: usize) -> i64 { if cmd == DFL_FPGA_FME_PORT_PR { fme_pr(pdev,arg) as i64 } else { -ENODEV as i64 } }

pub static fme_pr_mgmt_id_table: [dfl_feature_id; 2] = [dfl_feature_id { id: FME_FEATURE_ID_PR_MGMT }, dfl_feature_id { id: 0 }];
pub static fme_pr_mgmt_ops: dfl_feature_ops = dfl_feature_ops { init: Some(pr_mgmt_init), uinit: Some(pr_mgmt_uinit), ioctl: Some(fme_pr_ioctl) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
