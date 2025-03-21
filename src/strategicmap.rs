

pub mod map{
    #[derive(Component)]
    pub struct Camera{}

    use bevy::input::keyboard::KeyboardInput;
    use bevy::{prelude::*, sprite};
    //use std::thread::spawn;

    use bevy::prelude::Camera2d;
    use bevy_egui::egui::Rgba;
    const PLAYER_SPEED: f32 = 350.;


    pub struct Coordinates{
        x: f32,
        y: f32,
    }
    
    pub struct Province{
        province_id: i32,
        central_coordinates: Coordinates,
    }

    pub struct Port{
        name: String,
        port_id: i32,
        port_coordinates: Coordinates,
    }

    pub struct Ship{
        ship_id:i32,
        cannons:i32,
        speed:i32,
        company_id:i32,
        province_id:i32,
        port_id:i32,
        
    }


    pub fn main(){
        println!("map test");
    }

    pub fn setup(mut commands:Commands, asset_server:Res<AssetServer>){
        commands
        .spawn(Camera2dBundle {
            transform: Transform::from_xyz(100.0, 200.0, 0.0),
            ..default()
        }).insert(Camera{});
        
        commands.spawn(SpriteBundle{
            texture: asset_server.load("mapa.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        });
        commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::BLACK),
            style: Style {
                width: Val::Percent(100.0), 
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0), 
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..default()
                })
                .with_children(|button| {
                    button.spawn(TextBundle {
                        text: Text::from_section(
                            "Kliknij mnie",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    });
                });
        });
        println!("sigma");
    }

    pub fn movement(keyboard_input:Res<ButtonInput<KeyCode>>, mut movement:Query<&mut Transform,With<Camera>>,time: Res<Time>){
        let Ok(mut movement) = movement.get_single_mut() else {
            return;
        };
        let mut shift=1.0;
        let mut direction = Vec2::ZERO;
        if(keyboard_input.pressed(KeyCode::KeyW)){
            direction.y+=1.;
        }
        if(keyboard_input.pressed(KeyCode::KeyS)){
            direction.y-=1.;
        }
        if(keyboard_input.pressed(KeyCode::KeyD)){
            direction.x+=1.;
        }
        if(keyboard_input.pressed(KeyCode::KeyA)){
            direction.x-=1.;
        }
        if(keyboard_input.pressed(KeyCode::ShiftLeft)){
            shift=3.0;
        }
        let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * shift * time.delta_seconds();
        movement.translation += move_delta.extend(0.);
    
    }
}