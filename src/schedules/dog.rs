use std::collections::HashMap;

use crate::commands;
use crate::commands::animal::dog::DogCache;
use crate::scheduler::ScheduleArguments;

pub fn fetch_dogs(args: ScheduleArguments) {
    let mut ret: Vec<commands::animal::dog::DogBreed> = Vec::new();

    {
        // GET ALL BREEDS
        let mut breeds: Vec<String> = Vec::new(); // DOG BREED NAME

        {
            let mut res = match reqwest::get("https://dog.ceo/api/breeds/list/all") {
                Ok(k) => k,
                Err(e) => {
                    error!("DOG SCHEDULER: Could not fetch dog breeds: {}", e);
                    return;
                }
            };

            let res_text = match res.text() {
                Ok(k) => k,
                Err(e) => {
                    error!("DOG SCHEDULER: Could not read body of dog breeds request: {}", e);
                    return;
                }
            };

            let dog_breeds: DogBreedsResponse = match serde_json::from_str(&res_text) {
                Ok(k) => k,
                Err(e) => {
                    error!("DOG SCHEDULER: Could not parse body of dog breeds request: {}", e);
                    return;
                }
            };

            if dog_breeds.status != "success" {
                error!(r#"DOG SCHEDULER: Dog breeds response is not "success""#);
                return;
            }

            for (key, _) in dog_breeds.message.into_iter() {
                breeds.push(key);
            }
        }

        // GET IMAGES TO BREEDS

        let http_client = reqwest::Client::new();
        for breed in breeds.into_iter() {
            let mut res = match http_client.execute(http_client.get(&format!("https://dog.ceo/api/breed/{}/images", &breed)).build().unwrap()) {
                Ok(k) => k,
                Err(e) => {
                    error!("DOG SCHEDULER: Could not fetch images for dog breed: {}: {}", &breed, e);
                    continue;
                }
            };

            let res_text = match res.text() {
                Ok(k) => k,
                Err(e) => {
                    error!("DOG SCHEDULER: Could not read body of dog breed image request for breed: {}: {}", &breed, e);
                    continue;
                }
            };

            let breed_img: DogBreedImagesResponse = match serde_json::from_str(&res_text) {
                Ok(k) => k,
                Err(e) => {
                    error!("DOG SCHEDULER: Could not parse json body of dog breed image request for breed: {}: {}", &breed, e);
                    continue;
                }
            };

            if breed_img.status != "success" {
                error!(r#"DOG SCHEDULER: Dog Api responded not with "success" for breed: {}"#, &breed);
                continue;
            }

            ret.push(commands::animal::dog::DogBreed {
                name: breed,
                images: breed_img.message,
            });
        }
    }
    ret.shrink_to_fit();

    let mut dogs = 0usize;
    for d in ret.iter().map(|d| d.images.len()) {
        dogs += d;
    }

    info!("DOG SCHEDULER: Fetched {} images for {} dog breeds", dogs, ret.len());
    let mut safe = args.safe.write();
    safe.store(DogCache {
        breeds: ret
    });
    info!("DOG SCHEDULER: Successfully updated dog cache!");
}

#[derive(Serialize, Deserialize)]
struct DogBreedsResponse {
    status: String,
    message: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
struct DogBreedImagesResponse {
    status: String,
    message: Vec<String>,
}
