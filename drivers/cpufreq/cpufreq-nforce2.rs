// SPDX-License-Identifier: GPL-2.0-only
/*
 * (C) 2004-2006  Sebastian Witt <se.witt@gmx.net>
 *
 *  Based upon reverse engineered information
 *
 *  BIG FAT DISCLAIMER: Work in progress code. Possibly *dangerous*
 */

// Kernel dependencies supplied by the surrounding translation unit.

const NFORCE2_XTAL: i32 = 25;
const NFORCE2_BOOTFSB: u32 = 0x48;
const NFORCE2_PLLENABLE: u32 = 0xa8;
const NFORCE2_PLLREG: u32 = 0xa4;
const NFORCE2_PLLADR: u32 = 0xa0;
const NFORCE2_MIN_FSB: i32 = 50;
const NFORCE2_SAFE_DISTANCE: i32 = 50;

#[inline]
const fn nforce2_pll(mul: u8, div: u8) -> i32 {
    0x100000 | ((mul as i32) << 8) | div as i32
}

static mut nforce2_dev: *mut pci_dev = core::ptr::null_mut();
static mut fid: i32 = 0;
static mut min_fsb: i32 = 0;
static mut max_fsb: i32 = 0;

static mut nforce2_driver: cpufreq_driver = cpufreq_driver {
    name: "nforce2",
    flags: CPUFREQ_NO_AUTO_DYNAMIC_SWITCHING,
    verify: Some(nforce2_verify),
    target: Some(nforce2_target),
    get: Some(nforce2_get),
    init: Some(nforce2_cpu_init),
};

#[cfg(feature = "module")]
static nforce2_ids: [pci_device_id; 2] = [
    pci_device_id { vendor: PCI_VENDOR_ID_NVIDIA, device: PCI_DEVICE_ID_NVIDIA_NFORCE2 },
    pci_device_id { vendor: 0, device: 0 },
];

unsafe fn nforce2_calc_fsb(pll: i32) -> i32 {
    let mul = ((pll >> 8) & 0xff) as u8;
    let div = (pll & 0xff) as u8;
    if div > 0 { NFORCE2_XTAL * mul as i32 / div as i32 } else { 0 }
}

unsafe fn nforce2_calc_pll(fsb: u32) -> i32 {
    let mut mul: u8 = 0;
    let mut div: u8 = 0;
    let mut tried = 0;
    while (mul == 0 || div == 0) && tried <= 3 {
        let mut xdiv: u8 = 2;
        while xdiv <= 0x80 {
            let mut xmul: u8 = 1;
            while xmul <= 0xfe {
                if nforce2_calc_fsb(nforce2_pll(xmul, xdiv)) == (fsb + tried as u32) as i32 {
                    mul = xmul;
                    div = xdiv;
                }
                xmul = xmul.wrapping_add(1);
            }
            xdiv = xdiv.wrapping_add(1);
        }
        tried += 1;
    }
    if mul == 0 || div == 0 { -1 } else { nforce2_pll(mul, div) }
}

unsafe fn nforce2_write_pll(pll: i32) {
    pci_write_config_dword(nforce2_dev, NFORCE2_PLLADR, 0);
    for temp in 0..=0x3f { let _ = temp; pci_write_config_dword(nforce2_dev, NFORCE2_PLLREG, pll as u32); }
}

unsafe fn nforce2_fsb_read(bootfsb: i32) -> u32 {
    let nforce2_sub5 = pci_get_subsys(PCI_VENDOR_ID_NVIDIA, 0x01EF, PCI_ANY_ID, PCI_ANY_ID, core::ptr::null_mut());
    if nforce2_sub5.is_null() { return 0; }
    let mut fsb: u32 = 0;
    pci_read_config_dword(nforce2_sub5, NFORCE2_BOOTFSB, &mut fsb);
    fsb /= 1000000;
    pci_dev_put(nforce2_sub5);
    let mut temp: u32 = 0;
    pci_read_config_byte(nforce2_dev, NFORCE2_PLLENABLE, &mut temp as *mut u32 as *mut u8);
    if bootfsb != 0 || temp == 0 { return fsb; }
    pci_read_config_dword(nforce2_dev, NFORCE2_PLLREG, &mut temp);
    nforce2_calc_fsb(temp as i32) as u32
}

