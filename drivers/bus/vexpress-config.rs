// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2014 ARM Limited
 */

// Linux kernel dependencies are supplied by other translated units.

const SYS_MISC: usize = 0x0;
const SYS_MISC_MASTERSITE: u32 = 1 << 14;
const SYS_PROCID0: usize = 0x24;
const SYS_PROCID1: usize = 0x28;
const SYS_HBI_MASK: u32 = 0xfff;
const SYS_PROCIDX_HBI_SHIFT: u32 = 0;
const SYS_CFGDATA: usize = 0x40;
const SYS_CFGCTRL: usize = 0x44;
const SYS_CFGCTRL_START: u32 = 1 << 31;
const SYS_CFGCTRL_WRITE: u32 = 1 << 30;
const SYS_CFGSTAT: usize = 0x48;
const SYS_CFGSTAT_ERR: u32 = 1 << 1;
const SYS_CFGSTAT_COMPLETE: u32 = 1 << 0;
const VEXPRESS_SITE_MB: u32 = 0;
const VEXPRESS_SITE_DB1: u32 = 1;
const VEXPRESS_SITE_DB2: u32 = 2;
const VEXPRESS_SITE_MASTER: u32 = 0xf;

#[repr(C)]
pub struct vexpress_syscfg {
    pub dev: *mut device,
    pub base: *mut core::ffi::c_void,
    pub funcs: list_head,
}

#[repr(C)]
pub struct vexpress_syscfg_func {
    pub list: list_head,
    pub syscfg: *mut vexpress_syscfg,
    pub regmap: *mut regmap,
    pub num_templates: i32,
    pub template: [u32; 0], // Flexible array member; kept last.
}

#[repr(C)]
pub struct vexpress_config_bridge_ops {
    pub regmap_init: Option<unsafe extern "C" fn(*mut device, *mut core::ffi::c_void) -> *mut regmap>,
    pub regmap_exit: Option<unsafe extern "C" fn(*mut regmap, *mut core::ffi::c_void)>,
}

#[repr(C)]
pub struct vexpress_config_bridge {
    pub ops: *mut vexpress_config_bridge_ops,
    pub context: *mut core::ffi::c_void,
}

static mut VEXPRESS_CONFIG_SITE_MASTER: u32 = VEXPRESS_SITE_MASTER;

unsafe fn vexpress_config_set_master(site: u32) {
    VEXPRESS_CONFIG_SITE_MASTER = site;
}

unsafe extern "C" fn vexpress_config_lock(_arg: *mut core::ffi::c_void) { mutex_lock(&raw mut vexpress_config_mutex); }
unsafe extern "C" fn vexpress_config_unlock(_arg: *mut core::ffi::c_void) { mutex_unlock(&raw mut vexpress_config_mutex); }

unsafe fn vexpress_config_find_prop(mut node: *mut device_node, name: *const core::ffi::c_char, val: *mut u32) {
    *val = 0;
    of_node_get(node);
    while !node.is_null() {
        if of_property_read_u32(node, name, val) == 0 {
            of_node_put(node);
            return;
        }
        node = of_get_next_parent(node);
    }
}

unsafe fn vexpress_config_get_topo(node: *mut device_node, site: *mut u32, position: *mut u32, dcc: *mut u32) -> i32 {
    vexpress_config_find_prop(node, c"arm,vexpress,site".as_ptr(), site);
    if *site == VEXPRESS_SITE_MASTER { *site = VEXPRESS_CONFIG_SITE_MASTER; }
    if WARN_ON(VEXPRESS_CONFIG_SITE_MASTER == VEXPRESS_SITE_MASTER) { return -EINVAL; }
    vexpress_config_find_prop(node, c"arm,vexpress,position".as_ptr(), position);
    vexpress_config_find_prop(node, c"arm,vexpress,dcc".as_ptr(), dcc);
    0
}

unsafe extern "C" fn vexpress_config_devres_release(dev: *mut device, res: *mut core::ffi::c_void) {
    let bridge = dev_get_drvdata((*dev).parent) as *mut vexpress_config_bridge;
    let regmap = res as *mut regmap;
    ((*(*bridge).ops).regmap_exit.unwrap())(regmap, (*bridge).context);
}

