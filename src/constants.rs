//! Crate-owned value types for the DIA SDK.
//!
//! Idiomatic Rust value types for the DIA SDK: enums, flags, and machine
//! codes, defined in this crate rather than re-exported from the private
//! `bindings` module, so the public surface has no raw binding items at all.
//! Values match the ABI exactly.

/// A DIA symbol tag, identifying the kind of a symbol
///
/// Symbols are looked up by tag through the `find_children` methods; the tag
/// is also returned by [`crate::Symbol::sym_tag`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SymTag {
    /// The symbol has no type.
    Null = 0,
    /// The symbol is an .exe file. There is only one `Exe` symbol per symbol
    /// store; it serves as the global scope and has no lexical parent.
    Exe = 1,
    /// The compiland symbol for each compiland component of the symbol store.
    /// For native applications, compiland symbols correspond to the object
    /// files linked into the image; for some MSIL images there is one
    /// compiland per class.
    Compiland = 2,
    /// The symbol contains extended attributes of the compiland.
    CompilandDetails = 3,
    /// The symbol is an environment string defined for the compiland.
    CompilandEnv = 4,
    /// The symbol is a function.
    Function = 5,
    /// The symbol is a nested block.
    Block = 6,
    /// The symbol is data.
    Data = 7,
    /// The symbol is a code annotation. Its children are constant data strings
    /// (`Data`, `LocIsConstant`, `DataIsConstant`); most clients ignore it.
    Annotation = 8,
    /// The symbol is a label.
    Label = 9,
    /// The symbol is a public symbol. For native applications this is the COFF
    /// external symbol encountered while linking the image.
    PublicSymbol = 10,
    /// The symbol is a user-defined type (structure, class, or union).
    Udt = 11,
    /// The symbol is an enumeration.
    Enum = 12,
    /// The symbol is a function signature type.
    FunctionType = 13,
    /// The symbol is a pointer type.
    PointerType = 14,
    /// The symbol is an array type.
    ArrayType = 15,
    /// The symbol is a base type.
    BaseType = 16,
    /// The symbol is a typedef, an alias for another type.
    Typedef = 17,
    /// The symbol is a base class of a user-defined type.
    BaseClass = 18,
    /// The symbol is a friend of a user-defined type.
    Friend = 19,
    /// The symbol is a function argument.
    FunctionArgType = 20,
    /// The symbol is the end location of the function's prologue code.
    FuncDebugStart = 21,
    /// The symbol is the beginning location of the function's epilogue code.
    FuncDebugEnd = 22,
    /// The symbol is a namespace name active in the current scope.
    UsingNamespace = 23,
    /// The symbol is a virtual table description.
    VtableShape = 24,
    /// The symbol is a virtual table pointer.
    Vtable = 25,
    /// The symbol is a custom symbol; the DIA SDK provides no further
    /// interpretation of these symbols.
    Custom = 26,
    /// The symbol is a thunk used for sharing data between 16-bit and 32-bit
    /// code.
    Thunk = 27,
    /// The symbol is a custom compiler symbol.
    CustomType = 28,
    /// The symbol is in metadata.
    ManagedType = 29,
    /// The symbol is a FORTRAN multi-dimensional array.
    Dimension = 30,
    /// The symbol represents the call site.
    CallSite = 31,
    /// The symbol represents the inline site.
    InlineSite = 32,
    /// The symbol is a base interface.
    BaseInterface = 33,
    /// The symbol is a vector type.
    VectorType = 34,
    /// The symbol is a matrix type.
    MatrixType = 35,
    /// The symbol is a High Level Shader Language type.
    HslType = 36,
    /// The symbol represents Profile-guided optimization (PGO) caller
    /// information.
    Caller = 37,
    /// The symbol represents PGO callee information.
    Callee = 38,
    /// The symbol is an export from a DLL.
    Export = 39,
    /// The symbol represents a heap allocation site, for example a call to
    /// `operator new`.
    HeapAllocationSite = 40,
    /// The symbol is a COFF group.
    CoffGroup = 41,
    /// The symbol represents the inlinee of an inline site (see
    /// [`SymTag::InlineSite`]).
    Inlinee = 42,
    /// The symbol is a tagged union (for example, Rust's `enum` type).
    TaggedUnionCase = 43,
    /// The highest symbol tag value; used as a `Max` sentinel, mirroring the
    Max = 44,
}

impl SymTag {
    /// The raw ABI value of this tag, as used on the COM boundary.
    pub(crate) fn abi(self) -> u32 {
        self as u32
    }

    /// Converts a raw symbol tag value , as returned by the underlying COM method,
    /// to a [`SymTag`].
    ///
    /// Unknown discriminants (for example a tag a newer compiler adds) map to
    /// [`SymTag::Null`].
    pub(crate) fn from_abi(value: u32) -> Self {
        match value {
            0 => SymTag::Null,
            1 => SymTag::Exe,
            2 => SymTag::Compiland,
            3 => SymTag::CompilandDetails,
            4 => SymTag::CompilandEnv,
            5 => SymTag::Function,
            6 => SymTag::Block,
            7 => SymTag::Data,
            8 => SymTag::Annotation,
            9 => SymTag::Label,
            10 => SymTag::PublicSymbol,
            11 => SymTag::Udt,
            12 => SymTag::Enum,
            13 => SymTag::FunctionType,
            14 => SymTag::PointerType,
            15 => SymTag::ArrayType,
            16 => SymTag::BaseType,
            17 => SymTag::Typedef,
            18 => SymTag::BaseClass,
            19 => SymTag::Friend,
            20 => SymTag::FunctionArgType,
            21 => SymTag::FuncDebugStart,
            22 => SymTag::FuncDebugEnd,
            23 => SymTag::UsingNamespace,
            24 => SymTag::VtableShape,
            25 => SymTag::Vtable,
            26 => SymTag::Custom,
            27 => SymTag::Thunk,
            28 => SymTag::CustomType,
            29 => SymTag::ManagedType,
            30 => SymTag::Dimension,
            31 => SymTag::CallSite,
            32 => SymTag::InlineSite,
            33 => SymTag::BaseInterface,
            34 => SymTag::VectorType,
            35 => SymTag::MatrixType,
            36 => SymTag::HslType,
            37 => SymTag::Caller,
            38 => SymTag::Callee,
            39 => SymTag::Export,
            40 => SymTag::HeapAllocationSite,
            41 => SymTag::CoffGroup,
            42 => SymTag::Inlinee,
            43 => SymTag::TaggedUnionCase,
            44 => SymTag::Max,
            _ => SymTag::Null,
        }
    }
}

/// Name-search options for the DIA finders.
///
/// These are flags; combine them with `|`. For example
/// `case_sensitive() | file_name_extension()` treats names as paths.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct NameSearchOptions(u32);

impl NameSearchOptions {
    /// No options are specified.
    pub const NONE: Self = Self(0);

    /// Applies a case-sensitive name match.
    pub const CASE_SENSITIVE: Self = Self(1);

    /// Applies a case-insensitive name match.
    pub const CASE_INSENSITIVE: Self = Self(2);

    /// Treats names as paths and applies a `filename.ext` name match.
    pub const FILE_NAME_EXTENSION: Self = Self(4);

    /// Applies a name match using asterisks (`*`) and question marks (`?`) as
    /// wildcards. Other common regular-expression characters are not
    /// supported.
    pub const WILDCARD: Self = Self(8);

    /// Applies only to symbols that have both undecorated and decorated
    /// names.
    pub const UNDECORATED_NAME: Self = Self(16);

    /// The raw ABI value of these flags.
    pub(crate) fn abi(self) -> u32 {
        self.0
    }
}

