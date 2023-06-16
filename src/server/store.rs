use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use super::table::HTable;

#[derive(Eq, Clone)]
struct TTL {
    expire: u128,
    id: u128
}

impl PartialEq for TTL {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Ord for TTL {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.expire.cmp(&other.expire) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            _ => self.id.cmp(&other.id),
        }
    }
}

impl PartialOrd for TTL {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.expire.partial_cmp(&other.expire)
    }
}

pub struct DataStore {
    cache: HTable,
    ttls: BTreeMap<TTL, String>,
    cache_ttls: HashMap<String, TTL>
}

impl DataStore {
    pub fn new(size: usize) -> Self {
        let cache = HTable::new(size);
        let ttls = BTreeMap::new();
        let cache_ttls = HashMap::new();

        DataStore{cache, ttls, cache_ttls}
    }

    pub fn keys(&self) -> &Vec<String> {
        self.cache.keys()
    }

    pub fn get(&self, key: &str) -> Option<Arc<Vec<u8>>> {
        self.cache.get(key)
    }

    pub fn insert(&mut self, key: &str, value: Vec<u8>, ttl: u64) {
        if ttl != 0 {
            let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            let expire = current_time.checked_add(Duration::from_millis(ttl)).unwrap();
            let ttl = TTL{expire: expire.as_millis(), id: current_time.as_nanos()};
            println!("Inserted ({},{})", ttl.expire, ttl.id);
            self.ttls.insert(ttl.clone(), key.to_string());
            self.cache_ttls.insert(key.to_string(), ttl);
        }
        self.cache.insert(key, value);
        println!("Insert: cache_ttls:{}, ttls:{}, cache:{}", self.cache_ttls.len(), self.ttls.len(), self.cache.len());
    }

    pub fn delete(&mut self, key: &str) {
        let ttl = self.cache_ttls.remove(key).unwrap();
        self.ttls.remove(&ttl);
        self.cache.delete(key);
        println!("Delete: cache_ttls:{}, ttls:{}, cache:{}", self.cache_ttls.len(), self.ttls.len(), self.cache.len());
    }

    pub fn expire(&mut self) -> Option<Vec<String>> {
        let mut expired_keys = vec![];
        while let Some(key) = self.try_expire() {
            expired_keys.push(key);
        }
        if expired_keys.len() > 0 {
            return Some(expired_keys)
        }
        None
    }

    pub fn try_expire(&mut self) -> Option<String> {
        let entry = self.ttls.first_entry()?;
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        if entry.key().expire > current_time.as_millis() {
            return None;
        }
        let ttl = self.ttls.pop_first()?;
        self.cache_ttls.remove(&ttl.1).unwrap();
        self.cache.delete(&ttl.1);
        println!("Expire: cache_ttls:{}, ttls:{}, cache:{}", self.cache_ttls.len(), self.ttls.len(), self.cache.len());
        Some(ttl.1)
    }

}
