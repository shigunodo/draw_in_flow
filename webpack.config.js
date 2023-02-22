module.exports = {
  entry: `./www/main.js`,
  output: {
    path: `${__dirname}/dist`,
    filename: "main.js",
    publicPath: ""
  },
  mode: "production",
  experiments: {
    topLevelAwait: true,
  },
};