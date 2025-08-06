use crate::Pool;

pub struct DatabaseRepo {
    pub pool: Pool,
}

impl DatabaseRepo {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}
