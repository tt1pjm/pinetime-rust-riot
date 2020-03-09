//! Watch Face in Rust
use core::{
    fmt::Write,
    ptr,
};
use lvgl::{
    result::*,
    core::{
        obj,
    },
    objx::{
        label,
    },
    Strn, fill_zero,
};
use lvgl_macros::{
    strn,
};

/// Style for the Time Label
static mut style_time: obj::lv_style_t = fill_zero!(obj::lv_style_t);

/// Create the Time Screen, populated with widgets. Called by screen_time_create() below.
fn create_screen(widgets: &WatchFaceWidgets) -> LvglResult<()> {
    let scr = widgets.screen;
    assert!(!scr.is_null(), "null screen");

    //  Create a label for time (00:00)
    let label1 = label::create(scr, ptr::null()) ? ;
    label::set_long_mode(label1, label::LV_LABEL_LONG_BREAK);
    label::set_text(label1, strn!("00:00"));  //  strn creates a null-terminated string
    obj::set_width(label1, 240);
    obj::set_height(label1, 200);
    label::set_align(label1, label::LV_LABEL_ALIGN_CENTER);
    obj::align(label1, scr, obj::LV_ALIGN_CENTER, 0, -30);
    label::set_style(label1, label::LV_LABEL_STYLE_MAIN, &style_time);
    widgets.time_label = label1;

    //  Create a label for Bluetooth state
    let l_state = label::create(scr, ptr::null()) ? ;
    obj::set_width(l_state, 50);
    obj::set_height(l_state, 80);
    label::set_text(l_state, strn!(""));  //  strn creates a null-terminated string
    label::set_recolor(l_state, true);
    label::set_align(l_state, label::LV_LABEL_ALIGN_LEFT);
    obj::align(l_state, scr, obj::LV_ALIGN_IN_TOP_LEFT, 0, 0);
    widgets.ble_label = l_state;

    //  Create a label for Power indicator
    let l_power = label::create(scr, ptr::null()) ? ;
    obj::set_width(l_power, 80);
    obj::set_height(l_power, 20);
    label::set_text(l_power, strn!(""));  //  strn creates a null-terminated string
    label::set_recolor(l_power, true);
    label::set_align(l_power, label::LV_LABEL_ALIGN_RIGHT);
    obj::align(l_power, scr, obj::LV_ALIGN_IN_TOP_RIGHT, 0, 0);
    widgets.power_label = l_power;

    //  Create a label for Date
    let label_date = label::create(scr, ptr::null()) ? ;
    label::set_long_mode(label_date, label::LV_LABEL_LONG_BREAK);
    obj::set_width(label_date, 200);
    obj::set_height(label_date, 200);
    label::set_align(label_date, label::LV_LABEL_ALIGN_CENTER);
    obj::align(label_date, scr, obj::LV_ALIGN_CENTER, 0, 40);
    widgets.date_label = label_date;

    //  Allow touch events
    obj::set_click(scr, true);

    //  Set touch callbacks on the screen and the time label
    obj::set_event_cb(scr, screen_time_pressed);  //  TODO: Create Rust binding for screen_time_pressed() from screen_time.c
    obj::set_event_cb(label1, screen_time_pressed);
    Ok(())
}

/// Populate the screen with the current state. Called by screen_time_update_screen() below.
fn update_screen(widgets: &WatchFaceWidgets, state: &WatchFaceState) -> LvglResult<()> {
    set_time_label(widgets, state) ? ;
    set_bt_label(widgets, state) ? ;
    set_power_label(widgets, state) ? ;
    Ok(())
}

/// Populate the Bluetooth Label with the Bluetooth status. Called by screen_time_update_screen() above.
fn set_bt_label(widgets: &WatchFaceWidgets, state: &WatchFaceState) -> LvglResult<()> {
    if state.ble_state == BLEMAN_BLE_STATE_DISCONNECTED {
        label::set_text(widgets.ble_label, strn!(""));
    } else {
        let color = state2color[state.ble_state];
        //  Create a string buffer with max size 16 and format the Bluetooth status
        let mut status = heapless::String::<heapless::consts::U16>::new();
        write!(&mut status, 
            "{} \u{F293}#\0",  //  LV_SYMBOL_BLUETOOTH. Must terminate Rust strings with null.
            color)
            .expect("bt fail");
        label::set_text(widgets.ble_label, &Strn::new(status.as_bytes()));  //  TODO: Simplify
    }
    Ok(())
}

/// Populate the Power Label with the battery status. Called by screen_time_update_screen() above.
fn set_power_label(widgets: &WatchFaceWidgets, state: &WatchFaceState) -> LvglResult<()> {
    let percentage = hal_battery_get_percentage(state.millivolts);
    let color =   //  Charging color
        if percentage <= battery_low 
            { battery_low_color }
        else if state.powered && !(state.charging) 
            { battery_full_color }  //  Battery charge cycle finished
        else 
            { battery_mid_color };
    let symbol =  //  Charging symbol
        if state.powered { "\u{F0E7}" }  //  LV_SYMBOL_CHARGE
        else { " " };
    //  Create a string buffer with max size 16 and format the battery status
    let mut status = heapless::String::<heapless::consts::U16>::new();
    write!(&mut status, 
        "{} {}%{}#\n({}mV)\0",  //  Must terminate Rust strings with null
        color,
        percentage,
        symbol,
        state.millivolts)
        .expect("batt fail");
    label::set_text(widgets.power_label, &Strn::new(status.as_bytes()));  //  TODO: Simplify
    obj::align(widgets.power_label, widgets.screen, obj::LV_ALIGN_IN_TOP_RIGHT, 0, 0);
    Ok(())
}

