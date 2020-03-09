//! Watch Face in Rust
use core::ptr;
use lvgl::{
    result::*,
    core::{
        obj,
    },
    objx::{
        label,
    },
    fill_zero,
};

/// Style for the Time Label
static mut style_time: obj::lv_style_t = fill_zero!(obj::lv_style_t);

/// Create the Time Screen, populated with widgets
#[no_mangle]  //  Don't mangle the function name
extern "C" fn screen_time_create<'a>(ht: &'a home_time_widget_t) -> LvglResult<*mut obj::lv_obj_t> {  //  Declare extern "C" because it will be called by RIOT OS firmware
    let scr = obj::create(ptr::null_mut(), ptr::null()) ? ;

    //  Create a label for time (00:00)
    let label1 = label::create(scr, ptr::null()) ? ;
    label::set_long_mode(label1, label::LV_LABEL_LONG_BREAK);
    label::set_text(label1, "00:00");
    obj::set_width(label1, 240);
    obj::set_height(label1, 200);
    label::set_align(label1, label::LV_LABEL_ALIGN_CENTER);
    obj::align(label1, scr, obj::LV_ALIGN_CENTER, 0, -30);
    label::set_style(label1, label::LV_LABEL_STYLE_MAIN, &style_time);
    ht.lv_time = label1;

    //  Create a label for BLE state
    let l_state = label::create(scr, ptr::null()) ? ;
    obj::set_width(l_state, 50);
    obj::set_height(l_state, 80);
    label::set_text(l_state, "");
    label::set_recolor(l_state, true);
    label::set_align(l_state, label::LV_LABEL_ALIGN_LEFT);
    obj::align(l_state, scr, obj::LV_ALIGN_IN_TOP_LEFT, 0, 0);
    ht.lv_ble = l_state;

    //  Create a label for Power indicator
    let l_power = label::create(scr, ptr::null()) ? ;
    obj::set_width(l_power, 80);
    obj::set_height(l_power, 20);
    label::set_text(l_power, "");
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
    obj::set_event_cb(scr, _screen_time_pressed);
    obj::set_event_cb(label1, _screen_time_pressed);

    //  Update the screen
    _screen_time_update_screen(&ht.widget);

    Ok(scr)  //  Return the screen
}

//  TODO
#[repr(C)]
struct home_time_widget_t {
    widget: widget_t,
    handler: control_event_handler_t,
    screen:   *const obj::lv_obj_t,
    lv_time:  *const obj::lv_obj_t,
    lv_date:  *const obj::lv_obj_t,
    lv_ble:   *const obj::lv_obj_t,
    lv_power: *const obj::lv_obj_t,
    ble_state: bleman_ble_state_t,
    /* Shared storage between gui and control */
    time: controller_time_spec_t,
    millivolts: u32,
    charging: bool,
    powered: bool,
}

//  TODO
struct widget_t {}
struct control_event_handler_t {}
struct controller_time_spec_t {}