impl core::ops::BitOr for NameSearchOptions {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

/// The base (primitive) type of a symbol.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum BasicType {
    NoType = 0,
    Void = 1,
    Char = 2,
    WChar = 3,
    Int = 6,
    UInt = 7,
    Float = 8,
    Bcd = 9,
    Bool = 10,
    Long = 13,
    ULong = 14,
    Currency = 25,
    Date = 26,
    Variant = 27,
    Complex = 28,
    Bit = 29,
    Bstr = 30,
    Hresult = 31,
    Char16 = 32,
    Char32 = 33,
    Char8 = 34,
    Vector = 35,
    /// An unrecognized value.
    Unknown(i32),
}

impl BasicType {
    /// The raw ABI value of this kind, as used on the COM boundary.
    #[allow(dead_code)]
    pub(crate) fn abi(self) -> i32 {
        match self {
            BasicType::NoType => 0,
            BasicType::Void => 1,
            BasicType::Char => 2,
            BasicType::WChar => 3,
            BasicType::Int => 6,
            BasicType::UInt => 7,
            BasicType::Float => 8,
            BasicType::Bcd => 9,
            BasicType::Bool => 10,
            BasicType::Long => 13,
            BasicType::ULong => 14,
            BasicType::Currency => 25,
            BasicType::Date => 26,
            BasicType::Variant => 27,
            BasicType::Complex => 28,
            BasicType::Bit => 29,
            BasicType::Bstr => 30,
            BasicType::Hresult => 31,
            BasicType::Char16 => 32,
            BasicType::Char32 => 33,
            BasicType::Char8 => 34,
            BasicType::Vector => 35,
            Self::Unknown(v) => v,
        }
    }

    /// Converts a raw value to a `BasicType`; unknown values map to `Unknown`.
    pub(crate) fn from_abi(value: i32) -> Self {
        match value {
            0 => BasicType::NoType,
            1 => BasicType::Void,
            2 => BasicType::Char,
            3 => BasicType::WChar,
            6 => BasicType::Int,
            7 => BasicType::UInt,
            8 => BasicType::Float,
            9 => BasicType::Bcd,
            10 => BasicType::Bool,
            13 => BasicType::Long,
            14 => BasicType::ULong,
            25 => BasicType::Currency,
            26 => BasicType::Date,
            27 => BasicType::Variant,
            28 => BasicType::Complex,
            29 => BasicType::Bit,
            30 => BasicType::Bstr,
            31 => BasicType::Hresult,
            32 => BasicType::Char16,
            33 => BasicType::Char32,
            34 => BasicType::Char8,
            35 => BasicType::Vector,
            v => Self::Unknown(v),
        }
    }
}

/// The data kind of a data symbol.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DataKind {
    DataIsUnknown = 0,
    Local = 1,
    StaticLocal = 2,
    Param = 3,
    ObjectPtr = 4,
    FileStatic = 5,
    Global = 6,
    Member = 7,
    StaticMember = 8,
    Constant = 9,
    /// An unrecognized value.
    Unknown(i32),
}

impl DataKind {
    /// The raw ABI value of this kind, as used on the COM boundary.
    #[allow(dead_code)]
    pub(crate) fn abi(self) -> i32 {
        match self {
            DataKind::DataIsUnknown => 0,
            DataKind::Local => 1,
            DataKind::StaticLocal => 2,
            DataKind::Param => 3,
            DataKind::ObjectPtr => 4,
            DataKind::FileStatic => 5,
            DataKind::Global => 6,
            DataKind::Member => 7,
            DataKind::StaticMember => 8,
            DataKind::Constant => 9,
            Self::Unknown(v) => v,
        }
    }

    /// Converts a raw value to a `DataKind`; unknown values map to `Unknown`.
    pub(crate) fn from_abi(value: i32) -> Self {
        match value {
            0 => DataKind::DataIsUnknown,
            1 => DataKind::Local,
            2 => DataKind::StaticLocal,
            3 => DataKind::Param,
            4 => DataKind::ObjectPtr,
            5 => DataKind::FileStatic,
            6 => DataKind::Global,
            7 => DataKind::Member,
            8 => DataKind::StaticMember,
            9 => DataKind::Constant,
            v => Self::Unknown(v),
        }
    }
}

/// Where a data symbol is located.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum LocationType {
    IsNull = 0,
    Static = 1,
    Tls = 2,
    RegisterRelative = 3,
    ThisRelative = 4,
    Enregistered = 5,
    BitField = 6,
    Slot = 7,
    IlRelative = 8,
    InMetadata = 9,
    Constant = 10,
    RegisterRelativeAliasIndirect = 11,
    TypeMax = 12,
    /// An unrecognized value.
    Unknown(i32),
}

impl LocationType {
    /// The raw ABI value of this kind, as used on the COM boundary.
    #[allow(dead_code)]
    pub(crate) fn abi(self) -> i32 {
        match self {
            LocationType::IsNull => 0,
            LocationType::Static => 1,
            LocationType::Tls => 2,
            LocationType::RegisterRelative => 3,
            LocationType::ThisRelative => 4,
            LocationType::Enregistered => 5,
            LocationType::BitField => 6,
            LocationType::Slot => 7,
            LocationType::IlRelative => 8,
            LocationType::InMetadata => 9,
            LocationType::Constant => 10,
            LocationType::RegisterRelativeAliasIndirect => 11,
            LocationType::TypeMax => 12,
            Self::Unknown(v) => v,
        }
    }

    /// Converts a raw value to a `LocationType`; unknown values map to `Unknown`.
    pub(crate) fn from_abi(value: i32) -> Self {
        match value {
            0 => LocationType::IsNull,
            1 => LocationType::Static,
            2 => LocationType::Tls,
            3 => LocationType::RegisterRelative,
            4 => LocationType::ThisRelative,
            5 => LocationType::Enregistered,
            6 => LocationType::BitField,
            7 => LocationType::Slot,
            8 => LocationType::IlRelative,
            9 => LocationType::InMetadata,
            10 => LocationType::Constant,
            11 => LocationType::RegisterRelativeAliasIndirect,
            12 => LocationType::TypeMax,
            v => Self::Unknown(v),
        }
    }
}

/// The kind of a user-defined type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum UdtKind {
    Struct = 0,
    Class = 1,
    Union = 2,
    Interface = 3,
    TaggedUnion = 4,
    /// An unrecognized value.
    Unknown(i32),
}

impl UdtKind {
    /// The raw ABI value of this kind, as used on the COM boundary.
    #[allow(dead_code)]
    pub(crate) fn abi(self) -> i32 {
        match self {
            UdtKind::Struct => 0,
            UdtKind::Class => 1,
            UdtKind::Union => 2,
            UdtKind::Interface => 3,
            UdtKind::TaggedUnion => 4,
            Self::Unknown(v) => v,
        }
    }

    /// Converts a raw value to an `UdtKind`; unknown values map to `Unknown`.
    pub(crate) fn from_abi(value: i32) -> Self {
        match value {
            0 => UdtKind::Struct,
            1 => UdtKind::Class,
            2 => UdtKind::Union,
            3 => UdtKind::Interface,
            4 => UdtKind::TaggedUnion,
            v => Self::Unknown(v),
        }
    }
}

/// The access rights of a symbol member.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Access {
    Private = 1,
    Protected = 2,
    Public = 3,
    /// An unrecognized value.
    Unknown(i32),
}

impl Access {
    /// The raw ABI value of this kind, as used on the COM boundary.
    #[allow(dead_code)]
    pub(crate) fn abi(self) -> i32 {
        match self {
            Access::Private => 1,
            Access::Protected => 2,
            Access::Public => 3,
            Self::Unknown(v) => v,
        }
    }

    /// Converts a raw value to an `Access`; unknown values map to `Unknown`.
    pub(crate) fn from_abi(value: i32) -> Self {
        match value {
            1 => Access::Private,
            2 => Access::Protected,
            3 => Access::Public,
            v => Self::Unknown(v),
        }
    }
}

