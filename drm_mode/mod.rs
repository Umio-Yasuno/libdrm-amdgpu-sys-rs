mod resource;
pub use resource::*;

mod connector;
pub use connector::*;

mod object_property;
pub use object_property::*;

mod property;
pub use property::*;

mod property_blob;
pub use property_blob::*;

mod mode_info;
#[allow(unused_imports)]
pub use mode_info::*;

// TODO: crtc, encoder

pub(crate) fn c_char_to_string(c: &[core::ffi::c_char]) -> String {
    let c_name: Vec<u8> = c.iter().map(|c| *c as u8).collect();

    if let Some(index) = c_name.iter().position(|&x| x == 0) {
        String::from_utf8_lossy(c_name.get(..index).unwrap_or_default())
    } else {
        String::from_utf8_lossy(&c_name)
    }.to_string()
}
