use crate::common::get_test_session;
use microsoft_dia::{nsfRegularExpression, SymTagNull};
use windows::Win32::Foundation::S_OK;
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
            w!("main::enumerators::TEST_VALUE_[0-9]+"),
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
            w!("main::enumerators::TEST_VALUE_[0-9]+"),
            nsfRegularExpression.0 as u32,
        )?;

        let mut found = Vec::new();
        let mut batch = [None, None];
        let mut fetched = 0;
        while symbols.Next(&mut batch, &mut fetched) == S_OK {
            found.extend(
                batch[0..fetched as usize]
                    .iter()
                    .filter_map(|s| s.as_ref()?.name().ok()),
            );
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
