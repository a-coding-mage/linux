// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020, Oracle and/or its affiliates. */

/* Translated from testing/selftests/bpf/progs/netif_receive_skb.c.
 * Depends on Rust bindings/equivalents for btf_ptr.h, bpf_helpers.h,
 * bpf_tracing.h, bpf_core_read.h, bpf_misc.h, errno.h, and kernel BTF types.
 */

pub static mut ret: c_long = 0;
pub static mut num_subtests: c_int = 0;
pub static mut ran_subtests: c_int = 0;
pub static mut skip: bool = false;

pub const STRSIZE: usize = 2048;
pub const EXPECTED_STRSIZE: usize = 256;

#[cfg(bpf_target_s390)]
pub const BADPTR: *mut c_void = 0xFFFFFFFFFFFFF000u64 as *mut c_void;
#[cfg(not(bpf_target_s390))]
pub const BADPTR: *mut c_void = core::ptr::null_mut();

#[repr(C)]
pub struct strdata_map {
    /* __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY); */
    /* __uint(max_entries, 1); */
    /* __type(key, __u32); */
    /* __type(value, char[STRSIZE]); */
    _private: [u8; 0],
}

#[link_section = ".maps"]
pub static mut strdata: strdata_map = strdata_map { _private: [] };

unsafe fn __strncmp(m1: *const c_void, m2: *const c_void, len: usize) -> c_int {
    let s1 = m1 as *const c_uchar;
    let s2 = m2 as *const c_uchar;
    let mut i: c_int;
    let mut delta: c_int = 0;

    i = 0;
    while (i as usize) < len {
        delta = *s1.add(i as usize) as c_int - *s2.add(i as usize) as c_int;
        if delta != 0 || *s1.add(i as usize) == 0 || *s2.add(i as usize) == 0 {
            break;
        }
        i += 1;
    }
    delta
}

/* C condition: #if __has_builtin(__builtin_btf_type_id) */
macro_rules! test_btf {
    ($str_:expr, $type_:ty, $flags_:expr, $expected_:expr, $ptrdata_:expr) => {{
        static EXPECTEDVAL: [c_char; EXPECTED_STRSIZE] = const_c_str_array::<EXPECTED_STRSIZE>($expected_);
        let hflags: __u64 = ($flags_ as __u64) | BTF_F_COMPACT as __u64;
        static mut PTRDATA: $type_ = $ptrdata_;
        static mut PTR: btf_ptr = btf_ptr {
            ptr: core::ptr::null_mut(),
            type_id: 0,
        };
        let cmp: c_int;

        num_subtests += 1;
        if ret < 0 {
            break;
        }
        ran_subtests += 1;
        PTR.ptr = core::ptr::addr_of_mut!(PTRDATA) as *mut c_void;
        PTR.type_id = bpf_core_type_id_kernel!($type_);
        if PTR.type_id <= 0 {
            ret = -EINVAL;
            break;
        }
        ret = bpf_snprintf_btf(
            $str_ as *mut c_char,
            STRSIZE as __u32,
            core::ptr::addr_of_mut!(PTR) as *mut c_void,
            core::mem::size_of::<btf_ptr>() as __u32,
            hflags,
        ) as c_long;
        if ret != 0 {
            break;
        }
        cmp = __strncmp(
            $str_ as *const c_void,
            EXPECTEDVAL.as_ptr() as *const c_void,
            EXPECTED_STRSIZE,
        );
        if cmp != 0 {
            bpf_printk!("(%d) got %s", cmp, $str_);
            bpf_printk!("(%d) expected %s", cmp, EXPECTEDVAL.as_ptr());
            ret = -EBADMSG;
            break;
        }
    }};
}

/* Use where expected data string matches its stringified declaration. */
macro_rules! test_btf_c {
    ($str_:expr, $type_:ty, $flags_:expr, $ptrdata_:expr) => {{
        test_btf!(
            $str_,
            $type_,
            $flags_,
            concat!("(", stringify!($type_), ")", stringify!($ptrdata_)),
            $ptrdata_
        );
    }};
}

/* TRACE_EVENT(netif_receive_skb,
 *      TP_PROTO(struct sk_buff *skb),
 */
