use bevy::{
    app::{Plugin, PreStartup},
    asset::{Assets, Handle},
    pbr::StandardMaterial,
    prelude::{Commands, ResMut, Resource},
    render::color::Color,
};

pub struct MaterialHandlesPlugin;

impl Plugin for MaterialHandlesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreStartup, setup);
    }
}

fn setup(mut commands: Commands, materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(MaterialHandles::new(materials));
}

#[derive(Resource, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MaterialHandles {
    pub alice_blue: Handle<StandardMaterial>,
    pub antique_white: Handle<StandardMaterial>,
    pub aquamarine: Handle<StandardMaterial>,
    pub azure: Handle<StandardMaterial>,
    pub beige: Handle<StandardMaterial>,
    pub bisque: Handle<StandardMaterial>,
    pub black: Handle<StandardMaterial>,
    pub blue: Handle<StandardMaterial>,
    pub crimson: Handle<StandardMaterial>,
    pub cyan: Handle<StandardMaterial>,
    pub dark_gray: Handle<StandardMaterial>,
    pub dark_green: Handle<StandardMaterial>,
    pub fuchsia: Handle<StandardMaterial>,
    pub gold: Handle<StandardMaterial>,
    pub gray: Handle<StandardMaterial>,
    pub green: Handle<StandardMaterial>,
    pub indigo: Handle<StandardMaterial>,
    pub lime_green: Handle<StandardMaterial>,
    pub maroon: Handle<StandardMaterial>,
    pub midnight_blue: Handle<StandardMaterial>,
    pub navy: Handle<StandardMaterial>,
    pub none: Handle<StandardMaterial>,
    pub olive: Handle<StandardMaterial>,
    pub orange: Handle<StandardMaterial>,
    pub orange_red: Handle<StandardMaterial>,
    pub pink: Handle<StandardMaterial>,
    pub purple: Handle<StandardMaterial>,
    pub red: Handle<StandardMaterial>,
    pub salmon: Handle<StandardMaterial>,
    pub sea_green: Handle<StandardMaterial>,
    pub silver: Handle<StandardMaterial>,
    pub teal: Handle<StandardMaterial>,
    pub tomato: Handle<StandardMaterial>,
    pub turquoise: Handle<StandardMaterial>,
    pub violet: Handle<StandardMaterial>,
    pub white: Handle<StandardMaterial>,
    pub yellow: Handle<StandardMaterial>,
    pub yellow_green: Handle<StandardMaterial>,
}

impl MaterialHandles {
    pub fn new(mut materials: ResMut<Assets<StandardMaterial>>) -> Self {
        Self {
            black: materials.add(Color::BLACK),
            alice_blue: materials.add(Color::ALICE_BLUE),
            antique_white: materials.add(Color::ANTIQUE_WHITE),
            aquamarine: materials.add(Color::AQUAMARINE),
            azure: materials.add(Color::AZURE),
            beige: materials.add(Color::BEIGE),
            bisque: materials.add(Color::BISQUE),
            blue: materials.add(Color::BLUE),
            crimson: materials.add(Color::CRIMSON),
            cyan: materials.add(Color::CYAN),
            dark_gray: materials.add(Color::DARK_GRAY),
            dark_green: materials.add(Color::DARK_GREEN),
            fuchsia: materials.add(Color::FUCHSIA),
            gold: materials.add(Color::GOLD),
            gray: materials.add(Color::GRAY),
            green: materials.add(Color::GREEN),
            indigo: materials.add(Color::INDIGO),
            lime_green: materials.add(Color::LIME_GREEN),
            maroon: materials.add(Color::MAROON),
            midnight_blue: materials.add(Color::MIDNIGHT_BLUE),
            navy: materials.add(Color::NAVY),
            none: materials.add(Color::NONE),
            olive: materials.add(Color::OLIVE),
            orange: materials.add(Color::ORANGE),
            orange_red: materials.add(Color::ORANGE_RED),
            pink: materials.add(Color::PINK),
            purple: materials.add(Color::PURPLE),
            red: materials.add(Color::RED),
            salmon: materials.add(Color::SALMON),
            sea_green: materials.add(Color::SEA_GREEN),
            silver: materials.add(Color::SILVER),
            teal: materials.add(Color::TEAL),
            tomato: materials.add(Color::TOMATO),
            turquoise: materials.add(Color::TURQUOISE),
            violet: materials.add(Color::VIOLET),
            white: materials.add(Color::WHITE),
            yellow: materials.add(Color::YELLOW),
            yellow_green: materials.add(Color::YELLOW_GREEN),
        }
    }

