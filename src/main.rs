/*****************************************
  Project   :   CS410 Final Project
  Written by:   Hassan Kaboyo
******************************************/


extern crate rand;
extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate hyper;

//use std::io::prelude::*;
//use std::fs::File;
use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use rustc_serialize::json;

use std::collections::HashMap;
use rand::Rng;
use iron::headers;
use std::io::Read;
use std::sync::{Mutex, Arc};

extern crate staticfile;
extern crate mount;
extern crate http;
extern crate hyper_staticfile;

use staticfile::Static;
use mount::Mount;



//Handles default request to just local host, Input is just the
//server request, Output is Iron Result
fn root_handler( _req: &mut Request) -> IronResult<Response> {
    let content_type = "text/plain".parse::<Mime>().unwrap();
    let direction = "\n\nUse the link below to access my Web Application:\r\n
    				\r\n  http://localhost:8080/home/GameProject.html";
    let mut resp = Response::with((content_type, status::Ok, direction));
    resp.headers.set(headers::AccessControlAllowOrigin::Any);
    Ok(resp)
}


//Handle Spanish request whrere Inputs are, hashmap (as a shared resouurce amongst middleware)
//that is populated with randomly selected key-value pairs, two vectors for spanish words and
//their transltions. Output the Iron Result
fn spanish_handler(correct: &Arc<Mutex<HashMap<String, String>>>, spanish_word: &Vec<&str>, 
    spanglish: &Vec<&str>, _req: &mut Request) -> IronResult<Response> {

    //Obtaining the lock to the shared resource
    let mut spanish_qsn = correct.lock().unwrap();

    *spanish_qsn = create_randomly_5pairs(spanish_word, spanglish);
    print!("\nReturn From rand 5 pairs:\n\n{:?}\n", *spanish_qsn);
    let user_sp_qn = randomise_values(&spanish_qsn.clone());
    print!("\nReturn From randomise_answers:\n\n{:?}\n", user_sp_qn);

    //Setting up the client/front-end payload
    let content_type = "text/plain".parse::<Mime>().unwrap();
    let out_data = json::encode(&user_sp_qn).unwrap();
    let mut resp = Response::with((content_type, status::Ok, out_data));
    resp.headers.set(headers::AccessControlAllowOrigin::Any);
    Ok(resp)
}


//Handle Swahili request whrere Inputs are, hashmap (as a shared resouurce amongst middleware)
//that is populated with randomly selected key-value pairs, two vectors for swahili words and
//their transltions. Output the Iron Result
fn swahili_handler(correct: &Arc<Mutex<HashMap<String, String>>>, swahili_word: &Vec<&str>, 
    swanglish: &Vec<&str>, _req: &mut Request) -> IronResult<Response> {

    //Obtaining the lock to the shared resource
    let mut swahili_qsn = correct.lock().unwrap();  

    *swahili_qsn = create_randomly_5pairs(swahili_word, swanglish);
    print!("\nReturn From rand 5 pairs:\n\n{:?}\n", *swahili_qsn);
    let user_sw_qn = randomise_values(&swahili_qsn.clone());
    print!("\nReturn From randomise_answers:\n\n{:?}\n", user_sw_qn);

    //Setting up the client/front-end payload
    let content_type = "text/plain".parse::<Mime>().unwrap();
    let out_data = json::encode(&user_sw_qn).unwrap();
    let mut resp = Response::with((content_type, status::Ok, out_data));
    resp.headers.set(headers::AccessControlAllowOrigin::Any);
    Ok(resp)
}


//Uses a previously populated hashmap(correct) by either swahili handler or sapanish 
//handler together with a hashmap put together from fron-end as user's ans sent in
//via request, then calls compare_result func that returns a vector contains user score
fn post_handler(correct: &Arc<Mutex<HashMap<String, String>>>, req: &mut Request) 
-> IronResult<Response> {

    //Decoding user request payload into json object (hashmap)
    let mut payload = String::new();
    req.body.read_to_string(&mut payload).expect("Failed to read request body");
    let payload_hm: HashMap<String, String> = json::decode(&payload).unwrap();
    println!("\nPost Handler Received:\n{:?}\n", payload_hm);

    let correct_ans = correct.lock().unwrap();  
    let score = compare_result(&payload_hm, &correct_ans);  

    //Prints on the console for debuging purposes
    for key in payload_hm.keys() { println!("Payload value: {}\n", payload_hm[key]); }

    //Encodes the vector returned from compare result(...) and post it
    let content_type = "application/json".parse::<Mime>().expect("Failed to parse mime type");   
    let out_data = json::encode(&score).unwrap();                     
    let mut resp = Response::with((content_type, status::Ok, out_data));
    resp.headers.set(headers::AccessControlAllowOrigin::Any);

    Ok(resp)
}


