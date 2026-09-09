// SPDX-License-Identifier: GPL-2.0
/*
 * Alchemy Development Board example suspend userspace interface.
 *
 * (c) 2008 Manuel Lauss <mano@roarinelk.homelinux.net>
 */

// Linux and Alchemy platform dependencies supplied by other translation units.

static mut db1x_pm_sleep_secs: libc::c_ulong = 0;
static mut db1x_pm_wakemsk: libc::c_ulong = 0;
static mut db1x_pm_last_wakesrc: libc::c_ulong = 0;

unsafe extern "C" {
    fn bcsr_read(reg: libc::c_int) -> libc::c_ushort;
    fn bcsr_write(reg: libc::c_int, value: libc::c_ushort);
    fn alchemy_gpio1_input_enable();
    fn alchemy_wrsys(value: libc::c_ulong, reg: libc::c_int);
    fn alchemy_rdsys(reg: libc::c_int) -> libc::c_ulong;
    fn au_sleep();
    fn printk(format: *const libc::c_char, ...) -> libc::c_int;
    fn suspend_valid_only_mem(state: suspend_state_t) -> bool;
    fn suspend_set_ops(ops: *const platform_suspend_ops);
    fn sysfs_create_group(kobj: *mut kobject, group: *const attribute_group) -> libc::c_int;
    fn kstrtoul(
        string: *const libc::c_char,
        base: libc::c_uint,
        result: *mut libc::c_ulong,
    ) -> libc::c_int;
    fn sprintf(buf: *mut libc::c_char, format: *const libc::c_char, ...) -> libc::c_int;
}

type suspend_state_t = libc::c_int;

