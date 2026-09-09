/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 Maxime Bizon <mbizon@freebox.fr>
 * Copyright (C) 2008 Florian Fainelli <florian@openwrt.org>
 * Copyright (C) 2012 Jonas Gorski <jonas.gorski@gmail.com>
 */

// Dependencies supplied by the Linux bcm63xx headers and other translation units.

use core::ffi::c_void;

const BCM63XX_DEFAULT_PSI_SIZE: i32 = 64;
const ETH_ALEN: usize = 6;
const BCM963XX_NVRAM_V5_SIZE: usize = 256;
const ENODEV: i32 = 19;

#[repr(C)]
pub struct bcm963xx_nvram {
    pub name: [u8; 64],
    pub mac_addr_base: [u8; ETH_ALEN],
    pub mac_addr_count: i32,
    pub psi_size: i32,
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn bcm963xx_nvram_checksum(
        nvram: *const bcm963xx_nvram,
        expected_crc: *mut u32,
        crc: *mut u32,
    ) -> i32;
    fn BCMCPU_IS_3368() -> i32;
    fn pr_warn(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
    fn EXPORT_SYMBOL(symbol: *const c_void);
}

static mut nvram: bcm963xx_nvram = bcm963xx_nvram {
    name: [0; 64],
    mac_addr_base: [0; ETH_ALEN],
    mac_addr_count: 0,
    psi_size: 0,
};
static mut mac_addr_used: i32 = 0;

pub unsafe extern "C" fn bcm63xx_nvram_init(addr: *mut c_void) {
    let mut crc: u32 = 0;
    let mut expected_crc: u32 = 0;
    let hcs_mac_addr: [u8; ETH_ALEN] = [0x00, 0x10, 0x18, 0xff, 0xff, 0xff];

    /* extract nvram data */
    memcpy(
        &raw mut nvram as *mut c_void,
        addr as *const c_void,
        BCM963XX_NVRAM_V5_SIZE,
    );

    /* check checksum before using data */
    if bcm963xx_nvram_checksum(&raw const nvram, &mut expected_crc, &mut crc) != 0 {
        pr_warn(b"nvram checksum failed, contents may be invalid (expected %08x, got %08x)\n\0".as_ptr(), expected_crc, crc);
    }

    /* Cable modems have a different NVRAM which is embedded in the eCos
     * firmware and not easily extractible, give at least a MAC address
     * pool.
     */
    if BCMCPU_IS_3368() != 0 {
        (&raw mut nvram).as_mut().unwrap().mac_addr_base.copy_from_slice(&hcs_mac_addr);
        (&raw mut nvram).as_mut().unwrap().mac_addr_count = 2;
    }
}

pub unsafe extern "C" fn bcm63xx_nvram_get_name() -> *mut u8 {
    (&raw mut nvram).as_mut().unwrap().name.as_mut_ptr()
}

pub unsafe extern "C" fn bcm63xx_nvram_get_mac_address(mac: *mut u8) -> i32 {
    let nvram_ref = (&raw const nvram).as_ref().unwrap();
    if mac_addr_used >= nvram_ref.mac_addr_count {
        pr_err(b"not enough mac addresses\n\0".as_ptr());
        return -ENODEV;
    }

    core::ptr::copy_nonoverlapping(nvram_ref.mac_addr_base.as_ptr(), mac, ETH_ALEN);
    let oui = mac.add(ETH_ALEN / 2 - 1);
    let mut count = mac_addr_used;

    while count != 0 {
        let mut p = mac.add(ETH_ALEN - 1);
        loop {
            *p = (*p).wrapping_add(1);
            if *p != 0 {
                break;
            }
            p = p.sub(1);
            if p == oui {
                break;
            }
        }

        if p == oui {
            pr_err(b"unable to fetch mac address\n\0".as_ptr());
            return -ENODEV;
        }
        count -= 1;
    }

    mac_addr_used += 1;
    0
}

pub unsafe extern "C" fn bcm63xx_nvram_get_psi_size() -> i32 {
    if (&raw const nvram).as_ref().unwrap().psi_size > 0 {
        return (&raw const nvram).as_ref().unwrap().psi_size;
    }

    BCM63XX_DEFAULT_PSI_SIZE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
