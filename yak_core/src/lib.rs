#[derive(Debug)]
pub struct Message {
    topic: String, 
    partition: Option<usize>,
    content: Vec<u8>
}

impl Message { 
    pub fn new(topic: String, content: Vec<u8> ) -> Self { 
        Self { 
            topic,
            partition: None,
            content
        }
    }

    pub fn new_with_partition(topic: String, partition: usize, content: Vec<u8>) -> Self {
        Self { 
            topic, 
            partition: Some(partition),
            content
        }
    }

    pub fn has_partition(&self) -> bool {
        return self.partition.is_some()
    }

    pub fn serialize(&self) -> Vec<u8> {
        Vec::new()
    }

    pub fn deserialize(bytes :&[u8]) -> Self {
        return Self::new("Nothing".to_owned(), vec![]);
    }
}
