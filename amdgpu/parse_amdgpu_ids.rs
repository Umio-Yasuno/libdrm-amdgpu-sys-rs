use crate::*;

fn parse_amdgpu_ids(dev: AMDGPU::DEVICE_HANDLE) -> String {
    use std::fs::File;
    use std::io::BufReader;

    let dev_info = dev.device_info().unwrap();
    let ids = File::open("/usr/local/share/libdrm/amdgpu.ids")?;
    let reader = BufReader::new(ids);

    "".to_string()
}
