import {
  calcButton, paintButton,
  playerSection, plotTypeSection, streamSwitchSection,
  playerWrapper, painterWrapper, calculatorWrapper,
} from "./modules/setup.js"; // for initial set up
import {initCanvasAsPlotter} from "./modules/drawer.js"; // drawing methods for canvas tag
import {initFluidWithCylinder, plotFluidQuantity, pause, setScalesDisabled, setScalesAbled} from "./modules/player.js";
import {initCanvasAsPainter, endPainter} from "./modules/painter.js";

document.addEventListener("dblclick", function(e){ e.preventDefault();}, { passive: false });
initCanvasAsPlotter();
initFluidWithCylinder();

paintButton.addEventListener("click", event => {
  pause();
  setScalesDisabled();
  initCanvasAsPainter();
  playerSection.setAttribute("disabled", true);
  plotTypeSection.setAttribute("disabled", true);
  streamSwitchSection.setAttribute("disabled", true);
  playerWrapper.style.display = "none";
  painterWrapper.style.display = "flex";
  calculatorWrapper.style.touchAction = "none";
});

calcButton.addEventListener("click", event => {
  endPainter();
  initCanvasAsPlotter();
  setScalesAbled();
  plotFluidQuantity();
  playerSection.removeAttribute("disabled");
  plotTypeSection.removeAttribute("disabled");
  streamSwitchSection.removeAttribute("disabled");
  playerWrapper.style.display = "flex";
  painterWrapper.style.display = "none";
  calculatorWrapper.style.touchAction = "auto";
});

const startTime = new Date().getTime();
let endTime;
do {
  endTime = new Date().getTime();
} while (endTime - startTime < 500);
const spinner = document.getElementById('loading');
spinner.classList.add('loaded');