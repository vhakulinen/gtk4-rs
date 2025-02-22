// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RenderNode;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GskRenderer")]
    pub struct Renderer(Object<ffi::GskRenderer, ffi::GskRendererClass>);

    match fn {
        type_ => || ffi::gsk_renderer_get_type(),
    }
}

impl Renderer {
    pub const NONE: Option<&'static Renderer> = None;

    #[doc(alias = "gsk_renderer_new_for_surface")]
    #[doc(alias = "new_for_surface")]
    pub fn for_surface(surface: &gdk::Surface) -> Option<Renderer> {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gsk_renderer_new_for_surface(surface.to_glib_none().0)) }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Renderer>> Sealed for T {}
}

pub trait GskRendererExt: IsA<Renderer> + sealed::Sealed + 'static {
    #[doc(alias = "gsk_renderer_get_surface")]
    #[doc(alias = "get_surface")]
    fn surface(&self) -> Option<gdk::Surface> {
        unsafe {
            from_glib_none(ffi::gsk_renderer_get_surface(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_renderer_is_realized")]
    fn is_realized(&self) -> bool {
        unsafe {
            from_glib(ffi::gsk_renderer_is_realized(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_renderer_realize")]
    fn realize(&self, surface: Option<&gdk::Surface>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::gsk_renderer_realize(
                self.as_ref().to_glib_none().0,
                surface.to_glib_none().0,
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

    #[doc(alias = "gsk_renderer_render")]
    fn render(&self, root: impl AsRef<RenderNode>, region: Option<&cairo::Region>) {
        unsafe {
            ffi::gsk_renderer_render(
                self.as_ref().to_glib_none().0,
                root.as_ref().to_glib_none().0,
                region.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gsk_renderer_render_texture")]
    fn render_texture(
        &self,
        root: impl AsRef<RenderNode>,
        viewport: Option<&graphene::Rect>,
    ) -> gdk::Texture {
        unsafe {
            from_glib_full(ffi::gsk_renderer_render_texture(
                self.as_ref().to_glib_none().0,
                root.as_ref().to_glib_none().0,
                viewport.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_renderer_unrealize")]
    fn unrealize(&self) {
        unsafe {
            ffi::gsk_renderer_unrealize(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "realized")]
    fn connect_realized_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_realized_trampoline<P: IsA<Renderer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GskRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Renderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::realized\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_realized_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "surface")]
    fn connect_surface_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_surface_trampoline<P: IsA<Renderer>, F: Fn(&P) + 'static>(
            this: *mut ffi::GskRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Renderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::surface\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_surface_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Renderer>> GskRendererExt for O {}
