use structopt::StructOpt;
use serde::{Serialize, Deserialize};
use std::path::Path;
use rand::Rng;
use std::fs;
use std::io::Write;

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(short="n", long="num")]
    pub no: Option<u32>,
    #[structopt(short="a", long="add")]
    pub add: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct Topic {
    topic: String
}
impl Topic {
    pub fn new(topic: String)->Self {
        Self {topic}
    }
    pub fn log_topics(&self){
        let item_log=[self];
        let item_log=serde_yaml::to_vec(&item_log).unwrap();
        let path = Path::new("/data/data/com.termux/files/home/.cargo/topics.yaml");
    
        if path.exists(){
            let mut file=fs::OpenOptions::new().append(true).open(path).expect("cant open file");
            let item_log=&item_log[4..];
            file.write_all(&item_log).expect("cant write into..");
        }else{
            let mut file=fs::File::create(path).expect("cant open file");
            file.write_all(&item_log).expect("cant write into..");
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Choices {
    list: Vec<Topic>
}

impl Choices {
    pub fn push(&mut self, topic: &Topic)->Self {
        self.list.push(topic.to_owned());
        self.clone()
    }
    pub fn len(&self)->usize{
        self.list.len()
    }
}

pub fn fetch_topics()->Choices {
    let path = Path::new("/data/data/com.termux/files/home/.cargo/topics.yaml");
    if path.exists(){
        let data = std::fs::read(path).expect("failed to read path");
        let log: Vec<Topic> = serde_yaml::from_slice(&data).unwrap();
        let choices = Choices{list: log};
        choices
    }else{
        Choices::default()
    }
}

pub fn how_many_random(amt:u32, lst:&Choices){
    println!();
    let mut index_list: Vec<Topic> = vec![];
    for i in 0..amt {
        let random_index = rand::thread_rng().gen_range(0.. lst.len());
        let mut topic_obj = &lst.list[random_index];
        
        while index_list.contains(&topic_obj) {
            let random_index =rand::thread_rng().gen_range(0..lst.len());
            topic_obj = &lst.list[random_index];
        }
        index_list.push(topic_obj.clone());
        print!("| {} |",index_list[i as usize].topic);
    }
    //dbg!(&index_list);
    println!();
    println!();
}