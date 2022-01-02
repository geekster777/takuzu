import {useMemo} from 'preact';

const styles = {
  tileOutline: {
    height: '1*',
    width: '1*',
    border: '1dip rgba(30, 30, 30, 0.5) solid',
    backgroundColor: 'rgba(255, 255, 255, 0.25)',
  },
  tile: {
    height: '1*',
    width: '1*',
    backdropFilter: 'blur(8dip)',
    opacity: '0',

  },
  primary: {
    backgroundColor: 'rgba(255, 0, 0, 0.3)',
    opacity: '1',
  },
  secondary: {
    backgroundColor: 'rgba(0, 0, 255, 0.3)',
    opacity: '1',
  },
};

function tileStyles(state) {

  switch(state) {
    case 'PRIMARY':
      return {...styles.tile, ...styles.primary};
    case 'SECONDARY':
      return {...styles.tile, ...styles.secondary};
    default:
      return styles.tile;
  }
}

export default function Tile({state, locked, onClick}) {
  const tileStyle = useMemo(() => {
    return tileStyles(state);
  }, [state]);
  return (
    <div style={styles.tileOutline}>
      <div
        style={tileStyle}
        class="transitionBg"
        onClick={() => !locked && onClick()}>
      </div>
    </div>
  );
}