use crate::{bindings, query_error, LibDrm};
use core::ptr::addr_of;

pub use bindings::drmModeObjectPropertiesPtr;
use crate::drmModeProperty;

#[derive(Clone)]
pub struct drmModeObjectProperties {
    pub(crate) ptr: drmModeObjectPropertiesPtr,
    pub(crate) lib: LibDrm,
}

impl LibDrm {
    pub fn get_drm_mode_object_properties(
        &self,
        fd: i32,
        object_id: u32,
        object_type: u32,
    ) -> Option<drmModeObjectProperties> {
        #[cfg(feature = "link_drm")]
        let func = bindings::drmModeObjectGetProperties;
        #[cfg(feature = "dynamic_loading")]
        let func = self.libdrm.drmModeObjectGetProperties;

        let obj_ptr = unsafe { func(fd, object_id, object_type) };

        if obj_ptr.is_null() {
            None
        } else {
            Some(drmModeObjectProperties {
                ptr: obj_ptr,
                lib: self.clone(),
            })
        }
    }

    pub fn set_drm_mode_object_property(
        &self,
        fd: i32,
        object_id: u32,
        object_type: u32,
        property_id: u32,
        value: u64,
    ) -> Result<(), i32> {
        #[cfg(feature = "link_drm")]
        let func = bindings::drmModeObjectSetProperty;
        #[cfg(feature = "dynamic_loading")]
        let func = self.libdrm.drmModeObjectSetProperty;

        let r = unsafe { func(fd, object_id, object_type, property_id, value) };

        query_error!(r);

        Ok(())
    }

}

impl drmModeObjectProperties {
    #[cfg(feature = "link_drm")]
    pub fn get(fd: i32, object_id: u32, object_type: u32) -> Option<Self> {
        let obj_ptr = unsafe { bindings::drmModeObjectGetProperties(
            fd,
            object_id,
            object_type,
        ) };

        if obj_ptr.is_null() {
            None
        } else {
            Some(Self { ptr: obj_ptr, lib: LibDrm::new().unwrap() })
        }
    }

    #[cfg(feature = "link_drm")]
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
        let props_ptr = unsafe { addr_of!((*self.ptr).props).read() };
        let values_ptr = unsafe { addr_of!((*self.ptr).prop_values).read() };

        if props_ptr.is_null() || values_ptr.is_null() {
            return Vec::new();
        }

        let count = unsafe { addr_of!((*self.ptr).count_props).read() as usize };
        let props = unsafe { std::slice::from_raw_parts(props_ptr, count) };
        let values = unsafe { std::slice::from_raw_parts(values_ptr, count) };

        props.iter().zip(values.iter()).filter_map(|(prop_id, value)| {
            let prop = self.lib.get_drm_mode_property(fd, *prop_id)?;

            Some((prop, *value))
        }).collect()
    }
}

impl Drop for drmModeObjectProperties {
    fn drop(&mut self) {
        #[cfg(feature = "link_drm")]
        let func = bindings::drmModeFreeObjectProperties;
        #[cfg(feature = "dynamic_loading")]
        let func = self.lib.libdrm.drmModeFreeObjectProperties;

	    unsafe { func(self.ptr); }
    }
}
