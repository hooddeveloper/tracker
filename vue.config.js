const path = require("path");

module.exports = {
chainWebpack: config => {
    config
      .entry("app")
      .clear()
      .add("./client/main.js")
      .end();
    config.resolve.alias
      .set("@", path.join(__dirname, "./client"))
  },
    outputDir: 'dist',
    assetsDir: 'static',
    devServer: {
        proxy: {
            '/api*': {
                // Forward frontend dev server request for /api to django dev server
                target: 'http://localhost:8000/',
            }
        }
    }
  }