#[link_section = "tp_btf/netif_receive_skb"]
pub unsafe extern "C" fn trace_netif_receive_skb(skb: *mut sk_buff) -> c_int {
    static mut flags: [__u64; 6] = [
        0,
        BTF_F_COMPACT as __u64,
        BTF_F_ZERO as __u64,
        BTF_F_PTR_RAW as __u64,
        BTF_F_NONAME as __u64,
        (BTF_F_COMPACT | BTF_F_ZERO | BTF_F_PTR_RAW | BTF_F_NONAME) as __u64,
    ];
    static mut p: btf_ptr = btf_ptr {
        ptr: core::ptr::null_mut(),
        type_id: 0,
    };
    let mut key: __u32 = 0;
    let mut i: c_int;
    let mut __ret: c_int;
    let str_: *mut c_char;

    /* C condition: #if __has_builtin(__builtin_btf_type_id) */
    if cfg!(has_builtin_btf_type_id) {
        str_ = bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(strdata) as *mut c_void,
            core::ptr::addr_of_mut!(key) as *mut c_void,
        ) as *mut c_char;
        if str_.is_null() {
            return 0;
        }

        /* Ensure we can write skb string representation */
        p.type_id = bpf_core_type_id_kernel!(sk_buff);
        p.ptr = skb as *mut c_void;
        i = 0;
        while (i as usize) < flags.len() {
            num_subtests += 1;
            ret = bpf_snprintf_btf(
                str_,
                STRSIZE as __u32,
                core::ptr::addr_of_mut!(p) as *mut c_void,
                core::mem::size_of::<btf_ptr>() as __u32,
                0,
            ) as c_long;
            if ret < 0 {
                bpf_printk!("returned %d when writing skb", ret);
            }
            ran_subtests += 1;
            i += 1;
        }

        /* Check invalid ptr value */
        p.ptr = BADPTR;
        __ret = bpf_snprintf_btf(
            str_,
            STRSIZE as __u32,
            core::ptr::addr_of_mut!(p) as *mut c_void,
            core::mem::size_of::<btf_ptr>() as __u32,
            0,
        );
        if __ret >= 0 {
            bpf_printk!(
                "printing %llx should generate error, got (%d)",
                BADPTR as c_ulonglong,
                __ret
            );
            ret = -ERANGE;
        }

        /* Verify type display for various types. */

        /* simple int */
        test_btf_c!(str_, c_int, 0, 1234);
        test_btf!(str_, c_int, BTF_F_NONAME, "1234", 1234);
        /* zero value should be printed at toplevel */
        test_btf!(str_, c_int, 0, "(int)0", 0);
        test_btf!(str_, c_int, BTF_F_NONAME, "0", 0);
        test_btf!(str_, c_int, BTF_F_ZERO, "(int)0", 0);
        test_btf!(str_, c_int, BTF_F_NONAME | BTF_F_ZERO, "0", 0);
        test_btf_c!(str_, c_int, 0, -4567);
        test_btf!(str_, c_int, BTF_F_NONAME, "-4567", -4567);

        /* simple char */
        test_btf_c!(str_, c_char, 0, 100);
        test_btf!(str_, c_char, BTF_F_NONAME, "100", 100);
        /* zero value should be printed at toplevel */
        test_btf!(str_, c_char, 0, "(char)0", 0);
        test_btf!(str_, c_char, BTF_F_NONAME, "0", 0);
        test_btf!(str_, c_char, BTF_F_ZERO, "(char)0", 0);
        test_btf!(str_, c_char, BTF_F_NONAME | BTF_F_ZERO, "0", 0);

        /* simple typedef */
        test_btf_c!(str_, uint64_t, 0, 100);
        test_btf!(str_, u64, BTF_F_NONAME, "1", 1);
        /* zero value should be printed at toplevel */
        test_btf!(str_, u64, 0, "(u64)0", 0);
        test_btf!(str_, u64, BTF_F_NONAME, "0", 0);
        test_btf!(str_, u64, BTF_F_ZERO, "(u64)0", 0);
        test_btf!(str_, u64, BTF_F_NONAME | BTF_F_ZERO, "0", 0);

        /* typedef struct */
        test_btf_c!(str_, atomic_t, 0, atomic_t { counter: 1 as c_int });
        test_btf!(str_, atomic_t, BTF_F_NONAME, "{1,}", atomic_t { counter: 1 });
        /* typedef with 0 value should be printed at toplevel */
        test_btf!(str_, atomic_t, 0, "(atomic_t){}", atomic_t { counter: 0 });
        test_btf!(str_, atomic_t, BTF_F_NONAME, "{}", atomic_t { counter: 0 });
        test_btf!(
            str_,
            atomic_t,
            BTF_F_ZERO,
            "(atomic_t){.counter = (int)0,}",
            atomic_t { counter: 0 }
        );
        test_btf!(
            str_,
            atomic_t,
            BTF_F_NONAME | BTF_F_ZERO,
            "{0,}",
            atomic_t { counter: 0 }
        );

        /* enum where enum value does (and does not) exist */
        test_btf_c!(str_, bpf_cmd, 0, BPF_MAP_CREATE);
        test_btf!(str_, bpf_cmd, 0, "(enum bpf_cmd)BPF_MAP_CREATE", 0);
        test_btf!(
            str_,
            bpf_cmd,
            BTF_F_NONAME,
            "BPF_MAP_CREATE",
            BPF_MAP_CREATE
        );
        test_btf!(
            str_,
            bpf_cmd,
            BTF_F_NONAME | BTF_F_ZERO,
            "BPF_MAP_CREATE",
            0
        );

        test_btf!(
            str_,
            bpf_cmd,
            BTF_F_ZERO,
            "(enum bpf_cmd)BPF_MAP_CREATE",
            BPF_MAP_CREATE
        );
        test_btf!(
            str_,
            bpf_cmd,
            BTF_F_NONAME | BTF_F_ZERO,
            "BPF_MAP_CREATE",
            BPF_MAP_CREATE
        );
        test_btf_c!(str_, bpf_cmd, 0, 2000);
        test_btf!(str_, bpf_cmd, BTF_F_NONAME, "2000", 2000);

        /* simple struct */
        test_btf_c!(
            str_,
            btf_enum,
            0,
            btf_enum {
                name_off: 3 as __u32,
                val: -1 as __s32
            }
        );
        test_btf!(
            str_,
            btf_enum,
            BTF_F_NONAME,
            "{3,-1,}",
            btf_enum {
                name_off: 3,
                val: -1
            }
        );
        test_btf!(
            str_,
            btf_enum,
            BTF_F_NONAME,
            "{-1,}",
            btf_enum {
                name_off: 0,
                val: -1
            }
        );
        test_btf!(
            str_,
            btf_enum,
            BTF_F_NONAME | BTF_F_ZERO,
            "{0,-1,}",
            btf_enum {
                name_off: 0,
                val: -1
            }
        );
        /* empty struct should be printed */
        test_btf!(
            str_,
            btf_enum,
            0,
            "(struct btf_enum){}",
            btf_enum {
                name_off: 0,
                val: 0
            }
        );
        test_btf!(
            str_,
            btf_enum,
            BTF_F_NONAME,
            "{}",
            btf_enum {
                name_off: 0,
                val: 0
            }
        );
        test_btf!(
            str_,
            btf_enum,
            BTF_F_ZERO,
            "(struct btf_enum){.name_off = (__u32)0,.val = (__s32)0,}",
            btf_enum {
                name_off: 0,
                val: 0
            }
        );

        /* struct with pointers */
        test_btf!(
            str_,
            list_head,
            BTF_F_PTR_RAW,
            "(struct list_head){.next = (struct list_head *)0x0000000000000001,}",
            list_head {
                next: 1usize as *mut list_head
            }
        );
        /* NULL pointer should not be displayed */
        test_btf!(
            str_,
            list_head,
            BTF_F_PTR_RAW,
            "(struct list_head){}",
            list_head {
                next: core::ptr::null_mut()
            }
        );

        /* struct with char array */
        test_btf!(
            str_,
            bpf_prog_info,
            0,
            "(struct bpf_prog_info){.name = (char[])['f','o','o',],}",
            bpf_prog_info {
                name: c_char_array!("foo")
            }
        );
        test_btf!(
            str_,
            bpf_prog_info,
            BTF_F_NONAME,
            "{['f','o','o',],}",
            bpf_prog_info {
                name: c_char_array!("foo")
            }
        );
        /* leading null char means do not display string */
        test_btf!(
            str_,
            bpf_prog_info,
            0,
            "(struct bpf_prog_info){}",
            bpf_prog_info {
                name: [0, b'f' as c_char, b'o' as c_char, b'o' as c_char]
            }
        );
        /* handle non-printable characters */
        test_btf!(
            str_,
            bpf_prog_info,
            0,
            "(struct bpf_prog_info){.name = (char[])[1,2,3,],}",
            bpf_prog_info {
                name: [1, 2, 3, 0]
            }
        );

        /* struct with non-char array */
        test_btf!(
            str_,
            __sk_buff,
            0,
            "(struct __sk_buff){.cb = (__u32[])[1,2,3,4,5,],}",
            __sk_buff { cb: [1, 2, 3, 4, 5] }
        );
        test_btf!(
            str_,
            __sk_buff,
            BTF_F_NONAME,
            "{[1,2,3,4,5,],}",
            __sk_buff { cb: [1, 2, 3, 4, 5] }
        );
        /* For non-char, arrays, show non-zero values only */
        test_btf!(
            str_,
            __sk_buff,
            0,
            "(struct __sk_buff){.cb = (__u32[])[1,],}",
            __sk_buff { cb: [0, 0, 1, 0, 0] }
        );

        /* struct with bitfields */
        test_btf_c!(
            str_,
            bpf_insn,
            0,
            bpf_insn {
                code: 1 as __u8,
                dst_reg: 0x2 as __u8,
                src_reg: 0x3 as __u8,
                off: 4 as __s16,
                imm: 5 as __s32
            }
        );
        test_btf!(
            str_,
            bpf_insn,
            BTF_F_NONAME,
            "{1,0x2,0x3,4,5,}",
            bpf_insn {
                code: 1,
                dst_reg: 0x2,
                src_reg: 0x3,
                off: 4,
                imm: 5
            }
        );
    } else {
        skip = true;
    }

    0
}

#[link_section = "license"]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];
