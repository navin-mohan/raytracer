import * as raytracer from "raytracer";

console.log(raytracer.greet());

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



function handler() {
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
    setTimeout(function() {
        const startTime = performance.now()
        const image = raytracer.render_image(image_height, image_width, samples_per_pixel, max_depth);
        const endTime = performance.now()

        setTimeTakenValue(endTime - startTime);

        const image_data = new ImageData(new Uint8ClampedArray(image), image_width, image_height);
        ctx.putImageData(image_data, 0, 0);
    }, 0);
}

document.getElementById("generate_button").addEventListener('click', handler);