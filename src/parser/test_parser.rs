#[test]
fn parse_empty_bytes() {
    let bytes: [u8; 0] = [];
    let result = super::parse(&bytes);

    assert!(result.is_err())
}

#[test]
fn parse_invalid_nop() {
    let bytes: [u8; 1] = [0x42];
    let result = super::parse(&bytes);

    assert!(result.is_err())
}

#[test]
fn parse_valid_nop() {
    let expected: instructions::Instruction = instructions::Nop::new().into();

    let bytes: [u8; 1] = [0x00];
    let result = super::parse(&bytes).unwrap();

    assert_eq!(expected, result)
}
