// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2008 Openmoko, Inc.
// Copyright 2008 Simtec Electronics
//	Ben Dooks <ben@simtec.co.uk>
//	http://armlinux.simtec.co.uk/
//
// S3C series GPIO PM code

// Dependency declarations are supplied by the surrounding kernel translation.

const OFFS_CON: usize = 0x00;
const OFFS_DAT: usize = 0x04;
const OFFS_UP: usize = 0x08;

unsafe fn samsung_gpio_pm_1bit_save(chip: *mut samsung_gpio_chip) {
    (*chip).pm_save[0] = __raw_readl((*chip).base.add(OFFS_CON));
    (*chip).pm_save[1] = __raw_readl((*chip).base.add(OFFS_DAT));
}

unsafe fn samsung_gpio_pm_1bit_resume(chip: *mut samsung_gpio_chip) {
    let base = (*chip).base;
    let old_gpcon = __raw_readl(base.add(OFFS_CON));
    let old_gpdat = __raw_readl(base.add(OFFS_DAT));
    let gps_gpcon = (*chip).pm_save[0];
    let gps_gpdat = (*chip).pm_save[1];
    let gpcon;

    // GPACON only has one bit per control / data and no PULLUPs.
    // GPACON[x] = 0 => Output, 1 => SFN
    // first set all SFN bits to SFN
    gpcon = old_gpcon | gps_gpcon;
    __raw_writel(gpcon, base.add(OFFS_CON));

    // now set all the other bits
    __raw_writel(gps_gpdat, base.add(OFFS_DAT));
    __raw_writel(gps_gpcon, base.add(OFFS_CON));

    S3C_PMDBG!("%s: CON %08x => %08x, DAT %08x => %08x\n",
        (*chip).chip.label, old_gpcon, gps_gpcon, old_gpdat, gps_gpdat);
}

pub static mut samsung_gpio_pm_1bit: samsung_gpio_pm = samsung_gpio_pm {
    save: Some(samsung_gpio_pm_1bit_save),
    resume: Some(samsung_gpio_pm_1bit_resume),
};

unsafe fn samsung_gpio_pm_2bit_save(chip: *mut samsung_gpio_chip) {
    (*chip).pm_save[0] = __raw_readl((*chip).base.add(OFFS_CON));
    (*chip).pm_save[1] = __raw_readl((*chip).base.add(OFFS_DAT));
    (*chip).pm_save[2] = __raw_readl((*chip).base.add(OFFS_UP));
}

#[inline]
fn is_sfn(con: ::core::ffi::c_ulong) -> bool { con >= 2 }

#[inline]
fn is_in(con: ::core::ffi::c_ulong) -> bool { con == 0 }

#[inline]
fn is_out(con: ::core::ffi::c_ulong) -> bool { con == 1 }

unsafe fn samsung_gpio_pm_2bit_resume(chip: *mut samsung_gpio_chip) {
    let base = (*chip).base;
    let old_gpcon = __raw_readl(base.add(OFFS_CON));
    let old_gpdat = __raw_readl(base.add(OFFS_DAT));
    let gps_gpcon = (*chip).pm_save[0];
    let gps_gpdat = (*chip).pm_save[1];
    let mut change_mask: u32 = 0;

    __raw_writel((*chip).pm_save[2], base.add(OFFS_UP));

    let mut nr: u32 = 0;
    let mut mask: u32 = 0x03;
    while nr < 32 {
        let old = (old_gpcon & mask) >> nr;
        let new = (gps_gpcon & mask) >> nr;
        if old != new && !(is_sfn(old as _) && is_sfn(new as _))
            && !(is_in(old as _) && is_out(new as _))
            && !(is_sfn(old as _) && is_out(new as _)) {
            change_mask |= mask;
        }
        nr += 2;
        mask <<= 2;
    }

    let gpcon = (old_gpcon & !change_mask) | (gps_gpcon & change_mask);
    __raw_writel(gpcon, base.add(OFFS_CON));
    __raw_writel(gps_gpdat, base.add(OFFS_DAT));
    __raw_writel(gps_gpcon, base.add(OFFS_CON));

    S3C_PMDBG!("%s: CON %08x => %08x, DAT %08x => %08x\n",
        (*chip).chip.label, old_gpcon, gps_gpcon, old_gpdat, gps_gpdat);
}

pub static mut samsung_gpio_pm_2bit: samsung_gpio_pm = samsung_gpio_pm {
    save: Some(samsung_gpio_pm_2bit_save),
    resume: Some(samsung_gpio_pm_2bit_resume),
};

