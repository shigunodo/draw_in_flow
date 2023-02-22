import {wasm, fluid, niPlot, njPlot, canvas, ctx, cells} from "./setup.js";

export const CELL_SIZE = 4; // px

const image = await loadImage("./logo-reversed.png");
const imageAspect = 133 / 400;
const height = CELL_SIZE * njPlot;
const width = CELL_SIZE * niPlot;
const logoWidth = 400;
const logoHeight = logoWidth * imageAspect;

function loadImage(url) {
  return new Promise(r => { let i = new Image(); i.onload = (() => r(i)); i.src = url; });
}

export const initCanvasAsPlotter = () => {
  fluid.find_physical_boundary();
  canvas.height = height;
  canvas.width = width;
  ctx.translate( 0, canvas.height );
  ctx.scale(1, -1);
}

export const drawCells = () => {
  ctx.beginPath();
  for (let j = 0; j < njPlot; j++) {
    for (let i = 0; i < niPlot; i++) {
      const idx = j * niPlot * 3 + i * 3;
      ctx.fillStyle = 'rgb(' + cells[idx] + ',' + cells[idx+1] + ',' + cells[idx+2] + ')';
      ctx.fillRect(
        i * CELL_SIZE,
        j * CELL_SIZE,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }
  ctx.stroke();
  ctx.drawImage(image, width-logoWidth, -20, logoWidth, logoHeight);
};

export const drawOneCell = (i, j) => {
  ctx.beginPath();
  const idx = j * niPlot * 3 + i * 3;
  ctx.fillStyle = 'rgb(' + cells[idx] + ',' + cells[idx+1] + ',' + cells[idx+2] + ')';
  ctx.fillRect(
    i * CELL_SIZE,
    j * CELL_SIZE,
    CELL_SIZE,
    CELL_SIZE
  );
  ctx.stroke();
}