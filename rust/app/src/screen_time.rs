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
fn create_screen(ht: &home_time_widget_t) -> LvglResult<()> {
    let scr = ht.screen;

    //  Create a label for time (00:00)
    let label1 = label::create(scr, ptr::null()) ? ;
    label::set_long_mode(label1, label::LV_LABEL_LONG_BREAK);
    label::set_text(label1, strn!("00:00"));  //  strn creates a null-terminated string
    obj::set_width(label1, 240);
    obj::set_height(label1, 200);
    label::set_align(label1, label::LV_LABEL_ALIGN_CENTER);
    obj::align(label1, scr, obj::LV_ALIGN_CENTER, 0, -30);
    label::set_style(label1, label::LV_LABEL_STYLE_MAIN, &style_time);
    ht.lv_time = label1;

    //  Create a label for Bluetooth state
    let l_state = label::create(scr, ptr::null()) ? ;
    obj::set_width(l_state, 50);
    obj::set_height(l_state, 80);
    label::set_text(l_state, strn!(""));  //  strn creates a null-terminated string
    label::set_recolor(l_state, true);
    label::set_align(l_state, label::LV_LABEL_ALIGN_LEFT);
    obj::align(l_state, scr, obj::LV_ALIGN_IN_TOP_LEFT, 0, 0);
    ht.lv_ble = l_state;

    //  Create a label for Power indicator
    let l_power = label::create(scr, ptr::null()) ? ;
    obj::set_width(l_power, 80);
    obj::set_height(l_power, 20);
    label::set_text(l_power, strn!(""));  //  strn creates a null-terminated string
    label::set_recolor(l_power, true);
    label::set_align(l_power, label::LV_LABEL_ALIGN_RIGHT);
    obj::align(l_power, scr, obj::LV_ALIGN_IN_TOP_RIGHT, 0, 0);
    ht.lv_power = l_power;

    //  Create a label for Date
    let label_date = label::create(scr, ptr::null()) ? ;
    label::set_long_mode(label_date, label::LV_LABEL_LONG_BREAK);
    obj::set_width(label_date, 200);
    obj::set_height(label_date, 200);
    label::set_align(label_date, label::LV_LABEL_ALIGN_CENTER);
    obj::align(label_date, scr, obj::LV_ALIGN_CENTER, 0, 40);
    ht.lv_date = label_date;

    //  Allow touch events
    obj::set_click(scr, true);

    //  Set touch callbacks on the screen and the time label
    obj::set_event_cb(scr, screen_time_pressed);  //  TODO: Create Rust binding for screen_time_pressed() from screen_time.c
    obj::set_event_cb(label1, screen_time_pressed);

    //  Update the screen
    update_screen(&ht) ? ;
    Ok(())
}

/// Populate the screen with the current state. Called by screen_time_update_screen() below.
fn update_screen(htwidget: &home_time_widget_t) -> LvglResult<()> {
    set_time_label(htwidget) ? ;
    set_bt_label(htwidget) ? ;
    set_power_label(htwidget) ? ;
    Ok(())
}

/// Populate the Bluetooth Label with the Bluetooth state. Called by screen_time_update_screen() above.
fn set_bt_label(htwidget: &home_time_widget_t) -> LvglResult<()> {
    if htwidget.ble_state == BLEMAN_BLE_STATE_DISCONNECTED {
        label::set_text(htwidget.lv_ble, strn!(""));
    } else {
        let color = state2color[htwidget.ble_state];
        label::set_text_fmt(htwidget.lv_ble,  //  TODO: Convert to heapless write
            //  TODO: strn!("%s "LV_SYMBOL_BLUETOOTH"#"),  //  LV_SYMBOL_BLUETOOTH="\xef\x8a\x93"
            strn!("%s BT#"),
            color
        );
    }
    Ok(())
}

/// Populate the Power Label with the battery status. Called by screen_time_update_screen() above.
fn set_power_label(htwidget: &home_time_widget_t) -> LvglResult<()> {
    let percentage = hal_battery_get_percentage(htwidget.millivolts);
    let color = 
        if percentage <= battery_low 
            { battery_low_color }
        else if htwidget.powered && !(htwidget.charging) 
            { battery_full_color }  //  Battery charge cycle finished
        else 
            { battery_mid_color };
    label::set_text_fmt(htwidget.lv_power,  //  TODO: Convert to heapless write
        strn!("%s %u%%%s#\n(%umV)"),
        color,
        percentage,
        if htwidget.powered { strn!("C") }  //  TODO: LV_SYMBOL_CHARGE="\xef\x83\xa7"
            else { strn!(" ") },
        htwidget.millivolts
    );
    obj::align(htwidget.lv_power, htwidget.screen, obj::LV_ALIGN_IN_TOP_RIGHT, 0, 0);
    Ok(())
}

