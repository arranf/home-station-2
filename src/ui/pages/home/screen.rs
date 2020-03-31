use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

use anyhow::Result;
use conrod_core::widget::Canvas;
use conrod_core::Widget;

use lib_service::{ServicePoller, ServicePollerToken};
use lib_service_common::{Time, Weather};
use lib_ui_framework::{Screen, ScreenCreationContext, ScreenSettingContext};

use crate::State;

use self::widgets::*;

mod widgets;

// The time_token and weather_token are stored here but not used because when they are droppped `ServicePollerToken` sends a termination request to the `Poller`.
pub struct HomeScreen {
    ids: Ids,

    time: TimeWidget,
    _time_token: ServicePollerToken,

    weather: WeatherWidget,
    _weather_token: ServicePollerToken,

    events: Receiver<HomeEvent>,
}

#[derive(Debug)]
enum HomeEvent {
    UpdateTime(Time),
    UpdateWeather(Result<Weather>),
}

impl HomeScreen {
    pub fn new(ScreenCreationContext { state, ui, .. }: ScreenCreationContext<State>) -> Self {
        let (event_tx, event_rx) = channel();

        // @todo move all the ServicePoller logic to the Scheduler;
        //       thanks to this we'll be able to skip all the token fiddling and do something like
        //       scoped polling

        let time_token = ServicePoller::new(&state.time_client)
            .each(Duration::from_millis(500))
            .send(&event_tx, |time_client| {
                // @todo Figure out a sensible error handling pattern here. If the time fails we probably do want it to panic.
                HomeEvent::UpdateTime(time_client.get_time().expect("Failed to get time"))
            });

        let weather_token = ServicePoller::new(&state.weather_client)
            .each(Duration::from_secs(120)) // @todo Make this a config option
            .send(&event_tx, |weather_client| {
                HomeEvent::UpdateWeather(weather_client.get_weather())
            });

        Self {
            ids: Ids::new(ui.widget_id_generator()),

            time: TimeWidget::new(ui),
            _time_token: time_token,

            weather: WeatherWidget::new(ui),
            _weather_token: weather_token,

            events: event_rx,
        }
    }
}

impl Screen for HomeScreen {
    fn update(&mut self) {
        for event in self.events.try_iter() {
            match event {
                HomeEvent::UpdateTime(time) => {
                    self.time.update(time);
                }

                HomeEvent::UpdateWeather(weather) => {
                    match weather {
                        Ok(weather) => {
                            self.weather.update(weather);
                        }
                        Err(_) => {
                            // @todo Mark weather as being stale
                        }
                    }
                }
            }
        }
    }

    fn set(&self, mut ctx: ScreenSettingContext) {
        Canvas::new()
            .flow_down(&[
                (self.ids.time, Canvas::new()),
                (self.ids.weather, Canvas::new().length_weight(2.0)),
            ])
            .set(self.ids.main, ctx.ui);

        self.time.set(self.ids.time, &mut ctx);
        self.weather.set(self.ids.weather, &mut ctx);
    }
}

widget_ids! {
    struct Ids {
        main,
            time,
            weather,
    }
}
