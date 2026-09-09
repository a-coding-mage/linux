/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of linux/leds.h. C includes and build-time configuration are external dependencies. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct kernfs_node { _private: [u8; 0] }
#[repr(C)] pub struct attribute_group { _private: [u8; 0] }
#[repr(C)] pub struct led_pattern;
#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct rw_semaphore { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }

#[repr(i32)] #[derive(Copy, Clone)] pub enum led_brightness { LED_OFF=0, LED_ON=1, LED_HALF=127, LED_FULL=255 }
#[repr(i32)] #[derive(Copy, Clone)] pub enum led_default_state { LEDS_DEFSTATE_OFF=0, LEDS_DEFSTATE_ON=1, LEDS_DEFSTATE_KEEP=2 }

#[repr(C)] pub struct led_lookup_data { pub list: list_head, pub provider: *const c_char, pub dev_id: *const c_char, pub con_id: *const c_char }
#[repr(C)] pub struct led_init_data { pub fwnode: *mut fwnode_handle, pub default_label: *const c_char, pub devicename: *const c_char, pub devname_mandatory: bool }
extern "C" { pub fn led_init_default_state_get(fwnode: *mut fwnode_handle) -> led_default_state; }
#[repr(C)] pub struct led_hw_trigger_type { pub dummy: c_int }

pub const LED_SUSPENDED: c_int=1<<0; pub const LED_UNREGISTERING: c_int=1<<1;
pub const LED_CORE_SUSPENDRESUME: c_int=1<<16; pub const LED_SYSFS_DISABLE: c_int=1<<17;
pub const LED_DEV_CAP_FLASH: c_int=1<<18; pub const LED_HW_PLUGGABLE: c_int=1<<19;
pub const LED_PANIC_INDICATOR: c_int=1<<20; pub const LED_BRIGHT_HW_CHANGED: c_int=1<<21;
pub const LED_RETAIN_AT_SHUTDOWN: c_int=1<<22; pub const LED_INIT_DEFAULT_TRIGGER: c_int=1<<23;
pub const LED_REJECT_NAME_CONFLICT: c_int=1<<24; pub const LED_MULTI_COLOR: c_int=1<<25;
pub const LED_BLINK_SW: c_int=0; pub const LED_BLINK_ONESHOT: c_int=1; pub const LED_BLINK_ONESHOT_STOP: c_int=2;
pub const LED_BLINK_INVERT: c_int=3; pub const LED_BLINK_BRIGHTNESS_CHANGE: c_int=4; pub const LED_BLINK_DISABLE: c_int=5;
pub const LED_SET_BRIGHTNESS_OFF: c_int=6; pub const LED_SET_BRIGHTNESS: c_int=7; pub const LED_SET_BLINK: c_int=8;

#[repr(C)] pub struct led_classdev {
 pub name:*const c_char, pub brightness:c_uint, pub max_brightness:c_uint, pub color:c_uint, pub flags:c_int, pub work_flags:c_ulong,
 pub brightness_set:Option<unsafe extern "C" fn(*mut led_classdev,led_brightness)>,
 pub brightness_set_blocking:Option<unsafe extern "C" fn(*mut led_classdev,led_brightness)->c_int>,
 pub brightness_get:Option<unsafe extern "C" fn(*mut led_classdev)->led_brightness>,
 pub blink_set:Option<unsafe extern "C" fn(*mut led_classdev,*mut c_ulong,*mut c_ulong)->c_int>,
 pub pattern_set:Option<unsafe extern "C" fn(*mut led_classdev,*mut led_pattern,u32,c_int)->c_int>, pub pattern_clear:Option<unsafe extern "C" fn(*mut led_classdev)->c_int>,
 pub dev:*mut device, pub groups:*const *const attribute_group, pub node:list_head, pub default_trigger:*const c_char,
 pub blink_delay_on:c_ulong, pub blink_delay_off:c_ulong, pub blink_timer:timer_list, pub blink_brightness:c_int, pub new_blink_brightness:c_int,
 pub flash_resume:Option<unsafe extern "C" fn(*mut led_classdev)>, pub wq:*mut workqueue_struct, pub set_brightness_work:work_struct,
 pub delayed_set_value:c_int, pub delayed_delay_on:c_ulong, pub delayed_delay_off:c_ulong, pub led_access:mutex,
}