/// The programming language of a compiland.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Language {
    C = 0,
    Cxx = 1,
    Fortran = 2,
    Masm = 3,
    Pascal = 4,
    Basic = 5,
    Cobol = 6,
    Link = 7,
    Cvtres = 8,
    Cvtpgd = 9,
    CSharp = 10,
    VisualBasic = 11,
    Ilasm = 12,
    Java = 13,
    JScript = 14,
    Msil = 15,
    Hlsl = 16,
    ObjC = 17,
    ObjCxx = 18,
    Swift = 19,
    AliasObj = 20,
    Rust = 21,
    Go = 22,
    /// An unrecognized value.
    Unknown(i32),
}

impl Language {
    /// The raw ABI value of this kind, as used on the COM boundary.
    #[allow(dead_code)]
    pub(crate) fn abi(self) -> i32 {
        match self {
            Language::C => 0,
            Language::Cxx => 1,
            Language::Fortran => 2,
            Language::Masm => 3,
            Language::Pascal => 4,
            Language::Basic => 5,
            Language::Cobol => 6,
            Language::Link => 7,
            Language::Cvtres => 8,
            Language::Cvtpgd => 9,
            Language::CSharp => 10,
            Language::VisualBasic => 11,
            Language::Ilasm => 12,
            Language::Java => 13,
            Language::JScript => 14,
            Language::Msil => 15,
            Language::Hlsl => 16,
            Language::ObjC => 17,
            Language::ObjCxx => 18,
            Language::Swift => 19,
            Language::AliasObj => 20,
            Language::Rust => 21,
            Language::Go => 22,
            Self::Unknown(v) => v,
        }
    }

    /// Converts a raw value to a `Language`; unknown values map to `Unknown`.
    pub(crate) fn from_abi(value: i32) -> Self {
        match value {
            0 => Language::C,
            1 => Language::Cxx,
            2 => Language::Fortran,
            3 => Language::Masm,
            4 => Language::Pascal,
            5 => Language::Basic,
            6 => Language::Cobol,
            7 => Language::Link,
            8 => Language::Cvtres,
            9 => Language::Cvtpgd,
            10 => Language::CSharp,
            11 => Language::VisualBasic,
            12 => Language::Ilasm,
            13 => Language::Java,
            14 => Language::JScript,
            15 => Language::Msil,
            16 => Language::Hlsl,
            17 => Language::ObjC,
            18 => Language::ObjCxx,
            19 => Language::Swift,
            20 => Language::AliasObj,
            21 => Language::Rust,
            22 => Language::Go,
            v => Self::Unknown(v),
        }
    }
}

/// The calling convention of a function.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CallingConvention {
    NearC = 0,
    FarC = 1,
    NearPascal = 2,
    FarPascal = 3,
    NearFast = 4,
    FarFast = 5,
    Skipped = 6,
    NearStd = 7,
    FarStd = 8,
    NearSys = 9,
    FarSys = 10,
    ThisCall = 11,
    MipsCall = 12,
    Generic = 13,
    AlphaCall = 14,
    PpcCall = 15,
    ShCall = 16,
    ArmCall = 17,
    Am33Call = 18,
    TriCall = 19,
    Sh5Call = 20,
    M32rCall = 21,
    ClrCall = 22,
    Inline = 23,
    NearVector = 24,
    Swift = 25,
    PreserveNone = 32,
    Reserved = 33,
    /// An unrecognized value.
    Unknown(i32),
}

impl CallingConvention {
    /// The raw ABI value of this kind, as used on the COM boundary.
    #[allow(dead_code)]
    pub(crate) fn abi(self) -> i32 {
        match self {
            CallingConvention::NearC => 0,
            CallingConvention::FarC => 1,
            CallingConvention::NearPascal => 2,
            CallingConvention::FarPascal => 3,
            CallingConvention::NearFast => 4,
            CallingConvention::FarFast => 5,
            CallingConvention::Skipped => 6,
            CallingConvention::NearStd => 7,
            CallingConvention::FarStd => 8,
            CallingConvention::NearSys => 9,
            CallingConvention::FarSys => 10,
            CallingConvention::ThisCall => 11,
            CallingConvention::MipsCall => 12,
            CallingConvention::Generic => 13,
            CallingConvention::AlphaCall => 14,
            CallingConvention::PpcCall => 15,
            CallingConvention::ShCall => 16,
            CallingConvention::ArmCall => 17,
            CallingConvention::Am33Call => 18,
            CallingConvention::TriCall => 19,
            CallingConvention::Sh5Call => 20,
            CallingConvention::M32rCall => 21,
            CallingConvention::ClrCall => 22,
            CallingConvention::Inline => 23,
            CallingConvention::NearVector => 24,
            CallingConvention::Swift => 25,
            CallingConvention::PreserveNone => 32,
            CallingConvention::Reserved => 33,
            Self::Unknown(v) => v,
        }
    }

    /// Converts a raw value to a `CallingConvention`; unknown values map to `Unknown`.
    pub(crate) fn from_abi(value: i32) -> Self {
        match value {
            0 => CallingConvention::NearC,
            1 => CallingConvention::FarC,
            2 => CallingConvention::NearPascal,
            3 => CallingConvention::FarPascal,
            4 => CallingConvention::NearFast,
            5 => CallingConvention::FarFast,
            6 => CallingConvention::Skipped,
            7 => CallingConvention::NearStd,
            8 => CallingConvention::FarStd,
            9 => CallingConvention::NearSys,
            10 => CallingConvention::FarSys,
            11 => CallingConvention::ThisCall,
            12 => CallingConvention::MipsCall,
            13 => CallingConvention::Generic,
            14 => CallingConvention::AlphaCall,
            15 => CallingConvention::PpcCall,
            16 => CallingConvention::ShCall,
            17 => CallingConvention::ArmCall,
            18 => CallingConvention::Am33Call,
            19 => CallingConvention::TriCall,
            20 => CallingConvention::Sh5Call,
            21 => CallingConvention::M32rCall,
            22 => CallingConvention::ClrCall,
            23 => CallingConvention::Inline,
            24 => CallingConvention::NearVector,
            25 => CallingConvention::Swift,
            32 => CallingConvention::PreserveNone,
            33 => CallingConvention::Reserved,
            v => Self::Unknown(v),
        }
    }
}

/// The kind of a coroutine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CoroutineKind {
    None = 0,
    Primary = 1,
    Init = 2,
    Resume = 3,
    Destroy = 4,
    /// An unrecognized value.
    Unknown(i32),
}

impl CoroutineKind {
    /// The raw ABI value of this kind, as used on the COM boundary.
    #[allow(dead_code)]
    pub(crate) fn abi(self) -> i32 {
        match self {
            CoroutineKind::None => 0,
            CoroutineKind::Primary => 1,
            CoroutineKind::Init => 2,
            CoroutineKind::Resume => 3,
            CoroutineKind::Destroy => 4,
            Self::Unknown(v) => v,
        }
    }

    /// Converts a raw value to a `CoroutineKind`; unknown values map to `Unknown`.
    pub(crate) fn from_abi(value: i32) -> Self {
        match value {
            0 => CoroutineKind::None,
            1 => CoroutineKind::Primary,
            2 => CoroutineKind::Init,
            3 => CoroutineKind::Resume,
            4 => CoroutineKind::Destroy,
            v => Self::Unknown(v),
        }
    }
}

/// The thunk ordinal kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ThunkOrdinal {
    NoType = 0,
    Adjustor = 1,
    VCall = 2,
    Pcode = 3,
    Load = 4,
    TrampIncremental = 5,
    TrampBranchIsland = 6,
    TrampStrictIcf = 7,
    TrampArm64XSameAddress = 8,
    TrampFuncOverriding = 9,
    /// An unrecognized value.
    Unknown(i32),
}

