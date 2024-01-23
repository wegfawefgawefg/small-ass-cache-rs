use std::path::PathBuf;

#[macro_export]
macro_rules! asset_enum {
    (base_path = $base_path:expr, load = $load_fn:ident, $name:ident { $($variant:ident = $path:expr),* $(,)? }) => {
        // Generate the enum
        enum $name {
            $($variant),*
        }

        // Generate the implementation of AssetEnum for the enum
        impl AssetEnum for $name {
            fn path(&self) -> PathBuf {
                let base_path = PathBuf::from($base_path);
                match self {
                    $(
                        $name::$variant => base_path.join($path),
                    )*
                }
            }

            fn load(&self) -> Image {
                let path = self.path();
                $load_fn(path.to_str().unwrap())
            }
        }
    };
}

// fake image struct
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

asset_enum! {
    base_path = "assets/images/",
    load = dummy_load_image,
    Images {
        Chips = "chips.png",
        Food = "food.png",
        Gear = "gear.png",
    }
}

// the above should spit out an enum, and a trait implementation for Images, which looks like
// this:
enum Images {
    Chips,
    Food,
    Gear,
}

impl AssetEnum for Images {
    fn path(&self) -> PathBuf {
        let base_path = PathBuf::from("assets/images/");
        match self {
            Images::Chips => base_path.join("chips.png"),
            Images::Food => base_path.join("food.png"),
            Images::Gear => base_path.join("gear.png"),
        }
    }

    fn load(&self) -> Image {
        let path = self.path();
        dummy_load_image(path.to_str().unwrap())
    }
}

fn main() {
    let path = Images::Chips.path(); // Gets the full path
    let asset = Images::Chips.load(); // Loads the asset
}

fn main() {
    let path = Audio::Go.path(); // Gets the full path
    let asset = Audio::Go.load(); // Loads the asset
}
