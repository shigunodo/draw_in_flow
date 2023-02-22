import {fluid, uUniform, 
  dragViewer, liftViewer, fpsViewer, 
  dtIter, 
  playPauseButton, playPauseLabel, stopButton, 
  maNumber,
  velocityButton, vorticityButton, pressureButton, 
  streamOnButton, streamOffButton, 
  reynoldsRange, reynoldsLabel,
  statusIndicator,
  velocityScale, vorticityScale, pressureScale,
  velocityScaleWrapper, vorticityScaleWrapper, pressureScaleWrapper,
  velocityOrigin, vorticityOrigin, pressureOrigin,
  velocityOriginWrapper, vorticityOriginWrapper, pressureOriginWrapper,
} from "./setup.js";
import {drawCells} from "./drawer.js";

let reNumber = 1.0e3;

let startTime = new Date().getTime();
let arrIntervalTime = [0.0, 0.0, 0.0, 0.0, 0.0];
let arrdrag = [0.0, 0.0, 0.0, 0.0, 0.0];
let arrLift = [0.0, 0.0, 0.0, 0.0, 0.0];

let animationId = null;

let vmaxForVelocity = 1.0;
let vmaxForVorticity = 0.015;
let vmaxForPressure = 0.01;
let vzeroForVelocity = 1.0;
let vzeroForVorticity = 0.0;
let vzeroForPressure = 1.0;
let plotQuantity = 0;
let streamPlot = 1;

let vmin = [];
let vmax = [];
const setScale = () => {
  vmin = [(vzeroForVelocity - vmaxForVelocity) * uUniform, 
    vzeroForVorticity - vmaxForVorticity, 
    vzeroForPressure - vmaxForPressure - 1.0];
  vmax = [(vzeroForVelocity + vmaxForVelocity) * uUniform, 
    vzeroForVorticity + vmaxForVorticity, 
    vzeroForPressure + vmaxForPressure - 1.0];
}
setScale();

const resetScale = () => {
  vmaxForVelocity = 1.0;
  vmaxForVorticity = 0.015;
  vmaxForPressure = 0.01;
  vzeroForVelocity = 1.0;
  vzeroForVorticity = 0.0;
  vzeroForPressure = 1.0;
  setScale();
  velocityScale.value = "1.0";
  vorticityScale.value = "0.015";
  pressureScale.value = "0.01";
  velocityOrigin.value = "1.0";
  vorticityOrigin.value = "0.0";
  pressureOrigin.value = "1.0";
}

const maxFps = 8.0;
const minInterval = 1000.0 / maxFps;

export const initFluid = () => {
  fluid.init(reNumber, maNumber);
  fluid.plot(plotQuantity, streamPlot, vmin[plotQuantity], vmax[plotQuantity], dtIter, 0);
  drawCells();
}

export const initFluidWithCylinder = () => {
  fluid.init(reNumber, maNumber);
  fluid.set_cylinder();
  fluid.find_physical_boundary();
  fluid.plot(plotQuantity, streamPlot, vmin[plotQuantity], vmax[plotQuantity], dtIter, 0);
  drawCells();
}

export const plotFluidQuantity = () => {
  fluid.plot(plotQuantity, streamPlot, vmin[plotQuantity], vmax[plotQuantity], dtIter, 0);
  drawCells();
}

const toggleStatus = () => {
  if (statusIndicator.checked === true) {
    statusIndicator.checked = false;
  } else {
    statusIndicator.checked = true;
  }
}

export const setScalesDisabled = () => {
  velocityScale.setAttribute("disabled", true);
  vorticityScale.setAttribute("disabled", true);
  pressureScale.setAttribute("disabled", true);
  velocityOrigin.setAttribute("disabled", true);
  vorticityOrigin.setAttribute("disabled", true);
  pressureOrigin.setAttribute("disabled", true);
}

export const setScalesAbled = () => {
  velocityScale.removeAttribute("disabled");
  vorticityScale.removeAttribute("disabled");
  pressureScale.removeAttribute("disabled");
  velocityOrigin.removeAttribute("disabled");
  vorticityOrigin.removeAttribute("disabled");
  pressureOrigin.removeAttribute("disabled");
}

const renderLoop = () => {

  fluid.iterate(dtIter, plotQuantity, streamPlot, vmin[plotQuantity], vmax[plotQuantity], reNumber, maNumber);
  toggleStatus();

  drawCells();

  arrdrag.shift();
  arrdrag.push(fluid.get_drag());
  const meandrag = arrdrag.reduce((sum, element) => sum + element, 0.0) / arrdrag.length;
  dragViewer.textContent = meandrag.toExponential(1);
  arrLift.shift();
  arrLift.push(fluid.get_lift());
  const meanLift = arrLift.reduce((sum, element) => sum + element, 0.0) / arrLift.length;
  liftViewer.textContent = meanLift.toExponential(1);

  let interval;
  let endTime;
  do {
    endTime = new Date().getTime();
    interval = endTime - startTime;
  } while (interval < minInterval);
  arrIntervalTime.shift();
  arrIntervalTime.push(interval);
  startTime = endTime;
  const meanInterval = arrIntervalTime.reduce((sum, element) => sum + element, 0.0) / arrIntervalTime.length;
  const fps = 1000.0 / meanInterval;
  fpsViewer.textContent = fps.toFixed(1);

  animationId = requestAnimationFrame(renderLoop);
};

