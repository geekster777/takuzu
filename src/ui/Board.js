import {useState, useEffect, useMemo} from 'preact';
import Tile from 'tile';
import {isValid, TILE_STATE} from 'validation';
import {calcVibrantColors} from 'colorUtils';

const BOARD = [
    [0, null, null, null],
    [null, 1, null, null],
    [null, null, null, null],
    [null, null, null, null]
];

const styles = {
  content: {
    backgroundImage: 'url(assets/castle.jpg)',
    backgroundSize: 'cover',
    backgroundPosition: 'center center',
    flow: 'vertical',
    width: '*',
    height: '*',
  },
  board: {
    flow: 'vertical',
    height: '*',
    width: '*',
    padding: '8dip',
    borderSpacing: '16dip',
  },
  row: {
    height: '1*',
    flow: 'horizontal',
    borderSpacing: '16dip',
  },
  button: {
    margin: '8dip *',
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

function rgbToHex(r, g, b) {
  return "#" + ((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1);
}

export default function Board() {
  const [board, setBoard] = useState(BOARD);
  const [showInvalidTiles, setShowInvalidTiles] = useState(false);
  const [palette, setPalette] = useState({
    primary: 'rgba(255, 255, 255, 0.5)',
    secondary: 'rgba(0, 0, 0, 0.5)',
  });

  useEffect(() => {
    const colors = Window.this.xcall('color_palette', 'assets/castle.jpg');
    const vibrantPalette = calcVibrantColors(colors);
    setPalette(vibrantPalette);
    // TODO: Run whenever the board image changes
  }, []);

  useEffect(() => {
    setShowInvalidTiles(false);
  }, [board]);


  const {invalidTiles} = useMemo(() => {
    if (!showInvalidTiles) {
      return {invalidTiles: []};
    }

    return isValid(board);
  }, [board, showInvalidTiles]);

  return <div style={styles.content}>
    <div>
      <button style={styles.button} onClick={() => {
        setShowInvalidTiles(true);
      }}>Check Solution</button>
    </div>
    <div style={styles.board}>
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
              palette={palette}
              />;
          })
        }</div>;
      })}
    </div>
  </div>;
}