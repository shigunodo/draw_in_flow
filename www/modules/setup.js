import init, {Fluid} from "../../pkg/draw_in_flow.js";

const wasm = await init();

export const canvas = document.getElementById("flow-calculator");
export const ctx = canvas.getContext('2d');

export const fpsViewer = document.getElementById("fps-viewer");

export const dragViewer = document.getElementById("drag-viewer");
export const liftViewer = document.getElementById("lift-viewer");

export const maNumber = 0.1;
export const uUniform = maNumber / Math.sqrt(3.0);
export const fluid = Fluid.new();

export const niPlot = fluid.get_ni();
export const njPlot = fluid.get_nj();
export const niCells = fluid.get_ni_cells();
export const njCells = fluid.get_nj_cells();
export const ratePlot = fluid.get_rate_plot();

const cellsPtr = fluid.get_ptr();
export const cells = new Uint8Array(wasm.memory.buffer, cellsPtr, niPlot * njPlot * 3);

export const dtIter = 0.05;

export const playPauseButton = document.getElementById("play-pause");
export const playPauseLabel = document.getElementById("play-pause-label");
export const stopButton = document.getElementById("initialize");

export const velocityButton = document.getElementById("velocity");
export const vorticityButton = document.getElementById("vorticity");
export const pressureButton = document.getElementById("pressure");
export const streamOnButton = document.getElementById("stream-on");
export const streamOffButton = document.getElementById("stream-off");

export const statusIndicator = document.getElementById("status");

export const calcButton = document.getElementById("calc");
export const paintButton = document.getElementById("paint");

export const reynoldsRange = document.getElementById("reynolds-number");
export const reynoldsLabel = document.getElementById("reynolds-label");

export const penButton = document.getElementById("pen");
export const eraserButton = document.getElementById("eraser");
export const cleanerButton = document.getElementById("cleaner");

export const cylinderButton = document.getElementById("cylinder");
export const airfoilButton = document.getElementById("airfoil");
export const naButton = document.getElementById("hiragana-na");

export const widthRange = document.getElementById("width-range");
export const widthLabel = document.getElementById("width-label");

export const velocityScale = document.getElementById("velocity-scale");
export const vorticityScale = document.getElementById("vorticity-scale");
export const pressureScale = document.getElementById("pressure-scale");
export const velocityOrigin = document.getElementById("velocity-origin");
export const vorticityOrigin = document.getElementById("vorticity-origin");
export const pressureOrigin = document.getElementById("pressure-origin");

export const playerSection = document.getElementById("player");
export const plotTypeSection = document.getElementById("plot-type");
export const streamSwitchSection = document.getElementById("stream-switch");
export const playerWrapper = document.getElementById("player-wrapper");
export const painterWrapper = document.getElementById("painter-wrapper");
export const calculatorWrapper = document.getElementById("calculator-wrapper");

export const velocityScaleWrapper = document.getElementById("velocity-scale-wrapper");
export const vorticityScaleWrapper = document.getElementById("vorticity-scale-wrapper");
export const pressureScaleWrapper = document.getElementById("pressure-scale-wrapper");
export const velocityOriginWrapper = document.getElementById("velocity-origin-wrapper");
export const vorticityOriginWrapper = document.getElementById("vorticity-origin-wrapper");
export const pressureOriginWrapper = document.getElementById("pressure-origin-wrapper");