const isPaused = () => {
  return animationId === null;
};

const play = () => {
  playPauseLabel.innerHTML = `<i class="fa-solid fa-pause"></i>`;
  playPauseButton.checked = true;
  setScalesDisabled();
  renderLoop();
};

export const pause = () => {
  playPauseLabel.innerHTML = `<i class="fa-solid fa-play"></i>`;
  playPauseButton.checked = false;
  cancelAnimationFrame(animationId);
  animationId = null;
  setScalesAbled();
};

playPauseButton.addEventListener("click", event => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

stopButton.addEventListener("click", event => {
  pause();
  resetScale();
  initFluid();
});

velocityButton.addEventListener("click", event => {
  plotQuantity = 0;
  fluid.plot(plotQuantity, streamPlot, vmin[plotQuantity], vmax[plotQuantity], dtIter, 0);
  drawCells();
  velocityScaleWrapper.style.display = "block";
  vorticityScaleWrapper.style.display = "none";
  pressureScaleWrapper.style.display = "none";
  velocityOriginWrapper.style.display = "block";
  vorticityOriginWrapper.style.display = "none";
  pressureOriginWrapper.style.display = "none";
});

vorticityButton.addEventListener("click", event => {
  plotQuantity = 1;
  fluid.plot(plotQuantity, streamPlot, vmin[plotQuantity], vmax[plotQuantity], dtIter, 0);
  drawCells();
  velocityScaleWrapper.style.display = "none";
  vorticityScaleWrapper.style.display = "block";
  pressureScaleWrapper.style.display = "none";
  velocityOriginWrapper.style.display = "none";
  vorticityOriginWrapper.style.display = "block";
  pressureOriginWrapper.style.display = "none";
});

pressureButton.addEventListener("click", event => {
  plotQuantity = 2;
  fluid.plot(plotQuantity, streamPlot, vmin[plotQuantity], vmax[plotQuantity], dtIter, 0);
  drawCells();
  velocityScaleWrapper.style.display = "none";
  vorticityScaleWrapper.style.display = "none";
  pressureScaleWrapper.style.display = "block";
  velocityOriginWrapper.style.display = "none";
  vorticityOriginWrapper.style.display = "none";
  pressureOriginWrapper.style.display = "block";
});

streamOnButton.addEventListener("click", event => {
  streamPlot = 1;
  fluid.plot(plotQuantity, streamPlot, vmin[plotQuantity], vmax[plotQuantity], dtIter, 0);
  drawCells();
});

streamOffButton.addEventListener("click", event => {
  streamPlot = 0;
  fluid.plot(plotQuantity, streamPlot, vmin[plotQuantity], vmax[plotQuantity], dtIter, 0);
  drawCells();
});

reynoldsRange.addEventListener("input", event => {
  const log = 0.69897 + reynoldsRange.valueAsNumber / 30;
  reNumber = Math.pow(10.0, log);
  reynoldsLabel.textContent = reNumber.toExponential(1);
});

velocityScale.addEventListener("input", event => {
  vmaxForVelocity = velocityScale.valueAsNumber;
  setScale();
  fluid.plot(plotQuantity, streamPlot, vmin[plotQuantity], vmax[plotQuantity], dtIter, 0);
  drawCells();
});

vorticityScale.addEventListener("input", event => {
  vmaxForVorticity = vorticityScale.valueAsNumber;
  setScale();
  fluid.plot(plotQuantity, streamPlot, vmin[plotQuantity], vmax[plotQuantity], dtIter, 0);
  drawCells();
});

pressureScale.addEventListener("input", event => {
  vmaxForPressure = pressureScale.valueAsNumber;
  setScale();
  fluid.plot(plotQuantity, streamPlot, vmin[plotQuantity], vmax[plotQuantity], dtIter, 0);
  drawCells();
});

velocityOrigin.addEventListener("input", event => {
  vzeroForVelocity = velocityOrigin.valueAsNumber;
  setScale();
  fluid.plot(plotQuantity, streamPlot, vmin[plotQuantity], vmax[plotQuantity], dtIter, 0);
  drawCells();
});

vorticityOrigin.addEventListener("input", event => {
  vzeroForVorticity = vorticityOrigin.valueAsNumber;
  setScale();
  fluid.plot(plotQuantity, streamPlot, vmin[plotQuantity], vmax[plotQuantity], dtIter, 0);
  drawCells();
});

pressureOrigin.addEventListener("input", event => {
  vzeroForPressure = pressureOrigin.valueAsNumber;
  setScale();
  fluid.plot(plotQuantity, streamPlot, vmin[plotQuantity], vmax[plotQuantity], dtIter, 0);
  drawCells();
});