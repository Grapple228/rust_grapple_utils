pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

#[cfg(not(feature = "cuuid"))]
fn main() {
    panic!("This example requires the 'cuuid' feature to be enabled.");
}

#[cfg(all(
    feature = "cuuid",
    not(any(feature = "b32", feature = "b58", feature = "b64"))
))]
fn main() {
    panic!("This example requires one of the `b32`, `b58`, `b64` features to be enabled.");
}

#[cfg(all(
    feature = "cuuid",
    any(feature = "b32", feature = "b58", feature = "b64")
))]
fn display_id(cuuid: grapple_utils::cuuid::CUuid, id: uuid::Uuid) {
    let compressed = cuuid.from(id);

    println!(
        "{:<30} ({})\t->\t{:?} {:<30} ({})",
        id,
        id.to_string().len(),
        cuuid,
        compressed,
        compressed.len()
    );
}

#[cfg(all(
    feature = "cuuid",
    any(feature = "b32", feature = "b58", feature = "b64")
))]
fn main() -> Result<()> {
    use grapple_utils::cuuid::CUuid;

    println!("Uuid v4");
    for _ in 0..3 {
        let id = uuid::Uuid::new_v4();

        #[cfg(feature = "b32")]
        display_id(CUuid::B32, id);
        #[cfg(feature = "b58")]
        display_id(CUuid::B58, id);
        #[cfg(feature = "b64")]
        display_id(CUuid::B64, id);
    }

    println!();

    println!("Uuid v7");
    for _ in 0..3 {
        let id = uuid::Uuid::now_v7();

        #[cfg(feature = "b32")]
        display_id(CUuid::B32, id);
        #[cfg(feature = "b58")]
        display_id(CUuid::B58, id);
        #[cfg(feature = "b64")]
        display_id(CUuid::B64, id);

        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    Ok(())
}
