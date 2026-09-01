// SPDX-License-Identifier: GPL-2.0+
/*
 * Machine driver for AMD Yellow Carp platform using DMIC
 *
 * Copyright 2021 Advanced Micro Devices, Inc.
 */

// C dependencies:
// <sound/soc.h>, <sound/soc-dapm.h>, <linux/module.h>, <sound/pcm.h>,
// <sound/pcm_params.h>, <linux/io.h>, <linux/dmi.h>, <linux/acpi.h>,
// "acp6x.h"

const DRV_NAME: &CStr = c"acp_yc_mach";

SND_SOC_DAILINK_DEF!(
    acp6x_pdm,
    DAILINK_COMP_ARRAY!(COMP_CPU!(c"acp_yc_pdm_dma.0"))
);

SND_SOC_DAILINK_DEF!(
    dmic_codec,
    DAILINK_COMP_ARRAY!(COMP_CODEC!(c"dmic-codec.0", c"dmic-hifi"))
);

SND_SOC_DAILINK_DEF!(
    pdm_platform,
    DAILINK_COMP_ARRAY!(COMP_PLATFORM!(c"acp_yc_pdm_dma.0"))
);

static mut acp6x_dai_pdm: [snd_soc_dai_link; 1] = [snd_soc_dai_link {
    name: c"acp6x-dmic-capture".as_ptr(),
    stream_name: c"DMIC capture".as_ptr(),
    capture_only: 1,
    SND_SOC_DAILINK_REG!(acp6x_pdm, dmic_codec, pdm_platform)
}];

static mut acp6x_card: snd_soc_card = snd_soc_card {
    name: c"acp6x".as_ptr(),
    owner: THIS_MODULE,
    dai_link: unsafe { acp6x_dai_pdm.as_mut_ptr() },
    num_links: 1,
};

macro_rules! yc_acp_quirk {
    ($kind0:ident, $value0:literal, $kind1:ident, $value1:literal) => {
        dmi_system_id {
            driver_data: unsafe { &raw mut acp6x_card as *mut _ as *mut c_void },
            matches: [
                DMI_MATCH!($kind0, $value0),
                DMI_MATCH!($kind1, $value1),
            ],
        }
    };
}

