function getValue(id) {
    const el = document.getElementById(id);
    return el.value
}

function setTimeTakenValue(val) {
    document.getElementById("time_taken").innerText = `Time taken: ${val.toFixed(2)}ms`;
}

function startMessage() {
    document.getElementById("time_taken").innerText = 'Rendering in progress...';
}

function enableGenerateButton(enabled) {
    document.getElementById("generate_button").disabled = !enabled;
}

const worker = new Worker(new URL("./rt.js", import.meta.url));

worker.onmessage = ({ data: { image, time_taken } }) => {
    setTimeTakenValue(time_taken);
    const canvas = document.getElementById("canvas");
    const ctx = canvas.getContext("2d");
    ctx.putImageData(image, 0, 0);
    enableGenerateButton(true);
};

function handler() {
    enableGenerateButton(false);
    startMessage();
    console.log("Hanlder called");
    const image_height = getValue("image_height");
    const image_width = getValue("image_width");
    const samples_per_pixel = getValue("samples_per_pixel");
    const max_depth = getValue("max_depth");
    const canvas = document.getElementById("canvas");
    canvas.width = image_width;
    canvas.height = image_height;
    const ctx = canvas.getContext("2d");
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    worker.postMessage({
        image_height, image_width, samples_per_pixel, max_depth
    });
}

enableGenerateButton(true);
document.getElementById("generate_button").addEventListener('click', handler);