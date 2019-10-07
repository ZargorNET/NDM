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
                    error!("Could not fetch dog breeds: {}", e);
                    return;
                }
            };

            let res_text = match res.text() {
                Ok(k) => k,
                Err(e) => {
                    error!("Could not read body of dog breeds request: {}", e);
                    return;
                }
            };

            let dog_breeds: DogBreedsResponse = match serde_json::from_str(&res_text) {
                Ok(k) => k,
                Err(e) => {
                    error!("Could not parse body of dog breeds request: {}", e);
                    return;
                }
            };

            if dog_breeds.status != "success" {
                error!(r#"Dog breeds response is not "success""#);
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
                    error!("Could not fetch images for dog breed: {}: {}", &breed, e);
                    continue;
                }
            };

            let res_text = match res.text() {
                Ok(k) => k,
                Err(e) => {
                    error!("Could not read body of dog breed image request for breed: {}: {}", &breed, e);
                    continue;
                }
            };

            let breed_img: DogBreedImagesResponse = match serde_json::from_str(&res_text) {
                Ok(k) => k,
                Err(e) => {
                    error!("Could not parse json body of dog breed image request for breed: {}: {}", &breed, e);
                    continue;
                }
            };

            if breed_img.status != "success" {
                error!(r#"Dog Api responded not with "success" for breed: {}"#, &breed);
                continue;
            }

            info!("Fetched {} images for dog breed {}", breed_img.message.len(), &breed);
            ret.push(commands::animal::dog::DogBreed {
                name: breed,
                images: breed_img.message,
            });
        }
    }

    let mut safe = args.safe.write();
    safe.store(commands::animal::dog::DOG_CACHE_KEY, DogCache {
        breeds: ret
    });
    info!("Successfully updated dog cache!");
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
