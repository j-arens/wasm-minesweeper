export type GamePointer = number;

type WasmImports = {
  onExplode: (idx: number, isCatalyst: number) => void;
  onFlag: (idx: number) => void;
  onHide: (idx: number) => void;
  onGameLost: () => void;
  onGameWon: () => void;
  onReveal: (idx: number, proximity: number) => void;
  onQuestion: (idx: number) => void;
}

export type WasmExports = {
  gameBuilderBuild: (
    rows: number,
    cols: number,
    difficulty: number,
    freeSectorIdx: number,
    action: number,
    randomNum: number,
  ) => GamePointer;
  gameOnSelect: (game: GamePointer, idx: number, action: number) => void;
};

export async function loadWasm(imports: WasmImports): Promise<WasmExports> {
  let {
    instance: {
      exports,
    },
  } = await WebAssembly.instantiateStreaming(fetch('minesweeper.wasm'), {
    env: {
      explode: imports.onExplode,
      flag: imports.onFlag,
      hide: imports.onHide,
      lost: imports.onGameLost,
      reveal: imports.onReveal,
      question: imports.onQuestion,
      won: imports.onGameWon,
    },
  });

  return {
    gameBuilderBuild: exports.game_builder_build,
    gameOnSelect: exports.game_on_select,
  } as WasmExports;
}
