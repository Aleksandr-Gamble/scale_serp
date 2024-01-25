use std::{env};
use std::vec::Vec;
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};
use serde_json;

pub struct Params {
    // The paramaters for making a call to ScaleSERP
    pub api_key: String, // your API key
    pub location: String, // "United+States" etc.
    pub q: String, // The query. Spaces are okay
}

impl Params {

    pub fn new_env(q: &str, location: &str) -> Self {
        // create a new Params object using an api_key from an environment variable
        let api_key = match env::var("SCALE_SERP_KEY") {
            Ok(val) => val,
            Err(_) => "".to_string(),
        };
        Params {
            api_key: api_key,
            location: location.to_string(),
            q: q.to_string()
        }
    }

    /// create a new Params object for a search from New York City
    /// Note searches are more likely to include ads if they are targeted to a specific geographic location
    pub fn new_env_nyc(q: &str) -> Self {
        Params::new_env(q, "New York,New York,United States")
    }

    /// create a new Params object for a search within the United States
    pub fn new_env_usa(q: &str) -> Self {
        Params::new_env(q, "United+States")
    }


    pub fn to_url(&self) -> String {
        // give the URL associated with these parameters
        format!("https://api.scaleserp.com/search?api_key={}&location={}&q={}", &self.api_key, &self.location, &self.q)
    }


}



#[derive(Deserialize, Debug)]
pub struct Resp {
    // This is the top-level object representing a response from ScaleSERP
    pub request_info: RequestInfo,
    pub search_metadata: SearchMetadata,
    pub search_parameters: SearchParameters,
    pub search_information: SearchInformation,
    pub ads: Option<Vec<Ad>>,
    //pub inline_images: String,
    pub top_stories: Option<Vec<TopStory>>, // will be None for 'uncommon' search terms
    //pub top_stories_extra: String,
    pub top_products: Option<Vec<TopProduct>>,
    //pub local_map: MOSTLY JUST B64 IMAGES,
    //pub local_results: MOSTLY JUST B64 IMAGES,,
    pub related_searches: Vec<RelatedSearch>,
    pub related_questions: Option<Vec<RelatedQuestion>>,
    //pub pagination: String,
    pub organic_results: Vec<OrganicResult>,
}

#[derive(Deserialize, Debug)]
pub struct RequestInfo {
    pub success: bool,
    pub credits_used: usize,
    pub credits_used_this_request: usize,
    pub credits_remaining: usize,
    pub credits_reset_at: String,  // i.e. '2021-07-31T01:00:37.000Z'
}

#[derive(Deserialize, Debug)]
pub struct SearchParameters {
    // this is basically your query parameters returned in the response
    pub location: String,
    pub q: String, //  your query string
}

#[derive(Deserialize, Debug)]
pub struct SearchMetadata {
    pub created_at: String,
    pub processed_at: String,
    pub total_time_taken: f64,
    pub engine_url: String,
    pub html_url: String,
    pub json_url: String,
    pub location_auto_message: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SearchInformation {
    pub original_query_yields_zero_results: bool,
    pub total_results: usize,
    pub time_taken_displayed: f64,
    pub query_displayed: String,
    pub detected_location: Option<String>,
}



#[derive(Deserialize, Debug)]
pub struct Ad {
    pub position: usize, 
    pub block_position: String,
    pub title: String,
    pub link: String,
    pub domain: String,
    pub displayed_link: String,
    pub description: String,
    pub sitelinks: Option<Vec<AdSitelink>>,
}

#[derive(Deserialize, Debug)]
pub struct AdSitelink {
    pub title: String,
    pub link: String,
}

#[derive(Deserialize, Debug)]
pub struct OrganicResult {
    pub position: usize,
    pub title: String,
    pub link: String,
    pub domain: String,
    pub displayed_link: String,
    pub snippet: String, 
    pub prerender: bool,
    pub snippet_matched: Option<Vec<String>>, 
    pub block_position: usize, 
}

#[derive(Deserialize, Debug)]
pub struct TopStory {
    pub link: String,
    pub title: String,
    pub visible_initially: bool,
    pub source: String,
    pub date: String,
    pub date_utc: String,
    pub block_position: usize,
}

#[derive(Deserialize, Debug)]
pub struct TopProduct {
    pub title: String,
    pub price: String,
    pub rating: f64,
    pub sources: Vec<TopProductSource>,
    pub specifications: Vec<TopProductSpecification>,
    pub block_position: usize,
}


#[derive(Deserialize, Debug)]
pub struct TopProductSource {
    pub name: String,
    pub link: String,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct TopProductSpecification {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct RelatedQuestion {
    pub question: String,
    pub answer: String,
    pub source: RelatedQuestionSource,
    pub block_position: usize,
}

#[derive(Deserialize, Debug)]
pub struct RelatedQuestionSource {
    pub link: String,
    pub displayed_link: String,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct RelatedSearch {
    pub query: String,
    pub link: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quick_demo() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async{
            let params = Params::new_env_nyc("anionic surfactants");
            let body: Resp = reqwest::get(&params.to_url())
            .await.unwrap()
            .json()
            .await.unwrap();
        
        println!("body = {:?}", body);
            
        });
    }
    #[test]
    fn search_with_top_products() {
        // This search should give some top products: no all searches do
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async{
            let params = Params::new_env_nyc("external ssd");
            let body: Resp = reqwest::get(&params.to_url())
            .await.unwrap()
            .json()
            .await.unwrap();
        
        println!("top_products = {:?}", body.top_products.unwrap());
        });
    }
    #[test]
    fn search_with_top_ads() {
        // This search should give some ads: no all searches do
        // NOTE: this test still seems to fail from time to time, perhaps something with the online bidding
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async{
            let params = Params::new_env("timeshare", "Chicago,Illinois,United+States");
            let body: Resp = reqwest::get(&params.to_url())
            .await.unwrap()
            .json()
            .await.unwrap();
        
        println!("ads = {:?}", body.ads.unwrap());
        });
    }
    #[test]
    fn common_and_uncommon() {
        // Some properties like top_stories appear for some searches but not for others
        // This test picks several phrases: some common, some obscure, some misspelled in an attempt to be robust against missing fields
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            for phrase in vec!["Taylor Swift", "how to prep to paint", "zwitterionic surfactant"].iter() {
                println!("testing '{}'", &phrase);
                let params = Params::new_env_nyc(phrase);
                let body: Resp = reqwest::get(&params.to_url())
                    .await.unwrap()
                    .json()
                    .await.unwrap();
                println!("body = {:?}", body);
            }
        });
    }
}