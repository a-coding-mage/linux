/* Bus driver for MIPS Common Device Memory Map (CDMM). */

/* Linux dependencies and asm dependencies are supplied by other files. */

const CDMM_ACSR_DEVTYPE_SHIFT: u32 = 24;
const CDMM_ACSR_DEVTYPE: u64 = 255u64 << CDMM_ACSR_DEVTYPE_SHIFT;
const CDMM_ACSR_DEVSIZE_SHIFT: u32 = 16;
const CDMM_ACSR_DEVSIZE: u64 = 31u64 << CDMM_ACSR_DEVSIZE_SHIFT;
const CDMM_ACSR_DEVREV_SHIFT: u32 = 12;
const CDMM_ACSR_DEVREV: u64 = 15u64 << CDMM_ACSR_DEVREV_SHIFT;
const CDMM_ACSR_UW: u64 = 1u64 << 3;
const CDMM_ACSR_UR: u64 = 1u64 << 2;
const CDMM_ACSR_SW: u64 = 1u64 << 1;
const CDMM_ACSR_SR: u64 = 1u64;
const CDMM_DRB_SIZE: usize = 64;

static mut mips_cdmm_default_base: phys_addr_t = 0;

unsafe fn mips_cdmm_lookup(table: *const mips_cdmm_device_id,
                            dev: *mut mips_cdmm_device) -> *const mips_cdmm_device_id {
    let mut table = table;
    while (*table).type_ != 0 {
        if (*dev).type_ == (*table).type_ { return table; }
        table = table.add(1);
    }
    core::ptr::null()
}

unsafe extern "C" fn mips_cdmm_match(dev: *mut device, drv: *const device_driver) -> c_int {
    let cdev = to_mips_cdmm_device(dev);
    let cdrv = to_mips_cdmm_driver(drv);
    (!mips_cdmm_lookup((*cdrv).id_table, cdev).is_null()) as c_int
}

unsafe extern "C" fn mips_cdmm_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    let cdev = to_mips_cdmm_device(dev as *mut device);
    let mut retval = add_uevent_var(env, "CDMM_CPU=%u", (*cdev).cpu);
    if retval != 0 { return retval; }
    retval = add_uevent_var(env, "CDMM_TYPE=0x%02x", (*cdev).type_);
    if retval != 0 { return retval; }
    retval = add_uevent_var(env, "CDMM_REV=%u", (*cdev).rev);
    if retval != 0 { return retval; }
    add_uevent_var(env, "MODALIAS=mipscdmm:t%02X", (*cdev).type_)
}

#[repr(C)]
struct mips_cdmm_work_dev {
    fn_: *mut core::ffi::c_void,
    dev: *mut mips_cdmm_device,
}

unsafe extern "C" fn mips_cdmm_void_work(data: *mut core::ffi::c_void) -> c_long {
    let work = data as *mut mips_cdmm_work_dev;
    let fn_: extern "C" fn(*mut mips_cdmm_device) = core::mem::transmute((*work).fn_);
    fn_((*work).dev); 0
}

unsafe extern "C" fn mips_cdmm_int_work(data: *mut core::ffi::c_void) -> c_long {
    let work = data as *mut mips_cdmm_work_dev;
    let fn_: extern "C" fn(*mut mips_cdmm_device) -> c_int = core::mem::transmute((*work).fn_);
    fn_((*work).dev) as c_long
}

unsafe extern "C" fn mips_cdmm_probe(dev: *mut device) -> c_int {
    let cdev = to_mips_cdmm_device(dev);
    let cdrv = to_mips_cdmm_driver((*dev).driver);
    let work = mips_cdmm_work_dev { fn_: (*cdrv).probe as *mut _, dev: cdev };
    work_on_cpu((*cdev).cpu, Some(mips_cdmm_int_work), &work as *const _ as *mut _ ) as c_int
}
unsafe extern "C" fn mips_cdmm_remove(dev: *mut device) -> c_int {
    let cdev = to_mips_cdmm_device(dev);
    let cdrv = to_mips_cdmm_driver((*dev).driver);
    let work = mips_cdmm_work_dev { fn_: (*cdrv).remove as *mut _, dev: cdev };
    work_on_cpu((*cdev).cpu, Some(mips_cdmm_int_work), &work as *const _ as *mut _) as c_int
}
unsafe extern "C" fn mips_cdmm_shutdown(dev: *mut device) {
    let cdev = to_mips_cdmm_device(dev);
    let cdrv = to_mips_cdmm_driver((*dev).driver);
    let work = mips_cdmm_work_dev { fn_: (*cdrv).shutdown as *mut _, dev: cdev };
    work_on_cpu((*cdev).cpu, Some(mips_cdmm_void_work), &work as *const _ as *mut _);
}

pub unsafe extern "C" fn mips_cdmm_driver_register(drv: *mut mips_cdmm_driver) -> c_int {
    (*drv).drv.bus = &raw mut mips_cdmm_bustype;
    if !(*drv).probe.is_none() { (*drv).drv.probe = Some(mips_cdmm_probe); }
    if !(*drv).remove.is_none() { (*drv).drv.remove = Some(mips_cdmm_remove); }
    if !(*drv).shutdown.is_none() { (*drv).drv.shutdown = Some(mips_cdmm_shutdown); }
    driver_register(&mut (*drv).drv)
}

