// SPDX-License-Identifier: GPL-2.0-or-later
/* ----------------------------------------------------------------------- *
 *
 *   Copyright 2000-2008 H. Peter Anvin - All Rights Reserved
 *
 * ----------------------------------------------------------------------- */

/*
 * x86 CPUID access device
 *
 * This device is accessed by lseek() to the appropriate CPUID level
 * and then read in chunks of 16 bytes.  A larger size means multiple
 * reads of consecutive levels.
 *
 * The lower 32 bits of the file position is used as the incoming %eax,
 * and the upper 32 bits of the file position as the incoming %ecx,
 * the latter intended for "counting" eax levels like eax=4.
 *
 * This driver uses /dev/cpu/%d/cpuid where %d is the minor number, and on
 * an SMP box will direct the access to CPU %d.
 */

// C header dependencies are supplied externally.

static mut cpuhp_cpuid_state: enum_cpuhp_state = 0 as enum_cpuhp_state;

#[repr(C)]
struct cpuid_regs_done {
    regs: cpuid_regs,
    done: completion,
}

unsafe fn cpuid_smp_cpuid(cmd_block: *mut core::ffi::c_void) {
    let cmd = cmd_block as *mut cpuid_regs_done;

    cpuid_count(
        (*cmd).regs.eax,
        (*cmd).regs.ecx,
        &mut (*cmd).regs.eax,
        &mut (*cmd).regs.ebx,
        &mut (*cmd).regs.ecx,
        &mut (*cmd).regs.edx,
    );

    complete(&mut (*cmd).done);
}

unsafe fn cpuid_read_f(
    file: *mut file,
    buf: *mut core::ffi::c_char,
    mut count: usize,
    ppos: *mut loff_t,
) -> ssize_t {
    let mut tmp = buf;
    let mut cmd: cpuid_regs_done = core::mem::zeroed();
    let cpu: i32 = iminor(file_inode(file));
    let mut pos: u64 = *ppos as u64;
    let mut bytes: ssize_t = 0;
    let mut err: i32 = 0;

    if count % 16 != 0 {
        return -EINVAL; // Invalid chunk size
    }

    init_completion(&mut cmd.done);
    while count != 0 {
        let mut csd: call_single_data_t = core::mem::zeroed();

        INIT_CSD(&mut csd, cpuid_smp_cpuid, &mut cmd as *mut _);

        cmd.regs.eax = pos as u32;
        cmd.regs.ecx = (pos >> 32) as u32;

        err = smp_call_function_single_async(cpu, &mut csd);
        if err != 0 {
            break;
        }
        wait_for_completion(&mut cmd.done);
        if copy_to_user(tmp, &cmd.regs as *const _ as *const core::ffi::c_void, 16) != 0 {
            err = -EFAULT;
            break;
        }
        tmp = tmp.add(16);
        bytes += 16;
        *ppos = (pos.wrapping_add(1)) as loff_t;
        pos = pos.wrapping_add(1);
        count -= 16;
        reinit_completion(&mut cmd.done);
    }

    if bytes != 0 { bytes } else { err as ssize_t }
}

unsafe fn cpuid_open(inode: *mut inode, _file: *mut file) -> i32 {
    let cpu: u32 = iminor(file_inode_from_inode(inode)) as u32;
    if cpu >= nr_cpu_ids || !cpu_online(cpu) {
        return -ENXIO; // No such CPU
    }

    let c: *mut cpuinfo_x86 = &mut cpu_data(cpu);
    if (*c).cpuid_level < 0 {
        return -EIO; // CPUID not supported
    }

    0
}

/*
 * File operations we support
 */
static cpuid_fops: file_operations = file_operations {
    owner: THIS_MODULE,
    llseek: no_seek_end_llseek,
    read: cpuid_read_f,
    open: cpuid_open,
};

unsafe fn cpuid_devnode(dev: *const device, mode: *mut umode_t) -> *mut core::ffi::c_char {
    kasprintf(GFP_KERNEL, b"cpu/%u/cpuid\0".as_ptr() as *const core::ffi::c_char, MINOR((*dev).devt))
}

static cpuid_class: class = class {
    name: b"cpuid\0".as_ptr() as *const core::ffi::c_char,
    devnode: cpuid_devnode,
};

unsafe fn cpuid_device_create(cpu: u32) -> i32 {
    let dev = device_create(
        &cpuid_class,
        core::ptr::null_mut(),
        MKDEV(CPUID_MAJOR, cpu),
        core::ptr::null_mut(),
        b"cpu%d\0".as_ptr() as *const core::ffi::c_char,
        cpu,
    );
    PTR_ERR_OR_ZERO(dev)
}

unsafe fn cpuid_device_destroy(cpu: u32) -> i32 {
    device_destroy(&cpuid_class, MKDEV(CPUID_MAJOR, cpu));
    0
}

unsafe fn cpuid_init() -> i32 {
    let mut err: i32;

    if __register_chrdev(CPUID_MAJOR, 0, NR_CPUS, b"cpu/cpuid\0".as_ptr() as *const core::ffi::c_char, &cpuid_fops) != 0 {
        printk(KERN_ERR, b"cpuid: unable to get major %d for cpuid\n\0".as_ptr() as *const core::ffi::c_char, CPUID_MAJOR);
        return -EBUSY;
    }
    err = class_register(&cpuid_class);
    if err != 0 {
        goto_out_chrdev(err);
    }

    err = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, b"x86/cpuid:online\0".as_ptr() as *const core::ffi::c_char, cpuid_device_create, cpuid_device_destroy);
    if err < 0 {
        class_unregister(&cpuid_class);
        __unregister_chrdev(CPUID_MAJOR, 0, NR_CPUS, b"cpu/cpuid\0".as_ptr() as *const core::ffi::c_char);
        return err;
    }

    cpuhp_cpuid_state = err as enum_cpuhp_state;
    0
}

unsafe fn goto_out_chrdev(err: i32) -> i32 {
    __unregister_chrdev(CPUID_MAJOR, 0, NR_CPUS, b"cpu/cpuid\0".as_ptr() as *const core::ffi::c_char);
    err
}

unsafe fn cpuid_exit() {
    cpuhp_remove_state(cpuhp_cpuid_state);
    class_unregister(&cpuid_class);
    __unregister_chrdev(CPUID_MAJOR, 0, NR_CPUS, b"cpu/cpuid\0".as_ptr() as *const core::ffi::c_char);
}

module_init!(cpuid_init);
module_exit!(cpuid_exit);

module_author!("H. Peter Anvin <hpa@zytor.com>");
module_description!("x86 generic CPUID driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
