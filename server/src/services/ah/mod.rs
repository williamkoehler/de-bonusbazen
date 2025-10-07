pub struct AhService {
    redis: crate::databases::redis::RedisDb,
}

impl AhService {
    pub fn new(redis: crate::databases::redis::RedisDb) -> Self {
        AhService { redis }
    }

    // pub fn get_
}
