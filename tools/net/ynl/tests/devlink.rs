// SPDX-License-Identifier: GPL-2.0
//
// Translated from C source using kselftest harness, YNL, and generated
// devlink-user bindings supplied externally.

use std::os::raw::{c_char, c_uint, c_void};

#[repr(C)]
pub struct ynl_family {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ynl_sock_err {
    pub msg: *const c_char,
}

#[repr(C)]
pub struct ynl_sock {
    pub err: ynl_sock_err,
}

#[repr(C)]
pub struct devlink_get_len {
    pub bus_name: usize,
    pub dev_name: usize,
}

#[repr(C)]
pub struct devlink_get {
    pub _len: devlink_get_len,
    pub bus_name: *const c_char,
    pub dev_name: *const c_char,
}

#[repr(C)]
pub struct devlink_get_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct devlink_info_get_req {
    _private: [u8; 0],
}

#[repr(C)]
pub struct devlink_info_version {
    pub info_version_name: *const c_char,
    pub info_version_value: *const c_char,
}

#[repr(C)]
pub struct devlink_info_get_rsp_len {
    pub info_driver_name: usize,
}

#[repr(C)]
pub struct devlink_info_get_rsp_count {
    pub info_version_running: c_uint,
}

#[repr(C)]
pub struct devlink_info_get_rsp {
    pub _len: devlink_info_get_rsp_len,
    pub _count: devlink_info_get_rsp_count,
    pub info_driver_name: *const c_char,
    pub info_version_running: *mut devlink_info_version,
}

unsafe extern "C" {
    static ynl_devlink_family: ynl_family;

    fn ynl_sock_create(family: *const ynl_family, yarg: *mut c_void) -> *mut ynl_sock;
    fn ynl_sock_destroy(ys: *mut ynl_sock);
    fn ynl_dump_empty(devs: *mut devlink_get_list) -> bool;

    fn devlink_get_dump(ys: *mut ynl_sock) -> *mut devlink_get_list;
    fn devlink_get_list_free(devs: *mut devlink_get_list);

    fn devlink_info_get_req_alloc() -> *mut devlink_info_get_req;
    fn devlink_info_get_req_set_bus_name(req: *mut devlink_info_get_req, bus_name: *const c_char);
    fn devlink_info_get_req_set_dev_name(req: *mut devlink_info_get_req, dev_name: *const c_char);
    fn devlink_info_get(ys: *mut ynl_sock, req: *mut devlink_info_get_req) -> *mut devlink_info_get_rsp;
    fn devlink_info_get_req_free(req: *mut devlink_info_get_req);
    fn devlink_info_get_rsp_free(rsp: *mut devlink_info_get_rsp);
}

// Original C harness fixture:
// FIXTURE(devlink) { struct ynl_sock *ys; };
pub struct devlink {
    pub ys: *mut ynl_sock,
}

// Original C harness setup:
// FIXTURE_SETUP(devlink)
pub unsafe fn devlink_setup(self_: *mut devlink) {
    unsafe {
        (*self_).ys = ynl_sock_create(&raw const ynl_devlink_family, std::ptr::null_mut());
        ASSERT_NE!(std::ptr::null_mut(), (*self_).ys, {
            TH_LOG!("failed to create devlink socket");
        });
    }
}

// Original C harness teardown:
// FIXTURE_TEARDOWN(devlink)
pub unsafe fn devlink_teardown(self_: *mut devlink) {
    unsafe {
        ynl_sock_destroy((*self_).ys);
    }
}

// Original C harness test:
// TEST_F(devlink, dump)
pub unsafe fn devlink_dump(self_: *mut devlink) {
    unsafe {
        let devs: *mut devlink_get_list;

        devs = devlink_get_dump((*self_).ys);
        ASSERT_NE!(std::ptr::null_mut(), devs, {
            TH_LOG!("dump failed: %s", (*(*self_).ys).err.msg);
        });

        if ynl_dump_empty(devs) {
            devlink_get_list_free(devs);
            SKIP!(return, "no entries in dump");
        }

        ynl_dump_foreach!(devs, d, {
            EXPECT_TRUE!((*d)._len.bus_name as bool);
            EXPECT_TRUE!((*d)._len.dev_name as bool);
            ksft_print_msg!("%s/%s\n", (*d).bus_name, (*d).dev_name);
        });

        devlink_get_list_free(devs);
    }
}

// Original C harness test:
// TEST_F(devlink, info)
pub unsafe fn devlink_info(self_: *mut devlink) {
    unsafe {
        let devs: *mut devlink_get_list;

        devs = devlink_get_dump((*self_).ys);
        ASSERT_NE!(std::ptr::null_mut(), devs, {
            TH_LOG!("dump failed: %s", (*(*self_).ys).err.msg);
        });

        if ynl_dump_empty(devs) {
            devlink_get_list_free(devs);
            SKIP!(return, "no devices to query");
        }

        ynl_dump_foreach!(devs, d, {
            let info_req: *mut devlink_info_get_req;
            let info_rsp: *mut devlink_info_get_rsp;
            let mut i: c_uint;

            EXPECT_TRUE!((*d)._len.bus_name as bool);
            EXPECT_TRUE!((*d)._len.dev_name as bool);
            ksft_print_msg!("%s/%s:\n", (*d).bus_name, (*d).dev_name);

            info_req = devlink_info_get_req_alloc();
            ASSERT_NE!(std::ptr::null_mut(), info_req);
            devlink_info_get_req_set_bus_name(info_req, (*d).bus_name);
            devlink_info_get_req_set_dev_name(info_req, (*d).dev_name);

            info_rsp = devlink_info_get((*self_).ys, info_req);
            devlink_info_get_req_free(info_req);
            ASSERT_NE!(std::ptr::null_mut(), info_rsp, {
                devlink_get_list_free(devs);
                TH_LOG!("info_get failed: %s", (*(*self_).ys).err.msg);
            });

            EXPECT_TRUE!((*info_rsp)._len.info_driver_name as bool);
            if (*info_rsp)._len.info_driver_name != 0 {
                ksft_print_msg!("  driver: %s\n", (*info_rsp).info_driver_name);
            }
            if (*info_rsp)._count.info_version_running != 0 {
                ksft_print_msg!("  running fw:\n");
            }
            i = 0;
            while i < (*info_rsp)._count.info_version_running {
                let version = (*info_rsp).info_version_running.add(i as usize);
                ksft_print_msg!(
                    "    %s: %s\n",
                    (*version).info_version_name,
                    (*version).info_version_value
                );
                i += 1;
            }
            devlink_info_get_rsp_free(info_rsp);
        });
        devlink_get_list_free(devs);
    }
}

// Original C harness entry point:
// TEST_HARNESS_MAIN
TEST_HARNESS_MAIN!();

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
