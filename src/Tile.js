import {useMemo} from 'preact';

const styles = {
  tile: {
    height: '1*',
    width: '1*',
    backgroundColor: 'rgba(100, 100, 100, 0.6)',
    backdropFilter: 'blur(0dip)',
    border: '1dip rgba(30, 30, 30, 0.6) solid',
  },
  primary: {
    backgroundColor: 'rgba(255, 0, 0, 0.3)',
    backdropFilter: 'blur(8dip)',
  },
  secondary: {
    backgroundColor: 'rgba(0, 0, 255, 0.3)',
    backdropFilter: 'blur(8dip)',
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
  const style = useMemo(() => {
    return tileStyles(state);
  }, [state]);
  return (
    <div
      style={style}
      onClick={() => !locked && onClick()}>
    </div>
  );
}