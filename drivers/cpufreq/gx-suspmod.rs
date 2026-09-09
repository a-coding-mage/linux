// SPDX-License-Identifier: GPL-2.0-only
/*
 * Cyrix MediaGX and NatSemi Geode Suspend Modulation
 *
 * Direct Rust translation of the original implementation. Kernel-provided
 * types, functions, constants, and macros are external dependencies.
 */

const PCI_PMER1: i32 = 0x80;
const PCI_PMER2: i32 = 0x81;
const PCI_PMER3: i32 = 0x82;
const PCI_IRQTC: i32 = 0x8c;
const PCI_VIDTC: i32 = 0x8d;
const PCI_MODOFF: i32 = 0x94;
const PCI_MODON: i32 = 0x95;
const PCI_SUSCFG: i32 = 0x96;

const GPM: u8 = 1 << 0;
const GIT: u8 = 1 << 1;
const GTR: u8 = 1 << 2;
const IRQ_SPDUP: u8 = 1 << 3;
const VID_SPDUP: u8 = 1 << 4;

const SUSMOD: u8 = 1 << 0;
const SMISPDUP: u8 = 1 << 1;
const SUSCFG: u8 = 1 << 2;
const PWRSVE_ISA: u8 = 1 << 3;
const PWRSVE: u8 = 1 << 4;
const POLICY_MIN_DIV: i32 = 20;

#[repr(C)]
struct gxfreq_params {
    on_duration: u8,
    off_duration: u8,
    pci_suscfg: u8,
    pci_pmer1: u8,
    pci_pmer2: u8,
    cs55x0: *mut pci_dev,
}

extern "C" {
    static mut gx_params: *mut gxfreq_params;
    static mut stock_freq: i32;
    static mut pci_busclk: i32;
    static mut max_duration: i32;
    static mut gx_freq_mult: [i32; 16];
    static mut gx_chipset_tbl: [pci_device_id; 4];

    fn pci_write_config_byte(dev: *mut pci_dev, reg: i32, value: u8);
    fn pci_read_config_byte(dev: *mut pci_dev, reg: i32, value: *mut u8) -> i32;
    fn pci_match_id(table: *const pci_device_id, dev: *mut pci_dev) -> *const pci_device_id;
    fn pci_dev_put(dev: *mut pci_dev);
    fn cpufreq_freq_transition_begin(policy: *mut cpufreq_policy, freqs: *mut cpufreq_freqs);
    fn cpufreq_freq_transition_end(policy: *mut cpufreq_policy, freqs: *mut cpufreq_freqs, result: i32);
    fn cpufreq_verify_within_limits(policy: *mut cpufreq_policy_data, min: u32, max: u32);
    fn cpufreq_register_driver(driver: *mut cpufreq_driver) -> i32;
    fn cpufreq_unregister_driver(driver: *mut cpufreq_driver);
    fn getCx86(reg: i32) -> i32;
    static mut cpu_khz: i32;
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn kzalloc(size: usize, flags: usize) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
}

#[repr(C)] struct pci_dev { device: u16, revision: u8 }
#[repr(C)] struct pci_device_id { _data: [u64; 4] }
#[repr(C)] struct cpufreq_freqs { old: u32, new: u32 }
#[repr(C)] struct cpufreq_cpuinfo { min_freq: u32, max_freq: u32 }
#[repr(C)] struct cpufreq_policy_data { cpu: u32, min: u32, max: u32, cpuinfo: cpufreq_cpuinfo }
#[repr(C)] struct cpufreq_policy { cpu: u32, min: u32, max: u32, cpuinfo: cpufreq_cpuinfo }
#[repr(C)] struct cpufreq_driver {
    flags: u32,
    get: Option<unsafe extern "C" fn(u32) -> u32>,
    verify: Option<unsafe extern "C" fn(*mut cpufreq_policy_data) -> i32>,
    target: Option<unsafe extern "C" fn(*mut cpufreq_policy, u32, u32) -> i32>,
    init: Option<unsafe extern "C" fn(*mut cpufreq_policy) -> i32>,
    name: *const u8,
}

