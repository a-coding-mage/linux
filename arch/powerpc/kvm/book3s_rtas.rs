// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2012 Michael Ellerman, IBM Corporation.
 */

// Kernel headers and build-time configuration are supplied by the surrounding kernel crate.

#[cfg(CONFIG_KVM_XICS)]
unsafe fn kvm_rtas_set_xive(vcpu: *mut kvm_vcpu, args: *mut rtas_args) {
    let mut rc: i32;
    if be32_to_cpu((*args).nargs) != 3 || be32_to_cpu((*args).nret) != 1 {
        rc = -3;
    } else {
        let irq = be32_to_cpu((*args).args[0]);
        let server = be32_to_cpu((*args).args[1]);
        let priority = be32_to_cpu((*args).args[2]);
        rc = if xics_on_xive() {
            kvmppc_xive_set_xive((*vcpu).kvm, irq, server, priority)
        } else {
            kvmppc_xics_set_xive((*vcpu).kvm, irq, server, priority)
        };
        if rc != 0 { rc = -3; }
    }
    (*args).rets[0] = cpu_to_be32(rc);
}

#[cfg(CONFIG_KVM_XICS)]
unsafe fn kvm_rtas_get_xive(vcpu: *mut kvm_vcpu, args: *mut rtas_args) {
    let mut rc: i32;
    if be32_to_cpu((*args).nargs) != 1 || be32_to_cpu((*args).nret) != 3 {
        rc = -3;
    } else {
        let irq = be32_to_cpu((*args).args[0]);
        let mut server: u32 = 0;
        let mut priority: u32 = 0;
        rc = if xics_on_xive() {
            kvmppc_xive_get_xive((*vcpu).kvm, irq, &mut server, &mut priority)
        } else {
            kvmppc_xics_get_xive((*vcpu).kvm, irq, &mut server, &mut priority)
        };
        if rc != 0 {
            rc = -3;
        } else {
            (*args).rets[1] = cpu_to_be32(server);
            (*args).rets[2] = cpu_to_be32(priority);
        }
    }
    (*args).rets[0] = cpu_to_be32(rc);
}

#[cfg(CONFIG_KVM_XICS)]
unsafe fn kvm_rtas_int_off(vcpu: *mut kvm_vcpu, args: *mut rtas_args) {
    let mut rc: i32;
    if be32_to_cpu((*args).nargs) != 1 || be32_to_cpu((*args).nret) != 1 {
        rc = -3;
    } else {
        let irq = be32_to_cpu((*args).args[0]);
        rc = if xics_on_xive() { kvmppc_xive_int_off((*vcpu).kvm, irq) }
             else { kvmppc_xics_int_off((*vcpu).kvm, irq) };
        if rc != 0 { rc = -3; }
    }
    (*args).rets[0] = cpu_to_be32(rc);
}

#[cfg(CONFIG_KVM_XICS)]
unsafe fn kvm_rtas_int_on(vcpu: *mut kvm_vcpu, args: *mut rtas_args) {
    let mut rc: i32;
    if be32_to_cpu((*args).nargs) != 1 || be32_to_cpu((*args).nret) != 1 {
        rc = -3;
    } else {
        let irq = be32_to_cpu((*args).args[0]);
        rc = if xics_on_xive() { kvmppc_xive_int_on((*vcpu).kvm, irq) }
             else { kvmppc_xics_int_on((*vcpu).kvm, irq) };
        if rc != 0 { rc = -3; }
    }
    (*args).rets[0] = cpu_to_be32(rc);
}

#[repr(C)]
struct rtas_handler {
    handler: Option<unsafe fn(*mut kvm_vcpu, *mut rtas_args)>,
    name: *mut u8,
}

#[cfg(CONFIG_KVM_XICS)]
static mut RTAS_HANDLERS: [rtas_handler; 4] = [
    rtas_handler { name: b"ibm,set-xive\0" as *const _ as *mut u8, handler: Some(kvm_rtas_set_xive) },
    rtas_handler { name: b"ibm,get-xive\0" as *const _ as *mut u8, handler: Some(kvm_rtas_get_xive) },
    rtas_handler { name: b"ibm,int-off\0" as *const _ as *mut u8, handler: Some(kvm_rtas_int_off) },
    rtas_handler { name: b"ibm,int-on\0" as *const _ as *mut u8, handler: Some(kvm_rtas_int_on) },
];
#[cfg(not(CONFIG_KVM_XICS))]
static mut RTAS_HANDLERS: [rtas_handler; 0] = [];

