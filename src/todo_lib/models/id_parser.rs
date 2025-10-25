/// Parses task ID specifications into a list of individual IDs.
///
/// Supports:
/// - Single IDs: `5`
/// - Ranges: `1-5` (inclusive)
/// - Lists: `1,3,5`
/// - Combined: `1-3,7,9-11`
///
/// # Examples
///
/// ```
/// use todo_manager::models::id_parser::parse_ids;
///
/// assert_eq!(parse_ids("5"), Ok(vec![5]));
/// assert_eq!(parse_ids("1-3"), Ok(vec![1, 2, 3]));
/// assert_eq!(parse_ids("1,3,5"), Ok(vec![1, 3, 5]));
/// assert_eq!(parse_ids("1-3,7,9-11"), Ok(vec![1, 2, 3, 7, 9, 10, 11]));
/// ```
pub fn parse_ids(input: &str) -> Result<Vec<usize>, String> {
    let mut ids = Vec::new();
    
    // Split by comma for multiple segments
    for segment in input.split(',') {
        let segment = segment.trim();
        
        if segment.is_empty() {
            continue;
        }
        
        // Check if it's a range (contains '-')
        if segment.contains('-') {
            let parts: Vec<&str> = segment.split('-').collect();
            
            if parts.len() != 2 {
                return Err(format!("Invalid range format: '{}'. Expected format: 'start-end'", segment));
            }
            
            let start = parts[0].trim().parse::<usize>()
                .map_err(|_| format!("Invalid number in range: '{}'", parts[0]))?;
            let end = parts[1].trim().parse::<usize>()
                .map_err(|_| format!("Invalid number in range: '{}'", parts[1]))?;
            
            if start > end {
                return Err(format!("Invalid range: {}-{}. Start must be less than or equal to end", start, end));
            }
            
            // Add all IDs in the range (inclusive)
            for id in start..=end {
                ids.push(id);
            }
        } else {
            // Single ID
            let id = segment.parse::<usize>()
                .map_err(|_| format!("Invalid task ID: '{}'", segment))?;
            ids.push(id);
        }
    }
    
    if ids.is_empty() {
        return Err("No task IDs provided".to_string());
    }
    
    // Remove duplicates and sort
    ids.sort_unstable();
    ids.dedup();
    
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_id() {
        assert_eq!(parse_ids("5"), Ok(vec![5]));
        assert_eq!(parse_ids("42"), Ok(vec![42]));
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_ids("1-3"), Ok(vec![1, 2, 3]));
        assert_eq!(parse_ids("5-7"), Ok(vec![5, 6, 7]));
        assert_eq!(parse_ids("1-1"), Ok(vec![1]));
    }

    #[test]
    fn test_parse_list() {
        assert_eq!(parse_ids("1,3,5"), Ok(vec![1, 3, 5]));
        assert_eq!(parse_ids("2,4,6,8"), Ok(vec![2, 4, 6, 8]));
    }

    #[test]
    fn test_parse_combined() {
        assert_eq!(parse_ids("1-3,7,9-11"), Ok(vec![1, 2, 3, 7, 9, 10, 11]));
        assert_eq!(parse_ids("1,3-5,10"), Ok(vec![1, 3, 4, 5, 10]));
    }

    #[test]
    fn test_parse_with_spaces() {
        assert_eq!(parse_ids(" 1 - 3 , 7 "), Ok(vec![1, 2, 3, 7]));
        assert_eq!(parse_ids("  5  ,  10  "), Ok(vec![5, 10]));
    }

    #[test]
    fn test_parse_removes_duplicates() {
        assert_eq!(parse_ids("1,2,1,3,2"), Ok(vec![1, 2, 3]));
        assert_eq!(parse_ids("1-3,2-4"), Ok(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_parse_invalid_range_order() {
        assert!(parse_ids("5-3").is_err());
        assert!(parse_ids("10-1").is_err());
    }

    #[test]
    fn test_parse_invalid_number() {
        assert!(parse_ids("abc").is_err());
        assert!(parse_ids("1,abc,3").is_err());
        assert!(parse_ids("1-abc").is_err());
    }

    #[test]
    fn test_parse_invalid_range_format() {
        assert!(parse_ids("1-2-3").is_err());
        assert!(parse_ids("1--3").is_err());
    }

    #[test]
    fn test_parse_empty_input() {
        assert!(parse_ids("").is_err());
        assert!(parse_ids("   ").is_err());
    }
}
