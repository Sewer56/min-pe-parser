#[macro_use]
mod macros {
    #[repr(C)] // guarantee 'bytes' comes after '_align'
    pub struct AlignedAs<Align, Bytes: ?Sized> {
        pub _align: [Align; 0],
        pub bytes: Bytes,
    }

    macro_rules! include_bytes_align_as {
        ($align_ty:ty, $path:literal) => {{
            // const block expression to encapsulate the static
            use $crate::utils::test_utils::macros::AlignedAs;

            // this assignment is made possible by CoerceUnsized
            static ALIGNED: &AlignedAs<$align_ty, [u8]> = &AlignedAs {
                _align: [],
                bytes: *include_bytes!($path),
            };

            &ALIGNED.bytes
        }};
    }
}

#[repr(align(4))]
struct Align4;

pub const RELOADED_BOOTSTRAPPER_DLL_X64: &[u8] = include_bytes_align_as!(
    Align4,
    "../../assets/test_data/x64/Reloaded.Mod.Loader.Bootstrapper.dll"
);
pub const RELOADED_BOOTSTRAPPER_DLL_X86: &[u8] = include_bytes_align_as!(
    Align4,
    "../../assets/test_data/x86/Reloaded.Mod.Loader.Bootstrapper.dll"
);