impl ThunkOrdinal {
    /// The raw ABI value of this kind, as used on the COM boundary.
    #[allow(dead_code)]
    pub(crate) fn abi(self) -> i32 {
        match self {
            ThunkOrdinal::NoType => 0,
            ThunkOrdinal::Adjustor => 1,
            ThunkOrdinal::VCall => 2,
            ThunkOrdinal::Pcode => 3,
            ThunkOrdinal::Load => 4,
            ThunkOrdinal::TrampIncremental => 5,
            ThunkOrdinal::TrampBranchIsland => 6,
            ThunkOrdinal::TrampStrictIcf => 7,
            ThunkOrdinal::TrampArm64XSameAddress => 8,
            ThunkOrdinal::TrampFuncOverriding => 9,
            Self::Unknown(v) => v,
        }
    }

    /// Converts a raw value to a `ThunkOrdinal`; unknown values map to `Unknown`.
    pub(crate) fn from_abi(value: i32) -> Self {
        match value {
            0 => ThunkOrdinal::NoType,
            1 => ThunkOrdinal::Adjustor,
            2 => ThunkOrdinal::VCall,
            3 => ThunkOrdinal::Pcode,
            4 => ThunkOrdinal::Load,
            5 => ThunkOrdinal::TrampIncremental,
            6 => ThunkOrdinal::TrampBranchIsland,
            7 => ThunkOrdinal::TrampStrictIcf,
            8 => ThunkOrdinal::TrampArm64XSameAddress,
            9 => ThunkOrdinal::TrampFuncOverriding,
            v => Self::Unknown(v),
        }
    }
}

/// The checksum algorithm of a source file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ChecksumType {
    None = 0,
    Md5 = 1,
    Sha1 = 2,
    Sha256 = 3,
    Sha384 = 4,
    Sha512 = 5,
    /// An unrecognized value.
    Unknown(i32),
}

impl ChecksumType {
    /// The raw ABI value of this kind, as used on the COM boundary.
    #[allow(dead_code)]
    pub(crate) fn abi(self) -> i32 {
        match self {
            ChecksumType::None => 0,
            ChecksumType::Md5 => 1,
            ChecksumType::Sha1 => 2,
            ChecksumType::Sha256 => 3,
            ChecksumType::Sha384 => 4,
            ChecksumType::Sha512 => 5,
            Self::Unknown(v) => v,
        }
    }

    /// Converts a raw value to a `ChecksumType`; unknown values map to `Unknown`.
    #[allow(dead_code)]
    pub(crate) fn from_abi(value: i32) -> Self {
        match value {
            0 => ChecksumType::None,
            1 => ChecksumType::Md5,
            2 => ChecksumType::Sha1,
            3 => ChecksumType::Sha256,
            4 => ChecksumType::Sha384,
            5 => ChecksumType::Sha512,
            v => Self::Unknown(v),
        }
    }
}

/// The type of a stack frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum StackFrameType {
    FrameTypeUnknown = -1,
    Fpo = 0,
    Trap = 1,
    Tss = 2,
    Standard = 3,
    FrameData = 4,
    /// An unrecognized value.
    Unknown(i32),
}

impl StackFrameType {
    /// The raw ABI value of this kind, as used on the COM boundary.
    #[allow(dead_code)]
    pub(crate) fn abi(self) -> i32 {
        match self {
            StackFrameType::FrameTypeUnknown => -1,
            StackFrameType::Fpo => 0,
            StackFrameType::Trap => 1,
            StackFrameType::Tss => 2,
            StackFrameType::Standard => 3,
            StackFrameType::FrameData => 4,
            Self::Unknown(v) => v,
        }
    }

    /// Converts a raw value to a `StackFrameType`; unknown values map to `Unknown`.
    pub(crate) fn from_abi(value: i32) -> Self {
        match value {
            -1 => StackFrameType::FrameTypeUnknown,
            0 => StackFrameType::Fpo,
            1 => StackFrameType::Trap,
            2 => StackFrameType::Tss,
            3 => StackFrameType::Standard,
            4 => StackFrameType::FrameData,
            v => Self::Unknown(v),
        }
    }
}

/// The memory region addressed by a read/write.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MemoryType {
    Any = -1,
    Code = 0,
    Data = 1,
    Stack = 2,
    CodeOnHeap = 3,
    /// An unrecognized value.
    Unknown(i32),
}

impl MemoryType {
    /// The raw ABI value of this kind, as used on the COM boundary.
    #[allow(dead_code)]
    pub(crate) fn abi(self) -> i32 {
        match self {
            MemoryType::Any => -1,
            MemoryType::Code => 0,
            MemoryType::Data => 1,
            MemoryType::Stack => 2,
            MemoryType::CodeOnHeap => 3,
            Self::Unknown(v) => v,
        }
    }

    /// Converts a raw value to a `MemoryType`; unknown values map to `Unknown`.
    pub(crate) fn from_abi(value: i32) -> Self {
        match value {
            -1 => MemoryType::Any,
            0 => MemoryType::Code,
            1 => MemoryType::Data,
            2 => MemoryType::Stack,
            3 => MemoryType::CodeOnHeap,
            v => Self::Unknown(v),
        }
    }
}

/// The target machine / CPU of a compiland.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MachineType {
    M8080 = 0,
    M8086 = 1,
    M80286 = 2,
    M80386 = 3,
    M80486 = 4,
    Pentium = 5,
    PentiumPro = 6,
    PentiumIii = 7,
    Mips = 16,
    Mips16 = 17,
    Mips32 = 18,
    Mips64 = 19,
    MipsI = 20,
    MipsIi = 21,
    MipsIii = 22,
    MipsIv = 23,
    MipsV = 24,
    M68000 = 32,
    M68010 = 33,
    M68020 = 34,
    M68030 = 35,
    M68040 = 36,
    Alpha = 48,
    Alpha21164 = 49,
    Alpha21164A = 50,
    Alpha21264 = 51,
    Alpha21364 = 52,
    Ppc601 = 64,
    Ppc603 = 65,
    Ppc604 = 66,
    Ppc620 = 67,
    PpcFp = 68,
    PpcBe = 69,
    Sh3 = 80,
    Sh3e = 81,
    Sh3Dsp = 82,
    Sh4 = 83,
    ShMedia = 84,
    Arm3 = 96,
    Arm4 = 97,
    Arm4T = 98,
    Arm5 = 99,
    Arm5T = 100,
    Arm6 = 101,
    ArmXmac = 102,
    ArmWmmx = 103,
    Arm7 = 104,
    Omni = 112,
    Ia64 = 128,
    Ia64Two = 129,
    CommonLanguageRuntime = 144,
    Am33 = 160,
    M32R = 176,
    Tricore = 192,
    Amd64 = 208,
    Ebc = 224,
    Thumb = 240,
    ArmNt = 244,
    Arm64 = 246,
    HybridX86Arm64 = 247,
    Arm64Ec = 248,
    Arm64X = 249,
    CflUnknown = 255,
    D3d11Shader = 256,
    /// An unrecognized value.
    Unknown(i32),
}