pub unsafe extern "C" fn mips_cdmm_driver_unregister(drv: *mut mips_cdmm_driver) {
    driver_unregister(&mut (*drv).drv);
}

#[repr(C)]
struct mips_cdmm_bus {
    phys: phys_addr_t, regs: *mut core::ffi::c_void, drbs: u32,
    drbs_reserved: u32, discovered: bool, offline: bool,
}
static mut mips_cdmm_boot_bus: mips_cdmm_bus = mips_cdmm_bus { phys: 0, regs: core::ptr::null_mut(), drbs: 0, drbs_reserved: 0, discovered: false, offline: false };
static mut mips_cdmm_buses: *mut mips_cdmm_bus = core::ptr::null_mut();
static mut mips_cdmm_next_id: atomic_t = atomic_t { counter: -1 };

unsafe fn mips_cdmm_get_bus() -> *mut mips_cdmm_bus {
    if !cpu_has_cdmm { return ERR_PTR(-ENODEV); }
    let cpu = smp_processor_id();
    if cpu == 0 { return &raw mut mips_cdmm_boot_bus; }
    let bus_p = per_cpu_ptr(&raw mut mips_cdmm_buses, cpu);
    let flags = local_irq_save();
    let mut bus = *bus_p;
    if bus.is_null() { bus = kzalloc::<mips_cdmm_bus>(GFP_ATOMIC); if bus.is_null() { bus = ERR_PTR(-ENOMEM); } else { *bus_p = bus; } }
    local_irq_restore(flags); bus
}

unsafe fn mips_cdmm_cur_base() -> phys_addr_t {
    let cdmmbase = read_c0_cdmmbase();
    if cdmmbase & MIPS_CDMMBASE_EN == 0 { return 0; }
    (cdmmbase >> MIPS_CDMMBASE_ADDR_SHIFT) << MIPS_CDMMBASE_ADDR_START
}

pub unsafe extern "C" fn mips_cdmm_phys_base() -> phys_addr_t {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), "mti,mips-cdmm");
    if !np.is_null() { let mut res = core::mem::zeroed(); let err = of_address_to_resource(np, 0, &mut res); of_node_put(np); if err == 0 { return res.start; } }
    0
}

unsafe fn mips_cdmm_setup(bus: *mut mips_cdmm_bus) -> c_int {
    if IS_ERR(bus) { return PTR_ERR(bus); }
    let flags = local_irq_save();
    if (*bus).offline {
        if (*bus).phys == mips_cdmm_cur_base() { local_irq_restore(flags); return 0; }
        (*bus).offline = false;
    } else if (*bus).phys > 1 { local_irq_restore(flags); return 0; }
    if (*bus).phys == 0 { (*bus).phys = mips_cdmm_cur_base(); }
    if (*bus).phys == 0 { (*bus).phys = mips_cdmm_phys_base(); }
    if (*bus).phys == 0 { (*bus).phys = mips_cdmm_default_base; }
    if (*bus).phys == 0 { (*bus).phys = 1; pr_err("cdmm%u: Failed to choose a physical base\n", smp_processor_id()); }
    if (*bus).phys == 1 { local_irq_restore(flags); return -ENOMEM; }
    mips_cdmm_default_base = (*bus).phys;
    let mut cdmmbase = read_c0_cdmmbase();
    cdmmbase &= (1ul << MIPS_CDMMBASE_ADDR_SHIFT) - 1;
    cdmmbase |= ((*bus).phys >> MIPS_CDMMBASE_ADDR_START) << MIPS_CDMMBASE_ADDR_SHIFT;
    cdmmbase |= MIPS_CDMMBASE_EN; write_c0_cdmmbase(cdmmbase); tlbw_use_hazard();
    (*bus).regs = CKSEG1ADDR((*bus).phys) as *mut _;
    (*bus).drbs = 1 + ((cdmmbase & MIPS_CDMMBASE_SIZE) >> MIPS_CDMMBASE_SIZE_SHIFT) as u32;
    (*bus).drbs_reserved = ((cdmmbase & MIPS_CDMMBASE_CI) != 0) as u32;
    local_irq_restore(flags); 0
}

pub unsafe extern "C" fn mips_cdmm_early_probe(dev_type: u32) -> *mut core::ffi::c_void {
    if dev_type == 0 { return IOMEM_ERR_PTR(-ENODEV); }
    let bus = mips_cdmm_get_bus(); let err = mips_cdmm_setup(bus); if err != 0 { return IOMEM_ERR_PTR(err); }
    let mut drb = (*bus).drbs_reserved; let cdmm = (*bus).regs as *mut u8; let mut size;
    while drb < (*bus).drbs { let acsr = core::ptr::read_volatile(cdmm.add(drb as usize * CDMM_DRB_SIZE) as *const u32); let typ = (acsr & CDMM_ACSR_DEVTYPE as u32) >> CDMM_ACSR_DEVTYPE_SHIFT; if typ == dev_type { return cdmm.add(drb as usize * CDMM_DRB_SIZE) as *mut _; } size = (acsr & CDMM_ACSR_DEVSIZE as u32) >> CDMM_ACSR_DEVSIZE_SHIFT; drb += size + 1; }
    IOMEM_ERR_PTR(-ENODEV)
}