#[cfg(CONFIG_ARCH_S3C64XX)]
unsafe fn samsung_gpio_pm_4bit_save(chip: *mut samsung_gpio_chip) {
    (*chip).pm_save[1] = __raw_readl((*chip).base.add(OFFS_CON));
    (*chip).pm_save[2] = __raw_readl((*chip).base.add(OFFS_DAT));
    (*chip).pm_save[3] = __raw_readl((*chip).base.add(OFFS_UP));
    if (*chip).chip.ngpio > 8 { (*chip).pm_save[0] = __raw_readl((*chip).base.offset(-4)); }
}

#[cfg(CONFIG_ARCH_S3C64XX)]
unsafe fn samsung_gpio_pm_4bit_mask(old_gpcon: u32, gps_gpcon: u32) -> u32 {
    let mut change_mask = 0;
    let mut nr = 0;
    let mut mask = 0x0f;
    while nr < 16 {
        let old = (old_gpcon & mask) >> nr;
        let new = (gps_gpcon & mask) >> nr;
        if old != new && !(is_sfn(old as _) && is_sfn(new as _))
            && !(is_in(old as _) && is_out(new as _))
            && !(is_sfn(old as _) && is_out(new as _)) { change_mask |= mask; }
        nr += 4; mask <<= 4;
    }
    change_mask
}

#[cfg(CONFIG_ARCH_S3C64XX)]
unsafe fn samsung_gpio_pm_4bit_con(chip: *mut samsung_gpio_chip, index: isize) {
    let con = (*chip).base.offset(index * 4);
    let old_gpcon = __raw_readl(con);
    let gps_gpcon = (*chip).pm_save[(index + 1) as usize];
    let mask = samsung_gpio_pm_4bit_mask(old_gpcon, gps_gpcon);
    __raw_writel((old_gpcon & !mask) | (gps_gpcon & mask), con);
}

#[cfg(CONFIG_ARCH_S3C64XX)]
unsafe fn samsung_gpio_pm_4bit_resume(chip: *mut samsung_gpio_chip) {
    let base = (*chip).base;
    let mut old_gpcon = [0u32; 2];
    let old_gpdat = __raw_readl(base.add(OFFS_DAT));
    let gps_gpdat = (*chip).pm_save[2];
    old_gpcon[1] = __raw_readl(base.add(OFFS_CON));
    samsung_gpio_pm_4bit_con(chip, 0);
    if (*chip).chip.ngpio > 8 { old_gpcon[0] = __raw_readl(base.offset(-4)); samsung_gpio_pm_4bit_con(chip, -1); }
    __raw_writel((*chip).pm_save[2], base.add(OFFS_DAT));
    __raw_writel((*chip).pm_save[1], base.add(OFFS_CON));
    if (*chip).chip.ngpio > 8 { __raw_writel((*chip).pm_save[0], base.offset(-4)); }
    __raw_writel((*chip).pm_save[2], base.add(OFFS_DAT));
    __raw_writel((*chip).pm_save[3], base.add(OFFS_UP));
    let _ = (old_gpcon, old_gpdat, gps_gpdat);
}

#[cfg(CONFIG_ARCH_S3C64XX)]
pub static mut samsung_gpio_pm_4bit: samsung_gpio_pm = samsung_gpio_pm { save: Some(samsung_gpio_pm_4bit_save), resume: Some(samsung_gpio_pm_4bit_resume) };

unsafe fn samsung_pm_save_gpio(ourchip: *mut samsung_gpio_chip) {
    let pm = (*ourchip).pm;
    if pm.is_null() || (*pm).save.is_none() { S3C_PMDBG!("%s: no pm for %s\n", core::module_path!(), (*ourchip).chip.label); }
    else { ((*pm).save.unwrap())(ourchip); }
}

pub unsafe fn samsung_pm_save_gpios() {
    let mut gpio_nr: u32 = 0;
    while gpio_nr < S3C_GPIO_END {
        let ourchip = samsung_gpiolib_getchip(gpio_nr);
        if ourchip.is_null() { gpio_nr += 1; continue; }
        samsung_pm_save_gpio(ourchip);
        gpio_nr += (*ourchip).chip.ngpio;
        gpio_nr += CONFIG_S3C_GPIO_SPACE;
    }
}

unsafe fn samsung_pm_resume_gpio(ourchip: *mut samsung_gpio_chip) {
    let pm = (*ourchip).pm;
    if !pm.is_null() && (*pm).resume.is_some() { ((*pm).resume.unwrap())(ourchip); }
}

pub unsafe fn samsung_pm_restore_gpios() {
    let mut gpio_nr: u32 = 0;
    while gpio_nr < S3C_GPIO_END {
        let ourchip = samsung_gpiolib_getchip(gpio_nr);
        if ourchip.is_null() { gpio_nr += 1; continue; }
        samsung_pm_resume_gpio(ourchip);
        gpio_nr += (*ourchip).chip.ngpio;
        gpio_nr += CONFIG_S3C_GPIO_SPACE;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
