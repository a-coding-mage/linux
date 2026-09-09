// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PowerPC 4xx Clock and Power Management
 *
 * Copyright (C) 2010, Applied Micro Circuits Corporation
 * Victor Gallardo (vgallardo@apm.com)
 *
 * Based on arch/powerpc/platforms/44x/idle.c:
 * Jerone Young <jyoung5@us.ibm.com>
 * Copyright 2008 IBM Corp.
 *
 * Based on arch/powerpc/sysdev/fsl_pmc.c:
 * Anton Vorontsov <avorontsov@ru.mvista.com>
 * Copyright 2009  MontaVista Software, Inc.
 *
 * See file CREDITS for list of people who contributed to this
 * project.
 */

// External kernel types, functions, constants, and macros are supplied by the surrounding kernel translation.

const CPM_ER: usize = 0;
const CPM_FR: usize = 1;
const CPM_SR: usize = 2;

const CPM_IDLE_WAIT: usize = 0;
const CPM_IDLE_DOZE: usize = 1;

#[repr(C)]
struct Cpm {
    dcr_host: dcr_host_t,
    dcr_offset: [c_uint; 3],
    powersave_off: c_uint,
    unused: c_uint,
    idle_doze: c_uint,
    standby: c_uint,
    suspend: c_uint,
}

static mut cpm: Cpm = Cpm {
    dcr_host: unsafe { core::mem::zeroed() },
    dcr_offset: [0; 3],
    powersave_off: 0,
    unused: 0,
    idle_doze: 0,
    standby: 0,
    suspend: 0,
};

#[repr(C)]
struct CpmIdleMode {
    enabled: c_uint,
    name: *const c_char,
}

static mut idle_mode: [CpmIdleMode; 2] = [
    CpmIdleMode { enabled: 1, name: b"wait\0".as_ptr() as *const c_char },
    CpmIdleMode { enabled: 0, name: b"doze\0".as_ptr() as *const c_char },
];

unsafe fn cpm_set(cpm_reg: c_uint, mask: c_uint) -> c_uint {
    let value: c_uint;

    /* CPM controller supports 3 different types of sleep interface
     * known as class 1, 2 and 3. For class 1 units, they are
     * unconditionally put to sleep when the corresponding CPM bit is
     * set. For class 2 and 3 units this is not case; if they can be
     * put to sleep, they will. Here we do not verify, we just
     * set them and expect them to eventually go off when they can.
     */
    value = dcr_read(cpm.dcr_host, cpm.dcr_offset[cpm_reg as usize]);
    dcr_write(cpm.dcr_host, cpm.dcr_offset[cpm_reg as usize], value | mask);

    /* return old state, to restore later if needed */
    value
}

unsafe fn cpm_idle_wait() {
    let msr_save: c_ulong;

    /* save off initial state */
    msr_save = mfmsr();
    /* sync required when CPM0_ER[CPU] is set */
    mb();
    /* set wait state MSR */
    mtmsr(msr_save | MSR_WE | MSR_EE | MSR_CE | MSR_DE);
    isync();
    /* return to initial state */
    mtmsr(msr_save);
    isync();
}

unsafe fn cpm_idle_sleep(mask: c_uint) {
    let er_save: c_uint;

    /* update CPM_ER state */
    er_save = cpm_set(CPM_ER as c_uint, mask);

    /* go to wait state so that CPM0_ER[CPU] can take effect */
    cpm_idle_wait();

    /* restore CPM_ER state */
    dcr_write(cpm.dcr_host, cpm.dcr_offset[CPM_ER], er_save);
}

unsafe fn cpm_idle_doze() {
    cpm_idle_sleep(cpm.idle_doze);
}

unsafe fn cpm_idle_config(mode: c_int) {
    if idle_mode[mode as usize].enabled != 0 {
        return;
    }

    for i in 0..idle_mode.len() {
        idle_mode[i].enabled = 0;
    }

    idle_mode[mode as usize].enabled = 1;
}

unsafe extern "C" fn cpm_idle_show(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let mut s = buf;

    for i in 0..idle_mode.len() {
        if idle_mode[i].enabled != 0 {
            s = s.add(sprintf(s, b"[%s] \0".as_ptr() as *const c_char, idle_mode[i].name) as usize);
        } else {
            s = s.add(sprintf(s, b"%s \0".as_ptr() as *const c_char, idle_mode[i].name) as usize);
        }
    }

    /* convert the last space to a newline */
    *s.sub(1) = b'\n' as c_char;
    s.offset_from(buf) as ssize_t
}

unsafe extern "C" fn cpm_idle_store(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *const c_char,
    n: size_t,
) -> ssize_t {
    let p = memchr(buf as *const c_void, b'\n' as c_int, n);
    let len = if p.is_null() { n } else { p.offset_from(buf as *const c_void) as usize };

    for i in 0..idle_mode.len() {
        if strncmp(buf, idle_mode[i].name, len) == 0 {
            cpm_idle_config(i as c_int);
            return n as ssize_t;
        }
    }

    -EINVAL as ssize_t
}

