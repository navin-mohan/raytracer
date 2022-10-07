import { createApp } from 'vue';


const worker = new Worker(new URL("./rt.js", import.meta.url));

worker.onmessage = ({ data: { image, time_taken } }) => {
    setTimeTakenValue(time_taken);
    const canvas = document.getElementById("canvas");
    const ctx = canvas.getContext("2d");
    ctx.putImageData(image, 0, 0);
    enableGenerateButton(true);
};

const app = createApp({
    data() {
        return {
            image_height: 100,
            image_width: 200,
            samples_per_pixel: 100,
            max_depth: 50,
            rendering_in_progress: false,
            time_taken: 0
        }
    },
    methods: {
        handle_generate(event) {
            const canvas = document.getElementById("canvas");
            const ctx = canvas.getContext("2d");
            ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
            this.rendering_in_progress = true;
            worker.postMessage({
                image_height: this.image_height,
                image_width: this.image_width, 
                samples_per_pixel: this.samples_per_pixel, 
                max_depth: this.max_depth
            });
        },
        handle_generate_complete(image, time_taken) {
            this.time_taken = time_taken;
            const canvas = document.getElementById("canvas");
            const ctx = canvas.getContext("2d");
            ctx.putImageData(image, 0, 0);
            this.rendering_in_progress = false;
        }
    },

    mounted() {
        worker.onmessage = ({ data: { image, time_taken } }) => this.handle_generate_complete(image, time_taken);
    }
});
app.mount('#app');