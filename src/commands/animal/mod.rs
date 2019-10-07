use crate::command_framework::CommandArguments;

pub mod cat;
pub mod dog;
pub mod dog_cat_war;
pub mod fox;
pub mod birb;

const DOG_CAT_WAR_STORAGE_KEY: &'static str = "dogcatwar";

#[derive(Clone)]
struct DogCatWar {
    cat_sup: i32,
    dog_sup: i32,
}

fn add_dog_sup(args: &CommandArguments) {
    ensure_dogcatwar_exists(&args);
    let mut safe = args.safe.write();
    let dcwar = safe.get_mut::<DogCatWar>(DOG_CAT_WAR_STORAGE_KEY).unwrap();
    dcwar.dog_sup += 1;
}

fn add_cat_sup(args: &CommandArguments) {
    ensure_dogcatwar_exists(&args);
    let mut safe = args.safe.write();
    let dcwar = safe.get_mut::<DogCatWar>(DOG_CAT_WAR_STORAGE_KEY).unwrap();
    dcwar.cat_sup += 1;
}

fn get_dog_cat_sup(args: &CommandArguments) -> DogCatWar {
    ensure_dogcatwar_exists(&args);
    let safe = args.safe.read();
    let dcwar = safe.get::<DogCatWar>(DOG_CAT_WAR_STORAGE_KEY).unwrap();
    let dcwar = *dcwar.clone();
    dcwar
}

fn ensure_dogcatwar_exists(args: &CommandArguments) {
    {
        let safe = args.safe.read();
        if safe.exists(DOG_CAT_WAR_STORAGE_KEY) {
            return;
        }
    }
    let mut safe = args.safe.write();
    safe.store(DOG_CAT_WAR_STORAGE_KEY, DogCatWar { cat_sup: 0, dog_sup: 0 });
}