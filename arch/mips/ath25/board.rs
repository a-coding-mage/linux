/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2003 Atheros Communications, Inc.,  All Rights Reserved.
 * Copyright (C) 2006 FON Technology, SL.
 * Copyright (C) 2006 Imre Kaloz <kaloz@openwrt.org>
 * Copyright (C) 2006-2009 Felix Fietkau <nbd@openwrt.org>
 */

// Linux and platform dependencies supplied by other translation units.

pub static mut ath25_irq_dispatch: Option<unsafe extern "C" fn()> = None;

#[inline]
unsafe fn check_radio_magic(addr: *const core::ffi::c_void) -> bool {
    let addr = (addr as *const u8).add(0x7a);
    __raw_readb(addr) == 0x5a && __raw_readb(addr.add(1)) == 0xa5
}

#[inline]
unsafe fn check_notempty(addr: *const core::ffi::c_void) -> bool {
    __raw_readl(addr as *const u32) != 0xffff_ffff
}

#[inline]
unsafe fn check_board_data(addr: *const core::ffi::c_void, broken: bool) -> bool {
    if __raw_readl(addr as *const u32) == ATH25_BD_MAGIC {
        return true;
    }

    if !broken {
        return false;
    }

    /* broken board data detected, use radio data to find the
     * offset, user will fix this */
    if check_radio_magic((addr as *const u8).add(0x1000) as *const core::ffi::c_void) {
        return true;
    }
    if check_radio_magic((addr as *const u8).add(0xf8) as *const core::ffi::c_void) {
        return true;
    }

    false
}

unsafe fn find_board_config(
    limit: *const core::ffi::c_void,
    broken: bool,
) -> *const core::ffi::c_void {
    let begin = (limit as *const u8).sub(0x1000);
    let end = (limit as *const u8).sub(0x30000);
    let mut addr = begin;

    while (addr >= end) {
        if check_board_data(addr as *const core::ffi::c_void, broken) {
            return addr as *const core::ffi::c_void;
        }
        addr = addr.sub(0x1000);
    }
    core::ptr::null()
}

unsafe fn find_radio_config(
    limit: *const core::ffi::c_void,
    bcfg: *const core::ffi::c_void,
) -> *const core::ffi::c_void {
    let begin = (bcfg as *const u8).add(0x1000);
    let end = limit as *const u8;
    let mut rcfg = begin;
    while rcfg < end {
        if check_notempty(rcfg as *const core::ffi::c_void)
            && check_radio_magic(rcfg as *const core::ffi::c_void)
        {
            return rcfg as *const core::ffi::c_void;
        }
        rcfg = rcfg.add(0x1000);
    }

    let begin = (bcfg as *const u8).add(0xf8);
    let end = (limit as *const u8).sub(0x1000).add(0xf8);
    let mut rcfg = begin;
    while rcfg < end {
        if check_notempty(rcfg as *const core::ffi::c_void)
            && check_radio_magic(rcfg as *const core::ffi::c_void)
        {
            return rcfg as *const core::ffi::c_void;
        }
        rcfg = rcfg.add(0x1000);
    }

    core::ptr::null()
}

/*
 * NB: Search region size could be larger than the actual flash size,
 * but this shouldn't be a problem here, because the flash
 * will simply be mapped multiple times.
 */
pub unsafe fn ath25_find_config(base: phys_addr_t, size: usize) -> i32 {
    let flash_base = ioremap(base, size);
    let flash_limit = (flash_base as *const u8).add(size) as *const core::ffi::c_void;
    let mut config: *mut ath25_boarddata;
    let mut broken_boarddata = 0;

    ath25_board.config = core::ptr::null_mut();
    ath25_board.radio = core::ptr::null_mut();

    let mut bcfg = find_board_config(flash_limit, false);
    if bcfg.is_null() {
        bcfg = find_board_config(flash_limit, true);
        broken_boarddata = 1;
    }
    if bcfg.is_null() {
        pr_warn!("WARNING: No board configuration data found!\n");
        iounmap(flash_base);
        return -ENODEV;
    }

    let board_data = kzalloc(BOARD_CONFIG_BUFSZ, GFP_KERNEL);
    if board_data.is_null() {
        iounmap(flash_base);
        return -ENODEV;
    }
    config = board_data as *mut ath25_boarddata;
    ath25_board.config = config;
    memcpy_fromio(board_data, bcfg, 0x100);
    if broken_boarddata != 0 {
        pr_warn!("WARNING: broken board data detected\n");
        config = ath25_board.config;
        if is_zero_ether_addr((*config).enet0_mac.as_ptr()) {
            pr_info!("Fixing up empty mac addresses\n");
            (*config).reset_config_gpio = 0xffff;
            (*config).sys_led_gpio = 0xffff;
            eth_random_addr((*config).wlan0_mac.as_mut_ptr());
            (*config).wlan0_mac[0] &= !0x06;
            eth_random_addr((*config).enet0_mac.as_mut_ptr());
            eth_random_addr((*config).enet1_mac.as_mut_ptr());
        }
    }

    let rcfg = find_radio_config(flash_limit, bcfg);
    if rcfg.is_null() {
        pr_warn!("WARNING: Could not find Radio Configuration data\n");
        iounmap(flash_base);
        return -ENODEV;
    }

    let radio_data = (board_data as *mut u8).add(0x100).add(
        (rcfg as usize).wrapping_sub(bcfg as usize) & 0xfff,
    );
    ath25_board.radio = radio_data;
    let offset = radio_data as usize - board_data as usize;
    pr_info!("Radio config found at offset 0x%x (0x%x)\n",
        (rcfg as usize).wrapping_sub(bcfg as usize), offset);
    let rcfg_size = BOARD_CONFIG_BUFSZ - offset;
    memcpy_fromio(radio_data, rcfg, rcfg_size);

    let mac_addr = radio_data.add(0x1d * 2);
    if is_broadcast_ether_addr(mac_addr) {
        pr_info!("Radio MAC is blank; using board-data\n");
        ether_addr_copy(mac_addr, (*ath25_board.config).wlan0_mac.as_ptr());
    }

    iounmap(flash_base);
    0
}

unsafe fn ath25_halt() {
    local_irq_disable();
    unreachable!();
}

pub unsafe fn plat_mem_setup() {
    _machine_halt = Some(ath25_halt);
    pm_power_off = Some(ath25_halt);

    if is_ar5312() {
        ar5312_plat_mem_setup();
    } else {
        ar2315_plat_mem_setup();
    }

    /* Disable data watchpoints */
    write_c0_watchlo0(0);
}

pub unsafe extern "C" fn plat_irq_dispatch() {
    if let Some(dispatch) = ath25_irq_dispatch {
        dispatch();
    }
}

pub unsafe fn plat_time_init() {
    if is_ar5312() {
        ar5312_plat_time_init();
    } else {
        ar2315_plat_time_init();
    }
}

pub unsafe fn get_c0_compare_int() -> u32 {
    CP0_LEGACY_COMPARE_IRQ
}

pub unsafe fn arch_init_irq() {
    clear_c0_status(ST0_IM);
    mips_cpu_irq_init();

    /* Initialize interrupt controllers */
    if is_ar5312() {
        ar5312_arch_init_irq();
    } else {
        ar2315_arch_init_irq();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
