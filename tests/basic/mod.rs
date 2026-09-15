use microsoft_dia::{NameSearchOptions, SymTag};
use windows_core::*;

use crate::common::get_test_session;

#[allow(dead_code)]
static TEST_DATA: u32 = 42;

#[test]
fn symbol_properties() -> Result<()> {
    let session = get_test_session()?;
    let symbol = session
        .global_scope()?
        .find_children(
            SymTag::Data,
            "main::basic::TEST_DATA",
            NameSearchOptions::NONE,
        )?
        .item(0)?;

    assert!(!symbol.unaligned_type()?);
    assert!(!symbol.const_type()?);
    assert!(!symbol.volatile_type()?);

    Ok(())
}

#[test]
fn syms_are_equiv_polarity() -> Result<()> {
    let session = get_test_session()?;
    let data = session
        .global_scope()?
        .find_children(
            SymTag::Data,
            "main::basic::TEST_DATA",
            NameSearchOptions::NONE,
        )?
        .item(0)?;
    let func = session
        .global_scope()?
        .find_children(
            SymTag::Function,
            "main::basic::symbol_properties",
            NameSearchOptions::NONE,
        )?
        .item(0)?;

    // The S_OK branch: a symbol is equivalent to itself.
    assert!(session.syms_are_equiv(&data, &data)?);
    // The S_FALSE branch: two distinct symbols are not equivalent.
    assert!(!session.syms_are_equiv(&data, &func)?);

    Ok(())
}
