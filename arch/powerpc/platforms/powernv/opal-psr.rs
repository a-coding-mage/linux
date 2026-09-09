// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PowerNV OPAL Power-Shift-Ratio interface
 *
 * Copyright 2017 IBM Corp.
 */

// Dependencies supplied by the surrounding kernel/Rust environment.

static mut PSR_MUTEX: Mutex = DEFINE_MUTEX!();

static mut psr_kobj: *mut kobject = core::ptr::null_mut();

#[repr(C)]
struct psr_attr {
	 handle: u32,
	 attr: kobj_attribute,
}

static mut psr_attrs: *mut psr_attr = core::ptr::null_mut();

unsafe fn psr_show(
	 kobj: *mut kobject,
	 attr: *mut kobj_attribute,
	 buf: *mut core::ffi::c_char,
) -> isize {
	 let psr_attr = container_of!(attr, psr_attr, attr);
	 let mut msg: opal_msg = core::mem::zeroed();
	 let mut psr: i32 = 0;
	 let mut ret: i32;
	 let token: i32;

	 token = opal_async_get_token_interruptible();
	 if token < 0 {
		 pr_devel!("Failed to get token\n");
		 return token as isize;
	 }

	 ret = mutex_lock_interruptible(&raw mut PSR_MUTEX);
	 if ret != 0 {
		 goto!(out_token);
	 }

	 ret = opal_get_power_shift_ratio((*psr_attr).handle, token,
		 (__pa!((&raw mut psr) as *mut core::ffi::c_void)) as u32);
	 match ret {
		 OPAL_ASYNC_COMPLETION => {
			 ret = opal_async_wait_response(token, &raw mut msg);
			 if ret != 0 {
				 pr_devel!("Failed to wait for the async response\n");
				 ret = -EIO;
				 goto!(out);
			 }
			 ret = opal_error_code(opal_get_async_rc(msg));
			 if ret == 0 {
				 ret = sysfs_emit(buf, c"%u\n", be32_to_cpu(psr as u32));
			 }
		 }
		 OPAL_SUCCESS => {
			 ret = sysfs_emit(buf, c"%u\n", be32_to_cpu(psr as u32));
		 }
		 _ => ret = opal_error_code(ret),
	 }

out:
	 mutex_unlock(&raw mut PSR_MUTEX);
out_token:
	 opal_async_release_token(token);
	 ret as isize
}

unsafe fn psr_store(
	 kobj: *mut kobject,
	 attr: *mut kobj_attribute,
	 buf: *const core::ffi::c_char,
	 count: usize,
) -> isize {
	 let psr_attr = container_of!(attr, psr_attr, attr);
	 let mut msg: opal_msg = core::mem::zeroed();
	 let mut psr: i32 = 0;
	 let mut ret: i32;
	 let token: i32;

	 ret = kstrtoint(buf, 0, &raw mut psr);
	 if ret != 0 {
		 return ret as isize;
	 }

	 token = opal_async_get_token_interruptible();
	 if token < 0 {
		 pr_devel!("Failed to get token\n");
		 return token as isize;
	 }

	 ret = mutex_lock_interruptible(&raw mut PSR_MUTEX);
	 if ret != 0 {
		 goto!(out_token);
	 }

	 ret = opal_set_power_shift_ratio((*psr_attr).handle, token, psr);
	 match ret {
		 OPAL_ASYNC_COMPLETION => {
			 ret = opal_async_wait_response(token, &raw mut msg);
			 if ret != 0 {
				 pr_devel!("Failed to wait for the async response\n");
				 ret = -EIO;
				 goto!(out);
			 }
			 ret = opal_error_code(opal_get_async_rc(msg));
			 if ret == 0 { ret = count as i32; }
		 }
		 OPAL_SUCCESS => ret = count as i32,
		 _ => ret = opal_error_code(ret),
	 }

out:
	 mutex_unlock(&raw mut PSR_MUTEX);
out_token:
	 opal_async_release_token(token);
	 ret as isize
}

unsafe fn opal_psr_init() {
	 let mut psr: *mut device_node;
	 let mut node: *mut device_node;
	 let mut i: i32 = 0;

	 psr = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(),
		 c"ibm,opal-power-shift-ratio");
	 if psr.is_null() {
		 pr_devel!("Power-shift-ratio node not found\n");
		 return;
	 }

	 psr_attrs = kzalloc_objs!(*psr_attrs, of_get_child_count(psr));
	 if psr_attrs.is_null() { goto!(out_put_psr); }

	 psr_kobj = kobject_create_and_add(c"psr", opal_kobj);
	 if psr_kobj.is_null() {
		 pr_warn!("Failed to create psr kobject\n");
		 goto!(out);
	 }

	 for_each_child_of_node!(psr, node) {
		 if of_property_read_u32(node, c"handle", &raw mut (*psr_attrs.add(i as usize)).handle) != 0 { goto!(out_kobj); }
		 sysfs_attr_init(&raw mut (*psr_attrs.add(i as usize)).attr.attr);
		 if of_property_read_string(node, c"label", &raw mut (*psr_attrs.add(i as usize)).attr.attr.name) != 0 { goto!(out_kobj); }
		 (*psr_attrs.add(i as usize)).attr.attr.mode = 0o664;
		 (*psr_attrs.add(i as usize)).attr.show = Some(psr_show);
		 (*psr_attrs.add(i as usize)).attr.store = Some(psr_store);
		 if sysfs_create_file(psr_kobj, &raw mut (*psr_attrs.add(i as usize)).attr.attr) != 0 {
			 pr_devel!("Failed to create psr sysfs file %s\n", (*psr_attrs.add(i as usize)).attr.attr.name);
			 goto!(out_kobj);
		 }
		 i += 1;
	 }
	 of_node_put(psr);
	 return;
out_kobj:
	 of_node_put(node);
	 kobject_put(psr_kobj);
out:
	 kfree(psr_attrs as *mut core::ffi::c_void);
out_put_psr:
	 of_node_put(psr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
