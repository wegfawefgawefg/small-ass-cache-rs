use std::any::Any;
use std::path::PathBuf;

trait AssetEnum {
    type AssetType: Clone + 'static;

    fn path(&self) -> PathBuf;
    fn load(&self) -> Self::AssetType;
}

#[macro_export]
macro_rules! asset_enum {
    (base_path = $base_path:expr, load = $load_fn:ident, $name:ident { $($variant:ident = $path:expr),* $(,)? }) => {
        // Generate the enum
        enum $name {
            $($variant),*
        }

        // Generate the implementation of AssetEnum for the enum
        impl AssetEnum for $name {
            type AssetType = Image; // Adjust this type as needed

            fn path(&self) -> PathBuf {
                let base_path = PathBuf::from($base_path);
                match self {
                    $(
                        $name::$variant => base_path.join($path),
                    )*
                }
            }

            fn load(&self) -> Self::AssetType {
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

use std::collections::HashMap;
use std::hash::Hash;

struct Cache {
    assets: HashMap<PathBuf, Box<dyn Any>>,
}

impl Cache {
    fn new() -> Cache {
        Cache {
            assets: HashMap::new(),
        }
    }

    fn get<T: AssetEnum + 'static>(&mut self, asset: T) -> Option<T> {
        let path = asset.path();
        match self.assets.get(&path) {
            Some(boxed_asset) => {
                if let Some(asset) = boxed_asset.downcast_ref::<T>() {
                    Some(asset.clone())
                } else {
                    None
                }
            }
            None => {
                let loaded_asset = asset.load();
                self.assets.insert(path, Box::new(loaded_asset.clone()));
                Some(loaded_asset)
            }
        }
    }
}

fn main() {
    let path = Images::Chips.path(); // Gets the full path
    println!("{:?}", path);
    let image = Images::Chips.load(); // Loads the asset
    println!("{:?}", image.width);
}
