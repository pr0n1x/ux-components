#![allow(unused_variables)]

use std::boxed::Box as Box_;
// use std::mem;
// use std::mem::transmute;

use super::{Toolbar, WindowRotation};
use crate::prelude::*;

use glib::signal::SignalHandlerId;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Stage {
    pub inner: clutter::Stage,

    // pub native_window: NativeWindow,
    pub has_toolbar: bool,
    pub small_screen: bool,
    pub fullscreen: bool,
    pub rotate_size: bool,

    pub icon_name: String,
    // pub icon_texture: cogl::Handle,
    pub stage: Option<clutter::Actor>,
    pub toolbar: Option<Toolbar>,
    pub child: Option<clutter::Actor>,
    pub resize_grip: Option<clutter::Actor>,
    pub debug_actor: Option<clutter::Actor>,

    pub rotation: WindowRotation,
    pub rotation_timeline: Option<clutter::Timeline>,
    pub start_angle: f32,
    pub end_angle: f32,
    pub angle: f32,
}

impl Stage {
    pub fn new() -> Stage {
        Self {
            inner: clutter::Stage::new(),
            has_toolbar: false,
            small_screen: false,
            fullscreen: false,
            rotate_size: false,
            icon_name: "".into(),
            stage: None,
            toolbar: None,
            child: None,
            resize_grip: None,
            debug_actor: None,
            rotation: WindowRotation::Rotation0,
            rotation_timeline: None,
            start_angle: 0.0,
            end_angle: 0.0,
            angle: 0.0,
        }
    }

    /// new_with_clutter_stage:
    /// @stage: A #ClutterStage
    ///
    /// Creates a new #Stage, using @stage as the backing #ClutterStage. This
    /// function is meant for use primarily for embedding a #Stage into
    /// a foreign stage when using a Clutter toolkit integration library.
    ///
    /// Returns: A #Stage
    ///
    pub fn with_clutter_stage(stage: &clutter::Stage) -> Stage {
        //    unsafe { TODO: call ffi:window_new_with_clutter_stage() }
        unimplemented!()
    }

    /// get_for_stage:
    /// @stage: A #ClutterStage
    ///
    /// Gets the #Stage parent of the #ClutterStage, if it exists.
    ///
    /// Returns: (transfer none): A #Stage, or %NULL
    ///
    pub fn get_for_stage(stage: &clutter::Stage) -> Option<Stage> {
        //    unsafe { TODO: call ffi:window_get_for_stage() }
        unimplemented!()
    }

    pub fn test_check(&self) -> String {
        "HERE".into()
    }
}

impl Default for Stage {
    fn default() -> Self {
        Self {
            inner: clutter::Stage::new(),
            has_toolbar: false,
            small_screen: false,
            fullscreen: false,
            rotate_size: false,
            icon_name: "".into(),
            stage: None,
            toolbar: None,
            child: None,
            resize_grip: None,
            debug_actor: None,
            rotation: WindowRotation::Rotation0,
            rotation_timeline: None,
            start_angle: 0.0,
            end_angle: 0.0,
            angle: 0.0,
        }
    }
}

impl Object for Stage {}
impl Is<Stage> for Stage {}

impl AsRef<Stage> for Stage {
    fn as_ref(&self) -> &Stage {
        self
    }
}

pub const NONE_WINDOW: Option<&Stage> = None;

