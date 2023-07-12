use leptos::{create_local_resource, log, Resource, Scope};

use self::api_model::{
    Background, BackgroundsAPI, Class, ClassesAPI, Species, SpeciesAPI,
};

pub mod api_extensions;
pub mod api_model;

#[derive(Clone, Copy)]
pub struct FuturesWrapper {
    pub classes: Resource<(), Vec<Class>>,
    pub species: Resource<(), Vec<Species>>,
    pub backgrounds: Resource<(), Vec<Background>>,
}
impl FuturesWrapper {
    pub fn new(cx: Scope) -> Self {
        Self {
            classes: create_local_resource(cx, || (), fetch_classes),
            species: create_local_resource(cx, || (), fetch_species),
            backgrounds: create_local_resource(cx, || (), fetch_backgrounds),
        }
    }
}

/// Fetch list of species options from Open5e
pub async fn fetch_species(_: ()) -> Vec<Species> {
    let res = reqwasm::http::Request::get("https://api.open5e.com/v1/races/")
        .send()
        .await;
    match res {
        Ok(response) => match response.json::<SpeciesAPI>().await {
            Ok(api) => api.results,
            // Handle deserialization error condition
            Err(e) => {
                log!("Could not deserialize data from Open5e to the SpeciesAPI struct!");
                log!("{}", e);
                vec![]
            }
        },
        // If our request errors, return an empty list
        Err(e) => {
            log!("Error fetching species data from Open5e!");
            log!("{}", e);
            vec![]
        }
    }
}

/// Fetch list of class options from Open5e
pub async fn fetch_classes(_: ()) -> Vec<Class> {
    let res = reqwasm::http::Request::get("https://api.open5e.com/v1/classes/")
        .send()
        .await;
    match res {
        Ok(response) => match response.json::<ClassesAPI>().await {
            Ok(api) => api.results,
            // Handle deserialization error condition
            Err(e) => {
                log!("Could not deserialize data from Open5e to the ClassAPI struct!");
                log!("{}", e);
                vec![]
            }
        },
        // If our request errors, return an empty list
        Err(e) => {
            log!("Error fetching class data from Open5e!");
            log!("{}", e);
            vec![]
        }
    }
}

/// Fetch list of background options from Open5e
pub async fn fetch_backgrounds(_: ()) -> Vec<Background> {
    let res =
        reqwasm::http::Request::get("https://api.open5e.com/v1/backgrounds/")
            .send()
            .await;
    match res {
        Ok(response) => match response.json::<BackgroundsAPI>().await {
            Ok(api) => api
                .results
                .iter()
                .filter(|b| b.document_slug != "a5e")
                .cloned()
                .collect::<Vec<Background>>(),
            // Handle deserialization error condition
            Err(e) => {
                log!("Could not deserialize data from Open5e to the BackgroundAPI struct!");
                log!("{}", e);
                vec![]
            }
        },
        // If our request errors, return an empty list
        Err(e) => {
            log!("Error fetching background data from Open5e!");
            log!("{}", e);
            vec![]
        }
    }
}