use conrod_core::Ui;

use crate::TaskDispatcher;

pub struct ScreenCreationContext<'sys, State> {
    pub state: &'sys mut State,
    pub task_dispatcher: TaskDispatcher,
    pub ui: &'sys mut Ui,
}
