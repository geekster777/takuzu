import { useState } from 'preact';
import Tile from 'tile';

const BOARD = [
    [null, null, null, null],
    [null, null, null, null],
    [null, null, null, null],
    [null, null, null, null]
];

const styles = {
  board: {
    backgroundImage: 'url(assets/castle.jpg)',
    backgroundSize: 'cover',
    backgroundPosition: 'center center',
    flow: 'vertical',
    width: '*',
    height: '*',
    padding: '8dip',
    borderSpacing: '16dip',
  },
  row: {
    height: '1*',
    flow: 'horizontal',
    borderSpacing: '16dip',
  }
}

export default function Board() {
  const [board, setBoard] = useState(BOARD);

  return <div style={styles.board}>
    {board.map((row, i) => {
      return <div style={styles.row}> {
        row.map((tileState, j) => {
          const toggleTile = () => {
            const newState = tileState === 'PRIMARY' ? 'SECONDARY' : tileState === 'SECONDARY' ? null : 'PRIMARY';
            const newBoard = board.map(row => row.slice());
            newBoard[i][j] = newState;
            setBoard(newBoard);
          };

          return <Tile state = {tileState}
          onClick = {toggleTile}
          locked = {BOARD[i][j] != null}
          />;
        })
      }</div>;
    })}
  </div>;
}