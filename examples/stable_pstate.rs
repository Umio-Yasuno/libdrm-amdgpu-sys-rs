use libdrm_amdgpu_sys::*;

fn info(pci_bus: &PCI::BUS_INFO) {
    let libdrm_amdgpu = LibDrmAmdgpu::new().unwrap();
    let Ok(device_path) = pci_bus.get_drm_render_path() else { return };
    let (amdgpu_dev, _major, _minor) = {
        use std::fs::File;
        use std::os::fd::IntoRawFd;

        let fd = File::open(device_path).unwrap();

        libdrm_amdgpu.init_device_handle(f.into_raw_fd()).unwrap()
    };

    println!("Marketing Name: [{}]", amdgpu_dev.get_marketing_name_or_default());

    {
        let ctx = amdgpu_dev.create_context().unwrap();
        let current_stable_pstate = ctx.get_stable_pstate().unwrap();
        println!("Current Stable PState: {current_stable_pstate:?}");

        println!("Set STANDARD PState");

        match ctx.set_stable_pstate(AMDGPU::StablePstateFlag::STANDARD) {
            Ok(_) => {
            },
            Err(err) => println!("    Error: {err}"),
        }

        println!("Press enter to revert stable_pstate");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }

    println!("  stable_pstate is reverted.");
}

fn main() {
    let pci_devs = AMDGPU::get_all_amdgpu_pci_bus();

    if pci_devs.is_empty() {
        panic!("No AMDGPU devices.");
    }

    for pci_bus in &pci_devs {
        info(pci_bus);
    }
}
