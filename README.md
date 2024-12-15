# libdrm-amdgpu-sys-rs
libdrm_amdgpu bindings for Rust, and some methods ported from Mesa3D (mainly [ac_gpu_info.c](https://gitlab.freedesktop.org/mesa/mesa/blob/main/src/amd/common/ac_gpu_info.c)).  

## Reference
 * [Mesa / drm · GitLab](https://gitlab.freedesktop.org/mesa/drm/)
 * [Mesa / mesa · GitLab](https://gitlab.freedesktop.org/mesa/mesa/)
 * Linux Kernel
    * `drivers/gpu/drm/amd/amdgpu/amdgpu_kms.c`

## License
 * MIT License

## Documentation
 * [libdrm_amdgpu_sys - Rust](https://docs.rs/libdrm_amdgpu_sys/latest/libdrm_amdgpu_sys/)

## Dependent dynamic libraries
 * libdrm
 * libdrm_amdgpu

### Distribution specific instructions
#### Debian/Ubuntu
```
sudo apt install libdrm-dev
```

## Examples
```
use libdrm_amdgpu_sys::AMDGPU::DeviceHandle;

let (amdgpu_dev, drm_major, drm_minor) = {
    use std::fs::OpenOptions;
    use std::os::fd::IntoRawFd;

    let fd = OpenOptions::new().read(true).write(true).open("/dev/dri/renderD128").unwrap();

    DeviceHandle::init(fd.into_raw_fd()).unwrap()
};
let mark_name = amdgpu_dev.get_marketing_name().unwrap();
let device_info = amdgpu_dev.device_info().unwrap();
```
### amdgpu_info
```
cargo run --example amdgpu_info
```
### vbios_dump
```
cargo run --example vbios_dump
```
### gpu_metrics
```
cargo run --example gpu_metrics
```
## Build
To generate a new `bindings/drm.rs` .

```
cargo build --features=buildtime_bindgen
```
