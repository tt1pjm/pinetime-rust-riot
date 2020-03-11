/*
 * Copyright (C) 2018 Koen Zandberg <koen@bergzand.net>
 *
 * This file is subject to the terms and conditions of the GNU Lesser
 * General Public License v2.1. See the file LICENSE in the top level
 * directory for more details.
 */

#ifndef WIDGET_HOME_TIME_H
#define WIDGET_HOME_TIME_H

#include "lvgl.h"
#include "widget.h"
#include "controller.h"

#ifdef __cplusplus
extern "C" {
#endif

/// State for the Watch Face, shared between GUI and control. TODO: Sync with rust/app/src/screen_time.rs
typedef struct WatchFaceStateStruct {
    uint8_t                 ble_state;  //  Previously bleman_ble_state_t, now synced with Rust
    controller_time_spec_t  time;
    uint32_t                millivolts;
    bool                    charging;
    bool                    powered;
} WatchFaceState;

/// Widgets for the Watch Face, private to Rust. TODO: Sync with rust/app/src/screen_time.rs
typedef struct WatchFaceWidgetsStruct {
    lv_obj_t *screen;       //  Shared with home_time_widget_t
    lv_obj_t *time_label;   //  TODO: Should be private to Rust
    lv_obj_t *date_label;   //  TODO: Should be private to Rust
    lv_obj_t *ble_label;    //  TODO: Should be private to Rust
    lv_obj_t *power_label;  //  TODO: Should be private to Rust
} WatchFaceWidgets;

/// LVGL Widget for Watch Face
typedef struct _home_time_widget {
    widget_t                widget;
    control_event_handler_t handler;
    lv_obj_t                *screen;     //  Shared with WatchFaceWidgets
    WatchFaceState          state;       //  State for the Watch Face, shared between GUI and control
    WatchFaceWidgets        subwidgets;  //  Child Widgets for the Watch Face
} home_time_widget_t;

#ifdef __cplusplus
}
#endif


#endif /* WIDGET_HOME_TIME_H */

