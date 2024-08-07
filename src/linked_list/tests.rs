#[cfg(test)]
mod tests{

    use crate::linked_list::LinkedList::TransactionLog;

    #[test]
    fn test_create() {
        let mut log = TransactionLog::new_empty();
        log.append("testing".to_string());
        log.append("testing".to_string());
        assert!(log.length == 2, "2 werent inserted");
    }

    
}