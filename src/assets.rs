use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
pub struct TileAssets {
	#[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 3, rows = 2))]
	#[asset(path = "tiles/terrain.png")]
	pub terrain_tiles: Handle<TextureAtlas>,
}

#[derive(AssetCollection)]
pub struct CharacterAssets {
	#[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 9, rows = 7))]
	#[asset(path = "characters/adventurer.png")]
	pub adventurer: Handle<TextureAtlas>,
}