impl MachineType {
    /// The raw ABI value of this kind, as used on the COM boundary.
    #[allow(dead_code)]
    pub(crate) fn abi(self) -> i32 {
        match self {
            MachineType::M8080 => 0,
            MachineType::M8086 => 1,
            MachineType::M80286 => 2,
            MachineType::M80386 => 3,
            MachineType::M80486 => 4,
            MachineType::Pentium => 5,
            MachineType::PentiumPro => 6,
            MachineType::PentiumIii => 7,
            MachineType::Mips => 16,
            MachineType::Mips16 => 17,
            MachineType::Mips32 => 18,
            MachineType::Mips64 => 19,
            MachineType::MipsI => 20,
            MachineType::MipsIi => 21,
            MachineType::MipsIii => 22,
            MachineType::MipsIv => 23,
            MachineType::MipsV => 24,
            MachineType::M68000 => 32,
            MachineType::M68010 => 33,
            MachineType::M68020 => 34,
            MachineType::M68030 => 35,
            MachineType::M68040 => 36,
            MachineType::Alpha => 48,
            MachineType::Alpha21164 => 49,
            MachineType::Alpha21164A => 50,
            MachineType::Alpha21264 => 51,
            MachineType::Alpha21364 => 52,
            MachineType::Ppc601 => 64,
            MachineType::Ppc603 => 65,
            MachineType::Ppc604 => 66,
            MachineType::Ppc620 => 67,
            MachineType::PpcFp => 68,
            MachineType::PpcBe => 69,
            MachineType::Sh3 => 80,
            MachineType::Sh3e => 81,
            MachineType::Sh3Dsp => 82,
            MachineType::Sh4 => 83,
            MachineType::ShMedia => 84,
            MachineType::Arm3 => 96,
            MachineType::Arm4 => 97,
            MachineType::Arm4T => 98,
            MachineType::Arm5 => 99,
            MachineType::Arm5T => 100,
            MachineType::Arm6 => 101,
            MachineType::ArmXmac => 102,
            MachineType::ArmWmmx => 103,
            MachineType::Arm7 => 104,
            MachineType::Omni => 112,
            MachineType::Ia64 => 128,
            MachineType::Ia64Two => 129,
            MachineType::CommonLanguageRuntime => 144,
            MachineType::Am33 => 160,
            MachineType::M32R => 176,
            MachineType::Tricore => 192,
            MachineType::Amd64 => 208,
            MachineType::Ebc => 224,
            MachineType::Thumb => 240,
            MachineType::ArmNt => 244,
            MachineType::Arm64 => 246,
            MachineType::HybridX86Arm64 => 247,
            MachineType::Arm64Ec => 248,
            MachineType::Arm64X => 249,
            MachineType::CflUnknown => 255,
            MachineType::D3d11Shader => 256,
            Self::Unknown(v) => v,
        }
    }

    /// Converts a raw value to a `MachineType`; unknown values map to `Unknown`.
    pub(crate) fn from_abi(value: i32) -> Self {
        match value {
            0 => MachineType::M8080,
            1 => MachineType::M8086,
            2 => MachineType::M80286,
            3 => MachineType::M80386,
            4 => MachineType::M80486,
            5 => MachineType::Pentium,
            6 => MachineType::PentiumPro,
            7 => MachineType::PentiumIii,
            16 => MachineType::Mips,
            17 => MachineType::Mips16,
            18 => MachineType::Mips32,
            19 => MachineType::Mips64,
            20 => MachineType::MipsI,
            21 => MachineType::MipsIi,
            22 => MachineType::MipsIii,
            23 => MachineType::MipsIv,
            24 => MachineType::MipsV,
            32 => MachineType::M68000,
            33 => MachineType::M68010,
            34 => MachineType::M68020,
            35 => MachineType::M68030,
            36 => MachineType::M68040,
            48 => MachineType::Alpha,
            49 => MachineType::Alpha21164,
            50 => MachineType::Alpha21164A,
            51 => MachineType::Alpha21264,
            52 => MachineType::Alpha21364,
            64 => MachineType::Ppc601,
            65 => MachineType::Ppc603,
            66 => MachineType::Ppc604,
            67 => MachineType::Ppc620,
            68 => MachineType::PpcFp,
            69 => MachineType::PpcBe,
            80 => MachineType::Sh3,
            81 => MachineType::Sh3e,
            82 => MachineType::Sh3Dsp,
            83 => MachineType::Sh4,
            84 => MachineType::ShMedia,
            96 => MachineType::Arm3,
            97 => MachineType::Arm4,
            98 => MachineType::Arm4T,
            99 => MachineType::Arm5,
            100 => MachineType::Arm5T,
            101 => MachineType::Arm6,
            102 => MachineType::ArmXmac,
            103 => MachineType::ArmWmmx,
            104 => MachineType::Arm7,
            112 => MachineType::Omni,
            128 => MachineType::Ia64,
            129 => MachineType::Ia64Two,
            144 => MachineType::CommonLanguageRuntime,
            160 => MachineType::Am33,
            176 => MachineType::M32R,
            192 => MachineType::Tricore,
            208 => MachineType::Amd64,
            224 => MachineType::Ebc,
            240 => MachineType::Thumb,
            244 => MachineType::ArmNt,
            246 => MachineType::Arm64,
            247 => MachineType::HybridX86Arm64,
            248 => MachineType::Arm64Ec,
            249 => MachineType::Arm64X,
            255 => MachineType::CflUnknown,
            256 => MachineType::D3d11Shader,
            v => Self::Unknown(v),
        }
    }
}

/// The kind of a scalable vector (SVE) type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ScalableVectorType {
    None = 0,
    Vector = 1,
    Predicate = 2,
    /// An unrecognized value.
    Unknown(i32),
}

impl ScalableVectorType {
    /// The raw ABI value of this kind, as used on the COM boundary.
    #[allow(dead_code)]
    pub(crate) fn abi(self) -> i32 {
        match self {
            ScalableVectorType::None => 0,
            ScalableVectorType::Vector => 1,
            ScalableVectorType::Predicate => 2,
            Self::Unknown(v) => v,
        }
    }

    /// Converts a raw value to a `ScalableVectorType`; unknown values map to `Unknown`.
    pub(crate) fn from_abi(value: i32) -> Self {
        match value {
            0 => ScalableVectorType::None,
            1 => ScalableVectorType::Vector,
            2 => ScalableVectorType::Predicate,
            v => Self::Unknown(v),
        }
    }
}

/// The association kind of a coroutine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum AssociationKind {
    None = 0,
    Coroutine = 1,
    /// An unrecognized value.
    Unknown(i32),
}

impl AssociationKind {
    /// The raw ABI value of this kind, as used on the COM boundary.
    #[allow(dead_code)]
    pub(crate) fn abi(self) -> i32 {
        match self {
            AssociationKind::None => 0,
            AssociationKind::Coroutine => 1,
            Self::Unknown(v) => v,
        }
    }

    /// Converts a raw value to an `AssociationKind`; unknown values map to `Unknown`.
    pub(crate) fn from_abi(value: i32) -> Self {
        match value {
            0 => AssociationKind::None,
            1 => AssociationKind::Coroutine,
            v => Self::Unknown(v),
        }
    }
}

/// The built-in kind of an HLSL symbol.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum BuiltinKind {
    Invalid = 0,
    HlslInterfacePointer = 512,
    HlslTexture1d = 513,
    HlslTexture1dArray = 514,
    HlslTexture2d = 515,
    HlslTexture2dArray = 516,
    HlslTexture3d = 517,
    HlslTextureCube = 518,
    HlslTextureCubeArray = 519,
    HlslTexture2dMs = 520,
    HlslTexture2dMsArray = 521,
    HlslSampler = 522,
    HlslSamplerComparison = 523,
    HlslBuffer = 524,
    HlslPointStream = 525,
    HlslLineStream = 526,
    HlslTriangleStream = 527,
    HlslInputPatch = 528,
    HlslOutputPatch = 529,
    HlslRwTexture1d = 530,
    HlslRwTexture1dArray = 531,
    HlslRwTexture2d = 532,
    HlslRwTexture2dArray = 533,
    HlslRwTexture3d = 534,
    HlslRwBuffer = 535,
    HlslByteAddressBuffer = 536,
    HlslRwByteAddressBuffer = 537,
    HlslStructuredBuffer = 538,
    HlslRwStructuredBuffer = 539,
    HlslAppendStructuredBuffer = 540,
    HlslConsumeStructuredBuffer = 541,
    HlslMin8Float = 542,
    HlslMin10Float = 543,
    HlslMin16Float = 544,
    HlslMin12Int = 545,
    HlslMin16Int = 546,
    HlslMin16Uint = 547,
    HlslConstantBuffer = 548,
    /// An unrecognized value.
    Unknown(i32),
}

