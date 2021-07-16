use detour::GenericDetour;

type Add5 = extern "Rust" fn(i32) -> i32;

extern "Rust" fn add_5(n: i32) -> i32 {
	n + 5
}

extern "Rust" fn hookfn(n: i32) -> i32 {
	n + 50
}

fn main() -> Result<(), detour::Error> {
	let hook = unsafe {
		GenericDetour::<Add5>::new(add_5, hookfn)?
	};
	let _res = hook.call(50);

	Ok(())
}
