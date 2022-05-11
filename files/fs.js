register_plugin = function (a) {
    a.env.fs_write_file = function (url, content) {
        var xhr = new XMLHttpRequest();
        xhr.open("POST", url, true);
        //Send the proper header information along with the request
        xhr.setRequestHeader("Content-type", "application/json");
        xhr.send(content);
    }
}

miniquad_add_plugin({
    register_plugin,
    version: '0.1',
    name: 'fs'
})