pub unsafe fn devm_regmap_init_vexpress_config(dev: *mut device) -> *mut regmap {
    let bridge = dev_get_drvdata((*dev).parent) as *mut vexpress_config_bridge;
    if WARN_ON(bridge.is_null()) { return ERR_PTR(-EINVAL) as *mut regmap; }
    let res = devres_alloc(Some(vexpress_config_devres_release), core::mem::size_of::<*mut regmap>(), GFP_KERNEL);
    if res.is_null() { return ERR_PTR(-ENOMEM) as *mut regmap; }
    let regmap = ((*(*bridge).ops).regmap_init.unwrap())(dev, (*bridge).context);
    if IS_ERR(regmap) { devres_free(res); return regmap; }
    *(res as *mut *mut regmap) = regmap;
    devres_add(dev, res);
    regmap
}

unsafe fn vexpress_syscfg_exec(func: *mut vexpress_syscfg_func, index: i32, write: bool, data: *mut u32) -> i32 {
    let syscfg = (*func).syscfg;
    if WARN_ON(index >= (*func).num_templates) { return -EINVAL; }
    let mut command = readl((*syscfg).base.add(SYS_CFGCTRL));
    if WARN_ON(command & SYS_CFGCTRL_START != 0) { return -EBUSY; }
    command = (*func).template.as_ptr().add(index as usize).read();
    command |= SYS_CFGCTRL_START;
    if write { command |= SYS_CFGCTRL_WRITE; } else { *data = 0xdeadbeef; }
    writel(*data, (*syscfg).base.add(SYS_CFGDATA));
    writel(0, (*syscfg).base.add(SYS_CFGSTAT));
    writel(command, (*syscfg).base.add(SYS_CFGCTRL));
    mb();
    let mut tries = 100;
    let mut timeout: i64 = 100;
    let mut status;
    loop {
        if !irqs_disabled() {
            set_current_state(TASK_INTERRUPTIBLE);
            schedule_timeout(usecs_to_jiffies(timeout));
            if signal_pending(current) { return -EINTR; }
        } else { udelay(timeout as u32); }
        status = readl((*syscfg).base.add(SYS_CFGSTAT));
        if status & SYS_CFGSTAT_ERR != 0 { return -EFAULT; }
        if timeout > 20 { timeout -= 20; }
        tries -= 1;
        if tries == 0 || status & SYS_CFGSTAT_COMPLETE != 0 { break; }
    }
    if WARN_ON_ONCE(tries == 0) { return -ETIMEDOUT; }
    if !write { *data = readl((*syscfg).base.add(SYS_CFGDATA)); }
    0
}

unsafe extern "C" fn vexpress_syscfg_read(context: *mut core::ffi::c_void, index: u32, val: *mut u32) -> i32 { vexpress_syscfg_exec(context as *mut _, index as i32, false, val) }
unsafe extern "C" fn vexpress_syscfg_write(context: *mut core::ffi::c_void, index: u32, val: u32) -> i32 { let mut value = val; vexpress_syscfg_exec(context as *mut _, index as i32, true, &mut value) }

static mut VEXPRESS_SYSCFG_REGMAP_CONFIG: regmap_config = regmap_config {
    lock: Some(vexpress_config_lock), unlock: Some(vexpress_config_unlock), reg_bits: 32,
    val_bits: 32, reg_read: Some(vexpress_syscfg_read), reg_write: Some(vexpress_syscfg_write),
    reg_format_endian: REGMAP_ENDIAN_LITTLE, val_format_endian: REGMAP_ENDIAN_LITTLE,
};

