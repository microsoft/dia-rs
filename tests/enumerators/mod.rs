use crate::common::get_test_session;
use microsoft_dia::{nsfRegularExpression, IDiaSymbol, SymTagNull};
use windows_core::*;

#[allow(dead_code)]
static TEST_VALUE_01: i32 = 1;
#[allow(dead_code)]
static TEST_VALUE_02: i32 = 2;
#[allow(dead_code)]
static TEST_VALUE_03: i32 = 3;

#[test]
fn simple_enumeration() -> Result<()> {
    unsafe {
        let session = get_test_session()?;
        let symbols = session.globalScope()?.findChildren(
            SymTagNull,
            w!("main::enumerators::TEST_VALUE_[0-9]+").0,
            nsfRegularExpression.0 as u32,
        )?;

        let mut found = Vec::new();
        for i in 0..symbols.Count()? {
            found.push(symbols.Item(i as u32)?.name()?);
        }

        found.sort_by(|a, b| a.cmp(b));
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
}

#[test]
fn batch_enumeration() -> Result<()> {
    unsafe {
        let session = get_test_session()?;
        let symbols = session.globalScope()?.findChildren(
            SymTagNull,
            w!("main::enumerators::TEST_VALUE_[0-9]+").0,
            nsfRegularExpression.0 as u32,
        )?;

        let mut symbol: Option<IDiaSymbol> = None;
        let mut found: Vec<BSTR> = Vec::new();
        let mut fetched = 0;
        while symbols.Next(1, &mut symbol, &mut fetched).is_ok() && fetched > 0 {
            found.push(symbol.take().unwrap().name().unwrap());
        }

        found.sort_by(|a, b| a.cmp(b));
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
}
