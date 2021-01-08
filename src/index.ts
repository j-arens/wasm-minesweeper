import { GameParams, parseQueryParams } from './query';
import { afterPaint, renderGame, updateSector } from './view';
import { GamePointer, loadWasm, WasmExports } from './wasm';

const happyFace = String.fromCodePoint(0x1F604);
const unhappyFace = String.fromCodePoint(0x1F635);

type Context = GameParams & WasmExports & {
  game: GamePointer | null;
};

(async function main() {
  let root = document.getElementById('root') as HTMLDivElement;
  let params = parseQueryParams(window.location.search);
  let context: Context;

  try {
    context = {
      game: null,
      ...params,
      ...await loadWasm({
        onExplode: (idx: number, isCatalyst: number) => updateSector(root, idx, {
          'data-catalyst': isCatalyst ? 'true' : 'false',
          'data-state': 'exploded',
        }),

        onFlag: (idx: number) => updateSector(root, idx, {
          'data-state': 'flagged',
        }),

        onHide: (idx: number) => updateSector(root, idx, {
          'data-state': 'hidden',
        }),

        onGameLost,

        onReveal: (idx: number, proximity: number) => updateSector(root, idx, {
          'data-proximity': `${proximity}`,
          'data-state': 'revealed'
        }),

        onQuestion: (idx: number) => updateSector(root, idx, {
          'data-state': 'questioned',
        }),

        onGameWon,
      }),
    };
  } catch (e) {
    console.error(e);
    alert('An error occurred while loading the game. Please make sure your browser is up to date.');
    return;
  }

  root.addEventListener('contextmenu', (e) => e.preventDefault());
  root.addEventListener('pointerup', (e) => onPointerUp(e, context));

  renderGame(root, params.rows, params.cols);
})();

function onPointerUp(e: PointerEvent, context: Context) {
  if (e.button === 1 || e.button > 2 || !(e.target instanceof Element)) {
    return;
  }

  e.preventDefault();

  let idx = e.target?.closest('.sector')?.getAttribute('data-idx');

  if (!idx) {
    return;
  }

  if (!context.game) {
    context.game = context.gameBuilderBuild(
      context.rows,
      context.cols,
      context.difficulty,
      parseInt(idx, 10),
      e.button,
      Math.round(Math.random() * 10),
    );

    return;
  }

  context.gameOnSelect(context.game, parseInt(idx, 10), e.button);
}

function onGameWon() {
  afterPaint(() => {
    alert(`${happyFace} Nice job, you've cleared the board without hitting any mines! Press OK to reload the page and play again.`)
    window.location.reload();
  });
}

function onGameLost() {
  afterPaint(() => {
    alert(`${unhappyFace} Oops, you hit a mine! Press OK to reload the page and play again.`);
    window.location.reload();
  });
}
