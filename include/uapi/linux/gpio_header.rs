/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/* <linux/gpio.h> - userspace ABI for the GPIO character devices */

/* C header dependencies: linux/const.h, linux/ioctl.h, linux/types.h */

pub const GPIO_MAX_NAME_SIZE: usize = 32;

#[repr(C)]
pub struct gpiochip_info {
    pub name: [::std::os::raw::c_char; GPIO_MAX_NAME_SIZE],
    pub label: [::std::os::raw::c_char; GPIO_MAX_NAME_SIZE],
    pub lines: __u32,
}

pub const GPIO_V2_LINES_MAX: usize = 64;
pub const GPIO_V2_LINE_NUM_ATTRS_MAX: usize = 10;

pub type gpio_v2_line_flag = __u64;
pub const GPIO_V2_LINE_FLAG_USED: gpio_v2_line_flag = 1u64 << 0;
pub const GPIO_V2_LINE_FLAG_ACTIVE_LOW: gpio_v2_line_flag = 1u64 << 1;
pub const GPIO_V2_LINE_FLAG_INPUT: gpio_v2_line_flag = 1u64 << 2;
pub const GPIO_V2_LINE_FLAG_OUTPUT: gpio_v2_line_flag = 1u64 << 3;
pub const GPIO_V2_LINE_FLAG_EDGE_RISING: gpio_v2_line_flag = 1u64 << 4;
pub const GPIO_V2_LINE_FLAG_EDGE_FALLING: gpio_v2_line_flag = 1u64 << 5;
pub const GPIO_V2_LINE_FLAG_OPEN_DRAIN: gpio_v2_line_flag = 1u64 << 6;
pub const GPIO_V2_LINE_FLAG_OPEN_SOURCE: gpio_v2_line_flag = 1u64 << 7;
pub const GPIO_V2_LINE_FLAG_BIAS_PULL_UP: gpio_v2_line_flag = 1u64 << 8;
pub const GPIO_V2_LINE_FLAG_BIAS_PULL_DOWN: gpio_v2_line_flag = 1u64 << 9;
pub const GPIO_V2_LINE_FLAG_BIAS_DISABLED: gpio_v2_line_flag = 1u64 << 10;
pub const GPIO_V2_LINE_FLAG_EVENT_CLOCK_REALTIME: gpio_v2_line_flag = 1u64 << 11;
pub const GPIO_V2_LINE_FLAG_EVENT_CLOCK_HTE: gpio_v2_line_flag = 1u64 << 12;

#[repr(C)]
pub struct gpio_v2_line_values { pub bits: __aligned_u64, pub mask: __aligned_u64 }

pub type gpio_v2_line_attr_id = __u32;
pub const GPIO_V2_LINE_ATTR_ID_FLAGS: gpio_v2_line_attr_id = 1;
pub const GPIO_V2_LINE_ATTR_ID_OUTPUT_VALUES: gpio_v2_line_attr_id = 2;
pub const GPIO_V2_LINE_ATTR_ID_DEBOUNCE: gpio_v2_line_attr_id = 3;

#[repr(C)]
pub union gpio_v2_line_attribute__bindgen_ty_1 {
    pub flags: __aligned_u64,
    pub values: __aligned_u64,
    pub debounce_period_us: __u32,
}
#[repr(C)]
pub struct gpio_v2_line_attribute {
    pub id: __u32,
    pub padding: __u32,
    pub __bindgen_anon_1: gpio_v2_line_attribute__bindgen_ty_1,
}

#[repr(C)]
pub struct gpio_v2_line_config_attribute { pub attr: gpio_v2_line_attribute, pub mask: __aligned_u64 }

#[repr(C)]
pub struct gpio_v2_line_config {
    pub flags: __aligned_u64,
    pub num_attrs: __u32,
    pub padding: [__u32; 5],
    pub attrs: [gpio_v2_line_config_attribute; GPIO_V2_LINE_NUM_ATTRS_MAX],
}

#[repr(C)]
pub struct gpio_v2_line_request {
    pub offsets: [__u32; GPIO_V2_LINES_MAX],
    pub consumer: [::std::os::raw::c_char; GPIO_MAX_NAME_SIZE],
    pub config: gpio_v2_line_config,
    pub num_lines: __u32,
    pub event_buffer_size: __u32,
    pub padding: [__u32; 5],
    pub fd: __s32,
}