//Randomly selects five key value pairs and returns a hashmap of 
//5 items randomly picked from each of the passed in vectors. Both 
//vectors must have same length.
fn create_randomly_5pairs(word: &Vec<&str>, meanings: &Vec<&str>) 
-> HashMap<String, String> {

    //let mut rand_num : usize = 0;
    let mut key_val_pairs = HashMap::new();
    let mut item_tracker : Vec<usize> = Vec::new(); //Stores unique items
    let mut found = 0;   //Flag to signal repeated random number

    //Make sure both vectrors have the same length
    assert_eq!(word.len(), meanings.len());

    while item_tracker.len() < (5 as usize) {
        //let rand_num : usize = rand::thread_rng().gen_range(0, 10);
        let rand_num : usize = rand::thread_rng().gen_range(0, word.len());

        if item_tracker.len() == 0 {
            item_tracker.push(rand_num);
            key_val_pairs.insert(word[rand_num].to_string(), 
                                    meanings[rand_num].to_string());
        } else {
            for index in 0..item_tracker.len() {
                //found flag prevent same word to be chosen more than once
                if item_tracker[index] == rand_num { 
                    found = 1; 
                }
            }

            if found == 0 {
                item_tracker.push(rand_num);
                key_val_pairs.insert(word[rand_num].to_string(), 
                                        meanings[rand_num].to_string());
            }
            found = 0;  //Reset the flag
        }
    }

    key_val_pairs
}



//Takes a hashmap and scrambles the values so they
//can be re matched with their keys as user's exersice
fn randomise_values(inputmap : &HashMap<String, String>) 
-> HashMap<String, String> {

    let mut words : Vec<String> = Vec::new();
    let mut meanings : Vec<String> = Vec::new();
    let mut temp_user_map = HashMap::new();

    for (key, val) in inputmap {
        //println!("{:?} : {:?} ", key, val);
        words.push(key.to_string());
        meanings.push(val.to_string());
    }

    for index in 0..meanings.len() {
        let rand_num : usize = rand::thread_rng().gen_range(0, 5);
        if rand_num != index {
            meanings.swap(index, rand_num);
        }
    } 

    //Put Key and scambled values back together
    for index in 0..words.len() {
        temp_user_map.insert(words[index].to_string(), 
                            meanings[index].to_string());
    }
    
    temp_user_map
}


//Compares user Answer brought in from front end as a request payload with 
//a correct ans stored as hashmap then return feedback as vector via response
fn compare_result(userhm: &HashMap<String, String>, answerhm: &HashMap<String, String>) 
-> Vec<String> {

    let mut score_result : Vec<String> = Vec::new();

    let mut points = 0;
    let mut feedback = String::from("Success for Words : ");

    //Iterates the asnwer hashmap keys then use the same keys to retrive
    //values from both hashmaps and check if values are equal.
    for key in answerhm.keys() {
        if answerhm[key].to_lowercase() == userhm[key].to_lowercase()  {
            points +=  1;
            //feedback.push_str(&(userhm[key].clone() + ",  ").clone());   
            feedback.push_str(&(key.clone() + ",  ").clone());  
        }
    }

    let earned_pts = (points as f64 /5.0) * 100.0;

    println!("\nCorrectly Matched : \n\n{:?}", feedback);
    println!("\nPoints Earned : {:?}\n", earned_pts);

    score_result.push(feedback);
    let mut temp = earned_pts.to_string();
    temp.push('%');
    score_result.push(temp);
    println!("\nScore Result : {:?}\n", score_result);

    score_result
}



