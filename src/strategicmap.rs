

pub mod map{
    #[derive(Component)]
    pub struct Camera{}

    use bevy::input::keyboard::KeyboardInput;
    use bevy::{prelude::*, sprite};
    //use std::thread::spawn;

    use bevy::prelude::Camera2d;
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
                    "Piniundze: 100", 
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
        });
        commands.spawn(SpriteBundle{
            texture: asset_server.load("mapa.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
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