#[repr(C)]
pub struct gpio_v2_line_info {
    pub name: [::std::os::raw::c_char; GPIO_MAX_NAME_SIZE],
    pub consumer: [::std::os::raw::c_char; GPIO_MAX_NAME_SIZE],
    pub offset: __u32,
    pub num_attrs: __u32,
    pub flags: __aligned_u64,
    pub attrs: [gpio_v2_line_attribute; GPIO_V2_LINE_NUM_ATTRS_MAX],
    pub padding: [__u32; 4],
}

pub type gpio_v2_line_changed_type = __u32;
pub const GPIO_V2_LINE_CHANGED_REQUESTED: gpio_v2_line_changed_type = 1;
pub const GPIO_V2_LINE_CHANGED_RELEASED: gpio_v2_line_changed_type = 2;
pub const GPIO_V2_LINE_CHANGED_CONFIG: gpio_v2_line_changed_type = 3;

#[repr(C)]
pub struct gpio_v2_line_info_changed {
    pub info: gpio_v2_line_info,
    pub timestamp_ns: __aligned_u64,
    pub event_type: __u32,
    pub padding: [__u32; 5],
}

pub type gpio_v2_line_event_id = __u32;
pub const GPIO_V2_LINE_EVENT_RISING_EDGE: gpio_v2_line_event_id = 1;
pub const GPIO_V2_LINE_EVENT_FALLING_EDGE: gpio_v2_line_event_id = 2;

#[repr(C)]
pub struct gpio_v2_line_event {
    pub timestamp_ns: __aligned_u64,
    pub id: __u32,
    pub offset: __u32,
    pub seqno: __u32,
    pub line_seqno: __u32,
    pub padding: [__u32; 6],
}

/* ABI v1 (deprecated). */
pub const GPIOLINE_FLAG_KERNEL: ::std::os::raw::c_ulong = 1 << 0;
pub const GPIOLINE_FLAG_IS_OUT: ::std::os::raw::c_ulong = 1 << 1;
pub const GPIOLINE_FLAG_ACTIVE_LOW: ::std::os::raw::c_ulong = 1 << 2;
pub const GPIOLINE_FLAG_OPEN_DRAIN: ::std::os::raw::c_ulong = 1 << 3;
pub const GPIOLINE_FLAG_OPEN_SOURCE: ::std::os::raw::c_ulong = 1 << 4;
pub const GPIOLINE_FLAG_BIAS_PULL_UP: ::std::os::raw::c_ulong = 1 << 5;
pub const GPIOLINE_FLAG_BIAS_PULL_DOWN: ::std::os::raw::c_ulong = 1 << 6;
pub const GPIOLINE_FLAG_BIAS_DISABLE: ::std::os::raw::c_ulong = 1 << 7;

#[repr(C)]
pub struct gpioline_info {
    pub line_offset: __u32,
    pub flags: __u32,
    pub name: [::std::os::raw::c_char; GPIO_MAX_NAME_SIZE],
    pub consumer: [::std::os::raw::c_char; GPIO_MAX_NAME_SIZE],
}

pub const GPIOHANDLES_MAX: usize = 64;
pub const GPIOLINE_CHANGED_REQUESTED: __u32 = 1;
pub const GPIOLINE_CHANGED_RELEASED: __u32 = 2;
pub const GPIOLINE_CHANGED_CONFIG: __u32 = 3;

#[repr(C)]
pub struct gpioline_info_changed {
    pub info: gpioline_info,
    pub timestamp: __u64,
    pub event_type: __u32,
    pub padding: [__u32; 5],
}

pub const GPIOHANDLE_REQUEST_INPUT: ::std::os::raw::c_ulong = 1 << 0;
pub const GPIOHANDLE_REQUEST_OUTPUT: ::std::os::raw::c_ulong = 1 << 1;
pub const GPIOHANDLE_REQUEST_ACTIVE_LOW: ::std::os::raw::c_ulong = 1 << 2;
pub const GPIOHANDLE_REQUEST_OPEN_DRAIN: ::std::os::raw::c_ulong = 1 << 3;
pub const GPIOHANDLE_REQUEST_OPEN_SOURCE: ::std::os::raw::c_ulong = 1 << 4;
pub const GPIOHANDLE_REQUEST_BIAS_PULL_UP: ::std::os::raw::c_ulong = 1 << 5;
pub const GPIOHANDLE_REQUEST_BIAS_PULL_DOWN: ::std::os::raw::c_ulong = 1 << 6;
pub const GPIOHANDLE_REQUEST_BIAS_DISABLE: ::std::os::raw::c_ulong = 1 << 7;

