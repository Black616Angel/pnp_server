var ctx = null
var memory

params_set_mem = function (wasm_memory, _wasm_exports) {
  memory = wasm_memory
  ctx = {}
  ctx.entries = []
  var some = new URLSearchParams(window.location.search)
  for (i of some.entries()) {
    ctx.entries.push(i)
  }
}

register_plugin = function (a) {
  a.env.miniquad_parameters_param_count = function () {
    return ctx.entries.length
  }
  a.env.miniquad_parameters_get_key = function (i) {
    return js_object(ctx.entries[i][0])
  }
  a.env.miniquad_parameters_get_value = function (i) {
    return js_object(ctx.entries[i][1])
  }
}

miniquad_add_plugin({
  register_plugin,
  on_init: params_set_mem,
  version: '0.1',
  name: 'params'
})
