extern crate lilv;

fn main() {
    let world = lilv::World::new();
    world.load_all();
    let plugins = world.get_all_plugins();

    for plugin in plugins {
    	println!("{:?}", plugin.get_uri().as_uri());
    }
}