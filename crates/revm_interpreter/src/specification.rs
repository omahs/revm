use revm_precompiles::SpecId as PrecompileId;

pub const SPEC_ID_FRONTIER: u8 = 0;
pub const SPEC_ID_FRONTIER_THAWING: u8 = 1;
pub const SPEC_ID_HOMESTEAD: u8 = 2;
pub const SPEC_ID_DAO_FORK: u8 = 3;
pub const SPEC_ID_TANGERINE: u8 = 4;
pub const SPEC_ID_SPURIOUS_DRAGON: u8 = 5;
pub const SPEC_ID_BYZANTIUM: u8 = 6;
pub const SPEC_ID_CONSTANTINOPLE: u8 = 7;
pub const SPEC_ID_PETERSBURG: u8 = 8;
pub const SPEC_ID_ISTANBUL: u8 = 9;
pub const SPEC_ID_MUIR_GLACIER: u8 = 10;
pub const SPEC_ID_BERLIN: u8 = 11;
pub const SPEC_ID_LONDON: u8 = 12;
pub const SPEC_ID_ARROW_GLACIER: u8 = 13;
pub const SPEC_ID_GRAY_GLACIER: u8 = 14;
pub const SPEC_ID_MERGE: u8 = 15;
pub const SPEC_ID_LATEST: u8 = 16;

/// SpecId and their activation block
/// Information was obtained from: https://github.com/ethereum/execution-specs
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(non_camel_case_types)]
pub enum SpecId {
    FRONTIER = SPEC_ID_FRONTIER,                 // Frontier	           0
    FRONTIER_THAWING = SPEC_ID_FRONTIER_THAWING, // Frontier Thawing       200000
    HOMESTEAD = SPEC_ID_HOMESTEAD,               // Homestead	           1150000
    DAO_FORK = SPEC_ID_DAO_FORK,                 // DAO Fork	           1920000
    TANGERINE = SPEC_ID_TANGERINE,               // Tangerine Whistle	   2463000
    SPURIOUS_DRAGON = SPEC_ID_SPURIOUS_DRAGON,   // Spurious Dragon        2675000
    BYZANTIUM = SPEC_ID_BYZANTIUM,               // Byzantium	           4370000
    CONSTANTINOPLE = SPEC_ID_CONSTANTINOPLE, // Constantinople          7280000 is overwriten with PETERSBURG
    PETERSBURG = SPEC_ID_PETERSBURG,         // Petersburg              7280000
    ISTANBUL = SPEC_ID_ISTANBUL,             // Istanbul	            9069000
    MUIR_GLACIER = SPEC_ID_MUIR_GLACIER,     // Muir Glacier	        9200000
    BERLIN = SPEC_ID_BERLIN,                 // Berlin	                12244000
    LONDON = SPEC_ID_LONDON,                 // London	                12965000
    ARROW_GLACIER = SPEC_ID_ARROW_GLACIER,   // Arrow Glacier	        13773000
    GRAY_GLACIER = SPEC_ID_GRAY_GLACIER,     // Gray Glacier	        15050000
    MERGE = SPEC_ID_MERGE,                   // Paris/Merge	            TBD (Depends on difficulty)
    LATEST = SPEC_ID_LATEST,
}

impl SpecId {
    pub const fn to_precompile_id(self) -> PrecompileId {
        match self {
            FRONTIER | FRONTIER_THAWING | HOMESTEAD | DAO_FORK | TANGERINE | SPURIOUS_DRAGON => {
                PrecompileId::HOMESTEAD
            }
            BYZANTIUM | CONSTANTINOPLE | PETERSBURG => PrecompileId::BYZANTIUM,
            ISTANBUL | MUIR_GLACIER => PrecompileId::ISTANBUL,
            BERLIN | LONDON | ARROW_GLACIER | GRAY_GLACIER | MERGE | LATEST => PrecompileId::BERLIN,
        }
    }

    pub const fn id(self) -> u8 {
        self as u8
    }

    pub fn try_from_u8(spec_id: u8) -> Option<Self> {
        if spec_id > Self::LATEST as u8 {
            None
        } else {
            Some(unsafe { core::mem::transmute(spec_id) })
        }
    }

    #[inline]
    pub const fn enabled_in(self, spec_id: u8) -> bool {
        spec_id >= self as u8
    }
}

pub use SpecId::*;

