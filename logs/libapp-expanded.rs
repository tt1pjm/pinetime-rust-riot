#![feature(prelude_import)]
/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
//!  Main Rust Application for PineTime with Apache Mynewt OS
#![no_std]
//  Don't link with standard Rust library, which is not compatible with embedded systems
#![feature(trace_macros)]
//  Allow macro tracing: `trace_macros!(true)`
#![feature(concat_idents)]
//  Allow `concat_idents!()` macro used in `coap!()` macro
#![feature(const_transmute)]
//  Allow `transmute` for initialising Mynewt structs
#![feature(proc_macro_hygiene)]
//  Allow Procedural Macros like `run!()`
#![feature(specialization)]
//  Allow Specialised Traits for druid UI library
#![feature(exclusive_range_pattern)]
#[prelude_import]
use core::prelude::v1::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;
//  Allow ranges like `0..128` in `match` statements

//  Declare the libraries that contain macros
extern crate cortex_m;
//  Declare the external library `cortex_m`
extern crate lvgl;
//  Declare the LittlevGL (LVGL) library
extern crate macros as lvgl_macros;
//  Declare the LVGL Procedural Macros library

//  Declare the modules in our application
mod screen_time {
    //  Declare `screen_time.rs` as Rust module `screen_time` for Watch Face

    //  Declare the system modules
    //  Import `PanicInfo` type which is used by `panic()` below
    //  Import cortex_m assembly function to inject breakpoint
    //  Import Semihosting Console functions

    //  Don't mangle the function name
    //  Declare extern "C" because it will be called by RIOT OS firmware

