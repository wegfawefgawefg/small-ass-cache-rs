use std::path::{Path, PathBuf};

#[macro_export]
macro_rules! gen_asset_struct {
    // Variant with base_path specified
    (load = $load_fn:ident, base_path = $base_path:expr, $struct_name:ident<$type:ty> { $($variant:ident = $path:expr),* $(,)? }) => {
        gen_asset_struct!(@internal $base_path, $load_fn, $struct_name<$type> { $($variant = $path),* });
    };

    // Variant without base_path specified (uses default "./")
    (load = $load_fn:ident, $struct_name:ident<$type:ty> { $($variant:ident = $path:expr),* $(,)? }) => {
        gen_asset_struct!(@internal "", $load_fn, $struct_name<$type> { $($variant = $path),* });
    };

    // Internal implementation
    (@internal $base_path:expr, $load_fn:ident, $struct_name:ident<$type:ty> { $($variant:ident = $path:expr),* $(,)? }) => {
        struct $struct_name {
            $(pub $variant: $type),*
        }

        impl $struct_name {
            pub fn new() -> Self {
                $struct_name {
                    $(
                        $variant: match $load_fn(PathBuf::from($base_path).join($path).as_path()) {
                            Ok(asset) => asset,
                            Err(e) => panic!("Failed to load asset '{}' at path '{}': {:?}", stringify!($variant), $path, e),
                        },
                    )*
                }
            }
        }
    };
}

//////////////// IMAGE //////////////////////
#[derive(Debug, Clone)]
pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Audio {
    channels: u32,
    sample_rate: u32,
    samples: Vec<u8>,
}

fn dummy_load_image(path: &Path) -> Result<Image, String> {
    Ok(Image {
        width: 0,
        height: 0,
        pixels: vec![],
    })
}

fn dummy_load_audio(path: &Path) -> Result<Audio, String> {
    Ok(Audio {
        channels: 0,
        sample_rate: 0,
        samples: vec![],
    })
}

gen_asset_struct! {
    load = dummy_load_image,
    base_path = "assets/images/",
    Images<Image> {
        chips = "chips.png",
        food = "food.png",
        gear = "gear.png",
    }
}

gen_asset_struct! {
    load = dummy_load_audio,
    base_path = "assets/audio/",
    Sounds<Audio> {
        go = "go.wav",
        away = "away.wav",
    }
}

fn main() {
    let images = Images::new();
    let sounds = Sounds::new();
    println!("{:?}", images.chips.width);
    println!("{:?}", sounds.go.channels);
}
