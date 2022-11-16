#[test]
fn vec_try_into_array() {
    let vec = vec![0u8, 1, 2, 3];
    assert_eq!(
        TryInto::<[u8; 4]>::try_into(vec).unwrap(),
        [0u8, 1, 2, 3]
    )
}