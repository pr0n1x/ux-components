#![allow(unused_variables)]

// use std::boxed::Box as Box_;
// use std::mem::transmute;

use super::{Menu, Style};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends clutter::Actor;
#[derive(Clone, Debug)]
pub struct Widget {}

impl UxObject for Widget {}
impl Is<Widget> for Widget {}

impl AsRef<Widget> for Widget {
    fn as_ref(&self) -> &Widget {
        unimplemented!()
    }
}

pub const NONE_WIDGET: Option<&Widget> = None;

pub trait WidgetExt: 'static {
    fn apply_style<P: Is<Style>>(&self, style: &P);

    fn get_available_area(&self, allocation: &clutter::ActorBox, area: &mut clutter::ActorBox);

    fn get_background_color(&self) -> Option<clutter::Color>;

    //fn get_background_texture(&self) -> /*Ignored*/Option<cogl::Handle>;

    fn get_disabled(&self) -> bool;

    fn get_menu(&self) -> Option<Menu>;

    //fn get_padding(&self, padding: /*Ignored*/Padding);

    fn get_tooltip_delay(&self) -> u32;

    fn get_tooltip_text(&self) -> Option<String>;

    fn hide_tooltip(&self);

    fn long_press_cancel(&self);

    fn long_press_query(&self, event: &mut clutter::Event);

    fn set_disabled(&self, disabled: bool);

    fn set_menu<P: Is<Menu>>(&self, menu: &P);

    fn set_tooltip_delay(&self, delay: u32);

    fn set_tooltip_text(&self, text: &str);

    fn show_tooltip(&self);

    //fn connect_long_press<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_menu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tooltip_delay_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_tooltip_text_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: Is<Widget>> WidgetExt for O {
    fn apply_style<P: Is<Style>>(&self, style: &P) {
        // unsafe {
        //     ffi::widget_apply_style(
        //         self.as_ref().to_glib_none().0,
        //         style.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_available_area(&self, allocation: &clutter::ActorBox, area: &mut clutter::ActorBox) {
        // unsafe {
        //     ffi::widget_get_available_area(
        //         self.as_ref().to_glib_none().0,
        //         allocation.to_glib_none().0,
        //         area.to_glib_none_mut().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_background_color(&self) -> Option<clutter::Color> {
        // unsafe {
        //     from_glib_none(ffi::widget_get_background_color(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    //fn get_background_texture(&self) -> /*Ignored*/Option<cogl::Handle> {
    //    unsafe { TODO: call ffi:widget_get_background_texture() }
    //}

    fn get_disabled(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::widget_get_disabled(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_menu(&self) -> Option<Menu> {
        // unsafe { from_glib_none(ffi::widget_get_menu(self.as_ref().to_glib_none().0)) }
        unimplemented!()
    }

    //fn get_padding(&self, padding: /*Ignored*/Padding) {
    //    unsafe { TODO: call ffi:widget_get_padding() }
    //}

    fn get_tooltip_delay(&self) -> u32 {
        // unsafe { ffi::widget_get_tooltip_delay(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_tooltip_text(&self) -> Option<String> {
        // unsafe {
        //     from_glib_none(ffi::widget_get_tooltip_text(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn hide_tooltip(&self) {
        // unsafe {
        //     ffi::widget_hide_tooltip(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn long_press_cancel(&self) {
        // unsafe {
        //     ffi::widget_long_press_cancel(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn long_press_query(&self, event: &mut clutter::Event) {
        // unsafe {
        //     ffi::widget_long_press_query(
        //         self.as_ref().to_glib_none().0,
        //         event.to_glib_none_mut().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_disabled(&self, disabled: bool) {
        // unsafe {
        //     ffi::widget_set_disabled(self.as_ref().to_glib_none().0, disabled.to_glib());
        // }
        unimplemented!()
    }

    fn set_menu<P: Is<Menu>>(&self, menu: &P) {
        // unsafe {
        //     ffi::widget_set_menu(
        //         self.as_ref().to_glib_none().0,
        //         menu.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_tooltip_delay(&self, delay: u32) {
        // unsafe {
        //     ffi::widget_set_tooltip_delay(self.as_ref().to_glib_none().0, delay);
        // }
        unimplemented!()
    }

    fn set_tooltip_text(&self, text: &str) {
        // unsafe {
        //     ffi::widget_set_tooltip_text(
        //         self.as_ref().to_glib_none().0,
        //         text.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn show_tooltip(&self) {
        // unsafe {
        //     ffi::widget_show_tooltip(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    //fn connect_long_press<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored p1: Mx.LongPressAction
    //}

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_disabled_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Widget,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Widget>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Widget::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::disabled\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_disabled_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_menu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_menu_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Widget,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Widget>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Widget::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::menu\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_menu_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_tooltip_delay_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_tooltip_delay_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Widget,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Widget>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Widget::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::tooltip-delay\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_tooltip_delay_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_tooltip_text_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_tooltip_text_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Widget,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Widget>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Widget::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::tooltip-text\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_tooltip_text_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Widget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Widget")
    }
}
