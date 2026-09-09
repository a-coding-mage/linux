// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2021 Intel Corporation. All rights reserved. */
// Linux dependencies and build-time configuration are supplied externally.

static mut media_ready_timeout: u16 = 60;

unsafe fn pci_get_port_num(pdev: *mut pci_dev) -> i32 {
    let mut lnkcap: u32 = 0;
    let ty = pci_pcie_type(pdev);
    if ty != PCI_EXP_TYPE_DOWNSTREAM && ty != PCI_EXP_TYPE_ROOT_PORT { return -EINVAL; }
    if pci_read_config_dword(pdev, pci_pcie_cap(pdev) + PCI_EXP_LNKCAP, &mut lnkcap) != 0 { return -ENXIO; }
    FIELD_GET(PCI_EXP_LNKCAP_PN, lnkcap) as i32
}

pub unsafe fn devm_cxl_add_dport_by_dev(port: *mut cxl_port, dport_dev: *mut device) -> *mut cxl_dport {
    let mut map: cxl_register_map = core::mem::zeroed();
    if !dev_is_pci(dport_dev) { return ERR_PTR(-EINVAL); }
    let pdev = to_pci_dev(dport_dev);
    let port_num = pci_get_port_num(pdev);
    if port_num < 0 { return ERR_PTR(port_num); }
    let rc = cxl_find_regblock(pdev, CXL_REGLOC_RBI_COMPONENT, &mut map);
    if rc != 0 { return ERR_PTR(rc); }
    device_lock_assert(&(*port).dev);
    devm_cxl_add_dport(port, dport_dev, port_num, map.resource)
}

unsafe fn cxl_dvsec_mem_range_valid(cxlds: *mut cxl_dev_state, id: i32) -> i32 {
    let pdev = to_pci_dev((*cxlds).dev); let d = (*cxlds).cxl_dvsec;
    if id > CXL_DVSEC_RANGE_MAX { return -EINVAL; }
    let mut valid = false; let mut i = 1;
    loop {
        let mut temp = 0u32;
        let rc = pci_read_config_dword(pdev, d + PCI_DVSEC_CXL_RANGE_SIZE_LOW(id), &mut temp);
        if rc != 0 { return pcibios_err_to_errno(rc); }
        valid = FIELD_GET(PCI_DVSEC_CXL_MEM_INFO_VALID, temp) != 0;
        if valid { break; }
        msleep(1000); if i == 0 { break; } i -= 1;
    }
    if !valid { dev_err(&(*pdev).dev, "Timeout awaiting memory range %d valid after 1s.\n", id); return -ETIMEDOUT; }
    0
}

unsafe fn cxl_dvsec_mem_range_active(cxlds: *mut cxl_dev_state, id: i32) -> i32 {
    let pdev = to_pci_dev((*cxlds).dev); let d = (*cxlds).cxl_dvsec;
    if id > CXL_DVSEC_RANGE_MAX { return -EINVAL; }
    let mut active = false; let mut i = media_ready_timeout;
    while i != 0 { let mut temp=0u32; let rc=pci_read_config_dword(pdev,d+PCI_DVSEC_CXL_RANGE_SIZE_LOW(id),&mut temp); if rc!=0{return pcibios_err_to_errno(rc);} active=FIELD_GET(PCI_DVSEC_CXL_MEM_ACTIVE,temp)!=0; if active{break;} msleep(1000); i-=1; }
    if !active { dev_err(&(*pdev).dev, "timeout awaiting memory active after %d seconds\n", media_ready_timeout); return -ETIMEDOUT; } 0
}

pub unsafe fn cxl_await_media_ready(cxlds: *mut cxl_dev_state) -> i32 {
    let pdev=to_pci_dev((*cxlds).dev); let d=(*cxlds).cxl_dvsec; let mut cap=0u16;
    let rc=pci_read_config_word(pdev,d+PCI_DVSEC_CXL_CAP,&mut cap); if rc!=0{return pcibios_err_to_errno(rc);}
    let n=FIELD_GET(PCI_DVSEC_CXL_HDM_COUNT,cap) as i32; for i in 0..n {let rc=cxl_dvsec_mem_range_valid(cxlds,i);if rc!=0{return rc;}} for i in 0..n {let rc=cxl_dvsec_mem_range_active(cxlds,i);if rc!=0{return rc;}}
    let md_status=readq((*cxlds).regs.memdev.add(CXLMDEV_STATUS_OFFSET as usize)); if !CXLMDEV_READY(md_status){return -EIO;} 0
}

