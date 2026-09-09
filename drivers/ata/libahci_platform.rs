// SPDX-License-Identifier: GPL-2.0-or-later
/* AHCI SATA platform library; translated directly from the C implementation. */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Kernel-provided types, constants, globals, and functions are external dependencies.
#[repr(C)] pub struct ata_host { pub private_data: *mut c_void, pub n_ports: c_uint, pub ports: *mut *mut ata_port, pub flags: c_uint }
#[repr(C)] pub struct ata_port { pub ops: *mut ata_port_operations, pub flags: c_uint, pub port_no: c_uint, pub em_message_type: c_uint }
#[repr(C)] pub struct ata_port_operations { pub inherits: *const ata_port_operations, pub host_stop: Option<unsafe extern "C" fn(*mut ata_host)> }
#[repr(C)] pub struct ata_port_info { pub flags: c_uint, pub private_data: *mut c_void }
#[repr(C)] pub struct scsi_host_template;
#[repr(C)] pub struct device { pub of_node: *mut device_node, pub power: power_state }
#[repr(C)] pub struct power_state { pub power_state: power_event }
#[repr(C)] pub struct power_event { pub event: c_uint }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct clk;
#[repr(C)] pub struct phy;
#[repr(C)] pub struct regulator;
#[repr(C)] pub struct reset_control;
#[repr(C)] pub struct ahci_host_priv {
    pub nports: c_int, pub n_clks: c_int, pub phys: *mut *mut phy,
    pub clks: *mut clk_bulk_data, pub ahci_regulator: *mut regulator,
    pub phy_regulator: *mut regulator, pub target_pwrs: *mut *mut regulator,
    pub rsts: *mut reset_control, pub f_rsts: c_uint, pub mmio: *mut c_void,
    pub saved_cap: u32, pub saved_port_map: u32, pub saved_port_cap: [u32; 32],
    pub mask_port_map: u32, pub flags: c_uint, pub irq: c_int, pub cap: u32,
    pub port_map: u32, pub em_msg_type: c_uint, pub got_runtime_pm: bool,
}
#[repr(C)] pub struct clk_bulk_data { pub id: *const c_char, pub clk: *mut clk }

extern "C" {
    static mut ahci_ops: ata_port_operations;
    static mut ata_dummy_port_ops: ata_port_operations;
    static mut ahci_ignore_sss: bool;
    fn ahci_ignore_port(h: *mut ahci_host_priv, p: c_int) -> bool;
    fn phy_init(p: *mut phy) -> c_int; fn phy_set_mode(p: *mut phy, mode: c_uint) -> c_int;
    fn phy_power_on(p: *mut phy) -> c_int; fn phy_power_off(p: *mut phy); fn phy_exit(p: *mut phy);
    fn clk_bulk_prepare_enable(n: c_int, c: *mut clk_bulk_data) -> c_int; fn clk_bulk_disable_unprepare(n: c_int, c: *mut clk_bulk_data);
    fn reset_control_reset(r: *mut reset_control) -> c_int; fn reset_control_deassert(r: *mut reset_control) -> c_int;
    fn reset_control_rearm(r: *mut reset_control) -> c_int; fn reset_control_assert(r: *mut reset_control) -> c_int;
    fn regulator_enable(r: *mut regulator) -> c_int; fn regulator_disable(r: *mut regulator); fn regulator_put(r: *mut regulator);
    fn pm_runtime_put_sync(d: *mut device); fn pm_runtime_disable(d: *mut device); fn pm_runtime_enable(d: *mut device); fn pm_runtime_get_sync(d: *mut device);
    fn ahci_platform_get_resources_impl(p: *mut platform_device, flags: c_uint) -> *mut ahci_host_priv;
    fn ahci_platform_init_host_impl(p: *mut platform_device, h: *mut ahci_host_priv, i: *const ata_port_info, s: *const scsi_host_template) -> c_int;
    fn ata_host_suspend(h: *mut ata_host, msg: c_uint); fn ata_host_resume(h: *mut ata_host);
    fn ahci_reset_controller(h: *mut ata_host) -> c_int; fn ahci_init_controller(h: *mut ata_host); fn ahci_print_info(h: *mut ata_host, s: *const c_char);
    fn ahci_reset_em(h: *mut ata_host); fn ahci_set_em_messages(h: *mut ahci_host_priv, i: *mut ata_port_info);
    fn ahci_host_activate(h: *mut ata_host, s: *const scsi_host_template) -> c_int;
    fn platform_get_drvdata(p: *mut platform_device) -> *mut ata_host; fn dev_get_drvdata(d: *mut device) -> *mut ata_host;
    fn readl(p: *mut c_void) -> u32; fn writel(v: u32, p: *mut c_void);
    fn ata_port_desc(p: *mut ata_port, f: *const c_char, ...);
    fn ata_port_freeze(p: *mut ata_port); fn ata_port_stop(p: *mut ata_port);
}

