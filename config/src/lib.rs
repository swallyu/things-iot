#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]

use std::string;
use serde::{Serialize,Deserialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Config{
    pub http:HttpConfig,
    pub mqtt:MqttConfig
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct HttpConfig{
    pub host:String,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct MqttConfig{
    pub host:String,
    pub port:u16,
    pub user:String,
    pub pwd:String,
    pub client_id_prefix:Option<String>
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::*;
    #[test]
    fn it_works() {

        let http= HttpConfig{host:String::from("AAA")};
        let mqtt=MqttConfig{host:String::from("127.0.0.1"),port:1883,user:String::from("ecoran"),
                pwd:String::from("123456"),client_id_prefix:None};

        let json = serde_json::to_string(&http);
        let mqtt_json = serde_json::to_string(&mqtt);

        let config = Config{http,mqtt};
        let config_json = serde_json::to_string(&config);

        println!("{} \r\n{} \r\n{}",json.unwrap(),mqtt_json.unwrap(),config_json.unwrap());
        
        //assert_eq!(result, 4);
    }
}
