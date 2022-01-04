import {useMemo} from 'preact';
import { TILE_STATE } from './validation';

const styles = {
  tileOutline: {
    height: '*',
    width: '*',
    transition: 'outline linear 150ms',
    outline: '0 rgba(255, 0, 0, 1) solid',
    border: '1dip rgba(30, 30, 30, 0.5) solid',
    backgroundColor: 'rgba(255, 255, 255, 0.25)',
  },
  invalid: {
    outlineWidth: '2dip',
  },
  tile: {
    height: '*',
    width: '*',
    backdropFilter: 'blur(8dip)',
    opacity: '0',

  },
  primary: {
    boxShadow: '0 0 4dip rgba(0, 0, 0, 0.5)',
    backgroundColor: 'rgba(255, 0, 0, 0.3)',
    opacity: '1',
  },
  secondary: {
    boxShadow: '0 0 4dip rgba(0, 0, 0, 0.5)',
    backgroundColor: 'rgba(0, 0, 255, 0.3)',
    opacity: '1',
  },
};

function tileStyles(state) {

  switch(state) {
    case TILE_STATE.PRIMARY:
      return {...styles.tile, ...styles.primary};
    case TILE_STATE.SECONDARY:
      return {...styles.tile, ...styles.secondary};
    default:
      return styles.tile;
  }
}

export default function Tile({state, locked, invalid, onClick}) {
  const tileStyle = useMemo(() => {
    return tileStyles(state);
  }, [state]);

  const outlineStyle = useMemo(() => {
    if (invalid && !locked) {
      return {
        ...styles.tileOutline,
        ...styles.invalid,
      };
     }
     
     return styles.tileOutline;
  }, [invalid, locked]);

  return (
    <div style={outlineStyle}>
      <div
        style={tileStyle}
        class="transitionBg"
        onClick={() => !locked && onClick()}>
      </div>
    </div>
  );
}