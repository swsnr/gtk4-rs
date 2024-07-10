// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, RenderNodeType};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GskRenderNode")]
    pub struct RenderNode(Shared<ffi::GskRenderNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr),
        unref => |ptr| ffi::gsk_render_node_unref(ptr),
    }
}

impl StaticType for RenderNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_render_node_get_type()) }
    }
}

impl RenderNode {
    pub const NONE: Option<&'static RenderNode> = None;

    #[doc(alias = "gsk_render_node_draw")]
    pub fn draw(&self, cr: &cairo::Context) {
        unsafe {
            ffi::gsk_render_node_draw(
                self.as_ref().to_glib_none().0,
                mut_override(cr.to_glib_none().0),
            );
        }
    }

    #[doc(alias = "gsk_render_node_get_bounds")]
    #[doc(alias = "get_bounds")]
    pub fn bounds(&self) -> graphene::Rect {
        unsafe {
            let mut bounds = graphene::Rect::uninitialized();
            ffi::gsk_render_node_get_bounds(
                self.as_ref().to_glib_none().0,
                bounds.to_glib_none_mut().0,
            );
            bounds
        }
    }

    #[doc(alias = "gsk_render_node_get_node_type")]
    #[doc(alias = "get_node_type")]
    pub fn node_type(&self) -> RenderNodeType {
        unsafe {
            from_glib(ffi::gsk_render_node_get_node_type(const_override(
                self.as_ref().to_glib_none().0,
            )))
        }
    }

    #[cfg(feature = "v4_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_16")))]
    #[doc(alias = "gsk_render_node_get_opaque_rect")]
    #[doc(alias = "get_opaque_rect")]
    pub fn opaque_rect(&self) -> Option<graphene::Rect> {
        unsafe {
            let mut out_opaque = graphene::Rect::uninitialized();
            let ret = from_glib(ffi::gsk_render_node_get_opaque_rect(
                self.as_ref().to_glib_none().0,
                out_opaque.to_glib_none_mut().0,
            ));
            if ret {
                Some(out_opaque)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gsk_render_node_serialize")]
    pub fn serialize(&self) -> glib::Bytes {
        unsafe {
            from_glib_full(ffi::gsk_render_node_serialize(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_render_node_write_to_file")]
    pub fn write_to_file(&self, filename: impl AsRef<std::path::Path>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::gsk_render_node_write_to_file(
                self.as_ref().to_glib_none().0,
                filename.as_ref().to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