/// Populate the Time and Date Labels with the time and date. Called by screen_time_update_screen() above.
fn set_time_label(widgets: &WatchFaceWidgets, state: &WatchFaceState) -> LvglResult<()> {
    //  Create a string buffer with max size 6 and format the time
    let mut time = heapless::String::<heapless::consts::U6>::new();
    write!(&mut time, "{:02}:{:02}\0",  //  Must terminate Rust strings with null
        state.time.hour,    //  TODO: Use C accessor function
        state.time.minute)  //  TODO: Use C accessor function
        .expect("time fail");
    label::set_text(widgets.time_label, &Strn::new(time.as_bytes()));  //  TODO: Simplify

    //  Create a string buffer with max size 15 and format the date
    let mut date = heapless::String::<heapless::consts::U15>::new();
    write!(&mut date, "{} {} {}\n\0",  //  Must terminate Rust strings with null
        state.time.dayofmonth,  //  TODO: Use C accessor function
        controller_time_month_get_short_name(&state.time),
        state.time.year)        //  TODO: Use C accessor function
        .expect("date fail");
    label::set_text(widgets.date_label, &Strn::new(date.as_bytes()));  //  TODO: Simplify
    Ok(())
}

/// Create the Time Screen, populated with widgets. Called by home_time_draw() in screen_time.c.
#[no_mangle]  //  Don't mangle the function name
extern "C" fn screen_time_create(widget: *const home_time_widget_t) -> *mut obj::lv_obj_t {  //  Declare extern "C" because it will be called by RIOT OS firmware
    //  Create the screen object and update the screen widget
    let screen = obj::create(ptr::null_mut(), ptr::null())
        .expect("create screen obj fail");
    (*widget).screen = screen;

    //  Populate the widgets in the screen
    let mut subwidgets = &(*widget).subwidgets;
    subwidgets.screen = screen;
    create_screen(subwidgets)
        .expect("create_screen fail");

    //  Update the screen
    let state = &(*widget).state;
    update_screen(subwidgets, state)
        .expect("update_screen fail");
    screen  //  Return the screen
}

/// Populate the Time Screen with the current status. Called by home_time_update_screen() in screen_time.c and by screen_time_create() above.
#[no_mangle]  //  Don't mangle the function name
extern "C" fn screen_time_update_screen(widget0: *const widget_t) -> i32 {
    let widget: *const home_time_widget_t = from_widget(widget0);  //  TODO: Create Rust binding for from_widget() from screen_time.c
    let subwidgets = &(*widget).subwidgets;
    let state = &(*widget).state;
    update_screen(subwidgets, state)
        .expect("update_screen fail");
    0  //  Return OK
}

/// LVGL Widget for Watch Face. TODO: Sync with widgets/home_time/include/home_time.h
#[repr(C)]
struct home_time_widget_t {
    widget:     widget_t,                 //  TODO: Should not be exposed to Rust
    handler:    control_event_handler_t,  //  TODO: Should not be exposed to Rust
    screen:     *mut obj::lv_obj_t,  //  TODO: Shared with WatchFaceWidgets
    state:      WatchFaceState,      //  TODO: State for the Watch Face, shared between GUI and control
    subwidgets: WatchFaceWidgets,    //  TODO: Child Widgets for the Watch Face
}

/// State for the Watch Face, shared between GUI and control. TODO: Sync with widgets/home_time/include/home_time.h
#[repr(C)]
struct WatchFaceState {
    ble_state:  bleman_ble_state_t,      //  TODO: Should not be exposed to Rust
    time:       controller_time_spec_t,  //  TODO: Should not be exposed to Rust
    millivolts: u32,
    charging:   bool,
    powered:    bool,
}

/// Widgets for the Watch Face, private to Rust. TODO: Sync with widgets/home_time/include/home_time.h
#[repr(C)]
struct WatchFaceWidgets {
    screen:      *mut obj::lv_obj_t,  //  TODO: Shared with home_time_widget_t
    time_label:  *mut obj::lv_obj_t,  //  TODO: Should be private to Rust
    date_label:  *mut obj::lv_obj_t,  //  TODO: Should be private to Rust
    ble_label:   *mut obj::lv_obj_t,  //  TODO: Should be private to Rust
    power_label: *mut obj::lv_obj_t,  //  TODO: Should be private to Rust
}

//  TODO: Sync with widgets/home_time/include/home_time.h
struct widget_t {}  //  TODO: Should not be exposed to Rust
struct control_event_handler_t {}  //  TODO: Should not be exposed to Rust
struct controller_time_spec_t {}   //  TODO: Should not be exposed to Rust

/* Stack Trace for screen_time_create:
#0  screen_time_create (ht=ht@entry=0x200008dc <home_time_widget>) at /Users/Luppy/PineTime/PineTime-apps/widgets/home_time/screen_time.c:68
#1  0x0001b36c in home_time_draw (widget=0x200008dc <home_time_widget>, parent=<optimized out>) at /Users/Luppy/PineTime/PineTime-apps/widgets/home_time/screen_time.c:222
#2  0x00003b32 in _switch_widget_draw (type=<optimized out>, widget=0x200008dc <home_time_widget>, gui=<optimized out>) at /Users/Luppy/PineTime/PineTime-apps/modules/gui/gui.c:159
#3  _gui_handle_msg (msg=0x20004c90 <_stack+1944>, gui=<optimized out>) at /Users/Luppy/PineTime/PineTime-apps/modules/gui/gui.c:299
#4  _lvgl_thread (arg=0x20004408 <_gui>) at /Users/Luppy/PineTime/PineTime-apps/modules/gui/gui.c:351
#5  0x000002f0 in sched_switch (other_prio=<optimized out>) at /Users/Luppy/PineTime/PineTime-apps/RIOT/core/sched.c:179
*/