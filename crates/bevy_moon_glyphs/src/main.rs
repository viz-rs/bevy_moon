use bevy_image::{self, TextureAtlas, TextureAtlasBuilder, TextureAtlasLayout};
use bevy_text;
use parley::{
    Glyph,
    swash::{FontRef, scale::Render},
};
use prost::{Message, bytes::Buf};
use versatiles_glyphs::protobuf::PbfGlyph;

fn main() {
    let glyphs =
        versatiles_glyphs::protobuf::PbfGlyphs::decode(&include_bytes!("../0-255.pbf")[..])
            .unwrap();

    let mut builder = TextureAtlasBuilder::default();
    let mut texture = TextureAtlas::default();

    for glyph in glyphs.into_glyphs().iter().take(2) {
        dbg!(&glyph.width);
        dbg!(&glyph.height);
        dbg!(&glyph.left);
        dbg!(&glyph.top);
        dbg!(&glyph.advance);
        dbg!(&glyph.bitmap().len());
        // let r = Render;
        let g = Glyph {
            id: glyph.id,
            x: glyph.left as f32,
            y: glyph.top as f32,
            advance: glyph.advance as f32,
            style_index: 0,
        };
        // Glyph
    }
}
