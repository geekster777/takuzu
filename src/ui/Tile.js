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
  selectedTile: {
    boxShadow: '0 0 4dip rgba(0, 0, 0, 0.5)',
    opacity: '1',
  },
};

function tileStyles(state, palette) {

  switch(state) {
    case TILE_STATE.PRIMARY:
      return {
        ...styles.tile,
        ...styles.selectedTile,
        backgroundColor: palette?.primary ?? 'rgba(0, 0, 0, 0.3)',
      };
    case TILE_STATE.SECONDARY:
      return {
        ...styles.tile,
        ...styles.selectedTile,
        backgroundColor: palette?.secondary ?? 'rgba(0, 0, 0, 0.3)',
      };
    default:
      return styles.tile;
  }
}

export default function Tile({state, locked, invalid, onClick, palette}) {
  const tileStyle = useMemo(() => {
    return tileStyles(state, palette);
  }, [state, palette]);

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