impl From<&str> for SpecId {
    fn from(name: &str) -> Self {
        match name {
            "Frontier" => SpecId::FRONTIER,
            "Homestead" => SpecId::HOMESTEAD,
            "Tangerine" => SpecId::TANGERINE,
            "Spurious" => SpecId::SPURIOUS_DRAGON,
            "Byzantium" => SpecId::BYZANTIUM,
            "Constantinople" => SpecId::CONSTANTINOPLE,
            "Petersburg" => SpecId::PETERSBURG,
            "Istanbul" => SpecId::ISTANBUL,
            "MuirGlacier" => SpecId::MUIR_GLACIER,
            "Berlin" => SpecId::BERLIN,
            "London" => SpecId::LONDON,
            "Merge" => SpecId::MERGE,
            _ => SpecId::LATEST,
        }
    }
}

pub(crate) trait NotStaticSpec {}

pub trait Spec: Sized {
    /// little bit of magic. We can have child version of Spec that contains static flag enabled
    type STATIC: Spec;

    #[inline(always)]
    fn enabled(spec_id: SpecId) -> bool {
        Self::SPEC_ID as u8 >= spec_id as u8
    }
    const SPEC_ID: SpecId;

    const SPEC_ID_U8: u8;
    /// static flag used in STATIC type;
    const IS_STATIC_CALL: bool;

    const ASSUME_PRECOMPILE_HAS_BALANCE: bool;
}

pub(crate) mod spec_impl {
    use super::{NotStaticSpec, Spec};

    macro_rules! spec {
        ($spec_id:tt) => {
            #[allow(non_snake_case)]
            pub mod $spec_id {
                use super::{NotStaticSpec, Spec};
                use crate::SpecId;

                pub struct SpecInner<
                    const STATIC_CALL: bool,
                    const ASSUME_PRECOMPILE_HAS_BALANCE: bool,
                >;

                pub type SpecImpl = SpecInner<false, true>;
                pub type SpecStaticImpl = SpecInner<true, true>;

                impl NotStaticSpec for SpecImpl {}

                impl<const IS_STATIC_CALL: bool, const ASSUME_PRECOMPILE_HAS_BALANCE: bool> Spec
                    for SpecInner<IS_STATIC_CALL, ASSUME_PRECOMPILE_HAS_BALANCE>
                {
                    type STATIC = SpecInner<true, ASSUME_PRECOMPILE_HAS_BALANCE>;

                    //specification id
                    const SPEC_ID: SpecId = SpecId::$spec_id;

                    const SPEC_ID_U8: u8 = SpecId::$spec_id as u8;

                    const IS_STATIC_CALL: bool = IS_STATIC_CALL;

                    const ASSUME_PRECOMPILE_HAS_BALANCE: bool = ASSUME_PRECOMPILE_HAS_BALANCE;
                }
            }
        };
    }

    spec!(FRONTIER);
    // FRONTIER_THAWING no EVM spec change
    spec!(HOMESTEAD);
    // DAO_FORK no EVM spec change
    spec!(TANGERINE);
    spec!(SPURIOUS_DRAGON);
    spec!(BYZANTIUM);
    // CONSTANTINOPLE was overriden with PETERSBURG
    spec!(PETERSBURG);
    spec!(ISTANBUL);
    // MUIR_GLACIER no EVM spec change
    spec!(BERLIN);
    spec!(LONDON);
    // ARROW_GLACIER no EVM spec change
    // GRAT_GLACIER no EVM spec change
    spec!(MERGE);
    spec!(LATEST);
}

pub use spec_impl::BERLIN::SpecImpl as BerlinSpec;
pub use spec_impl::BYZANTIUM::SpecImpl as ByzantiumSpec;
pub use spec_impl::FRONTIER::SpecImpl as FrontierSpec;
pub use spec_impl::HOMESTEAD::SpecImpl as HomesteadSpec;
pub use spec_impl::ISTANBUL::SpecImpl as IstanbulSpec;
pub use spec_impl::LATEST::SpecImpl as LatestSpec;
pub use spec_impl::LONDON::SpecImpl as LondonSpec;
pub use spec_impl::MERGE::SpecImpl as MergeSpec;
pub use spec_impl::PETERSBURG::SpecImpl as PetersburgSpec;
pub use spec_impl::SPURIOUS_DRAGON::SpecImpl as SpuriousDragonSpec;
pub use spec_impl::TANGERINE::SpecImpl as TangerineSpec;
