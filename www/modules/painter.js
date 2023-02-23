import {
  canvas, ctx, fluid,
  ratePlot,
  penButton, eraserButton, cleanerButton,
  cylinderButton, airfoilButton, naButton,
  widthRange, widthLabel,
} from "./setup.js";
import {CELL_SIZE, drawCells} from "./drawer.js";

let painting = false;
let cX = [];
let cY = [];
let rateClient = 1.0;
let height = canvas.height;
let drawMode = 0; // 0: pen, 1: eraser

let lineWidthForCell = 3;
let paintedLineWidth = lineWidthForCell * ratePlot * CELL_SIZE;

const updateCells = () => {
  fluid.plot_cells();
  drawCells();
}

const drawPaintedCells = (oldX, oldY, newX, newY) => {
  const oldCX = oldX/CELL_SIZE/ratePlot;
  const oldCY = oldY/CELL_SIZE/ratePlot;
  const newCX = newX/CELL_SIZE/ratePlot;
  const newCY = newY/CELL_SIZE/ratePlot;
  fluid.update_cells(oldCX, oldCY, newCX, newCY, lineWidthForCell, drawMode);
}

const paint = (e) => {
    if (!painting) return;
    const offsetX = e.offsetX * rateClient;
    const offsetY = height - e.offsetY * rateClient;
    ctx.beginPath();
    ctx.moveTo(cX[cX.length-1], cY[cY.length-1]);
    ctx.lineTo(offsetX, offsetY);
    ctx.stroke();
    cX.push(offsetX);
    cY.push(offsetY);
}

const paintTouched = (e) => {
  if (!painting) return;
  const rect = e.target.getBoundingClientRect();
  const imax = e.touches.length;
  for (let i = 0; i < imax; i++) {
    //const offsetX_sub = (e.touches[0].clientX - window.pageXOffset - rect.left); 
    //const offsetY_sub = (e.touches[0].clientY - window.pageYOffset - rect.top);
    const offsetX_sub = (e.touches[i].clientX - rect.left); 
    const offsetY_sub = (e.touches[i].clientY - rect.top);
    const offsetX = offsetX_sub * rateClient;
    const offsetY = height - offsetY_sub * rateClient;
    ctx.beginPath();
    ctx.moveTo(cX[cX.length-1], cY[cY.length-1]);
    ctx.lineTo(offsetX, offsetY);
    ctx.stroke();
    cX.push(offsetX);
    cY.push(offsetY);
  }
}

const touch = (e) => {
  painting = true;
  const {width: width, clientWidth: clientWidth} = canvas;
  rateClient = width / clientWidth;
  height = canvas.height;
  cX.push(e.offsetX * rateClient);
  cY.push(height - e.offsetY * rateClient);
  cX.push(e.offsetX * rateClient);
  cY.push(height - e.offsetY * rateClient);
}

const touchTouched = (e) => {
  painting = true;
  const rect = e.target.getBoundingClientRect();
  const offsetX = (e.touches[0].clientX - rect.left); 
  const offsetY = (e.touches[0].clientY - rect.top);
  const {width: width, clientWidth: clientWidth} = canvas;
  rateClient = width / clientWidth;
  height = canvas.height;
  cX.push(offsetX * rateClient);
  cY.push(height - offsetY * rateClient);
  cX.push(offsetX * rateClient);
  cY.push(height - offsetY * rateClient);
}

const detouch = () => {
  painting = false;
  for (let i = 0; i < cX.length-1; i++) {
    drawPaintedCells(cX[i], cY[i], cX[i+1], cY[i+1]);
  }
  drawCells();
  cX.length = 0;
  cY.length = 0;
}

export const initCanvasAsPainter = () => {
  if (drawMode === 1) {
    ctx.strokeStyle = '#06283D';
  } else {
    ctx.strokeStyle = '#47B5FF';
  }
  ctx.lineJoin = 'round'; 
  ctx.lineCap = 'round';
  ctx.lineWidth = paintedLineWidth;
  canvas.addEventListener('mousemove', paint);
  canvas.addEventListener('mousedown', touch);
  canvas.addEventListener('mouseup', detouch);
  canvas.addEventListener('mouseout', detouch);
  canvas.addEventListener('touchmove', paintTouched);
  canvas.addEventListener('touchstart', touchTouched);
  canvas.addEventListener('touchend', detouch);
  updateCells();
}

export const endPainter = () => {
  canvas.removeEventListener('mousemove', paint);
  canvas.removeEventListener('mousedown', touch);
  canvas.removeEventListener('mouseup', detouch);
  canvas.removeEventListener('mouseout', detouch);
  canvas.removeEventListener('touchmove', paintTouched);
  canvas.removeEventListener('touchstart', touchTouched);
  canvas.removeEventListener('touchend', detouch);
}

penButton.addEventListener("click", event => {
  ctx.strokeStyle = '#47B5FF';
  drawMode = 0;
});

eraserButton.addEventListener("click", event => {
  ctx.strokeStyle = '#06283D';
  drawMode = 1;
});

cleanerButton.addEventListener("click", event => {
  fluid.init_cells();
  updateCells();
});

cylinderButton.addEventListener("click", event => {
  fluid.init_cells();
  fluid.set_cylinder();
  updateCells();
});

airfoilButton.addEventListener("click", event => {
  fluid.init_cells();
  fluid.set_airfoil();
  updateCells();
});

naButton.addEventListener("click", event => {
  fluid.init_cells();
  fluid.set_na();
  updateCells();
});

widthRange.addEventListener("input", event => {
  lineWidthForCell = widthRange.valueAsNumber;
  paintedLineWidth = lineWidthForCell * ratePlot * CELL_SIZE;
  ctx.lineWidth = paintedLineWidth;
  widthLabel.textContent = ('0' + lineWidthForCell).slice(-2);
});