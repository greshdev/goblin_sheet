import DiceBox from "https://unpkg.com/@3d-dice/dice-box@1.0.8/dist/dice-box.es.min.js";

let Box = null
const colors = [
    "#348888",
    "#22BABB",
    "#9EF8EE",
    "#FA7F08",
    "#F24405",
    "#F25EB0",
    "#B9BF04",
    "#F2B705",
    "#F27405",
    "#F23005"
];
function get_random(list) {
    return list[Math.floor(Math.random() * list.length)];
}

export function roll_dice(dice_string) {
    let options = {
        themeColor: get_random(colors)
    };
    // Initialize the DiceBox runtime if we haven't initialized it already
    if (Box == null) {
        Box = new DiceBox("#dice-box", {
            assetPath: "assets/",
            origin: "https://unpkg.com/@3d-dice/dice-box@1.0.8/dist/",
            theme: "default",
            themeColor: "#feea03",
            offscreen: true,
            scale: 6
        });
        // We need to set the tab index for the canvas dice box will render on
        // to -1, so that it renders above items with a tab index of 0 (such
        // as certain list elements in Bootstrap)
        Box.init().then(async (_) => Box.roll([dice_string], options))
    } else {
        Box.roll([dice_string], options);
    }
}
