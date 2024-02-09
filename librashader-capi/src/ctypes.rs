//! Binding types for the librashader C API.
use crate::error::LibrashaderError;
use librashader::presets::context::{Orientation, VideoDriver, WildcardContext};
use librashader::presets::ShaderPreset;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

/// A handle to a shader preset object.
pub type libra_shader_preset_t = Option<NonNull<ShaderPreset>>;

/// A handle to a preset wildcard context object.
pub type libra_preset_ctx_t = Option<NonNull<WildcardContext>>;

/// A handle to a librashader error object.
pub type libra_error_t = Option<NonNull<LibrashaderError>>;

/// An enum representing orientation for use in preset contexts.
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum LIBRA_PRESET_CTX_ORIENTATION {
    Vertical = 0,
    Horizontal,
}
impl From<LIBRA_PRESET_CTX_ORIENTATION> for Orientation {
    fn from(value: LIBRA_PRESET_CTX_ORIENTATION) -> Self {
        match value {
            LIBRA_PRESET_CTX_ORIENTATION::Vertical => Orientation::Vertical,
            LIBRA_PRESET_CTX_ORIENTATION::Horizontal => Orientation::Horizontal,
        }
    }
}

// An enum representing graphics runtimes (video drivers) for use in preset contexts.
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum LIBRA_PRESET_CTX_RUNTIME {
    None = 0,
    GlCore,
    Vulkan,
    D3D11,
    D3D12,
    Metal,
}

impl From<LIBRA_PRESET_CTX_RUNTIME> for VideoDriver {
    fn from(value: LIBRA_PRESET_CTX_RUNTIME) -> Self {
        match value {
            LIBRA_PRESET_CTX_RUNTIME::None => VideoDriver::None,
            LIBRA_PRESET_CTX_RUNTIME::GlCore => VideoDriver::GlCore,
            LIBRA_PRESET_CTX_RUNTIME::Vulkan => VideoDriver::Vulkan,
            LIBRA_PRESET_CTX_RUNTIME::D3D11 => VideoDriver::Direct3D11,
            LIBRA_PRESET_CTX_RUNTIME::D3D12 => VideoDriver::Direct3D12,
            LIBRA_PRESET_CTX_RUNTIME::Metal => VideoDriver::Metal,
        }
    }
}

/// A handle to a OpenGL filter chain.
#[cfg(feature = "runtime-opengl")]
#[doc(cfg(feature = "runtime-opengl"))]
pub type libra_gl_filter_chain_t = Option<NonNull<librashader::runtime::gl::capi::FilterChainGL>>;

/// A handle to a Direct3D 11 filter chain.
#[cfg(all(target_os = "windows", feature = "runtime-d3d11"))]
#[doc(cfg(all(target_os = "windows", feature = "runtime-d3d11")))]
pub type libra_d3d11_filter_chain_t =
    Option<NonNull<librashader::runtime::d3d11::capi::FilterChainD3D11>>;

/// A handle to a Direct3D 12 filter chain.
#[cfg(all(target_os = "windows", feature = "runtime-d3d12"))]
#[doc(cfg(all(target_os = "windows", feature = "runtime-d3d12")))]
pub type libra_d3d12_filter_chain_t =
    Option<NonNull<librashader::runtime::d3d12::capi::FilterChainD3D12>>;

/// A handle to a Vulkan filter chain.
#[cfg(feature = "runtime-vulkan")]
#[doc(cfg(feature = "runtime-vulkan"))]
pub type libra_vk_filter_chain_t =
    Option<NonNull<librashader::runtime::vk::capi::FilterChainVulkan>>;

/// Defines the output viewport for a rendered frame.
#[repr(C)]
pub struct libra_viewport_t {
    /// The x offset in the viewport framebuffer to begin rendering from.
    pub x: f32,
    /// The y offset in the viewport framebuffer to begin rendering from.
    pub y: f32,
    /// The width of the viewport framebuffer.
    pub width: u32,
    /// The height of the viewport framebuffer.
    pub height: u32,
}

pub(crate) trait FromUninit<T>
where
    Self: Sized,
{
    fn from_uninit(value: MaybeUninit<Self>) -> T;
}

macro_rules! config_set_field {
    ($options:ident.$field:ident <- $ptr:ident) => {
        $options.$field = unsafe { ::std::ptr::addr_of!((*$ptr).$field).read() };
    };
}

macro_rules! config_version_set {
    ($version:literal => [$($field:ident),+ $(,)?] ($options:ident <- $ptr:ident)) => {
        let version = unsafe { ::std::ptr::addr_of!((*$ptr).version).read() };
        #[allow(unused_comparisons)]
        if version >= $version {
            $($crate::ctypes::config_set_field!($options.$field <- $ptr);)+
        }
    }
}

macro_rules! config_struct {
    (impl $rust:ty => $capi:ty {$($version:literal => [$($field:ident),+ $(,)?]);+ $(;)?}) => {
        impl $crate::ctypes::FromUninit<$rust> for $capi {
            fn from_uninit(value: ::std::mem::MaybeUninit<Self>) -> $rust {
                let ptr = value.as_ptr();
                let mut options = <$rust>::default();
                $(
                    $crate::ctypes::config_version_set!($version => [$($field),+] (options <- ptr));
                )+
                options
            }
        }
    }
}

pub(crate) use config_set_field;
pub(crate) use config_struct;
pub(crate) use config_version_set;
