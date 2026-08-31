/* SPDX-License-Identifier: GPL-2.0 */

macro_rules! DECLARE_PER_CPU {
    ($type:ty, $val:ident) => {
        extern "C" {
            pub static mut $val: $type;
        }
    };
}

macro_rules! DEFINE_PER_CPU {
    ($type:ty, $val:ident) => {
        static mut $val: $type
    };
}

macro_rules! __get_cpu_var {
    ($var:expr) => {
        $var
    };
}

macro_rules! this_cpu_ptr {
    ($var:expr) => {
        $var
    };
}

macro_rules! this_cpu_read {
    ($var:expr) => {
        $var
    };
}

macro_rules! this_cpu_xchg {
    ($var:expr, $val:expr) => {
        uatomic_xchg(&mut $var, $val)
    };
}

macro_rules! this_cpu_cmpxchg {
    ($var:expr, $old:expr, $new:expr) => {
        uatomic_cmpxchg(&mut $var, $old, $new)
    };
}

macro_rules! per_cpu_ptr {
    ($ptr:expr, $cpu:expr) => {{
        let _ = $cpu;
        $ptr
    }};
}

macro_rules! per_cpu {
    ($var:expr, $cpu:expr) => {
        *per_cpu_ptr!(&mut $var, $cpu)
    };
}
