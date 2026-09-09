// SPDX-License-Identifier: GPL-2.0-or-later
/* Powermac setup and early boot code plus other random bits. */
/* Translated from setup.c; declarations supplied by the surrounding kernel are external. */

static mut HAS_L2CACHE: i32 = 0;
pub static mut pmac_newworld: i32 = 0;
static mut current_root_goodness: i32 = -1;

const DEFAULT_ROOT_DEVICE: dev_t = MKDEV(SCSI_DISK0_MAJOR, 1);

pub static mut sys_ctrler: sys_ctrler_t = SYS_CTRLER_UNKNOWN;

unsafe fn pmac_show_cpuinfo(m: *mut seq_file) {
    let mut np: *mut device_node;
    let mut pp: *const c_char;
    let mut plen: i32 = 0;
    let mbmodel = pmac_call_feature(PMAC_FTR_GET_MB_INFO, core::ptr::null_mut(), PMAC_MB_INFO_MODEL, 0);
    let mbflags = pmac_call_feature(PMAC_FTR_GET_MB_INFO, core::ptr::null_mut(), PMAC_MB_INFO_FLAGS, 0) as u32;
    let mut mbname: *mut c_char = core::ptr::null_mut();
    if pmac_call_feature(PMAC_FTR_GET_MB_INFO, core::ptr::null_mut(), PMAC_MB_INFO_NAME,
                         (&mut mbname as *mut _ as long)) != 0 {
        mbname = b"Unknown\0".as_ptr() as *mut c_char;
    }
    seq_printf(m, b"machine\t\t: \0".as_ptr() as *const c_char);
    np = of_find_node_by_path(b"/\0".as_ptr() as *const c_char);
    if !np.is_null() {
        pp = of_get_property(np, b"model\0".as_ptr() as *const c_char, core::ptr::null_mut());
        if !pp.is_null() { seq_printf(m, b"%s\n\0".as_ptr() as *const c_char, pp); }
        else { seq_printf(m, b"PowerMac\n\0".as_ptr() as *const c_char); }
        pp = of_get_property(np, b"compatible\0".as_ptr() as *const c_char, &mut plen);
        if !pp.is_null() {
            seq_printf(m, b"motherboard\t:\0".as_ptr() as *const c_char);
            while plen > 0 { let l = strlen(pp) + 1; seq_printf(m, b" %s\0".as_ptr() as *const c_char, pp); plen -= l as i32; pp = pp.add(l); }
            seq_printf(m, b"\n\0".as_ptr() as *const c_char);
        }
        of_node_put(np);
    } else { seq_printf(m, b"PowerMac\n\0".as_ptr() as *const c_char); }
    seq_printf(m, b"detected as\t: %d (%s)\n\0".as_ptr() as *const c_char, mbmodel, mbname);
    seq_printf(m, b"pmac flags\t: %08x\n\0".as_ptr() as *const c_char, mbflags);
    np = of_find_node_by_name(core::ptr::null_mut(), b"l2-cache\0".as_ptr() as *const c_char);
    if np.is_null() { np = of_find_node_by_type(core::ptr::null_mut(), b"cache\0".as_ptr() as *const c_char); }
    if !np.is_null() {
        let ic = of_get_property(np, b"i-cache-size\0".as_ptr() as *const c_char, core::ptr::null_mut()) as *const u32;
        let dc = of_get_property(np, b"d-cache-size\0".as_ptr() as *const c_char, core::ptr::null_mut()) as *const u32;
        seq_printf(m, b"L2 cache\t:\0".as_ptr() as *const c_char); HAS_L2CACHE = 1;
        if of_property_read_bool(np, b"cache-unified\0".as_ptr() as *const c_char) && !dc.is_null() { seq_printf(m, b" %dK unified\0".as_ptr() as *const c_char, *dc / 1024); }
        else { if !ic.is_null() { seq_printf(m, b" %dK instruction\0".as_ptr() as *const c_char, *ic / 1024); } if !dc.is_null() { seq_printf(m, b"%s %dK data\0".as_ptr() as *const c_char, if !ic.is_null() { b" +\0".as_ptr() } else { b"\0".as_ptr() }, *dc / 1024); } }
        pp = of_get_property(np, b"ram-type\0".as_ptr() as *const c_char, core::ptr::null_mut()); if !pp.is_null() { seq_printf(m, b" %s\0".as_ptr() as *const c_char, pp); }
        seq_printf(m, b"\n\0".as_ptr() as *const c_char); of_node_put(np);
    }
    seq_printf(m, b"pmac-generation\t: %s\n\0".as_ptr() as *const c_char, if pmac_newworld != 0 { b"NewWorld\0".as_ptr() } else { b"OldWorld\0".as_ptr() });
}