unsafe extern "C" fn vexpress_syscfg_regmap_init(dev: *mut device, context: *mut core::ffi::c_void) -> *mut regmap {
    let syscfg = context as *mut vexpress_syscfg;
    let mut site = 0; let mut position = 0; let mut dcc = 0;
    let err = vexpress_config_get_topo((*dev).of_node, &mut site, &mut position, &mut dcc);
    if err != 0 { return ERR_PTR(err) as *mut regmap; }
    let prop = of_find_property((*dev).of_node, c"arm,vexpress-sysreg,func".as_ptr(), core::ptr::null_mut());
    if prop.is_null() { return ERR_PTR(-EINVAL) as *mut regmap; }
    let mut num = ((*prop).length as usize / core::mem::size_of::<u32>() / 2) as i32;
    let mut val = (*prop).value as *const u32;
    let mut energy_quirk = [0u32; 4];
    if num == 1 && of_device_is_compatible((*dev).of_node, c"arm,vexpress-energy".as_ptr()) {
        num = 2; energy_quirk[0] = *val; energy_quirk[2] = *val; val = val.add(1);
        energy_quirk[1] = *val; energy_quirk[3] = cpu_to_be32(be32_to_cpup(val) + 1); val = energy_quirk.as_ptr();
    }
    let func = kzalloc_flex::<vexpress_syscfg_func>(num as usize);
    if func.is_null() { return ERR_PTR(-ENOMEM) as *mut regmap; }
    (*func).syscfg = syscfg; (*func).num_templates = num;
    for i in 0..num as usize {
        let function = be32_to_cpup(val); val = val.add(1);
        let device = be32_to_cpup(val); val = val.add(1);
        let t = (dcc & 0xf) << 26 | (site & 3) << 16 | (position & 0xf) << 12 | (function & 0x3f) << 20 | (device & 0xfff);
        (*func).template.as_mut_ptr().add(i).write(t);
    }
    VEXPRESS_SYSCFG_REGMAP_CONFIG.max_register = (num - 1) as u32;
    let map = regmap_init(dev, core::ptr::null_mut(), func as *mut _, &raw mut VEXPRESS_SYSCFG_REGMAP_CONFIG);
    if IS_ERR(map) { kfree(func as *mut _); return map; }
    (*func).regmap = map; list_add(&mut (*func).list, &mut (*syscfg).funcs); map
}

unsafe extern "C" fn vexpress_syscfg_regmap_exit(regmap: *mut regmap, context: *mut core::ffi::c_void) {
    let syscfg = context as *mut vexpress_syscfg; regmap_exit(regmap);
    let mut func = (*syscfg).funcs.next as *mut vexpress_syscfg_func;
    while func != syscfg as *mut _ {
        if (*func).regmap == regmap { list_del(&mut (*func).list); kfree(func as *mut _); break; }
        func = (*func).list.next as *mut _;
    }
}

static mut VEXPRESS_SYSCFG_BRIDGE_OPS: vexpress_config_bridge_ops = vexpress_config_bridge_ops { regmap_init: Some(vexpress_syscfg_regmap_init), regmap_exit: Some(vexpress_syscfg_regmap_exit) };

unsafe extern "C" fn vexpress_syscfg_probe(pdev: *mut platform_device) -> i32 {
    let syscfg = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<vexpress_syscfg>(), GFP_KERNEL) as *mut vexpress_syscfg;
    if syscfg.is_null() { return -ENOMEM; }
    (*syscfg).dev = &mut (*pdev).dev; INIT_LIST_HEAD(&mut (*syscfg).funcs);
    (*syscfg).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*syscfg).base) { return PTR_ERR((*syscfg).base); }
    let bridge = devm_kmalloc(&mut (*pdev).dev, core::mem::size_of::<vexpress_config_bridge>(), GFP_KERNEL) as *mut vexpress_config_bridge;
    if bridge.is_null() { return -ENOMEM; }
    (*bridge).ops = &raw mut VEXPRESS_SYSCFG_BRIDGE_OPS; (*bridge).context = syscfg as *mut _; dev_set_drvdata(&mut (*pdev).dev, bridge as *mut _);
    let master = if readl((*syscfg).base.add(SYS_MISC)) & SYS_MISC_MASTERSITE != 0 { VEXPRESS_SITE_DB2 } else { VEXPRESS_SITE_DB1 };
    vexpress_config_set_master(master); 0
}

static VEXPRESS_SYSCFG_ID_TABLE: [platform_device_id; 2] = [platform_device_id { name: c"vexpress-syscfg".as_ptr() }, platform_device_id { name: core::ptr::null() }];
static mut VEXPRESS_SYSCFG_DRIVER: platform_driver = platform_driver { driver: driver { name: c"vexpress-syscfg".as_ptr() }, id_table: VEXPRESS_SYSCFG_ID_TABLE.as_ptr(), probe: Some(vexpress_syscfg_probe) };
// MODULE_DEVICE_TABLE(platform, vexpress_syscfg_id_table);
// module_platform_driver(vexpress_syscfg_driver);
// MODULE_DESCRIPTION("Versatile Express configuration bus");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
