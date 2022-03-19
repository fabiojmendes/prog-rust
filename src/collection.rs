#[test]
fn test_append() {
    let mut v1 = vec![1, 2, 3, 4];
    let mut v2 = vec![5, 6, 7, 8];
    v1.append(&mut v2);
    assert_eq!(v1, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    assert!(v2.is_empty());
}

#[test]
fn test_extend() {
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    v1.extend(v2);
    assert_eq!(v1, vec![1, 2, 3, 4, 5, 6, 7, 8]);
}
