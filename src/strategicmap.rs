//use bevy::prelude::*;

pub mod map{

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


    pub fn main(){
        println!("map test");
    }
}