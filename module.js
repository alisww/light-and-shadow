import init, { JsPalette } from './light_and_shadow.js';

async function build() {
    await init();
    return new JsPalette([[54, 57, 64], [255, 255, 255]], 3.4)
}

const palette = build();

export default await palette;