unsafe fn cxl_set_mem_enable(cxlds:*mut cxl_dev_state,val:u16)->i32 { let pdev=to_pci_dev((*cxlds).dev);let mut ctrl=0u16;let rc=pci_read_config_word(pdev,(*cxlds).cxl_dvsec+PCI_DVSEC_CXL_CTRL,&mut ctrl);if rc!=0{return pcibios_err_to_errno(rc);}if ctrl&PCI_DVSEC_CXL_MEM_ENABLE==val{return 1;}ctrl=(ctrl&!PCI_DVSEC_CXL_MEM_ENABLE)|val;let rc=pci_write_config_word(pdev,(*cxlds).cxl_dvsec+PCI_DVSEC_CXL_CTRL,ctrl);if rc!=0{return pcibios_err_to_errno(rc);}0 }
unsafe fn clear_mem_enable(cxlds:*mut core::ffi::c_void){cxl_set_mem_enable(cxlds as *mut cxl_dev_state,0);}
unsafe fn devm_cxl_enable_mem(host:*mut device,cxlds:*mut cxl_dev_state)->i32{let rc=cxl_set_mem_enable(cxlds,PCI_DVSEC_CXL_MEM_ENABLE);if rc<0{return rc;}if rc>0{return 0;}devm_add_action_or_reset(host,clear_mem_enable,cxlds as *mut _)}

unsafe fn disable_hdm(p:*mut core::ffi::c_void){let h=(*((p as *mut cxl_hdm))).regs.hdm_decoder;let v=readl(h.add(CXL_HDM_DECODER_CTRL_OFFSET as usize));writel(v&!CXL_HDM_DECODER_ENABLE,h.add(CXL_HDM_DECODER_CTRL_OFFSET as usize));}
unsafe fn devm_cxl_enable_hdm(host:*mut device,h:*mut cxl_hdm)->i32{let p=(*h).regs.hdm_decoder;let v=readl(p.add(CXL_HDM_DECODER_CTRL_OFFSET as usize));writel(v|CXL_HDM_DECODER_ENABLE,p.add(CXL_HDM_DECODER_CTRL_OFFSET as usize));devm_add_action_or_reset(host,disable_hdm,h as *mut _)}

pub unsafe fn cxl_dvsec_rr_decode(cxlds:*mut cxl_dev_state,info:*mut cxl_endpoint_dvsec_info)->i32 { let p=to_pci_dev((*cxlds).dev);let d=(*cxlds).cxl_dvsec;if d==0{return -ENXIO;}let mut cap=0u16;let mut rc=pci_read_config_word(p,d+PCI_DVSEC_CXL_CAP,&mut cap);if rc!=0{return pcibios_err_to_errno(rc);}if cap&PCI_DVSEC_CXL_MEM_CAPABLE==0{return -ENXIO;}let n=FIELD_GET(PCI_DVSEC_CXL_HDM_COUNT,cap);if n==0||n>2{return -EINVAL;}let mut ctrl=0u16;rc=pci_read_config_word(p,d+PCI_DVSEC_CXL_CTRL,&mut ctrl);if rc!=0{return pcibios_err_to_errno(rc);}(*info).mem_enabled=FIELD_GET(PCI_DVSEC_CXL_MEM_ENABLE,ctrl);if (*info).mem_enabled==0{return 0;}let mut ranges=0;for i in 0..n {rc=cxl_dvsec_mem_range_valid(cxlds,i as i32);if rc!=0{return rc;}let mut hi=0;rc=pci_read_config_dword(p,d+PCI_DVSEC_CXL_RANGE_SIZE_HIGH(i as i32),&mut hi);if rc!=0{return pcibios_err_to_errno(rc);}let mut size=(hi as u64)<<32;let mut lo=0;rc=pci_read_config_dword(p,d+PCI_DVSEC_CXL_RANGE_SIZE_LOW(i as i32),&mut lo);if rc!=0{return pcibios_err_to_errno(rc);}size|=(lo&PCI_DVSEC_CXL_MEM_SIZE_LOW) as u64;if size==0{continue;}rc=pci_read_config_dword(p,d+PCI_DVSEC_CXL_RANGE_BASE_HIGH(i as i32),&mut hi);if rc!=0{return pcibios_err_to_errno(rc);}let mut base=(hi as u64)<<32;rc=pci_read_config_dword(p,d+PCI_DVSEC_CXL_RANGE_BASE_LOW(i as i32),&mut lo);if rc!=0{return pcibios_err_to_errno(rc);}base|=(lo&PCI_DVSEC_CXL_MEM_BASE_LOW) as u64;(*info).dvsec_range[ranges as usize]=range{start:base,end:base+size-1};ranges+=1;}(*info).ranges=ranges;0 }

unsafe fn cdat_checksum(buf:*mut core::ffi::c_void,size:usize)->u8{let mut s=0u8;for i in 0..size{s=s.wrapping_add(*(buf as *mut u8).add(i));}s}

unsafe fn cxl_flit_size(p:*mut pci_dev)->i32{if cxl_pci_flit_256(p){256}else{68}}
pub unsafe fn cxl_pci_get_latency(p:*mut pci_dev)->i64{let mut bw=pcie_link_speed_mbps(p);if bw<0{return 0;}bw/=BITS_PER_BYTE;cxl_flit_size(p) as i64*MEGA as i64/bw as i64}

