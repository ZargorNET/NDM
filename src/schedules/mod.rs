pub use aww::fetch_aww;
pub use birb::fetch_birbs;
pub use dog::fetch_dogs;
pub use meme::fetch_memes;
pub use rabbit::fetch_rabbits;
pub use statistics::update_statistics;
pub use topgg_update::update_topgg;

mod meme;
mod dog;
mod birb;
mod rabbit;
mod aww;
mod statistics;
mod topgg_update;