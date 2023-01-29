use handle_errors::Error;
use std::collections::HashMap;

/// Pagination struct that is getting extracted
/// from query parameter
#[derive(Debug)]
pub struct Pagination {
    /// The index of the first item that has to be returned
    pub start: usize,
    /// The index of the last item that has to be returned
    pub end: usize,
}

/// Extract query parameters from the `/questions` route
/// # Example query
/// GET request to this route can have a pagination attached
/// so we just return the questions we need
/// `/questions?start=1&end=5`
/// # Example usage
/// ```rust
/// let mut query = HashMap::new();
/// query.insert("start".to_string(), "1".to_string());
/// query.insert("end".to_string(), "5".to_string());
/// let p = types::pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.start, 1);
/// assert_eq!(p.end, 5);
/// ```

pub fn extract_pagination(params: &HashMap<String, String>) -> Result<Pagination, Error> {
    // could be improved in future
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            // Takes the "start" parameter in the query
            // and tries to convert it into a number
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            // Takes the "end" parameter in the query
            // and tries to convert it into a number
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::MissingParameters)
}
