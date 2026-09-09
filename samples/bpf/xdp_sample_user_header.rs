// SPDX-License-Identifier: GPL-2.0-only
// Translated from xdp_sample_user.h.
// Dependencies supplied by the surrounding build are intentionally not defined here.

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StatsMask {
    _SAMPLE_REDIRECT_MAP = 1u32 << 0,
    SAMPLE_RX_CNT = 1u32 << 1,
    SAMPLE_REDIRECT_ERR_CNT = 1u32 << 2,
    SAMPLE_CPUMAP_ENQUEUE_CNT = 1u32 << 3,
    SAMPLE_CPUMAP_KTHREAD_CNT = 1u32 << 4,
    SAMPLE_EXCEPTION_CNT = 1u32 << 5,
    SAMPLE_DEVMAP_XMIT_CNT = 1u32 << 6,
    SAMPLE_REDIRECT_CNT = 1u32 << 7,
    SAMPLE_REDIRECT_MAP_CNT = (1u32 << 7) | (1u32 << 0),
    SAMPLE_REDIRECT_ERR_MAP_CNT = (1u32 << 2) | (1u32 << 0),
    SAMPLE_DEVMAP_XMIT_CNT_MULTI = 1u32 << 8,
    SAMPLE_SKIP_HEADING = 1u32 << 9,
}

/* Exit return codes */
pub const EXIT_OK: i32 = 0;
pub const EXIT_FAIL: i32 = 1;
pub const EXIT_FAIL_OPTION: i32 = 2;
pub const EXIT_FAIL_XDP: i32 = 3;
pub const EXIT_FAIL_BPF: i32 = 4;
pub const EXIT_FAIL_MEM: i32 = 5;

extern "C" {
    pub fn sample_setup_maps(maps: *mut *mut bpf_map) -> i32;
    pub fn __sample_init(mask: i32) -> i32;
    pub fn sample_exit(status: i32);
    pub fn sample_run(interval: i32, post_cb: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, ctx: *mut core::ffi::c_void) -> i32;

    pub fn sample_switch_mode();
    pub fn sample_install_xdp(xdp_prog: *mut bpf_program, ifindex: i32, generic: bool, force: bool) -> i32;
    pub fn sample_usage(argv: *mut *mut core::ffi::c_char, long_options: *const option,
                        doc: *const core::ffi::c_char, mask: i32, error: bool);

    pub fn get_driver_name(ifindex: i32) -> *const core::ffi::c_char;
    pub fn get_mac_addr(ifindex: i32, mac_addr: *mut core::ffi::c_void) -> i32;

    pub fn strncpy(dst: *mut core::ffi::c_char, src: *const core::ffi::c_char, size: usize)
        -> *mut core::ffi::c_char;
}

pub unsafe fn safe_strncpy(
    dst: *mut core::ffi::c_char,
    src: *const core::ffi::c_char,
    size: usize,
) -> *mut core::ffi::c_char {
    if size == 0 {
        return dst;
    }
    strncpy(dst, src, size - 1);
    *dst.add(size - 1) = 0;
    dst
}

/* C macro __attach_tp(name): attach a tracing program and return negative errno on failure. */
#[macro_export]
macro_rules! __attach_tp {
    ($skel:expr, $name:ident) => {{
        if bpf_program__type($skel.progs.$name) != BPF_PROG_TYPE_TRACING {
            return -EINVAL;
        }
        $skel.links.$name = bpf_program__attach($skel.progs.$name);
        if $skel.links.$name.is_null() {
            return -errno;
        }
    }};
}

/* C macro sample_init_pre_load(skel), preserved as an explicit Rust macro. */
#[macro_export]
macro_rules! sample_init_pre_load {
    ($skel:expr) => {{
        $skel.rodata.nr_cpus = libbpf_num_possible_cpus();
        sample_setup_maps([
            $skel.maps.rx_cnt, $skel.maps.redir_err_cnt,
            $skel.maps.cpumap_enqueue_cnt, $skel.maps.cpumap_kthread_cnt,
            $skel.maps.exception_cnt, $skel.maps.devmap_xmit_cnt,
            $skel.maps.devmap_xmit_cnt_multi,
        ].as_mut_ptr());
    }};
}

/* DEFINE_SAMPLE_INIT(name) is a declaration-generating C macro; its control flow is retained here. */
#[macro_export]
macro_rules! DEFINE_SAMPLE_INIT {
    ($name:ident) => {
        unsafe fn sample_init(skel: *mut $name, mask: i32) -> i32 {
            let ret = __sample_init(mask);
            if ret < 0 { return ret; }
            if (mask & SAMPLE_REDIRECT_MAP_CNT as i32) != 0 { __attach_tp!((*skel), tp_xdp_redirect_map); }
            if (mask & SAMPLE_REDIRECT_CNT as i32) != 0 { __attach_tp!((*skel), tp_xdp_redirect); }
            if (mask & SAMPLE_REDIRECT_ERR_MAP_CNT as i32) != 0 { __attach_tp!((*skel), tp_xdp_redirect_map_err); }
            if (mask & SAMPLE_REDIRECT_ERR_CNT as i32) != 0 { __attach_tp!((*skel), tp_xdp_redirect_err); }
            if (mask & SAMPLE_CPUMAP_ENQUEUE_CNT as i32) != 0 { __attach_tp!((*skel), tp_xdp_cpumap_enqueue); }
            if (mask & SAMPLE_CPUMAP_KTHREAD_CNT as i32) != 0 { __attach_tp!((*skel), tp_xdp_cpumap_kthread); }
            if (mask & SAMPLE_EXCEPTION_CNT as i32) != 0 { __attach_tp!((*skel), tp_xdp_exception); }
            if (mask & SAMPLE_DEVMAP_XMIT_CNT as i32) != 0 { __attach_tp!((*skel), tp_xdp_devmap_xmit); }
            if (mask & SAMPLE_DEVMAP_XMIT_CNT_MULTI as i32) != 0 { __attach_tp!((*skel), tp_xdp_devmap_xmit_multi); }
            0
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
