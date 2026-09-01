// SPDX-License-Identifier: GPL-2.0
// Rust translation of pci/au88x0/au8820.c.
//
// Original C dependency includes:
// #include "au8820.h"
// #include "au88x0.h"

pub const snd_vortex_ids: [pci_device_id; 2] = [
    pci_device_id {
        vendor: PCI_VENDOR_ID_AUREAL,
        device: PCI_DEVICE_ID_AUREAL_VORTEX_1,
        subvendor: PCI_ANY_ID,
        subdevice: PCI_ANY_ID,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    },
    pci_device_id {
        vendor: 0,
        device: 0,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    },
];

// Original C implementation includes, translated as external module dependencies:
// #include "au88x0_synth.c"
// #include "au88x0_core.c"
// #include "au88x0_pcm.c"
// #include "au88x0_mpu401.c"
// #include "au88x0_game.c"
// #include "au88x0_mixer.c"
// #include "au88x0.c"

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
