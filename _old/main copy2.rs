use std::path::PathBuf;

trait AssetEnum<T> {
    fn path(&self) -> PathBuf;
    fn load(&self) -> T;
}

#[macro_export]
macro_rules! asset_enum {
    (base_path = $base_path:expr, load = $load_fn:ident, $name:ident<$type:ty> { $($variant:ident = $path:expr),* $(,)? }) => {
        // Generate the enum
        enum $name {
            $($variant),*
        }

        // Generate the implementation of AssetEnum for the enum
        impl AssetEnum<$type> for $name {
            fn path(&self) -> PathBuf {
                let base_path = PathBuf::from($base_path);
                match self {
                    $(
                        $name::$variant => base_path.join($path),
                    )*
                }
            }

            fn load(&self) -> $type {
                let path = self.path();
                $load_fn(path.to_str().unwrap())
            }
        }
    };
}

//////////////// IMAGE //////////////////////
#[derive(Debug, Clone)]
struct Image {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

fn dummy_load_image(path: &str) -> Image {
    Image {
        width: 0,
        height: 0,
        pixels: vec![],
    }
}

#[derive(Debug, Clone)]
struct Audio {
    channels: u32,
    sample_rate: u32,
    samples: Vec<u8>,
}

fn dummy_load_audio(path: &str) -> Audio {
    Audio {
        channels: 0,
        sample_rate: 0,
        samples: vec![],
    }
}

asset_enum! {
    base_path = "assets/images/",
    load = dummy_load_image,
    Images<Image> {
        Chips = "chips.png",
        Food = "food.png",
        Gear = "gear.png",
    }
}

asset_enum! {
    base_path = "assets/audio/",
    load = dummy_load_audio,
    Sounds<Audio> {
        Go = "go.wav",
        Away = "away.wav",
    }
}

fn main() {}
