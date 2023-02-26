use std::{sync::Arc, thread, time::Duration};

use rdev::{simulate, Button, EventType};

use crate::context::Context;

fn req(context: Arc<Context>, button: Button) {
    if context.can_execute() {
        thread::sleep(context.get_min());

        simulate(&EventType::ButtonPress(button));
        context.execute();

        thread::sleep(context.get_min());
        dbg!(context.get_min());

        context.execute();
        simulate(&EventType::ButtonRelease(button));
    };
}

pub fn on_click(context: Arc<Context>, button: Button) {
    if context.is_enabled() {
        let context = context.clone();
        thread::spawn(move || req(context, button));
    }
}
