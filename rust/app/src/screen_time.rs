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
extern "C" fn screen_time_create(ht: *const home_time_widget_t) -> LvglResult<*mut obj::lv_obj_t> {  //  Declare extern "C" because it will be called by RIOT OS firmware
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

/* TODO
static void _home_time_set_bt_label(home_time_widget_t *htwidget)
{

    if (htwidget->ble_state == BLEMAN_BLE_STATE_DISCONNECTED ) {
        lv_label_set_text(htwidget->lv_ble, "");
    }
    else {
        const char *color = _state2color[htwidget->ble_state];
        lv_label_set_text_fmt(htwidget->lv_ble,
                              "%s "LV_SYMBOL_BLUETOOTH"#",
                              color);
    }
}

static void _home_time_set_power_label(home_time_widget_t *htwidget)
{
    const char *color = battery_mid_color;
    unsigned percentage = hal_battery_get_percentage(htwidget->millivolts);
    if (percentage <= battery_low) {
        color = battery_low_color;
    }
    if (htwidget->powered && !(htwidget->charging) ) {
        /* Battery charge cycle finished */
        color = battery_full_color;
    }
    lv_label_set_text_fmt(htwidget->lv_power,
                          "%s %u%%%s#\n(%"PRIu32"mV)",
                          color, percentage,
                          htwidget->powered ? LV_SYMBOL_CHARGE : " ",
                          htwidget->millivolts
                          );
    lv_obj_align(htwidget->lv_power, htwidget->screen, LV_ALIGN_IN_TOP_RIGHT, 0, 0);
}

static int _home_time_set_time_label(home_time_widget_t *ht)
{
    char time[6];
    char date[15];
    int res = snprintf(time, sizeof(time), "%02u:%02u", ht->time.hour,
                       ht->time.minute);
    if (res != sizeof(time) - 1) {
        LOG_ERROR("[home_time]: error formatting time string %*s\n", res, time);
        return -1;
    }
    lv_label_set_text(ht->lv_time, time);

    res = snprintf(date, sizeof(date), "%u %s %u\n", ht->time.dayofmonth,
                   controller_time_month_get_short_name(&ht->time),
                   ht->time.year);
    if (res == sizeof(date)) {
        LOG_ERROR("[home_time]: error formatting date string %*s\n", res, date);
        return -1;
    }
    lv_label_set_text(ht->lv_date, date);
    return 0;
}

static int _screen_time_update_screen(widget_t *widget)
{
    home_time_widget_t *ht = _from_widget(widget);

    _home_time_set_time_label(ht);
    _home_time_set_bt_label(ht);
    _home_time_set_power_label(ht);
    return 0;
}
*/

/* Stack Trace for screen_time_create:
#0  screen_time_create (ht=ht@entry=0x200008dc <home_time_widget>) at /Users/Luppy/PineTime/PineTime-apps/widgets/home_time/screen_time.c:68
#1  0x0001b36c in home_time_draw (widget=0x200008dc <home_time_widget>, parent=<optimized out>) at /Users/Luppy/PineTime/PineTime-apps/widgets/home_time/screen_time.c:222
#2  0x00003b32 in _switch_widget_draw (type=<optimized out>, widget=0x200008dc <home_time_widget>, gui=<optimized out>) at /Users/Luppy/PineTime/PineTime-apps/modules/gui/gui.c:159
#3  _gui_handle_msg (msg=0x20004c90 <_stack+1944>, gui=<optimized out>) at /Users/Luppy/PineTime/PineTime-apps/modules/gui/gui.c:299
#4  _lvgl_thread (arg=0x20004408 <_gui>) at /Users/Luppy/PineTime/PineTime-apps/modules/gui/gui.c:351
#5  0x000002f0 in sched_switch (other_prio=<optimized out>) at /Users/Luppy/PineTime/PineTime-apps/RIOT/core/sched.c:179
*/