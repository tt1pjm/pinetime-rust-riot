//! Watch Face in Rust
use lvgl::{
    core::{
        obj,
    },
    objx::{
        label,
    },
    NULL,
};

#[no_mangle]  //  Don't mangle the function name
extern "C" fn screen_time_create(ht: &home_time_widget_t) -> &obj::lv_obj_t {  //  Declare extern "C" because it will be called by RIOT OS firmware
    let scr = obj::lv_obj_create(NULL, NULL);

    //  time (00:00)
    let label1 = label::lv_label_create(scr, NULL);
    label::lv_label_set_long_mode(label1, LV_LABEL_LONG_BREAK);
    label::lv_label_set_text(label1, "00:00");
    label::lv_obj_set_width(label1, 240);
    label::lv_obj_set_height(label1, 200);
    label::lv_label_set_align(label1, LV_LABEL_ALIGN_CENTER);
    label::lv_obj_align(label1, scr, LV_ALIGN_CENTER, 0, -30);
    label::lv_label_set_style(label1, LV_LABEL_STYLE_MAIN, &style_time);
    ht.lv_time = label1;

    let l_state = label::lv_label_create(scr, NULL);
    obj::lv_obj_set_width(l_state, 50);
    obj::lv_obj_set_height(l_state, 80);
    label::lv_label_set_text(l_state, "");
    label::lv_label_set_recolor(l_state, true);
    label::lv_label_set_align(l_state, LV_LABEL_ALIGN_LEFT);
    obj::lv_obj_align(l_state, scr, LV_ALIGN_IN_TOP_LEFT, 0, 0);
    ht.lv_ble = l_state;

    //  Power indicator
    let l_power = label::lv_label_create(scr, NULL);
    obj::lv_obj_set_width(l_power, 80);
    obj::lv_obj_set_height(l_power, 20);
    label::lv_label_set_text(l_power, "");
    label::lv_label_set_recolor(l_power, true);
    label::lv_label_set_align(l_power, LV_LABEL_ALIGN_RIGHT);
    obj::lv_obj_align(l_power, scr, LV_ALIGN_IN_TOP_RIGHT, 0, 0);
    ht.lv_power = l_power;

    //  Date
    let label_date = label::lv_label_create(scr, NULL);
    label::lv_label_set_long_mode(label_date, LV_LABEL_LONG_BREAK);
    obj::lv_obj_set_width(label_date, 200);
    obj::lv_obj_set_height(label_date, 200);
    label::lv_label_set_align(label_date, LV_LABEL_ALIGN_CENTER);
    obj::lv_obj_align(label_date, scr, LV_ALIGN_CENTER, 0, 40);
    ht.lv_date = label_date;

    obj::lv_obj_set_click(scr, true);

    obj::lv_obj_set_event_cb(scr, _screen_time_pressed);
    obj::lv_obj_set_event_cb(label1, _screen_time_pressed);

    _screen_time_update_screen(&ht.widget);

    scr  //  Return the screen
}
