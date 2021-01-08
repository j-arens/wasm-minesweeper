const gameTemplate = document.createElement('template');

gameTemplate.innerHTML = `
  <table>
    <tbody>
    </tbody>
  </table>
`;

export function renderGame(
  root: HTMLElement,
  rows: number,
  cols: number,
): void {
  let templateClone = gameTemplate.content.cloneNode(true) as HTMLTemplateElement;
  let body = templateClone.querySelector('tbody');

  for (let i = 0; i < rows; i++) {
    let row = document.createElement('tr');

    for (let j = 0; j < cols; j++) {
      let sector = document.createElement('td');
      sector.classList.add('sector');
      sector.dataset.idx = `${cols * i + j}`;
      row.appendChild(sector);
    }

    body?.appendChild(row);
  }

  root.innerHTML = '';
  root.appendChild(templateClone);
}

type SectorAttributes = {
  [k: string]: string;
}

export function updateSector(
  root: HTMLElement,
  idx: number,
  attributes: SectorAttributes,
): void {
  let sector = root.querySelector(`.sector[data-idx="${idx}"]`);

  if (!sector) {
    return;
  }

  for (let [key, value] of Object.entries(attributes)) {
    sector.setAttribute(key, value);
  }
}

export function afterPaint(fn: () => void): void {
  requestAnimationFrame(() => {
    requestAnimationFrame(fn);
  });
}
