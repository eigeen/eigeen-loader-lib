#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum Code {
    Ok = 0,
    InvalidUtf8String = 1,
    NotFound = 2,
}

/// Managed address names.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AddressName(pub &'static str);

impl AsRef<str> for AddressName {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl std::ops::Deref for AddressName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl From<&'static str> for AddressName {
    fn from(s: &'static str) -> Self {
        AddressName(s)
    }
}

impl AddressName {
    pub const MID_AFTER_MH_MAIN_CTOR: AddressName = AddressName("Mid:AfterMhMainCtor");
    pub const C_SYSTEM_CTOR: AddressName = AddressName("cSystem:Ctor");
    pub const CORE_MH_MAIN_CTOR: AddressName = AddressName("Core:MhMainCtor");
    pub const QUEST_ABANDON: AddressName = AddressName("Quest:Abandon");
}

/// Managed singleton names.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SingletonName(pub &'static str);

impl AsRef<str> for SingletonName {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl std::ops::Deref for SingletonName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl From<&'static str> for SingletonName {
    fn from(s: &'static str) -> Self {
        SingletonName(s)
    }
}

impl SingletonName {
    pub const QUEST: SingletonName = SingletonName("sQuest");
    pub const PLAYER: SingletonName = SingletonName("sPlayer");
}
