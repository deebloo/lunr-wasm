module.exports = {
  workspaceRoot: false,
  mount: {
    www: '/',
    pkg: {
      url: '/lunr-wasm',
      static: true,
      resolve: false,
    },
  },
  optimize: {
    minify: true,
    target: 'esnext',
  },
  plugins: ['@snowpack/plugin-typescript'],
};