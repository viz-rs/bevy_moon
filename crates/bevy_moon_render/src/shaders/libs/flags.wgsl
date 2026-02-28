#define_import_path bevy_moon::flags

const NONE     = 0u;
const TEXTURED = 1u;
const GLYPH    = 3u;

fn enabled(flags: u32, mask: u32) -> bool {
    return (flags & mask) != NONE;
}
