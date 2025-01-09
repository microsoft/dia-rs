use microsoft_dia::{DiaSource, IDiaDataSource, IDiaSession, SymTagNull, nsfRegularExpression};
use windows::{
    Win32::{
        Foundation::S_OK,
        System::Com::{COINIT_MULTITHREADED, CoInitializeEx},
    },
    core::*,
};

fn get_test_session() -> Result<IDiaSession> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
        let source: IDiaDataSource = microsoft_dia::helpers::NoRegCoCreate(
            s!(
                r#"C:\Program Files\Microsoft Visual Studio\2022\Enterprise\DIA SDK\bin\amd64\msdia140.dll"#
            ),
            &DiaSource,
        )?;
        let executable = std::env::current_exe().unwrap();
        source.loadDataForExe(&HSTRING::from(executable.as_os_str()), None, None)?;
        source.openSession()
    }
}

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
            w!("tests::TEST_VALUE_[0-9]+"),
            nsfRegularExpression.0 as u32,
        )?;

        let mut found = Vec::new();
        for i in 0..symbols.Count()? {
            found.push(symbols.Item(i as u32)?.name()?);
        }
        assert_eq!(found, [
            "tests::TEST_VALUE_01",
            "tests::TEST_VALUE_02",
            "tests::TEST_VALUE_03",
        ]);

        Ok(())
    }
}

#[test]
fn batch_enumeration() -> Result<()> {
    unsafe {
        let session = get_test_session()?;
        let symbols = session.globalScope()?.findChildren(
            SymTagNull,
            w!("tests::TEST_VALUE_[0-9]+"),
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
        assert_eq!(found, [
            "tests::TEST_VALUE_01",
            "tests::TEST_VALUE_02",
            "tests::TEST_VALUE_03",
        ]);

        Ok(())
    }
}