/// Populate the Time and Date Labels with the time and date. Called by screen_time_update_screen() above.
fn set_time_label(ht: &home_time_widget_t) -> LvglResult<()> {
    //  Create a string buffer with max size 6 and format the time
    let mut time = heapless::String::<heapless::consts::U6>::new();
    write!(&mut time, "{:02}:{:02}", 
        ht.time.hour, 
        ht.time.minute)
        .expect("time fail");
    label::set_text(ht.lv_time, time);

    //  Create a string buffer with max size 15 and format the date
    let mut date = heapless::String::<heapless::consts::U15>::new();
    write!(&mut date, "{} {} {}\n",
        ht.time.dayofmonth,
        controller_time_month_get_short_name(&ht.time),
        ht.time.year)
        .expect("date fail");
    label::set_text(ht.lv_date, date);
    Ok(())
}

/// Create the Time Screen, populated with widgets. Called by home_time_draw() in screen_time.c.
#[no_mangle]  //  Don't mangle the function name
extern "C" fn screen_time_create(ht: *const home_time_widget_t) -> *mut obj::lv_obj_t {  //  Declare extern "C" because it will be called by RIOT OS firmware
    //  Create the screen object and update the screen widget
    let scr = obj::create(ptr::null_mut(), ptr::null())
        .expect("create screen obj fail");
    (*ht).screen = scr;

    //  Populate the widgets in the screen
    create_screen(&*ht)
        .expect("create_screen fail");
    scr  //  Return the screen
}

/// Populate the screen with the current state. Called by home_time_update_screen() in screen_time.c and by screen_time_create() above.
#[no_mangle]  //  Don't mangle the function name
extern "C" fn screen_time_update_screen(widget: &widget_t) -> i32 {
    let ht_widget = from_widget(widget);  //  TODO: Create Rust binding for from_widget() from screen_time.c
    update_screen(&ht_widget)
        .expect("update_screen fail");
    0  //  Return OK
}

//  TODO: Sync with screen_time.c
#[repr(C)]
struct home_time_widget_t {
    widget:     widget_t,
    handler:    control_event_handler_t,
    screen:     *mut obj::lv_obj_t,
    lv_time:    *mut obj::lv_obj_t,
    lv_date:    *mut obj::lv_obj_t,
    lv_ble:     *mut obj::lv_obj_t,
    lv_power:   *mut obj::lv_obj_t,
    ble_state:  bleman_ble_state_t,
    /* Shared storage between gui and control */
    time:       controller_time_spec_t,
    millivolts: u32,
    charging:   bool,
    powered:    bool,
}

//  TODO: Sync with screen_time.c
struct widget_t {}
struct control_event_handler_t {}
struct controller_time_spec_t {}

/* Stack Trace for screen_time_create:
#0  screen_time_create (ht=ht@entry=0x200008dc <home_time_widget>) at /Users/Luppy/PineTime/PineTime-apps/widgets/home_time/screen_time.c:68
#1  0x0001b36c in home_time_draw (widget=0x200008dc <home_time_widget>, parent=<optimized out>) at /Users/Luppy/PineTime/PineTime-apps/widgets/home_time/screen_time.c:222
#2  0x00003b32 in _switch_widget_draw (type=<optimized out>, widget=0x200008dc <home_time_widget>, gui=<optimized out>) at /Users/Luppy/PineTime/PineTime-apps/modules/gui/gui.c:159
#3  _gui_handle_msg (msg=0x20004c90 <_stack+1944>, gui=<optimized out>) at /Users/Luppy/PineTime/PineTime-apps/modules/gui/gui.c:299
#4  _lvgl_thread (arg=0x20004408 <_gui>) at /Users/Luppy/PineTime/PineTime-apps/modules/gui/gui.c:351
#5  0x000002f0 in sched_switch (other_prio=<optimized out>) at /Users/Luppy/PineTime/PineTime-apps/RIOT/core/sched.c:179
*/