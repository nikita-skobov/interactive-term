extern crate interactive_term;

use clap::App;
use yaml_rust::YamlLoader;

use interactive_term::styles;
use interactive_term::utils;
use interactive_term::interactive;

fn main() {
    println!("ex1 main:");
    let my_yaml_obj = YamlLoader::load_from_str(
        include_str!("cli.yml")
    ).unwrap();

    let my_styles = styles::get_styles_from_yaml(&my_yaml_obj[0]);

    let clap_matches = App::from_yaml(&my_yaml_obj[0])
        .get_matches()
        .clone();

    let mut my_list_items = utils::get_list_items_from_matches(
        &clap_matches,
        &my_styles,
    );

    interactive::interact();

    // match 

    println!("matches: {:?}", clap_matches);
}