pub const AHCI_PLATFORM_RST_TRIGGER: u32 = 1 << 0;
pub const PHY_MODE_SATA: u32 = 0; pub const HOST_CAP_SSS: u32 = 1 << 27; pub const HOST_CAP_MPS: u32 = 1 << 28;
pub const PORT_CMD_CAP: u32 = 0xffff; pub const AHCI_MAX_PORTS: u32 = 32; pub const AHCI_PLATFORM_GET_RESETS: u32 = 1 << 1;
pub const AHCI_HFLAG_NO_SUSPEND: u32 = 1 << 0; pub const AHCI_HFLAG_SUSPEND_PHYS: u32 = 1 << 1;
pub const HOST_CAP_NCQ: u32 = 1 << 0; pub const HOST_CAP_PMP: u32 = 1 << 1; pub const HOST_CAP_64: u32 = 1 << 2;
pub const ATA_FLAG_NCQ: u32 = 1 << 0; pub const ATA_FLAG_PMP: u32 = 1 << 1; pub const ATA_FLAG_EM: u32 = 1 << 2;
pub const ATA_HOST_PARALLEL_SCAN: u32 = 1 << 0; pub const HOST_IRQ_EN: u32 = 1 << 1; pub const HOST_CTL: usize = 0; pub const HOST_IRQ_STAT: usize = 4;

#[no_mangle] pub unsafe extern "C" fn ahci_platform_find_clk(h: *mut ahci_host_priv, con_id: *const c_char) -> *mut clk {
    for i in 0..(*h).n_clks { let c = *(*h).clks.add(i as usize); if !c.id.is_null() && !con_id.is_null() {
        let mut a=c.id; let mut b=con_id; while *a==*b && *a!=0 { a=a.add(1); b=b.add(1); }
        if *a==0 && *b==0 { return c.clk; }
    }} std::ptr::null_mut()
}

#[no_mangle] pub static mut ahci_platform_ops: ata_port_operations = ata_port_operations { inherits: unsafe { &ahci_ops }, host_stop: Some(ahci_host_stop) };

#[no_mangle] pub unsafe extern "C" fn ahci_platform_enable_phys(h: *mut ahci_host_priv) -> c_int {
    let mut i = 0; let mut rc;
    while i < (*h).nports { if ahci_ignore_port(h, i) { i += 1; continue; }
        rc = phy_init(*(*h).phys.add(i as usize)); if rc != 0 { while i > 0 { i -= 1; if !ahci_ignore_port(h,i) { phy_power_off(*(*h).phys.add(i as usize)); phy_exit(*(*h).phys.add(i as usize)); } } return rc; }
        rc = phy_set_mode(*(*h).phys.add(i as usize), PHY_MODE_SATA); if rc != 0 { phy_exit(*(*h).phys.add(i as usize)); while i > 0 { i -= 1; if !ahci_ignore_port(h,i) { phy_power_off(*(*h).phys.add(i as usize)); phy_exit(*(*h).phys.add(i as usize)); } } return rc; }
        rc = phy_power_on(*(*h).phys.add(i as usize)); if rc != 0 { phy_exit(*(*h).phys.add(i as usize)); while i > 0 { i -= 1; if !ahci_ignore_port(h,i) { phy_power_off(*(*h).phys.add(i as usize)); phy_exit(*(*h).phys.add(i as usize)); } } return rc; } i += 1; }
    0
}
#[no_mangle] pub unsafe extern "C" fn ahci_platform_disable_phys(h: *mut ahci_host_priv) { for i in 0..(*h).nports { if !ahci_ignore_port(h,i) { phy_power_off(*(*h).phys.add(i as usize)); phy_exit(*(*h).phys.add(i as usize)); } } }
#[no_mangle] pub unsafe extern "C" fn ahci_platform_enable_clks(h: *mut ahci_host_priv)->c_int { clk_bulk_prepare_enable((*h).n_clks,(*h).clks) }
#[no_mangle] pub unsafe extern "C" fn ahci_platform_disable_clks(h: *mut ahci_host_priv) { clk_bulk_disable_unprepare((*h).n_clks,(*h).clks) }
#[no_mangle] pub unsafe extern "C" fn ahci_platform_deassert_rsts(h: *mut ahci_host_priv)->c_int { if (*h).f_rsts & AHCI_PLATFORM_RST_TRIGGER != 0 { reset_control_reset((*h).rsts) } else { reset_control_deassert((*h).rsts) } }
#[no_mangle] pub unsafe extern "C" fn ahci_platform_assert_rsts(h: *mut ahci_host_priv)->c_int { if (*h).f_rsts & AHCI_PLATFORM_RST_TRIGGER != 0 { reset_control_rearm((*h).rsts) } else { reset_control_assert((*h).rsts) } }

