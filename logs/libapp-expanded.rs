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
    use core::ptr;
    use lvgl::{result::*, core::{obj}, objx::{label}, Strn, fill_zero};
    use lvgl_macros::{strn};
    /// Style for the Time Label
    static mut style_time: obj::lv_style_t =
        unsafe {
            ::core::mem::transmute::<[u8; ::core::mem::size_of::<obj::lv_style_t>()],
                                     obj::lv_style_t>([0;
                                                          ::core::mem::size_of::<obj::lv_style_t>()])
        };
    /// Create the Time Screen, populated with widgets
    #[no_mangle]
    extern "C" fn screen_time_create(ht: *const home_time_widget_t)
     -> LvglResult<*mut obj::lv_obj_t> {
        let scr = obj::create(ptr::null_mut(), ptr::null())?;
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
        obj::set_event_cb(scr, _screen_time_pressed);
        obj::set_event_cb(label1, _screen_time_pressed);
        _screen_time_update_screen(&ht.widget);
        Ok(scr)
    }
    #[repr(C)]
    struct home_time_widget_t {
        widget: widget_t,
        handler: control_event_handler_t,
        screen: *const obj::lv_obj_t,
        lv_time: *const obj::lv_obj_t,
        lv_date: *const obj::lv_obj_t,
        lv_ble: *const obj::lv_obj_t,
        lv_power: *const obj::lv_obj_t,
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
