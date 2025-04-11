pub mod ui{
    //use bevy::asset::Handle;
    use bevy::core_pipeline::deferred::node;
    use bevy::input::keyboard::{self, KeyboardInput};
    use bevy::log::tracing_subscriber::reload::Handle;
    use bevy::prelude::*;
    use bevy::prelude::Camera2d;
    use bevy::scene::ron::de;
    use bevy::transform::commands;
    use bevy::app::AppExit;

    use crate::economy::economy::Companies;

    #[derive(Component)]
    pub struct Camera{}

    #[derive(Component)]
    pub struct EscapeMenu;

    #[derive(Component)]
    pub struct ExitButton;

    #[derive(Component)]
    pub struct MarketButton;

    #[derive(Resource)]
    pub struct Escape{
        pub isclicked:bool
    }
    #[derive(Resource)]
    pub struct MarketMenu{
        pub isclicked:bool
    }
    




    
    pub fn ui_setup(mut commands:Commands, asset_server:Res<AssetServer>, companies: ResMut<Companies>){
        commands.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(15.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.2, 0.2, 0.2, 1.0)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(300.0),
                    height: Val::Px(300.0),
                    left: Val::Percent(-28.5),
                    ..default()
                },
                image: asset_server.load("piniundze.png").into(),
                ..default()
            });
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    format!("${}", companies.iteration.get(&1).map_or(0.0, |company| company.money)), 
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
                style: Style{
                    position_type: PositionType::Absolute,
                    left: Val::Percent((23.0)),
                    ..default()
                },
                ..default()
            });
        });

        commands.spawn(NodeBundle{
            style: Style {
                width: Val::Percent(10.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.2, 0.2, 0.2, 1.0)),
            ..default()
        }).with_children(|parent|{
            parent.spawn((ButtonBundle{
                style: Style {
                    top: Val::Percent(15.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(9.0),
                    ..default()
                },
                ..default()
            },MarketButton)).with_children(|parent|{
                parent.spawn(ImageBundle{
                    image: asset_server.load("uicargobox.png").into(),
                    style: Style {
                        ..default()
                    },
                    ..default()
                });
            });
        });

    }


    pub fn esc(keyboard_input:Res<ButtonInput<KeyCode>>, mut escape:ResMut<Escape>, mut exit: EventWriter<AppExit>){
        if(keyboard_input.just_pressed(KeyCode::Escape)){
            escape.isclicked = !escape.isclicked;
        }
    }


    pub fn escmenu(mut commands:Commands, asset_server:Res<AssetServer>, escape:Res<Escape>, mut exit: EventWriter<AppExit>, query: Query<Entity, With<EscapeMenu>>,){
        if escape.isclicked {
            if query.is_empty() {
                commands.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(30.0),
                            height: Val::Percent(60.0),
                            position_type: PositionType::Relative,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            justify_self: JustifySelf::Center,
                            align_self: AlignSelf::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.9)),
                        ..default()
                    },
                    EscapeMenu,
                )).with_children(|parent| {
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Percent(40.0),
                                height: Val::Percent(10.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                align_self: AlignSelf::FlexEnd,
                                margin: UiRect { 
                                    left: (Val::Percent(5.0)),
                                    right: (Val::Percent(5.0)), 
                                    top: (Val::Percent(5.0)), 
                                    bottom: (Val::Percent(5.0))
                                },
                                ..default()
                            },
                            background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
                            ..default()
                        },
                        ExitButton,
                    ))
                    .with_children(|button| {
                        button.spawn(TextBundle::from_section(
                            "Exit to main menu",
                            TextStyle {
                                font_size: 25.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ));
                    });
                });
            }
        } else {
            for entity in query.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    pub fn exit_system(
        mut exit: EventWriter<AppExit>,
        mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<ExitButton>)>,
    ) {
        for (interaction, mut color) in &mut query {
            match *interaction {
                Interaction::Pressed => {
                    println!("Program ended succesfully");
                    exit.send(AppExit::Success);
                }
                Interaction::Hovered => {
                    *color = BackgroundColor(Color::rgb(0.3, 0.3, 0.3));
                }
                Interaction::None => {
                    *color = BackgroundColor(Color::rgb(0.2, 0.2, 0.2));
                }
            }
        }
    }

    fn market_button(
        mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<MarketButton>)>,
    ) {
        for (interaction, mut color) in interaction_query.iter_mut() {
            match *interaction {
                Interaction::Pressed => {
                    *color = BackgroundColor(Color::rgb(0.3, 0.3, 0.3));
                    println!("Kliknięto przycisk!");
                }
                Interaction::Hovered => {
                    *color = BackgroundColor(Color::rgb(0.6, 0.6, 0.6));
                }
                Interaction::None => {
                    *color = BackgroundColor(Color::rgb(0.2, 0.2, 0.2));
                }
            }
        }
    }
}



