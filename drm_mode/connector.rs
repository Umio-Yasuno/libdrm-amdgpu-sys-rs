use crate::{bindings, query_error};
use crate::drmModeObjectProperties;
use core::ptr::addr_of;
pub use bindings::{drmModeConnectorPtr, drmModeModeInfo};

#[derive(Debug, Clone)]
pub struct drmModeConnector(pub(crate) drmModeConnectorPtr);

impl drmModeConnector {
    pub fn get(fd: i32, connector_id: u32) -> Option<Self> {
        let c_ptr = unsafe { bindings::drmModeGetConnector(fd, connector_id) };

        if c_ptr.is_null() { return None; }

        Some(Self(c_ptr))
    }

    pub fn get_current(fd: i32, connector_id: u32) -> Option<Self> {
        let c_ptr = unsafe { bindings::drmModeGetConnectorCurrent(fd, connector_id) };

        if c_ptr.is_null() { return None; }

        Some(Self(c_ptr))
    }

    pub fn set(
        fd: i32,
        connector_id: u32,
        property_id: u32,
        value: u64,
    ) -> Result<(), i32> {
        let r = unsafe {
            bindings::drmModeConnectorSetProperty(fd, connector_id, property_id, value)
        };

        query_error!(r);

        Ok(())
    }

    pub fn connection(&self) -> drmModeConnection {
        drmModeConnection::from(unsafe { addr_of!((*self.0).connection).read() })
    }

    pub fn connector_type(&self) -> drmModeConnectorType {
        drmModeConnectorType::from(unsafe { addr_of!((*self.0).connector_type).read() })
    }

    pub fn connector_id(&self) -> u32 {
        unsafe { addr_of!((*self.0).connector_id).read() }
    }

    pub fn connector_type_id(&self) -> u32 {
        unsafe { addr_of!((*self.0).connector_type_id).read() }
    }

    pub fn encoder_id(&self) -> u32 {
        unsafe { addr_of!((*self.0).encoder_id).read() }
    }

    pub fn mmWidth(&self) -> u32 {
        unsafe { addr_of!((*self.0).mmWidth).read() }
    }

    pub fn mmHeight(&self) -> u32 {
        unsafe { addr_of!((*self.0).mmHeight).read() }
    }

    pub fn get_connector_props(&self, fd: i32) -> Option<drmModeObjectProperties> {
        drmModeObjectProperties::get(
            fd,
            self.connector_id(),
            bindings::DRM_MODE_OBJECT_CONNECTOR,
        )
    }

    pub fn get_modes(&self) -> Vec<drmModeModeInfo> {
        unsafe { std::slice::from_raw_parts(
            addr_of!((*self.0).modes).read(),
            addr_of!((*self.0).count_modes).read() as usize,
        ) }.to_vec()
    }
}

impl Drop for drmModeConnector {
    fn drop(&mut self) {
	    unsafe { bindings::drmModeFreeConnector(self.0); }
    }
}

