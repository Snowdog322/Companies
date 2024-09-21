mod economy;
mod strategicmap;

use strategicmap::map as mapmod;
use economy::economy as economymod;
//use bevy::prelude::*;


fn main() {
    economymod::main();
    mapmod::print();
}