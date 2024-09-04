use crate::domain::{DatabaseError, DatabaseTrait, HandlerError, HandlerTrait};

impl<T: DatabaseTrait + ?Sized> HandlerTrait for T {
    fn set(&mut self, key: String, value: String) -> Result<(), HandlerError> {
        self.insert(key, value)
            .map_err(|e| db_into_handler_error(e))
    }

    fn get(&self, key: String) -> Result<String, HandlerError> {
        self.search(key).map_err(|e| db_into_handler_error(e))
    }

    fn del(&mut self, key: String) -> Result<String, HandlerError> {
        self.delete(key).map_err(|e| db_into_handler_error(e))
    }
}

fn db_into_handler_error(e: DatabaseError) -> HandlerError {
    match e {
        DatabaseError::KeyNotFound => HandlerError::KeyNotFound,
        DatabaseError::UnexpectedError => HandlerError::UnexpectedError,
    }
}
