use microsoft_dia::{NameSearchOptions, SymTag};
use windows_core::*;

use crate::common::get_test_session;

#[allow(dead_code)]
static TEST_VALUE_01: i32 = 1;
#[allow(dead_code)]
static TEST_VALUE_02: i32 = 2;
#[allow(dead_code)]
static TEST_VALUE_03: i32 = 3;

#[test]
fn simple_enumeration() -> Result<()> {
    let session = get_test_session()?;
    let symbols = session.global_scope()?.find_children(
        SymTag::Null,
        "main::enumerators::TEST_VALUE_[0-9]+",
        NameSearchOptions::WILDCARD,
    )?;

    let mut found: Vec<String> = symbols.map(|s| s.name().unwrap()).collect();
    found.sort();
    assert_eq!(
        found,
        [
            "main::enumerators::TEST_VALUE_01",
            "main::enumerators::TEST_VALUE_02",
            "main::enumerators::TEST_VALUE_03",
        ]
    );

    Ok(())
}

#[test]
fn fallible_enumeration() -> Result<()> {
    let session = get_test_session()?;
    let mut symbols = session.global_scope()?.find_children(
        SymTag::Null,
        "main::enumerators::TEST_VALUE_[0-9]+",
        NameSearchOptions::WILDCARD,
    )?;

    let mut found = Vec::new();
    while let Some(sym) = symbols.next_item()? {
        found.push(sym.name()?);
    }
    found.sort();
    assert_eq!(
        found,
        [
            "main::enumerators::TEST_VALUE_01",
            "main::enumerators::TEST_VALUE_02",
            "main::enumerators::TEST_VALUE_03",
        ]
    );

    Ok(())
}