#[repr(C)]
struct platform_suspend_ops {
    valid: Option<unsafe extern "C" fn(suspend_state_t) -> bool>,
    begin: Option<unsafe extern "C" fn(suspend_state_t) -> libc::c_int>,
    enter: Option<unsafe extern "C" fn(suspend_state_t) -> libc::c_int>,
    end: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
struct attribute {
    name: *const libc::c_char,
    mode: libc::c_uint,
}

#[repr(C)]
struct kobject {
    _private: [u8; 0],
}

#[repr(C)]
struct kobj_attribute {
    attr: attribute,
    show: Option<unsafe extern "C" fn(*mut kobject, *mut kobj_attribute, *mut libc::c_char) -> isize>,
    store: Option<unsafe extern "C" fn(*mut kobject, *mut kobj_attribute, *const libc::c_char, usize) -> isize>,
}

#[repr(C)]
struct attribute_group {
    name: *const libc::c_char,
    attrs: *mut *mut attribute,
}

unsafe fn db1x_pm_enter(_state: suspend_state_t) -> libc::c_int {
    let mut bcsrs = [0u16; 16];
    let mut i: libc::c_int;
    let mut j: libc::c_int;
    let hasint: libc::c_int;

    hasint = (bcsr_read(BCSR_WHOAMI) >= BCSR_WHOAMI_DB1200 as libc::c_ushort) as libc::c_int;
    j = if hasint != 0 { BCSR_MASKSET } else { BCSR_SYSTEM };

    i = BCSR_STATUS;
    while i <= j {
        bcsrs[i as usize] = bcsr_read(i);
        i += 1;
    }

    bcsr_write(BCSR_HEXCLEAR, 3);
    alchemy_gpio1_input_enable();
    alchemy_wrsys(0, AU1000_SYS_WAKEMSK);
    alchemy_wrsys(0, AU1000_SYS_WAKESRC);
    alchemy_wrsys(db1x_pm_wakemsk, AU1000_SYS_WAKEMSK);

    while alchemy_rdsys(AU1000_SYS_CNTRCTRL) & SYS_CNTRL_M20 != 0 {
        core::hint::spin_loop();
    }
    alchemy_wrsys(
        alchemy_rdsys(AU1000_SYS_TOYREAD).wrapping_add(db1x_pm_sleep_secs),
        AU1000_SYS_TOYMATCH2,
    );
    while alchemy_rdsys(AU1000_SYS_CNTRCTRL) & SYS_CNTRL_M20 != 0 {
        core::hint::spin_loop();
    }
    au_sleep();

    i = BCSR_STATUS;
    while i <= BCSR_SYSTEM {
        bcsr_write(i, bcsrs[i as usize]);
        i += 1;
    }
    if hasint != 0 {
        bcsr_write(BCSR_INTCLR, 0xffff);
        bcsr_write(BCSR_MASKCLR, 0xffff);
        bcsr_write(BCSR_INTSTAT, 0xffff);
        bcsr_write(BCSR_INTSET, bcsrs[BCSR_INTSET as usize]);
        bcsr_write(BCSR_MASKSET, bcsrs[BCSR_MASKSET as usize]);
    }
    bcsr_write(BCSR_HEXCLEAR, 0);
    0
}

unsafe fn db1x_pm_begin(_state: suspend_state_t) -> libc::c_int {
    if db1x_pm_wakemsk == 0 {
        printk(b"db1x: no wakeup source activated!\0".as_ptr() as *const libc::c_char);
        return -libc::EINVAL;
    }
    0
}

unsafe fn db1x_pm_end() {
    db1x_pm_last_wakesrc = alchemy_rdsys(AU1000_SYS_WAKESRC);
    alchemy_wrsys(0, AU1000_SYS_WAKEMSK);
    alchemy_wrsys(0, AU1000_SYS_WAKESRC);
}

static db1x_pm_ops: platform_suspend_ops = platform_suspend_ops {
    valid: Some(suspend_valid_only_mem),
    begin: Some(db1x_pm_begin),
    enter: Some(db1x_pm_enter),
    end: Some(db1x_pm_end),
};

unsafe fn db1x_pmattr_show(
    _kobj: *mut kobject,
    attr: *mut kobj_attribute,
    buf: *mut libc::c_char,
) -> isize {
    let name = (*attr).attr.name;
    if libc::strcmp(name, b"timer_timeout\0".as_ptr() as *const libc::c_char) == 0 {
        sprintf(buf, b"%lu\n\0".as_ptr() as *const libc::c_char, db1x_pm_sleep_secs) as isize
    } else if libc::strcmp(name, b"timer\0".as_ptr() as *const libc::c_char) == 0 {
        sprintf(buf, b"%u\n\0".as_ptr() as *const libc::c_char,
            (db1x_pm_wakemsk & SYS_WAKEMSK_M2 != 0) as libc::c_uint) as isize
    } else if libc::strcmp(name, b"wakesrc\0".as_ptr() as *const libc::c_char) == 0 {
        sprintf(buf, b"%lu\n\0".as_ptr() as *const libc::c_char, db1x_pm_last_wakesrc) as isize
    } else if (*name.add(0) == b'g' as libc::c_char) && (*name.add(1) == b'p' as libc::c_char) {
        let idx = (*name.add(4) - b'0' as libc::c_char) as usize;
        sprintf(buf, b"%d\n\0".as_ptr() as *const libc::c_char,
            (db1x_pm_wakemsk & SYS_WAKEMSK_GPIO(idx as libc::c_int) != 0) as libc::c_int) as isize
    } else if libc::strcmp(name, b"wakemsk\0".as_ptr() as *const libc::c_char) == 0 {
        sprintf(buf, b"%08lx\n\0".as_ptr() as *const libc::c_char, db1x_pm_wakemsk) as isize
    } else {
        -libc::ENOENT as isize
    }
}

unsafe fn db1x_pmattr_store(
    _kobj: *mut kobject,
    attr: *mut kobj_attribute,
    instr: *const libc::c_char,
    mut bytes: usize,
) -> isize {
    let mut value = 0 as libc::c_ulong;
    let name = (*attr).attr.name;
    if libc::strcmp(name, b"timer_timeout\0".as_ptr() as *const libc::c_char) == 0 {
        let result = kstrtoul(instr, 0, &mut value);
        if result != 0 { return result as isize; }
        db1x_pm_sleep_secs = value;
    } else if libc::strcmp(name, b"timer\0".as_ptr() as *const libc::c_char) == 0 {
        if *instr != b'0' as libc::c_char { db1x_pm_wakemsk |= SYS_WAKEMSK_M2; }
        else { db1x_pm_wakemsk &= !SYS_WAKEMSK_M2; }
    } else if *name == b'g' as libc::c_char && *name.add(1) == b'p' as libc::c_char {
        let idx = (*name.add(4) - b'0' as libc::c_char) as libc::c_int;
        if *instr != b'0' as libc::c_char { db1x_pm_wakemsk |= SYS_WAKEMSK_GPIO(idx); }
        else { db1x_pm_wakemsk &= !SYS_WAKEMSK_GPIO(idx); }
    } else if libc::strcmp(name, b"wakemsk\0".as_ptr() as *const libc::c_char) == 0 {
        let result = kstrtoul(instr, 0, &mut value);
        if result != 0 { return result as isize; }
        db1x_pm_wakemsk = value & 0x0000003f;
    } else {
        bytes = (-libc::ENOENT) as usize;
    }
    bytes as isize
}

// Attribute objects are emitted individually to preserve the C declarations.
macro_rules! attr { ($n:ident, $s:literal) => {
    static mut $n: kobj_attribute = kobj_attribute { attr: attribute { name: concat!($s, "\0").as_ptr() as *const libc::c_char, mode: 0o664 }, show: Some(db1x_pmattr_show), store: Some(db1x_pmattr_store) };
}; }
attr!(gpio0_attribute, "gpio0"); attr!(gpio1_attribute, "gpio1"); attr!(gpio2_attribute, "gpio2"); attr!(gpio3_attribute, "gpio3"); attr!(gpio4_attribute, "gpio4"); attr!(gpio5_attribute, "gpio5"); attr!(gpio6_attribute, "gpio6"); attr!(gpio7_attribute, "gpio7");
attr!(timer_attribute, "timer"); attr!(timer_timeout_attribute, "timer_timeout"); attr!(wakesrc_attribute, "wakesrc"); attr!(wakemsk_attribute, "wakemsk");

static mut db1x_pmattrs: [*mut attribute; 13] = [
    unsafe { &mut gpio0_attribute.attr }, unsafe { &mut gpio1_attribute.attr }, unsafe { &mut gpio2_attribute.attr }, unsafe { &mut gpio3_attribute.attr },
    unsafe { &mut gpio4_attribute.attr }, unsafe { &mut gpio5_attribute.attr }, unsafe { &mut gpio6_attribute.attr }, unsafe { &mut gpio7_attribute.attr },
    unsafe { &mut timer_attribute.attr }, unsafe { &mut timer_timeout_attribute.attr }, unsafe { &mut wakesrc_attribute.attr }, unsafe { &mut wakemsk_attribute.attr }, core::ptr::null_mut(),
];
static mut db1x_pmattr_group: attribute_group = attribute_group { name: b"db1x\0".as_ptr() as *const libc::c_char, attrs: db1x_pmattrs.as_mut_ptr() };

unsafe fn db1x_pm_init() -> libc::c_int {
    if alchemy_rdsys(AU1000_SYS_TOYTRIM) != 32767 {
        alchemy_wrsys(32767, AU1000_SYS_TOYTRIM);
    }
    db1x_pm_last_wakesrc = alchemy_rdsys(AU1000_SYS_WAKESRC);
    alchemy_wrsys(0, AU1000_SYS_WAKESRC);
    alchemy_wrsys(0, AU1000_SYS_WAKEMSK);
    suspend_set_ops(&db1x_pm_ops);
    sysfs_create_group(power_kobj, &db1x_pmattr_group)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
