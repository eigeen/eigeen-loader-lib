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

impl From<AddressName> for &'static str {
    fn from(val: AddressName) -> Self {
        val.0
    }
}

impl std::fmt::Display for AddressName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AddressName {
    pub const CHAT_MESSAGE_SENT: AddressName = AddressName("Chat:MessageSent");
    pub const CHAT_SYSTEM_MESSAGE: AddressName = AddressName("Chat:SystemMessage");
    pub const CORE_AFTER_MH_MAIN_CTOR: AddressName = AddressName("Core:AfterMhMainCtor");
    pub const CORE_GAME_REVISION: AddressName = AddressName("Core:GameRevision");
    pub const CORE_MH_MAIN_CTOR: AddressName = AddressName("Core:MhMainCtor");
    pub const QUEST_ABANDON: AddressName = AddressName("Quest:Abandon");
    pub const RESOURCE_MANAGER_CLOSE_FILE: AddressName = AddressName("ResourceManager:CloseFile");
    pub const RESOURCE_MANAGER_OPEN_FILE: AddressName = AddressName("ResourceManager:OpenFile");
    pub const C_SYSTEM_CTOR: AddressName = AddressName("cSystem:Ctor");
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

impl From<SingletonName> for &'static str {
    fn from(val: SingletonName) -> Self {
        val.0
    }
}

impl std::fmt::Display for SingletonName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SingletonName {
    pub const QUEST: SingletonName = SingletonName("sQuest");
    pub const PLAYER: SingletonName = SingletonName("sPlayer");
    pub const CHAT: SingletonName = SingletonName("sChat");
    pub const WWISE_BGM_MANAGER: SingletonName = SingletonName("sWwiseBgmManager");
}
