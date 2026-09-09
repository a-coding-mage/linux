/* Included by drivers/net/dsa/lan9303.h and net/dsa/tag_lan9303.c */
/* Dependency: <linux/if_ether.h> */

pub const LAN9303_NUM_ALR_RECORDS: usize = 512;

/* External types supplied by the surrounding kernel translation. */
pub struct lan9303;
pub struct device;
pub struct regmap;
pub struct regmap_irq_chip_data;
pub struct gpio_desc;
pub struct dsa_switch;
pub struct mutex;

#[repr(C)]
pub struct lan9303_phy_ops {
    /* PHY 1 and 2 access */
    pub phy_read:
        Option<unsafe extern "C" fn(chip: *mut lan9303, addr: ::core::ffi::c_int, regnum: ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub phy_write: Option<unsafe extern "C" fn(
        chip: *mut lan9303,
        addr: ::core::ffi::c_int,
        regnum: ::core::ffi::c_int,
        val: u16,
    ) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct lan9303_alr_cache_entry {
    pub mac_addr: [u8; 6], /* ETH_ALEN */
    pub port_map: u8,      /* Bitmap of ports. Zero if unused entry */
    pub stp_override: u8,  /* non zero if set LAN9303_ALR_DAT1_AGE_OVERRID */
}

#[repr(C)]
pub struct lan9303 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub irq_data: *mut regmap_irq_chip_data,
    pub reset_gpio: *mut gpio_desc,
    pub reset_duration: u32, /* in [ms] */
    pub phy_addr_base: ::core::ffi::c_int,
    pub ds: *mut dsa_switch,
    pub indirect_mutex: mutex, /* protect indexed register access */
    pub alr_mutex: mutex,      /* protect ALR access */
    pub ops: *const lan9303_phy_ops,
    pub is_bridged: bool, /* true if port 1 and 2 are bridged */

    /* remember LAN9303_SWE_PORT_STATE while not bridged */
    pub swe_port_state: u32,
    /* LAN9303 do not offer reading specific ALR entry. Cache all
     * static entries in a flat table
     **/
    pub alr_cache: [lan9303_alr_cache_entry; LAN9303_NUM_ALR_RECORDS],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
