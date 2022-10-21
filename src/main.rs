use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::io::{stdin, BufRead};

#[derive(Debug)]
struct Query<'a> {
    key: &'a str,
    value: Option<&'a str>,
}

impl Query<'_> {
    fn from_str(str: &str) -> HashSet<Query> {
        let mut v = HashSet::new();
        for pair in str.split('&') {
            let mut it = pair.splitn(2,"=");
            v.insert(Query {
                key: it.nth(0).unwrap(),
                value: it.nth(0),
            });
        }
        return v;
    }
}

impl Hash for Query<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl PartialEq for Query<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(other.key)
    }
}

impl Eq for Query<'_> {}

fn print(url:&str, query: &HashSet<Query>) {
    print!("{}", url);
    if !query.is_empty() {
        print!("?")
    }
    query.iter().enumerate().for_each(|(a,b)| {
        print!("{}", b.key);
        if let Some(v) = b.value {
            print!("={}", v);
        }
        if a != query.len()-1 {
            print!("&");
        }
    });
    print!("\n");
}

fn main() {
    let input = stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .unwrap();
    let mut map: HashMap<&str, HashSet<Query>> = HashMap::new();

    input.iter().map(|a| a.splitn(2, "?")).for_each(|mut a| {
        match map.entry(a.nth(0).unwrap()) {
            Entry::Vacant(e) => {
                match a.nth(0)  {
                    Some(t) => {e.insert(Query::from_str(t));},
                    None => {e.insert(HashSet::from([]));}
                }
            }
            Entry::Occupied(mut e) => {
                if let Some(x) = a.nth(0) {
                    e.get_mut().extend(Query::from_str(x));
                }
            }
        };
    });
    map.iter().for_each(|(a,b)|print(a,b));
}