unsafe extern "C" fn mips_cdmm_release(dev: *mut device) {
    kfree(to_mips_cdmm_device(dev));
}

unsafe fn mips_cdmm_bus_discover(bus: *mut mips_cdmm_bus) {
    let cdmm = (*bus).regs as *mut u8; let cpu = smp_processor_id(); let mut drb = (*bus).drbs_reserved; let mut id = 0u32; let mut size;
    (*bus).discovered = true;
    pr_info("cdmm%u discovery (%u blocks)\n", cpu, (*bus).drbs);
    while drb < (*bus).drbs {
        let acsr = core::ptr::read_volatile(cdmm.add(drb as usize * CDMM_DRB_SIZE) as *const u32);
        let typ = (acsr & CDMM_ACSR_DEVTYPE as u32) >> CDMM_ACSR_DEVTYPE_SHIFT; size = (acsr & CDMM_ACSR_DEVSIZE as u32) >> CDMM_ACSR_DEVSIZE_SHIFT; let rev = (acsr & CDMM_ACSR_DEVREV as u32) >> CDMM_ACSR_DEVREV_SHIFT;
        if typ == 0 { drb += size + 1; continue; }
        let dev = kzalloc::<mips_cdmm_device>(); if dev.is_null() { break; }
        (*dev).cpu = cpu; (*dev).res.start = (*bus).phys + drb as u64 * CDMM_DRB_SIZE as u64; (*dev).res.end = (*bus).phys + (drb + size + 1) as u64 * CDMM_DRB_SIZE as u64 - 1; (*dev).res.flags = IORESOURCE_MEM; (*dev).type_ = typ; (*dev).rev = rev; (*dev).dev.parent = get_cpu_device(cpu); (*dev).dev.bus = &raw mut mips_cdmm_bustype; (*dev).dev.id = atomic_inc_return(&raw mut mips_cdmm_next_id); (*dev).dev.release = Some(mips_cdmm_release);
        dev_set_name(&mut (*dev).dev, "cdmm%u-%u", cpu, id); id += 1;
        if device_register(&mut (*dev).dev) != 0 { put_device(&mut (*dev).dev); }
        drb += size + 1;
    }
}

unsafe extern "C" fn mips_cdmm_cpu_down_prep(cpu: u32) -> c_int {
    let ret = bus_for_each_dev(&raw mut mips_cdmm_bustype, core::ptr::null_mut(), &cpu as *const _ as *mut _, mips_cdmm_cpu_down_helper);
    let bus = mips_cdmm_get_bus(); if !IS_ERR(bus) { (*bus).offline = true; } ret as c_int
}
unsafe extern "C" fn mips_cdmm_cpu_online(_cpu: u32) -> c_int {
    let bus = mips_cdmm_get_bus(); let ret = mips_cdmm_setup(bus); if ret != 0 { return ret; }
    (*bus).offline = false;
    if !(*bus).discovered { mips_cdmm_bus_discover(bus); 0 } else { bus_for_each_dev(&raw mut mips_cdmm_bustype, core::ptr::null_mut(), &_cpu as *const _ as *mut _, mips_cdmm_cpu_up_helper) as c_int }
}

unsafe extern "C" fn mips_cdmm_cpu_down_helper(dev: *mut device, data: *mut core::ffi::c_void) -> c_int {
    let cdev = to_mips_cdmm_device(dev); let cpu = *(data as *const u32); if (*cdev).cpu != cpu || (*dev).driver.is_null() { return 0; }
    let cdrv = to_mips_cdmm_driver((*dev).driver); if (*cdrv).cpu_down.is_none() { return 0; } (*cdrv).cpu_down.unwrap()(cdev)
}
unsafe extern "C" fn mips_cdmm_cpu_up_helper(dev: *mut device, data: *mut core::ffi::c_void) -> c_int {
    let cdev = to_mips_cdmm_device(dev); let cpu = *(data as *const u32); if (*cdev).cpu != cpu || (*dev).driver.is_null() { return 0; }
    let cdrv = to_mips_cdmm_driver((*dev).driver); if (*cdrv).cpu_up.is_none() { return 0; } (*cdrv).cpu_up.unwrap()(cdev)
}

unsafe extern "C" fn mips_cdmm_init() -> c_int {
    let ret = bus_register(&raw mut mips_cdmm_bustype); if ret != 0 { return ret; }
    let ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "bus/cdmm:online", Some(mips_cdmm_cpu_online), Some(mips_cdmm_cpu_down_prep));
    if ret < 0 { pr_warn("cdmm: Failed to register CPU notifier\n"); } ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