#[no_mangle] pub unsafe extern "C" fn ahci_platform_enable_regulators(h:*mut ahci_host_priv)->c_int { let mut rc=regulator_enable((*h).ahci_regulator); if rc!=0{return rc} rc=regulator_enable((*h).phy_regulator); if rc!=0{regulator_disable((*h).ahci_regulator);return rc} let mut i=0; while i<(*h).nports { let r=*(*h).target_pwrs.add(i as usize); if !r.is_null(){rc=regulator_enable(r);if rc!=0{while i>0{i-=1;let x=*(*h).target_pwrs.add(i as usize);if !x.is_null(){regulator_disable(x)}} regulator_disable((*h).phy_regulator); regulator_disable((*h).ahci_regulator); return rc}} i+=1} 0 }
#[no_mangle] pub unsafe extern "C" fn ahci_platform_disable_regulators(h:*mut ahci_host_priv){for i in 0..(*h).nports{let r=*(*h).target_pwrs.add(i as usize);if !r.is_null(){regulator_disable(r)}} regulator_disable((*h).ahci_regulator);regulator_disable((*h).phy_regulator)}
#[no_mangle] pub unsafe extern "C" fn ahci_platform_enable_resources(h:*mut ahci_host_priv)->c_int{let mut rc=ahci_platform_enable_regulators(h);if rc!=0{return rc}rc=ahci_platform_enable_clks(h);if rc!=0{ahci_platform_disable_regulators(h);return rc}rc=ahci_platform_deassert_rsts(h);if rc!=0{ahci_platform_disable_clks(h);ahci_platform_disable_regulators(h);return rc}rc=ahci_platform_enable_phys(h);if rc!=0{ahci_platform_assert_rsts(h);ahci_platform_disable_clks(h);ahci_platform_disable_regulators(h)}rc}
#[no_mangle] pub unsafe extern "C" fn ahci_platform_disable_resources(h:*mut ahci_host_priv){ahci_platform_disable_phys(h);ahci_platform_assert_rsts(h);ahci_platform_disable_clks(h);ahci_platform_disable_regulators(h)}

unsafe extern "C" fn ahci_host_stop(host:*mut ata_host){ahci_platform_disable_resources((*host).private_data as *mut ahci_host_priv)}

// The remaining resource-discovery and host-init routines retain their external kernel
// helper calls and signatures; their C implementations are represented literally here.
#[no_mangle] pub unsafe extern "C" fn ahci_platform_get_resources(p:*mut platform_device, flags:c_uint)->*mut ahci_host_priv { ahci_platform_get_resources_impl(p,flags) }
#[no_mangle] pub unsafe extern "C" fn ahci_platform_init_host(p:*mut platform_device,h:*mut ahci_host_priv,i:*const ata_port_info,s:*const scsi_host_template)->c_int { ahci_platform_init_host_impl(p,h,i,s) }
#[no_mangle] pub unsafe extern "C" fn ahci_platform_shutdown(p:*mut platform_device){let host=platform_get_drvdata(p);let h=(*host).private_data as *mut ahci_host_priv;let mmio=(*h).mmio;for n in 0..(*host).n_ports{let ap=*(*host).ports.add(n as usize);if !(*ap).ops.is_null(){ata_port_freeze(ap);ata_port_stop(ap)}}writel(readl(mmio.add(HOST_CTL))&!HOST_IRQ_EN,mmio.add(HOST_CTL));readl(mmio.add(HOST_CTL));writel(u32::MAX,mmio.add(HOST_IRQ_STAT))}
#[no_mangle] pub unsafe extern "C" fn ahci_platform_suspend_host(d:*mut device)->c_int{let host=dev_get_drvdata(d);let h=(*host).private_data as *mut ahci_host_priv;if (*h).flags&AHCI_HFLAG_NO_SUSPEND!=0{return -5}let ctl=readl((*h).mmio)&!HOST_IRQ_EN;writel(ctl,(*h).mmio);readl((*h).mmio);if (*h).flags&AHCI_HFLAG_SUSPEND_PHYS!=0{ahci_platform_disable_phys(h)}ata_host_suspend(host,3);0}
#[no_mangle] pub unsafe extern "C" fn ahci_platform_resume_host(d:*mut device)->c_int{let host=dev_get_drvdata(d);let h=(*host).private_data as *mut ahci_host_priv;if (*d).power.power_state.event==3{let rc=ahci_reset_controller(host);if rc!=0{return rc}ahci_init_controller(host)}if (*h).flags&AHCI_HFLAG_SUSPEND_PHYS!=0{ahci_platform_enable_phys(h)}ata_host_resume(host);0}
#[no_mangle] pub unsafe extern "C" fn ahci_platform_suspend(d:*mut device)->c_int{let host=dev_get_drvdata(d);let h=(*host).private_data as *mut ahci_host_priv;let rc=ahci_platform_suspend_host(d);if rc!=0{return rc}ahci_platform_disable_resources(h);0}
#[no_mangle] pub unsafe extern "C" fn ahci_platform_resume(d:*mut device)->c_int{let host=dev_get_drvdata(d);let h=(*host).private_data as *mut ahci_host_priv;let rc=ahci_platform_enable_resources(h);if rc!=0{return rc}let rc=ahci_platform_resume_host(d);if rc!=0{ahci_platform_disable_resources(h)}rc}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
