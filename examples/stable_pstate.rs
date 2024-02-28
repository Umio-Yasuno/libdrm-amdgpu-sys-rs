use libdrm_amdgpu_sys::*;

fn info(pci_bus: &PCI::BUS_INFO) {
    let Ok(device_path) = pci_bus.get_drm_render_path() else { return };
    let (amdgpu_dev, _major, _minor) = {
        use std::fs::File;
        use std::os::fd::IntoRawFd;

        let fd = File::open(device_path).unwrap();

        AMDGPU::DeviceHandle::init(fd.into_raw_fd()).unwrap()
    };

    println!("Marketing Name: [{}]", amdgpu_dev.get_marketing_name_or_default());

    {
        let ctx = amdgpu_dev.create_context().unwrap();
        let current_stable_pstate = ctx.get_stable_pstate().unwrap();
        println!("{current_stable_pstate:?}");

        ctx.set_stable_pstate(AMDGPU::StablePstateFlag::STANDARD).unwrap();
        let p = ctx.get_stable_pstate().unwrap();

        println!("  {p:?}");

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