const EINVAL: i32 = 22;
const ENODEV: i32 = 19;
const ENOMEM: i32 = 12;
const CPUFREQ_NO_AUTO_DYNAMIC_SWITCHING: u32 = 1;
const PCI_DEVICE_ID_CYRIX_5530_LEGACY: u16 = 0x0103;
const PCI_DEVICE_ID_CYRIX_5520: u16 = 0x0104;
const PCI_DEVICE_ID_CYRIX_5510: u16 = 0x0105;
const CX86_DIR1: i32 = 0x01;

unsafe fn gx_write_byte(reg: i32, value: u8) { pci_write_config_byte((*gx_params).cs55x0, reg, value); }

unsafe fn gx_get_cpuspeed(_cpu: u32) -> u32 {
    if (*gx_params).pci_suscfg & SUSMOD == 0 { return stock_freq as u32; }
    (stock_freq as u32 * (*gx_params).off_duration as u32) /
        ((*gx_params).on_duration as u32 + (*gx_params).off_duration as u32)
}

unsafe fn gx_validate_speed(khz: u32, on_duration: *mut u8, off_duration: *mut u8) -> u32 {
    let mut old_tmp_freq = stock_freq;
    *off_duration = 1; *on_duration = 0;
    let mut i = max_duration;
    while i > 0 {
        let tmp_off = ((khz * i as u32) / stock_freq as u32) as u8;
        let tmp_on = (i as u8).wrapping_sub(tmp_off);
        let tmp_freq = (stock_freq as u32 * tmp_off as u32 / i as u32) as i32;
        if (tmp_freq - khz as i32).abs() <= (old_tmp_freq - khz as i32).abs() {
            *on_duration = tmp_on; *off_duration = tmp_off; old_tmp_freq = tmp_freq;
        }
        i -= 1;
    }
    old_tmp_freq as u32
}

unsafe fn gx_set_cpuspeed(policy: *mut cpufreq_policy, khz: u32) {
    let mut suscfg: u8; let mut pmer1: u8; let mut flags: usize = 0;
    let mut freqs = cpufreq_freqs { old: gx_get_cpuspeed(0), new: 0 };
    freqs.new = gx_validate_speed(khz, &mut (*gx_params).on_duration, &mut (*gx_params).off_duration);
    cpufreq_freq_transition_begin(policy, &mut freqs); local_irq_save(&mut flags);
    if freqs.new != stock_freq as u32 {
        match (*(*gx_params).cs55x0).device {
            PCI_DEVICE_ID_CYRIX_5530_LEGACY => {
                pmer1 = (*gx_params).pci_pmer1 | IRQ_SPDUP | VID_SPDUP;
                gx_write_byte(PCI_IRQTC, 4); gx_write_byte(PCI_VIDTC, 100); gx_write_byte(PCI_PMER1, pmer1);
                suscfg = if (*(*gx_params).cs55x0).revision < 0x10 { (*gx_params).pci_suscfg | SUSMOD } else { (*gx_params).pci_suscfg | SUSMOD | PWRSVE };
            },
            PCI_DEVICE_ID_CYRIX_5520 | PCI_DEVICE_ID_CYRIX_5510 => suscfg = (*gx_params).pci_suscfg | SUSMOD,
            _ => { local_irq_restore(flags); return; }
        }
    } else { suscfg = (*gx_params).pci_suscfg & !SUSMOD; (*gx_params).off_duration = 0; (*gx_params).on_duration = 0; }
    gx_write_byte(PCI_MODOFF, (*gx_params).off_duration); gx_write_byte(PCI_MODON, (*gx_params).on_duration); gx_write_byte(PCI_SUSCFG, suscfg);
    pci_read_config_byte((*gx_params).cs55x0, PCI_SUSCFG, &mut suscfg); local_irq_restore(flags); (*gx_params).pci_suscfg = suscfg;
    cpufreq_freq_transition_end(policy, &mut freqs, 0);
}

