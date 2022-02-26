
use structopt::StructOpt;
use topic::{
    Options,Topic,fetch_topics,how_many_random
};

fn main() {

    let opt = Options::from_args();
    let mut choices = fetch_topics();
    
    if let Some(topic) = opt.add {
        let tpc = Topic::new(topic);
        choices.push(&tpc);
        tpc.log_topics();
    }
    
    if let Some(number) = opt.no {
        assert!((number as usize) <= choices.len());
        how_many_random(number.into(), &choices);
    }
}
