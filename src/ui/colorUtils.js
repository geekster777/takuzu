const BLACK = {r: 0, g: 0, b: 0};
const WHITE = {r: 255, g: 255, b: 255};

// Returns {h: 0-360, s: 0-1, v: 0-1}
function rgbToHsv({r,g,b}) {
  // From https://stackoverflow.com/a/54070620
  r /= 255;
  g /= 255;
  b /= 255;
  let v = Math.max(r, g, b);
  let c = v - Math.min(r, g, b);
  let h = c && ((v==r) ? (g - b) / c : ((v === g) ? 2 + (b - r) / c : 4 + (r - g) / c)); 
  return {h: 60 * (h < 0 ? h + 6 : h), s: v && c / v, v};
}

export function calcVibrantColors(colors) {
  let palette = [...colors, BLACK, WHITE];
  palette = palette.map(color => ({
    ...color,
    ...rgbToHsv(color),
    css: `rgba(${color.r}, ${color.g}, ${color.b}, 0.5)`,
  }));

  // Sort by vividness
  palette.sort((left, right) => {
    return (right.s * right.v) - (left.s * left.v);
  });

  const primary = palette[0];

  palette = palette.map(color => ({
    ...color,
    distance: (180 - Math.abs((primary.h + 180) % 360 - color.h)) / 180,
  })).filter(color => color.distance > 0.15);

  const secondary = palette[0];

  return {primary: primary.css, secondary: secondary.css};
}