static yc_acp_quirk_table: [dmi_system_id; 103] = [
    yc_acp_quirk!(DMI_SYS_VENDOR, "Acer", DMI_PRODUCT_NAME, "Aspire AG14-22P"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Lecoo", DMI_PRODUCT_NAME, "Bellator N176"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "HP", DMI_PRODUCT_NAME, "HP Laptop 15-fc0xxx"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "HP", DMI_PRODUCT_NAME, "OMEN Gaming Laptop 16-ap0xxx"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Dell Inc.", DMI_PRODUCT_NAME, "Dell G15 5525"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21D0"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21D0"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21D1"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21D2"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21D3"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21D4"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21D5"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21CF"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21CG"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21CQ"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21CR"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21CM"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21CN"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21CH"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21CJ"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21CK"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21CL"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21EF"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21EM"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21EN"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21HY"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21J0"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21J5"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21J6"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21M1"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21M3"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21M4"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21M5"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21M6"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "21ME"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "82QF"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "82TL"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "82UG"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "82UU"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "82V2"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "82YM"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "83AS"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "83BS"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "83HN"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "83L3"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "83N6"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "83Q2"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "83Q3"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "RB", DMI_PRODUCT_NAME, "Nitro ANV15-41"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "83J2"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "LENOVO", DMI_PRODUCT_NAME, "83J3"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_PRODUCT_NAME, "UM5302TA"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_PRODUCT_NAME, "M5402RA"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_PRODUCT_NAME, "M5602RA"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_PRODUCT_NAME, "M6400RC"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_PRODUCT_NAME, "M3402RA"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_PRODUCT_NAME, "M6500RC"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_PRODUCT_NAME, "M6500RE"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_PRODUCT_NAME, "M6501RM"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_PRODUCT_NAME, "E1404FA"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_PRODUCT_NAME, "E1504FA"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_PRODUCT_NAME, "M7600RE"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_PRODUCT_NAME, "M3502RA"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Micro-Star International Co., Ltd.", DMI_PRODUCT_NAME, "Bravo 15 B7ED"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Micro-Star International Co., Ltd.", DMI_PRODUCT_NAME, "Bravo 15 C7VE"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Micro-Star International Co., Ltd.", DMI_PRODUCT_NAME, "Bravo 15 C7VF"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Micro-Star International Co., Ltd.", DMI_PRODUCT_NAME, "Bravo 17 D7VEK"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Micro-Star International Co., Ltd.", DMI_PRODUCT_NAME, "Bravo 17 D7VF"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Micro-Star International Co., Ltd.", DMI_PRODUCT_NAME, "Vector A16 HX A8WHG"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Micro-Star International Co., Ltd.", DMI_PRODUCT_NAME, "Crosshair A16 HX D7WFKG"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Micro-Star International Co., Ltd.", DMI_PRODUCT_NAME, "Raider A18 HX A7VHG"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Micro-Star International Co., Ltd.", DMI_PRODUCT_NAME, "Vector A16 HX A8WIG"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Micro-Star International Co., Ltd.", DMI_PRODUCT_NAME, "Raider A18 HX A9WJG"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Alienware", DMI_PRODUCT_NAME, "Alienware m15 R7 AMD"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Alienware", DMI_PRODUCT_NAME, "Alienware m17 R5 AMD"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Alienware", DMI_PRODUCT_NAME, "Alienware m18 R1 AMD"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "TIMI", DMI_PRODUCT_NAME, "Redmi Book Pro 14 2022"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "TIMI", DMI_PRODUCT_NAME, "Redmi Book Pro 15 2022"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "TIMI", DMI_PRODUCT_NAME, "Xiaomi Book Pro 14 2022"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "TIMI", DMI_PRODUCT_NAME, "Redmi G 2022"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Razer", DMI_PRODUCT_NAME, "Blade 14 (2022) - RZ09-0427"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "RB", DMI_PRODUCT_NAME, "Swift SFA16-41"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "IRBIS", DMI_PRODUCT_NAME, "15NBC1011"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_PRODUCT_NAME, "ASUS EXPERTBOOK PM1503CDA"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "HP", DMI_PRODUCT_NAME, "OMEN by HP Gaming Laptop 16z-n000"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "HP", DMI_PRODUCT_NAME, "Victus by HP Gaming Laptop 15-fb1xxx"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "HP", DMI_PRODUCT_NAME, "Victus by HP Gaming Laptop 15-fb2xxx"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "HP", DMI_BOARD_NAME, "8A42"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "HP", DMI_BOARD_NAME, "8A43"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "HP", DMI_BOARD_NAME, "8A44"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "HP", DMI_BOARD_NAME, "8A22"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "HP", DMI_BOARD_NAME, "8A3E"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "HP", DMI_BOARD_NAME, "8A7F"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "HP", DMI_BOARD_NAME, "8A81"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "HP", DMI_BOARD_NAME, "8B27"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "HP", DMI_BOARD_NAME, "8B2F"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "HP", DMI_BOARD_NAME, "8BD6"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "HP", DMI_BOARD_NAME, "8EE4"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "HP", DMI_BOARD_NAME, "8E35"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "HP", DMI_BOARD_NAME, "8F06"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "HP", DMI_PRODUCT_NAME, "Victus by HP Laptop 16-e1xxx"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "MECHREVO", DMI_BOARD_NAME, "MRID6"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "MDC", DMI_BOARD_NAME, "Herbag_MDU"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "System76", DMI_PRODUCT_VERSION, "pang12"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "System76", DMI_PRODUCT_VERSION, "pang13"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Micro-Star International Co., Ltd.", DMI_PRODUCT_NAME, "Bravo 15 C7UCX"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "HONOR", DMI_PRODUCT_NAME, "GOH-X"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "RB", DMI_BOARD_NAME, "XyloD5_RBU"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_PRODUCT_NAME, "Vivobook_ASUSLaptop M6501RR_M6501RR"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_PRODUCT_NAME, "ASUS EXPERTBOOK BM1503CDA"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_BOARD_NAME, "PM1503CDA"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_BOARD_NAME, "BM1403CDA"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_BOARD_NAME, "BM1403CDA"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Micro-Star International Co., Ltd.", DMI_PRODUCT_NAME, "Thin A15 B7VF"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Micro-Star International Co., Ltd.", DMI_PRODUCT_NAME, "Thin A15 B7VE"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Micro-Star International Co., Ltd.", DMI_PRODUCT_NAME, "Thin A15 B7UC"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_PRODUCT_NAME, "M7601RM"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "Micro-Star International Co., Ltd.", DMI_BOARD_NAME, "MS-17LN"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_BOARD_NAME, "PM1403CDA"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "ASUSTeK COMPUTER INC.", DMI_PRODUCT_NAME, "Vivobook_ASUSLaptop M6500RE_M6500RE"),
    yc_acp_quirk!(DMI_BOARD_VENDOR, "XIAOMI", DMI_BOARD_NAME, "TM2423"),
    dmi_system_id::default(),
];

unsafe extern "C" fn acp6x_probe(pdev: *mut platform_device) -> c_int {
    let mut dmi_id: *const dmi_system_id;
    let mut card: *mut snd_soc_card;
    let mut adev: *mut acpi_device;
    let mut handle: acpi_handle;
    let mut dmic_status: acpi_integer = 0;
    let mut ret: c_int;
    let mut is_dmic_enable: bool;
    let mut wov_en: bool;

    /* IF WOV entry not found, enable dmic based on AcpDmicConnected entry*/
    is_dmic_enable = false;
    wov_en = true;
    /* check the parent device's firmware node has _DSD or not */
    adev = ACPI_COMPANION((*pdev).dev.parent);
    if !adev.is_null() {
        let mut obj: *const acpi_object = core::ptr::null();

        if acpi_dev_get_property(adev, c"AcpDmicConnected".as_ptr(), ACPI_TYPE_INTEGER, &mut obj) == 0
            && unsafe { (*obj).integer.value == 1 }
        {
            is_dmic_enable = true;
        }
    }

    handle = ACPI_HANDLE((*pdev).dev.parent);
    ret = acpi_evaluate_integer(handle, c"_WOV".as_ptr(), core::ptr::null_mut(), &mut dmic_status);
    if !ACPI_FAILURE(ret) {
        wov_en = dmic_status != 0;
        if !wov_en {
            return -ENODEV;
        }

        if is_dmic_enable {
            platform_set_drvdata(pdev, &raw mut acp6x_card as *mut c_void);
        }
    } else {
        /* Incase of ACPI method read failure then jump to check_dmi_entry */
    }

    /* check for any DMI overrides */
    dmi_id = dmi_first_match(yc_acp_quirk_table.as_ptr());
    if !dmi_id.is_null() {
        platform_set_drvdata(pdev, (*dmi_id).driver_data);
    }

    card = platform_get_drvdata(pdev) as *mut snd_soc_card;
    if card.is_null() {
        return -ENODEV;
    }
    dev_info(
        &mut (*pdev).dev,
        c"Enabling ACP DMIC support via %s".as_ptr(),
        if !dmi_id.is_null() {
            c"DMI".as_ptr()
        } else {
            c"ACPI".as_ptr()
        },
    );
    acp6x_card.dev = &mut (*pdev).dev;

    ret = devm_snd_soc_register_card(&mut (*pdev).dev, card);
    if ret != 0 {
        return dev_err_probe(
            &mut (*pdev).dev,
            ret,
            c"snd_soc_register_card(%s) failed\n".as_ptr(),
            (*card).name,
        );
    }
    return 0;
}

static mut acp6x_mach_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"acp_yc_mach".as_ptr(),
        pm: unsafe { &raw const snd_soc_pm_ops },
    },
    probe: Some(acp6x_probe),
};

module_platform_driver!(acp6x_mach_driver);

MODULE_AUTHOR!(c"Vijendar.Mukunda@amd.com");
MODULE_DESCRIPTION!(c"AMD Yellow Carp support for DMIC");
MODULE_LICENSE!(c"GPL v2");
MODULE_ALIAS!(concatcp!(c"platform:", DRV_NAME));

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