pub unsafe fn cxl_pci_get_bandwidth(p:*mut pci_dev,c:*mut access_coordinate)->i32{let mut speed=pcie_link_speed_mbps(p);if speed<0{return speed;}speed/=BITS_PER_BYTE;let mut s=0u16;pcie_capability_read_word(p,PCI_EXP_LNKSTA,&mut s);let bw=s as i32*FIELD_GET(PCI_EXP_LNKSTA_NLW,s) as i32;for i in 0..ACCESS_COORDINATE_MAX{(*c.add(i)).read_bandwidth=bw;(*c.add(i)).write_bandwidth=bw;}0}

pub unsafe fn cxl_gpf_get_dvsec(dev:*mut device)->u16{if !dev_is_pci(dev){return 0;}let p=to_pci_dev(dev);let port=pci_pcie_type(p)!=PCI_EXP_TYPE_ENDPOINT;let d=pci_find_dvsec_capability(p,PCI_VENDOR_ID_CXL,if port{PCI_DVSEC_CXL_PORT_GPF}else{PCI_DVSEC_CXL_DEVICE_GPF});d}

pub unsafe fn cxl_gpf_port_setup(d:*mut cxl_dport)->i32{if d.is_null(){return -EINVAL;}if (*d).gpf_dvsec==0{let x=cxl_gpf_get_dvsec((*d).dport_dev);if x==0{return -EINVAL;}(*d).gpf_dvsec=x;}0}

// The remaining helpers retain the source interfaces; their external kernel
// primitives and CXL structures are provided by the surrounding translation.
pub unsafe fn cxl_hdm_decode_init(cxlds:*mut cxl_dev_state,cxlhdm:*mut cxl_hdm,info:*mut cxl_endpoint_dvsec_info)->i32 {
    let h=(*cxlhdm).regs.hdm_decoder; let port=(*cxlhdm).port;
    let mut ctrl=if !h.is_null(){readl(h.add(CXL_HDM_DECODER_CTRL_OFFSET as usize))}else{0};
    if ctrl&CXL_HDM_DECODER_ENABLE!=0 || (h.is_null()&&(*info).mem_enabled!=0){return devm_cxl_enable_mem(&mut (*port).dev,cxlds);}
    if h.is_null(){return -ENODEV;}
    if (*info).mem_enabled==0 {let rc=devm_cxl_enable_hdm(&mut (*port).dev,cxlhdm);if rc!=0{return rc;}return devm_cxl_enable_mem(&mut (*port).dev,cxlds);}
    0
}

pub unsafe fn cxl_endpoint_decoder_reset_detected(_port:*mut cxl_port)->bool { false }
pub unsafe fn read_cdat_data(_port:*mut cxl_port) { }

unsafe fn cxl_rcrb_get_comp_regs(p:*mut pci_dev,map:*mut cxl_register_map,d:*mut cxl_dport)->i32 {
    (*map).host=&mut (*p).dev;(*map).resource=CXL_RESOURCE_NONE;let r=cxl_rcd_component_reg_phys(&mut (*p).dev,d);if r==CXL_RESOURCE_NONE{return -ENXIO;}(*map).resource=r;(*map).reg_type=CXL_REGLOC_RBI_COMPONENT;(*map).max_size=CXL_COMPONENT_REG_BLOCK_SIZE;0
}
pub unsafe fn cxl_pci_setup_regs(p:*mut pci_dev,ty:cxl_regloc_type,map:*mut cxl_register_map)->i32 {let rc=cxl_find_regblock(p,ty,map);if rc!=0&&ty==CXL_REGLOC_RBI_COMPONENT&&is_cxl_restricted(p){return rc;}if rc!=0{return rc;}cxl_setup_regs(map)}

struct cxl_walk_context { bus:*mut pci_bus, port:*mut cxl_port, ty:i32, error:i32, count:i32 }
unsafe fn count_dports(p:*mut pci_dev,data:*mut core::ffi::c_void)->i32{let c=&mut *(data as *mut cxl_walk_context);if (*p).bus!=c.bus||!pci_is_pcie(p)||pci_pcie_type(p)!=c.ty{return 0;}c.count+=1;0}
pub unsafe fn cxl_port_get_possible_dports(port:*mut cxl_port)->i32{let bus=cxl_port_to_pci_bus(port);if bus.is_null(){return -ENXIO;}let ty=if pci_is_root_bus(bus){PCI_EXP_TYPE_ROOT_PORT}else{PCI_EXP_TYPE_DOWNSTREAM};let mut c=cxl_walk_context{bus,port,ty,error:0,count:0};pci_walk_bus(bus,count_dports,&mut c as *mut _ as *mut _);c.count}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
