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
    fn create_screen(ht: &home_time_widget_t) -> LvglResult<()> {
        let scr = ht.screen;
        let label1 = label::create(scr, ptr::null())?;
        label::set_long_mode(label1, label::LV_LABEL_LONG_BREAK);
        label::set_text(label1, &Strn::new(b"00:00\x00"));
        obj::set_width(label1, 240);
        obj::set_height(label1, 200);
        label::set_align(label1, label::LV_LABEL_ALIGN_CENTER);
        obj::align(label1, scr, obj::LV_ALIGN_CENTER, 0, -30);
        label::set_style(label1, label::LV_LABEL_STYLE_MAIN, &style_time);
        ht.lv_time = label1;
        let l_state = label::create(scr, ptr::null())?;
        obj::set_width(l_state, 50);
        obj::set_height(l_state, 80);
        label::set_text(l_state, &Strn::new(b"\x00"));
        label::set_recolor(l_state, true);
        label::set_align(l_state, label::LV_LABEL_ALIGN_LEFT);
        obj::align(l_state, scr, obj::LV_ALIGN_IN_TOP_LEFT, 0, 0);
        ht.lv_ble = l_state;
        let l_power = label::create(scr, ptr::null())?;
        obj::set_width(l_power, 80);
        obj::set_height(l_power, 20);
        label::set_text(l_power, &Strn::new(b"\x00"));
        label::set_recolor(l_power, true);
        label::set_align(l_power, label::LV_LABEL_ALIGN_RIGHT);
        obj::align(l_power, scr, obj::LV_ALIGN_IN_TOP_RIGHT, 0, 0);
        ht.lv_power = l_power;
        let label_date = label::create(scr, ptr::null())?;
        label::set_long_mode(label_date, label::LV_LABEL_LONG_BREAK);
        obj::set_width(label_date, 200);
        obj::set_height(label_date, 200);
        label::set_align(label_date, label::LV_LABEL_ALIGN_CENTER);
        obj::align(label_date, scr, obj::LV_ALIGN_CENTER, 0, 40);
        ht.lv_date = label_date;
        obj::set_click(scr, true);
        obj::set_event_cb(scr, screen_time_pressed);
        obj::set_event_cb(label1, screen_time_pressed);
        update_screen(&ht)?;
        Ok(())
    }
    /// Populate the screen with the current state. Called by screen_time_update_screen() below.
    fn update_screen(htwidget: &home_time_widget_t) -> LvglResult<()> {
        set_time_label(htwidget)?;
        set_bt_label(htwidget)?;
        set_power_label(htwidget)?;
        Ok(())
    }
    /// Populate the Bluetooth Label with the Bluetooth status. Called by screen_time_update_screen() above.
    fn set_bt_label(htwidget: &home_time_widget_t) -> LvglResult<()> {
        if htwidget.ble_state == BLEMAN_BLE_STATE_DISCONNECTED {
            label::set_text(htwidget.lv_ble, &Strn::new(b"\x00"));
        } else {
            let color = state2color[htwidget.ble_state];
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
            label::set_text(htwidget.lv_ble, &Strn::new(status.as_bytes()));
        }
        Ok(())
    }
    /// Populate the Power Label with the battery status. Called by screen_time_update_screen() above.
    fn set_power_label(htwidget: &home_time_widget_t) -> LvglResult<()> {
        let percentage = hal_battery_get_percentage(htwidget.millivolts);
        let color =
            if percentage <= battery_low {
                battery_low_color
            } else if htwidget.powered && !(htwidget.charging) {
                battery_full_color
            } else { battery_mid_color };
        let symbol = if htwidget.powered { "\u{F0E7}" } else { " " };
        let mut status = heapless::String::<heapless::consts::U16>::new();
        (&mut status).write_fmt(::core::fmt::Arguments::new_v1(&["", " ", "%",
                                                                 "#\n(",
                                                                 "mV)\u{0}"],
                                                               &match (&color,
                                                                       &percentage,
                                                                       &symbol,
                                                                       &htwidget.millivolts)
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
        label::set_text(htwidget.lv_power, &Strn::new(status.as_bytes()));
        obj::align(htwidget.lv_power, htwidget.screen,
                   obj::LV_ALIGN_IN_TOP_RIGHT, 0, 0);
        Ok(())
    }
    /// Populate the Time and Date Labels with the time and date. Called by screen_time_update_screen() above.
    fn set_time_label(htwidget: &home_time_widget_t) -> LvglResult<()> {
        let mut time = heapless::String::<heapless::consts::U6>::new();
        (&mut time).write_fmt(::core::fmt::Arguments::new_v1_formatted(&["",
                                                                         ":",
                                                                         "\u{0}"],
                                                                       &match (&htwidget.time.hour,
                                                                               &htwidget.time.minute)
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
        label::set_text(htwidget.lv_time, &Strn::new(time.as_bytes()));
        let mut date = heapless::String::<heapless::consts::U15>::new();
        (&mut date).write_fmt(::core::fmt::Arguments::new_v1(&["", " ", " ",
                                                               "\n\u{0}"],
                                                             &match (&htwidget.time.dayofmonth,
                                                                     &controller_time_month_get_short_name(&htwidget.time),
                                                                     &htwidget.time.year)
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
        label::set_text(htwidget.lv_date, &Strn::new(date.as_bytes()));
        Ok(())
    }
    /// Create the Time Screen, populated with widgets. Called by home_time_draw() in screen_time.c.
    #[no_mangle]
    extern "C" fn screen_time_create(htwidget: *const home_time_widget_t)
     -> *mut obj::lv_obj_t {
        let scr =
            obj::create(ptr::null_mut(),
                        ptr::null()).expect("create screen obj fail");
        (*htwidget).screen = scr;
        create_screen(&*htwidget).expect("create_screen fail");
        scr
    }
    /// Populate the screen with the current state. Called by home_time_update_screen() in screen_time.c and by screen_time_create() above.
    #[no_mangle]
    extern "C" fn screen_time_update_screen(widget: *const widget_t) -> i32 {
        let htwidget = from_widget(widget);
        update_screen(&htwidget).expect("update_screen fail");
        0
    }
    #[repr(C)]
    struct home_time_widget_t {
        widget: widget_t,
        handler: control_event_handler_t,
        screen: *mut obj::lv_obj_t,
        lv_time: *mut obj::lv_obj_t,
        lv_date: *mut obj::lv_obj_t,
        lv_ble: *mut obj::lv_obj_t,
        lv_power: *mut obj::lv_obj_t,
        ble_state: bleman_ble_state_t,
        time: controller_time_spec_t,
        millivolts: u32,
        charging: bool,
        powered: bool,
    }
    struct widget_t {
    }
    struct control_event_handler_t {
    }
    struct controller_time_spec_t {
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