// Equivalent of __ATTR(idle, 0644, cpm_idle_show, cpm_idle_store).
static mut cpm_idle_attr: kobj_attribute = unsafe { core::mem::zeroed() };

unsafe fn cpm_idle_config_sysfs() {
    let dev: *mut device;
    let ret: c_ulong;

    dev = get_cpu_device(0);
    ret = sysfs_create_file(&mut (*dev).kobj, &mut cpm_idle_attr.attr);
    if ret != 0 {
        printk(KERN_WARNING, b"cpm: failed to create idle sysfs entry\n\0".as_ptr() as *const c_char);
    }
}

unsafe extern "C" fn cpm_idle() {
    if idle_mode[CPM_IDLE_DOZE].enabled != 0 {
        cpm_idle_doze();
    } else {
        cpm_idle_wait();
    }
}

unsafe extern "C" fn cpm_suspend_valid(state: suspend_state_t) -> c_int {
    match state {
        PM_SUSPEND_STANDBY => (cpm.standby != 0) as c_int,
        PM_SUSPEND_MEM => (cpm.suspend != 0) as c_int,
        _ => 0,
    }
}

unsafe fn cpm_suspend_standby(mask: c_uint) {
    let tcr_save = mfspr(SPRN_TCR);
    mtspr(SPRN_TCR, tcr_save & !TCR_DIE);
    cpm_idle_sleep(mask);
    mtspr(SPRN_TCR, tcr_save);
}

unsafe extern "C" fn cpm_suspend_enter(state: suspend_state_t) -> c_int {
    match state {
        PM_SUSPEND_STANDBY => cpm_suspend_standby(cpm.standby),
        PM_SUSPEND_MEM => cpm_suspend_standby(cpm.suspend),
        _ => {}
    }
    0
}

static mut cpm_suspend_ops: platform_suspend_ops = unsafe { core::mem::zeroed() };

unsafe fn cpm_get_uint_property(np: *mut device_node, name: *const c_char) -> c_int {
    let mut len: c_int = 0;
    let prop = of_get_property(np, name, &mut len);
    if prop.is_null() || len < core::mem::size_of::<u32>() as c_int {
        return 0;
    }
    *prop as c_int
}

unsafe extern "C" fn cpm_init() -> c_int {
    let mut ret = 0;
    if cpm.powersave_off == 0 {
        cpm_idle_config(CPM_IDLE_WAIT as c_int);
        ppc_md.power_save = Some(cpm_idle);
    }

    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"ibm,cpm\0".as_ptr() as *const c_char);
    if np.is_null() { return -EINVAL; }

    let dcr_base = dcr_resource_start(np);
    let dcr_len = dcr_resource_len(np);
    if dcr_base == 0 || dcr_len == 0 {
        printk(KERN_ERR, b"cpm: could not parse dcr property for %pOF\n\0".as_ptr() as *const c_char, np);
        of_node_put(np); return -EINVAL;
    }
    cpm.dcr_host = dcr_map(np, dcr_base, dcr_len);
    if !DCR_MAP_OK(cpm.dcr_host) {
        printk(KERN_ERR, b"cpm: failed to map dcr property for %pOF\n\0".as_ptr() as *const c_char, np);
        of_node_put(np); return -EINVAL;
    }

    if cpm_get_uint_property(np, b"er-offset\0".as_ptr() as *const c_char) == 0 {
        cpm.dcr_offset = [0, 1, 2];
    } else {
        cpm.dcr_offset = [1, 2, 0];
    }

    cpm.unused = cpm_get_uint_property(np, b"unused-units\0".as_ptr() as *const c_char) as c_uint;
    cpm.idle_doze = cpm_get_uint_property(np, b"idle-doze\0".as_ptr() as *const c_char) as c_uint;
    cpm.standby = cpm_get_uint_property(np, b"standby\0".as_ptr() as *const c_char) as c_uint;
    cpm.suspend = cpm_get_uint_property(np, b"suspend\0".as_ptr() as *const c_char) as c_uint;

    if cpm.unused != 0 {
        cpm_set(CPM_ER as c_uint, cpm.unused);
        cpm_set(CPM_FR as c_uint, cpm.unused);
    }
    if cpm.powersave_off == 0 && cpm.idle_doze != 0 { cpm_idle_config_sysfs(); }
    if cpm.standby != 0 || cpm.suspend != 0 { suspend_set_ops(&cpm_suspend_ops); }
    of_node_put(np);
    ret
}

// Equivalent of late_initcall(cpm_init).

unsafe extern "C" fn cpm_powersave_off(_arg: *mut c_char) -> c_int {
    cpm.powersave_off = 1;
    1
}

// Equivalent of __setup("powersave=off", cpm_powersave_off).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
