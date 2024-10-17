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
}