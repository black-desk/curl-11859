use curl::easy::Easy;
use curl::multi::EasyHandle;
use curl::multi::Message;
use curl::multi::Multi;
use std::collections::HashMap;
use std::time::Duration;

fn main() {
    let mut multi = Multi::new();

    multi.set_max_host_connections(1).unwrap();
    multi.pipelining(true, true).unwrap();

    let mut map: HashMap<usize, EasyHandle> = HashMap::new();

    for i in 0..56 {
        let mut easy = Easy::new();
        easy.verbose(true).unwrap();
        easy.http_version(curl::easy::HttpVersion::V2PriorKnowledge)
            .unwrap();
        easy.url("http://localhost:8000").unwrap();
        let mut handle = multi.add(easy).unwrap();
        handle.set_token(i).unwrap();
        map.insert(i, handle);
    }

    let mut handle_messages = || -> () {
        multi.messages(|msg: Message| -> () {
            let token = msg.token().unwrap();
            println!("{}", token);
            match msg.result() {
                Some(result) => match result {
                    Ok(_) => {
                        let handle = map.remove(&token).unwrap();
                        multi.remove(handle).unwrap();
                    }
                    Err(err) => {
                        panic!("{}", err.description());
                    }
                },
                None => {}
            };
        });
    };

    while multi.perform().unwrap() > 0 {
        multi.wait(&mut [], Duration::from_secs(1)).unwrap();
        handle_messages();
    }
    handle_messages();

    assert!(map.is_empty())
}