pub trait WindowExt: 'static {
    /// get_child:
    /// @window: A #Stage
    ///
    /// Get the primary child of the window. See set_child().
    ///
    /// Returns: (transfer none): A #ClutterActor, or %NULL
    ///
    fn get_child(&self) -> Option<clutter::Actor>;

    /// get_clutter_stage:
    /// @window: A #Stage
    ///
    /// Gets the #ClutterStage managed by the window.
    ///
    /// Returns: (transfer none): A #ClutterStage
    ///
    fn get_clutter_stage(&self) -> Option<&clutter::Stage>;

    /// get_fullscreen:
    /// @window: A #Stage
    ///
    /// Determines if the window has been set to be in fullscreen mode.
    ///
    /// Returns: %true if the window has been set to be in fullscreen mode,
    ///   otherwise %false
    ///
    fn get_fullscreen(&self) -> bool;

    /// get_has_toolbar:
    /// @window: A #Stage
    ///
    /// Determines whether the window has a toolbar or not.
    /// See set_has_toolbar().
    ///
    /// Returns: %true if the window has a toolbar, otherwise %false
    ///
    fn get_has_toolbar(&self) -> bool;

    /// get_icon_name:
    /// @window: A #Stage
    ///
    /// Gets the currently set window icon name. This will be %NULL if there is none
    /// set, or the icon was set with set_icon_from_cogl_texture().
    ///
    /// Returns: The window icon name, or %NULL
    ///
    fn get_icon_name(&self) -> Option<String>;

    fn get_resisable(&self) -> bool;

    /// get_small_screen:
    /// @window: A #Stage
    ///
    /// Determines if the window is in small-screen mode.
    /// See set_small_screen().
    ///
    /// Returns: %true if the window is in small-screen mode, otherwise %false
    ///
    fn get_small_screen(&self) -> bool;

    /// get_title:
    /// @window: A #Stage
    ///
    /// Retrieves the title used for the window.
    ///
    /// Returns: The title used for the window
    ///
    fn get_title(&self) -> Option<String>;

    /// get_toolbar:
    /// @window: A #Stage
    ///
    /// Retrieves the toolbar associated with the window.
    ///
    /// Returns: (transfer none): A #Toolbar
    ///
    fn get_toolbar(&self) -> Option<Toolbar>;

    /// get_window_position:
    /// @window: an #Stage
    /// @x: (out) (allow-none): A pointer for the x-coordinate
    /// @y: (out) (allow-none): A pointer for the y-coordinate
    ///
    /// Retrieves the absolute position of the window on the screen.
    ///
    fn get_window_position(&self) -> (i32, i32);

    /// get_window_rotation:
    /// @window: A #Stage
    ///
    /// Retrieve the rotation of the window.
    ///
    /// Returns: An #StageRotation
    ///
    fn get_window_rotation(&self) -> WindowRotation;

    /// get_window_size:
    /// @window: A #Stage
    /// @width: (out) (allow-none): A #gint pointer for the window's width
    /// @height: (out) (allow-none): A #gint pointer for the window's height
    ///
    /// Retrieves the size of the display area of the window, taking into
    /// account any window border. This includes the area occupied by the
    /// window's toolbar, if it's enabled.
    ///
    fn get_window_size(&self) -> (i32, i32);

    /// hide:
    /// @window: A #Stage
    ///
    /// Hide the window
    ///
    fn hide(&self) -> &Self;

    /// present:
    /// @window: A #Stage
    ///
    /// Present the window. The actual behaviour is specific to the window system.
    ///
    fn present(&self);

    /// set_child:
    /// @window: A #Stage
    /// @actor: A #ClutterActor
    ///
    /// Adds @actor to the window and sets it as the primary child. When the
    /// stage managed in the window changes size, the child will be resized
    /// to match it.
    ///
    fn set_child<P: Is<clutter::Actor>>(&self, actor: &P);

    /// set_fullscreen:
    /// @window: A #Stage
    /// @fullscreen: %true to request fullscreen mode, %false to disable
    ///
    /// Set the window to be in fullscreen mode or windowed mode.
    ///
    /// <note><para>
    /// Setting fullscreen mode doesn't necessarily mean the window is actually
    /// fullscreen. Setting this property is only a request to the underlying
    /// window system.
    /// </para></note>
    ///
    fn set_fullscreen(&self, fullscreen: bool) -> &Self;

    /// set_has_toolbar:
    /// @window: A #Stage
    /// @toolbar: %true if the toolbar should be displayed
    ///
    /// Sets whether the window has a toolbar or not. If the window has a toolbar,
    /// client-side window decorations will be enabled.
    ///
    fn set_has_toolbar(&self, toolbar: bool);

    //fn set_icon_from_cogl_texture(&self, texture: cogl::Handle);

    /// set_icon_name:
    /// @window: A #Stage
    /// @icon_name: (allow-none): An icon name, or %NULL
    ///
    /// Set an icon-name to use for the window icon. The icon will be looked up
    /// from the default theme.
    ///
    fn set_icon_name(&self, icon_name: Option<&str>);

    fn set_resizable(&self, resizable: bool) -> &Self;

    /// get_small_screen:
    /// @window: A #Stage
    ///
    /// Determines if the window is in small-screen mode.
    /// See set_small_screen().
    ///
    /// Returns: %true if the window is in small-screen mode, otherwise %false
    ///
    fn set_small_screen(&self, small_screen: bool);

    /// set_title:
    /// @window: A #Stage
    /// @title: A string to use for the window title name
    ///
    /// Sets the title used for the window, the results of which are
    /// window-system specific.
    ///
    fn set_title(&self, title: &str) -> &Self;

    /// set_toolbar:
    /// @window: (allow-none): A #Stage
    ///
    /// Sets the toolbar associated with the window.
    ///
    fn set_toolbar<P: Is<Toolbar>>(&self, toolbar: &P);

    /// set_window_position:
    /// @window: A #Stage
    /// @x: An x-coordinate
    /// @y: A y-coordinate
    ///
    /// Sets the absolute position of the window on the screen.
    ///
    fn set_window_position(&self, x: i32, y: i32);

    /// set_window_rotation:
    /// @window: A #Stage
    /// @rotation: The #StageRotation
    ///
    /// Set the rotation of the window.
    ///
    fn set_window_rotation(&self, rotation: WindowRotation);

    /// set_window_size:
    /// @window: A #Stage
    /// @width: A width, in pixels
    /// @height: A height, in pixels
    ///
    /// Sets the size of the window, taking into account any window border. This
    /// corresponds to the window's available area for its child, minus the area
    /// occupied by the window's toolbar, if it's enabled.
    ///
    /// <note><para>
    /// Setting the window size may involve a request to the underlying windowing
    /// system, and may not immediately be reflected.
    /// </para></note>
    ///
    fn set_window_size(&self, width: i32, height: i32) -> &Self;

    /// show:
    /// @window: A #Stage
    ///
    /// Show the window
    ///
    fn show(&self) -> &Self;

    fn get_property_icon_cogl_texture(&self) -> Option<String>;

    fn set_property_icon_cogl_texture(&self, icon_cogl_texture: Option<&str>);

    fn get_property_window_rotation_angle(&self) -> f32;

    fn get_property_window_rotation_timeline(&self) -> Option<clutter::Timeline>;

    fn connect_destroy<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_fullscreen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_has_toolbar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_cogl_texture_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_small_screen_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_toolbar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_window_rotation_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_window_rotation_angle_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_window_rotation_timeline_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: Is<Stage>> WindowExt for O {
    /// get_child:
    /// @window: A #Stage
    ///
    /// Get the primary child of the window. See set_child().
    ///
    /// Returns: (transfer none): A #ClutterActor, or %NULL
    ///
    fn get_child(&self) -> Option<clutter::Actor> {
        // unsafe { from_glib_none(ffi::window_get_child(self.as_ref().to_glib_none().0)) }
        let stage = self.as_ref();
        stage.child.clone()
    }

    /// get_clutter_stage:
    /// @window: A #Stage
    ///
    /// Gets the #ClutterStage managed by the window.
    ///
    /// Returns: (transfer none): A #ClutterStage
    ///
    fn get_clutter_stage(&self) -> Option<&clutter::Stage> {
        let stage = self.as_ref();
        Some(&stage.inner)
    }

    /// get_fullscreen:
    /// @window: A #Stage
    ///
    /// Determines if the window has been set to be in fullscreen mode.
    ///
    /// Returns: %true if the window has been set to be in fullscreen mode,
    ///   otherwise %false
    ///
    fn get_fullscreen(&self) -> bool {
        let stage = self.as_ref();
        stage.inner.get_fullscreen()
    }

    /// get_has_toolbar:
    /// @window: A #Stage
    ///
    /// Determines whether the window has a toolbar or not.
    /// See set_has_toolbar().
    ///
    /// Returns: %true if the window has a toolbar, otherwise %false
    ///
    fn get_has_toolbar(&self) -> bool {
        let stage = self.as_ref();
        stage.has_toolbar
    }

    /// get_icon_name:
    /// @window: A #Stage
    ///
    /// Gets the currently set window icon name. This will be %NULL if there is none
    /// set, or the icon was set with set_icon_from_cogl_texture().
    ///
    /// Returns: The window icon name, or %NULL
    ///
    fn get_icon_name(&self) -> Option<String> {
        let stage = self.as_ref();
        Some(stage.icon_name.clone())
    }

    fn get_resisable(&self) -> bool {
        let stage = self.as_ref();
        stage.inner.get_user_resizable()
    }

    /// get_small_screen:
    /// @window: A #Stage
    ///
    /// Determines if the window is in small-screen mode.
    /// See set_small_screen().
    ///
    /// Returns: %true if the window is in small-screen mode, otherwise %false
    ///
    fn get_small_screen(&self) -> bool {
        let stage = self.as_ref();
        stage.small_screen
    }

    /// get_title:
    /// @window: A #Stage
    ///
    /// Retrieves the title used for the window.
    ///
    /// Returns: The title used for the window
    ///
    fn get_title(&self) -> Option<String> {
        let stage = self.as_ref();
        match stage.inner.get_title() {
            Some(title) => Some(title.as_str().into()),
            None => None,
        }
    }

    /// get_toolbar:
    /// @window: A #Stage
    ///
    /// Retrieves the toolbar associated with the window.
    ///
    /// Returns: (transfer none): A #Toolbar
    ///
    fn get_toolbar(&self) -> Option<Toolbar> {
        let stage = self.as_ref();
        stage.toolbar.clone()
    }

    /// get_window_position:
    /// @window: an #Stage
    /// @x: (out) (allow-none): A pointer for the x-coordinate
    /// @y: (out) (allow-none): A pointer for the y-coordinate
    ///
    /// Retrieves the absolute position of the window on the screen.
    ///
    fn get_window_position(&self) -> (i32, i32) {
        let stage = self.as_ref();

        // if stage.native_window {
        //     return stage.native_window.get_position();
        // }

        (0, 0)
    }

    /// get_window_rotation:
    /// @window: A #Stage
    ///
    /// Retrieve the rotation of the window.
    ///
    /// Returns: An #StageRotation
    ///
    fn get_window_rotation(&self) -> WindowRotation {
        let stage = self.as_ref();
        stage.rotation
    }

    /// get_window_size:
    /// @window: A #Stage
    /// @width: (out) (allow-none): A #gint pointer for the window's width
    /// @height: (out) (allow-none): A #gint pointer for the window's height
    ///
    /// Retrieves the size of the display area of the window, taking into
    /// account any window border. This includes the area occupied by the
    /// window's toolbar, if it's enabled.
    ///
    fn get_window_size(&self) -> (i32, i32) {
        let stage = self.as_ref();
        let (width, height) = stage.inner.get_size();
        (width as i32, height as i32)
    }

    /// hide:
    /// @window: A #Stage
    ///
    /// Hide the window
    ///
    fn hide(&self) -> &Self {
        let stage = self.as_ref();
        stage.inner.hide();
        self
    }

    /// present:
    /// @window: A #Stage
    ///
    /// Present the window. The actual behaviour is specific to the window system.
    ///
    fn present(&self) {
        let stage = self.as_ref();

        // if stage.native_window {
        //     stage.native_window.present();
        // }
    }

    /// set_child:
    /// @window: A #Stage
    /// @actor: A #ClutterActor
    ///
    /// Adds @actor to the window and sets it as the primary child. When the
    /// stage managed in the window changes size, the child will be resized
    /// to match it.
    ///
    fn set_child<P: Is<clutter::Actor>>(&self, actor: &P) {
        let stage = self.as_ref();
        let actor = actor.as_ref();

        // if !stage.stage {
        //     return;
        // }

        // if stage.child == actor {
        //     return;
        // }

        // if stage.child {
        //     clutter_actor_remove_child(stage.stage, stage.child);
        // }

        // if actor {
        //     stage.child = actor;
        //     clutter_actor_add_child(stage.stage, stage.child);
        // }

        // window_reallocate(window);
        // g_object_notify(G_OBJECT(window), "child");
    }

    /// set_fullscreen:
    /// @window: A #Stage
    /// @fullscreen: %true to request fullscreen mode, %false to disable
    ///
    /// Set the window to be in fullscreen mode or windowed mode.
    ///
    /// <note><para>
    /// Setting fullscreen mode doesn't necessarily mean the window is actually
    /// fullscreen. Setting this property is only a request to the underlying
    /// window system.
    /// </para></note>
    ///
    fn set_fullscreen(&self, fullscreen: bool) -> &Self {
        let stage = self.as_ref();
        stage.inner.set_fullscreen(fullscreen);
        self
    }

    /// set_has_toolbar:
    /// @window: A #Stage
    /// @toolbar: %true if the toolbar should be displayed
    ///
    /// Sets whether the window has a toolbar or not. If the window has a toolbar,
    /// client-side window decorations will be enabled.
    ///
    fn set_has_toolbar(&self, toolbar: bool) {
        let stage = self.as_ref();
        
        if stage.has_toolbar != toolbar {
            // stage.has_toolbar = toolbar;

            // if !toolbar {
            //     clutter_actor_hide(stage.toolbar);
            //     clutter_actor_hide(stage.resize_grip);
            // } else {
            //     clutter_actor_show(stage.toolbar);
            //     if clutter_stage_get_user_resizable((ClutterStage *)stage.stage) {
            //         clutter_actor_show(stage.resize_grip);
            //     }
            // }

            // g_object_notify(G_OBJECT(window), "has-toolbar");
            // window_reallocate(window);
        }
    }

    //fn set_icon_from_cogl_texture(&self, texture: /*Ignored*/cogl::Handle) {
    //    unsafe { TODO: call ffi:window_set_icon_from_cogl_texture() }
    //}

    /// set_icon_name:
    /// @window: A #Stage
    /// @icon_name: (allow-none): An icon name, or %NULL
    ///
    /// Set an icon-name to use for the window icon. The icon will be looked up
    /// from the default theme.
    ///
    fn set_icon_name(&self, icon_name: Option<&str>) {
        let stage = self.as_ref();
        
        // if stage.icon_name && icon_name && g_str_equal(stage.icon_name, icon_name)) {
        //     return;
        // }
        
        // if !stage.icon_name && !icon_name {
        //     return;
        // }

        // g_free(stage.icon_name);
        // stage.icon_name = g_strdup(icon_name);

        // g_object_notify(G_OBJECT(window), "icon-name");
    }

    fn set_resizable(&self, resizable: bool) -> &Self {
        let stage = self.as_ref();
        stage.inner.set_user_resizable(resizable);
        self
    }

    /// get_small_screen:
    /// @window: A #Stage
    ///
    /// Determines if the window is in small-screen mode.
    /// See set_small_screen().
    ///
    /// Returns: %true if the window is in small-screen mode, otherwise %false
    ///
    fn set_small_screen(&self, small_screen: bool) {
        let stage = self.as_ref();

        if stage.small_screen != small_screen {
            // stage.small_screen = small_screen;
            // g_object_notify(G_OBJECT(window), "small-screen");
        }
    }

    /// set_title:
    /// @window: A #Stage
    /// @title: A string to use for the window title name
    ///
    /// Sets the title used for the window, the results of which are
    /// window-system specific.
    ///
    fn set_title(&self, title: &str) -> &Self {
        let stage = self.as_ref();
        stage.inner.set_title(title);
        self
    }

    /// set_toolbar:
    /// @window: (allow-none): A #Stage
    ///
    /// Sets the toolbar associated with the window.
    ///
    fn set_toolbar<P: Is<Toolbar>>(&self, toolbar: &P) {
        let stage = self.as_ref();
        let toolbar = toolbar.as_ref();
        
        // if stage.toolbar == (ClutterActor *)toolbar {
        //     return;
        // }

        // // Remove old toolbar 
        // if stage.toolbar {
        //     g_signal_handlers_disconnect_by_func(stage.toolbar,
        //                                             mx_window_allocation_changed_cb,
        //                                             window);
        //     g_object_remove_weak_pointer(G_OBJECT(stage.toolbar), (gpointer *)&stage.toolbar);
        //     clutter_actor_remove_child(stage.stage, stage.toolbar);
        // }

        // stage.toolbar = (ClutterActor *)toolbar;

        // // Add new toolbar
        // if (toolbar) {
        //     clutter_actor_add_child(stage.stage, stage.toolbar);
        //     g_object_add_weak_pointer(G_OBJECT (stage.toolbar), (gpointer *)&stage.toolbar);
        //     g_signal_connect(stage.toolbar, "allocation-changed",
        //                         G_CALLBACK(mx_window_allocation_changed_cb), window);
        // }

        // stage.has_toolbar = stage.toolbar ? true : false;
    }

    /// set_window_position:
    /// @window: A #Stage
    /// @x: An x-coordinate
    /// @y: A y-coordinate
    ///
    /// Sets the absolute position of the window on the screen.
    ///
    fn set_window_position(&self, x: i32, y: i32) {
        let stage = self.as_ref();

        // if stage.native_window {
        //     stage.native_window.set_position(x, y);
        // }
    }

    /// set_window_rotation:
    /// @window: A #Stage
    /// @rotation: The #StageRotation
    ///
    /// Set the rotation of the window.
    ///
    fn set_window_rotation(&self, rotation: WindowRotation) {
        let stage = self.as_ref();
        
        if stage.rotation == rotation {
            return;
        }

        // if ((stage.rotation == WindowRotation::Rotation0) ||
        //     (stage.rotation == WindowRotation::Rotation180)) &&
        //     ((rotation == WindowRotation::Rotation90) ||
        //     (rotation == WindowRotation::Rotation270)) {
        //     stage.rotate_size = true;
        // } else if ((stage.rotation == WindowRotation::Rotation90) ||
        //             (stage.rotation == WindowRotation::Rotation270)) &&
        //         ((rotation == WindowRotation::Rotation0) ||
        //             (rotation == WindowRotation::Rotation180)) {
        //             stage.rotate_size = true;
        // }

        // stage.rotation = rotation;

        // stage.start_angle = stage.angle;
        // match rotation {
        //     WindowRotation::Rotation0 => {
        //         stage.end_angle = 0.0;
        //     }
        //     WindowRotation::Rotation90 => {
        //         stage.end_angle = 90.0;
        //     }
        //     WindowRotation::Rotation180 => {
        //         stage.end_angle = 180.0;
        //     }
        //     WindowRotation::Rotation270 => {
        //         stage.end_angle = 270.0;
        //     }
        // }

        // if stage.end_angle - stage.start_angle > 180.0 {
        //     stage.end_angle -= 360.0;
        // } else if stage.end_angle - stage.start_angle < -180.0 {
        //     stage.end_angle += 360.0;
        // }

        // let msecs = (guint)((ABS(stage.end_angle - stage.start_angle) / 90.0) * 400.0);
        // clutter_timeline_rewind(stage.rotation_timeline);
        // clutter_timeline_set_duration(stage.rotation_timeline, msecs);
        // clutter_timeline_start(stage.rotation_timeline);

        // g_object_notify(G_OBJECT(window), "window-rotation");
    }

    /// set_window_size:
    /// @window: A #Stage
    /// @width: A width, in pixels
    /// @height: A height, in pixels
    ///
    /// Sets the size of the window, taking into account any window border. This
    /// corresponds to the window's available area for its child, minus the area
    /// occupied by the window's toolbar, if it's enabled.
    ///
    /// <note><para>
    /// Setting the window size may involve a request to the underlying windowing
    /// system, and may not immediately be reflected.
    /// </para></note>
    ///
    fn set_window_size(&self, width: i32, height: i32) -> &Self {
        let stage = self.as_ref();
        stage.inner.set_size(width as f32, height as f32);
        self
    }

    /// show:
    /// @window: A #Stage
    ///
    /// Show the window
    ///
    fn show(&self) -> &Self {
        let stage = self.as_ref();
        stage.inner.show();
        self
    }

    fn get_property_icon_cogl_texture(&self) -> Option<String> {
        // unsafe {
        //     let mut value = Value::from_type(<String as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"icon-cogl-texture\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `icon-cogl-texture` getter")
        // }
        unimplemented!()
    }

    fn set_property_icon_cogl_texture(&self, icon_cogl_texture: Option<&str>) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"icon-cogl-texture\0".as_ptr() as *const _,
        //         Value::from(icon_cogl_texture).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_window_rotation_angle(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"window-rotation-angle\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `window-rotation-angle` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn get_property_window_rotation_timeline(&self) -> Option<clutter::Timeline> {
        // unsafe {
        //     let mut value = Value::from_type(<clutter::Timeline as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"window-rotation-timeline\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `window-rotation-timeline` getter")
        // }
        unimplemented!()
    }

    // unsafe fn unsafe_cast_ref<T: ObjectType>(&self) -> &T {
    //     debug_assert!(self.is::<T>());
    //     // This cast is safe because all our wrapper types have the
    //     // same representation except for the name and the phantom data
    //     // type. IsA<> is an unsafe trait that must only be implemented
    //     // if this is a valid wrapper type
    //     &*(self as *const Self as *const T)
    // }

    // TODO: &Self
    fn connect_destroy<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        let stage = self.as_ref();
        let this = unsafe { &*(stage as *const Stage as *const Self) };

        stage.inner.connect_destroy(move |_| {
            f(this);
        })
    }

    fn connect_property_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_child_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::child\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_child_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_fullscreen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_fullscreen_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::fullscreen\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_fullscreen_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_has_toolbar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_has_toolbar_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::has-toolbar\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_has_toolbar_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_icon_cogl_texture_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_icon_cogl_texture_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::icon-cogl-texture\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_icon_cogl_texture_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::icon-name\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_icon_name_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_small_screen_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_small_screen_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::small-screen\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_small_screen_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_title_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::title\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_title_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_toolbar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        // unsafe extern "C" fn notify_toolbar_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::toolbar\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_toolbar_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_window_rotation_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_window_rotation_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::window-rotation\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_window_rotation_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_window_rotation_angle_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_window_rotation_angle_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::window-rotation-angle\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_window_rotation_angle_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }

    fn connect_property_window_rotation_timeline_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        // unsafe extern "C" fn notify_window_rotation_timeline_trampoline<P, F: Fn(&P) + 'static>(
        //     this: *mut ffi::Window,
        //     _param_spec: glib_sys::gpointer,
        //     f: glib_sys::gpointer,
        // ) where
        //     P: Is<Window>,
        // {
        //     let f: &F = &*(f as *const F);
        //     f(&Window::from_glib_borrow(this).unsafe_cast_ref())
        // }
        // unsafe {
        //     let f: Box_<F> = Box_::new(f);
        //     connect_raw(
        //         self.as_ptr() as *mut _,
        //         b"notify::window-rotation-timeline\0".as_ptr() as *const _,
        //         Some(transmute::<_, unsafe extern "C" fn()>(
        //             notify_window_rotation_timeline_trampoline::<Self, F> as *const (),
        //         )),
        //         Box_::into_raw(f),
        //     )
        // }
        unimplemented!()
    }
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Window")
    }
}
