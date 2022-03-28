#![cfg_attr(not(feature = "std"), no_std)]

sp_api::decl_runtime_apis! {
	pub trait GetKittyCount
	{
        fn get_kitty_count() -> u32;
	}
}