impl BuiltinKind {
    /// The raw ABI value of this kind, as used on the COM boundary.
    #[allow(dead_code)]
    pub(crate) fn abi(self) -> i32 {
        match self {
            BuiltinKind::Invalid => 0,
            BuiltinKind::HlslInterfacePointer => 512,
            BuiltinKind::HlslTexture1d => 513,
            BuiltinKind::HlslTexture1dArray => 514,
            BuiltinKind::HlslTexture2d => 515,
            BuiltinKind::HlslTexture2dArray => 516,
            BuiltinKind::HlslTexture3d => 517,
            BuiltinKind::HlslTextureCube => 518,
            BuiltinKind::HlslTextureCubeArray => 519,
            BuiltinKind::HlslTexture2dMs => 520,
            BuiltinKind::HlslTexture2dMsArray => 521,
            BuiltinKind::HlslSampler => 522,
            BuiltinKind::HlslSamplerComparison => 523,
            BuiltinKind::HlslBuffer => 524,
            BuiltinKind::HlslPointStream => 525,
            BuiltinKind::HlslLineStream => 526,
            BuiltinKind::HlslTriangleStream => 527,
            BuiltinKind::HlslInputPatch => 528,
            BuiltinKind::HlslOutputPatch => 529,
            BuiltinKind::HlslRwTexture1d => 530,
            BuiltinKind::HlslRwTexture1dArray => 531,
            BuiltinKind::HlslRwTexture2d => 532,
            BuiltinKind::HlslRwTexture2dArray => 533,
            BuiltinKind::HlslRwTexture3d => 534,
            BuiltinKind::HlslRwBuffer => 535,
            BuiltinKind::HlslByteAddressBuffer => 536,
            BuiltinKind::HlslRwByteAddressBuffer => 537,
            BuiltinKind::HlslStructuredBuffer => 538,
            BuiltinKind::HlslRwStructuredBuffer => 539,
            BuiltinKind::HlslAppendStructuredBuffer => 540,
            BuiltinKind::HlslConsumeStructuredBuffer => 541,
            BuiltinKind::HlslMin8Float => 542,
            BuiltinKind::HlslMin10Float => 543,
            BuiltinKind::HlslMin16Float => 544,
            BuiltinKind::HlslMin12Int => 545,
            BuiltinKind::HlslMin16Int => 546,
            BuiltinKind::HlslMin16Uint => 547,
            BuiltinKind::HlslConstantBuffer => 548,
            Self::Unknown(v) => v,
        }
    }

    /// Converts a raw value to a `BuiltinKind`; unknown values map to `Unknown`.
    pub(crate) fn from_abi(value: i32) -> Self {
        match value {
            0 => BuiltinKind::Invalid,
            512 => BuiltinKind::HlslInterfacePointer,
            513 => BuiltinKind::HlslTexture1d,
            514 => BuiltinKind::HlslTexture1dArray,
            515 => BuiltinKind::HlslTexture2d,
            516 => BuiltinKind::HlslTexture2dArray,
            517 => BuiltinKind::HlslTexture3d,
            518 => BuiltinKind::HlslTextureCube,
            519 => BuiltinKind::HlslTextureCubeArray,
            520 => BuiltinKind::HlslTexture2dMs,
            521 => BuiltinKind::HlslTexture2dMsArray,
            522 => BuiltinKind::HlslSampler,
            523 => BuiltinKind::HlslSamplerComparison,
            524 => BuiltinKind::HlslBuffer,
            525 => BuiltinKind::HlslPointStream,
            526 => BuiltinKind::HlslLineStream,
            527 => BuiltinKind::HlslTriangleStream,
            528 => BuiltinKind::HlslInputPatch,
            529 => BuiltinKind::HlslOutputPatch,
            530 => BuiltinKind::HlslRwTexture1d,
            531 => BuiltinKind::HlslRwTexture1dArray,
            532 => BuiltinKind::HlslRwTexture2d,
            533 => BuiltinKind::HlslRwTexture2dArray,
            534 => BuiltinKind::HlslRwTexture3d,
            535 => BuiltinKind::HlslRwBuffer,
            536 => BuiltinKind::HlslByteAddressBuffer,
            537 => BuiltinKind::HlslRwByteAddressBuffer,
            538 => BuiltinKind::HlslStructuredBuffer,
            539 => BuiltinKind::HlslRwStructuredBuffer,
            540 => BuiltinKind::HlslAppendStructuredBuffer,
            541 => BuiltinKind::HlslConsumeStructuredBuffer,
            542 => BuiltinKind::HlslMin8Float,
            543 => BuiltinKind::HlslMin10Float,
            544 => BuiltinKind::HlslMin16Float,
            545 => BuiltinKind::HlslMin12Int,
            546 => BuiltinKind::HlslMin16Int,
            547 => BuiltinKind::HlslMin16Uint,
            548 => BuiltinKind::HlslConstantBuffer,
            v => Self::Unknown(v),
        }
    }
}

/// The HLSL memory space of a resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum HlMemorySpace {
    Data = 0,
    Sampler = 1,
    Resource = 2,
    RwResource = 3,
    Max = 15,
    /// An unrecognized value.
    Unknown(i32),
}

impl HlMemorySpace {
    /// The raw ABI value of this kind, as used on the COM boundary.
    #[allow(dead_code)]
    pub(crate) fn abi(self) -> i32 {
        match self {
            HlMemorySpace::Data => 0,
            HlMemorySpace::Sampler => 1,
            HlMemorySpace::Resource => 2,
            HlMemorySpace::RwResource => 3,
            HlMemorySpace::Max => 15,
            Self::Unknown(v) => v,
        }
    }

    /// Converts a raw value to a `HlMemorySpace`; unknown values map to `Unknown`.
    pub(crate) fn from_abi(value: i32) -> Self {
        match value {
            0 => HlMemorySpace::Data,
            1 => HlMemorySpace::Sampler,
            2 => HlMemorySpace::Resource,
            3 => HlMemorySpace::RwResource,
            15 => HlMemorySpace::Max,
            v => Self::Unknown(v),
        }
    }
}

/// An HLSL register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum HlRegister {
    Temp = 0,
    Input = 1,
    Output = 2,
    IndexableTemp = 3,
    Immediate32 = 4,
    Immediate64 = 5,
    Sampler = 6,
    Resource = 7,
    ConstantBuffer = 8,
    ImmediateConstantBuffer = 9,
    Label = 10,
    InputPrimitiveId = 11,
    OutputDepth = 12,
    Null = 13,
    Rasterizer = 14,
    OutputCoverageMask = 15,
    Stream = 16,
    FunctionBody = 17,
    FunctionTable = 18,
    Interface = 19,
    FunctionInput = 20,
    FunctionOutput = 21,
    OutputControlPointId = 22,
    InputForkInstanceId = 23,
    InputJoinInstanceId = 24,
    InputControlPoint = 25,
    OutputControlPoint = 26,
    InputPatchConstant = 27,
    InputDomainPoint = 28,
    ThisPointer = 29,
    UnorderedAccessView = 30,
    ThreadGroupSharedMemory = 31,
    InputThreadId = 32,
    InputThreadGroupId = 33,
    InputThreadIdInGroup = 34,
    InputCoverageMask = 35,
    InputThreadIdInGroupFlattened = 36,
    GsInstanceId = 37,
    OutputDepthGreaterEqual = 38,
    OutputDepthLessEqual = 39,
    CycleCounter = 40,
    /// An unrecognized value.
    Unknown(i32),
}

