use std::error::Error;
use thirtyfour::prelude::*;
use serde_json::json;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use serde::{Serialize, Deserialize};
/**************************/
#[derive(Clone,Debug)]
enum AreaUnity {
    M2,
}

#[derive(Clone,Debug)]
enum PriceUnity {
    Million,
    Billion,
    MillionDinar, 
    Euros,
}
#[derive(Clone,Debug)]
struct Ad {
    pub location : String ,
    pub area : u32 ,
    pub areaUnity : AreaUnity ,
    pub price : u32 , 
    pub priceUnity : PriceUnity,
    pub nb_chambers : u32 , 
    pub nb_pieces : u32 , 
}
/******************************************************************************************/
/******************************************************************************************/
#[tokio::main]
pub async fn collect_ads(number_of_ads : usize) -> Result<Vec<Ad>, Box<dyn Error + Send + Sync>> {
    // using google chrome driver
    let caps = DesiredCapabilities::chrome();
    // connect to the driver local server
    let driver = WebDriver::new("http://localhost:41529", caps).await?;
    let mut collected_ads = Vec::<Ad>::new(); 
    // navigate to the wanted website
    driver.goto("https://.....").await?;
    /********Scroll until the end of the page *******************
    let counter = 0 ;
    while counter < 10 {
        driver.execute_script("window.scrollBy(0, 2000);", (&[]).to_vec()).await?;   
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
    ************************************************************/
    if let Ok(elements) = driver.find_all(By::ClassName("search-list-item-alt")).await {
        for (index, element) in elements.iter().enumerate() {
            let element1 = match element.find(By::ClassName("item-title")).await{
                Ok(element1_content)=> element1_content,
                Err(_)=>{
                    continue;
                }
            };
            /**********************************************************************/ 
            let description = match element.find(By::ClassName("item-description")).await{
                Ok(description_element) => description_element.text().await.unwrap_or_else(|_| "Unknown description".to_string()),
                Err(_) => {continue;},
            };
            /***********************************************************************/
            let location = match element1.find(By::ClassName("h1")).await{
                Ok(location_element) => location_element.text().await.unwrap_or_else(|_| "Unknown location".to_string()),
                Err(_) => {continue ;},
            }; 
            /************************************************************/
            let price = match element1.find(By::ClassName("item-price")).await{
                Ok(price_element) => price_element.text().await.unwrap_or_else(|_| "Unknown price".to_string()),
                Err(_) => {continue;},
            };  
            /************************************************************/
            let infos = element1.find_all(By::Tag("li")).await?;
            if(infos.len()!=3){
                continue ;
            }
            let nb_pieces =  &infos[0];
            let nb_chambres =  &infos[1];
            let surface =  &infos[2];
            //let text = element1.text().await?;
            println!("===> description : {}", description);
            let price_text = price.split_whitespace().next().ok_or("Empty input")?;
            let cleaned_input = price_text.split('.').collect::<String>();
            let price: u32 = cleaned_input.parse().map_err(|e| format!("Parse error: {}", e))?;
            println!("===> price : {}", price);
            /****************************************************************/
            let text = surface.text().await? ;
            let surface = text.split_whitespace().next().ok_or("Empty Input")?;
            let surface : u32 = surface.parse().map_err(|e| format!("Parse error: {}", e))?;
            println!("===> surface : {}", surface);
            /**************************************/
            let text = nb_chambres.text().await? ;
            let nb_chambres = text.split_whitespace().next().ok_or("Empty Input")?;
            let nb_chambres : u32 = nb_chambres.parse().map_err(|e| format!("Parse error: {}", e))?;
            println!("===> chambres : {}", nb_chambres);
            /**********************************/
            let text = nb_pieces.text().await?;
            let nb_pieces = text.split_whitespace().next().ok_or("Empty Input")?;
            let nb_pieces : u32 = nb_pieces.parse().map_err(|e| format!("Parse error: {}", e))?;
            println!("===> pieces : {}", nb_pieces);
            /********************************************************************/
            println!("===> location : {}", location);
            let ad1 = Ad{
                location : location,
                area : surface ,
                areaUnity : AreaUnity::M2,
                price : price,
                priceUnity : PriceUnity::Euros,
                nb_chambers : nb_chambres , 
                nb_pieces : nb_pieces 
            };
            collected_ads.push(ad1);
        }
        // call a function to store these ads in a json file 
    } 
    driver.quit().await?;
    Ok(collected_ads)  
}
/*******************************************************************/
pub fn store_ads(collected_ads : Vec<Ad>) -> std::io::Result<()>{
        let mut json_list = serde_json::Value::Array(vec![]);
        for (index, ad) in collected_ads.iter().enumerate() {
            let item = json! ({
                "location" : ad.location.clone(),
                "area" : ad.area,
                "price" : ad.price,
                "nb_pieces" : ad.nb_pieces,
            });
            if let serde_json::Value::Array(ref mut arr) = json_list {
                arr.push(item);
            }
        }
        let mut file = File::create("output.json")?;
        // Write the JSON data to the file in pretty format
        file.write_all(serde_json::to_string_pretty(&json_list)?.as_bytes())?;
        /*let mut file = File::create("stored_ads.json")?;
        // Serialize and write the struct to the file in pretty format
        to_writer_pretty(file, &collected_ads)?;*/
        Ok(())    
}
/******************************************************************************************/
//Box<dyn std::error::Error> type can represent 
//any error type that implements the std::error::Error 
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let collected_ads : Vec<Ad> = collect_ads(10).expect("Some error occured while collecting ads");
    store_ads(collected_ads)?;
    Ok(())
}