use crate::plugins::interact_with_button;
use crate::plugins::prefs::layout::*;
use crate::plugins::prefs::*;
use crate::prefs::Prefs;

pub fn spawn_prefs_dialog(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    prefs: Res<Prefs>,
) {
    build_prefs_dialog(&mut commands, &asset_server, &prefs);
}

pub fn interact_with_checkbox(
    mut query: Query<(&Interaction, &mut BackgroundColor, &Checkbox), Changed<Interaction>>,
) {
    for (interaction, mut background_color, Checkbox(selected)) in query.iter_mut() {
        interact_with_button(interaction, &mut background_color, *selected);
    }
}
