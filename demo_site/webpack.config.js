module.exports = {
  experiments: {
    asyncWebAssembly: true,
    topLevelAwait: true
  },
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: 'webassembly/async'
      }
    ]
  },
  resolve: {
    extensions: ['.ts', '.js', '.wasm']
  },
  output: {
    webassemblyModuleFilename: '[hash].wasm',
    environment: {
      asyncFunction: true,
      bigIntLiteral: true,
      const: true,
      destructuring: true,
      dynamicImport: true,
      forOf: true,
      module: true
    }
  }
};
