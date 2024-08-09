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


    fn test_pop() {
        let mut log = TransactionLog::new_empty();
        log.append("value1".to_string());
        log.append("value2".to_string());
        log.append("value3".to_string());

        assert!(log.length == 3);

        log.pop();

        assert!(log.length == 2);
    }
    
}