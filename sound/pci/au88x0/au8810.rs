// SPDX-License-Identifier: GPL-2.0
// C dependencies: "au8810.h", "au88x0.h"

static snd_vortex_ids: [pci_device_id; 2] = [
    pci_device_id {
        vendor: PCI_VENDOR_ID_AUREAL,
        device: PCI_DEVICE_ID_AUREAL_ADVANTAGE,
        subvendor: PCI_ANY_ID,
        subdevice: PCI_ANY_ID,
        class: 0,
        class_mask: 0,
        driver_data: 1,
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

// C implementation inclusions translated as dependency intent:
// "au88x0_core.c"
// "au88x0_pcm.c"
// "au88x0_mixer.c"
// "au88x0_mpu401.c"
// "au88x0_game.c"
// "au88x0_eq.c"
// "au88x0_a3d.c"
// "au88x0_xtalk.c"
// "au88x0.c"

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
