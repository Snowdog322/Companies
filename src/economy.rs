pub mod economy{



    //use bevy::prelude::*;
    use std::collections::HashMap;


    pub struct Company{
        name: String,
        market_id: i32,
    }
    pub struct Commodity{
        name: String,
        market_id: i32,
        commodity_id: i32,
        price: f32,
        excise: f32,
    }

        

    pub fn main(){
        let england=Company{
            name: String::from("England"),
            market_id: 1,
        };

        let netherlands=Company{
            name: String::from("Netherlands"),
            market_id: 2,
        };

        let spain=Company{
            name: String::from("Spain"),
            market_id: 3,
        };



        let english_steel=Commodity{
            name: String::from("steel"),
            market_id: 1,
            commodity_id: 1,
            price: 1.0,
            excise: 10.0,
        };


        //companies{
        let mut companies=HashMap::new();

        companies.insert(1,england);
        companies.insert(2,netherlands);
        companies.insert(3,spain);

        //}companies
        


        //commodities{
        let mut commodities=HashMap::new();

        commodities.insert(1,english_steel);
        //}commodities
        

        if let Some(company)=companies.get_mut(&1) {
            company.name = String::from("England");
        }
        for(key, value) in &companies{
            println!("{key}: {}",value.name);
        }



        println!("economy test");
    }



}