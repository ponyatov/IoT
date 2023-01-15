import init, {hello, none, on_del, on_line, on_point, on_select, on_touch} from '/cad.js';


function register_callbacks() {
    // buttons
    $('#btn_select').click(on_select);
    $('#btn_point').click(on_point);
    $('#btn_line').click(on_line);
    $('#btn_del').click(on_del);
    // workspace
    $('#ws').click((e) => {on_touch([
                       e.clientX, e.clientY, e.offsetX, e.offsetY, e.screenX,
                       e.screenY, e.ctrlKey, e.altKey, e.shiftKey
                   ])});
}

export function show_status(s) { $('#status').text(s); }

(async () => {
    await init();
    // hello();
    register_callbacks();
    on_select();
})();


// $(document).ready(() => {
//     $('#btn_select').click(on_select);
// });
