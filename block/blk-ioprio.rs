// SPDX-License-Identifier: GPL-2.0
/*
 * Block rq-qos policy for assigning an I/O priority class to requests.
 *
 * Using an rq-qos policy for assigning I/O priority class has two advantages
 * over using the ioprio_set() system call:
 *
 * - This policy is cgroup based so it has all the advantages of cgroups.
 * - While ioprio_set() does not affect page cache writeback I/O, this rq-qos
 *   controller affects page cache writeback I/O for filesystems that support
 *   assiociating a cgroup with writeback I/O. See also
 *   Documentation/admin-guide/cgroup-v2.rst.
 */

// Dependencies supplied by the kernel headers and companion translation units.

#[repr(C)]
pub struct blkcg_policy_data { _private: [u8; 0] }
#[repr(C)]
pub struct blkcg { _private: [u8; 0] }
#[repr(C)]
pub struct cgroup_subsys_state { _private: [u8; 0] }
#[repr(C)]
pub struct seq_file { _private: [u8; 0] }
#[repr(C)]
pub struct kernfs_open_file { _private: [u8; 0] }
#[repr(C)]
pub struct bio { pub bi_blkg: *mut bio_blkg, pub bi_ioprio: u16 }
#[repr(C)]
pub struct bio_blkg { pub blkcg: *mut blkcg }
#[repr(C)]
pub struct blkcg_policy { _private: [u8; 0] }
#[repr(C)]
pub struct cftype { _private: [u8; 0] }

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum prio_policy {
    POLICY_NO_CHANGE = 0,
    POLICY_PROMOTE_TO_RT = 1,
    POLICY_RESTRICT_TO_BE = 2,
    POLICY_ALL_TO_IDLE = 3,
    POLICY_NONE_TO_RT = 4,
}

static policy_name: [&'static [u8]; 5] = [
    b"no-change\0", b"promote-to-rt\0", b"restrict-to-be\0", b"idle\0", b"none-to-rt\0",
];

static mut ioprio_policy: blkcg_policy = blkcg_policy { _private: [] };

#[repr(C)]
struct ioprio_blkcg {
    cpd: blkcg_policy_data,
    prio_policy: prio_policy,
}

unsafe fn blkcg_to_ioprio_blkcg(blkcg: *mut blkcg) -> *mut ioprio_blkcg {
    container_of(blkcg_to_cpd(blkcg, &raw mut ioprio_policy), 0)
}

unsafe fn ioprio_blkcg_from_css(css: *mut cgroup_subsys_state) -> *mut ioprio_blkcg {
    blkcg_to_ioprio_blkcg(css_to_blkcg(css))
}

unsafe fn ioprio_show_prio_policy(sf: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let blkcg = ioprio_blkcg_from_css(seq_css(sf));
    seq_printf(sf, b"%s\n\0".as_ptr(), policy_name[(*blkcg).prio_policy as usize].as_ptr());
    0
}

unsafe fn ioprio_set_prio_policy(of: *mut kernfs_open_file, buf: *mut i8,
                                 nbytes: usize, off: i64) -> isize {
    let blkcg = ioprio_blkcg_from_css(of_css(of));
    if off != 0 { return -5; } // -EIO
    // kernfs_fop_write_iter() terminates `buf` with `\0`.
    let ret = sysfs_match_string(policy_name.as_ptr(), buf);
    if ret < 0 { return ret as isize; }
    (*blkcg).prio_policy = core::mem::transmute(ret as i32);
    nbytes as isize
}

unsafe fn ioprio_alloc_cpd(_gfp: usize) -> *mut blkcg_policy_data {
    let blkcg = kzalloc_ioprio_blkcg();
    if blkcg.is_null() { return core::ptr::null_mut(); }
    (*blkcg).prio_policy = prio_policy::POLICY_NO_CHANGE;
    &mut (*blkcg).cpd
}

unsafe fn ioprio_free_cpd(cpd: *mut blkcg_policy_data) {
    let blkcg = container_of(cpd, 0);
    kfree(blkcg);
}

pub unsafe fn blkcg_set_ioprio(bio: *mut bio) {
    let blkcg = blkcg_to_ioprio_blkcg((*(*bio).bi_blkg).blkcg);
    if blkcg.is_null() || (*blkcg).prio_policy == prio_policy::POLICY_NO_CHANGE { return; }
    if (*blkcg).prio_policy == prio_policy::POLICY_PROMOTE_TO_RT ||
       (*blkcg).prio_policy == prio_policy::POLICY_NONE_TO_RT {
        /* For RT threads, the default priority level is 4 because task_nice is 0. */
        if IOPRIO_PRIO_CLASS((*bio).bi_ioprio) != IOPRIO_CLASS_RT {
            (*bio).bi_ioprio = IOPRIO_PRIO_VALUE(IOPRIO_CLASS_RT, 4);
        }
        return;
    }
    let prio = core::cmp::max((*bio).bi_ioprio,
        IOPRIO_PRIO_VALUE((*blkcg).prio_policy as u16, 0));
    if prio > (*bio).bi_ioprio { (*bio).bi_ioprio = prio; }
}

unsafe fn ioprio_init() -> i32 { blkcg_policy_register(&raw mut ioprio_policy) }
unsafe fn ioprio_exit() { blkcg_policy_unregister(&raw mut ioprio_policy); }

// External kernel symbols and macros.
extern "C" {
    fn blkcg_to_cpd(blkcg: *mut blkcg, policy: *mut blkcg_policy) -> *mut blkcg_policy_data;
    fn container_of<T>(ptr: *mut T, _offset: usize) -> *mut ioprio_blkcg;
    fn css_to_blkcg(css: *mut cgroup_subsys_state) -> *mut blkcg;
    fn seq_css(sf: *mut seq_file) -> *mut cgroup_subsys_state;
    fn seq_printf(sf: *mut seq_file, fmt: *const u8, ...) -> i32;
    fn of_css(of: *mut kernfs_open_file) -> *mut cgroup_subsys_state;
    fn sysfs_match_string(strings: *const *const u8, buf: *mut i8) -> i32;
    fn kzalloc_ioprio_blkcg() -> *mut ioprio_blkcg;
    fn kfree(ptr: *mut ioprio_blkcg);
    fn blkcg_policy_register(policy: *mut blkcg_policy) -> i32;
    fn blkcg_policy_unregister(policy: *mut blkcg_policy);
    fn IOPRIO_PRIO_CLASS(prio: u16) -> u16;
    fn IOPRIO_PRIO_VALUE(class: u16, data: u16) -> u16;
}

const IOPRIO_CLASS_RT: u16 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
