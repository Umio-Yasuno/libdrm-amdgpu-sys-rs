use crate::{bindings, query_error};
use core::ptr::addr_of;

pub use bindings::drmModeObjectPropertiesPtr;
use crate::drmModeProperty;

#[derive(Debug, Clone)]
pub struct drmModeObjectProperties(pub(crate) drmModeObjectPropertiesPtr);

impl drmModeObjectProperties {
    pub fn get(fd: i32, object_id: u32, object_type: u32) -> Option<Self> {
        let obj_ptr = unsafe { bindings::drmModeObjectGetProperties(
            fd,
            object_id,
            object_type,
        ) };

        if obj_ptr.is_null() {
            None
        } else {
            Some(Self(obj_ptr))
        }
    }

    pub fn set(
        fd: i32,
        object_id: u32,
        object_type: u32,
        property_id: u32,
        value: u64,
    ) -> Result<(), i32> {
        let r = unsafe { bindings::drmModeObjectSetProperty(
            fd,
            object_id,
            object_type,
            property_id,
            value,
        ) };

        query_error!(r);

        Ok(())
    }

    pub fn get_mode_property(&self, fd: i32) -> Vec<(drmModeProperty, u64)> {
        let props_ptr = unsafe { addr_of!((*self.0).props).read() };
        let values_ptr = unsafe { addr_of!((*self.0).prop_values).read() };

        if props_ptr.is_null() || values_ptr.is_null() {
            return Vec::new();
        }

        let count = unsafe { addr_of!((*self.0).count_props).read() as usize };

        let props = unsafe { std::slice::from_raw_parts(props_ptr, count) };
        let values = unsafe { std::slice::from_raw_parts(values_ptr, count) };

        props.iter().zip(values.iter()).filter_map(|(prop_id, value)| {
            let prop = drmModeProperty::get(fd, *prop_id)?;

            Some((prop, *value))
        }).collect()
    }
}

impl Drop for drmModeObjectProperties {
    fn drop(&mut self) {
	    unsafe { bindings::drmModeFreeObjectProperties(self.0); }
    }
}