use bindings::{
    DRM_MODE_CONNECTOR_Unknown,
    DRM_MODE_CONNECTOR_VGA,
    DRM_MODE_CONNECTOR_DVII,
    DRM_MODE_CONNECTOR_DVID,
    DRM_MODE_CONNECTOR_DVIA,
    DRM_MODE_CONNECTOR_Composite,
    DRM_MODE_CONNECTOR_SVIDEO,
    DRM_MODE_CONNECTOR_LVDS,
    DRM_MODE_CONNECTOR_Component,
    DRM_MODE_CONNECTOR_9PinDIN,
    DRM_MODE_CONNECTOR_DisplayPort,
    DRM_MODE_CONNECTOR_HDMIA,
    DRM_MODE_CONNECTOR_HDMIB,
    DRM_MODE_CONNECTOR_TV,
    DRM_MODE_CONNECTOR_eDP,
    DRM_MODE_CONNECTOR_VIRTUAL,
    DRM_MODE_CONNECTOR_DSI,
    DRM_MODE_CONNECTOR_DPI,
    DRM_MODE_CONNECTOR_WRITEBACK,
    DRM_MODE_CONNECTOR_SPI,
    DRM_MODE_CONNECTOR_USB,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum drmModeConnectorType {
    Unknown = DRM_MODE_CONNECTOR_Unknown,
    VGA = DRM_MODE_CONNECTOR_VGA,
    DVII = DRM_MODE_CONNECTOR_DVII,
    DVID = DRM_MODE_CONNECTOR_DVID,
    DVIA = DRM_MODE_CONNECTOR_DVIA,
    Composite = DRM_MODE_CONNECTOR_Composite,
    SVIDEO = DRM_MODE_CONNECTOR_SVIDEO,
    LVDS = DRM_MODE_CONNECTOR_LVDS,
    Component = DRM_MODE_CONNECTOR_Component,
    // 9PinDIN = DRM_MODE_CONNECTOR_9PinDIN,
    DIN_9Pin = DRM_MODE_CONNECTOR_9PinDIN,
    DisplayPort = DRM_MODE_CONNECTOR_DisplayPort,
    HDMIA = DRM_MODE_CONNECTOR_HDMIA,
    HDMIB = DRM_MODE_CONNECTOR_HDMIB,
    TV = DRM_MODE_CONNECTOR_TV,
    eDP = DRM_MODE_CONNECTOR_eDP,
    VIRTUAL = DRM_MODE_CONNECTOR_VIRTUAL,
    DSI = DRM_MODE_CONNECTOR_DSI,
    DPI = DRM_MODE_CONNECTOR_DPI,
    WRITEBACK = DRM_MODE_CONNECTOR_WRITEBACK,
    SPI = DRM_MODE_CONNECTOR_SPI,
    USB = DRM_MODE_CONNECTOR_USB,
}

impl From<u32> for drmModeConnectorType {
    fn from(value: u32) -> Self {
        match value {
            DRM_MODE_CONNECTOR_VGA => Self::VGA,
            DRM_MODE_CONNECTOR_DVII => Self::DVII,
            DRM_MODE_CONNECTOR_DVID => Self::DVID,
            DRM_MODE_CONNECTOR_DVIA => Self::DVIA,
            DRM_MODE_CONNECTOR_Composite => Self::Composite,
            DRM_MODE_CONNECTOR_SVIDEO => Self::SVIDEO,
            DRM_MODE_CONNECTOR_LVDS => Self::LVDS,
            DRM_MODE_CONNECTOR_Component => Self::Component,
            DRM_MODE_CONNECTOR_9PinDIN => Self::DIN_9Pin,
            DRM_MODE_CONNECTOR_DisplayPort => Self::DisplayPort,
            DRM_MODE_CONNECTOR_HDMIA => Self::HDMIA,
            DRM_MODE_CONNECTOR_HDMIB => Self::HDMIB,
            DRM_MODE_CONNECTOR_TV => Self::TV,
            DRM_MODE_CONNECTOR_eDP => Self::eDP,
            DRM_MODE_CONNECTOR_VIRTUAL => Self::VIRTUAL,
            DRM_MODE_CONNECTOR_DSI => Self::DSI,
            DRM_MODE_CONNECTOR_DPI => Self::DPI,
            DRM_MODE_CONNECTOR_WRITEBACK => Self::WRITEBACK,
            DRM_MODE_CONNECTOR_SPI => Self::SPI,
            DRM_MODE_CONNECTOR_USB => Self::USB,
            DRM_MODE_CONNECTOR_Unknown | _ => Self::Unknown,
        }
    }
}

#[cfg(feature = "std")]
use std::fmt;

#[cfg(feature = "std")]
impl fmt::Display for drmModeConnectorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::HDMIA => write!(f, "HDMI-A"),
            Self::HDMIB => write!(f, "HDMI-B"),
            _ => write!(f, "{:?}", self),
        }
    }
}

use bindings::{
    drmModeConnection_DRM_MODE_CONNECTED,
    drmModeConnection_DRM_MODE_DISCONNECTED,
    drmModeConnection_DRM_MODE_UNKNOWNCONNECTION,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum drmModeConnection {
    DRM_MODE_CONNECTED = drmModeConnection_DRM_MODE_CONNECTED,
    DRM_MODE_DISCONNECTED = drmModeConnection_DRM_MODE_DISCONNECTED,
    DRM_MODE_UNKNOWNCONNECTION = drmModeConnection_DRM_MODE_UNKNOWNCONNECTION,
}

#[cfg(feature = "std")]
impl fmt::Display for drmModeConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::DRM_MODE_CONNECTED => "connected",
            Self::DRM_MODE_DISCONNECTED => "disconnected",
            Self::DRM_MODE_UNKNOWNCONNECTION => "unknown",
        })
    }
}

impl From<u32> for drmModeConnection {
    fn from(value: u32) -> Self {
        match value {
            drmModeConnection_DRM_MODE_CONNECTED => Self::DRM_MODE_CONNECTED,
            drmModeConnection_DRM_MODE_DISCONNECTED => Self::DRM_MODE_DISCONNECTED,
            drmModeConnection_DRM_MODE_UNKNOWNCONNECTION | _ => Self::DRM_MODE_UNKNOWNCONNECTION,
        }
    }
}