#[repr(C)]
pub struct gpiohandle_request {
    pub lineoffsets: [__u32; GPIOHANDLES_MAX],
    pub flags: __u32,
    pub default_values: [__u8; GPIOHANDLES_MAX],
    pub consumer_label: [::std::os::raw::c_char; GPIO_MAX_NAME_SIZE],
    pub lines: __u32,
    pub fd: ::std::os::raw::c_int,
}
#[repr(C)]
pub struct gpiohandle_config {
    pub flags: __u32,
    pub default_values: [__u8; GPIOHANDLES_MAX],
    pub padding: [__u32; 4],
}
#[repr(C)]
pub struct gpiohandle_data { pub values: [__u8; GPIOHANDLES_MAX] }

pub const GPIOEVENT_REQUEST_RISING_EDGE: ::std::os::raw::c_ulong = 1 << 0;
pub const GPIOEVENT_REQUEST_FALLING_EDGE: ::std::os::raw::c_ulong = 1 << 1;
pub const GPIOEVENT_REQUEST_BOTH_EDGES: ::std::os::raw::c_ulong = (1 << 0) | (1 << 1);

#[repr(C)]
pub struct gpioevent_request {
    pub lineoffset: __u32,
    pub handleflags: __u32,
    pub eventflags: __u32,
    pub consumer_label: [::std::os::raw::c_char; GPIO_MAX_NAME_SIZE],
    pub fd: ::std::os::raw::c_int,
}
pub const GPIOEVENT_EVENT_RISING_EDGE: __u32 = 0x01;
pub const GPIOEVENT_EVENT_FALLING_EDGE: __u32 = 0x02;
#[repr(C)]
pub struct gpioevent_data { pub timestamp: __u64, pub id: __u32 }

/* ioctl constants depend on the external _IOR/_IOWR definitions. */
pub const GPIO_GET_CHIPINFO_IOCTL: _ = _IOR(0xB4, 0x01, gpiochip_info);
pub const GPIO_GET_LINEINFO_UNWATCH_IOCTL: _ = _IOWR(0xB4, 0x0C, __u32);
pub const GPIO_V2_GET_LINEINFO_IOCTL: _ = _IOWR(0xB4, 0x05, gpio_v2_line_info);
pub const GPIO_V2_GET_LINEINFO_WATCH_IOCTL: _ = _IOWR(0xB4, 0x06, gpio_v2_line_info);
pub const GPIO_V2_GET_LINE_IOCTL: _ = _IOWR(0xB4, 0x07, gpio_v2_line_request);
pub const GPIO_V2_LINE_SET_CONFIG_IOCTL: _ = _IOWR(0xB4, 0x0D, gpio_v2_line_config);
pub const GPIO_V2_LINE_GET_VALUES_IOCTL: _ = _IOWR(0xB4, 0x0E, gpio_v2_line_values);
pub const GPIO_V2_LINE_SET_VALUES_IOCTL: _ = _IOWR(0xB4, 0x0F, gpio_v2_line_values);
pub const GPIO_GET_LINEINFO_IOCTL: _ = _IOWR(0xB4, 0x02, gpioline_info);
pub const GPIO_GET_LINEHANDLE_IOCTL: _ = _IOWR(0xB4, 0x03, gpiohandle_request);
pub const GPIO_GET_LINEEVENT_IOCTL: _ = _IOWR(0xB4, 0x04, gpioevent_request);
pub const GPIOHANDLE_GET_LINE_VALUES_IOCTL: _ = _IOWR(0xB4, 0x08, gpiohandle_data);
pub const GPIOHANDLE_SET_LINE_VALUES_IOCTL: _ = _IOWR(0xB4, 0x09, gpiohandle_data);
pub const GPIOHANDLE_SET_CONFIG_IOCTL: _ = _IOWR(0xB4, 0x0A, gpiohandle_config);
pub const GPIO_GET_LINEINFO_WATCH_IOCTL: _ = _IOWR(0xB4, 0x0B, gpioline_info);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