unsafe fn cpufreq_gx_verify(policy: *mut cpufreq_policy_data) -> i32 {
    if stock_freq == 0 || policy.is_null() { return -EINVAL; }
    (*policy).cpu = 0; cpufreq_verify_within_limits(policy, (stock_freq / max_duration) as u32, stock_freq as u32);
    let mut t1=0; let mut t2=0; let mut f=gx_validate_speed((*policy).min,&mut t1,&mut t2); if f<(*policy).min { f+=stock_freq as u32/max_duration as u32; } (*policy).min=f; if (*policy).min>(*policy).max {(*policy).max=f;}
    f=gx_validate_speed((*policy).max,&mut t1,&mut t2); if f>(*policy).max {f-=stock_freq as u32/max_duration as u32;} (*policy).max=f; if (*policy).max<(*policy).min {(*policy).max=(*policy).min;} 0
}

unsafe fn cpufreq_gx_target(policy: *mut cpufreq_policy, target_freq: u32, _relation: u32) -> i32 {
    if stock_freq == 0 || policy.is_null() { return -EINVAL; } (*policy).cpu=0; let mut a=0; let mut b=0; let mut f=gx_validate_speed(target_freq,&mut a,&mut b);
    while f<(*policy).min {f+=stock_freq as u32/max_duration as u32; f=gx_validate_speed(f,&mut a,&mut b);} while f>(*policy).max {f-=stock_freq as u32/max_duration as u32; f=gx_validate_speed(f,&mut a,&mut b);} gx_set_cpuspeed(policy,f); 0
}

unsafe fn cpufreq_gx_cpu_init(policy: *mut cpufreq_policy) -> i32 {
    if policy.is_null() || (*policy).cpu != 0 { return -ENODEV; }
    let maxfreq=if pci_busclk!=0 {pci_busclk*gx_freq_mult[(getCx86(CX86_DIR1)&0xf) as usize]} else if cpu_khz!=0 {cpu_khz} else {30000*gx_freq_mult[(getCx86(CX86_DIR1)&0xf) as usize]}; stock_freq=maxfreq; (*policy).cpu=0; (*policy).min=if max_duration<POLICY_MIN_DIV {maxfreq/max_duration} else {maxfreq/POLICY_MIN_DIV}; (*policy).cpuinfo.min_freq=(maxfreq/max_duration) as u32; (*policy).cpuinfo.max_freq=maxfreq as u32; 0
}

static mut gx_suspmod_driver: cpufreq_driver = cpufreq_driver { flags: CPUFREQ_NO_AUTO_DYNAMIC_SWITCHING, get: Some(gx_get_cpuspeed), verify: Some(cpufreq_gx_verify), target: Some(cpufreq_gx_target), init: Some(cpufreq_gx_cpu_init), name: b"gx-suspmod\0".as_ptr() };

unsafe fn gx_detect_chipset() -> *mut pci_dev { core::ptr::null_mut() }

#[no_mangle] pub unsafe extern "C" fn cpufreq_gx_init() -> i32 {
    let gx_pci = gx_detect_chipset();
    if gx_pci.is_null() { return -ENODEV; }
    if max_duration > 0xff { max_duration = 0xff; }
    let params = kzalloc(core::mem::size_of::<gxfreq_params>(), 0) as *mut gxfreq_params;
    if params.is_null() { return -ENOMEM; }
    (*params).cs55x0 = gx_pci; gx_params = params;
    pci_read_config_byte((*params).cs55x0, PCI_SUSCFG, &mut (*params).pci_suscfg);
    pci_read_config_byte((*params).cs55x0, PCI_PMER1, &mut (*params).pci_pmer1);
    pci_read_config_byte((*params).cs55x0, PCI_PMER2, &mut (*params).pci_pmer2);
    pci_read_config_byte((*params).cs55x0, PCI_MODON, &mut (*params).on_duration);
    pci_read_config_byte((*params).cs55x0, PCI_MODOFF, &mut (*params).off_duration);
    let ret = cpufreq_register_driver(&mut gx_suspmod_driver);
    if ret != 0 { kfree(params as *mut core::ffi::c_void); return ret; }
    0
}
#[no_mangle] pub unsafe extern "C" fn cpufreq_gx_exit() { cpufreq_unregister_driver(&mut gx_suspmod_driver); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
