export const TILE_STATE = {
  PRIMARY: 1,
  SECONDARY: 0,
  UNSELECTED: null,
}

export function isValid(board) {
  const rows = board.map((row, i) => row.map((tileState, j) => ({id: `${i},${j}`, state: tileState})));
  const columns = rows.map((_, i) => rows.map(row => row[i]));
  const groups = [...rows, ...columns];

  const unselectedTiles = rows.flat().filter(tile => tile.state == TILE_STATE.UNSELECTED);
  if (unselectedTiles.length !== 0) {
    return {failure: 'unselected', invalidTiles: unselectedTiles.map(tile => tile.id)};
  }

  for (let group of groups) {
    let primaries = group.filter(tile => tile.state === TILE_STATE.PRIMARY);
    let secondaries = group.filter(tile => tile.state === TILE_STATE.SECONDARY);

    if (primaries.length !== secondaries.length) {
      // inequal number of tile colors
      return {failure: 'inequal', invalidTiles: group.map(tile => tile.id)};
    }

    for (let i = 0; i < group.length - 2; i++) {
      if (group[i] === group[i + 1] && group[i] === group[i + 2]) {
        // 3 consecutive tiles
        return {failure: 'consecutive', invalidTiles: [group[i].id, group[i+1].id, group[i+2].id]};
      }
    }
  }

  return {success: true, invalidTiles: []};
}