impl HlRegister {
    /// The raw ABI value of this kind, as used on the COM boundary.
    #[allow(dead_code)]
    pub(crate) fn abi(self) -> i32 {
        match self {
            HlRegister::Temp => 0,
            HlRegister::Input => 1,
            HlRegister::Output => 2,
            HlRegister::IndexableTemp => 3,
            HlRegister::Immediate32 => 4,
            HlRegister::Immediate64 => 5,
            HlRegister::Sampler => 6,
            HlRegister::Resource => 7,
            HlRegister::ConstantBuffer => 8,
            HlRegister::ImmediateConstantBuffer => 9,
            HlRegister::Label => 10,
            HlRegister::InputPrimitiveId => 11,
            HlRegister::OutputDepth => 12,
            HlRegister::Null => 13,
            HlRegister::Rasterizer => 14,
            HlRegister::OutputCoverageMask => 15,
            HlRegister::Stream => 16,
            HlRegister::FunctionBody => 17,
            HlRegister::FunctionTable => 18,
            HlRegister::Interface => 19,
            HlRegister::FunctionInput => 20,
            HlRegister::FunctionOutput => 21,
            HlRegister::OutputControlPointId => 22,
            HlRegister::InputForkInstanceId => 23,
            HlRegister::InputJoinInstanceId => 24,
            HlRegister::InputControlPoint => 25,
            HlRegister::OutputControlPoint => 26,
            HlRegister::InputPatchConstant => 27,
            HlRegister::InputDomainPoint => 28,
            HlRegister::ThisPointer => 29,
            HlRegister::UnorderedAccessView => 30,
            HlRegister::ThreadGroupSharedMemory => 31,
            HlRegister::InputThreadId => 32,
            HlRegister::InputThreadGroupId => 33,
            HlRegister::InputThreadIdInGroup => 34,
            HlRegister::InputCoverageMask => 35,
            HlRegister::InputThreadIdInGroupFlattened => 36,
            HlRegister::GsInstanceId => 37,
            HlRegister::OutputDepthGreaterEqual => 38,
            HlRegister::OutputDepthLessEqual => 39,
            HlRegister::CycleCounter => 40,
            Self::Unknown(v) => v,
        }
    }

    /// Converts a raw value to a `HlRegister`; unknown values map to `Unknown`.
    pub(crate) fn from_abi(value: i32) -> Self {
        match value {
            0 => HlRegister::Temp,
            1 => HlRegister::Input,
            2 => HlRegister::Output,
            3 => HlRegister::IndexableTemp,
            4 => HlRegister::Immediate32,
            5 => HlRegister::Immediate64,
            6 => HlRegister::Sampler,
            7 => HlRegister::Resource,
            8 => HlRegister::ConstantBuffer,
            9 => HlRegister::ImmediateConstantBuffer,
            10 => HlRegister::Label,
            11 => HlRegister::InputPrimitiveId,
            12 => HlRegister::OutputDepth,
            13 => HlRegister::Null,
            14 => HlRegister::Rasterizer,
            15 => HlRegister::OutputCoverageMask,
            16 => HlRegister::Stream,
            17 => HlRegister::FunctionBody,
            18 => HlRegister::FunctionTable,
            19 => HlRegister::Interface,
            20 => HlRegister::FunctionInput,
            21 => HlRegister::FunctionOutput,
            22 => HlRegister::OutputControlPointId,
            23 => HlRegister::InputForkInstanceId,
            24 => HlRegister::InputJoinInstanceId,
            25 => HlRegister::InputControlPoint,
            26 => HlRegister::OutputControlPoint,
            27 => HlRegister::InputPatchConstant,
            28 => HlRegister::InputDomainPoint,
            29 => HlRegister::ThisPointer,
            30 => HlRegister::UnorderedAccessView,
            31 => HlRegister::ThreadGroupSharedMemory,
            32 => HlRegister::InputThreadId,
            33 => HlRegister::InputThreadGroupId,
            34 => HlRegister::InputThreadIdInGroup,
            35 => HlRegister::InputCoverageMask,
            36 => HlRegister::InputThreadIdInGroupFlattened,
            37 => HlRegister::GsInstanceId,
            38 => HlRegister::OutputDepthGreaterEqual,
            39 => HlRegister::OutputDepthLessEqual,
            40 => HlRegister::CycleCounter,
            v => Self::Unknown(v),
        }
    }
}

/// The target CPU / machine type, as used by
/// [`StackWalker::get_enum_frames2`](crate::stack::StackWalker::get_enum_frames2)
#[non_exhaustive]
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuType {
    /// 0x00 (CV_CFL_8080).
    C8080 = 0x0,
    /// 0x01 (CV_CFL_8086).
    C8086 = 0x1,
    /// 0x02 (CV_CFL_80286).
    C80286 = 0x2,
    /// 0x03 (CV_CFL_80386).
    C80386 = 0x3,
    /// 0x04 (CV_CFL_80486).
    C80486 = 0x4,
    /// 0x05 (CV_CFL_PENTIUM).
    Pentium = 0x5,
    /// 0x06 (CV_CFL_PENTIUMII / CV_CFL_PENTIUMPRO).
    PentiumII = 0x6,
    /// 0x07 (CV_CFL_PENTIUMIII).
    PentiumIII = 0x7,
    /// 0x10 (CV_CFL_MIPS / CV_CFL_MIPSR4000).
    Mips = 0x10,
    /// 0x11 (CV_CFL_MIPS16).
    Mips16 = 0x11,
    /// 0x12 (CV_CFL_MIPS32).
    Mips32 = 0x12,
    /// 0x13 (CV_CFL_MIPS64).
    Mips64 = 0x13,
    /// 0x14 (CV_CFL_MIPSI).
    MipsI = 0x14,
    /// 0x15 (CV_CFL_MIPSII).
    MipsII = 0x15,
    /// 0x16 (CV_CFL_MIPSIII).
    MipsIII = 0x16,
    /// 0x17 (CV_CFL_MIPSIV).
    MipsIV = 0x17,
    /// 0x18 (CV_CFL_MIPSV).
    MipsV = 0x18,
    /// 0x20 (CV_CFL_M68000).
    M68000 = 0x20,
    /// 0x21 (CV_CFL_M68010).
    M68010 = 0x21,
    /// 0x22 (CV_CFL_M68020).
    M68020 = 0x22,
    /// 0x23 (CV_CFL_M68030).
    M68030 = 0x23,
    /// 0x24 (CV_CFL_M68040).
    M68040 = 0x24,
    /// 0x30 (CV_CFL_ALPHA / CV_CFL_ALPHA_21064).
    Alpha = 0x30,
    /// 0x31 (CV_CFL_ALPHA_21164).
    Alpha21164 = 0x31,
    /// 0x32 (CV_CFL_ALPHA_21164A).
    Alpha21164A = 0x32,
    /// 0x33 (CV_CFL_ALPHA_21264).
    Alpha21264 = 0x33,
    /// 0x34 (CV_CFL_ALPHA_21364).
    Alpha21364 = 0x34,
    /// 0x40 (CV_CFL_PPC601).
    Ppc601 = 0x40,
    /// 0x41 (CV_CFL_PPC603).
    Ppc603 = 0x41,
    /// 0x42 (CV_CFL_PPC604).
    Ppc604 = 0x42,
    /// 0x43 (CV_CFL_PPC620).
    Ppc620 = 0x43,
    /// 0x44 (CV_CFL_PPCFP).
    PpcFp = 0x44,
    /// 0x45 (CV_CFL_PPCBE).
    PpcBe = 0x45,
    /// 0x50 (CV_CFL_SH3).
    Sh3 = 0x50,
    /// 0x51 (CV_CFL_SH3E).
    Sh3E = 0x51,
    /// 0x52 (CV_CFL_SH3DSP).
    Sh3Dsp = 0x52,
    /// 0x53 (CV_CFL_SH4).
    Sh4 = 0x53,
    /// 0x54 (CV_CFL_SHMEDIA).
    ShMedia = 0x54,
    /// 0x60 (CV_CFL_ARM3).
    Arm3 = 0x60,
    /// 0x61 (CV_CFL_ARM4).
    Arm4 = 0x61,
    /// 0x62 (CV_CFL_ARM4T).
    Arm4T = 0x62,
    /// 0x63 (CV_CFL_ARM5).
    Arm5 = 0x63,
    /// 0x64 (CV_CFL_ARM5T).
    Arm5T = 0x64,
    /// 0x65 (CV_CFL_ARM6).
    Arm6 = 0x65,
    /// 0x66 (CV_CFL_ARM_XMAC).
    ArmXmac = 0x66,
    /// 0x67 (CV_CFL_ARM_WMMX).
    ArmWmmx = 0x67,
    /// 0x68 (CV_CFL_ARM7).
    Arm7 = 0x68,
    /// 0x70 (CV_CFL_OMNI).
    Omni = 0x70,
    /// 0x80 (CV_CFL_IA64 / CV_CFL_IA64_1).
    Ia64 = 0x80,
    /// 0x81 (CV_CFL_IA64_2).
    Ia642 = 0x81,
    /// 0x90 (CV_CFL_CEE).
    Cee = 0x90,
    /// 0xa0 (CV_CFL_AM33).
    Am33 = 0xa0,
    /// 0xb0 (CV_CFL_M32R).
    M32R = 0xb0,
    /// 0xc0 (CV_CFL_TRICORE).
    Tricore = 0xc0,
    /// 0xd0 (CV_CFL_X64 / CV_CFL_AMD64).
    X64 = 0xd0,
    /// 0xe0 (CV_CFL_EBC).
    Ebc = 0xe0,
    /// 0xf0 (CV_CFL_THUMB).
    Thumb = 0xf0,
    /// 0xf4 (CV_CFL_ARMNT).
    ArmNt = 0xf4,
    /// 0xf6 (CV_CFL_ARM64).
    Arm64 = 0xf6,
    /// 0xf7 (CV_CFL_HYBRID_X86_ARM64).
    HybridX86Arm64 = 0xf7,
    /// 0xf8 (CV_CFL_ARM64EC).
    Arm64Ec = 0xf8,
    /// 0xf9 (CV_CFL_ARM64X).
    Arm64X = 0xf9,
    /// 0xff (CV_CFL_UNKNOWN).
    CflUnknown = 0xff,
    /// 0x100 (CV_CFL_D3D11_SHADER).
    D3d11Shader = 0x100,
    /// A machine type the runtime does not recognize.
    Unknown(i32),
}