    pub fn black(&self) -> &Handle<StandardMaterial> {
        &self.black
    }

    pub fn alice_blue(&self) -> &Handle<StandardMaterial> {
        &self.alice_blue
    }

    pub fn antique_white(&self) -> &Handle<StandardMaterial> {
        &self.antique_white
    }

    pub fn aquamarine(&self) -> &Handle<StandardMaterial> {
        &self.aquamarine
    }

    pub fn azure(&self) -> &Handle<StandardMaterial> {
        &self.azure
    }

    pub fn beige(&self) -> &Handle<StandardMaterial> {
        &self.beige
    }

    pub fn bisque(&self) -> &Handle<StandardMaterial> {
        &self.bisque
    }

    pub fn blue(&self) -> &Handle<StandardMaterial> {
        &self.blue
    }

    pub fn crimson(&self) -> &Handle<StandardMaterial> {
        &self.crimson
    }

    pub fn cyan(&self) -> &Handle<StandardMaterial> {
        &self.cyan
    }

    pub fn dark_gray(&self) -> &Handle<StandardMaterial> {
        &self.dark_gray
    }

    pub fn dark_green(&self) -> &Handle<StandardMaterial> {
        &self.dark_green
    }

    pub fn fuchsia(&self) -> &Handle<StandardMaterial> {
        &self.fuchsia
    }

    pub fn gold(&self) -> &Handle<StandardMaterial> {
        &self.gold
    }

    pub fn gray(&self) -> &Handle<StandardMaterial> {
        &self.gray
    }

    pub fn green(&self) -> &Handle<StandardMaterial> {
        &self.green
    }

    pub fn indigo(&self) -> &Handle<StandardMaterial> {
        &self.indigo
    }

    pub fn lime_green(&self) -> &Handle<StandardMaterial> {
        &self.lime_green
    }

    pub fn maroon(&self) -> &Handle<StandardMaterial> {
        &self.maroon
    }

    pub fn midnight_blue(&self) -> &Handle<StandardMaterial> {
        &self.midnight_blue
    }

    pub fn navy(&self) -> &Handle<StandardMaterial> {
        &self.navy
    }

    pub fn none(&self) -> &Handle<StandardMaterial> {
        &self.none
    }

    pub fn olive(&self) -> &Handle<StandardMaterial> {
        &self.olive
    }

    pub fn orange(&self) -> &Handle<StandardMaterial> {
        &self.orange
    }

    pub fn orange_red(&self) -> &Handle<StandardMaterial> {
        &self.orange_red
    }

    pub fn pink(&self) -> &Handle<StandardMaterial> {
        &self.pink
    }

    pub fn purple(&self) -> &Handle<StandardMaterial> {
        &self.purple
    }

    pub fn red(&self) -> &Handle<StandardMaterial> {
        &self.red
    }

    pub fn salmon(&self) -> &Handle<StandardMaterial> {
        &self.salmon
    }

    pub fn sea_green(&self) -> &Handle<StandardMaterial> {
        &self.sea_green
    }

    pub fn silver(&self) -> &Handle<StandardMaterial> {
        &self.silver
    }

    pub fn teal(&self) -> &Handle<StandardMaterial> {
        &self.teal
    }

    pub fn tomato(&self) -> &Handle<StandardMaterial> {
        &self.tomato
    }

    pub fn turquoise(&self) -> &Handle<StandardMaterial> {
        &self.turquoise
    }

    pub fn violet(&self) -> &Handle<StandardMaterial> {
        &self.violet
    }

    pub fn white(&self) -> &Handle<StandardMaterial> {
        &self.white
    }

    pub fn yellow(&self) -> &Handle<StandardMaterial> {
        &self.yellow
    }

    pub fn yellow_green(&self) -> &Handle<StandardMaterial> {
        &self.yellow_green
    }
}
