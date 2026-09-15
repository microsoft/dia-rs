//! DIA table types.

use crate::bindings::*;
use crate::enums::Unknowns;
use windows_core::*;

/// A named table of a symbol store, as returned by
/// [`Session::tables`](crate::Session::tables).
///
/// A table exposes its items through a typed enumerator; use
/// [`Table::enumerate`] to get one.
#[derive(Clone, Debug)]
pub struct Table(pub(crate) IDiaTable);

impl Table {
    /// Retrieves the name of the table.
    pub fn name(&self) -> Result<String> {
        unsafe { Ok(self.0.name()?.display().to_string()) }
    }

    /// Retrieves the number of items in the table.
    pub fn len(&self) -> Result<u32> {
        unsafe { Ok(self.0.Count()? as u32) }
    }

    /// Returns whether the table has no items.
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.len()? == 0)
    }

    /// Retrieves an item of the table by index, as a raw `IUnknown`.
    ///
    /// Use [`Table::enumerate`] for typed iteration instead.
    pub fn item_unknown(&self, index: u32) -> Result<IUnknown> {
        unsafe { self.0.Item(index) }
    }

    /// Retrieves an enumerator of the table's raw items.
    ///
    /// DIA table items are heterogeneous, so the enumerator yields
    /// [`IUnknown`] values that the caller casts to the expected interface.
    pub fn enumerate(&self) -> Result<Unknowns> {
        unsafe { Ok(Unknowns(self.0._NewEnum()?.cast()?)) }
    }
}