impl CpuType {
    /// The raw ABI value.
    pub(crate) fn abi(self) -> i32 {
        match self {
            CpuType::C8080 => 0x0,
            CpuType::C8086 => 0x1,
            CpuType::C80286 => 0x2,
            CpuType::C80386 => 0x3,
            CpuType::C80486 => 0x4,
            CpuType::Pentium => 0x5,
            CpuType::PentiumII => 0x6,
            CpuType::PentiumIII => 0x7,
            CpuType::Mips => 0x10,
            CpuType::Mips16 => 0x11,
            CpuType::Mips32 => 0x12,
            CpuType::Mips64 => 0x13,
            CpuType::MipsI => 0x14,
            CpuType::MipsII => 0x15,
            CpuType::MipsIII => 0x16,
            CpuType::MipsIV => 0x17,
            CpuType::MipsV => 0x18,
            CpuType::M68000 => 0x20,
            CpuType::M68010 => 0x21,
            CpuType::M68020 => 0x22,
            CpuType::M68030 => 0x23,
            CpuType::M68040 => 0x24,
            CpuType::Alpha => 0x30,
            CpuType::Alpha21164 => 0x31,
            CpuType::Alpha21164A => 0x32,
            CpuType::Alpha21264 => 0x33,
            CpuType::Alpha21364 => 0x34,
            CpuType::Ppc601 => 0x40,
            CpuType::Ppc603 => 0x41,
            CpuType::Ppc604 => 0x42,
            CpuType::Ppc620 => 0x43,
            CpuType::PpcFp => 0x44,
            CpuType::PpcBe => 0x45,
            CpuType::Sh3 => 0x50,
            CpuType::Sh3E => 0x51,
            CpuType::Sh3Dsp => 0x52,
            CpuType::Sh4 => 0x53,
            CpuType::ShMedia => 0x54,
            CpuType::Arm3 => 0x60,
            CpuType::Arm4 => 0x61,
            CpuType::Arm4T => 0x62,
            CpuType::Arm5 => 0x63,
            CpuType::Arm5T => 0x64,
            CpuType::Arm6 => 0x65,
            CpuType::ArmXmac => 0x66,
            CpuType::ArmWmmx => 0x67,
            CpuType::Arm7 => 0x68,
            CpuType::Omni => 0x70,
            CpuType::Ia64 => 0x80,
            CpuType::Ia642 => 0x81,
            CpuType::Cee => 0x90,
            CpuType::Am33 => 0xa0,
            CpuType::M32R => 0xb0,
            CpuType::Tricore => 0xc0,
            CpuType::X64 => 0xd0,
            CpuType::Ebc => 0xe0,
            CpuType::Thumb => 0xf0,
            CpuType::ArmNt => 0xf4,
            CpuType::Arm64 => 0xf6,
            CpuType::HybridX86Arm64 => 0xf7,
            CpuType::Arm64Ec => 0xf8,
            CpuType::Arm64X => 0xf9,
            CpuType::CflUnknown => 0xff,
            CpuType::D3d11Shader => 0x100,
            CpuType::Unknown(v) => v,
        }
    }

    /// Converts a raw `CV_CPU_TYPE_e` value to a [`CpuType`].
    #[allow(dead_code)]
    pub(crate) fn from_abi(value: i32) -> Self {
        match value {
            0x0 => CpuType::C8080,
            0x1 => CpuType::C8086,
            0x2 => CpuType::C80286,
            0x3 => CpuType::C80386,
            0x4 => CpuType::C80486,
            0x5 => CpuType::Pentium,
            0x6 => CpuType::PentiumII,
            0x7 => CpuType::PentiumIII,
            0x10 => CpuType::Mips,
            0x11 => CpuType::Mips16,
            0x12 => CpuType::Mips32,
            0x13 => CpuType::Mips64,
            0x14 => CpuType::MipsI,
            0x15 => CpuType::MipsII,
            0x16 => CpuType::MipsIII,
            0x17 => CpuType::MipsIV,
            0x18 => CpuType::MipsV,
            0x20 => CpuType::M68000,
            0x21 => CpuType::M68010,
            0x22 => CpuType::M68020,
            0x23 => CpuType::M68030,
            0x24 => CpuType::M68040,
            0x30 => CpuType::Alpha,
            0x31 => CpuType::Alpha21164,
            0x32 => CpuType::Alpha21164A,
            0x33 => CpuType::Alpha21264,
            0x34 => CpuType::Alpha21364,
            0x40 => CpuType::Ppc601,
            0x41 => CpuType::Ppc603,
            0x42 => CpuType::Ppc604,
            0x43 => CpuType::Ppc620,
            0x44 => CpuType::PpcFp,
            0x45 => CpuType::PpcBe,
            0x50 => CpuType::Sh3,
            0x51 => CpuType::Sh3E,
            0x52 => CpuType::Sh3Dsp,
            0x53 => CpuType::Sh4,
            0x54 => CpuType::ShMedia,
            0x60 => CpuType::Arm3,
            0x61 => CpuType::Arm4,
            0x62 => CpuType::Arm4T,
            0x63 => CpuType::Arm5,
            0x64 => CpuType::Arm5T,
            0x65 => CpuType::Arm6,
            0x66 => CpuType::ArmXmac,
            0x67 => CpuType::ArmWmmx,
            0x68 => CpuType::Arm7,
            0x70 => CpuType::Omni,
            0x80 => CpuType::Ia64,
            0x81 => CpuType::Ia642,
            0x90 => CpuType::Cee,
            0xa0 => CpuType::Am33,
            0xb0 => CpuType::M32R,
            0xc0 => CpuType::Tricore,
            0xd0 => CpuType::X64,
            0xe0 => CpuType::Ebc,
            0xf0 => CpuType::Thumb,
            0xf4 => CpuType::ArmNt,
            0xf6 => CpuType::Arm64,
            0xf7 => CpuType::HybridX86Arm64,
            0xf8 => CpuType::Arm64Ec,
            0xf9 => CpuType::Arm64X,
            0xff => CpuType::CflUnknown,
            0x100 => CpuType::D3d11Shader,
            v => CpuType::Unknown(v),
        }
    }
}
