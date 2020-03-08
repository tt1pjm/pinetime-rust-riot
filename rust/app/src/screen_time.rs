//! Watch Face in Rust
use lvgl::{
    core::{
        obj,
    },
    objx::{
        label,
    },
};

#[no_mangle]  //  Don't mangle the function name
extern "C" fn screen_time_create(ht: &home_time_widget_t) -> &lv_obj_t {  //  Declare extern "C" because it will be called by RIOT OS firmware
    lv_obj_t *scr = lv_obj_create(NULL, NULL);

    /* time (00:00)*/
    lv_obj_t * label1 = lv_label_create(scr, NULL);
    lv_label_set_long_mode(label1, LV_LABEL_LONG_BREAK);
    lv_label_set_text(label1, "00:00");
    lv_obj_set_width(label1, 240);
    lv_obj_set_height(label1, 200);
    lv_label_set_align(label1, LV_LABEL_ALIGN_CENTER);
    lv_obj_align(label1, scr, LV_ALIGN_CENTER, 0, -30);
    lv_label_set_style(label1, LV_LABEL_STYLE_MAIN, &style_time);
    ht->lv_time = label1;

    lv_obj_t * l_state = lv_label_create(scr, NULL);
    lv_obj_set_width(l_state, 50);
    lv_obj_set_height(l_state, 80);
    lv_label_set_text(l_state, "");
    lv_label_set_recolor(l_state, true);
    lv_label_set_align(l_state, LV_LABEL_ALIGN_LEFT);
    lv_obj_align(l_state, scr, LV_ALIGN_IN_TOP_LEFT, 0, 0);
    ht->lv_ble = l_state;

    /* Power indicator */
    lv_obj_t * l_power = lv_label_create(scr, NULL);
    lv_obj_set_width(l_power, 80);
    lv_obj_set_height(l_power, 20);
    lv_label_set_text(l_power, "");
    lv_label_set_recolor(l_power, true);
    lv_label_set_align(l_power, LV_LABEL_ALIGN_RIGHT);
    lv_obj_align(l_power, scr, LV_ALIGN_IN_TOP_RIGHT, 0, 0);
    ht->lv_power = l_power;

    /* Date */
    lv_obj_t * label_date = lv_label_create(scr, NULL);
    lv_label_set_long_mode(label_date, LV_LABEL_LONG_BREAK);
    lv_obj_set_width(label_date, 200);
    lv_obj_set_height(label_date, 200);
    lv_label_set_align(label_date, LV_LABEL_ALIGN_CENTER);
    lv_obj_align(label_date, scr, LV_ALIGN_CENTER, 0, 40);
    ht->lv_date = label_date;

    lv_obj_set_click(scr, true);

    lv_obj_set_event_cb(scr, _screen_time_pressed);
    lv_obj_set_event_cb(label1, _screen_time_pressed);

    _screen_time_update_screen(&ht->widget);
    scr
}