#[repr(C)]
struct rtas_token_definition {
    list: list_head,
    handler: *mut rtas_handler,
    token: u64,
}

unsafe fn rtas_name_matches(s1: *mut u8, s2: *mut u8) -> i32 {
    (!strncmp(s1, s2, core::mem::size_of::<kvm_rtas_token_args>())) as i32
}

unsafe fn rtas_token_undefine(kvm: *mut kvm, name: *mut u8) -> i32 {
    lockdep_assert_held(&mut (*kvm).arch.rtas_token_lock);
    let mut d: *mut rtas_token_definition = core::ptr::null_mut();
    let mut tmp: *mut rtas_token_definition = core::ptr::null_mut();
    list_for_each_entry_safe!(d, tmp, &mut (*kvm).arch.rtas_tokens, list, {
        if rtas_name_matches((*(*d).handler).name, name) != 0 {
            list_del(&mut (*d).list); kfree(d); return 0;
        }
    });
    0
}

unsafe fn rtas_token_define(kvm: *mut kvm, name: *mut u8, token: u64) -> i32 {
    lockdep_assert_held(&mut (*kvm).arch.rtas_token_lock);
    let mut d: *mut rtas_token_definition = core::ptr::null_mut();
    list_for_each_entry!(&mut (*kvm).arch.rtas_tokens, d, list, {
        if (*d).token == token { return -EEXIST; }
    });
    let mut h: *mut rtas_handler = core::ptr::null_mut();
    let mut found = false;
    for i in 0..RTAS_HANDLERS.len() {
        h = &mut RTAS_HANDLERS[i];
        if rtas_name_matches((*h).name, name) != 0 { found = true; break; }
    }
    if !found { return -ENOENT; }
    d = kzalloc_obj::<rtas_token_definition>();
    if d.is_null() { return -ENOMEM; }
    (*d).handler = h; (*d).token = token;
    list_add_tail(&mut (*d).list, &mut (*kvm).arch.rtas_tokens);
    0
}

pub unsafe fn kvm_vm_ioctl_rtas_define_token(kvm: *mut kvm, argp: *mut core::ffi::c_void) -> i32 {
    let mut args: kvm_rtas_token_args = core::mem::zeroed();
    if copy_from_user(&mut args, argp, core::mem::size_of::<kvm_rtas_token_args>()) != 0 { return -EFAULT; }
    mutex_lock(&mut (*kvm).arch.rtas_token_lock);
    let rc = if args.token != 0 { rtas_token_define(kvm, args.name, args.token) } else { rtas_token_undefine(kvm, args.name) };
    mutex_unlock(&mut (*kvm).arch.rtas_token_lock);
    rc
}

pub unsafe fn kvmppc_rtas_hcall(vcpu: *mut kvm_vcpu) -> i32 {
    let mut args: rtas_args = core::mem::zeroed();
    let args_phys = kvmppc_get_gpr(vcpu, 4) & KVM_PAM;
    kvm_vcpu_srcu_read_lock(vcpu);
    let mut rc = kvm_read_guest((*vcpu).kvm, args_phys, &mut args, core::mem::size_of::<rtas_args>());
    kvm_vcpu_srcu_read_unlock(vcpu);
    if rc != 0 { return rc; }
    let orig_rets = args.rets;
    if be32_to_cpu(args.nargs) >= args.args.len() { return -EINVAL; }
    args.rets = args.args.as_mut_ptr().add(be32_to_cpu(args.nargs) as usize);
    mutex_lock(&mut (*vcpu).kvm.arch.rtas_token_lock);
    rc = -ENOENT;
    let mut d: *mut rtas_token_definition = core::ptr::null_mut();
    list_for_each_entry!(&mut (*vcpu).kvm.arch.rtas_tokens, d, list, {
        if (*d).token == be32_to_cpu(args.token) {
            ((*(*d).handler).handler.unwrap())(vcpu, &mut args); rc = 0; break;
        }
    });
    mutex_unlock(&mut (*vcpu).kvm.arch.rtas_token_lock);
    if rc == 0 { args.rets = orig_rets; rc = kvm_write_guest((*vcpu).kvm, args_phys, &args, core::mem::size_of::<rtas_args>()); }
    rc
}

pub unsafe fn kvmppc_rtas_tokens_free(kvm: *mut kvm) {
    let mut d: *mut rtas_token_definition = core::ptr::null_mut();
    let mut tmp: *mut rtas_token_definition = core::ptr::null_mut();
    list_for_each_entry_safe!(d, tmp, &mut (*kvm).arch.rtas_tokens, list, {
        list_del(&mut (*d).list); kfree(d);
    });
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