#[cfg(not(CONFIG_ADB_CUDA))]
unsafe fn find_via_cuda() -> i32 { let dn = of_find_node_by_name(core::ptr::null_mut(), b"via-cuda\0".as_ptr() as *const c_char); if dn.is_null() { return 0; } of_node_put(dn); printk(b"WARNING ! Your machine is CUDA-based but your kernel\n\0".as_ptr() as *const c_char); printk(b"          wasn't compiled with CONFIG_ADB_CUDA option !\n\0".as_ptr() as *const c_char); 0 }
#[cfg(not(CONFIG_ADB_PMU))]
unsafe fn find_via_pmu() -> i32 { let dn = of_find_node_by_name(core::ptr::null_mut(), b"via-pmu\0".as_ptr() as *const c_char); if dn.is_null() { return 0; } of_node_put(dn); printk(b"WARNING ! Your machine is PMU-based but your kernel\n\0".as_ptr() as *const c_char); printk(b"          wasn't compiled with CONFIG_ADB_PMU option !\n\0".as_ptr() as *const c_char); 0 }
#[cfg(not(CONFIG_PMAC_SMU))]
unsafe fn smu_init() -> i32 { 0 }

unsafe fn pmac_setup_arch() {
    let pvr = PVR_VER(mfspr(SPRN_PVR));
    loops_per_jiffy = 50000000 / HZ;
    for_each_of_cpu_node!(cpu => { let fp = of_get_property(cpu, b"clock-frequency\0".as_ptr() as *const c_char, core::ptr::null_mut()) as *const i32; if !fp.is_null() { loops_per_jiffy = if pvr >= 0x30 && pvr < 0x80 { *fp / (3 * HZ) } else if pvr == 4 || pvr >= 8 { *fp / HZ } else { *fp / (2 * HZ) }; of_node_put(cpu); break; } });
    let ic = of_find_node_with_property(core::ptr::null_mut(), b"interrupt-controller\0".as_ptr() as *const c_char); if !ic.is_null() { pmac_newworld = 1; of_node_put(ic); }
    find_via_cuda(); find_via_pmu(); smu_init();
}

static mut initializing: i32 = 1;
unsafe fn pmac_late_init() -> i32 { initializing = 0; 0 }

pub unsafe fn note_bootable_part(dev: dev_t, part: i32, goodness: i32) {
    if initializing == 0 { return; }
    if goodness <= current_root_goodness && ROOT_DEV != DEFAULT_ROOT_DEVICE { return; }
    let p = strstr(boot_command_line, b"root=\0".as_ptr() as *const c_char);
    if !p.is_null() && (p == boot_command_line || *p.offset(-1) == b' ' as c_char) { return; }
    ROOT_DEV = dev + part as dev_t; current_root_goodness = goodness;
}

unsafe fn pmac_restart(_cmd: *mut c_char) -> ! { match sys_ctrler { SYS_CTRLER_CUDA => cuda_restart(), SYS_CTRLER_PMU => pmu_restart(), SYS_CTRLER_SMU => smu_restart(), _ => {} } loop {} }
unsafe fn pmac_power_off() -> ! { match sys_ctrler { SYS_CTRLER_CUDA => cuda_shutdown(), SYS_CTRLER_PMU => pmu_shutdown(), SYS_CTRLER_SMU => smu_shutdown(), _ => {} } loop {} }
unsafe fn pmac_halt() -> ! { pmac_power_off() }

unsafe fn pmac_init() {
    if !strstr(boot_command_line, b"btextdbg\0".as_ptr() as *const c_char).is_null() { udbg_adb_init_early(); register_early_udbg_console(); }
    pmac_feature_init();
    udbg_scc_init(!strstr(boot_command_line, b"sccdbg\0".as_ptr() as *const c_char).is_null());
    udbg_adb_init(!strstr(boot_command_line, b"btextdbg\0".as_ptr() as *const c_char).is_null());
}