//Main function that 
//creates the server object
fn main() {

	//Holds original hashmap of 5 correctly matched key-value pairs 
    let correct_ans : HashMap<String, String> = HashMap::new();
    let correct_ans_ref = Arc::new(Mutex::new(correct_ans));

    //Swahili words vector with their English translation vector a line below.
    //These vectors can be extended to contain even more words, as long as their
    //lengths are kept equal since the function that builds HashMap uses their length
    //to control the range of the random number generator. However both can not contain less than five items since the 
    //front end expect five items.
    let swahili_word = vec!["Chungwa", "Gari", "Sahani", "Nyumba", "Karatasi", "Ndege", "Sea", "Raisi", "Mchina", "Tumbo"];
    let swanglish = vec!["Orange", "Car", "Plate", "House", "Paper", "Airplane", "Ocean", "President", "Chinese", "Stomach"];
  
    //Spanish words vector with their English translation vector a line below
    let spanish_word = vec!["Coche", "Mesa", "Casa", "Camisa", "Caballo", "Hombre", "Estufa", "Agua", "Mismo", "Hermana"]; 
    let spanglish = vec!["Car", "Table", "House", "Shirt", "Horse", "Man", "Stove", "Water", "Same", "Sister"];
    

    //A server with routers/middeleware
    let mut router = Router::new();

    router.get("/", root_handler, "index");

    let correct_ref =  Arc::clone(&correct_ans_ref);
    router.get("/spanish", move |req: &mut Request| 
    	spanish_handler(&correct_ref.clone(), &spanish_word, &spanglish, req), "spanish");

    let correct_ref =  Arc::clone(&correct_ans_ref);
    router.get("/swahili", move |req: &mut Request| 
    	swahili_handler(&correct_ref.clone(), &swahili_word, &swanglish, req), "swahili");

    let correct_ref =  Arc::clone(&correct_ans_ref);
    router.post("/scores", move |req: &mut Request| 
    	post_handler(&correct_ref.clone(), req), "scores");

    println!("\nListening on port 8080\n");

    let mut file_mount = Mount::new();

    file_mount.mount("/home/", Static::new("pages/"));
    file_mount.mount("/", router);

    let middeware_chain  = Chain::new(file_mount);

    Iron::new(middeware_chain).http("127.0.0.1:8080").expect("Error");
    //Iron::new(middeware_chain).http("172.19.1.67:8080").unwrap();
}




/*********************************************************************
//              Testing for the data processing functions
**********************************************************************/
#[cfg(test)]
mod functionality_test {
    use std::collections::HashMap;
    use create_randomly_5pairs;
    use randomise_values;
    use compare_result;

    #[test]
    fn randomply_creation_of_hashmaps() {
        let language1 = vec!["papa", "familia", "amigo", "hermano", "verde"]; 
        let english1 = vec!["dad", "family", "friend", "brother", "green"];
        let mut test_map : HashMap<String, String> = HashMap::new();

        test_map.insert("papa".to_string(), "dad".to_string());
        test_map.insert("familia".to_string(), "family".to_string());
        test_map.insert("amigo".to_string(), "friend".to_string());
        test_map.insert("hermano".to_string(), "brother".to_string());
        test_map.insert("verde".to_string(), "green".to_string());
        //let tmp = create_randomly_5pairs(&language1, &english1);
        assert_eq!(create_randomly_5pairs(&language1, &english1), test_map);
    }


    #[test]
    #[should_panic] //Since assert_eq is being used
    fn scrambling_of_hashmap_values() { 
        let mut test_map2 : HashMap<String, String> = HashMap::new();
        test_map2.insert("papa".to_string(), "dad".to_string());
        test_map2.insert("familia".to_string(), "family".to_string());
        test_map2.insert("amigo".to_string(), "friend".to_string());

        assert_eq!(randomise_values(&test_map2.clone()), test_map2);
    }


    #[test]
    fn checking_hashmaps_equality() { 
        //Original HashMap with correctly matched key-val pairs
        let mut origin_map : HashMap<String, String> = HashMap::new();
        origin_map.insert("papa".to_string(), "dad".to_string());
        origin_map.insert("familia".to_string(), "family".to_string());
        origin_map.insert("amigo".to_string(), "friend".to_string());
        origin_map.insert("hermano".to_string(), "brother".to_string());
        origin_map.insert("verde".to_string(), "green".to_string());

        //HashMap with scrabled or non matching key-values pairs 
        let mut from_user : HashMap<String, String> = HashMap::new();
        from_user.insert("papa".to_string(), "dad".to_string());

        //Something else not in the original hashmap (user invented meaning)
        from_user.insert("familia".to_string(), "book".to_string());

        //The following two lines have been switched in values
        from_user.insert("amigo".to_string(), "brother".to_string());
        from_user.insert("hermano".to_string(), "friend".to_string());

        //In correctly matched key-value pair
        from_user.insert("verde".to_string(), "family".to_string());

        let tmp_vec = compare_result(&from_user, &origin_map);  //Returs vector of score result

        assert_eq!(tmp_vec, vec!["Success for Words : papa,  ", "20%"]);
    }
}

