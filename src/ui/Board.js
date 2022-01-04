import {useState, useMemo} from 'preact';
import Tile from 'tile';
import {isValid} from 'validation';
import { TILE_STATE } from './validation';

const BOARD = [
    [0, null, null, null],
    [null, 1, null, null],
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

function nextTileState(state) {
  if (state === TILE_STATE.PRIMARY) {
    return TILE_STATE.SECONDARY;
  }
  
  if (state === TILE_STATE.SECONDARY) {
    return null;
  }
  
  return TILE_STATE.PRIMARY;
}

export default function Board() {
  const [board, setBoard] = useState(BOARD);

  const {invalidTiles} = useMemo(() => {
    return isValid(board);
  }, [board]);

  return <div style={styles.board}>
    {board.map((row, i) => {
      return <div style={styles.row}> {
        row.map((tileState, j) => {
          const toggleTile = () => {
            const newState = nextTileState(tileState);
            const newBoard = board.map(row => row.slice());
            newBoard[i][j] = newState;
            setBoard(newBoard);
          };

          return <Tile state={tileState}
            onClick={toggleTile}
            locked={BOARD[i][j] !== TILE_STATE.UNSELECTED}
            invalid={invalidTiles.indexOf(`${i},${j}`) !== -1}
            />;
        })
      }</div>;
    })}
  </div>;
}