unsafe fn pmac_declare_of_platform_devices() -> i32 {
    let mut np = of_find_node_by_name(core::ptr::null_mut(), b"valkyrie\0".as_ptr() as *const c_char); if !np.is_null() { of_platform_device_create(np, b"valkyrie\0".as_ptr() as *const c_char, core::ptr::null_mut()); of_node_put(np); }
    np = of_find_node_by_name(core::ptr::null_mut(), b"platinum\0".as_ptr() as *const c_char); if !np.is_null() { of_platform_device_create(np, b"platinum\0".as_ptr() as *const c_char, core::ptr::null_mut()); of_node_put(np); }
    np = of_find_node_by_type(core::ptr::null_mut(), b"smu\0".as_ptr() as *const c_char); if !np.is_null() { of_platform_device_create(np, b"smu\0".as_ptr() as *const c_char, core::ptr::null_mut()); of_node_put(np); }
    np = of_find_node_by_type(core::ptr::null_mut(), b"fcu\0".as_ptr() as *const c_char); if np.is_null() { np = of_find_node_by_path(b"/u3@0,f8000000/i2c@f8001000/fan@15e\0".as_ptr() as *const c_char); } if !np.is_null() { of_platform_device_create(np, b"temperature\0".as_ptr() as *const c_char, core::ptr::null_mut()); of_node_put(np); } 0
}

#[cfg(CONFIG_SERIAL_PMACZILOG_CONSOLE)]
unsafe fn check_pmac_serial_console() -> i32 {
    let mut prom_stdout: *mut device_node = core::ptr::null_mut();
    let mut offset = 0;
    let devname: *mut c_char = if cfg!(CONFIG_SERIAL_PMACZILOG_TTYS) { b"ttyS\0".as_ptr() as *mut c_char } else { b"ttyPZ\0".as_ptr() as *mut c_char };
    if !strstr(boot_command_line, b"console=\0".as_ptr() as *const c_char).is_null() { return -EBUSY; }
    if of_chosen.is_null() { return -ENODEV; }
    let name = of_get_property(of_chosen, b"linux,stdout-path\0".as_ptr() as *const c_char, core::ptr::null_mut());
    if name.is_null() { return -ENODEV; }
    prom_stdout = of_find_node_by_path(name); if prom_stdout.is_null() { return -ENODEV; }
    if of_node_name_eq(prom_stdout, b"ch-a\0".as_ptr() as *const c_char) { offset = 0; }
    else if of_node_name_eq(prom_stdout, b"ch-b\0".as_ptr() as *const c_char) { offset = 1; }
    else { of_node_put(prom_stdout); return -ENODEV; }
    of_node_put(prom_stdout); add_preferred_console(devname, offset, core::ptr::null())
}

unsafe fn pmac_probe() -> i32 {
    if !of_machine_is_compatible(b"Power Macintosh\0".as_ptr() as *const c_char) && !of_machine_is_compatible(b"MacRISC\0".as_ptr() as *const c_char) { return 0; }
    DMA_MODE_READ = 1; DMA_MODE_WRITE = 2; pm_power_off = Some(pmac_power_off); pmac_init(); 1
}

/* The C define_machine(powermac) registers this platform machine descriptor. */
#[repr(C)]
pub struct machine_desc {
    pub name: *const c_char,
    pub probe: Option<unsafe fn() -> i32>,
    pub setup_arch: Option<unsafe fn()>,
    pub discover_phbs: Option<unsafe fn()>,
    pub show_cpuinfo: Option<unsafe fn(*mut seq_file)>,
    pub init_IRQ: Option<unsafe fn()>,
    pub get_irq: Option<unsafe fn() -> i32>,
    pub pci_irq_fixup: Option<unsafe fn()>,
    pub restart: Option<unsafe fn(*mut c_char) -> !>,
    pub halt: Option<unsafe fn() -> !>,
}

#[no_mangle]
pub static mut powermac: machine_desc = machine_desc {
    name: b"PowerMac\0".as_ptr() as *const c_char,
    probe: Some(pmac_probe), setup_arch: Some(pmac_setup_arch), discover_phbs: Some(pmac_pci_init),
    show_cpuinfo: Some(pmac_show_cpuinfo), init_IRQ: Some(pmac_pic_init), get_irq: None,
    pci_irq_fixup: Some(pmac_pci_irq_fixup), restart: Some(pmac_restart), halt: Some(pmac_halt),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