    //  Display the filename and line number to the Semihosting Console.
    //  Pause in the debugger.
    //  Display the payload.
    //  Loop forever so that device won't restart.
    //! Watch Face in Rust
    use core::{fmt::Write, ptr};
    use lvgl::{result::*, core::{obj}, objx::{label}, Strn, fill_zero};
    use lvgl_macros::{strn};
    /// Style for the Time Label
    static mut style_time: obj::lv_style_t =
        unsafe {
            ::core::mem::transmute::<[u8; ::core::mem::size_of::<obj::lv_style_t>()],
                                     obj::lv_style_t>([0;
                                                          ::core::mem::size_of::<obj::lv_style_t>()])
        };
    /// Create the Time Screen, populated with widgets. Called by screen_time_create() below.
    fn create_screen(widgets: &mut WatchFaceWidgets) -> LvglResult<()> {
        let scr = widgets.screen;
        if !!scr.is_null() { ::core::panicking::panic("null screen") };
        let label1 = label::create(scr, ptr::null())?;
        label::set_long_mode(label1, label::LV_LABEL_LONG_BREAK);
        label::set_text(label1, &Strn::new(b"00:00\x00"));
        obj::set_width(label1, 240);
        obj::set_height(label1, 200);
        label::set_align(label1, label::LV_LABEL_ALIGN_CENTER);
        obj::align(label1, scr, obj::LV_ALIGN_CENTER, 0, -30);
        label::set_style(label1, label::LV_LABEL_STYLE_MAIN, &style_time);
        widgets.time_label = label1;
        let l_state = label::create(scr, ptr::null())?;
        obj::set_width(l_state, 50);
        obj::set_height(l_state, 80);
        label::set_text(l_state, &Strn::new(b"\x00"));
        label::set_recolor(l_state, true);
        label::set_align(l_state, label::LV_LABEL_ALIGN_LEFT);
        obj::align(l_state, scr, obj::LV_ALIGN_IN_TOP_LEFT, 0, 0);
        widgets.ble_label = l_state;
        let l_power = label::create(scr, ptr::null())?;
        obj::set_width(l_power, 80);
        obj::set_height(l_power, 20);
        label::set_text(l_power, &Strn::new(b"\x00"));
        label::set_recolor(l_power, true);
        label::set_align(l_power, label::LV_LABEL_ALIGN_RIGHT);
        obj::align(l_power, scr, obj::LV_ALIGN_IN_TOP_RIGHT, 0, 0);
        widgets.power_label = l_power;
        let label_date = label::create(scr, ptr::null())?;
        label::set_long_mode(label_date, label::LV_LABEL_LONG_BREAK);
        obj::set_width(label_date, 200);
        obj::set_height(label_date, 200);
        label::set_align(label_date, label::LV_LABEL_ALIGN_CENTER);
        obj::align(label_date, scr, obj::LV_ALIGN_CENTER, 0, 40);
        widgets.date_label = label_date;
        obj::set_click(scr, true);
        Ok(())
    }
    /// Populate the screen with the current state. Called by screen_time_update_screen() below.
    fn update_screen(widgets: &WatchFaceWidgets, state: &WatchFaceState)
     -> LvglResult<()> {
        set_time_label(widgets, state)?;
        set_bt_label(widgets, state)?;
        set_power_label(widgets, state)?;
        Ok(())
    }
    /// Populate the Bluetooth Label with the Bluetooth status. Called by screen_time_update_screen() above.
    fn set_bt_label(widgets: &WatchFaceWidgets, state: &WatchFaceState)
     -> LvglResult<()> {
        if state.ble_state == BleState::BLEMAN_BLE_STATE_DISCONNECTED {
            label::set_text(widgets.ble_label, &Strn::new(b"\x00"));
        } else {
            let color =
                match state.ble_state {
                    BLEMAN_BLE_STATE_INACTIVE => "#000000",
                    BLEMAN_BLE_STATE_DISCONNECTED => "#f2495c",
                    BLEMAN_BLE_STATE_ADVERTISING => "#5794f2",
                    BLEMAN_BLE_STATE_CONNECTED => "#37872d",
                };
            let mut status = heapless::String::<heapless::consts::U16>::new();
            (&mut status).write_fmt(::core::fmt::Arguments::new_v1(&["",
                                                                     " \u{f293}#\u{0}"],
                                                                   &match (&color,)
                                                                        {
                                                                        (arg0,)
                                                                        =>
                                                                        [::core::fmt::ArgumentV1::new(arg0,
                                                                                                      ::core::fmt::Display::fmt)],
                                                                    })).expect("bt fail");
            label::set_text(widgets.ble_label, &Strn::new(status.as_bytes()));
        }
        Ok(())
    }
    /// Populate the Power Label with the battery status. Called by screen_time_update_screen() above.
    fn set_power_label(widgets: &WatchFaceWidgets, state: &WatchFaceState)
     -> LvglResult<()> {
        let percentage = hal_battery_get_percentage(state.millivolts);
        let color =
            if percentage <= 20 {
                "#f2495c"
            } else if state.powered && !(state.charging) {
                "#73bf69"
            } else { "#fade2a" };
        let symbol = if state.powered { "\u{F0E7}" } else { " " };
        let mut status = heapless::String::<heapless::consts::U16>::new();
        (&mut status).write_fmt(::core::fmt::Arguments::new_v1(&["", " ", "%",
                                                                 "#\n(",
                                                                 "mV)\u{0}"],
                                                               &match (&color,
                                                                       &percentage,
                                                                       &symbol,
                                                                       &state.millivolts)
                                                                    {
                                                                    (arg0,
                                                                     arg1,
                                                                     arg2,
                                                                     arg3) =>
                                                                    [::core::fmt::ArgumentV1::new(arg0,
                                                                                                  ::core::fmt::Display::fmt),
                                                                     ::core::fmt::ArgumentV1::new(arg1,
                                                                                                  ::core::fmt::Display::fmt),
                                                                     ::core::fmt::ArgumentV1::new(arg2,
                                                                                                  ::core::fmt::Display::fmt),
                                                                     ::core::fmt::ArgumentV1::new(arg3,
                                                                                                  ::core::fmt::Display::fmt)],
                                                                })).expect("batt fail");
        label::set_text(widgets.power_label, &Strn::new(status.as_bytes()));
        obj::align(widgets.power_label, widgets.screen,
                   obj::LV_ALIGN_IN_TOP_RIGHT, 0, 0);
        Ok(())
    }
    /// Populate the Time and Date Labels with the time and date. Called by screen_time_update_screen() above.
    fn set_time_label(widgets: &WatchFaceWidgets, state: &WatchFaceState)
     -> LvglResult<()> {
        let mut time = heapless::String::<heapless::consts::U6>::new();
        (&mut time).write_fmt(::core::fmt::Arguments::new_v1_formatted(&["",
                                                                         ":",
                                                                         "\u{0}"],
                                                                       &match (&state.time.hour,
                                                                               &state.time.minute)
                                                                            {
                                                                            (arg0,
                                                                             arg1)
                                                                            =>
                                                                            [::core::fmt::ArgumentV1::new(arg0,
                                                                                                          ::core::fmt::Display::fmt),
                                                                             ::core::fmt::ArgumentV1::new(arg1,
                                                                                                          ::core::fmt::Display::fmt)],
                                                                        },
                                                                       &[::core::fmt::rt::v1::Argument{position:
                                                                                                           0usize,
                                                                                                       format:
                                                                                                           ::core::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                               ' ',
                                                                                                                                           align:
                                                                                                                                               ::core::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                           flags:
                                                                                                                                               8u32,
                                                                                                                                           precision:
                                                                                                                                               ::core::fmt::rt::v1::Count::Implied,
                                                                                                                                           width:
                                                                                                                                               ::core::fmt::rt::v1::Count::Is(2usize),},},
                                                                         ::core::fmt::rt::v1::Argument{position:
                                                                                                           1usize,
                                                                                                       format:
                                                                                                           ::core::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                               ' ',
                                                                                                                                           align:
                                                                                                                                               ::core::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                           flags:
                                                                                                                                               8u32,
                                                                                                                                           precision:
                                                                                                                                               ::core::fmt::rt::v1::Count::Implied,
                                                                                                                                           width:
                                                                                                                                               ::core::fmt::rt::v1::Count::Is(2usize),},}])).expect("time fail");
        label::set_text(widgets.time_label, &Strn::new(time.as_bytes()));
        let month_cstr = controller_time_month_get_short_name(&state.time);
        let month_str =
            cstr_core::CStr::from_ptr(month_cstr).to_str().expect("month fail");
        let mut date = heapless::String::<heapless::consts::U15>::new();
        (&mut date).write_fmt(::core::fmt::Arguments::new_v1(&["", " ", " ",
                                                               "\n\u{0}"],
                                                             &match (&state.time.dayofmonth,
                                                                     &month_str,
                                                                     &state.time.year)
                                                                  {
                                                                  (arg0, arg1,
                                                                   arg2) =>
                                                                  [::core::fmt::ArgumentV1::new(arg0,
                                                                                                ::core::fmt::Display::fmt),
                                                                   ::core::fmt::ArgumentV1::new(arg1,
                                                                                                ::core::fmt::Display::fmt),
                                                                   ::core::fmt::ArgumentV1::new(arg2,
                                                                                                ::core::fmt::Display::fmt)],
                                                              })).expect("date fail");
        label::set_text(widgets.date_label, &Strn::new(date.as_bytes()));
        Ok(())
    }
    /// Create the Time Screen, populated with widgets. Called by home_time_draw() in screen_time.c.
    #[no_mangle]
    extern "C" fn screen_time_create(widget: *const home_time_widget_t)
     -> *mut obj::lv_obj_t {
        let screen =
            obj::create(ptr::null_mut(),
                        ptr::null()).expect("create screen obj fail");
        (*widget).screen = screen;
        let mut subwidgets = &(*widget).subwidgets;
        subwidgets.screen = screen;
        create_screen(subwidgets).expect("create_screen fail");
        obj::set_event_cb(screen, Some(screen_time_pressed));
        let state = &(*widget).state;
        update_screen(subwidgets, state).expect("update_screen fail");
        screen
    }
    /// Populate the Time Screen with the current status. Called by home_time_update_screen() in screen_time.c and by screen_time_create() above.
    #[no_mangle]
    extern "C" fn screen_time_update_screen(widget0: *const widget_t) -> i32 {
        let widget: *const home_time_widget_t = from_widget(widget0);
        let subwidgets = &(*widget).subwidgets;
        let state = &(*widget).state;
        update_screen(subwidgets, state).expect("update_screen fail");
        0
    }
    /// LVGL Widget for Watch Face. TODO: Sync with widgets/home_time/include/home_time.h
    #[repr(C)]
    struct home_time_widget_t {
        widget: widget_t,
        handler: control_event_handler_t,
        screen: *mut obj::lv_obj_t,
        state: WatchFaceState,
        subwidgets: WatchFaceWidgets,
    }
    /// State for the Watch Face, shared between GUI and control. TODO: Sync with widgets/home_time/include/home_time.h
    #[repr(C)]
    struct WatchFaceState {
        ble_state: BleState,
        time: controller_time_spec_t,
        millivolts: u32,
        charging: bool,
        powered: bool,
    }
    /// Widgets for the Watch Face, private to Rust. TODO: Sync with widgets/home_time/include/home_time.h
    #[repr(C)]
    #[allow(non_camel_case_types)]
    struct WatchFaceWidgets {
        screen: *mut obj::lv_obj_t,
        time_label: *mut obj::lv_obj_t,
        date_label: *mut obj::lv_obj_t,
        ble_label: *mut obj::lv_obj_t,
        power_label: *mut obj::lv_obj_t,
    }
    #[repr(i32)]
    #[allow(non_camel_case_types)]
    enum BleState {
        BLEMAN_BLE_STATE_INACTIVE = 0,
        BLEMAN_BLE_STATE_ADVERTISING = 1,
        BLEMAN_BLE_STATE_DISCONNECTED = 2,
        BLEMAN_BLE_STATE_CONNECTED = 3,
    }
    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralPartialEq for BleState { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::PartialEq for BleState {
        #[inline]
        fn eq(&self, other: &BleState) -> bool {
            {
                let __self_vi =
                    unsafe { ::core::intrinsics::discriminant_value(&*self) }
                        as i32;
                let __arg_1_vi =
                    unsafe { ::core::intrinsics::discriminant_value(&*other) }
                        as i32;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) { _ => true, }
                } else { false }
            }
        }
    }
    #[repr(C)]
    #[allow(non_camel_case_types)]
    struct controller_time_spec_t {
        year: u16,
        month: u8,
        dayofmonth: u8,
        hour: u8,
        minute: u8,
        second: u8,
        fracs: u8,
    }
    /// Import C APIs
    extern {
        fn hal_battery_get_percentage(voltage: u32)
        -> i32;
        fn controller_time_month_get_short_name(time:
                                                    *const controller_time_spec_t)
        -> *const ::cty::c_char;
        fn from_widget(widget: *const widget_t)
        -> *const home_time_widget_t;
        fn screen_time_pressed(obj: *mut obj::lv_obj_t,
                               event: obj::lv_event_t);
    }
    #[allow(non_camel_case_types)]
    struct widget_t {
    }
    #[allow(non_camel_case_types)]
    struct control_event_handler_t {
    }
}
use core::panic::PanicInfo;
use cortex_m::asm::bkpt;
use lvgl::console;
///  Main program that initialises the sensor, network driver and starts reading and sending sensor data in the background.
///  Will be called at startup.
#[no_mangle]
extern "C" fn rust_main() { }
///  This function is called on panic, like an assertion failure. We display the filename and line number and pause in the debugger. From https://os.phil-opp.com/freestanding-rust-binary/
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    console::print("panic ");
    if let Some(location) = info.location() {
        let file = location.file();
        let line = location.line();
        console::print("at ");
        console::buffer(&file);
        console::print(" line ");
        console::printint(line as i32);
        console::print("\n");
        console::flush();
    } else { console::print("no loc\n"); console::flush(); }
    bkpt();
    console::print(info.payload().downcast_ref::<&str>().unwrap());
    console::print("\n");
    console::flush();
    loop  { }
}
