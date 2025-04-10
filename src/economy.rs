pub mod economy{


    use bevy::prelude::*;
    use std::collections::HashMap;


    pub struct Company{
        name: String,
        market_id: i32,
        pub money: f32,
        
    }
    impl Company{
        fn add_money(&mut self, ammount:f32){
            self.money+=ammount;
        }
    }
    #[derive(Resource)]
    pub struct Companies{
        pub iteration:HashMap<i32,Company>
    }



    pub struct Commodity{
        name: String,
        market_id: i32,
        commodity_id: i32,
        price: f32,
        excise: f32,
        count:i32,
    }
    impl Commodity{
        fn change_price(&mut self, change:f32){
            self.price+=change;
        }
    }

    pub struct TradeRoute{
        route_id:i32,
        selling_market_id:i32,
        buying_market_id:i32,
        commodity_id:i32,
        commodities_count: f32,
    }
    pub struct CommodityType{
        type_id: i32,
    }

    fn create_new_route(last_route_id:i32,selling_market_id:i32, buying_market_id:i32, commodity_id:i32,commodities_count:f32)->TradeRoute{
        let route_id=last_route_id+1;
        let returned_route=TradeRoute{
            route_id: route_id,
            selling_market_id: selling_market_id,
            buying_market_id: buying_market_id,
            commodity_id: commodity_id,
            commodities_count: commodities_count
        };
        return returned_route;
    }

    fn create_new_commodity(last_commodity_id:i32,name:String, market_id:i32,price:f32,excise:f32,count:i32)->Commodity{
        let commodity_id=last_commodity_id+1;
        let returned_commodity=Commodity{
            name:name,
            market_id:market_id,
            commodity_id:commodity_id,
            price:price,
            excise:excise,
            count:count,
        };
        return returned_commodity;
    }

    pub fn main(){
        println!("economy test");
    }


    pub fn trade(mut companies: ResMut<Companies>){
        let england=Company{
            name: String::from("England"),
            market_id: 1,
            money:10000.0,
        };

        let netherlands=Company{
            name: String::from("Netherlands"),
            market_id: 2,
            money:10000.0,
        };

        let spain=Company{
            name: String::from("Spain"),
            market_id: 3,
            money:10000.0,
        };

        let france=Company{
            name: String::from("France"),
            market_id: 4,
            money:10000.0,
        };

        let portugal=Company{
            name: String::from("Portugal"),
            market_id:5,
            money:10000.0,
        };

        //companies{
        

        companies.iteration.insert(1,england);
        companies.iteration.insert(2,netherlands);
        companies.iteration.insert(3,spain);
        companies.iteration.insert(4,france);
        companies.iteration.insert(5,portugal);

        //}companies^^^^^^^^^^^^^^^^^^^^^^^^^


        //commodities{
            let mut commodities=HashMap::new();


        let mut i=1;


        while(i<=5){
            let new_commodity=create_new_commodity(0, String::from("steel"), i, 1000.0, 10.0, 100);
            commodities.insert(i,new_commodity);
            i+=1;
        }

        for(key,value)in &mut commodities{
            if(*key==1){
                value.change_price(-1.0);
            }
        }



        //}commodities^^^^^^^^^^^^^^^^^^^^^^^^^



        //trade routes{
        let mut routes=HashMap::new();
        let mut new_route;
        
        new_route=create_new_route(0,1,3,1,100.0);
        routes.insert(new_route.route_id,new_route);

        new_route=create_new_route(1,1,2,1,100.0);
        routes.insert(new_route.route_id,new_route);
        for(key,value) in &routes{
            let cid=value.commodity_id;
            let i1=value.selling_market_id;
            let i2=value.buying_market_id;
            let mut excise = 0.0;
            let count=value.commodities_count;
            let mut money_transfer=0.0;
            for(key,value) in & mut commodities{
                if(cid==value.commodity_id&&i1==value.market_id){
                    money_transfer=value.price;
                    value.count-=count as i32;
                    excise=value.excise;
                }
                if(cid==value.commodity_id&&i2==value.market_id){
                    value.count+=count as i32;
                }
                
            }
            money_transfer=count*(money_transfer+(money_transfer*(excise/100.0)));
            for(key,value) in & mut companies.iteration{
                if(i1==value.market_id){
                    value.add_money(money_transfer);
                    
                }
            }
            println!("market {} transfered {}$ -> market {}",i2,money_transfer,i1);
        }

        if let Some(company)=companies.iteration.get_mut(&1) {
            company.name = String::from("England");
        }
        for(key, value) in &companies.iteration{
            println!("{key}: {} ${}",value.name,value.money);
        }
    }


}