unsafe fn nforce2_set_fsb(fsb: u32) -> i32 {
    let mut temp: u32 = 0;
    let tfsb;
    let mut pll = 0;
    if fsb as i32 > max_fsb || fsb < NFORCE2_MIN_FSB as u32 { pr_err!("FSB {} is out of range!\n", fsb); return -EINVAL; }
    tfsb = nforce2_fsb_read(0);
    if tfsb == 0 { pr_err!("Error while reading the FSB\n"); return -EINVAL; }
    pci_read_config_byte(nforce2_dev, NFORCE2_PLLENABLE, &mut temp as *mut u32 as *mut u8);
    if temp == 0 { pll = nforce2_calc_pll(tfsb); if pll < 0 { return -EINVAL; } nforce2_write_pll(pll); }
    temp = 1; pci_write_config_byte(nforce2_dev, NFORCE2_PLLENABLE, temp as u8);
    let diff = tfsb as i32 - fsb as i32;
    if diff == 0 { return 0; }
    let mut current = tfsb;
    while current != fsb && current as i32 <= max_fsb && current as i32 >= min_fsb {
        if diff < 0 { current += 1; } else { current -= 1; }
        pll = nforce2_calc_pll(current); if pll == -1 { return -EINVAL; }
        nforce2_write_pll(pll);
    }
    temp = 0x40; pci_write_config_byte(nforce2_dev, NFORCE2_PLLADR, temp as u8); 0
}

unsafe fn nforce2_get(cpu: u32) -> u32 {
    if cpu != 0 { return 0; }
    nforce2_fsb_read(0) * fid as u32 * 100
}

unsafe fn nforce2_target(policy: *mut cpufreq_policy, target_freq: u32, _relation: u32) -> i32 {
    let policy = &mut *policy;
    if target_freq > policy.max || target_freq < policy.min { return -EINVAL; }
    let target_fsb = target_freq / (fid as u32 * 100);
    let mut freqs = cpufreq_freqs { old: nforce2_get(policy.cpu), new: target_fsb * fid as u32 * 100 };
    if freqs.old == freqs.new { return 0; }
    pr_debug!("Old CPU frequency {} kHz, new {} kHz\n", freqs.old, freqs.new);
    cpufreq_freq_transition_begin(policy, &mut freqs);
    if nforce2_set_fsb(target_fsb) < 0 { pr_err!("Changing FSB to {} failed\n", target_fsb); } else { pr_debug!("Changed FSB successfully to {}\n", target_fsb); }
    cpufreq_freq_transition_end(policy, &mut freqs, 0); 0
}

unsafe fn nforce2_verify(policy: *mut cpufreq_policy_data) -> i32 {
    let policy = &mut *policy;
    let fsb_pol_max = policy.max / (fid as u32 * 100);
    if policy.min < fsb_pol_max * fid as u32 * 100 { policy.max = (fsb_pol_max + 1) * fid as u32 * 100; }
    cpufreq_verify_within_cpu_limits(policy); 0
}

unsafe fn nforce2_cpu_init(policy: *mut cpufreq_policy) -> i32 {
    let policy = &mut *policy;
    if policy.cpu != 0 { return -ENODEV; }
    let fsb = nforce2_fsb_read(0); if fsb == 0 { return -EIO; }
    if fid == 0 {
        if cpu_khz == 0 { pr_warn!("cpu_khz not set, can't calculate multiplier!\n"); return -ENODEV; }
        fid = cpu_khz as i32 / (fsb as i32 * 100);
        let rfid = fid % 5; if rfid != 0 { if rfid > 2 { fid += 5 - rfid; } else { fid -= rfid; } }
    }
    pr_info!("FSB currently at {} MHz, FID {}.{}\n", fsb, fid / 10, fid % 10);
    max_fsb = nforce2_fsb_read(1) as i32; if max_fsb == 0 { return -EIO; }
    if min_fsb == 0 { min_fsb = max_fsb - NFORCE2_SAFE_DISTANCE; }
    if min_fsb < NFORCE2_MIN_FSB { min_fsb = NFORCE2_MIN_FSB; }
    policy.cpuinfo.min_freq = min_fsb as u32 * fid as u32 * 100;
    policy.cpuinfo.max_freq = max_fsb as u32 * fid as u32 * 100; 0
}

unsafe fn nforce2_detect_chipset() -> i32 {
    nforce2_dev = pci_get_subsys(PCI_VENDOR_ID_NVIDIA, PCI_DEVICE_ID_NVIDIA_NFORCE2, PCI_ANY_ID, PCI_ANY_ID, core::ptr::null_mut());
    if nforce2_dev.is_null() { return -ENODEV; }
    pr_info!("Detected nForce2 chipset revision {:X}\n", (*nforce2_dev).revision);
    pr_info!("FSB changing is maybe unstable and can lead to crashes and data loss\n"); 0
}

unsafe fn nforce2_init() -> i32 {
    if nforce2_detect_chipset() != 0 { pr_info!("No nForce2 chipset\n"); return -ENODEV; }
    cpufreq_register_driver(&mut nforce2_driver)
}

unsafe fn nforce2_exit() { cpufreq_unregister_driver(&mut nforce2_driver); pci_dev_put(nforce2_dev); }

// module_init(nforce2_init); module_exit(nforce2_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
