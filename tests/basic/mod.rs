use microsoft_dia::{NameSearchOptions, SymTagData};
use windows_core::*;

use crate::common::get_test_session;

#[allow(dead_code)]
static TEST_DATA: u32 = 42;

#[test]
fn symbol_properties() -> Result<()> {
    unsafe {
        let session = get_test_session()?;
        let symbol = session
            .globalScope()?
            .findChildren(
                SymTagData,
                w!("main::basic::TEST_DATA"),
                NameSearchOptions::default().0 as u32,
            )?
            .Item(0)?;

        assert_eq!(symbol.unalignedType()?, false);
        assert_eq!(symbol.constType()?, false);
        assert_eq!(symbol.volatileType()?, false);

        Ok(())
    }
}
