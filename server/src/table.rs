use std::sync::Arc;

#[derive(Clone, Debug)]
struct HNode {
    code: usize,
    key: String,
    value: Arc<Vec<u8>>,
}

#[derive(Clone, Debug)]
pub struct HTable {
    table: Vec<Vec<Arc<HNode>>>,
    keys: Vec<String>,
    size: usize,
    mask: usize,
}

impl HTable {
    /// size must be a power of 2
    pub fn new(size: usize) -> Self {
        assert!(size.is_power_of_two());
        Self {
            table: vec![vec![]; size],
            size: 0,
            keys: vec![],
            mask: size - 1,
        }
    }

    pub fn insert(&mut self, key: &str, value: Vec<u8>) {
        let h_key = hash_key(key) & self.mask;

        if self.table[h_key].is_empty() {
            let new_node = HNode {
                code: h_key,
                key: key.to_string(),
                value: Arc::new(value),
            };
            self.table[h_key].push(Arc::new(new_node));
        } else {
            self.delete(key);
            let new_node = HNode {
                code: h_key,
                key: key.to_string(),
                value: Arc::new(value),
            };
            self.table[h_key].push(Arc::new(new_node));
        }
        self.keys.push(key.to_string());
        self.size += 1;
    }

    pub fn get(&self, key: &str) -> Option<Arc<Vec<u8>>> {
        let h_key = hash_key(key) & self.mask;
        let node = find_matching_node(key, &self.table[h_key]);

        if node.is_some() {
            let n = node.as_deref();
            Some(n.unwrap().value.clone())
        } else {
            None
        }
    }

    pub fn delete(&mut self, key: &str) {
        let h_key = hash_key(key) & self.mask;
        let bucket = &mut self.table[h_key];
        for i in 0..bucket.len() {
            if let Some(val) = bucket.get(i) {
                if val.key == key {
                    bucket.remove(i);
                    self.size -= 1;
                }
            }
        }
        if let Some(to_remove) = self.keys.iter().enumerate().find(|k| *k.1 == key ) {
            self.keys.swap_remove(to_remove.0);
        }
    }

    pub fn keys(&self) -> &Vec<String> {
        &self.keys
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

fn hash_key(key: &str) -> usize {
    key.chars().fold(0usize, |acc, val| acc + val as usize)
}

fn find_matching_node<'a, 'b>(key: &'a str, bucket: &'b Vec<Arc<HNode>>) -> Option<&'b Arc<HNode>> {
    bucket.iter().find(|n| n.key == key)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash_key() {
        let value = "hi";
        let hash_key = hash_key(value);

        // 104 + 105 = 209
        assert_eq!(209, hash_key);
        println!("{}", hash_key);
    }

    #[test]
    fn test_find_matching_node() {
        let bucket = vec!(
            Arc::new(
                HNode {
                    code: 123,
                    key: String::from("node1"),
                    value: Arc::new(String::from("val1").into_bytes()),
                }),
            Arc::new(
                HNode {
                    code: 123,
                    key: String::from("node2"),
                    value: Arc::new(String::from("val2").into_bytes()),
                }),
            Arc::new(
                HNode {
                    code: 123,
                    key: String::from("node3"),
                    value: Arc::new(String::from("val3").into_bytes()),
                })
        );

        assert_eq!(String::from("val3").into_bytes(), *find_matching_node("node3", &bucket).as_deref().unwrap().value);
        assert_eq!(String::from("val2").into_bytes(), *find_matching_node("node2", &bucket).as_deref().unwrap().value);
        assert_eq!(String::from("val1").into_bytes(), *find_matching_node("node1", &bucket).as_deref().unwrap().value);
        assert!(find_matching_node("nothing", &bucket).is_none());
        assert!(find_matching_node("anything", &vec![]).is_none());
    }

    #[tokio::test]
    async fn test_keys() {
        let mut table = HTable::new(2);
        let key1 = String::from("Key1");
        let key2 = String::from("Key2");
        let key3 = String::from("Key3");
        let key4 = String::from("Key4");

        table.insert(&key1, b"Value1".to_vec());
        table.insert(&key2, b"Value2".to_vec());
        table.insert(&key3, b"Value3".to_vec());
        table.insert(&key4, b"Value4".to_vec());
        assert_eq!(&vec![key1.clone(), key2.clone(), key3.clone(), key4.clone()], table.keys());

        table.delete(&key4);
        assert_eq!(&vec![key1, key2, key3], table.keys());
    }

    #[tokio::test]
    async fn test_insert_get_delete() {
        let key = "key";
        let value = String::from("value");

        let mut table = HTable::new(2);
        assert!(table.get(key).is_none());

        table.insert(key, value.into_bytes());
        assert_eq!(String::from("value").into_bytes(), *table.get(key).unwrap());

        table.delete(key);
        assert!(table.get(key).is_none());
    }

    #[tokio::test]
    async fn test_insert_collision() {
        let key = "key";
        let yek = "yek";

        let mut table = HTable::new(2);

        table.insert(key, String::from("value1").into_bytes());
        table.insert(yek, String::from("value2").into_bytes());

        assert_eq!(String::from("value1").into_bytes(), *table.get(key).unwrap());
        assert_eq!(String::from("value2").into_bytes(), *table.get(yek).unwrap());
    }

    #[tokio::test]
    async fn test_delete_collision() {
        let key = "key";
        let yek = "yek";

        let mut table = HTable::new(2);

        table.insert(key, String::from("value1").into_bytes());
        table.insert(yek, String::from("value2").into_bytes());

        assert_eq!(String::from("value1").into_bytes(), *table.get(key).unwrap());
        assert_eq!(String::from("value2").into_bytes(), *table.get(yek).unwrap());

        table.delete(key);
        assert!(table.get(key).is_none());
        assert_eq!(String::from("value2").into_bytes(), *table.get(yek).unwrap());

        table.delete(yek);
        assert!(table.get(key).is_none());
        assert!(table.get(yek).is_none());
    }

}
