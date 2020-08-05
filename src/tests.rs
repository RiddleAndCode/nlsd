use crate::*;

#[test]
fn serialize_bool() -> Result<()> {
    assert_eq!(to_string(&true)?, "true");
    assert_eq!(to_string(&false)?, "false");
    Ok(())
}

#[test]
fn serialize_num() -> Result<()> {
    assert_eq!(to_string(&0)?, "0");
    assert_eq!(to_string(&1)?, "1");
    assert_eq!(to_string(&-1)?, "-1");
    assert_eq!(to_string(&12)?, "12");
    assert_eq!(to_string(&-13)?, "-13");
    assert_eq!(to_string(&1.5)?, "1.5");
    assert_eq!(to_string(&1.0)?, "1");
    assert_eq!(to_string(&0.1)?, "0.1");
    assert_eq!(to_string(&-1.5)?, "-1.5");
    Ok(())
}

#[test]
fn serialize_empty() -> Result<()> {
    assert_eq!(to_string(&())?, "empty");
    assert_eq!(to_string(&Option::<()>::None)?, "nothing");
    Ok(())
}

#[test]
fn serialize_str() -> Result<()> {
    assert_eq!(to_string(&'a')?, "'a'");
    assert_eq!(to_string("")?, "''");
    assert_eq!(to_string("cool")?, "'cool'");
    assert_eq!(to_string("don't")?, r"'don\'t'");
    Ok(())
}

#[test]
fn serialize_list() -> Result<()> {
    assert_eq!(
        to_string(&vec![1, 2, 3])?,
        "the list where an item is 1 and another item is 2 and another item is 3"
    );
    Ok(())
}
