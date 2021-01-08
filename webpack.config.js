const path = require('path');

module.exports = {
  devServer: {
    open: true,
    port: 7333,
  },

  entry: {
    index: path.join(__dirname, 'src', 'index.ts'),
  },

  mode: process.env.NODE_ENV || 'development',

  module: {
    rules: [
      {
        test: /\.ts$/,
        use: 'ts-loader',
      },
    ],
  },

  output: {
    path: __dirname,
    filename: '[name].bundle.js',
  },

  resolve: {
    extensions: [
      '.js',
      '.ts',
    ],
  },

  target: 'web',
};
