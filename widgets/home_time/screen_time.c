/*
 * Copyright (C) 2018 Koen Zandberg <koen@bergzand.net>
 *
 * This file is subject to the terms and conditions of the GNU Lesser
 * General Public License v2.1. See the file LICENSE in the top level
 * directory for more details.
 */

#include <stdint.h>
#include "home_time.h"
#include "hal.h"
#include "log.h"
#include "lvgl.h"
#include "gui.h"
#include "gui/theme.h"
#include "controller.h"
#include "kernel_defines.h"
#include "bleman.h"
#include "fonts/noto_sans_numeric_80.h"

static lv_obj_t *_screen_time_create(home_time_widget_t *ht);
static int _screen_time_update_screen(widget_t *widget);
static void _screen_time_pressed(lv_obj_t *obj, lv_event_t event);

static const widget_spec_t home_time_spec;
lv_style_t style_time;  //  Exposed to Rust

/* Widget context */
home_time_widget_t home_time_widget = {
    .widget = {.spec = &home_time_spec }
};

///////////////////////////////////////////////////////////////////////////////
//  Handlers for Home Time

static home_time_widget_t *_from_widget(widget_t *widget)
{
    return container_of(widget, home_time_widget_t, widget);
}

static inline home_time_widget_t *active_widget(void)
{
    return &home_time_widget;
}

static int home_time_update_screen(widget_t *widget)
{
    if (widget_get_gui_lock(widget) == 0) {
        return 0;
    }
    LOG_DEBUG("[home_screen]: updating drawing\n");
    _screen_time_update_screen(widget);
    widget_release_gui_lock(widget);
    return 1;
}

int home_time_init(widget_t *widget)
{
    home_time_widget_t *htwidget = _from_widget(widget);
    widget_init_local(widget);
    htwidget->handler.events = CONTROLLER_EVENT_FLAG(CONTROLLER_EVENT_TICK) |
                               CONTROLLER_EVENT_FLAG(CONTROLLER_EVENT_BLUETOOTH);
    htwidget->handler.widget = widget;

    lv_style_copy(&style_time, gui_theme_get()->style.label.prim);
    style_time.text.font = &noto_sans_numeric_80;

    controller_add_control_handler(controller_get(), &htwidget->handler);
    return 0;
}

int home_time_launch(widget_t *widget)
{
    home_time_widget_t *htwidget = _from_widget(widget);
    (void)htwidget;
    return 0;
}

int home_time_draw(widget_t *widget, lv_obj_t *parent)
{
    LOG_INFO("drawing time widget\n");
    home_time_widget_t *htwidget = _from_widget(widget);
    htwidget->screen = _screen_time_create(htwidget);
    return 0;
}

lv_obj_t *home_time_get_container(widget_t *widget)
{
    home_time_widget_t *htwidget = _from_widget(widget);
    return htwidget->screen;
}

int home_time_close(widget_t *widget)
{
    home_time_widget_t *htwidget = _from_widget(widget);
    lv_obj_del(htwidget->screen);
    htwidget->screen = NULL;
    return 0;
}

/// Update the power state
static void _update_power_stats(WatchFaceState *state)
{
    state->powered = hal_battery_is_powered();
    state->charging = hal_battery_is_charging();
    state->millivolts = controller_get_battery_voltage(controller_get());
}

/// Upon receiving a tick or Bluetooth event, update the widget state
int home_time_event(widget_t *widget, controller_event_t event)
{
    home_time_widget_t *htwidget = _from_widget(widget);
    assert(htwidget != NULL);
    widget_get_control_lock(widget);

    //  Update the time and power state
    WatchFaceState *state = &(htwidget->state);
    if (event == CONTROLLER_EVENT_TICK) {
        memcpy(&state->time, controller_time_get_time(controller_get()), sizeof(controller_time_spec_t));
        _update_power_stats(state);
    }
#ifdef MODULE_BLEMAN
    //  Update the Bluetooth state
    if (event == CONTROLLER_EVENT_BLUETOOTH) {
        state->ble_state = bleman_get_conn_state(bleman_get(), NULL);
    }
#endif
    widget_release_control_lock(widget);
    return 0;
}

///////////////////////////////////////////////////////////////////////////////
//  Handlers for Screen Time: Create, Update, Pressed

int create_watch_face(WatchFaceWidgets *widgets);                         //  Exported by Rust: rust/app/src/screen_time.rs
int update_watch_face(WatchFaceWidgets *widgets, WatchFaceState *state);  //  Exported by Rust: rust/app/src/screen_time.rs

/// Create the Time Screen, populated with widgets. Called by home_time_draw() above.
static lv_obj_t *_screen_time_create(home_time_widget_t *ht)
{
    //  Create the screen object and update the screen widget
    assert(ht != NULL);
    lv_obj_t *screen = lv_obj_create(NULL, NULL);
    assert(screen != NULL);
    ht->screen = screen;

    //  Populate the widgets in the screen
    WatchFaceWidgets *subwidgets = &(ht->subwidgets);
    subwidgets->screen = screen;
    int res = create_watch_face(subwidgets);
    assert(res == 0);

    //  Set touch callbacks on the screen
    lv_obj_set_event_cb(screen, _screen_time_pressed);
    //  lv_obj_set_event_cb(label1, _screen_time_pressed);  //  TODO: Is this needed?

    //  Update the screen
    _screen_time_update_screen(&ht->widget);
    return screen;
}

/// Populate the Time Screen with the current status. Called by home_time_update_screen() and _screen_time_create() above.
static int _screen_time_update_screen(widget_t *widget)
{
    assert(widget != NULL);
    home_time_widget_t *ht = _from_widget(widget);
    assert(ht != NULL);

    //  Update the widgets in the screen
    WatchFaceWidgets *subwidgets = &(ht->subwidgets);
    WatchFaceState *state = &(ht->state);
    int res = update_watch_face(subwidgets, state);
    assert(res == 0);
    return 0;  //  Return OK
}

/// Handle touch events on the Time Screen
static void _screen_time_pressed(lv_obj_t *obj, lv_event_t event)
{
    home_time_widget_t *ht = active_widget();
    switch (event) {
        case LV_EVENT_CLICKED:
            LOG_INFO("Screen press event\n");
            controller_action_submit_input_action(&ht->widget,
                                                CONTROLLER_ACTION_WIDGET_MENU, NULL);
        default:
            break;
    }
}

///////////////////////////////////////////////////////////////////////////////
/// Define the Time Screen widget
static const widget_spec_t home_time_spec = {
    .name = "time",
    .init = home_time_init,
    .launch = home_time_launch,
    .draw = home_time_draw,
    .container = home_time_get_container,
    .close = home_time_close,
    .event = home_time_event,
    .update_draw = home_time_update_screen,
    .gui_event = widget_face_gui_event,
};
