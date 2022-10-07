import * as raytracer from "raytracer";

self.onmessage = ({ data : { image_height, image_width, samples_per_pixel, max_depth}}) => {
    console.log("Staring render on worker");
    const startTime = performance.now()
    const image = new ImageData(new Uint8ClampedArray(
        raytracer.render_image(image_height, image_width, samples_per_pixel, max_depth)
    ), image_width, image_height);
    const endTime = performance.now()
    console.log("Rendering complete", image);
    self.postMessage({
        image,
        time_taken: endTime - startTime
    });
};