extern "C" {
 pub fn led_classdev_register_ext(*mut device,*mut led_classdev,*mut led_init_data)->c_int;
 pub fn devm_led_classdev_register_ext(*mut device,*mut led_classdev,*mut led_init_data)->c_int;
 pub fn led_classdev_unregister(*mut led_classdev); pub fn devm_led_classdev_unregister(*mut device,*mut led_classdev);
 pub fn led_classdev_suspend(*mut led_classdev); pub fn led_classdev_resume(*mut led_classdev);
 pub fn led_add_lookup(*mut led_lookup_data); pub fn led_remove_lookup(*mut led_lookup_data);
 pub fn led_get(*mut device,*mut c_char)->*mut led_classdev; pub fn devm_led_get(*mut device,*mut c_char)->*mut led_classdev; pub fn led_put(*mut led_classdev);
 pub fn devm_of_led_get(*mut device,c_int)->*mut led_classdev; pub fn devm_of_led_get_optional(*mut device,c_int)->*mut led_classdev;
 pub fn led_blink_set(*mut led_classdev,*mut c_ulong,*mut c_ulong); pub fn led_blink_set_nosleep(*mut led_classdev,c_ulong,c_ulong);
 pub fn led_blink_set_oneshot(*mut led_classdev,*mut c_ulong,*mut c_ulong,c_int); pub fn led_set_brightness(*mut led_classdev,c_uint); pub fn led_set_brightness_sync(*mut led_classdev,c_uint)->c_int;
 pub fn led_mc_set_brightness(*mut led_classdev,*mut c_uint,c_uint,c_uint); pub fn led_update_brightness(*mut led_classdev)->c_int;
 pub fn led_get_default_pattern(*mut led_classdev,*mut c_uint)->*mut u32; pub fn led_sysfs_disable(*mut led_classdev); pub fn led_sysfs_enable(*mut led_classdev);
 pub fn led_compose_name(*mut device,*mut led_init_data,*mut c_char)->c_int; pub fn led_get_color_name(u8)->*const c_char;
}
#[inline] pub unsafe fn led_sysfs_is_disabled(x:*mut led_classdev)->bool { (*x).flags & LED_SYSFS_DISABLE != 0 }

#[repr(C)] pub struct led_trigger { _private: [u8;0] }
#[repr(i32)] pub enum led_trigger_netdev_modes { TRIGGER_NETDEV_LINK=0, TRIGGER_NETDEV_LINK_10, TRIGGER_NETDEV_LINK_100, TRIGGER_NETDEV_LINK_1000, TRIGGER_NETDEV_LINK_2500, TRIGGER_NETDEV_LINK_5000, TRIGGER_NETDEV_LINK_10000, TRIGGER_NETDEV_LINK_25000, TRIGGER_NETDEV_LINK_40000, TRIGGER_NETDEV_LINK_50000, TRIGGER_NETDEV_LINK_100000, TRIGGER_NETDEV_HALF_DUPLEX, TRIGGER_NETDEV_FULL_DUPLEX, TRIGGER_NETDEV_TX, TRIGGER_NETDEV_RX, TRIGGER_NETDEV_TX_ERR, TRIGGER_NETDEV_RX_ERR, __TRIGGER_NETDEV_MAX }

#[repr(C)] pub struct led_info { pub name:*const c_char, pub default_trigger:*const c_char, pub flags:c_int }
#[repr(C)] pub struct led_platform_data { pub num_leds:c_int, pub leds:*mut led_info }
#[repr(C)] pub struct led_properties { pub color:u32, pub color_present:bool, pub function:*const c_char, pub func_enum:u32, pub func_enum_present:bool, pub label:*const c_char }
pub type gpio_blink_set_t = Option<unsafe extern "C" fn(*mut gpio_desc,c_int,*mut c_ulong,*mut c_ulong)->c_int>;
#[repr(C)] pub struct gpio_led { pub name:*const c_char, pub default_trigger:*const c_char, pub gpio:c_uint, pub active_low:c_uint, pub retain_state_suspended:c_uint, pub panic_indicator:c_uint, pub default_state:c_uint, pub retain_state_shutdown:c_uint, pub gpiod:*mut gpio_desc }
pub const LEDS_GPIO_DEFSTATE_OFF:i32=0; pub const LEDS_GPIO_DEFSTATE_ON:i32=1; pub const LEDS_GPIO_DEFSTATE_KEEP:i32=2;
#[repr(C)] pub struct gpio_led_platform_data { pub num_leds:c_int, pub leds:*const gpio_led, pub gpio_blink_set:gpio_blink_set_t }
#[repr(C)] pub struct led_pattern { pub delta_t:u32, pub brightness:c_int }
#[repr(i32)] pub enum cpu_led_event { CPU_LED_IDLE_START, CPU_LED_IDLE_END, CPU_LED_START, CPU_LED_STOP, CPU_LED_HALTED }
#[repr(i32)] pub enum led_audio { LED_AUDIO_MUTE, LED_AUDIO_MICMUTE, NUM_AUDIO_LEDS }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
