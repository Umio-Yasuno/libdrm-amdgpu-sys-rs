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

## Feature flags
 * `link_drm`
   * Dynamically linking `libdrm.so.2` and `libdrm_amdgpu.so.1`
   * enabled by default
 * `dynamic_loading`
   * Dynamically loading `libdrm.so.2` and `libdrm_amdgpu.so.1`
   * `link_drm` and `dynamic_loading` cannot be enabled at the same time.
   * `cargo add libdrm_amdgpu_sys --no-default-features -F "dynamic_loading"`

### Distribution specific instructions
#### Debian/Ubuntu
```
sudo apt install libdrm-dev
```

## Examples
```
use libdrm_amdgpu_sys::LibDrmAmdgpu;
use libdrm_amdgpu_sys::AMDGPU::DeviceHandle;
use libdrm_amdgpu_sys::AMDGPU::GPU_INFO;

let libdrm_amdgpu = LibDrmAmdgpu::new().unwrap();
let (amdgpu_dev, drm_major, drm_minor) = {
    use std::fs::OpenOptions;
    use std::os::fd::IntoRawFd;

    let fd = OpenOptions::new().read(true).write(true).open("/dev/dri/renderD128").unwrap();

    libdrm_amdgpu.init_device_handle(fd.into_raw_fd()).unwrap()
};
let device_info = amdgpu_dev.device_info().unwrap();
let device_name = device_info.find_device_name_or_default();
```
### Dynamic Loading
#### Cargo.toml
```
libdrm_amdgpu_sys = { version = "0.8", default-features = false, features = ["dynamic_loading"] }
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
