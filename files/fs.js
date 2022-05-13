register_plugin = function (a) {
    a.env.fs_write_file = function (url_ptr, url_len, content_ptr, content_len) {
        var url = UTF8ToString(url_ptr, url_len);
        var content = UTF8ToString(content_ptr, content_len);
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