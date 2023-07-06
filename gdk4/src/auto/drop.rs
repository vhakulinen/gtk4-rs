// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ContentFormats, Device, Display, Drag, DragAction, Surface};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GdkDrop")]
    pub struct Drop(Object<ffi::GdkDrop>);

    match fn {
        type_ => || ffi::gdk_drop_get_type(),
    }
}

impl Drop {
    #[doc(alias = "gdk_drop_finish")]
    pub fn finish(&self, action: DragAction) {
        unsafe {
            ffi::gdk_drop_finish(self.to_glib_none().0, action.into_glib());
        }
    }

    #[doc(alias = "gdk_drop_get_actions")]
    #[doc(alias = "get_actions")]
    pub fn actions(&self) -> DragAction {
        unsafe { from_glib(ffi::gdk_drop_get_actions(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drop_get_device")]
    #[doc(alias = "get_device")]
    pub fn device(&self) -> Device {
        unsafe { from_glib_none(ffi::gdk_drop_get_device(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drop_get_display")]
    #[doc(alias = "get_display")]
    pub fn display(&self) -> Display {
        unsafe { from_glib_none(ffi::gdk_drop_get_display(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drop_get_drag")]
    #[doc(alias = "get_drag")]
    pub fn drag(&self) -> Option<Drag> {
        unsafe { from_glib_none(ffi::gdk_drop_get_drag(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drop_get_formats")]
    #[doc(alias = "get_formats")]
    pub fn formats(&self) -> ContentFormats {
        unsafe { from_glib_none(ffi::gdk_drop_get_formats(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drop_get_surface")]
    #[doc(alias = "get_surface")]
    pub fn surface(&self) -> Surface {
        unsafe { from_glib_none(ffi::gdk_drop_get_surface(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_drop_status")]
    pub fn status(&self, actions: DragAction, preferred: DragAction) {
        unsafe {
            ffi::gdk_drop_status(
                self.to_glib_none().0,
                actions.into_glib(),
                preferred.into_glib(),
            );
        }
    }

    #[doc(alias = "display")]
    pub fn connect_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_display_trampoline<F: Fn(&Drop) + 'static>(
            this: *mut ffi::GdkDrop,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::display\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_display_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Drop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Drop")
    }
}
