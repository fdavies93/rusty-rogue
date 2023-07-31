use crate::events::GameEvent;
use crate::game::GameManager;
use crate::systems::ui;

fn on_update(game: &mut GameManager, ev: &GameEvent) {
    ui::on_update(game, ev);
}
